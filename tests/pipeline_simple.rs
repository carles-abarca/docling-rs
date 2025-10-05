//! SimplePipeline tests

use docling_rs::datamodel::{ConversionStatus, InputDocument};
use docling_rs::pipeline::{Pipeline, SimplePipeline};
use docling_rs::InputFormat;

#[test]
fn test_simple_pipeline_new() {
    let _pipeline = SimplePipeline::new();
    // Pipeline should be created successfully
}

#[test]
fn test_simple_pipeline_execute_markdown() {
    let pipeline = SimplePipeline::new();
    let input = InputDocument::from_bytes(
        b"# Hello World\n\nThis is a test.".to_vec(),
        "test.md",
        InputFormat::Markdown,
    );

    let result = pipeline.execute(&input);
    assert!(result.is_ok());

    let conv_result = result.unwrap();
    assert_eq!(conv_result.status(), ConversionStatus::Success);
    assert_eq!(conv_result.document().name(), "test.md");
}

#[test]
fn test_simple_pipeline_execute_html() {
    let pipeline = SimplePipeline::new();
    let input = InputDocument::from_bytes(
        b"<html><body><h1>Test</h1></body></html>".to_vec(),
        "test.html",
        InputFormat::Html,
    );

    let result = pipeline.execute(&input);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().status(), ConversionStatus::Success);
}

#[test]
fn test_simple_pipeline_execute_csv() {
    let pipeline = SimplePipeline::new();
    let input = InputDocument::from_bytes(
        b"Name,Age\nAlice,30\n".to_vec(),
        "test.csv",
        InputFormat::Csv,
    );

    let result = pipeline.execute(&input);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().status(), ConversionStatus::Success);
}

#[test]
fn test_simple_pipeline_unsupported_format() {
    let pipeline = SimplePipeline::new();
    let input = InputDocument::from_bytes(vec![], "test.docx", InputFormat::Docx);

    // DOCX should fail with empty bytes
    let result = pipeline.execute(&input);
    assert!(result.is_err());
}
