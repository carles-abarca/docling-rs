# Quickstart: CLI Manual Testing Script

## Overview
The manual testing script (`scripts/test-cli-manual.sh`) processes all test documents in `tests/documents-test/` using the docling-rs CLI, displaying extraction and chunking results for manual verification.

## Prerequisites
- Rust toolchain installed (`cargo` command available)
- Project checked out at `/Users/carlesabarca/MyProjects/docling-rs/`
- Test documents present in `tests/documents-test/` (already copied)

## Quick Start

### 1. Run the Script
```bash
# From project root
./scripts/test-cli-manual.sh
```

The script will:
1. Check if release binary exists
2. Build if needed (`cargo build --release`)
3. Process each document in `tests/documents-test/`
4. Display extraction and chunking results
5. Show summary statistics

### 2. Expected Output Format

```
==========================================
Testing CLI with Real Documents
==========================================

[1/14] Processing: UseOfChatGPT.pdf
------------------------------------------
Format: PDF
Extracting text...
✓ Success (3.2s)

[Output - first 50 lines]
Use of ChatGPT in Education
...

Testing chunking...
✓ Success (2.1s)

[Chunks - first 50 lines]
Chunk 1 (0-1000 chars):
Use of ChatGPT in Education...
...

==========================================

[2/14] Processing: test_readme.md
------------------------------------------
Format: Markdown
Extracting text...
✓ Success (0.1s)

[Output - first 50 lines]
# Test README
...

==========================================
SUMMARY
==========================================
Total files: 14
✓ Passed: 10
✗ Failed: 3
⊘ Skipped: 1
Total duration: 45s
==========================================
```

## Test Documents

The script processes these document types:

| Format   | File Example                                    | Expected Result |
|----------|-------------------------------------------------|-----------------|
| PDF      | `UseOfChatGPT.pdf`                              | ✓ Extract text  |
| DOCX     | `Conferencistas_Futurologia_Educativa.docx`     | ✓ Extract text  |
| Markdown | `test_readme.md`                                | ✓ Extract text  |
| JSON     | `test_data.json`                                | ⊘ Unsupported   |
| TXT      | `test_document.txt`                             | ✓ Extract text  |
| YAML     | `test_config.yaml`                              | ⊘ Unsupported   |
| Shell    | `test_script.sh`                                | ⊘ Unsupported   |
| Python   | `test_code.py`                                  | ⊘ Unsupported   |
| Excel    | `Indian_CEOs.xlsx`                              | ⊘ Unsupported   |
| PPTX     | `RoadmaptoAccelerate...pptx`                    | ⊘ Unsupported   |
| JPEG     | `pie.jpeg`                                      | ⊘ Unsupported   |

**Note**: Current implementation supports PDF, DOCX, Markdown, HTML, CSV, and TXT. Other formats expected to skip gracefully.

## Usage Scenarios

### Scenario 1: Verify CLI Works End-to-End
**Goal**: Ensure CLI can process real documents without crashing

```bash
./scripts/test-cli-manual.sh
# Check exit code
echo $?  # Should be 0 if at least PDF/DOCX/MD work
```

### Scenario 2: Test After Code Changes
**Goal**: Verify changes didn't break extraction

```bash
# Make changes to extraction logic
cargo build --release

# Run manual tests
./scripts/test-cli-manual.sh
# Review output for regressions
```

### Scenario 3: Performance Testing
**Goal**: Identify slow documents/formats

```bash
./scripts/test-cli-manual.sh | grep "Duration:"
# Look for outliers (>10s)
```

### Scenario 4: Validate Chunking
**Goal**: Ensure chunking produces reasonable results

```bash
./scripts/test-cli-manual.sh | grep -A 20 "Testing chunking"
# Review chunk boundaries and sizes
```

## Interpreting Results

### Success Indicators
- ✓ Green checkmarks for working formats
- Readable extracted text (no garbled output)
- Chunk boundaries respect sentence/paragraph structure
- Reasonable processing times (<5s for small docs)

### Failure Indicators
- ✗ Red X for failed extractions
- Empty output for supported formats
- Crashes or panics (should not happen)
- Very slow processing (>30s for small doc)

### Skip Indicators
- ⊘ Yellow marker for unsupported formats
- Expected for JSON, YAML, SH, PY, XLSX, PPTX, JPEG
- Should not fail, just skip gracefully

## Troubleshooting

### Problem: Binary not found
**Error**: `Error: Release binary not found`
**Solution**:
```bash
cargo build --release
./scripts/test-cli-manual.sh
```

### Problem: Test directory missing
**Error**: `Error: Test directory not found`
**Solution**:
```bash
# Verify documents copied correctly
ls tests/documents-test/
# Should show 14 files
```

### Problem: All tests fail
**Error**: Every file shows red X
**Solution**:
```bash
# Test CLI directly
./target/release/docling-rs tests/documents-test/test_readme.md
# Check for compilation errors
cargo build --release 2>&1 | grep error
```

### Problem: Truncated output
**Behavior**: Output shows "... (truncated, 500 total lines)"
**Solution**: This is expected! To see full output:
```bash
# Run CLI directly on specific file
./target/release/docling-rs tests/documents-test/UseOfChatGPT.pdf --to text
```

## Advanced Usage

### Test Specific File
```bash
# Edit script to add file filter (future enhancement)
# For now, run CLI directly:
./target/release/docling-rs tests/documents-test/specific-file.pdf
```

### Save Results to File
```bash
./scripts/test-cli-manual.sh > test-results.log 2>&1
# Review later
less test-results.log
```

### Compare with Previous Run
```bash
# Run 1
./scripts/test-cli-manual.sh > results-before.log 2>&1

# Make changes
# ...

# Run 2
./scripts/test-cli-manual.sh > results-after.log 2>&1

# Compare
diff results-before.log results-after.log
```

## Exit Codes

| Code | Meaning                                    |
|------|--------------------------------------------|
| 0    | All supported formats processed successfully |
| 1    | One or more supported formats failed       |
| 2    | Binary build failed                        |
| 3    | Test directory not found                   |

## Limitations

- Windows: Requires WSL (no native cmd.exe support)
- Large files: Output truncated to 50 lines
- Serial processing: One file at a time (slow for many files)
- No diff comparison: Manual comparison of runs
- No HTML report: Terminal output only

## Next Steps

After running the script:

1. **If all tests pass**: CLI ready for production use
2. **If some fail**: Investigate errors in failed formats
3. **If performance issues**: Profile slow documents/formats
4. **If output garbled**: Check encoding handling in backends

---

**Last Updated**: 2025-10-05 | **Script Version**: 1.0
