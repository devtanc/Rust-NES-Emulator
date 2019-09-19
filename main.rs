extern crate termion;
extern crate tui;

mod bus;
mod cpu;
mod data_flow;
mod event;

use bus::Bus;
use cpu::Cpu;
use data_flow::{HexByte, MemoryAddress, ReadRange, ReadWrite};
use event::{Config, Event, Events};

use std::io;
use std::time::Duration;

use termion::event::Key;
use termion::raw::IntoRawMode;

use tui::backend::{Backend, TermionBackend};
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Paragraph, Text, Widget};
use tui::{Frame, Terminal};

const PROGRAM_START_ADDR: u16 = 0x8000;
const ZERO_PAGE_START: u16 = 0x0000;
const DEFAULT_TICK_RATE: u64 = 200;

fn main() -> Result<(), failure::Error> {
  let events = Events::with_config(Config {
    tick_rate: Duration::from_millis(DEFAULT_TICK_RATE),
    ..Config::default()
  });
  let stdout = io::stdout().into_raw_mode()?;
  let backend = TermionBackend::new(stdout);
  let mut terminal = Terminal::new(backend)?;

  let mut cpu = Cpu::new();

  load_program_memory(cpu.get_mut_bus_ref());

  cpu.reset();

  println!("{}", termion::clear::All);

  loop {
    draw_ui(&mut terminal, &mut cpu)?;
    match events.next()? {
      Event::Input(key) => match key {
        Key::Char(' ') => {
          cpu.step();
        }
        Key::Char('q') => {
          break;
        }
        _ => {}
      },
      Event::Tick => {
        () // Do something here if you need to in between clocks
      }
    }
  }

  println!("{}", termion::clear::All);
  Ok(())
}

fn draw_ui<B>(terminal: &mut Terminal<B>, cpu: &mut Cpu) -> Result<(), io::Error>
where
  B: Backend,
{
  terminal.draw(|mut f| {
    let chunks = Layout::default()
      .direction(Direction::Horizontal)
      .margin(1)
      .constraints([Constraint::Percentage(60), Constraint::Percentage(40)].as_ref())
      .split(f.size());
    draw_memory_panel(&mut f, chunks[0], cpu.get_mut_bus_ref());
    draw_instructions_panel(&mut f, chunks[1], cpu);
  })
}

fn load_program_memory(bus: &mut Bus) {
  // Load program counter reset addresses with program start address
  bus.write_addr(0xFFFC, ((PROGRAM_START_ADDR >> 8) & 0x00FF) as u8);
  bus.write_addr(0xFFFD, (PROGRAM_START_ADDR & 0x00FF) as u8);

  // Load program into memory
  const PROGRAM: &str =
    "A2 0A 8E 00 00 A2 03 8E 01 00 AC 00 00 A9 00 18 6D 01 00 88 D0 FA 8D 02 00 EA EA EA";
  let mut start = PROGRAM_START_ADDR;
  let steps = PROGRAM.split(" ");
  for step in steps {
    match u8::from_str_radix(step, 16) {
      Ok(data) => {
        bus.write_addr(start, data);
        start += 1;
      }
      Err(_) => (),
    }
  }
}

fn draw_instructions_panel<B>(f: &mut Frame<B>, area: Rect, cpu: &Cpu)
where
  B: Backend,
{
  let chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
    .split(area);
  draw_status_register(f, chunks[0], cpu);
  Block::default()
    .title(" Instructions ")
    .borders(Borders::ALL)
    .render(f, chunks[1]);
}

fn draw_status_register<B>(f: &mut Frame<B>, area: Rect, cpu: &Cpu)
where
  B: Backend,
{
  let flags = ['N', 'V', 'U', 'B', 'D', 'I', 'Z', 'C'];
  let mut text = vec![];
  text.push(Text::raw("Status:       "));
  for flag in &flags {
    match cpu.get_flag(*flag) {
      true => text.push(Text::styled(
        format!("{}  ", flag),
        Style::default().modifier(Modifier::BOLD).fg(Color::Green),
      )),
      false => text.push(Text::styled(
        format!("{}  ", flag),
        Style::default().modifier(Modifier::BOLD).fg(Color::Red),
      )),
    };
  }
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

  text.push(Text::raw("\nCurr Opcode:    "));
  let opcode = HexByte::new(*cpu.get_opcode());
  text.push(Text::raw(format!("0x{}", opcode)));

  text.push(Text::raw("\nData Addr:      "));
  let data_addr = MemoryAddress::new(*cpu.get_addr_of_data());
  text.push(Text::raw(format!("0x{}", data_addr)));

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
        .title(" Registers ")
        .title_style(Style::default().fg(Color::Cyan).modifier(Modifier::BOLD)),
    )
    .wrap(false)
    .render(f, area);
}

fn draw_memory_panel<B>(f: &mut Frame<B>, area: Rect, bus: &Bus)
where
  B: Backend,
{
  let chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
    .split(area);
  draw_memory_page(f, chunks[0], bus, ZERO_PAGE_START, " Zero Page ");
  draw_memory_page(f, chunks[1], bus, PROGRAM_START_ADDR, " Program Memory ");
}

fn draw_memory_page<B>(f: &mut Frame<B>, area: Rect, bus: &Bus, starting_address: u16, title: &str)
where
  B: Backend,
{
  let mut text = vec![];
  let ending_address = starting_address + 320;
  for i in (starting_address..=ending_address).step_by(16) {
    let root_addr = MemoryAddress::new(i);
    text.push(Text::styled(
      format!("${}:   ", root_addr),
      Style::default().modifier(Modifier::BOLD),
    ));
    text.push(Text::raw(format!("{}", bus.read_range(i, i + 16))));
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
