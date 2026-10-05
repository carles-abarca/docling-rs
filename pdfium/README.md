# PDFium Binaries

This directory contains pre-compiled PDFium libraries for different platforms.

## Source

These binaries are downloaded from [bblanchon/pdfium-binaries](https://github.com/bblanchon/pdfium-binaries), which provides automated builds of the PDFium library.

## License

PDFium is licensed under a BSD-style license. See the [PDFium project](https://pdfium.googlesource.com/pdfium/) for more information.

## Included Binaries

- `lib/macos-arm64/libpdfium.dylib` - macOS ARM64 (Apple Silicon)
- `lib/macos-x64/libpdfium.dylib` - macOS x86_64 (Intel)
- `lib/windows-x64/pdfium.dll` - Windows x64

## Binding compatibility

The Rust dependency is pinned to `pdfium-render = 0.8.37` with default features
 disabled and the `pdfium_7350`, `thread_safe`, and `image` features enabled.
The default/latest API in 0.8.37 requires symbols absent from the checked-in
macOS ARM64 library. The API 7350 binding passes the attachment regression
suite against that existing library (SHA-256
`1662275f090e1dae9f3289f4e87d160d707861e322731ea6ffa378068a9c8100`).

This API choice is **not** a claim that the binary's Chromium revision is 7350:
the original download revision was not recorded. Intel macOS and Windows
binaries still require runtime validation. Do not update from a moving `latest`
URL without verifying the binding API and recording release URLs/checksums for
each platform. Other dependencies can unify Cargo features; the final consumer
must avoid enabling a different PDFium API through another dependency.

For an embedded app, use an absolute path to its packaged dynamic library:

```rust,no_run
let converter = docling_rs::DocumentConverter::with_pdfium_library(
    "/absolute/app/resource/path/libpdfium.dylib",
);
```

An explicit path fails on an invalid/incompatible library rather than silently
loading a different library. `DocumentConverter::new()` retains its existing
current-directory/system-library lookup behavior. These files are not
implicitly installed or located by the Rust library.

## Why Bundle Binaries?

Bundling the PDFium binaries with docling-rs provides:

1. **Easy installation**: Users don't need to install PDFium separately
2. **Consistent behavior**: Everyone uses the same version of PDFium
3. **Cross-platform**: Works on macOS (both Intel and Apple Silicon) and Windows without extra setup
4. **Offline builds**: No internet connection required during compilation

## v1.0.5 distribution changes

The macOS release combines both checked-in libraries with lipo and ad-hoc signs
the result. Native release regression tests and a packaged-CLI smoke test verify
loading on the runner. Default lookup now checks beside the executable and its
app Frameworks directory before the prior working-directory/system fallback.

Linux uses `chromium/7350/pdfium-linux-x64.tgz` from bblanchon/pdfium-binaries,
SHA-256 `5a53c802a970d7d1414c0d2fdd8a0900e239b3e7e4eff7a5ddcbd433322606f2`,
and includes its LICENSE and third-party notices. Windows uses the checked-in
library and must pass the native release PDF tests before publication.
