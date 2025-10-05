# Quickstart: CLI for docling-rs

**Feature**: Command-Line Interface
**Date**: 2025-10-05
**Purpose**: Quick start guide and usage examples for the docling-rs CLI

## Installation

```bash
# Build the CLI from source
cargo build --release

# Binary location
./target/release/docling-rs

# Optional: Install to system
cargo install --path .
```

---

## Basic Usage

### Convert a Single File

Convert a PDF to Markdown (default behavior):

```bash
docling-rs document.pdf
```

**Result**: Creates `document.md` in the current directory.

---

### Multiple Output Formats

Generate multiple formats from one input:

```bash
docling-rs research-paper.pdf --to md --to json --to text
```

**Result**: Creates:
- `research-paper.md` (markdown)
- `research-paper.json` (structured JSON)
- `research-paper.txt` (plain text)

---

### Custom Output Directory

Save outputs to a specific directory:

```bash
docling-rs document.pdf --output ./converted
```

**Result**: Creates `./converted/document.md` (directory created if needed).

---

## Batch Processing

### Convert All PDFs in a Directory

```bash
docling-rs ./documents --from pdf
```

**Result**: Converts all PDF files in `./documents/` to markdown.

**Progress Output**:
```
[████████████████████████████████] 10/10 Processing report.pdf
Converted 10 files successfully
```

---

### Multiple Input Formats

Convert PDFs and DOCX files:

```bash
docling-rs ./documents --from pdf --from docx --output ./markdown
```

**Result**: Converts all PDF and DOCX files to markdown in `./markdown/`.

---

### Stop on First Error

Abort batch processing if any file fails:

```bash
docling-rs ./documents --from pdf --abort-on-error
```

**Default Behavior**: Without `--abort-on-error`, continues processing remaining files after errors.

---

## PDF-Specific Options

### OCR for Scanned PDFs

Enable optical character recognition:

```bash
docling-rs scanned-document.pdf --ocr
```

**Use Case**: PDFs without embedded text (scanned images).

---

### OCR with Language Selection

Specify OCR language (e.g., Spanish):

```bash
docling-rs documento.pdf --ocr --ocr-lang spa
```

**Supported Languages**: Any Tesseract-supported language code (eng, spa, fra, deu, etc.).

---

### Force OCR on Text PDFs

Replace PDF text with OCR output:

```bash
docling-rs document.pdf --force-ocr --ocr-lang eng
```

**Use Case**: PDFs with low-quality or incorrectly encoded text.

---

### Disable Tables or Images

Skip table detection or image extraction:

```bash
docling-rs document.pdf --no-tables --no-images
```

**Result**: Faster processing, text-only output.

---

## Content Enrichment (Phase 3f)

### Detect Code Blocks

Identify and classify code blocks in technical documents:

```bash
docling-rs technical-paper.pdf --enrich-code --to json
```

**Output JSON**: Code blocks marked with language and syntax.

---

### Detect Mathematical Formulas

Extract formulas from scientific papers:

```bash
docling-rs research-paper.pdf --enrich-formula --to json
```

**Output JSON**: Formulas identified as separate elements with LaTeX/Unicode notation.

---

### Detect Lists

Preserve list structures:

```bash
docling-rs specification.pdf --enrich-lists --to json
```

**Output JSON**: Nested lists with proper hierarchy.

---

### All Enrichments

Enable all content enrichment features:

```bash
docling-rs algorithms-book.pdf --enrich-code --enrich-formula --enrich-lists --to json
```

---

## Output Control

### Verbose Mode

Show detailed processing information:

```bash
docling-rs document.pdf -v
```

**Levels**:
- `-v`: Info (file detection, backend selection)
- `-vv`: Debug (processing steps, timings)
- `-vvv`: Trace (detailed internals)

---

### Quiet Mode

Suppress all non-error output:

```bash
docling-rs document.pdf --quiet
```

**Use Case**: Scripting, automation where output is not needed.

---

## Help and Version

### Show Help

```bash
docling-rs --help
```

**Output**: Complete usage guide with all options.

---

### Show Version

```bash
docling-rs --version
```

**Output**: `docling-rs 0.2.0` (or current version).

---

## Common Workflows

### 1. Research Paper to Markdown

```bash
# Convert with code and formula detection
docling-rs paper.pdf --enrich-code --enrich-formula --to md --to json
```

**Use Case**: Academic papers with equations and pseudocode.

---

### 2. Batch Document Archive Conversion

```bash
# Convert entire archive with progress
docling-rs ./archive --from pdf --from docx --output ./markdown
```

**Use Case**: Migrating document archive to markdown for version control.

---

### 3. Scanned Invoice Processing

```bash
# OCR with table detection
docling-rs invoices/*.pdf --ocr --tables --to json --output ./structured
```

**Use Case**: Extracting structured data from scanned invoices.

---

### 4. Technical Documentation Export

```bash
# Full enrichment for documentation
docling-rs ./docs --from md --from html --enrich-code --to json
```

**Use Case**: Consolidating mixed-format documentation.

---

### 5. Quick Text Extraction

```bash
# Fast text-only extraction
docling-rs report.pdf --to text --no-tables --no-images
```

**Use Case**: Quick text search or preview without formatting.

---

## Advanced Examples

### Combine Multiple Options

```bash
docling-rs ./technical-papers \
  --from pdf \
  --to md --to json \
  --output ./converted \
  --ocr \
  --ocr-lang eng \
  --enrich-code \
  --enrich-formula \
  --verbose \
  --abort-on-error
```

**Breakdown**:
- Input: All PDFs in `./technical-papers`
- Output: Markdown + JSON in `./converted`
- OCR: Enabled (English)
- Enrichment: Code blocks + formulas
- Logging: Verbose
- Errors: Abort on first failure

---

### Scripting Integration

```bash
#!/bin/bash
# Convert and check outputs
docling-rs input.pdf --output ./out || exit 1

# Verify output exists
if [ -f "./out/input.md" ]; then
    echo "Conversion successful"
    cat ./out/input.md | wc -l
else
    echo "Conversion failed"
    exit 1
fi
```

---

### CI/CD Pipeline

```yaml
# .github/workflows/docs.yml
- name: Convert documentation
  run: |
    docling-rs ./docs --from md --to json --output ./dist

- name: Upload artifacts
  uses: actions/upload-artifact@v3
  with:
    name: converted-docs
    path: ./dist/*.json
```

---

## Troubleshooting

### File Not Found Error

```
Error: File not found: document.pdf
```

**Solution**: Check file path, use absolute path, or verify current directory.

---

### Unsupported Format Error

```
Error: Unsupported format for file.xyz: xyz. Supported: pdf, md, html, csv, docx
```

**Solution**: Use `--from` to explicitly specify format or rename file with correct extension.

---

### OCR Feature Not Available

```
Error: OCR feature not available. Recompile with --features ocr
```

**Solution**: Rebuild with OCR feature enabled:
```bash
cargo build --release --features ocr
```

---

### Permission Denied

```
Error: Could not create output directory ./out: Permission denied
```

**Solution**: Check write permissions, use different output directory, or run with appropriate permissions.

---

## Performance Tips

### 1. Skip Unnecessary Features

For faster processing, disable unused features:

```bash
docling-rs document.pdf --no-tables --no-images
```

---

### 2. Batch Processing Optimization

Process files sequentially (default) to avoid memory issues:

```bash
# Good: Sequential (default)
docling-rs ./large-batch --from pdf

# Future: Parallel flag (not yet implemented)
# docling-rs ./large-batch --from pdf --parallel 4
```

---

### 3. Use Appropriate Verbosity

Reduce logging for faster processing:

```bash
docling-rs ./batch --quiet  # Fastest
docling-rs ./batch           # Default
docling-rs ./batch -v        # Slower (logging overhead)
```

---

## Exit Codes

The CLI uses standard exit codes:

```
0 - Success
1 - Error (any failure)
```

**Check in Scripts**:
```bash
if docling-rs document.pdf; then
    echo "Conversion succeeded"
else
    echo "Conversion failed"
fi
```

---

## What's Next?

After running the quickstart examples:

1. **Explore Output Formats**: Try different combinations of `--to` flags
2. **Test Batch Processing**: Convert a directory with mixed formats
3. **Experiment with Enrichment**: Use `--enrich-*` flags on technical docs
4. **Integrate into Workflow**: Use in scripts, CI/CD, or automation tools

---

## References

- **Contract**: [contracts/cli_interface.md](./contracts/cli_interface.md) - Full CLI specification
- **Data Model**: [data-model.md](./data-model.md) - Internal structures
- **Research**: [research.md](./research.md) - Technology decisions
- **Spec**: [spec.md](./spec.md) - Requirements
