use docling_rs::chunking::{HuggingFaceTokenizer, Tokenizer};
use docling_rs::{DocumentConverter, InputFormat, TableCell, TableData, TableRow};

#[test]
fn tokenizer_counts_content_without_padding_or_truncation() {
    let tokenizer = HuggingFaceTokenizer::default_embedded().unwrap();
    assert_eq!(tokenizer.count_tokens(""), 0);
    assert_eq!(tokenizer.count_tokens("hello"), 1);
    assert!(tokenizer.count_tokens(&"hello ".repeat(300)) >= 300);
}

#[test]
fn table_chunks_preserve_header_only_blank_keys_and_ragged_rows() {
    for rows in [
        vec![vec!["ONLY", "José"]],
        vec![vec!["", ""], vec!["", "value"]],
        vec![vec!["header"], vec!["row", "EXTRA"]],
        vec![vec!["a"], vec!["b"]],
    ] {
        let mut table = TableData::new();
        for row in &rows {
            table.add_row(TableRow::new(
                row.iter().map(|s| TableCell::new(*s)).collect(),
            ));
        }
        let chunks = table.rows_as_chunks().join(" ");
        let markdown = table.to_markdown();
        for cell in rows.iter().flatten().filter(|c| !c.is_empty()) {
            assert!(chunks.contains(cell), "Missing {cell} in chunks");
            assert!(markdown.contains(cell), "Missing {cell} in markdown");
        }
    }
}

#[test]
#[cfg(feature = "html")]
fn html_preserves_order_tables_and_excludes_scripts() {
    let html = b"<p>FIRST</p><h2>SECOND</h2><table><tr><td>THIRD</td><td>9876</td></tr></table><p>FOURTH<script>SECRET</script></p>";
    let result = DocumentConverter::new()
        .convert_bytes(html.to_vec(), "test.html".into(), InputFormat::Html)
        .unwrap();
    let text = docling_rs::output::to_text(result.document());
    assert!(!text.contains("SECRET"));
    assert!(text.contains("9876"));
    let offsets: Vec<_> = ["FIRST", "SECOND", "THIRD", "FOURTH"]
        .iter()
        .map(|s| text.find(s).unwrap())
        .collect();
    assert!(offsets.windows(2).all(|w| w[0] < w[1]));
}

#[test]
#[cfg(feature = "docx")]
fn docx_retains_table_cells() {
    let result = DocumentConverter::new()
        .convert_bytes(
            include_bytes!("fixtures/attachments/report.docx").to_vec(),
            "test.docx".into(),
            InputFormat::Docx,
        )
        .unwrap();
    let text = docling_rs::output::to_text(result.document());
    for expected in ["José", "2950", "México"] {
        assert!(text.contains(expected), "Missing {expected}");
    }
}

#[test]
#[cfg(feature = "xlsx")]
fn xlsx_retains_uncached_formulas_and_single_row_sheets() {
    let result = DocumentConverter::new()
        .convert_bytes(
            include_bytes!("fixtures/attachments/book.xlsx").to_vec(),
            "test.xlsx".into(),
            InputFormat::Xlsx,
        )
        .unwrap();
    let text = docling_rs::output::to_text(result.document());
    for expected in ["=SUM(B2:B3)", "SEGUNDA_HOJA", "José"] {
        assert!(text.contains(expected), "Missing {expected}");
    }
}

#[test]
#[cfg(feature = "pdf")]
fn explicit_missing_pdfium_returns_error_without_fallback() {
    let result = DocumentConverter::with_pdfium_library("/nonexistent/pdfium/library")
        .convert_bytes(
            include_bytes!("fixtures/attachments/text.pdf").to_vec(),
            "test.pdf".into(),
            InputFormat::PDF,
        );
    assert!(result.is_err());
}

/// Run explicitly with DOCLING_TEST_PDFIUM pointing to a compatible library file.
#[test]
#[ignore = "requires PDFium dynamic library"]
#[cfg(feature = "pdf")]
fn pdfium_extracts_text_and_marks_textless_pages_partial() {
    let converter =
        DocumentConverter::with_pdfium_library(std::env::var("DOCLING_TEST_PDFIUM").unwrap());
    for (bytes, partial) in [
        (
            include_bytes!("fixtures/attachments/text.pdf").as_slice(),
            false,
        ),
        (
            include_bytes!("fixtures/attachments/no_text.pdf").as_slice(),
            true,
        ),
        (
            include_bytes!("fixtures/attachments/scan.pdf").as_slice(),
            true,
        ),
    ] {
        let result = converter
            .convert_bytes(bytes.to_vec(), "test.pdf".into(), InputFormat::PDF)
            .unwrap();
        assert_eq!(
            matches!(
                result.status(),
                docling_rs::ConversionStatus::PartialSuccess
            ),
            partial
        );
        if partial {
            assert!(result.document().metadata().contains_key("textless_pages"));
        } else {
            assert!(docling_rs::output::to_text(result.document()).contains("FINAL_DOCUMENTO"));
        }
    }
}
