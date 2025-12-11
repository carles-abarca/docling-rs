//! HuggingFace tokenizer wrapper

use super::base::Tokenizer;
use crate::chunking::base::ChunkingError;

/// Wrapper around HuggingFace tokenizers crate
pub struct HuggingFaceTokenizer {
    tokenizer: tokenizers::Tokenizer,
    max_tokens: usize,
}

impl HuggingFaceTokenizer {
    /// Load tokenizer from HuggingFace Hub
    pub fn from_pretrained(model_id: &str) -> Result<Self, ChunkingError> {
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
        let tokenizer = tokenizers::Tokenizer::from_file(path)
            .map_err(|e| ChunkingError::TokenizerLoad(format!("Failed to load {}: {}", path, e)))?;

        let max_tokens = 512;

        Ok(Self {
            tokenizer,
            max_tokens,
        })
    }

    /// Create from existing tokenizer instance
    pub fn new(tokenizer: tokenizers::Tokenizer, max_tokens: usize) -> Self {
        Self {
            tokenizer,
            max_tokens,
        }
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
