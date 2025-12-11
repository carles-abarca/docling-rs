//! End-to-end integration tests

use docling_rs::datamodel::ConversionStatus;
use docling_rs::DocumentConverter;
use std::io::Write;

#[test]
fn test_e2e_markdown_to_document() {
    let converter = DocumentConverter::new();
    let markdown = r#"
# Document Title

This is a paragraph with **bold** and *italic* text.

## Section 2

- List item 1
- List item 2
- List item 3

```rust
fn main() {
    println!("Hello, world!");
}
```
"#;

    let result = converter.convert_bytes(
        markdown.as_bytes().to_vec(),
        "test.md".to_string(),
        docling_rs::InputFormat::Markdown,
    );

    assert!(result.is_ok());
    let conv_result = result.unwrap();
    assert_eq!(conv_result.status(), ConversionStatus::Success);
    assert_eq!(conv_result.document().name(), "test.md");
}

#[test]
fn test_e2e_html_to_document() {
    let converter = DocumentConverter::new();
    let html = r#"
<!DOCTYPE html>
<html>
<head><title>Test Document</title></head>
<body>
    <h1>Main Heading</h1>
    <p>This is a paragraph.</p>
    <ul>
        <li>Item 1</li>
        <li>Item 2</li>
    </ul>
    <table>
        <tr><th>Name</th><th>Value</th></tr>
        <tr><td>A</td><td>1</td></tr>
        <tr><td>B</td><td>2</td></tr>
    </table>
</body>
</html>
"#;

    let result = converter.convert_bytes(
        html.as_bytes().to_vec(),
        "test.html".to_string(),
        docling_rs::InputFormat::Html,
    );

    assert!(result.is_ok());
    assert_eq!(result.unwrap().status(), ConversionStatus::Success);
}

#[test]
fn test_e2e_csv_to_document() {
    let converter = DocumentConverter::new();
    let csv = r#"Name,Age,City,Occupation
Alice,30,New York,Engineer
Bob,25,San Francisco,Designer
Charlie,35,Seattle,Manager
Diana,28,Boston,Developer
"#;

    let result = converter.convert_bytes(
        csv.as_bytes().to_vec(),
        "test.csv".to_string(),
        docling_rs::InputFormat::Csv,
    );

    assert!(result.is_ok());
    assert_eq!(result.unwrap().status(), ConversionStatus::Success);
}

#[test]
fn test_e2e_multiple_formats() {
    let converter = DocumentConverter::new();

    // Convert Markdown
    let md_result = converter.convert_bytes(
        b"# Test".to_vec(),
        "test.md".to_string(),
        docling_rs::InputFormat::Markdown,
    );
    assert!(md_result.is_ok());

    // Convert HTML
    let html_result = converter.convert_bytes(
        b"<h1>Test</h1>".to_vec(),
        "test.html".to_string(),
        docling_rs::InputFormat::Html,
    );
    assert!(html_result.is_ok());

    // Convert CSV
    let csv_result = converter.convert_bytes(
        b"A,B\n1,2\n".to_vec(),
        "test.csv".to_string(),
        docling_rs::InputFormat::Csv,
    );
    assert!(csv_result.is_ok());
}

#[test]
fn test_e2e_file_workflow() {
    let converter = DocumentConverter::new();

    // Create temp file
    let mut temp_file = tempfile::Builder::new()
        .suffix(".md")
        .tempfile()
        .expect("Failed to create temp file");

    temp_file
        .write_all(b"# Integration Test\n\nThis tests the full file conversion workflow.")
        .expect("Failed to write");

    // Convert file
    let result = converter.convert_file(temp_file.path());
    assert!(result.is_ok());

    let conv_result = result.unwrap();
    assert_eq!(conv_result.status(), ConversionStatus::Success);
}

#[test]
fn test_e2e_serialization() {
    let converter = DocumentConverter::new();
    let result = converter.convert_bytes(
        b"# Test\n\nContent".to_vec(),
        "test.md".to_string(),
        docling_rs::InputFormat::Markdown,
    );

    assert!(result.is_ok());
    let conv_result = result.unwrap();

    // Serialize to JSON
    let json = serde_json::to_string(&conv_result).expect("Should serialize");
    assert!(!json.is_empty());

    // Deserialize from JSON
    let _deserialized: docling_rs::datamodel::ConversionResult =
        serde_json::from_str(&json).expect("Should deserialize");
}
