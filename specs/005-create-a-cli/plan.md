# Implementation Plan: CLI for docling-rs

**Branch**: `005-create-a-cli` | **Date**: 2025-10-05 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/Users/carlesabarca/MyProjects/docling-rs/specs/005-create-a-cli/spec.md`

## Execution Flow (/plan command scope)
```
1. Load feature spec from Input path ✓
2. Fill Technical Context ✓
3. Fill Constitution Check ✓
4. Evaluate Constitution Check → PASS (no violations)
5. Execute Phase 0 → research.md ✓
6. Execute Phase 1 → contracts, data-model.md, quickstart.md, CLAUDE.md ✓
7. Re-evaluate Constitution Check → PASS
8. Plan Phase 2 → Task generation approach ✓
9. STOP - Ready for /tasks command
```

## Summary

Build a command-line interface (CLI) binary for docling-rs that provides document conversion capabilities matching the original docling Python CLI. The CLI will expose all existing backend functionality (PDF, Markdown, HTML, CSV, DOCX) through an intuitive command-line interface with support for single-file conversion, batch directory processing, multiple output formats, and PDF-specific options (OCR, tables, images, enrichment).

**Technical Approach**: Use `clap` crate for argument parsing with derive macros, `indicatif` for progress bars, existing docling-rs Backend implementations for conversion logic, and create a new binary target (`src/bin/docling-rs.rs`) that orchestrates file I/O, format detection, backend selection, and output generation.

## Technical Context

**Language/Version**: Rust 1.75+
**Primary Dependencies**:
- `clap` v4.5+ (CLI argument parsing with derive macros)
- `indicatif` v0.17+ (progress bars for batch operations)
- `serde_json` (JSON output format - already in project)
- All existing docling-rs backends (Phase 1 + Phase 3)

**Storage**: Filesystem (input/output files only)
**Testing**: `cargo test` (contract tests for CLI interface, integration tests for end-to-end scenarios)
**Target Platform**: macOS and Windows (cross-platform CLI binary)
**Project Type**: Single project (adds binary target to existing library)
**Performance Goals**:
- Single file conversion: <5 seconds for simple PDFs (<10 pages)
- Batch processing: Handle 100+ files without memory issues
- CLI startup time: <100ms

**Constraints**:
- Exit codes: 0=success, 1=error (POSIX conventions)
- Output files must not corrupt or lose data
- Progress indication for operations >2 seconds
- Error messages must be actionable

**Scale/Scope**:
- ~1000 lines of CLI code (argument parsing, file I/O, orchestration)
- 6-8 main CLI commands/flags
- ~20 integration test scenarios

## Constitution Check

**GATE: All checks passed ✓**

### Principle I: Library-First Architecture ✓
- CLI uses existing library (docling-rs) without modifying core logic
- All conversion logic remains in library backends
- CLI is thin orchestration layer over library APIs

### Principle II: CLI Interface Contract ✓
- Input: command-line arguments + file paths
- Output: Structured files (markdown, JSON, text) to stdout/filesystem
- Errors: stderr with exit codes (0=success, 1=error)
- Follows POSIX conventions

### Principle III: Test-Driven Development ✓
- Contract tests for CLI argument parsing
- Integration tests for conversion workflows
- Tests written before implementation

### Principle IV: Integration & Contract Testing ✓
- CLI contract tests verify argument parsing and exit codes
- Integration tests verify end-to-end file conversion
- Tests validate interaction with existing backends

### Principle V: Rust Best Practices ✓
- Uses idiomatic `clap` derive macros
- Error handling via `Result<T, E>`
- No unsafe code
- Clippy-compliant code

### Principle VI: Cross-Platform Compatibility ✓
- Uses `std::path::PathBuf` for all file paths
- No platform-specific code (clap handles cross-platform arguments)
- CI tests on macOS and Windows

### Principle VII: Native Rust Dependencies ✓
- `clap`: Industry-standard Rust CLI parsing
- `indicatif`: Pure Rust progress bars
- No Python dependencies

**Result**: ✅ No constitutional violations

## Project Structure

### Documentation (this feature)
```
specs/005-create-a-cli/
├── plan.md              # This file (/plan command output)
├── research.md          # Phase 0 output (/plan command)
├── data-model.md        # Phase 1 output (/plan command)
├── quickstart.md        # Phase 1 output (/plan command)
├── contracts/           # Phase 1 output (/plan command)
│   └── cli_interface.md
└── tasks.md             # Phase 2 output (/tasks command - NOT created by /plan)
```

### Source Code (repository root)
```
src/
├── bin/
│   └── docling-rs.rs      # CLI binary entry point (NEW)
├── cli/                   # CLI module (NEW)
│   ├── mod.rs
│   ├── args.rs            # Clap argument structures
│   ├── converter.rs       # Orchestrates Backend calls
│   ├── output.rs          # Output file generation
│   └── progress.rs        # Progress bar utilities
├── backend/               # Existing backends (unchanged)
│   ├── pdf/
│   ├── markdown.rs
│   ├── html.rs
│   ├── csv.rs
│   └── docx.rs
├── datamodel/            # Existing (unchanged)
└── error.rs              # Existing (may add CLI-specific errors)

tests/
├── contract/
│   └── contract_cli.rs   # CLI contract tests (NEW)
└── integration/
    ├── integration_cli_single.rs     # Single file tests (NEW)
    ├── integration_cli_batch.rs      # Batch tests (NEW)
    └── integration_cli_pdf_opts.rs   # PDF options tests (NEW)
```

**Structure Decision**: Single project structure (Option 1) - adds binary target to existing docling-rs library. The CLI is a thin layer that uses existing Backend trait implementations.

## Phase 0: Outline & Research

**Research completed - findings documented in [research.md](./research.md)**

### Key Decisions:

1. **CLI Framework**: `clap` v4.5+ with derive macros
   - Rationale: Industry standard, excellent error messages, supports derive API
   - Alternatives: structopt (deprecated), argh (less features)

2. **Progress Indication**: `indicatif` v0.17+
   - Rationale: Pure Rust, integrates well with terminal I/O
   - Alternatives: pbr (less maintained), manual progress printing

3. **Binary Architecture**: Separate binary target (`src/bin/docling-rs.rs`)
   - Rationale: Standard Rust binary/library split, keeps library reusable
   - Alternatives: Single binary crate (loses library reusability)

4. **Output Formats**: Leverage existing serialization
   - Markdown: Use existing export logic
   - JSON: serde_json serialization of DoclingDocument
   - Text: Simple text extraction from DoclingDocument

5. **File Detection**: Extension-based format detection
   - Rationale: Simple, reliable, matches user expectations
   - Alternatives: Magic number detection (overkill for this use case)

## Phase 1: Design & Contracts

**Design artifacts completed:**
- [data-model.md](./data-model.md) - CLI domain entities
- [contracts/cli_interface.md](./contracts/cli_interface.md) - CLI argument contracts
- [quickstart.md](./quickstart.md) - Usage examples

### Data Model Summary:

**CliArgs**: Command-line arguments structure
- `input`: PathBuf (file or directory)
- `from`: Vec<InputFormat> (format filters)
- `to`: Vec<OutputFormat> (output formats)
- `output`: Option<PathBuf> (output directory)
- `ocr`: bool
- `tables`: bool
- `images`: bool
- `enrich_code`: bool
- `enrich_formula`: bool
- `verbose`: u8 (0-3)
- `abort_on_error`: bool

**ConversionJob**: Represents one file to convert
- `input_path`: PathBuf
- `input_format`: InputFormat
- `output_formats`: Vec<OutputFormat>
- `output_dir`: PathBuf
- `config`: ConversionConfig

**ConversionResult**: Result of processing one file
- `input_path`: PathBuf
- `success`: bool
- `outputs`: Vec<PathBuf> (generated files)
- `error`: Option<String>

### Contract Summary:

CLI must:
- Accept single file path or directory path
- Parse all flags correctly (--from, --to, --output, --ocr, etc.)
- Return exit code 0 on success, 1 on error
- Generate correctly named output files
- Show progress for batch operations
- Print clear error messages to stderr

### Contract Tests (failing until implementation):

1. `test_cli_args_parsing`: Verify clap parses arguments correctly
2. `test_single_file_conversion`: Verify single file workflow
3. `test_batch_conversion`: Verify directory processing
4. `test_multiple_output_formats`: Verify simultaneous formats
5. `test_error_handling`: Verify error messages and exit codes
6. `test_pdf_options`: Verify PDF-specific flags

## Phase 2: Task Planning Approach

**This section describes what the /tasks command will do - DO NOT execute during /plan**

### Task Generation Strategy:

1. **Setup Tasks** (3-4 tasks):
   - Add dependencies (clap, indicatif) to Cargo.toml
   - Create src/cli/ module structure
   - Create src/bin/docling-rs.rs binary
   - Configure Cargo.toml [[bin]] section

2. **Contract Test Tasks** (6 tasks) [P]:
   - One task per contract test from Phase 1
   - Tests must fail initially (no implementation)
   - Can run in parallel (independent test files)

3. **Core Implementation Tasks** (8-10 tasks):
   - Implement CliArgs with clap derives
   - Implement format detection logic
   - Implement Backend selection/dispatch
   - Implement output file generation
   - Implement progress bar utilities
   - Implement error message formatting
   - Implement main() orchestration

4. **Integration Test Tasks** (4-6 tasks):
   - Test single file conversion workflow
   - Test batch directory processing
   - Test PDF options (OCR, tables, images)
   - Test enrichment options
   - Test error scenarios

5. **Validation Tasks** (2-3 tasks):
   - Run all tests (cargo test)
   - Test cross-platform (macOS + Windows CI)
   - Update README.md with CLI usage

### Ordering Strategy:

```
1. Setup (T001-T004) → blocks all other tasks
2. Contract tests (T005-T010) [P] → blocks implementation
3. Core implementation (T011-T020) → sequential, some [P]
4. Integration tests (T021-T026) [P] → verify implementation
5. Validation (T027-T029) → final checks
```

### Task Dependencies:

- Setup must complete before any other work
- Contract tests must exist before implementation
- Core implementation can have some parallelism (independent modules)
- Integration tests require core implementation complete
- Validation is final gate

**Estimated Output**: 25-30 numbered, ordered tasks in tasks.md

**IMPORTANT**: This phase is executed by the /tasks command, NOT by /plan

## Phase 3+: Future Implementation

*These phases are beyond the scope of the /plan command*

**Phase 3**: Task execution (/tasks command creates tasks.md)
**Phase 4**: Implementation (execute tasks.md following TDD)
**Phase 5**: Validation (run tests, verify cross-platform, performance testing)

## Complexity Tracking

*No constitutional violations - this section is empty*

## Progress Tracking

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
- [x] Complexity deviations documented (none)

---
*Based on Constitution v1.2.0 - See `/memory/constitution.md`*
