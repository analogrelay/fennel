use std::convert::Infallible;

use thiserror::Error;

/// Represents an error that occurs while loading PE/CIL metadata
#[derive(Error, Debug)]
pub enum Error {
    /// Indicates that an I/O error occurred.
    #[error("i/o error occurred")]
    IoError(#[from] ::std::io::Error),

    /// Indicates that the file has an invalid signature (MS-DOS Signature, PE Signature, etc.).
    #[error("invalid file signature")]
    InvalidSignature,

    /// Indicates that the file is not a PE file, and thus has no PE header.
    #[error("not a portable executable")]
    NotAPortableExecutable,

    /// The requested PE data directory was not found.
    #[error("data directory not found")]
    DirectoryNotFound,

    /// The requested section was not found.
    #[error("section not found")]
    SectionNotFound,

    /// The image does not contain a CLI header
    #[error("CLI header not found")]
    CliHeaderNotFound,

    /// The requested metadata stream was not found.
    #[error("metadata stream not found")]
    StreamNotFound,

    /// The metadata file is invalid in an unexpected way.
    #[error("invalid metadata: {0}")]
    InvalidMetadata(String),

    /// An invalid heap reference was provided.
    #[error("invalid heap reference")]
    InvalidHeapReference,

    /// The provided table name was not recognized
    #[error("unknown table name")]
    UnknownTableName,

    /// The Coded Index data was invalid
    #[error("invalid coded index")]
    InvalidCodedIndex,

    /// The type code is not recognized
    #[error("unknown type code: {0}")]
    UnknownTypeCode(u32),
}

impl From<Infallible> for Error {
    fn from(_: Infallible) -> Self {
        unreachable!()
    }
}