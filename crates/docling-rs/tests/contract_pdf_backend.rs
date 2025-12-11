//! Contract test: PdfBackend implements Backend trait
//!
//! This test verifies that PdfBackend correctly implements the Backend trait contract.

mod helpers;
use helpers::pdf_fixtures::*;

use docling_rs::backend::{Backend, PdfBackend};
use docling_rs::datamodel::InputDocument;
use docling_rs::InputFormat;

#[test]
fn test_pdf_backend_implements_backend_trait() {
    // Arrange: Create a PdfBackend instance
    let backend = PdfBackend::new();

    // Assert: Backend trait is implemented (compile-time check)
    let _: &dyn Backend = &backend;
}

#[test]
fn test_pdf_backend_supports_pdf_format() {
    // Arrange
    let backend = PdfBackend::new();

    // Act
    let supports_pdf = backend.supports_format(InputFormat::PDF);
    let supports_md = backend.supports_format(InputFormat::Markdown);

    // Assert
    assert!(supports_pdf, "PdfBackend should support PDF format");
    assert!(
        !supports_md,
        "PdfBackend should not support Markdown format"
    );
}

#[test]
fn test_pdf_backend_convert_simple_pdf() {
    // This test verifies basic PDF conversion works

    // Arrange: Create a simple test PDF
    let test_pdf_path = create_simple_text_pdf("Test PDF content");
    let input = InputDocument::from_path(test_pdf_path, InputFormat::PDF);

    let backend = PdfBackend::new();

    // Act: Convert PDF
    let result = backend.convert(&input);

    // Assert: Conversion succeeds
    assert!(result.is_ok(), "PDF conversion should succeed");

    let doc = result.unwrap();
    assert!(!doc.nodes().is_empty(), "Document should have nodes");
}

// Helper functions now imported from helpers::pdf_fixtures
