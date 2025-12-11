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
