use std::path::PathBuf;
use cil::metadata::Assembly;

use clap::{ArgAction, Parser};
use tracing::{debug, event, info, info_span, Instrument, Level};
use tracing_subscriber::fmt::format::{self, FmtSpan};

#[derive(Parser, Debug)]
#[command(author, version, about = "Fennel CIL Interpreter", long_about = None)]
struct Args {
    /// The path to the CIL assembly to run.
    assembly: PathBuf,

    /// Search paths for assemblies.
    #[arg(long = "lib", short = 'L')]
    search_paths: Vec<PathBuf>,

    /// Set the verbosity. Use -v for DEBUG, -vv for TRACE.
    #[arg(long = "verbose", short = 'v', action = ArgAction::Count)]
    verbosity: u8,
}

fn main() {
    let args = Args::parse();

    // Prepare tracing infrastructure.
    tracing_subscriber::fmt()
        .with_ansi(true)
        .without_time()
        .with_max_level(match args.verbosity {
            0 => Level::INFO,
            1 => Level::DEBUG,
            _ => Level::TRACE,
        })
        .with_target(args.verbosity > 0)
        .init();

    let assembly = info_span!("assembly_load", path = ?args.assembly).in_scope(|| {
        Assembly::load(args.assembly)
            .expect("failed to load assembly")
    });
}