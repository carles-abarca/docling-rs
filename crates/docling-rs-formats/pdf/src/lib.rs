//! PDF backend for docling-rs

use docling_rs_core::{
    Backend, ConversionError, DoclingDocument, DocumentNode, DocumentSource, InputDocument,
    InputFormat, NodeType,
};
use pdfium_render::prelude::*;

/// PDF backend
pub struct PdfBackend {
    library_path: Option<std::path::PathBuf>,
}

impl PdfBackend {
    /// Create a new PdfBackend
    pub fn new() -> Self {
        Self { library_path: None }
    }

    /// Use an explicit PDFium dynamic library file. No fallback is attempted.
    pub fn with_library_path(path: impl Into<std::path::PathBuf>) -> Self {
        Self {
            library_path: Some(path.into()),
        }
    }

    fn create_pdfium(&self) -> Result<Pdfium, ConversionError> {
        let bindings = match &self.library_path {
            Some(path) => Pdfium::bind_to_library(path),
            None => {
                let executable = std::env::current_exe().ok();
                let dir = executable.as_ref().and_then(|p| p.parent());
                let adjacent = dir.map(Pdfium::pdfium_platform_library_name_at_path);
                let frameworks = dir.map(|p| {
                    Pdfium::pdfium_platform_library_name_at_path(&p.join("../Frameworks"))
                });
                adjacent
                    .into_iter()
                    .chain(frameworks)
                    .find_map(|p| Pdfium::bind_to_library(p).ok())
                    .map(Ok)
                    .unwrap_or_else(|| {
                        Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
                            .or_else(|_| Pdfium::bind_to_system_library())
                    })
            }
        }
        .map_err(|e| {
            ConversionError::ParseError(format!("Failed to load pdfium library: {}", e))
        })?;

        Ok(Pdfium::new(bindings))
    }
}

impl Default for PdfBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl Backend for PdfBackend {
    fn convert(&self, input: &InputDocument) -> Result<DoclingDocument, ConversionError> {
        let name = match input.source() {
            DocumentSource::FilePath(path) => path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("document")
                .to_string(),
            DocumentSource::Bytes { name, .. } => name.clone(),
        };

        let mut doc = DoclingDocument::new(&name);

        let pdfium = self.create_pdfium()?;

        let pdf_doc = match input.source() {
            DocumentSource::FilePath(path) => pdfium
                .load_pdf_from_file(path, None)
                .map_err(|e| ConversionError::ParseError(format!("Failed to load PDF: {}", e)))?,
            DocumentSource::Bytes { data, .. } => pdfium
                .load_pdf_from_byte_slice(data, None)
                .map_err(|e| ConversionError::ParseError(format!("Failed to load PDF: {}", e)))?,
        };

        let mut textless_pages = Vec::new();
        let mut reordered_pages = Vec::new();
        for (page_num, page) in pdf_doc.pages().iter().enumerate() {
            doc.add_node(DocumentNode::new(
                NodeType::Heading,
                format!("Page {}", page_num + 1),
            ));

            let page_text = page
                .text()
                .map_err(|e| ConversionError::ParseError(format!("Failed to extract text: {e}")))?;
            let native = page_text.all();
            let text = if let Some(ordered) = column_text(&page_text, page.page_size()) {
                reordered_pages.push(page_num + 1);
                ordered
            } else {
                native
            };

            if text.trim().is_empty() {
                textless_pages.push(page_num + 1);
            }
            for line in text.lines() {
                let trimmed = line.trim();
                if !trimmed.is_empty() {
                    doc.add_node(DocumentNode::new(NodeType::Paragraph, trimmed));
                }
            }
        }

        let mut warnings = Vec::new();
        if !reordered_pages.is_empty() {
            doc = doc.with_metadata("column_order_pages", serde_json::json!(reordered_pages));
            warnings.push("Column order was inferred from a clear vertical gutter; complex layout/table semantics are not reconstructed.");
        }
        if !textless_pages.is_empty() {
            doc = doc.with_metadata("textless_pages", serde_json::json!(textless_pages));
            warnings.push("Some pages have no extractable text; they may be blank or require OCR. OCR was not performed.");
        }
        if !warnings.is_empty() {
            doc = doc.with_metadata("conversion_warnings", serde_json::json!(warnings));
        }
        Ok(doc)
    }

    fn supports_format(&self, format: InputFormat) -> bool {
        matches!(format, InputFormat::PDF)
    }
}

/// Conservative two-column ordering. Require a gutter across the entire page and
/// at least two vertically overlapping text rectangles on each side. Otherwise
/// preserve PDFium's order rather than guessing about a complex layout.
fn column_text(text: &PdfPageText<'_>, bounds: PdfRect) -> Option<String> {
    let rectangles: Vec<_> = text
        .segments()
        .iter()
        .map(|s| s.bounds())
        .filter(|r| r.width().value > 0.0)
        .collect();
    if rectangles.len() < 4 {
        return None;
    }
    let mut intervals: Vec<(f32, f32)> = rectangles
        .iter()
        .map(|r| (r.left().value, r.right().value))
        .collect();
    intervals.sort_by(|a, b| a.0.total_cmp(&b.0));
    let mut end = intervals[0].1;
    for &(left, right) in intervals.iter().skip(1) {
        if left - end > 36.0 {
            let split = (left + end) / 2.0;
            let a: Vec<_> = rectangles
                .iter()
                .filter(|r| r.right().value < split)
                .collect();
            let b: Vec<_> = rectangles
                .iter()
                .filter(|r| r.left().value > split)
                .collect();
            if a.len() >= 2 && b.len() >= 2 {
                let amin = a
                    .iter()
                    .map(|r| r.bottom().value)
                    .fold(f32::INFINITY, f32::min);
                let amax = a
                    .iter()
                    .map(|r| r.top().value)
                    .fold(f32::NEG_INFINITY, f32::max);
                let bmin = b
                    .iter()
                    .map(|r| r.bottom().value)
                    .fold(f32::INFINITY, f32::min);
                let bmax = b
                    .iter()
                    .map(|r| r.top().value)
                    .fold(f32::NEG_INFINITY, f32::max);
                if amin.max(bmin) < amax.min(bmax) {
                    let first = text.inside_rect(PdfRect::new_from_values(
                        bounds.bottom().value,
                        bounds.left().value,
                        bounds.top().value,
                        split,
                    ));
                    let second = text.inside_rect(PdfRect::new_from_values(
                        bounds.bottom().value,
                        split,
                        bounds.top().value,
                        bounds.right().value,
                    ));
                    return Some(format!("{}\n{}", first.trim(), second.trim()));
                }
            }
        }
        end = end.max(right);
    }
    None
}
