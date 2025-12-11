//! Conversion result types

use crate::datamodel::DoclingDocument;
use serde::{Deserialize, Serialize};

/// Conversion result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionResult {
    document: DoclingDocument,
    status: ConversionStatus,
    metrics: ConversionMetrics,
}

impl ConversionResult {
    /// Create a new conversion result
    pub fn new(document: DoclingDocument, status: ConversionStatus) -> Self {
        Self {
            document,
            status,
            metrics: ConversionMetrics::default(),
        }
    }

    /// Get the converted document
    pub fn document(&self) -> &DoclingDocument {
        &self.document
    }

    /// Get the conversion status
    pub fn status(&self) -> ConversionStatus {
        self.status
    }

    /// Get the conversion metrics
    pub fn metrics(&self) -> &ConversionMetrics {
        &self.metrics
    }
}

/// Conversion status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConversionStatus {
    Success,
    PartialSuccess,
    Failure,
}

/// Conversion metrics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConversionMetrics {
    total_pages: Option<usize>,
    processing_time_ms: Option<u64>,
}

impl ConversionMetrics {
    /// Create new empty metrics
    pub fn new() -> Self {
        Self::default()
    }

    /// Set total pages
    pub fn with_total_pages(mut self, pages: usize) -> Self {
        self.total_pages = Some(pages);
        self
    }

    /// Set processing time in milliseconds
    pub fn with_processing_time_ms(mut self, time_ms: u64) -> Self {
        self.processing_time_ms = Some(time_ms);
        self
    }

    /// Get total pages
    pub fn total_pages(&self) -> usize {
        self.total_pages.unwrap_or(0)
    }

    /// Get processing time in milliseconds
    pub fn processing_time_ms(&self) -> u64 {
        self.processing_time_ms.unwrap_or(0)
    }
}
