//! Input document types

use crate::InputFormat;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Input document representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputDocument {
    source: DocumentSource,
    format: InputFormat,
}

impl InputDocument {
    /// Create from file path
    pub fn from_path(path: PathBuf, format: InputFormat) -> Self {
        Self {
            source: DocumentSource::FilePath(path),
            format,
        }
    }

    /// Create from bytes
    pub fn from_bytes(data: Vec<u8>, name: impl Into<String>, format: InputFormat) -> Self {
        Self {
            source: DocumentSource::Bytes {
                data,
                name: name.into(),
            },
            format,
        }
    }

    /// Get the document source
    pub fn source(&self) -> &DocumentSource {
        &self.source
    }

    /// Get the input format
    pub fn format(&self) -> InputFormat {
        self.format
    }
}

/// Document source (file path or bytes)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DocumentSource {
    FilePath(PathBuf),
    Bytes { data: Vec<u8>, name: String },
}
