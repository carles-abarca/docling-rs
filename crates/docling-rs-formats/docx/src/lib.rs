//! DOCX backend for docling-rs

use docling_rs_core::{
    Backend, ConversionError, DoclingDocument, DocumentNode, DocumentSource, InputDocument,
    InputFormat, NodeType, TableCell as CoreCell, TableData, TableRow as CoreRow,
};
use docx_rs::*;
use std::io::Read;

/// DOCX backend
pub struct DocxBackend;

impl DocxBackend {
    /// Create a new DocxBackend
    pub fn new() -> Self {
        Self
    }
}

impl Default for DocxBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl Backend for DocxBackend {
    fn convert(&self, input: &InputDocument) -> Result<DoclingDocument, ConversionError> {
        let bytes = match input.source() {
            DocumentSource::FilePath(path) => {
                let mut file = std::fs::File::open(path)?;
                let mut buffer = Vec::new();
                file.read_to_end(&mut buffer)?;
                buffer
            }
            DocumentSource::Bytes { data, .. } => data.clone(),
        };

        let name = match input.source() {
            DocumentSource::FilePath(path) => path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("document")
                .to_string(),
            DocumentSource::Bytes { name, .. } => name.clone(),
        };

        let mut doc = DoclingDocument::new(&name);

        let docx = read_docx(&bytes)
            .map_err(|e| ConversionError::ParseError(format!("Failed to parse DOCX: {:?}", e)))?;

        for child in docx.document.children {
            if let DocumentChild::Table(table) = &child {
                doc.add_node(DocumentNode::new_table(extract_table(table)));
            }
            if let DocumentChild::Paragraph(para) = child {
                let text = extract_paragraph_text(&para);
                if !text.trim().is_empty() {
                    let node_type = if is_heading_style(&para) {
                        NodeType::Heading
                    } else {
                        NodeType::Paragraph
                    };
                    doc.add_node(DocumentNode::new(node_type, text.trim()));
                }
            }
        }

        Ok(doc)
    }

    fn supports_format(&self, format: InputFormat) -> bool {
        matches!(format, InputFormat::Docx)
    }
}

fn extract_paragraph_text(para: &Paragraph) -> String {
    let mut text = String::new();
    for child in &para.children {
        if let ParagraphChild::Run(run) = child {
            for run_child in &run.children {
                match run_child {
                    RunChild::Text(t) => text.push_str(&t.text),
                    RunChild::Tab(_) | RunChild::PTab(_) => text.push('\t'),
                    RunChild::Break(_) | RunChild::CarriageReturn(_) => text.push('\n'),
                    _ => {}
                }
            }
        }
    }
    text
}

fn is_heading_style(para: &Paragraph) -> bool {
    if let Some(style) = &para.property.style {
        let style_id = &style.val;
        style_id.starts_with("Heading") || style_id == "Title" || style_id == "Subtitle"
    } else {
        false
    }
}

fn extract_table(table: &docx_rs::Table) -> TableData {
    let mut data = TableData::new();
    for TableChild::TableRow(row) in &table.rows {
        let cells = row
            .cells
            .iter()
            .map(|TableRowChild::TableCell(cell)| {
                let parts: Vec<String> = cell
                    .children
                    .iter()
                    .filter_map(|child| match child {
                        TableCellContent::Paragraph(p) => Some(extract_paragraph_text(p)),
                        TableCellContent::Table(t) => Some(extract_table(t).to_markdown()),
                        _ => None,
                    })
                    .collect();
                CoreCell::new(parts.join("\n"))
            })
            .collect();
        data.add_row(CoreRow::new(cells));
    }
    data
}
