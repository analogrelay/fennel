use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};

use crate::error::Error;

#[derive(Clone, Copy)]
pub struct MemoryRange {
    pub start: u32,
    pub len: u32,
}

impl MemoryRange {
    pub fn new(start: u32, len: u32) -> MemoryRange {
        MemoryRange {
            start: start,
            len: len,
        }
    }

    pub fn read<A: Read>(mut buf: A) -> Result<MemoryRange, Error> {
        Ok(MemoryRange::new(
            buf.read_u32::<LittleEndian>()?,
            buf.read_u32::<LittleEndian>()?,
        ))
    }

    pub fn end(&self) -> u32 {
        self.start + self.len
    }
}

impl ::std::fmt::Display for MemoryRange {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        write!(
            f,
            "0x{:08X} - 0x{:08X} [Size: 0x{:08X}]",
            self.start,
            self.end(),
            self.len
        )
    }
}
