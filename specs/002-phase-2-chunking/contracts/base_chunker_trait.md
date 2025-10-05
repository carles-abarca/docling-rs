# Contract: BaseChunker Trait

**API Type**: Trait (Abstract Interface)
**Module**: `docling_rs::chunking`
**Requirements**: FR-003, FR-023, FR-024, FR-025

---

## Trait Definition

```rust
pub trait BaseChunker {
    fn chunk<'a>(&'a self, doc: &'a DoclingDocument)
        -> Box<dyn Iterator<Item = BaseChunk> + 'a>;

    fn contextualize(&self, chunk: &BaseChunk) -> String;
}
```

---

## Contract: `chunk()` Method

### Signature
```rust
fn chunk<'a>(&'a self, doc: &'a DoclingDocument)
    -> Box<dyn Iterator<Item = BaseChunk> + 'a>;
```

### Inputs
- `doc`: Reference to a `DoclingDocument` from Phase 1

### Outputs
- Boxed iterator yielding `BaseChunk` instances

### Contract Guarantees

1. **Sequential Order** (FR-025):
   - Chunks MUST be returned in document order
   - If chunk A appears before chunk B in document, then A MUST be yielded before B

2. **Lazy Evaluation** (NFR-001, FR-034):
   - Iterator MUST be lazy (no upfront chunk generation)
   - Memory usage MUST be O(1) for iterator creation
   - Chunks generated on-demand when `next()` is called

3. **Completeness**:
   - All document content MUST be represented in chunks
   - No text content lost or duplicated (except intentional overlap in HybridChunker)

4. **Metadata Preservation** (FR-026 to FR-029):
   - Each chunk MUST include valid ChunkMetadata
   - Metadata MUST accurately reflect chunk's position and context

### Test Scenarios

**Test 1: Order Preservation**
```rust
#[test]
fn test_chunks_are_sequential() {
    let chunker = SomeChunker::new();
    let doc = create_test_document();
    let chunks: Vec<_> = chunker.chunk(&doc).collect();

    // Verify start_offset is monotonically increasing
    for i in 1..chunks.len() {
        assert!(chunks[i].meta.start_offset >= chunks[i-1].meta.end_offset);
    }

    // Verify index is sequential
    for (i, chunk) in chunks.iter().enumerate() {
        assert_eq!(chunk.meta.index, i);
    }
}
```

**Test 2: Lazy Iterator**
```rust
#[test]
fn test_iterator_is_lazy() {
    let chunker = SomeChunker::new();
    let doc = create_large_document(); // 1MB document

    // Creating iterator should be fast (no processing yet)
    let start = Instant::now();
    let iter = chunker.chunk(&doc);
    let creation_time = start.elapsed();

    assert!(creation_time < Duration::from_millis(10));
    // Actual test: iterator creation doesn't process document
}
```

**Test 3: Completeness**
```rust
#[test]
fn test_all_content_represented() {
    let chunker = SomeChunker::new();
    let doc = create_test_document();
    let original_text = doc.export_to_text(); // Phase 1 method

    let chunks: Vec<_> = chunker.chunk(&doc).collect();
    let chunked_text: String = chunks.iter()
        .map(|c| c.text.as_str())
        .collect::<Vec<_>>()
        .join("");

    // All text accounted for (ignoring whitespace differences)
    assert_eq!(
        original_text.chars().filter(|c| !c.is_whitespace()).count(),
        chunked_text.chars().filter(|c| !c.is_whitespace()).count()
    );
}
```

---

## Contract: `contextualize()` Method

### Signature
```rust
fn contextualize(&self, chunk: &BaseChunk) -> String;
```

### Inputs
- `chunk`: Reference to a `BaseChunk` to contextualize

### Outputs
- String with metadata-prefixed chunk text

### Contract Guarantees

1. **Metadata Prefix** (FR-030):
   - Output MUST include hierarchical metadata (headings, captions)
   - Metadata MUST appear before chunk text
   - Format: One heading per line, then chunk text

2. **Determinism**:
   - Same chunk → same output (pure function)
   - No side effects

3. **Non-Empty**:
   - If chunk.text is non-empty, output MUST be non-empty
   - At minimum, returns chunk.text (if no metadata)

### Output Format

**With Metadata**:
```
Heading Level 1
Heading Level 2
[caption if present]
{chunk.text}
```

**Without Metadata**:
```
{chunk.text}
```

### Test Scenarios

**Test 4: Metadata Prefixing**
```rust
#[test]
fn test_contextualize_includes_headings() {
    let chunker = SomeChunker::new();
    let chunk = BaseChunk {
        text: "This is content.".to_string(),
        meta: ChunkMetadata {
            doc_name: "test.md".to_string(),
            headings: vec!["Chapter 1".to_string(), "Section 1.1".to_string()],
            caption: None,
            start_offset: 0,
            end_offset: 16,
            index: 0,
        },
    };

    let contextualized = chunker.contextualize(&chunk);

    assert!(contextualized.contains("Chapter 1"));
    assert!(contextualized.contains("Section 1.1"));
    assert!(contextualized.contains("This is content."));

    // Headings come before content
    let chapter_pos = contextualized.find("Chapter 1").unwrap();
    let content_pos = contextualized.find("This is content.").unwrap();
    assert!(chapter_pos < content_pos);
}
```

**Test 5: Determinism**
```rust
#[test]
fn test_contextualize_is_deterministic() {
    let chunker = SomeChunker::new();
    let chunk = create_test_chunk();

    let result1 = chunker.contextualize(&chunk);
    let result2 = chunker.contextualize(&chunk);

    assert_eq!(result1, result2);
}
```

**Test 6: Empty Metadata**
```rust
#[test]
fn test_contextualize_without_metadata() {
    let chunker = SomeChunker::new();
    let chunk = BaseChunk {
        text: "Plain text.".to_string(),
        meta: ChunkMetadata {
            doc_name: "test.md".to_string(),
            headings: vec![], // No headings
            caption: None,
            start_offset: 0,
            end_offset: 11,
            index: 0,
        },
    };

    let contextualized = chunker.contextualize(&chunk);

    // Should return chunk text (possibly with document name)
    assert!(contextualized.contains("Plain text."));
}
```

---

## Implementation Requirements

All implementations of `BaseChunker` MUST:
1. Pass all 6 contract tests above
2. Document any implementation-specific behavior
3. Handle edge cases:
   - Empty documents → empty iterator
   - Documents with no structure → flat chunks
   - Very large documents → lazy processing

---

## Breaking Changes

Changes that break this contract:
- Modifying method signatures
- Changing iterator semantics (lazy → eager)
- Removing contract guarantees

Non-breaking changes:
- Adding new methods (with defaults)
- Adding optional parameters (builder pattern)

---

**Contract Version**: 1.0
**Last Updated**: 2025-10-04
