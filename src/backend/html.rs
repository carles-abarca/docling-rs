//! HTML backend implementation

use crate::backend::Backend;
use crate::datamodel::{DoclingDocument, InputDocument};
use crate::error::ConversionError;
use crate::InputFormat;
use scraper::{Html, Selector};

/// HTML backend
pub struct HtmlBackend {}

impl HtmlBackend {
    /// Create a new HTML backend
    pub fn new() -> Self {
        Self {}
    }

    fn get_content(input: &InputDocument) -> Result<String, ConversionError> {
        match input.source() {
            crate::datamodel::DocumentSource::FilePath(path) => {
                std::fs::read_to_string(path).map_err(ConversionError::Io)
            }
            crate::datamodel::DocumentSource::Bytes { data, .. } => String::from_utf8(data.clone())
                .map_err(|e| ConversionError::InvalidFile(format!("Invalid UTF-8: {}", e))),
        }
    }
}

impl Default for HtmlBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl Backend for HtmlBackend {
    fn convert(&self, input: &InputDocument) -> Result<DoclingDocument, ConversionError> {
        let content = Self::get_content(input)?;

        // Get document name from input
        let name = match input.source() {
            crate::datamodel::DocumentSource::FilePath(path) => path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown")
                .to_string(),
            crate::datamodel::DocumentSource::Bytes { name, .. } => name.clone(),
        };

        // Parse HTML
        let _document = Html::parse_document(&content);

        // Verify parsing works by selecting body
        let _body_selector = Selector::parse("body").unwrap();

        // Create document
        let doc = DoclingDocument::new(name);

        // Full parsing logic will be added in REFACTOR phase
        Ok(doc)
    }

    fn supports_format(&self, format: InputFormat) -> bool {
        format == InputFormat::Html
    }
}
