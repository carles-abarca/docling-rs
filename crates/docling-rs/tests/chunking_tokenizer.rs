//! Tokenizer trait contract tests

use docling_rs::chunking::tokenizer::Tokenizer;

// Mock tokenizer for testing
struct MockTokenizer {
    max_tokens: usize,
    chars_per_token: usize,
}

impl MockTokenizer {
    fn new(max_tokens: usize) -> Self {
        Self {
            max_tokens,
            chars_per_token: 4, // Approximate 4 chars per token
        }
    }
}

impl Tokenizer for MockTokenizer {
    fn count_tokens(&self, text: &str) -> usize {
        // Simple approximation: count by characters / 4
        text.len().div_ceil(self.chars_per_token)
    }

    fn max_tokens(&self) -> usize {
        self.max_tokens
    }
}

// Test 1: Basic token counting
#[test]
fn test_count_tokens_basic() {
    let tokenizer = MockTokenizer::new(512);

    let text = "Hello world";
    let count = tokenizer.count_tokens(text);

    // "Hello world" = 11 chars / 4 = ~3 tokens
    assert!(count > 0, "Should count tokens");
    assert!(count <= 5, "Short text should have few tokens");
}

// Test 2: Max tokens exposed
#[test]
fn test_max_tokens_exposed() {
    let tokenizer = MockTokenizer::new(512);
    assert_eq!(tokenizer.max_tokens(), 512);

    let tokenizer = MockTokenizer::new(1024);
    assert_eq!(tokenizer.max_tokens(), 1024);
}

// Test 3: Tokenizer is deterministic
#[test]
fn test_tokenizer_deterministic() {
    let tokenizer = MockTokenizer::new(512);

    let text = "This is a test sentence for tokenization.";
    let count1 = tokenizer.count_tokens(text);
    let count2 = tokenizer.count_tokens(text);

    assert_eq!(count1, count2, "Token counting should be deterministic");
}

// Test 4: Empty text
#[test]
fn test_empty_text() {
    let tokenizer = MockTokenizer::new(512);

    let count = tokenizer.count_tokens("");
    assert_eq!(count, 0, "Empty text should have 0 tokens");
}

// Test 5: Long text
#[test]
fn test_long_text() {
    let tokenizer = MockTokenizer::new(512);

    let long_text = "word ".repeat(200); // 200 words * 5 chars = 1000 chars
    let count = tokenizer.count_tokens(&long_text);

    // Should produce reasonable token count
    assert!(count > 100, "Long text should have many tokens");
    assert!(count < 500, "Token count should be reasonable");
}

// Test 6: Different max_tokens values
#[test]
fn test_different_max_tokens() {
    let tokenizer1 = MockTokenizer::new(128);
    let tokenizer2 = MockTokenizer::new(512);
    let tokenizer3 = MockTokenizer::new(2048);

    assert_eq!(tokenizer1.max_tokens(), 128);
    assert_eq!(tokenizer2.max_tokens(), 512);
    assert_eq!(tokenizer3.max_tokens(), 2048);

    // Token counting should be same regardless of max_tokens
    let text = "Test text";
    assert_eq!(
        tokenizer1.count_tokens(text),
        tokenizer2.count_tokens(text),
        "Token counting should not depend on max_tokens setting"
    );
}

// Test 7: Whitespace handling
#[test]
fn test_whitespace_handling() {
    let tokenizer = MockTokenizer::new(512);

    let text1 = "Hello world";
    let text2 = "Hello    world"; // Extra spaces
    let text3 = "Hello\nworld"; // Newline

    let count1 = tokenizer.count_tokens(text1);
    let count2 = tokenizer.count_tokens(text2);
    let count3 = tokenizer.count_tokens(text3);

    // All should produce token counts (exact values depend on implementation)
    assert!(count1 > 0);
    assert!(count2 > 0);
    assert!(count3 > 0);
}

// Test 8: Unicode handling
#[test]
fn test_unicode_handling() {
    let tokenizer = MockTokenizer::new(512);

    let text = "Hello 世界 🌍";
    let count = tokenizer.count_tokens(text);

    assert!(count > 0, "Should handle Unicode text");
}
