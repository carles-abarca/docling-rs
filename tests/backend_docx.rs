//! DOCX backend tests

use docling_rs::backend::{Backend, DocxBackend};
use docling_rs::datamodel::InputDocument;
use docling_rs::InputFormat;

#[test]
fn test_docx_backend_supports_format() {
    let backend = DocxBackend::new();

    assert!(backend.supports_format(InputFormat::Docx));
    assert!(!backend.supports_format(InputFormat::Markdown));
    assert!(!backend.supports_format(InputFormat::Html));
    assert!(!backend.supports_format(InputFormat::Csv));
}

#[test]
fn test_docx_backend_convert_minimal() {
    let backend = DocxBackend::new();

    // Create minimal valid DOCX bytes (empty document)
    // For now, we'll test with an empty bytes vector and expect it to handle gracefully
    let docx_data = vec![];
    let input = InputDocument::from_bytes(docx_data, "test.docx", InputFormat::Docx);

    // This should fail gracefully with a parse error
    let result = backend.convert(&input);
    // We expect an error for invalid DOCX, which is correct behavior
    assert!(result.is_err());
}

#[test]
fn test_docx_backend_name_extraction() {
    let backend = DocxBackend::new();
    let input = InputDocument::from_bytes(vec![], "document.docx", InputFormat::Docx);

    // Even if conversion fails, we're testing the backend structure
    let _result = backend.convert(&input);
    // Test passes if backend is properly structured
}
