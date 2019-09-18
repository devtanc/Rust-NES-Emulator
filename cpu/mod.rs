pub mod instruction;

use bus::Bus;
use cpu::instruction::{get_instruction, AddressMode};
use data_flow::ReadWrite;

const RESET_ADDRESS: u16 = 0xFFFC;
const LO_BYTE_MASK: u16 = 0x00FF;
const HI_BYTE_MASK: u16 = 0xFF00;

fn flag_from_char(flag: char) -> u8 {
  match flag {
    'C' => 1,
    'Z' => 2,
    'I' => 3,
    'D' => 4,
    'B' => 5,
    'U' => 6,
    'V' => 7,
    'N' => 8,
    _ => 0,
  }
}

struct Cpu {
  bus: Bus, // Connected BUS
  // 6502 registers
  acc: u8,    // Accumulator
  x: u8,      // X Register
  y: u8,      // Y Register
  stkp: u8,   // Stack Pointer
  pc: u16,    // Program Counter
  status: u8, // Status Register
  // Code variables
  current_tick: usize, // Current clock tick count
  cycles: u8,          // Cycles remaining for current instruction
  opcode: u8,          // Current instruction byte
  addr_abs: u16,       // Any absolute address value
  addr_rel: u16,       // Absolute address, but used when following a branch
}

impl Cpu {
  pub fn new() -> Cpu {
    Cpu {
      bus: Bus::new(),

      acc: 0x00,
      x: 0x00,
      y: 0x00,
      stkp: 0x00,
      pc: 0xFFFC,
      status: 0x00,

      current_tick: 0,
      cycles: 0x00,
      opcode: 0x00,
      addr_abs: 0x0000,
      addr_rel: 0x0000,
    }
  }

  pub fn new_with_bus(bus: Bus) -> Cpu {
    Cpu {
      bus,

      acc: 0x00,
      x: 0x00,
      y: 0x00,
      stkp: 0x00,
      pc: 0xFFFC,
      status: 0x00,

      current_tick: 0,
      cycles: 0x00,
      opcode: 0x00,
      addr_abs: 0x0000,
      addr_rel: 0x0000,
    }
  }

  pub fn new_with_values(acc: u8, x: u8, y: u8, stkp: u8, pc: u16, status: u8, bus: Bus) -> Cpu {
    Cpu {
      bus,

      acc,
      x,
      y,
      stkp,
      pc,
      status,
      current_tick: 0,
      cycles: 0x00,
      opcode: 0x00,
      addr_abs: 0x0000,
      addr_rel: 0x0000,
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
  // Reset all internal values to defaults
  pub fn reset(&mut self) {
    let hi = self.read_addr(RESET_ADDRESS + 0) as u16;
    let lo = self.read_addr(RESET_ADDRESS + 1) as u16;

    self.pc = (hi << 8) | lo;

    self.acc = 0;
    self.x = 0;
    self.y = 0;
    self.stkp = 0xFD;
    self.status = match self.get_flag('U') {
      true => 0b00000100,
      false => 0b00000000,
    };

    self.addr_abs = 0x0000;
    self.addr_rel = 0x0000;

    self.cycles = 8;
  }
  // Interrupt: request
  pub fn irq(&self) {}
  // Interrupt: non-maskable
  pub fn nmi(&self) {}
  // Perform one clock cycle
  pub fn clock(&mut self) {
    if self.cycles == 0 {
      // Read the program counter
      self.opcode = self.read_pc_addr();
      // Always set the unused flag to 1
      self.set_flag('U', 1);
      // Get the instruction for the opcode
      let instruction = get_instruction(self.opcode);

      // Get Starting number of cycles
      self.cycles = instruction.get_cycles();

      // Perform fetch of intermediate data using the
      // required addressing mode
      let data_addr = match *instruction.get_address_mode() {
        AddressMode::Absolute => self.get_byte_at_pc(),
        AddressMode::AbsoluteIndirect => {
          // Read contents of next two memory locations
          // containing lo byte, then hi byte of pointer to data
          let ptr = self.get_byte_at_pc();

          // Read lo and hi bytes at ptr location for final addr
          let lo_pc_addr = self.read_addr(ptr) as u16;
          let hi_pc_addr: u16;

          // SIMULATE PAGE BOUNDARY HARDWARE BUG
          if ptr & LO_BYTE_MASK == 0xFF {
            hi_pc_addr = self.read_addr(ptr & 0xFF00) as u16;
          } else {
            hi_pc_addr = self.read_addr(ptr + 1) as u16;
          }

          // Return final, "dereferenced" addr
          (hi_pc_addr << 8) | lo_pc_addr
        }
        AddressMode::AbsoluteX => {
          let addr = self.get_byte_at_pc();
          let offset_addr = addr + (self.x as u16);

          if (offset_addr & HI_BYTE_MASK) != (addr & HI_BYTE_MASK) {
            self.cycles += 1;
          }
          offset_addr
        }
        AddressMode::AbsoluteY => {
          let addr = self.get_byte_at_pc();
          let offset_addr = addr + (self.y as u16);

          if (offset_addr & HI_BYTE_MASK) != (addr & HI_BYTE_MASK) {
            self.cycles += 1;
          }
          offset_addr
        }
        AddressMode::Accumulator => self.acc as u16,
        AddressMode::Immediate => self.read_pc_addr() as u16, // TODO: is this right? This looks like ZP...
        AddressMode::Implied => self.acc as u16,              // TODO: is this right?
        AddressMode::IndirectX => {
          let offset = self.read_pc_addr();
          let zero_ptr = (self.x + offset) as u16;

          self.read_addr_from(zero_ptr)
        }
        AddressMode::IndirectY => {
          let zero_ptr = self.read_pc_addr() as u16;
          let contents_from_zero_page = self.read_addr(zero_ptr) as u16;

          let sum_with_carry = (self.y as u16) + contents_from_zero_page; // Cast to u16 to preserve carry
          let carry = (sum_with_carry & HI_BYTE_MASK) >> 8;

          let lo = sum_with_carry & LO_BYTE_MASK;
          let hi = (self.read_addr(zero_ptr + 1) as u16) + carry;

          (hi << 8) | lo
        }
        AddressMode::Relative => self.read_pc_addr() as u16, // This is the offset (8 bit signed) to add to the PC
        AddressMode::ZeroPage => self.read_pc_addr() as u16,
        AddressMode::ZeroPageX => (self.read_pc_addr() + self.x) as u16,
        AddressMode::ZeroPageY => (self.read_pc_addr() + self.y) as u16,
        AddressMode::XXX => 0x0000,
      };

    // Perform operation

    // The addressmode and opcode may have altered the number
    // of cycles this instruction requires before its completed
    } else {
      self.current_tick += 1;
      self.cycles -= 1;
    }
  }
  // Returns if current cycle is complete
  pub fn is_cycle_complete(&self) -> bool {
    self.cycles == 0
  }
  // Connects a new bus
  pub fn replace_bus(&mut self, bus: Bus) {
    self.bus = bus;
  }

  // TODO: Why not just use read_addr for this?
  fn fetch_from_addr(&self, addr: u16) -> u8 {
    self.read_addr(addr)
  }

  fn get_flag(&self, flag: char) -> bool {
    let flag_val = flag_from_char(flag);

    match flag_val {
      1..=8 => {
        const MASK: u8 = 0b00000001;
        let shift_amt: u32 = (flag_val - 1) as u32;

        ((self.status & MASK.rotate_left(shift_amt)) >> (shift_amt)) != 0
      }
      _ => false,
    }
  }

  fn set_flag(&mut self, flag: char, value: u8) {
    let flag_val = flag_from_char(flag);
    match flag_val {
      1..=8 => match value {
        0 => self.set_flag_false(flag_val),
        1 => self.set_flag_true(flag_val),
        _ => (),
      },
      _ => (),
    }
  }

  fn set_flag_with_bool(&mut self, flag: u8, value: bool) {
    match value {
      false => self.set_flag_false(flag),
      true => self.set_flag_true(flag),
    }
  }

  fn set_flag_true(&mut self, flag: u8) {
    self.status |= 0b1 << (flag - 1)
  }

  fn set_flag_false(&mut self, flag: u8) {
    const INV_MASK: u8 = 0b11111110;
    let shift_amt: u32 = (flag - 1) as u32;

    self.status &= INV_MASK.rotate_left(shift_amt)
  }

  fn read_pc_addr(&mut self) -> u8 {
    let result = self.read_addr(self.pc);
    self.pc += 1;
    result
  }

  fn get_byte_at_pc(&mut self) -> u16 {
    let lo = self.read_pc_addr() as u16;
    let hi = self.read_pc_addr() as u16;
    (hi << 8) | lo
  }

  fn read_addr_from(&self, addr: u16) -> u16 {
    let lo = self.read_addr(addr) as u16;
    let hi = self.read_addr(addr + 1) as u16;
    (hi << 8) | lo
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
