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

    /// Get the maximum number of columns, including ragged rows
    pub fn num_cols(&self) -> usize {
        self.rows.iter().map(|r| r.cells.len()).max().unwrap_or(0)
    }

    /// Serialize every nonempty row for RAG chunking. The first row is retained;
    /// subsequent rows use its nonempty cells as column labels.
    /// Returns a vector of row strings suitable for individual chunks
    pub fn rows_as_chunks(&self) -> Vec<String> {
        let headers = self.rows.first().map(|r| r.cells.as_slice()).unwrap_or(&[]);
        self.rows
            .iter()
            .enumerate()
            .filter_map(|(row_index, row)| {
                let values: Vec<String> = row
                    .cells
                    .iter()
                    .enumerate()
                    .filter_map(|(i, cell)| {
                        if cell.content.is_empty() {
                            return None;
                        }
                        let header = headers.get(i).map(|c| c.content.as_str()).unwrap_or("");
                        Some(if row_index == 0 || header.is_empty() {
                            cell.content.clone()
                        } else {
                            format!("{} = {}", header, cell.content)
                        })
                    })
                    .collect();
                if values.is_empty() {
                    None
                } else {
                    Some(values.join(". ") + ".")
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
                let content = content
                    .replace('|', "\\|")
                    .replace("\r\n", "<br>")
                    .replace(['\n', '\r'], "<br>");
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
