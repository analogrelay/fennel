use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};

use crate::error::Error;
use crate::pe::{DirectoryEntry, DirectoryType, PeMagic, Subsystem};

pub struct PeHeader {
    pub magic: PeMagic,
    pub major_linker_version: u8,
    pub minor_linker_version: u8,
    pub code_size: u32,
    pub initialized_data_size: u32,
    pub uninitialized_data_size: u32,
    pub entry_point_rva: u32,
    pub code_base: u32,
    pub data_base: u32,
    pub image_base: u64,
    pub section_alignment: u32,
    pub file_alignment: u32,
    pub major_os_version: u16,
    pub minor_os_version: u16,
    pub major_image_version: u16,
    pub minor_image_version: u16,
    pub major_subsystem_version: u16,
    pub minor_subsystem_version: u16,
    pub win32_version: u32,
    pub size_of_image: u32,
    pub size_of_headers: u32,
    pub checksum: u32,
    pub subsystem: Subsystem,
    pub dll_flags: u16,
    pub stack_reserve_size: u64,
    pub stack_commit_size: u64,
    pub heap_reserve_size: u64,
    pub heap_commit_size: u64,
    pub loader_flags: u32,
    pub number_of_data_directories: u32,
    directories: Vec<DirectoryEntry>,
}

impl PeHeader {
    pub const SIZE: usize = 28;

    pub fn read<A: Read>(buf: &mut A) -> Result<PeHeader, Error> {
        // Check the magic number
        let magic = PeMagic::new(buf.read_u16::<LittleEndian>()?);
        if magic != PeMagic::PE32 && magic != PeMagic::PE32PLUS {
            Err(Error::InvalidSignature)
        } else {
            Ok(PeHeader {
                magic: magic,
                major_linker_version: buf.read_u8()?,
                minor_linker_version: buf.read_u8()?,
                code_size: buf.read_u32::<LittleEndian>()?,
                initialized_data_size: buf.read_u32::<LittleEndian>()?,
                uninitialized_data_size: buf.read_u32::<LittleEndian>()?,
                entry_point_rva: buf.read_u32::<LittleEndian>()?,
                code_base: buf.read_u32::<LittleEndian>()?,
                data_base: if magic.is_pe32plus() {
                    0
                } else {
                    buf.read_u32::<LittleEndian>()?
                },
                image_base: if magic.is_pe32plus() {
                    buf.read_u64::<LittleEndian>()?
                } else {
                    buf.read_u32::<LittleEndian>()? as u64
                },
                section_alignment: buf.read_u32::<LittleEndian>()?,
                file_alignment: buf.read_u32::<LittleEndian>()?,
                major_os_version: buf.read_u16::<LittleEndian>()?,
                minor_os_version: buf.read_u16::<LittleEndian>()?,
                major_image_version: buf.read_u16::<LittleEndian>()?,
                minor_image_version: buf.read_u16::<LittleEndian>()?,
                major_subsystem_version: buf.read_u16::<LittleEndian>()?,
                minor_subsystem_version: buf.read_u16::<LittleEndian>()?,
                win32_version: buf.read_u32::<LittleEndian>()?,
                size_of_image: buf.read_u32::<LittleEndian>()?,
                size_of_headers: buf.read_u32::<LittleEndian>()?,
                checksum: buf.read_u32::<LittleEndian>()?,
                subsystem: Subsystem::new(buf.read_u16::<LittleEndian>()?),
                dll_flags: buf.read_u16::<LittleEndian>()?,
                stack_reserve_size: if magic.is_pe32plus() {
                    buf.read_u64::<LittleEndian>()?
                } else {
                    buf.read_u32::<LittleEndian>()? as u64
                },
                stack_commit_size: if magic.is_pe32plus() {
                    buf.read_u64::<LittleEndian>()?
                } else {
                    buf.read_u32::<LittleEndian>()? as u64
                },
                heap_reserve_size: if magic.is_pe32plus() {
                    buf.read_u64::<LittleEndian>()?
                } else {
                    buf.read_u32::<LittleEndian>()? as u64
                },
                heap_commit_size: if magic.is_pe32plus() {
                    buf.read_u64::<LittleEndian>()?
                } else {
                    buf.read_u32::<LittleEndian>()? as u64
                },
                loader_flags: buf.read_u32::<LittleEndian>()?,
                number_of_data_directories: buf.read_u32::<LittleEndian>()?,
                directories: vec![
                    DirectoryEntry::read(DirectoryType::ExportTable, buf)?,
                    DirectoryEntry::read(DirectoryType::ImportTable, buf)?,
                    DirectoryEntry::read(DirectoryType::ResourceTable, buf)?,
                    DirectoryEntry::read(DirectoryType::ExceptionTable, buf)?,
                    DirectoryEntry::read(DirectoryType::CertificateTable, buf)?,
                    DirectoryEntry::read(DirectoryType::BaseRelocationTable, buf)?,
                    DirectoryEntry::read(DirectoryType::DebugData, buf)?,
                    DirectoryEntry::read(DirectoryType::CopyrightData, buf)?,
                    DirectoryEntry::read(DirectoryType::GlobalPtrData, buf)?,
                    DirectoryEntry::read(DirectoryType::TlsTable, buf)?,
                    DirectoryEntry::read(DirectoryType::LoadConfigTable, buf)?,
                    DirectoryEntry::read(DirectoryType::BoundImport, buf)?,
                    DirectoryEntry::read(DirectoryType::ImportAddressTable, buf)?,
                    DirectoryEntry::read(DirectoryType::DelayImportDescriptor, buf)?,
                    DirectoryEntry::read(DirectoryType::CliHeader, buf)?,
                    DirectoryEntry::read(DirectoryType::Reserved, buf)?,
                ],
            })
        }
    }

    pub fn directories(&self) -> &Vec<DirectoryEntry> {
        &self.directories
    }
}
