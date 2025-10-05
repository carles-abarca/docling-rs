//! Integration tests for error handling
//!
//! CT-016: Error scenarios and edge cases

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_conflicting_flags_quiet_verbose() {
    let temp = TempDir::new().unwrap();
    let input = temp.path().join("test.md");
    fs::write(&input, "# Test").unwrap();

    let mut cmd = Command::cargo_bin("docling-rs").unwrap();
    cmd.arg(&input)
        .arg("--quiet")
        .arg("--verbose")
        .assert()
        .failure()
        .code(1)
        .stderr(
            predicate::str::contains("conflict").or(predicate::str::contains("cannot be used")),
        );
}

#[test]
fn test_conflicting_error_handling_flags() {
    let temp = TempDir::new().unwrap();
    let input_dir = temp.path().join("docs");
    fs::create_dir(&input_dir).unwrap();
    fs::write(input_dir.join("test.md"), "# Test").unwrap();

    let mut cmd = Command::cargo_bin("docling-rs").unwrap();
    cmd.arg(&input_dir)
        .arg("--continue-on-error")
        .arg("--abort-on-error")
        .assert()
        .failure()
        .code(1)
        .stderr(
            predicate::str::contains("conflict").or(predicate::str::contains("cannot be used")),
        );
}

#[test]
fn test_invalid_output_format() {
    let temp = TempDir::new().unwrap();
    let input = temp.path().join("test.md");
    fs::write(&input, "# Test").unwrap();

    let mut cmd = Command::cargo_bin("docling-rs").unwrap();
    cmd.arg(&input)
        .arg("--to")
        .arg("invalid-format")
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("invalid").or(predicate::str::contains("Invalid")));
}

#[test]
fn test_invalid_chunk_size() {
    let temp = TempDir::new().unwrap();
    let input = temp.path().join("test.md");
    fs::write(&input, "# Test").unwrap();

    let mut cmd = Command::cargo_bin("docling-rs").unwrap();
    cmd.arg(&input)
        .arg("--chunk-size")
        .arg("0")
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("invalid").or(predicate::str::contains("must be")));
}

#[test]
fn test_empty_input_directory() {
    let temp = TempDir::new().unwrap();
    let input_dir = temp.path().join("empty");
    fs::create_dir(&input_dir).unwrap();

    let mut cmd = Command::cargo_bin("docling-rs").unwrap();
    cmd.arg(&input_dir)
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("No").or(predicate::str::contains("empty")));
}

#[test]
fn test_output_permission_error() {
    let temp = TempDir::new().unwrap();
    let input = temp.path().join("test.md");
    fs::write(&input, "# Test").unwrap();

    let output_dir = "/root/forbidden";

    let mut cmd = Command::cargo_bin("docling-rs").unwrap();
    cmd.arg(&input)
        .arg("--output-dir")
        .arg(output_dir)
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("Permission").or(predicate::str::contains("permission")));
}

#[test]
fn test_malformed_input_file() {
    let temp = TempDir::new().unwrap();
    let input = temp.path().join("malformed.html");
    // Write intentionally malformed HTML
    fs::write(&input, "<html><body><unclosed>").unwrap();

    let mut cmd = Command::cargo_bin("docling-rs").unwrap();
    cmd.arg(&input).assert().success(); // Should still succeed with best-effort parsing
}

#[test]
fn test_missing_required_argument() {
    let mut cmd = Command::cargo_bin("docling-rs").unwrap();
    cmd.assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("required").or(predicate::str::contains("INPUT")));
}

#[test]
fn test_chunk_without_size() {
    let temp = TempDir::new().unwrap();
    let input = temp.path().join("test.md");
    fs::write(&input, "# Test").unwrap();

    let mut cmd = Command::cargo_bin("docling-rs").unwrap();
    cmd.arg(&input).arg("--chunk").assert().success(); // Should use default chunk size
}
