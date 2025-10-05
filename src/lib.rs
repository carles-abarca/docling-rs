//! Docling-rs: Native Rust document processing library
//!
//! Extract structured text from Markdown, HTML, CSV, and DOCX files.
//!
//! # Example
//!
//! ```ignore
//! use docling_rs::DocumentConverter;
//!
//! let converter = DocumentConverter::new();
//! let result = converter.convert_file("document.md")?;
//! ```

pub mod backend;
pub mod chunking;
pub mod datamodel;
pub mod error;
pub mod format;
pub mod pipeline;

mod converter;

// Re-exports
pub use converter::DocumentConverter;
pub use datamodel::{ConversionResult, DoclingDocument, InputDocument};
pub use error::ConversionError;
pub use format::InputFormat;
