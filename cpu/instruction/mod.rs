// Source: https://www.masswerk.at/6502/6502_instruction_set.html

use std::fmt;

pub enum Operation {
  ADC, //  add with carry
  AND, //  and (with accumulator)
  ASL, //  arithmetic shift left
  BCC, //  branch on carry clear
  BCS, //  branch on carry set
  BEQ, //  branch on equal (zero set)
  BIT, //  bit test
  BMI, //  branch on minus (negative set)
  BNE, //  branch on not equal (zero clear)
  BPL, //  branch on plus (negative clear)
  BRK, //  break / interrupt
  BVC, //  branch on overflow clear
  BVS, //  branch on overflow set
  CLC, //  clear carry
  CLD, //  clear decimal
  CLI, //  clear interrupt disable
  CLV, //  clear overflow
  CMP, //  compare (with accumulator)
  CPX, //  compare with X
  CPY, //  compare with Y
  DEC, //  decrement
  DEX, //  decrement X
  DEY, //  decrement Y
  EOR, //  exclusive or (with accumulator)
  INC, //  increment
  INX, //  increment X
  INY, //  increment Y
  JMP, //  jump
  JSR, //  jump subroutine
  LDA, //  load accumulator
  LDX, //  load X
  LDY, //  load Y
  LSR, //  logical shift right
  NOP, //  no operation
  ORA, //  or with accumulator
  PHA, //  push accumulator
  PHP, //  push processor status (SR)
  PLA, //  pull accumulator
  PLP, //  pull processor status (SR)
  ROL, //  rotate left
  ROR, //  rotate right
  RTI, //  return from interrupt
  RTS, //  return from subroutine
  SBC, //  subtract with carry
  SEC, //  set carry
  SED, //  set decimal
  SEI, //  set interrupt disable
  STA, //  store accumulator
  STX, //  store X
  STY, //  store Y
  TAX, //  transfer accumulator to X
  TAY, //  transfer accumulator to Y
  TSX, //  transfer stack pointer to X
  TXA, //  transfer X to accumulator
  TXS, //  transfer X to stack pointer
  TYA, //  transfer Y to accumulator
  XXX, //  invalid operation
}

impl fmt::Display for Operation {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Operation::ADC => write!(f, "ADC"),
      Operation::AND => write!(f, "AND"),
      Operation::ASL => write!(f, "ASL"),
      Operation::BCC => write!(f, "BCC"),
      Operation::BCS => write!(f, "BCS"),
      Operation::BEQ => write!(f, "BEQ"),
      Operation::BIT => write!(f, "BIT"),
      Operation::BMI => write!(f, "BMI"),
      Operation::BNE => write!(f, "BNE"),
      Operation::BPL => write!(f, "BPL"),
      Operation::BRK => write!(f, "BRK"),
      Operation::BVC => write!(f, "BVC"),
      Operation::BVS => write!(f, "BVS"),
      Operation::CLC => write!(f, "CLC"),
      Operation::CLD => write!(f, "CLD"),
      Operation::CLI => write!(f, "CLI"),
      Operation::CLV => write!(f, "CLV"),
      Operation::CMP => write!(f, "CMP"),
      Operation::CPX => write!(f, "CPX"),
      Operation::CPY => write!(f, "CPY"),
      Operation::DEC => write!(f, "DEC"),
      Operation::DEX => write!(f, "DEX"),
      Operation::DEY => write!(f, "DEY"),
      Operation::EOR => write!(f, "EOR"),
      Operation::INC => write!(f, "INC"),
      Operation::INX => write!(f, "INX"),
      Operation::INY => write!(f, "INY"),
      Operation::JMP => write!(f, "JMP"),
      Operation::JSR => write!(f, "JSR"),
      Operation::LDA => write!(f, "LDA"),
      Operation::LDX => write!(f, "LDX"),
      Operation::LDY => write!(f, "LDY"),
      Operation::LSR => write!(f, "LSR"),
      Operation::NOP => write!(f, "NOP"),
      Operation::ORA => write!(f, "ORA"),
      Operation::PHA => write!(f, "PHA"),
      Operation::PHP => write!(f, "PHP"),
      Operation::PLA => write!(f, "PLA"),
      Operation::PLP => write!(f, "PLP"),
      Operation::ROL => write!(f, "ROL"),
      Operation::ROR => write!(f, "ROR"),
      Operation::RTI => write!(f, "RTI"),
      Operation::RTS => write!(f, "RTS"),
      Operation::SBC => write!(f, "SBC"),
      Operation::SEC => write!(f, "SEC"),
      Operation::SED => write!(f, "SED"),
      Operation::SEI => write!(f, "SEI"),
      Operation::STA => write!(f, "STA"),
      Operation::STX => write!(f, "STX"),
      Operation::STY => write!(f, "STY"),
      Operation::TAX => write!(f, "TAX"),
      Operation::TAY => write!(f, "TAY"),
      Operation::TSX => write!(f, "TSX"),
      Operation::TXA => write!(f, "TXA"),
      Operation::TXS => write!(f, "TXS"),
      Operation::TYA => write!(f, "TYA"),
      Operation::XXX => write!(f, "XXX"),
    }
  }
}

pub enum AddressMode {
  Accumulator,      // OPC A	 	    operand is AC (implied single byte instruction)
  Immediate,        // OPC $LLHH	 	operand is address $HHLL *
  Absolute, // OPC $LLHH,X	operand is address; effective address is address incremented by X with carry **
  ZeroPage, // OPC $LLHH,Y	operand is address; effective address is address incremented by Y with carry **
  ZeroPageX, // OPC #$BB	 	  operand is byte BB
  ZeroPageY, // OPC	 	      operand implied
  AbsoluteX, // OPC ($LLHH)	operand is address; effective address is contents of word at address: C.w($HHLL)
  AbsoluteY, // OPC ($LL,X)	operand is zeropage address; effective address is word in (LL + X, LL + X + 1), inc. without carry: C.w($00LL + X)
  Implied, // OPC ($LL),Y	operand is zeropage address; effective address is word in (LL, LL + 1) incremented by Y with carry: C.w($00LL) + Y
  Relative, // OPC $BB	 	  branch target is PC + signed offset BB ***
  IndirectX, // OPC $LL	 	  operand is zeropage address (hi-byte is zero, address = $00LL)
  IndirectY, // OPC $LL,X	 	operand is zeropage address; effective address is address incremented by X without carry **
  AbsoluteIndirect, // OPC $LL,Y	 	operand is zeropage address; effective address is address incremented by Y without carry **
  XXX,              // Address mode for invalid operation
}

impl fmt::Display for AddressMode {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      AddressMode::Accumulator => write!(f, "ACC"),
      AddressMode::Immediate => write!(f, "IMM"),
      AddressMode::Absolute => write!(f, "ABS"),
      AddressMode::ZeroPage => write!(f, "ZP0"),
      AddressMode::ZeroPageX => write!(f, "ZPX"),
      AddressMode::ZeroPageY => write!(f, "ZPY"),
      AddressMode::AbsoluteX => write!(f, "ABX"),
      AddressMode::AbsoluteY => write!(f, "ABY"),
      AddressMode::Implied => write!(f, "IMP"),
      AddressMode::Relative => write!(f, "REL"),
      AddressMode::IndirectX => write!(f, "INX"),
      AddressMode::IndirectY => write!(f, "INY"),
      AddressMode::AbsoluteIndirect => write!(f, "ABI"),
      AddressMode::XXX => write!(f, "XXX"),
    }
  }
}

pub struct Instruction {
  operation: Operation,
  address_mode: AddressMode,
  instruction_bytes: u8,
  cycles: u8,
}

impl Instruction {
  pub fn new(
    operation: Operation,
    address_mode: AddressMode,
    instruction_bytes: u8,
    cycles: u8,
  ) -> Instruction {
    Instruction {
      operation,
      address_mode,
      instruction_bytes,
      cycles,
    }
  }

  pub fn invalid() -> Instruction {
    Instruction {
      operation: Operation::XXX,
      address_mode: AddressMode::XXX,
      instruction_bytes: 0,
      cycles: 0,
    }
  }

  pub fn get_operation(&self) -> &Operation {
    &self.operation
  }

  pub fn get_address_mode(&self) -> &AddressMode {
    &self.address_mode
  }

  pub fn get_instruction_bytes(&self) -> u8 {
    self.instruction_bytes
  }

  pub fn get_cycles(&self) -> u8 {
    self.cycles
  }
}

pub fn get_instruction(opcode: u8) -> Instruction {
  // From http://archive.6502.org/datasheets/rockwell_r650x_r651x.pdf
  // *1 - Add 1 to cycles if page boundary is crossed
  // *2 - Add 1 to cycles if branch occurs to same page; add 2 if branch occurs to different page
  match opcode {
    0x00 => Instruction::new(Operation::BRK, AddressMode::Implied, 1, 7),
    0x01 => Instruction::new(Operation::ORA, AddressMode::IndirectX, 2, 6),
    0x05 => Instruction::new(Operation::ORA, AddressMode::ZeroPage, 2, 3),
    0x06 => Instruction::new(Operation::ASL, AddressMode::ZeroPage, 2, 5),
    0x08 => Instruction::new(Operation::PHP, AddressMode::Implied, 1, 3),
    0x09 => Instruction::new(Operation::ORA, AddressMode::Immediate, 2, 2),
    0x0A => Instruction::new(Operation::ASL, AddressMode::Accumulator, 1, 2),
    0x0D => Instruction::new(Operation::ORA, AddressMode::Absolute, 3, 4),
    0x0E => Instruction::new(Operation::ASL, AddressMode::Absolute, 3, 6),
    0x10 => Instruction::new(Operation::BPL, AddressMode::Relative, 2, 2), // *2
    0x11 => Instruction::new(Operation::ORA, AddressMode::IndirectY, 2, 5), // *1
    0x15 => Instruction::new(Operation::ORA, AddressMode::ZeroPageX, 2, 4),
    0x16 => Instruction::new(Operation::ASL, AddressMode::ZeroPageX, 2, 6),
    0x18 => Instruction::new(Operation::CLC, AddressMode::Implied, 1, 2),
    0x19 => Instruction::new(Operation::ORA, AddressMode::AbsoluteY, 3, 4), // *1
    0x1D => Instruction::new(Operation::ORA, AddressMode::AbsoluteX, 3, 4), // *1
    0x1E => Instruction::new(Operation::ASL, AddressMode::AbsoluteX, 3, 7),
    0x20 => Instruction::new(Operation::JSR, AddressMode::Absolute, 3, 6),
    0x21 => Instruction::new(Operation::AND, AddressMode::IndirectX, 2, 6),
    0x24 => Instruction::new(Operation::BIT, AddressMode::ZeroPage, 2, 3),
    0x25 => Instruction::new(Operation::AND, AddressMode::ZeroPage, 2, 3),
    0x26 => Instruction::new(Operation::ROL, AddressMode::ZeroPage, 2, 5),
    0x28 => Instruction::new(Operation::PLP, AddressMode::Implied, 1, 4),
    0x29 => Instruction::new(Operation::AND, AddressMode::Immediate, 2, 2),
    0x2A => Instruction::new(Operation::ROL, AddressMode::Accumulator, 1, 2),
    0x2C => Instruction::new(Operation::BIT, AddressMode::Absolute, 3, 4),
    0x2D => Instruction::new(Operation::AND, AddressMode::Absolute, 3, 4),
    0x2E => Instruction::new(Operation::ROL, AddressMode::Absolute, 3, 6),
    0x30 => Instruction::new(Operation::BMI, AddressMode::Relative, 2, 2), // *2
    0x31 => Instruction::new(Operation::AND, AddressMode::IndirectY, 2, 5), // *1
    0x35 => Instruction::new(Operation::AND, AddressMode::ZeroPageX, 2, 4),
    0x36 => Instruction::new(Operation::ROL, AddressMode::ZeroPageX, 2, 6),
    0x38 => Instruction::new(Operation::SEC, AddressMode::Implied, 1, 2),
    0x39 => Instruction::new(Operation::AND, AddressMode::AbsoluteY, 3, 4), // *1
    0x3D => Instruction::new(Operation::AND, AddressMode::AbsoluteX, 3, 4), // *1
    0x3E => Instruction::new(Operation::ROL, AddressMode::AbsoluteX, 3, 7),
    0x40 => Instruction::new(Operation::RTI, AddressMode::Implied, 1, 6),
    0x41 => Instruction::new(Operation::EOR, AddressMode::IndirectX, 2, 6),
    0x45 => Instruction::new(Operation::EOR, AddressMode::ZeroPage, 2, 3),
    0x46 => Instruction::new(Operation::LSR, AddressMode::ZeroPage, 2, 5),
    0x48 => Instruction::new(Operation::PHA, AddressMode::Implied, 1, 3),
    0x49 => Instruction::new(Operation::EOR, AddressMode::Immediate, 2, 2),
    0x4A => Instruction::new(Operation::LSR, AddressMode::Accumulator, 1, 2),
    0x4C => Instruction::new(Operation::JMP, AddressMode::Absolute, 3, 3),
    0x4D => Instruction::new(Operation::EOR, AddressMode::Absolute, 3, 4),
    0x4E => Instruction::new(Operation::LSR, AddressMode::Absolute, 3, 6),
    0x50 => Instruction::new(Operation::BVC, AddressMode::Relative, 2, 2), // *2
    0x51 => Instruction::new(Operation::EOR, AddressMode::IndirectY, 2, 5), // *1
    0x55 => Instruction::new(Operation::EOR, AddressMode::ZeroPageX, 2, 4),
    0x56 => Instruction::new(Operation::LSR, AddressMode::ZeroPageX, 2, 6),
    0x58 => Instruction::new(Operation::CLI, AddressMode::Implied, 1, 2),
    0x59 => Instruction::new(Operation::EOR, AddressMode::AbsoluteY, 3, 4), // *1
    0x5D => Instruction::new(Operation::EOR, AddressMode::AbsoluteX, 3, 4), // *1
    0x5E => Instruction::new(Operation::LSR, AddressMode::AbsoluteX, 3, 7),
    0x60 => Instruction::new(Operation::RTS, AddressMode::Implied, 1, 6),
    0x61 => Instruction::new(Operation::ADC, AddressMode::IndirectX, 2, 6),
    0x65 => Instruction::new(Operation::ADC, AddressMode::ZeroPage, 2, 3),
    0x66 => Instruction::new(Operation::ROR, AddressMode::ZeroPage, 2, 5),
    0x68 => Instruction::new(Operation::PLA, AddressMode::Implied, 1, 4),
    0x69 => Instruction::new(Operation::ADC, AddressMode::Immediate, 2, 2),
    0x6A => Instruction::new(Operation::ROR, AddressMode::Accumulator, 1, 2),
    0x6C => Instruction::new(Operation::JMP, AddressMode::AbsoluteIndirect, 3, 5),
    0x6D => Instruction::new(Operation::ADC, AddressMode::Absolute, 3, 4),
    0x6E => Instruction::new(Operation::ROR, AddressMode::Absolute, 3, 6),
    0x70 => Instruction::new(Operation::BVS, AddressMode::Relative, 2, 2), // *2
    0x71 => Instruction::new(Operation::ADC, AddressMode::IndirectY, 2, 5), // *1
    0x75 => Instruction::new(Operation::ADC, AddressMode::ZeroPageX, 2, 4),
    0x76 => Instruction::new(Operation::ROR, AddressMode::ZeroPageX, 2, 6),
    0x78 => Instruction::new(Operation::SEI, AddressMode::Implied, 1, 2),
    0x79 => Instruction::new(Operation::ADC, AddressMode::AbsoluteY, 3, 4), // *1
    0x7D => Instruction::new(Operation::ADC, AddressMode::AbsoluteX, 3, 4), // *1
    0x7E => Instruction::new(Operation::ROR, AddressMode::AbsoluteX, 3, 7),
    0x81 => Instruction::new(Operation::STA, AddressMode::IndirectX, 2, 6),
    0x84 => Instruction::new(Operation::STY, AddressMode::ZeroPage, 2, 3),
    0x85 => Instruction::new(Operation::STA, AddressMode::ZeroPage, 2, 3),
    0x86 => Instruction::new(Operation::STX, AddressMode::ZeroPage, 2, 3),
    0x88 => Instruction::new(Operation::DEY, AddressMode::Implied, 1, 2),
    0x8A => Instruction::new(Operation::TXA, AddressMode::Implied, 1, 2),
    0x8C => Instruction::new(Operation::STY, AddressMode::Absolute, 3, 4),
    0x8D => Instruction::new(Operation::STA, AddressMode::Absolute, 3, 4),
    0x8E => Instruction::new(Operation::STX, AddressMode::Absolute, 3, 4),
    0x90 => Instruction::new(Operation::BCC, AddressMode::Relative, 2, 2), // *2
    0x91 => Instruction::new(Operation::STA, AddressMode::IndirectY, 2, 6),
    0x94 => Instruction::new(Operation::STY, AddressMode::ZeroPageX, 2, 4),
    0x95 => Instruction::new(Operation::STA, AddressMode::ZeroPageX, 2, 4),
    0x96 => Instruction::new(Operation::STX, AddressMode::ZeroPageY, 2, 4),
    0x98 => Instruction::new(Operation::TYA, AddressMode::Implied, 1, 2),
    0x99 => Instruction::new(Operation::STA, AddressMode::AbsoluteY, 3, 5),
    0x9A => Instruction::new(Operation::TXS, AddressMode::Implied, 1, 2),
    0x9D => Instruction::new(Operation::STA, AddressMode::AbsoluteX, 3, 5),
    0xA0 => Instruction::new(Operation::LDY, AddressMode::Immediate, 2, 2),
    0xA1 => Instruction::new(Operation::LDA, AddressMode::IndirectX, 2, 6),
    0xA2 => Instruction::new(Operation::LDX, AddressMode::Immediate, 2, 2),
    0xA4 => Instruction::new(Operation::LDY, AddressMode::ZeroPage, 2, 3),
    0xA5 => Instruction::new(Operation::LDA, AddressMode::ZeroPage, 2, 3),
    0xA6 => Instruction::new(Operation::LDX, AddressMode::ZeroPage, 2, 3),
    0xA8 => Instruction::new(Operation::TAY, AddressMode::Implied, 1, 2),
    0xA9 => Instruction::new(Operation::LDA, AddressMode::Immediate, 2, 2),
    0xAA => Instruction::new(Operation::TAX, AddressMode::Implied, 1, 2),
    0xAC => Instruction::new(Operation::LDY, AddressMode::Absolute, 3, 4),
    0xAD => Instruction::new(Operation::LDA, AddressMode::Absolute, 3, 4),
    0xAE => Instruction::new(Operation::LDX, AddressMode::Absolute, 3, 4),
    0xB0 => Instruction::new(Operation::BCS, AddressMode::Relative, 2, 2), // *2
    0xB1 => Instruction::new(Operation::LDA, AddressMode::IndirectY, 2, 5), // *1
    0xB4 => Instruction::new(Operation::LDY, AddressMode::ZeroPageX, 2, 4),
    0xB5 => Instruction::new(Operation::LDA, AddressMode::ZeroPageX, 2, 4),
    0xB6 => Instruction::new(Operation::LDX, AddressMode::ZeroPageY, 2, 4),
    0xB8 => Instruction::new(Operation::CLV, AddressMode::Implied, 1, 2),
    0xB9 => Instruction::new(Operation::LDA, AddressMode::AbsoluteY, 3, 4), // *1
    0xBA => Instruction::new(Operation::TSX, AddressMode::Implied, 1, 2),
    0xBC => Instruction::new(Operation::LDY, AddressMode::AbsoluteX, 3, 4), // *1
    0xBD => Instruction::new(Operation::LDA, AddressMode::AbsoluteX, 3, 4), // *1
    0xBE => Instruction::new(Operation::LDX, AddressMode::AbsoluteY, 3, 4), // *1
    0xC0 => Instruction::new(Operation::CPY, AddressMode::Immediate, 2, 2),
    0xC1 => Instruction::new(Operation::CMP, AddressMode::IndirectX, 2, 6),
    0xC4 => Instruction::new(Operation::CPY, AddressMode::ZeroPage, 2, 3),
    0xC5 => Instruction::new(Operation::CMP, AddressMode::ZeroPage, 2, 3),
    0xC6 => Instruction::new(Operation::DEC, AddressMode::ZeroPage, 2, 5),
    0xC8 => Instruction::new(Operation::INY, AddressMode::Implied, 1, 2),
    0xC9 => Instruction::new(Operation::CMP, AddressMode::Immediate, 2, 2),
    0xCA => Instruction::new(Operation::DEX, AddressMode::Implied, 1, 2),
    0xCC => Instruction::new(Operation::CPY, AddressMode::Absolute, 3, 4),
    0xCD => Instruction::new(Operation::CMP, AddressMode::Absolute, 3, 4),
    0xCE => Instruction::new(Operation::DEC, AddressMode::Absolute, 3, 6),
    0xD0 => Instruction::new(Operation::BNE, AddressMode::Relative, 2, 2), // *2
    0xD1 => Instruction::new(Operation::CMP, AddressMode::IndirectY, 2, 5), // *1
    0xD5 => Instruction::new(Operation::CMP, AddressMode::ZeroPageX, 2, 4),
    0xD6 => Instruction::new(Operation::DEC, AddressMode::ZeroPageX, 2, 6),
    0xD8 => Instruction::new(Operation::CLD, AddressMode::Implied, 1, 2),
    0xD9 => Instruction::new(Operation::CMP, AddressMode::AbsoluteY, 3, 4), // *1
    0xDD => Instruction::new(Operation::CMP, AddressMode::AbsoluteX, 3, 4), // *1
    0xDE => Instruction::new(Operation::DEC, AddressMode::AbsoluteX, 3, 7),
    0xE0 => Instruction::new(Operation::CPX, AddressMode::Immediate, 2, 2),
    0xE1 => Instruction::new(Operation::SBC, AddressMode::IndirectX, 2, 6),
    0xE4 => Instruction::new(Operation::CPX, AddressMode::ZeroPage, 2, 3),
    0xE5 => Instruction::new(Operation::SBC, AddressMode::ZeroPage, 2, 3),
    0xE6 => Instruction::new(Operation::INC, AddressMode::ZeroPage, 2, 5),
    0xE8 => Instruction::new(Operation::INX, AddressMode::Implied, 1, 2),
    0xE9 => Instruction::new(Operation::SBC, AddressMode::Immediate, 2, 2),
    0xEA => Instruction::new(Operation::NOP, AddressMode::Implied, 1, 2),
    0xEC => Instruction::new(Operation::CPX, AddressMode::Absolute, 3, 4),
    0xED => Instruction::new(Operation::SBC, AddressMode::Absolute, 3, 4),
    0xEE => Instruction::new(Operation::INC, AddressMode::Absolute, 3, 6),
    0xF0 => Instruction::new(Operation::BEQ, AddressMode::Relative, 2, 2), // *2
    0xF1 => Instruction::new(Operation::SBC, AddressMode::IndirectY, 2, 5), // *1
    0xF5 => Instruction::new(Operation::SBC, AddressMode::ZeroPageX, 2, 4),
    0xF6 => Instruction::new(Operation::INC, AddressMode::ZeroPageX, 2, 6),
    0xF8 => Instruction::new(Operation::SED, AddressMode::Implied, 1, 2),
    0xF9 => Instruction::new(Operation::SBC, AddressMode::AbsoluteY, 3, 4), // *1
    0xFD => Instruction::new(Operation::SBC, AddressMode::AbsoluteX, 3, 4), // *1
    0xFE => Instruction::new(Operation::INC, AddressMode::AbsoluteX, 3, 7),
    _ => Instruction::invalid(),
  }
}
