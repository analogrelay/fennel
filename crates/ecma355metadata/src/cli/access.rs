#[repr(u16)]
#[derive(Debug, PartialEq, Eq)]
pub enum Access {
    CompilerControlled = 0,
    Private = 1,
    FamANDAssem = 2,
    Assembly = 3,
    Family = 4,
    FamORAssem = 5,
    Public = 6,
}
impl_display_via_debug!(Access);

impl Access {
    pub const MASK: u16 = 0x07;
    pub const SHIFT: u16 = 0;
}
