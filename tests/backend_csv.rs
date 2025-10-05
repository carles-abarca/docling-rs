//! CSV backend tests

use docling_rs::backend::{Backend, CsvBackend};
use docling_rs::datamodel::InputDocument;
use docling_rs::InputFormat;

#[test]
fn test_csv_backend_supports_format() {
    let backend = CsvBackend::new();

    assert!(backend.supports_format(InputFormat::Csv));
    assert!(!backend.supports_format(InputFormat::Markdown));
    assert!(!backend.supports_format(InputFormat::Html));
    assert!(!backend.supports_format(InputFormat::Docx));
}

#[test]
fn test_csv_backend_convert_simple() {
    let backend = CsvBackend::new();
    let csv_data = b"Name,Age\nAlice,30\nBob,25\n".to_vec();
    let input = InputDocument::from_bytes(csv_data, "test.csv", InputFormat::Csv);

    let result = backend.convert(&input);
    assert!(result.is_ok());

    let doc = result.unwrap();
    assert_eq!(doc.name(), "test.csv");
}

#[test]
fn test_csv_backend_convert_with_quotes() {
    let backend = CsvBackend::new();
    let csv_data = b"\"Name\",\"Value\"\n\"Test\",\"123\"\n".to_vec();
    let input = InputDocument::from_bytes(csv_data, "test.csv", InputFormat::Csv);

    let result = backend.convert(&input);
    assert!(result.is_ok());
}

#[test]
fn test_csv_backend_convert_empty() {
    let backend = CsvBackend::new();
    let csv_data = b"".to_vec();
    let input = InputDocument::from_bytes(csv_data, "empty.csv", InputFormat::Csv);

    let result = backend.convert(&input);
    assert!(result.is_ok());
}

#[test]
fn test_csv_backend_convert_single_column() {
    let backend = CsvBackend::new();
    let csv_data = b"Header\nValue1\nValue2\n".to_vec();
    let input = InputDocument::from_bytes(csv_data, "test.csv", InputFormat::Csv);

    let result = backend.convert(&input);
    assert!(result.is_ok());
}
