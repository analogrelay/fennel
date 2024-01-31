use byteorder::{ByteOrder, LittleEndian};

use crate::cli::tables::{TableHandle, TableIndex, TableMask};
use crate::cli::{BlobHandle, GuidHandle, HeapSizes, MetadataSizes, StringHandle};
use crate::Error;

pub trait TableRow: Sized {
    const INDEX: TableIndex;

    fn decode(decoder: &RowDecoder, buf: &[u8]) -> Result<Self, Error>;
    fn row_size(decoder: &RowDecoder) -> usize;
}

pub trait CodedIndex: Sized {
    fn read(decoder: &RowDecoder, buf: &mut &[u8]) -> Result<TableHandle, Error>;
    fn size(decoder: &RowDecoder) -> usize;
}

pub struct RowDecoder<'a> {
    metadata_sizes: &'a MetadataSizes,
}

impl<'a> RowDecoder<'a> {
    pub fn new(metadata_sizes: &'a MetadataSizes) -> RowDecoder<'a> {
        RowDecoder {
            metadata_sizes,
        }
    }

    pub fn row_count(&self, table_index: TableIndex) -> usize {
        self.metadata_sizes.row_count(table_index)
    }

    pub fn decode_u16(&self, buf: &mut &[u8]) -> Result<u16, Error> {
        let val = LittleEndian::read_u16(buf);
        *buf = &buf[2..];
        Ok(val)
    }

    pub fn decode_u32(&self, buf: &mut &[u8]) -> Result<u32, Error> {
        let val = LittleEndian::read_u32(buf);
        *buf = &buf[4..];
        Ok(val)
    }

    pub fn decode_string(&self, buf: &mut &[u8]) -> Result<StringHandle, Error> {
        if self.metadata_sizes.heap_sizes().contains(HeapSizes::LARGE_STRINGS) {
            Ok(StringHandle(self.decode_u32(buf)? as usize))
        } else {
            Ok(StringHandle(self.decode_u16(buf)? as usize))
        }
    }

    pub fn size_of_string(&self) -> usize {
        if self.metadata_sizes.heap_sizes().contains(HeapSizes::LARGE_STRINGS) {
            4
        } else {
            2
        }
    }

    pub fn decode_guid(&self, buf: &mut &[u8]) -> Result<GuidHandle, Error> {
        if self.metadata_sizes.heap_sizes().contains(HeapSizes::LARGE_STRINGS) {
            Ok(GuidHandle(self.decode_u32(buf)? as usize))
        } else {
            Ok(GuidHandle(self.decode_u16(buf)? as usize))
        }
    }

    pub fn size_of_guid(&self) -> usize {
        if self.metadata_sizes.heap_sizes().contains(HeapSizes::LARGE_GUIDS) {
            4
        } else {
            2
        }
    }

    pub fn decode_blob(&self, buf: &mut &[u8]) -> Result<BlobHandle, Error> {
        if self.metadata_sizes.heap_sizes().contains(HeapSizes::LARGE_STRINGS) {
            Ok(BlobHandle(self.decode_u32(buf)? as usize))
        } else {
            Ok(BlobHandle(self.decode_u16(buf)? as usize))
        }
    }

    pub fn size_of_blob(&self) -> usize {
        if self.metadata_sizes.heap_sizes().contains(HeapSizes::LARGE_BLOBS) {
            4
        } else {
            2
        }
    }
    
    pub fn decode_index(&self, table: TableIndex, buf: &mut &[u8]) -> Result<TableHandle, Error> {
        if self.has_large_index(table) {
            Ok(TableHandle::new(self.decode_u32(buf)? as usize, table))
        } else {
            Ok(TableHandle::new(self.decode_u16(buf)? as usize, table))
        }
    }
    
    pub fn size_of_index(&self, table: TableIndex) -> usize {
        if self.has_large_index(table) {
            4
        } else {
            2
        }
    }

    pub fn any_large(&self, mask: TableMask) -> bool {
        for table in mask.tables() {
            if self.has_large_index(table) {
                tracing::trace!(table = ?table, "has large index");
                return true;
            }
        }
        false
    }
    
    fn has_large_index(&self, table: TableIndex) -> bool {
        self.row_count(table) >= u16::MAX as usize
    }
}
