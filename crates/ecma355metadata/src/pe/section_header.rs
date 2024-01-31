use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};

use crate::error::Error;
use crate::pe::SectionCharacteristics;

pub struct SectionHeader {
    pub name: String,
    pub virtual_size: u32,
    pub virtual_address: u32,
    pub size_of_raw_data: u32,
    pub pointer_to_raw_data: u32,
    pub pointer_to_relocations: u32,
    pub pointer_to_linenumbers: u32,
    pub number_of_relocations: u16,
    pub number_of_linenumbers: u16,
    pub characteristics: SectionCharacteristics,
}

impl SectionHeader {
    pub const SIZE: usize = 40;

    pub fn read<A: Read>(buf: &mut A) -> Result<SectionHeader, Error> {
        let mut name_bytes = [0u8; 8];
        buf.read(&mut name_bytes)?;
        let end = match name_bytes.iter().position(|x| *x == 0) {
            Some(x) => x,
            None => 8,
        };
        let name = String::from_utf8_lossy(&name_bytes[0..end]).into_owned();

        Ok(SectionHeader {
            name: name,
            virtual_size: buf.read_u32::<LittleEndian>()?,
            virtual_address: buf.read_u32::<LittleEndian>()?,
            size_of_raw_data: buf.read_u32::<LittleEndian>()?,
            pointer_to_raw_data: buf.read_u32::<LittleEndian>()?,
            pointer_to_relocations: buf.read_u32::<LittleEndian>()?,
            pointer_to_linenumbers: buf.read_u32::<LittleEndian>()?,
            number_of_relocations: buf.read_u16::<LittleEndian>()?,
            number_of_linenumbers: buf.read_u16::<LittleEndian>()?,
            characteristics: SectionCharacteristics::from_bits_truncate(
                buf.read_u32::<LittleEndian>()?,
            ),
        })
    }

    pub fn virtual_end(&self) -> u32 {
        self.virtual_address + self.virtual_size
    }

    pub fn contains_rva(&self, rva: u32) -> bool {
        rva >= self.virtual_address && rva < self.virtual_end()
    }
}
