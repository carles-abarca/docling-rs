# Contributing to docling-rs

Thank you for your interest in contributing to docling-rs! This guide will help you get started.

## Development Setup

### Prerequisites

- Rust 1.75 or later
- Git

### Clone and Build

```bash
git clone https://github.com/your-org/docling-rs.git
cd docling-rs
cargo build
```

## Development Workflow

### 1. Run Tests

All tests must pass before submitting changes.

```bash
# Run all tests (PDF tests require single-threaded execution)
cargo test -- --test-threads=1

# Run specific test suite
cargo test --test integration_chunking

# Run with verbose output
cargo test -- --test-threads=1 --nocapture
```

**Important**: PDF-related tests must run with `--test-threads=1` due to pdfium's thread-safety requirements.

### 2. Code Style

We follow standard Rust conventions:

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt --check

# Run linter
cargo clippy

# Fix clippy warnings automatically (when possible)
cargo clippy --fix
```

### 3. Test-Driven Development (TDD)

This project follows TDD principles:

1. **Write failing tests first** - Before implementing a feature, write tests that define the expected behavior
2. **Implement minimal code** - Write just enough code to make the tests pass
3. **Refactor** - Improve code quality while keeping tests green
4. **Document** - Add doc comments and examples

Example:
```rust
#[test]
fn test_new_feature() {
    // Arrange
    let input = "test input";

    // Act
    let result = new_feature(input);

    // Assert
    assert_eq!(result, expected_output);
}
```

### 4. Running Examples

Test your changes with the examples:

```bash
cargo run --example basic_conversion
cargo run --example chunking_rag
cargo run --example json_serialization
```

## Project Structure

```
docling-rs/
├── src/
│   ├── lib.rs              # Library entry point
│   ├── converter.rs        # DocumentConverter
│   ├── backend/            # Format backends
│   │   ├── markdown.rs
│   │   ├── html.rs
│   │   ├── csv.rs
│   │   ├── docx.rs
│   │   └── pdf.rs
│   ├── chunking/           # Chunking implementations
│   │   ├── base.rs         # BaseChunker trait
│   │   ├── hierarchical.rs # HierarchicalChunker
│   │   ├── hybrid.rs       # HybridChunker
│   │   └── tokenizer.rs    # Tokenizer implementations
│   ├── datamodel/          # Core data structures
│   └── cli/                # CLI implementation
├── tests/                  # Integration tests
│   ├── integration_*.rs    # Test suites
│   └── helpers/            # Test utilities
├── examples/               # Usage examples
└── specs/                  # Design documents
```

## Adding a New Feature

### 1. Create Specification

Document your feature in `specs/`:

```bash
mkdir specs/NNN-feature-name
cd specs/NNN-feature-name
```

Create these files:
- `spec.md` - Feature specification
- `plan.md` - Implementation plan
- `tasks.md` - Task breakdown

### 2. Write Tests

Create integration tests in `tests/`:

```rust
// tests/integration_new_feature.rs
use docling_rs::*;

#[test]
fn test_feature_basic_case() {
    // Test implementation
}
```

### 3. Implement Feature

Implement in `src/`:

```rust
// src/new_feature.rs
pub struct NewFeature {
    // Implementation
}

impl NewFeature {
    pub fn new() -> Self {
        // Constructor
    }
}
```

### 4. Add Examples

Create example in `examples/`:

```rust
// examples/new_feature.rs
//! Example demonstrating NewFeature
//!
//! Run with:
//! ```bash
//! cargo run --example new_feature
//! ```

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example code
    Ok(())
}
```

Update `examples/README.md` with your new example.

### 5. Update Documentation

- Add doc comments to public APIs
- Update README.md if needed
- Update CHANGELOG.md

## Adding a New Format Backend

To add support for a new document format:

### 1. Create Backend Module

```rust
// src/backend/newformat.rs
use crate::backend::Backend;
use crate::datamodel::{DoclingDocument, InputDocument};
use crate::error::ConversionResult;

pub struct NewFormatBackend;

impl Backend for NewFormatBackend {
    fn convert(&self, input: &InputDocument) -> ConversionResult<DoclingDocument> {
        // Parse format and convert to DoclingDocument
        todo!()
    }
}
```

### 2. Add to InputFormat Enum

```rust
// src/lib.rs
#[derive(Debug, Clone, Copy)]
pub enum InputFormat {
    // ...existing formats
    NewFormat,
}
```

### 3. Register in DocumentConverter

```rust
// src/converter.rs
impl DocumentConverter {
    fn get_backend(&self, format: InputFormat) -> Box<dyn Backend> {
        match format {
            // ...existing backends
            InputFormat::NewFormat => Box::new(NewFormatBackend::new()),
        }
    }
}
```

### 4. Write Integration Tests

```rust
// tests/integration_newformat.rs
#[test]
fn test_newformat_basic_conversion() {
    let converter = DocumentConverter::new();
    let input = b"format-specific content";

    let result = converter.convert_bytes(
        input.to_vec(),
        "test.ext".to_string(),
        InputFormat::NewFormat,
    );

    assert!(result.is_ok());
}
```

## Commit Guidelines

### Commit Message Format

```
type(scope): brief description

Detailed explanation (optional)

Fixes #123
```

**Types**:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `test`: Test additions/changes
- `refactor`: Code refactoring
- `perf`: Performance improvements
- `chore`: Maintenance tasks

**Examples**:
```
feat(chunking): Add hybrid chunker with token awareness

Implements HybridChunker that combines hierarchical chunking
with token-based splitting for RAG applications.

Closes #42
```

```
fix(pdf): Handle encrypted PDFs with correct password

Adds password support to PdfConfig and updates backend
to decrypt password-protected documents.

Fixes #58
```

## Pull Request Process

### 1. Create Feature Branch

```bash
git checkout -b feature/your-feature-name
```

### 2. Make Changes

- Write tests first (TDD)
- Implement feature
- Ensure all tests pass
- Run `cargo fmt` and `cargo clippy`

### 3. Commit Changes

```bash
git add .
git commit -m "feat(scope): description"
```

### 4. Push and Create PR

```bash
git push origin feature/your-feature-name
```

Create pull request on GitHub with:
- Clear description of changes
- Reference to related issues
- Test results
- Examples of usage (if applicable)

### 5. Review Process

- Maintainers will review your PR
- Address feedback and make requested changes
- Once approved, PR will be merged

## Testing Guidelines

### Test Organization

- **Unit tests**: In same file as implementation (using `#[cfg(test)]`)
- **Integration tests**: In `tests/` directory
- **Examples**: In `examples/` directory (serve as documentation)

### Test Naming

```rust
#[test]
fn test_component_action_expected() {
    // test_chunker_split_creates_multiple_chunks
    // test_backend_convert_handles_empty_input
}
```

### Test Structure (AAA Pattern)

```rust
#[test]
fn test_feature() {
    // Arrange - Set up test data
    let input = create_test_input();

    // Act - Execute the code under test
    let result = function_under_test(input);

    // Assert - Verify the results
    assert_eq!(result, expected_value);
}
```

### Test Coverage

Aim for:
- 100% coverage of public APIs
- Edge cases and error conditions
- Integration tests for major features

## Documentation

### Doc Comments

All public APIs must have doc comments:

```rust
/// Converts a document to the unified DoclingDocument format.
///
/// # Arguments
///
/// * `input` - The input document to convert
///
/// # Returns
///
/// Returns `Ok(ConversionResult)` on success, or `ConversionError` on failure.
///
/// # Examples
///
/// ```
/// use docling_rs::DocumentConverter;
///
/// let converter = DocumentConverter::new();
/// let result = converter.convert_file("doc.md")?;
/// ```
pub fn convert(&self, input: InputDocument) -> Result<ConversionResult, ConversionError> {
    // Implementation
}
```

### README Updates

When adding features:
- Update Quick Start section if API changes
- Add examples for new capabilities
- Update feature list
- Keep examples concise and focused

## Questions or Issues?

- Open an issue on GitHub
- Check existing issues and PRs first
- Provide minimal reproducible examples
- Include version information and environment details

## Code of Conduct

- Be respectful and constructive
- Welcome newcomers
- Focus on technical merit
- Help others learn and improve

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
