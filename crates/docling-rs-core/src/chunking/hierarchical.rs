//! Hierarchical (structure-based) chunker implementation

use super::base::{BaseChunk, BaseChunker};
use super::metadata::ChunkMetadata;
use crate::datamodel::{DoclingDocument, NodeType};

/// Creates chunks based on document structure elements
#[derive(Debug, Clone)]
pub struct HierarchicalChunker {
    /// Whether to merge list items into single chunks (default: true)
    pub merge_list_items: bool,
}

impl HierarchicalChunker {
    /// Create a new HierarchicalChunker with default settings
    pub fn new() -> Self {
        Self {
            merge_list_items: true,
        }
    }

    /// Create a new HierarchicalChunker with custom merge_list_items setting
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
    fn chunk<'a>(&'a self, doc: &'a DoclingDocument) -> Box<dyn Iterator<Item = BaseChunk> + 'a> {
        let doc_name = doc.name().to_string();
        let nodes = doc.nodes().to_vec();

        let mut current_offset = 0;
        let mut chunk_index = 0;
        let mut chunks: Vec<BaseChunk> = Vec::new();

        for node in nodes {
            // Handle tables specially - generate one chunk per row
            if node.node_type() == NodeType::Table {
                if let Some(table_data) = node.table_data() {
                    let row_chunks = table_data.rows_as_chunks();
                    for row_text in row_chunks {
                        if row_text.trim().is_empty() {
                            continue;
                        }

                        let start_offset = current_offset;
                        let end_offset = current_offset + row_text.len();
                        current_offset = end_offset + 1;

                        chunks.push(BaseChunk {
                            text: row_text,
                            meta: ChunkMetadata {
                                doc_name: doc_name.clone(),
                                headings: vec![],
                                caption: None,
                                start_offset,
                                end_offset,
                                index: chunk_index,
                            },
                        });
                        chunk_index += 1;
                    }
                }
                continue;
            }

            // Handle regular nodes
            let text = match node.text_content() {
                Some(t) => t.to_string(),
                None => continue,
            };

            if text.trim().is_empty() {
                continue;
            }

            let (start_offset, end_offset) = if let Some(pos) = node.position() {
                let start = pos.start_offset();
                let end = pos.end_offset();
                current_offset = end;
                (start, end)
            } else {
                let start = current_offset;
                let end = current_offset + text.len();
                current_offset = end + 1;
                (start, end)
            };

            chunks.push(BaseChunk {
                text,
                meta: ChunkMetadata {
                    doc_name: doc_name.clone(),
                    headings: vec![],
                    caption: None,
                    start_offset,
                    end_offset,
                    index: chunk_index,
                },
            });
            chunk_index += 1;
        }

        Box::new(chunks.into_iter())
    }

    fn contextualize(&self, chunk: &BaseChunk) -> String {
        let mut result = String::new();

        for heading in &chunk.meta.headings {
            result.push_str(heading);
            result.push('\n');
        }

        if let Some(caption) = &chunk.meta.caption {
            result.push_str(caption);
            result.push('\n');
        }

        result.push_str(&chunk.text);

        result
    }
}
