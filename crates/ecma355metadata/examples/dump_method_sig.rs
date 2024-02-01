extern crate ecma355metadata;

use std::env;
use std::fs::File;
use std::io::Cursor;

use ecma355metadata::cli::{Access, MethodFlags, MethodVTableLayout};
use ecma355metadata::cli::tables::{Param, MethodDef};
use ecma355metadata::cli::signatures::MethodSignature;
use ecma355metadata::MetadataImage;

pub fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: dump_method_sig <file>");
    } else {
        let file_path = &args[1];

        let file = File::open(file_path).unwrap();
        let image = MetadataImage::read(file).expect("failed to read metadata image");

        let methods: Vec<_> = image
            .table::<MethodDef>()
            .iter()
            .map(|o| o.unwrap())
            .collect();

        let params: Vec<_> = image
            .table::<Param>()
            .iter()
            .map(|o| o.unwrap())
            .collect();

        for idx in 0..methods.len() {
            let method = &methods[idx];

            // Load the method signature blob
            let mut sig_blob = Cursor::new(image.get_blob(method.signature).unwrap());
            let signature = MethodSignature::read(&mut sig_blob).unwrap();

            print!(" [0x{:08X}] ", method.rva);

            write_flags(method);

            let name =
                image.get_string(method.name).unwrap();

            print!("{} ", signature.return_type);

            print!("{:?}(", name);

            // Identify the end of the param list by looking at the next method
            let end = if idx == methods.len() - 1 {
                params.len()
            } else {
                methods[idx + 1].params.index() - 1
            };

            // Iterate over the params
            let mut first = true;
            for (idx, param) in params[(method.params.index() - 1)..end].iter().enumerate() {
                let param_sig = &signature.parameters[idx];
                if first {
                    first = false;
                } else {
                    print!(", ");
                }
                let name = image.get_string(param.name).unwrap();
                print!("{} {:?}", param_sig, name);
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
