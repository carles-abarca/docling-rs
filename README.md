# docling-rs

**A native Rust implementation inspired by [IBM's Docling](https://github.com/DS4SD/docling) Python library.**

docling-rs brings document processing capabilities to the Rust ecosystem, offering a high-performance alternative for converting documents into structured, machine-readable formats optimized for RAG (Retrieval-Augmented Generation) and LLM applications.

## Why docling-rs?

The original [Docling](https://github.com/DS4SD/docling) by IBM is an excellent Python library for document processing. This Rust adaptation provides:

- **Native Performance**: No Python runtime required, significantly faster processing
- **Single Binary Distribution**: Easy deployment with self-contained executables
- **Memory Safety**: Rust's guarantees for reliable production use
- **Cross-Platform**: Pre-built binaries for Windows, macOS (Intel & Apple Silicon), and Linux
- **Batteries Included**: PDF support with bundled PDFium library

## Features

- **Multi-format Support**: Markdown, HTML, CSV, DOCX, XLSX, PPTX, and PDF
- **Unified Document Model**: All formats convert to a common `DoclingDocument` structure
- **Intelligent Chunking**: Hierarchical and hybrid chunking strategies for RAG applications
- **Pure Rust**: No Python dependencies, native performance
- **Cross-platform**: Windows, macOS (Intel & Apple Silicon), and Linux
- **Modular Architecture**: Workspace-based design with separate crates per format
- **CLI Included**: Full-featured command-line interface for batch processing
- **Batteries Included**: PDF support with bundled PDFium binaries

## Status

**v1.0.1** - Production-ready with 7 format backends (all enabled by default)

| Component | Status |
|-----------|--------|
| Core Library | ✅ Complete |
| CLI | ✅ Complete |
| Markdown Backend | ✅ Complete |
| HTML Backend | ✅ Complete |
| CSV Backend | ✅ Complete |
| DOCX Backend | ✅ Complete |
| XLSX Backend | ✅ Complete |
| PPTX Backend | ✅ Complete |
| PDF Backend | ✅ Complete |
| Chunking | ✅ Complete |
| Documentation | ✅ Complete |

## Installation

### Pre-built Binaries

Download from [Releases](https://github.com/carles-abarca/docling-rs/releases):

- **Windows**: `docling-rs-x86_64-windows.msi` or `.zip`
- **macOS Intel**: `docling-rs-x86_64-macos.dmg`
- **macOS Apple Silicon**: `docling-rs-aarch64-macos.dmg`
- **Linux**: `docling-rs-x86_64-linux.tar.gz`

### Rust Library

Add to your `Cargo.toml`:

```toml
[dependencies]
docling-rs = "1.0"

# Or with minimal features (no PDF/Office)
docling-rs = { version = "1.0", default-features = false, features = ["markdown", "html", "csv"] }
```

### Feature Flags

| Feature | Description | Default |
|---------|-------------|---------|
| `full` | All format backends | **Yes** |
| `markdown` | Markdown support | Yes |
| `html` | HTML support | Yes |
| `csv` | CSV support | Yes |
| `docx` | Microsoft Word support | Yes |
| `xlsx` | Microsoft Excel support | Yes |
| `pptx` | Microsoft PowerPoint support | Yes |
| `pdf` | PDF support (requires PDFium) | Yes |

## Quick Start

### CLI Usage

```bash
# Convert a single file
docling-rs document.pdf --to markdown --output-dir ./output

# Batch convert a directory
docling-rs ./documents/ --to json --output-dir ./converted

# Enable chunking for RAG
docling-rs document.pdf --chunk --chunk-size 512 --to json

# Filter by input format
docling-rs ./docs/ --from pdf,docx --to markdown
```

### Library Usage

```rust
use docling_rs::DocumentConverter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let converter = DocumentConverter::new();

    // Convert a file (format auto-detected)
    let result = converter.convert_file("document.pdf")?;
    let doc = result.document();

    // Export to different formats
    let markdown = doc.to_markdown();
    let text = doc.to_text();
    let json = serde_json::to_string_pretty(&doc)?;

    println!("Document: {}", doc.name());
    println!("Nodes: {}", doc.nodes().len());

    Ok(())
}
```

### Converting from Bytes

```rust
use docling_rs::{DocumentConverter, InputFormat};

let converter = DocumentConverter::new();
let result = converter.convert_bytes(
    pdf_bytes,
    "document.pdf".to_string(),
    InputFormat::PDF,
)?;
```

## Supported Formats

| Format | Extensions | Description |
|--------|------------|-------------|
| Markdown | `.md`, `.markdown` | CommonMark and GFM |
| HTML | `.html`, `.htm` | Semantic HTML extraction |
| CSV | `.csv` | Tabular data |
| Word | `.docx`, `.dotx`, `.docm` | Microsoft Word |
| Excel | `.xlsx`, `.xlsm`, `.xls` | Microsoft Excel |
| PowerPoint | `.pptx`, `.potx`, `.ppsx` | Microsoft PowerPoint |
| PDF | `.pdf` | PDF documents |

## Document Chunking

Intelligent chunking for RAG and embedding applications:

```rust
use docling_rs::{DocumentConverter, HierarchicalChunker};

let converter = DocumentConverter::new();
let result = converter.convert_file("document.pdf")?;
let doc = result.document();

// Create hierarchical chunker
let chunker = HierarchicalChunker::new()
    .with_max_chunk_size(512)
    .with_overlap(50);

// Generate chunks
let chunks = chunker.chunk(doc)?;

for chunk in &chunks {
    println!("Chunk: {} chars", chunk.text().len());
    println!("Context: {:?}", chunk.metadata().headings());
}
```

## CLI Options

```
Usage: docling-rs [OPTIONS] <INPUT>

Arguments:
  <INPUT>  Input file or directory

Options:
  -t, --to <FORMAT>              Output format: json, markdown, text [default: markdown]
  -o, --output-dir <DIR>         Output directory
  -f, --from <FORMATS>           Filter input formats (comma-separated)
      --chunk                    Enable document chunking
      --chunk-strategy <STRAT>   Chunking strategy: hierarchical, hybrid [default: hierarchical]
      --chunk-max-tokens <N>     Max tokens per chunk (hybrid) [default: 512]
      --chunk-merge-peers        Merge undersized peer chunks (hybrid) [default: true]
      --continue-on-error        Continue on errors (batch mode)
      --abort-on-error           Stop on first error (batch mode)
  -v, --verbose                  Verbose output
  -q, --quiet                    Suppress output
  -h, --help                     Print help
  -V, --version                  Print version
```

### Chunking Strategies

```bash
# Hierarchical chunking (default) - preserves document structure
docling-rs document.pdf --chunk --to json

# Hybrid chunking - token-aware, ideal for embeddings
docling-rs document.pdf --chunk --chunk-strategy hybrid --chunk-max-tokens 512 --to json

# Hybrid without merging small chunks
docling-rs document.pdf --chunk --chunk-strategy hybrid --chunk-merge-peers false --to json
```

## Architecture

docling-rs uses a modular workspace structure:

```
crates/
├── docling-rs/              # Main facade library
├── docling-rs-core/         # Core types and traits
├── docling-rs-cli/          # Command-line interface
└── docling-rs-formats/      # Format backends
    ├── markdown/
    ├── html/
    ├── csv/
    ├── docx/
    ├── xlsx/
    ├── pptx/
    └── pdf/
```

## Documentation

Full documentation is available in the `manual/` directory:

- [Overview](manual/index.html) - Introduction and features
- [Installation](manual/installation.html) - Setup guide
- [Quick Start](manual/quickstart.html) - Get started in 5 minutes
- [API Reference](manual/api-reference.html) - Library API documentation
- [CLI Usage](manual/cli.html) - Command-line interface guide
- [Supported Formats](manual/formats.html) - Format details
- [Chunking for RAG](manual/chunking.html) - Chunking strategies
- [Architecture](manual/architecture.html) - System design

## Development

### Prerequisites

- Rust 1.75 or later
- PDFium library (for PDF support)

### Building

```bash
# Build all crates
cargo build --workspace

# Build CLI only
cargo build -p docling-rs-cli --release

# Build with all features
cargo build -p docling-rs --features full
```

### Testing

```bash
# Run all tests
cargo test --workspace

# Run specific crate tests
cargo test -p docling-rs
cargo test -p docling-rs-cli

# Manual CLI testing
./scripts/test-cli-manual.sh
```

### Linting

```bash
cargo clippy --workspace
cargo fmt --check
```

## Acknowledgments

This project is inspired by and pays tribute to [IBM's Docling](https://github.com/DS4SD/docling) project. While docling-rs is an independent Rust implementation and not affiliated with IBM, it aims to provide similar document processing capabilities for the Rust ecosystem.

## License

MIT

## Contributing

Contributions are welcome! See `CLAUDE.md` for development guidelines.
