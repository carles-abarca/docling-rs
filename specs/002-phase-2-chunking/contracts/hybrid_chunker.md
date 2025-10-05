# Contract: HybridChunker

**API Type**: Struct + BaseChunker Implementation
**Module**: `docling_rs::chunking`
**Requirements**: FR-002, FR-010 to FR-017

---

## Public API

```rust
pub struct HybridChunker {
    // Private fields - use builder
}

impl HybridChunker {
    pub fn new(tokenizer: Box<dyn Tokenizer>) -> Self;
    pub fn builder() -> HybridChunkerBuilder;
}

pub struct HybridChunkerBuilder {
    // Builder pattern for configuration
}

impl HybridChunkerBuilder {
    pub fn tokenizer(self, tokenizer: Box<dyn Tokenizer>) -> Self;
    pub fn max_tokens(self, max: usize) -> Self;
    pub fn merge_peers(self, merge: bool) -> Self;
    pub fn build(self) -> Result<HybridChunker, ChunkingError>;
}
```

---

## Constructor Contract

### `new()` - Basic Constructor

```rust
pub fn new(tokenizer: Box<dyn Tokenizer>) -> Self
```

**Preconditions**:
- `tokenizer` is valid tokenizer implementation

**Postconditions**:
- Returns `HybridChunker` with:
  - `max_tokens` = tokenizer.max_tokens()
  - `merge_peers` = true (default)

**Test**:
```rust
#[test]
fn test_new_uses_tokenizer_max() {
    let tokenizer = Box::new(MockTokenizer::new(512));
    let chunker = HybridChunker::new(tokenizer);

    // Verify max_tokens matches tokenizer
    // (Internal field - test via behavior)
}
```

---

### Builder Pattern

**Test**:
```rust
#[test]
fn test_builder_pattern() {
    let tokenizer = Box::new(MockTokenizer::new(512));

    let chunker = HybridChunker::builder()
        .tokenizer(tokenizer)
        .max_tokens(256)
        .merge_peers(false)
        .build()
        .unwrap();

    // Verify configuration via chunking behavior
}
```

---

## Chunking Behavior Contracts

### Behavior 1: Two-Pass Processing (FR-010)

**Contract**: HybridChunker starts with hierarchical chunks then refines

**Test**:
```rust
#[test]
fn test_hybrid_refines_hierarchical() {
    let tokenizer = Box::new(MockTokenizer::new(100));
    let hybrid = HybridChunker::new(tokenizer);
    let hierarchical = HierarchicalChunker::new();

    let doc = create_test_document();

    let hybrid_chunks: Vec<_> = hybrid.chunk(&doc).collect();
    let hierarchical_chunks: Vec<_> = hierarchical.chunk(&doc).collect();

    // Hybrid should produce different chunking (split or merged)
    // but preserve same content coverage
    assert_ne!(hybrid_chunks.len(), hierarchical_chunks.len());
}
```

---

### Behavior 2: Split Oversized Chunks (FR-013)

**Precondition**: Document element exceeds `max_tokens` when contextualized

**Postcondition**: Element split into multiple chunks, each ≤ `max_tokens`

**Test**:
```rust
#[test]
fn test_split_oversized_chunks() {
    // Tokenizer with low max_tokens
    let tokenizer = Box::new(MockTokenizer::new(50));

    let hybrid = HybridChunker::builder()
        .tokenizer(tokenizer)
        .max_tokens(50)
        .build()
        .unwrap();

    // Create document with very long paragraph (>50 tokens)
    let long_text = "word ".repeat(100); // 100 words >> 50 tokens
    let doc = DoclingDocument::from_text(&long_text);

    let chunks: Vec<_> = hybrid.chunk(&doc).collect();

    // Should be split into multiple chunks
    assert!(chunks.len() > 1);

    // Each chunk should be <= max_tokens when contextualized
    for chunk in chunks {
        let contextualized = hybrid.contextualize(&chunk);
        let token_count = tokenizer.count_tokens(&contextualized);
        assert!(token_count <= 50);
    }
}
```

---

### Behavior 3: Merge Peers When Enabled (FR-015)

**Precondition**:
- `merge_peers = true`
- Multiple consecutive chunks with same headings/captions
- Combined size ≤ `max_tokens`

**Postcondition**: Chunks merged into single chunk

**Test**:
```rust
#[test]
fn test_merge_undersized_peers() {
    let tokenizer = Box::new(MockTokenizer::new(100));

    let hybrid = HybridChunker::builder()
        .tokenizer(tokenizer)
        .max_tokens(100)
        .merge_peers(true)
        .build()
        .unwrap();

    // Document with multiple small paragraphs under same heading
    let doc = DoclingDocument::from_markdown(
        "# Section\n\nSmall para 1.\n\nSmall para 2.\n\nSmall para 3."
    );

    let chunks: Vec<_> = hybrid.chunk(&doc).collect();

    // Should merge small paragraphs
    // Fewer chunks than hierarchical chunker would produce
    let hierarchical = HierarchicalChunker::new();
    let hier_chunks: Vec<_> = hierarchical.chunk(&doc).collect();

    assert!(chunks.len() < hier_chunks.len());
}
```

---

### Behavior 4: No Merge When Disabled (FR-016)

**Precondition**: `merge_peers = false`

**Postcondition**: Undersized chunks NOT merged

**Test**:
```rust
#[test]
fn test_no_merge_when_disabled() {
    let tokenizer = Box::new(MockTokenizer::new(100));

    let hybrid = HybridChunker::builder()
        .tokenizer(tokenizer)
        .max_tokens(100)
        .merge_peers(false)
        .build()
        .unwrap();

    let doc = DoclingDocument::from_markdown(
        "# Section\n\nSmall para 1.\n\nSmall para 2."
    );

    let chunks: Vec<_> = hybrid.chunk(&doc).collect();

    // Should NOT merge - each para separate
    assert!(chunks.len() >= 2);
}
```

---

### Behavior 5: Token Boundary Respect (FR-017)

**Contract**: When splitting, respect token boundaries (don't split mid-token)

**Test**:
```rust
#[test]
fn test_respects_token_boundaries() {
    let tokenizer = Box::new(WordTokenizer::new(20)); // 20 words max

    let hybrid = HybridChunker::builder()
        .tokenizer(Box::new(tokenizer))
        .max_tokens(20)
        .build()
        .unwrap();

    let text = (0..50).map(|i| format!("word{}", i)).collect::<Vec<_>>().join(" ");
    let doc = DoclingDocument::from_text(&text);

    let chunks: Vec<_> = hybrid.chunk(&doc).collect();

    // Verify no chunk splits a word
    for chunk in chunks {
        // All words in chunk should be complete
        assert!(!chunk.text.starts_with(" "));
        assert!(!chunk.text.ends_with(" word")); // incomplete word
    }
}
```

---

## Configuration Validation

### Invalid max_tokens

**Test**:
```rust
#[test]
fn test_max_tokens_validation() {
    let tokenizer = Box::new(MockTokenizer::new(512));

    let result = HybridChunker::builder()
        .tokenizer(tokenizer)
        .max_tokens(0) // Invalid
        .build();

    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), ChunkingError::InvalidConfig(_)));
}
```

---

## Integration Tests

### End-to-End Chunking

**Test**:
```rust
#[test]
fn test_e2e_hybrid_chunking() {
    // Use real HuggingFace tokenizer (requires network)
    let tokenizer = HuggingFaceTokenizer::from_pretrained(
        "sentence-transformers/all-MiniLM-L6-v2"
    ).expect("Failed to load tokenizer");

    let hybrid = HybridChunker::builder()
        .tokenizer(Box::new(tokenizer))
        .max_tokens(128)
        .merge_peers(true)
        .build()
        .unwrap();

    // Real document from Phase 1
    let doc = DocumentConverter::new()
        .convert_file("tests/data/sample.md")
        .unwrap()
        .document();

    let chunks: Vec<_> = hybrid.chunk(&doc).collect();

    // Assertions
    assert!(chunks.len() > 0);

    // All chunks within token limit
    for chunk in &chunks {
        let contextualized = hybrid.contextualize(chunk);
        let tokens = tokenizer.count_tokens(&contextualized);
        assert!(tokens <= 128, "Chunk {} exceeds 128 tokens", chunk.meta.index);
    }

    // Chunks are sequential
    for i in 1..chunks.len() {
        assert!(chunks[i].meta.start_offset >= chunks[i-1].meta.end_offset);
    }

    // All chunks have metadata
    for chunk in &chunks {
        assert!(!chunk.meta.doc_name.is_empty());
        assert!(chunk.meta.index < chunks.len());
    }
}
```

---

## Performance Requirements

**Memory** (NFR-001):
- O(1) per chunk (lazy iterator)
- Tokenizer may cache models (acceptable)

**Time** (NFR-002):
- O(n * k) where n = #chunks, k = avg chunk size in tokens
- Two passes over hierarchical chunks

---

## Breaking Changes

- Changing splitting algorithm (affects chunk boundaries)
- Modifying merge heuristics (affects chunk count)
- Changing default `merge_peers` value

---

**Contract Version**: 1.0
**Last Updated**: 2025-10-04
