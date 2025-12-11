//! DOCX backend for docling-rs

use docling_rs_core::{
    Backend, ConversionError, DoclingDocument, DocumentNode, DocumentSource, InputDocument,
    InputFormat, NodeType,
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
                if let RunChild::Text(t) = run_child {
                    text.push_str(&t.text);
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
