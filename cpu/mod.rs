pub mod instruction;

use bus::Bus;
use cpu::instruction::{get_instruction, AddressMode, Operation};
use data_flow::ReadWrite;

const RESET_ADDRESS: u16 = 0xFFFC;
const LO_BYTE_MASK: u16 = 0x00FF;
const HI_BYTE_MASK: u16 = 0xFF00;
const STACK_BASE_ADDR: u16 = 0x0100;
const BRK_ADDR_BEGIN: u16 = 0xFFFE;

fn flag_from_char(flag: char) -> u8 {
  match flag {
    'C' => 1, // C	Carry
    'Z' => 2, // Z	Zero
    'I' => 3, // I	Interrupt (IRQ disable)
    'D' => 4, // D	Decimal (use BCD for arithmetics)
    'B' => 5, // B	Break
    'U' => 6, // -	ignored
    'V' => 7, // V	Overflow
    'N' => 8, // N	Negative
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
  pub fn irq(&mut self) {
    if !self.get_flag('I') {
      self.stack_push(((self.pc >> 8) & 0x00FF) as u8);
      self.stack_push((self.pc & 0x00FF) as u8);

      self.set_flag('B', 0);
      self.set_flag('U', 1);
      self.set_flag('I', 1);
      self.stack_push(self.status);
      self.pc = self.read_addr_from(BRK_ADDR_BEGIN);

      self.cycles = 7;
    }
  }
  // Interrupt: non-maskable
  pub fn nmi(&mut self) {
    self.stack_push(((self.pc >> 8) & 0x00FF) as u8);
    self.stack_push((self.pc & 0x00FF) as u8);

    self.set_flag('B', 0);
    self.set_flag('U', 1);
    self.set_flag('I', 1);
    self.stack_push(self.status);
    self.pc = self.read_addr_from(BRK_ADDR_BEGIN);

    self.cycles = 8;
  }
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

      // Get pointer to the data that will be used in the operation
      let data_ptr = self.get_data_ptr(instruction.get_address_mode());

      // Perform the operation
      self.perform_operation(
        data_ptr,
        instruction.get_operation(),
        instruction.get_address_mode(),
      );
    } else {
      self.current_tick += 1;
      self.cycles -= 1;
    }
  }

  fn perform_operation(&mut self, ptr: u16, operation: &Operation, address_mode: &AddressMode) {
    let data: u8 = self.read_addr(ptr);

    match operation {
      Operation::ADC => {
        let result = (self.acc as u16) + (data as u16) + (self.get_flag('C') as u16);
        self.set_flag_with_bool('C', result > 0xFF);
        self.set_flag_with_bool('Z', result == 0);
        self.set_flag_with_bool(
          'V',
          ((self.acc as u16) ^ result & !((self.acc as u16) ^ (data as u16))) & 0x0080 > 0,
        );
        self.set_flag_with_bool('N', result & 0x0080 > 0);
        self.acc = (result & 0xFF) as u8;
      }
      Operation::AND => {
        let result = self.acc & data;
        self.set_flag_with_bool('Z', result == 0);
        self.set_flag('N', (result & 0x0080) >> 7);
      }
      Operation::ASL => {
        let result = data << 1;
        self.set_flag_with_bool('C', result < data);
        self.set_flag_with_bool('Z', result == 0);
        self.set_flag_with_bool('N', result & 0x0080 > 0);
      }
      Operation::BCC => {
        if !self.get_flag('C') {
          self.branch(data as u16);
        }
      }
      Operation::BCS => {
        if self.get_flag('C') {
          self.branch(data as u16);
        }
      }
      Operation::BEQ => {
        if self.get_flag('Z') {
          self.branch(data as u16);
        }
      }
      Operation::BIT => {
        let result = self.acc & data;
        self.set_flag_with_bool('Z', result == 0);
        self.set_flag_with_bool('N', data & 0x80 > 0);
        self.set_flag_with_bool('V', data & 0x40 > 0);
      }
      Operation::BMI => {
        if self.get_flag('N') {
          self.branch(data as u16);
        }
      }
      Operation::BNE => {
        if !self.get_flag('Z') {
          self.branch(data as u16);
        }
      }
      Operation::BPL => {
        if !self.get_flag('N') {
          self.branch(data as u16);
        }
      }
      Operation::BRK => {
        // Set interrput flag
        self.set_flag_with_bool('I', true);
        self.pc += 1;
        // Store incremented PC address bytes on stack
        let pc_hi = (self.pc & 0xFF00) >> 8;
        let pc_lo = self.pc & 0x00FF;
        self.stack_push(pc_hi as u8);
        self.stack_push(pc_lo as u8);
        // Store status on stack with B flag set
        self.set_flag('B', 1);
        self.stack_push(self.status);
        // Reset B flag
        self.set_flag('B', 0);
        // Read destination into pc
        self.pc = self.read_addr_from(BRK_ADDR_BEGIN);
      }
      Operation::BVC => {
        if !self.get_flag('V') {
          self.branch(data as u16);
        }
      }
      Operation::BVS => {
        if self.get_flag('V') {
          self.branch(data as u16);
        }
      }
      Operation::CLC => self.set_flag('C', 0),
      Operation::CLD => self.set_flag('D', 0),
      Operation::CLI => self.set_flag('I', 0),
      Operation::CLV => self.set_flag('V', 0),
      Operation::CMP => {
        let result = (self.acc as u16) - (data as u16);
        self.set_flag_with_bool('Z', result == 0);
        self.set_flag_with_bool('C', self.acc >= data);
        self.set_flag_with_bool('N', result & 0x80 > 0);
      }
      Operation::CPX => {
        let result = (self.x as u16) - (data as u16);
        self.set_flag_with_bool('Z', result == 0);
        self.set_flag_with_bool('C', self.x >= data);
        self.set_flag_with_bool('N', result & 0x80 > 0);
      }
      Operation::CPY => {
        let result = (self.y as u16) - (data as u16);
        self.set_flag_with_bool('Z', result == 0);
        self.set_flag_with_bool('C', self.y >= data);
        self.set_flag_with_bool('N', result & 0x80 > 0);
      }
      Operation::DEC => {
        let result = data - 1;
        self.set_flag_with_bool('Z', result == 0);
        self.set_flag_with_bool('N', (result & 0x80) > 0);
        self.write_addr(ptr, result & 0x00FF);
      }
      Operation::DEX => {
        let result = self.x - 1;
        self.set_flag_with_bool('Z', result == 0);
        self.set_flag_with_bool('N', (result & 0x80) > 0);
        self.x = result & 0x00FF;
      }
      Operation::DEY => {
        let result = self.y - 1;
        self.set_flag_with_bool('Z', result == 0);
        self.set_flag_with_bool('N', (result & 0x80) > 0);
        self.y = result & 0x00FF;
      }
      Operation::EOR => {
        let result = self.acc ^ data;
        self.set_flag_with_bool('Z', result == 0);
        self.set_flag_with_bool('N', (result & 0x80) > 0);
        self.acc = result & 0x00FF;
      }
      Operation::INC => {
        let result = data + 1;
        self.set_flag_with_bool('Z', result == 0);
        self.set_flag_with_bool('N', (result & 0x80) > 0);
        self.write_addr(ptr, result & 0x00FF);
      }
      Operation::INX => {
        let result = self.x + 1;
        self.set_flag_with_bool('Z', result == 0);
        self.set_flag_with_bool('N', (result & 0x80) > 0);
        self.x = result & 0x00FF;
      }
      Operation::INY => {
        let result = self.y + 1;
        self.set_flag_with_bool('Z', result == 0);
        self.set_flag_with_bool('N', (result & 0x80) > 0);
        self.y = result & 0x00FF;
      }
      Operation::JMP => self.pc = self.get_byte_at_pc(),
      Operation::JSR => {
        self.pc -= 1;
        self.stack_push(((self.pc >> 8) & 0xFF00) as u8);
        self.stack_push((self.pc & 0x00FF) as u8);
        self.pc = ptr;
      }
      Operation::LDA => {
        self.set_flag_with_bool('Z', data == 0);
        self.set_flag_with_bool('N', (data & 0x80) > 0);
        self.acc = data
      }
      Operation::LDX => {
        self.set_flag_with_bool('Z', data == 0);
        self.set_flag_with_bool('N', (data & 0x80) > 0);
        self.x = data
      }
      Operation::LDY => {
        self.set_flag_with_bool('Z', data == 0);
        self.set_flag_with_bool('N', (data & 0x80) > 0);
        self.y = data
      }
      Operation::LSR => {
        self.set_flag_with_bool('C', (data & 0x01) > 0);
        let result = data >> 1;
        self.set_flag('N', 0);
        self.set_flag_with_bool('Z', result == 0);
        match address_mode {
          &AddressMode::Implied => self.acc = result & 0x00FF,
          _ => self.write_addr(ptr, result & 0x00FF),
        };
      }
      Operation::NOP => {
        match self.opcode {
          0xFC => self.cycles += 1,
          _ => (),
        };
      }
      Operation::ORA => {
        let result = self.acc | data;
        self.set_flag_with_bool('Z', result == 0);
        self.set_flag_with_bool('N', (result & 0x80) > 0);
        self.acc = result;
      }
      Operation::PHA => {
        self.write_addr(STACK_BASE_ADDR + self.stkp as u16, self.acc);
        self.stkp -= 1;
      }
      Operation::PHP => self.stack_push(self.status),
      Operation::PLA => self.acc = self.stack_pop(),
      Operation::PLP => self.status = self.stack_pop(),
      Operation::ROL => {
        let result = (data as u16) << 1 | self.get_flag('C') as u16;
        self.set_flag_with_bool('C', result & 0xFF00 > 0);
        self.set_flag_with_bool('Z', result == 0);
        self.set_flag_with_bool('N', result & 0x0080 > 0);
        match address_mode {
          &AddressMode::Implied => self.acc = (result & 0x00FF) as u8,
          _ => self.write_addr(ptr, (result & 0x00FF) as u8),
        };
      }
      Operation::ROR => {
        let result = (self.get_flag('C') as u16) << 7 | ((data as u16) >> 1);
        self.set_flag_with_bool('C', result & 0x0001 > 0);
        self.set_flag_with_bool('Z', (result & 0x00FF) == 0);
        self.set_flag_with_bool('N', result & 0x0080 > 0);
        match address_mode {
          &AddressMode::Implied => self.acc = (result & 0x00FF) as u8,
          _ => self.write_addr(ptr, (result & 0x00FF) as u8),
        };
      }
      Operation::RTI => {
        self.status = self.stack_pop();
        self.set_flag('B', 0);
        self.set_flag('U', 0);
        let lo = self.stack_pop() as u16;
        let hi = self.stack_pop() as u16;
        self.pc = (hi << 8) | lo;
      }
      Operation::RTS => {
        let lo = self.stack_pop() as u16;
        let hi = self.stack_pop() as u16;
        self.pc = (hi << 8) | lo;
        self.pc += 1;
      }
      Operation::SBC => {
        let inverted_data = (data as u16) ^ 0x00FF;
        let result = (self.acc as u16) + inverted_data + (self.get_flag('C') as u16);
        self.set_flag_with_bool('C', result > 0xFF);
        self.set_flag_with_bool('Z', result == 0);
        self.set_flag_with_bool(
          'V',
          ((self.acc as u16) ^ result & !((self.acc as u16) ^ inverted_data)) & 0x0080 > 0,
        );
        self.set_flag_with_bool('N', result & 0x80 > 0);
        self.acc = (result & 0xFF) as u8;
      }
      Operation::SEC => self.set_flag('C', 1),
      Operation::SED => self.set_flag('D', 1),
      Operation::SEI => self.set_flag('I', 1),
      Operation::STA => self.write_addr(ptr, self.acc),
      Operation::STX => self.write_addr(ptr, self.x),
      Operation::STY => self.write_addr(ptr, self.y),
      Operation::TAX => self.x = self.acc,
      Operation::TAY => self.y = self.acc,
      Operation::TSX => self.x = self.stkp,
      Operation::TXA => self.acc = self.x,
      Operation::TXS => self.status = self.x,
      Operation::TYA => self.acc = self.y,
      Operation::XXX => (),
    }
  }

  fn stack_push(&mut self, data: u8) {
    self.write_addr(STACK_BASE_ADDR + (self.stkp as u16), data);
    self.stkp -= 1;
  }

  fn stack_pop(&mut self) -> u8 {
    self.stkp += 1;
    self.read_addr(STACK_BASE_ADDR + (self.stkp as u16))
  }

  fn branch(&mut self, data: u16) {
    self.cycles += 1;
    let destination = self.pc + data;
    if destination & 0xFF00 != self.pc & 0xFF00 {
      self.cycles += 1
    }
    self.pc = destination;
  }

  fn get_data_ptr(&mut self, addressing_mode: &AddressMode) -> u16 {
    match addressing_mode {
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
          self.cycles += 1; // TODO: Is this the right way to do this? Or is it dependant on how the operation is executed
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
      AddressMode::Immediate => self.pc + 1,
      AddressMode::Implied => self.acc as u16, // TODO: is this right?
      AddressMode::IndirectX => {
        let offset = self.read_pc_addr();
        let zero_ptr = (self.x + offset) as u16;

        self.read_addr_from(zero_ptr)
      }
      // TODO: This is what I interpreted the spec to be
      // AddressMode::IndirectY => {
      //   let zero_ptr = self.read_pc_addr() as u16;
      //   let contents_from_zero_page = self.read_addr(zero_ptr) as u16;

      //   let sum_with_carry = (self.y as u16) + contents_from_zero_page; // Cast to u16 to preserve carry
      //   let carry = (sum_with_carry & HI_BYTE_MASK) >> 8;

      //   let lo = sum_with_carry & LO_BYTE_MASK;
      //   let hi = (self.read_addr(zero_ptr + 1) as u16) + carry;

      //   (hi << 8) | lo
      // }
      // TODO: This is what javidx9 interpreted the spec to be
      AddressMode::IndirectY => {
        let ptr = self.get_byte_at_pc();
        let addr = ptr + (self.y as u16);
        if (addr & HI_BYTE_MASK) != (ptr & HI_BYTE_MASK) {
          self.cycles += 1
        }

        addr
      }
      AddressMode::Relative => {
        let offset = self.read_pc_addr() as u16;

        if offset & 0x80 > 0 {
          offset | 0xFF00
        } else {
          offset
        }
      }
      AddressMode::ZeroPage => self.read_pc_addr() as u16,
      AddressMode::ZeroPageX => (self.read_pc_addr() + self.x) as u16,
      AddressMode::ZeroPageY => (self.read_pc_addr() + self.y) as u16,
      AddressMode::XXX => 0x0000,
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

  fn set_flag_with_bool(&mut self, flag: char, value: bool) {
    let flag_val = flag_from_char(flag);
    match value {
      false => self.set_flag_false(flag_val),
      true => self.set_flag_true(flag_val),
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
