//! DOCX text and tables via bounded Office XML parsing.
use docling_rs_core::{
    office, Backend, ConversionError, DoclingDocument, DocumentNode, InputDocument, InputFormat,
    NodeType, TableCell, TableData, TableRow,
};
use roxmltree::Node;
const W: &str = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";
pub struct DocxBackend;
impl DocxBackend {
    pub fn new() -> Self {
        Self
    }
}
impl Default for DocxBackend {
    fn default() -> Self {
        Self::new()
    }
}
fn is(n: Node<'_, '_>, name: &str) -> bool {
    n.has_tag_name((W, name))
}
fn text(n: Node<'_, '_>) -> String {
    let mut out = String::new();
    for c in n.descendants() {
        if c.ancestors().any(|a| is(a, "del")) {
            continue;
        }
        if is(c, "t") {
            out.push_str(c.text().unwrap_or(""))
        } else if is(c, "tab") {
            out.push('\t')
        } else if is(c, "br") || is(c, "cr") {
            out.push('\n')
        }
    }
    out
}
fn table(n: Node<'_, '_>) -> TableData {
    let mut data = TableData::new();
    for row in n.children().filter(|c| is(*c, "tr")) {
        data.add_row(TableRow::new(
            row.children()
                .filter(|c| is(*c, "tc"))
                .map(|cell| {
                    let values: Vec<String> = cell
                        .children()
                        .filter_map(|c| {
                            if is(c, "p") {
                                Some(text(c))
                            } else if is(c, "tbl") {
                                Some(table(c).to_markdown())
                            } else {
                                None
                            }
                        })
                        .collect();
                    TableCell::new(values.join("\n"))
                })
                .collect(),
        ));
    }
    data
}
fn blocks(root: Node<'_, '_>, doc: &mut DoclingDocument) {
    for n in root.children() {
        if is(n, "p") {
            let value = text(n);
            if value.trim().is_empty() {
                continue;
            }
            let heading = n
                .descendants()
                .filter(|c| is(*c, "pStyle"))
                .filter_map(|c| c.attribute((W, "val")))
                .any(|v| {
                    v.to_lowercase().starts_with("heading") || v == "Title" || v == "Subtitle"
                });
            doc.add_node(DocumentNode::new(
                if heading {
                    NodeType::Heading
                } else {
                    NodeType::Paragraph
                },
                value,
            ));
        } else if is(n, "tbl") {
            doc.add_node(DocumentNode::new_table(table(n)))
        } else if n.is_element() {
            blocks(n, doc)
        }
    }
}
impl Backend for DocxBackend {
    fn convert(&self, input: &InputDocument) -> Result<DoclingDocument, ConversionError> {
        let parts = office::read_parts(input)?;
        let xml = office::parse(office::part(&parts, "word/document.xml")?)?;
        let mut doc = DoclingDocument::new(office::name(input));
        let rels = office::relationships(&parts, "word/document.xml")?;
        // Emit each referenced header/footer once, not once per rendered page.
        for kind in ["header", "body", "footer"] {
            if kind == "body" {
                let body = xml
                    .descendants()
                    .find(|n| is(*n, "body"))
                    .ok_or_else(|| office::error("Missing DOCX body"))?;
                blocks(body, &mut doc);
                continue;
            }
            let mut seen = std::collections::BTreeSet::new();
            for n in xml
                .descendants()
                .filter(|n| is(*n, &format!("{kind}Reference")))
            {
                if let Some(id) = n.attributes().find(|a| a.name() == "id").map(|a| a.value()) {
                    let (path, _) = rels
                        .get(id)
                        .ok_or_else(|| office::error("Missing DOCX header/footer relationship"))?;
                    if seen.insert(path) {
                        let part = office::parse(office::part(&parts, path)?)?;
                        blocks(part.root_element(), &mut doc)
                    }
                }
            }
        }
        if xml
            .descendants()
            .any(|n| is(n, "drawing") || is(n, "pict") || is(n, "altChunk"))
        {
            doc=doc.with_metadata("conversion_warnings",serde_json::json!(["Embedded drawings/images or alternate content are not fully extracted; OCR was not performed."]));
        }
        Ok(doc)
    }
    fn supports_format(&self, f: InputFormat) -> bool {
        f == InputFormat::Docx
    }
}
