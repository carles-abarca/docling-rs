# Quickstart: Advanced PDF Processing

**Feature**: 003-phase-3-pdf
**Date**: 2025-10-05

## Overview

Quick examples demonstrating PDF processing capabilities with text extraction, layout analysis, table detection, OCR, and integration with chunking.

## Basic PDF Text Extraction

```rust
use docling_rs::{DocumentConverter, InputFormat};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create converter
    let converter = DocumentConverter::new();

    // Convert PDF
    let result = converter.convert_file("document.pdf")?;
    let doc = result.document();

    // Access text content
    for node in doc.nodes() {
        if let Some(text) = node.text_content() {
            println!("{}", text);
        }
    }

    Ok(())
}
```

## PDF with Tables

```rust
use docling_rs::backend::pdf::{PdfBackend, PdfConfig};
use docling_rs::InputDocument;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configure PDF backend with tables enabled
    let config = PdfConfig::default()
        .enable_tables(true);

    let backend = PdfBackend::with_config(config);
    let input = InputDocument::from_path("report.pdf", InputFormat::PDF);

    // Convert
    let doc_result = backend.convert(&input)?;
    let doc = doc_result.document();

    // Extract tables
    for node in doc.nodes() {
        if let NodeType::Table = node.node_type() {
            println!("Found table with {} rows", node.table_data().rows());
        }
    }

    Ok(())
}
```

## Scanned PDF with OCR

```rust
use docling_rs::backend::pdf::{PdfBackend, PdfConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Enable OCR for scanned PDFs
    let config = PdfConfig::default()
        .enable_ocr(true)
        .ocr_language("eng");  // English

    let backend = PdfBackend::with_config(config);
    let input = InputDocument::from_path("scanned.pdf", InputFormat::PDF);

    let result = backend.convert(&input)?;
    let doc = result.document();

    // OCR text is included in nodes
    for node in doc.nodes() {
        if let Some(text) = node.text_content() {
            println!("{}", text);
        }
    }

    Ok(())
}
```

## Password-Protected PDF

```rust
use docling_rs::backend::pdf::{PdfBackend, PdfConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Provide password for encrypted PDF
    let config = PdfConfig::default()
        .password(Some("secret123".to_string()));

    let backend = PdfBackend::with_config(config);
    let input = InputDocument::from_path("encrypted.pdf", InputFormat::PDF);

    let result = backend.convert(&input)?;
    println!("Successfully decrypted and processed PDF");

    Ok(())
}
```

## PDF with Images

```rust
use docling_rs::backend::pdf::{PdfBackend, PdfConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Enable image extraction
    let config = PdfConfig::default()
        .enable_images(true);

    let backend = PdfBackend::with_config(config);
    let input = InputDocument::from_path("presentation.pdf", InputFormat::PDF);

    let result = backend.convert(&input)?;
    let doc = result.document();

    // Access image regions
    for node in doc.nodes() {
        if let NodeType::Image = node.node_type() {
            let bbox = node.position().unwrap();
            println!("Image at ({}, {}) - {}x{}",
                bbox.start_offset(), bbox.end_offset(),
                bbox.width(), bbox.height()
            );
        }
    }

    Ok(())
}
```

## Integration with Phase 2 Chunking

```rust
use docling_rs::{DocumentConverter, InputFormat};
use docling_rs::chunking::{HybridChunker, HierarchicalChunker};
use docling_rs::chunking::tokenizer::HuggingFaceTokenizer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Step 1: Convert PDF
    let converter = DocumentConverter::new();
    let result = converter.convert_file("document.pdf")?;
    let doc = result.document();

    // Step 2: Hierarchical chunking (structure-preserving)
    let hier_chunker = HierarchicalChunker::new();
    for chunk in hier_chunker.chunk(&doc) {
        println!("Chunk {}: {}", chunk.meta.index, chunk.text);
    }

    // Step 3: Hybrid token-aware chunking
    let tokenizer = HuggingFaceTokenizer::from_pretrained("bert-base-uncased")?;
    let hybrid_chunker = HybridChunker::builder()
        .tokenizer(Box::new(tokenizer))
        .max_tokens(512)
        .merge_peers(true)
        .build()?;

    let chunks: Vec<_> = hybrid_chunker.chunk(&doc).collect();
    println!("Generated {} token-aware chunks", chunks.len());

    // Step 4: Contextualize for RAG
    for chunk in &chunks {
        let contextualized = hybrid_chunker.contextualize(chunk);
        println!("Context: {}", contextualized);
    }

    Ok(())
}
```

## Multi-Page Processing

```rust
use docling_rs::backend::pdf::{PdfBackend, PdfConfig};
use std::ops::Range;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Process only pages 1-10
    let config = PdfConfig::default()
        .page_range(Some(Range { start: 0, end: 10 }));

    let backend = PdfBackend::with_config(config);
    let input = InputDocument::from_path("large_doc.pdf", InputFormat::PDF);

    let result = backend.convert(&input)?;
    println!("Processed pages 1-10");

    Ok(())
}
```

## Advanced: Custom Layout Analysis

```rust
use docling_rs::backend::pdf::{PdfBackend, PdfConfig, LayoutAnalyzerType};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Use ML model for layout analysis
    let config = PdfConfig::default()
        .layout_analyzer(LayoutAnalyzerType::MlModel(
            "models/layout_model.onnx".to_string()
        ));

    let backend = PdfBackend::with_config(config);
    let input = InputDocument::from_path("complex_layout.pdf", InputFormat::PDF);

    let result = backend.convert(&input)?;
    println!("Processed with ML layout analysis");

    Ok(())
}
```

## Complete Workflow: PDF → Chunks → JSON Export

```rust
use docling_rs::{DocumentConverter, InputFormat};
use docling_rs::chunking::HybridChunker;
use docling_rs::chunking::tokenizer::HuggingFaceTokenizer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Convert PDF
    let converter = DocumentConverter::new();
    let result = converter.convert_file("research_paper.pdf")?;
    let doc = result.document();

    // 2. Chunk for RAG
    let tokenizer = HuggingFaceTokenizer::from_pretrained("sentence-transformers/all-MiniLM-L6-v2")?;
    let chunker = HybridChunker::builder()
        .tokenizer(Box::new(tokenizer))
        .max_tokens(256)
        .build()?;

    let chunks: Vec<_> = chunker.chunk(&doc).collect();

    // 3. Export to JSON
    let chunks_json = serde_json::to_string_pretty(&chunks)?;
    std::fs::write("chunks.json", chunks_json)?;

    println!("Exported {} chunks to chunks.json", chunks.len());

    Ok(())
}
```

## Error Handling

```rust
use docling_rs::backend::pdf::{PdfBackend, PdfConfig};
use docling_rs::error::ConversionError;

fn main() {
    let backend = PdfBackend::new();
    let input = InputDocument::from_path("document.pdf", InputFormat::PDF);

    match backend.convert(&input) {
        Ok(result) => {
            println!("Successfully converted PDF");
        }
        Err(ConversionError::FileNotFound(path)) => {
            eprintln!("PDF file not found: {}", path.display());
        }
        Err(ConversionError::EncryptionError(msg)) => {
            eprintln!("PDF is encrypted: {}", msg);
        }
        Err(ConversionError::ParseError(msg)) => {
            eprintln!("Failed to parse PDF: {}", msg);
        }
        Err(e) => {
            eprintln!("Conversion error: {}", e);
        }
    }
}
```

## Testing Examples

All examples above can be tested with:

```bash
# Run basic conversion
cargo run --example pdf_basic document.pdf

# Run with OCR
cargo run --example pdf_ocr scanned.pdf

# Run with chunking
cargo run --example pdf_chunking research.pdf

# Run tests
cargo test --test pdf_integration
```

## Next Steps

After quickstart:
1. Explore `PdfConfig` options for customization
2. Review contract tests in `tests/contract/pdf_*.rs`
3. Check integration examples in `tests/integration/pdf_*.rs`
4. See full API documentation: `cargo doc --open`
