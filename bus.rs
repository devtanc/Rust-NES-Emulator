use connection::Connection;
use device::{ExactSize, Readable, Writable};

const MAX_MEMORY: usize = 65_536;

pub struct Bus<T: ExactSize + Readable + Writable> {
  connections: Vec<Connection<T>>,
  size: usize,
}

impl<T: ExactSize + Readable + Writable> Bus<T> {
  pub fn new() -> Bus<T> {
    Bus {
      connections: vec![],
      size: MAX_MEMORY,
    }
  }
  pub fn with_connection(device: T, addr: u16) -> Bus<T> {
    println!(
      "Device added at [{:#x?}] of size [{:#x?}]",
      addr,
      device.mem_size()
    );

    Bus {
      connections: vec![Connection::new(device, addr)],
      size: MAX_MEMORY,
    }
  }

  pub fn add_connection(&mut self, device: T, addr: u16) -> Result<u16, u16> {
    if self.is_memory_free(addr as usize, device.mem_size()) {
      println!(
        "Device added at [{:#x?}] of size [{:#x?}]",
        addr,
        device.mem_size()
      );
      self.connections.push(Connection::new(device, addr));
      Ok(addr)
    } else {
      Err(addr)
    }
  }

  // TODO: Implement this function to check that new connections don't overlap existing ones
  #[allow(dead_code)]
  fn is_memory_free(&self, addr: usize, size: usize) -> bool {
    println!(
      "Memory required from [{:#x?}] to [{:#x?}]",
      addr,
      addr + size
    );
    true
  }
}

impl<T: ExactSize + Readable + Writable> Readable for Bus<T> {
  fn read_addr(&self, addr: usize) -> Result<u8, u16> {
    if addr >= self.size {
      panic!(
        "Maximum memory size exceeded. Must not exceed {}",
        MAX_MEMORY
      )
    }

    let result = self
      .connections
      .iter()
      .find(|&c| c.is_addr_in_range(addr as u16));
    match result {
      Some(conn) => conn.read_addr(addr as u16),
      None => Err(addr as u16),
    }
  }
}
