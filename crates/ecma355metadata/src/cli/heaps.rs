use std::ffi::CStr;
use std::fmt::Display;
use std::io::Read;
use std::marker::PhantomData;

use byteorder::{LittleEndian, ReadBytesExt};

use crate::cli::{HeapSizes, MetadataSizes};
use crate::{Error, Guid};

pub trait Handle {
    const SIZE_FLAG: HeapSizes;

    fn new(offset: usize) -> Self;
}

#[derive(Clone, Copy)]
pub struct StringHandle(pub usize);

impl Display for StringHandle {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        write!(f, "(S)0x{:08X}", self.0)
    }
}

#[derive(Clone, Copy)]
pub struct GuidHandle(pub usize);

impl Display for GuidHandle {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        write!(f, "(G)0x{:08X}", self.0)
    }
}

#[derive(Clone, Copy)]
pub struct BlobHandle(pub usize);

impl Display for BlobHandle {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        write!(f, "(B)0x{:08X}", self.0)
    }
}

pub struct HandleReader<T: Handle>(bool, PhantomData<T>);

impl<T: Handle> HandleReader<T> {
    pub fn new(sizes: &MetadataSizes) -> HandleReader<T> {
        HandleReader(sizes.heap_sizes().contains(T::SIZE_FLAG), PhantomData)
    }

    pub fn read(&self, mut buf: impl Read) -> Result<T, Error> {
        if self.0 {
            Ok(T::new(buf.read_u32::<LittleEndian>()? as usize))
        } else {
            Ok(T::new(buf.read_u16::<LittleEndian>()? as usize))
        }
    }

    pub fn size(&self) -> usize {
        if self.0 {
            4
        } else {
            2
        }
    }
}

pub struct Heaps {
    pub string_heap: Option<usize>,
    pub userstring_heap: Option<usize>,
    pub guid_heap: Option<usize>,
    pub blob_heap: Option<usize>,
}

impl Heaps {
    pub fn get_string<'a>(&self, metadata_buf: &'a [u8], handle: StringHandle) -> Option<&'a CStr> {
        match self.string_heap {
            Some(offset) => {
                handle.read(&metadata_buf[offset..])
            },
            None => None,
        }
    }

    pub fn get_guid(&self, metadata_buf: &[u8], handle: GuidHandle) -> Option<Guid> {
        match self.guid_heap {
            Some(offset) => {
                handle.read(&metadata_buf[offset..])
            },
            None => None,
        }
    }

    pub fn get_blob<'a>(&self, metadata_buf: &'a [u8], handle: BlobHandle) -> Option<&'a [u8]> {
        match self.blob_heap {
            Some(offset) => {
                handle.read(&metadata_buf[offset..])
            },
            None => None,
        }
    }
}

impl Handle for StringHandle {
    const SIZE_FLAG: HeapSizes = HeapSizes::LARGE_STRINGS;

    fn new(offset: usize) -> Self { StringHandle(offset) }
}

impl StringHandle {
    pub fn read<'a>(&self, buf: &'a [u8]) -> Option<&'a CStr> {
        if self.0 > buf.len() {
            None
        } else {
            CStr::from_bytes_until_nul(&buf[self.0..]).ok()
        }
    }
}

impl Handle for GuidHandle {
    const SIZE_FLAG: HeapSizes = HeapSizes::LARGE_GUIDS;

    fn new(offset: usize) -> Self { GuidHandle(offset) }
}

impl GuidHandle {
    fn read(&self, buf: &[u8]) -> Option<Guid> {
        // Bounds check
        let start = self.0;
        let end = self.0 + 16;
        if end > buf.len() {
            return None;
        }

        let buf = &buf[start..end];
        Some(Guid::from_bytes(buf))
    }
}

impl Handle for BlobHandle {
    const SIZE_FLAG: HeapSizes = HeapSizes::LARGE_BLOBS;

    fn new(offset: usize) -> Self { BlobHandle(offset) }
}

impl BlobHandle {
    fn read<'a>(&self, buf: &'a [u8]) -> Option<&'a [u8]> {
        // Bounds check
        if self.0 == 0 || self.0 >= buf.len() {
            None
        } else {
            // Read the header
            if buf[self.0] & 0x80 == 0 {
                // 1-byte length
                let start = self.0 + 1;
                let len = (buf[self.0] as usize) & 0x7F;
                Some(&buf[start..(start + len)])
            } else if buf[self.0] & 0xC0 == 0 {
                // 2-byte length
                let start = self.0 + 2;
                let len = ((buf[self.0] as usize & 0x3F) << 8) + buf[self.0 + 1] as usize;
                Some(&buf[start..(start + len)])
            } else {
                // 4-byte length
                let start = self.0 + 4;
                let len = ((buf[self.0] as usize) & 0x1F << 24) + ((buf[self.0 + 1] as usize) << 16)
                    + ((buf[self.0 + 2] as usize) << 8)
                    + buf[self.0 + 3] as usize;
                Some(&buf[start..(start + len)])
            }
        }
    }
}
