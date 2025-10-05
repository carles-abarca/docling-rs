//! OCR types and results for PDF processing.

use super::types::BoundingBox;
use serde::{Deserialize, Serialize};

/// Result of OCR text recognition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcrResult {
    /// Recognized text
    pub text: String,

    /// Overall confidence score (0.0 to 100.0)
    pub confidence: f32,

    /// Individual words with positions and confidence
    pub words: Vec<OcrWord>,

    /// Optional language used for recognition
    pub language: Option<String>,
}

impl OcrResult {
    /// Create a new OCR result.
    pub fn new(text: String, confidence: f32, words: Vec<OcrWord>) -> Self {
        Self {
            text,
            confidence,
            words,
            language: None,
        }
    }

    /// Set the language.
    pub fn with_language(mut self, language: String) -> Self {
        self.language = Some(language);
        self
    }

    /// Check if the result is empty.
    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    /// Get the number of words recognized.
    pub fn word_count(&self) -> usize {
        self.words.len()
    }
}

/// A single word recognized by OCR.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcrWord {
    /// Word text
    pub text: String,

    /// Word confidence score (0.0 to 100.0)
    pub confidence: f32,

    /// Bounding box of the word
    pub bbox: BoundingBox,
}

impl OcrWord {
    /// Create a new OCR word.
    pub fn new(text: String, confidence: f32, bbox: BoundingBox) -> Self {
        Self {
            text,
            confidence,
            bbox,
        }
    }

    /// Check if this word has high confidence (> 80%).
    pub fn is_high_confidence(&self) -> bool {
        self.confidence > 80.0
    }

    /// Check if this word has low confidence (< 50%).
    pub fn is_low_confidence(&self) -> bool {
        self.confidence < 50.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ocr_result_creation() {
        let word = OcrWord::new(
            "test".to_string(),
            90.0,
            BoundingBox::new(0.0, 0.0, 50.0, 20.0),
        );

        let result = OcrResult::new("test".to_string(), 90.0, vec![word]);

        assert_eq!(result.text, "test");
        assert_eq!(result.confidence, 90.0);
        assert_eq!(result.word_count(), 1);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_ocr_result_with_language() {
        let result = OcrResult::new("hello".to_string(), 85.0, vec![])
            .with_language("eng".to_string());

        assert_eq!(result.language, Some("eng".to_string()));
    }

    #[test]
    fn test_ocr_word_confidence_levels() {
        let high_conf = OcrWord::new(
            "high".to_string(),
            95.0,
            BoundingBox::new(0.0, 0.0, 50.0, 20.0),
        );

        let low_conf = OcrWord::new(
            "low".to_string(),
            30.0,
            BoundingBox::new(0.0, 0.0, 50.0, 20.0),
        );

        assert!(high_conf.is_high_confidence());
        assert!(!high_conf.is_low_confidence());

        assert!(!low_conf.is_high_confidence());
        assert!(low_conf.is_low_confidence());
    }

    #[test]
    fn test_empty_ocr_result() {
        let result = OcrResult::new(String::new(), 0.0, vec![]);

        assert!(result.is_empty());
        assert_eq!(result.word_count(), 0);
    }
}
