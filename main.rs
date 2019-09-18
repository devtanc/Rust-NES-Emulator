mod bus;
mod cpu;
mod data_flow;

use bus::Bus;
use data_flow::{ReadRange, ReadWrite};

const PROGRAM_START_ADDR: u16 = 0x8000;
const ZERO_PAGE_START: u16 = 0x0000;

fn main() {
  let mut main_bus = Bus::new();

  load_memory(&mut main_bus);

  let program = main_bus.read_range(PROGRAM_START_ADDR, 0x8020);
  for (i, dat) in program.iter().enumerate() {
    println!(
      "Data at {:#x?}: {:#x?}",
      (PROGRAM_START_ADDR as usize) + i,
      dat
    );
  }

  let zero_page = main_bus.read_range(ZERO_PAGE_START, 0x0010);
  for (i, dat) in zero_page.iter().enumerate() {
    println!(
      "Data at {:#x?}: {:#x?}",
      (ZERO_PAGE_START as usize) + i,
      dat
    );
  }
}

fn load_memory(bus: &mut Bus) {
  // Program counter reset address
  bus.write_addr(0xFFFC, ((PROGRAM_START_ADDR >> 8) & 0x00FF) as u8);
  bus.write_addr(0xFFFD, (PROGRAM_START_ADDR & 0x00FF) as u8);

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
