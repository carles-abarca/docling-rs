//! PDF backend implementation.

use super::config::PdfConfig;
use super::document::PdfDocument;
use crate::backend::Backend;
use crate::datamodel::{DoclingDocument, InputDocument};
use crate::error::ConversionError;
use crate::InputFormat;

/// PDF backend for document conversion.
pub struct PdfBackend {
    config: PdfConfig,
}

impl PdfBackend {
    /// Create a new PDF backend with default configuration.
    pub fn new() -> Self {
        Self {
            config: PdfConfig::default(),
        }
    }

    /// Create a new PDF backend with custom configuration.
    pub fn with_config(config: PdfConfig) -> Self {
        Self { config }
    }
}

impl Default for PdfBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl Backend for PdfBackend {
    fn convert(&self, input: &InputDocument) -> Result<DoclingDocument, ConversionError> {
        // TODO: Implement PDF conversion
        // This is a placeholder that will be implemented in subsequent tasks
        Err(ConversionError::UnsupportedFormat(
            "PDF backend not yet implemented".to_string(),
        ))
    }

    fn supports_format(&self, format: InputFormat) -> bool {
        matches!(format, InputFormat::PDF)
    }
}
