//! Markdown backend implementation

use crate::backend::Backend;
use crate::datamodel::{DoclingDocument, DocumentNode, InputDocument, NodeType};
use crate::error::ConversionError;
use crate::InputFormat;

/// Markdown backend
pub struct MarkdownBackend {}

impl MarkdownBackend {
    /// Create a new Markdown backend
    pub fn new() -> Self {
        Self {}
    }

    fn get_content(input: &InputDocument) -> Result<String, ConversionError> {
        match input.source() {
            crate::datamodel::DocumentSource::FilePath(path) => {
                std::fs::read_to_string(path).map_err(ConversionError::Io)
            }
            crate::datamodel::DocumentSource::Bytes { data, .. } => String::from_utf8(data.clone())
                .map_err(|e| ConversionError::InvalidFile(format!("Invalid UTF-8: {}", e))),
        }
    }
}

impl Default for MarkdownBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl Backend for MarkdownBackend {
    fn convert(&self, input: &InputDocument) -> Result<DoclingDocument, ConversionError> {
        let content = Self::get_content(input)?;

        // Get document name from input
        let name = match input.source() {
            crate::datamodel::DocumentSource::FilePath(path) => path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown")
                .to_string(),
            crate::datamodel::DocumentSource::Bytes { name, .. } => name.clone(),
        };

        // For now, create simple nodes from the content
        // Full parsing logic will be enhanced later
        let mut doc = DoclingDocument::new(name);

        // Create basic nodes from paragraphs
        // This is a minimal implementation to support chunking
        for line in content.lines() {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                // Determine node type based on markdown syntax
                let node_type = if trimmed.starts_with('#') {
                    NodeType::Heading
                } else if trimmed.starts_with('-') || trimmed.starts_with('*') || trimmed.starts_with('+') {
                    NodeType::ListItem
                } else {
                    NodeType::Paragraph
                };

                doc.add_node(DocumentNode::new(node_type, trimmed));
            }
        }

        Ok(doc)
    }

    fn supports_format(&self, format: InputFormat) -> bool {
        format == InputFormat::Markdown
    }
}
