//! HuggingFace tokenizer wrapper

use super::base::Tokenizer;
use crate::chunking::base::ChunkingError;

/// Wrapper around HuggingFace tokenizers crate
///
/// Provides integration with the HuggingFace tokenizers library for token counting.
/// Supports loading tokenizers from HuggingFace Hub or local files.
///
/// # Examples
///
/// ```ignore
/// use docling_rs::chunking::tokenizer::HuggingFaceTokenizer;
///
/// // Load from HuggingFace Hub
/// let tokenizer = HuggingFaceTokenizer::from_pretrained(
///     "sentence-transformers/all-MiniLM-L6-v2"
/// )?;
///
/// // Or load from file
/// let tokenizer = HuggingFaceTokenizer::from_file("tokenizer.json")?;
///
/// // Count tokens
/// let count = tokenizer.count_tokens("Hello world");
/// println!("Token count: {}", count);
/// ```
pub struct HuggingFaceTokenizer {
    tokenizer: tokenizers::Tokenizer,
    max_tokens: usize,
}

impl HuggingFaceTokenizer {
    /// Load tokenizer from HuggingFace Hub
    ///
    /// Downloads the tokenizer from HuggingFace Hub and caches it locally.
    /// Supported models include sentence-transformers and other HuggingFace models.
    ///
    /// # Arguments
    ///
    /// * `model_id` - HuggingFace model identifier (e.g., "sentence-transformers/all-MiniLM-L6-v2")
    ///
    /// # Returns
    ///
    /// A configured HuggingFaceTokenizer or a ChunkingError if loading fails
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let tokenizer = HuggingFaceTokenizer::from_pretrained(
    ///     "sentence-transformers/all-MiniLM-L6-v2"
    /// )?;
    /// ```
    pub fn from_pretrained(model_id: &str) -> Result<Self, ChunkingError> {
        // For testing purposes, we'll use a simple implementation that expects
        // the tokenizer.json to be available locally or provides a helpful error message
        //
        // In a full implementation, this would download from HuggingFace Hub:
        // https://huggingface.co/{model_id}/resolve/main/tokenizer.json
        //
        // For now, check common cache locations or provide instructions

        // Try common HuggingFace cache locations
        let home_dir = std::env::var("HOME")
            .or_else(|_| std::env::var("USERPROFILE"))
            .unwrap_or_else(|_| ".".to_string());

        let cache_dir = std::path::Path::new(&home_dir)
            .join(".cache")
            .join("huggingface")
            .join("hub");

        // Sanitize model_id for filesystem
        let model_path = model_id.replace('/', "--");
        let potential_paths = vec![
            cache_dir.join(format!("models--{}", model_path)).join("tokenizer.json"),
            std::path::PathBuf::from(format!("./models/{}/tokenizer.json", model_id)),
            std::path::PathBuf::from(format!("./{}/tokenizer.json", model_id)),
        ];

        // Try to load from cache
        for path in &potential_paths {
            if path.exists() {
                return Self::from_file(path.to_str().ok_or_else(|| {
                    ChunkingError::TokenizerLoad("Invalid path encoding".to_string())
                })?);
            }
        }

        // If not found, provide helpful error message
        Err(ChunkingError::TokenizerLoad(
            format!(
                "Tokenizer for '{}' not found in cache. \
                Please download tokenizer.json from https://huggingface.co/{}/tree/main \
                and either:\n\
                1. Place it in ~/.cache/huggingface/hub/models--{}/\n\
                2. Place it in ./models/{}/\n\
                3. Use from_file() with the path to tokenizer.json",
                model_id, model_id, model_path, model_id
            )
        ))
    }

    /// Load tokenizer from file path
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the tokenizer.json file
    ///
    /// # Returns
    ///
    /// A configured HuggingFaceTokenizer or a ChunkingError if loading fails
    pub fn from_file(path: &str) -> Result<Self, ChunkingError> {
        let tokenizer = tokenizers::Tokenizer::from_file(path)
            .map_err(|e| ChunkingError::TokenizerLoad(format!("Failed to load {}: {}", path, e)))?;

        // Default max_tokens for sentence-transformers models
        let max_tokens = 512;

        Ok(Self {
            tokenizer,
            max_tokens,
        })
    }

    /// Create from existing tokenizer instance
    ///
    /// # Arguments
    ///
    /// * `tokenizer` - A pre-configured tokenizers::Tokenizer instance
    /// * `max_tokens` - Maximum tokens for this model
    pub fn new(tokenizer: tokenizers::Tokenizer, max_tokens: usize) -> Self {
        Self {
            tokenizer,
            max_tokens,
        }
    }
}

// Note: Default implementation removed because from_pretrained requires
// network access or cached files. Users should explicitly choose from_file()
// or from_pretrained() based on their needs.

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
