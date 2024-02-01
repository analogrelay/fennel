// We want ParamAttributes to use the same names as in the ECMA spec, which are PascalCased, not UPPER_SNAKE_CASE
#![allow(non_upper_case_globals)]

use std::convert::Infallible;

bitflags! {
    pub struct ParamAttributes : u16 {
        const In = 0x0001;
        const Out = 0x0002;
        const Optional = 0x0010;
        const HasDefault = 0x1000;
        const HasFieldMarshal = 0x2000;
    }
}

impl std::convert::TryFrom<u16> for ParamAttributes {
    type Error = Infallible;

    fn try_from(value: u16) -> Result<ParamAttributes, Infallible> {
        Ok(ParamAttributes::from_bits_truncate(value))
    }
}

impl std::fmt::Display for ParamAttributes {
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