//! PPTX extraction follows the presentation relationship order, not ZIP filenames.
use docling_rs_core::{
    office, Backend, ConversionError, DoclingDocument, DocumentNode, InputDocument, InputFormat,
    NodeType, TableCell, TableData, TableRow,
};
use roxmltree::Node;
const A: &str = "http://schemas.openxmlformats.org/drawingml/2006/main";
const P: &str = "http://schemas.openxmlformats.org/presentationml/2006/main";
pub struct PptxBackend;
impl PptxBackend {
    pub fn new() -> Self {
        Self
    }
}
impl Default for PptxBackend {
    fn default() -> Self {
        Self::new()
    }
}
fn text(n: Node<'_, '_>) -> String {
    let mut out = String::new();
    for c in n.descendants() {
        if c.has_tag_name((A, "t")) {
            out.push_str(c.text().unwrap_or(""))
        } else if c.has_tag_name((A, "br")) {
            out.push('\n')
        }
    }
    out
}
fn table(n: Node<'_, '_>) -> TableData {
    let mut t = TableData::new();
    for row in n.children().filter(|c| c.has_tag_name((A, "tr"))) {
        t.add_row(TableRow::new(
            row.children()
                .filter(|c| c.has_tag_name((A, "tc")))
                .map(|c| {
                    TableCell::new(
                        c.descendants()
                            .filter(|p| p.has_tag_name((A, "p")))
                            .map(text)
                            .collect::<Vec<_>>()
                            .join("\n"),
                    )
                })
                .collect(),
        ))
    }
    t
}
fn blocks(root: Node<'_, '_>, doc: &mut DoclingDocument) {
    for n in root.descendants() {
        if n.has_tag_name((A, "tbl")) {
            doc.add_node(DocumentNode::new_table(table(n)))
        } else if n.has_tag_name((A, "p")) && !n.ancestors().any(|a| a.has_tag_name((A, "tbl"))) {
            let s = text(n);
            if !s.trim().is_empty() {
                doc.add_node(DocumentNode::new(NodeType::Paragraph, s));
            }
        }
    }
}
impl Backend for PptxBackend {
    fn convert(&self, input: &InputDocument) -> Result<DoclingDocument, ConversionError> {
        let parts = office::read_parts(input)?;
        let presentation = office::parse(office::part(&parts, "ppt/presentation.xml")?)?;
        let rels = office::relationships(&parts, "ppt/presentation.xml")?;
        let mut doc = DoclingDocument::new(office::name(input));
        let mut omitted_visuals = false;
        for (index, n) in presentation
            .descendants()
            .filter(|n| n.has_tag_name((P, "sldId")))
            .enumerate()
        {
            let id = n
                .attributes()
                .find(|a| a.name() == "id" && a.namespace().is_some())
                .ok_or_else(|| office::error("Missing PPTX slide relationship ID"))?
                .value();
            let (path, _) = rels
                .get(id)
                .ok_or_else(|| office::error("Missing PPTX slide relationship"))?;
            let slide = office::parse(office::part(&parts, path)?)?;
            doc.add_node(DocumentNode::new(
                NodeType::Heading,
                format!("Slide {}", index + 1),
            ));
            blocks(slide.root_element(), &mut doc);
            omitted_visuals |= slide
                .descendants()
                .any(|n| n.has_tag_name((P, "pic")) || n.tag_name().name() == "chart");
            for (_, (note_path, kind)) in office::relationships(&parts, path)? {
                if kind.ends_with("/notesSlide") {
                    let notes = office::parse(office::part(&parts, &note_path)?)?;
                    for shape in notes.descendants().filter(|n| n.has_tag_name((P, "sp"))) {
                        let placeholder = shape
                            .descendants()
                            .find(|n| n.has_tag_name((P, "ph")))
                            .and_then(|n| n.attribute("type"));
                        if matches!(
                            placeholder,
                            Some("sldNum" | "dt" | "hdr" | "ftr" | "sldImg")
                        ) {
                            continue;
                        }
                        blocks(shape, &mut doc);
                    }
                }
            }
        }
        if omitted_visuals {
            doc=doc.with_metadata("conversion_warnings",serde_json::json!(["Slide images/charts are not rendered or OCR-processed; text extraction may be incomplete."]));
        }
        Ok(doc)
    }
    fn supports_format(&self, f: InputFormat) -> bool {
        f == InputFormat::Pptx
    }
}
