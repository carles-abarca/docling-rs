//! Error types for document conversion

use std::path::PathBuf;
use thiserror::Error;

/// Errors during document conversion
#[derive(Debug, Error)]
pub enum ConversionError {
    #[error("File not found: {0}")]
    FileNotFound(PathBuf),

    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),

    #[error("Invalid file: {0}")]
    InvalidFile(String),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Encryption error: {0}")]
    EncryptionError(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}
