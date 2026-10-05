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
            None => Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
                .or_else(|_| Pdfium::bind_to_system_library()),
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
        for (page_num, page) in pdf_doc.pages().iter().enumerate() {
            doc.add_node(DocumentNode::new(
                NodeType::Heading,
                format!("Page {}", page_num + 1),
            ));

            let text = page
                .text()
                .map_err(|e| ConversionError::ParseError(format!("Failed to extract text: {}", e)))?
                .all();

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

        if !textless_pages.is_empty() {
            doc = doc.with_metadata("textless_pages", serde_json::json!(textless_pages));
            doc = doc.with_metadata("conversion_warnings", serde_json::json!(["Some pages have no extractable text; they may be blank or require OCR. OCR was not performed."]));
        }
        Ok(doc)
    }

    fn supports_format(&self, format: InputFormat) -> bool {
        matches!(format, InputFormat::PDF)
    }
}
