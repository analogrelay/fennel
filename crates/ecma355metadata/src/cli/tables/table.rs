use crate::cli::tables::table_row::{RowDecoder, TableRow};
use crate::Error;

pub struct Table<'buffer, 'decoder, T: TableRow> {
    buffer: &'buffer [u8],
    decoder: RowDecoder<'decoder>,
    row_count: usize,
    row_size: usize,
    _phantom: std::marker::PhantomData<T>,
}

struct TableIterator<'table, 'buffer, 'decoder, T: TableRow> {
    table: &'table Table<'buffer, 'decoder, T>,
    index: usize,
}

impl<'buffer, 'decoder, T: TableRow> Table<'buffer, 'decoder, T> {
    pub fn new(buffer: &'buffer [u8], decoder: RowDecoder<'decoder>) -> Table<'buffer, 'decoder, T> {
        let row_count = decoder.row_count(T::INDEX);
        let row_size = T::row_size(&decoder);
        Table {
            buffer,
            decoder,
            row_count,
            row_size,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn len(&self) -> usize { self.row_count }

    pub fn read(&self, index: usize) -> Result<T, Error> {
        if index > self.row_count {
            panic!("row index {} exceeds table size {}", index, self.row_count);
        }
        let offset = index * self.row_size;
        let row = &self.buffer.as_ref()[offset..(offset + self.row_size)];
        T::decode(&self.decoder, row)
    }

    pub fn iter<'b>(&'b self) -> impl Iterator<Item = Result<T, Error>> + 'b {
        TableIterator {
            table: self,
            index: 0,
        }
    }
}

impl<'table, 'buffer, 'decoder, T: TableRow> Iterator for TableIterator<'table, 'buffer, 'decoder, T> {
    type Item = Result<T, Error>;

    fn next(&mut self) -> Option<Result<T, Error>> {
        if self.index >= self.table.row_count {
            None
        } else {
            let result = self.table.read(self.index);
            self.index += 1;
            Some(result)
        }
    }
}