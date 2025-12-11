//! Helper functions to create PDF fixtures for testing.
//!
//! These functions create real PDF files for use in integration tests.

use printpdf::*;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

/// Create a simple text PDF with given content.
#[allow(dead_code)]
pub fn create_simple_text_pdf(content: &str) -> PathBuf {
    // Use tempfile for cross-platform compatibility
    let temp_file = tempfile::Builder::new()
        .prefix("test_simple_")
        .suffix(".pdf")
        .tempfile()
        .expect("Failed to create temp file");

    let path = temp_file.path().to_path_buf();

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
    {
        let mut writer = BufWriter::new(temp_file.as_file());
        doc.save(&mut writer).unwrap();
        writer.flush().unwrap();
    } // writer is dropped here

    // Persist the temp file so it doesn't get deleted when temp_file is dropped
    temp_file.keep().unwrap();

    path
}

/// Create an empty PDF (one page, no content).
#[allow(dead_code)]
pub fn create_empty_pdf() -> PathBuf {
    // Use tempfile for cross-platform compatibility
    let temp_file = tempfile::Builder::new()
        .prefix("test_empty_")
        .suffix(".pdf")
        .tempfile()
        .expect("Failed to create temp file");

    let path = temp_file.path().to_path_buf();

    // Create PDF with empty page
    let (doc, _page1, _layer1) = PdfDocument::new("Empty PDF", Mm(210.0), Mm(297.0), "Layer 1");

    // Save without adding any content
    {
        let mut writer = BufWriter::new(temp_file.as_file());
        doc.save(&mut writer).unwrap();
        writer.flush().unwrap();
    } // writer is dropped here

    // Persist the temp file
    temp_file.keep().unwrap();

    path
}

/// Create a multi-page PDF with specific text on each page.
#[allow(dead_code)]
pub fn create_multipage_pdf(page_count: usize) -> PathBuf {
    // Use tempfile for cross-platform compatibility
    let temp_file = tempfile::Builder::new()
        .prefix(&format!("test_multipage_{}_", page_count))
        .suffix(".pdf")
        .tempfile()
        .expect("Failed to create temp file");

    let path = temp_file.path().to_path_buf();

    let (doc, page1, layer1) = PdfDocument::new("Multi-page PDF", Mm(210.0), Mm(297.0), "Layer 1");
    let font = doc.add_builtin_font(BuiltinFont::TimesRoman).unwrap();

    // First page
    let current_layer = doc.get_page(page1).get_layer(layer1);
    current_layer.use_text("Page 1", 12.0, Mm(10.0), Mm(280.0), &font);

    // Add additional pages
    for i in 2..=page_count {
        let (page, layer) = doc.add_page(Mm(210.0), Mm(297.0), "Layer 1");
        let current_layer = doc.get_page(page).get_layer(layer);
        current_layer.use_text(format!("Page {}", i), 12.0, Mm(10.0), Mm(280.0), &font);
    }

    // Save PDF
    {
        let mut writer = BufWriter::new(temp_file.as_file());
        doc.save(&mut writer).unwrap();
        writer.flush().unwrap();
    } // writer is dropped here

    // Persist the temp file
    temp_file.keep().unwrap();

    path
}

/// Create a PDF with specific text on each page (for testing reading order).
#[allow(dead_code)]
pub fn create_pdf_with_page_texts(texts: &[&str]) -> PathBuf {
    // Use tempfile for cross-platform compatibility
    let temp_file = tempfile::Builder::new()
        .prefix(&format!("test_pages_{}_", texts.len()))
        .suffix(".pdf")
        .tempfile()
        .expect("Failed to create temp file");

    let path = temp_file.path().to_path_buf();

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
    {
        let mut writer = BufWriter::new(temp_file.as_file());
        doc.save(&mut writer).unwrap();
        writer.flush().unwrap();
    } // writer is dropped here

    // Persist the temp file
    temp_file.keep().unwrap();

    path
}

/// Get path to an encrypted PDF with a password.
/// Uses pre-made encrypted PDFs from tests/fixtures/pdfs/
/// These PDFs were created using qpdf with 256-bit AES encryption.
#[allow(dead_code)]
pub fn create_encrypted_pdf(_content: &str, password: &str) -> PathBuf {
    // Map passwords to pre-made encrypted PDF files
    let filename = match password {
        "secret123" => "encrypted_secret123.pdf",
        "password123" => "encrypted_password123.pdf",
        "correct_password" => "encrypted_correct_password.pdf",
        _ => "encrypted_secret123.pdf", // Default
    };

    PathBuf::from("tests/fixtures/pdfs").join(filename)
}
