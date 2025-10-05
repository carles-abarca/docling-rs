//! Pipeline trait definitions

use crate::datamodel::{ConversionResult, InputDocument};
use crate::error::ConversionError;

/// Pipeline trait
pub trait Pipeline {
    /// Execute the pipeline on an input document
    fn execute(&self, input: &InputDocument) -> Result<ConversionResult, ConversionError>;
}
