//! PPTX backend tests

use docling_rs::backend::{Backend, PptxBackend};
use docling_rs::datamodel::InputDocument;
use docling_rs::InputFormat;
use std::path::PathBuf;

#[test]
fn test_pptx_backend_supports_format() {
    let backend = PptxBackend::new();

    assert!(backend.supports_format(InputFormat::Pptx));
    assert!(!backend.supports_format(InputFormat::Markdown));
    assert!(!backend.supports_format(InputFormat::Html));
    assert!(!backend.supports_format(InputFormat::Csv));
    assert!(!backend.supports_format(InputFormat::Docx));
    assert!(!backend.supports_format(InputFormat::Xlsx));
}

#[test]
fn test_pptx_backend_convert_invalid() {
    let backend = PptxBackend::new();

    // Test with invalid bytes (empty)
    let pptx_data = vec![];
    let input = InputDocument::from_bytes(pptx_data, "test.pptx", InputFormat::Pptx);

    // Should fail gracefully with a parse error
    let result = backend.convert(&input);
    assert!(result.is_err());
}

#[test]
fn test_pptx_backend_convert_from_file() {
    let backend = PptxBackend::new();

    // Use the test fixture
    let path = PathBuf::from(
        "tests/documents-test/RoadmaptoAccelerateAIStrategyforHigherEducationCIOs_829565.pptx",
    );
    if !path.exists() {
        eprintln!("Test fixture not found, skipping test");
        return;
    }

    let input = InputDocument::from_path(path.clone(), InputFormat::Pptx);
    let result = backend.convert(&input);

    assert!(result.is_ok(), "Failed to convert PPTX: {:?}", result.err());

    let doc = result.unwrap();
    assert_eq!(
        doc.name(),
        "RoadmaptoAccelerateAIStrategyforHigherEducationCIOs_829565.pptx"
    );
}

#[test]
fn test_pptx_backend_convert_from_bytes() {
    let backend = PptxBackend::new();

    // Read the test fixture as bytes
    let path = PathBuf::from(
        "tests/documents-test/RoadmaptoAccelerateAIStrategyforHigherEducationCIOs_829565.pptx",
    );
    if !path.exists() {
        eprintln!("Test fixture not found, skipping test");
        return;
    }

    let bytes = std::fs::read(&path).expect("Failed to read test fixture");
    let input = InputDocument::from_bytes(bytes, "presentation.pptx", InputFormat::Pptx);

    let result = backend.convert(&input);
    assert!(
        result.is_ok(),
        "Failed to convert PPTX from bytes: {:?}",
        result.err()
    );

    let doc = result.unwrap();
    assert_eq!(doc.name(), "presentation.pptx");
}

#[test]
fn test_pptx_backend_default() {
    // Test Default implementation
    let backend: PptxBackend = Default::default();
    assert!(backend.supports_format(InputFormat::Pptx));
}
