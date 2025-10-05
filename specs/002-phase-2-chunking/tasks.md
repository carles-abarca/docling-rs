# Tasks: Document Chunking System

**Input**: Design documents from `/specs/002-phase-2-chunking/`
**Prerequisites**: plan.md (✅), research.md (✅), data-model.md (✅), contracts/ (✅)

## Execution Flow (main)
```
1. Load plan.md from feature directory ✅
   → Tech stack: Rust 1.75+, tokenizers, unicode-segmentation
   → Structure: Single library project, src/chunking/ module
2. Load optional design documents: ✅
   → data-model.md: 7 entities extracted
   → contracts/: 3 contract files (15+ tests)
   → research.md: Dependency decisions extracted
3. Generate tasks by category: ✅
   → Setup: Cargo.toml, module structure
   → Tests: BaseChunker trait tests, chunker behavior tests, tokenizer tests
   → Core: Traits, data structures, chunker implementations
   → Integration: End-to-end tests, serialization tests
   → Polish: Documentation, examples validation
4. Apply task rules: ✅
   → [P] for different files (parallel execution)
   → Sequential for same file modifications
   → TDD: Tests before implementation
5. Number tasks sequentially (T001-T031) ✅
6. Validate task completeness: ✅
   → All 3 contracts have tests
   → All 7 entities have tasks
   → All tests before implementation
9. Return: SUCCESS (31 tasks ready for execution)
```

## Format: `[ID] [P?] Description`
- **[P]**: Can run in parallel (different files, no dependencies)
- File paths are relative to repository root

---

## Phase 3.1: Setup (2 tasks)

- [x] **T001** Add chunking dependencies to Cargo.toml
  - **File**: `Cargo.toml`
  - **Action**: Add `tokenizers = "0.15"` and `unicode-segmentation = "1.11"` to [dependencies]
  - **Why**: Required for HuggingFace tokenizers and sentence boundary detection (from research.md)
  - **Test**: `cargo check` succeeds

- [x] **T002** Create chunking module structure
  - **Files**:
    - `src/chunking/mod.rs`
    - `src/chunking/base.rs`
    - `src/chunking/hierarchical.rs`
    - `src/chunking/hybrid.rs`
    - `src/chunking/metadata.rs`
    - `src/chunking/tokenizer/mod.rs`
    - `src/chunking/tokenizer/base.rs`
    - `src/chunking/tokenizer/huggingface.rs`
  - **Action**: Create empty modules with proper re-exports in mod.rs
  - **Why**: Establish module structure from plan.md
  - **Test**: `cargo check` succeeds (empty modules compile)

---

## Phase 3.2: Data Models (7 tasks, parallel)

**IMPORTANT**: Define types BEFORE writing tests (tests need these types to compile)

- [x] **T003** [P] Define ChunkingError type
  - **File**: `src/chunking/base.rs`
  - **Action**: Define error enum using thiserror (TokenizerLoad, InvalidConfig, ProcessingError, SerializationError)
  - **From**: data-model.md "Error Handling" section
  - **Test**: `cargo check` succeeds

- [x] **T004** [P] Define ChunkMetadata struct
  - **File**: `src/chunking/metadata.rs`
  - **Action**: Define struct with fields: doc_name, headings, caption, start_offset, end_offset, index
  - **From**: data-model.md "ChunkMetadata" entity
  - **Derives**: Serialize, Deserialize, Debug, Clone
  - **Test**: `cargo check` succeeds

- [x] **T005** [P] Define BaseChunk struct
  - **File**: `src/chunking/base.rs`
  - **Action**: Define struct with fields: text (String), meta (ChunkMetadata)
  - **From**: data-model.md "BaseChunk" entity
  - **Derives**: Serialize, Deserialize, Debug, Clone
  - **Test**: `cargo check` succeeds

- [x] **T006** [P] Define BaseChunker trait
  - **File**: `src/chunking/base.rs`
  - **Action**: Define trait with methods: chunk<'a>(&'a self, doc: &'a DoclingDocument) -> Box<dyn Iterator<Item = BaseChunk> + 'a>, contextualize(&self, chunk: &BaseChunk) -> String
  - **From**: data-model.md "BaseChunker (Trait)" and contracts/base_chunker_trait.md
  - **Test**: `cargo check` succeeds

- [x] **T007** [P] Define Tokenizer trait
  - **File**: `src/chunking/tokenizer/base.rs`
  - **Action**: Define trait with methods: count_tokens(&self, text: &str) -> usize, max_tokens(&self) -> usize
  - **From**: data-model.md "Tokenizer (Trait)" entity
  - **Test**: `cargo check` succeeds

- [x] **T008** [P] Define HierarchicalChunker struct
  - **File**: `src/chunking/hierarchical.rs`
  - **Action**: Define struct with field: merge_list_items (bool)
  - **From**: data-model.md "HierarchicalChunker" entity
  - **Derives**: Debug, Clone
  - **Test**: `cargo check` succeeds

- [x] **T009** [P] Define HybridChunker struct and builder
  - **File**: `src/chunking/hybrid.rs`
  - **Action**: Define struct with private fields, HybridChunkerBuilder for configuration
  - **From**: data-model.md "HybridChunker" entity, contracts/hybrid_chunker.md builder pattern
  - **Test**: `cargo check` succeeds

---

## Phase 3.3: Tests First (TDD) - 10 tasks ⚠️ MUST COMPLETE BEFORE 3.4

**CRITICAL: These tests MUST be written and MUST FAIL before ANY implementation**

- [x] **T010** [P] BaseChunker trait contract tests (Test 1-3)
  - **File**: `tests/chunking_contract.rs`
  - **Action**: Write 3 contract tests from contracts/base_chunker_trait.md:
    - test_chunks_are_sequential (FR-025)
    - test_iterator_is_lazy (NFR-001)
    - test_all_content_represented (completeness)
  - **Why**: Verify BaseChunker contract before implementation
  - **Expected**: Tests FAIL (no implementation yet)

- [x] **T011** [P] BaseChunker contextualize tests (Test 4-6)
  - **File**: `tests/chunking_contract.rs`
  - **Action**: Write 3 contextualize tests from contracts/base_chunker_trait.md:
    - test_contextualize_includes_headings (FR-030)
    - test_contextualize_is_deterministic
    - test_contextualize_without_metadata
  - **Why**: Verify contextualize() behavior
  - **Expected**: Tests FAIL

- [x] **T012** [P] HierarchicalChunker behavior tests
  - **File**: `tests/chunking_hierarchical.rs`
  - **Action**: Write 6 tests from contracts/hierarchical_chunker.md:
    - test_new_has_default_merge
    - test_with_merge_list_items
    - test_one_chunk_per_element (FR-005)
    - test_merge_list_items_true (FR-007)
    - test_merge_list_items_false (FR-008)
    - test_metadata_includes_headings (FR-009)
  - **Why**: Verify hierarchical chunking behavior
  - **Expected**: Tests FAIL

- [x] **T013** [P] HierarchicalChunker edge case tests
  - **File**: `tests/chunking_hierarchical.rs`
  - **Action**: Write 3 edge case tests from contracts/hierarchical_chunker.md:
    - test_empty_document
    - test_flat_document
    - test_nested_lists_merged
  - **Why**: Handle edge cases
  - **Expected**: Tests FAIL

- [x] **T014** [P] HybridChunker constructor tests
  - **File**: `tests/chunking_hybrid.rs`
  - **Action**: Write 3 tests from contracts/hybrid_chunker.md:
    - test_new_uses_tokenizer_max
    - test_builder_pattern
    - test_max_tokens_validation
  - **Why**: Verify configuration and builder
  - **Expected**: Tests FAIL

- [x] **T015** [P] HybridChunker split/merge tests
  - **File**: `tests/chunking_hybrid.rs`
  - **Action**: Write 5 tests from contracts/hybrid_chunker.md:
    - test_hybrid_refines_hierarchical (FR-010)
    - test_split_oversized_chunks (FR-013)
    - test_merge_undersized_peers (FR-015)
    - test_no_merge_when_disabled (FR-016)
    - test_respects_token_boundaries (FR-017)
  - **Why**: Verify tokenization-aware splitting and merging
  - **Expected**: Tests FAIL

- [x] **T016** [P] Tokenizer trait contract tests
  - **File**: `tests/chunking_tokenizer.rs`
  - **Action**: Write 3 tokenizer tests:
    - test_count_tokens_basic
    - test_max_tokens_exposed
    - test_tokenizer_deterministic
  - **Why**: Verify tokenizer abstraction
  - **Expected**: Tests FAIL

- [x] **T017** [P] Integration test: Basic hierarchical chunking
  - **File**: `tests/chunking_integration.rs`
  - **Action**: Write test from quickstart.md "Basic Usage" example
    - Convert Markdown document
    - Apply HierarchicalChunker
    - Verify chunks generated
    - Verify contextualize() output
  - **Why**: User story validation (FR-031, FR-032)
  - **Expected**: Test FAILS

- [x] **T018** [P] Integration test: Hybrid chunking with tokenizer (covered in integration suite)
  - **File**: `tests/chunking_integration.rs`
  - **Action**: Write test from quickstart.md "Advanced Usage" example
    - Load HuggingFace tokenizer
    - Configure HybridChunker with max_tokens
    - Verify chunks respect token limit
    - Verify token counting
  - **Why**: Token-aware chunking validation (FR-011, FR-012, FR-013)
  - **Expected**: Test FAILS

- [x] **T019** [P] Serialization tests
  - **File**: `tests/chunking_integration.rs`
  - **Action**: Write tests from quickstart.md "Serialization" section:
    - test_serialize_chunks_to_json (FR-035)
    - test_deserialize_chunks_from_json (FR-036)
    - test_serialized_metadata_preserved (FR-037)
  - **Why**: Verify serde integration
  - **Expected**: Tests FAIL

**Checkpoint**: Verify ALL tests fail with `cargo test`. Do NOT proceed to Phase 3.4 until confirmed.

---

## Phase 3.4: Core Implementation (8 tasks)

**ONLY proceed after Phase 3.3 tests are written and failing**

- [x] **T020** Implement HierarchicalChunker constructors
  - **File**: `src/chunking/hierarchical.rs`
  - **Action**: Implement new() and with_merge_list_items()
  - **From**: contracts/hierarchical_chunker.md constructor contracts
  - **Goal**: Make T012 constructor tests pass
  - **Test**: `cargo test test_new_has_default_merge test_with_merge_list_items`

- [x] **T021** Implement HierarchicalChunker::chunk() method
  - **File**: `src/chunking/hierarchical.rs`
  - **Action**: Implement BaseChunker::chunk() for HierarchicalChunker
    - Iterate over DoclingDocument nodes (from Phase 1)
    - Create chunks per element (FR-005)
    - Merge list items if configured (FR-007, FR-008)
    - Attach metadata (headings, offsets) (FR-009)
    - Return lazy iterator
  - **From**: contracts/hierarchical_chunker.md behavior contracts
  - **Goal**: Make T012-T013 tests pass
  - **Test**: `cargo test chunking_hierarchical`

- [x] **T022** Implement HierarchicalChunker::contextualize() method
  - **File**: `src/chunking/hierarchical.rs`
  - **Action**: Implement BaseChunker::contextualize()
    - Prefix chunk text with headings (FR-030)
    - Format: one heading per line, then chunk text
  - **From**: contracts/base_chunker_trait.md contextualize contract
  - **Goal**: Make T011 contextualize tests pass
  - **Test**: `cargo test test_contextualize`

- [x] **T023** Implement HuggingFaceTokenizer
  - **File**: `src/chunking/tokenizer/huggingface.rs`
  - **Action**: Implement Tokenizer trait using tokenizers crate
    - from_pretrained(model_id) constructor (FR-022)
    - count_tokens() using tokenizer.encode()
    - max_tokens() getter
    - default() using sentence-transformers/all-MiniLM-L6-v2
  - **From**: research.md tokenization decision, data-model.md HuggingFaceTokenizer
  - **Goal**: Make T016 tokenizer tests pass
  - **Test**: `cargo test chunking_tokenizer`

- [x] **T024** Implement HybridChunker builder
  - **File**: `src/chunking/hybrid.rs`
  - **Action**: Implement HybridChunkerBuilder
    - tokenizer(), max_tokens(), merge_peers() setters
    - build() with validation (max_tokens > 0) (FR-012)
    - new() convenience constructor
  - **From**: contracts/hybrid_chunker.md builder pattern, data-model.md
  - **Goal**: Make T014 constructor tests pass
  - **Test**: `cargo test test_builder_pattern test_max_tokens_validation`

- [x] **T025** Implement HybridChunker Pass 1: Hierarchical
  - **File**: `src/chunking/hybrid.rs`
  - **Action**: Implement first pass using internal HierarchicalChunker (FR-010)
  - **From**: contracts/hybrid_chunker.md two-pass processing
  - **Goal**: Partial implementation for T015
  - **Test**: `cargo test test_hybrid_refines_hierarchical`

- [x] **T026** Implement HybridChunker Pass 2: Split oversized
  - **File**: `src/chunking/hybrid.rs`
  - **Action**: Implement splitting logic (FR-013, FR-017)
    - For each hierarchical chunk, check contextualized token count
    - If > max_tokens, split respecting token boundaries
    - Use tokenizer.count_tokens() for measurement
  - **From**: contracts/hybrid_chunker.md split behavior
  - **Goal**: Make test_split_oversized_chunks pass
  - **Test**: `cargo test test_split_oversized_chunks test_respects_token_boundaries`

- [x] **T027** Implement HybridChunker Pass 3: Merge peers
  - **File**: `src/chunking/hybrid.rs`
  - **Action**: Implement merging logic (FR-015, FR-016)
    - If merge_peers=true, merge consecutive undersized chunks with same headings
    - Check combined size <= max_tokens
    - Preserve order
  - **From**: contracts/hybrid_chunker.md merge behavior
  - **Goal**: Make test_merge_undersized_peers and test_no_merge_when_disabled pass
  - **Test**: `cargo test test_merge_undersized_peers test_no_merge_when_disabled`

---

## Phase 3.5: Integration & Polish (4 tasks)

- [x] **T028** End-to-end integration tests
  - **File**: `tests/chunking_integration.rs`
  - **Action**: Make T017-T019 integration tests pass
    - Ensure hierarchical chunking works with Phase 1 documents
    - Ensure hybrid chunking with real tokenizer works
    - Ensure serialization round-trips correctly
  - **From**: quickstart.md examples
  - **Goal**: All integration tests green
  - **Test**: `cargo test chunking_integration`

- [x] **T029** [P] Add rustdoc documentation
  - **Files**: All `src/chunking/*.rs` files
  - **Action**: Add /// doc comments to all public APIs
    - Module-level docs with examples
    - Trait method docs with contracts
    - Struct field docs
    - Constructor docs with usage examples
  - **From**: Rust Best Practices (Constitution V)
  - **Test**: `cargo doc --open` generates docs without warnings

- [x] **T030** Update lib.rs exports
  - **File**: `src/lib.rs`
  - **Action**: Re-export chunking module
    - pub use chunking::{BaseChunker, BaseChunk, ChunkMetadata, HierarchicalChunker, HybridChunker};
    - pub use chunking::tokenizer::{Tokenizer, HuggingFaceTokenizer};
  - **Why**: Make chunking API accessible to users
  - **Test**: Example from quickstart.md compiles with `use docling_rs::chunking::*;`

- [x] **T031** Validate quickstart examples
  - **File**: Manual validation
  - **Action**: Run each example from quickstart.md
    - Basic hierarchical chunking example
    - Hybrid chunking with tokenizer example
    - RAG pipeline integration example
    - Serialization examples
  - **Why**: Ensure user-facing documentation is correct
  - **Test**: All examples compile and run without errors

---

## Dependencies

```
Setup (T001-T002)
  ↓
Data Models (T003-T009) [All parallel]
  ↓
Tests First (T010-T019) [All parallel - TDD GATE]
  ⚠️ STOP: Verify all tests FAIL
  ↓
HierarchicalChunker (T020-T022)
  ↓
Tokenizer (T023)
  ↓
HybridChunker (T024-T027)
  T024 (builder) → T025 (pass 1) → T026 (pass 2) → T027 (pass 3)
  ↓
Integration & Polish (T028-T031)
  T028 → T029, T030, T031 [T029 parallel with others]
```

### Critical Path:
T001 → T002 → T003-T009 → T010-T019 (verify fail) → T020-T022 → T023 → T024-T027 → T028 → T029-T031

---

## Parallel Execution Examples

### Phase 3.2: Data Models (All parallel)
```bash
# All data model tasks can run in parallel:
cargo check  # After each task completes
```

### Phase 3.3: Tests (All parallel)
```bash
# Launch all test-writing tasks together:
# T010-T019 can all be written in parallel (different test files)
cargo test  # Should FAIL for all new tests
```

### Phase 3.5: Documentation
```bash
# T029 can run parallel with T030-T031
# Different files, no dependencies
```

---

## Validation Checklist

**Before starting implementation (Phase 3.4)**:
- [x] All 3 contracts have corresponding test tasks (T010-T016)
- [x] All 7 entities have definition tasks (T003-T009)
- [x] All tests come before implementation (T010-T019 before T020-T027)
- [x] Each task specifies exact file path
- [x] [P] tasks modify different files

**After completing all tasks**:
- [x] `cargo test` passes (all 38 tests green)
- [x] `cargo clippy` produces no warnings in chunking code
- [x] `cargo doc --open` generates documentation
- [x] Quickstart examples run successfully

---

## Notes

- **TDD Enforcement**: Phase 3.3 MUST complete before Phase 3.4
- **Parallel Execution**: Tasks marked [P] can run concurrently
- **Commit Strategy**: Commit after each task or logical group
- **Test Coverage**: Aim for 100% of public API (Constitution III)
- **Documentation**: All public items need rustdoc (Constitution V)

---

**Task Count**: 31 tasks
**Estimated Time**: 8-12 hours (experienced Rust developer)
**Constitution Version**: 1.2.0
**Generated**: 2025-10-04
