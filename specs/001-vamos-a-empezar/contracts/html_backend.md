# HTML Backend Contract

## Conversion Rules

### Semantic Mapping
- `<h1>` - `<h6>` → NodeType::Heading { level: 1-6 }
- `<p>` → NodeType::Paragraph
- `<ul>` → NodeType::List { ordered: false }
- `<ol>` → NodeType::List { ordered: true }
- `<li>` → NodeType::ListItem
- `<table>` → NodeType::Table with TableData
- `<blockquote>` → NodeType::Blockquote
- `<hr>` → NodeType::HorizontalRule

### Tables
- `<thead>` → headers in TableData
- `<tbody>` → rows in TableData
- `<td colspan="2">` → TableCell { colspan: 2 }

### Inline Formatting
- `<strong>`, `<b>` → bold
- `<em>`, `<i>` → italic
- `<u>` → underline
- `<a href>` → link

### Malformed HTML
- Use scraper's error recovery
- Return partial results with warnings
- Don't fail on unclosed tags

## Test Cases
1. Well-formed semantic HTML
2. Table with colspan/rowspan
3. Malformed HTML (Edge Case)
4. Nested lists
5. Mixed inline formatting

**Backend**: HtmlBackend
**Crate**: scraper
