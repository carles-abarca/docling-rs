//! Hierarchical (structure-based) chunker implementation

use super::base::{BaseChunk, BaseChunker};
use super::metadata::ChunkMetadata;
use crate::datamodel::DoclingDocument;

/// Creates chunks based on document structure elements
///
/// `HierarchicalChunker` preserves the hierarchical structure of the document by creating
/// one chunk per document element (paragraph, section, table, etc.). It maintains metadata
/// about headings and hierarchical context.
///
/// # Examples
///
/// ```ignore
/// use docling_rs::chunking::{HierarchicalChunker, BaseChunker};
///
/// // Default configuration (merge_list_items = true)
/// let chunker = HierarchicalChunker::new();
///
/// // Custom configuration
/// let chunker = HierarchicalChunker::with_merge_list_items(false);
///
/// let chunks: Vec<_> = chunker.chunk(&doc).collect();
/// ```
#[derive(Debug, Clone)]
pub struct HierarchicalChunker {
    /// Whether to merge list items into single chunks (default: true)
    pub merge_list_items: bool,
}

impl HierarchicalChunker {
    /// Create a new HierarchicalChunker with default settings
    ///
    /// Default configuration:
    /// - `merge_list_items = true`: All list items in the same list are merged into a single chunk
    pub fn new() -> Self {
        Self {
            merge_list_items: true,
        }
    }

    /// Create a new HierarchicalChunker with custom merge_list_items setting
    ///
    /// # Arguments
    ///
    /// * `merge` - If true, list items in the same list are merged into a single chunk.
    ///   If false, each list item becomes a separate chunk.
    pub fn with_merge_list_items(merge: bool) -> Self {
        Self {
            merge_list_items: merge,
        }
    }
}

impl Default for HierarchicalChunker {
    fn default() -> Self {
        Self::new()
    }
}

impl BaseChunker for HierarchicalChunker {
    fn chunk<'a>(
        &'a self,
        doc: &'a DoclingDocument,
    ) -> Box<dyn Iterator<Item = BaseChunk> + 'a> {
        let doc_name = doc.name().to_string();
        let nodes = doc.nodes().to_vec();

        // Create chunks from nodes that have text content
        // Track current offset for sequential positioning
        let mut current_offset = 0;
        let mut chunk_index = 0;

        let chunks: Vec<BaseChunk> = nodes
            .into_iter()
            .filter_map(|node| {
                // Extract text content from node
                let text = node.text_content()?.to_string();

                // Skip empty text nodes
                if text.trim().is_empty() {
                    return None;
                }

                // Extract position if available, otherwise calculate sequential offsets
                let (start_offset, end_offset) = if let Some(pos) = node.position() {
                    let start = pos.start_offset();
                    let end = pos.end_offset();
                    current_offset = end;
                    (start, end)
                } else {
                    let start = current_offset;
                    let end = current_offset + text.len();
                    current_offset = end + 1; // +1 for newline/separator
                    (start, end)
                };

                let chunk = BaseChunk {
                    text,
                    meta: ChunkMetadata {
                        doc_name: doc_name.clone(),
                        headings: vec![],
                        caption: None,
                        start_offset,
                        end_offset,
                        index: chunk_index,
                    },
                };

                chunk_index += 1;
                Some(chunk)
            })
            .collect();

        Box::new(chunks.into_iter())
    }

    fn contextualize(&self, chunk: &BaseChunk) -> String {
        let mut result = String::new();

        // Add headings
        for heading in &chunk.meta.headings {
            result.push_str(heading);
            result.push('\n');
        }

        // Add caption if present
        if let Some(caption) = &chunk.meta.caption {
            result.push_str(caption);
            result.push('\n');
        }

        // Add chunk text
        result.push_str(&chunk.text);

        result
    }
}
