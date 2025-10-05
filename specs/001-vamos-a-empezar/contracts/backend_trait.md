# Backend Trait Contract

## Overview

The `Backend` trait defines the interface all format-specific backends must implement.

## Trait Definition

```rust
pub trait Backend {
    /// Validate that the input can be processed
    fn is_valid(&self) -> bool;

    /// List of formats this backend supports
    fn supported_formats() -> &'static [InputFormat];

    /// Convert input to DoclingDocument
    fn convert(&self, input: &InputDocument) -> Result<DoclingDocument, ConversionError>;
}

pub trait DeclarativeBackend: Backend {
    // Marker trait for backends that convert directly without pipeline stages
}
```

## Contract Requirements

### is_valid()
- MUST return `true` if file/stream is parseable by this backend
- MUST return `false` for corrupted/malformed files
- MUST NOT panic on invalid input

### supported_formats()
- MUST return non-empty slice
- MUST be static (compile-time constant)

### convert()
- MUST return `Ok(DoclingDocument)` for valid input
- MUST return `Err(ConversionError)` for invalid input
- MUST NOT panic (except unrecoverable bugs)
- MUST preserve document structure
- MUST populate metadata correctly

## DeclarativeBackend

Marker trait for backends that:
- Convert directly to `DoclingDocument`
- Don't need multi-stage pipeline
- Examples: Markdown, HTML, CSV, DOCX (Phase 1)

---

**Status**: Defines backend interface for all format implementations
