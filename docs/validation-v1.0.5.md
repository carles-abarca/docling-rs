# docling-rs v1.0.5 validation

Local validation: 2026-10-05, Apple M4 Pro / 24 GiB, macOS 26.5 ARM64, Rust 1.98.1.
The candidate was built from the changes following v1.0.4 (`983d103`). The JSON
records that baseline commit and explicitly marks the modified candidate; it does
not pretend that v1.0.5 results came from a clean v1.0.4 checkout.

## Outcome

- All ten targeted regression tests pass, including a PDFium test normally ignored.
- The original extended matrix completed all **1,783 requested conversions** with
  controlled results (success or an expected error), instead of three DOCX panics.
- **47/49 synthetic content/error/order/chunk contracts pass**, versus 39/49 in
  v1.0.4. The remaining two are unsupported TXT and scanned PDF requiring OCR.
- Deep HTML now deliberately returns a depth-limit error instead of accepting
  2,000 nested containers; this changed safety expectation is explicit.
- **20/20 corrupted mutations are handled**, versus 17/20 previously. CRC/deflate
  failures return errors without invoking the old docx-rs parser.
- 900 repeated conversions and 400 shared-converter threaded conversions completed.
- After the final HTML whitespace fix, 15 content checks, a 150-cycle HTML soak,
  and two **1,000-cycle DOCX/PDF soaks** also passed. Combined: **3,948 conversions**.
- Workspace tests/doctests and strict Clippy are release gates; see CI for results.

## Performance (candidate, 25 repetitions per workload)

| Workload | Median total ms | Sampled peak RSS MiB |
| --- | ---: | ---: |
| Markdown 10,000 paragraphs | 1,257.9 | 40.5 |
| CSV 5,000 rows | 492.5 | 28.2 |
| DOCX 1,000 paragraphs | 91.5 | 25.7 |
| XLSX 10,000 rows | 978.7 | 35.5 |
| PPTX 100 slides | 31.7 | 23.1 |
| PDF 100 pages | 323.7 | 36.0 |
| PDF 500 pages | 1,567.9 | 59.7 |

Total includes extraction, text/Markdown export, chunking and token recount for
validation; excludes startup, artifact writes and sampling pauses. Chunking remains
the main cost. PDF column analysis adds work; this release is not a speedup claim.

RSS is sampled every 20 ms and may miss transient peaks. In the longer soaks,
DOCX grows from ~24.9 to 34.8 MiB and PDF from ~32.8 to 34.8 MiB, then both remain
flat from approximately iteration 200 through 1,000. This shows a plateau in these
inputs, not proof of no leaks, allocator attribution or a process memory ceiling.
The test harness has separate 768 MiB/time guards, not implemented by the library.

The four existing real corpus documents retain reference token coverage of 100%
(DOCX/XLSX), 99.88% (PPTX, improved from 99.11%) and 99.98% (PDF). Coverage uses
independent XML/openpyxl/PyMuPDF references; it is not human-scored semantic,
reading-order, table-layout or OCR accuracy.

## Changed contracts and limitations

- DOCX/PPTX use bounded XML extraction, not full Office rendering. Headers/footers
  are emitted once per referenced part. PPTX includes speaker notes in slide order.
- Images/charts/alternate content trigger partial-conversion warnings where detected.
- Simple two-column PDF reconstruction requires a clear full-page gutter and
  overlapping text rectangles. It returns PartialSuccess with an explicit heuristic
  warning. Arbitrary layouts, tables and spanning headings remain outside the claim.
- `HybridChunker::try_chunk` reports an impossible scalar/context budget as an error.
  The infallible legacy iterator preserves content in that case, so strict consumers
  must use try_chunk or validate output. The CLI uses the strict method.
- Office limits: 256 MiB input, 20,000 ZIP entries, 16 MiB per XML part, 64 MiB total
  declared XML size and depth 128. HTML nesting limit: 256. These do not cover every
  native allocation, XLSX expansion or cancellation of a running native parser.
- TXT and OCR remain unimplemented. Tesseract fast belongs to the planned OCR work.
- Distribution gates run native regression tests and packaged-CLI PDF smoke checks
  on macOS, Windows and Linux. The macOS PDFium artifact contains both architectures;
  only the runner's native architecture is exercised, not both Intel and ARM64.
- Long-duration mixed workloads, low-memory machines, advanced PDF layouts and
  adversarial/fuzz coverage remain necessary before general LG attachment release.

Detailed candidate measurements: [validation-v1.0.5.json](validation-v1.0.5.json).
The previous [attachment fidelity report](attachment-fidelity.md) remains historical.
