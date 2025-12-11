//! Hybrid (structure + tokenization-aware) chunker implementation

use super::base::{BaseChunk, BaseChunker, ChunkingError};
use super::hierarchical::HierarchicalChunker;
use super::metadata::ChunkMetadata;
use super::tokenizer::Tokenizer;
use crate::datamodel::DoclingDocument;

/// Advanced chunker combining structure + tokenization awareness
pub struct HybridChunker {
    tokenizer: Box<dyn Tokenizer>,
    max_tokens: usize,
    merge_peers: bool,
    hierarchical: HierarchicalChunker,
}

impl HybridChunker {
    /// Create a new HybridChunker with default settings
    pub fn new(tokenizer: Box<dyn Tokenizer>) -> Self {
        let max_tokens = tokenizer.max_tokens();
        Self {
            tokenizer,
            max_tokens,
            merge_peers: true,
            hierarchical: HierarchicalChunker::new(),
        }
    }

    /// Create a builder for configuring HybridChunker
    pub fn builder() -> HybridChunkerBuilder {
        HybridChunkerBuilder::new()
    }
}

/// Builder for configuring HybridChunker
pub struct HybridChunkerBuilder {
    tokenizer: Option<Box<dyn Tokenizer>>,
    max_tokens: Option<usize>,
    merge_peers: bool,
}

impl HybridChunkerBuilder {
    fn new() -> Self {
        Self {
            tokenizer: None,
            max_tokens: None,
            merge_peers: true,
        }
    }

    /// Set the tokenizer for token counting
    pub fn tokenizer(mut self, tokenizer: Box<dyn Tokenizer>) -> Self {
        self.tokenizer = Some(tokenizer);
        self
    }

    /// Set the maximum tokens per chunk
    pub fn max_tokens(mut self, max: usize) -> Self {
        self.max_tokens = Some(max);
        self
    }

    /// Set whether to merge undersized peer chunks
    pub fn merge_peers(mut self, merge: bool) -> Self {
        self.merge_peers = merge;
        self
    }

    /// Build the HybridChunker
    pub fn build(self) -> Result<HybridChunker, ChunkingError> {
        let tokenizer = self
            .tokenizer
            .ok_or_else(|| ChunkingError::InvalidConfig("tokenizer is required".to_string()))?;

        let max_tokens = self.max_tokens.unwrap_or_else(|| tokenizer.max_tokens());

        if max_tokens == 0 {
            return Err(ChunkingError::InvalidConfig(
                "max_tokens must be greater than 0".to_string(),
            ));
        }

        Ok(HybridChunker {
            tokenizer,
            max_tokens,
            merge_peers: self.merge_peers,
            hierarchical: HierarchicalChunker::new(),
        })
    }
}

impl HybridChunker {
    fn split_oversized_chunk(&self, chunk: BaseChunk) -> Vec<BaseChunk> {
        let contextualized = self.contextualize(&chunk);
        let token_count = self.tokenizer.count_tokens(&contextualized);

        if token_count <= self.max_tokens {
            return vec![chunk];
        }

        let text = &chunk.text;
        let words: Vec<&str> = text.split_whitespace().collect();

        if words.is_empty() {
            return vec![chunk];
        }

        let mut result = Vec::new();
        let mut current_text = String::new();
        let mut current_start = chunk.meta.start_offset;
        let mut chunk_index = chunk.meta.index;

        for word in words {
            let test_text = if current_text.is_empty() {
                word.to_string()
            } else {
                format!("{} {}", current_text, word)
            };

            let test_chunk = BaseChunk {
                text: test_text.clone(),
                meta: chunk.meta.clone(),
            };
            let test_contextualized = self.contextualize(&test_chunk);
            let test_tokens = self.tokenizer.count_tokens(&test_contextualized);

            if test_tokens > self.max_tokens && !current_text.is_empty() {
                let end_offset = current_start + current_text.len();
                result.push(BaseChunk {
                    text: current_text.clone(),
                    meta: ChunkMetadata {
                        doc_name: chunk.meta.doc_name.clone(),
                        headings: chunk.meta.headings.clone(),
                        caption: chunk.meta.caption.clone(),
                        start_offset: current_start,
                        end_offset,
                        index: chunk_index,
                    },
                });
                chunk_index += 1;
                current_text = word.to_string();
                current_start = end_offset + 1;
            } else {
                current_text = test_text;
            }
        }

        if !current_text.is_empty() {
            let end_offset = current_start + current_text.len();
            result.push(BaseChunk {
                text: current_text,
                meta: ChunkMetadata {
                    doc_name: chunk.meta.doc_name.clone(),
                    headings: chunk.meta.headings.clone(),
                    caption: chunk.meta.caption.clone(),
                    start_offset: current_start,
                    end_offset,
                    index: chunk_index,
                },
            });
        }

        result
    }

    fn merge_undersized_peers(&self, chunks: Vec<BaseChunk>) -> Vec<BaseChunk> {
        if !self.merge_peers || chunks.is_empty() {
            return chunks;
        }

        let mut result = Vec::new();
        let mut current: Option<BaseChunk> = None;

        for chunk in chunks {
            match current.take() {
                None => {
                    current = Some(chunk);
                }
                Some(mut prev) => {
                    let can_merge = prev.meta.headings == chunk.meta.headings
                        && prev.meta.caption == chunk.meta.caption;

                    if can_merge {
                        let merged_text = format!("{} {}", prev.text, chunk.text);
                        let test_chunk = BaseChunk {
                            text: merged_text.clone(),
                            meta: prev.meta.clone(),
                        };
                        let contextualized = self.contextualize(&test_chunk);
                        let token_count = self.tokenizer.count_tokens(&contextualized);

                        if token_count <= self.max_tokens {
                            prev.text = merged_text;
                            prev.meta.end_offset = chunk.meta.end_offset;
                            current = Some(prev);
                        } else {
                            result.push(prev);
                            current = Some(chunk);
                        }
                    } else {
                        result.push(prev);
                        current = Some(chunk);
                    }
                }
            }
        }

        if let Some(chunk) = current {
            result.push(chunk);
        }

        for (i, chunk) in result.iter_mut().enumerate() {
            chunk.meta.index = i;
        }

        result
    }
}

impl BaseChunker for HybridChunker {
    fn chunk<'a>(&'a self, doc: &'a DoclingDocument) -> Box<dyn Iterator<Item = BaseChunk> + 'a> {
        let hierarchical_chunks: Vec<BaseChunk> = self.hierarchical.chunk(doc).collect();

        let mut split_chunks = Vec::new();
        for chunk in hierarchical_chunks {
            let mut chunks = self.split_oversized_chunk(chunk);
            split_chunks.append(&mut chunks);
        }

        let final_chunks = self.merge_undersized_peers(split_chunks);

        Box::new(final_chunks.into_iter())
    }

    fn contextualize(&self, chunk: &BaseChunk) -> String {
        self.hierarchical.contextualize(chunk)
    }
}
