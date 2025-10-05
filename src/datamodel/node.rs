//! Document node types

use serde::{Deserialize, Serialize};

/// Document node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentNode {
    item: NodeItem,
}

impl DocumentNode {
    /// Create a new document node
    pub fn new(node_type: NodeType, text: impl Into<String>) -> Self {
        Self {
            item: NodeItem::new(node_type, text),
        }
    }

    /// Get the text content
    pub fn text_content(&self) -> Option<&str> {
        self.item.text_content()
    }

    /// Get the node type
    pub fn node_type(&self) -> NodeType {
        self.item.node_type()
    }

    /// Get the source position
    pub fn position(&self) -> Option<&SourcePosition> {
        self.item.position()
    }

    /// Set the source position
    pub fn with_position(mut self, position: SourcePosition) -> Self {
        self.item = self.item.with_position(position);
        self
    }
}

/// Node item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeItem {
    node_type: NodeType,
    text_content: Option<String>,
    position: Option<SourcePosition>,
}

impl NodeItem {
    /// Create a new node item
    pub fn new(node_type: NodeType, text: impl Into<String>) -> Self {
        Self {
            node_type,
            text_content: Some(text.into()),
            position: None,
        }
    }

    /// Get the node type
    pub fn node_type(&self) -> NodeType {
        self.node_type
    }

    /// Get the text content
    pub fn text_content(&self) -> Option<&str> {
        self.text_content.as_deref()
    }

    /// Get the source position
    pub fn position(&self) -> Option<&SourcePosition> {
        self.position.as_ref()
    }

    /// Set the source position
    pub fn with_position(mut self, position: SourcePosition) -> Self {
        self.position = Some(position);
        self
    }
}

/// Node metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeMetadata {
    // Placeholder - will be implemented in T014
}

/// Source position
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SourcePosition {
    start_offset: usize,
    end_offset: usize,
    start_line: usize,
    end_line: usize,
}

impl SourcePosition {
    /// Create a new source position
    pub fn new(start_offset: usize, end_offset: usize, start_line: usize, end_line: usize) -> Self {
        Self {
            start_offset,
            end_offset,
            start_line,
            end_line,
        }
    }

    /// Get start offset
    pub fn start_offset(&self) -> usize {
        self.start_offset
    }

    /// Get end offset
    pub fn end_offset(&self) -> usize {
        self.end_offset
    }

    /// Get start line
    pub fn start_line(&self) -> usize {
        self.start_line
    }

    /// Get end line
    pub fn end_line(&self) -> usize {
        self.end_line
    }
}

/// Node type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NodeType {
    Text,
    Heading,
    Paragraph,
    List,
    ListItem,
    Table,
    TableRow,
    TableCell,
}
