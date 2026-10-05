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

        if html
            .tree
            .nodes()
            .any(|n| n.ancestors().take(258).count() > 256)
        {
            return Err(ConversionError::ParseError(
                "HTML nesting exceeds 256 levels".into(),
            ));
        }
        let body_selector = Selector::parse("body").unwrap();
        if let Some(body) = html.select(&body_selector).next() {
            extract_blocks(body, &mut doc);
        }

        Ok(doc)
    }

    fn supports_format(&self, format: InputFormat) -> bool {
        matches!(format, InputFormat::Html)
    }
}

fn visible_text(element: ElementRef<'_>) -> String {
    let mut out = String::new();
    // Traverse edges to insert breaks around block children without splitting inline words.
    for edge in element.traverse() {
        match edge {
            ego_tree::iter::Edge::Open(node) => {
                if node
                    .ancestors()
                    .filter_map(ElementRef::wrap)
                    .any(|a| matches!(a.value().name(), "script" | "style" | "template"))
                {
                    continue;
                }
                if let Some(t) = node.value().as_text() {
                    out.push_str(t)
                }
                if let Some(e) = ElementRef::wrap(node) {
                    if matches!(e.value().name(), "br" | "p" | "li" | "div" | "tr") {
                        out.push('\n')
                    }
                }
            }
            ego_tree::iter::Edge::Close(node) => {
                if let Some(e) = ElementRef::wrap(node) {
                    if matches!(e.value().name(), "p" | "li" | "div" | "tr") {
                        out.push('\n')
                    }
                }
            }
        }
    }
    out
}
fn flush(text: &mut String, doc: &mut DoclingDocument) {
    if !text.trim().is_empty() {
        doc.add_node(DocumentNode::new(NodeType::Paragraph, text.trim()));
    }
    text.clear();
}
fn extract_blocks(root: ElementRef<'_>, doc: &mut DoclingDocument) {
    let mut text = String::new();
    for child in root.children() {
        if let Some(t) = child.value().as_text() {
            text.push_str(t);
            continue;
        }
        let Some(e) = ElementRef::wrap(child) else {
            continue;
        };
        match e.value().name() {
            "script" | "style" | "template" => {}
            "table" => {
                flush(&mut text, doc);
                let mut table = TableData::new();
                let rows = Selector::parse("tr").unwrap();
                for row in e.select(&rows) {
                    if row
                        .ancestors()
                        .filter_map(ElementRef::wrap)
                        .find(|a| a.value().name() == "table")
                        .map(|a| a.id())
                        != Some(e.id())
                    {
                        continue;
                    }
                    table.add_row(TableRow::new(
                        row.children()
                            .filter_map(ElementRef::wrap)
                            .filter(|c| matches!(c.value().name(), "td" | "th"))
                            .map(|c| TableCell::new(visible_text(c)))
                            .collect(),
                    ));
                }
                doc.add_node(DocumentNode::new_table(table));
            }
            "h1" | "h2" | "h3" | "h4" | "h5" | "h6" | "p" | "li" | "pre" => {
                flush(&mut text, doc);
                let value = visible_text(e);
                let kind = match e.value().name() {
                    "li" => NodeType::ListItem,
                    "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => NodeType::Heading,
                    _ => NodeType::Paragraph,
                };
                if !value.trim().is_empty() {
                    doc.add_node(DocumentNode::new(kind, value));
                }
            }
            "br" => text.push('\n'),
            "div" | "section" | "article" | "main" | "header" | "footer" | "nav" | "aside"
            | "ul" | "ol" | "body" => {
                flush(&mut text, doc);
                extract_blocks(e, doc)
            }
            _ => text.push_str(&visible_text(e)),
        }
    }
    flush(&mut text, doc);
}
