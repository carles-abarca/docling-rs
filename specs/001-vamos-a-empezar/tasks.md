# Implementation Tasks: Core Document Processing Library (MVP Phase 1)

**Feature**: Core Document Processing Library
**Branch**: `001-vamos-a-empezar`
**Generated**: 2025-10-04

## Overview

Total tasks: 36
Estimated parallel groups: 8
TDD Workflow: RED → GREEN → REFACTOR (tests before implementation)

## Task Execution Order

### Phase 1: Setup (Sequential)
- T001-T003: Project initialization

### Phase 2: Data Models (Tests Parallel, Then Implementation Parallel)
- T004-T010: Contract tests for data types [P]
- T011-T017: Type implementations [P]

### Phase 3: Backend Infrastructure (Tests First, Then Sequential Implementation)
- T018-T021: Backend contract tests [P]
- T022: Backend trait definition
- T023-T026: Backend implementations (Sequential - shared trait)

### Phase 4: Pipeline (Sequential - shared types)
- T027-T028: Pipeline tests and implementation

### Phase 5: Converter (Sequential - orchestrates backends)
- T029-T030: Converter tests and implementation

### Phase 6: Integration Tests (Parallel by format)
- T031-T034: End-to-end tests [P]

### Phase 7: Documentation & Polish (Parallel)
- T035-T036: Examples and docs [P]

---

## Setup Tasks

### T001: Initialize Cargo Project [Sequential]
**File**: `Cargo.toml`, `.gitignore`, `README.md`
**Dependencies**: None
**Description**: Create new Rust library project with proper metadata and Git ignore

```bash
cargo init --lib
```

**Cargo.toml contents**:
```toml
[package]
name = "docling-rs"
version = "0.1.0"
edition = "2021"
rust-version = "1.75"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
pulldown-cmark = "0.9"
scraper = "0.17"
csv = "1.3"
docx-rs = "0.4"
infer = "0.15"
thiserror = "1.0"
chrono = { version = "0.4", features = ["serde"] }

[dev-dependencies]
```

**Acceptance**: `cargo check` succeeds, project structure created

---

### T002: Create Module Structure [Sequential]
**File**: `src/lib.rs`, `src/datamodel/mod.rs`, `src/backend/mod.rs`, `src/pipeline/mod.rs`
**Dependencies**: T001
**Description**: Create module hierarchy and public API exports

**src/lib.rs**:
```rust
//! Docling-rs: Native Rust document processing library
//!
//! Extract structured text from Markdown, HTML, CSV, and DOCX files.

pub mod datamodel;
pub mod backend;
pub mod pipeline;
pub mod error;
pub mod format;

mod converter;

// Re-exports
pub use converter::DocumentConverter;
pub use datamodel::{DoclingDocument, InputDocument, ConversionResult};
pub use error::ConversionError;
pub use format::InputFormat;
```

**Acceptance**: `cargo check` succeeds, modules accessible

---

### T003: Setup CI/CD Configuration [Sequential] [P]
**File**: `.github/workflows/ci.yml`, `rustfmt.toml`, `clippy.toml`
**Dependencies**: T001
**Description**: Configure GitHub Actions for testing, linting, formatting on Windows and macOS

**CI workflow must**:
- Run on both Windows and macOS
- Execute: `cargo test`, `cargo clippy`, `cargo fmt --check`
- Cache dependencies

**Acceptance**: CI configuration validates

---

## Data Model Tasks (TDD: Tests First)

### T004: [TEST] DoclingDocument Type Tests [P]
**File**: `src/datamodel/document.rs` (test module)
**Dependencies**: T002
**Description**: Write contract tests for DoclingDocument before implementation

**Test cases**:
1. Create document with metadata
2. Serialize to JSON
3. Export to Markdown
4. Get text content (flattened)
5. Iterate nodes

**Acceptance**: Tests written, compile but fail (RED phase)

---

### T005: [TEST] InputDocument Type Tests [P]
**File**: `src/datamodel/input.rs` (test module)
**Dependencies**: T002
**Description**: Write contract tests for InputDocument

**Test cases**:
1. Create from file path
2. Create from byte stream
3. Format detection

**Acceptance**: Tests written, compile but fail (RED phase)

---

### T006: [TEST] ConversionResult Type Tests [P]
**File**: `src/datamodel/result.rs` (test module)
**Dependencies**: T002
**Description**: Write contract tests for ConversionResult

**Test cases**:
1. Success status
2. Failure status with errors
3. Partial status with warnings
4. Metrics tracking

**Acceptance**: Tests written, compile but fail (RED phase)

---

### T007: [TEST] NodeItem Type Tests [P]
**File**: `src/datamodel/node.rs` (test module)
**Dependencies**: T002
**Description**: Write contract tests for NodeItem (hierarchical)

**Test cases**:
1. Create node with type
2. Add children
3. Get text recursively
4. Iterate descendants

**Acceptance**: Tests written, compile but fail (RED phase)

---

### T008: [TEST] TextItem Type Tests [P]
**File**: `src/datamodel/text.rs` (test module)
**Dependencies**: T002
**Description**: Write contract tests for TextItem with formatting

**Test cases**:
1. Plain text
2. Bold, italic formatting
3. Links
4. Combined formatting

**Acceptance**: Tests written, compile but fail (RED phase)

---

### T009: [TEST] TableData Type Tests [P]
**File**: `src/datamodel/table.rs` (test module)
**Dependencies**: T002
**Description**: Write contract tests for TableData

**Test cases**:
1. Table with headers
2. Table without headers
3. Cells with colspan/rowspan
4. Table metadata

**Acceptance**: Tests written, compile but fail (RED phase)

---

### T010: [TEST] Error Types Tests [P]
**File**: `src/error.rs` (test module)
**Dependencies**: T002
**Description**: Write tests for ConversionError variants

**Test cases**:
1. FileNotFound error
2. UnsupportedFormat error
3. ParseError with context
4. Error message formatting

**Acceptance**: Tests written, compile but fail (RED phase)

---

### T011: [IMPL] Implement DoclingDocument Type [P]
**File**: `src/datamodel/document.rs`
**Dependencies**: T004 (test must exist first)
**Description**: Implement DoclingDocument to pass T004 tests

**Implementation must**:
- Define struct with metadata and root node
- Implement `to_json()` using serde
- Implement `to_markdown()` (traverse tree)
- Implement `get_text()` (flatten)
- Implement `iter_nodes()` iterator

**Acceptance**: T004 tests pass (GREEN phase)

---

### T012: [IMPL] Implement InputDocument Type [P]
**File**: `src/datamodel/input.rs`
**Dependencies**: T005
**Description**: Implement InputDocument to pass T005 tests

**Acceptance**: T005 tests pass (GREEN phase)

---

### T013: [IMPL] Implement ConversionResult Type [P]
**File**: `src/datamodel/result.rs`
**Dependencies**: T006
**Description**: Implement ConversionResult to pass T006 tests

**Acceptance**: T006 tests pass (GREEN phase)

---

### T014: [IMPL] Implement NodeItem Type [P]
**File**: `src/datamodel/node.rs`
**Dependencies**: T007
**Description**: Implement NodeItem and NodeType enum to pass T007 tests

**Acceptance**: T007 tests pass (GREEN phase)

---

### T015: [IMPL] Implement TextItem Type [P]
**File**: `src/datamodel/text.rs`
**Dependencies**: T008
**Description**: Implement TextItem and Formatting to pass T008 tests

**Acceptance**: T008 tests pass (GREEN phase)

---

### T016: [IMPL] Implement TableData Type [P]
**File**: `src/datamodel/table.rs`
**Dependencies**: T009
**Description**: Implement TableData, TableCell, TableMetadata to pass T009 tests

**Acceptance**: T009 tests pass (GREEN phase)

---

### T017: [IMPL] Implement Error Types [P]
**File**: `src/error.rs`
**Dependencies**: T010
**Description**: Implement ConversionError enum using thiserror to pass T010 tests

**Acceptance**: T010 tests pass (GREEN phase)

---

## Backend Tasks (TDD: Contract Tests First)

### T018: [TEST] Backend Trait Contract Tests [P]
**File**: `tests/contract/backend_trait.rs`
**Dependencies**: T011-T017 (data types needed)
**Description**: Write contract tests for Backend trait interface

**Test cases** (using mock backend):
1. `is_valid()` returns bool
2. `supported_formats()` returns non-empty slice
3. `convert()` returns Result
4. `convert()` doesn't panic on invalid input

**Acceptance**: Tests written and compile (RED phase)

---

### T019: [TEST] Markdown Backend Contract Tests [P]
**File**: `tests/contract/backend_markdown.rs`, `tests/fixtures/sample.md`
**Dependencies**: T011-T017
**Description**: Write contract tests for Markdown backend per `contracts/markdown_backend.md`

**Test cases**:
1. Headings (H1-H6)
2. Paragraphs
3. Lists (ordered/unordered)
4. Code blocks
5. Inline formatting

**Acceptance**: Tests written, compile but fail (RED phase)

---

### T020: [TEST] HTML Backend Contract Tests [P]
**File**: `tests/contract/backend_html.rs`, `tests/fixtures/sample.html`
**Dependencies**: T011-T017
**Description**: Write contract tests for HTML backend per `contracts/html_backend.md`

**Test cases**:
1. Semantic tags (h1-h6, p, ul, ol)
2. Tables with headers
3. Malformed HTML (error recovery)
4. Inline formatting

**Acceptance**: Tests written, compile but fail (RED phase)

---

### T021: [TEST] CSV & DOCX Backend Contract Tests [P]
**File**: `tests/contract/backend_csv.rs`, `tests/contract/backend_docx.rs`
**Dependencies**: T011-T017
**Description**: Write contract tests for CSV and DOCX backends

**CSV test cases**:
1. CSV with headers
2. Inconsistent column counts

**DOCX test cases**:
1. Paragraphs with formatting
2. Tables
3. Lists
4. Images (metadata only)

**Acceptance**: Tests written, compile but fail (RED phase)

---

### T022: [IMPL] Backend Traits Definition [Sequential]
**File**: `src/backend/traits.rs`, `src/backend/mod.rs`
**Dependencies**: T018
**Description**: Define Backend and DeclarativeBackend traits to pass T018

**Implementation**:
```rust
pub trait Backend {
    fn is_valid(&self) -> bool;
    fn supported_formats() -> &'static [InputFormat];
    fn convert(&self, input: &InputDocument) -> Result<DoclingDocument, ConversionError>;
}

pub trait DeclarativeBackend: Backend {}
```

**Acceptance**: T018 tests pass (GREEN phase)

---

### T023: [IMPL] Markdown Backend Implementation [Sequential]
**File**: `src/backend/markdown.rs`
**Dependencies**: T019, T022
**Description**: Implement MarkdownBackend using pulldown-cmark to pass T019 tests

**Must handle**:
- Headings → NodeType::Heading
- Paragraphs → NodeType::Paragraph
- Lists → NodeType::List
- Code blocks → NodeType::CodeBlock
- Inline formatting (bold, italic, links)

**Acceptance**: T019 tests pass (GREEN phase)

---

### T024: [IMPL] HTML Backend Implementation [Sequential]
**File**: `src/backend/html.rs`
**Dependencies**: T020, T022
**Description**: Implement HtmlBackend using scraper to pass T020 tests

**Must handle**:
- Semantic HTML tags
- Tables with CSS selectors
- Malformed HTML (graceful degradation)
- Inline formatting

**Acceptance**: T020 tests pass (GREEN phase)

---

### T025: [IMPL] CSV Backend Implementation [Sequential]
**File**: `src/backend/csv.rs`
**Dependencies**: T021, T022
**Description**: Implement CsvBackend using csv crate to pass CSV tests

**Must handle**:
- Headers detection
- Inconsistent columns (pad/truncate)
- Create TableData structure

**Acceptance**: CSV tests from T021 pass (GREEN phase)

---

### T026: [IMPL] DOCX Backend Implementation [Sequential]
**File**: `src/backend/docx.rs`
**Dependencies**: T021, T022
**Description**: Implement DocxBackend using docx-rs to pass DOCX tests

**Must handle**:
- Paragraphs with formatting
- Tables (colspan/rowspan)
- Lists
- Images (metadata extraction only)

**Acceptance**: DOCX tests from T021 pass (GREEN phase)

---

## Pipeline Tasks

### T027: [TEST] SimplePipeline Tests [Sequential]
**File**: `src/pipeline/simple.rs` (test module)
**Dependencies**: T023-T026 (backends needed)
**Description**: Write tests for SimplePipeline (build → assemble → enrich)

**Test cases**:
1. Execute pipeline with valid backend
2. Handle backend errors
3. Populate metadata
4. Return ConversionResult

**Acceptance**: Tests written, compile but fail (RED phase)

---

### T028: [IMPL] SimplePipeline Implementation [Sequential]
**File**: `src/pipeline/simple.rs`, `src/pipeline/traits.rs`
**Dependencies**: T027
**Description**: Implement SimplePipeline to pass T027 tests

**Implementation**:
- Define Pipeline trait
- Implement SimplePipeline
- Call backend.convert() in build phase
- Validate and enrich in later phases

**Acceptance**: T027 tests pass (GREEN phase)

---

## Converter Tasks

### T029: [TEST] DocumentConverter Tests [Sequential]
**File**: `src/converter.rs` (test module), `src/format.rs` (test module)
**Dependencies**: T028
**Description**: Write tests for DocumentConverter (main entry point)

**Test cases**:
1. Convert from file path
2. Convert from byte stream
3. Auto-detect format (extension + magic bytes)
4. Route to correct backend
5. Handle unsupported format error

**Acceptance**: Tests written, compile but fail (RED phase)

---

### T030: [IMPL] DocumentConverter Implementation [Sequential]
**File**: `src/converter.rs`, `src/format.rs`
**Dependencies**: T029
**Description**: Implement DocumentConverter to pass T029 tests

**Implementation**:
- Format detection (infer crate + extension)
- Backend selection
- Pipeline execution
- Return ConversionResult

**Acceptance**: T029 tests pass (GREEN phase)

---

## Integration Tests (End-to-End)

### T031: [TEST] Markdown Integration Tests [P]
**File**: `tests/integration/markdown_conversion.rs`
**Dependencies**: T030
**Description**: End-to-end tests for Markdown conversion per quickstart.md

**Test cases**:
1. Convert real Markdown file
2. Verify structure (headings, paragraphs, lists)
3. Export to JSON
4. Export to Markdown

**Acceptance**: All test cases pass

---

### T032: [TEST] HTML Integration Tests [P]
**File**: `tests/integration/html_conversion.rs`
**Dependencies**: T030
**Description**: End-to-end tests for HTML conversion

**Test cases**:
1. Convert HTML with tables
2. Verify semantic structure
3. Handle malformed HTML

**Acceptance**: All test cases pass

---

### T033: [TEST] CSV Integration Tests [P]
**File**: `tests/integration/csv_conversion.rs`
**Dependencies**: T030
**Description**: End-to-end tests for CSV conversion

**Test cases**:
1. CSV with headers
2. Inconsistent columns
3. Table structure validation

**Acceptance**: All test cases pass

---

### T034: [TEST] DOCX Integration Tests [P]
**File**: `tests/integration/docx_conversion.rs`
**Dependencies**: T030
**Description**: End-to-end tests for DOCX conversion

**Test cases**:
1. DOCX with mixed content
2. Formatting preservation
3. Image metadata extraction

**Acceptance**: All test cases pass

---

## Documentation & Polish

### T035: [DOC] Create Basic Example [P]
**File**: `examples/basic_conversion.rs`
**Dependencies**: T030
**Description**: Create runnable example from quickstart.md

**Example must demonstrate**:
- Convert file from each format
- Export to JSON
- Export to Markdown
- Error handling

**Acceptance**: `cargo run --example basic_conversion` succeeds

---

### T036: [DOC] Write Public API Documentation [P]
**File**: `src/**/*.rs` (rustdoc comments)
**Dependencies**: T030
**Description**: Add comprehensive rustdoc to all public types and functions

**Must document**:
- All public structs, enums, traits
- All public functions with examples
- Module-level documentation

**Acceptance**: `cargo doc --open` generates complete documentation

---

## Parallel Execution Guide

### Group 1: Setup (Run Sequentially)
```bash
# T001 → T002 → T003
```

### Group 2: Data Model Tests (Parallel)
```bash
# All can run in parallel (different files)
Task: T004, T005, T006, T007, T008, T009, T010
```

### Group 3: Data Model Implementation (Parallel)
```bash
# After all tests from Group 2 exist
Task: T011, T012, T013, T014, T015, T016, T017
```

### Group 4: Backend Contract Tests (Parallel)
```bash
Task: T018, T019, T020, T021
```

### Group 5: Backend Implementation (Sequential)
```bash
# T022 first (trait), then T023 → T024 → T025 → T026
# Sequential because they share the Backend trait
```

### Group 6: Pipeline (Sequential)
```bash
# T027 → T028
```

### Group 7: Converter (Sequential)
```bash
# T029 → T030
```

### Group 8: Integration Tests (Parallel)
```bash
Task: T031, T032, T033, T034
```

### Group 9: Documentation (Parallel)
```bash
Task: T035, T036
```

---

## Task Checklist

**Setup** (3 tasks):
- [ ] T001: Initialize Cargo Project
- [ ] T002: Create Module Structure
- [ ] T003: Setup CI/CD Configuration

**Data Models** (14 tasks):
- [ ] T004-T010: Data type tests [7 tests]
- [ ] T011-T017: Data type implementations [7 impls]

**Backends** (9 tasks):
- [ ] T018-T021: Backend contract tests [4 tests]
- [ ] T022: Backend traits
- [ ] T023-T026: Backend implementations [4 impls]

**Pipeline** (2 tasks):
- [ ] T027-T028: Pipeline test + implementation

**Converter** (2 tasks):
- [ ] T029-T030: Converter test + implementation

**Integration** (4 tasks):
- [ ] T031-T034: End-to-end tests [4 tests]

**Documentation** (2 tasks):
- [ ] T035-T036: Examples + rustdoc

**Total**: 36 tasks

---

## Success Criteria

All tasks complete when:
- [x] `cargo test` passes (100% of tests)
- [x] `cargo clippy` produces no warnings
- [x] `cargo fmt --check` passes
- [x] `cargo doc` generates complete documentation
- [x] All 4 backends convert sample documents successfully
- [x] JSON and Markdown export works
- [x] No unsafe code (unless justified)
- [x] Cross-platform (Windows + macOS tests pass)

---

**Ready for Implementation**: YES
**TDD Workflow**: Tests before implementation (RED → GREEN → REFACTOR)
**Next Command**: Start with T001 (Initialize Cargo Project)
