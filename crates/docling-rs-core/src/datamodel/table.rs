//! Table types

use serde::{Deserialize, Serialize};

/// Table structure (placeholder)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Table {
    // Placeholder - will be implemented in future
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

    /// Add a row (mutable reference version)
    pub fn add_row(&mut self, row: TableRow) {
        self.rows.push(row);
    }

    /// Get the number of columns (from first row)
    pub fn num_cols(&self) -> usize {
        self.rows.first().map(|r| r.cells.len()).unwrap_or(0)
    }

    /// Serialize each row for RAG chunking (key=value format, like Python docling)
    /// Returns a vector of row strings suitable for individual chunks
    pub fn rows_as_chunks(&self) -> Vec<String> {
        if self.rows.is_empty() {
            return vec![];
        }

        // Get headers from first row
        let headers: Vec<&str> = self
            .rows
            .first()
            .map(|r| r.cells.iter().map(|c| c.content.as_str()).collect())
            .unwrap_or_default();

        // Skip header row, process data rows
        self.rows
            .iter()
            .skip(1)
            .filter_map(|row| {
                // Use first cell as the row identifier (like "Name" in Python docling)
                let row_id = row.cells.first().map(|c| c.content.as_str()).unwrap_or("");
                if row_id.is_empty() {
                    return None;
                }

                // Build "RowId, Header = Value" pairs
                let pairs: Vec<String> = row
                    .cells
                    .iter()
                    .enumerate()
                    .skip(1) // Skip the first cell (already used as row_id)
                    .filter_map(|(i, cell)| {
                        let header = headers.get(i).unwrap_or(&"");
                        let value = cell.content.as_str();
                        if !value.is_empty() && !header.is_empty() {
                            Some(format!("{}, {} = {}", row_id, header, value))
                        } else {
                            None
                        }
                    })
                    .collect();

                if pairs.is_empty() {
                    None
                } else {
                    Some(pairs.join(". ") + ".")
                }
            })
            .collect()
    }

    /// Convert the table to Markdown format
    pub fn to_markdown(&self) -> String {
        if self.rows.is_empty() {
            return String::new();
        }

        // Calculate column widths
        let num_cols = self.num_cols();
        let mut col_widths: Vec<usize> = vec![0; num_cols];

        for row in &self.rows {
            for (i, cell) in row.cells.iter().enumerate() {
                if i < num_cols {
                    col_widths[i] = col_widths[i].max(cell.content.len());
                }
            }
        }

        // Ensure minimum width of 3 for separator
        for width in &mut col_widths {
            *width = (*width).max(3);
        }

        let mut output = String::new();

        for (row_idx, row) in self.rows.iter().enumerate() {
            // Build the row
            output.push('|');
            for (i, width) in col_widths.iter().enumerate() {
                let content = row.cells.get(i).map(|c| c.content.as_str()).unwrap_or("");
                output.push_str(&format!(" {:<width$} |", content, width = width));
            }
            output.push('\n');

            // Add separator after first row (header)
            if row_idx == 0 {
                output.push('|');
                for width in &col_widths {
                    output.push_str(&format!("{:-<width$}|", "", width = width + 2));
                }
                output.push('\n');
            }
        }

        output
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
    pub(crate) cells: Vec<TableCell>,
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

/// Table metadata (placeholder)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableMetadata {
    // Placeholder - will be implemented in future
}
