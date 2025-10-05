//! PDF backend configuration.

use std::ops::Range;

/// Configuration for PDF processing.
#[derive(Debug, Clone)]
pub struct PdfConfig {
    /// Password for encrypted PDFs.
    pub password: Option<String>,

    /// Page range to process (None = all pages).
    pub page_range: Option<Range<usize>>,

    /// Enable OCR for scanned PDFs.
    pub enable_ocr: bool,

    /// Enable table detection and extraction.
    pub enable_tables: bool,

    /// Enable image extraction.
    pub enable_images: bool,

    /// OCR language (default: "eng").
    pub ocr_language: String,
}

impl Default for PdfConfig {
    fn default() -> Self {
        Self {
            password: None,
            page_range: None,
            enable_ocr: false,
            enable_tables: true,
            enable_images: true,
            ocr_language: "eng".to_string(),
        }
    }
}

impl PdfConfig {
    /// Set password for encrypted PDF.
    pub fn password(mut self, password: Option<String>) -> Self {
        self.password = password;
        self
    }

    /// Set page range to process.
    pub fn page_range(mut self, range: Option<Range<usize>>) -> Self {
        self.page_range = range;
        self
    }

    /// Enable or disable OCR.
    pub fn enable_ocr(mut self, enable: bool) -> Self {
        self.enable_ocr = enable;
        self
    }

    /// Enable or disable table detection.
    pub fn enable_tables(mut self, enable: bool) -> Self {
        self.enable_tables = enable;
        self
    }

    /// Enable or disable image extraction.
    pub fn enable_images(mut self, enable: bool) -> Self {
        self.enable_images = enable;
        self
    }

    /// Set OCR language.
    pub fn ocr_language(mut self, language: &str) -> Self {
        self.ocr_language = language.to_string();
        self
    }
}
