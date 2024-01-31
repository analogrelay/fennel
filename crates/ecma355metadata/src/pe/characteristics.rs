bitflags! {
    pub struct FileCharacteristics: u16 {
        const RELOCS_STRIPPED = 0x0001;
        const EXECUTABLE_IMAGE = 0x0002;
        const LINE_NUMS_STRIPPED = 0x0004;
        const LOCAL_SYMS_STRIPPED = 0x0008;
        const AGGRESIVE_WS_TRIM = 0x0010;
        const LARGE_ADDRESS_AWARE = 0x0020;
        const BYTES_REVERSED_LO = 0x0040;
        const SUPPORTS_32_BIT_WORDS = 0x0100;
        const DEBUG_STRIPPED = 0x0200;
        const REMOVABLE_RUN_FROM_SWAP = 0x0400;
        const NET_RUN_FROM_SWAP = 0x0800;
        const SYSTEM = 0x1000;
        const DLL = 0x2000;
        const UP_SYSTEM_ONLY = 0x4000;
        const BYTES_REVERSED_HI = 0x8000;
    }
}

impl std::fmt::Debug for FileCharacteristics {
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

impl std::fmt::Display for FileCharacteristics {
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

bitflags! {
    pub struct SectionCharacteristics: u32 {
        const TYPE_NO_PAD = 0x00000008;
        const CNT_CODE = 0x00000020;
        const CNT_INITIALIZED_DATA = 0x00000040;
        const CNT_UNINITIALIZED_DATA = 0x00000080;
        const LNK_OTHER = 0x00000100;
        const LNK_INFO = 0x00000200;
        const LNK_REMOVE = 0x00000800;
        const LNK_COMDAT = 0x00001000;
        const NO_DEFER_SPEC_EXC = 0x00004000;
        const GPREL = 0x00008000;
        const MEM_PURGEABLE = 0x00020000;
        const MEM_LOCKED = 0x00040000;
        const MEM_PRELOAD = 0x00080000;
        const ALIGN_1BYTES = 0x00100000;
        const ALIGN_2BYTES = 0x00200000;
        const ALIGN_4BYTES = 0x00300000;
        const ALIGN_8BYTES = 0x00400000;
        const ALIGN_16BYTES = 0x00500000;
        const ALIGN_32BYTES = 0x00600000;
        const ALIGN_64BYTES = 0x00700000;
        const ALIGN_128BYTES = 0x00800000;
        const ALIGN_256BYTES = 0x00900000;
        const ALIGN_512BYTES = 0x00A00000;
        const ALIGN_1024BYTES = 0x00B00000;
        const ALIGN_2048BYTES = 0x00C00000;
        const ALIGN_4096BYTES = 0x00D00000;
        const ALIGN_8192BYTES = 0x00E00000;
        const LNK_NRELOC_OVFL = 0x01000000;
        const MEM_DISCARDABLE = 0x02000000;
        const MEM_NOT_CACHED = 0x04000000;
        const MEM_NOT_PAGED = 0x08000000;
        const MEM_SHARED = 0x10000000;
        const MEM_EXECUTE = 0x20000000;
        const MEM_READ = 0x40000000;
        const MEM_WRITE = 0x80000000;
    }
}

impl std::fmt::Debug for SectionCharacteristics {
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

impl std::fmt::Display for SectionCharacteristics {
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