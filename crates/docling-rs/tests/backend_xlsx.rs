//! XLSX backend tests

use docling_rs::backend::{Backend, XlsxBackend};
use docling_rs::datamodel::InputDocument;
use docling_rs::InputFormat;
use std::path::PathBuf;

#[test]
fn test_xlsx_backend_supports_format() {
    let backend = XlsxBackend::new();

    assert!(backend.supports_format(InputFormat::Xlsx));
    assert!(!backend.supports_format(InputFormat::Markdown));
    assert!(!backend.supports_format(InputFormat::Html));
    assert!(!backend.supports_format(InputFormat::Csv));
    assert!(!backend.supports_format(InputFormat::Docx));
    assert!(!backend.supports_format(InputFormat::Pptx));
}

#[test]
fn test_xlsx_backend_convert_invalid() {
    let backend = XlsxBackend::new();

    // Test with invalid bytes (empty)
    let xlsx_data = vec![];
    let input = InputDocument::from_bytes(xlsx_data, "test.xlsx", InputFormat::Xlsx);

    // Should fail gracefully with a parse error
    let result = backend.convert(&input);
    assert!(result.is_err());
}

#[test]
fn test_xlsx_backend_convert_from_file() {
    let backend = XlsxBackend::new();

    // Use the test fixture
    let path = PathBuf::from("tests/documents-test/Indian_CEOs.xlsx");
    if !path.exists() {
        eprintln!("Test fixture not found, skipping test");
        return;
    }

    let input = InputDocument::from_path(path.clone(), InputFormat::Xlsx);
    let result = backend.convert(&input);

    assert!(result.is_ok(), "Failed to convert XLSX: {:?}", result.err());

    let doc = result.unwrap();
    assert_eq!(doc.name(), "Indian_CEOs.xlsx");

    // Document should have content (sheets with data)
    let nodes = doc.nodes();
    assert!(!nodes.is_empty(), "Document should have nodes");
}

#[test]
fn test_xlsx_backend_convert_from_bytes() {
    let backend = XlsxBackend::new();

    // Read the test fixture as bytes
    let path = PathBuf::from("tests/documents-test/Indian_CEOs.xlsx");
    if !path.exists() {
        eprintln!("Test fixture not found, skipping test");
        return;
    }

    let bytes = std::fs::read(&path).expect("Failed to read test fixture");
    let input = InputDocument::from_bytes(bytes, "Indian_CEOs.xlsx", InputFormat::Xlsx);

    let result = backend.convert(&input);
    assert!(
        result.is_ok(),
        "Failed to convert XLSX from bytes: {:?}",
        result.err()
    );

    let doc = result.unwrap();
    assert_eq!(doc.name(), "Indian_CEOs.xlsx");
}

#[test]
fn test_xlsx_backend_default() {
    // Test Default implementation
    let backend: XlsxBackend = Default::default();
    assert!(backend.supports_format(InputFormat::Xlsx));
}
