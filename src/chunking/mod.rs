//! Document chunking module
//!
//! This module provides intelligent chunking strategies for converting
//! [`DoclingDocument`](crate::datamodel::DoclingDocument) instances into semantically meaningful chunks
//! suitable for RAG (Retrieval-Augmented Generation) applications.
//!
//! # Overview
//!
//! The chunking system offers two main strategies:
//!
//! - **[`HierarchicalChunker`]**: Structure-based chunking that preserves document hierarchy
//! - **[`HybridChunker`]**: Advanced chunking with token-awareness for embedding models
//!
//! # Examples
//!
//! ## Basic Hierarchical Chunking
//!
//! ```ignore
//! use docling_rs::{DocumentConverter, chunking::{HierarchicalChunker, BaseChunker}};
//!
//! let converter = DocumentConverter::new();
//! let result = converter.convert_file("document.md")?;
//! let doc = result.document();
//!
//! let chunker = HierarchicalChunker::new();
//! for chunk in chunker.chunk(&doc) {
//!     println!("Chunk {}: {}", chunk.meta.index, chunk.text);
//! }
//! ```
//!
//! ## Hybrid Chunking with Token Limits
//!
//! ```ignore
//! use docling_rs::chunking::{HybridChunker, tokenizer::HuggingFaceTokenizer, BaseChunker};
//!
//! let tokenizer = Box::new(HuggingFaceTokenizer::from_file("tokenizer.json")?);
//! let chunker = HybridChunker::builder()
//!     .tokenizer(tokenizer)
//!     .max_tokens(512)
//!     .merge_peers(true)
//!     .build()?;
//!
//! let chunks: Vec<_> = chunker.chunk(&doc).collect();
//! ```

pub mod base;
pub mod hierarchical;
pub mod hybrid;
pub mod metadata;
pub mod tokenizer;

pub use base::{BaseChunk, BaseChunker, ChunkingError};
pub use hierarchical::HierarchicalChunker;
pub use hybrid::{HybridChunker, HybridChunkerBuilder};
pub use metadata::ChunkMetadata;
pub use tokenizer::{HuggingFaceTokenizer, Tokenizer};
