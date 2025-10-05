//! Command-line interface module for docling-rs.
//!
//! Provides CLI argument parsing, conversion orchestration, and output generation.

pub mod args;
pub mod converter;
pub mod output;
pub mod progress;

pub use args::{CliArgs, InputFormat, OutputFormat};
pub use converter::Converter;
