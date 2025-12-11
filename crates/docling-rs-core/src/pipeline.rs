//! Pipeline trait and implementations for document processing

use crate::backend::Backend;
use crate::datamodel::{ConversionResult, ConversionStatus, InputDocument};
use crate::error::ConversionError;

/// Pipeline trait for document processing
pub trait Pipeline: Send + Sync {
    /// Execute the pipeline on an input document
    fn execute(&self, input: &InputDocument) -> Result<ConversionResult, ConversionError>;
}

/// Simple pipeline that uses registered backends
pub struct SimplePipeline {
    backends: Vec<Box<dyn Backend>>,
}

impl SimplePipeline {
    /// Create a new empty SimplePipeline
    pub fn new() -> Self {
        Self {
            backends: Vec::new(),
        }
    }

    /// Register a backend with the pipeline
    pub fn register_backend(&mut self, backend: Box<dyn Backend>) {
        self.backends.push(backend);
    }

    /// Register a backend with builder pattern
    pub fn with_backend(mut self, backend: Box<dyn Backend>) -> Self {
        self.backends.push(backend);
        self
    }

    /// Get the number of registered backends
    pub fn backend_count(&self) -> usize {
        self.backends.len()
    }
}

impl Default for SimplePipeline {
    fn default() -> Self {
        Self::new()
    }
}

impl Pipeline for SimplePipeline {
    fn execute(&self, input: &InputDocument) -> Result<ConversionResult, ConversionError> {
        let format = input.format();

        // Find a backend that supports this format
        for backend in &self.backends {
            if backend.supports_format(format) {
                let document = backend.convert(input)?;
                return Ok(ConversionResult::new(document, ConversionStatus::Success));
            }
        }

        Err(ConversionError::UnsupportedFormat(format!(
            "No backend registered for format: {:?}",
            format
        )))
    }
}
