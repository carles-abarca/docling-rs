# TecGPT Desktop Testing Framework

A comprehensive testing suite for the TecGPT Desktop application, designed to validate AI-powered document indexing and search functionality.

## Overview

This testing framework provides a collection of test documents and utilities to validate the following TecGPT features:

- **Document Indexing**: File system monitoring and content extraction
- **Embeddings Generation**: Vector representation of document content
- **Hybrid Search**: Full-text and semantic search capabilities
- **AI Integration**: Azure OpenAI GPT-4o chat functionality

## Test Document Types

### Supported File Formats

| Format | Extension | Description |
|--------|-----------|-------------|
| Plain Text | `.txt` | Basic text documents for content indexing |
| Markdown | `.md` | Structured documents with formatting |
| Python | `.py` | Source code files with syntax highlighting |
| JSON | `.json` | Structured data files |
| YAML | `.yaml` | Configuration files |
| Shell Script | `.sh` | Executable scripts |

## Installation and Setup

### Prerequisites

- Node.js 18+ and npm
- Rust 1.70+
- Tauri CLI v2

### Development Environment

```bash
# Clone the repository
git clone https://github.com/your-org/tecgpt-desktop
cd tecgpt-desktop

# Install dependencies
cd apps/desktop
npm install

# Run in development mode
npm run tauri dev
```

### Production Build

```bash
# Build for production
npm run tauri build

# The built application will be in:
# - macOS: target/release/bundle/macos/
# - Windows: target/release/bundle/msi/
```

## Testing Procedures

### 1. Document Indexing Test

Test the application's ability to index various document types:

```typescript
// Example test configuration
const testConfig = {
  indexPath: '/path/to/test/documents',
  enableEmbeddings: true,
  watchForChanges: true
};
```

### 2. Search Functionality Test

Validate both full-text and semantic search:

```python
# Example search queries
search_queries = [
    "artificial intelligence",
    "machine learning algorithms",
    "database configuration",
    "API endpoints"
]
```

### 3. Performance Benchmarks

Monitor system performance during indexing:

- File processing speed (files/second)
- Memory usage during large batch operations
- Embedding generation time
- Search response latency

## Configuration Options

### Database Settings

```yaml
database:
  provider: sqlite
  path: ./data/tecgpt.db
  enable_fts: true
  max_connections: 10
```

### AI Model Configuration

```yaml
ai_models:
  embeddings:
    model: "sentence-transformers/all-MiniLM-L6-v2"
    device: "cpu"
  chat:
    provider: "azure_openai"
    model: "gpt-4o"
    max_tokens: 4096
```

## API Reference

### Core Commands

#### File Operations
- `index_folder(path: string)` - Index documents in specified folder
- `search_files(query: string)` - Perform hybrid search
- `get_file_info(id: number)` - Retrieve file metadata

#### Embeddings
- `generate_embeddings(files: FileList)` - Create vector embeddings
- `rag_search(query: string)` - Semantic search with context

#### Chat Integration
- `send_chat_message(message: string, context?: FileContext[])` - AI chat with optional file context

## Contributing

### Code Style

We use the following tools for code quality:

- **Rust**: `rustfmt` and `clippy`
- **TypeScript**: ESLint and Prettier
- **Commit Messages**: Conventional Commits format

### Pull Request Process

1. Fork the repository
2. Create a feature branch
3. Make your changes with appropriate tests
4. Ensure all tests pass
5. Submit a pull request with detailed description

### Development Guidelines

```rust
// Example Rust code style
use crate::store::Store;
use anyhow::Result;

#[tauri::command]
pub async fn process_document(
    path: String,
    store: tauri::State<'_, Store>
) -> Result<DocumentInfo, String> {
    // Implementation here
    Ok(DocumentInfo::new(path))
}
```

## Troubleshooting

### Common Issues

**Issue**: Files not being indexed
- **Solution**: Check folder permissions and file system watcher status

**Issue**: Slow search performance
- **Solution**: Verify database indexes and consider rebuilding embeddings

**Issue**: High memory usage
- **Solution**: Adjust batch size in configuration and monitor embedding generation

### Debug Commands

```bash
# Check application logs
tail -f ~/Library/Application\ Support/TecGPT/logs/latest.log

# Verify database integrity
sqlite3 data/tecgpt.db "PRAGMA integrity_check;"

# Test embeddings model
python -c "from sentence_transformers import SentenceTransformer; SentenceTransformer('all-MiniLM-L6-v2')"
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Support

For technical support and bug reports:

- **GitHub Issues**: [Project Issues](https://github.com/your-org/tecgpt-desktop/issues)
- **Documentation**: [Wiki](https://github.com/your-org/tecgpt-desktop/wiki)
- **Email**: support@tecgpt.com