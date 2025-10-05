//! Basic chunking example from quickstart.md
//!
//! Run with: cargo run --example chunking_basic

use docling_rs::chunking::{BaseChunker, HierarchicalChunker};
use docling_rs::DocumentConverter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Step 1: Convert a Document
    println!("=== Basic Hierarchical Chunking Example ===\n");

    // Create test document
    let test_content = r#"# Introduction

This is a test document with multiple sections.

## Section 1

First paragraph in section 1.

Second paragraph in section 1.

## Section 2

Content in section 2.

### Subsection 2.1

Nested content here.
"#;

    // Write to temp file
    let temp_file = "/tmp/test_chunking.md";
    std::fs::write(temp_file, test_content)?;

    // Convert document
    let converter = DocumentConverter::new();
    let result = converter.convert_file(temp_file)?;
    let doc = result.document();

    println!("Document converted: {}", doc.name());
    println!("Number of nodes: {}\n", doc.nodes().len());

    // Step 2: Create Hierarchical Chunker
    let chunker = HierarchicalChunker::new();

    // Step 3: Generate Chunks
    println!("=== Generated Chunks ===\n");
    for chunk in chunker.chunk(&doc) {
        println!("Chunk {}: \"{}\"", chunk.meta.index, chunk.text);
        println!("  Headings: {:?}", chunk.meta.headings);
        println!(
            "  Offsets: {}-{}",
            chunk.meta.start_offset, chunk.meta.end_offset
        );
        println!();
    }

    // Step 4: Contextualize for Embedding
    println!("=== Contextualized Chunks ===\n");
    for chunk in chunker.chunk(&doc) {
        let contextualized = chunker.contextualize(&chunk);
        println!("Chunk {}: {}", chunk.meta.index, contextualized.len());
        println!("  Content: {}", contextualized.trim());
        println!();
    }

    // Clean up
    std::fs::remove_file(temp_file)?;

    println!("✓ Basic chunking example completed successfully!");

    Ok(())
}
