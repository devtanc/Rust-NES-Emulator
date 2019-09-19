extern crate hex;
use std::fmt;

pub struct HexSlice<'a>(&'a [u8]);

impl<'a> HexSlice<'a> {
  pub fn new(data: &'a [u8]) -> HexSlice<'a> {
    HexSlice(data.as_ref())
  }
}

impl<'a> fmt::Display for HexSlice<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    static CHARS: &'static [u8] = b"0123456789ABCDEF";
    const MASK: u8 = 0x0F;

    for byte in self.0 {
      let hi_char: char = CHARS[((byte >> 4) & MASK) as usize].into();
      let lo_char: char = CHARS[(byte & MASK) as usize].into();

      write!(f, "{}{}  ", hi_char, lo_char)?;
    }

    Ok(())
  }
}

pub struct MemoryAddress {
  addr: u16,
}

impl MemoryAddress {
  pub fn new(addr: u16) -> MemoryAddress {
    MemoryAddress { addr }
  }
}

impl fmt::Display for MemoryAddress {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    static CHARS: &'static [u8] = b"0123456789ABCDEF";
    const MASK: u16 = 0x000F;

    let nib_3: char = CHARS[((self.addr >> 12) & MASK) as usize].into();
    let nib_2: char = CHARS[((self.addr >> 8) & MASK) as usize].into();
    let nib_1: char = CHARS[((self.addr >> 4) & MASK) as usize].into();
    let nib_0: char = CHARS[(self.addr & MASK) as usize].into();

    write!(f, "{}{}{}{}", nib_3, nib_2, nib_1, nib_0)?;

    Ok(())
  }
}

pub struct HexByte {
  byte: u8,
}

impl HexByte {
  pub fn new(byte: u8) -> HexByte {
    HexByte { byte }
  }
}

impl fmt::Display for HexByte {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    static CHARS: &'static [u8] = b"0123456789ABCDEF";
    const MASK: u8 = 0x000F;

    let nib_1: char = CHARS[((self.byte >> 4) & MASK) as usize].into();
    let nib_0: char = CHARS[(self.byte & MASK) as usize].into();

    write!(f, "{}{}", nib_1, nib_0)?;

    Ok(())
  }
}

pub trait Read {
  fn read_addr(&self, addr: u16) -> u8;
}

pub trait Write {
  fn write_addr(&mut self, addr: u16, data: u8);
}

pub trait ReadWrite {
  fn read_addr(&self, addr: u16) -> u8;

  fn write_addr(&mut self, addr: u16, data: u8);
}

pub trait ReadRange {
  fn read_range(&self, addr_start: u16, addr_end: u16) -> HexSlice;
}
