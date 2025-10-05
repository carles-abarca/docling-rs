# Quickstart: Document Chunking

**Feature**: Phase 2 - Document Chunking System
**Status**: Implementation Guide
**Prerequisites**: Phase 1 MVP (Document Conversion)

---

## Installation

Add to `Cargo.toml`:
```toml
[dependencies]
docling-rs = "0.2.0"  # Phase 2 version
tokenizers = "0.15"   # For HuggingFace tokenizers
```

---

## Basic Usage - Hierarchical Chunking

###Step 1: Convert a Document (Phase 1)

```rust
use docling_rs::DocumentConverter;

let converter = DocumentConverter::new();
let result = converter.convert_file("document.md")?;
let doc = result.document();
```

### Step 2: Create Hierarchical Chunker

```rust
use docling_rs::chunking::HierarchicalChunker;

let chunker = HierarchicalChunker::new(); // Default: merge_list_items = true
```

### Step 3: Generate Chunks

```rust
use docling_rs::chunking::BaseChunker;

for chunk in chunker.chunk(&doc) {
    println!("Chunk {}: {}", chunk.meta.index, chunk.text);
    println!("Headings: {:?}", chunk.meta.headings);
    println!("---");
}
```

### Step 4: Contextualize for Embedding

```rust
for chunk in chunker.chunk(&doc) {
    let contextualized = chunker.contextualize(&chunk);
    // Send to embedding model:
    // let embedding = embed_model.encode(&contextualized);
}
```

---

## Advanced Usage - Hybrid Chunking with Token Limits

### Step 1: Load Tokenizer

```rust
use docling_rs::chunking::tokenizer::HuggingFaceTokenizer;

// Load tokenizer for your embedding model
let tokenizer = HuggingFaceTokenizer::from_pretrained(
    "sentence-transformers/all-MiniLM-L6-v2"
)?;
```

### Step 2: Configure Hybrid Chunker

```rust
use docling_rs::chunking::HybridChunker;

let chunker = HybridChunker::builder()
    .tokenizer(Box::new(tokenizer))
    .max_tokens(512)      // Model's context window
    .merge_peers(true)    // Merge small chunks
    .build()?;
```

### Step 3: Chunk with Token Awareness

```rust
let doc = converter.convert_file("large_document.md")?.document();

let chunks: Vec<_> = chunker.chunk(&doc).collect();

println!("Generated {} chunks", chunks.len());

for chunk in chunks {
    let contextualized = chunker.contextualize(&chunk);
    let token_count = tokenizer.count_tokens(&contextualized);

    println!("Chunk {}: {} tokens", chunk.meta.index, token_count);
    assert!(token_count <= 512); // Guaranteed by HybridChunker
}
```

---

## Example: RAG Pipeline Integration

```rust
use docling_rs::{DocumentConverter, chunking::*};

fn prepare_chunks_for_rag(file_path: &str) -> Result<Vec<(String, ChunkMetadata)>> {
    // Step 1: Convert document
    let converter = DocumentConverter::new();
    let doc = converter.convert_file(file_path)?.document();

    // Step 2: Setup tokenizer for embedding model
    let tokenizer = HuggingFaceTokenizer::from_pretrained(
        "sentence-transformers/all-MiniLM-L6-v2"
    )?;

    // Step 3: Configure chunker
    let chunker = HybridChunker::builder()
        .tokenizer(Box::new(tokenizer))
        .max_tokens(384)  // Leave room for query tokens
        .merge_peers(true)
        .build()?;

    // Step 4: Generate contextualized chunks
    let chunks: Vec<_> = chunker.chunk(&doc)
        .map(|chunk| {
            let text = chunker.contextualize(&chunk);
            (text, chunk.meta.clone())
        })
        .collect();

    Ok(chunks)
}

// Usage
let chunks = prepare_chunks_for_rag("technical_doc.pdf")?;

for (text, meta) in chunks {
    // Embed and store in vector database
    // let embedding = embedding_model.encode(&text)?;
    // vector_db.insert(embedding, meta)?;
}
```

---

## Configuration Examples

### Custom List Handling

```rust
// Don't merge list items (each item = separate chunk)
let chunker = HierarchicalChunker::with_merge_list_items(false);
```

### Aggressive Chunking (Small Chunks)

```rust
let chunker = HybridChunker::builder()
    .tokenizer(Box::new(tokenizer))
    .max_tokens(128)      // Small chunks
    .merge_peers(false)   // No merging
    .build()?;
```

### Conservative Chunking (Large Chunks)

```rust
let chunker = HybridChunker::builder()
    .tokenizer(Box::new(tokenizer))
    .max_tokens(2048)     // Large chunks
    .merge_peers(true)    // Aggressive merging
    .build()?;
```

---

## Serialization

### Save Chunks to JSON

```rust
use serde_json;

let chunks: Vec<BaseChunk> = chunker.chunk(&doc).collect();
let json = serde_json::to_string_pretty(&chunks)?;

std::fs::write("chunks.json", json)?;
```

### Load Chunks from JSON

```rust
let json = std::fs::read_to_string("chunks.json")?;
let chunks: Vec<BaseChunk> = serde_json::from_str(&json)?;
```

---

## Testing Your Integration

### Validate Chunk Quality

```rust
fn validate_chunks(chunks: &[BaseChunk]) {
    // Check sequential order
    for i in 1..chunks.len() {
        assert!(chunks[i].meta.start_offset >= chunks[i-1].meta.end_offset);
        assert_eq!(chunks[i].meta.index, i);
    }

    // Check metadata completeness
    for chunk in chunks {
        assert!(!chunk.text.is_empty());
        assert!(!chunk.meta.doc_name.is_empty());
    }

    println!("✓ All {} chunks validated", chunks.len());
}
```

### Performance Benchmarking

```rust
use std::time::Instant;

let start = Instant::now();
let chunks: Vec<_> = chunker.chunk(&doc).collect();
let duration = start.elapsed();

println!("Chunked {} elements in {:?}", chunks.len(), duration);
println!("Rate: {:.2} chunks/sec", chunks.len() as f64 / duration.as_secs_f64());
```

---

## Troubleshooting

### Tokenizer Loading Fails

**Problem**: `Failed to load tokenizer: sentence-transformers/all-MiniLM-L6-v2`

**Solution**:
1. Check internet connection (tokenizers download from HuggingFace Hub)
2. Pre-download tokenizer: `tokenizers-cli download sentence-transformers/all-MiniLM-L6-v2`
3. Use local tokenizer file: `Tokenizer::from_file("path/to/tokenizer.json")?`

### Chunks Too Large

**Problem**: Chunks exceed embedding model's context window

**Solution**:
1. Reduce `max_tokens`: `.max_tokens(256)` instead of 512
2. Disable peer merging: `.merge_peers(false)`
3. Check contextualized size: `tokenizer.count_tokens(&chunker.contextualize(&chunk))`

### Chunks Too Small

**Problem**: Too many tiny chunks

**Solution**:
1. Increase `max_tokens`: `.max_tokens(1024)`
2. Enable peer merging: `.merge_peers(true)`
3. Use HierarchicalChunker instead of HybridChunker

---

## Next Steps

- **Phase 3**: PDF processing with OCR (coming soon)
- **Phase 4**: Advanced features (table extraction, figure handling)

---

## API Reference

Full API documentation:
```bash
cargo doc --open
```

---

**Quickstart Version**: 1.0
**Last Updated**: 2025-10-04
