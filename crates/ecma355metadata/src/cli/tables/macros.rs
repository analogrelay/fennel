#[macro_export]
macro_rules! coded_index {
    ($name: ident, [$($table: ident $(,)?)+]) => {
        pub struct $name($crate::cli::tables::table_handle::TableHandle);

        impl $name {
            const TABLE_COUNT: usize = coded_index!(@COUNT $($table),+);
            const SHIFT_DISTANCE: usize = 
                0usize.leading_zeros() as usize -
                (Self::TABLE_COUNT-1).leading_zeros() as usize;
            const TAG_MASK: usize = (1 << $name::SHIFT_DISTANCE) - 1;
            const TABLE_MASK: $crate::cli::tables::table_index::TableMask = 
                $crate::cli::tables::table_index::TableMask::from_bits_truncate(
                    $(
                        $crate::cli::tables::table_index::TableMask::$table.bits()
                    )|+
                );
            const TAG_MAP: [$crate::cli::tables::table_index::TableIndex; $name::TABLE_COUNT] = [
                $(
                    $crate::cli::tables::table_index::TableIndex::$table,
                )+
            ];
        }

        impl $crate::cli::tables::table_row::CodedIndex for $name {
            fn read(decoder: &$crate::cli::tables::table_row::RowDecoder, buf: &mut &[u8]) -> Result<$crate::cli::tables::table_handle::TableHandle, $crate::error::Error> {
                let index = if decoder.any_large($name::TABLE_MASK) {
                    decoder.decode_u32(buf)? as usize
                } else {
                    decoder.decode_u16(buf)? as usize
                };

                let tag = index & $name::TAG_MASK;
                let index = index >> $name::SHIFT_DISTANCE;
                let table = $name::TAG_MAP[tag];
                Ok($crate::cli::tables::table_handle::TableHandle::new(index, table))
            }

            fn size(decoder: &$crate::cli::tables::table_row::RowDecoder) -> usize {
                if decoder.any_large($name::TABLE_MASK) {
                    4
                } else {
                    2
                }
            }
        }
    };
    (@COUNT $table: ident) => {
        1usize
    };
    (@COUNT $table: ident, $($tables: ident),+) => {
        1usize + coded_index!(@COUNT $($tables),+)
    };
}

#[macro_export]
macro_rules! table_def {
    (@FIELD[$table: ident]) => {
        $crate::cli::tables::table_handle::TableHandle
    };
    (@FIELD($index: ident)) => {
        $crate::cli::tables::table_handle::TableHandle
    };
    (@FIELD $t: ident) => {
        $t
    };

    (@DECODE [$table: ident], $decoder: ident, $buf: ident) => {
        $decoder.decode_index($crate::cli::tables::table_index::TableIndex::$table, &mut $buf)?
    };
    (@SIZE [$table: ident], $decoder: ident) => {
        $decoder.size_of_index($crate::cli::tables::table_index::TableIndex::$table)
    };

    (@DECODE ($coded_index: ident), $decoder: ident, $buf: ident) => {
        <$coded_index as $crate::cli::tables::table_row::CodedIndex>::read($decoder, &mut $buf)?
    };
    (@SIZE ($coded_index: ident), $decoder: ident) => {
        <$coded_index as $crate::cli::tables::table_row::CodedIndex>::size($decoder)
    };

    (@DECODE $ty: ident as $from_ty: ident, $decoder: ident, $buf: ident) => {
        $ty::try_from(table_def!(@DECODE $from_ty, $decoder, $buf))?
    };
    (@SIZE $ty: ident as $from_ty: ident, $decoder: ident) => {
        table_def!(@SIZE $from_ty, $decoder)
    };

    (@DECODE u16, $decoder: ident, $buf: ident) => {
        $decoder.decode_u16(&mut $buf)?
    };
    (@SIZE u16, $decoder: ident) => {
        2
    };

    (@DECODE u32, $decoder: ident, $buf: ident) => {
        $decoder.decode_u32(&mut $buf)?
    };
    (@SIZE u32, $decoder: ident) => {
        4
    };

    (@DECODE u8, $decoder: ident, $buf: ident) => {
        $decoder.decode_u8(&mut $buf)?
    };
    (@SIZE u8, $decoder: ident) => {
        1
    };

    (@DECODE StringHandle, $decoder: ident, $buf: ident) => {
        $decoder.decode_string(&mut $buf)?
    };
    (@SIZE StringHandle, $decoder: ident) => {
        $decoder.size_of_string()
    };

    (@DECODE GuidHandle, $decoder: ident, $buf: ident) => {
        $decoder.decode_guid(&mut $buf)?
    };
    (@SIZE GuidHandle, $decoder: ident) => {
        $decoder.size_of_guid()
    };

    (@DECODE BlobHandle, $decoder: ident, $buf: ident) => {
        $decoder.decode_blob(&mut $buf)?
    };
    (@SIZE BlobHandle, $decoder: ident) => {
        $decoder.size_of_blob()
    };

    (@DECODE $ty: ident, $decoder: ident, $buf: ident) => {
        compile_error!("Unsupported column type");
    };
    (@SIZE $ty: ident, $decoder: ident) => {
        compile_error!("Unsupported column type");
    };

    (
        $ty: ident, 
        [
            $(
                $col_name: ident : $col_ty: tt $(as $col_from_type: ident)?,
            )+
        ]
    ) => {
        pub struct $ty {
            $(
                pub $col_name : table_def!(@FIELD $col_ty),
            )+
        }

        impl $crate::cli::tables::table_row::TableRow for $ty {
            const INDEX: $crate::cli::tables::table_index::TableIndex = $crate::cli::tables::table_index::TableIndex::$ty;

            fn decode(decoder: &$crate::cli::tables::table_row::RowDecoder, mut buf: &[u8]) -> std::result::Result<Self, $crate::error::Error> {
                $(
                    let $col_name = table_def!(@DECODE $col_ty $(as $col_from_type)?, decoder, buf);
                )+
                Ok($ty {
                    $($col_name),+
                })
            }

            fn row_size(decoder: &$crate::cli::tables::table_row::RowDecoder) -> usize {
                let mut size = 0;
                $(
                size += table_def!(@SIZE $col_ty $(as $col_from_type)?, decoder);
                )+
                size
            }
        }
    };
}