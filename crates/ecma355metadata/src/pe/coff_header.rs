use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};

use crate::error::Error;
use crate::pe::FileCharacteristics;

#[derive(Debug)]
pub struct CoffHeader {
    pub machine: u16,
    pub number_of_sections: u16,
    pub timestamp: u32,
    pub symbol_table_addr: u32,
    pub symbol_count: u32,
    pub optional_header_size: u16,
    pub characteristics: FileCharacteristics,
}

impl CoffHeader {
    pub const SIZE: usize = 20;

    pub fn read<A: Read>(buf: &mut A) -> Result<CoffHeader, Error> {
        Ok(CoffHeader {
            machine: buf.read_u16::<LittleEndian>()?,
            number_of_sections: buf.read_u16::<LittleEndian>()?,
            timestamp: buf.read_u32::<LittleEndian>()?,
            symbol_table_addr: buf.read_u32::<LittleEndian>()?,
            symbol_count: buf.read_u32::<LittleEndian>()?,
            optional_header_size: buf.read_u16::<LittleEndian>()?,
            characteristics: FileCharacteristics::from_bits_truncate(buf.read_u16::<LittleEndian>()?),
        })
    }
}
