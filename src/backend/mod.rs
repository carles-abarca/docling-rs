//! Backend implementations for different document formats

pub mod csv;
pub mod docx;
pub mod html;
pub mod markdown;
pub mod pdf;
pub mod traits;

// Re-exports
pub use csv::CsvBackend;
pub use docx::DocxBackend;
pub use html::HtmlBackend;
pub use markdown::MarkdownBackend;
pub use pdf::PdfBackend;
pub use traits::{Backend, DeclarativeBackend};
