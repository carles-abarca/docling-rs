//! Table detection for PDF pages.

use super::page::TextBlock;
use super::table::{Table, TableCell, TableStructure};
use super::types::BoundingBox;

/// Trait for table detection implementations.
pub trait TableDetector {
    /// Detect tables in a collection of text blocks.
    ///
    /// # Arguments
    ///
    /// * `text_blocks` - The text blocks to analyze
    /// * `page_width` - Width of the page
    /// * `page_height` - Height of the page
    ///
    /// # Returns
    ///
    /// Vector of detected tables
    fn detect_tables(
        &self,
        text_blocks: &[TextBlock],
        page_width: f64,
        page_height: f64,
    ) -> Vec<Table>;
}

/// Grid-based table detector.
///
/// Detects tables by analyzing alignment of text blocks:
/// - Looks for horizontally and vertically aligned blocks
/// - Identifies grid patterns
/// - Detects cell boundaries
pub struct GridBasedTableDetector {
    /// Minimum number of rows to consider it a table
    min_rows: usize,

    /// Minimum number of columns to consider it a table
    min_cols: usize,

    /// Alignment tolerance (in points)
    alignment_tolerance: f64,

    /// Minimum confidence score to return a table
    min_confidence: f32,
}

impl GridBasedTableDetector {
    /// Create a new grid-based table detector with default settings.
    pub fn new() -> Self {
        Self {
            min_rows: 2,
            min_cols: 2,
            alignment_tolerance: 5.0,
            min_confidence: 0.5,
        }
    }

    /// Create a detector with custom settings.
    pub fn with_settings(
        min_rows: usize,
        min_cols: usize,
        alignment_tolerance: f64,
        min_confidence: f32,
    ) -> Self {
        Self {
            min_rows,
            min_cols,
            alignment_tolerance,
            min_confidence,
        }
    }

    /// Find groups of aligned blocks that might form a table.
    fn find_grid_groups(&self, blocks: &[TextBlock]) -> Vec<Vec<usize>> {
        if blocks.len() < self.min_rows * self.min_cols {
            return Vec::new();
        }

        // Group blocks by vertical alignment (rows)
        let mut row_groups: Vec<Vec<usize>> = Vec::new();

        for (idx, block) in blocks.iter().enumerate() {
            let block_y = block.bbox.y;

            // Find existing row group that this block aligns with
            let mut found_group = false;
            for group in &mut row_groups {
                if let Some(&first_idx) = group.first() {
                    let first_y = blocks[first_idx].bbox.y;
                    if (block_y - first_y).abs() < self.alignment_tolerance {
                        group.push(idx);
                        found_group = true;
                        break;
                    }
                }
            }

            if !found_group {
                row_groups.push(vec![idx]);
            }
        }

        // Sort row groups by y-position
        row_groups.sort_by(|a, b| {
            let y_a = blocks[a[0]].bbox.y;
            let y_b = blocks[b[0]].bbox.y;
            y_a.partial_cmp(&y_b).unwrap_or(std::cmp::Ordering::Equal)
        });

        // Sort blocks within each row by x-position
        for group in &mut row_groups {
            group.sort_by(|&a, &b| {
                blocks[a]
                    .bbox
                    .x
                    .partial_cmp(&blocks[b].bbox.x)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
        }

        row_groups
    }

    /// Check if row groups form a consistent grid (table).
    fn is_table_grid(&self, row_groups: &[Vec<usize>], _blocks: &[TextBlock]) -> bool {
        if row_groups.len() < self.min_rows {
            return false;
        }

        // Check if rows have consistent column count
        let col_counts: Vec<usize> = row_groups.iter().map(|row| row.len()).collect();
        let first_col_count = col_counts[0];

        if first_col_count < self.min_cols {
            return false;
        }

        // Allow some variation in column count (merged cells)
        let max_variation = 2;
        col_counts
            .iter()
            .all(|&count| (count as i32 - first_col_count as i32).abs() <= max_variation)
    }

    /// Build a table from aligned row groups.
    fn build_table(&self, row_groups: &[Vec<usize>], blocks: &[TextBlock]) -> Option<Table> {
        if !self.is_table_grid(row_groups, blocks) {
            return None;
        }

        // Determine table dimensions
        let rows = row_groups.len();
        let cols = row_groups.iter().map(|row| row.len()).max().unwrap_or(0);

        // Calculate table bounding box
        let all_indices: Vec<usize> = row_groups
            .iter()
            .flat_map(|row| row.iter())
            .copied()
            .collect();

        let min_x = all_indices
            .iter()
            .map(|&idx| blocks[idx].bbox.x)
            .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(0.0);

        let max_x = all_indices
            .iter()
            .map(|&idx| blocks[idx].bbox.x + blocks[idx].bbox.width)
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(0.0);

        let min_y = all_indices
            .iter()
            .map(|&idx| blocks[idx].bbox.y)
            .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(0.0);

        let max_y = all_indices
            .iter()
            .map(|&idx| blocks[idx].bbox.y + blocks[idx].bbox.height)
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(0.0);

        let table_bbox = BoundingBox::new(min_x, min_y, max_x - min_x, max_y - min_y);
        let structure = TableStructure::new(rows, cols);
        let mut table = Table::new(table_bbox, structure).with_header_rows(1); // Assume first row is header

        // Create cells
        for (row_idx, row_group) in row_groups.iter().enumerate() {
            for (col_idx, &block_idx) in row_group.iter().enumerate() {
                let block = &blocks[block_idx];
                let cell = TableCell::new(row_idx, col_idx, block.text.clone(), block.bbox.clone())
                    .with_header(row_idx == 0);

                table.add_cell(cell);
            }
        }

        Some(table)
    }
}

impl Default for GridBasedTableDetector {
    fn default() -> Self {
        Self::new()
    }
}

impl TableDetector for GridBasedTableDetector {
    fn detect_tables(
        &self,
        text_blocks: &[TextBlock],
        _page_width: f64,
        _page_height: f64,
    ) -> Vec<Table> {
        if text_blocks.is_empty() {
            return Vec::new();
        }

        let mut tables = Vec::new();

        // Find grid-like groups
        let row_groups = self.find_grid_groups(text_blocks);

        // Try to build a table from the groups
        if let Some(table) = self.build_table(&row_groups, text_blocks) {
            tables.push(table);
        }

        tables
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
    fn test_simple_table_detection() {
        let detector = GridBasedTableDetector::new();

        // Create a 2x2 grid
        let blocks = vec![
            create_test_block("A", 100.0, 100.0, 100.0, 40.0),
            create_test_block("B", 210.0, 100.0, 100.0, 40.0),
            create_test_block("C", 100.0, 150.0, 100.0, 40.0),
            create_test_block("D", 210.0, 150.0, 100.0, 40.0),
        ];

        let tables = detector.detect_tables(&blocks, 600.0, 800.0);

        assert_eq!(tables.len(), 1);
        assert_eq!(tables[0].structure.rows, 2);
        assert_eq!(tables[0].structure.cols, 2);
    }

    #[test]
    fn test_no_table_in_random_blocks() {
        let detector = GridBasedTableDetector::new();

        // Random non-aligned blocks
        let blocks = vec![
            create_test_block("A", 100.0, 100.0, 200.0, 20.0),
            create_test_block("B", 150.0, 200.0, 150.0, 20.0),
            create_test_block("C", 300.0, 350.0, 100.0, 20.0),
        ];

        let tables = detector.detect_tables(&blocks, 600.0, 800.0);

        assert_eq!(tables.len(), 0);
    }

    #[test]
    fn test_empty_input() {
        let detector = GridBasedTableDetector::new();
        let blocks: Vec<TextBlock> = vec![];

        let tables = detector.detect_tables(&blocks, 600.0, 800.0);

        assert_eq!(tables.len(), 0);
    }
}
