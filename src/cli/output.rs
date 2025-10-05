//! Output file generation (markdown, JSON, text).

use crate::datamodel::{DoclingDocument, NodeType};
use anyhow::Result;

/// Convert document to Markdown format
pub fn to_markdown(doc: &DoclingDocument) -> String {
    let mut output = String::new();

    // Title
    output.push_str(&format!("# {}\n\n", doc.name()));

    // Content (iterate through document nodes)
    for node in doc.nodes() {
        let text = node.text_content().unwrap_or("");
        match node.node_type() {
            NodeType::Heading => {
                output.push_str(&format!("## {}\n\n", text));
            }
            NodeType::Paragraph | NodeType::Text => {
                output.push_str(&format!("{}\n\n", text));
            }
            NodeType::Table => {
                output.push_str("(Table content)\n\n");
            }
            _ => {
                output.push_str(&format!("{}\n\n", text));
            }
        }
    }

    output
}

/// Convert document to JSON format
pub fn to_json(doc: &DoclingDocument) -> Result<String> {
    Ok(serde_json::to_string_pretty(doc)?)
}

/// Convert document to plain text format
pub fn to_text(doc: &DoclingDocument) -> String {
    let mut output = String::new();

    // Title
    output.push_str(&format!("{}\n\n", doc.name()));

    // Extract all text from nodes
    for node in doc.nodes() {
        if let Some(text) = node.text_content() {
            if !text.is_empty() {
                output.push_str(&format!("{}\n\n", text));
            }
        }
    }

    output
}
