# Phase 0: Research - Document Chunking Dependencies

**Date**: 2025-10-04
**Feature**: Phase 2 - Document Chunking System

## Research Questions

1. **Tokenization**: How to implement HuggingFace-compatible tokenization in native Rust?
2. **Sentence Boundary Detection**: How to detect sentence boundaries for text processing?
3. **Token Counting**: How to count tokens for different embedding models (sentence-transformers, OpenAI)?

---

## 1. Tokenization in Rust

### Decision: Use `tokenizers` crate (HuggingFace Rust library)

**Rationale**:
- Official HuggingFace tokenizers library with native Rust implementation
- Used internally by HuggingFace transformers (Python library uses Rust bindings)
- Supports loading pretrained tokenizers from HuggingFace Hub
- Zero Python dependencies (pure Rust)
- Actively maintained by HuggingFace team
- Performance: 10-100x faster than Python tokenizers

**Alternatives Considered**:
1. **tiktoken-rs** (Rust port of OpenAI tiktoken)
   - Pros: Native Rust, supports OpenAI models (GPT-3.5, GPT-4)
   - Cons: OpenAI-specific, doesn't support HuggingFace models (sentence-transformers)
   - Verdict: Use for OpenAI tokenizer support, but `tokenizers` is primary

2. **Implement custom BPE tokenizer**
   - Pros: Full control, minimal dependencies
   - Cons: Reinventing the wheel, won't match embedding model tokenizers exactly
   - Verdict: Rejected - violates YAGNI, `tokenizers` crate already exists

3. **rust-tokenizers** (older community crate)
   - Pros: Pure Rust
   - Cons: Less maintained, smaller feature set than HuggingFace `tokenizers`
   - Verdict: Rejected - `tokenizers` is official and better maintained

**Implementation Strategy**:
```rust
// Use tokenizers crate
[dependencies]
tokenizers = "0.15"  // Latest stable version

// Example usage (from research):
use tokenizers::Tokenizer;

let tokenizer = Tokenizer::from_pretrained("sentence-transformers/all-MiniLM-L6-v2", None)?;
let encoding = tokenizer.encode("text to tokenize", false)?;
let token_count = encoding.get_ids().len();
```

**Compatibility with docling-original**:
- Python docling uses `transformers.AutoTokenizer` → loads from HuggingFace Hub
- Rust `tokenizers` crate can load same models → compatible approach
- Both use same underlying Rust library (Python wraps the Rust implementation)

---

## 2. Sentence Boundary Detection

### Decision: Use `unicode-segmentation` + custom heuristics

**Rationale**:
- Unicode-aware sentence boundary detection
- Standard Rust crate (maintained by rust-lang-nursery)
- Lightweight, no ML models required
- Good enough for most document chunking use cases
- Can be enhanced later if needed

**Alternatives Considered**:
1. **srx** (Segmentation Rules eXchange)
   - Pros: Rule-based, language-specific
   - Cons: Requires rule files, more complex setup
   - Verdict: Rejected - overkill for initial implementation

2. **Simple regex-based splitting**
   - Pros: Minimal dependencies
   - Cons: Doesn't handle edge cases (abbreviations, quotes, etc.)
   - Verdict: Rejected - too simplistic, will cause bad chunking

3. **ML-based NLP models** (e.g., spaCy equivalent in Rust)
   - Pros: Most accurate
   - Cons: Heavy dependencies, slower, requires model files
   - Verdict: Rejected - violates simplicity principle, not needed for v1

**Implementation Strategy**:
```rust
[dependencies]
unicode-segmentation = "1.11"

// Use UAX#29 sentence boundaries + custom heuristics:
use unicode_segmentation::UnicodeSegmentation;

fn split_sentences(text: &str) -> Vec<&str> {
    // UAX#29 provides basic sentence boundary detection
    text.unicode_sentences().collect()
}

// Enhancement: Handle common abbreviations
// "Dr. Smith" should not split, "End. Start" should split
```

**Paragraph Detection**:
- Use double newline `\n\n` as paragraph delimiter (Markdown convention)
- Trim whitespace to handle varying newline counts
- Simple and effective for structured documents

---

## 3. Token Counting for Embedding Models

### Decision: Trait-based abstraction with model-specific implementations

**Rationale**:
- Different embedding models use different tokenizers
- sentence-transformers: WordPiece tokenizer (BERT-based)
- OpenAI: BPE tokenizer (tiktoken)
- Need flexibility to support both

**Implementation Strategy**:
```rust
// Base trait
pub trait Tokenizer {
    fn count_tokens(&self, text: &str) -> usize;
    fn max_tokens(&self) -> usize;
}

// HuggingFace implementation
pub struct HuggingFaceTokenizer {
    tokenizer: tokenizers::Tokenizer,
    max_tokens: usize,
}

impl Tokenizer for HuggingFaceTokenizer {
    fn count_tokens(&self, text: &str) -> usize {
        self.tokenizer.encode(text, false)
            .map(|enc| enc.get_ids().len())
            .unwrap_or(0)
    }

    fn max_tokens(&self) -> usize {
        self.max_tokens
    }
}

// OpenAI implementation (future)
pub struct OpenAITokenizer {
    // tiktoken-rs integration
}
```

**Default Tokenizer**:
- Use `sentence-transformers/all-MiniLM-L6-v2` as default
- Small, fast, widely used for RAG applications
- Max tokens: 512 (model limit)

---

## Dependency Summary

### New Dependencies to Add

```toml
[dependencies]
# Existing from Phase 1
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# New for Phase 2
tokenizers = "0.15"              # HuggingFace tokenizers (primary)
unicode-segmentation = "1.11"    # Sentence boundary detection

# Optional (for OpenAI support in future)
# tiktoken-rs = "0.5"           # OpenAI tokenizer
```

### Cargo Features Strategy

```toml
[features]
default = ["tokenizers-hf"]
tokenizers-hf = ["dep:tokenizers"]       # HuggingFace tokenizers
tokenizers-openai = ["dep:tiktoken-rs"]  # OpenAI tokenizers (future)
```

---

## Python docling Dependency Mapping

| Python Library | Functionality | Rust Equivalent | Status |
|----------------|---------------|-----------------|--------|
| `transformers.AutoTokenizer` | Load HF tokenizers | `tokenizers::Tokenizer` | ✅ Resolved |
| `tiktoken` | OpenAI tokenizers | `tiktoken-rs` (optional) | ✅ Resolved |
| N/A (uses nltk/spacy) | Sentence detection | `unicode-segmentation` | ✅ Resolved |

---

## Open Questions & Decisions

### Q1: Should we download tokenizer models at runtime or bundle them?

**Decision**: Download at runtime (similar to Python docling)
- Use `tokenizers::Tokenizer::from_pretrained()` to download from HuggingFace Hub
- Cache models in user's home directory (XDG cache on Linux/macOS, AppData on Windows)
- Rationale: Tokenizer files can be large (few MB), bundling would bloat binary

### Q2: How to handle offline scenarios?

**Decision**: Support pre-cached tokenizers, fail gracefully if not available
- If tokenizer not in cache and no network: return clear error message
- Recommend users to pre-download tokenizers for offline use
- Future enhancement: Bundle minimal default tokenizer

### Q3: What's the fallback if tokenizer loading fails?

**Decision**: Character-based estimation as fallback
- Heuristic: ~4 characters per token for English (common approximation)
- Log warning that exact token counting unavailable
- Rationale: Allows graceful degradation, better than failing completely

---

## Constitutional Compliance

✅ **VII. Native Rust Dependencies**: All dependencies are native Rust crates
- `tokenizers` - Pure Rust (HuggingFace official)
- `unicode-segmentation` - Pure Rust (rust-lang-nursery)
- `tiktoken-rs` - Pure Rust (community port)

✅ **VI. Cross-Platform**: All crates are cross-platform
- Tested on Windows, macOS, Linux
- No platform-specific code required

✅ **V. Rust Best Practices**: Using well-maintained crates from reputable sources
- HuggingFace official library
- rust-lang-nursery for Unicode handling

---

## Next Steps (Phase 1)

1. Add dependencies to `Cargo.toml`
2. Define `Tokenizer` trait in `src/chunking/tokenizer/base.rs`
3. Implement `HuggingFaceTokenizer` wrapper
4. Implement sentence/paragraph detection utilities
5. Write contract tests for tokenizer abstraction

---

**Research Complete**: 2025-10-04
**Status**: All NEEDS CLARIFICATION resolved ✅
**Ready for**: Phase 1 (Design & Contracts)
