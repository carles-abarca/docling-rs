//! Basic Document Conversion Example
//!
//! This example shows how to convert a document from one format to another.
//!
//! Run with:
//! ```bash
//! cargo run --example basic_conversion
//! ```

use docling_rs::DocumentConverter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Basic Document Conversion ===\n");

    // Create a new document converter
    let converter = DocumentConverter::new();

    // Example 1: Convert Markdown
    let markdown = "# Hello World\n\nThis is a **test** document with some content.";
    let result = converter.convert_bytes(
        markdown.as_bytes().to_vec(),
        "test.md".to_string(),
        docling_rs::InputFormat::Markdown,
    )?;

    println!("Converted Markdown:");
    println!("  Name: {}", result.document().name());
    println!("  Nodes: {}", result.document().nodes().len());
    println!("  Status: {:?}\n", result.status());

    // Example 2: Convert HTML
    let html = r#"<html><body><h1>Title</h1><p>Paragraph</p></body></html>"#;
    let result = converter.convert_bytes(
        html.as_bytes().to_vec(),
        "test.html".to_string(),
        docling_rs::InputFormat::Html,
    )?;

    println!("Converted HTML:");
    println!("  Name: {}", result.document().name());
    println!("  Nodes: {}", result.document().nodes().len());
    println!("  Status: {:?}\n", result.status());

    // Example 3: Convert CSV
    let csv = "Name,Age\nAlice,30\nBob,25";
    let result = converter.convert_bytes(
        csv.as_bytes().to_vec(),
        "test.csv".to_string(),
        docling_rs::InputFormat::Csv,
    )?;

    println!("Converted CSV:");
    println!("  Name: {}", result.document().name());
    println!("  Nodes: {}", result.document().nodes().len());
    println!("  Status: {:?}\n", result.status());

    Ok(())
}
