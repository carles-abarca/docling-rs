//! Output file generation (markdown, JSON, text).
//!
//! Re-exports output functions from docling-rs for CLI use.

use anyhow::Result;
use docling_rs::DoclingDocument;

/// Convert document to Markdown format
pub fn to_markdown(doc: &DoclingDocument) -> String {
    docling_rs::output::to_markdown(doc)
}

/// Convert document to JSON format
pub fn to_json(doc: &DoclingDocument) -> Result<String> {
    Ok(docling_rs::output::to_json(doc)?)
}

/// Convert document to plain text format
pub fn to_text(doc: &DoclingDocument) -> String {
    docling_rs::output::to_text(doc)
}
