//! Simple pipeline implementation

use crate::backend::{CsvBackend, DocxBackend, HtmlBackend, MarkdownBackend, PdfBackend};
use crate::datamodel::{ConversionResult, ConversionStatus, InputDocument};
use crate::error::ConversionError;
use crate::pipeline::Pipeline;
use crate::InputFormat;

/// Simple pipeline (build → assemble → enrich)
pub struct SimplePipeline {
    markdown_backend: MarkdownBackend,
    html_backend: HtmlBackend,
    csv_backend: CsvBackend,
    docx_backend: DocxBackend,
    pdf_backend: PdfBackend,
}

impl SimplePipeline {
    /// Create a new SimplePipeline
    pub fn new() -> Self {
        Self {
            markdown_backend: MarkdownBackend::new(),
            html_backend: HtmlBackend::new(),
            csv_backend: CsvBackend::new(),
            docx_backend: DocxBackend::new(),
            pdf_backend: PdfBackend::new(),
        }
    }
}

impl Default for SimplePipeline {
    fn default() -> Self {
        Self::new()
    }
}

impl Pipeline for SimplePipeline {
    fn execute(&self, input: &InputDocument) -> Result<ConversionResult, ConversionError> {
        use crate::backend::Backend;

        // Select backend based on format
        let document = match input.format() {
            InputFormat::Markdown => self.markdown_backend.convert(input)?,
            InputFormat::Html => self.html_backend.convert(input)?,
            InputFormat::Csv => self.csv_backend.convert(input)?,
            InputFormat::Docx => self.docx_backend.convert(input)?,
            InputFormat::PDF => self.pdf_backend.convert(input)?,
        };

        // Create conversion result
        let result = ConversionResult::new(document, ConversionStatus::Success);

        Ok(result)
    }
}
