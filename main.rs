extern crate termion;
extern crate tui;

mod bus;
mod cpu;
mod data_flow;
mod event;

use bus::Bus;
use cpu::instruction::{get_instruction, Instruction};
use cpu::Cpu;
use data_flow::{HexByte, HexSlice, MemoryAddress, ReadRange, ReadWrite};
use event::{Config, Event, Events};

use std::convert::TryInto;
use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::time::Duration;

use termion::event::Key;
use termion::raw::IntoRawMode;

use tui::backend::{Backend, TermionBackend};
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Paragraph, Text, Widget};
use tui::{Frame, Terminal};

const CARTRIDGE_ADDR: u16 = 0x4020;
const ZERO_PAGE_START: u16 = 0x0000;
const STACK_BASE_ADDR: u16 = 0x0100;
const DEFAULT_TICK_RATE: u64 = 200;
const BYTES_PER_ROW: u16 = 16;
// const MEMORY_WINDOW_START_ADDRESS: u16 = 0xC000;

fn main() -> Result<(), failure::Error> {
  let args: Vec<String> = env::args().collect();
  let filename = &args[1];
  let events = Events::with_config(Config {
    tick_rate: Duration::from_millis(DEFAULT_TICK_RATE),
    ..Config::default()
  });
  let stdout = io::stdout().into_raw_mode()?;
  let backend = TermionBackend::new(stdout);
  let mut terminal = Terminal::new(backend)?;
  let mut history: Vec<Text> = Vec::new();
  let mut current_tick = 4;
  let mut new_tick: bool;

  let mut cpu = Cpu::new();

  load_program_memory(cpu.get_mut_bus_ref(), filename)?;

  cpu.reset();
  // This section sets the clock to 4 after the reset
  cpu.step();
  println!("{}", termion::clear::All);

  loop {
    if current_tick != *cpu.get_current_tick() {
      current_tick = *cpu.get_current_tick();
      new_tick = true;
    } else {
      new_tick = false;
    }

    // if *cpu.get_current_tick() < 14_600 {
    //   cpu.clock();
    //   continue;
    // }
    draw_ui(&mut terminal, &mut cpu, &mut history, new_tick)?;
    match events.next()? {
      Event::Input(key) => match key {
        Key::Char(' ') => {
          cpu.step();
        }
        Key::Right => {
          cpu.clock();
        }
        Key::Char('r') => {
          cpu.reset();
        }
        Key::Char('q') => {
          break;
        }
        _ => {}
      },
      Event::Tick => (),
    }
  }

  println!("{}", termion::clear::All);
  Ok(())
}

fn draw_ui<B>(terminal: &mut Terminal<B>, cpu: &mut Cpu, history: &mut Vec<Text>, new_tick: bool) -> Result<(), io::Error>
where
  B: Backend,
{
  terminal.draw(|mut f| {
    let chunks = Layout::default()
      .direction(Direction::Horizontal)
      .margin(1)
      .constraints([Constraint::Percentage(60), Constraint::Percentage(40)].as_ref())
      .split(f.size());
    draw_memory_panel(&mut f, chunks[0], cpu);
    draw_instructions_panel(&mut f, chunks[1], cpu, history, new_tick);
  })
}

fn load_program_memory(bus: &mut Bus, filename: &String) -> std::io::Result<()> {
  let mut cartridge = File::open(filename)?;
  let metadata = cartridge.metadata()?;
  let mut cartridge_memory = Vec::with_capacity(metadata.len().try_into().unwrap());
  cartridge.read_to_end(&mut cartridge_memory)?;

  if filename.contains("/nestest.nes") {
    load_nestest(bus, &cartridge_memory);
  } else {
    let mut current_address = CARTRIDGE_ADDR;
    for byte in cartridge_memory {
      bus.write_addr(current_address, byte);
      if current_address == 0xFFFF {
        break;
      }
      current_address += 1;
    }
  }

  Ok(())
}

fn load_nestest(bus: &mut Bus, memory: &Vec<u8>) {
  let mirrored_memory = &memory[0x0010..0x4010];
  let mut current_address = 0x8000 - 1;

  for byte in mirrored_memory {
    bus.write_addr(current_address, *byte);
    current_address += 1;
  }
  current_address = 0xC000 - 1;
  for byte in mirrored_memory {
    current_address += 1;
    bus.write_addr(current_address, *byte);
  }
}

fn log(cpu: &Cpu, history: &mut Vec<Text>) {
  let current_clock_cycle = cpu.get_current_tick();
  let ppc = MemoryAddress::new(*cpu.get_ppc());

  let opcode = cpu.get_opcode_at(*cpu.get_ppc());
  let hex_opcode = HexByte::new(opcode);
  let instruction = get_instruction(opcode);

  // Format data addr
  let data_addr = *cpu.get_addr_of_data();
  let data_addr_struct = MemoryAddress::new(data_addr);
  let data_hi = (data_addr & 0xFF00) >> 8;
  let data_lo = data_addr & 0x00FF;
  let data_lo_hex = HexByte::new(data_lo as u8);
  let data_hi_hex = HexByte::new(data_hi as u8);

  // Registers
  let status_hex = HexByte::new(*cpu.get_status());
  let acc_hex = HexByte::new(*cpu.get_acc());
  let x_hex = HexByte::new(*cpu.get_x());
  let y_hex = HexByte::new(*cpu.get_y());
  let stack_ptr_hex = HexByte::new(*cpu.get_stkp());
  
  history.push(
    Text::raw(
      format!("{}  {} {} {}  {} ${}                       A:{} X:{} Y:{} P:{} SP:{} PPU:{},{} CYC:{}\n",
        ppc,
        hex_opcode,
        data_lo_hex,
        data_hi_hex,
        instruction.get_operation(),
        data_addr_struct,
        acc_hex,
        x_hex,
        y_hex,
        status_hex,
        stack_ptr_hex,
        "000",
        "000",
        current_clock_cycle
      )
    )
  );
}

fn draw_instructions_panel<B>(f: &mut Frame<B>, area: Rect, cpu: &mut Cpu, history: &mut Vec<Text>, new_tick: bool)
where
  B: Backend,
{
  // Populate the log
  if new_tick {
    log(cpu, history);
  }

  // Write to the screen
  // let addr_hex = MemoryAddress::new(MEMORY_WINDOW_START_ADDRESS);
  // let memory_view_title = format!(" Program memory starting at 0x{} ", addr_hex);
  let chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints(
      [
        Constraint::Percentage(20),
        Constraint::Percentage(20),
        Constraint::Percentage(60),
      ]
      .as_ref(),
    )
    .split(area);
  draw_registers(f, chunks[0], cpu);
  draw_cpu_process_info(f, chunks[1], cpu);
  draw_history(f, chunks[2], history);
  // draw_memory_page(
  //   f,
  //   chunks[2],
  //   cpu,
  //   MEMORY_WINDOW_START_ADDRESS,
  //   48 * BYTES_PER_ROW,
  //   &memory_view_title,
  //   0x0200,
  // );
}

fn draw_registers<B>(f: &mut Frame<B>, area: Rect, cpu: &Cpu)
where
  B: Backend,
{
  let flags = ['N', 'V', 'U', 'B', 'D', 'I', 'Z', 'C'];
  let mut text = Vec::with_capacity(35);

  text.push(Text::raw("Status Flags:   "));
  for flag in &flags {
    match cpu.get_flag(*flag) {
      true => text.push(Text::styled(
        format!("{}  ", expand_flag(*flag)),
        Style::default().modifier(Modifier::BOLD).fg(Color::Green),
      )),
      false => text.push(Text::styled(
        format!("{}  ", expand_flag(*flag)),
        Style::default().modifier(Modifier::BOLD).fg(Color::Red),
      )),
    };
  }

  text.push(Text::raw("\n\nStatus HEX:     "));
  let status = *cpu.get_status();
  let status_hex = HexByte::new(status);
  text.push(Text::raw(format!("0x{}  ", status_hex)));
  text.push(Text::raw(format!("[{}]", status)));

  text.push(Text::raw("\nAccumulator:    "));
  let acc = *cpu.get_acc();
  let acc_hex = HexByte::new(acc);
  text.push(Text::raw(format!("0x{}  ", acc_hex)));
  text.push(Text::raw(format!("[{}]", acc)));

  text.push(Text::raw("\nX:              "));
  let x = *cpu.get_x();
  let x_hex = HexByte::new(x);
  text.push(Text::raw(format!("0x{}  ", x_hex)));
  text.push(Text::raw(format!("[{}]", x)));

  text.push(Text::raw("\nY:              "));
  let y = *cpu.get_y();
  let y_hex = HexByte::new(y);
  text.push(Text::raw(format!("0x{}  ", y_hex)));
  text.push(Text::raw(format!("[{}]", y)));

  text.push(Text::raw("\nStack Ptr:      "));
  let stkp = HexByte::new(*cpu.get_stkp());
  text.push(Text::raw(format!("0x{}", stkp)));

  text.push(Text::raw("\nProg Counter:   "));
  let pc = MemoryAddress::new(*cpu.get_pc());
  text.push(Text::raw(format!("0x{}", pc)));

  Paragraph::new(text.iter())
    .block(
      Block::default()
        .borders(Borders::ALL)
        .title(" Registers ")
        .title_style(Style::default().fg(Color::Cyan).modifier(Modifier::BOLD)),
    )
    .wrap(false)
    .render(f, area);
}

fn draw_cpu_process_info<B>(f: &mut Frame<B>, area: Rect, cpu: &Cpu)
where
  B: Backend,
{
  let mut text = Vec::with_capacity(35);

  text.push(Text::raw("Executed opcode addr: "));
  let ppc = MemoryAddress::new(*cpu.get_ppc());
  text.push(Text::raw(format!("0x{}", ppc)));

  text.push(Text::raw("\nExecuted opcode: "));
  let opcode = cpu.get_opcode_at(*cpu.get_ppc());
  let hex_opcode = HexByte::new(opcode);
  let instruction = get_instruction(opcode);
  let data_addr = *cpu.get_addr_of_data();
  let data_addr_struct = MemoryAddress::new(data_addr);
  let data = HexByte::new(cpu.read_addr(data_addr));
  text.push(Text::raw(format!("0x{}  -> ", hex_opcode)));
  text.push(Text::raw(format!("[{}]:", instruction.get_operation())));
  text.push(Text::raw(format!("[{}]:", instruction.get_address_mode())));
  text.push(Text::raw(format!("[0x{}]:", data_addr_struct)));
  text.push(Text::raw(format!("[0x{}]", data)));

  text.push(Text::raw("\nOpcode Cycles:  "));
  let curr_command_cycles = cpu.get_cycles();
  text.push(Text::raw(format!("{}", curr_command_cycles)));

  text.push(Text::raw("\nCurr Cycle:     "));
  let tick_counter = cpu.get_current_tick();
  text.push(Text::raw(format!("{}", tick_counter)));

  Paragraph::new(text.iter())
    .block(
      Block::default()
        .borders(Borders::ALL)
        .title(" CPU Instruction Processing ")
        .title_style(Style::default().fg(Color::Cyan).modifier(Modifier::BOLD)),
    )
    .wrap(false)
    .render(f, area);
}

fn expand_flag(flag: char) -> String {
  match flag {
    'N' => "NEG".to_string(),
    'V' => "OVR".to_string(),
    'U' => "-U-".to_string(),
    'B' => "BRK".to_string(),
    'D' => "DEC".to_string(),
    'I' => "INT".to_string(),
    'Z' => "ZR0".to_string(),
    'C' => "CAR".to_string(),
    _ => "".to_string(),
  }
}

fn draw_memory_panel<B>(f: &mut Frame<B>, area: Rect, cpu: &mut Cpu)
where
  B: Backend,
{
  let chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints(
      [
        Constraint::Percentage(32),
        Constraint::Percentage(32),
        Constraint::Percentage(32),
        Constraint::Percentage(4),
      ]
      .as_ref(),
    )
    .split(area);
  draw_memory_page(
    f,
    chunks[0],
    cpu,
    ZERO_PAGE_START,
    18 * BYTES_PER_ROW,
    " Zero Page ",
    *cpu.get_addr_of_data(),
  );
  draw_memory_page(
    f,
    chunks[1],
    cpu,
    STACK_BASE_ADDR,
    15 * BYTES_PER_ROW,
    " Stack ",
    *cpu.get_stkp() as u16 + STACK_BASE_ADDR,
  );
  draw_memory_page(
    f,
    chunks[2],
    cpu,
    *cpu.get_pc(),
    18 * BYTES_PER_ROW,
    " Program Memory ",
    *cpu.get_pc(),
  );
  draw_memory_page(
    f,
    chunks[3],
    cpu,
    0xFFF0,
    1 * BYTES_PER_ROW,
    " Interrupt Vectors ",
    *cpu.get_addr_of_data(),
  );
}

fn draw_history<B>(f: &mut Frame<B>, area: Rect, history: &mut Vec<Text>)
where
  B: Backend,
{
  Paragraph::new(history.iter().rev().take(42).rev())
    .block(
      Block::default()
        .borders(Borders::ALL)
        .title("OPERATION HISTORY")
        .title_style(Style::default().fg(Color::Cyan).modifier(Modifier::BOLD)),
    )
    .wrap(true)
    .render(f, area);
}

fn draw_memory_page<B>(
  f: &mut Frame<B>,
  area: Rect,
  cpu: &mut Cpu,
  address: u16,
  offset: u16,
  title: &str,
  highlight_address: u16,
) where
  B: Backend,
{
  let bus = cpu.get_mut_bus_ref();
  let starting_address = match address {
    0x0000..=0xFFF0 => address & 0xFFF0,
    0xFFF1..=0xFFFF => 0xFFF0,
  };
  let ending_address = match (starting_address as u32) + (offset as u32) {
    0x0000..=0xFFFF => starting_address + offset,
    _ => 0xFFFF,
  };
  let vec_size = if highlight_address != 0 {
    (((ending_address - starting_address) / 16) as usize) * 5
  } else {
    (((ending_address - starting_address) / 16) as usize) * 3
  };
  let mut text = Vec::with_capacity(vec_size);

  for i in (starting_address..=ending_address).step_by(16) {
    let root_addr = MemoryAddress::new(i);
    let offset_addr = i + 15;
    let range = bus.read_range(i, offset_addr);
    text.push(Text::styled(
      format!("${}:   ", root_addr),
      Style::default().modifier(Modifier::BOLD).fg(Color::Yellow),
    ));
    if highlight_address != 0 && highlight_address >= i && highlight_address - i < 0x10 {
      let highlight_offset = (highlight_address - i) as usize;
      match highlight_offset {
        0x00 => {
          let highlight = &[range[0]];
          let after = &range[1..];

          text.push(Text::styled(
            format!("{}", HexSlice::new(highlight)),
            Style::default().bg(Color::DarkGray),
          ));
          text.push(Text::raw(format!("{}", HexSlice::new(after))));
        }
        0x01..=0xFE => {
          let before = &range[..highlight_offset];
          let highlight = &[range[highlight_offset]];
          let after = &range[(highlight_offset + 1)..];

          text.push(Text::raw(format!("{}", HexSlice::new(before))));
          text.push(Text::styled(
            format!("{}", HexSlice::new(highlight)),
            Style::default().bg(Color::DarkGray),
          ));
          text.push(Text::raw(format!("{}", HexSlice::new(after))));
        }
        0xFF => {
          let before = &range[..(highlight_offset - 1)];
          let highlight = &[range[0xFF]];

          text.push(Text::raw(format!("{}", HexSlice::new(before))));
          text.push(Text::styled(
            format!("{}", HexSlice::new(highlight)),
            Style::default().bg(Color::DarkGray),
          ));
        }
        _ => (),
      };
    } else {
      text.push(Text::raw(format!("{}", HexSlice::new(range))));
    }
    text.push(Text::raw("\n"));
  }

  Paragraph::new(text.iter())
    .block(
      Block::default()
        .borders(Borders::ALL)
        .title(title)
        .title_style(Style::default().fg(Color::Cyan).modifier(Modifier::BOLD)),
    )
    .wrap(true)
    .render(f, area);
}
