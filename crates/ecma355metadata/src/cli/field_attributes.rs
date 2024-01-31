// We want FieldAttributes to use the same names as in the ECMA spec, which are PascalCased, not UPPER_SNAKE_CASE
#![allow(non_upper_case_globals)]

use std::convert::Infallible;
use std::mem;

use crate::cli::Access;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct FieldAttributes(u16);

impl FieldAttributes {
    pub fn new(value: u16) -> FieldAttributes {
        FieldAttributes(value)
    }

    pub fn access(self) -> Access {
        unsafe {
            mem::transmute((self.0 & Access::MASK) >> Access::SHIFT)
        }
    }

    pub fn flags(self) -> FieldFlags {
        FieldFlags::from_bits_truncate(self.0 & FLAGS_MASK)
    }
}

impl std::convert::From<u16> for FieldAttributes {
    fn from(value: u16) -> FieldAttributes {
        FieldAttributes::new(value)
    }
}

impl std::fmt::Display for FieldAttributes {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        write!(f, "{}", self.access())?;
        if !self.flags().is_empty() {
            write!(f, " [{}]", self.flags())?;
        }
        Ok(())
    }
}

const FLAGS_MASK: u16 = !(Access::MASK);

bitflags! {
    pub struct FieldFlags : u16 {
        const Static = 0x0010;
        const InitOnly = 0x0020;
        const Literal = 0x0040;
        const NotSerialized = 0x0080;
        const HasFieldRVA = 0x0100;
        const SpecialName = 0x0200;
        const RTSpecialName = 0x0400;
        const HasFieldMarshal = 0x1000;
        const HasDefault = 0x8000;
    }
}

impl std::fmt::Display for FieldFlags {
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