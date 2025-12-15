//! XLSX backend for docling-rs

use calamine::{open_workbook_auto_from_rs, Reader, Sheets};
use docling_rs_core::{
    Backend, ConversionError, DoclingDocument, DocumentNode, DocumentSource, InputDocument,
    InputFormat, TableCell, TableData, TableRow,
};
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
            DocumentSource::FilePath(path) => std::fs::read(path).map_err(ConversionError::Io),
            DocumentSource::Bytes { data, .. } => Ok(data.clone()),
        }
    }

    fn cell_to_string(cell: &calamine::Data) -> String {
        match cell {
            calamine::Data::Empty => String::new(),
            calamine::Data::String(s) => s.clone(),
            calamine::Data::Float(f) => {
                // Format integers without decimal point
                if f.fract() == 0.0 && *f >= i64::MIN as f64 && *f <= i64::MAX as f64 {
                    (*f as i64).to_string()
                } else {
                    f.to_string()
                }
            }
            calamine::Data::Int(i) => i.to_string(),
            calamine::Data::Bool(b) => b.to_string(),
            calamine::Data::Error(e) => format!("#ERROR: {:?}", e),
            calamine::Data::DateTime(dt) => dt.to_string(),
            calamine::Data::DateTimeIso(s) => s.clone(),
            calamine::Data::DurationIso(s) => s.clone(),
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
                let (num_rows, _num_cols) = range.get_size();

                if num_rows > 0 {
                    let mut table_data = TableData::new();

                    for row in range.rows() {
                        let cells: Vec<TableCell> = row
                            .iter()
                            .map(|cell| TableCell::new(Self::cell_to_string(cell)))
                            .collect();
                        table_data.add_row(TableRow::new(cells));
                    }

                    // Add the table as a single structured node
                    doc.add_node(DocumentNode::new_table(table_data));
                }
            }
        }

        Ok(doc)
    }

    fn supports_format(&self, format: InputFormat) -> bool {
        format == InputFormat::Xlsx
    }
}
