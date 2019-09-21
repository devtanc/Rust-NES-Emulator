#[cfg(test)]
use cpu::instruction::{AddressMode, Operation};

#[cfg(test)]
use cpu::Cpu;

#[cfg(test)]
const DEF_ADDR_MODE: &AddressMode = &AddressMode::XXX;

#[cfg(test)]
mod clear {
  use super::*;
  #[test]
  fn clc() {
    let mut cpu = Cpu::new();
    cpu.status = 0xFF;
    cpu.perform_operation(0, &Operation::CLC, DEF_ADDR_MODE);
    assert_eq!(cpu.status, 0b1111_1110);
  }
  #[test]
  fn cld() {
    let mut cpu = Cpu::new();
    cpu.status = 0xFF;
    cpu.perform_operation(0, &Operation::CLD, DEF_ADDR_MODE);
    assert_eq!(cpu.status, 0b1111_0111);
  }
  #[test]
  fn cli() {
    let mut cpu = Cpu::new();
    cpu.status = 0xFF;
    cpu.perform_operation(0, &Operation::CLI, DEF_ADDR_MODE);
    assert_eq!(cpu.status, 0b1111_1011);
  }
  #[test]
  fn clv() {
    let mut cpu = Cpu::new();
    cpu.status = 0xFF;
    cpu.perform_operation(0, &Operation::CLV, DEF_ADDR_MODE);
    assert_eq!(cpu.status, 0b1011_1111);
  }
}

#[cfg(test)]
mod set {
  use super::*;
  #[test]
  fn sec() {
    let mut cpu = Cpu::new();
    cpu.status = 0x00;
    cpu.perform_operation(0, &Operation::SEC, DEF_ADDR_MODE);
    assert_eq!(cpu.status, 0b0000_0001);
  }
  #[test]
  fn sed() {
    let mut cpu = Cpu::new();
    cpu.status = 0x00;
    cpu.perform_operation(0, &Operation::SED, DEF_ADDR_MODE);
    assert_eq!(cpu.status, 0b0000_1000);
  }
  #[test]
  fn sei() {
    let mut cpu = Cpu::new();
    cpu.status = 0x00;
    cpu.perform_operation(0, &Operation::SEI, DEF_ADDR_MODE);
    assert_eq!(cpu.status, 0b0000_0100);
  }
}
