//! DocumentConverter integration tests

use docling_rs::datamodel::ConversionStatus;
use docling_rs::DocumentConverter;
use std::io::Write;

#[test]
fn test_converter_new() {
    let converter = DocumentConverter::new();
    // Converter should be created successfully
    assert!(true);
}

#[test]
fn test_converter_convert_markdown_bytes() {
    let converter = DocumentConverter::new();
    let result = converter.convert_bytes(
        b"# Hello\n\nWorld".to_vec(),
        "test.md".to_string(),
        docling_rs::InputFormat::Markdown,
    );

    assert!(result.is_ok());
    let conv_result = result.unwrap();
    assert_eq!(conv_result.status(), ConversionStatus::Success);
    assert_eq!(conv_result.document().name(), "test.md");
}

#[test]
fn test_converter_convert_html_bytes() {
    let converter = DocumentConverter::new();
    let result = converter.convert_bytes(
        b"<html><body><h1>Test</h1></body></html>".to_vec(),
        "test.html".to_string(),
        docling_rs::InputFormat::Html,
    );

    assert!(result.is_ok());
    assert_eq!(result.unwrap().status(), ConversionStatus::Success);
}

#[test]
fn test_converter_convert_csv_bytes() {
    let converter = DocumentConverter::new();
    let result = converter.convert_bytes(
        b"Name,Value\nTest,123\n".to_vec(),
        "test.csv".to_string(),
        docling_rs::InputFormat::Csv,
    );

    assert!(result.is_ok());
    assert_eq!(result.unwrap().status(), ConversionStatus::Success);
}

#[test]
fn test_converter_convert_markdown_file() {
    let converter = DocumentConverter::new();

    // Create a temporary file with .md extension
    let mut temp_file = tempfile::Builder::new()
        .suffix(".md")
        .tempfile()
        .expect("Failed to create temp file");
    temp_file
        .write_all(b"# Test Heading\n\nParagraph text.")
        .expect("Failed to write to temp file");

    let result = converter.convert_file(temp_file.path());
    assert!(result.is_ok());
    assert_eq!(result.unwrap().status(), ConversionStatus::Success);
}

#[test]
fn test_converter_convert_file_not_found() {
    let converter = DocumentConverter::new();
    let result = converter.convert_file("/nonexistent/file.md");

    assert!(result.is_err());
}
