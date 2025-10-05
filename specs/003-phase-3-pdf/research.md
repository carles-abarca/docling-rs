# Research: Advanced PDF Processing (Phase 3)

**Date**: 2025-10-05
**Feature**: Advanced PDF Processing with Layout Analysis, Tables, OCR
**Branch**: 003-phase-3-pdf

## Overview

Research findings for implementing advanced PDF processing capabilities in docling-rs using native Rust libraries. This research investigated Python docling's architecture and identified equivalent Rust solutions for all core PDF features.

## 1. PDF Parsing Library

### Decision: pdfium-render

**Rationale**:
- Official Rust bindings to Google's Pdfium library
- Cross-platform support (Windows, macOS, Linux)
- Comprehensive API: text extraction with positions, page rendering, metadata, encryption
- Active maintenance and community support
- Used successfully in production Rust applications

**Alternatives Considered**:
1. **pdf crate**: Lower-level, less feature-complete, lacks some advanced features
2. **lopdf**: Very low-level, would require significant custom implementation
3. **mupdf-rs**: Bindings to MuPDF, less mature Rust integration

**API Coverage**:
- ✅ Text extraction with character positions and font information
- ✅ Page rendering to bitmaps with configurable resolution
- ✅ Metadata extraction (title, author, dates, etc.)
- ✅ Password/encryption support
- ✅ Image extraction from PDF pages
- ✅ Form field detection (bonus feature)

**Integration Plan**:
- Use `pdfium-render` v0.8+ for stable API
- Wrap in `PdfDocument` and `PdfPage` abstractions
- Handle thread safety (Pdfium requires locking for concurrent access)

## 2. ML Model Inference

### Decision: tract (ONNX Runtime for Rust)

**Rationale**:
- Production-ready ONNX model runtime in pure Rust
- CPU-only inference (no GPU dependency)
- Good performance with optimized operations
- Used by Hugging Face's Rust projects
- Supports common model architectures (CNN, transformers)

**Alternatives Considered**:
1. **candle**: Newer ML framework from HuggingFace, less battle-tested
2. **burn**: Still maturing, API unstable
3. **tch-rs**: PyTorch bindings, introduces C++ dependency

**Model Support**:
- ✅ Layout analysis models (object detection, segmentation)
- ✅ Table detection models (YOLO, faster R-CNN)
- ✅ Image classification models
- ✅ Reading order prediction models

**Integration Plan**:
- Convert Python docling models to ONNX format
- Load models lazily (only when needed)
- Provide fallback to rule-based approaches if model unavailable
- Cache model instances for performance

## 3. OCR Engine

### Decision: tesseract-rs

**Rationale**:
- Rust bindings to industry-standard Tesseract OCR
- Proven accuracy across languages
- Wide language model availability
- Confidence scores per word/line
- Active maintenance

**Alternatives Considered**:
1. **leptess**: Alternative Tesseract bindings, similar features
2. **ocrs**: Pure Rust OCR, not mature enough for production
3. **paddle-ocr-rs**: Good accuracy but larger dependency footprint

**Requirements**:
- System dependency: Tesseract library must be installed
- Language data files for target languages
- Windows: via vcpkg or manual install
- macOS: via Homebrew (`brew install tesseract`)

**Integration Plan**:
- Wrap in `OcrEngine` trait for swappable implementations
- Auto-detect if PDF requires OCR (check for text layer)
- Parallel OCR processing per page
- Return confidence scores with all results

## 4. Layout Analysis Strategy

### Decision: Hybrid Approach (Rule-based + Optional ML)

**Phase 1 - Rule-Based Layout Analysis**:
- Column detection via whitespace analysis
- Reading order via geometric sorting (top-to-bottom, left-to-right)
- Block classification via font size/style patterns
- Margin and zone detection

**Phase 2 - ML-Enhanced (Optional)**:
- Load layout analysis model (LayoutLMv3 or similar via ONNX)
- Classify regions: heading, paragraph, caption, footer, etc.
- Predict reading order for complex multi-column layouts
- Confidence scores for all predictions

**Rationale**:
- Rule-based provides immediate value without model dependency
- ML models improve accuracy for complex documents
- Incremental adoption: start simple, add ML when needed

**Implementation**:
```rust
pub trait LayoutAnalyzer {
    fn analyze(&self, page: &PdfPage) -> Result<LayoutInfo>;
}

pub struct RuleBasedLayoutAnalyzer { /* ... */ }
pub struct MlLayoutAnalyzer { /* ... */ }
```

## 5. Table Detection

### Decision: Hybrid Rule-Based + ML

**Rule-Based Detection** (Phase 1):
- Detect grid lines and aligned text blocks
- Identify cell boundaries by whitespace/borders
- Handle simple merged cells
- Extract table structure (rows, cols, headers)

**ML-Enhanced Detection** (Phase 2):
- Table region detection model (faster R-CNN or YOLO)
- Structure recognition for borderless tables
- Complex merged cell handling

**Rationale**:
- Grid-based detection works for 80% of tables
- ML handles edge cases (borderless, complex layouts)
- Python docling uses similar hybrid approach

**Integration**:
```rust
pub trait TableDetector {
    fn detect(&self, page: &PdfPage) -> Result<Vec<Table>>;
}

pub struct GridBasedDetector { /* rule-based */ }
pub struct MlTableDetector { /* ML model */ }
```

## 6. Image Processing

### Decision: image crate

**Rationale**:
- De facto standard for image processing in Rust
- Comprehensive format support (PNG, JPEG, TIFF, etc.)
- Image manipulation operations (resize, crop, convert)
- Well-maintained with stable API

**Features Used**:
- Extract image regions from PDF (via pdfium-render)
- Convert to standard formats for saving/analysis
- Basic classification (heuristics): photo vs diagram vs logo
- Bitmap rendering for ML model input

**Integration Plan**:
```rust
pub struct ImageExtractor {
    // Extract images from PDF pages
    pub fn extract(&self, page: &PdfPage) -> Result<Vec<ImageRegion>>;

    // Classify image type (heuristic-based initially)
    pub fn classify(&self, image: &ImageRegion) -> ImageType;
}
```

## 7. Architecture Design

### PDF Processing Pipeline

```
Input PDF
    ↓
PdfBackend::convert()
    ↓
1. Load PDF (pdfium-render)
    ↓
2. Extract Text + Positions
    ↓
3. Analyze Layout (rule-based or ML)
    ↓
4. Detect Tables (hybrid)
    ↓
5. Extract Images
    ↓
6. Run OCR (if needed)
    ↓
7. Enrich Content (code, formulas)
    ↓
8. Map to DoclingDocument
    ↓
Output: DoclingDocument
```

### Modular Components

```rust
// Core abstractions
pub trait LayoutAnalyzer { ... }
pub trait TableDetector { ... }
pub trait OcrEngine { ... }
pub trait ContentEnricher { ... }

// Implementations
pub struct RuleBasedLayoutAnalyzer;
pub struct MlLayoutAnalyzer;
pub struct GridBasedTableDetector;
pub struct MlTableDetector;
pub struct TesseractOcr;
pub struct PatternBasedEnricher;

// Configuration
pub struct PdfConfig {
    pub password: Option<String>,
    pub page_range: Option<Range<usize>>,
    pub enable_ocr: bool,
    pub enable_tables: bool,
    pub enable_images: bool,
    pub layout_analyzer: Box<dyn LayoutAnalyzer>,
    pub table_detector: Box<dyn TableDetector>,
    pub ocr_engine: Option<Box<dyn OcrEngine>>,
}
```

## 8. Performance Considerations

**Memory Management**:
- Process PDFs page-by-page (streaming)
- Lazy load ML models
- Release Pdfium resources after page processing
- Limit bitmap resolution for large pages

**Parallelization**:
- OCR can be parallelized per page
- Table detection independent per page
- Use rayon for parallel iterators

**Caching**:
- Cache loaded ML models
- Cache Pdfium document handles (with locking)
- Memoize expensive layout calculations

## 9. Dependency Matrix

| Python (docling) | Rust (docling-rs) | Purpose |
|------------------|-------------------|---------|
| pypdfium2 | pdfium-render | PDF parsing |
| docling-parse | (in-house) | Custom parsing rules |
| torch/transformers | tract | ML inference |
| pytesseract | tesseract-rs | OCR |
| Pillow | image | Image processing |
| numpy | ndarray | Array operations |
| pydantic | serde | Serialization |

## 10. Risk Mitigation

**Risk 1: ML Model Complexity**
- Mitigation: Start with rule-based, add ML incrementally
- Fallback: Always provide non-ML alternative

**Risk 2: OCR Dependency**
- Mitigation: Make OCR optional, detect automatically
- Fallback: Return empty text for scanned PDFs if OCR unavailable

**Risk 3: Cross-Platform Issues**
- Mitigation: CI/CD tests on Windows and macOS
- pdfium-render handles platform specifics

**Risk 4: Performance**
- Mitigation: Benchmark against Python docling
- Optimize hot paths, use profiling tools

## 11. Testing Strategy

**Contract Tests**:
- Backend trait compliance
- API contract adherence
- Error handling verification

**Integration Tests**:
- Real PDF samples (academic, business, scanned)
- Multi-column layout accuracy
- Table extraction correctness
- OCR quality on scanned docs

**Benchmarks**:
- Processing speed vs Python docling
- Memory usage profiling
- Accuracy metrics (precision/recall for tables, layout)

## 12. Implementation Phases

**Phase 3a: Foundation** (Week 1)
- pdfium-render integration
- Basic text extraction with positions
- Password handling
- DoclingDocument mapping

**Phase 3b: Layout Analysis** (Week 2)
- Rule-based layout analyzer
- Column detection
- Reading order determination

**Phase 3c: Table Detection** (Week 3)
- Grid-based table detector
- Cell boundary detection
- Structure extraction

**Phase 3d: Image Processing** (Week 4)
- Image extraction via pdfium
- Format conversion
- Basic classification

**Phase 3e: OCR Integration** (Week 5)
- tesseract-rs wrapper
- Scanned PDF detection
- Confidence scoring

**Phase 3f: Content Enrichment** (Week 6)
- Code block detection
- Formula detection
- Final integration

## Summary

All PDF processing requirements can be met with native Rust libraries:
- ✅ pdfium-render for PDF parsing
- ✅ tract for ML inference
- ✅ tesseract-rs for OCR
- ✅ image for image processing
- ✅ Hybrid approach (rules + ML) for layout and tables
- ✅ Incremental implementation from simple to advanced features

No Python dependencies required. All choices align with Constitution Principle VII (Native Rust Dependencies).
