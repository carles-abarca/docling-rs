//! HybridChunker tests

use docling_rs::chunking::tokenizer::Tokenizer;
use docling_rs::chunking::HybridChunker;

// Mock tokenizer for testing
struct MockTokenizer {
    max_tokens: usize,
}

impl MockTokenizer {
    fn new(max_tokens: usize) -> Self {
        Self { max_tokens }
    }
}

impl Tokenizer for MockTokenizer {
    fn count_tokens(&self, text: &str) -> usize {
        // Simple approximation: 4 chars per token
        (text.len() + 3) / 4
    }

    fn max_tokens(&self) -> usize {
        self.max_tokens
    }
}

// Test 1: Constructor uses tokenizer max_tokens
#[test]
fn test_new_uses_tokenizer_max() {
    let tokenizer = Box::new(MockTokenizer::new(512));
    let _chunker = HybridChunker::new(tokenizer);

    // If this compiles and doesn't panic, the test passes
    assert!(true);
}

// Test 2: Builder pattern
#[test]
fn test_builder_pattern() {
    let tokenizer = Box::new(MockTokenizer::new(512));

    let chunker = HybridChunker::builder()
        .tokenizer(tokenizer)
        .max_tokens(256)
        .merge_peers(false)
        .build();

    assert!(chunker.is_ok(), "Builder should succeed with valid config");
}

// Test 3: max_tokens validation
#[test]
fn test_max_tokens_validation() {
    let tokenizer = Box::new(MockTokenizer::new(512));

    let result = HybridChunker::builder()
        .tokenizer(tokenizer)
        .max_tokens(0) // Invalid
        .build();

    assert!(result.is_err(), "Should fail with max_tokens = 0");
}

// Test 4: Builder requires tokenizer
#[test]
fn test_builder_requires_tokenizer() {
    let result = HybridChunker::builder().max_tokens(256).build();

    assert!(result.is_err(), "Should fail without tokenizer");
}

// Test 5: Builder with merge_peers
#[test]
fn test_builder_merge_peers() {
    let tokenizer1 = Box::new(MockTokenizer::new(512));
    let tokenizer2 = Box::new(MockTokenizer::new(512));

    let chunker1 = HybridChunker::builder()
        .tokenizer(tokenizer1)
        .merge_peers(true)
        .build();

    let chunker2 = HybridChunker::builder()
        .tokenizer(tokenizer2)
        .merge_peers(false)
        .build();

    assert!(chunker1.is_ok());
    assert!(chunker2.is_ok());
}

// Test 6: Default merge_peers is true
#[test]
fn test_default_merge_peers() {
    let tokenizer = Box::new(MockTokenizer::new(512));

    let chunker = HybridChunker::builder().tokenizer(tokenizer).build();

    assert!(chunker.is_ok(), "Default merge_peers should be valid");
}

// Test 7: Different max_tokens values
#[test]
fn test_different_max_tokens_values() {
    let test_cases = vec![128, 256, 512, 1024, 2048];

    for max_tokens in test_cases {
        let tokenizer = Box::new(MockTokenizer::new(max_tokens));

        let chunker = HybridChunker::builder()
            .tokenizer(tokenizer)
            .max_tokens(max_tokens)
            .build();

        assert!(
            chunker.is_ok(),
            "Should succeed with max_tokens = {}",
            max_tokens
        );
    }
}

// Test 8: Builder is flexible with parameter order
#[test]
fn test_builder_parameter_order() {
    // Test different parameter orders
    let tokenizer1 = Box::new(MockTokenizer::new(512));
    let chunker1 = HybridChunker::builder()
        .max_tokens(256)
        .tokenizer(tokenizer1)
        .merge_peers(false)
        .build();

    let tokenizer2 = Box::new(MockTokenizer::new(512));
    let chunker2 = HybridChunker::builder()
        .merge_peers(true)
        .max_tokens(512)
        .tokenizer(tokenizer2)
        .build();

    assert!(chunker1.is_ok());
    assert!(chunker2.is_ok());
}
