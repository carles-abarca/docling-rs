# CSV Backend Contract

## Conversion Rules

### Table Structure
- First row → TableData.headers (if has_header = true)
- Remaining rows → TableData.rows
- Each cell → TableCell with plain text (no formatting)

### Inconsistent Columns
- Pad short rows with empty cells
- Truncate long rows to header count (with warning)

### Output Structure
```
DoclingDocument
└── root: NodeItem
    └── children: [NodeItem]
        └── table: Some(TableData)
```

## Test Cases
1. CSV with headers
2. CSV without headers
3. Inconsistent column counts (Edge Case)
4. Empty CSV
5. CSV with quoted fields

**Backend**: CsvBackend
**Crate**: csv
