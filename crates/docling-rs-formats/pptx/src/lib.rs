//! PPTX backend for docling-rs

use docling_rs_core::{
    Backend, ConversionError, DoclingDocument, DocumentNode, DocumentSource, InputDocument,
    InputFormat, NodeType,
};
use pptx_to_md::{
    ListElement, ParserConfig, PptxContainer, SlideElement, TableElement, TextElement,
};
use std::io::Write;
use std::path::PathBuf;

/// PPTX backend using pptx-to-md
pub struct PptxBackend;

impl PptxBackend {
    /// Create a new PPTX backend
    pub fn new() -> Self {
        Self
    }

    fn get_path(input: &InputDocument) -> Result<(PathBuf, bool), ConversionError> {
        match input.source() {
            DocumentSource::FilePath(path) => Ok((path.clone(), false)),
            DocumentSource::Bytes { data, .. } => {
                let mut temp_file = tempfile::NamedTempFile::new().map_err(ConversionError::Io)?;
                temp_file.write_all(data).map_err(ConversionError::Io)?;
                let path = temp_file.into_temp_path().keep().map_err(|e| {
                    ConversionError::ParseError(format!("Failed to keep temp file: {}", e))
                })?;
                Ok((path, true))
            }
        }
    }

    fn extract_text(text_element: &TextElement) -> String {
        text_element
            .runs
            .iter()
            .map(|run| run.text.clone())
            .collect::<Vec<_>>()
            .join("")
    }

    fn extract_table_text(table: &TableElement) -> String {
        let mut rows_text: Vec<String> = Vec::new();

        for row in &table.rows {
            let mut cells_text: Vec<String> = Vec::new();
            for cell in &row.cells {
                let cell_text: String = cell
                    .runs
                    .iter()
                    .map(|run| run.text.clone())
                    .collect::<Vec<_>>()
                    .join("");
                cells_text.push(cell_text);
            }
            rows_text.push(cells_text.join("\t"));
        }

        rows_text.join("\n")
    }

    fn extract_list_text(list: &ListElement) -> String {
        list.items
            .iter()
            .map(|item| {
                let text: String = item
                    .runs
                    .iter()
                    .map(|run| run.text.clone())
                    .collect::<Vec<_>>()
                    .join("");
                let prefix = if item.is_ordered { "1." } else { "•" };
                format!("{} {}", prefix, text)
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl Default for PptxBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl Backend for PptxBackend {
    fn convert(&self, input: &InputDocument) -> Result<DoclingDocument, ConversionError> {
        let name = match input.source() {
            DocumentSource::FilePath(path) => path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown")
                .to_string(),
            DocumentSource::Bytes { name, .. } => name.clone(),
        };

        let (path, is_temp) = Self::get_path(input)?;

        let config = ParserConfig::default();
        let mut container = PptxContainer::open(&path, config)
            .map_err(|e| ConversionError::ParseError(format!("PPTX open error: {}", e)))?;

        let mut doc = DoclingDocument::new(name);

        let slides = container
            .parse_all()
            .map_err(|e| ConversionError::ParseError(format!("PPTX parse error: {}", e)))?;

        for slide in slides {
            let slide_heading =
                DocumentNode::new(NodeType::Heading, format!("Slide {}", slide.slide_number));
            doc.add_node(slide_heading);

            let mut first_text = true;

            for element in &slide.elements {
                match element {
                    SlideElement::Text(text_elem, _pos) => {
                        let text = Self::extract_text(text_elem);
                        if text.trim().is_empty() {
                            continue;
                        }

                        let node_type = if first_text {
                            first_text = false;
                            NodeType::Heading
                        } else {
                            NodeType::Paragraph
                        };

                        let node = DocumentNode::new(node_type, text);
                        doc.add_node(node);
                    }
                    SlideElement::Table(table_elem, _pos) => {
                        let text = Self::extract_table_text(table_elem);
                        if !text.trim().is_empty() {
                            let node = DocumentNode::new(NodeType::Table, text);
                            doc.add_node(node);
                        }
                    }
                    SlideElement::List(list_elem, _pos) => {
                        let text = Self::extract_list_text(list_elem);
                        if !text.trim().is_empty() {
                            let node = DocumentNode::new(NodeType::List, text);
                            doc.add_node(node);
                        }
                    }
                    SlideElement::Image(_img_ref, _pos) => {
                        // Skip images for now
                    }
                    SlideElement::Unknown => {
                        // Skip unknown elements
                    }
                }
            }
        }

        if is_temp {
            let _ = std::fs::remove_file(&path);
        }

        Ok(doc)
    }

    fn supports_format(&self, format: InputFormat) -> bool {
        format == InputFormat::Pptx
    }
}
