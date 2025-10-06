//! Document Chunking for RAG Applications
//!
//! This example demonstrates how to chunk documents for Retrieval-Augmented Generation (RAG).
//!
//! Run with:
//! ```bash
//! cargo run --example chunking_rag
//! ```

use docling_rs::chunking::{BaseChunker, HierarchicalChunker, HybridChunker};
use docling_rs::chunking::tokenizer::HuggingFaceTokenizer;
use docling_rs::DocumentConverter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Document Chunking for RAG ===\n");

    // 1. Convert a document
    let converter = DocumentConverter::new();
    let markdown = r#"# Introduction to Rust

Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety.

## Memory Safety

Rust achieves memory safety without garbage collection through its ownership system.

### Ownership Rules

1. Each value in Rust has a variable that's called its owner
2. There can only be one owner at a time
3. When the owner goes out of scope, the value will be dropped

## Performance

Rust is designed to be fast and efficient, with zero-cost abstractions."#;

    let result = converter.convert_bytes(
        markdown.as_bytes().to_vec(),
        "rust_guide.md".to_string(),
        docling_rs::InputFormat::Markdown,
    )?;

    let doc = result.document();
    println!("Original document: {} ({} nodes)\n", doc.name(), doc.nodes().len());

    // 2. Hierarchical Chunking (simple, preserves structure)
    println!("--- Hierarchical Chunking ---");
    let hierarchical = HierarchicalChunker::new();
    let chunks: Vec<_> = hierarchical.chunk(doc).collect();
    
    println!("Generated {} chunks:", chunks.len());
    for (i, chunk) in chunks.iter().enumerate() {
        println!("  Chunk {}: {} chars", i + 1, chunk.text.len());
        println!("    Headings: {:?}", chunk.meta.headings);
        let preview = chunk.text.chars().take(60).collect::<String>().replace('\n', " ");
        println!("    Text preview: {}...\n", preview);
    }

    // 3. Hybrid Chunking (advanced, token-aware)
    println!("\n--- Hybrid Chunking (Token-Aware) ---");
    let tokenizer = Box::new(HuggingFaceTokenizer::from_pretrained("bert-base-uncased")?);
    let hybrid = HybridChunker::builder()
        .tokenizer(tokenizer)
        .max_tokens(100)
        .merge_peers(true)
        .build()?;
    
    let chunks: Vec<_> = hybrid.chunk(doc).collect();
    println!("Generated {} chunks (max 100 tokens each):", chunks.len());
    for (i, chunk) in chunks.iter().enumerate() {
        println!("  Chunk {}: {} chars", i + 1, chunk.text.len());
        let context = hybrid.contextualize(&chunk);
        let context_preview = context.chars().take(80).collect::<String>().replace('\n', " ");
        println!("    Context: {}\n", context_preview);
    }

    Ok(())
}
