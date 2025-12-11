//! Core types and traits for docling-rs document processing
//!
//! This crate provides the fundamental types, traits, and utilities
//! shared across all docling-rs format backends.

pub mod backend;
pub mod chunking;
pub mod datamodel;
pub mod error;
pub mod format;
pub mod pipeline;

// Re-exports for convenience
pub use backend::Backend;
pub use datamodel::{
    ConversionMetrics, ConversionResult, ConversionStatus, DoclingDocument, DocumentNode,
    DocumentSource, Formatting, InputDocument, NodeItem, NodeMetadata, NodeType, SourcePosition,
    TableCell, TableData, TableMetadata, TableRow, TextItem,
};
pub use error::ConversionError;
pub use format::InputFormat;
pub use pipeline::{Pipeline, SimplePipeline};
