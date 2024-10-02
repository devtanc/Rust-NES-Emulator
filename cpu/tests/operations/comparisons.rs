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
mod cmp {
  use super::*;
  const OP: &Operation = &Operation::CMP;

  #[test]
  fn flag_c_gt() {
    let mut cpu = Cpu::new();
    cpu.acc = 0x10;
    cpu.write_addr(ROOT, 0x02);
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.status, 0b0000_0001);
  }

  #[test]
  fn flag_c_et() {
    let mut cpu = Cpu::new();
    cpu.acc = 0x10;
    cpu.write_addr(ROOT, 0x10);
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.status, 0b0000_0011);
  }

  #[test]
  fn flag_z() {
    let mut cpu = Cpu::new();
    cpu.acc = 0x10;
    cpu.write_addr(ROOT, 0x10);
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.status, 0b0000_0011);
  }

  #[test]
  fn flag_n() {
    let mut cpu = Cpu::new();
    cpu.acc = 0x83;
    cpu.write_addr(ROOT, 0x02);
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.status, 0b1000_0001);
  }
}

#[cfg(test)]
mod cpx {
  use super::*;
  const OP: &Operation = &Operation::CPX;
  #[test]
  fn flag_c_gt() {
    let mut cpu = Cpu::new();
    cpu.x = 0x10;
    cpu.write_addr(ROOT, 0x02);
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.status, 0b0000_0001);
  }

  #[test]
  fn flag_c_et() {
    let mut cpu = Cpu::new();
    cpu.x = 0x10;
    cpu.write_addr(ROOT, 0x10);
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.status, 0b0000_0011);
  }

  #[test]
  fn flag_z() {
    let mut cpu = Cpu::new();
    cpu.x = 0x10;
    cpu.write_addr(ROOT, 0x10);
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.status, 0b0000_0011);
  }

  #[test]
  fn flag_n() {
    let mut cpu = Cpu::new();
    cpu.x = 0x83;
    cpu.write_addr(ROOT, 0x02);
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.status, 0b1000_0001);
  }
}

#[cfg(test)]
mod cpy {
  use super::*;
  const OP: &Operation = &Operation::CPY;
  #[test]
  fn flag_c_gt() {
    let mut cpu = Cpu::new();
    cpu.y = 0x10;
    cpu.write_addr(ROOT, 0x02);
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.status, 0b0000_0001);
  }

  #[test]
  fn flag_c_et() {
    let mut cpu = Cpu::new();
    cpu.y = 0x10;
    cpu.write_addr(ROOT, 0x10);
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.status, 0b0000_0011);
  }

  #[test]
  fn flag_z() {
    let mut cpu = Cpu::new();
    cpu.y = 0x10;
    cpu.write_addr(ROOT, 0x10);
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.status, 0b0000_0011);
  }

  #[test]
  fn flag_n() {
    let mut cpu = Cpu::new();
    cpu.y = 0x83;
    cpu.write_addr(ROOT, 0x02);
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.status, 0b1000_0001);
  }
}
