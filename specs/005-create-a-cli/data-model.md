# Data Model: CLI for docling-rs

**Feature**: Command-Line Interface
**Date**: 2025-10-05
**Purpose**: Define core data structures for CLI implementation

## Overview

The CLI data model consists of argument structures, conversion jobs, and result tracking. All structures designed for CLI orchestration layer - no modification to existing library data models.

---

## Core Entities

### 1. CliArgs

**Purpose**: Represents parsed command-line arguments

**Fields**:
```rust
pub struct CliArgs {
    /// Input file path or directory path
    pub input: PathBuf,

    /// Filter input files by format (--from pdf --from docx)
    pub from: Vec<InputFormat>,

    /// Output formats to generate (--to md --to json)
    pub to: Vec<OutputFormat>,

    /// Output directory (default: current directory)
    pub output: Option<PathBuf>,

    /// Enable OCR for PDFs
    pub ocr: bool,

    /// Enable table detection for PDFs
    pub tables: bool,

    /// Enable image extraction for PDFs
    pub images: bool,

    /// Enable code block detection (Phase 3f)
    pub enrich_code: bool,

    /// Enable formula detection (Phase 3f)
    pub enrich_formula: bool,

    /// Enable list detection (Phase 3f)
    pub enrich_lists: bool,

    /// OCR language code (default: "eng")
    pub ocr_lang: String,

    /// Force OCR even if PDF has embedded text
    pub force_ocr: bool,

    /// Verbosity level (0=normal, 1=-v, 2=-vv, 3=-vvv)
    pub verbose: u8,

    /// Suppress non-error output
    pub quiet: bool,

    /// Stop processing on first error
    pub abort_on_error: bool,
}
```

**Validation Rules**:
- `input` must be a valid path (file or directory)
- If `input` is file, at most one `from` filter allowed
- If `to` is empty, default to `[OutputFormat::Markdown]`
- `ocr_lang` must be valid language code (validated by OCR engine)
- `verbose` and `quiet` are mutually exclusive

**Relationships**:
- Converts to `ConversionConfig` for each file
- Drives `ConversionJob` creation

---

### 2. InputFormat

**Purpose**: Enum of supported input file formats

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum InputFormat {
    Pdf,
    Markdown,
    Html,
    Csv,
    Docx,
}
```

**String Mapping**:
- "pdf" → InputFormat::Pdf
- "md" → InputFormat::Markdown
- "html" → InputFormat::Html
- "csv" → InputFormat::Csv
- "docx" → InputFormat::Docx

**Usage**: File extension detection, `--from` filter

---

### 3. OutputFormat

**Purpose**: Enum of supported output formats

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum OutputFormat {
    Markdown,
    Json,
    Text,
}
```

**String Mapping**:
- "md" → OutputFormat::Markdown
- "json" → OutputFormat::Json
- "text" → OutputFormat::Text

**File Extensions**:
- Markdown → `.md`
- Json → `.json`
- Text → `.txt`

**Usage**: `--to` flag, output file generation

---

### 4. ConversionJob

**Purpose**: Represents a single file conversion task

**Fields**:
```rust
pub struct ConversionJob {
    /// Input file path
    pub input_path: PathBuf,

    /// Detected or specified input format
    pub input_format: InputFormat,

    /// Output formats to generate
    pub output_formats: Vec<OutputFormat>,

    /// Output directory for generated files
    pub output_dir: PathBuf,

    /// Conversion configuration (PDF-specific options, etc.)
    pub config: ConversionConfig,
}
```

**Creation Logic**:
```
For single file:
    job = ConversionJob from CliArgs + input file

For directory:
    jobs = [ConversionJob for each file in directory matching filters]
```

**Relationships**:
- Created from `CliArgs` + file path
- Uses `ConversionConfig` for backend options
- Produces `ConversionResult`

---

### 5. ConversionConfig

**Purpose**: Backend-agnostic conversion options

**Fields**:
```rust
pub struct ConversionConfig {
    /// PDF-specific configuration (if input is PDF)
    pub pdf_config: Option<PdfConfig>,

    /// Verbosity level for logging
    pub verbose: u8,
}
```

**Mapping from CliArgs**:
```rust
impl From<&CliArgs> for ConversionConfig {
    fn from(args: &CliArgs) -> Self {
        let pdf_config = Some(PdfConfig {
            enable_ocr: args.ocr,
            ocr_language: args.ocr_lang.clone(),
            force_ocr: args.force_ocr,
            enable_tables: args.tables,
            enable_images: args.images,
            enable_enrichment: Enrichment {
                code: args.enrich_code,
                formula: args.enrich_formula,
                lists: args.enrich_lists,
            },
            ..Default::default()
        });

        ConversionConfig {
            pdf_config,
            verbose: args.verbose,
        }
    }
}
```

**Relationships**:
- Derived from `CliArgs`
- Passed to Backend implementations

---

### 6. ConversionResult

**Purpose**: Result of processing one file

**Fields**:
```rust
pub struct ConversionResult {
    /// Input file that was processed
    pub input_path: PathBuf,

    /// Whether conversion succeeded
    pub success: bool,

    /// Generated output file paths (empty if failed)
    pub output_files: Vec<PathBuf>,

    /// Error message if failed
    pub error: Option<String>,
}
```

**States**:
- **Success**: `success=true`, `output_files` populated, `error=None`
- **Failure**: `success=false`, `output_files` empty, `error=Some(...)`

**Usage**: Progress tracking, error reporting, final summary

---

### 7. BatchProgress

**Purpose**: Tracks progress of batch operations

**Fields**:
```rust
pub struct BatchProgress {
    /// Total files to process
    pub total: usize,

    /// Files successfully processed
    pub succeeded: usize,

    /// Files that failed
    pub failed: usize,

    /// Errors encountered (input_path, error_message)
    pub errors: Vec<(PathBuf, String)>,
}
```

**Operations**:
```rust
impl BatchProgress {
    pub fn new(total: usize) -> Self { ... }
    pub fn record_success(&mut self, path: PathBuf) { ... }
    pub fn record_failure(&mut self, path: PathBuf, error: String) { ... }
    pub fn is_complete(&self) -> bool { ... }
    pub fn summary(&self) -> String { ... }
}
```

**Usage**: Progress bar updates, final statistics

---

## Data Flow

### Single File Conversion

```
1. Parse CliArgs from command line
2. Detect InputFormat from file extension
3. Create ConversionJob from CliArgs + input file
4. Select Backend based on InputFormat
5. Call backend.convert() with ConversionConfig
6. For each OutputFormat:
   - Generate output file from DoclingDocument
   - Write to output directory
7. Return ConversionResult
```

### Batch Directory Conversion

```
1. Parse CliArgs from command line
2. Scan directory for files
3. Filter files by --from if specified
4. Create ConversionJob for each file
5. Initialize BatchProgress(total=len(jobs))
6. For each job:
   a. Process as single file
   b. Update BatchProgress
   c. Update progress bar
   d. If abort_on_error and failed: break
7. Print final summary
```

---

## File Naming Convention

### Output Filename Generation

**Rule**: Replace input extension with output extension

```
Examples:
- document.pdf + Markdown → document.md
- document.pdf + Json → document.json
- paper.docx + Markdown → paper.md
- data.csv + Json → data.json
```

**Implementation**:
```rust
fn output_filename(input: &Path, format: OutputFormat) -> PathBuf {
    let stem = input.file_stem().unwrap();
    let ext = match format {
        OutputFormat::Markdown => "md",
        OutputFormat::Json => "json",
        OutputFormat::Text => "txt",
    };
    PathBuf::from(format!("{}.{}", stem.to_string_lossy(), ext))
}
```

---

## Error Handling

### Error Types

```rust
pub enum CliError {
    /// Input file or directory not found
    InputNotFound(PathBuf),

    /// Unsupported file format
    UnsupportedFormat(PathBuf, String),

    /// Conversion failed
    ConversionFailed(PathBuf, String),

    /// Output directory creation failed
    OutputDirError(PathBuf, std::io::Error),

    /// File write failed
    OutputWriteFailed(PathBuf, std::io::Error),

    /// Invalid arguments
    InvalidArgs(String),
}
```

**Error Messages** (user-facing):
- `InputNotFound`: "File not found: {path}"
- `UnsupportedFormat`: "Unsupported format for {path}: {ext}. Supported: pdf, md, html, csv, docx"
- `ConversionFailed`: "Failed to convert {path}: {error}"
- `OutputDirError`: "Could not create output directory {path}: {error}"
- `OutputWriteFailed`: "Could not write output file {path}: {error}"
- `InvalidArgs`: "{message}"

---

## Validation Rules

### Pre-Processing Validation

1. **Input Exists**: Validate `input` path exists before processing
2. **Format Compatibility**: If `from` specified, validate input format matches
3. **Output Directory**: Create output directory if doesn't exist
4. **Argument Conflicts**: Check mutually exclusive flags (verbose vs quiet)

### During Processing Validation

1. **Extension Detection**: Validate file has recognizable extension
2. **Backend Availability**: Verify backend exists for format
3. **Configuration Validity**: Validate PdfConfig options are compatible

### Post-Processing Validation

1. **Output Generation**: Verify output files created successfully
2. **File Size**: Warn if output is empty (possible conversion issue)

---

## Performance Characteristics

### Memory Usage

- **CliArgs**: O(1) - small struct with vectors of enums
- **ConversionJob**: O(1) - lightweight wrapper
- **BatchProgress**: O(N) where N = number of files (stores errors)
- **Per-File Processing**: O(1) - one file at a time, no batching in memory

### Time Complexity

- **Argument Parsing**: O(1) - clap handles internally
- **Directory Scanning**: O(N) - N files in directory
- **Batch Processing**: O(N * C) - N files, C = conversion time per file
- **Progress Updates**: O(1) per file

---

## Testing Considerations

### Data Model Tests

1. **CliArgs Parsing**: Verify clap correctly parses all flags
2. **Format Detection**: Test extension → InputFormat mapping
3. **Output Naming**: Verify filename generation logic
4. **Config Conversion**: Test CliArgs → ConversionConfig
5. **BatchProgress**: Test tracking logic, edge cases (all success, all fail)

### Integration Tests

1. **Single File Flow**: CliArgs → ConversionJob → Result
2. **Batch Flow**: Directory → Jobs → BatchProgress → Summary
3. **Error Scenarios**: Invalid paths, unsupported formats, conversion failures

---

## Future Extensions

**Not in Current Scope** (for reference):

1. **URL Input**: Add URL variant to input field
2. **Config Files**: Support loading CliArgs from TOML/YAML
3. **Watch Mode**: File watching for continuous conversion
4. **Parallel Processing**: Process multiple files concurrently
5. **Resume**: Support resuming interrupted batch operations

---

## References

- Spec: [spec.md](./spec.md) - Functional requirements
- Research: [research.md](./research.md) - Technology decisions
- Contract: [contracts/cli_interface.md](./contracts/cli_interface.md) - CLI interface contract
