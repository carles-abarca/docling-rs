//! Input format detection and enumeration

use serde::{Deserialize, Serialize};

/// Supported input document formats
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InputFormat {
    Markdown,
    Html,
    Csv,
    Docx,
    Xlsx,
    Pptx,
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
            InputFormat::Xlsx => "xlsx",
            InputFormat::Pptx => "pptx",
            InputFormat::PDF => "pdf",
        }
    }

    /// Detect format from file extension
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "md" | "markdown" => Some(InputFormat::Markdown),
            "html" | "htm" => Some(InputFormat::Html),
            "csv" => Some(InputFormat::Csv),
            "docx" | "dotx" | "docm" | "dotm" => Some(InputFormat::Docx),
            "xlsx" | "xlsm" | "xlsb" | "xls" => Some(InputFormat::Xlsx),
            "pptx" | "potx" | "ppsx" | "pptm" | "potm" | "ppsm" => Some(InputFormat::Pptx),
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
                "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet" => {
                    Some(InputFormat::Xlsx)
                }
                "application/vnd.openxmlformats-officedocument.presentationml.presentation" => {
                    Some(InputFormat::Pptx)
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
