//! HTML backend for docling-rs

use docling_rs_core::{
    Backend, ConversionError, DoclingDocument, DocumentNode, DocumentSource, InputDocument,
    InputFormat, NodeType,
};
use scraper::{Html, Selector};

/// HTML backend
pub struct HtmlBackend;

impl HtmlBackend {
    /// Create a new HtmlBackend
    pub fn new() -> Self {
        Self
    }
}

impl Default for HtmlBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl Backend for HtmlBackend {
    fn convert(&self, input: &InputDocument) -> Result<DoclingDocument, ConversionError> {
        let content = match input.source() {
            DocumentSource::FilePath(path) => std::fs::read_to_string(path)?,
            DocumentSource::Bytes { data, .. } => String::from_utf8(data.clone())
                .map_err(|e| ConversionError::ParseError(e.to_string()))?,
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
        let html = Html::parse_document(&content);

        // Extract headings h1-h6
        for i in 1..=6 {
            let selector = Selector::parse(&format!("h{}", i)).unwrap();
            for element in html.select(&selector) {
                let text: String = element.text().collect::<Vec<_>>().join(" ");
                if !text.trim().is_empty() {
                    doc.add_node(DocumentNode::new(NodeType::Heading, text.trim()));
                }
            }
        }

        // Extract paragraphs
        let p_selector = Selector::parse("p").unwrap();
        for element in html.select(&p_selector) {
            let text: String = element.text().collect::<Vec<_>>().join(" ");
            if !text.trim().is_empty() {
                doc.add_node(DocumentNode::new(NodeType::Paragraph, text.trim()));
            }
        }

        // Extract list items
        let li_selector = Selector::parse("li").unwrap();
        for element in html.select(&li_selector) {
            let text: String = element.text().collect::<Vec<_>>().join(" ");
            if !text.trim().is_empty() {
                doc.add_node(DocumentNode::new(NodeType::ListItem, text.trim()));
            }
        }

        Ok(doc)
    }

    fn supports_format(&self, format: InputFormat) -> bool {
        matches!(format, InputFormat::Html)
    }
}
