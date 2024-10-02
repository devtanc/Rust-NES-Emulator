#[cfg(test)]
use cpu::instruction::{AddressMode, Operation};

#[cfg(test)]
use cpu::Cpu;

#[cfg(test)]
const DEF_ADDR_MODE: &AddressMode = &AddressMode::XXX;

#[cfg(test)]
mod tax {
  use super::*;
  const OP: &Operation = &Operation::TAX;
  #[test]
  fn stores_accumulator_into_x_register() {
    let mut cpu = Cpu::new();
    cpu.acc = 0xFA;
    cpu.x = 0x00;
    cpu.perform_operation(0x0000, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.x, 0xFA);
  }
}

#[cfg(test)]
mod tay {
  use super::*;
  const OP: &Operation = &Operation::TAY;
  #[test]
  fn stores_accumulator_into_y_register() {
    let mut cpu = Cpu::new();
    cpu.acc = 0xFA;
    cpu.y = 0x00;
    cpu.perform_operation(0x0000, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.y, 0xFA);
  }
}

#[cfg(test)]
mod tsx {
  use super::*;
  const OP: &Operation = &Operation::TSX;
  #[test]
  fn stores_stkp_into_x_register() {
    let mut cpu = Cpu::new();
    cpu.stkp = 0xFA;
    cpu.x = 0x00;
    cpu.perform_operation(0x0000, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.x, 0xFA);
  }
}

#[cfg(test)]
mod txa {
  use super::*;
  const OP: &Operation = &Operation::TXA;
  #[test]
  fn stores_x_register_into_accumulator() {
    let mut cpu = Cpu::new();
    cpu.x = 0xFA;
    cpu.acc = 0x00;
    cpu.perform_operation(0x0000, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.acc, 0xFA);
  }
}

#[cfg(test)]
mod txs {
  use super::*;
  const OP: &Operation = &Operation::TXS;
  #[test]
  fn stores_x_register_into_stkp() {
    let mut cpu = Cpu::new();
    cpu.x = 0xFA;
    cpu.stkp = 0x00;
    cpu.perform_operation(0x0000, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.stkp, 0xFA);
  }
}

#[cfg(test)]
mod tya {
  use super::*;
  const OP: &Operation = &Operation::TYA;
  #[test]
  fn stores_y_register_into_accumulator() {
    let mut cpu = Cpu::new();
    cpu.y = 0xFA;
    cpu.acc = 0x00;
    cpu.perform_operation(0x0000, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.acc, 0xFA);
  }
}
