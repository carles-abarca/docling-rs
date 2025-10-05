//! PDF backend implementation.

use super::config::PdfConfig;
use super::image_extractor::{ImageExtractor, PdfiumImageExtractor};
use crate::backend::Backend;
use crate::datamodel::{DoclingDocument, DocumentNode, DocumentSource, InputDocument, NodeType};
use crate::error::ConversionError;
use crate::InputFormat;
use pdfium_render::prelude::*;

// OCR engine imports (conditional on feature flag)
#[cfg(feature = "ocr")]
use super::ocr_engine::TesseractOcr;

// Note: text_extractor with detailed position tracking is available but not used in basic implementation
// It will be integrated in future iterations for advanced layout analysis

/// PDF backend for document conversion.
pub struct PdfBackend {
    config: PdfConfig,
    pdfium: Option<Pdfium>,
}

impl PdfBackend {
    /// Create a new PDF backend with default configuration.
    pub fn new() -> Self {
        let pdfium = Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
            .or_else(|_| Pdfium::bind_to_system_library())
            .ok()
            .map(Pdfium::new);

        Self {
            config: PdfConfig::default(),
            pdfium,
        }
    }

    /// Create a new PDF backend with custom configuration.
    pub fn with_config(config: PdfConfig) -> Self {
        let pdfium = Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
            .or_else(|_| Pdfium::bind_to_system_library())
            .ok()
            .map(Pdfium::new);

        Self { config, pdfium }
    }

    /// Get the pdfium instance, returning an error if not available.
    fn get_pdfium(&self) -> Result<&Pdfium, ConversionError> {
        self.pdfium.as_ref().ok_or_else(|| {
            ConversionError::ParseError(
                "Pdfium library not available. Please install pdfium-render library.".to_string(),
            )
        })
    }

    /// Load and convert a PDF document.
    fn convert_pdf(&self, input: &InputDocument) -> Result<DoclingDocument, ConversionError> {
        // Get pdfium instance
        let pdfium = self.get_pdfium()?;

        // Load PDF using pdfium
        let pdf = match input.source() {
            DocumentSource::FilePath(path) => {
                if let Some(password) = &self.config.password {
                    pdfium
                        .load_pdf_from_file(path, Some(password))
                        .map_err(|e| {
                            ConversionError::ParseError(format!("Failed to load PDF: {}", e))
                        })?
                } else {
                    pdfium.load_pdf_from_file(path, None).map_err(|e| {
                        ConversionError::ParseError(format!("Failed to load PDF: {}", e))
                    })?
                }
            }
            DocumentSource::Bytes { data, name } => {
                if let Some(password) = &self.config.password {
                    pdfium
                        .load_pdf_from_byte_slice(data, Some(password))
                        .map_err(|e| {
                            ConversionError::ParseError(format!(
                                "Failed to load PDF ({}): {}",
                                name, e
                            ))
                        })?
                } else {
                    pdfium.load_pdf_from_byte_slice(data, None).map_err(|e| {
                        ConversionError::ParseError(format!("Failed to load PDF ({}): {}", name, e))
                    })?
                }
            }
        };

        // Extract text from all pages
        let page_count = pdf.pages().len() as usize;
        let mut full_text = String::new();
        let mut all_images = Vec::new();

        // Initialize image extractor if enabled
        let image_extractor = if self.config.enable_images {
            Some(PdfiumImageExtractor::new())
        } else {
            None
        };

        // Initialize OCR engine if enabled
        #[cfg(feature = "ocr")]
        let ocr_engine = if self.config.enable_ocr {
            Some(TesseractOcr::new())
        } else {
            None
        };

        // Determine page range
        let range = self.config.page_range.clone().unwrap_or(0..page_count);

        for page_index in range {
            if page_index >= page_count {
                break;
            }

            let page = pdf.pages().get(page_index as u16).map_err(|e| {
                ConversionError::ParseError(format!("Failed to get page {}: {}", page_index, e))
            })?;

            // Extract text
            let text_page = page.text().map_err(|e| {
                ConversionError::ParseError(format!(
                    "Failed to get text from page {}: {}",
                    page_index, e
                ))
            })?;

            let page_text = text_page.all();

            // If no text and OCR is enabled, try OCR (indicates scanned PDF)
            #[cfg(feature = "ocr")]
            if page_text.trim().is_empty() && self.config.enable_ocr {
                if let Some(ref _ocr) = ocr_engine {
                    // TODO: Implement actual OCR here
                    // This requires:
                    // 1. Rendering the page to an image
                    // 2. Passing image to OCR engine
                    // 3. Extracting text from OCR result
                    // For now, we just log that OCR would be attempted
                    // page_text = perform_ocr(&page, ocr)?;
                }
            }

            if !page_text.is_empty() {
                full_text.push_str(&page_text);
                full_text.push('\n');
            }

            // Extract images if enabled
            if let Some(ref extractor) = image_extractor {
                let images = extractor.extract_images(&page);
                all_images.extend(images);
            }
        }

        // Create DoclingDocument
        let doc_name = match input.source() {
            DocumentSource::FilePath(path) => path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("document.pdf")
                .to_string(),
            DocumentSource::Bytes { name, .. } => name.clone(),
        };

        let mut doc = DoclingDocument::new(doc_name);

        // Create a single text node with all content
        if !full_text.trim().is_empty() {
            let node = DocumentNode::new(NodeType::Text, full_text);
            doc.add_node(node);
        }

        // Add image count as metadata
        if !all_images.is_empty() {
            doc = doc.with_metadata("image_count", all_images.len());
            // TODO: In a future phase, add actual Image nodes to the document
            // For now, we've successfully extracted and classified the images
        }

        Ok(doc)
    }
}

impl Default for PdfBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl Backend for PdfBackend {
    fn convert(&self, input: &InputDocument) -> Result<DoclingDocument, ConversionError> {
        // Verify input format
        if input.format() != InputFormat::PDF {
            return Err(ConversionError::UnsupportedFormat(format!(
                "Expected PDF format, got {:?}",
                input.format()
            )));
        }

        self.convert_pdf(input)
    }

    fn supports_format(&self, format: InputFormat) -> bool {
        matches!(format, InputFormat::PDF)
    }
}
