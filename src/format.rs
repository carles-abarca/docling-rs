//! Input format detection and enumeration

use serde::{Deserialize, Serialize};

/// Supported input document formats
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InputFormat {
    Markdown,
    Html,
    Csv,
    Docx,
    PDF,
}

impl InputFormat {
    /// Get file extension for this format
    pub fn extension(&self) -> &'static str {
        match self {
            InputFormat::Markdown => "md",
            InputFormat::Html => "html",
            InputFormat::Csv => "csv",
            InputFormat::Docx => "docx",
            InputFormat::PDF => "pdf",
        }
    }

    /// Detect format from file extension
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "md" | "markdown" => Some(InputFormat::Markdown),
            "html" | "htm" => Some(InputFormat::Html),
            "csv" => Some(InputFormat::Csv),
            "docx" => Some(InputFormat::Docx),
            "pdf" => Some(InputFormat::PDF),
            _ => None,
        }
    }

    /// Detect format from magic bytes
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        // Use infer crate for magic number detection
        if let Some(kind) = infer::get(bytes) {
            match kind.mime_type() {
                "application/vnd.openxmlformats-officedocument.wordprocessingml.document" => {
                    Some(InputFormat::Docx)
                }
                "application/pdf" => Some(InputFormat::PDF),
                "text/html" => Some(InputFormat::Html),
                "text/csv" => Some(InputFormat::Csv),
                _ => None,
            }
        } else {
            None
        }
    }
}
