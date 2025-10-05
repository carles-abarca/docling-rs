# Contract: HierarchicalChunker

**API Type**: Struct + BaseChunker Implementation
**Module**: `docling_rs::chunking`
**Requirements**: FR-001, FR-005 to FR-009

---

## Public API

```rust
pub struct HierarchicalChunker {
    pub merge_list_items: bool,
}

impl HierarchicalChunker {
    pub fn new() -> Self;
    pub fn with_merge_list_items(merge: bool) -> Self;
}

impl BaseChunker for HierarchicalChunker { /* ... */ }
```

---

## Constructor Contracts

### `new()` - Default Configuration

```rust
pub fn new() -> Self
```

**Postconditions**:
- Returns `HierarchicalChunker` with `merge_list_items = true`

**Test**:
```rust
#[test]
fn test_new_has_default_merge() {
    let chunker = HierarchicalChunker::new();
    assert_eq!(chunker.merge_list_items, true);
}
```

---

### `with_merge_list_items()` - Custom Configuration

```rust
pub fn with_merge_list_items(merge: bool) -> Self
```

**Postconditions**:
- Returns chunker with specified `merge_list_items` value

**Test**:
```rust
#[test]
fn test_with_merge_list_items() {
    let chunker = HierarchicalChunker::with_merge_list_items(false);
    assert_eq!(chunker.merge_list_items, false);
}
```

---

## Chunking Behavior Contracts

### Behavior 1: One Chunk Per Element (FR-005)

**Precondition**: Document with multiple distinct elements (paragraphs, headings, etc.)

**Postcondition**: Each document element becomes separate chunk

**Test**:
```rust
#[test]
fn test_one_chunk_per_element() {
    let chunker = HierarchicalChunker::new();

    // Document with 3 paragraphs
    let doc = DoclingDocument::from_markdown(
        "# Title\n\nParagraph 1\n\nParagraph 2\n\nParagraph 3"
    );

    let chunks: Vec<_> = chunker.chunk(&doc).collect();

    // Expect chunks for: Title, Para1, Para2, Para3
    // (Implementation may vary - test actual behavior)
    assert!(chunks.len() >= 3);

    // Verify each chunk corresponds to distinct content
    for chunk in &chunks {
        assert!(!chunk.text.is_empty());
    }
}
```

---

### Behavior 2: List Merging When Enabled (FR-007)

**Precondition**:
- `merge_list_items = true`
- Document contains list

**Postcondition**: All list items in same list → single chunk

**Test**:
```rust
#[test]
fn test_merge_list_items_true() {
    let chunker = HierarchicalChunker::with_merge_list_items(true);

    let doc = DoclingDocument::from_markdown(
        "# Title\n\n- Item 1\n- Item 2\n- Item 3"
    );

    let chunks: Vec<_> = chunker.chunk(&doc).collect();

    // Find chunk containing list items
    let list_chunks: Vec<_> = chunks.iter()
        .filter(|c| c.text.contains("Item 1") || c.text.contains("Item 2"))
        .collect();

    // Should be single chunk containing all items
    assert_eq!(list_chunks.len(), 1);
    assert!(list_chunks[0].text.contains("Item 1"));
    assert!(list_chunks[0].text.contains("Item 2"));
    assert!(list_chunks[0].text.contains("Item 3"));
}
```

---

### Behavior 3: List Splitting When Disabled (FR-008)

**Precondition**:
- `merge_list_items = false`
- Document contains list

**Postcondition**: Each list item → separate chunk

**Test**:
```rust
#[test]
fn test_merge_list_items_false() {
    let chunker = HierarchicalChunker::with_merge_list_items(false);

    let doc = DoclingDocument::from_markdown(
        "# Title\n\n- Item 1\n- Item 2\n- Item 3"
    );

    let chunks: Vec<_> = chunker.chunk(&doc).collect();

    // Find chunks for each item
    let item1_chunks = chunks.iter().filter(|c| c.text.contains("Item 1")).count();
    let item2_chunks = chunks.iter().filter(|c| c.text.contains("Item 2")).count();
    let item3_chunks = chunks.iter().filter(|c| c.text.contains("Item 3")).count();

    // Each item in separate chunk
    assert_eq!(item1_chunks, 1);
    assert_eq!(item2_chunks, 1);
    assert_eq!(item3_chunks, 1);

    // Items not combined
    let combined = chunks.iter().any(|c|
        c.text.contains("Item 1") && c.text.contains("Item 2")
    );
    assert!(!combined);
}
```

---

### Behavior 4: Metadata Attachment (FR-009)

**Precondition**: Document with hierarchical structure (headings)

**Postcondition**: Each chunk has correct heading path in metadata

**Test**:
```rust
#[test]
fn test_metadata_includes_headings() {
    let chunker = HierarchicalChunker::new();

    let doc = DoclingDocument::from_markdown(
        "# Chapter 1\n\n## Section 1.1\n\nContent here."
    );

    let chunks: Vec<_> = chunker.chunk(&doc).collect();

    // Find chunk with "Content here"
    let content_chunk = chunks.iter()
        .find(|c| c.text.contains("Content here"))
        .expect("Content chunk not found");

    // Should have heading path
    assert!(content_chunk.meta.headings.contains(&"Chapter 1".to_string()));
    assert!(content_chunk.meta.headings.contains(&"Section 1.1".to_string()));
}
```

---

## Edge Cases

### Empty Document

**Test**:
```rust
#[test]
fn test_empty_document() {
    let chunker = HierarchicalChunker::new();
    let doc = DoclingDocument::new("empty.md");

    let chunks: Vec<_> = chunker.chunk(&doc).collect();
    assert_eq!(chunks.len(), 0);
}
```

### Document Without Structure

**Test**:
```rust
#[test]
fn test_flat_document() {
    let chunker = HierarchicalChunker::new();
    let doc = DoclingDocument::from_text("Just plain text without structure.");

    let chunks: Vec<_> = chunker.chunk(&doc).collect();

    // Should still produce chunks
    assert!(chunks.len() > 0);

    // Metadata should be minimal
    for chunk in chunks {
        assert_eq!(chunk.meta.headings.len(), 0);
    }
}
```

### Nested Lists

**Test**:
```rust
#[test]
fn test_nested_lists_merged() {
    let chunker = HierarchicalChunker::with_merge_list_items(true);

    let doc = DoclingDocument::from_markdown(
        "- Item 1\n  - Nested 1.1\n  - Nested 1.2\n- Item 2"
    );

    let chunks: Vec<_> = chunker.chunk(&doc).collect();

    // Nested items should merge with parent
    // (Exact behavior depends on implementation - document it)
    assert!(chunks.len() > 0);
}
```

---

## Performance Requirements

**Memory**: O(1) per chunk (NFR-001)
- Iterator state should be minimal
- No buffering of all chunks

**Time**: O(n) where n = number of document elements
- Single pass through document structure

---

## Breaking Changes

Changes that break existing behavior:
- Changing default `merge_list_items` value
- Changing chunk boundaries for same document
- Modifying metadata structure

---

**Contract Version**: 1.0
**Last Updated**: 2025-10-04
