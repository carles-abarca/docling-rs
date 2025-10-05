# Feature Specification: Command-Line Interface for docling-rs

**Feature Branch**: `005-create-a-cli`
**Created**: 2025-10-05
**Status**: Draft
**Input**: User description: "Create a CLI for docling-rs that matches the original docling Python CLI functionality"

## Execution Flow (main)
```
1. Parse user description from Input
   → Extract: CLI tool for document conversion with docling-rs library
2. Extract key concepts from description
   → Actors: Command-line users, batch processors, developers
   → Actions: Convert documents, configure processing, view results
   → Data: Documents (PDF, DOCX, MD, HTML, CSV), conversion outputs
   → Constraints: Match docling Python CLI behavior, Rust ecosystem
3. For each unclear aspect:
   → All requirements clearly specified in user input
4. Fill User Scenarios & Testing section
   → Single file conversion, batch processing, format-specific options
5. Generate Functional Requirements
   → Input handling, output generation, configuration, error handling
6. Identify Key Entities
   → CLI Arguments, Conversion Jobs, Output Files
7. Run Review Checklist
   → No implementation details (kept at requirements level)
   → All requirements testable and measurable
8. Return: SUCCESS (spec ready for planning)
```

---

## ⚡ Quick Guidelines
- ✅ Focus on WHAT users need and WHY
- ❌ Avoid HOW to implement (no tech stack, APIs, code structure)
- 👥 Written for business stakeholders, not developers

---

## User Scenarios & Testing

### Primary User Story
As a **developer or researcher**, I want to **convert documents (PDFs, Word files, Markdown, etc.) to structured formats from the command line** so that I can **easily integrate document processing into scripts, pipelines, and workflows without writing code**.

### Acceptance Scenarios

#### Scenario 1: Simple Single File Conversion
1. **Given** I have a PDF file `research-paper.pdf` in my current directory
2. **When** I run `docling-rs research-paper.pdf`
3. **Then** the system creates `research-paper.md` (Markdown) in the current directory with extracted content

#### Scenario 2: Multiple Output Formats
1. **Given** I have a PDF file `document.pdf`
2. **When** I run `docling-rs document.pdf --to json --to md --to text`
3. **Then** the system creates three files: `document.json`, `document.md`, `document.txt` with identical content in different formats

#### Scenario 3: Batch Directory Processing
1. **Given** I have a directory `./papers/` containing 10 PDF files and 5 DOCX files
2. **When** I run `docling-rs ./papers --from pdf --from docx --output ./converted`
3. **Then** the system converts all 15 files to Markdown format in `./converted/` directory, showing progress as it processes

#### Scenario 4: PDF-Specific OCR Processing
1. **Given** I have a scanned PDF `scan.pdf` with no embedded text
2. **When** I run `docling-rs scan.pdf --ocr --ocr-lang eng`
3. **Then** the system performs OCR on the PDF and outputs text extracted via optical character recognition

#### Scenario 5: Advanced PDF Enrichment
1. **Given** I have a technical paper `algorithms.pdf` with code blocks and formulas
2. **When** I run `docling-rs algorithms.pdf --enrich-code --enrich-formula --to json`
3. **Then** the output JSON contains structured code blocks and mathematical formulas identified separately from regular text

#### Scenario 6: Error Handling in Batch
1. **Given** I have a directory with 3 valid PDFs and 1 corrupted PDF
2. **When** I run `docling-rs ./batch --output ./results` (default: continue on error)
3. **Then** the system converts the 3 valid files, logs an error for the corrupted file, and continues processing

#### Scenario 7: Abort on First Error
1. **Given** I have a directory with files where the 2nd file is corrupted
2. **When** I run `docling-rs ./batch --abort-on-error`
3. **Then** the system converts the 1st file, encounters an error on the 2nd file, and stops immediately without processing remaining files

### Edge Cases
- What happens when **output file already exists**? → System overwrites (or errors with clear message)
- What happens when **input file doesn't exist**? → System shows error: "File not found: path/to/file.pdf" and exits with code 1
- What happens when **output directory doesn't exist**? → System creates the directory automatically
- What happens when **unsupported format is specified**? → System shows error: "Unsupported format: xyz" and lists supported formats
- What happens when **no input files match filter** (e.g., `--from pdf` but directory has no PDFs)? → System shows warning: "No files found matching criteria" and exits with code 0
- What happens when **user provides invalid flag combination**? → System shows error with helpful message and suggests correct usage
- What happens when **OCR feature is used but not compiled** (no `ocr` feature)? → System shows error: "OCR feature not available. Recompile with --features ocr"
- What happens with **very large batch** (1000+ files)? → System shows progress bar and processes all files without memory issues
- What happens when **file paths contain spaces**? → System handles paths correctly (both quoted and unquoted)

---

## Requirements

### Functional Requirements

#### Input Handling
- **FR-001**: System MUST accept a single file path as input for conversion
- **FR-002**: System MUST accept a directory path as input for batch conversion of all supported files
- **FR-003**: System MUST support filtering input files by format using `--from FORMAT` flag (can be specified multiple times)
- **FR-004**: System MUST validate that input files/directories exist before processing
- **FR-005**: System MUST handle file paths with spaces, special characters, and Unicode correctly
- **FR-006**: System MUST support both absolute and relative file paths

#### Output Handling
- **FR-007**: System MUST output to Markdown format by default
- **FR-008**: System MUST support multiple output formats simultaneously via `--to FORMAT` flag (can be specified multiple times: md, json, text)
- **FR-009**: System MUST write output to current directory by default
- **FR-010**: System MUST support custom output directory via `--output PATH` flag
- **FR-011**: System MUST create output directory if it doesn't exist
- **FR-012**: System MUST generate output filenames based on input filenames with appropriate extensions
- **FR-013**: System MUST handle filename collisions (overwrite with warning or error clearly)

#### Format Support
- **FR-014**: System MUST support PDF input format with full Phase 3 capabilities (text, layout, tables, images, OCR)
- **FR-015**: System MUST support Markdown input format (Phase 1)
- **FR-016**: System MUST support HTML input format (Phase 1)
- **FR-017**: System MUST support CSV input format (Phase 1)
- **FR-018**: System MUST support DOCX input format (Phase 1)
- **FR-019**: System MUST support Markdown output format
- **FR-020**: System MUST support JSON output format (serialized DoclingDocument)
- **FR-021**: System MUST support plain text output format

#### PDF-Specific Options
- **FR-022**: System MUST support enabling/disabling OCR via `--ocr` / `--no-ocr` flags
- **FR-023**: System MUST support OCR language selection via `--ocr-lang LANG` flag (default: "eng")
- **FR-024**: System MUST support forcing OCR on PDFs with embedded text via `--force-ocr` flag
- **FR-025**: System MUST support enabling/disabling table detection via `--tables` / `--no-tables` flags
- **FR-026**: System MUST support enabling/disabling image extraction via `--images` / `--no-images` flags
- **FR-027**: System MUST support PDF backend selection via `--pdf-backend BACKEND` flag (default: pdfium)

#### Content Enrichment (Phase 3f Integration)
- **FR-028**: System MUST support code block detection via `--enrich-code` flag
- **FR-029**: System MUST support mathematical formula detection via `--enrich-formula` flag
- **FR-030**: System MUST support list structure detection via `--enrich-lists` flag

#### User Experience
- **FR-031**: System MUST display version information via `--version` flag
- **FR-032**: System MUST display comprehensive help text via `--help` flag
- **FR-033**: System MUST support verbosity levels via `--verbose` / `-v` flag (can be repeated: -v, -vv, -vvv)
- **FR-034**: System MUST support quiet mode via `--quiet` / `-q` flag (suppress non-error output)
- **FR-035**: System MUST show progress indication for batch operations (progress bar or percentage)
- **FR-036**: System MUST provide clear, actionable error messages for all failure scenarios
- **FR-037**: System MUST exit with code 0 on success, code 1 on error

#### Error Handling
- **FR-038**: System MUST continue processing remaining files after encountering an error by default
- **FR-039**: System MUST support stopping on first error via `--abort-on-error` flag
- **FR-040**: System MUST log all errors with file path and error description
- **FR-041**: System MUST validate flag combinations and show helpful error messages for invalid combinations

#### Performance & Reliability
- **FR-042**: System MUST handle large batch operations (100+ files) without excessive memory usage
- **FR-043**: System MUST process files sequentially to avoid resource exhaustion
- **FR-044**: System MUST complete simple single-file conversions in under 5 seconds (for PDFs <10 pages)

### Key Entities

- **CLI Arguments**: User-provided command-line options and flags controlling conversion behavior (input path, output formats, feature toggles, verbosity level)

- **Conversion Job**: A unit of work representing one document to be converted (input file path, detected format, output formats requested, configuration options)

- **Output File**: Generated file containing converted document content (file path, format, content derived from DoclingDocument)

- **Progress Tracker**: Status information shown to user during batch operations (files processed, files remaining, current file being processed, errors encountered)

- **Error Report**: Information about failed conversions (file path, error type, error message, timestamp)

---

## Review & Acceptance Checklist

### Content Quality
- [x] No implementation details (languages, frameworks, APIs)
- [x] Focused on user value and business needs
- [x] Written for non-technical stakeholders
- [x] All mandatory sections completed

### Requirement Completeness
- [x] No [NEEDS CLARIFICATION] markers remain
- [x] Requirements are testable and unambiguous
- [x] Success criteria are measurable
- [x] Scope is clearly bounded (CLI only, no GUI, no web interface)
- [x] Dependencies and assumptions identified (existing backends from Phases 1-3)

---

## Execution Status

- [x] User description parsed
- [x] Key concepts extracted
- [x] Ambiguities marked (none - user input was comprehensive)
- [x] User scenarios defined
- [x] Requirements generated
- [x] Entities identified
- [x] Review checklist passed

---
