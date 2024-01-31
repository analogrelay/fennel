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
        println!("Usage: dump_asm <file>");
    } else {
        let file = File::open(&args[1]).unwrap();
        let image = MetadataImage::read(file).unwrap();

        println!("CLI Header");
        println!("  Size: {}", image.cli_header().header_size);
        println!(
            "  Runtime Version: {}.{}",
            image.cli_header().major_runtime_version,
            image.cli_header().minor_runtime_version
        );
        println!("  Metadata: {}", image.cli_header().metadata);
        println!("  Flags: {}", image.cli_header().flags);
        println!(
            "  Entrypoint Token: {}",
            image.cli_header().entry_point_token
        );
        println!("  Resources: {}", image.cli_header().resources);
        println!("  Strong Name: {}", image.cli_header().strong_name);
        println!(
            "  Code Manager Table: {}",
            image.cli_header().code_manager_table
        );
        println!("  VTable Fixups: {}", image.cli_header().vtable_fixups);
        println!(
            "  Export Address Table Jumps: {}",
            image.cli_header().export_address_table_jumps
        );
        println!(
            "  Managed/Native Header: {}",
            image.cli_header().managed_native_header
        );
        println!();

        println!("Metadata Header:");
        println!(
            "  Version: {}.{} ({})",
            image.metadata_header().major_version,
            image.metadata_header().minor_version,
            image.metadata_header().version
        );
        println!("  Flags: 0x{:04X}", image.metadata_header().flags);
        println!("  Streams:");
        for stream in image.metadata_header().streams.iter() {
            println!(
                "    *  {} 0x{:04X} - 0x{:04X} (Size: 0x{:04X})",
                stream.name,
                stream.offset,
                stream.offset + stream.size,
                stream.size
            );
        }
        println!();
    }
}
