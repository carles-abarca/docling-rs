//! Text extraction from PDF pages.

use super::page::{PdfPage, TextBlock, TextBlockType};
use super::types::{BoundingBox, FontInfo};
use crate::error::ConversionError;
use pdfium_render::prelude::*;

/// Extracts text with positions from a PDF page.
pub struct TextExtractor;

impl TextExtractor {
    /// Create a new text extractor.
    pub fn new() -> Self {
        Self
    }

    /// Extract text blocks from a pdfium page.
    pub fn extract_from_page<'a>(
        &self,
        page: &'a PdfPage<'a>,
        page_number: usize,
    ) -> Result<Vec<TextBlock>, ConversionError> {
        let mut text_blocks = Vec::new();
        let mut reading_order = 0;

        // Get all text objects on the page
        let text_page = page
            .text()
            .map_err(|e| ConversionError::ParseError(format!("Failed to get text page: {}", e)))?;

        // Extract text with character-level positions
        let char_count = text_page.chars().len();

        if char_count == 0 {
            return Ok(text_blocks);
        }

        // Group characters into text blocks
        // For now, we'll create one text block per line
        let mut current_text = String::new();
        let mut current_bounds: Option<PdfRect> = None;
        let mut current_font_size = 0.0;

        for char_index in 0..char_count {
            if let Some(text_char) = text_page.chars().get(char_index) {
                let char_str = text_char.text();
                let bounds = text_char.loose_bounds();

                // Check if this is a newline or if we should start a new block
                if char_str == "\n" || char_str == "\r" {
                    if !current_text.is_empty() {
                        if let Some(bbox) = current_bounds {
                            text_blocks.push(self.create_text_block(
                                current_text.clone(),
                                bbox,
                                current_font_size,
                                reading_order,
                            ));
                            reading_order += 1;
                        }
                        current_text.clear();
                        current_bounds = None;
                    }
                    continue;
                }

                // Add character to current block
                current_text.push_str(&char_str);

                // Update bounding box
                if let Some(existing_bounds) = current_bounds {
                    current_bounds = Some(self.merge_bounds(existing_bounds, bounds));
                } else {
                    current_bounds = Some(bounds);
                }

                // Estimate font size from character height
                current_font_size = bounds.height().value;
            }
        }

        // Add final block if any
        if !current_text.is_empty() {
            if let Some(bbox) = current_bounds {
                text_blocks.push(self.create_text_block(
                    current_text,
                    bbox,
                    current_font_size,
                    reading_order,
                ));
            }
        }

        Ok(text_blocks)
    }

    /// Create a text block from extracted data.
    fn create_text_block(
        &self,
        text: String,
        bounds: PdfRect,
        font_size: f32,
        reading_order: usize,
    ) -> TextBlock {
        let bbox = BoundingBox::new(
            bounds.left.value as f64,
            bounds.top.value as f64,
            bounds.width().value as f64,
            bounds.height().value as f64,
        );

        let font_info = FontInfo {
            name: "Unknown".to_string(), // pdfium-render doesn't easily expose font names
            size: font_size as f64,
            bold: false, // Would need more analysis to detect
            italic: false,
        };

        TextBlock {
            text,
            bbox,
            font_info,
            reading_order,
            column_id: None,
            block_type: TextBlockType::Paragraph,
            confidence: None,
        }
    }

    /// Merge two bounding boxes.
    fn merge_bounds(&self, a: PdfRect, b: PdfRect) -> PdfRect {
        let left = a.left.value.min(b.left.value);
        let top = a.top.value.min(b.top.value);
        let right = a.right.value.max(b.right.value);
        let bottom = a.bottom.value.max(b.bottom.value);

        PdfRect::new(
            PdfPoints::new(left),
            PdfPoints::new(top),
            PdfPoints::new(right),
            PdfPoints::new(bottom),
        )
    }
}

impl Default for TextExtractor {
    fn default() -> Self {
        Self::new()
    }
}
