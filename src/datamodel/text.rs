//! Text content types

use serde::{Deserialize, Serialize};

/// Text content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextContent {
    // Placeholder - will be implemented in T015
}

/// Text item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextItem {
    content: String,
    formatting: Vec<Formatting>,
}

impl TextItem {
    /// Create a new text item
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            formatting: Vec::new(),
        }
    }

    /// Get the text content
    pub fn content(&self) -> &str {
        &self.content
    }

    /// Get the formatting
    pub fn formatting(&self) -> &[Formatting] {
        &self.formatting
    }

    /// Add formatting
    pub fn with_formatting(mut self, formatting: Formatting) -> Self {
        self.formatting.push(formatting);
        self
    }
}

/// Text formatting
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Formatting {
    Bold,
    Italic,
    Code,
}

impl Formatting {
    /// Create bold formatting
    pub fn bold() -> Self {
        Formatting::Bold
    }

    /// Create italic formatting
    pub fn italic() -> Self {
        Formatting::Italic
    }

    /// Create code formatting
    pub fn code() -> Self {
        Formatting::Code
    }

    /// Check if bold
    pub fn is_bold(&self) -> bool {
        matches!(self, Formatting::Bold)
    }

    /// Check if italic
    pub fn is_italic(&self) -> bool {
        matches!(self, Formatting::Italic)
    }

    /// Check if code
    pub fn is_code(&self) -> bool {
        matches!(self, Formatting::Code)
    }
}

/// Text metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextMetadata {
    // Placeholder - will be implemented in T015
}
