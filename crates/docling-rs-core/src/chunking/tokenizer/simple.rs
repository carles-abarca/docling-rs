//! Simple character-based tokenizer for CLI usage
//!
//! This tokenizer estimates token count based on character count,
//! using the approximation that 1 token ≈ 4 characters for English text.

use super::base::Tokenizer;

/// Simple tokenizer that estimates tokens based on character count
///
/// Uses the approximation: 1 token ≈ chars_per_token characters
/// Default is 4 characters per token (suitable for English text with GPT-style tokenizers)
pub struct SimpleTokenizer {
    max_tokens: usize,
    chars_per_token: usize,
}

impl SimpleTokenizer {
    /// Create a new SimpleTokenizer with default settings (4 chars/token, 512 max)
    pub fn new() -> Self {
        Self {
            max_tokens: 512,
            chars_per_token: 4,
        }
    }

    /// Create with custom max tokens
    pub fn with_max_tokens(max_tokens: usize) -> Self {
        Self {
            max_tokens,
            chars_per_token: 4,
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
        assert_eq!(tokenizer.max_tokens(), 512);

        // 20 chars / 4 = 5 tokens
        assert_eq!(tokenizer.count_tokens("hello world, test!"), 5);
    }

    #[test]
    fn test_simple_tokenizer_custom() {
        let tokenizer = SimpleTokenizer::with_max_tokens(256);
        assert_eq!(tokenizer.max_tokens(), 256);
    }

    #[test]
    fn test_empty_text() {
        let tokenizer = SimpleTokenizer::new();
        assert_eq!(tokenizer.count_tokens(""), 0);
    }
}
