//! JSON Serialization Example
//!
//! This example shows how to convert documents to JSON.
//!
//! Run with:
//! ```bash
//! cargo run --example json_serialization
//! ```

use docling_rs::DocumentConverter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== JSON Serialization Example ===\n");

    let converter = DocumentConverter::new();

    // Convert a markdown document
    let markdown = "# Title\n\nParagraph with **bold** text.";
    let result = converter.convert_bytes(
        markdown.as_bytes().to_vec(),
        "doc.md".to_string(),
        docling_rs::InputFormat::Markdown,
    )?;

    // Serialize to JSON (pretty-printed)
    let json = serde_json::to_string_pretty(&result)?;
    println!("JSON Output:\n{}\n", json);

    // You can also serialize just the document
    let doc_json = serde_json::to_string_pretty(result.document())?;
    println!("Document Only:\n{}\n", doc_json);

    Ok(())
}
