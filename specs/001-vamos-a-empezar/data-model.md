# Phase 1: Data Model Design

**Feature**: Core Document Processing Library (MVP Phase 1)
**Date**: 2025-10-04

## Overview

The data model provides a unified document representation (`DoclingDocument`) that all backends convert to. The design prioritizes:
- **Type safety**: Rust's type system enforces structural validity
- **Ser**ialization**: Serde for JSON export
- **Extensibility**: Enum-based node types allow future additions
- **Memory efficiency**: Avoid deep copying, use references where appropriate

---

## Core Types

### 1. DoclingDocument

**Purpose**: Unified representation of any document, regardless of source format

```rust
/// Unified document representation produced by all backends
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DoclingDocument {
    /// Document metadata (origin, format, etc.)
    pub metadata: DocumentMetadata,

    /// Root node of the document hierarchy
    pub root: NodeItem,
}

impl DoclingDocument {
    /// Export document to JSON string
    pub fn to_json(&self) -> Result<String, SerializationError>;

    /// Export document to Markdown string
    pub fn to_markdown(&self) -> String;

    /// Get all text content (flattened)
    pub fn get_text(&self) -> String;

    /// Iterate over all nodes in document order
    pub fn iter_nodes(&self) -> NodeIterator;
}
```

**Relationships**:
- Contains 1 `DocumentMetadata`
- Contains 1 root `NodeItem` (which may have children)

**Invariants**:
- `root` node type should be `NodeType::Document`
- Metadata `format` must match backend that created it

---

### 2. DocumentMetadata

**Purpose**: Store document origin and processing information

```rust
/// Document metadata and origin information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentMetadata {
    /// Original file path or identifier
    pub origin: String,

    /// Source format (Markdown, HTML, CSV, DOCX)
    pub format: InputFormat,

    /// Page count (optional, for paginated formats)
    pub page_count: Option<usize>,

    /// When the document was converted
    pub conversion_time: chrono::DateTime<chrono::Utc>,

    /// Additional format-specific metadata
    pub extra: HashMap<String, serde_json::Value>,
}
```

**Design Notes**:
- `origin`: File path or "stream" for byte streams
- `page_count`: None for Markdown/HTML, Some for future PDF support
- `extra`: Extensibility for format-specific data (e.g., DOCX styles, HTML charset)

---

### 3. InputFormat

**Purpose**: Enumerate supported document formats

```rust
/// Supported input document formats
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InputFormat {
    Markdown,
    Html,
    Csv,
    Docx,
    // Future: Pdf, Xlsx, Pptx, etc.
}

impl InputFormat {
    /// Get file extension for this format
    pub fn extension(&self) -> &'static str;

    /// Detect format from file extension
    pub fn from_extension(ext: &str) -> Option<Self>;

    /// Detect format from magic bytes
    pub fn from_bytes(bytes: &[u8]) -> Option<Self>;
}
```

---

### 4. NodeItem

**Purpose**: Represent hierarchical document structure (tree nodes)

```rust
/// Hierarchical document element (section, paragraph, list, table, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeItem {
    /// Type of this node (heading, paragraph, table, etc.)
    pub node_type: NodeType,

    /// Child nodes (for hierarchical structure)
    pub children: Vec<NodeItem>,

    /// Text content (if this is a text node)
    pub text: Option<TextItem>,

    /// Table content (if this is a table node)
    pub table: Option<TableData>,

    /// Node metadata (depth, position, etc.)
    pub metadata: NodeMetadata,
}

impl NodeItem {
    /// Create a new node
    pub fn new(node_type: NodeType) -> Self;

    /// Add a child node
    pub fn add_child(&mut self, child: NodeItem);

    /// Get all text content recursively
    pub fn get_text_recursive(&self) -> String;

    /// Check if node has children
    pub fn has_children(&self) -> bool;
}
```

**Invariants**:
- If `node_type` is `Text`, `text` should be `Some` and `table` should be `None`
- If `node_type` is `Table`, `table` should be `Some` and `text` should be `None`
- If `node_type` is container (Section, List), `children` may be non-empty

---

### 5. NodeType

**Purpose**: Classify node types in the document hierarchy

```rust
/// Type of document node
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeType {
    /// Root document node
    Document,

    /// Section/chapter (logical grouping)
    Section,

    /// Heading (H1-H6)
    Heading { level: u8 }, // 1-6

    /// Paragraph
    Paragraph,

    /// List (ordered or unordered)
    List { ordered: bool },

    /// List item
    ListItem,

    /// Table
    Table,

    /// Code block
    CodeBlock { language: Option<String> },

    /// Blockquote
    Blockquote,

    /// Horizontal rule / divider
    HorizontalRule,

    /// Generic text node
    Text,
}
```

**Design Notes**:
- Heading level embedded in enum (type-safe 1-6)
- List ordered flag distinguishes `<ol>` vs `<ul>`
- CodeBlock language for syntax highlighting hints
- Extensible for future types (Image, Video, etc.)

---

### 6. NodeMetadata

**Purpose**: Store node-level metadata (position, styling hints, etc.)

```rust
/// Metadata for a node
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NodeMetadata {
    /// Depth in hierarchy (0 = root)
    pub depth: usize,

    /// Index among siblings (0-based)
    pub index: usize,

    /// Source line/character position (if available)
    pub source_position: Option<SourcePosition>,

    /// Extra metadata (format-specific)
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourcePosition {
    pub line: usize,
    pub column: usize,
}
```

---

### 7. TextItem

**Purpose**: Represent text content with optional formatting

```rust
/// Text content with formatting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextItem {
    /// Text content
    pub content: String,

    /// Formatting (optional)
    pub formatting: Option<Formatting>,
}

/// Text formatting
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Formatting {
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub strikethrough: bool,
    pub code: bool, // inline code
    pub link: Option<String>, // href for links
}
```

**Design Notes**:
- Simple boolean flags for common formatting
- `link` for hyperlinks (href)
- Extensible via additional fields

---

### 8. TableData

**Purpose**: Represent tabular data (CSV, HTML tables, DOCX tables)

```rust
/// Tabular data representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableData {
    /// Column headers (if present)
    pub headers: Vec<String>,

    /// Table rows (each row is a vector of cells)
    pub rows: Vec<Vec<TableCell>>,

    /// Table metadata
    pub metadata: TableMetadata,
}

/// Individual table cell
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableCell {
    /// Cell content
    pub content: String,

    /// Cell formatting (optional)
    pub formatting: Option<Formatting>,

    /// Colspan (default 1)
    pub colspan: usize,

    /// Rowspan (default 1)
    pub rowspan: usize,
}

/// Table-level metadata
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TableMetadata {
    /// Whether first row is header
    pub has_header: bool,

    /// Caption/title (if present)
    pub caption: Option<String>,
}
```

**Design Notes**:
- CSV: All cells plain text, no formatting
- HTML: Supports colspan/rowspan, formatting
- DOCX: Supports formatting, merged cells

---

### 9. InputDocument

**Purpose**: Represent source document before conversion

```rust
/// Input document to be converted
pub struct InputDocument {
    /// File path or identifier
    pub source: DocumentSource,

    /// Detected format
    pub format: InputFormat,

    /// Backend instance (trait object)
    backend: Box<dyn Backend>,
}

pub enum DocumentSource {
    FilePath(PathBuf),
    ByteStream(Vec<u8>, String), // bytes + name
}

impl InputDocument {
    /// Create from file path
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, ConversionError>;

    /// Create from byte stream
    pub fn from_bytes(bytes: Vec<u8>, name: String, format: InputFormat) -> Result<Self, ConversionError>;
}
```

**Design Notes**:
- `backend` is trait object for dynamic dispatch
- `DocumentSource` enum for file vs bytes

---

### 10. ConversionResult

**Purpose**: Wrap conversion output with status and errors

```rust
/// Result of document conversion
#[derive(Debug)]
pub struct ConversionResult {
    /// Input document
    pub input: InputDocument,

    /// Output document (None if conversion failed)
    pub document: Option<DoclingDocument>,

    /// Conversion status
    pub status: ConversionStatus,

    /// Errors encountered
    pub errors: Vec<ConversionError>,

    /// Warnings (non-fatal)
    pub warnings: Vec<String>,

    /// Performance metrics
    pub metrics: ConversionMetrics,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConversionStatus {
    Success,
    Partial, // Conversion succeeded with warnings
    Failure,
}

#[derive(Debug, Default)]
pub struct ConversionMetrics {
    pub duration_ms: u64,
    pub input_size_bytes: usize,
    pub node_count: usize,
}
```

---

## Error Types

### ConversionError

```rust
/// Errors during document conversion
#[derive(Debug, thiserror::Error)]
pub enum ConversionError {
    #[error("File not found: {0}")]
    FileNotFound(PathBuf),

    #[error("Unsupported format: {0:?}")]
    UnsupportedFormat(String),

    #[error("Invalid file: {0}")]
    InvalidFile(String),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}
```

**Design Notes**:
- Use `thiserror` for error derive macros
- Specific error types for common failures
- Chain I/O and serialization errors

---

## Type Relationships (UML-style)

```
DoclingDocument
├── metadata: DocumentMetadata
│   ├── origin: String
│   ├── format: InputFormat
│   └── extra: HashMap
└── root: NodeItem
    ├── node_type: NodeType
    ├── children: Vec<NodeItem> (recursive)
    ├── text: Option<TextItem>
    │   ├── content: String
    │   └── formatting: Option<Formatting>
    ├── table: Option<TableData>
    │   ├── headers: Vec<String>
    │   └── rows: Vec<Vec<TableCell>>
    └── metadata: NodeMetadata

ConversionResult
├── input: InputDocument
├── document: Option<DoclingDocument>
├── status: ConversionStatus
├── errors: Vec<ConversionError>
└── warnings: Vec<String>
```

---

## Serialization Example

**JSON Output Structure**:
```json
{
  "metadata": {
    "origin": "/path/to/doc.md",
    "format": "Markdown",
    "page_count": null,
    "conversion_time": "2025-10-04T12:00:00Z",
    "extra": {}
  },
  "root": {
    "node_type": "Document",
    "children": [
      {
        "node_type": {"Heading": {"level": 1}},
        "text": {"content": "Title", "formatting": null},
        "metadata": {"depth": 1, "index": 0}
      },
      {
        "node_type": "Paragraph",
        "text": {"content": "Some text", "formatting": null},
        "metadata": {"depth": 1, "index": 1}
      }
    ],
    "metadata": {"depth": 0, "index": 0}
  }
}
```

---

## Design Decisions

| Decision | Rationale |
|----------|-----------|
| Serde for serialization | De facto standard, derive macros reduce boilerplate |
| Enum for NodeType | Type-safe, exhaustive matching |
| Option<TextItem> + Option<TableData> | Clear which nodes contain what (vs enum) |
| HashMap for extra metadata | Forward compatibility without breaking changes |
| thiserror for errors | Clean error message generation |
| chrono for timestamps | Standard datetime library |

---

## Testing Strategy

1. **Unit tests**: Each type's methods (to_json, get_text, etc.)
2. **Property tests**: Serialization round-trip (serde -> JSON -> serde)
3. **Edge cases**: Empty documents, deeply nested structures
4. **Integration**: Backend-produced documents validate correctly

---

**Data Model Status**: ✅ COMPLETE
**Ready for Contracts**: Yes
