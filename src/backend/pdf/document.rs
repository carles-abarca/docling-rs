//! PDF document representation.

use super::page::PdfPage;
use serde::{Deserialize, Serialize};

/// Represents a loaded PDF document.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfDocument {
    /// All pages in the document.
    pub pages: Vec<PdfPage>,

    /// Document metadata.
    pub metadata: PdfMetadata,

    /// Encryption information if document is encrypted.
    pub encryption_info: Option<EncryptionInfo>,

    /// Total number of pages.
    pub page_count: usize,
}

/// PDF metadata.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PdfMetadata {
    /// Document title.
    pub title: Option<String>,

    /// Document author.
    pub author: Option<String>,

    /// Document subject.
    pub subject: Option<String>,

    /// Document keywords.
    pub keywords: Option<String>,

    /// Creator application.
    pub creator: Option<String>,

    /// Producer application.
    pub producer: Option<String>,

    /// Creation date.
    pub creation_date: Option<String>,

    /// Modification date.
    pub mod_date: Option<String>,

    /// PDF version.
    pub version: Option<String>,
}

/// Encryption information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionInfo {
    /// Whether document is encrypted.
    pub is_encrypted: bool,

    /// Encryption method used.
    pub method: Option<String>,

    /// Whether password was required.
    pub required_password: bool,
}
