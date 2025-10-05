# Tasks: Complete PDF Backend Implementation

**Input**: Design documents from `/Users/carlesabarca/MyProjects/docling-rs/specs/007-complete-pdf-backend/`
**Prerequisites**: plan.md ✓, spec.md ✓

## Execution Summary

Implementation broken into 6 phases with 45 tasks:
- **Phase 1**: Core text extraction (10 tests)
- **Phase 2**: Layout analysis (4 tests)
- **Phase 3**: Table detection (5 tests)
- **Phase 4**: Image extraction (6 tests)
- **Phase 5**: OCR integration (3 tests)
- **Phase 6**: Backend contracts & polish (2 tests + CLI)

**Total**: 30+ ignored tests to enable

## Format: `[ID] [P?] Description`
- **[P]**: Can run in parallel (different files, no dependencies)
- File paths are absolute from repository root

---

## Phase 1: Core Text Extraction (P0 - Critical)

**Goal**: Load PDFs with pdfium and extract basic text
**Tests**: 10 tests (text_extraction: 3, multipage: 3, encrypted: 4)

### Setup & Infrastructure
- [ ] **T001** Create feature branch `007-complete-pdf-backend` from master
- [ ] **T002** Verify pdfium-render dependency and add platform-specific pdfium binaries if needed
- [ ] **T003** Create PDF test fixtures directory: `tests/fixtures/pdfs/` with sample PDFs

### Backend Implementation
- [ ] **T004** Implement `PdfBackend::convert_pdf()` to load PDF using pdfium-render
  - File: `src/backend/pdf/backend.rs`
  - Load from bytes or file path
  - Handle password-protected PDFs using `PdfConfig::password`
  - Return `ConversionError::EncryptionError` if password wrong/missing

- [ ] **T005** Implement `PdfBackend::extract_pages()` to iterate through all pages
  - File: `src/backend/pdf/backend.rs`
  - Get page count
  - Extract text from each page
  - Create DocumentNode for each page's text

- [ ] **T006** Implement basic `TextExtractor::extract_from_page()` using pdfium text API
  - File: `src/backend/pdf/text_extractor.rs`
  - Use pdfium's `page.text()` method
  - Return plain text without positions (basic version)
  - Handle empty pages gracefully

### Error Handling
- [ ] **T007** Add comprehensive error handling for PDF loading failures
  - File: `src/backend/pdf/backend.rs`
  - Corrupt PDF → `ConversionError::InvalidFile`
  - Missing password → `ConversionError::EncryptionError`
  - Pdfium unavailable → `ConversionError::ParseError` with helpful message

### Testing
- [ ] **T008** Enable tests in `tests/integration_pdf_text_extraction.rs` (remove `#[ignore]`)
  - `test_extract_text_from_simple_pdf`
  - `test_extract_text_with_positions` (will need Phase 2)
  - `test_extract_text_from_empty_pdf`

- [ ] **T009** Enable tests in `tests/integration_pdf_multipage.rs` (remove `#[ignore]`)
  - `test_multipage_pdf_page_count`
  - `test_multipage_pdf_text_extraction`
  - `test_multipage_pdf_reading_order`

- [ ] **T010** Enable tests in `tests/integration_pdf_encrypted.rs` (remove `#[ignore]`)
  - `test_encrypted_pdf_with_correct_password`
  - `test_encrypted_pdf_without_password`
  - `test_encrypted_pdf_with_wrong_password`
  - `test_unencrypted_pdf_with_password_provided`

### Deliverable
- [ ] **T011** Run `cargo test --test integration_pdf_text_extraction --test integration_pdf_multipage --test integration_pdf_encrypted`
- [ ] **T012** Verify all 10 tests pass before proceeding to Phase 2

---

## Phase 2: Layout Analysis (P1 - High)

**Goal**: Detect document structure and reading order
**Tests**: 4 tests (multicolumn layout)

### Layout Analysis Implementation
- [ ] **T013** Implement `TextExtractor::extract_with_positions()` to get text bounding boxes
  - File: `src/backend/pdf/text_extractor.rs`
  - Use pdfium `page.text_chars()` for character positions
  - Build TextBlock objects with bbox coordinates
  - Return Vec<TextBlock> with position metadata

- [ ] **T014** Implement `LayoutAnalyzer::analyze_page()` to detect columns
  - File: `src/backend/pdf/layout_analyzer.rs`
  - Use RuleBasedLayoutAnalyzer
  - Cluster text blocks by horizontal position
  - Detect column boundaries using gaps

- [ ] **T015** Implement `LayoutAnalyzer::determine_reading_order()` for multi-column text
  - File: `src/backend/pdf/layout_analyzer.rs`
  - Sort columns left-to-right
  - Within each column, sort blocks top-to-bottom
  - Return ordered list of text blocks

- [ ] **T016** Update `PdfBackend::convert_pdf()` to use layout analysis
  - File: `src/backend/pdf/backend.rs`
  - Call layout analyzer on each page
  - Create DocumentNodes in correct reading order
  - Preserve position metadata in nodes

### Testing
- [ ] **T017** Enable tests in `tests/integration_pdf_multicolumn.rs` (remove `#[ignore]`)
  - `test_two_column_layout`
  - `test_three_column_layout`
  - `test_mixed_single_and_multi_column`
  - `test_column_reading_order`

### Deliverable
- [ ] **T018** Run `cargo test --test integration_pdf_multicolumn` and verify all 4 tests pass

---

## Phase 3: Table Detection (P1 - High)

**Goal**: Extract tables with structure
**Tests**: 5 tests (table detection and extraction)

### Table Detection Implementation
- [ ] **T019** Implement `TableDetector::detect_tables()` using grid-based approach
  - File: `src/backend/pdf/table_detector.rs`
  - Use GridBasedTableDetector
  - Analyze text block positions to find grid patterns
  - Detect aligned rows and columns

- [ ] **T020** Implement `TableDetector::build_table_structure()` to create TableStructure
  - File: `src/backend/pdf/table_detector.rs`
  - Group text blocks into cells
  - Build row/column structure
  - Create TableCell objects with content

- [ ] **T021** Implement `TableStructure::to_document_node()` to convert to DocumentNode
  - File: `src/backend/pdf/table.rs`
  - Create NodeType::Table nodes
  - Preserve cell structure in metadata
  - Include row/column information

- [ ] **T022** Update `PdfBackend::convert_pdf()` to detect and include tables
  - File: `src/backend/pdf/backend.rs`
  - Run table detector on each page
  - Insert table nodes in document structure
  - Handle pages with no tables gracefully

### Testing
- [ ] **T023** Enable tests in `tests/integration_pdf_tables.rs` (remove `#[ignore]`)
  - `test_simple_table_detection`
  - `test_table_structure_extraction`
  - `test_table_cell_content`
  - `test_multi_table_page`
  - `test_no_false_positives`

### Deliverable
- [ ] **T024** Run `cargo test --test integration_pdf_tables` and verify all 5 tests pass

---

## Phase 4: Image Extraction (P2 - Medium)

**Goal**: Extract and classify images
**Tests**: 6 tests (image extraction and metadata)

### Image Extraction Implementation
- [ ] **T025** [P] Implement `ImageExtractor::extract_images()` using pdfium image API
  - File: `src/backend/pdf/image_extractor.rs`
  - Use PdfiumImageExtractor
  - Extract embedded images from page objects
  - Get image data as bytes

- [ ] **T026** [P] Implement `ImageExtractor::get_image_metadata()` to extract properties
  - File: `src/backend/pdf/image_extractor.rs`
  - Get width, height, DPI
  - Detect image format (JPEG, PNG, etc.)
  - Calculate aspect ratio

- [ ] **T027** [P] Implement `ImageClassifier::classify_image()` for basic type detection
  - File: `src/backend/pdf/image.rs`
  - Classify as Photo, Diagram, or Chart
  - Use aspect ratio and size heuristics
  - Simple rule-based classification

- [ ] **T028** Update `PdfBackend::convert_pdf()` to extract and include images
  - File: `src/backend/pdf/backend.rs`
  - If `PdfConfig::extract_images` is true
  - Run image extractor on each page
  - Create DocumentNodes for images with metadata

### Testing
- [ ] **T029** Enable tests in `tests/integration_pdf_images.rs` (remove `#[ignore]`)
  - `test_basic_image_extraction`
  - `test_image_metadata_extraction`
  - `test_multiple_images_per_page`
  - `test_image_type_classification`
  - `test_pdf_without_images`
  - `test_image_format_detection`

### Deliverable
- [ ] **T030** Run `cargo test --test integration_pdf_images` and verify all 6 tests pass

---

## Phase 5: OCR Integration (P2 - Medium)

**Goal**: Extract text from scanned PDFs
**Tests**: 3 tests (OCR functionality)

### OCR Implementation
- [ ] **T031** Implement `TesseractOcr::recognize_text()` using rusty-tesseract
  - File: `src/backend/pdf/ocr_engine.rs`
  - Convert page to image if needed
  - Call Tesseract OCR engine
  - Return OcrResult with words and confidence

- [ ] **T032** Implement scanned PDF detection in `PdfBackend`
  - File: `src/backend/pdf/backend.rs`
  - Check if page has no text layer
  - If `PdfConfig::ocr_enabled` and scanned, use OCR
  - Fall back to text extraction if OCR fails

- [ ] **T033** Implement `OcrEngine::extract_with_confidence()` for word-level results
  - File: `src/backend/pdf/ocr_engine.rs`
  - Get bounding boxes for each word
  - Include confidence scores in metadata
  - Support multiple languages from config

### Testing
- [ ] **T034** Enable tests in `tests/integration_pdf_ocr.rs` (remove `#[ignore]`)
  - `test_scanned_pdf_detection`
  - `test_scanned_pdf_ocr` (will need real scanned PDF fixture)
  - `test_ocr_with_word_level_confidence`

### Deliverable
- [ ] **T035** Run `cargo test --test integration_pdf_ocr --features ocr` and verify 3 tests pass

---

## Phase 6: Backend Contracts & Polish (P0 - Critical)

**Goal**: Ensure all contracts met, tests pass, CI green
**Tests**: Contract tests + CLI tests + full suite

### Contract Compliance
- [ ] **T036** Enable tests in `tests/contract_pdf_backend.rs` (remove `#[ignore]`)
  - `test_pdf_backend_implements_backend_trait`
  - `test_pdf_backend_config_options`

- [ ] **T037** Verify Backend trait implementation is complete
  - File: `src/backend/pdf/backend.rs`
  - `convert()` method works for all PDF types
  - `supports_format()` returns true for PDF format
  - All error cases handled properly

### CLI Integration
- [ ] **T038** Enable CLI tests that use PDF backend in `tests/contract_cli.rs`
  - Remove `#[ignore]` from CT-008 (PDF with OCR)
  - Remove `#[ignore]` from CT-009 (PDF options)
  - Verify tests pass

- [ ] **T039** Test CLI with real PDF files manually
  - Single PDF conversion
  - Batch PDF processing
  - PDF with --ocr-enabled flag
  - PDF with --pdf-extract-tables and --pdf-extract-images flags

### Code Quality
- [ ] **T040** Run `cargo clippy --all-targets --all-features -- -D warnings`
  - Fix all clippy warnings in PDF backend modules
  - Ensure no dead code in PDF modules
  - Check for proper error handling

- [ ] **T041** Add documentation comments to public PDF backend APIs
  - Document PdfBackend struct and methods
  - Document PdfConfig options
  - Add examples for common use cases

- [ ] **T042** Run `cargo fmt --all` and commit formatting

### Performance Testing
- [ ] **T043** Create performance benchmarks for PDF backend
  - Measure single-page PDF processing time
  - Measure 10-page PDF processing time
  - Verify memory usage is acceptable
  - Document performance in README or docs

### Final Verification
- [ ] **T044** Run full test suite: `cargo test --all-features`
  - Verify all 30+ PDF tests pass
  - Ensure no regressions in other backends
  - Check CI/CD passes on both macOS and Windows

- [ ] **T045** Update documentation and README
  - Update README.md to reflect PDF support
  - Document PDF backend capabilities
  - Add PDF examples to docs

---

## Summary by Phase

| Phase | Tasks | Tests | Priority | Estimated Time |
|-------|-------|-------|----------|----------------|
| Phase 1: Text Extraction | T001-T012 | 10 | P0 | 2-3 days |
| Phase 2: Layout Analysis | T013-T018 | 4 | P1 | 2-3 days |
| Phase 3: Table Detection | T019-T024 | 5 | P1 | 2-3 days |
| Phase 4: Image Extraction | T025-T030 | 6 | P2 | 1-2 days |
| Phase 5: OCR Integration | T031-T035 | 3 | P2 | 1-2 days |
| Phase 6: Polish & Contracts | T036-T045 | 2+ | P0 | 1 day |
| **Total** | **45 tasks** | **30+ tests** | - | **9-14 days** |

## Parallel Execution Opportunities

Can work in parallel (different files):
- T025, T026, T027 (Image extraction - independent module)
- T040, T041, T042 (Code quality - can run anytime)

Must run sequentially:
- Phase 1 → Phase 2 (layout needs text extraction)
- Phase 2 → Phase 3 (tables need layout analysis)
- Phase 4 → Phase 5 (OCR needs image conversion)
- All phases → Phase 6 (final verification)

## Next Steps

1. Review plan.md and tasks.md
2. Create feature branch: `git checkout -b 007-complete-pdf-backend`
3. Start with T001 (create branch)
4. Work through tasks sequentially by phase
5. Commit after each phase completes with passing tests
