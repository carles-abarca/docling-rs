# Markdown Backend Contract

## Conversion Rules

### Headings
- `#` → NodeType::Heading { level: 1 }
- `##` → NodeType::Heading { level: 2 }
- Up to `######` → level 6

### Paragraphs
- Text blocks → NodeType::Paragraph with TextItem

### Lists
- `- item` → NodeType::List { ordered: false }
- `1. item` → NodeType::List { ordered: true }
- Each item → NodeType::ListItem

### Code Blocks
```
\`\`\`rust
code
\`\`\`
```
→ NodeType::CodeBlock { language: Some("rust") }

### Inline Formatting
- `**bold**` → Formatting { bold: true }
- `*italic*` → Formatting { italic: true }
- `[text](url)` → Formatting { link: Some(url) }

### Tables (if using comrak/GFM)
- Deferred to future phase or best-effort

## Test Cases
1. Document with all heading levels
2. Mixed lists (ordered + unordered)
3. Code blocks with/without language
4. Inline formatting combinations
5. Edge: Empty document → empty root node

**Backend**: MarkdownBackend
**Crate**: pulldown-cmark
