# CLI Interface Contract

**Feature**: Command-Line Interface
**Contract Type**: CLI behavior and exit codes
**Date**: 2025-10-05

## Contract Overview

This contract defines the expected behavior of the `docling-rs` CLI binary, including argument parsing, exit codes, output generation, and error handling.

---

## Command Signature

```
docling-rs [OPTIONS] <INPUT>
```

### Positional Arguments

- `<INPUT>`: File path or directory path (required)

### Options

```
--from <FORMAT>...         Filter input files by format (pdf, md, html, csv, docx)
                           Can be specified multiple times for batch processing

--to <FORMAT>...           Output formats (md, json, text)
                           Default: md
                           Can be specified multiple times

-o, --output <DIR>         Output directory (default: current directory)

--ocr                      Enable OCR for PDFs (default: false)
--no-ocr                   Disable OCR for PDFs

--force-ocr                Force OCR even if PDF has text

--ocr-lang <LANG>          OCR language code (default: eng)

--tables                   Enable table detection (default: true for PDFs)
--no-tables                Disable table detection

--images                   Enable image extraction (default: true for PDFs)
--no-images                Disable image extraction

--enrich-code              Detect code blocks (Phase 3f)
--enrich-formula           Detect mathematical formulas (Phase 3f)
--enrich-lists             Detect list structures (Phase 3f)

-v, --verbose              Increase verbosity (-v, -vv, -vvv)
-q, --quiet                Suppress non-error output

--abort-on-error           Stop on first error in batch processing

--version                  Show version information
--help                     Show help message
```

---

## Exit Codes

```
0 - Success (all files processed successfully)
1 - Error (conversion failure, invalid arguments, file not found)
```

---

## Contract Tests

### CT-001: Single File Conversion (Basic)

**Given**: A valid PDF file `test.pdf` exists
**When**: Run `docling-rs test.pdf`
**Then**:
- Exit code: 0
- Output file: `test.md` created in current directory
- Content: Valid markdown from PDF

### CT-002: Multiple Output Formats

**Given**: A valid PDF file `test.pdf` exists
**When**: Run `docling-rs test.pdf --to json --to md --to text`
**Then**:
- Exit code: 0
- Output files: `test.json`, `test.md`, `test.txt` created
- All files contain same content in different formats

### CT-003: Custom Output Directory

**Given**: A valid PDF file `test.pdf` exists, directory `./output` does not exist
**When**: Run `docling-rs test.pdf --output ./output`
**Then**:
- Exit code: 0
- Directory `./output` created
- Output file: `./output/test.md` exists

### CT-004: Batch Directory Processing

**Given**: Directory `./docs` contains 3 PDFs: `a.pdf`, `b.pdf`, `c.pdf`
**When**: Run `docling-rs ./docs --from pdf`
**Then**:
- Exit code: 0
- Output files: `a.md`, `b.md`, `c.md` created
- Progress shown during processing

### CT-005: Format Filtering

**Given**: Directory `./docs` contains 2 PDFs and 2 DOCX files
**When**: Run `docling-rs ./docs --from pdf`
**Then**:
- Exit code: 0
- Only PDF files converted (2 output files)
- DOCX files ignored

### CT-006: Input File Not Found

**Given**: File `missing.pdf` does not exist
**When**: Run `docling-rs missing.pdf`
**Then**:
- Exit code: 1
- Error to stderr: "File not found: missing.pdf"
- No output files created

### CT-007: Unsupported Format

**Given**: File `test.xyz` exists with unknown extension
**When**: Run `docling-rs test.xyz`
**Then**:
- Exit code: 1
- Error to stderr: "Unsupported format for test.xyz: xyz. Supported: pdf, md, html, csv, docx"
- No output files created

### CT-008: PDF with OCR

**Given**: A scanned PDF `scan.pdf` with no embedded text
**When**: Run `docling-rs scan.pdf --ocr --ocr-lang eng`
**Then**:
- Exit code: 0
- Output file: `scan.md` with OCR-extracted text
- OCR performed using Tesseract

### CT-009: PDF Options (Tables, Images)

**Given**: A PDF `document.pdf` with tables and images
**When**: Run `docling-rs document.pdf --tables --images --to json`
**Then**:
- Exit code: 0
- Output JSON contains table structures
- Output JSON contains image metadata

### CT-010: Enrichment Options

**Given**: A technical PDF `paper.pdf` with code and formulas
**When**: Run `docling-rs paper.pdf --enrich-code --enrich-formula --to json`
**Then**:
- Exit code: 0
- Output JSON contains code blocks as separate elements
- Output JSON contains formulas as separate elements

### CT-011: Batch Error Handling (Continue)

**Given**: Directory `./mixed` contains `good.pdf` (valid) and `bad.pdf` (corrupted)
**When**: Run `docling-rs ./mixed` (default: continue on error)
**Then**:
- Exit code: 1 (had errors)
- `good.md` created successfully
- Error logged for `bad.pdf`
- Processing continued after error

### CT-012: Batch Error Handling (Abort)

**Given**: Directory `./mixed` contains `a.pdf`, `bad.pdf` (corrupted), `c.pdf`
**When**: Run `docling-rs ./mixed --abort-on-error`
**Then**:
- Exit code: 1
- `a.md` created (processed before error)
- Processing stopped at `bad.pdf`
- `c.pdf` not processed

### CT-013: Help Output

**Given**: CLI installed
**When**: Run `docling-rs --help`
**Then**:
- Exit code: 0
- Help text printed to stdout
- Help includes all flags and descriptions
- Usage examples shown

### CT-014: Version Output

**Given**: CLI installed (version 0.2.0)
**When**: Run `docling-rs --version`
**Then**:
- Exit code: 0
- Output to stdout: "docling-rs 0.2.0"

### CT-015: Verbose Output

**Given**: A valid PDF `test.pdf`
**When**: Run `docling-rs test.pdf -vv`
**Then**:
- Exit code: 0
- Verbose logging to stderr showing:
  - Input file detected as PDF
  - Backend selection
  - Processing steps
  - Output file generation

### CT-016: Quiet Mode

**Given**: A valid PDF `test.pdf`
**When**: Run `docling-rs test.pdf --quiet`
**Then**:
- Exit code: 0
- No output to stdout/stderr (except errors)
- `test.md` created silently

### CT-017: Conflicting Flags

**Given**: CLI accepts arguments
**When**: Run `docling-rs test.pdf --verbose --quiet`
**Then**:
- Exit code: 1 (or clap handles conflict)
- Error: "Cannot use --verbose and --quiet together"

### CT-018: Invalid Argument

**Given**: CLI accepts arguments
**When**: Run `docling-rs test.pdf --invalid-flag`
**Then**:
- Exit code: 1
- Error showing available flags
- Suggestion for similar valid flags (clap feature)

---

## Output File Specifications

### Markdown Output (.md)

- **Format**: CommonMark-compliant markdown
- **Encoding**: UTF-8
- **Newlines**: Platform-specific (LF on Unix, CRLF on Windows)
- **Structure**: Preserves document hierarchy (headings, paragraphs, lists, tables)

### JSON Output (.json)

- **Format**: Pretty-printed JSON (indented)
- **Schema**: DoclingDocument serialization
- **Encoding**: UTF-8
- **Fields**: All document metadata, content nodes, positions

### Text Output (.txt)

- **Format**: Plain text
- **Encoding**: UTF-8
- **Content**: Extracted text only (no formatting, no metadata)
- **Structure**: Paragraphs separated by blank lines

---

## Behavioral Contracts

### BC-001: Idempotency

Running the same command twice with same inputs produces identical outputs (same content, may update timestamps).

### BC-002: Filesystem Safety

- Output files overwrite existing files without warning
- Creates parent directories as needed
- Preserves input files (read-only operations)

### BC-003: Progress Indication

For batch operations with >2 files:
- Show progress bar with current/total
- Update progress after each file
- Clear progress bar on completion

### BC-004: Error Messages

All error messages must:
- Print to stderr (not stdout)
- Include file path when applicable
- Be actionable (tell user what to fix)
- Follow format: "Error: {description}"

### BC-005: Default Behavior

Without flags:
- Input format: auto-detected from extension
- Output format: Markdown
- Output directory: current directory
- OCR: disabled
- Tables: enabled (for PDF)
- Images: enabled (for PDF)
- Error handling: continue on error

---

## Performance Contracts

### PC-001: Startup Time

CLI must be ready to process first file within:
- **Target**: <100ms from invocation to first action
- **Measurement**: `time docling-rs --help` (cold start)

### PC-002: Memory Usage

For batch processing:
- **Target**: O(1) memory (stream files, don't load all)
- **Max**: <500MB for processing any single file
- **Measurement**: `ps aux` during batch conversion

### PC-003: Single File Speed

Simple PDF (<10 pages, no OCR):
- **Target**: <5 seconds total time
- **Breakdown**: <50ms CLI overhead, rest is backend processing

---

## Cross-Platform Contracts

### XP-001: Path Handling

- Accept both Unix (`/`) and Windows (`\`) path separators
- Handle spaces in paths (with or without quotes)
- Support Unicode filenames

### XP-002: Exit Codes

Consistent exit codes across macOS and Windows:
- 0 = success
- 1 = error

### XP-003: Line Endings

Output files use platform-specific line endings:
- Unix/macOS: LF (`\n`)
- Windows: CRLF (`\r\n`)

---

## Security Contracts

### SC-001: Input Validation

- Validate all file paths before opening
- Reject paths with directory traversal attempts (`..`)
- Limit file sizes to prevent DoS (delegated to backends)

### SC-002: No Code Execution

CLI never executes code from input files, only parses and converts content.

---

## Compatibility Contracts

### CC-001: Docling Python CLI Parity

Match Python docling CLI behavior for:
- Argument names (`--from`, `--to`, `--output`, `--ocr`)
- Default values
- Output formats
- Error messages (where reasonable)

**Deviations** (documented):
- No VLM pipeline (not implemented in Rust version yet)
- No URL downloading (future enhancement)
- Different backend implementations (pdfium vs pypdfium2)

---

## Test Implementation Notes

Contract tests should be implemented in:
- `tests/contract/contract_cli.rs` - Argument parsing and behavior
- `tests/integration/integration_cli_*.rs` - End-to-end scenarios

Use `assert_cmd` crate for CLI testing:
```rust
use assert_cmd::Command;

#[test]
fn ct_001_single_file_conversion() {
    let mut cmd = Command::cargo_bin("docling-rs").unwrap();
    cmd.arg("test.pdf");
    cmd.assert().success();

    assert!(Path::new("test.md").exists());
}
```

---

## References

- Spec: [../spec.md](../spec.md) - Functional requirements
- Data Model: [../data-model.md](../data-model.md) - CLI entities
- Research: [../research.md](../research.md) - Technology decisions
