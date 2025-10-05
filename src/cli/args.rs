//! CLI argument parsing with clap.

use clap::{Parser, ValueEnum};
use std::path::PathBuf;

/// Validate chunk size is greater than 0
fn validate_chunk_size(s: &str) -> Result<usize, String> {
    let size: usize = s
        .parse()
        .map_err(|_| format!("invalid chunk size: {}", s))?;
    if size == 0 {
        return Err("chunk size must be greater than 0".to_string());
    }
    Ok(size)
}

/// docling-rs CLI - Document conversion tool
#[derive(Parser, Debug)]
#[command(name = "docling-rs")]
#[command(about = "Convert documents to structured text and metadata", long_about = None)]
#[command(version)]
pub struct CliArgs {
    /// Input file or directory path
    #[arg(value_name = "INPUT")]
    pub input: PathBuf,

    /// Output format (markdown, json, text)
    #[arg(
        short = 't',
        long = "to",
        value_name = "FORMAT",
        default_value = "markdown"
    )]
    pub output_format: OutputFormat,

    /// Output directory (default: current directory)
    #[arg(short = 'o', long = "output-dir", value_name = "DIR")]
    pub output_dir: Option<PathBuf>,

    /// Filter input files by format (for batch processing)
    #[arg(short = 'f', long = "from", value_name = "FORMAT")]
    pub input_format_filter: Option<String>,

    /// Enable OCR for scanned PDFs (requires tesseract)
    #[arg(long = "ocr-enabled")]
    pub ocr_enabled: bool,

    /// Extract tables from PDFs
    #[arg(long = "pdf-extract-tables")]
    pub pdf_extract_tables: bool,

    /// Extract images from PDFs
    #[arg(long = "pdf-extract-images")]
    pub pdf_extract_images: bool,

    /// Enable document chunking
    #[arg(long = "chunk")]
    pub chunk: bool,

    /// Chunk size in characters (default: 1000)
    #[arg(long = "chunk-size", value_name = "SIZE", default_value = "1000", value_parser = validate_chunk_size)]
    pub chunk_size: usize,

    /// Continue processing on error (batch mode)
    #[arg(long = "continue-on-error")]
    pub continue_on_error: bool,

    /// Abort on first error (batch mode)
    #[arg(long = "abort-on-error", conflicts_with = "continue_on_error")]
    pub abort_on_error: bool,

    /// Verbose output
    #[arg(short = 'v', long = "verbose")]
    pub verbose: bool,

    /// Quiet mode (no output except errors)
    #[arg(short = 'q', long = "quiet", conflicts_with = "verbose")]
    pub quiet: bool,
}

/// Input document format
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum InputFormat {
    /// Portable Document Format
    Pdf,
    /// Markdown
    Markdown,
    /// HTML
    Html,
    /// CSV
    Csv,
    /// Microsoft Word (DOCX)
    Docx,
}

impl InputFormat {
    /// Detect format from file extension
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "pdf" => Some(Self::Pdf),
            "md" | "markdown" => Some(Self::Markdown),
            "html" | "htm" => Some(Self::Html),
            "csv" => Some(Self::Csv),
            "docx" => Some(Self::Docx),
            _ => None,
        }
    }

    /// Convert to docling-rs InputFormat
    pub fn to_docling_format(self) -> crate::format::InputFormat {
        match self {
            Self::Pdf => crate::format::InputFormat::PDF,
            Self::Markdown => crate::format::InputFormat::Markdown,
            Self::Html => crate::format::InputFormat::Html,
            Self::Csv => crate::format::InputFormat::Csv,
            Self::Docx => crate::format::InputFormat::Docx,
        }
    }
}

/// Output format for converted documents
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum OutputFormat {
    /// Markdown format
    Markdown,
    /// JSON format
    Json,
    /// Plain text format
    Text,
}

impl std::fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Markdown => write!(f, "markdown"),
            Self::Json => write!(f, "json"),
            Self::Text => write!(f, "text"),
        }
    }
}

impl OutputFormat {
    /// Get file extension for this format
    pub fn extension(&self) -> &'static str {
        match self {
            Self::Markdown => "md",
            Self::Json => "json",
            Self::Text => "txt",
        }
    }
}
