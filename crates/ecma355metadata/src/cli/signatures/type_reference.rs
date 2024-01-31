use std::fmt;
use std::io::Read;

use crate::cli::tables::TableHandle;
use crate::cli::signatures::{CustomModifier, MethodSignature};
use crate::cli::signatures::utils;
use crate::error::Error;

#[derive(Debug, PartialEq, Eq)]
pub struct ArrayShape {
    pub rank: u32,
    pub sizes: Vec<u32>,
    pub lo_bounds: Vec<u32>,
}

impl ArrayShape {
    pub fn new(rank: u32, sizes: Vec<u32>, lo_bounds: Vec<u32>) -> ArrayShape {
        ArrayShape {
            rank,
            sizes,
            lo_bounds,
        }
    }

    pub fn read<R: Read>(reader: &mut R) -> Result<ArrayShape, Error> {
        let rank = utils::read_compressed_u32(reader)?;
        let num_sizes = utils::read_compressed_u32(reader)?;
        let mut sizes = Vec::with_capacity(num_sizes as usize);
        for _ in 0..num_sizes {
            sizes.push(utils::read_compressed_u32(reader)?);
        }
        let num_lo_bounds = utils::read_compressed_u32(reader)?;
        let mut lo_bounds = Vec::with_capacity(num_lo_bounds as usize);
        for _ in 0..num_lo_bounds {
            lo_bounds.push(utils::read_compressed_u32(reader)?);
        }

        Ok(ArrayShape::new(rank, sizes, lo_bounds))
    }
}

impl fmt::Display for ArrayShape {
    fn fmt(&self, _f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        unimplemented!()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TypeReference {
    End,
    Void,
    Boolean,
    Char,
    I1,
    U1,
    I2,
    U2,
    I4,
    U4,
    I8,
    U8,
    R4,
    R8,
    String,
    Ptr(Vec<CustomModifier>, Box<TypeReference>),
    ByRef(Box<TypeReference>),
    ValueType(TableHandle),
    Class(TableHandle),
    Var(u32),
    Array(Box<TypeReference>, ArrayShape),
    GenericInst(Box<TypeReference>, Vec<TypeReference>),
    TypedByRef,
    I,
    U,
    FnPtr(Box<MethodSignature>),
    Object,
    SzArray(Vec<CustomModifier>, Box<TypeReference>),
    MVar(u32),
    Sentinel,
}

impl TypeReference {
    pub fn read<R: Read>(reader: &mut R) -> Result<TypeReference, Error> {
        utils::read_type(utils::read_compressed_u32(reader)?, reader)
    }
}

impl fmt::Display for TypeReference {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            TypeReference::End => write!(f, "!"),
            TypeReference::Void => write!(f, "void"),
            TypeReference::Boolean => write!(f, "boolean"),
            TypeReference::Char => write!(f, "char"),
            TypeReference::I1 => write!(f, "int8"),
            TypeReference::U1 => write!(f, "unsigned int8"),
            TypeReference::I2 => write!(f, "int16"),
            TypeReference::U2 => write!(f, "unsigned int16"),
            TypeReference::I4 => write!(f, "int32"),
            TypeReference::U4 => write!(f, "unsigned int32"),
            TypeReference::I8 => write!(f, "int64"),
            TypeReference::U8 => write!(f, "unsigned int64"),
            TypeReference::R4 => write!(f, "float32"),
            TypeReference::R8 => write!(f, "float64"),
            TypeReference::TypedByRef => write!(f, "TypedReference"),
            TypeReference::I => write!(f, "native int"),
            TypeReference::U => write!(f, "native unsigned int"),
            TypeReference::Object => write!(f, "object"),
            TypeReference::Sentinel => write!(f, "..."),
            TypeReference::String => write!(f, "string"),
            TypeReference::Ptr(ref modifiers, ref inner) => {
                write_list!(f, modifiers, " ");
                write!(f, "*{}", inner)
            },
            TypeReference::ByRef(ref inner) => write!(f, "ref {}", inner),
            TypeReference::ValueType(ref handle) => write!(f, "struct({})", handle),
            TypeReference::Class(ref handle) => write!(f, "class({})", handle),
            TypeReference::Var(idx) => write!(f, "!!{}", idx),
            TypeReference::MVar(idx) => write!(f, "!!{}", idx),
            TypeReference::Array(ref inner, ref shape) => write!(f, "{}{}", inner, shape),
            TypeReference::GenericInst(ref inner, ref types) => {
                write!(f, "{}<", inner)?;
                let mut first = true;
                for typ in types {
                    if first {
                        first = false;
                    } else {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", typ)?;
                }
                write!(f, ">")
            },
            TypeReference::SzArray(ref modifiers, ref inner) => {
                write_list!(f, modifiers, " ");
                write!(f, "{}[]", inner)
            }
            TypeReference::FnPtr(ref sig) => write!(f, "*({})", sig),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::cli::tables::{TableIndex, TableHandle};
    use crate::cli::signatures::{RetType,Param,MethodSignature,SignatureHeader};

    macro_rules! type_parse_tests {
        ($($name:ident($data:expr, $expected:expr);)*) => {
            $(
                #[test]
                pub fn $name() {
                    let mut buf = ::std::io::Cursor::new($data);
                    let typ = TypeReference::read(&mut buf).unwrap();
                    assert_eq!($expected, typ);
                }
            )*
        };
    }

    type_parse_tests! {
        end([0x00], TypeReference::End);
        void([0x01], TypeReference::Void);
        boolean([0x02], TypeReference::Boolean);
        char([0x03], TypeReference::Char);
        i1([0x04], TypeReference::I1);
        u1([0x05], TypeReference::U1);
        i2([0x06], TypeReference::I2);
        u2([0x07], TypeReference::U2);
        i4([0x08], TypeReference::I4);
        u4([0x09], TypeReference::U4);
        i8([0x0A], TypeReference::I8);
        u8([0x0B], TypeReference::U8);
        r4([0x0C], TypeReference::R4);
        r8([0x0D], TypeReference::R8);
        string([0x0E], TypeReference::String);
        byref_object([0x10, 0x1C], TypeReference::ByRef(Box::new(TypeReference::Object)));
        byref_ptr_byref_i8([0x10, 0x0F, 0x10, 0x0A], TypeReference::ByRef(
            Box::new(TypeReference::Ptr(
                vec![], 
                Box::new(TypeReference::ByRef(
                    Box::new(TypeReference::I8)))))));
        typedbyref([0x16], TypeReference::TypedByRef);
        i([0x18], TypeReference::I);
        u([0x19], TypeReference::U);
        object([0x1C], TypeReference::Object);
        array_boolean([0x14, 0x02, 0x01, 0x01, 0x0A, 0x01, 0x00], TypeReference::Array(
            Box::new(TypeReference::Boolean),
            ArrayShape::new(1, vec![10], vec![0]),
        ));
        valuetype([0x11, 0x42], TypeReference::ValueType(TableHandle::new(0x10, TableIndex::TypeSpec)));
        class([0x12, 0x42], TypeReference::Class(TableHandle::new(0x10, TableIndex::TypeSpec)));
        generic_inst_class([0x15, 0x12, 0x42, 0x02, 0x04, 0x05], TypeReference::GenericInst(
            Box::new(TypeReference::Class(TableHandle::new(0x10, TableIndex::TypeSpec))),
            vec![TypeReference::I1, TypeReference::U1]
        ));
        generic_inst_value_type([0x15, 0x11, 0x42, 0x02, 0x04, 0x05], TypeReference::GenericInst(
            Box::new(TypeReference::ValueType(TableHandle::new(0x10, TableIndex::TypeSpec))),
            vec![TypeReference::I1, TypeReference::U1]
        ));
        var([0x13, 0x42], TypeReference::Var(0x42));
        mvar([0x1E, 0x42], TypeReference::MVar(0x42));
        ptr_char([0x0F, 0x1F, 0x42, 0x20, 0x42, 0x03], TypeReference::Ptr(
            vec![
                CustomModifier::new(true, TableHandle::new(0x10, TableIndex::TypeSpec)),
                CustomModifier::new(false, TableHandle::new(0x10, TableIndex::TypeSpec)),
            ],
            Box::new(TypeReference::Char),
        ));
        ptr_ptr_char([0x0F, 0x0F, 0x03], TypeReference::Ptr(
            vec![],
            Box::new(TypeReference::Ptr(
                vec![], 
                Box::new(TypeReference::Char))),
        ));
        szarray_string([0x1D, 0x1F, 0x42, 0x20, 0x42, 0x0E], TypeReference::SzArray(
            vec![
                CustomModifier::new(true, TableHandle::new(0x10, TableIndex::TypeSpec)),
                CustomModifier::new(false, TableHandle::new(0x10, TableIndex::TypeSpec)),
            ],
            Box::new(TypeReference::String)
        ));
        sentinel([0x41], TypeReference::Sentinel);
        fnptr_simple([0x1B, 0x20, 0x02, 0x0E, 0x08, 0x0E], TypeReference::FnPtr(Box::new(MethodSignature::new(
            SignatureHeader::new(0x20),
            RetType::new(vec![], TypeReference::String),
            2,
            0,
            vec![
                Param::new(vec![], TypeReference::I4),
                Param::new(vec![], TypeReference::String),
            ]
        ))));
        fnptr_varargs([0x1B, 0x25, 0x03, 0x0E, 0x08, 0x0E, 0x41, 0x0C], TypeReference::FnPtr(Box::new(MethodSignature::new(
            SignatureHeader::new(0x25),
            RetType::new(vec![], TypeReference::String),
            2,
            0,
            vec![
                Param::new(vec![], TypeReference::I4),
                Param::new(vec![], TypeReference::String),
                Param::new(vec![], TypeReference::R4),
            ]
        ))));
    }
}
