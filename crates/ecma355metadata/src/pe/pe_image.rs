use std::ops::{Deref, Index, Range};
use std::io::{Cursor, Read, Seek, SeekFrom};

use byteorder::{LittleEndian, ReadBytesExt};
use tracing::trace;

use crate::pe::{CoffHeader, MemoryRange, PeHeader, SectionHeader};
use crate::error::Error;

// TODO: We could probably use a trait other than Deref in order to
// allow lazy-loading. But for now, this works.

/// Represents a Portable Executable Image, loaded into memory.
pub struct PeImage<D: Deref<Target = [u8]>> {
    coff_header: CoffHeader,
    pe_header: Option<PeHeader>,
    sections: Vec<SectionHeader>,
    data: D,
}

const DOS_SIGNATURE: u16 = 0x5A4D;
const PE_SIGNATURE: u32 = 0x00004550;

/// Represents a PE Image in memory
/// 
/// This type can be indexed or sliced like a [u8], but the offsets are RVAs (relative virtual adresses)
impl<D: Deref<Target = [u8]>> PeImage<D> {
    pub fn load(data: D) -> Result<PeImage<D>, Error> {
        let (coff_header, pe_header, sections) = {
            let mut reader = Cursor::new(data.deref());

            // Verify the MZ signature
            let mz_sig = reader.read_u16::<LittleEndian>()?;
            if mz_sig != DOS_SIGNATURE {
                return Err(Error::InvalidSignature);
            } else {
                // Seek to the lfanew field
                reader.seek(SeekFrom::Start(0x3C))?;

                // Read the lfanew offset
                let lfanew = reader.read_u32::<LittleEndian>()?;

                // Seek to the PE header
                reader.seek(SeekFrom::Start(lfanew as u64))?;

                // Read the PE signature
                let pe_sig = reader.read_u32::<LittleEndian>()?;

                // Read the COFF header
                let coff_header = CoffHeader::read(&mut reader)?;

                // Read the PE header if there is one
                let pe_header = if pe_sig != PE_SIGNATURE {
                    None
                } else {
                    Some(PeHeader::read(&mut reader)?)
                };

                // Read section headers
                let section_count = coff_header.number_of_sections as usize;
                let mut sections = Vec::with_capacity(section_count);
                for _ in 0..section_count {
                    sections.push(SectionHeader::read(&mut reader)?);
                }

                (coff_header, pe_header, sections)
            }
        };

        Ok(PeImage {
            coff_header,
            pe_header,
            sections,
            data,
        })
    }

    pub fn coff_header(&self) -> &CoffHeader {
        &self.coff_header
    }

    pub fn pe_header(&self) -> Option<&PeHeader> {
        self.pe_header.as_ref()
    }

    pub fn sections(&self) -> &Vec<SectionHeader> {
        &self.sections
    }

    pub fn data(&self) -> &D {
        &self.data
    }

    fn map_rva(&self, rva: usize) -> Option<(usize, usize)> {
        self.sections.iter().find(|x| x.contains_rva(rva as u32)).map(|x| {
            let offset = rva - x.virtual_address as usize;
            (x.pointer_to_raw_data as usize + offset, x.size_of_raw_data as usize - offset)
        })
    }
}

impl PeImage<Vec<u8>> {
    pub fn read<R: Read>(mut reader: R) -> Result<PeImage<Vec<u8>>, Error> {
        // Load the entire contents into memory first
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;

        // Return the image
        PeImage::load(buf)
    }
}

impl<D: Deref<Target=[u8]>> Index<usize> for PeImage<D> {
    type Output = u8;
    fn index(&self, index: usize) -> &u8 {
        let (phys, _) = self.map_rva(index).expect("RVA out of range");
        &self.data()[phys]
    }
}

impl<D: Deref<Target=[u8]>> Index<Range<usize>> for PeImage<D> {
    type Output = [u8];
    fn index(&self, index: Range<usize>) -> &[u8] {
        let requested_size = index.end - index.start;
        let (phys, size) = self.map_rva(index.start).expect("RVA out of range");
        if size < requested_size {
            panic!("RVA range is larger than physical range");
        }
        &self.data()[phys..(phys + requested_size)]
    }
}

impl<D: Deref<Target=[u8]>> Index<MemoryRange> for PeImage<D> {
    type Output = [u8];
    fn index(&self, index: MemoryRange) -> &[u8] {
        let (phys, size) = self.map_rva(index.start as usize)
            .expect("RVA out of range");
        if size < index.len as usize {
            panic!("RVA range is larger than physical range");
        }
        &self.data()[phys..(phys + index.len as usize)]
    }
}