//! docling-rs CLI binary
//!
//! Command-line interface for document conversion.

use clap::Parser;
use docling_rs::cli::{CliArgs, Converter};
use std::process;

fn main() {
    // Parse CLI arguments, handle clap errors to return exit code 1
    let args = match CliArgs::try_parse() {
        Ok(args) => args,
        Err(e) => {
            // Handle --version and --help which should exit successfully
            if e.kind() == clap::error::ErrorKind::DisplayHelp
                || e.kind() == clap::error::ErrorKind::DisplayVersion
            {
                print!("{}", e);
                process::exit(0);
            }
            // Print clap error and exit with code 1 (not 2)
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    // Create converter
    let converter = Converter::new(args);

    // Run conversion
    match converter.run() {
        Ok(()) => {
            process::exit(0);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}
