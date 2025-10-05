# Docling-rs

Native Rust document processing library for extracting structured text and metadata from common document formats.

## Features

- **Multi-format Support**: Markdown, HTML, CSV, and DOCX
- **Unified Document Model**: All formats convert to a common `DoclingDocument` structure
- **Pure Rust**: No Python dependencies, native performance
- **Cross-platform**: Works on Windows and macOS
- **Type-safe**: Leverages Rust's type system for reliability
- **Serialization**: JSON serialization/deserialization via serde

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
cargo test
```

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
