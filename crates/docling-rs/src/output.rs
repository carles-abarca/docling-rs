//! Output file generation (markdown, JSON, text).

use crate::{DoclingDocument, NodeType};

/// Convert document to Markdown format
/// This produces output similar to Python docling's export_to_markdown()
pub fn to_markdown(doc: &DoclingDocument) -> String {
    let mut output = String::new();

    // Content (iterate through document nodes)
    for node in doc.nodes() {
        match node.node_type() {
            NodeType::Heading => {
                let text = node.text_content().unwrap_or("");
                // Preserve original heading markers if present, otherwise add ##
                if text.starts_with('#') {
                    output.push_str(&format!("{}\n\n", text));
                } else {
                    output.push_str(&format!("## {}\n\n", text));
                }
            }
            NodeType::Table => {
                // Use structured table data if available
                if let Some(table_data) = node.table_data() {
                    output.push_str(&table_data.to_markdown());
                } else if let Some(text) = node.text_content() {
                    // Fallback to text content
                    if !text.is_empty() {
                        output.push_str(text);
                        if !text.ends_with('\n') {
                            output.push('\n');
                        }
                    }
                }
            }
            NodeType::Paragraph | NodeType::Text | NodeType::ListItem => {
                let text = node.text_content().unwrap_or("");
                if !text.is_empty() {
                    output.push_str(&format!("{}\n\n", text));
                }
            }
            NodeType::TableRow => {
                // TableRow nodes are handled as part of Table
                // If encountered standalone, output as text
                let text = node.text_content().unwrap_or("");
                if !text.is_empty() {
                    output.push_str(&format!("{}\n\n", text));
                }
            }
            _ => {
                let text = node.text_content().unwrap_or("");
                if !text.is_empty() {
                    output.push_str(&format!("{}\n\n", text));
                }
            }
        }
    }

    output
}

/// Convert document to JSON format
pub fn to_json(doc: &DoclingDocument) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(doc)
}

/// Convert document to plain text format
/// This now produces Markdown-formatted output similar to Python docling
pub fn to_text(doc: &DoclingDocument) -> String {
    // Use the same markdown output as Python docling does
    to_markdown(doc)
}
