mod bus;
mod connection;
mod cpu;
mod device;

use bus::Bus;
use device::{Device, Readable};

const MAX_MEMORY: usize = 65_536;

fn main() {
  let mut memory = Box::new([0 as u8; MAX_MEMORY]);

  load_memory(&mut memory);

  let ram = Device::new(memory);
  let main_bus = Bus::with_connection(ram, 0);

  match main_bus.read_addr(0xFFFC) {
    Ok(data) => println!("Data at 0: {:#x?}", data),
    Err(err) => println!("couldn't read {}", err),
  }
}

fn load_memory(mem: &mut Box<[u8; MAX_MEMORY]>) {
  // Program counter reset address
  mem[0xFFFC] = 0x40;
  mem[0xFFFD] = 0x20;
  // 5 steps of "program"
  mem[0x4020] = 0x01;
  mem[0x4021] = 0x02;
  mem[0x4022] = 0x03;
  mem[0x4023] = 0x04;
  mem[0x4024] = 0x05;
}
