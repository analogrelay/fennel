mod access;
mod cli_header;
mod metadata_header;
mod cli_flags;
mod stream_header;
mod metadata_sizes;
mod type_attributes;
mod field_attributes;
mod method_attributes;
mod method_impl_attributes;
mod param_attributes;

pub mod tables;
pub mod signatures;
pub mod heaps;

pub use self::heaps::{BlobHandle, StringHandle, GuidHandle};
pub use self::access::Access;
pub use self::cli_header::CliHeader;
pub use self::metadata_header::MetadataHeader;
pub use self::cli_flags::CliFlags;
pub use self::stream_header::StreamHeader;
pub use self::type_attributes::{TypeAttributes, TypeFlags, TypeLayout, TypeSemantics,
                                TypeStringFormat, TypeVisibility};
pub use self::field_attributes::{FieldAttributes, FieldFlags};
pub use self::method_attributes::{MethodAttributes, MethodFlags, MethodVTableLayout};
pub use self::method_impl_attributes::{MethodCodeType, MethodImplAttributes, MethodImplFlags};
pub use self::param_attributes::ParamAttributes;
pub use self::metadata_sizes::{HeapSizes, MetadataSizes, LARGE_INDEX_SIZE, SMALL_INDEX_SIZE,
                               SMALL_TABLE_MAX_SIZE};
