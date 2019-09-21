pub mod instruction;
mod tests;

use bus::Bus;
use cpu::instruction::{get_instruction, AddressMode, Operation};
use data_flow::ReadWrite;

const STACK_BASE_ADDR: u16 = 0x0100;
const RESET_ADDRESS: u16 = 0xFFFC;
const LO_BYTE_MASK: u16 = 0x00FF;
const HI_BYTE_MASK: u16 = 0xFF00;
const BRK_ADDR_BEGIN: u16 = 0xFFFE;
const NMI_ADDR_BEGIN: u16 = 0xFFFA;

fn flag_from_char(flag: char) -> u8 {
  match flag {
    'C' => 1, // C	Carry
    'Z' => 2, // Z	Zero
    'I' => 3, // I	Interrupt (IRQ disable)
    'D' => 4, // D	Decimal (use BCD for arithmetics)
    'B' => 5, // B	B-Flag
    'U' => 6, // -	ignored
    'V' => 7, // V	Overflow
    'N' => 8, // N	Negative
    _ => 0,
  }
}

pub struct Cpu {
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
  addr_of_data: u16,   // Any absolute address value
  ppc: u16,            // Beginning of instruction pc value
}

impl Cpu {
  pub fn new() -> Cpu {
    Cpu {
      bus: Bus::new(),

      acc: 0x00,
      x: 0x00,
      y: 0x00,
      stkp: 0xFD,
      pc: 0xFFFC,
      status: 0x00,

      current_tick: 0,
      cycles: 0x00,
      opcode: 0x00,
      addr_of_data: 0x0000,
      ppc: 0xFFFC,
    }
  }

  pub fn get_mut_bus_ref(&mut self) -> &mut Bus {
    &mut self.bus
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

  pub fn get_current_tick(&self) -> &usize {
    &self.current_tick
  }

  pub fn get_cycles(&self) -> &u8 {
    &self.cycles
  }

  pub fn get_opcode(&self) -> &u8 {
    &self.opcode
  }

  pub fn get_addr_of_data(&self) -> &u16 {
    &self.addr_of_data
  }

  pub fn get_ppc(&self) -> &u16 {
    &self.ppc
  }

  // Interrupt: reset
  // Reset all internal values to defaults
  pub fn reset(&mut self) {
    let lo = self.read_addr(RESET_ADDRESS + 0) as u16;
    let hi = self.read_addr(RESET_ADDRESS + 1) as u16;

    self.pc = (hi << 8) | lo;

    self.pc = 0xC000;

    self.acc = 0;
    self.x = 0;
    self.y = 0;
    self.stkp = 0xFD;
    self.status = match self.get_flag('U') {
      true => 0b00000100,
      false => 0b00000000,
    };

    self.addr_of_data = 0x0000;

    self.cycles = 7;
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
    self.pc = self.read_addr_from(NMI_ADDR_BEGIN);

    self.cycles = 8;
  }
  // Perform one clock cycle
  pub fn clock(&mut self) {
    if self.is_cycle_complete() {
      self.ppc = self.pc;
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
      self.addr_of_data = data_ptr;

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

  pub fn step(&mut self) {
    if self.is_cycle_complete() {
      self.clock()
    }
    while !self.is_cycle_complete() {
      self.clock()
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
          (((self.acc as u16) ^ result) & !((self.acc as u16) ^ (data as u16))) & 0x0080 > 0,
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
          self.branch(ptr as i16);
        }
      }
      Operation::BCS => {
        if self.get_flag('C') {
          self.branch(ptr as i16);
        }
      }
      Operation::BEQ => {
        if self.get_flag('Z') {
          self.branch(ptr as i16);
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
          self.branch(ptr as i16);
        }
      }
      Operation::BNE => {
        if !self.get_flag('Z') {
          self.branch(ptr as i16);
        }
      }
      Operation::BPL => {
        if !self.get_flag('N') {
          self.branch(ptr as i16);
        }
      }
      Operation::BRK => {
        // Set flags
        self.set_flag('I', 1);
        self.set_flag('B', 1);
        // Increment pc (so RTI executes correctly)
        self.pc = self.pc.wrapping_add(1);
        // Store incremented PC address bytes on stack
        let pc_hi = (self.pc & 0xFF00) >> 8;
        let pc_lo = self.pc & 0x00FF;
        self.stack_push(pc_hi as u8);
        self.stack_push(pc_lo as u8);
        // Store status on stack
        self.stack_push(self.status);
        // Reset B flag
        self.set_flag('B', 0);
        // Read destination into pc
        self.pc = self.read_addr_from(BRK_ADDR_BEGIN);
      }
      Operation::BVC => {
        if !self.get_flag('V') {
          self.branch(ptr as i16);
        }
      }
      Operation::BVS => {
        if self.get_flag('V') {
          self.branch(ptr as i16);
        }
      }
      Operation::CLC => self.set_flag('C', 0),
      Operation::CLD => self.set_flag('D', 0),
      Operation::CLI => self.set_flag('I', 0),
      Operation::CLV => self.set_flag('V', 0),
      Operation::CMP => {
        let result = self.acc.wrapping_sub(data);
        self.set_flag_with_bool('Z', result == 0);
        self.set_flag_with_bool('C', self.acc >= data);
        self.set_flag_with_bool('N', result & 0x80 > 0);
      }
      Operation::CPX => {
        let result = (self.x as u16).wrapping_sub(data as u16);
        self.set_flag_with_bool('Z', result == 0);
        self.set_flag_with_bool('C', self.x >= data);
        self.set_flag_with_bool('N', result & 0x80 > 0);
      }
      Operation::CPY => {
        let result = (self.y as u16).wrapping_sub(data as u16);
        self.set_flag_with_bool('Z', result == 0);
        self.set_flag_with_bool('C', self.y >= data);
        self.set_flag_with_bool('N', result & 0x80 > 0);
      }
      Operation::DEC => {
        let result = data.wrapping_sub(1);
        self.set_flag_with_bool('Z', result == 0);
        self.set_flag_with_bool('N', (result & 0x80) > 0);
        self.write_addr(ptr, result & 0x00FF);
      }
      Operation::DEX => {
        let result = self.x.wrapping_sub(1);
        self.set_flag_with_bool('Z', result == 0);
        self.set_flag_with_bool('N', (result & 0x80) > 0);
        self.x = result & 0x00FF;
      }
      Operation::DEY => {
        let result = self.y.wrapping_sub(1);
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
        let result = data.wrapping_add(1);
        self.set_flag_with_bool('Z', result == 0);
        self.set_flag_with_bool('N', (result & 0x80) > 0);
        self.write_addr(ptr, result & 0x00FF);
      }
      Operation::INX => {
        let result = self.x.wrapping_add(1);
        self.set_flag_with_bool('Z', result == 0);
        self.set_flag_with_bool('N', (result & 0x80) > 0);
        self.x = result & 0x00FF;
      }
      Operation::INY => {
        let result = self.y.wrapping_add(1);
        self.set_flag_with_bool('Z', result == 0);
        self.set_flag_with_bool('N', (result & 0x80) > 0);
        self.y = result & 0x00FF;
      }
      Operation::JMP => self.pc = ptr,
      Operation::JSR => {
        self.pc -= 1;
        self.push_pc_to_stack();
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
      Operation::LSR => match address_mode {
        &AddressMode::Accumulator => {
          let result = self.acc >> 1;
          self.set_flag_with_bool('C', (self.acc & 0x01) > 0);
          self.set_flag('N', 0);
          self.set_flag_with_bool('Z', result == 0);
          self.acc = result;
        }
        _ => {
          let result = data >> 1;
          self.set_flag_with_bool('C', (data & 0x01) > 0);
          self.set_flag('N', 0);
          self.set_flag_with_bool('Z', result == 0);
          self.write_addr(ptr, result);
        }
      },
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
      Operation::PHA => self.stack_push(self.acc),
      Operation::PHP => self.stack_push(self.status),
      Operation::PLA => self.acc = self.stack_pop(),
      Operation::PLP => self.status = self.stack_pop(),
      Operation::ROL => match address_mode {
        &AddressMode::Accumulator => {
          let result = (self.acc as u16) << 1 | self.get_flag('C') as u16;
          self.set_flag_with_bool('C', result & 0xFF00 > 0);
          self.set_flag_with_bool('Z', result == 0);
          self.set_flag_with_bool('N', result & 0x0080 > 0);
          self.acc = (result & 0x00FF) as u8
        }
        _ => {
          let result = (data as u16) << 1 | self.get_flag('C') as u16;
          self.set_flag_with_bool('C', result & 0xFF00 > 0);
          self.set_flag_with_bool('Z', result == 0);
          self.set_flag_with_bool('N', result & 0x0080 > 0);
          self.write_addr(ptr, (result & 0x00FF) as u8)
        }
      },
      Operation::ROR => match address_mode {
        &AddressMode::Accumulator => {
          let result = ((self.get_flag('C') as u8) << 7) | (self.acc >> 1);
          self.set_flag_with_bool('C', data & 0x01 > 0);
          self.set_flag_with_bool('Z', result == 0);
          self.set_flag_with_bool('N', result & 0x80 > 0);
          self.acc = result;
        }
        _ => {
          let result = ((self.get_flag('C') as u8) << 7) | (data >> 1);
          self.set_flag_with_bool('C', data & 0x01 > 0);
          self.set_flag_with_bool('Z', result == 0);
          self.set_flag_with_bool('N', result & 0x80 > 0);
          self.write_addr(ptr, result)
        }
      },
      Operation::RTI => {
        self.status = self.stack_pop();
        self.set_flag('B', 0);
        self.set_flag('U', 0);
        self.set_flag('I', 0);
        self.pc = self.pop_address_from_stack();
      }
      Operation::RTS => {
        self.pc = self.pop_address_from_stack();
        self.pc = self.pc.wrapping_add(1);
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
      Operation::TXS => self.stkp = self.x,
      Operation::TYA => self.acc = self.y,
      Operation::XXX => (),
    }
    match address_mode {
      AddressMode::Immediate => {
        self.pc = self.pc.wrapping_add(1);
      }
      _ => (),
    };
  }

  fn stack_push(&mut self, data: u8) {
    self.write_addr(STACK_BASE_ADDR + (self.stkp as u16), data);
    self.stkp -= 1;
  }

  fn stack_pop(&mut self) -> u8 {
    self.stkp += 1;
    self.read_addr(STACK_BASE_ADDR + (self.stkp as u16))
  }

  fn push_pc_to_stack(&mut self) {
    self.stack_push(((self.pc >> 8) & 0x00FF) as u8);
    self.stack_push((self.pc & 0x00FF) as u8);
  }
  fn pop_address_from_stack(&mut self) -> u16 {
    let lo = self.stack_pop() as u16;
    let hi = self.stack_pop() as u16;
    (hi << 8) | lo
  }

  fn branch(&mut self, offset: i16) {
    self.cycles += 1;
    let destination = self.pc.wrapping_add(offset as u16);
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

        // SIMULATE PAGE BOUNDARY HARDWARE BUG
        if (ptr & 0x00FF) == 0xFF {
          let lo_pc_addr = self.read_addr(ptr) as u16;
          let hi_pc_addr = self.read_addr(ptr & 0xFF00) as u16;
          (hi_pc_addr << 8) | lo_pc_addr
        } else {
          let lo_pc_addr = self.read_addr(ptr) as u16;
          let hi_pc_addr = self.read_addr(ptr + 1) as u16;
          (hi_pc_addr << 8) | lo_pc_addr
        }
      }
      AddressMode::AbsoluteX => {
        let addr = self.get_byte_at_pc();
        let offset_addr = addr.wrapping_add(self.x as u16);

        if (offset_addr & HI_BYTE_MASK) != (addr & HI_BYTE_MASK) {
          // TODO: Need tests for cycle increment
          self.cycles += 1; // TODO: Is this the right way to do this? Or is it dependant on how the operation is executed
        }
        offset_addr
      }
      AddressMode::AbsoluteY => {
        let addr = self.get_byte_at_pc();
        let offset_addr = addr.wrapping_add(self.y as u16);

        if (offset_addr & HI_BYTE_MASK) != (addr & HI_BYTE_MASK) {
          self.cycles += 1;
        }
        offset_addr
      }
      // If the address mode is accumulator, it doesn't need an address
      AddressMode::Accumulator => 0x0000,
      AddressMode::Immediate => self.pc,
      // If the address mode is implied, it doesn't need an address
      AddressMode::Implied => 0x0000,
      AddressMode::IndirectX => {
        let offset = self.read_pc_addr();
        let zero_ptr = self.x.wrapping_add(offset);

        let lo = self.read_addr(zero_ptr as u16) as u16;
        let hi = self.read_addr(zero_ptr.wrapping_add(1) as u16) as u16;
        (hi << 8) | lo
      }
      AddressMode::IndirectY => {
        let zero_ptr = self.read_pc_addr();
        let lo = self.read_addr(zero_ptr as u16) as u16;
        let hi = self.read_addr(zero_ptr.wrapping_add(1) as u16) as u16;
        let addr = (hi << 8) | lo;
        addr.wrapping_add(self.y as u16)
        // TODO: How to figure out clock cycles for crossing pages here
        // TODO: Need tests for cycle increment once written
      }
      AddressMode::Relative => {
        let offset = self.read_pc_addr() as u16;

        if offset & 0x80 > 0 {
          // If offset is negative, make it start 0xFF for binary addition to work
          offset | 0xFF00
        } else {
          // If offset is positive
          offset
        }
      }
      AddressMode::ZeroPage => self.read_pc_addr() as u16,
      AddressMode::ZeroPageX => self.x.wrapping_add(self.read_pc_addr()) as u16,
      AddressMode::ZeroPageY => self.y.wrapping_add(self.read_pc_addr()) as u16,
      AddressMode::XXX => 0x0000,
    }
  }
  // Returns if current cycle is complete
  pub fn is_cycle_complete(&self) -> bool {
    self.cycles == 0
  }

  pub fn get_flag(&self, flag: char) -> bool {
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
    self.pc = self.pc.wrapping_add(1);
    result
  }

  fn get_byte_at_pc(&mut self) -> u16 {
    let lo = self.read_pc_addr() as u16;
    let hi = self.read_pc_addr() as u16;
    (hi << 8) | lo
  }

  fn read_addr_from(&self, addr: u16) -> u16 {
    let lo = self.read_addr(addr) as u16;
    let hi = self.read_addr(addr.wrapping_add(1)) as u16;
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
