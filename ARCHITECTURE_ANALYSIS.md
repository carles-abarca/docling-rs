# Docling Architecture Analysis

**Date**: 2025-10-04
**Source**: docling-original/ (Python version 2.55.1)
**Purpose**: Understanding the architecture for Rust port

## Architecture Overview

### High-Level Flow

```
DocumentConverter
    ↓
InputDocument → Backend (format-specific) → Pipeline → DoclingDocument
                    ↓                           ↓
                Validates                   Builds/Assembles/Enriches
                Loads Format                     ↓
                                            ConversionResult
```

### Core Components

#### 1. **DocumentConverter** (`document_converter.py`)
- Main entry point for document conversion
- Manages format detection and routing
- Handles concurrent conversion of multiple documents
- Maps input formats to appropriate backends and pipelines

#### 2. **Backends** (`backend/`)
- **Abstract Base**: `AbstractDocumentBackend`
  - `DeclarativeDocumentBackend`: For formats that can convert directly to DoclingDocument
  - `PaginatedDocumentBackend`: For formats with page concept (PDF, Excel, PowerPoint)

- **Supported Formats**:
  - **PDF**: `DoclingParseV4DocumentBackend` (complex pipeline with layout analysis)
  - **DOCX**: `MsWordDocumentBackend` (declarative)
  - **XLSX**: `MsExcelDocumentBackend` (declarative + paginated)
  - **PPTX**: `MsPowerpointDocumentBackend` (declarative + paginated)
  - **HTML**: `HTMLDocumentBackend` (declarative)
  - **Markdown**: `MarkdownDocumentBackend` (declarative)
  - **CSV**: `CsvDocumentBackend` (declarative)
  - **AsciiDoc**: `AsciiDocBackend` (declarative)
  - **XML**: `JatsDocumentBackend`, `PatentUsptoDocumentBackend`
  - **WebVTT**: `WebVTTDocumentBackend`
  - **Audio**: `NoOpBackend` + ASR pipeline

#### 3. **Pipelines** (`pipeline/`)
- **SimplePipeline**: For declarative backends (direct conversion)
- **StandardPdfPipeline**: Complex pipeline for PDF (layout analysis, table detection, etc.)
- **AsrPipeline**: Audio transcription pipeline
- **VlmPipeline**: Vision-Language Model pipeline for advanced PDF understanding

Pipeline stages:
1. **Build**: Extract structure from document
2. **Assemble**: Organize elements into hierarchy
3. **Enrich**: Add metadata, classifications, etc.

#### 4. **Data Model** (`datamodel/`)
- **InputDocument**: Input file representation
- **ConversionResult**: Contains input + output + metadata
- **DoclingDocument**: Unified output format (from `docling_core`)

### Text Extraction Strategy

#### Simple Formats (Declarative Backends)
For DOCX, XLSX, PPTX, HTML, Markdown, CSV:
1. Backend loads document using format-specific library
2. Backend directly produces `DoclingDocument`
3. SimplePipeline passes through + enrichment

#### Complex Formats (PDF, Images)
1. Backend loads document
2. StandardPdfPipeline:
   - Page-by-page processing
   - Layout analysis (detect text blocks, tables, figures)
   - OCR for scanned pages
   - Table structure recognition
   - Reading order determination
3. Assembly into DoclingDocument

### Chunking Architecture

Located in `chunking/__init__.py` (wrapper around `docling_core`):

```python
from docling_core.transforms.chunker.base import BaseChunk, BaseChunker, BaseMeta
from docling_core.transforms.chunker.hierarchical_chunker import (
    DocChunk, DocMeta, HierarchicalChunker
)
from docling_core.transforms.chunker.hybrid_chunker import HybridChunker
```

**Chunking Strategy**:
- **HierarchicalChunker**: Respects document structure (sections, paragraphs)
- **HybridChunker**: Combines semantic and size-based chunking
- **BaseChunker**: Abstract interface for custom chunkers

## Python Dependencies Analysis

### Core Dependencies (MUST port to Rust)

| Python Package | Purpose | Rust Equivalent | Priority |
|---------------|---------|-----------------|----------|
| `docling-core` | Data models, chunkers | **PORT IN-HOUSE** | HIGH |
| `docling-parse` | PDF parsing engine | `pdfium-render` or `pdf` | HIGH |
| `pypdfium2` | PDF rendering | `pdfium-render` | HIGH |
| `python-docx` | DOCX parsing | `docx-rs` or `docx` | HIGH |
| `python-pptx` | PPTX parsing | **RESEARCH** | MEDIUM |
| `openpyxl` | XLSX parsing | `calamine` or `rust_xlsxwriter` | MEDIUM |
| `beautifulsoup4` | HTML parsing | `scraper` or `html5ever` | MEDIUM |
| `lxml` | XML parsing | `quick-xml` or `roxmltree` | MEDIUM |
| `pillow` | Image processing | `image` | MEDIUM |
| `pandas` | Data manipulation | `polars` | LOW |
| `marko` | Markdown parsing | `pulldown-cmark` or `comrak` | MEDIUM |
| `pydantic` | Data validation | `serde` + validation crate | HIGH |
| `typer` | CLI framework | `clap` | HIGH |
| `requests` | HTTP client | `reqwest` | LOW |
| `filetype` | File type detection | `infer` or `tree_magic` | MEDIUM |

### ML/OCR Dependencies (DEFER for MVP)

| Python Package | Purpose | Rust Strategy |
|---------------|---------|---------------|
| `easyocr` | OCR engine | **DEFER** - Use external OCR service |
| `transformers` | VLM models | **DEFER** - Optional feature |
| `accelerate` | ML acceleration | **DEFER** |
| `huggingface_hub` | Model downloads | **DEFER** |
| `scipy` | Scientific computing | **DEFER** or `nalgebra` |

### Optional Dependencies (Not in MVP)

- `tesserocr`, `ocrmac`, `rapidocr`: Alternative OCR engines
- `openai-whisper`: Audio transcription
- `mlx-vlm`, `vllm`: VLM inference engines

## Port Strategy Recommendations

### Phase 1: Core Library (Text Extraction - Simple Formats)
**Focus**: DOCX, Markdown, HTML, CSV
- Implement `AbstractBackend` trait
- Implement `DeclarativeBackend` trait
- Create backends for simple formats
- Implement `DoclingDocument` data model
- Implement `SimplePipeline`

**Rust Crates**:
- `docx-rs` or `docx` for DOCX
- `pulldown-cmark` or `comrak` for Markdown
- `scraper` for HTML
- `csv` for CSV
- `serde` + `serde_json` for data models

### Phase 2: Chunking
**Focus**: HierarchicalChunker, HybridChunker
- Port `docling_core` chunking logic
- Implement `BaseChunker` trait
- Implement chunking strategies

**Rust Crates**:
- Custom implementation based on `DoclingDocument` structure
- Possibly use `unicode-segmentation` for text splitting

### Phase 3: PDF Support (Complex)
**Focus**: PDF text extraction (without ML)
- Implement PDF backend using `pdfium-render`
- Basic text extraction without layout analysis
- Simple page-by-page processing

**Rust Crates**:
- `pdfium-render` or `pdf`
- `image` for embedded images

### Phase 4: Advanced PDF (Optional)
**Focus**: Layout analysis, table detection
- Port or integrate layout analysis
- OCR integration (external service)
- Table structure recognition

**Deferred**: ML models, VLM pipelines

## File Structure Insights

### Backend Interface Pattern
All backends implement:
```python
class AbstractDocumentBackend(ABC):
    def __init__(self, in_doc: InputDocument, path_or_stream: Union[BytesIO, Path])
    def is_valid(self) -> bool
    def supports_pagination(cls) -> bool
    def supported_formats(cls) -> Set[InputFormat]
    def unload(self)

class DeclarativeDocumentBackend(AbstractDocumentBackend):
    def convert(self) -> DoclingDocument  # Direct conversion
```

### Pipeline Interface Pattern
All pipelines implement:
```python
class BasePipeline(ABC):
    def execute(self, in_doc: InputDocument, raises_on_error: bool) -> ConversionResult
    def _build_document(self, conv_res: ConversionResult) -> ConversionResult
    def _assemble_document(self, conv_res: ConversionResult) -> ConversionResult
    def _enrich_document(self, conv_res: ConversionResult) -> ConversionResult
```

## Key Observations

1. **Clean Separation**: Backend (format loading) vs Pipeline (processing)
2. **Declarative Pattern**: Simple formats convert directly to unified model
3. **Extensible**: New formats = new backend + reuse existing pipeline
4. **DoclingDocument**: Central unified representation (from `docling_core`)
5. **Chunking**: Separate concern, operates on `DoclingDocument`
6. **CLI**: Uses `typer` for command-line interface

## Rust Port Architecture Proposal

```
docling-rs/
├── src/
│   ├── lib.rs                 # Public API
│   ├── backend/
│   │   ├── mod.rs
│   │   ├── traits.rs          # Backend traits
│   │   ├── docx.rs            # DOCX backend
│   │   ├── markdown.rs        # Markdown backend
│   │   ├── html.rs            # HTML backend
│   │   └── pdf.rs             # PDF backend (phase 3)
│   ├── pipeline/
│   │   ├── mod.rs
│   │   ├── traits.rs          # Pipeline traits
│   │   ├── simple.rs          # Simple pipeline
│   │   └── pdf.rs             # PDF pipeline (phase 3)
│   ├── datamodel/
│   │   ├── mod.rs
│   │   ├── document.rs        # DoclingDocument
│   │   ├── input.rs           # InputDocument
│   │   └── result.rs          # ConversionResult
│   ├── chunking/
│   │   ├── mod.rs
│   │   ├── traits.rs          # Chunker traits
│   │   ├── hierarchical.rs    # HierarchicalChunker
│   │   └── hybrid.rs          # HybridChunker
│   ├── converter.rs           # DocumentConverter
│   └── cli/
│       └── main.rs            # CLI entry point
├── tests/
│   ├── integration/
│   └── contract/
└── examples/

```

## Next Steps

1. Create feature branch: `001-core-architecture`
2. Use `/specify` to create specification for core data models
3. Implement `DoclingDocument` and related types
4. Implement backend traits
5. Start with Markdown backend (simplest)
6. Implement simple pipeline
7. Add CLI
8. Add chunking support
