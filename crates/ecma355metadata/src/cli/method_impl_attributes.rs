// We want MethodAttributes to use the same names as in the ECMA spec, which are PascalCased, not UPPER_SNAKE_CASE
#![allow(non_upper_case_globals)]

use std::mem;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct MethodImplAttributes(u16);

impl MethodImplAttributes {
    pub fn new(value: u16) -> MethodImplAttributes {
        MethodImplAttributes(value)
    }

    pub fn code_type(self) -> MethodCodeType {
        unsafe {
            mem::transmute((self.0 & MethodCodeType::MASK) >> MethodCodeType::SHIFT)
        }
    }

    pub fn flags(self) -> MethodImplFlags {
        MethodImplFlags::from_bits_truncate(self.0 & FLAGS_MASK)
    }
}

impl std::convert::From<u16> for MethodImplAttributes {
    fn from(value: u16) -> MethodImplAttributes {
        MethodImplAttributes::new(value)
    }
}

impl std::fmt::Display for MethodImplAttributes {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        write!(f, "{}", self.code_type())?;
        if !self.flags().is_empty() {
            write!(f, " [{}]", self.flags())?;
        }
        Ok(())
    }
}

#[repr(u16)]
#[derive(Debug, PartialEq, Eq)]
pub enum MethodCodeType {
    IL = 0,
    Native = 1,
    OPTIL = 2,
    Runtime = 3,
}
impl_display_via_debug!(MethodCodeType);

impl MethodCodeType {
    const MASK: u16 = 0x0003;
    const SHIFT: u16 = 0;
}

const FLAGS_MASK: u16 = !(MethodCodeType::MASK);

bitflags! {
    pub struct MethodImplFlags : u16 {
        const Unmanaged = 0x0004;
        const NoInlining = 0x0008;
        const ForwardRef = 0x0010;
        const PreserveSig = 0x0080;
        const InternalCall = 0x1000;
        const Synchronized = 0x0020;
        const NoOptimization = 0x0040;
    }
}

impl std::fmt::Display for MethodImplFlags {
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