// Based on:
// https://github.com/dotnet/corefx/blob/master/src/System.Reflection.Metadata/src/System/Reflection/Metadata/Signatures/SignatureHeader.cs

use std::mem;

const CONV_OR_KIND_MASK: u8 = 0x0F;
const MAX_CALLING_CONVENTION: u8 = SignatureCallingConvention::VarArgs as u8;
const MAX_HEADER_VALUE: u8 = SignatureKind::MethodSpecification as u8;

#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum SignatureCallingConvention {
    Default = 0x00,
    CDecl = 0x01,
    StdCall = 0x02,
    ThisCall = 0x03,
    FastCall = 0x04,
    VarArgs = 0x05,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum SignatureKind {
    Method = 0x00,
    Field = 0x06,
    LocalVariables = 0x07,
    Property = 0x08,
    MethodSpecification = 0x0A,
}

bitflags! {
    #[derive(Debug, PartialEq, Eq)]
    pub struct SignatureAttributes : u8 {
        const GENERIC = 0x10;
        const HAS_THIS = 0x20;
        const EXPLICIT_THIS = 0x40;
    }
}

// The header value indicates the calling convention, if the signature refers to a method,
// using values 0x0-0x5, OR the type of the signature if it isn't a method (values 0x6-0xA)

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SignatureHeader(u8);

impl SignatureHeader {
    pub fn new(value: u8) -> SignatureHeader {
        debug_assert!(value & CONV_OR_KIND_MASK <= MAX_HEADER_VALUE);
        assert!((value & CONV_OR_KIND_MASK) != 0x09, "Signature Header contains invalid value");

        SignatureHeader(value)
    }

    pub fn is_generic(self) -> bool {
        self.attributes().contains(SignatureAttributes::GENERIC)
    }

    pub fn has_this(self) -> bool {
        self.attributes().contains(SignatureAttributes::HAS_THIS)
    }

    pub fn explicit_this(self) -> bool {
        self.attributes().contains(SignatureAttributes::EXPLICIT_THIS)
    }

    pub fn kind(self) -> SignatureKind {
        let calling_convention_or_kind = self.0 & CONV_OR_KIND_MASK;

        if calling_convention_or_kind <= MAX_CALLING_CONVENTION {
            // The value is within the range of calling conventions,
            // so it's a method
            SignatureKind::Method
        } else {
            // The value is outside the range of calling conventions,
            // so it indicates the kind.
            unsafe { mem::transmute(calling_convention_or_kind) }
        }
    }

    pub fn calling_convention(self) -> SignatureCallingConvention {
        let calling_convention_or_kind = self.0 & CONV_OR_KIND_MASK;

        if calling_convention_or_kind > MAX_CALLING_CONVENTION {
            // The value is within the range of kinds,
            // so it's a default calling convention
            SignatureCallingConvention::Default
        } else {
            // The value is outside the range of kinds,
            // so it indicates the calling convention.
            unsafe { mem::transmute(calling_convention_or_kind) }
        }
    }

    pub fn attributes(self) -> SignatureAttributes {
        SignatureAttributes::from_bits_truncate(self.0 & !CONV_OR_KIND_MASK)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! header_tests {
        ($($name:ident: ($value:expr, $kind: expr, $cconv: expr, $attr: expr);)+) => {
            $(
                #[test]
                pub fn $name() {
                    let v = SignatureHeader::new($value);
                    assert_eq!($kind, v.kind());
                    assert_eq!($cconv, v.calling_convention());
                    assert_eq!($attr, v.attributes());
                }
            )+
        };
    }

    header_tests! {
        method_sig_default: (0x00, SignatureKind::Method, SignatureCallingConvention::Default, SignatureAttributes::empty());
        method_sig_cdecl: (0x01, SignatureKind::Method, SignatureCallingConvention::CDecl, SignatureAttributes::empty());
        method_sig_stdcall: (0x02, SignatureKind::Method, SignatureCallingConvention::StdCall, SignatureAttributes::empty());
        method_sig_thiscall: (0x03, SignatureKind::Method, SignatureCallingConvention::ThisCall, SignatureAttributes::empty());
        method_sig_fastcall: (0x04, SignatureKind::Method, SignatureCallingConvention::FastCall, SignatureAttributes::empty());
        method_sig_varargs: (0x05, SignatureKind::Method, SignatureCallingConvention::VarArgs, SignatureAttributes::empty());
        field_sig: (0x06, SignatureKind::Field, SignatureCallingConvention::Default, SignatureAttributes::empty());
        local_sig: (0x07, SignatureKind::LocalVariables, SignatureCallingConvention::Default, SignatureAttributes::empty());
        property_sig: (0x08, SignatureKind::Property, SignatureCallingConvention::Default, SignatureAttributes::empty());
        method_spec_sig: (0x0A, SignatureKind::MethodSpecification, SignatureCallingConvention::Default, SignatureAttributes::empty());
        method_sig_varargs_generic: (0x15, SignatureKind::Method, SignatureCallingConvention::VarArgs, SignatureAttributes::GENERIC);
        method_sig_cdecl_has_this: (0x21, SignatureKind::Method, SignatureCallingConvention::CDecl, SignatureAttributes::HAS_THIS);
        method_sig_fastcall_explicit_this: (0x44, SignatureKind::Method, SignatureCallingConvention::FastCall, SignatureAttributes::EXPLICIT_THIS);
        method_sig_stdcall_everything: (0x72, SignatureKind::Method, SignatureCallingConvention::StdCall, SignatureAttributes::GENERIC | SignatureAttributes::HAS_THIS | SignatureAttributes::EXPLICIT_THIS);
    }
}