use std::io::Read;

use crate::error::Error;
use crate::pe::MemoryRange;

#[derive(Debug, Eq, PartialEq)]
pub enum DirectoryType {
    ExportTable,
    ImportTable,
    ResourceTable,
    ExceptionTable,
    CertificateTable,
    BaseRelocationTable,
    DebugData,
    CopyrightData,
    GlobalPtrData,
    TlsTable,
    LoadConfigTable,
    BoundImport,
    ImportAddressTable,
    DelayImportDescriptor,
    CliHeader,
    Reserved,
}

impl ::std::fmt::Display for DirectoryType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        let s = match *self {
            DirectoryType::ExportTable => "Export Table",
            DirectoryType::ImportTable => "Import Table",
            DirectoryType::ResourceTable => "Resource Table",
            DirectoryType::ExceptionTable => "Exception Table",
            DirectoryType::CertificateTable => "Certificate Table",
            DirectoryType::BaseRelocationTable => "Base Relocation Table",
            DirectoryType::DebugData => "Debug Data",
            DirectoryType::CopyrightData => "Copyright Data",
            DirectoryType::GlobalPtrData => "Global Pointer Data",
            DirectoryType::TlsTable => "Thread Local Storage",
            DirectoryType::LoadConfigTable => "Loader Configuration Table",
            DirectoryType::BoundImport => "Bound Import Table",
            DirectoryType::ImportAddressTable => "Import Address Table",
            DirectoryType::DelayImportDescriptor => "Delay Import Descriptor",
            DirectoryType::CliHeader => "CLI Header",
            DirectoryType::Reserved => "Reserved",
        };

        f.write_str(s)
    }
}

pub struct DirectoryEntry {
    pub directory_type: DirectoryType,
    pub range: MemoryRange,
}

impl DirectoryEntry {
    pub fn new(directory_type: DirectoryType, range: MemoryRange) -> DirectoryEntry {
        DirectoryEntry {
            directory_type: directory_type,
            range: range,
        }
    }

    pub fn read<A: Read>(
        directory_type: DirectoryType,
        buf: &mut A,
    ) -> Result<DirectoryEntry, Error> {
        Ok(DirectoryEntry::new(directory_type, MemoryRange::read(buf)?))
    }
}

impl ::std::fmt::Display for DirectoryEntry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        write!(f, "{}: {}", self.range, self.directory_type)
    }
}
