extern crate ecma355metadata;

use std::env;
use std::str;
use std::fs::File;
use std::io::Cursor;

use ecma355metadata::{MetadataReader, PeImage};
use ecma355metadata::cli::{Access, CliHeader, MethodFlags, MethodVTableLayout};
use ecma355metadata::cli::tables::MethodDef;
use ecma355metadata::cli::signatures::MethodSignature;
use ecma355metadata::pe::DirectoryType;

pub fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: dump_method_sig <file>");
    } else {
        let file_path = &args[1];

        let mut file = File::open(file_path).unwrap();
        let pe = PeImage::read(&mut file).unwrap();
        let cli_header = CliHeader::from_pe_image(&pe).unwrap();
        let assembly = MetadataReader::new(
            pe.load(cli_header.metadata.rva, cli_header.metadata.size as usize)
                .unwrap(),
        ).unwrap();

        let methods: Vec<_> = assembly
            .tables()
            .method_def()
            .iter()
            .map(|o| o.unwrap())
            .collect();

        let params: Vec<_> = assembly
            .tables()
            .param()
            .iter()
            .map(|o| o.unwrap())
            .collect();

        for idx in 0..methods.len() {
            let method = &methods[idx];

            // Load the method signature blob
            let mut sig_blob = Cursor::new(assembly.get_blob(method.signature).unwrap());
            let signature = MethodSignature::read(&mut sig_blob).unwrap();

            print!(" [0x{:08X}] ", method.rva);

            write_flags(method);

            let name =
                str::from_utf8(assembly.get_string(method.name).unwrap_or(b"<null>")).unwrap();

            print!("{} ", signature.return_type);

            print!("{}(", name);

            // Identify the end of the param list by looking at the next method
            let end = if idx == methods.len() - 1 {
                params.len()
            } else {
                methods[idx + 1].params.index()
            };

            // Iterate over the params
            let mut first = true;
            for (idx, param) in params[method.params.index()..end].iter().enumerate() {
                let param_sig = &signature.parameters[idx];
                if first {
                    first = false;
                } else {
                    print!(", ");
                }
                let name =
                    str::from_utf8(assembly.get_string(param.name).unwrap_or(b"<null>")).unwrap();
                print!("{} {}", param_sig, name);
            }
            println!(")")
        }
    }
}

fn write_flags(method: &MethodDef) {
    match method.flags.access() {
        Access::CompilerControlled => {}
        Access::Assembly => print!("internal "),
        Access::FamANDAssem => print!("private protected "),
        Access::Family => print!("protected"),
        Access::FamORAssem => print!("protected internal "),
        Access::Private => print!("private "),
        Access::Public => print!("public "),
    }
    let flags = method.flags.flags();
    if flags.contains(MethodFlags::Static) {
        print!("static ");
    } else if flags.contains(MethodFlags::Final) {
        print!("sealed ");
    } else if flags.contains(MethodFlags::Virtual) {
        print!("virtual ");
    } else if flags.contains(MethodFlags::Abstract) {
        print!("abstract ");
    }
    if flags.contains(MethodFlags::PInvokeImpl) {
        print!("extern ");
    }
    if flags.contains(MethodFlags::UnmanagedExport) {
        print!(".export ");
    }
    if flags.contains(MethodFlags::HideBySig) {
        print!(".hidebysig ");
    }
    if flags.contains(MethodFlags::Strict) {
        print!(".strict ");
    }
    if flags.contains(MethodFlags::SpecialName) {
        print!(".specialname ");
    }
    if flags.contains(MethodFlags::RTSpecialName) {
        print!(".rtspecialname ");
    }
    if flags.contains(MethodFlags::HasSecurity) {
        print!(".hassecurity ");
    }
    if flags.contains(MethodFlags::RequireSecObject) {
        print!(".requiresecobject ");
    }
    match method.flags.vtable_layout() {
        MethodVTableLayout::NewSlot => print!("new "),
        MethodVTableLayout::ReuseSlot => print!("override "),
    }
}
