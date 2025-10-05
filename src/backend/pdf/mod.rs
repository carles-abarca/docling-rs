//! PDF backend for document conversion.
//!
//! This module provides comprehensive PDF processing capabilities including:
//! - Text extraction with position metadata
//! - Layout analysis and reading order detection
//! - Table structure detection and extraction
//! - Image processing and classification
//! - OCR for scanned PDFs
//! - Content enrichment (code blocks, formulas, lists)
//!
//! # Example
//!
//! ```rust,ignore
//! use docling_rs::backend::pdf::{PdfBackend, PdfConfig};
//! use docling_rs::InputDocument;
//!
//! let config = PdfConfig::default()
//!     .enable_tables(true)
//!     .enable_images(true);
//!
//! let backend = PdfBackend::with_config(config);
//! let input = InputDocument::from_path("document.pdf", InputFormat::PDF);
//! let result = backend.convert(&input)?;
//! ```

// Re-export main types
pub use backend::PdfBackend;
pub use config::PdfConfig;
pub use document::PdfDocument;
pub use page::PdfPage;

// Module declarations
mod backend;
mod config;
mod document;
mod page;
// mod text_extractor; // TODO: Re-enable for Phase 3b layout analysis
mod types;
