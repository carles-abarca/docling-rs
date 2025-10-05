//! Layout analysis for PDF pages.
//!
//! This module provides layout analysis capabilities including:
//! - Column detection
//! - Reading order determination
//! - Text block organization

use super::layout::{Column, LayoutInfo};
use super::page::TextBlock;
use super::types::BoundingBox;

/// Trait for layout analysis implementations.
///
/// Layout analyzers detect columns and determine reading order
/// for text blocks on a PDF page.
pub trait LayoutAnalyzer {
    /// Analyze the layout of text blocks on a page.
    ///
    /// # Arguments
    ///
    /// * `text_blocks` - The text blocks to analyze
    /// * `page_width` - Width of the page
    /// * `page_height` - Height of the page
    ///
    /// # Returns
    ///
    /// Layout information including columns and reading order
    fn analyze(&self, text_blocks: &[TextBlock], page_width: f64, page_height: f64) -> LayoutInfo;
}

/// Rule-based layout analyzer.
///
/// Uses heuristics and rules to detect columns and determine reading order:
/// - Detects columns via whitespace analysis
/// - Determines reading order via left-to-right, top-to-bottom ordering
pub struct RuleBasedLayoutAnalyzer {
    /// Minimum gap between columns (as fraction of page width)
    column_gap_threshold: f64,

    /// Tolerance for vertical alignment (in points)
    vertical_alignment_tolerance: f64,
}

impl RuleBasedLayoutAnalyzer {
    /// Create a new rule-based layout analyzer with default settings.
    pub fn new() -> Self {
        Self {
            column_gap_threshold: 0.05,         // 5% of page width
            vertical_alignment_tolerance: 10.0, // 10 points
        }
    }

    /// Create a new layout analyzer with custom settings.
    pub fn with_settings(column_gap_threshold: f64, vertical_alignment_tolerance: f64) -> Self {
        Self {
            column_gap_threshold,
            vertical_alignment_tolerance,
        }
    }

    /// Detect columns based on horizontal gaps in text blocks.
    fn detect_columns(&self, text_blocks: &[TextBlock], page_width: f64) -> Vec<Column> {
        if text_blocks.is_empty() {
            return Vec::new();
        }

        // Sort blocks by x-position
        let mut sorted_blocks: Vec<(usize, &TextBlock)> = text_blocks.iter().enumerate().collect();
        sorted_blocks.sort_by(|a, b| {
            a.1.bbox
                .x
                .partial_cmp(&b.1.bbox.x)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        let mut columns = Vec::new();
        let min_gap = page_width * self.column_gap_threshold;

        let mut current_column_blocks = vec![sorted_blocks[0]];
        let mut column_id = 0;

        for i in 1..sorted_blocks.len() {
            let prev_block = sorted_blocks[i - 1].1;
            let curr_block = sorted_blocks[i].1;

            let gap = curr_block.bbox.x - (prev_block.bbox.x + prev_block.bbox.width);

            if gap > min_gap {
                // Start new column
                let column =
                    self.create_column_from_blocks(column_id, &current_column_blocks, text_blocks);
                columns.push(column);
                column_id += 1;
                current_column_blocks = vec![sorted_blocks[i]];
            } else {
                current_column_blocks.push(sorted_blocks[i]);
            }
        }

        // Add the last column
        if !current_column_blocks.is_empty() {
            let column =
                self.create_column_from_blocks(column_id, &current_column_blocks, text_blocks);
            columns.push(column);
        }

        // If only one column detected and it doesn't cover most of page, treat as single column
        if columns.len() == 1 || columns.is_empty() {
            return self.create_single_column(text_blocks, page_width);
        }

        columns
    }

    /// Create a column from a list of text block indices.
    fn create_column_from_blocks(
        &self,
        id: usize,
        blocks: &[(usize, &TextBlock)],
        _all_blocks: &[TextBlock],
    ) -> Column {
        if blocks.is_empty() {
            return Column::new(id, BoundingBox::new(0.0, 0.0, 0.0, 0.0));
        }

        // Calculate bounding box for the column
        let min_x = blocks
            .iter()
            .map(|(_, b)| b.bbox.x)
            .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(0.0);

        let max_x = blocks
            .iter()
            .map(|(_, b)| b.bbox.x + b.bbox.width)
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(0.0);

        let min_y = blocks
            .iter()
            .map(|(_, b)| b.bbox.y)
            .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(0.0);

        let max_y = blocks
            .iter()
            .map(|(_, b)| b.bbox.y + b.bbox.height)
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(0.0);

        let bbox = BoundingBox::new(min_x, min_y, max_x - min_x, max_y - min_y);

        let mut column = Column::new(id, bbox);
        for (idx, _) in blocks {
            column.add_text_block(*idx);
        }

        column
    }

    /// Create a single column containing all text blocks.
    fn create_single_column(&self, text_blocks: &[TextBlock], page_width: f64) -> Vec<Column> {
        if text_blocks.is_empty() {
            return Vec::new();
        }

        let mut column = Column::new(0, BoundingBox::new(0.0, 0.0, page_width, 0.0));
        for (idx, _) in text_blocks.iter().enumerate() {
            column.add_text_block(idx);
        }

        vec![column]
    }

    /// Determine reading order for text blocks.
    ///
    /// For multi-column layouts: left-to-right column order,
    /// top-to-bottom within each column.
    fn determine_reading_order(&self, columns: &[Column], text_blocks: &[TextBlock]) -> Vec<usize> {
        let mut reading_order = Vec::new();

        // Process columns from left to right
        for column in columns {
            // Get blocks in this column
            let mut column_blocks: Vec<(usize, &TextBlock)> = column
                .text_block_indices
                .iter()
                .map(|&idx| (idx, &text_blocks[idx]))
                .collect();

            // Sort by y-position (top to bottom)
            column_blocks.sort_by(|a, b| {
                a.1.bbox
                    .y
                    .partial_cmp(&b.1.bbox.y)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });

            // Add to reading order
            for (idx, _) in column_blocks {
                reading_order.push(idx);
            }
        }

        reading_order
    }
}

impl Default for RuleBasedLayoutAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl LayoutAnalyzer for RuleBasedLayoutAnalyzer {
    fn analyze(&self, text_blocks: &[TextBlock], page_width: f64, page_height: f64) -> LayoutInfo {
        let mut layout = LayoutInfo::new(page_width, page_height);

        if text_blocks.is_empty() {
            return layout;
        }

        // Detect columns
        let columns = self.detect_columns(text_blocks, page_width);

        // Determine reading order
        let reading_order = self.determine_reading_order(&columns, text_blocks);

        // Update layout info
        for column in columns {
            layout.add_column(column);
        }
        layout.set_reading_order(reading_order);

        layout
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::pdf::page::TextBlockType;
    use crate::backend::pdf::types::FontInfo;

    fn create_test_block(text: &str, x: f64, y: f64, width: f64, height: f64) -> TextBlock {
        TextBlock {
            text: text.to_string(),
            bbox: BoundingBox::new(x, y, width, height),
            font_info: FontInfo {
                name: "Arial".to_string(),
                size: 12.0,
                bold: false,
                italic: false,
            },
            reading_order: 0,
            column_id: None,
            block_type: TextBlockType::Paragraph,
            confidence: None,
        }
    }

    #[test]
    fn test_single_column_layout() {
        let analyzer = RuleBasedLayoutAnalyzer::new();
        let blocks = vec![
            create_test_block("Block 1", 100.0, 100.0, 400.0, 50.0),
            create_test_block("Block 2", 100.0, 200.0, 400.0, 50.0),
            create_test_block("Block 3", 100.0, 300.0, 400.0, 50.0),
        ];

        let layout = analyzer.analyze(&blocks, 600.0, 800.0);

        assert_eq!(layout.columns.len(), 1);
        assert_eq!(layout.reading_order, vec![0, 1, 2]);
    }

    #[test]
    fn test_two_column_layout() {
        let analyzer = RuleBasedLayoutAnalyzer::new();
        let blocks = vec![
            // Left column
            create_test_block("L1", 50.0, 100.0, 200.0, 50.0),
            create_test_block("L2", 50.0, 200.0, 200.0, 50.0),
            // Right column (with sufficient gap)
            create_test_block("R1", 350.0, 100.0, 200.0, 50.0),
            create_test_block("R2", 350.0, 200.0, 200.0, 50.0),
        ];

        let layout = analyzer.analyze(&blocks, 600.0, 800.0);

        assert_eq!(layout.columns.len(), 2);
        // Reading order should be left column first, then right
        assert!(layout.reading_order.len() == 4);
    }

    #[test]
    fn test_empty_input() {
        let analyzer = RuleBasedLayoutAnalyzer::new();
        let blocks: Vec<TextBlock> = vec![];

        let layout = analyzer.analyze(&blocks, 600.0, 800.0);

        assert_eq!(layout.columns.len(), 0);
        assert_eq!(layout.reading_order.len(), 0);
    }
}
