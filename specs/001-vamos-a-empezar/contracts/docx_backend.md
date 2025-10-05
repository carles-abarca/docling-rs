# DOCX Backend Contract

## Conversion Rules

### Paragraphs
- DOCX paragraph → NodeType::Paragraph with TextItem
- Preserve formatting (bold, italic, underline)

### Tables
- DOCX table → NodeType::Table with TableData
- Support merged cells (colspan/rowspan)

### Lists
- Numbered lists → NodeType::List { ordered: true }
- Bullet lists → NodeType::List { ordered: false }

### Images
- Extract metadata only (Edge Case requirement)
- Image refs in NodeMetadata.extra
- Actual image data not extracted

### Styles/Formatting
- Bold, italic, underline → Formatting
- Hyperlinks → Formatting { link: Some(url) }

## Test Cases
1. Mixed content (text, tables, lists)
2. DOCX with embedded images (metadata only)
3. Complex formatting
4. Tables with merged cells
5. Corrupted DOCX (Edge Case)

**Backend**: DocxBackend
**Crate**: docx-rs
