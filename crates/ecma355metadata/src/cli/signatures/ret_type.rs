use std::fmt;
use std::io::Read;

use crate::cli::signatures::{CustomModifier, TypeReference};
use crate::cli::signatures::utils;
use crate::error::Error;

#[derive(Debug, PartialEq, Eq)]
pub struct RetType {
    pub modifiers: Vec<CustomModifier>,
    pub type_reference: TypeReference,
}

impl RetType {
    pub fn new(modifiers: Vec<CustomModifier>, type_reference: TypeReference) -> RetType {
        RetType {
            modifiers,
            type_reference,
        }
    }

    pub fn read(reader: &mut impl Read) -> Result<RetType, Error> {
        let (mods, typ) = utils::read_modifiers_and_type(reader)?;
        Ok(RetType::new(mods, typ))
    }
}

impl fmt::Display for RetType {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write_list!(f, self.modifiers.iter(), " ");
        write!(f, "{}", self.type_reference)
    }
}
