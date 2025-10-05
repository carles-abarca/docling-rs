//! Data model types for unified document representation

pub mod document;
pub mod input;
pub mod node;
pub mod result;
pub mod table;
pub mod text;

// Re-exports
pub use document::DoclingDocument;
pub use input::{DocumentSource, InputDocument};
pub use node::{DocumentNode, NodeItem, NodeMetadata, NodeType, SourcePosition};
pub use result::{ConversionMetrics, ConversionResult, ConversionStatus};
pub use table::{TableCell, TableData, TableMetadata, TableRow};
pub use text::{Formatting, TextItem};
