# Research: CLI Manual Testing Script

## Overview
This document captures technology and design decisions for the manual testing script that validates CLI functionality with real-world documents.

## Technology Decisions

### TD-001: Shell Language (Bash vs POSIX sh)
**Decision**: Bash with POSIX fallback detection
**Alternatives Considered**:
- Pure POSIX sh (maximum compatibility)
- Bash-only (richer features)
- Python script (cross-platform, but adds dependency)

**Rationale**:
- Bash available on all target platforms (macOS, Linux)
- POSIX-compatible constructs where possible
- Bash-specific features only for nice-to-have (colors, arrays)
- No Python dependency (aligns with native Rust philosophy)

**Trade-offs**:
- ✅ Pro: No additional dependencies
- ✅ Pro: Fast execution
- ✅ Pro: Native terminal integration
- ⚠️ Con: Not Windows-native (WSL required)

### TD-002: Color Output (ANSI Escape Codes)
**Decision**: ANSI color codes with auto-detection
**Alternatives Considered**:
- No colors (plain text)
- External tool (tput)
- Rich formatting library

**Rationale**:
- ANSI codes universally supported on macOS/Linux terminals
- Auto-detect terminal capability (check $TERM)
- Graceful degradation to plain text
- Improves readability significantly

**Implementation**:
```bash
# Color definitions (if terminal supports it)
if [ -t 1 ] && [ "$TERM" != "dumb" ]; then
    GREEN='\033[0;32m'
    RED='\033[0;31m'
    YELLOW='\033[1;33m'
    BLUE='\033[0;34m'
    NC='\033[0m' # No Color
else
    GREEN='' RED='' YELLOW='' BLUE='' NC=''
fi
```

### TD-003: Build Management
**Decision**: Conditional rebuild (check cargo metadata)
**Alternatives Considered**:
- Always rebuild (slow, but always fresh)
- Never rebuild (fast, but potentially stale)
- Timestamp comparison (complex, fragile)

**Rationale**:
- Check if `target/release/docling-rs` exists
- If missing, run `cargo build --release`
- User can manually rebuild if needed
- Clear output shows build happening

**Trade-offs**:
- ✅ Pro: Developer convenience
- ✅ Pro: Clear feedback
- ⚠️ Con: Might test stale binary if user forgot to rebuild
- Solution: Print warning with binary timestamp

### TD-004: Output Truncation Strategy
**Decision**: Head truncation (first 50 lines + indicator)
**Alternatives Considered**:
- No truncation (overwhelming for large docs)
- Head + tail (shows both ends)
- Configurable limit (adds complexity)

**Rationale**:
- Most relevant content usually at start (title, headers, first paragraphs)
- 50 lines enough to assess extraction quality
- Clear indicator shows truncation happened
- Full output available by running CLI directly

**Implementation**:
```bash
output | head -n 50
line_count=$(echo "$output" | wc -l)
if [ "$line_count" -gt 50 ]; then
    echo "... (truncated, $line_count total lines)"
fi
```

### TD-005: Error Handling Pattern
**Decision**: Collect errors, continue processing, report at end
**Alternatives Considered**:
- Fail fast (exit on first error)
- Silently skip errors
- Interactive prompts (breaks automation)

**Rationale**:
- Maximizes information per test run
- Developer sees all problems at once
- Non-zero exit if any failures
- Summary shows success rate

**Error Categories**:
1. Binary not found / build failed → Fatal (exit immediately)
2. Test directory missing → Fatal (exit immediately)
3. Individual file fails → Non-fatal (continue, count failure)
4. Unsupported format → Non-fatal (expected, mark as skipped)

### TD-006: Performance Measurement
**Decision**: Per-file timing with `time` command
**Alternatives Considered**:
- No timing (simpler)
- Millisecond precision (overkill for this use case)
- External benchmarking tool

**Rationale**:
- `time` command built-in, portable
- Second precision sufficient for manual testing
- Helps identify slow formats/documents
- Minimal implementation overhead

**Implementation**:
```bash
start_time=$(date +%s)
# ... process file ...
end_time=$(date +%s)
duration=$((end_time - start_time))
echo "Duration: ${duration}s"
```

### TD-007: CLI Invocation Pattern
**Decision**: Two-phase testing per file (extract + chunk)
**Alternatives Considered**:
- Single invocation (extract only)
- Multiple formats (markdown, JSON, text)
- Comprehensive flag matrix

**Rationale**:
- Tests both core capabilities (extraction + chunking)
- Reasonable output volume
- Matches primary use cases
- Easy to extend later if needed

**Commands**:
```bash
# Phase 1: Text extraction
./target/release/docling-rs "$file" --to text

# Phase 2: With chunking
./target/release/docling-rs "$file" --chunk --chunk-size 1000
```

## Dependencies
**Runtime**:
- Bash 3.2+ (macOS default, Linux standard)
- Standard POSIX utilities (ls, wc, head, date)
- cargo (for building release binary)

**Build-time**:
- None (shell script, no compilation)

## Performance Considerations
- Script overhead: <1s (negligible)
- Main cost: CLI processing time per document
- Expected: 1-30s per document depending on size/format
- Total: ~2-5 minutes for all test documents

## Platform Compatibility
**Supported**:
- ✅ macOS (Bash 3.2+)
- ✅ Linux (Bash 4.0+)
- ✅ WSL on Windows (Bash 4.4+)

**Not Supported**:
- ❌ Windows cmd.exe (use WSL instead)
- ❌ PowerShell (different scripting language)

## Security Considerations
- Script only reads from tests/documents-test/ (safe)
- Invokes locally-built binary (trusted)
- No network access
- No privileged operations
- User can review script before execution

## Future Enhancements (Out of Scope)
- Parallel file processing (faster, but more complex)
- Diff comparison with previous runs (regression detection)
- HTML report generation (nice-to-have)
- Configuration file for customization (YAGNI for now)
- Windows native support (requires separate script or cross-platform tool)

---

**Research Complete**: All technology decisions documented. Ready for implementation phase.
