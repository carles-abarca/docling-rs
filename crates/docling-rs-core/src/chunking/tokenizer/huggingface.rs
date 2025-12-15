//! HuggingFace tokenizer wrapper
//!
//! This module provides a wrapper around the HuggingFace tokenizers crate
//! with a default embedded tokenizer (all-MiniLM-L6-v2) for RAG applications.

use super::base::Tokenizer;
use crate::chunking::base::ChunkingError;

/// Embedded tokenizer for sentence-transformers/all-MiniLM-L6-v2
/// This is the default tokenizer used for chunking, optimized for semantic search.
const EMBEDDED_TOKENIZER_BYTES: &[u8] =
    include_bytes!("../../../assets/tokenizer-all-MiniLM-L6-v2.json");

/// Default model name for the embedded tokenizer
pub const DEFAULT_TOKENIZER_MODEL: &str = "sentence-transformers/all-MiniLM-L6-v2";

/// Default max tokens for the embedded tokenizer (optimized for RAG retrieval)
/// 128 tokens produces more granular chunks suitable for precise semantic search
pub const DEFAULT_MAX_TOKENS: usize = 128;

/// Wrapper around HuggingFace tokenizers crate
pub struct HuggingFaceTokenizer {
    tokenizer: tokenizers::Tokenizer,
    max_tokens: usize,
}

impl HuggingFaceTokenizer {
    /// Create a tokenizer with the embedded all-MiniLM-L6-v2 model
    ///
    /// This is the recommended default for RAG applications as it uses the same
    /// tokenizer as the popular sentence-transformers embedding model.
    pub fn default_embedded() -> Result<Self, ChunkingError> {
        Self::from_bytes(EMBEDDED_TOKENIZER_BYTES, DEFAULT_MAX_TOKENS)
    }

    /// Load tokenizer from bytes (e.g., embedded or downloaded)
    pub fn from_bytes(bytes: &[u8], max_tokens: usize) -> Result<Self, ChunkingError> {
        let mut tokenizer = tokenizers::Tokenizer::from_bytes(bytes)
            .map_err(|e| ChunkingError::TokenizerLoad(format!("Failed to load tokenizer: {}", e)))?;

        // Disable truncation so we can count actual token length
        // (the default tokenizer has truncation enabled at 128 tokens)
        let _ = tokenizer.with_truncation(None);

        Ok(Self {
            tokenizer,
            max_tokens,
        })
    }

    /// Load tokenizer from HuggingFace Hub cache
    pub fn from_pretrained(model_id: &str) -> Result<Self, ChunkingError> {
        // First check if it's the default model - use embedded version
        if model_id == DEFAULT_TOKENIZER_MODEL {
            return Self::default_embedded();
        }

        let home_dir = std::env::var("HOME")
            .or_else(|_| std::env::var("USERPROFILE"))
            .unwrap_or_else(|_| ".".to_string());

        let cache_dir = std::path::Path::new(&home_dir)
            .join(".cache")
            .join("huggingface")
            .join("hub");

        let model_path = model_id.replace('/', "--");
        let potential_paths = vec![
            cache_dir
                .join(format!("models--{}", model_path))
                .join("tokenizer.json"),
            std::path::PathBuf::from(format!("./models/{}/tokenizer.json", model_id)),
            std::path::PathBuf::from(format!("./{}/tokenizer.json", model_id)),
        ];

        for path in &potential_paths {
            if path.exists() {
                return Self::from_file(path.to_str().ok_or_else(|| {
                    ChunkingError::TokenizerLoad("Invalid path encoding".to_string())
                })?);
            }
        }

        Err(ChunkingError::TokenizerLoad(format!(
            "Tokenizer for '{}' not found in cache. \
                Please download tokenizer.json from https://huggingface.co/{}/tree/main \
                and either:\n\
                1. Place it in ~/.cache/huggingface/hub/models--{}/\n\
                2. Place it in ./models/{}/\n\
                3. Use from_file() with the path to tokenizer.json",
            model_id, model_id, model_path, model_id
        )))
    }

    /// Load tokenizer from file path
    pub fn from_file(path: &str) -> Result<Self, ChunkingError> {
        let mut tokenizer = tokenizers::Tokenizer::from_file(path)
            .map_err(|e| ChunkingError::TokenizerLoad(format!("Failed to load {}: {}", path, e)))?;

        // Disable truncation so we can count actual token length
        let _ = tokenizer.with_truncation(None);

        Ok(Self {
            tokenizer,
            max_tokens: DEFAULT_MAX_TOKENS,
        })
    }

    /// Create from existing tokenizer instance
    pub fn new(tokenizer: tokenizers::Tokenizer, max_tokens: usize) -> Self {
        Self {
            tokenizer,
            max_tokens,
        }
    }

    /// Set the maximum tokens per chunk
    pub fn with_max_tokens(mut self, max_tokens: usize) -> Self {
        self.max_tokens = max_tokens;
        self
    }
}

impl Tokenizer for HuggingFaceTokenizer {
    fn count_tokens(&self, text: &str) -> usize {
        self.tokenizer
            .encode(text, false)
            .map(|enc| enc.get_ids().len())
            .unwrap_or(0)
    }

    fn max_tokens(&self) -> usize {
        self.max_tokens
    }
}
