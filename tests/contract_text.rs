//! Contract tests for Text types

use docling_rs::datamodel::{Formatting, TextItem};

#[test]
fn test_textitem_new() {
    let text = TextItem::new("Hello, world!");

    assert_eq!(text.content(), "Hello, world!");
    assert!(text.formatting().is_empty());
}

#[test]
fn test_textitem_with_formatting() {
    let formatting = Formatting::bold();
    let text = TextItem::new("Bold text").with_formatting(formatting.clone());

    assert_eq!(text.content(), "Bold text");
    assert!(text.formatting().contains(&formatting));
}

#[test]
fn test_formatting_types() {
    assert!(Formatting::bold().is_bold());
    assert!(Formatting::italic().is_italic());
    assert!(Formatting::code().is_code());
}

#[test]
fn test_textitem_serialization() {
    let text = TextItem::new("Test").with_formatting(Formatting::bold());
    let json = serde_json::to_string(&text).expect("Should serialize");
    let _deserialized: TextItem = serde_json::from_str(&json).expect("Should deserialize");
}
