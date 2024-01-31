extern crate ecma355metadata;

use std::env;
use std::ffi::CStr;
use std::fs::File;

use ecma355metadata::MetadataImage;
use ecma355metadata::cli::tables::{Field, MethodDef, Module, Param, Table, TableIndex, TypeDef, TypeRef};
use tracing::Level;

pub fn main() {
    // Prepare tracing infrastructure.
    tracing_subscriber::fmt()
        .with_ansi(true)
        .without_time()
        .with_max_level(Level::TRACE)
        .init();

    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: dump_table <file> <table>");
    } else {
        let file_path = &args[1];

        let file = File::open(file_path).unwrap();
        let image = MetadataImage::read(file).unwrap();

        if args.len() < 3 {
            dump_table_names(&image);
        } else {
            let table_name = &args[2];
            let table: TableIndex = table_name.parse().expect("Unknown metadata table");

            match table {
                TableIndex::Module => dump_module_table(&image),
                TableIndex::TypeRef => dump_type_ref_table(&image),
                TableIndex::TypeDef => dump_type_def_table(&image),
                TableIndex::Field => dump_field_table(&image),
                TableIndex::MethodDef => dump_method_def_table(&image),
                TableIndex::Param => dump_param_table(&image),
                x => println!("Table not yet implemented: {}", x),
            }
        }
    }
}

pub fn dump_table_names(image: &MetadataImage) {
    println!("Table Row Counts:");
    for idx in TableIndex::each() {
        println!(
            "  {}: {} rows",
            idx,
            image.row_count(idx)
        );
    }
}

pub fn dump_param_table(image: &MetadataImage) {
    let param_table: Table<Param> = image.table();
    println!("Param Table: {} rows", param_table.len());
    for row in param_table.iter() {
        let row = row.unwrap();
        let name = image.get_string(row.name).unwrap_or(CStr::from_bytes_with_nul(b"<null>\0").unwrap());
        print!("* {:?} #{}", name, row.sequence);
        if !row.flags.is_empty() {
            print!(" ({})", row.flags);
        }
        println!();
    }
}

pub fn dump_method_def_table(image: &MetadataImage) {
    let method_def_table: Table<MethodDef> = image.table();
    println!("MethodDef Table: {} rows", method_def_table.len());
    for row in method_def_table.iter() {
        let row = row.unwrap();
        let name = image.get_string(row.name).unwrap_or(CStr::from_bytes_with_nul(b"<null>\0").unwrap());
        println!(
            " * {:?} @ 0x{:08X} ({}, {}, Sig: {}, Params: {})",
            name,
            row.rva,
            row.flags,
            row.impl_flags,
            row.signature,
            row.params,
        );
    }
}

pub fn dump_type_def_table(image: &MetadataImage) {
    let type_def_table: Table<TypeDef> = image.table();
    println!("TypeDef Table: {} rows", type_def_table.len());
    for row in type_def_table.iter() {
        let row = row.unwrap();
        let name = image.get_string(row.type_name).unwrap_or(CStr::from_bytes_with_nul(b"<null>\0").unwrap());
        let namespace = image.get_string(row.type_namespace);

        print!(" * ");


        if let Some(ns) = namespace {
            print!(
                "{:?}.{:?} ",
                ns,
                name,
            );
        } else {
            print!("{:?} ", name);
        }

        println!(
            "({}, Extends: {}, Fields: {}, Methods: {})",
            row.flags,
            row.extends,
            row.field_list,
            row.method_list
        );
    }
    println!()
}

pub fn dump_field_table(image: &MetadataImage) {
    let field_table: Table<Field> = image.table();
    println!("Field Table: {} rows", field_table.len());
    for row in field_table.iter() {
        let row = row.unwrap();
        println!(
            " * {:?} ({}, Signature: {})",
            image.get_string(row.name).unwrap_or(CStr::from_bytes_with_nul(b"<null>\0").unwrap()),
            row.flags,
            row.signature
        );
    }
}

pub fn dump_type_ref_table(image: &MetadataImage) {
    let type_ref_table: Table<TypeRef> = image.table();

    println!("TypeRef Table: {} rows", type_ref_table.len());
    for row in type_ref_table.iter() {
        let row = row.unwrap();
        let name = image.get_string(row.name).unwrap_or(CStr::from_bytes_with_nul(b"<null>\0").unwrap());
        let namespace = image.get_string(row.namespace);

        if let Some(ns) = namespace {
            println!(
                " * {:?}.{:?} (Scope: {})",
                ns,
                name,
                row.resolution_scope
            );
        } else {
            println!(
                " * {:?} (Scope: {})",
                name,
                row.resolution_scope
            );
        }
    }
    println!()
}

pub fn dump_module_table(image: &MetadataImage) {
    let module_table: Table<Module> = image.table();

    println!("Module Table: {} rows", module_table.len());
    for row in module_table.iter() {
        let row = row.unwrap();
        println!("  Generation: {}", row.generation);
        println!(
            "  Name: ({}) {:?}",
            row.name,
            image.get_string(row.name).unwrap(),
        );
        println!(
            "  MVID: {}",
            image.get_guid(row.mvid).unwrap_or_default()
        );
    }
    println!();
}
