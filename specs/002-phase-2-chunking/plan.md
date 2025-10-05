
# Implementation Plan: Document Chunking System

**Branch**: `002-phase-2-chunking` | **Date**: 2025-10-04 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/002-phase-2-chunking/spec.md`

## Execution Flow (/plan command scope)
```
1. Load feature spec from Input path
   → If not found: ERROR "No feature spec at {path}"
2. Fill Technical Context (scan for NEEDS CLARIFICATION)
   → Detect Project Type from file system structure or context (web=frontend+backend, mobile=app+api)
   → Set Structure Decision based on project type
3. Fill the Constitution Check section based on the content of the constitution document.
4. Evaluate Constitution Check section below
   → If violations exist: Document in Complexity Tracking
   → If no justification possible: ERROR "Simplify approach first"
   → Update Progress Tracking: Initial Constitution Check
5. Execute Phase 0 → research.md
   → If NEEDS CLARIFICATION remain: ERROR "Resolve unknowns"
6. Execute Phase 1 → contracts, data-model.md, quickstart.md, agent-specific template file (e.g., `CLAUDE.md` for Claude Code, `.github/copilot-instructions.md` for GitHub Copilot, `GEMINI.md` for Gemini CLI, `QWEN.md` for Qwen Code, or `AGENTS.md` for all other agents).
7. Re-evaluate Constitution Check section
   → If new violations: Refactor design, return to Phase 1
   → Update Progress Tracking: Post-Design Constitution Check
8. Plan Phase 2 → Describe task generation approach (DO NOT create tasks.md)
9. STOP - Ready for /tasks command
```

**IMPORTANT**: The /plan command STOPS at step 7. Phases 2-4 are executed by other commands:
- Phase 2: /tasks command creates tasks.md
- Phase 3-4: Implementation execution (manual or via tools)

## Summary
Implement intelligent document chunking for docling-rs with two chunker strategies: HierarchicalChunker (structure-based) and HybridChunker (structure + tokenization-aware). The system must preserve document metadata and hierarchical context while respecting token limits for RAG applications. Chunkers operate on DoclingDocument instances from Phase 1 and return lazy iterators of chunks with contextualization support for embedding models.

## Technical Context
**Language/Version**: Rust 1.75+ (matches Phase 1 MVP)
**Primary Dependencies**:
- Core: `serde`, `serde_json` (serialization from Phase 1)
- Tokenization: NEEDS CLARIFICATION (HuggingFace tokenizers via `tokenizers` crate, or tiktoken equivalent)
- Text processing: NEEDS CLARIFICATION (sentence/paragraph boundary detection)
**Storage**: N/A (in-memory processing, iterator-based lazy evaluation)
**Testing**: `cargo test` (TDD workflow, contract tests required)
**Target Platform**: Cross-platform (Windows + macOS, matching Phase 1)
**Project Type**: Single library project (extends existing docling-rs)
**Performance Goals**: Memory-efficient O(n) chunking, lazy iterators to avoid loading all chunks
**Constraints**:
- Zero Python dependencies (native Rust only per Constitution VII)
- Must preserve all document metadata without mutation
- Iterators must be lazy (no upfront memory allocation for all chunks)
**Scale/Scope**: Support documents from Phase 1 (Markdown, HTML, CSV, DOCX), chunk size controlled by token limits (e.g., 512-8192 tokens)

## Constitution Check
*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Requirement | Status | Notes |
|-----------|-------------|--------|-------|
| I. Library-First | Chunking as standalone module | ✅ PASS | Module `chunking/` with clear boundaries |
| II. CLI Interface | Not applicable for chunking | ✅ PASS | Chunking is library-only feature (no CLI needed) |
| III. TDD | Tests before implementation | ✅ PLAN | Contract tests in Phase 1, TDD workflow enforced |
| IV. Contract Testing | Public API contract tests | ✅ PLAN | BaseChunker trait tests, chunker behavior tests |
| V. Rust Best Practices | Idiomatic Rust, clippy, docs | ✅ PLAN | Traits for abstraction, Result<> for errors |
| VI. Cross-Platform | Windows + macOS support | ✅ PASS | Text processing only, no platform-specific code |
| VII. Native Rust Deps | No Python dependencies | ⚠️ RESEARCH | Tokenizer dependency needs research (Phase 0) |

**Decision Points**:
- Tokenizer dependency: Must find native Rust alternative to HuggingFace transformers (Python)
- Sentence boundary detection: Need Rust NLP crate or implement simple heuristics

**Initial Assessment**: CONDITIONAL PASS
- Proceed to Phase 0 research to resolve tokenizer dependency
- Re-check after dependency research complete

## Project Structure

### Documentation (this feature)
```
specs/[###-feature]/
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
├── lib.rs                  # Re-exports chunking module
├── datamodel/              # Phase 1 - DoclingDocument, NodeItem, etc.
├── backend/                # Phase 1 - Format parsers
├── pipeline/               # Phase 1 - SimplePipeline
├── chunking/               # Phase 2 - NEW MODULE
│   ├── mod.rs             # Module exports
│   ├── base.rs            # BaseChunker trait, BaseChunk struct
│   ├── hierarchical.rs    # HierarchicalChunker implementation
│   ├── hybrid.rs          # HybridChunker implementation
│   ├── tokenizer/         # Tokenizer abstractions
│   │   ├── mod.rs
│   │   ├── base.rs        # Tokenizer trait
│   │   └── huggingface.rs # HuggingFace tokenizer wrapper
│   └── metadata.rs        # ChunkMetadata struct

tests/
├── chunking_hierarchical.rs  # HierarchicalChunker tests
├── chunking_hybrid.rs         # HybridChunker tests
├── chunking_contract.rs       # BaseChunker trait contract tests
└── chunking_integration.rs    # End-to-end chunking tests
```

**Structure Decision**: Single library project extending existing docling-rs codebase. New `chunking/` module added at src level, following the established pattern from Phase 1 (datamodel/, backend/, pipeline/). Tests follow Phase 1 naming convention (feature_subject.rs).

## Phase 0: Outline & Research

✅ **COMPLETED** - See `research.md`

**Key Decisions**:
1. **Tokenization**: Use `tokenizers` crate (HuggingFace Rust library)
   - Native Rust, zero Python dependencies
   - Official HuggingFace library (same underlying code as Python transformers)
   - Supports all HuggingFace tokenizers (sentence-transformers, BERT, etc.)

2. **Sentence Detection**: Use `unicode-segmentation` + custom heuristics
   - UAX#29 Unicode sentence boundaries
   - Lightweight, no ML models required
   - Good enough for document chunking (not NLU)

3. **Token Counting**: Trait-based abstraction
   - `Tokenizer` trait with `count_tokens()` and `max_tokens()`
   - `HuggingFaceTokenizer` implementation
   - Future: `OpenAITokenizer` using `tiktoken-rs`

**Dependencies Added**:
```toml
tokenizers = "0.15"              # HuggingFace tokenizers
unicode-segmentation = "1.11"    # Sentence boundaries
```

**Constitutional Compliance**: ✅ All dependencies are native Rust (Principle VII)

## Phase 1: Design & Contracts

✅ **COMPLETED** - See artifacts below

**Artifacts Created**:
1. **data-model.md**: Defines 7 core entities
   - `BaseChunk`: Chunk with text + metadata
   - `ChunkMetadata`: Hierarchical context, offsets, index
   - `BaseChunker` (trait): Abstract chunker interface
   - `HierarchicalChunker`: Structure-based chunking
   - `HybridChunker`: Tokenization-aware refinement
   - `Tokenizer` (trait): Token counting abstraction
   - `HuggingFaceTokenizer`: HuggingFace tokenizers wrapper

2. **contracts/**: API contracts with test scenarios
   - `base_chunker_trait.md`: 6 contract tests for BaseChunker
   - `hierarchical_chunker.md`: Behavior contracts for structure-based chunking
   - `hybrid_chunker.md`: Token-aware splitting/merging contracts

3. **quickstart.md**: User guide with examples
   - Basic hierarchical chunking example
   - Advanced hybrid chunking with tokenizers
   - RAG pipeline integration example
   - Configuration examples
   - Troubleshooting guide

4. **CLAUDE.md**: Updated with Phase 2 context
   - Language: Rust 1.75+
   - New dependencies: tokenizers, unicode-segmentation
   - Project structure with chunking/ module

**Design Highlights**:
- Trait-based abstraction (BaseChunker, Tokenizer)
- Builder pattern for HybridChunker configuration
- Lazy iterators for memory efficiency (NFR-001)
- Serde serialization for all types (FR-035 to FR-037)

**Post-Design Constitutional Re-check**: ✅ PASS (see section below)

---

## Post-Design Constitution Check

✅ **RE-EVALUATED** after Phase 1 design completion

| Principle | Requirement | Status | Notes |
|-----------|-------------|--------|-------|
| I. Library-First | Chunking as standalone module | ✅ PASS | Clean module boundaries in `src/chunking/` |
| II. CLI Interface | Not applicable | ✅ PASS | Library-only (no CLI for chunking API) |
| III. TDD | Tests before implementation | ✅ READY | 15+ contract tests defined in contracts/ |
| IV. Contract Testing | Public API tests | ✅ READY | BaseChunker trait tests, chunker behavior tests |
| V. Rust Best Practices | Idiomatic patterns | ✅ PASS | Traits, builders, Result<>, iterators |
| VI. Cross-Platform | Windows + macOS | ✅ PASS | No platform-specific code in design |
| VII. Native Rust Deps | No Python | ✅ PASS | `tokenizers` (Rust), `unicode-segmentation` (Rust) |

**Design Quality Assessment**:
- ✅ Uses trait abstraction (BaseChunker, Tokenizer) for extensibility
- ✅ Builder pattern for configuration (Rust idiom)
- ✅ Lazy iterators for memory efficiency (Constitution principle)
- ✅ Serde for serialization (standard Rust practice)
- ✅ Error handling via Result<T, ChunkingError> (no panics)

**No New Violations**: Design complies with all constitutional principles

---

## Phase 2: Task Planning Approach
*This section describes what the /tasks command will do - DO NOT execute during /plan*

**Task Generation Strategy**:

The `/tasks` command will generate tasks following TDD workflow and dependency order:

1. **Setup Tasks** (Cargo configuration):
   - Add dependencies to Cargo.toml
   - Create module structure (src/chunking/)

2. **Data Model Tasks** (parallel where possible):
   - Define error types (ChunkingError)
   - Define BaseChunk and ChunkMetadata structs [P]
   - Define BaseChunker trait [P]
   - Define Tokenizer trait [P]
   - Add Serde derives to all types [P]

3. **Contract Test Tasks** (TDD - tests first):
   - Write BaseChunker trait contract tests (6 tests from base_chunker_trait.md)
   - Write HierarchicalChunker behavior tests (from hierarchical_chunker.md)
   - Write HybridChunker behavior tests (from hybrid_chunker.md)
   - Write Tokenizer contract tests
   - **All tests MUST fail** (no implementation yet)

4. **Implementation Tasks** (make tests pass):
   - Implement HuggingFaceTokenizer wrapper [P with tokenizer tests]
   - Implement HierarchicalChunker (make hierarchical tests pass)
   - Implement HybridChunker (make hybrid tests pass)
   - Implement BaseChunker::contextualize() for both chunkers

5. **Integration Tasks**:
   - Write end-to-end chunking tests (from quickstart.md examples)
   - Test serialization/deserialization
   - Test cross-format chunking (Markdown, HTML, CSV, DOCX from Phase 1)

6. **Documentation Tasks**:
   - Add rustdoc comments to all public APIs
   - Update lib.rs to re-export chunking module
   - Verify quickstart.md examples work

**Ordering Strategy**:
- **TDD**: Contract tests → Implementation → Integration tests
- **Dependencies**: Traits → Data structures → Implementations
- **Parallelization**: Mark [P] for independent tasks (e.g., separate struct definitions)

**Estimated Task Count**: 28-32 tasks

**Task Categories**:
- Setup: 2 tasks
- Data Models: 5 tasks (parallel)
- Contract Tests: 8 tasks (TDD)
- Implementations: 6 tasks
- Integration: 4 tasks
- Documentation: 3 tasks

**IMPORTANT**: This phase is executed by the `/tasks` command, NOT by /plan

## Phase 3+: Future Implementation
*These phases are beyond the scope of the /plan command*

**Phase 3**: Task execution (/tasks command creates tasks.md)  
**Phase 4**: Implementation (execute tasks.md following constitutional principles)  
**Phase 5**: Validation (run tests, execute quickstart.md, performance validation)

## Complexity Tracking
*Fill ONLY if Constitution Check has violations that must be justified*

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| [e.g., 4th project] | [current need] | [why 3 projects insufficient] |
| [e.g., Repository pattern] | [specific problem] | [why direct DB access insufficient] |


## Progress Tracking
*This checklist is updated during execution flow*

**Phase Status**:
- [x] Phase 0: Research complete (/plan command) ✅
- [x] Phase 1: Design complete (/plan command) ✅
- [x] Phase 2: Task planning complete (/plan command - describe approach only) ✅
- [ ] Phase 3: Tasks generated (/tasks command) - **NEXT STEP**
- [ ] Phase 4: Implementation complete
- [ ] Phase 5: Validation passed

**Gate Status**:
- [x] Initial Constitution Check: PASS ✅
- [x] Post-Design Constitution Check: PASS ✅
- [x] All NEEDS CLARIFICATION resolved ✅ (research.md)
- [x] Complexity deviations documented: N/A (no violations)

**Artifacts Generated**:
- [x] research.md (Phase 0)
- [x] data-model.md (Phase 1)
- [x] contracts/ (3 contract files)
- [x] quickstart.md (Phase 1)
- [x] CLAUDE.md updated (Phase 1)
- [ ] tasks.md (Phase 2 - to be generated by /tasks command)

---
*Based on Constitution v1.2.0 - See `.specify/memory/constitution.md`*

---

## Summary

**Planning Complete**: Phase 2 - Document Chunking System

**Branch**: `002-phase-2-chunking`
**Status**: Ready for `/tasks` command

**Key Achievements**:
- ✅ All research complete (tokenizers, sentence detection)
- ✅ Data model designed (7 entities, trait-based)
- ✅ Contracts defined (15+ test scenarios)
- ✅ Quickstart guide written (examples + troubleshooting)
- ✅ Constitutional compliance verified (all 7 principles)

**Next Command**: `/tasks` to generate implementation task list

**Estimated Implementation**: 28-32 tasks following TDD workflow
