use docling_rs::chunking::{HuggingFaceTokenizer, HybridChunker, Tokenizer};
use docling_rs::{DoclingDocument, DocumentConverter, DocumentNode, InputFormat, NodeType};
fn convert(data: &[u8], format: InputFormat) -> docling_rs::ConversionResult {
    DocumentConverter::new()
        .convert_bytes(data.to_vec(), "fixture".into(), format)
        .unwrap()
}
#[test]
fn docx_corruption_is_an_error_not_a_panic() {
    for data in [
        include_bytes!("fixtures/attachments/mutation-docx-1.docx").as_slice(),
        include_bytes!("fixtures/attachments/mutation-docx-3.docx").as_slice(),
        include_bytes!("fixtures/attachments/mutation-docx-4.docx").as_slice(),
    ] {
        assert!(DocumentConverter::new()
            .convert_bytes(data.to_vec(), "bad.docx".into(), InputFormat::Docx)
            .is_err());
    }
}
#[test]
fn docx_headers_links_nested_tables_and_footer_survive() {
    let r = convert(
        include_bytes!("fixtures/attachments/docx-edge.docx"),
        InputFormat::Docx,
    );
    let text = docling_rs::output::to_text(r.document());
    for marker in [
        "HEADER_MARKER",
        "HYPERLINK_MARKER",
        "NESTED_MARKER",
        "FOOTER_MARKER",
        "LINE_ONE\nLINE_TWO\tTAB_MARKER",
    ] {
        assert!(text.contains(marker), "missing {marker}");
    }
}
#[test]
fn pptx_tables_and_notes_survive_chunking() {
    let r = convert(
        include_bytes!("fixtures/attachments/pptx-edge.pptx"),
        InputFormat::Pptx,
    );
    let c = HybridChunker::new(Box::new(HuggingFaceTokenizer::default_embedded().unwrap()));
    let text = c
        .try_chunk(r.document())
        .unwrap()
        .iter()
        .map(|c| c.text.as_str())
        .collect::<Vec<_>>()
        .join(" ");
    for marker in ["TABLE_TITLE", "SLIDE_CELL", "SPEAKER_NOTES"] {
        assert!(text.contains(marker), "missing {marker}");
    }
}
#[test]
fn pptx_uses_relationship_order_including_reordered_slides() {
    for (data, reverse) in [
        (
            include_bytes!("fixtures/attachments/slides-20.pptx").as_slice(),
            false,
        ),
        (
            include_bytes!("fixtures/attachments/reordered.pptx").as_slice(),
            true,
        ),
    ] {
        let r = convert(data, InputFormat::Pptx);
        let text = docling_rs::output::to_text(r.document());
        let mut ids: Vec<_> = (0..20).collect();
        if reverse {
            ids.reverse()
        }
        let positions: Vec<_> = ids
            .iter()
            .map(|i| text.find(&format!("SLIDE{i:06}")).unwrap())
            .collect();
        assert!(positions.windows(2).all(|w| w[0] < w[1]));
    }
}
#[test]
fn html_keeps_direct_text_and_inline_words_without_scripts() {
    let r = convert(
        b"<div>DIRECT<p>in<strong>voice</strong><br>next</p>AFTER<script>HIDDEN</script></div>",
        InputFormat::Html,
    );
    let text = docling_rs::output::to_text(r.document());
    for s in ["DIRECT", "invoice", "next", "AFTER"] {
        assert!(text.contains(s));
    }
    assert!(!text.contains("HIDDEN"));
}
#[test]
fn excessive_html_depth_returns_error() {
    let s = format!("{}text{}", "<div>".repeat(300), "</div>".repeat(300));
    assert!(DocumentConverter::new()
        .convert_bytes(s.into_bytes(), "deep.html".into(), InputFormat::Html)
        .is_err());
}
#[test]
fn cjk_token_limit_and_full_content_are_preserved() {
    let text = "中".repeat(1000);
    let doc =
        DoclingDocument::new("cjk").with_nodes(vec![DocumentNode::new(NodeType::Paragraph, &text)]);
    let c = HybridChunker::new(Box::new(HuggingFaceTokenizer::default_embedded().unwrap()));
    let chunks = c.try_chunk(&doc).unwrap();
    let counter = HuggingFaceTokenizer::default_embedded().unwrap();
    assert!(chunks.len() > 1);
    assert!(chunks.iter().all(|c| counter.count_tokens(&c.text) <= 128));
    assert_eq!(
        chunks.iter().map(|c| c.text.as_str()).collect::<String>(),
        text
    );
}
#[test]
fn impossible_token_budget_reports_error() {
    struct Two;
    impl Tokenizer for Two {
        fn count_tokens(&self, s: &str) -> usize {
            s.chars().count() * 2
        }
        fn max_tokens(&self) -> usize {
            1
        }
    }
    let doc =
        DoclingDocument::new("test").with_nodes(vec![DocumentNode::new(NodeType::Paragraph, "x")]);
    let c = HybridChunker::new(Box::new(Two));
    assert!(c.try_chunk(&doc).is_err());
}
#[test]
#[ignore = "requires PDFium dynamic library"]
fn pdf_two_columns_are_ordered_and_heuristic_is_disclosed() {
    let c = DocumentConverter::with_pdfium_library(std::env::var("DOCLING_TEST_PDFIUM").unwrap());
    let r = c
        .convert_bytes(
            include_bytes!("fixtures/attachments/columns.pdf").to_vec(),
            "columns.pdf".into(),
            InputFormat::PDF,
        )
        .unwrap();
    let t = docling_rs::output::to_text(r.document());
    let p: Vec<_> = ["LEFT_ONE", "LEFT_TWO", "RIGHT_ONE", "RIGHT_TWO"]
        .iter()
        .map(|s| t.find(s).unwrap())
        .collect();
    assert!(p.windows(2).all(|w| w[0] < w[1]), "{t}");
    assert!(matches!(
        r.status(),
        docling_rs::ConversionStatus::PartialSuccess
    ));
}

#[test]
fn html_preserves_spaces_at_inline_boundaries() {
    let r = convert(
        b"<div>hello<span> world </span>again</div>",
        InputFormat::Html,
    );
    assert!(docling_rs::output::to_text(r.document()).contains("hello world again"));
}
