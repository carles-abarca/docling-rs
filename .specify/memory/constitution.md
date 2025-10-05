<!--
Sync Impact Report:
Version: 1.1.0 → 1.2.0
Change type: MINOR (New principle added: Native Rust Dependencies)

Modified principles: None
Added sections:
  - New principle VII: Native Rust Dependencies
  - Enhanced Python Docling Compatibility section with dependency guidance
Removed sections: None

Templates requiring updates:
  ✅ plan-template.md (No structural changes needed - constitution check will include new principle)
  ✅ spec-template.md (No changes needed - tech-agnostic)
  ✅ tasks-template.md (No changes needed - dependency research covered by research phase)
  ✅ commands/*.md (No changes needed)

Follow-up TODOs: None - all placeholders resolved
-->

# Docling-rs Constitution

## Project Purpose

Docling-rs is a native Rust port of the Python docling library, providing high-performance document processing capabilities for cross-platform Rust applications. The project focuses on two core capabilities:

1. **Text Extraction**: Extracting structured text from various document formats
2. **Intelligent Chunking**: Semantic segmentation of documents for downstream processing

This library enables native Rust applications on Windows and macOS to perform document processing without Python runtime dependencies, while maintaining API compatibility and feature parity with the original docling library where applicable.

## Core Principles

### I. Library-First Architecture
Every feature starts as a standalone library with clear boundaries. Libraries MUST be:
- Self-contained with explicit dependencies
- Independently testable without external infrastructure
- Documented with usage examples and API contracts
- Designed with a clear, focused purpose—no organizational-only modules

**Rationale**: Library-first design enforces modularity, enables reuse, and simplifies testing by decoupling business logic from integration concerns.

### II. CLI Interface Contract
Every library MUST expose its functionality through a command-line interface following these rules:
- Input via stdin and/or command-line arguments
- Structured output to stdout (JSON and human-readable formats supported)
- Errors and diagnostics to stderr
- Exit codes follow POSIX conventions (0=success, 1=error, 2=usage error)

**Rationale**: Consistent CLI interfaces enable composability, scriptability, and integration testing without requiring complex harnesses.

### III. Test-Driven Development (NON-NEGOTIABLE)
TDD is mandatory for all code changes:
1. Write tests based on specifications
2. User reviews and approves test coverage
3. Verify tests fail (Red)
4. Implement minimal code to pass (Green)
5. Refactor while maintaining green tests

**Rationale**: TDD ensures specifications are testable, prevents over-engineering, and provides regression protection from the first commit.

### IV. Integration & Contract Testing
Integration tests are REQUIRED for:
- New library public APIs (contract tests)
- Changes to existing contracts (breaking or non-breaking)
- Cross-library communication
- Shared data structures and serialization formats

**Rationale**: Contract tests prevent breaking changes, document expected behavior, and catch integration issues before deployment.

### V. Rust Best Practices
All Rust code MUST adhere to:
- Idiomatic Rust patterns and conventions
- Clippy lints at default level (warnings must be addressed or explicitly allowed with justification)
- Zero unsafe code unless justified in documentation with safety proof
- Error handling via Result<T, E> (no panics in library code)
- Comprehensive documentation for public APIs (rustdoc with examples)

**Rationale**: Rust's type system and ownership model provide strong guarantees when used correctly. Following best practices maximizes safety, performance, and maintainability.

### VI. Cross-Platform Compatibility
All features MUST work correctly on both Windows and macOS without platform-specific workarounds in application code:
- File path handling uses platform-agnostic APIs (std::path)
- Platform differences isolated in abstraction layers
- CI/CD tests run on both target platforms
- Platform-specific code clearly marked and justified
- No hardcoded platform assumptions (path separators, line endings, etc.)

**Rationale**: As a native port replacing Python runtime dependencies, docling-rs must provide consistent behavior across target platforms to be a viable alternative for cross-platform applications.

### VII. Native Rust Dependencies
All dependencies MUST be native Rust crates to eliminate Python runtime requirements:
- For each Python library used by docling, identify equivalent Rust crate(s)
- Prefer mature, well-maintained crates from crates.io with active communities
- Document mapping between Python dependencies and chosen Rust alternatives
- If no suitable Rust alternative exists, implement minimal required functionality in-house
- NEVER use PyO3 or similar Python bindings as primary solution (defeats native port purpose)
- Python bindings MAY be used only for optional compatibility layer or tooling

**Rationale**: The primary value proposition of docling-rs is eliminating Python runtime dependencies. Using native Rust dependencies ensures true portability, better performance, smaller binary size, and simpler deployment across all target platforms.

## Development Standards

### Observability
All code MUST be observable and debuggable:
- Structured logging using standard logging framework (e.g., `tracing`, `log`)
- Log levels used appropriately (error, warn, info, debug, trace)
- CLI text I/O ensures debuggability without special tools
- Critical operations logged with sufficient context

### Versioning & Breaking Changes
- Semantic versioning (MAJOR.MINOR.PATCH) strictly followed
- MAJOR: Breaking API changes, removed public items
- MINOR: New features, backward-compatible additions
- PATCH: Bug fixes, performance improvements, documentation
- Breaking changes MUST include migration guide and deprecation period when possible

### Simplicity & YAGNI
- Start with the simplest solution that could work
- Complexity MUST be justified with concrete requirements
- Abstract only when duplication becomes problematic (Rule of Three)
- Prefer explicit code over clever abstractions

### Python Docling Compatibility
When porting features from Python docling:
- Maintain similar API surface where Rust idioms allow
- Document deviations from Python API with justification
- Prioritize Rust safety and performance over strict API mirroring
- Provide migration examples for users coming from Python docling

### Dependency Selection & Research
For each Python dependency in original docling:
1. **Identify functionality**: Document what the Python library does in context
2. **Research Rust alternatives**: Search crates.io, awesome-rust, and GitHub
3. **Evaluate candidates**: Consider maturity, maintenance, license, features, performance
4. **Document decision**: Record chosen crate and rationale in research.md
5. **Fallback strategy**: If no adequate crate exists, document in-house implementation scope

Common Python-to-Rust dependency mappings to consider:
- PDF processing: `pypdfium2` → `pdfium-render` or `pdf` crates
- Image processing: `Pillow` → `image` crate
- HTTP/networking: `requests` → `reqwest` crate
- JSON/serialization: `pydantic` → `serde` + `serde_json`
- CLI argument parsing: `argparse` → `clap` crate
- Logging: `logging` → `tracing` or `log` + `env_logger`

## Quality Gates

All changes MUST pass:
1. **Compilation**: `cargo check` and `cargo build` succeed on Windows and macOS
2. **Tests**: `cargo test` passes (100% of existing tests) on both platforms
3. **Linting**: `cargo clippy` produces no warnings
4. **Formatting**: `cargo fmt --check` passes
5. **Documentation**: Public APIs have rustdoc comments with examples
6. **Contract Tests**: All contract tests pass before and after changes
7. **Platform Tests**: Platform-specific code tested on target platforms
8. **Dependency Audit**: `cargo deny check` passes (no banned/vulnerable dependencies)

## Governance

### Authority
This constitution supersedes all other development practices and guidelines. Any proposed change that violates these principles MUST either:
- Be rejected and redesigned to comply, OR
- Include a formal justification documented in the feature plan's Complexity Tracking section

### Amendment Process
Amendments to this constitution require:
1. Written proposal documenting the change and rationale
2. Review and approval from project maintainers
3. Migration plan for existing code if applicable
4. Version bump following semantic versioning rules

### Compliance Review
- All pull requests MUST verify constitutional compliance
- Code reviews MUST check adherence to core principles
- Unjustified complexity MUST be rejected in review
- Constitution version referenced in plan.md templates

### Runtime Guidance
For agent-specific development guidance (e.g., Claude Code, GitHub Copilot), refer to the appropriate agent file in the repository root (e.g., `CLAUDE.md`, `.github/copilot-instructions.md`). These files contain incremental context and recent changes that complement this constitution.

**Version**: 1.2.0 | **Ratified**: 2025-10-04 | **Last Amended**: 2025-10-04
