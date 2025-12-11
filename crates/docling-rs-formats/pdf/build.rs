//! Build script for docling-rs PDF backend
//!
//! This script copies bundled pdfium libraries to the target directory
//! so they can be dynamically loaded at runtime by pdfium-render.

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    // Get the target triple
    let target = env::var("TARGET").unwrap();

    // Get the workspace root (3 levels up from crates/docling-rs-formats/pdf)
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let workspace_root = manifest_dir
        .parent() // docling-rs-formats
        .and_then(|p| p.parent()) // crates
        .and_then(|p| p.parent()) // workspace root
        .expect("Could not find workspace root");

    println!(
        "cargo:rerun-if-changed={}",
        workspace_root.join("pdfium").display()
    );

    // Determine which pdfium binary to use based on target
    let lib_subdir = if target.contains("darwin") {
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

    let lib_path = workspace_root.join(lib_subdir);

    // Set PDFIUM_DYNAMIC_LIB_PATH environment variable so pdfium-render can find the library
    println!(
        "cargo:rustc-env=PDFIUM_DYNAMIC_LIB_PATH={}",
        lib_path.display()
    );

    // Copy the library to the workspace target directory for runtime access
    let profile = env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());
    let target_dir = workspace_root.join("target").join(&profile);

    #[cfg(target_os = "macos")]
    let lib_file = "libpdfium.dylib";
    #[cfg(target_os = "windows")]
    let lib_file = "pdfium.dll";

    #[cfg(any(target_os = "macos", target_os = "windows"))]
    {
        let src = lib_path.join(lib_file);

        if src.exists() {
            // Copy to target directory so executables can find it
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
