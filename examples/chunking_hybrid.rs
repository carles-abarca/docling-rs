//! Hybrid chunking example with tokenizer from quickstart.md
//!
//! Run with: cargo run --example chunking_hybrid

use docling_rs::chunking::tokenizer::{HuggingFaceTokenizer, Tokenizer};
use docling_rs::chunking::{BaseChunker, HybridChunker};
use docling_rs::DocumentConverter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Hybrid Chunking with Token Limits Example ===\n");

    // Create test document
    let test_content = r#"# Machine Learning Basics

Machine learning is a subset of artificial intelligence that enables systems to learn and improve from experience without being explicitly programmed.

## Supervised Learning

Supervised learning uses labeled training data to learn the mapping between input features and output labels. Common algorithms include linear regression, logistic regression, decision trees, and neural networks.

## Unsupervised Learning

Unsupervised learning discovers patterns in unlabeled data. Popular techniques include clustering algorithms like K-means, hierarchical clustering, and dimensionality reduction methods like PCA.

## Deep Learning

Deep learning uses neural networks with multiple layers to learn hierarchical representations of data. It has revolutionized fields like computer vision and natural language processing.
"#;

    // Write to temp file
    let temp_file = "/tmp/test_hybrid_chunking.md";
    std::fs::write(temp_file, test_content)?;

    // Step 1: Convert document
    let converter = DocumentConverter::new();
    let result = converter.convert_file(temp_file)?;
    let doc = result.document();

    println!("Document converted: {}", doc.name());
    println!("Number of nodes: {}\n", doc.nodes().len());

    // Step 2: Load Tokenizer (from file for this example)
    // In production, you would use from_pretrained(), but that requires
    // either a downloaded tokenizer.json or network access

    // Create a simple tokenizer for demonstration
    // Note: In real usage, you'd use HuggingFaceTokenizer::from_file()
    // with an actual tokenizer.json file
    println!("Note: This example uses from_file() since from_pretrained()");
    println!("requires a cached tokenizer. Download tokenizer.json from:");
    println!("https://huggingface.co/sentence-transformers/all-MiniLM-L6-v2/tree/main\n");

    // For this demo, we'll create a mock tokenizer from a test file
    let tokenizer_content = r#"{
  "version": "1.0",
  "truncation": null,
  "padding": null,
  "added_tokens": [],
  "normalizer": null,
  "pre_tokenizer": {
    "type": "Whitespace"
  },
  "post_processor": null,
  "decoder": null,
  "model": {
    "type": "WordLevel",
    "vocab": {},
    "unk_token": "[UNK]"
  }
}"#;

    let tokenizer_file = "/tmp/test_tokenizer.json";
    std::fs::write(tokenizer_file, tokenizer_content)?;

    let tokenizer = HuggingFaceTokenizer::from_file(tokenizer_file)?;
    println!("Tokenizer loaded. Max tokens: {}\n", tokenizer.max_tokens());

    // Step 3: Configure Hybrid Chunker
    let chunker = HybridChunker::builder()
        .tokenizer(Box::new(tokenizer))
        .max_tokens(50) // Small limit for demo purposes
        .merge_peers(true)
        .build()?;

    // Step 4: Generate chunks with token awareness
    println!("=== Generated Chunks (Token-Aware) ===\n");

    let chunks: Vec<_> = chunker.chunk(doc).collect();
    println!("Generated {} chunks\n", chunks.len());

    for chunk in chunks {
        let contextualized = chunker.contextualize(&chunk);
        // Note: We can't easily count tokens without the tokenizer reference
        // In real usage, you'd keep a reference to the tokenizer

        println!("Chunk {}: \"{}\"", chunk.meta.index, chunk.text);
        println!("  Contextualized length: {} chars", contextualized.len());
        println!("  Headings: {:?}", chunk.meta.headings);
        println!();
    }

    // Clean up
    std::fs::remove_file(temp_file)?;
    std::fs::remove_file(tokenizer_file)?;

    println!("✓ Hybrid chunking example completed successfully!");
    println!("\nNote: For production use with real tokenizers:");
    println!("1. Download tokenizer.json from HuggingFace");
    println!("2. Use HuggingFaceTokenizer::from_file(\"path/to/tokenizer.json\")");
    println!("3. Or cache it and use from_pretrained()");

    Ok(())
}
