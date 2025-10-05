# Implementation Plan: Complete PDF Backend

**Branch**: `007-complete-pdf-backend` | **Date**: 2025-10-05 | **Spec**: [spec.md](./spec.md)

## Summary

Implement full PDF processing capabilities by connecting existing stub modules with pdfium-render library. Enable text extraction, layout analysis, table detection, image extraction, and OCR to convert real PDF documents into DoclingDocument format.

**Technical Approach**: Use pdfium-render for PDF loading and basic text extraction, implement layout analysis using text position data, build grid-based table detector, integrate image extraction via pdfium's image APIs, and connect rusty-tesseract for OCR on scanned documents.

## Technical Context

**Language/Version**: Rust 1.75+

**Primary Dependencies**:
- `pdfium-render` v0.8+ (PDF rendering and text extraction)
- `rusty-tesseract` v1.1+ (OCR engine - feature gated)
- `image` v0.25+ (image processing)
- All dependencies already in Cargo.toml

**Storage**: Filesystem (PDF input files)

**Testing**:
- Integration tests: `tests/integration_pdf_*.rs` (30+ tests currently ignored)
- Contract tests: `tests/contract_pdf_backend.rs`
- All tests must pass before completion

**Target Platform**: macOS and Windows (pdfium binaries for both)

**Performance Goals**:
- Single-page PDF: <1 second
- 10-page PDF: <5 seconds
- Memory usage: <100MB for typical documents
- No memory leaks in batch processing

**Constraints**:
- Must maintain existing PdfBackend API
- Must implement Backend trait correctly
- Cannot break existing datamodel types
- Must work without pdfium binary (graceful error)

## Phase Breakdown

### Phase 0: Foundation (Already Complete ✓)
- Module structure created
- Types defined
- Dependencies added
- Stub implementations in place

### Phase 1: Core Text Extraction (P0 - Critical)
**Goal**: Load PDFs and extract basic text

**Modules**: `backend.rs`, `text_extractor.rs`

**Tasks**:
1. Implement PDF loading with pdfium-render
2. Extract raw text from pages
3. Handle multi-page documents
4. Support encrypted PDFs with passwords
5. Error handling for corrupt PDFs

**Tests to Enable**:
- `integration_pdf_text_extraction.rs` (3 tests)
- `integration_pdf_multipage.rs` (3 tests)
- `integration_pdf_encrypted.rs` (4 tests)

**Deliverable**: Basic PDF → DoclingDocument conversion with text nodes

### Phase 2: Layout Analysis (P1 - High)
**Goal**: Detect document structure and reading order

**Modules**: `layout_analyzer.rs`, `page.rs`

**Tasks**:
1. Extract text with position information (bbox)
2. Detect text blocks and columns
3. Determine reading order
4. Group related text blocks
5. Handle multi-column layouts

**Tests to Enable**:
- `integration_pdf_multicolumn.rs` (4 tests)

**Deliverable**: Structured document with proper reading order

### Phase 3: Table Detection (P1 - High)
**Goal**: Extract tables with structure

**Modules**: `table_detector.rs`, `table.rs`

**Tasks**:
1. Implement grid-based table detection
2. Extract cell boundaries using text positions
3. Build TableStructure with rows/columns
4. Handle cell merging (basic)
5. Preserve cell content

**Tests to Enable**:
- `integration_pdf_tables.rs` (5 tests)
- `contract_pdf_tables.rs` (existing contract tests)

**Deliverable**: Tables extracted as structured data

### Phase 4: Image Extraction (P2 - Medium)
**Goal**: Extract and classify images

**Modules**: `image_extractor.rs`, `image.rs`

**Tasks**:
1. Use pdfium to extract embedded images
2. Capture image metadata (format, dimensions, DPI)
3. Classify image types (photo/diagram/chart)
4. Handle multiple images per page
5. Include images in DoclingDocument

**Tests to Enable**:
- `integration_pdf_images.rs` (6 tests)

**Deliverable**: Images extracted with metadata

### Phase 5: OCR Integration (P2 - Medium)
**Goal**: Extract text from scanned PDFs

**Modules**: `ocr_engine.rs`, `ocr.rs`

**Tasks**:
1. Detect scanned PDFs (no text layer)
2. Convert PDF pages to images
3. Integrate rusty-tesseract
4. Extract text with confidence scores
5. Support multiple languages (eng, spa, fra, deu)

**Tests to Enable**:
- `integration_pdf_ocr.rs` (3 tests)

**Deliverable**: OCR-based text extraction for scanned documents

### Phase 6: Backend Contract & Polish (P0 - Critical)
**Goal**: Ensure all contracts met and tests pass

**Tasks**:
1. Verify Backend trait implementation
2. Test all PdfConfig options
3. Improve error messages
4. Optimize performance
5. Document public APIs
6. Enable all tests and verify CI passes

**Tests to Enable**:
- `contract_pdf_backend.rs` (2 tests)
- All CLI tests using PDF backend

**Deliverable**: Fully functional, tested, documented PDF backend

## Implementation Strategy

### Incremental Approach
1. Start with Phase 1 (text extraction)
2. Verify tests pass before moving to next phase
3. Each phase builds on previous phases
4. Tests guide implementation (TDD-style)

### Testing Strategy
- Remove `#[ignore]` from tests one phase at a time
- Run `cargo test --test integration_pdf_*` after each phase
- Ensure no regressions in existing backends
- Use real PDF samples for integration tests

### Error Handling
- Return `ConversionError::ParseError` for PDF parsing errors
- Return `ConversionError::EncryptionError` for password issues
- Return `ConversionError::InvalidFile` for corrupted files
- Graceful degradation when pdfium unavailable

### Performance Considerations
- Lazy loading of pages (don't load all at once)
- Stream processing for large PDFs
- Cache frequently accessed data
- Minimize memory allocations

## Dependencies Between Phases

```
Phase 1 (Text Extraction)
    ↓
Phase 2 (Layout Analysis) ← Uses text positions from Phase 1
    ↓
Phase 3 (Table Detection) ← Uses layout blocks from Phase 2
    ↓
Phase 4 (Image Extraction) ← Independent, can run in parallel
    ↓
Phase 5 (OCR) ← Uses image extraction from Phase 4
    ↓
Phase 6 (Polish) ← Integrates all phases
```

## Success Criteria

- [ ] All 30+ ignored PDF tests passing
- [ ] CI/CD pipeline green (macOS + Windows)
- [ ] Performance benchmarks met (NFR-001)
- [ ] All clippy warnings resolved
- [ ] Public APIs documented
- [ ] Real-world PDFs convert successfully

## Risks and Mitigations

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| pdfium library not available | Medium | High | Detect at runtime, provide clear error message |
| Performance issues with large PDFs | Medium | Medium | Implement streaming, lazy loading |
| Complex table detection edge cases | High | Low | Start simple, iterate based on real cases |
| OCR accuracy issues | Medium | Low | Surface confidence scores, allow fallback |
| Platform-specific pdfium issues | Low | High | Test on both macOS and Windows early |

## Timeline Estimate

- Phase 1: 2-3 days (Critical foundation)
- Phase 2: 2-3 days (Complex layout logic)
- Phase 3: 2-3 days (Table detection algorithms)
- Phase 4: 1-2 days (Straightforward image API)
- Phase 5: 1-2 days (OCR integration)
- Phase 6: 1 day (Polish and documentation)

**Total**: 9-14 days of focused implementation

## Next Steps

1. Create tasks.md with detailed task breakdown
2. Set up feature branch `007-complete-pdf-backend`
3. Start with Phase 1: Text Extraction
4. Enable tests incrementally as features complete
