// We want MethodAttributes to use the same names as in the ECMA spec, which are PascalCased, not UPPER_SNAKE_CASE
#![allow(non_upper_case_globals)]

use std::convert::Infallible;
use std::mem;

use crate::cli::Access;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct MethodAttributes(u16);

impl MethodAttributes {
    pub fn new(value: u16) -> MethodAttributes {
        MethodAttributes(value)
    }

    pub fn access(self) -> Access {
        unsafe {
            mem::transmute((self.0 & Access::MASK) >> Access::SHIFT)
        }
    }

    pub fn vtable_layout(self) -> MethodVTableLayout {
        unsafe {
            mem::transmute((self.0 & MethodVTableLayout::MASK) >> MethodVTableLayout::SHIFT)
        }
    }

    pub fn flags(self) -> MethodFlags {
        MethodFlags::from_bits_truncate(self.0 & FLAGS_MASK)
    }
}

impl std::convert::TryFrom<u16> for MethodAttributes {
    type Error = Infallible;

    fn try_from(value: u16) -> Result<MethodAttributes, Infallible> {
        Ok(MethodAttributes::new(value))
    }
}

impl std::fmt::Display for MethodAttributes {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        write!(f, "{} {}", self.access(), self.vtable_layout())?;
        if !self.flags().is_empty() {
            write!(f, " [{}]", self.flags())?;
        }
        Ok(())
    }
}

#[repr(u16)]
#[derive(Debug, PartialEq, Eq)]
pub enum MethodVTableLayout {
    ReuseSlot = 0,
    NewSlot = 1,
}
impl_display_via_debug!(MethodVTableLayout);

impl MethodVTableLayout {
    const MASK: u16 = 0x0100;
    const SHIFT: u16 = 8;
}

const FLAGS_MASK: u16 = !(Access::MASK | MethodVTableLayout::MASK);

bitflags! {
    pub struct MethodFlags : u16 {
        const UnmanagedExport = 0x0008;
        const Static = 0x0010;
        const Final = 0x0020;
        const Virtual = 0x0040;
        const HideBySig = 0x0080;
        const Strict = 0x0200;
        const Abstract = 0x0400;
        const SpecialName = 0x0800;
        const RTSpecialName = 0x1000;
        const PInvokeImpl = 0x2000;
        const HasSecurity = 0x4000;
        const RequireSecObject = 0x8000;
    }
}

impl std::fmt::Display for MethodFlags {
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