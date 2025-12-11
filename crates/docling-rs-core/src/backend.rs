//! Backend trait definitions

use crate::datamodel::{DoclingDocument, InputDocument};
use crate::error::ConversionError;
use crate::format::InputFormat;

/// Backend trait for document conversion
pub trait Backend: Send + Sync {
    /// Convert an input document to a DoclingDocument
    fn convert(&self, input: &InputDocument) -> Result<DoclingDocument, ConversionError>;

    /// Check if this backend supports the given format
    fn supports_format(&self, format: InputFormat) -> bool;
}

/// Declarative backend trait (for future use)
pub trait DeclarativeBackend: Backend {
    // Placeholder - will be implemented in future
}
