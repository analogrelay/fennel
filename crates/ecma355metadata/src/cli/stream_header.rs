use std::io::{Read, Seek};

use byteorder::{LittleEndian, ReadBytesExt};

use crate::error::Error;

pub struct StreamHeader {
    pub offset: u32,
    pub size: u32,
    pub name: String,
}


impl StreamHeader {
    pub fn read<A: Read + Seek>(mut stream: A) -> Result<StreamHeader, Error> {
        let offset = stream.read_u32::<LittleEndian>()?;
        let size = stream.read_u32::<LittleEndian>()?;

        // Read no more than 32 nul-terminated bytes
        let name_bytes = read_nul_terminated_bytes(stream, 32)?;
        let name = String::from_utf8(name_bytes)
            .or(Err(Error::InvalidMetadata("invalid UTF-8 string".into())))?;

        Ok(StreamHeader {
            offset: offset,
            size: size,
            name: name,
        })
    }
}

fn read_nul_terminated_bytes<A: Read>(mut stream: A, max: usize) -> Result<Vec<u8>, Error> {
    let mut bytes = Vec::new();
    let mut buf = [0u8; 4];
    loop {
        stream.read_exact(&mut buf)?;
        for b in buf.iter() {
            if *b == 0 {
                return Ok(bytes);
            }

            bytes.push(*b);

            // Panic if we go over the max
            assert!(bytes.len() <= max);
        }
    }
}
