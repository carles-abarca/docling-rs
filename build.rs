//! Build script for docling-rs
//!
//! This script copies bundled pdfium libraries to the target directory
//! so they can be dynamically loaded at runtime by pdfium-render.

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=pdfium/");

    // Get the target triple
    let target = env::var("TARGET").unwrap();

    // Determine which pdfium binary to use based on target
    let lib_dir = if target.contains("darwin") {
        // macOS
        if target.contains("aarch64") {
            "pdfium/lib/macos-arm64"
        } else {
            "pdfium/lib/macos-x64"
        }
    } else if target.contains("windows") {
        "pdfium/lib/windows-x64"
    } else {
        // For other platforms, skip pdfium configuration
        return;
    };

    // Get the absolute path to the library directory
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let lib_path = PathBuf::from(&manifest_dir).join(lib_dir);

    // Set PDFIUM_DYNAMIC_LIB_PATH environment variable so pdfium-render can find the library
    println!("cargo:rustc-env=PDFIUM_DYNAMIC_LIB_PATH={}", lib_path.display());

    // Copy the library to multiple locations for runtime access
    let profile = env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());
    let target_dir = PathBuf::from(&manifest_dir).join("target").join(&profile);

    #[cfg(target_os = "macos")]
    let lib_file = "libpdfium.dylib";
    #[cfg(target_os = "windows")]
    let lib_file = "pdfium.dll";

    #[cfg(any(target_os = "macos", target_os = "windows"))]
    {
        let src = lib_path.join(lib_file);

        // Copy to target directory so executables can find it
        if src.exists() {
            // Create target directory if it doesn't exist
            let _ = std::fs::create_dir_all(&target_dir);
            let dst = target_dir.join(lib_file);
            let _ = std::fs::copy(&src, &dst);

            // Also copy to deps directory for tests
            let deps_dir = target_dir.join("deps");
            let _ = std::fs::create_dir_all(&deps_dir);
            let deps_dst = deps_dir.join(lib_file);
            let _ = std::fs::copy(&src, &deps_dst);
        }
    }
}
