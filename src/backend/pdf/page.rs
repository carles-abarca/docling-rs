//! PDF page representation.

use super::types::{BoundingBox, FontInfo, PageDimensions, Rotation};
use serde::{Deserialize, Serialize};

/// Represents a single PDF page.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfPage {
    /// Page number (0-indexed).
    pub page_number: usize,

    /// Page dimensions.
    pub dimensions: PageDimensions,

    /// Page rotation.
    pub rotation: Rotation,

    /// Text blocks extracted from the page.
    pub text_blocks: Vec<TextBlock>,
}

impl PdfPage {
    /// Create a new PDF page.
    pub fn new(page_number: usize, dimensions: PageDimensions) -> Self {
        Self {
            page_number,
            dimensions,
            rotation: Rotation::None,
            text_blocks: Vec::new(),
        }
    }

    /// Add a text block to the page.
    pub fn add_text_block(&mut self, block: TextBlock) {
        self.text_blocks.push(block);
    }
}

/// Text block with position and formatting.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextBlock {
    /// Text content.
    pub text: String,

    /// Bounding box.
    pub bbox: BoundingBox,

    /// Font information.
    pub font_info: FontInfo,

    /// Reading order index.
    pub reading_order: usize,

    /// Column ID (for multi-column layouts).
    pub column_id: Option<usize>,

    /// Block type.
    pub block_type: TextBlockType,

    /// Confidence score (if from OCR or ML).
    pub confidence: Option<f32>,
}

/// Type of text block.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextBlockType {
    /// Regular paragraph text.
    Paragraph,

    /// Heading (level unspecified).
    Heading,

    /// List item.
    ListItem,

    /// Caption.
    Caption,

    /// Footer.
    Footer,

    /// Header.
    Header,

    /// Unknown or unclassified.
    Unknown,
}

impl Default for TextBlockType {
    fn default() -> Self {
        Self::Unknown
    }
}
