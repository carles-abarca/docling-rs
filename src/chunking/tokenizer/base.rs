//! Base tokenizer trait

/// Abstract interface for token counting
///
/// This trait provides an abstraction over different tokenization strategies,
/// allowing the chunking system to work with various embedding models and their
/// associated tokenizers (e.g., HuggingFace, OpenAI, etc.).
///
/// # Contract
///
/// Implementations must guarantee:
/// - `count_tokens()` is deterministic (same input → same output)
/// - `max_tokens()` returns the model's token limit
pub trait Tokenizer: Send + Sync {
    /// Count tokens in text according to this tokenizer's algorithm
    ///
    /// # Arguments
    ///
    /// * `text` - The text to tokenize
    ///
    /// # Returns
    ///
    /// The number of tokens in the text
    fn count_tokens(&self, text: &str) -> usize;

    /// Maximum tokens supported by this tokenizer's model
    ///
    /// # Returns
    ///
    /// The maximum context window size in tokens
    fn max_tokens(&self) -> usize;
}
