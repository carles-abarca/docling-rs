//! Conversion orchestration logic.

use crate::chunking::{BaseChunker, HierarchicalChunker};
use crate::cli::args::{CliArgs, InputFormat, OutputFormat};
use crate::cli::output;
use crate::DocumentConverter;
use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

/// Single file conversion job
#[derive(Debug)]
pub struct ConversionJob {
    pub input_path: PathBuf,
    pub output_path: PathBuf,
    pub format: InputFormat,
}

/// Result of a single conversion
#[derive(Debug)]
pub struct ConversionResult {
    pub job: ConversionJob,
    pub success: bool,
    pub error: Option<String>,
}

/// Batch conversion progress tracker
#[derive(Debug, Default)]
pub struct BatchProgress {
    pub total: usize,
    pub processed: usize,
    pub successful: usize,
    pub failed: usize,
}

/// Main CLI converter orchestrator
pub struct Converter {
    args: CliArgs,
    converter: DocumentConverter,
}

impl Converter {
    /// Create new converter with CLI arguments
    pub fn new(args: CliArgs) -> Self {
        Self {
            args,
            converter: DocumentConverter::new(),
        }
    }

    /// Run conversion based on CLI arguments
    pub fn run(&self) -> Result<()> {
        // Validate input exists
        if !self.args.input.exists() {
            anyhow::bail!("Input path does not exist: {:?}", self.args.input);
        }

        // Determine if batch or single file
        if self.args.input.is_dir() {
            self.run_batch()
        } else {
            self.run_single()
        }
    }

    /// Convert single file
    fn run_single(&self) -> Result<()> {
        let input_path = &self.args.input;

        // Detect format
        let format = self.detect_format(input_path)?;

        // Determine output path
        let output_path = self.get_output_path(input_path)?;

        if self.args.verbose {
            eprintln!("Converting {:?} to {:?}", input_path, output_path);
        }

        // Create job
        let job = ConversionJob {
            input_path: input_path.clone(),
            output_path: output_path.clone(),
            format,
        };

        // Execute conversion
        let result = self.convert_file(&job);

        // Handle result
        if result.success {
            if !self.args.quiet {
                // Print input filename instead of output path
                if let Some(filename) = input_path.file_name() {
                    println!("{}", filename.to_string_lossy());
                }
            }
            Ok(())
        } else {
            anyhow::bail!(
                "Conversion failed: {}",
                result.error.unwrap_or_else(|| "Unknown error".to_string())
            );
        }
    }

    /// Convert batch of files in directory
    fn run_batch(&self) -> Result<()> {
        let input_dir = &self.args.input;

        // Collect all files
        let jobs = self.collect_jobs(input_dir)?;

        if jobs.is_empty() {
            anyhow::bail!("No supported files found in directory");
        }

        let mut progress = BatchProgress {
            total: jobs.len(),
            ..Default::default()
        };

        if !self.args.quiet && self.args.verbose {
            eprintln!("Processing {} files...", jobs.len());
        }

        // Process each file
        for job in jobs {
            let result = self.convert_file(&job);
            progress.processed += 1;

            if result.success {
                progress.successful += 1;
                if !self.args.quiet {
                    // Print input filename instead of output path
                    if let Some(filename) = job.input_path.file_name() {
                        println!("{}", filename.to_string_lossy());
                    }
                }
            } else {
                progress.failed += 1;
                if !self.args.quiet {
                    // Print input filename in error message
                    if let Some(filename) = job.input_path.file_name() {
                        eprintln!(
                            "{}",
                            filename.to_string_lossy()
                        );
                    }
                }

                // Handle abort on error
                if self.args.abort_on_error {
                    anyhow::bail!("Aborting due to error");
                }
            }
        }

        // Final result
        if progress.failed > 0 && !self.args.continue_on_error {
            anyhow::bail!("{} files failed to convert", progress.failed);
        }

        Ok(())
    }

    /// Collect conversion jobs from directory (recursive)
    fn collect_jobs(&self, dir: &Path) -> Result<Vec<ConversionJob>> {
        let mut jobs = Vec::new();
        self.collect_jobs_recursive(dir, dir, &mut jobs)?;
        Ok(jobs)
    }

    /// Recursively collect jobs from directory
    fn collect_jobs_recursive(
        &self,
        root_dir: &Path,
        current_dir: &Path,
        jobs: &mut Vec<ConversionJob>,
    ) -> Result<()> {
        for entry in fs::read_dir(current_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                // Recursively process subdirectories
                self.collect_jobs_recursive(root_dir, &path, jobs)?;
            } else if path.is_file() {
                // Check format filter
                if let Some(ref filter) = self.args.input_format_filter {
                    if let Some(format) = self.try_detect_format(&path) {
                        let format_str = format!("{:?}", format).to_lowercase();
                        if !filter.to_lowercase().contains(&format_str) {
                            continue;
                        }
                    } else {
                        continue;
                    }
                }

                // Detect format and create job
                if let Some(format) = self.try_detect_format(&path) {
                    if let Ok(output_path) = self.get_output_path_for_batch(&path, root_dir) {
                        jobs.push(ConversionJob {
                            input_path: path.clone(),
                            output_path,
                            format,
                        });
                    }
                } else {
                    // Unsupported format - report to stderr if not quiet
                    if !self.args.quiet {
                        if let Some(filename) = path.file_name() {
                            eprintln!("Unsupported format, skipping: {}", filename.to_string_lossy());
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Convert a single file job
    fn convert_file(&self, job: &ConversionJob) -> ConversionResult {
        match self.do_convert(job) {
            Ok(()) => ConversionResult {
                job: ConversionJob {
                    input_path: job.input_path.clone(),
                    output_path: job.output_path.clone(),
                    format: job.format,
                },
                success: true,
                error: None,
            },
            Err(e) => ConversionResult {
                job: ConversionJob {
                    input_path: job.input_path.clone(),
                    output_path: job.output_path.clone(),
                    format: job.format,
                },
                success: false,
                error: Some(e.to_string()),
            },
        }
    }

    /// Perform actual conversion
    fn do_convert(&self, job: &ConversionJob) -> Result<()> {
        // Convert using convert_file
        let result = self
            .converter
            .convert_file(&job.input_path)
            .with_context(|| format!("Failed to convert {:?}", job.input_path))?;

        // Get document
        let doc = result.document();

        // Apply chunking if enabled
        let output_content = if self.args.chunk {
            self.generate_chunked_output(doc)?
        } else {
            // Generate output based on format (no chunking)
            match self.args.output_format {
                OutputFormat::Markdown => output::to_markdown(doc),
                OutputFormat::Json => output::to_json(doc)?,
                OutputFormat::Text => output::to_text(doc),
            }
        };

        // Ensure output directory exists
        if let Some(parent) = job.output_path.parent() {
            fs::create_dir_all(parent).with_context(|| {
                format!(
                    "Permission denied or unable to create output directory: {:?}",
                    parent
                )
            })?;
        }

        // Write output
        fs::write(&job.output_path, output_content).with_context(|| {
            format!(
                "Permission denied or unable to write output file: {:?}",
                job.output_path
            )
        })?;

        Ok(())
    }

    /// Detect format from file path (error if unsupported)
    fn detect_format(&self, path: &Path) -> Result<InputFormat> {
        self.try_detect_format(path)
            .ok_or_else(|| anyhow::anyhow!("Unsupported file format: {:?}", path))
    }

    /// Try to detect format (returns None if unsupported)
    fn try_detect_format(&self, path: &Path) -> Option<InputFormat> {
        path.extension()
            .and_then(|ext| ext.to_str())
            .and_then(InputFormat::from_extension)
    }

    /// Get output path for input file (single file mode)
    fn get_output_path(&self, input_path: &Path) -> Result<PathBuf> {
        let file_stem = input_path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| anyhow::anyhow!("Invalid filename"))?;

        let extension = self.args.output_format.extension();
        let output_filename = format!("{}.{}", file_stem, extension);

        let output_path = if let Some(ref output_dir) = self.args.output_dir {
            output_dir.join(output_filename)
        } else {
            PathBuf::from(output_filename)
        };

        Ok(output_path)
    }

    /// Get output path for batch processing (preserves directory structure)
    fn get_output_path_for_batch(&self, input_path: &Path, root_dir: &Path) -> Result<PathBuf> {
        let file_stem = input_path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| anyhow::anyhow!("Invalid filename"))?;

        let extension = self.args.output_format.extension();
        let output_filename = format!("{}.{}", file_stem, extension);

        // Get relative path from root_dir
        let rel_path = input_path
            .parent()
            .and_then(|p| p.strip_prefix(root_dir).ok())
            .unwrap_or_else(|| Path::new(""));

        let output_path = if let Some(ref output_dir) = self.args.output_dir {
            output_dir.join(rel_path).join(output_filename)
        } else {
            rel_path.join(output_filename)
        };

        Ok(output_path)
    }

    /// Generate chunked output from document
    fn generate_chunked_output(&self, doc: &crate::datamodel::DoclingDocument) -> Result<String> {
        // Create hierarchical chunker
        let chunker = HierarchicalChunker::new();

        // Collect all chunks
        let chunks: Vec<_> = chunker.chunk(doc).collect();

        // Format based on output format
        match self.args.output_format {
            OutputFormat::Json => {
                // Output chunks as JSON array
                Ok(serde_json::to_string_pretty(&chunks)?)
            }
            OutputFormat::Markdown | OutputFormat::Text => {
                // Output chunks separated by newlines with metadata
                let mut output = String::new();
                for (i, chunk) in chunks.iter().enumerate() {
                    if i > 0 {
                        output.push_str("\n---\n\n");
                    }
                    // Add chunk metadata
                    output.push_str(&format!("# Chunk {} of {}\n", i + 1, chunks.len()));
                    if !chunk.meta.headings.is_empty() {
                        output.push_str(&format!("Context: {}\n", chunk.meta.headings.join(" > ")));
                    }
                    output.push_str(&format!("Size: {} characters\n\n", chunk.text.len()));
                    // Add chunk text
                    output.push_str(&chunk.text);
                    output.push('\n');
                }
                Ok(output)
            }
        }
    }
}
