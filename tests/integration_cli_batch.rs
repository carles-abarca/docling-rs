//! Integration tests for batch processing workflows
//!
//! CT-015: Batch processing scenarios

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_batch_mixed_formats() {
    let temp = TempDir::new().unwrap();
    let input_dir = temp.path().join("docs");
    fs::create_dir(&input_dir).unwrap();

    fs::write(input_dir.join("doc1.md"), "# Markdown").unwrap();
    fs::write(input_dir.join("doc2.html"), "<h1>HTML</h1>").unwrap();
    fs::write(input_dir.join("doc3.csv"), "a,b\n1,2").unwrap();

    let mut cmd = Command::cargo_bin("docling-rs").unwrap();
    cmd.arg(&input_dir)
        .arg("--to")
        .arg("json")
        .assert()
        .success()
        .stdout(predicate::str::contains("doc1.md"))
        .stdout(predicate::str::contains("doc2.html"))
        .stdout(predicate::str::contains("doc3.csv"));
}

#[test]
fn test_batch_with_subdirectories() {
    let temp = TempDir::new().unwrap();
    let input_dir = temp.path().join("docs");
    let subdir = input_dir.join("subdir");
    fs::create_dir_all(&subdir).unwrap();

    fs::write(input_dir.join("root.md"), "# Root").unwrap();
    fs::write(subdir.join("nested.md"), "# Nested").unwrap();

    let mut cmd = Command::cargo_bin("docling-rs").unwrap();
    cmd.arg(&input_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("root.md"))
        .stdout(predicate::str::contains("nested.md"));
}

#[test]
fn test_batch_partial_failure() {
    let temp = TempDir::new().unwrap();
    let input_dir = temp.path().join("docs");
    fs::create_dir(&input_dir).unwrap();

    fs::write(input_dir.join("good.md"), "# Good").unwrap();
    fs::write(input_dir.join("bad.xyz"), "unsupported").unwrap();
    fs::write(input_dir.join("good2.md"), "# Also Good").unwrap();

    let mut cmd = Command::cargo_bin("docling-rs").unwrap();
    cmd.arg(&input_dir)
        .arg("--continue-on-error")
        .assert()
        .success()
        .stdout(predicate::str::contains("good.md"))
        .stdout(predicate::str::contains("good2.md"))
        .stderr(predicate::str::contains("bad.xyz"));
}

#[test]
fn test_batch_output_directory_structure() {
    let temp = TempDir::new().unwrap();
    let input_dir = temp.path().join("docs");
    let output_dir = temp.path().join("output");
    let subdir = input_dir.join("subdir");
    fs::create_dir_all(&subdir).unwrap();

    fs::write(input_dir.join("root.md"), "# Root").unwrap();
    fs::write(subdir.join("nested.md"), "# Nested").unwrap();

    let mut cmd = Command::cargo_bin("docling-rs").unwrap();
    cmd.arg(&input_dir)
        .arg("--output-dir")
        .arg(&output_dir)
        .assert()
        .success();

    // Verify output structure mirrors input
    assert!(output_dir.join("root.md").exists());
    assert!(output_dir.join("subdir").join("nested.md").exists());
}

#[test]
fn test_batch_format_filter_multiple() {
    let temp = TempDir::new().unwrap();
    let input_dir = temp.path().join("docs");
    fs::create_dir(&input_dir).unwrap();

    fs::write(input_dir.join("doc.md"), "# MD").unwrap();
    fs::write(input_dir.join("doc.html"), "<h1>HTML</h1>").unwrap();
    fs::write(input_dir.join("doc.csv"), "a,b").unwrap();
    fs::write(input_dir.join("doc.txt"), "text").unwrap();

    let mut cmd = Command::cargo_bin("docling-rs").unwrap();
    cmd.arg(&input_dir)
        .arg("--from")
        .arg("markdown,html")
        .assert()
        .success()
        .stdout(predicate::str::contains("doc.md"))
        .stdout(predicate::str::contains("doc.html"))
        .stdout(predicate::str::contains("doc.csv").not())
        .stdout(predicate::str::contains("doc.txt").not());
}
