//! Build script for docling-rs
//!
//! This script configures the linker to use bundled pdfium libraries
//! based on the target platform.

use std::env;
use std::path::PathBuf;

fn main() {
    // Only configure pdfium linking if we're building the library
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=pdfium/");

    // Get the target triple
    let target = env::var("TARGET").unwrap();

    // Determine which pdfium binary to use based on target
    let (lib_dir, lib_name) = if target.contains("darwin") {
        // macOS
        if target.contains("aarch64") {
            // ARM64 (Apple Silicon)
            ("pdfium/lib/macos-arm64", "pdfium")
        } else {
            // x86_64 (Intel Mac)
            ("pdfium/lib/macos-x64", "pdfium")
        }
    } else if target.contains("windows") {
        // Windows
        ("pdfium/lib/windows-x64", "pdfium")
    } else {
        // For other platforms, skip pdfium configuration
        // Tests will gracefully handle missing pdfium
        return;
    };

    // Get the absolute path to the library directory
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let lib_path = PathBuf::from(&manifest_dir).join(lib_dir);

    // Tell cargo where to find the pdfium library
    println!("cargo:rustc-link-search=native={}", lib_path.display());
    println!("cargo:rustc-link-lib=dylib={}", lib_name);

    // On macOS, we also need to set the rpath so the binary can find the dylib
    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-link-arg=-Wl,-rpath,@executable_path");
        println!("cargo:rustc-link-arg=-Wl,-rpath,@loader_path");
        println!("cargo:rustc-link-arg=-Wl,-rpath,{}", lib_path.display());
    }

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
