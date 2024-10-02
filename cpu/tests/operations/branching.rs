#[cfg(test)]
use cpu::instruction::{AddressMode, Operation};

#[cfg(test)]
use cpu::Cpu;

#[test]
fn bcc_no_branch() {
  let mut cpu = Cpu::new();
  cpu.pc = 0x0000;
  cpu.status = 0b0000_0001;
  cpu.perform_operation(0b0000_0001, &Operation::BCC, &AddressMode::XXX);
  assert_eq!(cpu.pc, 0);
}

#[test]
fn bcc_branch() {
  let mut cpu = Cpu::new();
  cpu.pc = 0x0000;
  cpu.status = 0b0000_0000;
  cpu.perform_operation(0b0000_0001, &Operation::BCC, &AddressMode::XXX);
  assert_eq!(cpu.pc, 1);
}

#[test]
fn bcs_no_branch() {
  let mut cpu = Cpu::new();
  cpu.pc = 0x0000;
  cpu.status = 0b0000_0000;
  cpu.perform_operation(0b0000_0001, &Operation::BCS, &AddressMode::XXX);
  assert_eq!(cpu.pc, 0);
}

#[test]
fn bcs_branch() {
  let mut cpu = Cpu::new();
  cpu.pc = 0x0000;
  cpu.status = 0b0000_0001;
  cpu.perform_operation(0b0000_0001, &Operation::BCS, &AddressMode::XXX);
  assert_eq!(cpu.pc, 1);
}

#[test]
fn beq_no_branch() {
  let mut cpu = Cpu::new();
  cpu.pc = 0x0000;
  cpu.status = 0b0000_0000;
  cpu.perform_operation(0b0000_0001, &Operation::BEQ, &AddressMode::XXX);
  assert_eq!(cpu.pc, 0);
}

#[test]
fn beq_branch() {
  let mut cpu = Cpu::new();
  cpu.pc = 0x0000;
  cpu.status = 0b0000_0010;
  cpu.perform_operation(0b0000_0001, &Operation::BEQ, &AddressMode::XXX);
  assert_eq!(cpu.pc, 1);
}

#[test]
fn bmi_no_branch() {
  let mut cpu = Cpu::new();
  cpu.pc = 0x0000;
  cpu.status = 0b0000_0000;
  cpu.perform_operation(0b0000_0001, &Operation::BMI, &AddressMode::XXX);
  assert_eq!(cpu.pc, 0);
}

#[test]
fn bmi_branch() {
  let mut cpu = Cpu::new();
  cpu.pc = 0x0000;
  cpu.status = 0b1000_0000;
  cpu.perform_operation(0b0000_0001, &Operation::BMI, &AddressMode::XXX);
  assert_eq!(cpu.pc, 1);
}

#[test]
fn bne_no_branch() {
  let mut cpu = Cpu::new();
  cpu.pc = 0x0000;
  cpu.status = 0b0000_0010;
  cpu.perform_operation(0b0000_0001, &Operation::BNE, &AddressMode::XXX);
  assert_eq!(cpu.pc, 0);
}

#[test]
fn bne_branch() {
  let mut cpu = Cpu::new();
  cpu.pc = 0x0000;
  cpu.status = 0b0000_0000;
  cpu.perform_operation(0b0000_0001, &Operation::BNE, &AddressMode::XXX);
  assert_eq!(cpu.pc, 1);
}

#[test]
fn bpl_no_branch() {
  let mut cpu = Cpu::new();
  cpu.pc = 0x0000;
  cpu.status = 0b1000_0000;
  cpu.perform_operation(0b0000_0001, &Operation::BPL, &AddressMode::XXX);
  assert_eq!(cpu.pc, 0);
}

#[test]
fn bpl_branch() {
  let mut cpu = Cpu::new();
  cpu.pc = 0x0000;
  cpu.status = 0b0000_0000;
  cpu.perform_operation(0b0000_0001, &Operation::BPL, &AddressMode::XXX);
  assert_eq!(cpu.pc, 1);
}

#[test]
fn bvc_no_branch() {
  let mut cpu = Cpu::new();
  cpu.pc = 0x0000;
  cpu.status = 0b0100_0000;
  cpu.perform_operation(0b0000_0001, &Operation::BVC, &AddressMode::XXX);
  assert_eq!(cpu.pc, 0);
}

#[test]
fn bvc_branch() {
  let mut cpu = Cpu::new();
  cpu.pc = 0x0000;
  cpu.status = 0b0000_0000;
  cpu.perform_operation(0b0000_0001, &Operation::BVC, &AddressMode::XXX);
  assert_eq!(cpu.pc, 1);
}

#[test]
fn bvs_no_branch() {
  let mut cpu = Cpu::new();
  cpu.pc = 0x0000;
  cpu.status = 0b0000_0000;
  cpu.perform_operation(0b0000_0001, &Operation::BVS, &AddressMode::XXX);
  assert_eq!(cpu.pc, 0);
}

#[test]
fn bvs_branch() {
  let mut cpu = Cpu::new();
  cpu.pc = 0x0000;
  cpu.status = 0b0100_0000;
  cpu.perform_operation(0b0000_0001, &Operation::BVS, &AddressMode::XXX);
  assert_eq!(cpu.pc, 1);
}
