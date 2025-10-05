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

## Version

These binaries are from the latest release as of the time they were downloaded.

To update to a newer version:

```bash
# macOS ARM64
curl -L -o /tmp/pdfium-mac-arm64.tgz https://github.com/bblanchon/pdfium-binaries/releases/latest/download/pdfium-mac-arm64.tgz
tar -xzf /tmp/pdfium-mac-arm64.tgz -C /tmp
cp /tmp/lib/libpdfium.dylib pdfium/lib/macos-arm64/

# macOS x64
curl -L -o /tmp/pdfium-mac-x64.tgz https://github.com/bblanchon/pdfium-binaries/releases/latest/download/pdfium-mac-x64.tgz
tar -xzf /tmp/pdfium-mac-x64.tgz -C /tmp
cp /tmp/lib/libpdfium.dylib pdfium/lib/macos-x64/

# Windows x64
curl -L -o /tmp/pdfium-win-x64.tgz https://github.com/bblanchon/pdfium-binaries/releases/latest/download/pdfium-win-x64.tgz
tar -xzf /tmp/pdfium-win-x64.tgz -C /tmp
cp /tmp/bin/pdfium.dll pdfium/lib/windows-x64/
```

## Why Bundle Binaries?

Bundling the PDFium binaries with docling-rs provides:

1. **Easy installation**: Users don't need to install PDFium separately
2. **Consistent behavior**: Everyone uses the same version of PDFium
3. **Cross-platform**: Works on macOS (both Intel and Apple Silicon) and Windows without extra setup
4. **Offline builds**: No internet connection required during compilation
