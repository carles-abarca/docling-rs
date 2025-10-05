# Tasks: CLI for docling-rs

**Input**: Design documents from `/Users/carlesabarca/MyProjects/docling-rs/specs/005-create-a-cli/`
**Prerequisites**: plan.md ✓, research.md ✓, data-model.md ✓, contracts/ ✓, quickstart.md ✓

## Execution Summary

Generated 28 implementation tasks based on:
- **Tech Stack**: Rust 1.75+, clap 4.5+, indicatif 0.17+, anyhow 1.0
- **Entities**: CliArgs, ConversionJob, ConversionResult, BatchProgress (from data-model.md)
- **Contracts**: 18 CLI contract tests (CT-001 to CT-018 from contracts/cli_interface.md)
- **Integration**: 7 quickstart scenarios from quickstart.md

## Format: `[ID] [P?] Description`
- **[P]**: Can run in parallel (different files, no dependencies)
- File paths are absolute from repository root

## Phase 3.1: Setup & Dependencies

### Dependencies
- [x] **T001** Add CLI dependencies to Cargo.toml: clap, indicatif, anyhow
- [x] **T002** Configure Cargo.toml [[bin]] section for docling-rs binary
- [x] **T003** [P] Run cargo clippy for CLI module compliance

### Module Structure
- [ ] **T004** Create CLI module structure: `src/cli/mod.rs` with submodule declarations
- [ ] **T005** Create binary entry point: `src/bin/docling-rs.rs` with minimal main()

## Phase 3.2: Tests First (TDD) ⚠️ MUST COMPLETE BEFORE 3.3

**CRITICAL: These tests MUST be written and MUST FAIL before ANY implementation**

### Contract Tests (can run in parallel - different test files)
- [ ] **T006** [P] Contract test CT-001: Single file conversion in `tests/contract/contract_cli.rs`
- [ ] **T007** [P] Contract test CT-002: Multiple output formats in `tests/contract/contract_cli.rs`
- [ ] **T008** [P] Contract test CT-003: Custom output directory in `tests/contract/contract_cli.rs`
- [ ] **T009** [P] Contract test CT-004: Batch directory processing in `tests/contract/contract_cli.rs`
- [ ] **T010** [P] Contract test CT-005: Format filtering in `tests/contract/contract_cli.rs`
- [ ] **T011** [P] Contract test CT-006: Input file not found (error handling) in `tests/contract/contract_cli.rs`
- [ ] **T012** [P] Contract test CT-007: Unsupported format error in `tests/contract/contract_cli.rs`
- [ ] **T013** [P] Contract test CT-008: PDF with OCR in `tests/contract/contract_cli.rs`

### Integration Tests (can run in parallel - different test files)
- [ ] **T014** [P] Integration test: Single file workflow in `tests/integration/integration_cli_single.rs`
- [ ] **T015** [P] Integration test: Batch conversion in `tests/integration/integration_cli_batch.rs`
- [ ] **T016** [P] Integration test: PDF options (OCR, tables, images) in `tests/integration/integration_cli_pdf_opts.rs`

## Phase 3.3: Core Implementation (ONLY after tests are failing)

### Data Structures
- [ ] **T017** [P] Implement CliArgs with clap derives in `src/cli/args.rs`
- [ ] **T018** [P] Implement InputFormat and OutputFormat enums in `src/cli/args.rs`
- [ ] **T019** [P] Implement ConversionJob struct in `src/cli/converter.rs`
- [ ] **T020** [P] Implement ConversionResult and BatchProgress in `src/cli/converter.rs`

### Core Logic
- [ ] **T021** Implement format detection from file extension in `src/cli/converter.rs`
- [ ] **T022** Implement Backend selection/dispatch logic in `src/cli/converter.rs`
- [ ] **T023** Implement single file conversion workflow in `src/cli/converter.rs`
- [ ] **T024** Implement batch directory processing in `src/cli/converter.rs`

### Output Generation
- [ ] **T025** [P] Implement markdown output writer in `src/cli/output.rs`
- [ ] **T026** [P] Implement JSON output writer in `src/cli/output.rs`
- [ ] **T027** [P] Implement text output writer in `src/cli/output.rs`
- [ ] **T028** Implement output file naming and path generation in `src/cli/output.rs`

### Progress & UI
- [ ] **T029** Implement progress bar for batch operations in `src/cli/progress.rs`
- [ ] **T030** Implement error message formatting in `src/cli/mod.rs`

### Binary Entry Point
- [ ] **T031** Implement main() orchestration in `src/bin/docling-rs.rs`
- [ ] **T032** Implement exit code handling (0=success, 1=error) in `src/bin/docling-rs.rs`

## Phase 3.4: Integration & Polish

### Integration Tests Validation
- [ ] **T033** Verify all contract tests pass (T006-T013)
- [ ] **T034** Verify all integration tests pass (T014-T016)

### Code Quality
- [ ] **T035** Run cargo clippy and fix all warnings
- [ ] **T036** Run cargo fmt across CLI codebase
- [ ] **T037** Add rustdoc comments to public CLI APIs

### Cross-Platform Testing
- [ ] **T038** Test CLI on macOS (current platform)
- [ ] **T039** Test CLI on Windows (via CI or manual)

### Documentation
- [ ] **T040** Update README.md with CLI usage section
- [ ] **T041** Create examples/cli_usage.sh with quickstart commands

## Dependencies

### Critical Paths
1. **Setup (T001-T005)** → blocks all other tasks
2. **Contract tests (T006-T013)** → MUST complete before implementation (T017-T032)
3. **Integration tests (T014-T016)** → MUST complete before implementation
4. **Data structures (T017-T020)** → blocks core logic (T021-T024)
5. **Core logic (T021-T024)** → blocks output generation (T025-T028)
6. **All implementation (T017-T032)** → blocks integration validation (T033-T034)
7. **Integration validation (T033-T034)** → blocks polish (T035-T041)

### Parallel Opportunities
- All contract tests can run in parallel: T006-T013 (different test cases in same file)
- All integration tests can run in parallel: T014-T016 (different test files)
- Data structure tasks can run in parallel: T017-T020 (independent modules)
- Output writers can run in parallel: T025-T027 (independent functions)
- Documentation tasks can run in parallel: T037, T040, T041

## Parallel Execution Examples

### Example 1: Contract Tests (T006-T013)
All contract tests can be written simultaneously since they're independent test cases:

```bash
# Launch T006-T013 together (all in tests/contract/contract_cli.rs):
Task: "Write CT-001 single file conversion test"
Task: "Write CT-002 multiple output formats test"
Task: "Write CT-003 custom output directory test"
Task: "Write CT-004 batch directory processing test"
Task: "Write CT-005 format filtering test"
Task: "Write CT-006 file not found error test"
Task: "Write CT-007 unsupported format error test"
Task: "Write CT-008 PDF with OCR test"
```

### Example 2: Integration Tests (T014-T016)
Independent test files can be written in parallel:

```bash
# Launch T014-T016 together:
Task: "Integration test single file workflow in tests/integration/integration_cli_single.rs"
Task: "Integration test batch conversion in tests/integration/integration_cli_batch.rs"
Task: "Integration test PDF options in tests/integration/integration_cli_pdf_opts.rs"
```

### Example 3: Data Structures (T017-T020)
Independent struct definitions can be created in parallel:

```bash
# Launch T017-T020 together:
Task: "Implement CliArgs with clap derives in src/cli/args.rs"
Task: "Implement InputFormat/OutputFormat enums in src/cli/args.rs"
Task: "Implement ConversionJob in src/cli/converter.rs"
Task: "Implement ConversionResult and BatchProgress in src/cli/converter.rs"
```

### Example 4: Output Writers (T025-T027)
Independent output formatters can be written in parallel:

```bash
# Launch T025-T027 together:
Task: "Implement markdown output writer in src/cli/output.rs"
Task: "Implement JSON output writer in src/cli/output.rs"
Task: "Implement text output writer in src/cli/output.rs"
```

## Task Details

### T001: Add CLI Dependencies

**File**: `Cargo.toml`

**Actions**:
1. Add to `[dependencies]`:
   ```toml
   clap = { version = "4.5", features = ["derive"] }
   indicatif = "0.17"
   anyhow = "1.0"
   ```

2. Add to `[dev-dependencies]`:
   ```toml
   assert_cmd = "2.0"
   predicates = "3.0"
   ```

**Verification**: `cargo check` succeeds

---

### T002: Configure Binary Target

**File**: `Cargo.toml`

**Actions**:
1. Add binary configuration:
   ```toml
   [[bin]]
   name = "docling-rs"
   path = "src/bin/docling-rs.rs"
   ```

**Verification**: `cargo build --bin docling-rs` compiles (even if empty)

---

### T004: Create CLI Module Structure

**File**: `src/cli/mod.rs`

**Actions**:
1. Create module declarations:
   ```rust
   pub mod args;
   pub mod converter;
   pub mod output;
   pub mod progress;

   pub use args::{CliArgs, InputFormat, OutputFormat};
   pub use converter::Converter;
   ```

2. Create empty submodule files:
   - `src/cli/args.rs`
   - `src/cli/converter.rs`
   - `src/cli/output.rs`
   - `src/cli/progress.rs`

**Verification**: `cargo check` succeeds

---

### T005: Create Binary Entry Point

**File**: `src/bin/docling-rs.rs`

**Actions**:
1. Create minimal main():
   ```rust
   fn main() {
       println!("docling-rs CLI - TODO: implement");
       std::process::exit(0);
   }
   ```

**Verification**: `cargo run --bin docling-rs` prints message and exits

---

### T006-T013: Contract Tests

**File**: `tests/contract/contract_cli.rs`

**Actions**: For each CT-001 to CT-008, write a test using `assert_cmd`:

```rust
use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

#[test]
fn ct_001_single_file_conversion() {
    let temp = TempDir::new().unwrap();
    // Create test PDF
    // Run CLI
    let mut cmd = Command::cargo_bin("docling-rs").unwrap();
    cmd.arg("test.pdf");
    cmd.assert().success();

    // Verify output exists
    assert!(temp.path().join("test.md").exists());
}

// Repeat for CT-002 through CT-008
```

**Verification**: Tests compile and fail (no implementation yet)

---

### T017: Implement CliArgs

**File**: `src/cli/args.rs`

**Actions**:
1. Define CliArgs with clap derives (from data-model.md):
   ```rust
   use clap::{Parser, ValueEnum};
   use std::path::PathBuf;

   #[derive(Parser)]
   #[command(name = "docling-rs")]
   #[command(about = "Document conversion tool")]
   pub struct CliArgs {
       #[arg(help = "Input file or directory")]
       pub input: PathBuf,

       #[arg(long, short = 'o')]
       pub output: Option<PathBuf>,

       #[arg(long, value_enum)]
       pub to: Vec<OutputFormat>,

       #[arg(long, value_enum)]
       pub from: Vec<InputFormat>,

       #[arg(long)]
       pub ocr: bool,

       // ... all other flags from data-model.md
   }
   ```

**Verification**: `cargo check` succeeds, argument parsing compiles

---

### T031: Implement main() Orchestration

**File**: `src/bin/docling-rs.rs`

**Actions**:
1. Parse arguments with clap
2. Call Converter logic
3. Handle errors and set exit codes

```rust
use docling_rs::cli::{CliArgs, Converter};
use clap::Parser;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args = CliArgs::parse();

    match run(args) {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("Error: {:#}", e);
            ExitCode::FAILURE
        }
    }
}

fn run(args: CliArgs) -> anyhow::Result<()> {
    let converter = Converter::new(args);
    converter.execute()?;
    Ok(())
}
```

**Verification**: `cargo run --bin docling-rs --help` shows help text

---

## Notes

### TDD Compliance
- **Phase 3.2 MUST complete before Phase 3.3**
- All tests must be written and failing
- Run `cargo test` to verify tests fail
- Only then proceed to implementation

### File Organization
- `src/cli/args.rs`: Argument parsing with clap
- `src/cli/converter.rs`: Conversion orchestration
- `src/cli/output.rs`: Output file generation
- `src/cli/progress.rs`: Progress bar utilities
- `src/bin/docling-rs.rs`: Main entry point

### Testing Strategy
- Contract tests: CLI behavior, exit codes, output files
- Integration tests: End-to-end conversion workflows
- Use `assert_cmd` for CLI testing
- Use `tempfile` for test file management

### Cross-Platform Considerations
- Use `PathBuf` for all file paths
- Test on both macOS and Windows
- Handle line endings appropriately
- Exit codes must be consistent

---

## Validation Checklist

Before considering CLI complete:
- [ ] All 8 contract tests passing (CT-001 to CT-008)
- [ ] All 3 integration tests passing
- [ ] `cargo clippy` clean
- [ ] `cargo fmt --check` passes
- [ ] Help text is comprehensive (`--help`)
- [ ] Version flag works (`--version`)
- [ ] Cross-platform tested (macOS + Windows)
- [ ] README.md updated with CLI usage
- [ ] Quickstart examples work

---

## Task Execution Order

**Sequential Execution** (recommended for solo development):
```
T001-T005 (setup) → T006-T016 (all tests) → T017-T032 (implementation) → T033-T041 (validation)
```

**Parallel Execution** (for team):
```
Phase 1: T001-T005 (sequential, setup)
Phase 2: T006-T016 (parallel, all tests)
Phase 3: T017-T020 (parallel, data structures) → T021-T024 (sequential, core logic) → T025-T028 (parallel, output) → T029-T032 (sequential, main)
Phase 4: T033-T034 (sequential, validation) → T035-T041 (parallel, polish)
```

---

**Total Tasks**: 41 tasks
**Estimated Time**: 8-10 hours for full implementation
**Parallel Opportunities**: 18 tasks can run in parallel (marked with [P])

