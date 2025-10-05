//! Contract tests for Table types

use docling_rs::datamodel::{TableCell, TableData, TableRow};

#[test]
fn test_tabledata_new() {
    let table = TableData::new();

    assert_eq!(table.rows().len(), 0);
    assert_eq!(table.num_cols(), 0);
}

#[test]
fn test_tabledata_with_rows() {
    let row1 = TableRow::new(vec![TableCell::new("A1"), TableCell::new("B1")]);
    let row2 = TableRow::new(vec![TableCell::new("A2"), TableCell::new("B2")]);

    let table = TableData::new().with_row(row1).with_row(row2);

    assert_eq!(table.rows().len(), 2);
    assert_eq!(table.num_cols(), 2);
}

#[test]
fn test_tablecell_new() {
    let cell = TableCell::new("Content");

    assert_eq!(cell.content(), "Content");
    assert_eq!(cell.col_span(), 1);
    assert_eq!(cell.row_span(), 1);
}

#[test]
fn test_tablecell_with_span() {
    let cell = TableCell::new("Merged").with_col_span(2).with_row_span(3);

    assert_eq!(cell.col_span(), 2);
    assert_eq!(cell.row_span(), 3);
}

#[test]
fn test_table_serialization() {
    let table = TableData::new().with_row(TableRow::new(vec![TableCell::new("Test")]));
    let json = serde_json::to_string(&table).expect("Should serialize");
    let _deserialized: TableData = serde_json::from_str(&json).expect("Should deserialize");
}
