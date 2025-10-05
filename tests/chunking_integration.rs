//! Integration tests for document chunking

use docling_rs::chunking::{BaseChunk, BaseChunker, HierarchicalChunker};
use docling_rs::{DoclingDocument, DocumentConverter};
use std::fs;

// Helper to create test document
fn create_test_document(content: &str) -> DoclingDocument {
    use std::time::{SystemTime, UNIX_EPOCH};

    // Create unique temp file to avoid race conditions
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let temp_file = format!("/tmp/test_integration_{}.md", timestamp);

    fs::write(&temp_file, content).expect("Failed to write test file");

    let converter = DocumentConverter::new();
    let result = converter
        .convert_file(&temp_file)
        .expect("Failed to convert");

    // Clean up
    let _ = fs::remove_file(&temp_file);

    result.document().clone()
}

// Test 1: Basic hierarchical chunking (from quickstart.md)
#[test]
fn test_basic_hierarchical_chunking() {
    // Step 1: Convert a document
    let content = r#"# IBM

International Business Machines Corporation (IBM) is an American multinational technology company.

## History

### 1910s–1950s

IBM originated with several technological innovations developed by Herman Hollerith."#;

    let doc = create_test_document(content);

    // Step 2: Create hierarchical chunker
    let chunker = HierarchicalChunker::new();

    // Step 3: Generate chunks
    let chunks: Vec<_> = chunker.chunk(&doc).collect();

    // Verify chunks were generated (at least 1)
    assert!(
        chunks.len() >= 1,
        "Should generate chunks, got {}",
        chunks.len()
    );

    // Verify all chunks have metadata
    for chunk in &chunks {
        assert!(!chunk.meta.doc_name.is_empty(), "Should have document name");
        assert!(
            chunk.meta.end_offset >= chunk.meta.start_offset,
            "Should have valid offsets"
        );
    }

    // Step 4: Test contextualize
    for chunk in &chunks {
        let contextualized = chunker.contextualize(chunk);
        assert!(
            !contextualized.is_empty(),
            "Contextualized output should not be empty"
        );
        assert!(
            contextualized.contains(&chunk.text),
            "Should contain chunk text"
        );
    }
}

// Test 2: End-to-end workflow
#[test]
fn test_end_to_end_workflow() {
    // Simulate a complete workflow from document to chunks
    let content = r#"# Technical Documentation

## Introduction

This document describes the system architecture.

## Components

### Frontend

The frontend is built with React.

### Backend

The backend uses Rust and PostgreSQL.

## Deployment

The system is deployed on AWS."#;

    let doc = create_test_document(content);
    let chunker = HierarchicalChunker::new();

    let chunks: Vec<_> = chunker.chunk(&doc).collect();

    // Should produce multiple chunks
    assert!(
        chunks.len() >= 5,
        "Complex document should produce multiple chunks"
    );

    // All chunks should be sequential
    for i in 1..chunks.len() {
        assert_eq!(chunks[i].meta.index, i, "Chunks should be sequential");
        assert!(
            chunks[i].meta.start_offset >= chunks[i - 1].meta.end_offset,
            "Chunks should not overlap"
        );
    }

    // Test serialization (FR-035, FR-036, FR-037)
    let first_chunk = &chunks[0];
    let json = serde_json::to_string(first_chunk).expect("Should serialize to JSON");
    assert!(json.contains("text"), "JSON should contain text field");
    assert!(json.contains("meta"), "JSON should contain meta field");

    // Test deserialization
    let deserialized: BaseChunk =
        serde_json::from_str(&json).expect("Should deserialize from JSON");
    assert_eq!(deserialized.text, first_chunk.text, "Text should match");
    assert_eq!(
        deserialized.meta.doc_name, first_chunk.meta.doc_name,
        "Document name should match"
    );
}

// Test 3: Serialization round-trip
#[test]
fn test_serialization_roundtrip() {
    let content = r#"# Document

This is a test paragraph."#;

    let doc = create_test_document(content);
    let chunker = HierarchicalChunker::new();
    let chunks: Vec<_> = chunker.chunk(&doc).collect();

    // Serialize all chunks
    let json = serde_json::to_string(&chunks).expect("Should serialize chunks");

    // Deserialize
    let deserialized: Vec<BaseChunk> =
        serde_json::from_str(&json).expect("Should deserialize chunks");

    // Verify round-trip
    assert_eq!(
        deserialized.len(),
        chunks.len(),
        "Should have same number of chunks"
    );

    for (i, (original, restored)) in chunks.iter().zip(deserialized.iter()).enumerate() {
        assert_eq!(
            original.text, restored.text,
            "Chunk {} text should match",
            i
        );
        assert_eq!(
            original.meta.doc_name, restored.meta.doc_name,
            "Chunk {} doc_name should match",
            i
        );
        assert_eq!(
            original.meta.index, restored.meta.index,
            "Chunk {} index should match",
            i
        );
    }
}

// Test 4: Multiple document formats
#[test]
fn test_multiple_formats() {
    // Test with different markdown structures
    let test_cases = vec![
        ("# Simple heading\n\nParagraph.", "simple"),
        ("Plain text without headings.", "plain"),
        ("# H1\n## H2\n### H3\nNested headings.", "nested"),
        ("- List item 1\n- List item 2", "list"),
    ];

    for (content, name) in test_cases {
        let doc = create_test_document(content);
        let chunker = HierarchicalChunker::new();
        let chunks: Vec<_> = chunker.chunk(&doc).collect();

        assert!(
            chunks.len() > 0,
            "Document '{}' should produce chunks",
            name
        );

        // All chunks should have valid metadata
        for chunk in &chunks {
            assert!(!chunk.meta.doc_name.is_empty());
            assert!(chunk.meta.end_offset >= chunk.meta.start_offset);
        }
    }
}

// Test 5: Contextualize for RAG workflow
#[test]
fn test_contextualize_for_rag() {
    let content = r#"# Product Manual

## Safety Instructions

Always wear protective equipment.

## Operating Procedures

### Startup

Turn on the main power switch."#;

    let doc = create_test_document(content);
    let chunker = HierarchicalChunker::new();
    let chunks: Vec<_> = chunker.chunk(&doc).collect();

    // Prepare chunks for RAG (like in quickstart.md)
    let contextualized_chunks: Vec<_> = chunks
        .iter()
        .map(|chunk| {
            let text = chunker.contextualize(chunk);
            (text, chunk.meta.clone())
        })
        .collect();

    assert!(
        contextualized_chunks.len() > 0,
        "Should produce contextualized chunks"
    );

    // Verify contextualized text is usable
    for (text, meta) in &contextualized_chunks {
        assert!(!text.is_empty(), "Contextualized text should not be empty");
        assert!(!meta.doc_name.is_empty(), "Metadata should be preserved");
    }
}

// Test 6: Large document handling
#[test]
fn test_large_document() {
    // Create a larger document
    let mut content = String::from("# Large Document\n\n");
    for i in 0..50 {
        content.push_str(&format!("## Section {}\n\n", i));
        content.push_str(&format!("This is paragraph {} in the document.\n\n", i));
    }

    let doc = create_test_document(&content);
    let chunker = HierarchicalChunker::new();

    // Use lazy iterator - should not consume much memory
    let chunk_count = chunker.chunk(&doc).count();

    // Should produce at least some chunks (exact count depends on backend implementation)
    assert!(
        chunk_count >= 1,
        "Should produce chunks for large document, got {}",
        chunk_count
    );
}

// Test 7: Metadata preservation
#[test]
fn test_metadata_preservation() {
    let content = r#"# Chapter

Content here."#;

    let doc = create_test_document(content);
    let chunker = HierarchicalChunker::new();
    let chunks: Vec<_> = chunker.chunk(&doc).collect();

    // Serialize and verify metadata is preserved
    let json = serde_json::to_string_pretty(&chunks).expect("Should serialize");

    assert!(json.contains("doc_name"), "Should preserve doc_name");
    assert!(json.contains("headings"), "Should preserve headings");
    assert!(
        json.contains("start_offset"),
        "Should preserve start_offset"
    );
    assert!(json.contains("end_offset"), "Should preserve end_offset");
    assert!(json.contains("index"), "Should preserve index");
}
