#[cfg(test)]
use cpu::instruction::{AddressMode, Operation};

#[cfg(test)]
use data_flow::ReadWrite;

#[cfg(test)]
use cpu::Cpu;

#[cfg(test)]
const DEF_ADDR_MODE: &AddressMode = &AddressMode::XXX;

#[cfg(test)]
const ROOT: u16 = 0x0000;

#[cfg(test)]
mod lda {
  use super::*;
  const OP: &Operation = &Operation::LDA;
  #[test]
  fn loads_memory_value_to_accumulator() {
    let mut cpu = Cpu::new();
    cpu.write_addr(ROOT, 0x0D);
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.acc, 0x0D);
  }
  #[test]
  fn flag_z() {
    let mut cpu = Cpu::new();
    cpu.write_addr(ROOT, 0x00);
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.status, 0b0000_0010);
  }
  #[test]
  fn flag_n() {
    let mut cpu = Cpu::new();
    cpu.write_addr(ROOT, 0x80);
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.status, 0b1000_0000);
  }
}

#[cfg(test)]
mod ldx {
  use super::*;
  const OP: &Operation = &Operation::LDX;
  #[test]
  fn loads_memory_value_to_register() {
    let mut cpu = Cpu::new();
    cpu.write_addr(ROOT, 0x0D);
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.x, 0x0D);
  }
  #[test]
  fn flag_z() {
    let mut cpu = Cpu::new();
    cpu.write_addr(ROOT, 0x00);
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.status, 0b0000_0010);
  }
  #[test]
  fn flag_n() {
    let mut cpu = Cpu::new();
    cpu.write_addr(ROOT, 0x80);
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.status, 0b1000_0000);
  }
}

#[cfg(test)]
mod ldy {
  use super::*;
  const OP: &Operation = &Operation::LDY;
  #[test]
  fn loads_memory_value_to_register() {
    let mut cpu = Cpu::new();
    cpu.write_addr(ROOT, 0x0D);
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.y, 0x0D);
  }
  #[test]
  fn flag_z() {
    let mut cpu = Cpu::new();
    cpu.write_addr(ROOT, 0x00);
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.status, 0b0000_0010);
  }
  #[test]
  fn flag_n() {
    let mut cpu = Cpu::new();
    cpu.write_addr(ROOT, 0x80);
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.status, 0b1000_0000);
  }
}
