# Data Model - Document Chunking System

**Date**: 2025-10-04
**Feature**: Phase 2 - Document Chunking

## Overview

The chunking system operates on `DoclingDocument` instances (from Phase 1) and produces collections of `BaseChunk` instances with associated metadata. The design uses trait-based abstraction to support multiple chunking strategies while maintaining a consistent API.

---

## Core Entities

### 1. BaseChunk

Represents a single chunk of document text with metadata.

**Fields**:
```rust
pub struct BaseChunk {
    /// The text content of this chunk
    pub text: String,

    /// Metadata describing this chunk's context and position
    pub meta: ChunkMetadata,
}
```

**Invariants**:
- `text` must not be empty for valid chunks
- `meta` must contain valid references (non-empty paths)

**State Transitions**: Immutable once created

**Serialization**:
```json
{
  "text": "International Business Machines Corporation...",
  "meta": {
    "doc_name": "ibm.md",
    "headings": ["IBM", "History"],
    "start_offset": 0,
    "end_offset": 245,
    "index": 0
  }
}
```

---

### 2. ChunkMetadata

Structured metadata attached to each chunk.

**Fields**:
```rust
pub struct ChunkMetadata {
    /// Source document name/identifier
    pub doc_name: String,

    /// Hierarchical path of headings (e.g., ["Chapter 1", "Section 1.1"])
    pub headings: Vec<String>,

    /// Optional caption (for tables, figures)
    pub caption: Option<String>,

    /// Character offset where chunk starts in original document
    pub start_offset: usize,

    /// Character offset where chunk ends in original document
    pub end_offset: usize,

    /// Sequential index of this chunk (0-based)
    pub index: usize,
}
```

**Validation Rules** (from FR-026 to FR-029):
- `doc_name` must not be empty
- `start_offset` < `end_offset`
- `index` must be sequential (enforced by chunker)
- `headings` path represents document structure (may be empty for flat documents)

---

### 3. BaseChunker (Trait)

Abstract interface for all chunking strategies.

**Methods**:
```rust
pub trait BaseChunker {
    /// Generate chunks from a document
    /// Returns lazy iterator to avoid loading all chunks into memory (NFR-001)
    fn chunk<'a>(&'a self, doc: &'a DoclingDocument)
        -> Box<dyn Iterator<Item = BaseChunk> + 'a>;

    /// Contextualize a chunk by prefixing with hierarchical metadata
    /// Used for embedding model input (FR-024, FR-030)
    fn contextualize(&self, chunk: &BaseChunk) -> String;
}
```

**Contract** (from FR-023 to FR-025):
- `chunk()` must return chunks in document order
- Iterator must be lazy (no upfront allocation)
- `contextualize()` must be deterministic (same chunk → same output)

---

### 4. HierarchicalChunker

Creates chunks based on document structure elements.

**Fields**:
```rust
pub struct HierarchicalChunker {
    /// Whether to merge list items into single chunks (default: true)
    pub merge_list_items: bool,
}
```

**Behavior** (from FR-005 to FR-009):
- One chunk per document element (paragraph, section, table, etc.)
- When `merge_list_items = true`: all list items → single chunk
- When `merge_list_items = false`: each list item → separate chunk
- Attaches all relevant metadata (headings, captions)

**State Transitions**: Immutable configuration

---

### 5. HybridChunker

Advanced chunker combining structure + tokenization awareness.

**Fields**:
```rust
pub struct HybridChunker {
    /// Tokenizer for token counting
    pub tokenizer: Box<dyn Tokenizer>,

    /// Maximum tokens per chunk (contextualized form)
    pub max_tokens: usize,

    /// Whether to merge undersized peer chunks (default: true)
    pub merge_peers: bool,

    /// Internal: hierarchical chunker for first pass
    hierarchical: HierarchicalChunker,
}
```

**Behavior** (from FR-010 to FR-017):
1. **Pass 1** (hierarchical): Get structure-based chunks
2. **Pass 2** (split): Split chunks exceeding `max_tokens`
3. **Pass 3** (merge): If `merge_peers = true`, merge consecutive chunks with same headings/captions that fit within `max_tokens`

**Invariants**:
- `max_tokens` > 0
- `tokenizer.max_tokens()` ≥ `max_tokens` (warning if not)

---

### 6. Tokenizer (Trait)

Abstract interface for token counting.

**Methods**:
```rust
pub trait Tokenizer {
    /// Count tokens in text according to this tokenizer's algorithm
    fn count_tokens(&self, text: &str) -> usize;

    /// Maximum tokens supported by this tokenizer's model
    fn max_tokens(&self) -> usize;
}
```

**Implementations**:
- `HuggingFaceTokenizer`: Wraps `tokenizers::Tokenizer`
- `OpenAITokenizer`: Future implementation using `tiktoken-rs`

---

### 7. HuggingFaceTokenizer

Wrapper around HuggingFace `tokenizers` crate.

**Fields**:
```rust
pub struct HuggingFaceTokenizer {
    /// Underlying tokenizer from tokenizers crate
    tokenizer: tokenizers::Tokenizer,

    /// Model's maximum token limit
    max_tokens: usize,
}
```

**Constructors** (from FR-022):
```rust
impl HuggingFaceTokenizer {
    /// Load tokenizer from HuggingFace Hub (e.g., "sentence-transformers/all-MiniLM-L6-v2")
    pub fn from_pretrained(model_id: &str) -> Result<Self, ChunkingError>;

    /// Create from existing tokenizer instance
    pub fn new(tokenizer: tokenizers::Tokenizer, max_tokens: usize) -> Self;

    /// Default tokenizer (sentence-transformers/all-MiniLM-L6-v2)
    pub fn default() -> Result<Self, ChunkingError>;
}
```

---

## Relationships

```
DoclingDocument (Phase 1)
    ↓
BaseChunker (trait)
    ├── HierarchicalChunker
    │       ↓
    │   BaseChunk
    │
    └── HybridChunker
            ├── uses → Tokenizer (trait)
            │           ├── HuggingFaceTokenizer
            │           └── OpenAITokenizer (future)
            ├── uses → HierarchicalChunker
            └── produces → BaseChunk

BaseChunk
    └── contains → ChunkMetadata
```

---

## Error Handling

**Error Type**:
```rust
#[derive(Debug, thiserror::Error)]
pub enum ChunkingError {
    #[error("Failed to load tokenizer: {0}")]
    TokenizerLoad(String),

    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    #[error("Document processing error: {0}")]
    ProcessingError(String),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}
```

**Error Scenarios**:
- Tokenizer not found / network error → `TokenizerLoad`
- `max_tokens` = 0 → `InvalidConfig`
- Empty document → Return empty iterator (not error)

---

## Validation Rules Summary

| Entity | Rule | Source Requirement |
|--------|------|-------------------|
| BaseChunk | text not empty | FR-026 |
| ChunkMetadata | doc_name not empty | FR-028 |
| ChunkMetadata | start_offset < end_offset | FR-017 |
| ChunkMetadata | index sequential | FR-018 |
| HybridChunker | max_tokens > 0 | FR-012 |
| HierarchicalChunker | merge_list_items boolean | FR-006 |

---

## Serialization Format

All types implement `serde::Serialize` and `serde::Deserialize` (FR-035 to FR-037).

**Example JSON** (full chunk):
```json
{
  "text": "IBM originated with several technological innovations...",
  "meta": {
    "doc_name": "ibm.md",
    "headings": ["IBM", "History", "1910s–1950s"],
    "caption": null,
    "start_offset": 512,
    "end_offset": 845,
    "index": 1
  }
}
```

**Contextualized Output** (from `contextualize()`):
```
IBM
History
1910s–1950s
IBM originated with several technological innovations...
```

---

## Next Steps

1. Implement data structures in `src/chunking/`
2. Write contract tests for `BaseChunker` trait
3. Implement `HierarchicalChunker` with tests
4. Implement `Tokenizer` trait and `HuggingFaceTokenizer`
5. Implement `HybridChunker` with tests

**Data Model Complete**: Ready for contract generation
