//! Contract test: PDF Table Detection
//!
//! This test verifies that the table detection system correctly implements
//! its contract for detecting tables, cells, and structure.

use docling_rs::backend::pdf::{
    page::TextBlock, table::TableStructure, table_detector::TableDetector, types::BoundingBox,
};

#[test]
fn test_table_detector_trait_implemented() {
    // Arrange: Create a table detector
    let detector = create_test_detector();

    // Assert: TableDetector trait is implemented (compile-time check)
    let _: &dyn TableDetector = &detector;
}

#[test]
fn test_simple_grid_table_detection() {
    // Arrange: Create text blocks in a grid pattern (3x3 table)
    let detector = create_test_detector();

    let blocks = vec![
        // Header row
        create_text_block("Name", 100.0, 100.0, 150.0, 30.0),
        create_text_block("Age", 260.0, 100.0, 100.0, 30.0),
        create_text_block("City", 370.0, 100.0, 150.0, 30.0),
        // Data row 1
        create_text_block("Alice", 100.0, 140.0, 150.0, 30.0),
        create_text_block("25", 260.0, 140.0, 100.0, 30.0),
        create_text_block("NYC", 370.0, 140.0, 150.0, 30.0),
        // Data row 2
        create_text_block("Bob", 100.0, 180.0, 150.0, 30.0),
        create_text_block("30", 260.0, 180.0, 100.0, 30.0),
        create_text_block("LA", 370.0, 180.0, 150.0, 30.0),
    ];

    // Act: Detect tables
    let tables = detector.detect_tables(&blocks, 600.0, 800.0);

    // Assert: One table detected with correct structure
    assert_eq!(tables.len(), 1, "Should detect exactly one table");

    let table = &tables[0];
    assert_eq!(table.structure.rows, 3, "Table should have 3 rows");
    assert_eq!(table.structure.cols, 3, "Table should have 3 columns");
    assert_eq!(table.cells.len(), 9, "Table should have 9 cells");
}

#[test]
fn test_table_with_header_detection() {
    // Arrange: Create a table with header row
    let detector = create_test_detector();

    let blocks = vec![
        // Header row (different formatting could be detected)
        create_text_block("Column 1", 100.0, 100.0, 200.0, 30.0),
        create_text_block("Column 2", 310.0, 100.0, 200.0, 30.0),
        // Data rows
        create_text_block("Data 1", 100.0, 140.0, 200.0, 30.0),
        create_text_block("Data 2", 310.0, 140.0, 200.0, 30.0),
    ];

    // Act: Detect tables
    let tables = detector.detect_tables(&blocks, 600.0, 800.0);

    // Assert: Table has header row
    assert_eq!(tables.len(), 1);
    let table = &tables[0];
    assert!(
        table.header_rows > 0,
        "Table should have at least one header row"
    );
}

#[test]
fn test_table_cell_bounds() {
    // Arrange: Create a simple 2x2 table
    let detector = create_test_detector();

    let blocks = vec![
        create_text_block("A", 100.0, 100.0, 100.0, 40.0),
        create_text_block("B", 210.0, 100.0, 100.0, 40.0),
        create_text_block("C", 100.0, 150.0, 100.0, 40.0),
        create_text_block("D", 210.0, 150.0, 100.0, 40.0),
    ];

    // Act
    let tables = detector.detect_tables(&blocks, 600.0, 800.0);

    // Assert: Cells have correct positions
    assert_eq!(tables.len(), 1);
    let table = &tables[0];

    // Verify cells are in expected rows and columns
    for cell in &table.cells {
        assert!(cell.row < 2, "Cell row should be < 2");
        assert!(cell.col < 2, "Cell col should be < 2");
        assert_eq!(cell.rowspan, 1, "Basic cells should have rowspan=1");
        assert_eq!(cell.colspan, 1, "Basic cells should have colspan=1");
    }
}

#[test]
fn test_table_structure_validation() {
    // Test that table structure is valid

    // Arrange
    let structure = TableStructure {
        rows: 3,
        cols: 2,
        merged_cells: vec![],
    };

    // Assert
    assert_eq!(structure.rows, 3);
    assert_eq!(structure.cols, 2);
    assert_eq!(structure.merged_cells.len(), 0);
}

#[test]
fn test_no_table_in_random_text() {
    // Arrange: Random text blocks that don't form a table
    let detector = create_test_detector();

    let blocks = vec![
        create_text_block("Some text", 100.0, 100.0, 300.0, 20.0),
        create_text_block("More text", 150.0, 200.0, 250.0, 20.0),
        create_text_block("Random", 200.0, 350.0, 100.0, 20.0),
    ];

    // Act
    let tables = detector.detect_tables(&blocks, 600.0, 800.0);

    // Assert: No tables detected
    assert_eq!(tables.len(), 0, "Should not detect tables in random text");
}

#[test]
fn test_empty_input() {
    // Arrange
    let detector = create_test_detector();
    let blocks: Vec<TextBlock> = vec![];

    // Act
    let tables = detector.detect_tables(&blocks, 600.0, 800.0);

    // Assert
    assert_eq!(tables.len(), 0, "Should handle empty input gracefully");
}

// Helper functions

fn create_test_detector() -> impl TableDetector {
    // This will be replaced with actual GridBasedTableDetector
    use docling_rs::backend::pdf::table_detector::GridBasedTableDetector;
    GridBasedTableDetector::new()
}

fn create_text_block(text: &str, x: f64, y: f64, width: f64, height: f64) -> TextBlock {
    use docling_rs::backend::pdf::page::TextBlockType;
    use docling_rs::backend::pdf::types::FontInfo;

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
