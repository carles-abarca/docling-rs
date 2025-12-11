//! Base chunking types and traits

use super::metadata::ChunkMetadata;
use crate::datamodel::DoclingDocument;
use serde::{Deserialize, Serialize};

/// Error type for chunking operations
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseChunk {
    /// The text content of this chunk
    pub text: String,

    /// Metadata describing this chunk's context and position
    pub meta: ChunkMetadata,
}

/// Abstract interface for all chunking strategies
pub trait BaseChunker {
    /// Generate chunks from a document
    fn chunk<'a>(&'a self, doc: &'a DoclingDocument) -> Box<dyn Iterator<Item = BaseChunk> + 'a>;

    /// Contextualize a chunk by prefixing with hierarchical metadata
    fn contextualize(&self, chunk: &BaseChunk) -> String;
}
