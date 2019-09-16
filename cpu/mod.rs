use bus::Bus;
use data_flow::ReadWrite;

struct Cpu {
  acc: u8,    // Accumulator
  x: u8,      // X Register
  y: u8,      // Y Register
  stkp: u8,   // Stack Pointer
  pc: u16,    // Program Counter
  status: u8, // Status Register
  bus: Bus,   // Connected BUS
}

impl Cpu {
  pub fn new() -> Cpu {
    Cpu {
      acc: 0x00,
      x: 0x00,
      y: 0x00,
      stkp: 0x00,
      pc: 0xFFFC,
      status: 0x00,
      bus: Bus::new(),
    }
  }

  pub fn new_with_bus(bus: Bus) -> Cpu {
    Cpu {
      acc: 0x00,
      x: 0x00,
      y: 0x00,
      stkp: 0x00,
      pc: 0xFFFC,
      status: 0x00,
      bus,
    }
  }

  pub fn new_with_values(acc: u8, x: u8, y: u8, stkp: u8, pc: u16, status: u8, bus: Bus) -> Cpu {
    Cpu {
      acc,
      x,
      y,
      stkp,
      pc,
      status,
      bus,
    }
  }

  pub fn get_acc(&self) -> &u8 {
    &self.acc
  }
  pub fn get_x(&self) -> &u8 {
    &self.x
  }
  pub fn get_y(&self) -> &u8 {
    &self.y
  }
  pub fn get_stkp(&self) -> &u8 {
    &self.stkp
  }
  pub fn get_pc(&self) -> &u16 {
    &self.pc
  }

  pub fn set_acc(&mut self, val: u8) {
    self.acc = val;
  }
  pub fn set_x(&mut self, val: u8) {
    self.x = val;
  }
  pub fn set_y(&mut self, val: u8) {
    self.y = val;
  }
  pub fn set_stkp(&mut self, val: u8) {
    self.stkp = val;
  }
  pub fn set_pc(&mut self, val: u16) {
    self.pc = val;
  }

  // Interrupt: reset
  pub fn reset(&self) {}
  // Interrupt: request
  pub fn irq(&self) {}
  // Interrupt: non-maskable
  pub fn nmi(&self) {}
  // Perform one clock cycle
  pub fn clock(&self) {}
  // Returns if current cycle is complete
  pub fn is_cycle_complete(&self) -> bool {
    true
  }
  // Connects a new bus
  pub fn replace_bus(&mut self, bus: Bus) {
    self.bus = bus;
  }

  fn fetch_from_addr(&self, addr: u16) -> u8 {
    // TODO: IMPLEMENT THIS
    0b1
  }

  fn get_flag(&self, flag: u8) -> bool {
    const MASK: u8 = 0b00000001;
    let shift_amt: u32 = (flag - 1) as u32;
    match flag {
      1 => ((self.status & MASK.rotate_left(shift_amt)) >> (shift_amt)) != 0,
      2 => ((self.status & MASK.rotate_left(shift_amt)) >> (shift_amt)) != 0,
      3 => ((self.status & MASK.rotate_left(shift_amt)) >> (shift_amt)) != 0,
      4 => ((self.status & MASK.rotate_left(shift_amt)) >> (shift_amt)) != 0,
      5 => ((self.status & MASK.rotate_left(shift_amt)) >> (shift_amt)) != 0,
      6 => ((self.status & MASK.rotate_left(shift_amt)) >> (shift_amt)) != 0,
      7 => ((self.status & MASK.rotate_left(shift_amt)) >> (shift_amt)) != 0,
      8 => ((self.status & MASK.rotate_left(shift_amt)) >> (shift_amt)) != 0,
      _ => false,
    }
  }

  fn set_flag(&mut self, flag: u8, value: u8) {
    match value {
      0 => self.set_flag_false(flag),
      1 => self.set_flag_true(flag),
      _ => (),
    }
  }

  fn set_flag_true(&mut self, flag: u8) {
    match flag {
      1 => self.status |= 0b1 << (flag - 1),
      2 => self.status |= 0b1 << (flag - 1),
      3 => self.status |= 0b1 << (flag - 1),
      4 => self.status |= 0b1 << (flag - 1),
      5 => self.status |= 0b1 << (flag - 1),
      6 => self.status |= 0b1 << (flag - 1),
      7 => self.status |= 0b1 << (flag - 1),
      8 => self.status |= 0b1 << (flag - 1),
      _ => (),
    }
  }

  fn set_flag_false(&mut self, flag: u8) {
    const INV_MASK: u8 = 0b11111110;
    let shift_amt: u32 = (flag - 1) as u32;

    match flag {
      1 => self.status &= INV_MASK.rotate_left(shift_amt),
      2 => self.status &= INV_MASK.rotate_left(shift_amt),
      3 => self.status &= INV_MASK.rotate_left(shift_amt),
      4 => self.status &= INV_MASK.rotate_left(shift_amt),
      5 => self.status &= INV_MASK.rotate_left(shift_amt),
      6 => self.status &= INV_MASK.rotate_left(shift_amt),
      7 => self.status &= INV_MASK.rotate_left(shift_amt),
      8 => self.status &= INV_MASK.rotate_left(shift_amt),
      _ => (),
    }
  }
}

impl ReadWrite for Cpu {
  fn read_addr(&self, addr: u16) -> u8 {
    self.bus.read_addr(addr)
  }
  fn write_addr(&mut self, addr: u16, data: u8) {
    self.bus.write_addr(addr, data)
  }
}
