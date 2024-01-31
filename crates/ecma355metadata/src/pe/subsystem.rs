#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub struct Subsystem(u16);

impl Subsystem {
    pub const UNKNOWN: Subsystem = Subsystem(0);
    pub const NATIVE: Subsystem = Subsystem(1);
    pub const WINDOWS_GUI: Subsystem = Subsystem(2);
    pub const WINDOWS_CUI: Subsystem = Subsystem(3);
    pub const OS2_CUI: Subsystem = Subsystem(5);
    pub const POSIX_CUI: Subsystem = Subsystem(7);
    pub const NATIVE_WINDOWS: Subsystem = Subsystem(8);
    pub const WINDOWS_CE_GUI: Subsystem = Subsystem(9);
    pub const EFI_APPLICATION: Subsystem = Subsystem(10);
    pub const EFI_BOOT_SERVICE_DRIVER: Subsystem = Subsystem(11);
    pub const EFI_RUNTIME_DRIVER: Subsystem = Subsystem(12);
    pub const EFI_ROM: Subsystem = Subsystem(13);
    pub const XBOX: Subsystem = Subsystem(14);
    pub const WINDOWS_BOOT_APPLICATION: Subsystem = Subsystem(16);

    pub fn new(val: u16) -> Subsystem {
        Subsystem(val)
    }
}

impl ::std::fmt::Display for Subsystem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        match self {
            &Subsystem::UNKNOWN => f.write_str("Unknown"),
            &Subsystem::NATIVE => f.write_str("Native"),
            &Subsystem::WINDOWS_GUI => f.write_str("WindowsGui"),
            &Subsystem::WINDOWS_CUI => f.write_str("WindowsConsole"),
            &Subsystem::OS2_CUI => f.write_str("OS/2"),
            &Subsystem::POSIX_CUI => f.write_str("POSIX"),
            &Subsystem::NATIVE_WINDOWS => f.write_str("NativeWindowsDriver"),
            &Subsystem::WINDOWS_CE_GUI => f.write_str("WindowsCE"),
            &Subsystem::EFI_APPLICATION => f.write_str("EFIApplication"),
            &Subsystem::EFI_BOOT_SERVICE_DRIVER => f.write_str("EFIBootServiceDriver"),
            &Subsystem::EFI_RUNTIME_DRIVER => f.write_str("EFIRuntimeDriver"),
            &Subsystem::EFI_ROM => f.write_str("EFIRom"),
            &Subsystem::XBOX => f.write_str("Xbox"),
            &Subsystem::WINDOWS_BOOT_APPLICATION => f.write_str("WindowsBootApplication"),
            &Subsystem(x) => write!(f, "0x{:X}", x),
        }
    }
}
