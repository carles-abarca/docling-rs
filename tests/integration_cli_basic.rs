//! Integration tests for basic CLI workflows
//!
//! CT-014: Basic integration scenarios

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_cli_markdown_to_json() {
    let temp = TempDir::new().unwrap();
    let input = temp.path().join("input.md");
    let output_dir = temp.path().join("output");

    fs::write(&input, "# Heading\n\nParagraph text.").unwrap();

    let mut cmd = Command::cargo_bin("docling-rs").unwrap();
    cmd.arg(&input)
        .arg("--to")
        .arg("json")
        .arg("--output-dir")
        .arg(&output_dir)
        .assert()
        .success();

    let output_file = output_dir.join("input.json");
    assert!(output_file.exists(), "JSON output file should be created");

    let content = fs::read_to_string(&output_file).unwrap();
    assert!(
        content.contains("Heading") || content.contains("Paragraph"),
        "Output should contain document content"
    );
}

#[test]
fn test_cli_html_to_markdown() {
    let temp = TempDir::new().unwrap();
    let input = temp.path().join("input.html");
    let output_dir = temp.path().join("output");

    fs::write(
        &input,
        "<html><body><h1>Title</h1><p>Content</p></body></html>",
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("docling-rs").unwrap();
    cmd.arg(&input)
        .arg("--to")
        .arg("markdown")
        .arg("--output-dir")
        .arg(&output_dir)
        .assert()
        .success();

    let output_file = output_dir.join("input.md");
    assert!(
        output_file.exists(),
        "Markdown output file should be created"
    );
}

#[test]
fn test_cli_csv_conversion() {
    let temp = TempDir::new().unwrap();
    let input = temp.path().join("data.csv");

    fs::write(&input, "name,value\nAlice,100\nBob,200").unwrap();

    let mut cmd = Command::cargo_bin("docling-rs").unwrap();
    cmd.arg(&input)
        .arg("--to")
        .arg("markdown")
        .assert()
        .success();
}

#[test]
fn test_cli_version_flag() {
    let mut cmd = Command::cargo_bin("docling-rs").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn test_cli_verbose_output() {
    let temp = TempDir::new().unwrap();
    let input = temp.path().join("test.md");
    fs::write(&input, "# Test").unwrap();

    let mut cmd = Command::cargo_bin("docling-rs").unwrap();
    cmd.arg(&input)
        .arg("--verbose")
        .assert()
        .success()
        .stderr(predicate::str::contains("Processing").or(predicate::str::contains("Converting")));
}

#[test]
fn test_cli_quiet_mode() {
    let temp = TempDir::new().unwrap();
    let input = temp.path().join("test.md");
    fs::write(&input, "# Test").unwrap();

    let mut cmd = Command::cargo_bin("docling-rs").unwrap();
    cmd.arg(&input)
        .arg("--quiet")
        .assert()
        .success()
        .stdout(predicate::str::is_empty());
}
