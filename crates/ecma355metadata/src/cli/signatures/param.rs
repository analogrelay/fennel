use std::fmt;
use std::io::Read;

use crate::cli::signatures::{CustomModifier, TypeReference};
use crate::cli::signatures::utils;
use crate::error::Error;

#[derive(Debug, PartialEq, Eq)]
pub struct Param {
    pub modifiers: Vec<CustomModifier>,
    pub type_reference: TypeReference,
}

impl Param {
    pub fn new(modifiers: Vec<CustomModifier>, type_reference: TypeReference) -> Param {
        Param {
            modifiers,
            type_reference,
        }
    }

    pub fn read(reader: &mut impl Read) -> Result<Param, Error> {
        let (mods, typ) = utils::read_modifiers_and_type(reader)?;
        Ok(Param::new(mods, typ))
    }
}

impl fmt::Display for Param {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write_list!(f, self.modifiers.iter(), " ");
        write!(f, "{}", self.type_reference)
    }
}
