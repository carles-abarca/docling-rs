//! Markdown backend tests

use docling_rs::backend::{Backend, MarkdownBackend};
use docling_rs::datamodel::{InputDocument, NodeType};
use docling_rs::InputFormat;
use std::path::PathBuf;

#[test]
fn test_markdown_backend_supports_format() {
    let backend = MarkdownBackend::new();

    assert!(backend.supports_format(InputFormat::Markdown));
    assert!(!backend.supports_format(InputFormat::Html));
    assert!(!backend.supports_format(InputFormat::Csv));
    assert!(!backend.supports_format(InputFormat::Docx));
}

#[test]
fn test_markdown_backend_convert_heading() {
    let backend = MarkdownBackend::new();
    let input = InputDocument::from_bytes(
        b"# Hello World\n".to_vec(),
        "test.md",
        InputFormat::Markdown,
    );

    let result = backend.convert(&input);
    assert!(result.is_ok());

    let doc = result.unwrap();
    assert_eq!(doc.name(), "test.md");
    // Node parsing will be fully implemented in REFACTOR phase
}

#[test]
fn test_markdown_backend_convert_paragraph() {
    let backend = MarkdownBackend::new();
    let input = InputDocument::from_bytes(
        b"This is a paragraph.\n".to_vec(),
        "test.md",
        InputFormat::Markdown,
    );

    let result = backend.convert(&input);
    assert!(result.is_ok());
}

#[test]
fn test_markdown_backend_convert_list() {
    let backend = MarkdownBackend::new();
    let input = InputDocument::from_bytes(
        b"- Item 1\n- Item 2\n".to_vec(),
        "test.md",
        InputFormat::Markdown,
    );

    let result = backend.convert(&input);
    assert!(result.is_ok());
}

#[test]
fn test_markdown_backend_convert_code_block() {
    let backend = MarkdownBackend::new();
    let input = InputDocument::from_bytes(
        b"```rust\nfn main() {}\n```\n".to_vec(),
        "test.md",
        InputFormat::Markdown,
    );

    let result = backend.convert(&input);
    assert!(result.is_ok());
}
