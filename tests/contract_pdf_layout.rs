//! Contract test: PDF Layout Analysis
//!
//! This test verifies that the layout analysis system correctly implements
//! its contract for detecting columns, reading order, and text block organization.

use docling_rs::backend::pdf::{
    layout_analyzer::{LayoutAnalyzer, RuleBasedLayoutAnalyzer},
    page::TextBlock,
    types::BoundingBox,
};

#[test]
fn test_layout_analyzer_trait_implemented() {
    // Arrange: Create a layout analyzer
    let analyzer = RuleBasedLayoutAnalyzer::new();

    // Assert: LayoutAnalyzer trait is implemented (compile-time check)
    let _: &dyn LayoutAnalyzer = &analyzer;
}

#[test]
fn test_single_column_layout() {
    // Arrange: Create text blocks in a single column
    let analyzer = RuleBasedLayoutAnalyzer::new();

    let blocks = vec![
        create_text_block("First paragraph", 100.0, 100.0, 400.0, 50.0),
        create_text_block("Second paragraph", 100.0, 200.0, 400.0, 50.0),
        create_text_block("Third paragraph", 100.0, 300.0, 400.0, 50.0),
    ];

    // Act: Analyze layout
    let layout = analyzer.analyze(&blocks, 600.0, 800.0);

    // Assert: Single column detected
    assert_eq!(layout.columns.len(), 1, "Should detect exactly one column");
    assert_eq!(
        layout.reading_order.len(),
        3,
        "Reading order should include all blocks"
    );

    // Verify reading order is top-to-bottom
    assert_eq!(layout.reading_order[0], 0);
    assert_eq!(layout.reading_order[1], 1);
    assert_eq!(layout.reading_order[2], 2);
}

#[test]
fn test_two_column_layout() {
    // Arrange: Create text blocks in two columns
    let analyzer = RuleBasedLayoutAnalyzer::new();

    let blocks = vec![
        // Left column
        create_text_block("Left top", 50.0, 100.0, 200.0, 50.0),
        create_text_block("Left middle", 50.0, 200.0, 200.0, 50.0),
        create_text_block("Left bottom", 50.0, 300.0, 200.0, 50.0),
        // Right column
        create_text_block("Right top", 350.0, 100.0, 200.0, 50.0),
        create_text_block("Right middle", 350.0, 200.0, 200.0, 50.0),
        create_text_block("Right bottom", 350.0, 300.0, 200.0, 50.0),
    ];

    // Act: Analyze layout
    let layout = analyzer.analyze(&blocks, 600.0, 800.0);

    // Assert: Two columns detected
    assert_eq!(layout.columns.len(), 2, "Should detect exactly two columns");

    // Verify reading order (left-to-right, top-to-bottom within columns)
    assert_eq!(layout.reading_order.len(), 6);

    // First three should be left column (indices 0, 1, 2)
    // Next three should be right column (indices 3, 4, 5)
    let left_indices: Vec<usize> = layout.reading_order[0..3].to_vec();
    let right_indices: Vec<usize> = layout.reading_order[3..6].to_vec();

    assert!(left_indices.contains(&0));
    assert!(left_indices.contains(&1));
    assert!(left_indices.contains(&2));

    assert!(right_indices.contains(&3));
    assert!(right_indices.contains(&4));
    assert!(right_indices.contains(&5));
}

#[test]
fn test_column_bounds() {
    // Arrange: Create text blocks with known positions
    let analyzer = RuleBasedLayoutAnalyzer::new();

    let blocks = vec![
        create_text_block("Left", 50.0, 100.0, 200.0, 300.0),
        create_text_block("Right", 350.0, 100.0, 200.0, 300.0),
    ];

    // Act: Analyze layout
    let layout = analyzer.analyze(&blocks, 600.0, 800.0);

    // Assert: Column bounds are calculated correctly
    assert_eq!(layout.columns.len(), 2);

    let left_col = &layout.columns[0];
    let right_col = &layout.columns[1];

    assert!(
        left_col.bbox.x < 300.0,
        "Left column should be on left side"
    );
    assert!(
        right_col.bbox.x > 300.0,
        "Right column should be on right side"
    );
}

#[test]
fn test_empty_input() {
    // Arrange: Empty text blocks
    let analyzer = RuleBasedLayoutAnalyzer::new();
    let blocks: Vec<TextBlock> = vec![];

    // Act: Analyze layout
    let layout = analyzer.analyze(&blocks, 600.0, 800.0);

    // Assert: Should handle gracefully
    assert_eq!(
        layout.columns.len(),
        0,
        "Should have no columns for empty input"
    );
    assert_eq!(
        layout.reading_order.len(),
        0,
        "Should have empty reading order"
    );
}

// Helper function
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
        reading_order: 0, // Will be set by analyzer
        column_id: None,
        block_type: TextBlockType::Paragraph,
        confidence: None,
    }
}
