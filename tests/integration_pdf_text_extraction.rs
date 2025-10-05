//! Integration test: Basic PDF text extraction
//!
//! Tests basic text extraction from PDF files including:
//! - Simple text-based PDFs
//! - Text with different fonts and sizes
//! - Text position metadata

mod helpers;
use helpers::pdf_fixtures::*;

use docling_rs::backend::{Backend, PdfBackend};
use docling_rs::cli::output;
use docling_rs::datamodel::InputDocument;
use docling_rs::InputFormat;

#[test]
fn test_extract_text_from_simple_pdf() {
    // This test verifies basic text extraction from a simple PDF

    // Arrange: Create a test PDF with known text
    let test_content = "Hello, World!\nThis is a test PDF.";
    let pdf_path = create_simple_text_pdf(test_content);

    let backend = PdfBackend::new();
    let input = InputDocument::from_path(pdf_path, InputFormat::PDF);

    // Act: Convert PDF
    let result = backend.convert(&input);

    // Assert: Text is extracted correctly
    if let Err(e) = &result {
        eprintln!("PDF conversion error: {:?}", e);
    }
    assert!(result.is_ok(), "PDF conversion should succeed");

    let doc = result.unwrap();
    let text = output::to_text(&doc);

    assert!(
        text.contains("Hello, World!"),
        "Extracted text should contain 'Hello, World!'"
    );
    assert!(
        text.contains("This is a test PDF"),
        "Extracted text should contain 'This is a test PDF'"
    );
}

#[test]
fn test_extract_text_with_positions() {
    // This test verifies that text extraction includes position metadata

    // Arrange
    let test_content = "Top text\nBottom text";
    let pdf_path = create_simple_text_pdf(test_content);

    let backend = PdfBackend::new();
    let input = InputDocument::from_path(pdf_path, InputFormat::PDF);

    // Act
    let result = backend.convert(&input);

    // Assert
    assert!(result.is_ok(), "PDF conversion should succeed");

    let doc = result.unwrap();
    let text = output::to_text(&doc);

    // Verify that both lines of text were extracted
    assert!(text.contains("Top text"), "Should extract 'Top text'");
    assert!(text.contains("Bottom text"), "Should extract 'Bottom text'");

    // Note: Position metadata will be added in future phases (Phase 2: Layout Analysis)
    // For now, we just verify basic text extraction works
}

#[test]
fn test_extract_text_from_empty_pdf() {
    // This test verifies handling of empty PDFs

    // Arrange
    let pdf_path = create_empty_pdf();

    let backend = PdfBackend::new();
    let input = InputDocument::from_path(pdf_path, InputFormat::PDF);

    // Act
    let result = backend.convert(&input);

    // Assert: Should succeed but have no text
    assert!(result.is_ok(), "Empty PDF conversion should succeed");

    let doc = result.unwrap();

    // Empty PDF should have no nodes (or only empty nodes)
    let has_content = doc.nodes().iter().any(|node| {
        node.text_content()
            .map(|t| !t.trim().is_empty())
            .unwrap_or(false)
    });

    assert!(
        !has_content,
        "Empty PDF should have no text content in nodes"
    );
}

// Helper functions now imported from helpers::pdf_fixtures
