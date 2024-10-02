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
mod dec {
  use super::*;
  const OP: &Operation = &Operation::DEC;
  #[test]
  fn decrements_memory() {
    let mut cpu = Cpu::new();
    cpu.write_addr(ROOT, 0xFF);
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.read_addr(ROOT), 0xFE);
  }
  #[test]
  fn flag_n() {
    let mut cpu = Cpu::new();
    cpu.write_addr(ROOT, 0xFF);
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.status, 0b1000_0000);
  }
  #[test]
  fn flag_z() {
    let mut cpu = Cpu::new();
    cpu.write_addr(ROOT, 0x01);
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.status, 0b0000_0010);
  }
}

#[cfg(test)]
mod dex {
  use super::*;
  const OP: &Operation = &Operation::DEX;
  #[test]
  fn decrements_register() {
    let mut cpu = Cpu::new();
    cpu.x = 0xFF;
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.x, 0xFE);
  }
  #[test]
  fn flag_n() {
    let mut cpu = Cpu::new();
    cpu.x = 0xFF;
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.status, 0b1000_0000);
  }
  #[test]
  fn flag_z() {
    let mut cpu = Cpu::new();
    cpu.x = 0x01;
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.status, 0b0000_0010);
  }
}

#[cfg(test)]
mod dey {
  use super::*;
  const OP: &Operation = &Operation::DEY;
  #[test]
  fn decrements_register() {
    let mut cpu = Cpu::new();
    cpu.y = 0xFF;
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.y, 0xFE);
  }
  #[test]
  fn flag_n() {
    let mut cpu = Cpu::new();
    cpu.y = 0xFF;
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.status, 0b1000_0000);
  }
  #[test]
  fn flag_z() {
    let mut cpu = Cpu::new();
    cpu.y = 0x01;
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.status, 0b0000_0010);
  }
}
