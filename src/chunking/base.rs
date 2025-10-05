//! Base chunking types and traits

use super::metadata::ChunkMetadata;
use crate::datamodel::DoclingDocument;
use serde::{Deserialize, Serialize};

/// Error type for chunking operations
///
/// This error type covers all possible failures that can occur during chunking,
/// including tokenizer loading, configuration validation, and processing errors.
#[derive(Debug, thiserror::Error)]
pub enum ChunkingError {
    #[error("Failed to load tokenizer: {0}")]
    TokenizerLoad(String),

    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    #[error("Document processing error: {0}")]
    ProcessingError(String),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

/// A single chunk of document text with metadata
///
/// Represents a semantically meaningful segment of a document with associated
/// metadata including hierarchical context (headings), position offsets, and index.
///
/// # Examples
///
/// ```ignore
/// use docling_rs::chunking::{BaseChunk, ChunkMetadata};
///
/// let chunk = BaseChunk {
///     text: "This is a paragraph.".to_string(),
///     meta: ChunkMetadata {
///         doc_name: "document.md".to_string(),
///         headings: vec!["Chapter 1".to_string()],
///         caption: None,
///         start_offset: 0,
///         end_offset: 20,
///         index: 0,
///     },
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseChunk {
    /// The text content of this chunk
    pub text: String,

    /// Metadata describing this chunk's context and position
    pub meta: ChunkMetadata,
}

/// Abstract interface for all chunking strategies
///
/// This trait defines the contract that all chunking implementations must follow.
/// It provides two core methods: `chunk()` for generating chunks and `contextualize()`
/// for adding hierarchical context to chunks.
///
/// # Contract
///
/// All implementations must guarantee:
/// - Chunks are returned in document order
/// - Iterator is lazy (no upfront allocation)
/// - `contextualize()` is deterministic
/// - All document content is represented in chunks
pub trait BaseChunker {
    /// Generate chunks from a document
    ///
    /// Returns a lazy iterator to avoid loading all chunks into memory.
    /// Chunks are guaranteed to be in document order with sequential indices.
    ///
    /// # Arguments
    ///
    /// * `doc` - The document to chunk
    ///
    /// # Returns
    ///
    /// A boxed iterator yielding [`BaseChunk`] instances
    fn chunk<'a>(&'a self, doc: &'a DoclingDocument) -> Box<dyn Iterator<Item = BaseChunk> + 'a>;

    /// Contextualize a chunk by prefixing with hierarchical metadata
    ///
    /// Used for embedding model input. The output includes headings,
    /// captions, and other metadata before the chunk text, providing
    /// context for better semantic understanding.
    ///
    /// # Arguments
    ///
    /// * `chunk` - The chunk to contextualize
    ///
    /// # Returns
    ///
    /// A string with metadata-prefixed chunk text
    fn contextualize(&self, chunk: &BaseChunk) -> String;
}
