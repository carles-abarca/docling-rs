#!/usr/bin/env bash
#
# Testing Script for Original Python Docling Library
#
# This script processes all test documents in tests/documents-test/
# using the original Python docling library for comparison with docling-rs.
#
# Usage: ./scripts/test-cli-original.sh [options]
#
# Options:
#   -h, --help     Show help message
#   -v, --verbose  Show full output (no truncation)
#   -s, --save     Save outputs to scripts/output-original/
#   -c, --clean    Clean output directory before running
#
# Requirements:
#   - Python 3.9+
#   - pip install docling docling-core[chunking]
#
# Supported formats: Markdown, HTML, CSV, DOCX, XLSX, PPTX, PDF
#
# Exit codes:
#   0 = All tests passed
#   1 = Some tests failed
#   2 = Python/dependencies not available
#   3 = Test directory not found
#

set -euo pipefail

# ============================================
# Configuration
# ============================================

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PYTHON_WRAPPER="$REPO_ROOT/scripts/docling_python_wrapper.py"
TEST_DOCS_DIR="$REPO_ROOT/crates/docling-rs/tests/documents-test"
OUTPUT_DIR="$REPO_ROOT/scripts/output-original"

# Supported file extensions
SUPPORTED_EXTENSIONS=("md" "html" "htm" "csv" "docx" "xlsx" "pptx" "pdf")

# Options
VERBOSE=false
SAVE_OUTPUT=false
CLEAN_OUTPUT=false

# Statistics
TOTAL_FILES=0
PASSED_FILES=0
FAILED_FILES=0
SKIPPED_FILES=0
TOTAL_START_TIME=$(date +%s)

# ============================================
# Color Support
# ============================================

if [ -t 1 ] && [ "${TERM:-dumb}" != "dumb" ]; then
    GREEN='\033[0;32m'
    RED='\033[0;31m'
    YELLOW='\033[1;33m'
    BLUE='\033[0;34m'
    CYAN='\033[0;36m'
    BOLD='\033[1m'
    NC='\033[0m'
else
    GREEN='' RED='' YELLOW='' BLUE='' CYAN='' BOLD='' NC=''
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

print_info() {
    echo -e "${CYAN}ℹ${NC} $1"
}

show_help() {
    cat << EOF
Usage: $0 [options]

Testing script for original Python docling library.
Processes all test documents for comparison with docling-rs.

Options:
  -h, --help     Show this help message
  -v, --verbose  Show full output (no truncation)
  -s, --save     Save outputs to scripts/output-original/
  -c, --clean    Clean output directory before running

Supported Formats:
  - Markdown (.md)
  - HTML (.html, .htm)
  - CSV (.csv)
  - DOCX (.docx)
  - XLSX (.xlsx)
  - PPTX (.pptx)
  - PDF (.pdf)

Test Documents Directory: tests/documents-test/
Output Directory: scripts/output-original/

Requirements:
  Python 3.9+ with:
    pip install docling docling-core[chunking]

Exit codes:
  0 - All supported formats processed successfully
  1 - One or more files failed to process
  2 - Python/dependencies not available
  3 - Test directory not found
EOF
}

is_supported_format() {
    local ext="${1##*.}"
    ext=$(echo "$ext" | tr '[:upper:]' '[:lower:]')
    for supported in "${SUPPORTED_EXTENSIONS[@]}"; do
        if [ "$ext" = "$supported" ]; then
            return 0
        fi
    done
    return 1
}

get_format_name() {
    local ext="${1##*.}"
    ext=$(echo "$ext" | tr '[:upper:]' '[:lower:]')
    case "$ext" in
        md) echo "Markdown" ;;
        html|htm) echo "HTML" ;;
        csv) echo "CSV" ;;
        docx) echo "DOCX" ;;
        xlsx) echo "XLSX" ;;
        pptx) echo "PPTX" ;;
        pdf) echo "PDF" ;;
        *) echo "Unknown" ;;
    esac
}

truncate_output() {
    local output="$1"
    local max_lines="${2:-30}"

    if [ "$VERBOSE" = true ]; then
        echo "$output"
    else
        local line_count=$(echo "$output" | wc -l | tr -d ' ')
        echo "$output" | head -n "$max_lines"
        if [ "$line_count" -gt "$max_lines" ]; then
            echo -e "${YELLOW}... (truncated, $line_count total lines)${NC}"
        fi
    fi
}

# ============================================
# Core Functions
# ============================================

check_python_dependencies() {
    echo "Checking Python dependencies..."

    # Check Python
    if ! command -v python3 &> /dev/null; then
        print_error "Python 3 not found!"
        exit 2
    fi

    local python_version=$(python3 --version 2>&1)
    print_info "Python: $python_version"

    # Check docling
    if ! python3 -c "import docling" 2>/dev/null; then
        print_error "docling not installed!"
        echo "Install with: pip install docling docling-core[chunking]"
        exit 2
    fi

    # Check docling-core chunking
    if ! python3 -c "from docling_core.transforms.chunker import HierarchicalChunker" 2>/dev/null; then
        print_error "docling-core[chunking] not installed!"
        echo "Install with: pip install docling-core[chunking]"
        exit 2
    fi

    local docling_version=$(python3 -c "import docling; print(docling.__version__)" 2>/dev/null || echo "unknown")
    print_info "docling version: $docling_version"

    # Check wrapper script
    if [ ! -f "$PYTHON_WRAPPER" ]; then
        print_error "Python wrapper script not found: $PYTHON_WRAPPER"
        exit 2
    fi

    print_success "All dependencies available"
}

verify_test_directory() {
    if [ ! -d "$TEST_DOCS_DIR" ]; then
        print_error "Test directory not found: $TEST_DOCS_DIR"
        exit 3
    fi

    local total_files=$(find "$TEST_DOCS_DIR" -type f -not -name ".DS_Store" -not -name ".*" | wc -l | tr -d ' ')
    local supported_files=0

    for file in "$TEST_DOCS_DIR"/*; do
        [ -f "$file" ] || continue
        local filename=$(basename "$file")
        [[ "$filename" == .* ]] && continue
        if is_supported_format "$file"; then
            supported_files=$((supported_files + 1))
        fi
    done

    echo "Found $total_files files in test directory"
    print_info "$supported_files files have supported formats"
}

setup_output_directory() {
    if [ "$CLEAN_OUTPUT" = true ] && [ -d "$OUTPUT_DIR" ]; then
        rm -rf "$OUTPUT_DIR"
        print_info "Cleaned output directory"
    fi
    mkdir -p "$OUTPUT_DIR"
    print_info "Output directory: $OUTPUT_DIR"
}

test_text_extraction() {
    local file="$1"
    local filename="$2"
    local basename="${filename%.*}"
    local output_file="$OUTPUT_DIR/${basename}.txt"

    echo ""
    echo "Testing: Text extraction"
    local start_time=$(date +%s)

    if python3 "$PYTHON_WRAPPER" "$file" --to text --output-dir "$OUTPUT_DIR" >/dev/null 2>&1; then
        local end_time=$(date +%s)
        local duration=$((end_time - start_time))

        if [ -f "$output_file" ]; then
            local file_size=$(wc -c < "$output_file" | tr -d ' ')
            local line_count=$(wc -l < "$output_file" | tr -d ' ')

            if [ "$file_size" -gt 50 ]; then
                print_success "Text extraction (${duration}s) - ${file_size} bytes, ${line_count} lines"

                echo "[Output preview]"
                head -n 20 "$output_file"
                if [ "$line_count" -gt 20 ]; then
                    echo -e "${YELLOW}... (truncated, $line_count total lines)${NC}"
                fi
                return 0
            else
                print_error "Text extraction produced minimal output (${file_size} bytes)"
                cat "$output_file"
                return 1
            fi
        else
            print_error "Text extraction failed - no output file created"
            return 1
        fi
    else
        local end_time=$(date +%s)
        local duration=$((end_time - start_time))
        print_error "Text extraction failed (${duration}s)"
        return 1
    fi
}

test_json_export() {
    local file="$1"
    local filename="$2"
    local basename="${filename%.*}"
    local output_file="$OUTPUT_DIR/${basename}.json"

    echo ""
    echo "Testing: JSON export"
    local start_time=$(date +%s)

    if python3 "$PYTHON_WRAPPER" "$file" --to json --output-dir "$OUTPUT_DIR" >/dev/null 2>&1; then
        local end_time=$(date +%s)
        local duration=$((end_time - start_time))

        if [ -f "$output_file" ]; then
            local file_size=$(wc -c < "$output_file" | tr -d ' ')

            # Validate JSON structure (starts with {)
            if head -c 1 "$output_file" | grep -q '{'; then
                if [ "$file_size" -gt 100 ]; then
                    print_success "JSON export (${duration}s) - ${file_size} bytes"
                    print_info "Valid JSON structure"

                    echo "[Output preview]"
                    head -n 15 "$output_file"
                    return 0
                else
                    print_error "JSON export produced minimal output (${file_size} bytes)"
                    cat "$output_file"
                    return 1
                fi
            else
                print_error "JSON export produced invalid JSON"
                head -c 100 "$output_file"
                return 1
            fi
        else
            print_error "JSON export failed - no output file created"
            return 1
        fi
    else
        local end_time=$(date +%s)
        local duration=$((end_time - start_time))
        print_error "JSON export failed (${duration}s)"
        return 1
    fi
}

test_chunking_hierarchical() {
    local file="$1"
    local filename="$2"
    local basename="${filename%.*}"
    local chunk_output_dir="$OUTPUT_DIR/chunked-hierarchical"
    local output_file="$chunk_output_dir/${basename}.md"

    echo ""
    echo "Testing: Chunking (hierarchical strategy)"
    local start_time=$(date +%s)

    mkdir -p "$chunk_output_dir"

    if python3 "$PYTHON_WRAPPER" "$file" --chunk --chunk-strategy hierarchical --to markdown --output-dir "$chunk_output_dir" >/dev/null 2>&1; then
        local end_time=$(date +%s)
        local duration=$((end_time - start_time))

        if [ -f "$output_file" ]; then
            local file_size=$(wc -c < "$output_file" | tr -d ' ')
            local chunk_count=$(grep -c "^## Chunk" "$output_file" 2>/dev/null || echo "0")

            if [ "$file_size" -gt 50 ]; then
                print_success "Hierarchical chunking (${duration}s) - ${chunk_count} chunks, ${file_size} bytes"

                echo "[Output preview]"
                head -n 15 "$output_file"
                return 0
            else
                print_skip "Hierarchical chunking produced minimal output (${file_size} bytes)"
                return 2
            fi
        else
            print_skip "Hierarchical chunking - no output file created"
            return 2
        fi
    else
        local end_time=$(date +%s)
        local duration=$((end_time - start_time))
        print_skip "Hierarchical chunking skipped/failed (${duration}s)"
        return 2
    fi
}

test_chunking_hybrid() {
    local file="$1"
    local filename="$2"
    local basename="${filename%.*}"
    local chunk_output_dir="$OUTPUT_DIR/chunked-hybrid"
    local output_file="$chunk_output_dir/${basename}.md"

    echo ""
    echo "Testing: Chunking (hybrid strategy, max-tokens=256)"
    local start_time=$(date +%s)

    mkdir -p "$chunk_output_dir"

    if python3 "$PYTHON_WRAPPER" "$file" --chunk --chunk-strategy hybrid --chunk-max-tokens 256 --to markdown --output-dir "$chunk_output_dir" >/dev/null 2>&1; then
        local end_time=$(date +%s)
        local duration=$((end_time - start_time))

        if [ -f "$output_file" ]; then
            local file_size=$(wc -c < "$output_file" | tr -d ' ')
            local chunk_count=$(grep -c "^## Chunk" "$output_file" 2>/dev/null || echo "0")

            if [ "$file_size" -gt 50 ]; then
                print_success "Hybrid chunking (${duration}s) - ${chunk_count} chunks, ${file_size} bytes"

                echo "[Output preview]"
                head -n 15 "$output_file"
                return 0
            else
                print_skip "Hybrid chunking produced minimal output (${file_size} bytes)"
                return 2
            fi
        else
            print_skip "Hybrid chunking - no output file created"
            return 2
        fi
    else
        local end_time=$(date +%s)
        local duration=$((end_time - start_time))
        print_skip "Hybrid chunking skipped/failed (${duration}s)"
        return 2
    fi
}

test_chunking() {
    local file="$1"
    local filename="$2"

    # Test both chunking strategies
    test_chunking_hierarchical "$file" "$filename"
    test_chunking_hybrid "$file" "$filename"
}

process_file() {
    local file="$1"
    local filename=$(basename "$file")
    local index="$2"
    local total="$3"

    # Skip hidden files
    if [[ "$filename" == .* ]]; then
        return 2
    fi

    # Check if format is supported
    if ! is_supported_format "$file"; then
        return 2
    fi

    local format_name=$(get_format_name "$filename")

    echo ""
    print_header "[$index/$total] $filename"
    print_info "Format: $format_name"
    print_separator

    local test_passed=true

    # Test 1: Text extraction
    if ! test_text_extraction "$file" "$filename"; then
        test_passed=false
    fi

    # Test 2: JSON export
    if ! test_json_export "$file" "$filename"; then
        test_passed=false
    fi

    # Test 3: Chunking
    test_chunking "$file" "$filename"  # Chunking failure doesn't fail the whole test

    if [ "$test_passed" = true ]; then
        return 0
    else
        return 1
    fi
}

print_summary() {
    local total_end_time=$(date +%s)
    local total_duration=$((total_end_time - TOTAL_START_TIME))

    echo ""
    print_header "TEST SUMMARY (Python docling)"
    echo ""
    echo "Total files tested: $TOTAL_FILES"
    print_success "Passed: $PASSED_FILES"
    print_error "Failed: $FAILED_FILES"
    print_skip "Skipped (unsupported): $SKIPPED_FILES"
    echo ""
    echo "Total duration: ${total_duration}s"

    if [ "$SAVE_OUTPUT" = true ]; then
        local output_count=$(find "$OUTPUT_DIR" -type f 2>/dev/null | wc -l | tr -d ' ')
        print_info "Output files saved: $output_count (in $OUTPUT_DIR)"
    fi

    print_separator

    if [ "$FAILED_FILES" -gt 0 ]; then
        echo -e "${RED}Some tests failed!${NC}"
        return 1
    else
        echo -e "${GREEN}All tests passed!${NC}"
        return 0
    fi
}

# ============================================
# Main
# ============================================

main() {
    # Parse arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            -h|--help)
                show_help
                exit 0
                ;;
            -v|--verbose)
                VERBOSE=true
                shift
                ;;
            -s|--save)
                SAVE_OUTPUT=true
                shift
                ;;
            -c|--clean)
                CLEAN_OUTPUT=true
                shift
                ;;
            *)
                echo "Unknown option: $1"
                show_help
                exit 1
                ;;
        esac
    done

    print_header "Python docling Library Testing"
    echo ""
    echo "Comparing with original Python implementation"
    echo "Supported formats: ${SUPPORTED_EXTENSIONS[*]}"
    echo ""

    check_python_dependencies
    echo ""

    verify_test_directory
    echo ""

    setup_output_directory

    # Process each file
    local files=("$TEST_DOCS_DIR"/*)
    local total=${#files[@]}
    local index=1

    for file in "${files[@]}"; do
        [ -f "$file" ] || continue

        local filename=$(basename "$file")

        # Skip hidden files
        if [[ "$filename" == .* ]]; then
            continue
        fi

        # Check if supported format
        if ! is_supported_format "$file"; then
            SKIPPED_FILES=$((SKIPPED_FILES + 1))
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

    # Print summary
    if print_summary; then
        exit 0
    else
        exit 1
    fi
}

main "$@"
