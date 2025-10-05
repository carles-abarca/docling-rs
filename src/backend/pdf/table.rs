//! Table detection and extraction types.

use super::types::BoundingBox;
use serde::{Deserialize, Serialize};

/// A table detected in a PDF page.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Table {
    /// Bounding box of the entire table
    pub bbox: BoundingBox,

    /// Table cells
    pub cells: Vec<TableCell>,

    /// Table structure information
    pub structure: TableStructure,

    /// Number of header rows
    pub header_rows: usize,

    /// Confidence score (0.0-1.0) if available
    pub confidence: Option<f32>,
}

impl Table {
    /// Create a new table.
    pub fn new(bbox: BoundingBox, structure: TableStructure) -> Self {
        Self {
            bbox,
            cells: Vec::new(),
            structure,
            header_rows: 0,
            confidence: None,
        }
    }

    /// Add a cell to the table.
    pub fn add_cell(&mut self, cell: TableCell) {
        self.cells.push(cell);
    }

    /// Set the number of header rows.
    pub fn with_header_rows(mut self, header_rows: usize) -> Self {
        self.header_rows = header_rows;
        self
    }

    /// Set the confidence score.
    pub fn with_confidence(mut self, confidence: f32) -> Self {
        self.confidence = Some(confidence);
        self
    }

    /// Get the total number of cells.
    pub fn cell_count(&self) -> usize {
        self.cells.len()
    }

    /// Get a cell by row and column.
    pub fn get_cell(&self, row: usize, col: usize) -> Option<&TableCell> {
        self.cells.iter().find(|c| c.row == row && c.col == col)
    }
}

/// A cell in a table.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableCell {
    /// Row index (0-based)
    pub row: usize,

    /// Column index (0-based)
    pub col: usize,

    /// Number of rows this cell spans (default 1)
    pub rowspan: usize,

    /// Number of columns this cell spans (default 1)
    pub colspan: usize,

    /// Cell content (text)
    pub content: String,

    /// Bounding box of the cell
    pub bbox: BoundingBox,

    /// Whether this is a header cell
    pub is_header: bool,
}

impl TableCell {
    /// Create a new table cell.
    pub fn new(row: usize, col: usize, content: String, bbox: BoundingBox) -> Self {
        Self {
            row,
            col,
            rowspan: 1,
            colspan: 1,
            content,
            bbox,
            is_header: false,
        }
    }

    /// Set the cell as a header cell.
    pub fn with_header(mut self, is_header: bool) -> Self {
        self.is_header = is_header;
        self
    }

    /// Set the rowspan.
    pub fn with_rowspan(mut self, rowspan: usize) -> Self {
        self.rowspan = rowspan;
        self
    }

    /// Set the colspan.
    pub fn with_colspan(mut self, colspan: usize) -> Self {
        self.colspan = colspan;
        self
    }

    /// Check if this cell spans multiple rows.
    pub fn is_multirow(&self) -> bool {
        self.rowspan > 1
    }

    /// Check if this cell spans multiple columns.
    pub fn is_multicolumn(&self) -> bool {
        self.colspan > 1
    }

    /// Check if this cell is merged (spans multiple rows or columns).
    pub fn is_merged(&self) -> bool {
        self.is_multirow() || self.is_multicolumn()
    }
}

/// Table structure information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableStructure {
    /// Number of rows
    pub rows: usize,

    /// Number of columns
    pub cols: usize,

    /// Merged cells: (row, col, rowspan, colspan)
    pub merged_cells: Vec<(usize, usize, usize, usize)>,
}

impl TableStructure {
    /// Create a new table structure.
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            merged_cells: Vec::new(),
        }
    }

    /// Add a merged cell.
    pub fn add_merged_cell(&mut self, row: usize, col: usize, rowspan: usize, colspan: usize) {
        if rowspan > 1 || colspan > 1 {
            self.merged_cells.push((row, col, rowspan, colspan));
        }
    }

    /// Get the total number of logical cells.
    pub fn total_cells(&self) -> usize {
        self.rows * self.cols
    }

    /// Check if a cell position is valid.
    pub fn is_valid_position(&self, row: usize, col: usize) -> bool {
        row < self.rows && col < self.cols
    }

    /// Check if the structure has merged cells.
    pub fn has_merged_cells(&self) -> bool {
        !self.merged_cells.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_creation() {
        let bbox = BoundingBox::new(100.0, 100.0, 400.0, 200.0);
        let structure = TableStructure::new(3, 2);
        let table = Table::new(bbox, structure);

        assert_eq!(table.structure.rows, 3);
        assert_eq!(table.structure.cols, 2);
        assert_eq!(table.cell_count(), 0);
        assert_eq!(table.header_rows, 0);
    }

    #[test]
    fn test_table_add_cell() {
        let bbox = BoundingBox::new(100.0, 100.0, 400.0, 200.0);
        let structure = TableStructure::new(2, 2);
        let mut table = Table::new(bbox, structure);

        let cell = TableCell::new(
            0,
            0,
            "Test".to_string(),
            BoundingBox::new(100.0, 100.0, 200.0, 100.0),
        );
        table.add_cell(cell);

        assert_eq!(table.cell_count(), 1);
    }

    #[test]
    fn test_table_cell_header() {
        let cell = TableCell::new(
            0,
            0,
            "Header".to_string(),
            BoundingBox::new(0.0, 0.0, 100.0, 50.0),
        )
        .with_header(true);

        assert!(cell.is_header);
    }

    #[test]
    fn test_table_cell_merged() {
        let cell = TableCell::new(
            0,
            0,
            "Merged".to_string(),
            BoundingBox::new(0.0, 0.0, 200.0, 100.0),
        )
        .with_rowspan(2)
        .with_colspan(2);

        assert_eq!(cell.rowspan, 2);
        assert_eq!(cell.colspan, 2);
        assert!(cell.is_multirow());
        assert!(cell.is_multicolumn());
        assert!(cell.is_merged());
    }

    #[test]
    fn test_table_structure() {
        let mut structure = TableStructure::new(3, 3);

        assert_eq!(structure.total_cells(), 9);
        assert!(structure.is_valid_position(2, 2));
        assert!(!structure.is_valid_position(3, 3));
        assert!(!structure.has_merged_cells());

        structure.add_merged_cell(0, 0, 2, 1);
        assert!(structure.has_merged_cells());
        assert_eq!(structure.merged_cells.len(), 1);
    }

    #[test]
    fn test_get_cell() {
        let bbox = BoundingBox::new(100.0, 100.0, 400.0, 200.0);
        let structure = TableStructure::new(2, 2);
        let mut table = Table::new(bbox, structure);

        let cell1 = TableCell::new(
            0,
            0,
            "A".to_string(),
            BoundingBox::new(100.0, 100.0, 200.0, 100.0),
        );
        let cell2 = TableCell::new(
            0,
            1,
            "B".to_string(),
            BoundingBox::new(300.0, 100.0, 200.0, 100.0),
        );

        table.add_cell(cell1);
        table.add_cell(cell2);

        assert!(table.get_cell(0, 0).is_some());
        assert_eq!(table.get_cell(0, 0).unwrap().content, "A");
        assert!(table.get_cell(0, 1).is_some());
        assert_eq!(table.get_cell(0, 1).unwrap().content, "B");
        assert!(table.get_cell(1, 0).is_none());
    }
}
