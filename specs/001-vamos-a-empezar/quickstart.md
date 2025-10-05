# Quickstart Guide

## Installation

```toml
[dependencies]
docling-rs = "0.1.0"
```

## Basic Usage

### Convert Markdown File

```rust
use docling_rs::{DocumentConverter, ConversionResult};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create converter
    let converter = DocumentConverter::new();

    // Convert Markdown file
    let result = converter.convert_file("document.md")?;

    // Access document
    if let Some(doc) = result.document {
        println!("Title: {}", doc.get_text());

        // Export to JSON
        let json = doc.to_json()?;
        println!("{}", json);
    }

    Ok(())
}
```

### Convert from Bytes

```rust
use docling_rs::{DocumentConverter, InputFormat};

let markdown_bytes = b"# Hello\n\nWorld";
let result = converter.convert_bytes(
    markdown_bytes.to_vec(),
    "doc.md",
    InputFormat::Markdown
)?;
```

### Export to Markdown

```rust
let markdown_output = doc.to_markdown();
println!("{}", markdown_output);
```

## Format-Specific Examples

### HTML with Tables

```rust
let html = r#"
<html>
  <body>
    <h1>Report</h1>
    <table>
      <thead><tr><th>Name</th><th>Value</th></tr></thead>
      <tbody>
        <tr><td>A</td><td>1</td></tr>
        <tr><td>B</td><td>2</td></tr>
      </tbody>
    </table>
  </body>
</html>
"#;

let result = converter.convert_bytes(
    html.as_bytes().to_vec(),
    "report.html",
    InputFormat::Html
)?;
```

### CSV to Table

```rust
let csv_data = "Name,Age\nAlice,30\nBob,25";
let result = converter.convert_bytes(
    csv_data.as_bytes().to_vec(),
    "data.csv",
    InputFormat::Csv
)?;

// Access table data
if let Some(doc) = result.document {
    if let Some(table_node) = doc.root.children.first() {
        if let Some(table) = &table_node.table {
            println!("Headers: {:?}", table.headers);
            for row in &table.rows {
                println!("Row: {:?}", row);
            }
        }
    }
}
```

### DOCX with Formatting

```rust
let result = converter.convert_file("document.docx")?;

// Iterate nodes to find formatted text
for node in doc.iter_nodes() {
    if let Some(text) = &node.text {
        if let Some(fmt) = &text.formatting {
            if fmt.bold {
                println!("Bold text: {}", text.content);
            }
        }
    }
}
```

## Auto-Format Detection

```rust
// Format detected from extension or content
let result = converter.convert_file("unknown.md")?;
println!("Detected format: {:?}", result.input.format);
```

## Error Handling

```rust
use docling_rs::ConversionError;

match converter.convert_file("missing.md") {
    Ok(result) => println!("Success"),
    Err(ConversionError::FileNotFound(path)) => {
        eprintln!("File not found: {:?}", path);
    },
    Err(ConversionError::UnsupportedFormat(fmt)) => {
        eprintln!("Unsupported: {}", fmt);
    },
    Err(e) => eprintln!("Error: {}", e),
}
```

## Testing Your Integration

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_markdown_conversion() {
        let converter = DocumentConverter::new();
        let md = "# Test\n\nParagraph";

        let result = converter.convert_bytes(
            md.as_bytes().to_vec(),
            "test.md",
            InputFormat::Markdown
        ).unwrap();

        assert!(result.document.is_some());
        let doc = result.document.unwrap();

        // Verify structure
        assert_eq!(doc.metadata.format, InputFormat::Markdown);
        assert!(!doc.root.children.is_empty());
    }
}
```

## Advanced: Custom Backend Configuration

```rust
// Future: Backend-specific options
let converter = DocumentConverter::builder()
    .markdown_options(MarkdownOptions {
        gfm_tables: true,
        ..Default::default()
    })
    .build();
```

## Performance Tips

1. **Reuse Converter**: Create once, convert many files
2. **Stream Large Files**: Use async variants (future)
3. **Batch Processing**: Process multiple files in parallel

## Next Steps

- See `data-model.md` for full API reference
- See `contracts/` for backend implementation details
- Run `/tasks` to generate implementation tasks
