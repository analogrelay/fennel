use bitflags::bitflags;

use crate::Error;

#[repr(u32)]
#[derive(Debug)]
pub enum AssemblyHashAlgorithm {
    None = 0x0,
    MD5 = 0x8003,
    SHA1 = 0x8004,
}

impl TryFrom<u32> for AssemblyHashAlgorithm {
    type Error = Error;

    fn try_from(value: u32) -> Result<AssemblyHashAlgorithm, Error> {
        match value {
            0x0 => Ok(AssemblyHashAlgorithm::None),
            0x8003 => Ok(AssemblyHashAlgorithm::MD5),
            0x8004 => Ok(AssemblyHashAlgorithm::SHA1),
            _ => Err(Error::InvalidMetadata(format!("invalid AssemblyHashAlgorithm: 0x{:04X}", value))),
        }
    }
}

bitflags! {
    pub struct AssemblyFlags: u32 {
        const PublicKey = 0x0001;
        const Retargetable = 0x0100;
        const DisableJITCompileOptimizer = 0x4000;
        const EnableJITCompileTracking = 0x8000;
    }
}

impl std::fmt::Display for AssemblyFlags {
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

impl TryFrom<u32> for AssemblyFlags {
    type Error = Error;

    fn try_from(value: u32) -> Result<AssemblyFlags, Error> {
        AssemblyFlags::from_bits(value).ok_or(Error::InvalidMetadata(format!("invalid AssemblyFlags: 0x{:04X}", value)))
    }
}