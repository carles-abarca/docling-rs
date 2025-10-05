//! Chunk metadata structures

use serde::{Deserialize, Serialize};

/// Structured metadata attached to each chunk
///
/// Contains information about the chunk's position in the original document,
/// hierarchical context (headings), optional caption, and sequential index.
///
/// # Fields
///
/// * `doc_name` - Source document identifier
/// * `headings` - Hierarchical path of headings (e.g., ["Chapter 1", "Section 1.1"])
/// * `caption` - Optional caption for tables or figures
/// * `start_offset` - Character offset where chunk starts
/// * `end_offset` - Character offset where chunk ends
/// * `index` - Sequential index of this chunk (0-based)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkMetadata {
    /// Source document name/identifier
    pub doc_name: String,

    /// Hierarchical path of headings (e.g., ["Chapter 1", "Section 1.1"])
    pub headings: Vec<String>,

    /// Optional caption (for tables, figures)
    pub caption: Option<String>,

    /// Character offset where chunk starts in original document
    pub start_offset: usize,

    /// Character offset where chunk ends in original document
    pub end_offset: usize,

    /// Sequential index of this chunk (0-based)
    pub index: usize,
}
