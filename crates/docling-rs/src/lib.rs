//! docling-rs - Document processing library
//!
//! Re-exports all core types and enabled format backends.
//!
//! # Example
//!
//! ```rust,no_run
//! use docling_rs::{DocumentConverter, InputFormat};
//!
//! let converter = DocumentConverter::new();
//! let result = converter.convert_file("document.pdf").unwrap();
//! println!("Converted: {}", result.document().name());
//! ```

mod converter;
pub mod output;

pub use converter::DocumentConverter;
pub use docling_rs_core::*;

/// Backwards compatibility - re-exports CLI-related types
pub mod cli {
    pub use crate::output;
}

#[cfg(feature = "markdown")]
pub use docling_rs_markdown::MarkdownBackend;

#[cfg(feature = "html")]
pub use docling_rs_html::HtmlBackend;

#[cfg(feature = "csv")]
pub use docling_rs_csv::CsvBackend;

#[cfg(feature = "docx")]
pub use docling_rs_docx::DocxBackend;

#[cfg(feature = "xlsx")]
pub use docling_rs_xlsx::XlsxBackend;

#[cfg(feature = "pptx")]
pub use docling_rs_pptx::PptxBackend;

#[cfg(feature = "pdf")]
pub use docling_rs_pdf::PdfBackend;

/// Backwards compatibility module - re-exports backend types
pub mod backend {
    pub use docling_rs_core::Backend;

    #[cfg(feature = "markdown")]
    pub use docling_rs_markdown::MarkdownBackend;

    #[cfg(feature = "html")]
    pub use docling_rs_html::HtmlBackend;

    #[cfg(feature = "csv")]
    pub use docling_rs_csv::CsvBackend;

    #[cfg(feature = "docx")]
    pub use docling_rs_docx::DocxBackend;

    #[cfg(feature = "xlsx")]
    pub use docling_rs_xlsx::XlsxBackend;

    #[cfg(feature = "pptx")]
    pub use docling_rs_pptx::PptxBackend;

    #[cfg(feature = "pdf")]
    pub use docling_rs_pdf::PdfBackend;
}

/// Backwards compatibility module - re-exports datamodel types
pub mod datamodel {
    pub use docling_rs_core::{
        ConversionMetrics, ConversionResult, ConversionStatus, DoclingDocument, DocumentNode,
        DocumentSource, Formatting, InputDocument, NodeItem, NodeMetadata, NodeType,
        SourcePosition, TableCell, TableData, TableMetadata, TableRow, TextItem,
    };
}

/// Create a pipeline with all enabled format backends
///
/// This function creates a `SimplePipeline` pre-configured with all
/// backends that are enabled via feature flags.
pub fn create_pipeline() -> SimplePipeline {
    let mut pipeline = SimplePipeline::new();

    #[cfg(feature = "markdown")]
    pipeline.register_backend(Box::new(MarkdownBackend::new()));

    #[cfg(feature = "html")]
    pipeline.register_backend(Box::new(HtmlBackend::new()));

    #[cfg(feature = "csv")]
    pipeline.register_backend(Box::new(CsvBackend::new()));

    #[cfg(feature = "docx")]
    pipeline.register_backend(Box::new(DocxBackend::new()));

    #[cfg(feature = "xlsx")]
    pipeline.register_backend(Box::new(XlsxBackend::new()));

    #[cfg(feature = "pptx")]
    pipeline.register_backend(Box::new(PptxBackend::new()));

    #[cfg(feature = "pdf")]
    pipeline.register_backend(Box::new(PdfBackend::new()));

    pipeline
}
