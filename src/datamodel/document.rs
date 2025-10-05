//! Document representation

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::datamodel::DocumentNode;

/// Main document representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DoclingDocument {
    name: String,
    nodes: Vec<DocumentNode>,
    metadata: HashMap<String, Value>,
}

impl DoclingDocument {
    /// Create a new document with the given name
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            nodes: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Get the document name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the document nodes
    pub fn nodes(&self) -> &[DocumentNode] {
        &self.nodes
    }

    /// Get the document metadata
    pub fn metadata(&self) -> &HashMap<String, Value> {
        &self.metadata
    }

    /// Add metadata to the document
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Add a node to the document
    pub fn add_node(&mut self, node: DocumentNode) {
        self.nodes.push(node);
    }

    /// Add multiple nodes to the document
    pub fn with_nodes(mut self, nodes: Vec<DocumentNode>) -> Self {
        self.nodes = nodes;
        self
    }
}
