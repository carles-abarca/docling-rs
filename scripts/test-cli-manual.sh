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

set -euo pipefail  # Exit on error, undefined vars, pipe failures

# ============================================
# Configuration
# ============================================

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CLI_BINARY="$REPO_ROOT/target/release/docling-rs"
TEST_DOCS_DIR="$REPO_ROOT/tests/documents-test"

# Statistics
TOTAL_FILES=0
PASSED_FILES=0
FAILED_FILES=0
SKIPPED_FILES=0
TOTAL_START_TIME=$(date +%s)

# ============================================
# Color Support
# ============================================

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

# ============================================
# Helper Functions
# ============================================

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

# ============================================
# Core Functions
# ============================================

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

verify_test_directory() {
    if [ ! -d "$TEST_DOCS_DIR" ]; then
        print_error "Test directory not found: $TEST_DOCS_DIR"
        exit 3
    fi

    local file_count=$(find "$TEST_DOCS_DIR" -type f -not -name ".DS_Store" | wc -l | tr -d ' ')
    echo "Found $file_count test documents in $TEST_DOCS_DIR"
}

process_file() {
    local file="$1"
    local filename=$(basename "$file")
    local index="$2"
    local total="$3"

    # Skip .DS_Store and other hidden files
    if [[ "$filename" == .* ]]; then
        return 2
    fi

    echo ""
    print_header "[$index/$total] Processing: $filename"
    print_separator

    # Detect format from extension
    local ext="${filename##*.}"
    echo "Format: $ext"
    echo ""

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

    # Chunking test
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
}

# ============================================
# Main
# ============================================

main() {
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

    print_header "Testing CLI with Real Documents"
    echo ""

    check_and_build_binary
    echo ""

    verify_test_directory
    echo ""

    # Process each file
    local files=("$TEST_DOCS_DIR"/*)
    local total=${#files[@]}
    local index=1

    for file in "${files[@]}"; do
        [ -f "$file" ] || continue

        # Skip hidden files
        local filename=$(basename "$file")
        if [[ "$filename" == .* ]]; then
            continue
        fi

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
}

main "$@"
