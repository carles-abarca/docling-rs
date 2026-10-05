//! XLSX backend for docling-rs

use calamine::{open_workbook_auto_from_rs, Reader, Sheets};
use docling_rs_core::{
    Backend, ConversionError, DoclingDocument, DocumentNode, DocumentSource, InputDocument,
    InputFormat, NodeType, TableCell, TableData, TableRow,
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

        for sheet_name in &sheet_names {
            let range = workbook
                .worksheet_range(sheet_name)
                .map_err(|e| ConversionError::ParseError(format!("Sheet {sheet_name}: {e}")))?;
            let formulas = workbook.worksheet_formula(sheet_name).map_err(|e| {
                ConversionError::ParseError(format!("Sheet formulas {sheet_name}: {e}"))
            })?;
            doc.add_node(DocumentNode::new(NodeType::Heading, sheet_name));
            let starts: Vec<_> = [range.start(), formulas.start()]
                .into_iter()
                .flatten()
                .collect();
            let ends: Vec<_> = [range.end(), formulas.end()]
                .into_iter()
                .flatten()
                .collect();
            if starts.is_empty() {
                continue;
            }
            let start = (
                starts.iter().map(|p| p.0).min().unwrap(),
                starts.iter().map(|p| p.1).min().unwrap(),
            );
            let end = (
                ends.iter().map(|p| p.0).max().unwrap(),
                ends.iter().map(|p| p.1).max().unwrap(),
            );
            let mut table = TableData::new();
            for row in start.0..=end.0 {
                let cells = (start.1..=end.1)
                    .map(|col| {
                        let mut value = range
                            .get_value((row, col))
                            .map(Self::cell_to_string)
                            .unwrap_or_default();
                        if let Some(formula) =
                            formulas.get_value((row, col)).filter(|f| !f.is_empty())
                        {
                            let formula = format!("={}", formula.trim_start_matches('='));
                            value = if value.is_empty() {
                                formula
                            } else {
                                format!("{value} [formula: {formula}]")
                            };
                        }
                        TableCell::new(value)
                    })
                    .collect();
                table.add_row(TableRow::new(cells));
            }
            doc.add_node(DocumentNode::new_table(table));
        }

        Ok(doc)
    }

    fn supports_format(&self, format: InputFormat) -> bool {
        format == InputFormat::Xlsx
    }
}
