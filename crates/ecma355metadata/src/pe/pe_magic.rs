#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub struct PeMagic(u16);

impl PeMagic {
    pub const PE32: PeMagic = PeMagic(0x010B);
    pub const PE32PLUS: PeMagic = PeMagic(0x020B);

    pub fn new(val: u16) -> PeMagic {
        PeMagic(val)
    }

    pub fn is_pe32plus(self) -> bool { self == PeMagic::PE32PLUS }
}

impl ::std::fmt::Display for PeMagic {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        match self {
            &PeMagic::PE32 => f.write_str("PE32"),
            &PeMagic::PE32PLUS => f.write_str("PE32+"),
            &PeMagic(x) => write!(f, "0x{:X}", x),
        }
    }
}
