# Feature Specification: CLI Manual Testing Script with Real Documents

**Feature Branch**: `006-cli-manual-testing`
**Created**: 2025-10-05
**Status**: Draft
**Input**: User description: "CLI manual testing script with real documents"

## Execution Flow (main)
```
1. Parse user description from Input
   → Create manual testing infrastructure for CLI
2. Extract key concepts from description
   → Actors: Developers testing CLI functionality
   → Actions: Execute CLI on real documents, display results
   → Data: Test documents (PDF, DOCX, MD, JSON, TXT, etc.)
   → Constraints: Use release build for performance testing
3. For each unclear aspect:
   → [CLARIFIED] Test documents location: tests/documents-test/
   → [CLARIFIED] Output format: Display to screen with clear formatting
   → [CLARIFIED] Execution mode: Batch script processing all test files
4. Fill User Scenarios & Testing section
   → Manual testing workflow with real-world documents
5. Generate Functional Requirements
   → Each requirement is testable by running the script
6. Identify Key Entities
   → Test script, test documents, CLI executable
7. Run Review Checklist
   → No implementation details in spec
8. Return: SUCCESS (spec ready for planning)
```

---

## ⚡ Quick Guidelines
- ✅ Focus on WHAT users need and WHY
- ❌ Avoid HOW to implement (no tech stack, APIs, code structure)
- 👥 Written for business stakeholders, not developers

---

## User Scenarios & Testing *(mandatory)*

### Primary User Story
As a **developer** working on docling-rs, I need to **manually test the CLI with real-world documents** so that I can **verify text extraction and chunking work correctly across different file formats** before releasing the software.

### Acceptance Scenarios
1. **Given** a collection of real test documents (PDF, DOCX, MD, JSON, TXT, YAML, etc.), **When** I run the manual testing script, **Then** the CLI should process each document and display the extracted text and chunks on screen
2. **Given** the manual testing script, **When** I execute it, **Then** it should show clear output for each file including filename, format, extraction status, and chunk information
3. **Given** a test document that fails processing, **When** the script encounters it, **Then** the error should be displayed clearly but processing should continue with remaining files
4. **Given** the CLI release build, **When** the script runs, **Then** it should use the optimized release binary for realistic performance testing
5. **Given** different document formats in the test folder, **When** the script processes them, **Then** each format should be tested (MD, PDF, DOCX, TXT, JSON, YAML, etc.)

### Edge Cases
- What happens when a test document is corrupted or unreadable?
  → Script should display error but continue with other files
- How does the script handle unsupported file formats?
  → Script should skip or report unsupported formats clearly
- What if the CLI binary is not built yet?
  → Script should provide clear error message to build first
- How are large documents handled in the output?
  → Output should be paginated or truncated for readability

## Requirements *(mandatory)*

### Functional Requirements
- **FR-001**: System MUST provide a shell script that processes all test documents in tests/documents-test/
- **FR-002**: System MUST invoke the CLI release binary for each test document
- **FR-003**: System MUST display extracted text results on screen for each document
- **FR-004**: System MUST display chunking results on screen for each document
- **FR-005**: System MUST show clear separators between different test files in the output
- **FR-006**: System MUST continue processing remaining files if one file fails
- **FR-007**: System MUST test documents with multiple formats: PDF, DOCX, MD, TXT, JSON, YAML, SH, PY, XLSX, PPTX, JPEG
- **FR-008**: System MUST build the release binary before running tests if not present
- **FR-009**: System MUST display summary statistics at the end (total files, successful, failed)
- **FR-010**: Script MUST be executable and runnable from project root
- **FR-011**: Output MUST include filename, format detection, extraction status for each file
- **FR-012**: System MUST support running script multiple times without side effects
- **FR-013**: System MUST provide clear instructions on how to run the script
- **FR-014**: Script MUST handle both text extraction and chunking for each document
- **FR-015**: Output MUST be human-readable with proper formatting and indentation

### Key Entities
- **Test Document Collection**: Real-world files in various formats (PDF, DOCX, MD, JSON, TXT, YAML, SH, PY, XLSX, PPTX, JPEG) located in tests/documents-test/
- **Testing Script**: Shell script that orchestrates CLI execution across all test documents
- **CLI Release Binary**: Optimized executable built with cargo build --release
- **Test Results Output**: Formatted display showing extraction and chunking results for each document
- **Summary Report**: Final statistics showing success/failure counts and overall status

---

## Review & Acceptance Checklist
*GATE: Automated checks run during main() execution*

### Content Quality
- [x] No implementation details (languages, frameworks, APIs)
- [x] Focused on user value and business needs
- [x] Written for non-technical stakeholders
- [x] All mandatory sections completed

### Requirement Completeness
- [x] No [NEEDS CLARIFICATION] markers remain
- [x] Requirements are testable and unambiguous
- [x] Success criteria are measurable
- [x] Scope is clearly bounded
- [x] Dependencies and assumptions identified

---

## Execution Status
*Updated by main() during processing*

- [x] User description parsed
- [x] Key concepts extracted
- [x] Ambiguities marked (all clarified)
- [x] User scenarios defined
- [x] Requirements generated
- [x] Entities identified
- [x] Review checklist passed

---
