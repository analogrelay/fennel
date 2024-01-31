extern crate ecma355metadata;

use std::env;
use std::fs::File;

use ecma355metadata::MetadataImage;
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
        println!("Usage: dump_pe <file>");
    } else {
        let file = File::open(&args[1]).unwrap();
        let image = MetadataImage::read(file).unwrap();

        println!("COFF Header:");
        println!("  Machine: 0x{:04X}", image.pe().coff_header().machine);
        println!(
            "  Number of Sections: {}",
            image.pe().coff_header().number_of_sections
        );
        println!("  Timestamp: {}", image.pe().coff_header().timestamp);
        println!(
            "  Symbol Table Offset: 0x{:04X}",
            image.pe().coff_header().symbol_table_addr
        );
        println!("  Symbol Count: {}", image.pe().coff_header().symbol_count);
        println!(
            "  Optional Header Size: {}",
            image.pe().coff_header().optional_header_size
        );
        println!(
            "  Characteristics: {}",
            image.pe().coff_header().characteristics
        );
        println!();

        if let Some(pe_header) = image.pe().pe_header() {
            println!("PE Header:");
            println!("  Magic: {}", pe_header.magic);
            println!(
                "  Linker Version: {}.{}",
                pe_header.major_linker_version,
                pe_header.minor_linker_version
            );
            println!("  Code Size: {}", pe_header.code_size);
            println!(
                "  Initialized Data Size: {}",
                pe_header.initialized_data_size
            );
            println!(
                "  Uninitialized Data Size: {}",
                pe_header.uninitialized_data_size
            );
            println!("  Entrypoint RVA: 0x{:08X}", pe_header.entry_point_rva);
            println!("  Base of Code: 0x{:08X}", pe_header.code_base);
            println!("  Base of Data: 0x{:08X}", pe_header.data_base);
            println!("  Image Base: 0x{:08X}", pe_header.image_base);
            println!("  Section Alignment: 0x{:08X}", pe_header.section_alignment);
            println!("  File Alignment: 0x{:08X}", pe_header.file_alignment);
            println!(
                "  OS Version: {}.{}",
                pe_header.major_os_version,
                pe_header.minor_os_version
            );
            println!(
                "  Image Version: {}.{}",
                pe_header.major_image_version,
                pe_header.minor_image_version
            );
            println!(
                "  Subsystem: {} (Version {}.{})",
                pe_header.subsystem,
                pe_header.major_subsystem_version,
                pe_header.minor_subsystem_version
            );
            println!("  Win32 Version: {}", pe_header.win32_version);
            println!("  Size of Image: {}", pe_header.size_of_image);
            println!("  Size of Headers: {}", pe_header.size_of_headers);
            println!("  Checksum: {}", pe_header.checksum);
            println!("  DLL Flags: 0x{:X}", pe_header.dll_flags);
            println!(
                "  Stack Reserve Size: 0x{:04X}",
                pe_header.stack_reserve_size
            );
            println!("  Stack Commit Size: 0x{:08X}", pe_header.stack_commit_size);
            println!("  Heap Reserve Size: 0x{:08X}", pe_header.heap_reserve_size);
            println!("  Heap Commit Size: 0x{:08X}", pe_header.heap_commit_size);
            println!("  Loader Flags: 0x{:08X}", pe_header.loader_flags);
            println!(
                "  Number of Data Directories: {}",
                pe_header.number_of_data_directories
            );
            println!();
            println!("Data Directories:");

            for dir in pe_header.directories() {
                println!("  {}", dir);
            }
            println!();
        }

        println!("Sections:");
        for section in image.pe().sections() {
            println!("  {}", section.name);
            println!("    Virtual Size: 0x{:08X}", section.virtual_size);
            println!("    Virtual Address: 0x{:08X}", section.virtual_address);
            println!("    Size of Raw Data: 0x{:08X}", section.size_of_raw_data);
            println!(
                "    Pointer to Raw Data: 0x{:08X}",
                section.pointer_to_raw_data
            );
            println!(
                "    Pointer to Relocations: 0x{:08X}",
                section.pointer_to_relocations
            );
            println!(
                "    Pointer to Line Numbers: 0x{:08X}",
                section.pointer_to_linenumbers
            );
            println!(
                "    Number of Relocations: {}",
                section.number_of_relocations
            );
            println!(
                "    Number of Line Numbers: {}",
                section.number_of_linenumbers
            );
            println!("    Characteristics: {}", section.characteristics);
        }
    }
}
