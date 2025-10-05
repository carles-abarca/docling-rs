//! Backend trait definitions

use crate::datamodel::{DoclingDocument, InputDocument};
use crate::error::ConversionError;
use crate::InputFormat;

/// Backend trait for document conversion
pub trait Backend {
    /// Convert an input document to a DoclingDocument
    fn convert(&self, input: &InputDocument) -> Result<DoclingDocument, ConversionError>;

    /// Check if this backend supports the given format
    fn supports_format(&self, format: InputFormat) -> bool;
}

/// Declarative backend trait
pub trait DeclarativeBackend: Backend {
    // Placeholder - will be implemented in T018
}
