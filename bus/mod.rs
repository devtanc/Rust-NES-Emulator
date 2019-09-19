use data_flow::{ReadWrite, ReadRange, HexSlice};

const MAX_MEMORY: usize = 64 * 1024; // 64 KB

pub struct Bus {
  ram: [u8; MAX_MEMORY],
}

impl Bus {
  pub fn new() -> Bus {
    Bus {
      ram: [0; MAX_MEMORY],
    }
  }
}

impl ReadWrite for Bus {
  fn write_addr(&mut self, addr: u16, data: u8) {
    match addr {
      0x0000..=0xFFFF => {
        self.ram[addr as usize] = data;
      }
    }
  }

  fn read_addr(&self, addr: u16) -> u8 {
    match addr {
      0x0000..=0xFFFF => self.ram[addr as usize],
    }
  }
}

impl ReadRange for Bus {
  fn read_range(&self, addr_start: u16, addr_end: u16) -> HexSlice {
    let start = addr_start as usize;
    let end = addr_end as usize;
    HexSlice::new(&self.ram[start..end])
  }
}
