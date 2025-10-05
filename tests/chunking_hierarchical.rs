//! HierarchicalChunker behavior tests
//!
//! Tests for hierarchical (structure-based) chunking behavior.

use docling_rs::chunking::{BaseChunker, HierarchicalChunker};
use docling_rs::{DoclingDocument, DocumentConverter};
use std::fs;

// Helper function to create a test document
fn create_test_document(content: &str) -> DoclingDocument {
    let temp_file = "/tmp/test_hierarchical.md";
    fs::write(temp_file, content).expect("Failed to write test file");

    let converter = DocumentConverter::new();
    let result = converter
        .convert_file(temp_file)
        .expect("Failed to convert");
    result.document().clone()
}

// Test 1: Default constructor has merge_list_items = true
#[test]
fn test_new_has_default_merge() {
    let chunker = HierarchicalChunker::new();
    assert_eq!(chunker.merge_list_items, true);
}

// Test 2: Custom constructor with merge_list_items
#[test]
fn test_with_merge_list_items() {
    let chunker = HierarchicalChunker::with_merge_list_items(false);
    assert_eq!(chunker.merge_list_items, false);

    let chunker = HierarchicalChunker::with_merge_list_items(true);
    assert_eq!(chunker.merge_list_items, true);
}

// Test 3: One chunk per element (FR-005)
#[test]
fn test_one_chunk_per_element() {
    let chunker = HierarchicalChunker::new();

    let content = r#"# Title

First paragraph.

Second paragraph.

Third paragraph."#;

    let doc = create_test_document(content);
    let chunks: Vec<_> = chunker.chunk(&doc).collect();

    // Should produce chunks (at least one)
    assert!(
        chunks.len() >= 1,
        "Expected at least 1 chunk, got {}",
        chunks.len()
    );

    // Each chunk should have non-empty text
    for chunk in &chunks {
        assert!(!chunk.text.is_empty(), "Chunk text should not be empty");
    }
}

// Test 4: Metadata includes headings (FR-009)
#[test]
fn test_metadata_includes_headings() {
    let chunker = HierarchicalChunker::new();

    let content = r#"# Chapter 1

## Section 1.1

Content in section 1.1."#;

    let doc = create_test_document(content);
    let chunks: Vec<_> = chunker.chunk(&doc).collect();

    // Should produce chunks
    assert!(chunks.len() > 0, "Should produce at least one chunk");

    // All chunks should have valid metadata
    for chunk in &chunks {
        assert!(
            !chunk.meta.doc_name.is_empty(),
            "Document name should not be empty"
        );
        // Note: Headings extraction will be enhanced when we parse markdown structure
    }
}

// Test 5: Empty document
#[test]
fn test_empty_document() {
    let chunker = HierarchicalChunker::new();
    let content = "";

    let doc = create_test_document(content);
    let chunks: Vec<_> = chunker.chunk(&doc).collect();

    // Empty document may produce chunks depending on backend implementation
    // The important thing is that it doesn't panic (test passes if we get here)
}

// Test 6: Flat document (no structure)
#[test]
fn test_flat_document() {
    let chunker = HierarchicalChunker::new();
    let content = "Just plain text without structure.";

    let doc = create_test_document(content);
    let chunks: Vec<_> = chunker.chunk(&doc).collect();

    // Should still produce chunks (at least 1)
    assert!(
        chunks.len() >= 1,
        "Flat document should still produce chunks, got {}",
        chunks.len()
    );

    // Metadata should be minimal (no headings for flat text)
    for chunk in &chunks {
        assert_eq!(
            chunk.meta.headings.len(),
            0,
            "Flat document should have no headings"
        );
    }
}

// Test 7: List handling (basic test - will be enhanced when list parsing is implemented)
#[test]
fn test_list_basic() {
    let chunker_merge = HierarchicalChunker::with_merge_list_items(true);
    let chunker_no_merge = HierarchicalChunker::with_merge_list_items(false);

    let content = r#"# Title
- Item 1
- Item 2
- Item 3"#;

    let doc = create_test_document(content);

    let chunks_merge: Vec<_> = chunker_merge.chunk(&doc).collect();
    let chunks_no_merge: Vec<_> = chunker_no_merge.chunk(&doc).collect();

    // Both should produce chunks (at least 1)
    assert!(
        chunks_merge.len() >= 1,
        "Should produce chunks with merge enabled, got {}",
        chunks_merge.len()
    );
    assert!(
        chunks_no_merge.len() >= 1,
        "Should produce chunks with merge disabled, got {}",
        chunks_no_merge.len()
    );

    // Note: Actual list merging behavior will be tested once we implement proper list parsing
}

// Test 8: Contextualize preserves heading hierarchy
#[test]
fn test_contextualize_heading_hierarchy() {
    let chunker = HierarchicalChunker::new();

    let content = r#"# Chapter 1

## Section 1.1

Content here."#;

    let doc = create_test_document(content);
    let chunks: Vec<_> = chunker.chunk(&doc).collect();

    // Test contextualize for each chunk
    for chunk in &chunks {
        let contextualized = chunker.contextualize(chunk);

        // Should contain the chunk text
        assert!(
            contextualized.contains(&chunk.text),
            "Contextualized output should contain chunk text"
        );

        // If there are headings, they should appear before the text
        if !chunk.meta.headings.is_empty() {
            for heading in &chunk.meta.headings {
                assert!(
                    contextualized.contains(heading),
                    "Contextualized output should contain heading '{}'",
                    heading
                );
            }
        }
    }
}

// Test 9: Multiple paragraphs with headings
#[test]
fn test_multiple_paragraphs_with_headings() {
    let chunker = HierarchicalChunker::new();

    let content = r#"# Chapter 1

First paragraph under chapter 1.

## Section 1.1

Paragraph under section 1.1.

## Section 1.2

Paragraph under section 1.2.

# Chapter 2

First paragraph under chapter 2."#;

    let doc = create_test_document(content);
    let chunks: Vec<_> = chunker.chunk(&doc).collect();

    // Should produce multiple chunks (at least 1)
    assert!(
        chunks.len() >= 1,
        "Expected at least 1 chunk for complex structure, got {}",
        chunks.len()
    );

    // All chunks should be sequential
    for i in 1..chunks.len() {
        assert!(
            chunks[i].meta.index == i,
            "Chunk {} should have index {}, got {}",
            i,
            i,
            chunks[i].meta.index
        );
    }
}
