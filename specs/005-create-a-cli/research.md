# Research: CLI for docling-rs

**Feature**: Command-Line Interface
**Date**: 2025-10-05
**Purpose**: Technical research and technology selection for CLI implementation

## Research Questions

### 1. CLI Argument Parsing Framework

**Question**: Which Rust CLI framework best fits our needs?

**Decision**: `clap` v4.5+ with derive macros

**Rationale**:
- **Industry Standard**: Most widely used Rust CLI framework (28M+ downloads/month)
- **Excellent UX**: Auto-generated help messages, suggestions for typos, rich error messages
- **Derive API**: Clean, declarative argument definitions via procedural macros
- **Validation**: Built-in validation, custom validators, value parsers
- **Features**: Subcommands, flag groups, environment variables, config files
- **Maintenance**: Actively maintained by clap-rs organization
- **Documentation**: Comprehensive docs and examples

**Alternatives Considered**:
1. **structopt** (deprecated)
   - Merged into clap v3+
   - No longer maintained
   - Rejected: Use clap directly

2. **argh** (Google's minimal parser)
   - Pros: Smaller compile times, minimal dependencies
   - Cons: Fewer features (no bash completion, less validation)
   - Rejected: Missing features we need (progress, rich errors)

3. **pico-args** (minimal parser)
   - Pros: Tiny, fast
   - Cons: Manual parsing, no derive macros
   - Rejected: Too low-level for our needs

**Implementation Notes**:
- Use `#[derive(Parser)]` for main Args struct
- Use `#[command]` attributes for help text and metadata
- Leverage `ValueEnum` for format types
- Use `value_parser!` for PathBuf validation

**Example**:
```rust
#[derive(Parser)]
#[command(name = "docling-rs")]
#[command(about = "Document conversion tool")]
struct CliArgs {
    #[arg(help = "Input file or directory")]
    input: PathBuf,

    #[arg(long, short = 'o')]
    output: Option<PathBuf>,

    #[arg(long, value_enum)]
    to: Vec<OutputFormat>,
}
```

---

### 2. Progress Indication

**Question**: How should we show progress for batch operations?

**Decision**: `indicatif` v0.17+ for progress bars

**Rationale**:
- **Pure Rust**: No C dependencies, cross-platform
- **Terminal Integration**: Handles terminal resize, cursor positioning
- **Rich Features**: Spinners, progress bars, multi-progress, custom styles
- **Non-Blocking**: Works with both sync and async code
- **Small Footprint**: Minimal dependencies
- **Active Maintenance**: Regular updates, responsive maintainers

**Alternatives Considered**:
1. **pbr** (progress bar crate)
   - Pros: Simple API
   - Cons: Less maintained (last update 2 years ago)
   - Rejected: Prefer actively maintained

2. **Manual progress printing**
   - Pros: Zero dependencies
   - Cons: Harder to get right (terminal handling, formatting)
   - Rejected: indicatif is worth the dependency

**Implementation Notes**:
- Use `ProgressBar::new(len)` for batch operations
- Use `ProgressStyle::with_template()` for custom format
- Update on each file completion
- Clear on error or completion

**Example**:
```rust
let pb = ProgressBar::new(files.len() as u64);
pb.set_style(ProgressStyle::with_template(
    "[{bar:40.cyan/blue}] {pos}/{len} {msg}"
)?);

for file in files {
    pb.set_message(format!("Processing {}", file.display()));
    // ... conversion
    pb.inc(1);
}
pb.finish_with_message("Complete");
```

---

### 3. Binary vs Library Architecture

**Question**: Should CLI be in main.rs or separate binary?

**Decision**: Separate binary target at `src/bin/docling-rs.rs`

**Rationale**:
- **Library Reusability**: Keeps docling-rs usable as library dependency
- **Clean Separation**: CLI code separate from library code
- **Standard Pattern**: Matches Rust ecosystem conventions (cargo, rustc, etc.)
- **Multiple Binaries**: Allows future additional binaries if needed
- **Testing**: Easier to test library without CLI dependencies

**Alternatives Considered**:
1. **Single main.rs**
   - Pros: Simpler structure
   - Cons: Library users get CLI dependencies
   - Rejected: Violates library-first architecture

2. **Workspace with separate crates**
   - Pros: Complete isolation
   - Cons: Overkill for thin CLI wrapper
   - Rejected: Too much overhead

**Implementation Notes**:
- Create `src/bin/docling-rs.rs` as binary entry point
- Create `src/cli/` module for CLI logic
- Binary imports from library crate
- Keep CLI dependencies out of library

**Cargo.toml Structure**:
```toml
[package]
name = "docling-rs"

[[bin]]
name = "docling-rs"
path = "src/bin/docling-rs.rs"

[dependencies]
# Library dependencies (minimal)
serde = "1.0"
# ...

[dev-dependencies]
# CLI-only dependencies don't affect library
clap = "4.5"
indicatif = "0.17"
```

---

### 4. Output Format Generation

**Question**: How should we generate different output formats?

**Decision**: Leverage existing DoclingDocument serialization + format-specific logic

**Rationale**:
- **Markdown**: Use existing export logic (already implemented)
- **JSON**: Use serde_json::to_string_pretty(&doc)
- **Text**: Extract text content from DoclingDocument nodes
- **Reuse**: No new serialization code needed
- **Consistency**: Same output as library API

**Alternatives Considered**:
1. **Custom formatters for each backend**
   - Pros: More control
   - Cons: Duplicate serialization logic
   - Rejected: DRY principle

2. **Template-based formatting**
   - Pros: Flexible output
   - Cons: Adds complexity, new dependency
   - Rejected: YAGNI

**Implementation Notes**:
- JSON: `serde_json::to_string_pretty(&doc)?`
- Markdown: `doc.to_markdown()?` (existing method)
- Text: Walk DoclingDocument tree, collect text nodes
- Each format = function: `fn write_format(doc: &DoclingDocument, path: &Path) -> Result<()>`

---

### 5. File Format Detection

**Question**: How should we detect input file format?

**Decision**: Extension-based detection with fallback to manifest

**Rationale**:
- **Simple**: Reliable, fast, no magic numbers
- **User Expectations**: Users name files with correct extensions
- **Explicit Override**: `--from` flag allows manual specification
- **Error Handling**: Clear error if format unsupported

**Alternatives Considered**:
1. **Magic number detection** (file content inspection)
   - Pros: More accurate
   - Cons: Slower, more complex, overkill
   - Rejected: Extensions sufficient for this use case

2. **Forced --from flag**
   - Pros: Explicit
   - Cons: Poor UX, extra typing
   - Rejected: Auto-detection better UX

**Implementation**:
```rust
fn detect_format(path: &Path) -> Result<InputFormat> {
    match path.extension().and_then(|s| s.to_str()) {
        Some("pdf") => Ok(InputFormat::PDF),
        Some("md") => Ok(InputFormat::Markdown),
        Some("html") => Ok(InputFormat::HTML),
        Some("csv") => Ok(InputFormat::CSV),
        Some("docx") => Ok(InputFormat::DOCX),
        _ => Err("Unsupported format. Use --from to specify.")
    }
}
```

---

### 6. Error Handling Strategy

**Question**: How should CLI errors be handled and displayed?

**Decision**: Use `anyhow` for error context, stderr for output, exit codes

**Rationale**:
- **anyhow**: Provides context chains for debugging
- **stderr**: Standard practice for CLI errors
- **Exit Codes**: POSIX conventions (0=success, 1=error)
- **User-Friendly**: Clear, actionable error messages

**Error Categories**:
1. **Usage Errors** (exit 1): Invalid arguments, missing files
2. **Conversion Errors** (exit 1): Backend failures, corrupt files
3. **System Errors** (exit 1): I/O errors, permissions

**Implementation Pattern**:
```rust
fn main() -> ExitCode {
    if let Err(e) = run() {
        eprintln!("Error: {:#}", e);  // Pretty context chain
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

fn run() -> anyhow::Result<()> {
    let args = CliArgs::parse();
    // ... conversion logic with ? operator
    Ok(())
}
```

---

### 7. Batch Processing Strategy

**Question**: How should directory batch processing work?

**Decision**: Sequential processing with fail-fast option

**Rationale**:
- **Sequential**: Predictable resource usage, simpler error handling
- **Fail-Fast Option**: `--abort-on-error` for strict mode
- **Continue by Default**: Maximize successful conversions
- **Progress Tracking**: Show success/failure count

**Alternatives Considered**:
1. **Parallel processing**
   - Pros: Faster for large batches
   - Cons: More complex, resource usage unpredictable
   - Rejected: YAGNI, can add later if needed

2. **Always fail-fast**
   - Pros: Simpler
   - Cons: Poor UX for batch jobs
   - Rejected: Continue-on-error is better default

**Implementation**:
```rust
fn process_batch(files: Vec<PathBuf>, args: &CliArgs) -> Result<()> {
    let mut errors = Vec::new();

    for file in files {
        match convert_file(&file, args) {
            Ok(_) => {},
            Err(e) => {
                eprintln!("Error processing {}: {}", file.display(), e);
                errors.push(e);
                if args.abort_on_error {
                    return Err(anyhow!("Aborting on first error"));
                }
            }
        }
    }

    if !errors.is_empty() && !args.abort_on_error {
        eprintln!("Completed with {} errors", errors.len());
    }
    Ok(())
}
```

---

## Dependencies Summary

### Required Dependencies
```toml
[dependencies]
# CLI framework
clap = { version = "4.5", features = ["derive"] }

# Progress bars
indicatif = "0.17"

# Error handling
anyhow = "1.0"

# Already in project (library deps)
serde_json = "1.0"
```

### Dev Dependencies
```toml
[dev-dependencies]
assert_cmd = "2.0"  # CLI testing
predicates = "3.0"  # Assertion helpers
tempfile = "3.8"    # Temporary test files
```

---

## Performance Considerations

### Startup Time
- **Target**: <100ms to first output
- **Measurement**: `time docling-rs --help`
- **Optimization**: Minimal imports in main(), lazy initialization

### Memory Usage
- **Target**: O(1) memory for batch (stream files)
- **Implementation**: Process files one at a time, don't load all at once
- **Large PDFs**: Let pdfium handle streaming

### Conversion Speed
- **Target**: <5s for simple PDFs
- **Bottleneck**: PDF parsing (pdfium), not CLI overhead
- **CLI Overhead**: <50ms (argument parsing, file I/O setup)

---

## Testing Strategy

### Contract Tests
- Argument parsing (`clap` integration)
- Exit codes
- Error message format
- Help text generation

### Integration Tests
- End-to-end file conversion
- Batch directory processing
- Multiple output formats
- Error scenarios

### Cross-Platform Tests
- Path handling (Windows vs Unix)
- Exit codes (consistent across platforms)
- CI: macOS + Windows runners

---

## Open Questions

*None - all decisions made*

---

## References

- [clap documentation](https://docs.rs/clap/)
- [indicatif documentation](https://docs.rs/indicatif/)
- [Rust CLI Book](https://rust-cli.github.io/book/)
- [docling Python CLI reference](https://docling-project.github.io/docling/reference/cli/)
