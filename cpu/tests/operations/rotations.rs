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
mod rol {
  use super::*;
  const OP: &Operation = &Operation::ROL;
  #[test]
  fn shifts_bits_left() {
    let mut cpu = Cpu::new();
    cpu.write_addr(ROOT, 0b0110_0110);
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.read_addr(ROOT), 0b1100_1100);
  }
  #[test]
  fn adds_carry_in_bit_0() {
    let mut cpu = Cpu::new();
    cpu.status = 0b0000_0001;
    cpu.write_addr(ROOT, 0b0110_0110);
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.read_addr(ROOT), 0b1100_1101);
  }
  #[test]
  fn rotates_accumulator_when_accumulator_address_mode() {
    let mut cpu = Cpu::new();
    cpu.status = 0b0000_0001;
    cpu.acc = 0b0110_0110;
    cpu.perform_operation(ROOT, OP, &AddressMode::Accumulator);
    assert_eq!(cpu.acc, 0b1100_1101);
  }
  #[test]
  fn flag_n() {
    let mut cpu = Cpu::new();
    cpu.write_addr(ROOT, 0b0110_0110);
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.status, 0b1000_0000);
  }
  #[test]
  fn flag_z() {
    let mut cpu = Cpu::new();
    cpu.write_addr(ROOT, 0b0000_0000);
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.status, 0b0000_0010);
  }
  #[test]
  fn flag_c() {
    let mut cpu = Cpu::new();
    cpu.write_addr(ROOT, 0b1010_0000);
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.status, 0b0000_0001);
  }
}

#[cfg(test)]
mod ror {
  use super::*;
  const OP: &Operation = &Operation::ROR;
  #[test]
  fn shifts_bits_right() {
    let mut cpu = Cpu::new();
    cpu.write_addr(ROOT, 0b0110_0110);
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.read_addr(ROOT), 0b0011_0011);
  }
  #[test]
  fn adds_carry_in_bit_7() {
    let mut cpu = Cpu::new();
    cpu.status = 0b0000_0001;
    cpu.write_addr(ROOT, 0b0110_0110);
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.read_addr(ROOT), 0b1011_0011);
  }
  #[test]
  fn rotates_accumulator_when_accumulator_address_mode() {
    let mut cpu = Cpu::new();
    cpu.status = 0b0000_0001;
    cpu.acc = 0b0110_0110;
    cpu.perform_operation(ROOT, OP, &AddressMode::Accumulator);
    assert_eq!(cpu.acc, 0b1011_0011);
  }
  #[test]
  fn flag_n() {
    let mut cpu = Cpu::new();
    cpu.status = 0b0000_0001;
    cpu.write_addr(ROOT, 0b0110_0110);
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.status, 0b1000_0000);
  }
  #[test]
  fn flag_z() {
    let mut cpu = Cpu::new();
    cpu.write_addr(ROOT, 0b0000_0000);
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.status, 0b0000_0010);
  }
  #[test]
  fn flag_c() {
    let mut cpu = Cpu::new();
    cpu.write_addr(ROOT, 0b1010_0001);
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.status, 0b0000_0001);
  }
}
