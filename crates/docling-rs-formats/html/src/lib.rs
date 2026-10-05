//! HTML backend for docling-rs

use docling_rs_core::{
    Backend, ConversionError, DoclingDocument, DocumentNode, DocumentSource, InputDocument,
    InputFormat, NodeType, TableCell, TableData, TableRow,
};
use scraper::{ElementRef, Html, Selector};

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

        let selector = Selector::parse("h1,h2,h3,h4,h5,h6,p,li,table,pre").unwrap();
        let rows = Selector::parse("tr").unwrap();
        for element in html.select(&selector) {
            if element.ancestors().filter_map(ElementRef::wrap).any(|a| {
                matches!(
                    a.value().name(),
                    "table" | "li" | "p" | "pre" | "script" | "style"
                )
            }) {
                continue;
            }
            let tag = element.value().name();
            if tag == "table" {
                let mut table = TableData::new();
                for row in element.select(&rows) {
                    if row
                        .ancestors()
                        .filter_map(ElementRef::wrap)
                        .find(|a| a.value().name() == "table")
                        .map(|a| a.id())
                        != Some(element.id())
                    {
                        continue;
                    }
                    let cells = row
                        .children()
                        .filter_map(ElementRef::wrap)
                        .filter(|c| matches!(c.value().name(), "td" | "th"))
                        .map(|c| TableCell::new(visible_text(c)))
                        .collect();
                    table.add_row(TableRow::new(cells));
                }
                doc.add_node(DocumentNode::new_table(table));
            } else {
                let text = visible_text(element);
                if !text.trim().is_empty() {
                    let kind = match tag {
                        "li" => NodeType::ListItem,
                        "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => NodeType::Heading,
                        _ => NodeType::Paragraph,
                    };
                    doc.add_node(DocumentNode::new(kind, text.trim()));
                }
            }
        }

        Ok(doc)
    }

    fn supports_format(&self, format: InputFormat) -> bool {
        matches!(format, InputFormat::Html)
    }
}

fn visible_text(element: ElementRef<'_>) -> String {
    element
        .descendants()
        .filter(|node| {
            !node
                .ancestors()
                .filter_map(ElementRef::wrap)
                .any(|a| matches!(a.value().name(), "script" | "style"))
        })
        .filter_map(|node| node.value().as_text().map(|t| t.to_string()))
        .collect::<Vec<_>>()
        .join(" ")
}
