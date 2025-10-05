# Data Model: Advanced PDF Processing

**Feature**: 003-phase-3-pdf
**Date**: 2025-10-05

## Overview

Data model for advanced PDF processing supporting text extraction, layout analysis, table detection, image extraction, OCR, and content enrichment.

## Core Entities

### 1. PdfDocument

**Purpose**: Represents a loaded PDF document with all pages and metadata.

**Fields**:
```rust
pub struct PdfDocument {
    pub pages: Vec<PdfPage>,
    pub metadata: PdfMetadata,
    pub encryption_info: Option<EncryptionInfo>,
    pub page_count: usize,
}
```

**Validation Rules**:
- `page_count` MUST be > 0
- `pages.len()` MUST equal `page_count`
- `metadata` MUST be present (even if empty)

**Methods**:
- `load_from_file(path: &Path, config: Option<PdfConfig>) -> Result<Self>`
- `load_from_bytes(bytes: &[u8], config: Option<PdfConfig>) -> Result<Self>`
- `get_page(&self, index: usize) -> Option<&PdfPage>`
- `iter_pages(&self) -> impl Iterator<Item = &PdfPage>`

### 2. PdfPage

**Purpose**: Represents a single page with all extracted elements.

**Fields**:
```rust
pub struct PdfPage {
    pub page_number: usize,          // 1-indexed
    pub dimensions: PageDimensions,
    pub rotation: Rotation,          // 0, 90, 180, 270 degrees
    pub text_blocks: Vec<TextBlock>,
    pub tables: Vec<Table>,
    pub images: Vec<ImageRegion>,
    pub layout_info: Option<LayoutInfo>,
    pub ocr_result: Option<OcrResult>,
}
```

**State Transitions**:
1. **Raw**: Just loaded from PDF
2. **Analyzed**: Layout analysis complete
3. **Enriched**: Tables, images, OCR, content enrichment complete

**Relationships**:
- Belongs to one `PdfDocument`
- Contains multiple `TextBlock`, `Table`, `ImageRegion`

### 3. PdfElement (Enum)

**Purpose**: Unified type for all PDF content elements.

**Definition**:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PdfElement {
    TextBlock(TextBlock),
    Table(Table),
    Image(ImageRegion),
    Formula(Formula),
    CodeBlock(CodeBlock),
    List(ListStructure),
}

impl PdfElement {
    pub fn bounding_box(&self) -> &BoundingBox;
    pub fn confidence_score(&self) -> Option<f32>;
    pub fn element_type(&self) -> ElementType;
}
```

**Common Traits**:
- Serializable via `serde`
- Convertible to `DoclingDocument` nodes

### 4. TextBlock

**Purpose**: A block of text with position and formatting information.

**Fields**:
```rust
pub struct TextBlock {
    pub text: String,
    pub bbox: BoundingBox,
    pub font_info: FontInfo,
    pub reading_order: usize,
    pub column_id: Option<usize>,
    pub block_type: TextBlockType,  // Heading, Paragraph, ListItem, Caption
    pub confidence: Option<f32>,
}

pub struct FontInfo {
    pub family: String,
    pub size: f32,
    pub weight: FontWeight,  // Normal, Bold
    pub style: FontStyle,    // Normal, Italic
}

pub enum TextBlockType {
    Heading(u8),  // Level 1-6
    Paragraph,
    ListItem,
    Caption,
    Footer,
    Header,
}
```

**Validation**:
- `text` MUST NOT be empty (empty blocks filtered out)
- `bbox` MUST have positive width and height
- `reading_order` MUST be unique within page

### 5. Table

**Purpose**: Structured table with cells and layout.

**Fields**:
```rust
pub struct Table {
    pub bbox: BoundingBox,
    pub cells: Vec<TableCell>,
    pub structure: TableStructure,
    pub header_rows: usize,
    pub confidence: Option<f32>,
}

pub struct TableCell {
    pub row: usize,
    pub col: usize,
    pub rowspan: usize,  // Default 1
    pub colspan: usize,  // Default 1
    pub content: String,
    pub bbox: BoundingBox,
    pub is_header: bool,
}

pub struct TableStructure {
    pub rows: usize,
    pub cols: usize,
    pub merged_cells: Vec<(usize, usize, usize, usize)>,  // (row, col, rowspan, colspan)
}
```

**Validation**:
- All `cells` MUST fit within `structure` dimensions
- `rowspan` and `colspan` >= 1
- No overlapping cells

### 6. ImageRegion

**Purpose**: Image detected on page with metadata.

**Fields**:
```rust
pub struct ImageRegion {
    pub bbox: BoundingBox,
    pub image_type: ImageType,
    pub bitmap: Option<Vec<u8>>,  // Optional: actual image data
    pub metadata: ImageMetadata,
}

pub enum ImageType {
    Photo,
    Diagram,
    Logo,
    Chart,
    Unknown,
}

pub struct ImageMetadata {
    pub width: u32,
    pub height: u32,
    pub format: ImageFormat,  // PNG, JPEG, etc.
    pub dpi: Option<u32>,
}
```

**Validation**:
- `bbox` MUST have positive dimensions
- `bitmap` size MUST match `width * height * bytes_per_pixel` (if present)

### 7. LayoutInfo

**Purpose**: Document layout structure and reading order.

**Fields**:
```rust
pub struct LayoutInfo {
    pub columns: Vec<Column>,
    pub reading_order: Vec<ElementId>,
    pub confidence: f32,
}

pub struct Column {
    pub bbox: BoundingBox,
    pub element_ids: Vec<ElementId>,
}

pub type ElementId = String;  // Unique identifier for elements
```

**Invariants**:
- `reading_order` MUST include all text blocks on page
- `columns` MUST NOT overlap
- `confidence` in range [0.0, 1.0]

### 8. OcrResult

**Purpose**: OCR output with confidence scores.

**Fields**:
```rust
pub struct OcrResult {
    pub text: String,
    pub confidence: f32,       // Overall confidence
    pub words: Vec<OcrWord>,
    pub language: String,      // ISO 639-1 code (e.g., "en", "es")
}

pub struct OcrWord {
    pub text: String,
    pub bbox: BoundingBox,
    pub confidence: f32,
}
```

**Validation**:
- `confidence` in range [0.0, 1.0]
- `words` confidences also in [0.0, 1.0]

### 9. PdfMetadata

**Purpose**: PDF document metadata.

**Fields**:
```rust
pub struct PdfMetadata {
    pub title: Option<String>,
    pub author: Option<String>,
    pub subject: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub creator: Option<String>,      // Creating application
    pub producer: Option<String>,     // PDF producer
    pub creation_date: Option<DateTime<Utc>>,
    pub modification_date: Option<DateTime<Utc>>,
    pub page_count: usize,
    pub pdf_version: String,          // e.g., "1.7"
}
```

### 10. PdfConfig

**Purpose**: Configuration for PDF processing pipeline.

**Fields**:
```rust
pub struct PdfConfig {
    pub password: Option<String>,
    pub page_range: Option<Range<usize>>,  // Process subset of pages
    pub enable_ocr: bool,
    pub enable_tables: bool,
    pub enable_images: bool,
    pub enable_enrichment: bool,           // Code blocks, formulas
    pub ocr_language: String,              // Default "eng"
    pub layout_analyzer: LayoutAnalyzerType,
    pub table_detector: TableDetectorType,
}

pub enum LayoutAnalyzerType {
    RuleBased,
    MlModel(String),  // Path to ONNX model
}

pub enum TableDetectorType {
    GridBased,
    MlModel(String),  // Path to ONNX model
}
```

**Defaults**:
```rust
impl Default for PdfConfig {
    fn default() -> Self {
        Self {
            password: None,
            page_range: None,
            enable_ocr: false,
            enable_tables: true,
            enable_images: true,
            enable_enrichment: true,
            ocr_language: "eng".to_string(),
            layout_analyzer: LayoutAnalyzerType::RuleBased,
            table_detector: TableDetectorType::GridBased,
        }
    }
}
```

## Supporting Types

### BoundingBox
```rust
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BoundingBox {
    pub x: f32,      // Left edge
    pub y: f32,      // Top edge
    pub width: f32,
    pub height: f32,
}

impl BoundingBox {
    pub fn intersects(&self, other: &BoundingBox) -> bool;
    pub fn contains(&self, point: (f32, f32)) -> bool;
    pub fn area(&self) -> f32;
}
```

### PageDimensions
```rust
#[derive(Debug, Clone, Copy)]
pub struct PageDimensions {
    pub width: f32,   // In points (1/72 inch)
    pub height: f32,  // In points
}
```

### Rotation
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rotation {
    None,
    Clockwise90,
    Clockwise180,
    Clockwise270,
}
```

### EncryptionInfo
```rust
pub struct EncryptionInfo {
    pub is_encrypted: bool,
    pub has_user_password: bool,
    pub has_owner_password: bool,
    pub permissions: PdfPermissions,
}

pub struct PdfPermissions {
    pub can_print: bool,
    pub can_copy: bool,
    pub can_modify: bool,
    pub can_extract_content: bool,
}
```

## Content Enrichment Types

### Formula
```rust
pub struct Formula {
    pub bbox: BoundingBox,
    pub latex: Option<String>,  // If detected/converted
    pub image: Vec<u8>,         // Rendered formula image
    pub confidence: f32,
}
```

### CodeBlock
```rust
pub struct CodeBlock {
    pub bbox: BoundingBox,
    pub code: String,
    pub language: Option<String>,  // Detected language
    pub confidence: f32,
}
```

### ListStructure
```rust
pub struct ListStructure {
    pub list_type: ListType,
    pub items: Vec<ListItem>,
    pub bbox: BoundingBox,
}

pub enum ListType {
    Ordered,
    Unordered,
}

pub struct ListItem {
    pub text: String,
    pub bbox: BoundingBox,
    pub level: usize,  // Nesting level
}
```

## Mapping to DoclingDocument

All PDF elements map to `DoclingDocument` nodes:

```rust
impl From<PdfDocument> for DoclingDocument {
    fn from(pdf: PdfDocument) -> Self {
        // Map pages to document sections
        // Map elements to nodes with positions
        // Preserve hierarchy and reading order
    }
}
```

**Mapping Rules**:
- `TextBlock` → `DocumentNode` with `NodeType::Paragraph` or `NodeType::Heading`
- `Table` → `TableNode` with cell structure
- `ImageRegion` → `DocumentNode` with image metadata
- `Formula` → `DocumentNode` with special formula type
- `CodeBlock` → `DocumentNode` with code type
- Layout/reading order → Preserve in node ordering

## Serialization

All types implement `serde::Serialize` and `serde::Deserialize` for:
- JSON export (Phase 1 compatibility)
- Intermediate storage
- Debugging/inspection

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfDocument { /* ... */ }
```

## Error Handling

```rust
#[derive(Debug, thiserror::Error)]
pub enum PdfError {
    #[error("Failed to load PDF: {0}")]
    LoadError(String),

    #[error("PDF is encrypted and requires password")]
    EncryptedError,

    #[error("Invalid page number: {0}")]
    InvalidPage(usize),

    #[error("OCR failed: {0}")]
    OcrError(String),

    #[error("Layout analysis failed: {0}")]
    LayoutError(String),

    #[error("Table detection failed: {0}")]
    TableError(String),
}
```

## Summary

The data model provides:
- ✅ Complete representation of PDF content
- ✅ Support for all processing stages (text → layout → tables → OCR → enrichment)
- ✅ Flexible configuration
- ✅ Clear mapping to DoclingDocument
- ✅ Serialization support
- ✅ Type safety with Rust enums and validation
