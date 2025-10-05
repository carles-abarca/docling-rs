//! Contract tests for Backend trait

use docling_rs::backend::Backend;
use docling_rs::datamodel::{DoclingDocument, InputDocument};
use docling_rs::error::ConversionError;
use docling_rs::InputFormat;
use std::path::PathBuf;

struct MockBackend;

impl Backend for MockBackend {
    fn convert(&self, _input: &InputDocument) -> Result<DoclingDocument, ConversionError> {
        Ok(DoclingDocument::new("mock"))
    }

    fn supports_format(&self, format: InputFormat) -> bool {
        format == InputFormat::Markdown
    }
}

#[test]
fn test_backend_convert() {
    let backend = MockBackend;
    let input = InputDocument::from_path(PathBuf::from("test.md"), InputFormat::Markdown);

    let result = backend.convert(&input);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().name(), "mock");
}

#[test]
fn test_backend_supports_format() {
    let backend = MockBackend;

    assert!(backend.supports_format(InputFormat::Markdown));
    assert!(!backend.supports_format(InputFormat::Html));
}
