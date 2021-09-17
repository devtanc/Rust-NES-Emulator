#[cfg(test)]
use cpu::instruction::{AddressMode, Operation};

#[cfg(test)]
use cpu::Cpu;

#[cfg(test)]
use data_flow::ReadWrite;

#[cfg(test)]
const ROOT: u16 = 0x0000;

#[cfg(test)]
const DEF_ADDR_MODE: &AddressMode = &AddressMode::XXX;

#[cfg(test)]
mod adc {
  use super::*;
  const OP: &Operation = &Operation::ADC;
  #[test]
  fn flag_none() {
    let mut cpu = Cpu::new();
    cpu.acc = 0b0000_0001;
    cpu.write_addr(ROOT, 0b0000_0001);
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.acc, 0b0000_0010);
    assert_eq!(cpu.status, 0b0000_0000);
  }

  #[test]
  fn flag_z() {
    let mut cpu = Cpu::new();
    cpu.acc = 0b0000_0000;
    cpu.write_addr(ROOT, 0b0000_0000);
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.acc, 0b0000_0000);
    assert_eq!(cpu.status, 0b0000_0010);
  }

  #[test]
  fn flag_v_pos_neg() {
    let mut cpu = Cpu::new();
    cpu.acc = 0b0111_1111;
    cpu.write_addr(ROOT, 0b0111_1111);
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.acc, 0b1111_1110);
    assert_eq!(cpu.status, 0b1100_0000);
  }

  #[test]
  fn flag_v_neg_pos() {
    let mut cpu = Cpu::new();
    cpu.acc = 0b1011_1111;
    cpu.write_addr(ROOT, 0b1011_1111);
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.acc, 0b0111_1110);
    assert_eq!(cpu.status, 0b0100_0001);
  }

  #[test]
  fn flag_c() {
    let mut cpu = Cpu::new();
    cpu.acc = 0b1111_1111;
    cpu.write_addr(ROOT, 0b1111_1111);
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.acc, 0b1111_1110);
    assert_eq!(cpu.status, 0b1000_0001);
  }
}

#[cfg(test)]
mod sbc {
  use super::*;
  const OP: &Operation = &Operation::SBC;
  #[test]
  fn flag_none() {
    let mut cpu = Cpu::new();
    cpu.acc = 0b0000_0001;
    cpu.write_addr(ROOT, !0b0000_0001);
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.acc, 0b0000_0010);
    assert_eq!(cpu.status, 0b0000_0000);
  }

  #[test]
  fn flag_z() {
    let mut cpu = Cpu::new();
    cpu.acc = 0b0000_0000;
    cpu.write_addr(ROOT, !0b0000_0000);
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.acc, 0b0000_0000);
    assert_eq!(cpu.status, 0b0000_0010);
  }

  #[test]
  fn flag_v_pos_neg() {
    let mut cpu = Cpu::new();
    cpu.acc = 0b0111_1111;
    cpu.write_addr(ROOT, !0b0111_1111);
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.acc, 0b1111_1110);
    assert_eq!(cpu.status, 0b1100_0000);
  }

  #[test]
  fn flag_v_neg_pos() {
    let mut cpu = Cpu::new();
    cpu.acc = 0b1011_1111;
    cpu.write_addr(ROOT, !0b1011_1111);
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.acc, 0b0111_1110);
    assert_eq!(cpu.status, 0b0100_0001);
  }

  #[test]
  fn flag_c() {
    let mut cpu = Cpu::new();
    cpu.acc = 0b1111_1111;
    cpu.write_addr(ROOT, !0b1111_1111);
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.acc, 0b1111_1110);
    assert_eq!(cpu.status, 0b1000_0001);
  }
}

#[cfg(test)]
mod and {
  use super::*;
  const OP: &Operation = &Operation::AND;

  #[test]
  fn flag_none() {
    let mut cpu = Cpu::new();
    cpu.acc = 0b1000_0001;
    cpu.write_addr(ROOT, 0b0000_0011);
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.status, 0b0000_0000);
  }

  #[test]
  fn flag_z() {
    let mut cpu = Cpu::new();
    cpu.acc = 0b0001_0001;
    cpu.write_addr(ROOT, 0b000_0000);
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.status, 0b0000_0010);
  }

  #[test]
  fn flag_n() {
    let mut cpu = Cpu::new();
    cpu.acc = 0b1000_0001;
    cpu.write_addr(ROOT, 0b1000_0000);
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.status, 0b1000_0000);
  }
}

#[cfg(test)]
mod asl {
  use super::*;
  const OP: &Operation = &Operation::ASL;
  #[test]
  fn flag_c() {
    let mut cpu = Cpu::new();
    cpu.write_addr(ROOT, 0b1000_1000);
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.status, 0b0000_0001);
  }

  #[test]
  fn flag_z() {
    let mut cpu = Cpu::new();
    cpu.write_addr(ROOT, 0b0000_0000);
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.status, 0b0000_0010);
  }

  #[test]
  fn flag_n() {
    let mut cpu = Cpu::new();
    cpu.write_addr(ROOT, 0b0100_0000);
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.status, 0b1000_0000);
  }
}

#[cfg(test)]
mod bit {
  use super::*;
  const OP: &Operation = &Operation::BIT;
  #[test]
  fn flag_z() {
    let mut cpu = Cpu::new();
    cpu.acc = 0;
    cpu.write_addr(ROOT, 0);
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.status, 0b0000_0010);
  }

  #[test]
  fn flag_n() {
    let mut cpu = Cpu::new();
    cpu.acc = 0x80;
    cpu.write_addr(ROOT, 0x80);
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.status, 0b1000_0000);
  }

  #[test]
  fn flag_v() {
    let mut cpu = Cpu::new();
    cpu.acc = 0x40;
    cpu.write_addr(ROOT, 0x40);
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.status, 0b0100_0000);
  }
}

#[cfg(test)]
mod brk {
  use super::*;
  const OP: &Operation = &Operation::BRK;
  #[test]
  fn set_pc_to_brk_vector_address() {
    let mut cpu = Cpu::new();
    cpu.write_addr(0xFFFE, 0x04);
    cpu.write_addr(0xFFFF, 0x40);
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.pc, 0x4004);
  }
  #[test]
  fn pushes_status_register_to_stack() {
    let mut cpu = Cpu::new();
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    let status = cpu.stack_pop();
    assert_eq!(status, 0b0001_0100);
  }
  #[test]
  fn pushes_pc_to_stack_before_status_register() {
    let mut cpu = Cpu::new();
    cpu.pc = 0x10F0;
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    // Remove status register
    let _ = cpu.stack_pop();
    // Get pc
    let pc_lo = cpu.stack_pop();
    let pc_hi = cpu.stack_pop();
    assert_eq!(pc_lo, 0xF1);
    assert_eq!(pc_hi, 0x10);
  }
  #[test]
  fn flag_i_but_not_b() {
    let mut cpu = Cpu::new();
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.status, 0b0000_0100);
  }
}

#[cfg(test)]
mod eor {
  use super::*;
  const OP: &Operation = &Operation::EOR;
  #[test]
  fn stores_result_in_accumulator() {
    let mut cpu = Cpu::new();
    cpu.acc = 0b1100_1100;
    cpu.write_addr(ROOT, 0b1010_1010);
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.acc, 0b0110_0110);
  }
  #[test]
  fn flag_n() {
    let mut cpu = Cpu::new();
    cpu.acc = 0b1100_1100;
    cpu.write_addr(ROOT, 0b0010_1010);
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.status, 0b1000_0000);
  }
  #[test]
  fn flag_z() {
    let mut cpu = Cpu::new();
    cpu.acc = 0b0000_0000;
    cpu.write_addr(ROOT, 0b0000_0000);
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.status, 0b0000_0010);
  }
}

#[cfg(test)]
mod jmp {
  use super::*;
  const OP: &Operation = &Operation::JMP;
  #[test]
  fn sets_pc_to_pointer_addr() {
    let mut cpu = Cpu::new();
    cpu.pc = ROOT + 0x00F0;
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.pc, ROOT);
  }
}

#[cfg(test)]
mod jsr {
  use super::*;
  const OP: &Operation = &Operation::JSR;
  #[test]
  fn sets_pc_to_pointer_addr() {
    let mut cpu = Cpu::new();
    cpu.pc = ROOT + 0x00F0;
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.pc, ROOT);
  }
  #[test]
  fn pushes_decremented_pc_to_stack() {
    let mut cpu = Cpu::new();
    cpu.pc = 0x10FE;
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);

    let pc_lo = cpu.stack_pop();
    let pc_hi = cpu.stack_pop();
    assert_eq!(pc_lo, 0xFD);
    assert_eq!(pc_hi, 0x10);
  }
}

#[cfg(test)]
mod lsr {
  use super::*;
  const OP: &Operation = &Operation::LSR;
  #[test]
  fn shifts_value_right() {
    let mut cpu = Cpu::new();
    cpu.write_addr(ROOT, 0b0110_0110);
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.read_addr(ROOT), 0b0011_0011);
  }
  #[test]
  fn rotates_accumulator_when_accumulator_address_mode() {
    let mut cpu = Cpu::new();
    cpu.acc = 0b0110_0110;
    cpu.perform_operation(ROOT, OP, &AddressMode::Accumulator);
    assert_eq!(cpu.acc, 0b0011_0011);
  }
  #[test]
  fn does_not_shift_in_carry_bit() {
    let mut cpu = Cpu::new();
    cpu.status = 0b0000_0001;
    cpu.acc = 0b0110_0110;
    cpu.perform_operation(ROOT, OP, &AddressMode::Accumulator);
    assert_eq!(cpu.acc, 0b0011_0011);
  }
  #[test]
  fn flag_c() {
    let mut cpu = Cpu::new();
    cpu.write_addr(ROOT, 0b0110_0111);
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.status, 0b0000_0001);
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
    cpu.write_addr(ROOT, 0x08);
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.status, 0b0000_0000);
  }
}

#[cfg(test)]
mod nop {
  use super::*;
  const OP: &Operation = &Operation::NOP;
  #[test]
  fn increments_cycle_on_0xfc_opcode() {
    let mut cpu = Cpu::new();
    cpu.opcode = 0xFC;
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.cycles, 1);
  }
}

#[cfg(test)]
mod ora {
  use super::*;
  const OP: &Operation = &Operation::ORA;
  #[test]
  fn stores_result_in_accumulator() {
    let mut cpu = Cpu::new();
    cpu.acc = 0b0101_0101;
    cpu.write_addr(ROOT, 0b1111_1111);
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.acc, 0xFF);
  }
  #[test]
  fn flag_n() {
    let mut cpu = Cpu::new();
    cpu.acc = 0b0101_0101;
    cpu.write_addr(ROOT, 0b1111_1111);
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.status, 0b1000_0000);
  }
  #[test]
  fn flag_z() {
    let mut cpu = Cpu::new();
    cpu.acc = 0x00;
    cpu.write_addr(ROOT, 0x00);
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.status, 0b0000_0010);
  }
}

#[cfg(test)]
mod pha {
  use super::*;
  const OP: &Operation = &Operation::PHA;
  #[test]
  fn pushes_accumulator_onto_stack() {
    let mut cpu = Cpu::new();
    cpu.acc = 0xAD;
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.stack_pop(), 0xAD);
  }
}

#[cfg(test)]
mod php {
  use super::*;
  const OP: &Operation = &Operation::PHP;
  #[test]
  // TODO: Do we need to worry about this: https://stackoverflow.com/questions/52017657/6502-emulator-testing-nestest
  fn pushes_status_register_onto_stack() {
    let mut cpu = Cpu::new();
    cpu.status = 0xAD;
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.stack_pop(), 0xAD);
  }
}

#[cfg(test)]
mod pla {
  use super::*;
  const OP: &Operation = &Operation::PLA;
  #[test]
  fn pops_stack_value_into_accumulator() {
    let mut cpu = Cpu::new();
    cpu.acc = 0x00;
    cpu.stack_push(0xAD);
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.acc, 0xAD);
  }

  #[test]
  fn sets_zero_flag_when_stack_value_is_zero() {
    let mut cpu = Cpu::new();
    cpu.acc = 0x00;
    cpu.status = 0x00;
    cpu.stack_push(0x00);
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.status, 0x02);
  }

  #[test]
  fn sets_neg_flag_when_stack_value_is_neg() {
    let mut cpu = Cpu::new();
    cpu.acc = 0x00;
    cpu.status = 0x00;
    cpu.stack_push(0x85);
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.status, 0x80);
  }
}

#[cfg(test)]
mod plp {
  use super::*;
  const OP: &Operation = &Operation::PLP;
  #[test]
  fn pops_stack_value_into_status() {
    let mut cpu = Cpu::new();
    cpu.status = 0x00;
    cpu.stack_push(0xAD);
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.status, 0xAD);
  }
}

#[cfg(test)]
mod rti {
  use super::*;
  const OP: &Operation = &Operation::RTI;
  #[test]
  fn pops_stack_into_status_register_with_bui_flags_cleared() {
    let mut cpu = Cpu::new();
    cpu.status = 0x00;
    cpu.stack_push(0b1111_1111);
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.status, 0b1100_1011);
  }
  #[test]
  fn pops_pc_after_status_register() {
    let mut cpu = Cpu::new();
    cpu.status = 0x00;
    cpu.stack_push(0xAC);
    cpu.stack_push(0xD0);
    cpu.stack_push(0xFF);
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.pc, 0xACD0);
  }
}

#[cfg(test)]
mod rts {
  use super::*;
  const OP: &Operation = &Operation::RTS;
  #[test]
  fn pops_stack_into_pc_and_increments() {
    let mut cpu = Cpu::new();
    cpu.stack_push(0xDE);
    cpu.stack_push(0x40);
    cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE);
    assert_eq!(cpu.pc, 0xDE41);
  }
}

#[cfg(test)]
mod xxx {
  use super::*;
  const OP: &Operation = &Operation::XXX;
  #[test]
  fn does_nothing() {
    let mut cpu = Cpu::new();
    assert_eq!((), cpu.perform_operation(ROOT, OP, DEF_ADDR_MODE));
  }
}

#[test]
fn immediate_addressing_increments_pc() {
  let mut cpu = Cpu::new();
  cpu.pc = 0x0000;
  cpu.perform_operation(ROOT, &Operation::XXX, &AddressMode::Immediate);
  assert_eq!(cpu.pc, 1);
}
