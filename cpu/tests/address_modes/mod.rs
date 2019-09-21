#[cfg(test)]
use cpu::instruction::AddressMode;

#[cfg(test)]
use cpu::Cpu;

#[cfg(test)]
use data_flow::ReadWrite;

#[cfg(test)]
const ROOT: u16 = 0x8000;

#[cfg(test)]
mod absolute {
  use super::*;
  const ADDR_MODE: &AddressMode = &AddressMode::Absolute;
  #[test]
  fn ptr_retrieved_from_memory_at_pc_addr() {
    let mut cpu = Cpu::new();
    cpu.pc = ROOT;
    cpu.write_addr(ROOT + 0, 0xED);
    cpu.write_addr(ROOT + 1, 0xCD);
    let ptr = cpu.get_data_ptr(ADDR_MODE);
    assert_eq!(ptr, 0xCDED);
  }
}

#[cfg(test)]
mod accumulator {
  use super::*;
  const ADDR_MODE: &AddressMode = &AddressMode::Accumulator;
  #[test]
  fn ptr_should_be_zero_since_unnecessary() {
    let mut cpu = Cpu::new();
    cpu.acc = 0xDE;
    cpu.pc = ROOT + 0x3000;
    let ptr = cpu.get_data_ptr(ADDR_MODE);
    assert_eq!(ptr, 0x00);
  }
}

#[cfg(test)]
mod implied {
  use super::*;
  const ADDR_MODE: &AddressMode = &AddressMode::Implied;
  #[test]
  fn ptr_should_be_zero_since_unnecessary() {
    let mut cpu = Cpu::new();
    cpu.acc = 0xDE;
    cpu.pc = 0xFBCD;
    let ptr = cpu.get_data_ptr(ADDR_MODE);
    assert_eq!(ptr, 0x00);
  }
}

#[cfg(test)]
mod immediate {
  use super::*;
  const ADDR_MODE: &AddressMode = &AddressMode::Immediate;
  #[test]
  fn ptr_should_be_equal_to_pc() {
    let mut cpu = Cpu::new();
    cpu.pc = ROOT;
    cpu.write_addr(ROOT, 0xFC);
    let ptr = cpu.get_data_ptr(ADDR_MODE);
    assert_eq!(ptr, ROOT);
  }
}

#[cfg(test)]
mod zero_page {
  use super::*;
  const ADDR_MODE: &AddressMode = &AddressMode::ZeroPage;
  #[test]
  fn ptr_should_be_zero_page() {
    let mut cpu = Cpu::new();
    cpu.pc = ROOT;
    cpu.write_addr(ROOT, 0xFC);
    let ptr = cpu.get_data_ptr(ADDR_MODE);
    assert_eq!(ptr & 0xFF00, 0x0000);
  }
  #[test]
  fn ptr_lo_bit_should_be_data_from_pc_addr() {
    let mut cpu = Cpu::new();
    cpu.pc = ROOT;
    cpu.write_addr(ROOT, 0xFC);
    let ptr = cpu.get_data_ptr(ADDR_MODE);
    assert_eq!(ptr & 0x00FF, 0x00FC);
  }
}

#[cfg(test)]
mod zero_page_x {
  use super::*;
  const ADDR_MODE: &AddressMode = &AddressMode::ZeroPageX;
  #[test]
  fn ptr_should_be_zero_page() {
    let mut cpu = Cpu::new();
    cpu.pc = ROOT;
    cpu.write_addr(ROOT, 0xFC);
    let ptr = cpu.get_data_ptr(ADDR_MODE);
    assert_eq!(ptr & 0xFF00, 0x0000);
  }
  #[test]
  fn ptr_should_be_sum_of_x_register_and_data_at_pc_addr() {
    let mut cpu = Cpu::new();
    cpu.pc = ROOT;
    cpu.x = 0x01;
    cpu.write_addr(ROOT, 0xFC);
    let ptr = cpu.get_data_ptr(ADDR_MODE);
    assert_eq!(ptr & 0x00FF, 0x00FD);
  }
  #[test]
  fn ptr_should_not_cross_pages_on_overflow() {
    let mut cpu = Cpu::new();
    cpu.pc = ROOT;
    cpu.x = 0x04;
    cpu.write_addr(ROOT, 0xFC);
    let ptr = cpu.get_data_ptr(ADDR_MODE);
    assert_eq!(ptr & 0xFF00, 0x0000);
  }
  #[test]
  fn ptr_should_wrap_on_overflow() {
    let mut cpu = Cpu::new();
    cpu.pc = ROOT;
    cpu.x = 0x05;
    cpu.write_addr(ROOT, 0xFC);
    let ptr = cpu.get_data_ptr(ADDR_MODE);
    assert_eq!(ptr, 0x0001);
  }
}

#[cfg(test)]
mod zero_page_y {
  use super::*;
  const ADDR_MODE: &AddressMode = &AddressMode::ZeroPageY;
  #[test]
  fn ptr_should_be_zero_page() {
    let mut cpu = Cpu::new();
    cpu.pc = ROOT;
    cpu.write_addr(ROOT, 0xFC);
    let ptr = cpu.get_data_ptr(ADDR_MODE);
    assert_eq!(ptr & 0xFF00, 0x0000);
  }
  #[test]
  fn ptr_should_be_sum_of_y_register_and_data_at_pc_addr() {
    let mut cpu = Cpu::new();
    cpu.pc = ROOT;
    cpu.y = 0x01;
    cpu.write_addr(ROOT, 0xFC);
    let ptr = cpu.get_data_ptr(ADDR_MODE);
    assert_eq!(ptr & 0x00FF, 0x00FD);
  }
  #[test]
  fn ptr_should_not_cross_pages_on_overflow() {
    let mut cpu = Cpu::new();
    cpu.pc = ROOT;
    cpu.y = 0x04;
    cpu.write_addr(ROOT, 0xFC);
    let ptr = cpu.get_data_ptr(ADDR_MODE);
    assert_eq!(ptr & 0xFF00, 0x0000);
  }
  #[test]
  fn ptr_should_wrap_on_overflow() {
    let mut cpu = Cpu::new();
    cpu.pc = ROOT;
    cpu.y = 0x05;
    cpu.write_addr(ROOT, 0xFC);
    let ptr = cpu.get_data_ptr(ADDR_MODE);
    assert_eq!(ptr, 0x0001);
  }
}

#[cfg(test)]
mod absolute_x {
  use super::*;
  const ADDR_MODE: &AddressMode = &AddressMode::AbsoluteX;
  #[test]
  fn ptr_should_be_addr_at_pc_address() {
    let mut cpu = Cpu::new();
    cpu.pc = ROOT;
    cpu.write_addr(ROOT + 0, 0xFC);
    cpu.write_addr(ROOT + 1, 0xFC);
    let ptr = cpu.get_data_ptr(ADDR_MODE);
    assert_eq!(ptr, 0xFCFC);
  }
  #[test]
  fn ptr_should_be_sum_of_x_register_and_addr_at_pc_address() {
    let mut cpu = Cpu::new();
    cpu.pc = ROOT;
    cpu.write_addr(ROOT + 0, 0xFC);
    cpu.write_addr(ROOT + 1, 0xFC);
    cpu.x = 0x04;
    let ptr = cpu.get_data_ptr(ADDR_MODE);
    assert_eq!(ptr, 0xFD00);
  }
  #[test]
  fn ptr_should_wrap_on_overflow() {
    let mut cpu = Cpu::new();
    cpu.pc = ROOT;
    cpu.write_addr(ROOT + 0, 0xFF);
    cpu.write_addr(ROOT + 1, 0xFF);
    cpu.x = 0x02;
    let ptr = cpu.get_data_ptr(ADDR_MODE);
    assert_eq!(ptr, 0x0001);
  }
}

#[cfg(test)]
mod absolute_y {
  use super::*;
  const ADDR_MODE: &AddressMode = &AddressMode::AbsoluteY;
  #[test]
  fn ptr_should_be_addr_at_pc_address() {
    let mut cpu = Cpu::new();
    cpu.pc = ROOT;
    cpu.write_addr(ROOT + 0, 0xFC);
    cpu.write_addr(ROOT + 1, 0xFC);
    let ptr = cpu.get_data_ptr(ADDR_MODE);
    assert_eq!(ptr, 0xFCFC);
  }
  #[test]
  fn ptr_should_be_sum_of_x_register_and_addr_at_pc_address() {
    let mut cpu = Cpu::new();
    cpu.pc = ROOT;
    cpu.write_addr(ROOT + 0, 0xFC);
    cpu.write_addr(ROOT + 1, 0xFC);
    cpu.y = 0x04;
    let ptr = cpu.get_data_ptr(ADDR_MODE);
    assert_eq!(ptr, 0xFD00);
  }
  #[test]
  fn ptr_should_wrap_on_overflow() {
    let mut cpu = Cpu::new();
    cpu.pc = ROOT;
    cpu.write_addr(ROOT + 0, 0xFF);
    cpu.write_addr(ROOT + 1, 0xFF);
    cpu.y = 0x02;
    let ptr = cpu.get_data_ptr(ADDR_MODE);
    assert_eq!(ptr, 0x0001);
  }
}

#[cfg(test)]
mod relative {
  use super::*;
  const ADDR_MODE: &AddressMode = &AddressMode::Relative;
  #[test]
  fn pc_should_increment() {
    let mut cpu = Cpu::new();
    cpu.pc = ROOT;
    cpu.get_data_ptr(ADDR_MODE);
    assert_eq!(cpu.pc, ROOT + 1);
  }
  #[test]
  fn ptr_should_be_sum_of_data_at_pc_addr_and_pc() {
    const OFFSET: u8 = 0x0C;
    let mut cpu = Cpu::new();
    cpu.pc = ROOT;
    cpu.write_addr(ROOT, OFFSET);
    let ptr = cpu.get_data_ptr(ADDR_MODE);
    assert_eq!(ptr, cpu.pc + (OFFSET as u16));
  }
  #[test]
  fn ptr_should_wrap_on_overflow() {
    const OFFSET: u8 = 0x0C;
    let mut cpu = Cpu::new();
    cpu.pc = 0xFFFD;
    cpu.write_addr(0xFFFD, OFFSET);
    let ptr = cpu.get_data_ptr(ADDR_MODE);
    assert_eq!(ptr, 0x000A);
  }
}

#[cfg(test)]
mod indirect_x {
  use super::*;
  const ADDR_MODE: &AddressMode = &AddressMode::IndirectX;
  #[test]
  fn pc_should_increment() {
    let mut cpu = Cpu::new();
    cpu.pc = ROOT;
    cpu.get_data_ptr(ADDR_MODE);
    assert_eq!(cpu.pc, ROOT + 1);
  }
  #[test]
  fn ptr_should_be_address_from_zero_page_addr_specified_by_byte_at_pc() {
    const ZERO_PAGE_ADDR: u16 = 0x0020;
    let mut cpu = Cpu::new();
    cpu.pc = ROOT;
    cpu.write_addr(ZERO_PAGE_ADDR + 0, 0xAD); // lo
    cpu.write_addr(ZERO_PAGE_ADDR + 1, 0xFA); // hi
    cpu.write_addr(ROOT, ZERO_PAGE_ADDR as u8); // zp addr
    let ptr = cpu.get_data_ptr(ADDR_MODE);
    assert_eq!(ptr, 0xFAAD);
  }
  #[test]
  fn should_wrap_zero_page_on_overflow() {
    const ZERO_PAGE_ADDR: u8 = 0xFF;
    let mut cpu = Cpu::new();
    cpu.pc = ROOT;
    cpu.write_addr(ZERO_PAGE_ADDR as u16, 0xAD); // lo
    cpu.write_addr(ZERO_PAGE_ADDR.wrapping_add(1) as u16, 0xFA); // hi
    cpu.write_addr(ROOT, ZERO_PAGE_ADDR); // zp addr
    let ptr = cpu.get_data_ptr(ADDR_MODE);
    assert_eq!(ptr, 0xFAAD);
  }
}

#[cfg(test)]
mod indirect_y {
  use super::*;
  const ADDR_MODE: &AddressMode = &AddressMode::IndirectY;
  #[test]
  fn pc_should_increment() {
    let mut cpu = Cpu::new();
    cpu.pc = ROOT;
    cpu.get_data_ptr(ADDR_MODE);
    assert_eq!(cpu.pc, ROOT + 1);
  }
  #[test]
  fn ptr_should_be_address_from_zero_page_addr_specified_by_byte_at_pc() {
    const ZERO_PAGE_ADDR: u16 = 0x0020;
    let mut cpu = Cpu::new();
    cpu.pc = ROOT;
    cpu.y = 0x02;
    cpu.write_addr(ZERO_PAGE_ADDR + 0, 0xAD); // lo
    cpu.write_addr(ZERO_PAGE_ADDR + 1, 0xFA); // hi
    cpu.write_addr(ROOT, ZERO_PAGE_ADDR as u8); // zp addr
    let ptr = cpu.get_data_ptr(ADDR_MODE);
    assert_eq!(ptr, 0xFAAF);
  }
  #[test]
  fn should_wrap_zero_page_on_overflow() {
    const ZERO_PAGE_ADDR: u8 = 0xFF;
    let mut cpu = Cpu::new();
    cpu.pc = ROOT;
    cpu.y = 0x02;
    cpu.write_addr(ZERO_PAGE_ADDR as u16, 0xAD); // lo
    cpu.write_addr(ZERO_PAGE_ADDR.wrapping_add(1) as u16, 0xFA); // hi
    cpu.write_addr(ROOT, ZERO_PAGE_ADDR); // zp addr
    let ptr = cpu.get_data_ptr(ADDR_MODE);
    assert_eq!(ptr, 0xFAAF);
  }
  #[test]
  fn should_wrap_address_on_overflow_with_y_register() {
    const ZERO_PAGE_ADDR: u8 = 0xFF;
    let mut cpu = Cpu::new();
    cpu.pc = ROOT;
    cpu.y = 0x02;
    cpu.write_addr(ZERO_PAGE_ADDR as u16, 0xFF); // lo
    cpu.write_addr(ZERO_PAGE_ADDR.wrapping_add(1) as u16, 0xFF); // hi
    cpu.write_addr(ROOT, ZERO_PAGE_ADDR); // zp addr
    let ptr = cpu.get_data_ptr(ADDR_MODE);
    assert_eq!(ptr, 0x0001);
  }
}

#[cfg(test)]
mod absolute_indirect {
  use super::*;
  const ADDR_MODE: &AddressMode = &AddressMode::AbsoluteIndirect;
  #[test]
  fn pc_should_increment_twice() {
    let mut cpu = Cpu::new();
    cpu.pc = ROOT;
    cpu.get_data_ptr(ADDR_MODE);
    assert_eq!(cpu.pc, ROOT + 2);
  }
  #[test]
  fn ptr_should_be_addr_in_memory_at_addr_at_pc() {
    let mut cpu = Cpu::new();
    cpu.pc = ROOT;
    cpu.write_addr(ROOT + 0, 0xAB);
    cpu.write_addr(ROOT + 1, 0xCD);
    cpu.write_addr(0xCDAB + 0, 0xAF);
    cpu.write_addr(0xCDAB + 1, 0xFA);
    let ptr = cpu.get_data_ptr(ADDR_MODE);
    assert_eq!(ptr, 0xFAAF);
  }
  #[test]
  fn should_respect_page_boundary_hardware_bug() {
    let mut cpu = Cpu::new();
    cpu.pc = ROOT;
    cpu.write_addr(ROOT + 0, 0xFF);
    cpu.write_addr(ROOT + 1, 0xCD);
    cpu.write_addr(0xCDFF + 0, 0xAF);
    cpu.write_addr(0xCDFF + 1, 0xFA); // proper hi value
    cpu.write_addr(0xCD00, 0xBA); // bug hi value
    let ptr = cpu.get_data_ptr(ADDR_MODE);
    assert_eq!(ptr, 0xBAAF);
  }
}
