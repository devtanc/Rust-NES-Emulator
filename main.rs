mod bus;
mod connection;
mod cpu;
mod device;

use bus::Bus;
use device::{Device, Readable, Writable};

const MAX_MEMORY: usize = 65_536;

fn main() {
  let mut ram = Device::new(Box::new([0; MAX_MEMORY]));
  match ram.write_addr(0, 0xFF) {
    Ok(_) => {
      let main_bus = Bus::with_connection(ram, 0);
      match main_bus.read_addr(0) {
        Ok(data) => println!("Data at 0: {:#x?}", data),
        Err(err) => println!("couldn't read {}", err),
      }
    }
    Err(res) => println!("couldn't write {}", res),
  };
}
