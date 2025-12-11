//! HTML backend tests

use docling_rs::backend::{Backend, HtmlBackend};
use docling_rs::datamodel::InputDocument;
use docling_rs::InputFormat;

#[test]
fn test_html_backend_supports_format() {
    let backend = HtmlBackend::new();

    assert!(backend.supports_format(InputFormat::Html));
    assert!(!backend.supports_format(InputFormat::Markdown));
    assert!(!backend.supports_format(InputFormat::Csv));
    assert!(!backend.supports_format(InputFormat::Docx));
}

#[test]
fn test_html_backend_convert_simple() {
    let backend = HtmlBackend::new();
    let html = b"<html><body><h1>Hello World</h1></body></html>".to_vec();
    let input = InputDocument::from_bytes(html, "test.html", InputFormat::Html);

    let result = backend.convert(&input);
    assert!(result.is_ok());

    let doc = result.unwrap();
    assert_eq!(doc.name(), "test.html");
}

#[test]
fn test_html_backend_convert_paragraph() {
    let backend = HtmlBackend::new();
    let html = b"<html><body><p>This is a paragraph.</p></body></html>".to_vec();
    let input = InputDocument::from_bytes(html, "test.html", InputFormat::Html);

    let result = backend.convert(&input);
    assert!(result.is_ok());
}

#[test]
fn test_html_backend_convert_list() {
    let backend = HtmlBackend::new();
    let html = b"<html><body><ul><li>Item 1</li><li>Item 2</li></ul></body></html>".to_vec();
    let input = InputDocument::from_bytes(html, "test.html", InputFormat::Html);

    let result = backend.convert(&input);
    assert!(result.is_ok());
}

#[test]
fn test_html_backend_convert_table() {
    let backend = HtmlBackend::new();
    let html = b"<html><body><table><tr><td>A1</td><td>B1</td></tr></table></body></html>".to_vec();
    let input = InputDocument::from_bytes(html, "test.html", InputFormat::Html);

    let result = backend.convert(&input);
    assert!(result.is_ok());
}
