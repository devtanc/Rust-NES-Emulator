use bus::Bus;
use device::{ExactSize, Readable, Writable};

struct Cpu<'a, T: ExactSize + Readable + Writable> {
  acc: u8,         // Accumulator
  x: u8,           // X Register
  y: u8,           // Y Register
  stkp: u8,        // Stack Pointer
  pc: u16,         // Program Counter
  status: u8,      // Status Register
  bus: &'a Bus<T>, // Connected BUS
}

impl<'a, T: ExactSize + Readable + Writable> Cpu<'a, T> {
  pub fn new_with_bus(bus: &'a Bus<T>) -> Cpu<'a, T> {
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

  pub fn new_with_values(
    acc: u8,
    x: u8,
    y: u8,
    stkp: u8,
    pc: u16,
    status: u8,
    bus: &'a Bus<T>,
  ) -> Cpu<'a, T> {
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

  // GETTERS
  #[allow(dead_code)]
  pub fn get_acc(&self) -> u8 {
    self.acc
  }
  #[allow(dead_code)]
  pub fn get_x(&self) -> u8 {
    self.x
  }
  #[allow(dead_code)]
  pub fn get_y(&self) -> u8 {
    self.y
  }
  #[allow(dead_code)]
  pub fn get_stkp(&self) -> u8 {
    self.stkp
  }
  #[allow(dead_code)]
  pub fn get_pc(&self) -> u16 {
    self.pc
  }
  #[allow(dead_code)]
  pub fn get_status(&self) -> u8 {
    self.status
  }
  // SETTERS
  #[allow(dead_code)]
  pub fn set_acc(&mut self, val: u8) {
    self.acc = val;
  }
  #[allow(dead_code)]
  pub fn set_x(&mut self, val: u8) {
    self.x = val;
  }
  #[allow(dead_code)]
  pub fn set_y(&mut self, val: u8) {
    self.y = val;
  }
  #[allow(dead_code)]
  pub fn set_stkp(&mut self, val: u8) {
    self.stkp = val;
  }
  #[allow(dead_code)]
  pub fn set_pc(&mut self, val: u16) {
    self.pc = val;
  }
  #[allow(dead_code)]
  pub fn set_status(&mut self, val: u8) {
    self.status = val;
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
  pub fn replace_bus(&mut self, bus: &'a Bus<T>) {
    self.bus = bus;
  }
}
