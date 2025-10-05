# Implementation Plan: Advanced PDF Processing (Phase 3)

**Branch**: `003-phase-3-pdf` | **Date**: 2025-10-05 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/003-phase-3-pdf/spec.md`

## Execution Flow (/plan command scope)
```
1. Load feature spec from Input path ✓
2. Fill Technical Context ✓
3. Fill Constitution Check ✓
4. Evaluate Constitution Check ✓
5. Execute Phase 0 → research.md ✓
6. Execute Phase 1 → contracts, data-model.md, quickstart.md ✓
7. Re-evaluate Constitution Check ✓
8. Plan Phase 2 → Describe task generation approach ✓
9. STOP - Ready for /tasks command ✓
```

## Summary

Advanced PDF processing system implementing all core docling Python capabilities using native Rust libraries. The system provides text extraction with position metadata, layout analysis for reading order detection, table structure detection and extraction, image processing with classification, OCR for scanned PDFs, and content enrichment (code blocks, formulas). Implementation follows a phased approach (3a-3f) to manage complexity while maintaining compatibility with Phase 1 Backend trait and Phase 2 chunking.

## Technical Context

**Language/Version**: Rust 1.75+
**Primary Dependencies**:
- `pdfium-render` (PDF parsing and text extraction)
- `tract` or `candle` (ML model inference for layout analysis)
- `tesseract-rs` or `leptess` (OCR engine)
- `image` (image processing and extraction)
- `serde` + `serde_json` (serialization)

**Storage**: N/A (in-memory processing with file I/O)
**Testing**: `cargo test` (contract tests, integration tests, unit tests)
**Target Platform**: Windows and macOS (cross-platform native)
**Project Type**: single (Rust library with Backend trait implementation)
**Performance Goals**:
- Simple PDFs (<10 pages) process in <2 seconds
- Memory usage <2x PDF file size
- Parallel OCR processing per page

**Constraints**:
- Text extraction accuracy >98% for standard PDFs
- Layout reading order >90% correct for 2-column layouts
- Table detection precision/recall >85%
- OCR accuracy >92% for clear scanned documents
- NO Python dependencies (native Rust only)
- NO GPU requirement (CPU-only ML inference)

**Scale/Scope**:
- Support PDFs with hundreds of pages
- Handle complex multi-column layouts
- Process tables with merged cells
- Extract and classify multiple image types per page

## Constitution Check
*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

✅ **I. Library-First Architecture**: PdfBackend will be self-contained module implementing Backend trait with clear API boundaries
✅ **II. CLI Interface Contract**: Will expose PDF processing via CLI with stdin/stdout following existing converter pattern
✅ **III. Test-Driven Development**: TDD mandatory - tests written from spec before implementation
✅ **IV. Integration & Contract Testing**: Contract tests required for PdfBackend implementing Backend trait
✅ **V. Rust Best Practices**: Follow all Rust idioms, clippy lints, Result<T,E> error handling, comprehensive rustdoc
✅ **VI. Cross-Platform Compatibility**: All PDF processing works on Windows and macOS via platform-agnostic APIs
✅ **VII. Native Rust Dependencies**: ALL dependencies are native Rust crates (pdfium-render, tract/candle, tesseract-rs, image)

**Status**: PASS - No constitutional violations

## Project Structure

### Documentation (this feature)
```
specs/003-phase-3-pdf/
├── plan.md              # This file (/plan command output)
├── research.md          # Phase 0 output (/plan command)
├── data-model.md        # Phase 1 output (/plan command)
├── quickstart.md        # Phase 1 output (/plan command)
├── contracts/           # Phase 1 output (/plan command)
└── tasks.md             # Phase 2 output (/tasks command - NOT created by /plan)
```

### Source Code (repository root)
```
src/
├── backend/
│   ├── mod.rs
│   ├── traits.rs
│   ├── pdf/                    # NEW: PDF backend module
│   │   ├── mod.rs
│   │   ├── backend.rs          # PdfBackend implementing Backend trait
│   │   ├── document.rs         # PdfDocument representation
│   │   ├── page.rs             # PdfPage with text, layout, tables, images
│   │   ├── text_extractor.rs   # Text extraction with positions
│   │   ├── layout_analyzer.rs  # Layout analysis and reading order
│   │   ├── table_detector.rs   # Table structure detection
│   │   ├── image_extractor.rs  # Image detection and extraction
│   │   ├── ocr_engine.rs       # OCR integration
│   │   ├── content_enricher.rs # Code/formula/list detection
│   │   └── pipeline.rs         # PDF processing pipeline orchestration
│   ├── markdown.rs
│   ├── html.rs
│   ├── csv.rs
│   └── docx.rs
├── datamodel/
│   ├── mod.rs
│   ├── pdf_element.rs          # NEW: PDF-specific element types
│   └── ...
└── ...

tests/
├── contract/
│   └── pdf_backend.rs          # NEW: Contract tests for PdfBackend
├── integration/
│   ├── pdf_text_extraction.rs  # NEW: Text extraction integration tests
│   ├── pdf_layout_analysis.rs  # NEW: Layout analysis tests
│   ├── pdf_table_detection.rs  # NEW: Table detection tests
│   ├── pdf_ocr.rs              # NEW: OCR integration tests
│   └── pdf_end_to_end.rs       # NEW: Complete PDF workflow tests
└── ...
```

**Structure Decision**: Single project structure (Option 1) - PDF backend is a new module within existing `src/backend/` directory, following the same pattern as markdown, html, csv, and docx backends. All PDF-specific functionality is contained within `src/backend/pdf/` submodule.

## Phase 0: Outline & Research

**Research Tasks** (all written to research.md):

1. **pdfium-render capabilities research**:
   - Decision: Use `pdfium-render` for PDF parsing
   - Rationale: Mature Rust binding to Google's Pdfium library, active maintenance, cross-platform
   - Alternatives: `pdf` crate (less feature-complete), `lopdf` (lower-level)
   - API coverage: text extraction with positions, page rendering, metadata extraction, encryption support

2. **ML inference library selection**:
   - Decision: Use `tract` for ML model inference
   - Rationale: Production-ready ONNX runtime in Rust, CPU-only support, good performance
   - Alternatives: `candle` (newer, less battle-tested), `burn` (still maturing)
   - Integration: Load layout analysis models in ONNX format

3. **OCR engine selection**:
   - Decision: Use `tesseract-rs` for OCR
   - Rationale: Rust bindings to Tesseract OCR, proven accuracy, language support
   - Alternatives: `leptess` (similar), pure Rust OCR (not mature enough)
   - Requirements: tesseract system library must be available

4. **Layout analysis approach**:
   - Decision: Start with rule-based layout analysis, prepare for ML model integration
   - Rationale: Rule-based provides immediate value, ML models can be added incrementally
   - Rules: Column detection via whitespace analysis, reading order via position sorting
   - ML path: Support for ONNX layout models via tract

5. **Table detection strategy**:
   - Decision: Hybrid approach - rule-based grid detection + optional ML model
   - Rationale: Grid lines/alignment rules work for most tables, ML for complex cases
   - Rules: Detect aligned text blocks, identify cell boundaries
   - ML path: Table detection model via tract (Phase 3c)

6. **Image processing requirements**:
   - Decision: Use `image` crate for bitmap operations
   - Rationale: De facto standard for image processing in Rust, format support
   - Operations: Extract image regions from PDF, convert to standard formats, basic classification

**Output**: research.md complete with all technology decisions documented

## Phase 1: Design & Contracts

### Data Model (data-model.md)

**Core Entities**:

1. **PdfDocument**
   - Fields: pages (Vec<PdfPage>), metadata (PdfMetadata), encryption_info (Option<EncryptionInfo>)
   - Validation: page_count > 0, metadata required
   - Methods: load_from_file(), load_from_bytes(), get_page(), iter_pages()

2. **PdfPage**
   - Fields: page_number (usize), text_blocks (Vec<TextBlock>), tables (Vec<Table>), images (Vec<ImageRegion>), dimensions (PageDimensions)
   - Relationships: belongs to PdfDocument
   - State: raw → analyzed → enriched

3. **PdfElement** (enum)
   - Variants: TextBlock, Table, Image, Formula, CodeBlock, List
   - Common: bounding_box, confidence_score, element_type
   - Serialization: serde compatible for DoclingDocument mapping

4. **TextBlock**
   - Fields: text (String), font_info (FontInfo), position (BoundingBox), reading_order (usize)
   - Layout: column_id, block_type (heading/paragraph/list_item)

5. **Table**
   - Fields: cells (Vec<TableCell>), structure (TableStructure), header_rows (usize)
   - TableCell: row, col, rowspan, colspan, content
   - TableStructure: rows, cols, merged_cells

6. **ImageRegion**
   - Fields: image_type (ImageType), bbox (BoundingBox), bitmap (Option<Vec<u8>>), metadata (ImageMetadata)
   - ImageType: Photo, Diagram, Logo, Chart (classification)

7. **LayoutInfo**
   - Fields: columns (Vec<Column>), reading_order (Vec<ElementId>), confidence (f32)
   - Column: bbox, elements

8. **OcrResult**
   - Fields: text (String), confidence (f32), words (Vec<OcrWord>), language (String)
   - OcrWord: text, bbox, confidence

9. **PdfMetadata**
   - Fields: title, author, subject, keywords, creator, producer, creation_date, mod_date, page_count, version

10. **PdfConfig**
    - Fields: password (Option<String>), page_range (Option<Range>), enable_ocr (bool), enable_tables (bool), enable_images (bool), ocr_language (String)

### Contracts

**Backend Trait Contract** (contracts/pdf_backend.md):
```rust
impl Backend for PdfBackend {
    fn convert(&self, input: &InputDocument) -> Result<DoclingDocument, ConversionError>;
}
```

**PdfBackend API Contract** (contracts/pdf_backend_api.md):
- `PdfBackend::new() -> Self`
- `PdfBackend::with_config(config: PdfConfig) -> Self`
- `fn load_document(&self, input: &InputDocument) -> Result<PdfDocument>`
- `fn extract_text(&self, page: &PdfPage) -> Result<Vec<TextBlock>>`
- `fn analyze_layout(&self, page: &PdfPage) -> Result<LayoutInfo>`
- `fn detect_tables(&self, page: &PdfPage) -> Result<Vec<Table>>`
- `fn extract_images(&self, page: &PdfPage) -> Result<Vec<ImageRegion>>`
- `fn run_ocr(&self, page: &PdfPage) -> Result<OcrResult>`
- `fn to_docling_document(&self, pdf_doc: &PdfDocument) -> Result<DoclingDocument>`

**Contract Tests** (failing initially):
- tests/contract/pdf_backend.rs - Backend trait compliance
- tests/contract/pdf_text_extraction.rs - Text extraction contract
- tests/contract/pdf_layout.rs - Layout analysis contract
- tests/contract/pdf_tables.rs - Table detection contract
- tests/contract/pdf_ocr.rs - OCR contract

### Integration Test Scenarios (from user stories)

1. **Multi-content PDF extraction** (integration/pdf_multi_content.rs):
   - Given PDF with text, tables, images
   - When processed
   - Then all elements extracted with types and positions

2. **Multi-column layout** (integration/pdf_multicolumn.rs):
   - Given 2-column PDF
   - When layout analyzed
   - Then reading order matches column flow

3. **Table structure extraction** (integration/pdf_tables.rs):
   - Given PDF with tables
   - When table detected
   - Then cells, structure, headers extracted

4. **Image extraction** (integration/pdf_images.rs):
   - Given PDF with images
   - When processed
   - Then images detected with bboxes, extractable as bitmaps

5. **Scanned PDF OCR** (integration/pdf_ocr.rs):
   - Given scanned PDF
   - When OCR enabled
   - Then text extracted with positions and confidence

6. **Code and formula detection** (integration/pdf_enrichment.rs):
   - Given PDF with code/formulas
   - When enrichment enabled
   - Then elements identified and classified

7. **Document hierarchy** (integration/pdf_hierarchy.rs):
   - Given PDF with sections/headings
   - When processed
   - Then structure preserved with nesting

### Quickstart (quickstart.md)

```rust
use docling_rs::{DocumentConverter, InputFormat};
use docling_rs::backend::pdf::PdfConfig;

// Basic PDF text extraction
let converter = DocumentConverter::new();
let result = converter.convert_file("document.pdf")?;
let doc = result.document();

// With OCR for scanned PDFs
let config = PdfConfig::default()
    .enable_ocr(true)
    .ocr_language("eng");
let backend = PdfBackend::with_config(config);
// ... use backend

// With Phase 2 chunking
use docling_rs::chunking::HybridChunker;
let chunker = HybridChunker::new(tokenizer);
for chunk in chunker.chunk(&doc) {
    println!("{}", chunk.text);
}
```

### Agent File Update

Execute: `.specify/scripts/bash/update-agent-context.sh claude`
- Add: Rust 1.75+, pdfium-render, tract, tesseract-rs, image
- Update recent changes: Phase 3 PDF processing with layout analysis, tables, OCR
- Preserve manual additions

**Output**: data-model.md, contracts/*, failing contract tests, quickstart.md, CLAUDE.md updated

## Phase 2: Task Planning Approach
*This section describes what the /tasks command will do - DO NOT execute during /plan*

**Task Generation Strategy**:
1. Load `.specify/templates/tasks-template.md` as base
2. Generate from Phase 1 artifacts:
   - Each contract → contract test task [P] (can run in parallel)
   - Each entity → model/struct creation task [P]
   - Each integration scenario → integration test task
   - Implementation tasks in dependency order

**Task Categories**:

**Phase 3a: Foundation** (T001-T010)
- T001: [P] Create PdfBackend struct and Backend trait impl skeleton
- T002: [P] Create PdfDocument, PdfPage, PdfMetadata structs
- T003: [P] Contract test: pdf_backend.rs (Backend trait compliance)
- T004: Integrate pdfium-render for PDF loading
- T005: Implement basic text extraction with positions
- T006: Implement password/encryption handling
- T007: Map PDF content to DoclingDocument
- T008: [P] Integration test: basic text extraction
- T009: [P] Integration test: multi-page handling
- T010: [P] Integration test: encrypted PDFs

**Phase 3b: Layout Analysis** (T011-T015)
- T011: [P] Create LayoutAnalyzer struct with rule-based algorithm
- T012: Implement column detection via whitespace analysis
- T013: Implement reading order determination
- T014: [P] Contract test: pdf_layout.rs
- T015: [P] Integration test: multi-column layout

**Phase 3c: Table Detection** (T016-T020)
- T016: [P] Create TableDetector with grid detection rules
- T017: Implement cell boundary detection
- T018: Implement table structure extraction
- T019: [P] Contract test: pdf_tables.rs
- T020: [P] Integration test: complex table extraction

**Phase 3d: Image Processing** (T021-T025)
- T021: [P] Create ImageExtractor using image crate
- T022: Implement image region detection from PDF
- T023: Implement bitmap extraction and format conversion
- T024: Implement basic image type classification
- T025: [P] Integration test: image extraction

**Phase 3e: OCR Integration** (T026-T030)
- T026: [P] Create OcrEngine wrapper for tesseract-rs
- T027: Implement scanned PDF detection logic
- T028: Implement OCR with confidence scores
- T029: [P] Contract test: pdf_ocr.rs
- T030: [P] Integration test: scanned PDF processing

**Phase 3f: Content Enrichment** (T031-T036)
- T031: [P] Create ContentEnricher with pattern detection
- T032: Implement code block detection (regex)
- T033: Implement formula detection (symbol patterns)
- T034: Implement list structure detection
- T035: [P] Integration test: code and formula detection
- T036: Final end-to-end integration test

**Ordering Strategy**:
- TDD order: Contract/integration tests before implementation
- Dependency order: Foundation → Layout → Tables → Images → OCR → Enrichment
- [P] marks indicate parallelizable tasks (independent modules)

**Estimated Output**: 36 numbered, dependency-ordered tasks in tasks.md

**IMPORTANT**: This phase is executed by the /tasks command, NOT by /plan

## Phase 3+: Future Implementation
*These phases are beyond the scope of the /plan command*

**Phase 3**: Task execution (/tasks command creates tasks.md)
**Phase 4**: Implementation (execute tasks.md following TDD and constitutional principles)
**Phase 5**: Validation (all tests pass on Windows/macOS, quickstart validates, performance benchmarks meet targets)

## Complexity Tracking
*No constitutional violations - this section intentionally left empty*

## Progress Tracking
*This checklist is updated during execution flow*

**Phase Status**:
- [x] Phase 0: Research complete (/plan command)
- [x] Phase 1: Design complete (/plan command)
- [x] Phase 2: Task planning complete (/plan command - describe approach only)
- [ ] Phase 3: Tasks generated (/tasks command)
- [ ] Phase 4: Implementation complete
- [ ] Phase 5: Validation passed

**Gate Status**:
- [x] Initial Constitution Check: PASS
- [x] Post-Design Constitution Check: PASS
- [x] All NEEDS CLARIFICATION resolved
- [x] Complexity deviations documented (N/A - no violations)

---
*Based on Constitution v1.2.0 - See `/memory/constitution.md`*
