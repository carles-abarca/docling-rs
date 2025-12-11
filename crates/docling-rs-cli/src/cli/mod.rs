//! Command-line interface module for docling-rs.
//!
//! Provides CLI argument parsing, conversion orchestration, and output generation.

pub mod args;
pub mod converter;
pub mod output;

pub use args::CliArgs;
pub use converter::Converter;
