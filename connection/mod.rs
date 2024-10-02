use device::{ExactSize, Readable, Writable};

pub struct Connection<T: ExactSize + Readable + Writable> {
  addr_start: u16,
  device: T,
}

impl<T: ExactSize + Readable + Writable> Connection<T> {
  pub fn new(device: T, addr_start: u16) -> Connection<T> {
    Connection { addr_start, device }
  }

  pub fn get_start(&self) -> u16 {
    self.addr_start
  }

  pub fn get_end(&self) -> u16 {
    let end = self.addr_start as usize + self.device.mem_size() - 1;
    end as u16
  }

  pub fn is_addr_in_range(&self, addr: u16) -> bool {
    addr >= self.get_start() && addr <= self.get_end()
  }

  pub fn write_addr(&mut self, addr: u16, data: u8) -> Result<u8, u16> {
    self.device.write_addr(addr as usize, data)
  }

  pub fn read_addr(&self, addr: u16) -> Result<u8, u16> {
    self.device.read_addr(addr as usize)
  }
}
