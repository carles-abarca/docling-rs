# Tasks: CLI Manual Testing Script

**Input**: Design documents from `/Users/carlesabarca/MyProjects/docling-rs/specs/006-cli-manual-testing/`
**Prerequisites**: plan.md ✅, research.md ✅, quickstart.md ✅

## Execution Flow (main)
```
1. Load plan.md from feature directory ✅
   → Tech stack: Bash shell script
   → Dependencies: CLI release binary
   → Structure: scripts/ directory
2. Load optional design documents ✅
   → research.md: 7 technology decisions extracted
   → quickstart.md: Usage scenarios extracted
   → No data-model.md (N/A for shell script)
   → No contracts/ (testing infrastructure, not tested itself)
3. Generate tasks by category ✅
   → Setup: script directory, basic structure
   → Core: build logic, processing loop, output formatting
   → Polish: error handling, documentation, testing
4. Apply task rules ✅
   → Single file (shell script) = all sequential (no [P])
   → No TDD for testing infrastructure itself
5. Number tasks sequentially (T001-T013) ✅
6. Generate dependency graph ✅
7. Skip parallel examples (single file, sequential) ✅
8. Validate task completeness ✅
9. Return: SUCCESS (tasks ready for execution) ✅
```

## Format: `[ID] Description`
- **No [P] markers**: All tasks modify same file (`scripts/test-cli-manual.sh`)
- Include exact file paths and specific implementation details

## Path Conventions
- **Single project**: Repository root `/Users/carlesabarca/MyProjects/docling-rs/`
- **Script location**: `scripts/test-cli-manual.sh` (NEW)
- **Test documents**: `tests/documents-test/` (already exists with 14 files)
- **CLI binary**: `target/release/docling-rs`

## Phase 1: Setup & Structure

### T001: ✅ Create scripts directory and basic script file
**File**: `scripts/test-cli-manual.sh`
**Action**:
- Create `scripts/` directory if it doesn't exist
- Create `scripts/test-cli-manual.sh` with shebang and basic structure:
  ```bash
  #!/usr/bin/env bash
  # Manual testing script for docling-rs CLI
  # Processes all test documents and displays results

  set -euo pipefail  # Exit on error, undefined vars, pipe failures

  # Script will be filled in subsequent tasks
  ```
- Make script executable: `chmod +x scripts/test-cli-manual.sh`
- Verify: `ls -la scripts/test-cli-manual.sh` shows executable permissions

**Dependencies**: None
**Estimated time**: 2 minutes

---

### T002: ✅ Add color support and helper functions
**File**: `scripts/test-cli-manual.sh`
**Action**:
- Add ANSI color code definitions with terminal detection:
  ```bash
  # Color definitions (auto-detect terminal support)
  if [ -t 1 ] && [ "$TERM" != "dumb" ]; then
      GREEN='\033[0;32m'
      RED='\033[0;31m'
      YELLOW='\033[1;33m'
      BLUE='\033[0;34m'
      BOLD='\033[1m'
      NC='\033[0m' # No Color
  else
      GREEN='' RED='' YELLOW='' BLUE='' BOLD='' NC=''
  fi
  ```
- Add helper functions:
  ```bash
  print_header() {
      echo -e "${BOLD}========================================${NC}"
      echo -e "${BOLD}$1${NC}"
      echo -e "${BOLD}========================================${NC}"
  }

  print_separator() {
      echo -e "${BOLD}------------------------------------------${NC}"
  }

  print_success() {
      echo -e "${GREEN}✓${NC} $1"
  }

  print_error() {
      echo -e "${RED}✗${NC} $1"
  }

  print_skip() {
      echo -e "${YELLOW}⊘${NC} $1"
  }
  ```

**Dependencies**: T001
**Estimated time**: 5 minutes

---

## Phase 2: Core Logic

### T003: ✅ Add binary check and build logic
**File**: `scripts/test-cli-manual.sh`
**Action**:
- Add constants for paths:
  ```bash
  REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
  CLI_BINARY="$REPO_ROOT/target/release/docling-rs"
  TEST_DOCS_DIR="$REPO_ROOT/tests/documents-test"
  ```
- Add binary check function:
  ```bash
  check_and_build_binary() {
      if [ ! -f "$CLI_BINARY" ]; then
          echo "Release binary not found. Building..."
          cd "$REPO_ROOT"
          cargo build --release || {
              echo "Build failed!"
              exit 2
          }
      else
          echo "Using existing release binary: $CLI_BINARY"
          echo "Binary last modified: $(stat -f "%Sm" "$CLI_BINARY" 2>/dev/null || stat -c "%y" "$CLI_BINARY" 2>/dev/null)"
      fi
  }
  ```
- Add main initialization:
  ```bash
  main() {
      print_header "Testing CLI with Real Documents"
      echo ""

      check_and_build_binary
      echo ""
  }

  main "$@"
  ```

**Dependencies**: T002
**Estimated time**: 10 minutes

---

### T004: ✅ Add test directory verification
**File**: `scripts/test-cli-manual.sh`
**Action**:
- Add directory check function:
  ```bash
  verify_test_directory() {
      if [ ! -d "$TEST_DOCS_DIR" ]; then
          print_error "Test directory not found: $TEST_DOCS_DIR"
          exit 3
      fi

      local file_count=$(find "$TEST_DOCS_DIR" -type f | wc -l | tr -d ' ')
      echo "Found $file_count test documents in $TEST_DOCS_DIR"
  }
  ```
- Call from main() after check_and_build_binary()

**Dependencies**: T003
**Estimated time**: 5 minutes

---

### T005: ✅ Implement document processing loop
**File**: `scripts/test-cli-manual.sh`
**Action**:
- Add file processing function:
  ```bash
  process_file() {
      local file="$1"
      local filename=$(basename "$file")
      local index="$2"
      local total="$3"

      echo ""
      print_header "[$index/$total] Processing: $filename"
      print_separator

      # Detect format from extension
      local ext="${filename##*.}"
      echo "Format: $ext"
      echo ""

      # Return values for statistics
      echo "$filename"  # Return filename for tracking
  }
  ```
- Add main loop in main():
  ```bash
  # Process each file
  local files=("$TEST_DOCS_DIR"/*)
  local total=${#files[@]}
  local index=1

  for file in "${files[@]}"; do
      [ -f "$file" ] || continue
      process_file "$file" "$index" "$total"
      index=$((index + 1))
  done
  ```

**Dependencies**: T004
**Estimated time**: 10 minutes

---

### T006: ✅ Add text extraction invocation
**File**: `scripts/test-cli-manual.sh`
**Action**:
- Modify process_file() to invoke CLI for text extraction:
  ```bash
  process_file() {
      local file="$1"
      local filename=$(basename "$file")
      local index="$2"
      local total="$3"

      # ... existing code ...

      # Text extraction
      echo "Extracting text..."
      local start_time=$(date +%s)

      if output=$("$CLI_BINARY" "$file" --to text 2>&1); then
          local end_time=$(date +%s)
          local duration=$((end_time - start_time))
          print_success "Success (${duration}s)"

          # Show output with truncation
          echo ""
          echo "[Output - first 50 lines]"
          echo "$output" | head -n 50

          local line_count=$(echo "$output" | wc -l | tr -d ' ')
          if [ "$line_count" -gt 50 ]; then
              echo -e "${YELLOW}... (truncated, $line_count total lines)${NC}"
          fi

          return 0  # Success
      else
          local end_time=$(date +%s)
          local duration=$((end_time - start_time))
          print_error "Failed (${duration}s)"
          echo "$output" | head -n 20  # Show error

          return 1  # Failure
      fi
  }
  ```

**Dependencies**: T005
**Estimated time**: 15 minutes

---

### T007: ✅ Add chunking invocation
**File**: `scripts/test-cli-manual.sh`
**Action**:
- Extend process_file() to add chunking test:
  ```bash
  # After text extraction section, add:

  echo ""
  echo "Testing chunking..."
  local chunk_start=$(date +%s)

  if chunk_output=$("$CLI_BINARY" "$file" --chunk --chunk-size 1000 2>&1); then
      local chunk_end=$(date +%s)
      local chunk_duration=$((chunk_end - chunk_start))
      print_success "Success (${chunk_duration}s)"

      echo ""
      echo "[Chunks - first 50 lines]"
      echo "$chunk_output" | head -n 50

      local chunk_lines=$(echo "$chunk_output" | wc -l | tr -d ' ')
      if [ "$chunk_lines" -gt 50 ]; then
          echo -e "${YELLOW}... (truncated, $chunk_lines total lines)${NC}"
      fi

      return 0
  else
      local chunk_end=$(date +%s)
      local chunk_duration=$((chunk_end - chunk_start))
      print_skip "Skipped or failed (${chunk_duration}s)"

      return 2  # Skip/unsupported
  fi
  ```

**Dependencies**: T006
**Estimated time**: 10 minutes

---

## Phase 3: Statistics & Error Handling

### T008: ✅ Add result tracking and statistics
**File**: `scripts/test-cli-manual.sh`
**Action**:
- Add global counters at top of script:
  ```bash
  # Statistics
  TOTAL_FILES=0
  PASSED_FILES=0
  FAILED_FILES=0
  SKIPPED_FILES=0
  TOTAL_START_TIME=$(date +%s)
  ```
- Modify main loop to track results:
  ```bash
  for file in "${files[@]}"; do
      [ -f "$file" ] || continue
      TOTAL_FILES=$((TOTAL_FILES + 1))

      if process_file "$file" "$index" "$total"; then
          PASSED_FILES=$((PASSED_FILES + 1))
      else
          result=$?
          if [ "$result" -eq 2 ]; then
              SKIPPED_FILES=$((SKIPPED_FILES + 1))
          else
              FAILED_FILES=$((FAILED_FILES + 1))
          fi
      fi

      index=$((index + 1))
  done
  ```
- Add summary at end of main():
  ```bash
  # Summary
  TOTAL_END_TIME=$(date +%s)
  TOTAL_DURATION=$((TOTAL_END_TIME - TOTAL_START_TIME))

  echo ""
  print_header "SUMMARY"
  echo "Total files: $TOTAL_FILES"
  print_success "Passed: $PASSED_FILES"
  print_error "Failed: $FAILED_FILES"
  print_skip "Skipped: $SKIPPED_FILES"
  echo "Total duration: ${TOTAL_DURATION}s"
  print_separator

  # Exit code based on failures
  if [ "$FAILED_FILES" -gt 0 ]; then
      exit 1
  else
      exit 0
  fi
  ```

**Dependencies**: T007
**Estimated time**: 15 minutes

---

### T009: ✅ Add graceful error handling for unsupported formats
**File**: `scripts/test-cli-manual.sh`
**Action**:
- Modify process_file() to distinguish between errors and unsupported formats:
  ```bash
  # In text extraction section:
  if output=$("$CLI_BINARY" "$file" --to text 2>&1); then
      # ... success case ...
  else
      local end_time=$(date +%s)
      local duration=$((end_time - start_time))

      # Check if unsupported format
      if echo "$output" | grep -qi "unsupported\|not supported"; then
          print_skip "Unsupported format (${duration}s)"
          return 2  # Skip
      else
          print_error "Failed (${duration}s)"
          echo "$output" | head -n 20
          return 1  # Error
      fi
  fi
  ```

**Dependencies**: T008
**Estimated time**: 10 minutes

---

## Phase 4: Documentation & Polish

### T010: ✅ Add usage instructions and help text
**File**: `scripts/test-cli-manual.sh`
**Action**:
- Add help function at top:
  ```bash
  show_help() {
      cat << EOF
  Usage: $0 [options]

  Manual testing script for docling-rs CLI.
  Processes all test documents in tests/documents-test/ and displays results.

  Options:
    -h, --help     Show this help message

  Requirements:
    - Rust toolchain (cargo command)
    - Test documents in tests/documents-test/

  The script will:
    1. Check for release binary (build if needed)
    2. Process each document for text extraction
    3. Test chunking on each document
    4. Display results with color-coded status
    5. Show summary statistics

  Exit codes:
    0 - All supported formats processed successfully
    1 - One or more files failed to process
    2 - Binary build failed
    3 - Test directory not found
  EOF
  }
  ```
- Add argument parsing in main():
  ```bash
  # Parse arguments
  for arg in "$@"; do
      case $arg in
          -h|--help)
              show_help
              exit 0
              ;;
          *)
              echo "Unknown option: $arg"
              show_help
              exit 1
              ;;
      esac
  done
  ```

**Dependencies**: T009
**Estimated time**: 10 minutes

---

### T011: ✅ Add inline comments and documentation
**File**: `scripts/test-cli-manual.sh`
**Action**:
- Add file header comment:
  ```bash
  #!/usr/bin/env bash
  #
  # Manual Testing Script for docling-rs CLI
  #
  # This script processes all test documents in tests/documents-test/
  # and displays the results of text extraction and chunking.
  #
  # Usage: ./scripts/test-cli-manual.sh
  #
  # Requirements:
  #   - Cargo (Rust toolchain)
  #   - Test documents in tests/documents-test/
  #
  # Exit codes:
  #   0 = All tests passed
  #   1 = Some tests failed
  #   2 = Build failed
  #   3 = Test directory not found
  #
  ```
- Add section comments throughout script:
  ```bash
  # ============================================
  # Configuration
  # ============================================

  # ============================================
  # Color Support
  # ============================================

  # ============================================
  # Helper Functions
  # ============================================

  # etc.
  ```

**Dependencies**: T010
**Estimated time**: 10 minutes

---

### T012: ✅ Test script with all document types
**File**: `scripts/test-cli-manual.sh`
**Action**:
- Run the script and verify it processes all 14 test documents:
  ```bash
  ./scripts/test-cli-manual.sh
  ```
- Verify output shows:
  - Header and initialization messages
  - Processing status for each file
  - Color-coded results (green ✓, red ✗, yellow ⊘)
  - Truncated output for large documents
  - Summary statistics
  - Correct exit code (0 if no failures)
- Test help flag:
  ```bash
  ./scripts/test-cli-manual.sh --help
  ```
- Test error cases:
  - Missing binary (temporarily rename target/release/docling-rs)
  - Missing test directory (verify error message)

**Dependencies**: T011
**Estimated time**: 15 minutes

---

### T013: ✅ Update project documentation
**File**: `README.md` or create `scripts/README.md`
**Action**:
- Add section to project README.md about manual testing:
  ```markdown
  ## Manual Testing

  To manually test the CLI with real documents:

  ```bash
  ./scripts/test-cli-manual.sh
  ```

  This script processes all documents in `tests/documents-test/` and displays:
  - Text extraction results
  - Chunking results
  - Processing times
  - Summary statistics

  See [quickstart guide](specs/006-cli-manual-testing/quickstart.md) for details.
  ```
- Or create `scripts/README.md` with:
  - Purpose of scripts directory
  - Description of test-cli-manual.sh
  - Link to quickstart guide

**Dependencies**: T012
**Estimated time**: 10 minutes

---

## Dependencies

```
T001 (create script)
  ↓
T002 (colors & helpers)
  ↓
T003 (binary check)
  ↓
T004 (directory verification)
  ↓
T005 (processing loop)
  ↓
T006 (text extraction)
  ↓
T007 (chunking)
  ↓
T008 (statistics)
  ↓
T009 (error handling)
  ↓
T010 (help text)
  ↓
T011 (documentation)
  ↓
T012 (testing)
  ↓
T013 (README update)
```

**All tasks are sequential** - single file modification (`scripts/test-cli-manual.sh`), each builds on previous

## Parallel Execution

**Not applicable** - All tasks modify the same shell script file and must be executed sequentially.

## Total Estimates

- **Setup (T001-T002)**: ~7 minutes
- **Core Logic (T003-T007)**: ~50 minutes
- **Statistics & Errors (T008-T009)**: ~25 minutes
- **Documentation & Testing (T010-T013)**: ~45 minutes

**Total estimated time**: ~2 hours

## Notes

- **No TDD phase**: This is testing infrastructure itself, exempt from TDD per constitution
- **Single file**: All tasks modify `scripts/test-cli-manual.sh` sequentially
- **Manual verification**: T012 requires running script and verifying output manually
- **Cross-platform**: Test on macOS (primary) and Linux if available
- **Iterative testing**: Can test incrementally after T006, T007, etc.

## Validation Checklist

Before marking complete:
- [ ] Script is executable (`chmod +x`)
- [ ] Runs without errors on all test documents
- [ ] Colors display correctly in terminal
- [ ] Gracefully handles unsupported formats
- [ ] Summary statistics match actual results
- [ ] Help text displays with --help flag
- [ ] Exit codes correct (0 on success, 1 on failure)
- [ ] Documentation updated (README.md)
- [ ] Script follows POSIX best practices (shellcheck clean)

---

**Status**: Ready for implementation | **Total Tasks**: 13 | **Estimated Duration**: ~2 hours
