//! SimplePipeline tests

use docling_rs::datamodel::{ConversionStatus, InputDocument};
use docling_rs::pipeline::{Pipeline, SimplePipeline};
use docling_rs::InputFormat;

#[test]
fn test_simple_pipeline_new() {
    let _pipeline = SimplePipeline::new();
    // Pipeline should be created successfully (empty, no backends)
}

#[test]
fn test_simple_pipeline_execute_markdown() {
    // Use create_pipeline() to get a pipeline with registered backends
    let pipeline = docling_rs::create_pipeline();
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
    let pipeline = docling_rs::create_pipeline();
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
    let pipeline = docling_rs::create_pipeline();
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
fn test_simple_pipeline_empty_has_no_backends() {
    let pipeline = SimplePipeline::new();
    let input = InputDocument::from_bytes(vec![], "test.md", InputFormat::Markdown);

    // Empty pipeline should fail since no backend is registered
    let result = pipeline.execute(&input);
    assert!(result.is_err());
}
