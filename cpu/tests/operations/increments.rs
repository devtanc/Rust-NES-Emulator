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
mod inc {
  use super::*;
  const OP: &Operation = &Operation::INC;
  #[test]
  fn increments_memory() {
    let mut cpu = Cpu::new();
    cpu.write_addr(ROOT, 0x00);
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.read_addr(ROOT), 0x01);
  }
  #[test]
  fn flag_n() {
    let mut cpu = Cpu::new();
    cpu.write_addr(ROOT, 0b0111_1111);
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.status, 0b1000_0000);
  }
  #[test]
  fn flag_z() {
    let mut cpu = Cpu::new();
    cpu.write_addr(ROOT, 0xFF);
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.status, 0b0000_0010);
  }
}

#[cfg(test)]
mod inx {
  use super::*;
  const OP: &Operation = &Operation::INX;
  #[test]
  fn increments_register() {
    let mut cpu = Cpu::new();
    cpu.x = 0x00;
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.x, 0x01);
  }
  #[test]
  fn flag_n() {
    let mut cpu = Cpu::new();
    cpu.x = 0b0111_1111;
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.status, 0b1000_0000);
  }
  #[test]
  fn flag_z() {
    let mut cpu = Cpu::new();
    cpu.x = 0xFF;
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.status, 0b0000_0010);
  }
}

#[cfg(test)]
mod iny {
  use super::*;
  const OP: &Operation = &Operation::INY;
  #[test]
  fn increments_register() {
    let mut cpu = Cpu::new();
    cpu.y = 0x00;
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.y, 0x01);
  }
  #[test]
  fn flag_n() {
    let mut cpu = Cpu::new();
    cpu.y = 0b0111_1111;
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.status, 0b1000_0000);
  }
  #[test]
  fn flag_z() {
    let mut cpu = Cpu::new();
    cpu.y = 0xFF;
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.status, 0b0000_0010);
  }
}
