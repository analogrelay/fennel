extern crate ecma355metadata;

use std::env;
use std::fs::File;

use ecma355metadata::MetadataImage;
use ecma355metadata::cli::tables::{Assembly, AssemblyRef, Constant, CustomAttribute, Field, InterfaceImpl, MemberRef, MethodDef, Module, Param, Table, TableIndex, TypeDef, TypeRef};
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
                TableIndex::InterfaceImpl => dump_interface_impl_table(&image),
                TableIndex::MemberRef => dump_member_ref_table(&image),
                TableIndex::Constant => dump_constant_table(&image),
                TableIndex::CustomAttribute => dump_custom_attribute_table(&image),
                TableIndex::Assembly => dump_assembly_table(&image),
                TableIndex::AssemblyRef => dump_assembly_ref_table(&image),
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

pub fn dump_assembly_ref_table(image: &MetadataImage) {
    let assembly_ref_table: Table<AssemblyRef> = image.table();
    println!("AssemblyRef Table: {} rows", assembly_ref_table.len());
    for (index, row) in assembly_ref_table.iter().enumerate() {
        let row = row.unwrap();
        println!("[0x{:04x}] * {:?}", index + 1, image.get_string(row.name).unwrap());
        println!("           Version: {}.{}.{}.{}", row.major_version, row.minor_version, row.build_number, row.revision_number);
        println!("           Flags: {}", row.flags);
        println!("           Public Key or Token: {}", row.public_key_or_token);
        println!("           Culture: {}", row.culture);
        println!("           Hash Value: {}", row.hash_value);
    }
}

pub fn dump_assembly_table(image: &MetadataImage) {
    let assembly_table: Table<Assembly> = image.table();
    println!("Assembly Table: {} rows", assembly_table.len());
    for (index, row) in assembly_table.iter().enumerate() {
        let row = row.unwrap();
        println!("[0x{:04x}] * {:?}", index + 1, image.get_string(row.name).unwrap());
        println!("           Hash Algorithm: {:?}", row.hash_alg_id);
        println!("           Version: {}.{}.{}.{}", row.major_version, row.minor_version, row.build_number, row.revision_number);
        println!("           Flags: {}", row.flags);
        println!("           Public Key: {}", row.public_key);
        println!("           Culture: {}", row.culture);
    }
}

pub fn dump_custom_attribute_table(image: &MetadataImage) {
    let custom_attribute_table: Table<CustomAttribute> = image.table();
    println!("CustomAttribute Table: {} rows", custom_attribute_table.len());
    for (index, row) in custom_attribute_table.iter().enumerate() {
        let row = row.unwrap();
        println!("[0x{:04X}] * Type: {} Parent: {} Value: {}", 
            index + 1, 
            row.typ, 
            row.parent, 
            row.value);
    }
}

pub fn dump_constant_table(image: &MetadataImage) {
    let constant_table: Table<Constant> = image.table();
    println!("Constant Table: {} rows", constant_table.len());
    for (index, row) in constant_table.iter().enumerate() {
        let row = row.unwrap();
        println!("[0x{:04X}] * Type: 0x{:02X} Parent: {} Value: {}", 
            index + 1, 
            row.typ, 
            row.parent, 
            row.value);
    }
}

pub fn dump_member_ref_table(image: &MetadataImage) {
    let member_ref_table: Table<MemberRef> = image.table();
    println!("MemberRef Table: {} rows", member_ref_table.len());
    for (index, row) in member_ref_table.iter().enumerate() {
        let row = row.unwrap();
        println!("[0x{:04X}] * Parent: {}, Name: {:?}, Signature: {}", index + 1, row.class, image.get_string(row.name).unwrap(), row.signature);
    }
}

pub fn dump_interface_impl_table(image: &MetadataImage) {
    let interface_impl_table: Table<InterfaceImpl> = image.table();
    println!("InterfaceImpl Table: {} rows", interface_impl_table.len());
    for (index, row) in interface_impl_table.iter().enumerate() {
        let row = row.unwrap();
        println!("[0x{:04X}] * Class: {} Interface: {}", index + 1, row.class, row.interface);
    }
}

pub fn dump_param_table(image: &MetadataImage) {
    let param_table: Table<Param> = image.table();
    println!("Param Table: {} rows", param_table.len());
    for (index, row) in param_table.iter().enumerate() {
        let row = row.unwrap();
        let name = image.get_string(row.name).unwrap();
        print!("[0x{:04X}] * {:?} #{}", index + 1, name, row.sequence);
        if !row.flags.is_empty() {
            print!("           ({})", row.flags);
        }
        println!();
    }
}

pub fn dump_method_def_table(image: &MetadataImage) {
    let method_def_table: Table<MethodDef> = image.table();
    println!("MethodDef Table: {} rows", method_def_table.len());
    for (index, row) in method_def_table.iter().enumerate() {
        let row = row.unwrap();
        let name = image.get_string(row.name).unwrap();
        println!(
            "[0x{:04X}] * {:?} @ 0x{:08X} ({}, {}, Sig: {}, Params: {})",
            index + 1,
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
    for (index, row) in type_def_table.iter().enumerate() {
        let row = row.unwrap();
        let name = image.get_string(row.type_name).unwrap();
        let namespace = image.get_string(row.type_namespace);

        print!("[0x{:04X}] * ", index + 1);

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
    for (index, row) in field_table.iter().enumerate() {
        let row = row.unwrap();
        println!(
            "[0x{:04X}] * {:?} ({}, Signature: {})",
            index + 1,
            image.get_string(row.name).unwrap(),
            row.flags,
            row.signature
        );
    }
}

pub fn dump_type_ref_table(image: &MetadataImage) {
    let type_ref_table: Table<TypeRef> = image.table();

    println!("TypeRef Table: {} rows", type_ref_table.len());
    for (index, row) in type_ref_table.iter().enumerate() {
        let row = row.unwrap();
        let name = image.get_string(row.name).unwrap();
        let namespace = image.get_string(row.namespace);

        if let Some(ns) = namespace {
            println!(
                "[0x{:04X}] * {:?}.{:?} (Scope: {})",
                index + 1,
                ns,
                name,
                row.resolution_scope
            );
        } else {
            println!(
                "[0x{:04X}] * {:?} (Scope: {})",
                index + 1,
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
    for (index, row) in module_table.iter().enumerate() {
        let row = row.unwrap();
        println!("[0x{:04X}] * Generation: {}", index + 1, row.generation);
        println!(
            "           Name: ({}) {:?}",
            row.name,
            image.get_string(row.name).unwrap(),
        );
        println!(
            "           MVID: {}",
            image.get_guid(row.mvid).unwrap_or_default()
        );
    }
    println!();
}
