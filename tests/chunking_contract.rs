//! BaseChunker trait contract tests
//!
//! These tests verify the contract for all implementations of BaseChunker.

use docling_rs::chunking::{BaseChunk, BaseChunker, ChunkMetadata, HierarchicalChunker};
use docling_rs::{DoclingDocument, DocumentConverter};
use std::fs;
use std::io::Write;

// Helper function to create a test document
fn create_test_document() -> DoclingDocument {
    // Create a temporary markdown file
    let content = r#"# Chapter 1

This is the first paragraph.

This is the second paragraph.

## Section 1.1

Content in section 1.1."#;

    let temp_file = "/tmp/test_chunking.md";
    fs::write(temp_file, content).expect("Failed to write test file");

    // Convert using DocumentConverter
    let converter = DocumentConverter::new();
    let result = converter
        .convert_file(temp_file)
        .expect("Failed to convert");
    result.document().clone()
}

// Test 1: Sequential Order (FR-025)
#[test]
fn test_chunks_are_sequential() {
    let chunker = HierarchicalChunker::new();
    let doc = create_test_document();
    let chunks: Vec<_> = chunker.chunk(&doc).collect();

    // Verify chunks returned in order
    assert!(chunks.len() > 0, "Should produce chunks");

    // Verify start_offset is monotonically increasing
    for i in 1..chunks.len() {
        assert!(
            chunks[i].meta.start_offset >= chunks[i - 1].meta.end_offset,
            "Chunk {} start_offset {} should be >= chunk {} end_offset {}",
            i,
            chunks[i].meta.start_offset,
            i - 1,
            chunks[i - 1].meta.end_offset
        );
    }

    // Verify index is sequential
    for (i, chunk) in chunks.iter().enumerate() {
        assert_eq!(
            chunk.meta.index, i,
            "Chunk at position {} should have index {}",
            i, i
        );
    }
}

// Test 2: Lazy Iterator (NFR-001)
#[test]
fn test_iterator_is_lazy() {
    use std::time::{Duration, Instant};

    let chunker = HierarchicalChunker::new();
    // Create a larger document to test laziness
    let large_text = "# Heading\n\n".to_string() + &"Paragraph.\n\n".repeat(100);

    let temp_file = "/tmp/test_chunking_large.md";
    fs::write(temp_file, &large_text).expect("Failed to write test file");

    let converter = DocumentConverter::new();
    let doc = converter
        .convert_file(temp_file)
        .expect("Failed to convert")
        .document()
        .clone();

    // Creating iterator should be fast (no processing yet)
    let start = Instant::now();
    let _iter = chunker.chunk(&doc);
    let creation_time = start.elapsed();

    // Iterator creation should be very fast (< 10ms)
    assert!(
        creation_time < Duration::from_millis(10),
        "Iterator creation took {:?}, expected < 10ms",
        creation_time
    );
}

// Test 3: Completeness
#[test]
fn test_all_content_represented() {
    let chunker = HierarchicalChunker::new();
    let doc = create_test_document();

    let chunks: Vec<_> = chunker.chunk(&doc).collect();

    // Should produce non-empty chunks
    assert!(chunks.len() > 0, "Should produce at least one chunk");

    // Each chunk should have non-empty text
    for chunk in &chunks {
        assert!(!chunk.text.is_empty(), "Chunk text should not be empty");
    }

    // All chunks should have valid metadata
    for chunk in &chunks {
        assert!(
            !chunk.meta.doc_name.is_empty(),
            "Document name should not be empty"
        );
        assert!(
            chunk.meta.end_offset >= chunk.meta.start_offset,
            "End offset should be >= start offset"
        );
    }
}

// Test 4: Metadata Prefixing (FR-030)
#[test]
fn test_contextualize_includes_headings() {
    let chunker = HierarchicalChunker::new();
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

    // Verify metadata is included
    assert!(
        contextualized.contains("Chapter 1"),
        "Should contain 'Chapter 1'"
    );
    assert!(
        contextualized.contains("Section 1.1"),
        "Should contain 'Section 1.1'"
    );
    assert!(
        contextualized.contains("This is content."),
        "Should contain chunk text"
    );

    // Headings should come before content
    let chapter_pos = contextualized.find("Chapter 1").unwrap();
    let content_pos = contextualized.find("This is content.").unwrap();
    assert!(
        chapter_pos < content_pos,
        "Headings should come before content"
    );
}

// Test 5: Determinism
#[test]
fn test_contextualize_is_deterministic() {
    let chunker = HierarchicalChunker::new();
    let chunk = BaseChunk {
        text: "Test content.".to_string(),
        meta: ChunkMetadata {
            doc_name: "test.md".to_string(),
            headings: vec!["Heading".to_string()],
            caption: None,
            start_offset: 0,
            end_offset: 13,
            index: 0,
        },
    };

    let result1 = chunker.contextualize(&chunk);
    let result2 = chunker.contextualize(&chunk);

    assert_eq!(result1, result2, "contextualize() should be deterministic");
}

// Test 6: Empty Metadata
#[test]
fn test_contextualize_without_metadata() {
    let chunker = HierarchicalChunker::new();
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

    // Should return chunk text (possibly with minimal metadata)
    assert!(
        contextualized.contains("Plain text."),
        "Should contain chunk text"
    );
}
