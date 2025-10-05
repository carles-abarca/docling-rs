//! Contract tests for DoclingDocument
//!
//! These tests define the contract for DoclingDocument, ensuring it:
//! - Can be created from builder pattern
//! - Supports serialization/deserialization
//! - Contains nodes and metadata
//! - Provides access to document structure

use docling_rs::datamodel::{
    ConversionMetrics, ConversionResult, ConversionStatus, DoclingDocument,
};
use serde_json;

#[test]
fn test_doclingdocument_new() {
    let doc = DoclingDocument::new("test.md");
    assert_eq!(doc.name(), "test.md");
    assert!(doc.nodes().is_empty());
}

#[test]
fn test_doclingdocument_serialization() {
    let doc = DoclingDocument::new("test.md");

    // Should serialize to JSON
    let json = serde_json::to_string(&doc).expect("Should serialize");
    assert!(!json.is_empty());

    // Should deserialize from JSON
    let deserialized: DoclingDocument = serde_json::from_str(&json).expect("Should deserialize");
    assert_eq!(deserialized.name(), doc.name());
}

#[test]
fn test_doclingdocument_with_metadata() {
    let doc = DoclingDocument::new("test.md").with_metadata("key", "value");

    assert_eq!(
        doc.metadata().get("key"),
        Some(&serde_json::Value::String("value".to_string()))
    );
}

#[test]
fn test_doclingdocument_conversion_result() {
    let doc = DoclingDocument::new("test.md");
    let result = ConversionResult::new(doc, ConversionStatus::Success);

    assert_eq!(result.status(), ConversionStatus::Success);
    assert_eq!(result.document().name(), "test.md");
}

#[test]
fn test_doclingdocument_metrics() {
    let metrics = ConversionMetrics::new()
        .with_total_pages(10)
        .with_processing_time_ms(250);

    assert_eq!(metrics.total_pages(), 10);
    assert_eq!(metrics.processing_time_ms(), 250);
}
