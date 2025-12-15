//! Simple character-based tokenizer for CLI usage
//!
//! This tokenizer estimates token count based on character count,
//! using the approximation that 1 token ≈ 3 characters for mixed content.
//!
//! The default of 3 chars/token is more conservative than the typical 4 chars/token
//! for pure English text, accounting for code, markdown syntax, and special characters
//! which tend to produce more tokens per character.

use super::base::Tokenizer;

/// Default characters per token (conservative estimate for mixed content)
pub const DEFAULT_CHARS_PER_TOKEN: usize = 3;

/// Default maximum tokens per chunk (optimized for RAG/embeddings)
/// 128 tokens produces more granular chunks suitable for precise semantic search
pub const DEFAULT_MAX_TOKENS: usize = 128;

/// Simple tokenizer that estimates tokens based on character count
///
/// Uses the approximation: 1 token ≈ chars_per_token characters
/// Default is 3 characters per token (conservative for mixed content including code/markdown)
pub struct SimpleTokenizer {
    max_tokens: usize,
    chars_per_token: usize,
}

impl SimpleTokenizer {
    /// Create a new SimpleTokenizer with default settings (3 chars/token, 256 max)
    ///
    /// These defaults are optimized for RAG/embedding use cases where smaller,
    /// more focused chunks tend to produce better retrieval results.
    pub fn new() -> Self {
        Self {
            max_tokens: DEFAULT_MAX_TOKENS,
            chars_per_token: DEFAULT_CHARS_PER_TOKEN,
        }
    }

    /// Create with custom max tokens (uses default chars_per_token)
    pub fn with_max_tokens(max_tokens: usize) -> Self {
        Self {
            max_tokens,
            chars_per_token: DEFAULT_CHARS_PER_TOKEN,
        }
    }

    /// Create with custom settings
    pub fn with_settings(max_tokens: usize, chars_per_token: usize) -> Self {
        Self {
            max_tokens,
            chars_per_token: chars_per_token.max(1), // Prevent division by zero
        }
    }
}

impl Default for SimpleTokenizer {
    fn default() -> Self {
        Self::new()
    }
}

impl Tokenizer for SimpleTokenizer {
    fn count_tokens(&self, text: &str) -> usize {
        // Estimate: 1 token ≈ chars_per_token characters
        // This is a rough approximation for English text
        (text.chars().count() + self.chars_per_token - 1) / self.chars_per_token
    }

    fn max_tokens(&self) -> usize {
        self.max_tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_tokenizer_default() {
        let tokenizer = SimpleTokenizer::new();
        assert_eq!(tokenizer.max_tokens(), DEFAULT_MAX_TOKENS);

        // 18 chars / 3 = 6 tokens
        assert_eq!(tokenizer.count_tokens("hello world, test!"), 6);
    }

    #[test]
    fn test_simple_tokenizer_custom() {
        let tokenizer = SimpleTokenizer::with_max_tokens(512);
        assert_eq!(tokenizer.max_tokens(), 512);
    }

    #[test]
    fn test_empty_text() {
        let tokenizer = SimpleTokenizer::new();
        assert_eq!(tokenizer.count_tokens(""), 0);
    }
}
