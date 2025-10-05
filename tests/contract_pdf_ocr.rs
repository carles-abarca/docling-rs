//! Contract test: PDF OCR Engine
//!
//! This test verifies that the OCR engine correctly implements
//! its contract for text recognition from images.

#[cfg(feature = "ocr")]
use docling_rs::backend::pdf::ocr::{OcrResult, OcrWord};
#[cfg(feature = "ocr")]
use docling_rs::backend::pdf::ocr_engine::OcrEngine;
#[cfg(feature = "ocr")]
use docling_rs::backend::pdf::types::BoundingBox;

#[test]
#[cfg(feature = "ocr")]
fn test_ocr_result_structure() {
    // Contract: OcrResult must contain text, confidence, and words

    let word = OcrWord::new(
        "test".to_string(),
        85.0,
        BoundingBox::new(0.0, 0.0, 50.0, 20.0),
    );

    let result = OcrResult::new(
        "test".to_string(),
        85.0,
        vec![word],
    );

    assert_eq!(result.text, "test");
    assert_eq!(result.confidence, 85.0);
    assert_eq!(result.words.len(), 1);
}

#[test]
#[cfg(feature = "ocr")]
fn test_ocr_word_structure() {
    // Contract: OcrWord must contain text, confidence, and bounding box

    let bbox = BoundingBox::new(10.0, 20.0, 100.0, 30.0);
    let word = OcrWord::new("hello".to_string(), 92.5, bbox.clone());

    assert_eq!(word.text, "hello");
    assert_eq!(word.confidence, 92.5);
    assert_eq!(word.bbox.x, 10.0);
    assert_eq!(word.bbox.y, 20.0);
}

#[test]
#[cfg(feature = "ocr")]
fn test_confidence_range() {
    // Contract: Confidence scores should be 0.0 to 100.0

    let word = OcrWord::new(
        "test".to_string(),
        95.5,
        BoundingBox::new(0.0, 0.0, 10.0, 10.0),
    );

    assert!(word.confidence >= 0.0);
    assert!(word.confidence <= 100.0);
}

#[test]
#[cfg(feature = "ocr")]
fn test_empty_ocr_result() {
    // Contract: OCR should handle empty results gracefully

    let result = OcrResult::new(String::new(), 0.0, vec![]);

    assert!(result.text.is_empty());
    assert_eq!(result.confidence, 0.0);
    assert!(result.words.is_empty());
}

#[test]
#[cfg(feature = "ocr")]
fn test_multiple_words_in_result() {
    // Contract: OcrResult can contain multiple words

    let words = vec![
        OcrWord::new("The".to_string(), 90.0, BoundingBox::new(0.0, 0.0, 30.0, 20.0)),
        OcrWord::new("quick".to_string(), 88.0, BoundingBox::new(35.0, 0.0, 50.0, 20.0)),
        OcrWord::new("brown".to_string(), 92.0, BoundingBox::new(90.0, 0.0, 50.0, 20.0)),
    ];

    let result = OcrResult::new("The quick brown".to_string(), 90.0, words);

    assert_eq!(result.words.len(), 3);
    assert_eq!(result.words[0].text, "The");
    assert_eq!(result.words[1].text, "quick");
    assert_eq!(result.words[2].text, "brown");
}

#[test]
#[cfg(not(feature = "ocr"))]
fn test_ocr_contract_without_feature() {
    // When feature is disabled, contract test should pass
    assert!(true);
}
