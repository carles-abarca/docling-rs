# Attachment fidelity corrections for LiteGate

Historical v1.0.4 report. See [v1.0.5 validation](validation-v1.0.5.md) for the expanded findings and corrections.

Evaluated on macOS 26.5 / Apple Silicon with Rust 1.98.1, against base commit
`c9ccbcd437c371ec403855968d0a90cda1301624`. Changes are on the local branch
`codex/litegate-attachment-fidelity`; not published to crates.io.

## Changes

- DOCX tables (including nested table text), run line breaks and tabs are retained.
- HTML headings, paragraphs, lists and tables retain document order; script/style
  content is excluded from selected blocks.
- XLSX includes sheet headings and formula text, even without cached results.
  Cached values and formulas are both retained when present; formulas are not
  evaluated. Worksheet parse errors propagate instead of silently omitting sheets.
- Table chunks retain header-only tables, single-column rows, empty first cells,
  missing headings and extra cells in ragged rows. Markdown escapes pipes and
  line breaks. Chunk serialization now includes a separate first-row chunk and
  column labels on subsequent rows; consumers must allow changed chunk boundaries.
- Token counting disables padding and truncation for every constructor.
  Empty input counts as zero and `hello` as one token, rather than 128 each.
- PDFium bindings are pinned to an API verified with the checked-in ARM64 library.
  `DocumentConverter::with_pdfium_library(path)` and
  `PdfBackend::with_library_path(path)` support explicit dynamic library files.
- PDF pages with no extractable text add one-based `textless_pages` and
  `conversion_warnings` metadata. SimplePipeline returns `PartialSuccess` when
  that warnings array is nonempty. A blank page and a scanned page cannot be
  distinguished by this check. Callers must inspect status and warnings.
- README export examples use the public output functions.

## Verification

```sh
DYLD_LIBRARY_PATH="$PWD/pdfium/lib/macos-arm64" cargo +1.98.1 test --workspace --lib --tests --no-fail-fast
DOCLING_TEST_PDFIUM="$PWD/pdfium/lib/macos-arm64/libpdfium.dylib" cargo +1.98.1 test -p docling-rs --test attachment_fidelity -- --ignored
```

Full suite: 156 passed, 0 failed, 18 ignored. The separate PDFium test above
passes as well (one of the otherwise ignored tests). Synthetic fixtures are
under `crates/docling-rs/tests/fixtures/attachments`; no personal documents.

The LG evaluator covers 17 cases. 15 satisfy their original content/error
expectations. TXT is unsupported, and a scan cannot satisfy its OCR content
expectation (it now reports PartialSuccess). All successfully extracted content
markers also survive chunking; reported chunks remain within the 128-token budget.
Peak RSS in this small-fixture run is under 29 MiB. This is not a memory bound
or a hostile/large-document stress test. See `attachment-evaluation.json`.

## Remaining integration limits

No OCR implementation was added. The optional OCR dependency alone does not
perform recognition. PDF extraction still reads text lines, without reconstructing
complex layouts or tables. TXT/code can be read directly as UTF-8 by LiteGate.
HTML extraction remains block based, not browser rendering; DOCX structured
content controls and other unsupported elements need separate coverage.

Conversion is synchronous, without cancellation or strict resource limits.
Do not treat a client timeout as cancellation of the parser. Archive expansion,
sparse spreadsheets, nested input, giant documents and concurrent conversions
need a separate limits/stress effort before accepting untrusted attachments.
Windows and Intel macOS execution have not been validated here.

The workspace's advertised Rust 1.75 minimum is not reproducible with today's
unlocked dependencies (even 1.86 failed dependency MSRV checks). Tests here use
1.98.1; dependency/MSRV policy needs a separate release decision.
