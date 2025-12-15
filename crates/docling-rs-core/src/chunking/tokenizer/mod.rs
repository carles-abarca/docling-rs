//! Tokenizer abstraction module
//!
//! Provides tokenization for chunking strategies. The default tokenizer
//! is the embedded `sentence-transformers/all-MiniLM-L6-v2` which is
//! optimized for RAG/semantic search applications.

pub mod base;
pub mod huggingface;
pub mod simple;

pub use base::Tokenizer;
pub use huggingface::{HuggingFaceTokenizer, DEFAULT_MAX_TOKENS, DEFAULT_TOKENIZER_MODEL};
pub use simple::SimpleTokenizer;
