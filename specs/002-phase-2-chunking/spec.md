# Feature Specification: Document Chunking System

**Feature Branch**: `002-phase-2-chunking`
**Created**: 2025-10-04
**Status**: Draft
**Input**: User description: "Phase 2: Chunking - Add intelligent document chunking with configurable strategies (character, word, sentence, paragraph) and overlap settings. Must maintain semantic coherence and preserve metadata."

## Execution Flow (main)
```
1. Parse user description from Input
   ✓ Feature: Document chunking with hierarchical and hybrid strategies
2. Extract key concepts from description
   ✓ Actions: chunk documents, configure tokenizers, maintain coherence
   ✓ Data: chunks, metadata, hierarchical structure
   ✓ Constraints: semantic coherence, metadata preservation
3. For each unclear aspect:
   → Marked with [NEEDS CLARIFICATION: specific question]
4. Fill User Scenarios & Testing section
   ✓ User flow: convert document → configure chunking → get chunks with metadata
5. Generate Functional Requirements
   ✓ Each requirement must be testable
6. Identify Key Entities
   ✓ Entities: Chunk, Chunker, ChunkingConfig, Tokenizer
7. Run Review Checklist
   → Validate no implementation details
8. Return: SUCCESS (spec ready for planning)
```

---

## ⚡ Quick Guidelines
- ✅ Focus on WHAT users need and WHY
- ❌ Avoid HOW to implement (no tech stack, APIs, code structure)
- 👥 Written for business stakeholders, not developers

---

## Clarifications

### Session 2025-10-04

- Q: ¿Cuál es el objetivo de rendimiento aceptable para operaciones de chunking? → A: Balanced - <100ms para documentos típicos (~10 páginas, ~100 chunks)

---

## User Scenarios & Testing *(mandatory)*

### Primary User Story
As a library user processing documents for RAG (Retrieval Augmented Generation) applications, I need to break down DoclingDocuments into semantically coherent chunks that fit within embedding model token limits. The chunking must preserve document structure and metadata (headings, captions, hierarchical context) so that each chunk can be properly contextualized when embedded or retrieved.

### Acceptance Scenarios

1. **Given** a converted DoclingDocument with hierarchical structure, **When** I apply hierarchical chunking, **Then** I receive chunks where each chunk corresponds to one document element (paragraph, section, etc.) with all relevant metadata (headings, captions) attached

2. **Given** a converted DoclingDocument and a tokenizer (e.g., for sentence-transformers/all-MiniLM-L6-v2), **When** I apply hybrid chunking with max_tokens=512, **Then** I receive chunks where the contextualized form (text + metadata) does not exceed 512 tokens

3. **Given** a hybrid chunker with merge_peers=True, **When** multiple small consecutive chunks share the same heading/caption metadata and can fit together within the token limit, **Then** those chunks are merged into a single chunk

4. **Given** a hybrid chunker with merge_peers=False, **When** processing a document, **Then** small consecutive chunks are NOT merged even if they share metadata

5. **Given** a document element larger than max_tokens, **When** hybrid chunking is applied, **Then** the element is split into multiple chunks respecting the token limit

6. **Given** a chunk from any chunker, **When** I call contextualize(), **Then** I receive the chunk text prefixed with relevant hierarchical metadata (headings, captions in order)

7. **Given** hierarchical chunking with merge_list_items=True (default), **When** processing a list, **Then** all list items are merged into a single chunk

8. **Given** hierarchical chunking with merge_list_items=False, **When** processing a list, **Then** each list item becomes a separate chunk

9. **Given** chunks from any chunker, **When** I serialize them to JSON, **Then** all chunk content and metadata are preserved and can be deserialized

### Edge Cases

- What happens when a single document element (e.g., paragraph) exceeds max_tokens?
  → Hybrid chunker splits it into multiple chunks respecting token boundaries

- What happens when chunking an empty DoclingDocument?
  → Return empty chunk iterator (no chunks)

- What happens when max_tokens is extremely small (e.g., 10)?
  → Chunks may be very small but chunker still respects the limit

- What happens when a tokenizer is not provided to HybridChunker?
  → Use default tokenizer (e.g., sentence-transformers/all-MiniLM-L6-v2)

- What happens when document has no hierarchical metadata (no headings)?
  → Chunks still created based on document elements, contextualize() returns chunk text without prefix

- What happens when tokenizer max_tokens differs from user-specified max_tokens?
  → User-specified max_tokens takes precedence

## Requirements *(mandatory)*

### Functional Requirements

#### Chunker Types
- **FR-001**: System MUST provide a HierarchicalChunker that creates chunks based on document structure
- **FR-002**: System MUST provide a HybridChunker that combines hierarchical chunking with tokenization-aware refinements
- **FR-003**: Both chunkers MUST implement a common BaseChunker interface
- **FR-004**: Users MUST be able to select which chunker to use

#### HierarchicalChunker Behavior
- **FR-005**: HierarchicalChunker MUST create one chunk per document element by default
- **FR-006**: HierarchicalChunker MUST support merge_list_items parameter (default: true) to control list item merging
- **FR-007**: When merge_list_items=true, all items in a list MUST be merged into a single chunk
- **FR-008**: When merge_list_items=false, each list item MUST become a separate chunk
- **FR-009**: HierarchicalChunker MUST attach all relevant metadata (headings, captions) to each chunk

#### HybridChunker Behavior
- **FR-010**: HybridChunker MUST start from hierarchical chunks and apply tokenization-aware refinements
- **FR-011**: HybridChunker MUST accept a tokenizer parameter aligned with the embedding model tokenizer
- **FR-012**: HybridChunker MUST accept a max_tokens parameter defining the token limit
- **FR-013**: HybridChunker MUST split oversized chunks (exceeding max_tokens in contextualized form)
- **FR-014**: HybridChunker MUST support merge_peers parameter (default: true)
- **FR-015**: When merge_peers=true, HybridChunker MUST merge consecutive undersized chunks with same headings/captions when they fit within max_tokens
- **FR-016**: When merge_peers=false, HybridChunker MUST NOT merge undersized chunks
- **FR-017**: When splitting oversized chunks, HybridChunker MUST respect token boundaries

#### Tokenizer Support
- **FR-018**: System MUST support HuggingFace tokenizers (e.g., transformers library)
- **FR-019**: System MUST support OpenAI tokenizers (tiktoken)
- **FR-020**: Tokenizer MUST provide a count_tokens() method to count tokens in text
- **FR-021**: Tokenizer MUST expose max_tokens configuration
- **FR-022**: HybridChunker MUST use default tokenizer when none is provided

#### BaseChunker Interface
- **FR-023**: All chunkers MUST provide a chunk() method that accepts a DoclingDocument and returns an iterator of chunks
- **FR-024**: All chunkers MUST provide a contextualize() method that enriches a chunk with metadata for embedding
- **FR-025**: The chunk() method MUST return chunks in document order (sequential)

#### Chunk Metadata
- **FR-026**: Each chunk MUST include the chunk text content
- **FR-027**: Each chunk MUST include hierarchical metadata (headings path, captions)
- **FR-028**: Each chunk MUST include a reference to its source document
- **FR-029**: Chunk metadata MUST be structured to enable contextualization
- **FR-030**: The contextualize() method MUST return chunk text prefixed with hierarchical metadata (headings, captions)

#### Integration
- **FR-031**: Chunkers MUST accept a DoclingDocument (from Phase 1) as input
- **FR-032**: Chunking MUST work with documents converted from all Phase 1 formats (Markdown, HTML, CSV, DOCX)
- **FR-033**: Chunking MUST be a separate operation from document conversion (composition, not modification of DoclingDocument)
- **FR-034**: Chunk iterator MUST be lazy (not load all chunks into memory at once)

#### Serialization
- **FR-035**: Chunks MUST be serializable to JSON
- **FR-036**: Chunks MUST be deserializable from JSON
- **FR-037**: Serialized chunks MUST preserve all metadata and content

### Non-Functional Requirements
- **NFR-001**: Chunking MUST be memory-efficient (lazy evaluation, iterator-based)
- **NFR-002**: Chunking MUST work cross-platform (Windows, macOS, Linux)
- **NFR-003**: Chunking MUST use only native Rust dependencies (no Python bindings)
- **NFR-004**: API MUST be ergonomic and follow Rust best practices (builder pattern for configuration)
- **NFR-005**: Tokenizer integration MUST support common embedding models (sentence-transformers, OpenAI)
- **NFR-006**: Chunking performance MUST complete in <100ms for typical documents (~10 pages, ~100 chunks) measured end-to-end from chunk() call to collecting all chunks

### Key Entities

- **BaseChunker**: Abstract interface defining chunker contract. Provides chunk() method (returns iterator of chunks) and contextualize() method (enriches chunk with metadata).

- **HierarchicalChunker**: Chunker implementation that creates one chunk per document element, optionally merging list items. Preserves document structure metadata.

- **HybridChunker**: Advanced chunker that starts with hierarchical chunks and applies tokenization-aware refinements. Splits oversized chunks and optionally merges undersized peers based on token limits.

- **BaseChunk**: Represents a single chunk with text content and metadata (headings, captions, source document reference).

- **Tokenizer**: Component responsible for counting tokens in text. Supports HuggingFace and OpenAI tokenizers. Defines max_tokens limit.

- **ChunkMetadata**: Structured metadata attached to each chunk, including hierarchical context (headings path, captions), document reference, and position information.

---

## Review & Acceptance Checklist
*GATE: Automated checks run during main() execution*

### Content Quality
- [x] No implementation details (languages, frameworks, APIs)
- [x] Focused on user value and business needs
- [x] Written for non-technical stakeholders
- [x] All mandatory sections completed

### Requirement Completeness
- [x] No [NEEDS CLARIFICATION] markers remain (all resolved)
- [x] Requirements are testable and unambiguous
- [x] Success criteria are measurable
- [x] Scope is clearly bounded
- [x] Dependencies and assumptions identified (depends on Phase 1 DoclingDocument)

---

## Execution Status
*Updated by main() during processing*

- [x] User description parsed
- [x] Key concepts extracted
- [x] Ambiguities marked and resolved (1 clarification completed)
- [x] User scenarios defined
- [x] Requirements generated (37 FR + 6 NFR)
- [x] Entities identified (6 entities)
- [x] Review checklist passed (all clarifications resolved)

---

## Alignment with Docling-Original

This specification is based on the chunking functionality in docling-core (docling-project/docling-core), specifically:

- **HierarchicalChunker**: Creates chunks per document element, merges list items by default
- **HybridChunker**: Two-pass approach (split oversized, merge undersized peers)
- **Tokenizer support**: HuggingFace transformers and OpenAI tiktoken
- **Contextualization**: Prefixes chunk text with hierarchical metadata for embedding

Key differences from initial user description:
- Original description mentioned "character, word, sentence, paragraph strategies with overlap" (basic text splitting)
- Actual docling approach is structure-aware chunking with tokenization refinement (more sophisticated)
- No overlap parameter; instead uses intelligent merging based on metadata and token limits

---
