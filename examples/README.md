# Examples

This directory contains practical examples demonstrating how to use docling-rs.

## Running Examples

```bash
# Run a specific example
cargo run --example basic_conversion

# Run with output
cargo run --example chunking_rag

# List all examples
ls examples/*.rs
```

## Available Examples

### 1. Basic Conversion (`basic_conversion.rs`)

Shows how to convert documents from different formats (Markdown, HTML, CSV).

```bash
cargo run --example basic_conversion
```

**What it demonstrates:**
- Creating a DocumentConverter
- Converting from bytes
- Working with different input formats
- Accessing conversion results

### 2. Chunking for RAG (`chunking_rag.rs`)

Demonstrates document chunking for Retrieval-Augmented Generation applications.

```bash
cargo run --example chunking_rag
```

**What it demonstrates:**
- HierarchicalChunker for structure-aware chunking
- HybridChunker for token-aware chunking
- Using HuggingFace tokenizers
- Contextualization for RAG

### 3. JSON Serialization (`json_serialization.rs`)

Shows how to serialize documents to JSON format.

```bash
cargo run --example json_serialization
```

**What it demonstrates:**
- Serializing conversion results to JSON
- Pretty-printing JSON output
- Working with serde

## Example Data

All examples use inline data for simplicity. For working with files:

```rust
use docling_rs::DocumentConverter;

let converter = DocumentConverter::new();
let result = converter.convert_file("path/to/document.md")?;
```

## More Information

- See the [main README](../README.md) for library overview
- See the [API documentation](https://docs.rs/docling-rs) for detailed API reference
- See [tests/](../tests/) for more usage examples
