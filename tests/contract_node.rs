//! Contract tests for Node types

use docling_rs::datamodel::{NodeItem, NodeType, SourcePosition};

#[test]
fn test_nodeitem_new() {
    let node = NodeItem::new(NodeType::Paragraph, "content");

    assert_eq!(node.node_type(), NodeType::Paragraph);
    assert_eq!(node.text_content(), Some("content"));
}

#[test]
fn test_nodeitem_with_position() {
    let pos = SourcePosition::new(0, 10, 1, 1);
    let node = NodeItem::new(NodeType::Paragraph, "text").with_position(pos.clone());

    assert_eq!(node.position(), Some(&pos));
}

#[test]
fn test_nodeitem_serialization() {
    let node = NodeItem::new(NodeType::Heading, "Title");
    let json = serde_json::to_string(&node).expect("Should serialize");
    let _deserialized: NodeItem = serde_json::from_str(&json).expect("Should deserialize");
}

#[test]
fn test_source_position() {
    let pos = SourcePosition::new(0, 10, 1, 5);

    assert_eq!(pos.start_offset(), 0);
    assert_eq!(pos.end_offset(), 10);
    assert_eq!(pos.start_line(), 1);
    assert_eq!(pos.end_line(), 5);
}
