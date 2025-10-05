//! Helper functions to create PDF fixtures for testing.
//!
//! These functions create real PDF files for use in integration tests.

use printpdf::*;
use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;

/// Create a simple text PDF with given content.
pub fn create_simple_text_pdf(content: &str) -> PathBuf {
    let output_path = format!("/tmp/test_simple_{}.pdf", content.len());
    let path = PathBuf::from(&output_path);

    // Create PDF document
    let (doc, page1, layer1) = PdfDocument::new("Test PDF", Mm(210.0), Mm(297.0), "Layer 1");

    // Get font
    let font = doc.add_builtin_font(BuiltinFont::TimesRoman).unwrap();

    // Get current layer
    let current_layer = doc.get_page(page1).get_layer(layer1);

    // Write text
    let lines: Vec<&str> = content.lines().collect();
    let mut y_position = 280.0; // Start near top of page

    for line in lines {
        current_layer.use_text(line, 12.0, Mm(10.0), Mm(y_position), &font);
        y_position -= 6.0; // Move down for next line
    }

    // Save PDF
    let file = File::create(&path).unwrap();
    let mut writer = BufWriter::new(file);
    doc.save(&mut writer).unwrap();

    path
}

/// Create an empty PDF (one page, no content).
pub fn create_empty_pdf() -> PathBuf {
    let output_path = "/tmp/test_empty.pdf";
    let path = PathBuf::from(output_path);

    // Create PDF with empty page
    let (doc, _page1, _layer1) = PdfDocument::new("Empty PDF", Mm(210.0), Mm(297.0), "Layer 1");

    // Save without adding any content
    let file = File::create(&path).unwrap();
    let mut writer = BufWriter::new(file);
    doc.save(&mut writer).unwrap();

    path
}

/// Create a multi-page PDF with specific text on each page.
pub fn create_multipage_pdf(page_count: usize) -> PathBuf {
    let output_path = format!("/tmp/test_multipage_{}.pdf", page_count);
    let path = PathBuf::from(&output_path);

    let (doc, page1, layer1) = PdfDocument::new("Multi-page PDF", Mm(210.0), Mm(297.0), "Layer 1");
    let font = doc.add_builtin_font(BuiltinFont::TimesRoman).unwrap();

    // First page
    let current_layer = doc.get_page(page1).get_layer(layer1);
    current_layer.use_text("Page 1", 12.0, Mm(10.0), Mm(280.0), &font);

    // Add additional pages
    for i in 2..=page_count {
        let (page, layer) = doc.add_page(Mm(210.0), Mm(297.0), "Layer 1");
        let current_layer = doc.get_page(page).get_layer(layer);
        current_layer.use_text(&format!("Page {}", i), 12.0, Mm(10.0), Mm(280.0), &font);
    }

    // Save PDF
    let file = File::create(&path).unwrap();
    let mut writer = BufWriter::new(file);
    doc.save(&mut writer).unwrap();

    path
}

/// Create a PDF with specific text on each page (for testing reading order).
pub fn create_pdf_with_page_texts(texts: &[&str]) -> PathBuf {
    let output_path = format!("/tmp/test_pages_{}.pdf", texts.len());
    let path = PathBuf::from(&output_path);

    let (doc, page1, layer1) = PdfDocument::new("Page Texts PDF", Mm(210.0), Mm(297.0), "Layer 1");
    let font = doc.add_builtin_font(BuiltinFont::TimesRoman).unwrap();

    // First page
    if !texts.is_empty() {
        let current_layer = doc.get_page(page1).get_layer(layer1);
        current_layer.use_text(texts[0], 12.0, Mm(10.0), Mm(280.0), &font);
    }

    // Additional pages
    for text in texts.iter().skip(1) {
        let (page, layer) = doc.add_page(Mm(210.0), Mm(297.0), "Layer 1");
        let current_layer = doc.get_page(page).get_layer(layer);
        current_layer.use_text(*text, 12.0, Mm(10.0), Mm(280.0), &font);
    }

    // Save PDF
    let file = File::create(&path).unwrap();
    let mut writer = BufWriter::new(file);
    doc.save(&mut writer).unwrap();

    path
}

/// Create an encrypted PDF with a password.
/// Note: printpdf doesn't support encryption, so this returns a regular PDF.
/// For real encrypted PDF testing, you'd need pre-made encrypted PDFs.
pub fn create_encrypted_pdf(_content: &str, _password: &str) -> PathBuf {
    // printpdf doesn't support PDF encryption
    // For now, return path to a simple PDF
    // In production, you'd use pre-made encrypted PDFs or a different library
    create_simple_text_pdf("Encrypted content placeholder")
}
