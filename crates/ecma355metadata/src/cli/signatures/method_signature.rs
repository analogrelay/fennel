use std::fmt;
use std::io::Read;

use crate::cli::signatures::{Param, RetType, SignatureCallingConvention, SignatureHeader, TypeReference};
use crate::cli::signatures::utils;
use crate::error::Error;

#[derive(Debug, PartialEq, Eq)]
pub struct MethodSignature {
    pub header: SignatureHeader,
    pub return_type: RetType,
    pub required_parameter_count: u32,
    pub parameters: Vec<Param>,
}

impl MethodSignature {
    pub fn new(
        header: SignatureHeader,
        return_type: RetType,
        required_parameter_count: u32,
        parameters: Vec<Param>,
    ) -> MethodSignature {
        MethodSignature {
            header,
            return_type,
            required_parameter_count,
            parameters,
        }
    }

    pub fn read(reader: &mut impl Read) -> Result<MethodSignature, Error> {
        let header = SignatureHeader::read(reader)?;
        let param_count = utils::read_compressed_u32(reader)?;
        let return_type = RetType::read(reader)?;

        let mut parameters = Vec::with_capacity(param_count as usize);
        let mut required_parameter_count = None;
        for idx in 0..param_count {
            let mut param = Param::read(reader)?;
            if param.type_reference == TypeReference::Sentinel {
                // This is the marker for the varargs param
                required_parameter_count = Some(idx);
                param = Param::read(reader)?;
            }
            parameters.push(param);
        }

        Ok(MethodSignature::new(
            header,
            return_type,
            required_parameter_count.unwrap_or(param_count),
            parameters,
        ))
    }
}

impl fmt::Display for MethodSignature {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self.header.calling_convention() {
            SignatureCallingConvention::CDecl => write!(f, "cdecl ")?,
            SignatureCallingConvention::StdCall => write!(f, "stdcall ")?,
            SignatureCallingConvention::ThisCall => write!(f, "thiscall ")?,
            SignatureCallingConvention::FastCall => write!(f, "fastcall ")?,
            SignatureCallingConvention::VarArgs => write!(f, "varargs ")?,
            _ => {}
        }
        if self.header.is_generic() {
            write!(f, ".generic ")?;
        }
        if !self.header.has_this() {
            write!(f, "static ")?;
        }
        if !self.header.explicit_this() {
            write!(f, ".explicitthis ")?;
        }
        write!(f, "{} .method(", self.return_type)?;
        for param in self.parameters.iter() {
            write!(f, "{} ", param)?;
        }
        write!(f, ")")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::io::Cursor;

    #[test]
    pub fn simple_signature() {
        let mut buf = Cursor::new([0x20, 0x02, 0x0E, 0x08, 0x0E]);
        let sig = MethodSignature::read(&mut buf).unwrap();
        assert_eq!(
            MethodSignature::new(
                SignatureHeader::new(0x20, 0),
                RetType::new(vec![], TypeReference::String),
                2,
                vec![
                    Param::new(vec![], TypeReference::I4),
                    Param::new(vec![], TypeReference::String),
                ]
            ),
            sig
        );
    }

    #[test]
    pub fn varargs_signature() {
        let mut buf = Cursor::new([0x25, 0x03, 0x0E, 0x08, 0x0E, 0x41, 0x0C]);
        let sig = MethodSignature::read(&mut buf).unwrap();
        assert_eq!(
            MethodSignature::new(
                SignatureHeader::new(0x25, 0),
                RetType::new(vec![], TypeReference::String),
                2,
                vec![
                    Param::new(vec![], TypeReference::I4),
                    Param::new(vec![], TypeReference::String),
                    Param::new(vec![], TypeReference::R4),
                ]
            ),
            sig
        );
    }
}
