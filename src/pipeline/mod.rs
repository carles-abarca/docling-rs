//! Pipeline implementations for document processing

pub mod simple;
pub mod traits;

// Re-exports
pub use simple::SimplePipeline;
pub use traits::Pipeline;
