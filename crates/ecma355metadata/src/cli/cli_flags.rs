bitflags! {
    pub struct CliFlags: u32 {
        const ILONLY = 0x00001;
        const REQUIRE_32BIT = 0x00002;
        const IL_LIBRARY = 0x00004;
        const STRONGNAMESIGNED = 0x00008;
        const NATIVEENTRYPOINT = 0x00010;
        const TRACKDEBUGDATA = 0x10000;
        const PREFER_32BIT = 0x20000;
    }
}

impl std::fmt::Display for CliFlags {
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