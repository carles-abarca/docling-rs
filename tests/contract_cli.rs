//! Contract tests for CLI interface
//!
//! These tests verify the CLI behavior according to contracts/cli_interface.md
//! They use assert_cmd to test the binary as a black box.

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

/// CT-001: Single file conversion produces expected output
#[test]
fn test_ct001_single_file_conversion() {
    let temp = TempDir::new().unwrap();
    let input = temp.path().join("test.md");
    fs::write(&input, "# Test\nContent").unwrap();

    let mut cmd = Command::cargo_bin("docling-rs").unwrap();
    cmd.arg(&input)
        .assert()
        .success()
        .stdout(predicate::str::contains("test.md"));
}

/// CT-002: Multiple output formats (markdown, JSON, text)
#[test]
fn test_ct002_multiple_output_formats() {
    let temp = TempDir::new().unwrap();
    let input = temp.path().join("test.md");
    fs::write(&input, "# Test").unwrap();

    // Test markdown output
    let mut cmd = Command::cargo_bin("docling-rs").unwrap();
    cmd.arg(&input)
        .arg("--to")
        .arg("markdown")
        .assert()
        .success();

    // Test JSON output
    let mut cmd = Command::cargo_bin("docling-rs").unwrap();
    cmd.arg(&input)
        .arg("--to")
        .arg("json")
        .assert()
        .success();

    // Test text output
    let mut cmd = Command::cargo_bin("docling-rs").unwrap();
    cmd.arg(&input)
        .arg("--to")
        .arg("text")
        .assert()
        .success();
}

/// CT-003: Custom output directory (--output-dir)
#[test]
fn test_ct003_custom_output_dir() {
    let temp = TempDir::new().unwrap();
    let input = temp.path().join("test.md");
    let output_dir = temp.path().join("output");
    fs::write(&input, "# Test").unwrap();

    let mut cmd = Command::cargo_bin("docling-rs").unwrap();
    cmd.arg(&input)
        .arg("--output-dir")
        .arg(&output_dir)
        .assert()
        .success();

    // Verify output file exists in custom directory
    assert!(output_dir.join("test.md").exists());
}

/// CT-004: Batch directory processing
#[test]
fn test_ct004_batch_directory_processing() {
    let temp = TempDir::new().unwrap();
    let input_dir = temp.path().join("docs");
    fs::create_dir(&input_dir).unwrap();
    fs::write(input_dir.join("doc1.md"), "# Doc 1").unwrap();
    fs::write(input_dir.join("doc2.md"), "# Doc 2").unwrap();

    let mut cmd = Command::cargo_bin("docling-rs").unwrap();
    cmd.arg(&input_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("doc1.md"))
        .stdout(predicate::str::contains("doc2.md"));
}

/// CT-005: Format filtering in batch mode (--from)
#[test]
fn test_ct005_format_filtering() {
    let temp = TempDir::new().unwrap();
    let input_dir = temp.path().join("docs");
    fs::create_dir(&input_dir).unwrap();
    fs::write(input_dir.join("doc.md"), "# Markdown").unwrap();
    fs::write(input_dir.join("doc.html"), "<h1>HTML</h1>").unwrap();

    let mut cmd = Command::cargo_bin("docling-rs").unwrap();
    cmd.arg(&input_dir)
        .arg("--from")
        .arg("markdown")
        .assert()
        .success()
        .stdout(predicate::str::contains("doc.md"))
        .stdout(predicate::str::contains("doc.html").not());
}

/// CT-006: Input file not found error
#[test]
fn test_ct006_file_not_found() {
    let mut cmd = Command::cargo_bin("docling-rs").unwrap();
    cmd.arg("nonexistent.md")
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("not found").or(predicate::str::contains("No such file")));
}

/// CT-007: Unsupported format error
#[test]
fn test_ct007_unsupported_format() {
    let temp = TempDir::new().unwrap();
    let input = temp.path().join("test.xyz");
    fs::write(&input, "content").unwrap();

    let mut cmd = Command::cargo_bin("docling-rs").unwrap();
    cmd.arg(&input)
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("Unsupported").or(predicate::str::contains("unsupported")));
}

/// CT-008: PDF with OCR feature (--ocr-enabled)
#[cfg(feature = "ocr")]
#[test]
fn test_ct008_pdf_with_ocr() {
    let temp = TempDir::new().unwrap();
    let input = temp.path().join("test.pdf");
    // Create a minimal PDF for testing
    fs::write(&input, b"%PDF-1.4\n").unwrap();

    let mut cmd = Command::cargo_bin("docling-rs").unwrap();
    cmd.arg(&input)
        .arg("--ocr-enabled")
        .assert()
        .success();
}

/// CT-009: PDF options (tables, images)
#[test]
fn test_ct009_pdf_options() {
    let temp = TempDir::new().unwrap();
    let input = temp.path().join("test.pdf");
    fs::write(&input, b"%PDF-1.4\n").unwrap();

    let mut cmd = Command::cargo_bin("docling-rs").unwrap();
    cmd.arg(&input)
        .arg("--pdf-extract-tables")
        .arg("--pdf-extract-images")
        .assert()
        .success();
}

/// CT-010: Enrichment options (chunk, embeddings)
#[test]
fn test_ct010_enrichment_options() {
    let temp = TempDir::new().unwrap();
    let input = temp.path().join("test.md");
    fs::write(&input, "# Test").unwrap();

    let mut cmd = Command::cargo_bin("docling-rs").unwrap();
    cmd.arg(&input)
        .arg("--chunk")
        .arg("--chunk-size")
        .arg("1000")
        .assert()
        .success();
}

/// CT-011: Batch error handling (continue on error)
#[test]
fn test_ct011_batch_continue_on_error() {
    let temp = TempDir::new().unwrap();
    let input_dir = temp.path().join("docs");
    fs::create_dir(&input_dir).unwrap();
    fs::write(input_dir.join("good.md"), "# Good").unwrap();
    fs::write(input_dir.join("bad.xyz"), "bad").unwrap();

    let mut cmd = Command::cargo_bin("docling-rs").unwrap();
    cmd.arg(&input_dir)
        .arg("--continue-on-error")
        .assert()
        .success()
        .stderr(predicate::str::contains("error").or(predicate::str::contains("failed")));
}

/// CT-012: Batch error handling (abort on error)
#[test]
fn test_ct012_batch_abort_on_error() {
    let temp = TempDir::new().unwrap();
    let input_dir = temp.path().join("docs");
    fs::create_dir(&input_dir).unwrap();
    fs::write(input_dir.join("bad.xyz"), "bad").unwrap();
    fs::write(input_dir.join("good.md"), "# Good").unwrap();

    let mut cmd = Command::cargo_bin("docling-rs").unwrap();
    cmd.arg(&input_dir)
        .arg("--abort-on-error")
        .assert()
        .failure()
        .code(1);
}

/// CT-013: Help output displays usage information
#[test]
fn test_ct013_help_output() {
    let mut cmd = Command::cargo_bin("docling-rs").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("docling-rs"))
        .stdout(predicate::str::contains("USAGE").or(predicate::str::contains("Usage")));
}
