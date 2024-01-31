use std::io::{Read, Seek, SeekFrom};

use byteorder::{LittleEndian, ReadBytesExt};

use crate::cli::StreamHeader;

use crate::error::Error;

use crate::utils;

pub struct MetadataHeader {
    pub major_version: u16,
    pub minor_version: u16,
    pub version: String,
    pub flags: u16,
    pub streams: Vec<StreamHeader>,
}

const METADATA_SIGNATURE: u32 = 0x424A5342;

impl MetadataHeader {
    pub fn read<A: Read + Seek>(mut buf: A) -> Result<MetadataHeader, Error> {
        // Read signature
        let signature = buf.read_u32::<LittleEndian>()?;
        if signature != METADATA_SIGNATURE {
            Err(Error::InvalidSignature)
        } else {
            let major_version = buf.read_u16::<LittleEndian>()?;
            let minor_version = buf.read_u16::<LittleEndian>()?;

            // Skip reserved value
            buf.read_u32::<LittleEndian>()?;

            // Read version length
            let version_length = buf.read_u32::<LittleEndian>()? as usize;

            // Read the string (unsafe because we use set_len)
            let version_bytes = utils::read_bytes(&mut buf, version_length)?;
            let version = String::from_utf8(version_bytes)
                .or(Err(Error::InvalidMetadata("invalid UTF-8 string")))?;

            // Use Seek to get the current position
            let current_file_pos = buf.seek(SeekFrom::Current(0))?;

            // Check if it's aligned
            if current_file_pos & 0x3 != 0 {
                // Get the next 4-byte aligned value
                let flags_start = (current_file_pos + 4) & !0x4u64;
                if flags_start != current_file_pos {
                    buf.seek(SeekFrom::Start(flags_start))?;
                }
            }

            let flags = buf.read_u16::<LittleEndian>()?;
            let stream_count = buf.read_u16::<LittleEndian>()?;

            // Read stream headers
            let mut streams = Vec::with_capacity(stream_count as usize);
            for _ in 0..stream_count {
                streams.push(StreamHeader::read(&mut buf)?);
            }

            // Read flags and streams values and return
            Ok(MetadataHeader {
                major_version,
                minor_version,
                version,
                flags,
                streams,
            })
        }
    }

    pub fn get_stream(&self, name: &str) -> Option<&StreamHeader> {
        self.streams.iter().find(|x| x.name == name)
    }
}
