mod bus;
mod cpu;
mod data_flow;

use bus::Bus;
use data_flow::ReadWrite;

fn main() {
  let mut main_bus = Bus::new();
  let addr = 0xFFFC;

  load_memory(&mut main_bus);

  let data = main_bus.read_addr(addr);
  println!("Data at {:#x?}: {:#x?}", addr, data);
}

fn load_memory(bus: &mut Bus) {
  // Program counter reset address
  bus.write_addr(0xFFFC, 0x40);
  bus.write_addr(0xFFFD, 0x20);
  // 5 steps of "program"
  bus.write_addr(0x4020, 0x01);
  bus.write_addr(0x4021, 0x02);
  bus.write_addr(0x4022, 0x03);
  bus.write_addr(0x4023, 0x04);
  bus.write_addr(0x4024, 0x05);
}
