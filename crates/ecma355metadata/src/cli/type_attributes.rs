// We want TypeAttributes to use the same names as in the ECMA spec, which are PascalCased, not UPPER_SNAKE_CASE
#![allow(non_upper_case_globals)]

use std::convert::Infallible;
use std::mem;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct TypeAttributes(u32);

impl TypeAttributes {
    pub fn new(value: u32) -> TypeAttributes {
        TypeAttributes(value)
    }

    pub fn visibility(self) -> TypeVisibility {
        unsafe {
            mem::transmute((self.0 & TypeVisibility::MASK) >> TypeVisibility::SHIFT)
        }
    }

    pub fn layout(self) -> TypeLayout {
        unsafe {
            mem::transmute((self.0 & TypeLayout::MASK) >> TypeLayout::SHIFT)
        }
    }

    pub fn semantics(self) -> TypeSemantics {
        unsafe {
            mem::transmute((self.0 & TypeSemantics::MASK) >> TypeSemantics::SHIFT)
        }
    }

    pub fn string_format(self) -> TypeStringFormat {
        unsafe {
            mem::transmute((self.0 & TypeStringFormat::MASK) >> TypeStringFormat::SHIFT)
        }
    }

    pub fn flags(self) -> TypeFlags {
        TypeFlags::from_bits_truncate(self.0 & FLAGS_MASK)
    }
}

impl std::convert::TryFrom<u32> for TypeAttributes {
    type Error = Infallible;

    fn try_from(value: u32) -> Result<TypeAttributes, Infallible> {
        Ok(TypeAttributes::new(value))
    }
}

impl std::fmt::Display for TypeAttributes {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        if self.visibility() != TypeVisibility::NotPublic {
            write!(f, "{} ", self.visibility())?;
        }

        if self.string_format() != TypeStringFormat::Auto {
            write!(f, "{} ", self.string_format())?;
        }

        if self.layout() != TypeLayout::AutoLayout {
            write!(f, "{} ", self.visibility())?;
        }

        write!(f, "{}", self.semantics())?;

        let flags = self.flags();
        if !flags.is_empty() {
            write!(f, " ({})", flags)?;
        }
        Ok(())
    }
}

#[repr(u32)]
#[derive(Debug, PartialEq, Eq)]
pub enum TypeVisibility {
    NotPublic = 0,
    Public = 1,
    NestedPublic = 2,
    NestedPrivate = 3,
    NestedFamily = 4,
    NestedAssembly = 5,
    NestedFamAndAssem = 6,
    NestedFamOrAssem = 7,
}
impl_display_via_debug!(TypeVisibility);

impl TypeVisibility {
    const MASK: u32 = 0x07;
    const SHIFT: usize = 0x00;
}

#[repr(u32)]
#[derive(Debug, PartialEq, Eq)]
pub enum TypeLayout {
    AutoLayout = 0,
    SequentialLayout = 1,
    ExplicitLayout = 2,
}
impl_display_via_debug!(TypeLayout);

impl TypeLayout {
    const MASK: u32 = 0x18;
    const SHIFT: usize = 0x03;
}

#[repr(u32)]
#[derive(Debug, PartialEq, Eq)]
pub enum TypeSemantics {
    Class = 0,
    Interface = 1,
}
impl_display_via_debug!(TypeSemantics);

impl TypeSemantics {
    const MASK: u32 = 0x20;
    const SHIFT: usize = 0x05;
}

#[repr(u32)]
#[derive(Debug, PartialEq, Eq)]
pub enum TypeStringFormat {
    Ansi = 0,
    Unicode = 1,
    Auto = 2,
    Custom = 3,
}
impl_display_via_debug!(TypeStringFormat);

impl TypeStringFormat {
    const MASK: u32 = 0x30000;
    const SHIFT: usize = 0x10;
}

const FLAGS_MASK: u32 = !(TypeVisibility::MASK | TypeLayout::MASK | TypeSemantics::MASK | TypeStringFormat::MASK);

bitflags! {
    pub struct TypeFlags: u32 {
        const Abstract = 0x80;
        const Sealed = 0x100;
        const SpecialName = 0x400;
        const Import = 0x1000;
        const Serializable = 0x2000;
        const BeforeFieldInit = 0x00100000;
        const RTSpecialName = 0x00000800;
        const HasSecurity = 0x00040000;
        const IsTypeForwarder = 0x00200000;
    }
}

impl std::fmt::Display for TypeFlags {
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