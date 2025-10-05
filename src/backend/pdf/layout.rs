//! Layout analysis types and structures.

use super::types::BoundingBox;
use serde::{Deserialize, Serialize};

/// Layout information for a PDF page.
///
/// Contains the detected columns and the reading order of text blocks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutInfo {
    /// Detected columns on the page
    pub columns: Vec<Column>,

    /// Reading order of text blocks (indices into the text_blocks array)
    pub reading_order: Vec<usize>,

    /// Page dimensions
    pub page_width: f64,
    pub page_height: f64,
}

impl LayoutInfo {
    /// Create a new layout info.
    pub fn new(page_width: f64, page_height: f64) -> Self {
        Self {
            columns: Vec::new(),
            reading_order: Vec::new(),
            page_width,
            page_height,
        }
    }

    /// Add a column to the layout.
    pub fn add_column(&mut self, column: Column) {
        self.columns.push(column);
    }

    /// Set the reading order.
    pub fn set_reading_order(&mut self, order: Vec<usize>) {
        self.reading_order = order;
    }

    /// Get the number of columns.
    pub fn column_count(&self) -> usize {
        self.columns.len()
    }

    /// Check if the layout is single-column.
    pub fn is_single_column(&self) -> bool {
        self.columns.len() == 1
    }

    /// Check if the layout is multi-column.
    pub fn is_multi_column(&self) -> bool {
        self.columns.len() > 1
    }
}

/// A column in a multi-column layout.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Column {
    /// Column identifier (0-indexed from left to right)
    pub id: usize,

    /// Bounding box of the column
    pub bbox: BoundingBox,

    /// Indices of text blocks that belong to this column
    pub text_block_indices: Vec<usize>,

    /// Column type (main content, sidebar, etc.)
    pub column_type: ColumnType,
}

impl Column {
    /// Create a new column.
    pub fn new(id: usize, bbox: BoundingBox) -> Self {
        Self {
            id,
            bbox,
            text_block_indices: Vec::new(),
            column_type: ColumnType::MainContent,
        }
    }

    /// Add a text block index to this column.
    pub fn add_text_block(&mut self, index: usize) {
        self.text_block_indices.push(index);
    }

    /// Get the number of text blocks in this column.
    pub fn text_block_count(&self) -> usize {
        self.text_block_indices.len()
    }

    /// Set the column type.
    pub fn with_type(mut self, column_type: ColumnType) -> Self {
        self.column_type = column_type;
        self
    }
}

/// Type of column in the layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ColumnType {
    /// Main content column
    MainContent,

    /// Sidebar or supplementary content
    Sidebar,

    /// Header area
    Header,

    /// Footer area
    Footer,
}

impl Default for ColumnType {
    fn default() -> Self {
        Self::MainContent
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layout_info_creation() {
        let layout = LayoutInfo::new(600.0, 800.0);

        assert_eq!(layout.page_width, 600.0);
        assert_eq!(layout.page_height, 800.0);
        assert_eq!(layout.column_count(), 0);
        assert_eq!(layout.reading_order.len(), 0);
    }

    #[test]
    fn test_add_column() {
        let mut layout = LayoutInfo::new(600.0, 800.0);
        let col = Column::new(0, BoundingBox::new(0.0, 0.0, 300.0, 800.0));

        layout.add_column(col);

        assert_eq!(layout.column_count(), 1);
        assert!(layout.is_single_column());
        assert!(!layout.is_multi_column());
    }

    #[test]
    fn test_multi_column_detection() {
        let mut layout = LayoutInfo::new(600.0, 800.0);

        layout.add_column(Column::new(0, BoundingBox::new(0.0, 0.0, 280.0, 800.0)));
        layout.add_column(Column::new(1, BoundingBox::new(320.0, 0.0, 280.0, 800.0)));

        assert_eq!(layout.column_count(), 2);
        assert!(!layout.is_single_column());
        assert!(layout.is_multi_column());
    }

    #[test]
    fn test_reading_order() {
        let mut layout = LayoutInfo::new(600.0, 800.0);
        layout.set_reading_order(vec![0, 1, 3, 2, 4]);

        assert_eq!(layout.reading_order.len(), 5);
        assert_eq!(layout.reading_order[0], 0);
        assert_eq!(layout.reading_order[4], 4);
    }

    #[test]
    fn test_column_text_blocks() {
        let mut col = Column::new(0, BoundingBox::new(0.0, 0.0, 300.0, 800.0));

        col.add_text_block(0);
        col.add_text_block(1);
        col.add_text_block(2);

        assert_eq!(col.text_block_count(), 3);
        assert_eq!(col.text_block_indices[0], 0);
        assert_eq!(col.text_block_indices[2], 2);
    }

    #[test]
    fn test_column_type() {
        let col =
            Column::new(0, BoundingBox::new(0.0, 0.0, 300.0, 800.0)).with_type(ColumnType::Sidebar);

        assert_eq!(col.column_type, ColumnType::Sidebar);
    }
}
