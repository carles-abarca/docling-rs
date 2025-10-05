//! Contract tests for InputDocument

use docling_rs::datamodel::{DocumentSource, InputDocument};
use docling_rs::InputFormat;
use std::path::PathBuf;

#[test]
fn test_inputdocument_from_path() {
    let path = PathBuf::from("test.md");
    let input = InputDocument::from_path(path.clone(), InputFormat::Markdown);

    assert_eq!(input.format(), InputFormat::Markdown);
    match input.source() {
        DocumentSource::FilePath(p) => assert_eq!(p, &path),
        _ => unreachable!("Expected FilePath source"),
    }
}

#[test]
fn test_inputdocument_from_bytes() {
    let bytes = b"# Test".to_vec();
    let input = InputDocument::from_bytes(bytes.clone(), "test.md", InputFormat::Markdown);

    assert_eq!(input.format(), InputFormat::Markdown);
    match input.source() {
        DocumentSource::Bytes { data, name } => {
            assert_eq!(data, &bytes);
            assert_eq!(name, "test.md");
        }
        _ => unreachable!("Expected Bytes source"),
    }
}

#[test]
fn test_inputdocument_serialization() {
    let input = InputDocument::from_path(PathBuf::from("test.md"), InputFormat::Markdown);
    let json = serde_json::to_string(&input).expect("Should serialize");
    let _deserialized: InputDocument = serde_json::from_str(&json).expect("Should deserialize");
}
