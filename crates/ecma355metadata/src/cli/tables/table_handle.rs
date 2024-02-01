use std::fmt;

use crate::cli::tables::table_index::TableIndex;

#[derive(Debug, PartialEq, Eq)]
pub struct TableHandle {
    index: usize,
    table: TableIndex,
}

impl TableHandle {
    pub fn new(index: usize, table: TableIndex) -> TableHandle {
        TableHandle {
            index: index,
            table: table,
        }
    }

    pub fn from_metadata_token(token: u32) -> TableHandle {
        let table = (token >> 24) as u8;
        let index = token & 0x00FFFFFF;
        TableHandle::new(index as usize, table.into())
    }

    /// Gets the table index of the handle.
    pub fn table(&self) -> TableIndex {
        self.table
    }

    /// Gets the 1-based index of the row in the table.
    pub fn index(&self) -> usize {
        self.index
    }
}

impl fmt::Display for TableHandle {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}[0x{:04X}]", self.table, self.index)
    }
}