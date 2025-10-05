# Feature Specification: Basic PDF Text Extraction (Phase 3)

**Feature Branch**: `003-phase-3-pdf`
**Created**: 2025-10-04
**Status**: Draft
**Input**: User description: "Phase 3: PDF Básico - Backend PDF con pdfium-render - Extracción de texto simple (sin ML)"

## User Scenarios & Testing

### Primary User Story
As a Rust application developer, I need to extract text from PDF files in reading order so that I can process PDF documents alongside other formats without relying on machine learning models or Python dependencies.

### Acceptance Scenarios

1. **Given** a simple text-based PDF file, **When** the library converts it to a DoclingDocument, **Then** the document contains all extractable text in reading order

2. **Given** a multi-page PDF document, **When** the library processes it, **Then** each page is represented in the DoclingDocument with correct page numbers and text content

3. **Given** a PDF with mixed text and images, **When** the library extracts text, **Then** text content is extracted and image regions are noted in metadata (without OCR)

4. **Given** a PDF with embedded fonts, **When** the library extracts text, **Then** text is decoded correctly using the embedded font information

5. **Given** a PDF with simple table-like text arrangement, **When** the library extracts text, **Then** text is extracted in logical reading order (best effort without layout analysis)

6. **Given** a password-protected PDF with user password, **When** the library is provided the correct password, **Then** the PDF is unlocked and text is extracted successfully

7. **Given** a scanned PDF (image-only, no text layer), **When** the library attempts to extract text, **Then** the system returns an indication of no extractable text without errors

### Edge Cases

- What happens when a PDF is corrupted or invalid?
  - System MUST return a descriptive error indicating the PDF cannot be parsed

- What happens when a PDF page has no text (blank page)?
  - System MUST include the page in the document with empty text content

- What happens when a PDF uses non-standard or custom fonts?
  - System MUST make best effort to extract text, falling back to glyph mapping if available

- What happens when a PDF has complex multi-column layout?
  - System MUST extract text in the order provided by PDF (may not match visual reading order without layout analysis)

- What happens when a PDF contains forms with fillable fields?
  - System MUST extract static text and field labels (field values extraction is optional)

- What happens when a PDF is encrypted without permissions?
  - System MUST return an error indicating decryption is required

## Requirements

### Functional Requirements

#### PDF Backend Infrastructure
- **FR-001**: Library MUST provide a `PdfBackend` implementing the Backend trait from Phase 1
- **FR-002**: PdfBackend MUST use `pdfium-render` crate for PDF parsing and rendering
- **FR-003**: PdfBackend MUST support loading PDFs from file paths
- **FR-004**: PdfBackend MUST support loading PDFs from byte streams
- **FR-005**: PdfBackend MUST validate PDF structure before processing
- **FR-006**: PdfBackend MUST report PDF version and metadata (title, author, page count)

#### Page-Level Processing
- **FR-007**: PdfBackend MUST process PDF documents page-by-page
- **FR-008**: Each page MUST be represented as a distinct section in DoclingDocument
- **FR-009**: PdfBackend MUST extract page dimensions (width, height) in points
- **FR-010**: PdfBackend MUST preserve page numbers starting from 1
- **FR-011**: PdfBackend MUST support selective page range processing (e.g., pages 1-10)

#### Text Extraction
- **FR-012**: PdfBackend MUST extract text content from each page using pdfium text extraction API
- **FR-013**: Text extraction MUST preserve word boundaries and spaces
- **FR-014**: Text extraction MUST handle multi-byte characters and Unicode correctly
- **FR-015**: Text extraction MUST decode text using embedded font information when available
- **FR-016**: Extracted text MUST be returned in the order provided by the PDF structure
- **FR-017**: Text extraction MUST handle rotated text (0°, 90°, 180°, 270°)

#### Metadata Extraction
- **FR-018**: PdfBackend MUST extract document metadata (title, author, subject, keywords, creator, producer)
- **FR-019**: PdfBackend MUST extract creation and modification dates if available
- **FR-020**: PdfBackend MUST detect if PDF contains only scanned images (no text layer)
- **FR-021**: Metadata MUST be included in DoclingDocument origin information

#### Image Detection
- **FR-022**: PdfBackend MUST detect presence of images on each page
- **FR-023**: Image metadata (count, bounding boxes) MUST be recorded without extracting image data
- **FR-024**: System MUST distinguish between text and image regions
- **FR-025**: OCR is explicitly NOT performed in Phase 3 (deferred to future phases)

#### Encryption & Security
- **FR-026**: PdfBackend MUST detect if a PDF is encrypted
- **FR-027**: PdfBackend MUST support opening password-protected PDFs with user password
- **FR-028**: PdfBackend MUST respect PDF permissions (if extraction is disabled, return error)
- **FR-029**: Encrypted PDFs without correct password MUST return a specific error type

#### Error Handling
- **FR-030**: PdfBackend MUST handle corrupted PDFs gracefully with descriptive errors
- **FR-031**: PdfBackend MUST handle PDFs with missing or damaged fonts
- **FR-032**: PdfBackend MUST return Result types for all PDF operations
- **FR-033**: System MUST NOT panic when encountering malformed PDFs

#### Integration with Phase 1
- **FR-034**: PdfBackend MUST produce DoclingDocument compatible with Phase 1 data models
- **FR-035**: PdfBackend MUST integrate with DocumentConverter for automatic format detection
- **FR-036**: PDF documents MUST support all Phase 1 export formats (JSON, Markdown)

#### Cross-Platform Support
- **FR-037**: PdfBackend MUST work on both Windows and macOS
- **FR-038**: pdfium-render binaries MUST be available for target platforms
- **FR-039**: File paths MUST use platform-agnostic APIs

### Non-Functional Requirements

#### Performance
- **NFR-001**: Small PDFs (<10 pages, <1MB) MUST process in under 1 second
- **NFR-002**: Text extraction MUST not load entire PDF into memory
- **NFR-003**: Page-by-page processing MUST support lazy iteration for large PDFs
- **NFR-004**: Memory usage MUST scale linearly with page count (not document size)

#### Quality
- **NFR-005**: Text extraction accuracy MUST be >95% for standard PDFs with embedded fonts
- **NFR-006**: Reading order MUST be correct for simple single-column text
- **NFR-007**: Multi-column and complex layouts are best-effort (no guarantee of correct reading order without ML)

#### API Design
- **NFR-008**: PDF-specific configuration MUST follow Phase 1 backend configuration patterns
- **NFR-009**: API MUST allow password specification for encrypted PDFs
- **NFR-010**: API MUST allow page range specification for selective processing
- **NFR-011**: All public APIs MUST have rustdoc documentation with examples

#### Dependencies
- **NFR-012**: MUST use `pdfium-render` crate (native Rust binding to Pdfium)
- **NFR-013**: pdfium-render MUST NOT introduce Python dependencies
- **NFR-014**: PDF processing MUST NOT require external services or ML models

#### Testing
- **NFR-015**: Tests MUST include various PDF types (simple text, multi-page, encrypted, scanned)
- **NFR-016**: Tests MUST verify text extraction accuracy on sample PDFs
- **NFR-017**: Tests MUST verify handling of corrupted/malformed PDFs
- **NFR-018**: Integration tests MUST verify PDF documents work with Phase 2 chunking
- **NFR-019**: Tests MUST run on both Windows and macOS

#### Limitations (Explicit Non-Goals for Phase 3)
- **NFR-020**: Layout analysis is NOT included (no advanced reading order detection)
- **NFR-021**: OCR is NOT included (scanned PDFs return no text)
- **NFR-022**: Table structure detection is NOT included (tables extracted as plain text)
- **NFR-023**: Form field value extraction is optional (not required)
- **NFR-024**: Image extraction/processing is NOT included (metadata only)

### Key Entities

- **PdfBackend**: Backend implementation for PDF format using pdfium-render. Handles PDF loading, validation, text extraction, and conversion to DoclingDocument.

- **PdfPage**: Represents a single page in a PDF with text content, dimensions, page number, and image metadata.

- **PdfDocument**: Internal representation of loaded PDF with metadata, page count, encryption status, and page iterator.

- **PdfTextExtractor**: Component responsible for extracting text from PDF pages using pdfium API, handling Unicode, fonts, and reading order.

- **PdfMetadata**: Contains PDF document metadata including title, author, dates, version, encryption status, and page count.

- **EncryptionInfo**: Information about PDF encryption status, required passwords, and permissions.

- **PdfConfig**: Configuration for PDF processing including password, page range, and extraction options.

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
- [x] Scope is clearly bounded
- [x] Dependencies and assumptions identified

### Assumptions
- Phase 1 (Core + Simple Formats) is completed and available
- pdfium-render crate provides sufficient text extraction capabilities
- Basic text extraction without ML is valuable for many use cases
- Advanced PDF features (layout analysis, OCR, tables) are deferred to Phase 4
- Reading order for complex layouts is best-effort only

### Explicit Limitations
- NO layout analysis or advanced reading order detection
- NO OCR for scanned PDFs (returns empty text)
- NO table structure recognition (tables as plain text)
- NO form field value extraction (optional if trivial)
- NO image extraction (metadata only)

### Dependencies
- Requires Phase 1 Backend trait and DoclingDocument
- Requires `pdfium-render` crate with platform-specific binaries
- Must comply with constitution principle VII (Native Rust Dependencies)
- pdfium-render must support Windows and macOS

## Execution Status

- [x] User description parsed
- [x] Key concepts extracted
- [x] Ambiguities marked
- [x] User scenarios defined
- [x] Requirements generated
- [x] Entities identified
- [x] Review checklist passed
