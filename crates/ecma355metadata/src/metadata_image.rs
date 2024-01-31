use std::ffi::CStr;
use std::io::{Cursor, Read};
use std::ops::Deref;

use tracing::trace;

use crate::cli::tables::{self, TableIndex};
use crate::pe::{DirectoryType, PeImage};
use crate::cli::{BlobHandle, CliHeader, GuidHandle, MetadataHeader, MetadataSizes, StringHandle};
use crate::cli::tables::{Table, TableRow, RowDecoder};
use crate::error::Error;
use crate::Guid;
use crate::cli::heaps::{Handle, Heaps};

pub struct MetadataImage<D: Deref<Target = [u8]> = Vec<u8>> {
    pe: PeImage<D>,
    cli_header: CliHeader,
    metadata_header: MetadataHeader,
    metadata_sizes: MetadataSizes,
    table_offsets: Vec<(TableIndex, usize)>,
    heaps: Heaps,
}

impl<D: Deref<Target = [u8]>> MetadataImage<D> {
    pub fn load_data(data: D) -> Result<MetadataImage<D>, Error> {
        MetadataImage::load(PeImage::load(data)?)
    }

    pub fn load(pe: PeImage<D>) -> Result<MetadataImage<D>, Error> {
        // Find the CLI header
        let cli_header_dir = pe.pe_header()
            .ok_or(Error::CliHeaderNotFound)?
            .directories()
            .iter()
            .find(|x| x.directory_type == DirectoryType::CliHeader)
            .ok_or(Error::CliHeaderNotFound)?;
        let cli_header_buf = &pe[cli_header_dir.range];
        let cli_header = CliHeader::read(cli_header_buf)?;

        let metadata_buf = &pe[cli_header.metadata];
        trace!(%cli_header.metadata, "cil metadata located");
        let metadata_header = MetadataHeader::read(Cursor::new(metadata_buf))?;

        let stream = metadata_header
            .get_stream("#~")
            .ok_or(Error::InvalidMetadata(
                "image does not contain a '#~' metadata stream",
            ))?;
        let stream_buf = &metadata_buf[stream.offset as usize..(stream.offset + stream.size) as usize];
        let mut cursor = Cursor::new(stream_buf);
        let metadata_sizes = MetadataSizes::read(&mut cursor)?;

        // Scan the image to find the offsets of each table
        let mut table_offsets = Vec::new();
        let mut table_base_rva = cli_header.metadata.start as usize + stream.offset as usize + cursor.position() as usize;
        {
            fn load_table<T: TableRow>(table_offsets: &mut Vec<(TableIndex, usize)>, offset: &mut usize, row_decoder: &RowDecoder) {
                let idx = T::INDEX;
                let row_count = row_decoder.row_count(idx);
                if row_count > 0 {
                    let row_size = T::row_size(row_decoder);
                    let table_size = row_count * row_size;
                    table_offsets.push((idx, *offset));
                    *offset = *offset + table_size;
                }
            }
            fn skip_table(row_decoder: &RowDecoder, idx: TableIndex) {
                let row_count = row_decoder.row_count(idx);
                if row_count > 0 {
                    // TODO: See if we can skip this safely?
                    // We haven't seen it in the wild yet, but we can't safely skip it without knowing it's sizes.
                    panic!("table {} is not supported", idx)
                }
            }

            let row_decoder = RowDecoder::new(&metadata_sizes);
            load_table::<tables::Module>(&mut table_offsets, &mut table_base_rva, &row_decoder);
            load_table::<tables::TypeRef>(&mut table_offsets, &mut table_base_rva, &row_decoder);
            load_table::<tables::TypeDef>(&mut table_offsets, &mut table_base_rva, &row_decoder);
            skip_table(&row_decoder, TableIndex::FieldPtr);
            load_table::<tables::Field>(&mut table_offsets, &mut table_base_rva, &row_decoder);
            skip_table(&row_decoder, TableIndex::MethodPtr);
            load_table::<tables::MethodDef>(&mut table_offsets, &mut table_base_rva, &row_decoder);
            skip_table(&row_decoder, TableIndex::ParamPtr);
            load_table::<tables::Param>(&mut table_offsets, &mut table_base_rva, &row_decoder);
            load_table::<tables::InterfaceImpl>(&mut table_offsets, &mut table_base_rva, &row_decoder);
        }

        // Find heap offsets
        let string_heap = metadata_header.get_stream("#Strings").map(|x| x.offset as usize);
        let userstring_heap = metadata_header.get_stream("#US").map(|x| x.offset as usize);
        let guid_heap = metadata_header.get_stream("#GUID").map(|x| x.offset as usize);
        let blob_heap = metadata_header.get_stream("#Blob").map(|x| x.offset as usize);

        Ok(MetadataImage {
            pe,
            cli_header,
            metadata_header,
            metadata_sizes,
            table_offsets,
            heaps: Heaps {
                string_heap,
                userstring_heap,
                guid_heap,
                blob_heap,
            },
        })
    }

    pub fn pe(&self) -> &PeImage<D> {
        &self.pe
    }

    pub fn cli_header(&self) -> &CliHeader {
        &self.cli_header
    }

    pub fn metadata_header(&self) -> &MetadataHeader {
        &self.metadata_header
    }

    pub fn row_count(&self, table_index: TableIndex) -> usize {
        self.metadata_sizes.row_count(table_index)
    }

    pub fn table<T: TableRow>(&self) -> Table<T> {
        let decoder = RowDecoder::new(&self.metadata_sizes);
        let buffer = match self.table_offsets.binary_search_by_key(&T::INDEX, |(index, _)| *index) {
            Ok(index) => {
                let start = self.table_offsets[index].1;
                let size = self.row_count(T::INDEX) * T::row_size(&decoder);
                &self.pe[start..(start + size)]
            },
            Err(_) => &[]
        };
        Table::new(buffer, decoder)
    }

    pub fn get_string(&self, handle: StringHandle) -> Option<&CStr> {
        let metadata_buf = &self.pe[self.cli_header.metadata];
        self.heaps.get_string(metadata_buf, handle)
    }

    pub fn get_guid(&self, handle: GuidHandle) -> Option<Guid> {
        let metadata_buf = &self.pe[self.cli_header.metadata];
        self.heaps.get_guid(metadata_buf, handle)
    }

    pub fn get_blob(&self, handle: BlobHandle) -> Option<&[u8]> {
        let metadata_buf = &self.pe[self.cli_header.metadata];
        self.heaps.get_blob(metadata_buf, handle)
    }
}

impl MetadataImage<Vec<u8>> {
    pub fn read<R: Read>(reader: R) -> Result<MetadataImage<Vec<u8>>, Error> {
        MetadataImage::load(PeImage::read(reader)?)
    }
}