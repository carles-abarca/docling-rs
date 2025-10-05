# Docling-rs

Native Rust document processing library for extracting structured text and metadata from common document formats.

## Features

- **Multi-format Support**: Markdown, HTML, CSV, DOCX, and PDF
- **Unified Document Model**: All formats convert to a common `DoclingDocument` structure
- **Pure Rust**: No Python dependencies, native performance
- **Cross-platform**: Works on Windows and macOS (with bundled PDF libraries)
- **Type-safe**: Leverages Rust's type system for reliability
- **Serialization**: JSON serialization/deserialization via serde
- **Batteries Included**: PDF support with bundled pdfium binaries (no external dependencies needed)

## Status

✅ **Phase 1 MVP Complete** - Core library with 4 format backends and 59 passing tests

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
docling-rs = "0.1.0"
```

## Quick Start

### Converting from a File

```rust
use docling_rs::DocumentConverter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let converter = DocumentConverter::new();

    // Auto-detects format from extension
    let result = converter.convert_file("document.md")?;

    println!("Document: {}", result.document().name());
    println!("Status: {:?}", result.status());

    // Serialize to JSON
    let json = serde_json::to_string_pretty(&result)?;
    println!("{}", json);

    Ok(())
}
```

### Converting from Bytes

```rust
use docling_rs::{DocumentConverter, InputFormat};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let converter = DocumentConverter::new();

    let markdown = b"# Hello World\n\nThis is a test.";
    let result = converter.convert_bytes(
        markdown.to_vec(),
        "test.md".to_string(),
        InputFormat::Markdown,
    )?;

    println!("Converted: {}", result.document().name());

    Ok(())
}
```

### Working with Different Formats

```rust
use docling_rs::{DocumentConverter, InputFormat};

let converter = DocumentConverter::new();

// Markdown
let md_result = converter.convert_bytes(
    b"# Title\n\nParagraph".to_vec(),
    "doc.md".to_string(),
    InputFormat::Markdown,
)?;

// HTML
let html_result = converter.convert_bytes(
    b"<html><body><h1>Title</h1></body></html>".to_vec(),
    "doc.html".to_string(),
    InputFormat::Html,
)?;

// CSV
let csv_result = converter.convert_bytes(
    b"Name,Age\nAlice,30\nBob,25\n".to_vec(),
    "data.csv".to_string(),
    InputFormat::Csv,
)?;
```

## Supported Formats

- ✅ Markdown (CommonMark)
- ✅ HTML
- ✅ CSV
- ✅ DOCX (Microsoft Word)
- 🔜 PDF (planned for Phase 3)

## Development

### Prerequisites

- Rust 1.75 or later

### Building

```bash
cargo build
```

### Testing

```bash
# Run all tests (note: PDF tests require single-threaded execution due to pdfium)
cargo test -- --test-threads=1

# Run specific test suite
cargo test --test integration_pdf_multipage -- --test-threads=1
```

**Important**: PDF-related tests must be run with `--test-threads=1` due to pdfium's thread-safety requirements.

### Manual Testing with Real Documents

To manually test the CLI with real-world documents:

```bash
./scripts/test-cli-manual.sh
```

This script processes all documents in `tests/documents-test/` and displays:
- Text extraction results
- Chunking results
- Processing times
- Summary statistics

See [quickstart guide](specs/006-cli-manual-testing/quickstart.md) for details.

### Linting

```bash
cargo clippy
cargo fmt --check
```

## Architecture

See `specs/001-vamos-a-empezar/` for detailed design documentation:
- `plan.md` - Implementation plan
- `data-model.md` - Data structures
- `contracts/` - Backend contracts
- `tasks.md` - Task breakdown

## License

MIT

## Contributing

This project follows Test-Driven Development (TDD). See `CLAUDE.md` for development guidelines.

## CLI Usage

The `docling-rs` CLI provides powerful document conversion capabilities.

### Installation

```bash
cargo install docling-rs
```

### Basic Conversion

```bash
# Convert single file
docling-rs document.md

# Convert to specific format
docling-rs document.pdf --to json

# Batch convert directory
docling-rs docs/ --output-dir output/

# With format filtering
docling-rs docs/ --from markdown,html --to json
```

### Document Chunking

Enable intelligent document chunking for RAG applications:

```bash
# Chunk document (default: hierarchical chunking)
docling-rs document.md --chunk

# Output as JSON for easy processing
docling-rs document.pdf --chunk --to json

# Batch chunking
docling-rs docs/ --chunk --to json --output-dir chunks/
```

Chunking automatically:
- Preserves document structure
- Maintains heading hierarchy
- Includes metadata (headings, offsets, indices)
- Outputs semantically coherent chunks

### CLI Options

```
Options:
  -t, --to <FORMAT>          Output format (markdown, json, text) [default: markdown]
  -o, --output-dir <DIR>     Output directory
  -f, --from <FORMAT>        Filter input files by format (batch mode)
      --chunk                Enable document chunking
      --chunk-size <SIZE>    Chunk size in characters [default: 1000]
      --ocr-enabled          Enable OCR for scanned PDFs
      --continue-on-error    Continue processing on error (batch mode)
      --abort-on-error       Abort on first error (batch mode)
  -v, --verbose              Verbose output
  -q, --quiet                Quiet mode
  -h, --help                 Print help
  -V, --version              Print version
```

## Document Chunking (Library)

Intelligent chunking for RAG and embedding applications.

### Basic Hierarchical Chunking

```rust
use docling_rs::{DocumentConverter, chunking::{HierarchicalChunker, BaseChunker}};

let converter = DocumentConverter::new();
let result = converter.convert_file("document.md")?;
let doc = result.document();

// Create hierarchical chunker
let chunker = HierarchicalChunker::new();

// Generate chunks
for chunk in chunker.chunk(&doc) {
    println!("Chunk {}: {}", chunk.meta.index, chunk.text);
    println!("Context: {:?}", chunk.meta.headings);
    println!("Position: {}-{}", chunk.meta.start_offset, chunk.meta.end_offset);
}
```

### Advanced Hybrid Chunking

For token-aware chunking (useful with embedding models):

```rust
use docling_rs::chunking::{HybridChunker, tokenizer::HuggingFaceTokenizer, BaseChunker};

// Load tokenizer (compatible with HuggingFace tokenizers)
let tokenizer = Box::new(HuggingFaceTokenizer::from_file("tokenizer.json")?);

// Create hybrid chunker with token limit
let chunker = HybridChunker::builder()
    .tokenizer(tokenizer)
    .max_tokens(512)          // Maximum tokens per chunk
    .merge_peers(true)        // Merge small adjacent chunks
    .build()?;

// Chunk with token awareness
let chunks: Vec<_> = chunker.chunk(&doc).collect();
println!("Generated {} chunks", chunks.len());

// Each chunk respects token limit
for chunk in chunks {
    // Use for embedding models
    let contextualized = chunker.contextualize(&chunk);
    println!("Embedding input: {}", contextualized);
}
```

### Chunk Metadata

Each chunk includes rich metadata:

```rust
pub struct BaseChunk {
    pub text: String,           // Chunk text content
    pub meta: ChunkMetadata,
}

pub struct ChunkMetadata {
    pub doc_name: String,       // Source document name
    pub headings: Vec<String>,  // Hierarchical heading context
    pub caption: Option<String>, // Optional caption
    pub start_offset: usize,    // Start position in document
    pub end_offset: usize,      // End position in document
    pub index: usize,           // Sequential chunk index
}
```

### Chunking Strategies

- **Hierarchical**: Structure-based chunking that respects document hierarchy (headings, paragraphs, lists)
- **Hybrid**: Token-aware chunking with semantic boundaries and configurable merging

