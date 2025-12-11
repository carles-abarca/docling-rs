//! XLSX backend for docling-rs

use docling_rs_core::{
    Backend, ConversionError, DoclingDocument, DocumentNode, DocumentSource, InputDocument,
    InputFormat, NodeType,
};
use calamine::{open_workbook_auto_from_rs, Reader, Sheets};
use std::io::Cursor;

/// XLSX backend using calamine
pub struct XlsxBackend;

impl XlsxBackend {
    /// Create a new XLSX backend
    pub fn new() -> Self {
        Self
    }

    fn get_bytes(input: &InputDocument) -> Result<Vec<u8>, ConversionError> {
        match input.source() {
            DocumentSource::FilePath(path) => {
                std::fs::read(path).map_err(ConversionError::Io)
            }
            DocumentSource::Bytes { data, .. } => Ok(data.clone()),
        }
    }
}

impl Default for XlsxBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl Backend for XlsxBackend {
    fn convert(&self, input: &InputDocument) -> Result<DoclingDocument, ConversionError> {
        let bytes = Self::get_bytes(input)?;

        let name = match input.source() {
            DocumentSource::FilePath(path) => path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown")
                .to_string(),
            DocumentSource::Bytes { name, .. } => name.clone(),
        };

        // Parse XLSX using calamine
        let cursor = Cursor::new(bytes);
        let mut workbook: Sheets<Cursor<Vec<u8>>> = open_workbook_auto_from_rs(cursor)
            .map_err(|e| ConversionError::ParseError(format!("XLSX parse error: {}", e)))?;

        let mut doc = DoclingDocument::new(name);

        let sheet_names: Vec<String> = workbook.sheet_names().to_vec();

        for sheet_name in sheet_names.iter() {
            if let Ok(range) = workbook.worksheet_range(sheet_name) {
                let sheet_heading =
                    DocumentNode::new(NodeType::Heading, format!("Sheet: {}", sheet_name));
                doc.add_node(sheet_heading);

                let (num_rows, _num_cols) = range.get_size();

                if num_rows > 0 {
                    let mut table_text = String::new();

                    for row in range.rows() {
                        let row_text: Vec<String> = row
                            .iter()
                            .map(|cell| match cell {
                                calamine::Data::Empty => String::new(),
                                calamine::Data::String(s) => s.clone(),
                                calamine::Data::Float(f) => f.to_string(),
                                calamine::Data::Int(i) => i.to_string(),
                                calamine::Data::Bool(b) => b.to_string(),
                                calamine::Data::Error(e) => format!("#ERROR: {:?}", e),
                                calamine::Data::DateTime(dt) => dt.to_string(),
                                calamine::Data::DateTimeIso(s) => s.clone(),
                                calamine::Data::DurationIso(s) => s.clone(),
                            })
                            .collect();

                        table_text.push_str(&row_text.join("\t"));
                        table_text.push('\n');
                    }

                    let table_node = DocumentNode::new(NodeType::Table, table_text);
                    doc.add_node(table_node);
                }
            }
        }

        Ok(doc)
    }

    fn supports_format(&self, format: InputFormat) -> bool {
        format == InputFormat::Xlsx
    }
}
