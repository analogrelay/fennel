use std::fmt;

use crate::cli::tables::TableHandle;

#[derive(Debug, PartialEq, Eq)]
pub struct CustomModifier {
    required: bool,
    modifier_type: TableHandle,
}

impl CustomModifier {
    pub fn new(required: bool, modifier_type: TableHandle) -> CustomModifier {
        CustomModifier {
            required,
            modifier_type,
        }
    }
}

impl fmt::Display for CustomModifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        if self.required {
            write!(f, "modreq(")?;
        } else {
            write!(f, "modopt(")?;
        }
        write!(f, "{})", self.modifier_type)
    }
}
