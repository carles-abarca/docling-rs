#!/bin/bash

# TecGPT Desktop - Test Environment Setup Script
# This script sets up the testing environment and validates the TecGPT Desktop application

set -e  # Exit on any error

# Script configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"
TEST_DIR="$SCRIPT_DIR"
APP_DIR="$PROJECT_ROOT/apps/desktop"
LOG_FILE="$TEST_DIR/test_results.log"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Logging function
log() {
    echo -e "${BLUE}[$(date +'%Y-%m-%d %H:%M:%S')]${NC} $1" | tee -a "$LOG_FILE"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1" | tee -a "$LOG_FILE"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1" | tee -a "$LOG_FILE"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1" | tee -a "$LOG_FILE"
}

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to validate file exists and is readable
validate_file() {
    local file_path="$1"
    local file_desc="$2"

    if [[ -f "$file_path" && -r "$file_path" ]]; then
        log_success "$file_desc found and readable: $file_path"
        return 0
    else
        log_error "$file_desc not found or not readable: $file_path"
        return 1
    fi
}

# Function to check system requirements
check_system_requirements() {
    log "Checking system requirements..."

    # Check operating system
    OS="$(uname -s)"
    case "$OS" in
        Darwin*)
            log_success "Operating System: macOS"
            ;;
        Linux*)
            log_success "Operating System: Linux"
            ;;
        CYGWIN*|MINGW*|MSYS*)
            log_success "Operating System: Windows"
            ;;
        *)
            log_warning "Unknown operating system: $OS"
            ;;
    esac

    # Check available memory
    if command_exists free; then
        TOTAL_MEM=$(free -m | awk 'NR==2{printf "%.0f", $2}')
        if [[ $TOTAL_MEM -ge 8192 ]]; then
            log_success "Memory: ${TOTAL_MEM}MB (meets 8GB minimum)"
        else
            log_warning "Memory: ${TOTAL_MEM}MB (below 8GB recommendation)"
        fi
    elif [[ "$OS" == "Darwin" ]]; then
        TOTAL_MEM=$(sysctl -n hw.memsize | awk '{printf "%.0f", $1/1024/1024}')
        if [[ $TOTAL_MEM -ge 8192 ]]; then
            log_success "Memory: ${TOTAL_MEM}MB (meets 8GB minimum)"
        else
            log_warning "Memory: ${TOTAL_MEM}MB (below 8GB recommendation)"
        fi
    fi

    # Check disk space
    AVAILABLE_SPACE=$(df -h "$TEST_DIR" | awk 'NR==2 {print $4}')
    log_success "Available disk space: $AVAILABLE_SPACE"
}

# Function to check required tools
check_tools() {
    log "Checking required development tools..."

    # Check Node.js
    if command_exists node; then
        NODE_VERSION=$(node --version)
        log_success "Node.js: $NODE_VERSION"
    else
        log_error "Node.js not found. Please install Node.js 18+"
        return 1
    fi

    # Check npm
    if command_exists npm; then
        NPM_VERSION=$(npm --version)
        log_success "npm: $NPM_VERSION"
    else
        log_error "npm not found"
        return 1
    fi

    # Check Rust
    if command_exists rustc; then
        RUST_VERSION=$(rustc --version)
        log_success "Rust: $RUST_VERSION"
    else
        log_error "Rust not found. Please install Rust"
        return 1
    fi

    # Check Cargo
    if command_exists cargo; then
        CARGO_VERSION=$(cargo --version)
        log_success "Cargo: $CARGO_VERSION"
    else
        log_error "Cargo not found"
        return 1
    fi

    # Check Tauri CLI
    if command_exists cargo && cargo tauri --version >/dev/null 2>&1; then
        TAURI_VERSION=$(cargo tauri --version 2>/dev/null || echo "Unknown")
        log_success "Tauri CLI: $TAURI_VERSION"
    else
        log_warning "Tauri CLI not found. Install with: cargo install tauri-cli"
    fi
}

# Function to validate test documents
validate_test_documents() {
    log "Validating test documents..."

    local test_files=(
        "test_document.txt"
        "test_readme.md"
        "test_code.py"
        "test_data.json"
        "test_config.yaml"
        "test_script.sh"
        "test_notes.txt"
    )

    local valid_count=0
    local total_size=0

    for file in "${test_files[@]}"; do
        local file_path="$TEST_DIR/$file"
        if validate_file "$file_path" "$file"; then
            ((valid_count++))
            if [[ "$OS" == "Darwin" ]]; then
                local size=$(stat -f%z "$file_path" 2>/dev/null || echo 0)
            else
                local size=$(stat -c%s "$file_path" 2>/dev/null || echo 0)
            fi
            total_size=$((total_size + size))
            log "  Size: $(numfmt --to=iec $size)"
        fi
    done

    log_success "Valid test documents: $valid_count/${#test_files[@]}"
    log_success "Total test data size: $(numfmt --to=iec $total_size)"
}

# Function to check project structure
check_project_structure() {
    log "Checking project structure..."

    # Check main directories
    local required_dirs=(
        "$APP_DIR/src"
        "$APP_DIR/src-tauri"
        "$APP_DIR/src-tauri/src"
    )

    for dir in "${required_dirs[@]}"; do
        if [[ -d "$dir" ]]; then
            log_success "Directory exists: $dir"
        else
            log_error "Directory missing: $dir"
            return 1
        fi
    done

    # Check important files
    local required_files=(
        "$APP_DIR/package.json"
        "$APP_DIR/src-tauri/Cargo.toml"
        "$APP_DIR/src-tauri/tauri.conf.json"
    )

    for file in "${required_files[@]}"; do
        validate_file "$file" "$(basename "$file")"
    done
}

# Function to run dependency checks
check_dependencies() {
    log "Checking project dependencies..."

    # Check if node_modules exists
    if [[ -d "$APP_DIR/node_modules" ]]; then
        log_success "Frontend dependencies installed"
    else
        log_warning "Frontend dependencies not installed. Run: npm install"
    fi

    # Check Rust dependencies
    if [[ -f "$APP_DIR/src-tauri/Cargo.lock" ]]; then
        log_success "Rust dependencies resolved"
    else
        log_warning "Rust dependencies not resolved. Run: cargo check"
    fi
}

# Function to perform basic syntax validation
validate_syntax() {
    log "Performing syntax validation on test files..."

    # Validate Python syntax
    if command_exists python3; then
        if python3 -m py_compile "$TEST_DIR/test_code.py" 2>/dev/null; then
            log_success "Python file syntax is valid"
        else
            log_error "Python file has syntax errors"
        fi
    fi

    # Validate JSON syntax
    if command_exists jq; then
        if jq empty "$TEST_DIR/test_data.json" >/dev/null 2>&1; then
            log_success "JSON file syntax is valid"
        else
            log_error "JSON file has syntax errors"
        fi
    elif command_exists python3; then
        if python3 -c "import json; json.load(open('$TEST_DIR/test_data.json'))" 2>/dev/null; then
            log_success "JSON file syntax is valid"
        else
            log_error "JSON file has syntax errors"
        fi
    fi

    # Validate YAML syntax
    if command_exists python3; then
        if python3 -c "import yaml; yaml.safe_load(open('$TEST_DIR/test_config.yaml'))" 2>/dev/null; then
            log_success "YAML file syntax is valid"
        else
            log_warning "YAML file syntax validation failed (PyYAML may not be installed)"
        fi
    fi

    # Validate shell script syntax
    if bash -n "$TEST_DIR/test_script.sh"; then
        log_success "Shell script syntax is valid"
    else
        log_error "Shell script has syntax errors"
    fi
}

# Function to generate test summary
generate_summary() {
    log "Generating test summary..."

    local total_files=$(find "$TEST_DIR" -type f -name "test_*" | wc -l)
    local total_size=$(find "$TEST_DIR" -type f -name "test_*" -exec stat -f%z {} + 2>/dev/null | awk '{sum+=$1} END {print sum}' || echo "0")

    if [[ "$OS" != "Darwin" ]]; then
        total_size=$(find "$TEST_DIR" -type f -name "test_*" -exec stat -c%s {} + 2>/dev/null | awk '{sum+=$1} END {print sum}' || echo "0")
    fi

    cat << EOF | tee -a "$LOG_FILE"

========================================
TecGPT Desktop Test Environment Summary
========================================
Test Directory: $TEST_DIR
Total Test Files: $total_files
Total Size: $(numfmt --to=iec $total_size)
Log File: $LOG_FILE

Test Files Created:
- test_document.txt   (AI and technology content)
- test_readme.md      (Markdown documentation)
- test_code.py        (Python source code)
- test_data.json      (Structured JSON data)
- test_config.yaml    (YAML configuration)
- test_script.sh      (Shell script)
- test_notes.txt      (Machine learning notes)

Next Steps:
1. Run 'npm install' in $APP_DIR
2. Start development server: 'npm run tauri dev'
3. Index the test documents folder in TecGPT
4. Test search functionality with various queries
5. Validate embeddings generation
6. Test AI chat with document context

========================================
EOF
}

# Function to create a simple test runner
create_test_runner() {
    local runner_file="$TEST_DIR/run_tests.sh"

    cat << 'EOF' > "$runner_file"
#!/bin/bash

# Simple test runner for TecGPT Desktop
# This script provides basic testing functionality

TEST_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "TecGPT Desktop Test Runner"
echo "=========================="
echo "Test directory: $TEST_DIR"
echo ""

# Count files
echo "Test files available:"
find "$TEST_DIR" -name "test_*" -type f | while read -r file; do
    echo "  - $(basename "$file")"
done

echo ""
echo "To test TecGPT Desktop:"
echo "1. Start the application in development mode"
echo "2. Index this directory: $TEST_DIR"
echo "3. Try these search queries:"
echo "   - 'artificial intelligence'"
echo "   - 'python function'"
echo "   - 'database configuration'"
echo "   - 'machine learning'"
echo "   - 'embeddings vector'"
EOF

    chmod +x "$runner_file"
    log_success "Created test runner: $runner_file"
}

# Main execution
main() {
    log "Starting TecGPT Desktop test environment validation..."
    echo "Log file: $LOG_FILE"
    echo ""

    # Initialize log file
    echo "TecGPT Desktop Test Environment Validation" > "$LOG_FILE"
    echo "Started: $(date)" >> "$LOG_FILE"
    echo "==========================================" >> "$LOG_FILE"

    # Run all checks
    check_system_requirements
    echo ""

    check_tools
    echo ""

    check_project_structure
    echo ""

    check_dependencies
    echo ""

    validate_test_documents
    echo ""

    validate_syntax
    echo ""

    create_test_runner
    echo ""

    generate_summary

    log_success "Test environment validation completed!"
}

# Run main function if script is executed directly
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi