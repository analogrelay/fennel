use std::mem;
use std::io::Read;

use crate::cli::tables::{TableHandle, TableIndex};
use crate::cli::signatures::{ArrayShape, CustomModifier, TypeReference, MethodSignature};
use crate::error::Error;

// Utilities for reading, used by types within this module
pub fn read_type_def_or_ref_spec_encoded(reader: &mut impl Read) -> Result<TableHandle, Error> {
    let val = read_compressed_u32(reader)?;

    // Determine the table and index
    let tag = val & 0x03;
    let index = val >> 2;
    let table = match tag {
        0x00 => TableIndex::TypeDef,
        0x01 => TableIndex::TypeRef,
        0x02 => TableIndex::TypeSpec,
        _ => return Err(Error::InvalidMetadata("invalid TypeDefOrRefSpecEncoded value, tag value is out of range".into()))
    };
    Ok(TableHandle::new(index as usize, table))
}

pub fn read_type(discriminator: u32, reader: &mut impl Read) -> Result<TypeReference, Error> {
    match discriminator {
        0x00 => Ok(TypeReference::End),
        0x01 => Ok(TypeReference::Void),
        0x02 => Ok(TypeReference::Boolean),
        0x03 => Ok(TypeReference::Char),
        0x04 => Ok(TypeReference::I1),
        0x05 => Ok(TypeReference::U1),
        0x06 => Ok(TypeReference::I2),
        0x07 => Ok(TypeReference::U2),
        0x08 => Ok(TypeReference::I4),
        0x09 => Ok(TypeReference::U4),
        0x0A => Ok(TypeReference::I8),
        0x0B => Ok(TypeReference::U8),
        0x0C => Ok(TypeReference::R4),
        0x0D => Ok(TypeReference::R8),
        0x0E => Ok(TypeReference::String),
        0x0F => {
            // Ptr
            let (mods, typ) = read_modifiers_and_type(reader)?;
            Ok(TypeReference::Ptr(mods, Box::new(typ)))
        },
        0x10 => Ok(TypeReference::ByRef(Box::new(TypeReference::read(reader)?))),
        0x11 => {
            // ValueType
            let typ = read_type_def_or_ref_spec_encoded(reader)?;
            Ok(TypeReference::ValueType(typ))
        },
        0x12 => {
            // Class
            let typ = read_type_def_or_ref_spec_encoded(reader)?;
            Ok(TypeReference::Class(typ))
        },
        0x13 => Ok(TypeReference::Var(read_compressed_u32(reader)?)),
        0x14 => {
            // Array
            let element_type = TypeReference::read(reader)?;
            let shape = ArrayShape::read(reader)?;
            Ok(TypeReference::Array(Box::new(element_type), shape))
        },
        0x15 => {
            // GenericInst
            let inst_type = TypeReference::read(reader)?;
            let arg_count = read_compressed_u32(reader)?;
            let mut args = Vec::with_capacity(arg_count as usize);
            for _ in 0..arg_count {
                args.push(TypeReference::read(reader)?);
            }
            Ok(TypeReference::GenericInst(Box::new(inst_type), args))
        },
        0x16 => Ok(TypeReference::TypedByRef),
        0x18 => Ok(TypeReference::I),
        0x19 => Ok(TypeReference::U),
        0x1B => Ok(TypeReference::FnPtr(Box::new(MethodSignature::read(reader)?))),
        0x1C => Ok(TypeReference::Object),
        0x1D => {
            // SzArray
            let (mods, typ) = read_modifiers_and_type(reader)?;
            Ok(TypeReference::SzArray(mods, Box::new(typ)))
        }
        0x1E => Ok(TypeReference::MVar(read_compressed_u32(reader)?)),
        0x41 => Ok(TypeReference::Sentinel),
        x => Err(Error::UnknownTypeCode(x)),
    }
}

pub fn read_modifiers_and_type(reader: &mut impl Read) -> Result<(Vec<CustomModifier>, TypeReference), Error> {
    let mut cur = read_compressed_u32(reader)?;
    let mut mods = Vec::new();
    while cur == 0x20 || cur == 0x1F {
        let required = match cur {
            0x20 => false,
            0x1F => true,
            _ => {
                return Err(Error::InvalidMetadata(
                    "invalid CMOD value for custom modifier".into(),
                ))
            }
        };
        mods.push(CustomModifier::new(required, read_type_def_or_ref_spec_encoded(reader)?));
        cur = read_compressed_u32(reader)?;
    }
    let typ = read_type(cur, reader)?;
    Ok((mods, typ))
}

// From: https://source.dot.net/#System.Reflection.Metadata/System/Reflection/Metadata/BlobReader.cs,494
pub fn read_compressed_u32(reader: &mut impl Read) -> Result<u32, Error> {
    Ok(read_compressed_u32_helper(reader)?.0)
}

pub fn read_compressed_i32(reader: &mut impl Read) -> Result<i32, Error> {
    let (mut val, bytes) = read_compressed_u32_helper(reader)?;
    let sign_extend = (val & 0x1) != 0;
    val >>= 1;

    if sign_extend {
        match bytes {
            1 => Ok(unsafe { mem::transmute(val | 0xffffffc0) } ),
            2 => Ok(unsafe { mem::transmute(val | 0xffffe000) } ),
            4 => Ok(unsafe { mem::transmute(val | 0xf0000000) } ),
            _ => panic!("Unexpected compressed integer size"),
        }
    }
    else {
        Ok(val as i32)
    }
}

fn read_compressed_u32_helper(reader: &mut impl Read) -> Result<(u32, usize), Error> {
    let mut buf = [0u8; 4];
    reader.read_exact(&mut buf[0..1])?;
    if buf[0] & 0x80 == 0 {
        // 1-byte number
        Ok(((buf[0] & 0x7F) as u32, 1))
    } else if buf[0] & 0x40 == 0 {
        // 2-byte number
        reader.read_exact(&mut buf[1..2])?;
        let val = ((buf[0] & 0x3F) as u32) << 8 |
            buf[1] as u32;
        Ok((val, 2))
    } else {
        // 4-byte number
        reader.read_exact(&mut buf[1..4])?;
        let val =
            ((buf[0] & 0x1F) as u32) << 24 |
            (buf[1] as u32) << 16 |
            (buf[2] as u32) << 8 |
            (buf[3] as u32);
        Ok((val, 4))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    macro_rules! read_compressed_u32_tests {
        ($($name: ident($encoded: expr, $val: expr);)*) => {
            $(
                #[test]
                pub fn $name() {
                    let mut buf = Cursor::new($encoded);
                    assert_eq!($val, read_compressed_u32(&mut buf).unwrap());
                }
            )*
        };
    }

    macro_rules! read_compressed_i32_tests {
        ($($name: ident($encoded: expr, $val: expr);)*) => {
            $(
                #[test]
                pub fn $name() {
                    let mut buf = Cursor::new($encoded);
                    assert_eq!($val, read_compressed_i32(&mut buf).unwrap());
                }
            )*
        };
    }

    read_compressed_u32_tests!{
        u8_0x03([0x03], 0x03);
        u8_0x7f([0x7F], 0x7F);
        u16_0x80([0x80, 0x80], 0x80);
        u16_0x2e57([0xAE, 0x57], 0x2E57);
        u16_0x3fff([0xBF, 0xFF], 0x3FFF);
        u32_0x4000([0xC0, 0x00, 0x40, 0x00], 0x4000);
        u32_0x1fff_ffff([0xDF, 0xFF, 0xFF, 0xFF], 0x1FFF_FFFF);
    }

    read_compressed_i32_tests!{
        u8_pos_3([0x06], 3);
        u8_neg_3([0x7B], -3);
        u16_pos_64([0x80, 0x80], 64);
        u8_neg_64([0x01], -64);
        u32_pos_8192([0xC0, 0x00, 0x40, 0x00], 8192);
        u16_neg_8192([0x80, 0x01], -8192);
        u32_pos_2pow28([0xDF, 0xFF, 0xFF, 0xFE], 268435455);
        u32_neg_2pow28([0xC0, 0x00, 0x00, 0x01], -268435456);
    }

    #[test]
    pub fn type_def_or_ref_spec_encoded() {
        let mut buf = Cursor::new([0x49]);
        assert_eq!(TableHandle::new(0x12, TableIndex::TypeRef), read_type_def_or_ref_spec_encoded(&mut buf).unwrap());
    }

    #[test]
    pub fn type_def_or_ref_spec_encoded_large() {
        let mut buf = Cursor::new([0xC0, 0x48, 0xD1, 0x5A]);
        assert_eq!(TableHandle::new(0x123456, TableIndex::TypeSpec), read_type_def_or_ref_spec_encoded(&mut buf).unwrap());
    }
}