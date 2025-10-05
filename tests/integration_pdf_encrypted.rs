//! Integration test: Encrypted PDF handling
//!
//! Tests PDF password/encryption handling including:
//! - Password-protected PDFs
//! - Encryption detection
//! - Error handling for wrong passwords
//!
//! Note: printpdf library doesn't support creating encrypted PDFs,
//! so most tests require pre-made encrypted PDF files.

mod helpers;
use helpers::pdf_fixtures::*;

use docling_rs::backend::pdf::PdfConfig;
use docling_rs::backend::{Backend, PdfBackend};
use docling_rs::cli::output;
use docling_rs::datamodel::InputDocument;
use docling_rs::InputFormat;

#[test]
#[ignore = "Requires real encrypted PDF file (printpdf doesn't support encryption)"]
fn test_encrypted_pdf_with_correct_password() {
    // This test verifies successful decryption with correct password
    // Note: Requires a real encrypted PDF file to test properly

    // Arrange: Create an encrypted PDF with known password
    let password = "secret123";
    let pdf_path = create_encrypted_pdf("Test content", password);

    let config = PdfConfig::default().password(Some(password.to_string()));
    let backend = PdfBackend::with_config(config);

    let input = InputDocument::from_path(pdf_path, InputFormat::PDF);

    // Act
    let result = backend.convert(&input);

    // Assert: Decryption succeeds
    assert!(result.is_ok(), "Should decrypt PDF with correct password");

    let doc = result.unwrap();
    let text = output::to_text(&doc);

    assert!(
        text.contains("Test content"),
        "Should extract text from decrypted PDF"
    );
}

#[test]
#[ignore = "Requires real encrypted PDF file (printpdf doesn't support encryption)"]
fn test_encrypted_pdf_without_password() {
    // This test verifies error handling when no password is provided
    // Note: Requires a real encrypted PDF file to test properly

    // Arrange: Create an encrypted PDF
    let pdf_path = create_encrypted_pdf("Secure content", "password123");

    let backend = PdfBackend::new(); // No password configured

    let input = InputDocument::from_path(pdf_path, InputFormat::PDF);

    // Act
    let result = backend.convert(&input);

    // Assert: Conversion fails with encryption error
    assert!(
        result.is_err(),
        "Should fail to convert encrypted PDF without password"
    );

    // Check error type
    let error = result.unwrap_err();
    assert!(
        matches!(
            error,
            docling_rs::error::ConversionError::EncryptionError(_)
        ),
        "Should return EncryptionError"
    );
}

#[test]
#[ignore = "Requires real encrypted PDF file (printpdf doesn't support encryption)"]
fn test_encrypted_pdf_with_wrong_password() {
    // This test verifies error handling for incorrect password
    // Note: Requires a real encrypted PDF file to test properly

    // Arrange
    let pdf_path = create_encrypted_pdf("Secret content", "correct_password");

    let config = PdfConfig::default().password(Some("wrong_password".to_string()));
    let backend = PdfBackend::with_config(config);

    let input = InputDocument::from_path(pdf_path, InputFormat::PDF);

    // Act
    let result = backend.convert(&input);

    // Assert: Conversion fails
    assert!(
        result.is_err(),
        "Should fail to decrypt PDF with wrong password"
    );
}

#[test]
fn test_unencrypted_pdf_with_password_provided() {
    // This test verifies handling when password is provided for unencrypted PDF

    // Arrange: Create a regular (unencrypted) PDF
    let pdf_path = create_simple_text_pdf("Regular content");

    let config = PdfConfig::default().password(Some("unnecessary_password".to_string()));
    let backend = PdfBackend::with_config(config);

    let input = InputDocument::from_path(pdf_path, InputFormat::PDF);

    // Act
    let result = backend.convert(&input);

    // Assert: Conversion should succeed (password is simply ignored)
    assert!(
        result.is_ok(),
        "Should convert unencrypted PDF even with password configured"
    );

    let doc = result.unwrap();
    let text = output::to_text(&doc);

    assert!(
        text.contains("Regular content"),
        "Should extract text normally"
    );
}

// Helper functions now imported from helpers::pdf_fixtures
// Note: create_encrypted_pdf exists but returns unencrypted PDFs
// because printpdf doesn't support encryption
