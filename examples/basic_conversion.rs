//! Basic document conversion example
//!
//! Run with: cargo run --example basic_conversion

use docling_rs::{DocumentConverter, InputFormat};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Docling-rs Basic Conversion Example ===\n");

    let converter = DocumentConverter::new();

    // Example 1: Convert Markdown
    println!("1. Converting Markdown:");
    let markdown = r#"# Welcome to Docling-rs

This is a **Rust** library for document conversion.

## Features

- Fast and reliable
- Type-safe
- Cross-platform
"#;

    let result = converter.convert_bytes(
        markdown.as_bytes().to_vec(),
        "example.md".to_string(),
        InputFormat::Markdown,
    )?;

    println!("   Document: {}", result.document().name());
    println!("   Status: {:?}", result.status());
    println!("   Nodes: {}", result.document().nodes().len());
    println!();

    // Example 2: Convert HTML
    println!("2. Converting HTML:");
    let html = r#"
<!DOCTYPE html>
<html>
<head><title>Test</title></head>
<body>
    <h1>HTML Document</h1>
    <p>This is a paragraph.</p>
    <ul>
        <li>Item 1</li>
        <li>Item 2</li>
    </ul>
</body>
</html>
"#;

    let result = converter.convert_bytes(
        html.as_bytes().to_vec(),
        "example.html".to_string(),
        InputFormat::Html,
    )?;

    println!("   Document: {}", result.document().name());
    println!("   Status: {:?}", result.status());
    println!();

    // Example 3: Convert CSV
    println!("3. Converting CSV:");
    let csv = r#"Name,Role,Department
Alice,Engineer,Development
Bob,Designer,UX
Charlie,Manager,Operations
"#;

    let result = converter.convert_bytes(
        csv.as_bytes().to_vec(),
        "employees.csv".to_string(),
        InputFormat::Csv,
    )?;

    println!("   Document: {}", result.document().name());
    println!("   Status: {:?}", result.status());
    println!();

    // Example 4: Serialize to JSON
    println!("4. Serializing to JSON:");
    let json = serde_json::to_string_pretty(&result)?;
    println!("   JSON output (first 200 chars):");
    println!("   {}", &json.chars().take(200).collect::<String>());
    println!("   ...");
    println!();

    println!("=== All conversions completed successfully! ===");

    Ok(())
}
