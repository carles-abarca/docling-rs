# Implementation Plan: CLI Manual Testing Script

**Branch**: `006-cli-manual-testing` | **Date**: 2025-10-05 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/Users/carlesabarca/MyProjects/docling-rs/specs/006-cli-manual-testing/spec.md`

## Execution Flow (/plan command scope)
```
1. Load feature spec from Input path ✅
2. Fill Technical Context ✅
   → Project Type: single (Rust library with CLI)
   → Structure: Standard Rust project
3. Fill Constitution Check section ✅
4. Evaluate Constitution Check
   → No violations - manual testing script aligns with all principles ✅
5. Execute Phase 0 → research.md ✅
6. Execute Phase 1 → contracts, data-model.md, quickstart.md, CLAUDE.md ✅
7. Re-evaluate Constitution Check ✅
8. Plan Phase 2 → Describe task generation approach ✅
9. STOP - Ready for /tasks command ✅
```

## Summary
Create a manual testing shell script that processes all test documents in `tests/documents-test/` using the CLI release binary, displaying extracted text and chunking results for each file with clear formatting and summary statistics.

## Technical Context
**Language/Version**: Bash shell script (POSIX compatible)
**Primary Dependencies**: docling-rs CLI release binary (`cargo build --release`)
**Storage**: Test documents in `tests/documents-test/` directory
**Testing**: Manual verification of script output
**Target Platform**: macOS and Linux (cross-platform shell script)
**Project Type**: single (Rust library + CLI + testing infrastructure)
**Performance Goals**: Process all test documents quickly for developer feedback
**Constraints**: Must work with current CLI implementation status, gracefully handle unsupported formats
**Scale/Scope**: ~14 test documents (PDF, DOCX, MD, JSON, TXT, YAML, SH, PY, XLSX, PPTX, JPEG)

## Constitution Check
*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### I. Library-First Architecture ✅
- **Status**: COMPLIANT
- **Rationale**: This is a testing/tooling feature, not library code. Testing infrastructure supports library development.

### II. CLI Interface Contract ✅
- **Status**: COMPLIANT
- **Rationale**: Script invokes existing CLI which follows contract. Script itself is developer tooling.

### III. Test-Driven Development (NON-NEGOTIABLE) ⚠️
- **Status**: EXEMPT (Testing Infrastructure)
- **Rationale**: This IS the testing infrastructure. Manual testing script validates CLI behavior. TDD applies to CLI implementation (already covered in feature 005).

### IV. Integration & Contract Testing ✅
- **Status**: COMPLIANT
- **Rationale**: Script exercises integration tests by invoking CLI with real documents.

### V. Rust Best Practices ✅
- **Status**: N/A (Shell Script)
- **Rationale**: No Rust code in this feature. Shell script follows POSIX best practices.

### VI. Cross-Platform Compatibility ✅
- **Status**: COMPLIANT
- **Rationale**: Shell script uses POSIX-compatible syntax, works on macOS and Linux. Paths handled portably.

### VII. Native Rust Dependencies ✅
- **Status**: COMPLIANT
- **Rationale**: No new dependencies. Uses existing docling-rs CLI binary.

**Initial Check**: ✅ PASS (No violations)
**Post-Design Check**: ✅ PASS (Design complete, no violations introduced)

## Project Structure

### Documentation (this feature)
```
specs/006-cli-manual-testing/
├── plan.md              # This file (/plan command output)
├── research.md          # Phase 0: Script design decisions
├── quickstart.md        # Phase 1: How to use the testing script
└── tasks.md             # Phase 2: Implementation tasks (/tasks command)
```

### Source Code (repository root)
```
tests/
├── documents-test/      # Real test documents (already copied)
│   ├── UseOfChatGPT.pdf
│   ├── Conferencistas_Futurologia_Educativa.docx
│   ├── test_readme.md
│   ├── test_data.json
│   ├── test_document.txt
│   ├── test_config.yaml
│   ├── test_script.sh
│   ├── test_code.py
│   ├── Indian_CEOs.xlsx
│   ├── RoadmaptoAccelerateAIStrategyforHigherEducationCIOs_829565.pptx
│   ├── pie.jpeg
│   ├── test_notes.txt
│   └── ... (other test files)
└── scripts/
    └── test-cli-manual.sh  # NEW: Manual testing script

scripts/                 # Alternative location (TBD in research)
└── test-cli-manual.sh

target/
└── release/
    └── docling-rs       # CLI binary (built by script if needed)
```

## Phase 0: Research

### Decision 1: Script Location
**Question**: Where should the testing script live?
**Options**:
1. `tests/scripts/test-cli-manual.sh` - Co-located with test data
2. `scripts/test-cli-manual.sh` - General project scripts directory
3. Root directory `test-cli.sh` - Maximum visibility

**Decision**: `scripts/test-cli-manual.sh`
**Rationale**:
- Project-level scripts directory follows Rust conventions
- Separate from automated tests (different purpose)
- Easy to find and execute from project root
- Consistent with potential future manual testing scripts

### Decision 2: Output Format
**Question**: How should results be displayed?
**Options**:
1. Simple text dump - Just concatenate outputs
2. Formatted sections - Clear separators, headings, colors
3. Markdown output - Structured, pasteable into docs

**Decision**: Formatted sections with ANSI colors
**Rationale**:
- Terminal-friendly (primary use case)
- Clear visual separation between files
- Colors highlight success/failure at a glance
- Human-readable but structured

### Decision 3: Build Strategy
**Question**: How to handle CLI binary availability?
**Options**:
1. Assume binary exists - Fail if not built
2. Auto-build release binary - Always ensure latest
3. Check and prompt - Notify user, ask to build

**Decision**: Auto-build release binary if not present or outdated
**Rationale**:
- Developer convenience (one command to test)
- Ensures testing against latest code
- Clear feedback during build process
- Minimal surprise (shows build output)

### Decision 4: Error Handling
**Question**: How to handle individual file failures?
**Options**:
1. Fail fast - Stop on first error
2. Continue all - Process all files regardless
3. Configurable - Flag to control behavior

**Decision**: Continue all, display errors inline
**Rationale**:
- Matches FR-006 (continue on failure)
- Maximizes test coverage per run
- Shows which formats work vs fail
- Summary shows overall status

### Decision 5: Output Truncation
**Question**: How to handle large document outputs?
**Options**:
1. No truncation - Show full output (could be huge)
2. Smart truncation - First N lines + last N lines
3. Summary only - Just show statistics

**Decision**: Smart truncation (first 50 lines, then "..." indicator)
**Rationale**:
- Balances detail vs readability (FR-015)
- Shows sample of extraction quality
- Prevents terminal flooding
- Full output available by running CLI directly

### Decision 6: CLI Invocation
**Question**: What CLI arguments to use for testing?
**Options**:
1. Minimal - Just filename (default behavior)
2. Comprehensive - Test all major flags
3. Multiple passes - Test different output formats

**Decision**: Two passes per file: (1) Text extraction, (2) With chunking
**Rationale**:
- Tests both core capabilities (FR-014)
- Shows extraction and chunking separately
- Matches primary use cases
- Reasonable output length

### Decision 7: Performance Measurement
**Question**: Should script measure execution time?
**Options**:
1. No timing - Just results
2. Total time - Overall script duration
3. Per-file timing - Individual document metrics

**Decision**: Per-file timing with summary
**Rationale**:
- Identifies slow documents/formats
- Useful for performance testing
- Helps prioritize optimizations
- Minimal implementation cost

## Phase 1: Design Artifacts

### Data Model
*This feature is primarily a shell script with no complex data structures. See quickstart.md for usage examples.*

**Key Concepts**:
- **Test Document**: File in `tests/documents-test/` to be processed
- **Test Result**: Success/failure status + timing for one document
- **Test Summary**: Aggregate statistics (total, passed, failed, duration)

### Contracts
*No formal contract tests needed - this is testing infrastructure. The script validates CLI behavior manually.*

**Implicit Contract**: The testing script expects:
- CLI binary at `target/release/docling-rs`
- Test documents in `tests/documents-test/`
- CLI accepts file path as argument
- CLI returns exit code 0 on success, non-zero on failure

### Quickstart
See [quickstart.md](./quickstart.md)

## Phase 2: Task Generation Approach

The /tasks command will generate tasks.md with these task groups:

### Group 1: Script Creation (3-4 tasks)
- Create `scripts/` directory if needed
- Write basic shell script structure with error handling
- Implement build check and auto-build logic
- Add color output support (ANSI codes)

### Group 2: Document Processing Loop (3-4 tasks)
- Implement file discovery in tests/documents-test/
- Create per-file processing function
- Add text extraction test invocation
- Add chunking test invocation with output formatting

### Group 3: Output Formatting (2-3 tasks)
- Implement section headers and separators
- Add smart truncation logic (first 50 lines)
- Implement timing measurements per file

### Group 4: Summary & Error Handling (2-3 tasks)
- Collect test results (success/failure/timing)
- Display summary statistics
- Handle missing binary, missing test files gracefully
- Add usage instructions (help text)

### Group 5: Polish & Documentation (2 tasks)
- Make script executable (chmod +x)
- Add README.md section or inline documentation
- Test script on multiple document types
- Verify cross-platform compatibility (macOS/Linux)

**Estimated Total**: 12-16 tasks
**Parallelization**: Most tasks sequential (shell script is single file)
**Dependencies**: Group 1 → Group 2 → Group 3 → Group 4 → Group 5

## Progress Tracking
- [x] Step 1: Load feature spec
- [x] Step 2: Fill Technical Context
- [x] Step 3: Fill Constitution Check
- [x] Step 4: Initial Constitution Check - PASS
- [x] Step 5: Phase 0 Research (research.md)
- [x] Step 6: Phase 1 Design (quickstart.md)
- [x] Step 7: Post-Design Constitution Check - PASS
- [x] Step 8: Phase 2 Task Planning
- [x] Step 9: READY FOR /tasks COMMAND

## Complexity Tracking
*No violations or complexities to track.*

This is a straightforward shell script for developer convenience. No architectural complexity, no constitutional violations, no edge cases requiring special handling.

---

**Status**: ✅ Planning complete - Ready for /tasks command
