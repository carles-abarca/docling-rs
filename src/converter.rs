//! Document converter - main entry point

use crate::datamodel::{ConversionResult, InputDocument};
use crate::error::ConversionError;
use crate::format::InputFormat;
use crate::pipeline::{Pipeline, SimplePipeline};
use std::path::Path;

/// Main entry point for document conversion
pub struct DocumentConverter {
    pipeline: SimplePipeline,
}

impl DocumentConverter {
    /// Create a new DocumentConverter
    pub fn new() -> Self {
        Self {
            pipeline: SimplePipeline::new(),
        }
    }

    /// Convert a document from a file path
    pub fn convert_file<P: AsRef<Path>>(
        &self,
        path: P,
    ) -> Result<ConversionResult, ConversionError> {
        let path = path.as_ref();

        // Check if file exists
        if !path.exists() {
            return Err(ConversionError::FileNotFound(path.to_path_buf()));
        }

        // Detect format from extension
        let format = path
            .extension()
            .and_then(|ext| ext.to_str())
            .and_then(InputFormat::from_extension)
            .ok_or_else(|| {
                ConversionError::UnsupportedFormat(
                    path.extension()
                        .and_then(|e| e.to_str())
                        .unwrap_or("unknown")
                        .to_string(),
                )
            })?;

        // Create InputDocument
        let input = InputDocument::from_path(path.to_path_buf(), format);

        // Execute pipeline
        self.pipeline.execute(&input)
    }

    /// Convert a document from bytes
    pub fn convert_bytes(
        &self,
        bytes: Vec<u8>,
        name: String,
        format: crate::InputFormat,
    ) -> Result<ConversionResult, ConversionError> {
        // Create InputDocument
        let input = InputDocument::from_bytes(bytes, name, format);

        // Execute pipeline
        self.pipeline.execute(&input)
    }
}

impl Default for DocumentConverter {
    fn default() -> Self {
        Self::new()
    }
}
