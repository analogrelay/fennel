use std::fmt::{Display, Error, Formatter};

#[derive(Clone, Copy)]
pub struct Guid([u8; 16]);

impl Guid {
    pub const EMPTY: Guid = Guid([0u8; 16]);

    pub fn from_bytes(bytes: &[u8]) -> Guid {
        if bytes.len() != 16 {
            panic!("Guid must be 16 bytes");
        }

        let mut guid = Guid::EMPTY;
        guid.0.copy_from_slice(bytes);
        guid
    }
}

impl Default for Guid {
    fn default() -> Guid {
        Guid::EMPTY
    }
}

impl Display for Guid {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let &Guid(ref values) = self;
        write!(
            f,
            "{{{:02X}{:02X}{:02X}{:02X}-{:02X}{:02X}-{:02X}{:02X}-{:02X}{:02X}-{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}}}",
            values[0],
            values[1],
            values[2],
            values[3],
            values[4],
            values[5],
            values[6],
            values[7],
            values[8],
            values[9],
            values[10],
            values[11],
            values[12],
            values[13],
            values[14],
            values[15]
        )
    }
}
