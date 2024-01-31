// Macros go first because they are expanded in one pass, top-down.
macro_rules! impl_display_via_debug {
    ($typ:ty) => {
        impl ::std::fmt::Display for $typ {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
                write!(f, "{:?}", self)
            }
        }
    };
}

extern crate byteorder;

#[macro_use]
extern crate bitflags;

mod error;
mod utils;
mod guid;
mod metadata_image;

/// Contains CLI metadata structures
pub mod cli;

/// Contains PE structures
pub mod pe;

pub use error::Error;

pub use pe::PeImage;
pub use cli::CliHeader;
pub use guid::Guid;
pub use metadata_image::MetadataImage;
