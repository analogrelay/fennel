use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};

use crate::cli::tables::{TableIndex, TableMask};
use crate::error::Error;

pub const SMALL_TABLE_MAX_SIZE: usize = 0xFFFF;
pub const SMALL_INDEX_SIZE: usize = 2;
pub const LARGE_INDEX_SIZE: usize = 4;

bitflags! {
    #[derive(Clone, Copy)]
    pub struct HeapSizes: u8 {
        const LARGE_STRINGS = 0x01;
        const LARGE_GUIDS = 0x02;
        const LARGE_BLOBS = 0x04;
        const EXTRA_DATA = 0x40;
    }
}

impl std::fmt::Display for HeapSizes {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        self.iter_names().fold(String::new(), |mut acc, (name, _)| {
            if !acc.is_empty() {
                acc.push_str(" | ");
            }
            acc.push_str(name);
            acc
        }).fmt(f)
    }
}

pub struct MetadataSizes {
    heap_sizes: HeapSizes,
    row_counts: [usize; TableIndex::MAX + 1],
}

impl MetadataSizes {
    pub fn read<A: Read>(buf: &mut A) -> Result<MetadataSizes, Error> {
        // Skip reserved value, and version numbers
        buf.read_u32::<LittleEndian>()?;
        buf.read_u8()?;
        buf.read_u8()?;
        let heap_sizes = HeapSizes::from_bits_truncate(buf.read_u8()?);

        // Skip reserved value
        buf.read_u8()?;

        // Read valid and sorted vectors
        let valid_mask = TableMask::from_bits_truncate(buf.read_u64::<LittleEndian>()?);
        let _sorted_mask = TableMask::from_bits_truncate(buf.read_u64::<LittleEndian>()?);

        // Load row counts
        let mut row_counts = [0; TableIndex::MAX + 1];
        for idx in TableIndex::each() {
            if valid_mask.has_table(idx) {
                let size = buf.read_u32::<LittleEndian>()?;
                row_counts[idx as usize] = size as usize;
            }
        }

        Ok(MetadataSizes {
            heap_sizes: heap_sizes,
            row_counts: row_counts,
        })
    }

    pub fn heap_sizes(&self) -> HeapSizes {
        self.heap_sizes
    }

    pub fn row_count(&self, idx: TableIndex) -> usize {
        let idx = idx as usize;
        if idx > self.row_counts.len() {
            // Could panic here, but there may be tables we didn't expect to see.
            0
        } else {
            self.row_counts[idx as usize]
        }
    }

    pub fn index_size(&self, idx: TableIndex) -> usize {
        if self.row_count(idx) <= SMALL_TABLE_MAX_SIZE {
            SMALL_INDEX_SIZE
        } else {
            LARGE_INDEX_SIZE
        }
    }

    pub fn coded_index_size(&self, tables: TableMask) -> usize {
        let need_large_index = TableIndex::each()
            .filter(|&i| tables.has_table(i))
            .map(|i| self.index_size(i))
            .any(|i| i == LARGE_INDEX_SIZE);

        if need_large_index {
            LARGE_INDEX_SIZE
        } else {
            SMALL_INDEX_SIZE
        }
    }
}
