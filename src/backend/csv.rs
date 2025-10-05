//! CSV backend implementation

use crate::backend::Backend;
use crate::datamodel::{DoclingDocument, InputDocument};
use crate::error::ConversionError;
use crate::InputFormat;
use csv::ReaderBuilder;

/// CSV backend
pub struct CsvBackend {}

impl CsvBackend {
    /// Create a new CSV backend
    pub fn new() -> Self {
        Self {}
    }

    fn get_bytes(input: &InputDocument) -> Result<Vec<u8>, ConversionError> {
        match input.source() {
            crate::datamodel::DocumentSource::FilePath(path) => {
                std::fs::read(path).map_err(ConversionError::Io)
            }
            crate::datamodel::DocumentSource::Bytes { data, .. } => Ok(data.clone()),
        }
    }
}

impl Default for CsvBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl Backend for CsvBackend {
    fn convert(&self, input: &InputDocument) -> Result<DoclingDocument, ConversionError> {
        let bytes = Self::get_bytes(input)?;

        // Get document name from input
        let name = match input.source() {
            crate::datamodel::DocumentSource::FilePath(path) => path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown")
                .to_string(),
            crate::datamodel::DocumentSource::Bytes { name, .. } => name.clone(),
        };

        // Parse CSV
        let mut reader = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(bytes.as_slice());

        // Verify parsing works by reading headers
        let _headers = reader
            .headers()
            .map_err(|e| ConversionError::ParseError(format!("CSV parse error: {}", e)))?;

        // Create document
        let doc = DoclingDocument::new(name);

        // Full CSV table parsing will be added in REFACTOR phase
        Ok(doc)
    }

    fn supports_format(&self, format: InputFormat) -> bool {
        format == InputFormat::Csv
    }
}
