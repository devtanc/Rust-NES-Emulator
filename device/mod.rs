const MAX_MEMORY: usize = 65_536;

pub trait ExactSize {
  fn mem_size(&self) -> usize;
}

pub trait Readable {
  fn read_addr(&self, addr: usize) -> Result<u8, u16>;
}

pub trait Writable {
  fn write_addr(&mut self, addr: usize, data: u8) -> Result<u8, u16>;
}

pub struct Device {
  memory: Box<[u8]>,
}

impl Device {
  pub fn new(memory: Box<[u8]>) -> Device {
    println!("Memory size: {:#x?}", memory.len());
    
    if memory.len() > MAX_MEMORY {
      panic!("Maximum memory size exceeded. Must not exceed {}", MAX_MEMORY)
    }

    Device { memory }
  }
}

impl ExactSize for Device {
  fn mem_size(&self) -> usize {
    self.memory.len()
  }
}

impl Writable for Device {
  fn write_addr(&mut self, addr: usize, data: u8) -> Result<u8, u16> {
    if self.memory.len() > addr {
      self.memory[addr] = data;
      Ok(data)
    } else {
      Err(self.mem_size() as u16)
    }
  }
}

impl Readable for Device {
  fn read_addr(&self, addr: usize) -> Result<u8, u16> {
    if self.memory.len() > addr {
      Ok(self.memory[addr])
    } else {
      Err(self.mem_size() as u16)
    }
  }
}
