//! CSV backend for docling-rs

use docling_rs_core::{
    Backend, ConversionError, DoclingDocument, DocumentNode, DocumentSource, InputDocument,
    InputFormat, NodeType,
};
use csv::ReaderBuilder;
use std::io::Cursor;

/// CSV backend
pub struct CsvBackend;

impl CsvBackend {
    /// Create a new CsvBackend
    pub fn new() -> Self {
        Self
    }
}

impl Default for CsvBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl Backend for CsvBackend {
    fn convert(&self, input: &InputDocument) -> Result<DoclingDocument, ConversionError> {
        let content = match input.source() {
            DocumentSource::FilePath(path) => std::fs::read_to_string(path)?,
            DocumentSource::Bytes { data, .. } => {
                String::from_utf8(data.clone()).map_err(|e| ConversionError::ParseError(e.to_string()))?
            }
        };

        let name = match input.source() {
            DocumentSource::FilePath(path) => path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("document")
                .to_string(),
            DocumentSource::Bytes { name, .. } => name.clone(),
        };

        let mut doc = DoclingDocument::new(&name);
        let cursor = Cursor::new(content);
        let mut reader = ReaderBuilder::new().has_headers(true).from_reader(cursor);

        // Get headers
        if let Ok(headers) = reader.headers() {
            let header_text = headers.iter().collect::<Vec<_>>().join(" | ");
            doc.add_node(DocumentNode::new(NodeType::TableRow, header_text));
        }

        // Get rows
        for record in reader.records().flatten() {
            let row_text = record.iter().collect::<Vec<_>>().join(" | ");
            doc.add_node(DocumentNode::new(NodeType::TableRow, row_text));
        }

        Ok(doc)
    }

    fn supports_format(&self, format: InputFormat) -> bool {
        matches!(format, InputFormat::Csv)
    }
}
