# Phase 0: Dependency Research & Decisions

**Feature**: Core Document Processing Library (MVP Phase 1)
**Date**: 2025-10-04

## Rust Crate Research

### 1. Markdown Parsing

**Requirement**: Parse CommonMark/GFM Markdown into structured format

**Candidates**:
- **pulldown-cmark** v0.9+
  - ✅ CommonMark compliant
  - ✅ Zero-copy parsing (performance)
  - ✅ Mature, widely used (>10M downloads)
  - ✅ Pure Rust, no unsafe code
  - ❌ No GFM tables/task lists out of box
  - **crates.io**: https://crates.io/crates/pulldown-cmark

- **comrak** v0.18+
  - ✅ GitHub Flavored Markdown support
  - ✅ Tables, strikethrough, task lists
  - ✅ Based on cmark-gfm (battle-tested)
  - ⚠️ Slightly heavier than pulldown-cmark
  - **crates.io**: https://crates.io/crates/comrak

**Decision**: **pulldown-cmark**

**Rationale**:
- MVP focuses on basic Markdown → simplicity over GFM features
- Zero-copy parsing aligns with performance goals (<100ms)
- Extensive ecosystem (more examples, better documented)
- Can add comrak later if GFM needed

**Python Mapping**: `marko` → `pulldown-cmark`

---

### 2. HTML Parsing

**Requirement**: Parse HTML into semantic structure (headings, paragraphs, tables, lists)

**Candidates**:
- **scraper** v0.17+
  - ✅ Built on html5ever (standards-compliant)
  - ✅ CSS selector support (easy semantic extraction)
  - ✅ Good error handling for malformed HTML
  - ✅ Pure Rust
  - **crates.io**: https://crates.io/crates/scraper

- **html5ever** v0.26+
  - ✅ Low-level, full control
  - ❌ More verbose API
  - ⚠️ Requires more manual tree traversal
  - **crates.io**: https://crates.io/crates/html5ever

**Decision**: **scraper**

**Rationale**:
- CSS selectors simplify semantic extraction (h1-h6, p, table, ul/ol)
- Handles malformed HTML gracefully (Edge Case requirement)
- Higher-level API reduces implementation complexity
- Built on html5ever anyway (same parsing engine)

**Python Mapping**: `beautifulsoup4` → `scraper`

---

### 3. CSV Parsing

**Requirement**: Parse CSV files with headers into tabular structure

**Candidates**:
- **csv** v1.3+ (BurntSushi)
  - ✅ Industry standard Rust CSV library
  - ✅ Excellent performance
  - ✅ Flexible API (serde integration)
  - ✅ Handles edge cases (quoted fields, inconsistent columns)
  - **crates.io**: https://crates.io/crates/csv

- **polars** v0.35+
  - ✅ DataFrame library with CSV support
  - ❌ Overkill for simple parsing
  - ❌ Large dependency
  - **crates.io**: https://crates.io/crates/polars

**Decision**: **csv**

**Rationale**:
- Minimal, focused on CSV parsing only
- Handles inconsistent column counts (FR requirement)
- Serde integration for future flexibility
- Well-maintained, battle-tested

**Python Mapping**: `pandas.read_csv` → `csv` crate

---

### 4. DOCX Parsing

**Requirement**: Parse DOCX files (Office Open XML) into structured format with formatting

**Candidates**:
- **docx-rs** v0.4+
  - ✅ Full DOCX support (paragraphs, tables, images, formatting)
  - ✅ Actively maintained
  - ✅ Good API design
  - ⚠️ Relatively new (verify stability)
  - **crates.io**: https://crates.io/crates/docx-rs

- **docx** v0.3+
  - ⚠️ Less actively maintained
  - ⚠️ More limited feature set
  - **crates.io**: https://crates.io/crates/docx

- **Manual XML parsing** (quick-xml + ZIP)
  - ✅ Full control
  - ❌ Requires implementing OOXML spec ourselves
  - ❌ High complexity

**Decision**: **docx-rs** (pending verification)

**Rationale**:
- Most complete DOCX parsing solution in Rust ecosystem
- Handles formatting metadata (FR-004 requirement)
- Supports images (metadata extraction for Edge Case)
- Actively maintained with recent commits

**Action Required**: Verify docx-rs stability with test documents before final commitment

**Python Mapping**: `python-docx` → `docx-rs`

---

### 5. File Type Detection

**Requirement**: Auto-detect file format from extension or content (FR-024)

**Candidates**:
- **infer** v0.15+
  - ✅ Magic number detection (fast)
  - ✅ Supports common formats
  - ✅ Minimal dependencies
  - ✅ Pure Rust
  - **crates.io**: https://crates.io/crates/infer

- **tree_magic** v0.2+
  - ✅ FreeDesktop.org MIME database
  - ❌ More complex
  - ❌ Larger dependency footprint
  - **crates.io**: https://crates.io/crates/tree_magic

- **Extension checking only**
  - ✅ Simplest
  - ❌ Less reliable (renamed files)

**Decision**: **infer** + extension fallback

**Rationale**:
- Magic number detection is more reliable
- Minimal overhead (<1ms per file)
- Fallback to extension for text formats (Markdown, CSV)
- Aligns with NFR-010 (minimize dependencies)

**Python Mapping**: `filetype` → `infer`

---

### 6. Serialization

**Requirement**: JSON serialization (FR-005), general data modeling

**Candidates**:
- **serde** v1.0+ + **serde_json** v1.0+
  - ✅ De facto standard in Rust
  - ✅ Extensive ecosystem
  - ✅ Derive macros (minimal boilerplate)
  - ✅ Performance
  - **crates.io**: https://crates.io/crates/serde

**Decision**: **serde + serde_json**

**Rationale**:
- No viable alternative
- Required for JSON export
- Enables future format support (YAML, TOML, etc.)

**Python Mapping**: `pydantic` → `serde`

---

## Dependency Summary

| Python Library | Rust Crate | Version | Purpose |
|----------------|------------|---------|---------|
| `marko` | `pulldown-cmark` | 0.9+ | Markdown parsing |
| `beautifulsoup4` | `scraper` | 0.17+ | HTML parsing |
| `pandas` (CSV) | `csv` | 1.3+ | CSV parsing |
| `python-docx` | `docx-rs` | 0.4+ | DOCX parsing |
| `filetype` | `infer` | 0.15+ | File type detection |
| `pydantic` | `serde` + `serde_json` | 1.0+ | Serialization |

**Additional Dependencies**:
- `quick-xml` (potentially for DOCX if needed)
- `zip` (for DOCX file handling - likely included by docx-rs)

**Total Dependency Count**: ~6-8 crates (meets NFR-010: minimize dependencies)

---

## Open Questions (Resolved)

1. ✅ **DOCX crate stability**: Use docx-rs, verify with integration tests
2. ✅ **GFM support needed?**: No for MVP, can add comrak later if needed
3. ✅ **File detection strategy**: Magic numbers (infer) + extension fallback

---

## Next Steps (Phase 1)

1. Create detailed data model design (`data-model.md`)
2. Define backend contracts (`contracts/`)
3. Design API surface (quickstart examples)
4. Verify all dependencies compatible (check versions, licenses)

---

**Research Status**: ✅ COMPLETE
**Unknowns Remaining**: None
**Ready for Phase 1**: Yes
