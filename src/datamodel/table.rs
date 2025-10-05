//! Table types

use serde::{Deserialize, Serialize};

/// Table structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Table {
    // Placeholder - will be implemented in T016
}

/// Table data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableData {
    rows: Vec<TableRow>,
}

impl TableData {
    /// Create a new empty table
    pub fn new() -> Self {
        Self { rows: Vec::new() }
    }

    /// Get the rows
    pub fn rows(&self) -> &[TableRow] {
        &self.rows
    }

    /// Add a row
    pub fn with_row(mut self, row: TableRow) -> Self {
        self.rows.push(row);
        self
    }

    /// Get the number of columns (from first row)
    pub fn num_cols(&self) -> usize {
        self.rows.first().map(|r| r.cells.len()).unwrap_or(0)
    }
}

impl Default for TableData {
    fn default() -> Self {
        Self::new()
    }
}

/// Table cell
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableCell {
    content: String,
    col_span: usize,
    row_span: usize,
}

impl TableCell {
    /// Create a new table cell
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            col_span: 1,
            row_span: 1,
        }
    }

    /// Get the cell content
    pub fn content(&self) -> &str {
        &self.content
    }

    /// Get the column span
    pub fn col_span(&self) -> usize {
        self.col_span
    }

    /// Get the row span
    pub fn row_span(&self) -> usize {
        self.row_span
    }

    /// Set the column span
    pub fn with_col_span(mut self, span: usize) -> Self {
        self.col_span = span;
        self
    }

    /// Set the row span
    pub fn with_row_span(mut self, span: usize) -> Self {
        self.row_span = span;
        self
    }
}

/// Table row
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableRow {
    cells: Vec<TableCell>,
}

impl TableRow {
    /// Create a new table row
    pub fn new(cells: Vec<TableCell>) -> Self {
        Self { cells }
    }

    /// Get the cells
    pub fn cells(&self) -> &[TableCell] {
        &self.cells
    }
}

/// Table metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableMetadata {
    // Placeholder - will be implemented in T016
}
