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
pub mod layout;  // Public for Phase 3b
pub mod layout_analyzer;  // Public for Phase 3b
pub mod page;  // Public for Phase 3b (TextBlock, etc.)
pub mod table;  // Public for Phase 3c
pub mod table_detector;  // Public for Phase 3c
pub mod image;  // Public for Phase 3d
pub mod image_extractor;  // Public for Phase 3d
pub mod ocr;  // Public for Phase 3e
pub mod ocr_engine;  // Public for Phase 3e
// mod text_extractor;  // TODO: Fix pdfium API compatibility issues
pub mod types;  // Public for Phase 3b (BoundingBox, FontInfo, etc.)
