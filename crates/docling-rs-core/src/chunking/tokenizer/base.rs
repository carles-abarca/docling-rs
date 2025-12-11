//! Base tokenizer trait

/// Abstract interface for token counting
pub trait Tokenizer: Send + Sync {
    /// Count tokens in text according to this tokenizer's algorithm
    fn count_tokens(&self, text: &str) -> usize;

    /// Maximum tokens supported by this tokenizer's model
    fn max_tokens(&self) -> usize;
}
