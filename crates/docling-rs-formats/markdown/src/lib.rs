//! Markdown backend for docling-rs

use docling_rs_core::{
    Backend, ConversionError, DoclingDocument, DocumentNode, DocumentSource, InputDocument,
    InputFormat, NodeType,
};

/// Markdown backend
pub struct MarkdownBackend;

impl MarkdownBackend {
    /// Create a new MarkdownBackend
    pub fn new() -> Self {
        Self
    }

    fn get_content(input: &InputDocument) -> Result<String, ConversionError> {
        match input.source() {
            DocumentSource::FilePath(path) => {
                std::fs::read_to_string(path).map_err(ConversionError::Io)
            }
            DocumentSource::Bytes { data, .. } => String::from_utf8(data.clone())
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

        let name = match input.source() {
            DocumentSource::FilePath(path) => path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown")
                .to_string(),
            DocumentSource::Bytes { name, .. } => name.clone(),
        };

        let mut doc = DoclingDocument::new(name);

        for line in content.lines() {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                let node_type = if trimmed.starts_with('#') {
                    NodeType::Heading
                } else if trimmed.starts_with('-')
                    || trimmed.starts_with('*')
                    || trimmed.starts_with('+')
                {
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
