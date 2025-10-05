# Contract: PdfBackend (Backend Trait Implementation)

**Module**: `docling_rs::backend::pdf`
**Trait**: `Backend`
**Date**: 2025-10-05

## Contract Definition

```rust
use docling_rs::backend::traits::Backend;
use docling_rs::datamodel::{InputDocument, DoclingDocument};
use docling_rs::error::ConversionError;

impl Backend for PdfBackend {
    fn convert(&self, input: &InputDocument) -> Result<DoclingDocument, ConversionError>;
}
```

## Preconditions

1. `input.format()` MUST be `InputFormat::PDF`
2. Input source (file path or bytes) MUST contain valid PDF data
3. If PDF is encrypted, `PdfConfig.password` MUST be provided

## Postconditions

1. Returns `Ok(DoclingDocument)` with all extracted content
2. `DoclingDocument` contains nodes for all text, tables, images
3. `DoclingDocument.name()` matches input document name
4. All text preserves reading order (best effort for multi-column)
5. Tables extracted with structure (if `enable_tables` is true)
6. Images detected with bounding boxes (if `enable_images` is true)
7. OCR applied to scanned pages (if `enable_ocr` is true)

## Error Conditions

| Error | Condition | Error Type |
|-------|-----------|------------|
| File not found | Input file path doesn't exist | `ConversionError::FileNotFound` |
| Invalid PDF | Malformed PDF structure | `ConversionError::ParseError` |
| Encrypted PDF | Password required but not provided | `ConversionError::EncryptionError` |
| Wrong password | Incorrect password provided | `ConversionError::EncryptionError` |
| No extract permission | PDF permissions forbid extraction | `ConversionError::PermissionDenied` |
| Unsupported format | Input is not PDF | `ConversionError::UnsupportedFormat` |

## Invariants

1. **Idempotent**: Multiple calls with same input produce same output
2. **No side effects**: Conversion doesn't modify input file
3. **Resource cleanup**: All PDF resources released after conversion
4. **Thread safety**: Can be called from multiple threads (with internal locking)

## Contract Tests

### Test 1: Basic PDF Conversion
```rust
#[test]
fn test_pdf_backend_converts_simple_pdf() {
    let backend = PdfBackend::new();
    let input = InputDocument::from_path("test.pdf", InputFormat::PDF);

    let result = backend.convert(&input);

    assert!(result.is_ok());
    let doc = result.unwrap();
    assert_eq!(doc.name(), "test.pdf");
    assert!(doc.nodes().len() > 0);
}
```

### Test 2: Encrypted PDF with Password
```rust
#[test]
fn test_pdf_backend_handles_encrypted_pdf() {
    let config = PdfConfig::default()
        .password(Some("secret".to_string()));
    let backend = PdfBackend::with_config(config);
    let input = InputDocument::from_path("encrypted.pdf", InputFormat::PDF);

    let result = backend.convert(&input);

    assert!(result.is_ok());
}
```

### Test 3: Encrypted PDF without Password
```rust
#[test]
fn test_pdf_backend_fails_encrypted_without_password() {
    let backend = PdfBackend::new();
    let input = InputDocument::from_path("encrypted.pdf", InputFormat::PDF);

    let result = backend.convert(&input);

    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), ConversionError::EncryptionError(_)));
}
```

### Test 4: Invalid PDF
```rust
#[test]
fn test_pdf_backend_fails_invalid_pdf() {
    let backend = PdfBackend::new();
    let input = InputDocument::from_bytes(
        b"not a pdf".to_vec(),
        "invalid.pdf".to_string(),
        InputFormat::PDF
    );

    let result = backend.convert(&input);

    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), ConversionError::ParseError(_)));
}
```

### Test 5: Multi-page PDF
```rust
#[test]
fn test_pdf_backend_handles_multipage() {
    let backend = PdfBackend::new();
    let input = InputDocument::from_path("multipage.pdf", InputFormat::PDF);

    let result = backend.convert(&input);

    assert!(result.is_ok());
    let doc = result.unwrap();
    // Should have content from all pages
    assert!(doc.nodes().len() >= 3); // At least 3 pages worth
}
```

### Test 6: PDF with Tables
```rust
#[test]
fn test_pdf_backend_extracts_tables() {
    let config = PdfConfig::default().enable_tables(true);
    let backend = PdfBackend::with_config(config);
    let input = InputDocument::from_path("with_table.pdf", InputFormat::PDF);

    let result = backend.convert(&input);

    assert!(result.is_ok());
    let doc = result.unwrap();
    // Should have table nodes
    let has_table = doc.nodes().iter().any(|n| {
        matches!(n.node_type(), NodeType::Table)
    });
    assert!(has_table, "Should detect tables");
}
```

### Test 7: Scanned PDF with OCR
```rust
#[test]
fn test_pdf_backend_ocr_scanned_pdf() {
    let config = PdfConfig::default()
        .enable_ocr(true)
        .ocr_language("eng");
    let backend = PdfBackend::with_config(config);
    let input = InputDocument::from_path("scanned.pdf", InputFormat::PDF);

    let result = backend.convert(&input);

    assert!(result.is_ok());
    let doc = result.unwrap();
    // Should have text from OCR
    assert!(doc.nodes().iter().any(|n| !n.text_content().unwrap_or("").is_empty()));
}
```

## Performance Contract

- Simple PDFs (<10 pages, <1MB) MUST process in <2 seconds
- Memory usage MUST NOT exceed 2x PDF file size
- Resource cleanup MUST occur even if conversion fails

## Integration Points

- **Input**: `InputDocument` from Phase 1
- **Output**: `DoclingDocument` compatible with Phase 1 export and Phase 2 chunking
- **Configuration**: `PdfConfig` for customization
- **Errors**: Uses existing `ConversionError` enum

## Dependencies

- `pdfium-render` for PDF parsing
- `tesseract-rs` for OCR (if enabled)
- `tract` for ML models (if enabled)
- `image` for image processing
