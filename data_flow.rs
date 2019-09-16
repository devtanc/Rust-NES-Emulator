pub trait Read {
  fn read_addr(&self, addr: u16) -> u8;
}

pub trait Write {
  fn write_addr(&mut self, addr: u16, data: u8);
}

pub trait ReadWrite {
  fn read_addr(&self, addr: u16) -> u8;

  fn write_addr(&mut self, addr: u16, data: u8);
}