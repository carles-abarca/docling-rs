#!/usr/bin/env python3
"""
TecGPT Desktop - Document Processing Utilities

This module provides utilities for processing and analyzing documents
for the TecGPT Desktop application. It includes functions for text extraction,
embedding generation, and search optimization.

Author: TecGPT Development Team
License: MIT
"""

import os
import json
import logging
import hashlib
from typing import List, Dict, Optional, Tuple
from dataclasses import dataclass
from pathlib import Path
import sqlite3
from datetime import datetime

# Configure logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s'
)
logger = logging.getLogger(__name__)


@dataclass
class DocumentInfo:
    """Represents metadata for a processed document."""
    id: Optional[int]
    path: str
    name: str
    size: int
    modified_at: datetime
    content_hash: str
    content_preview: str
    file_type: str

    def to_dict(self) -> Dict:
        """Convert document info to dictionary format."""
        return {
            'id': self.id,
            'path': self.path,
            'name': self.name,
            'size': self.size,
            'modified_at': self.modified_at.isoformat(),
            'content_hash': self.content_hash,
            'content_preview': self.content_preview,
            'file_type': self.file_type
        }


class DocumentProcessor:
    """Main class for processing documents and managing the search index."""

    def __init__(self, db_path: str = "tecgpt.db"):
        """Initialize the document processor with database connection."""
        self.db_path = db_path
        self.supported_extensions = {
            '.txt', '.md', '.py', '.js', '.ts', '.json', '.yaml', '.yml',
            '.csv', '.xml', '.html', '.css', '.sql', '.sh', '.bat'
        }
        self._setup_database()

    def _setup_database(self) -> None:
        """Initialize the database schema for document storage."""
        try:
            with sqlite3.connect(self.db_path) as conn:
                cursor = conn.cursor()

                # Create main files table
                cursor.execute("""
                    CREATE TABLE IF NOT EXISTS files (
                        id INTEGER PRIMARY KEY AUTOINCREMENT,
                        path TEXT UNIQUE NOT NULL,
                        name TEXT NOT NULL,
                        size INTEGER NOT NULL,
                        modified_at TEXT NOT NULL,
                        content_hash TEXT NOT NULL,
                        content_preview TEXT,
                        file_type TEXT NOT NULL,
                        indexed_at TEXT DEFAULT CURRENT_TIMESTAMP
                    )
                """)

                # Create FTS5 virtual table for full-text search
                cursor.execute("""
                    CREATE VIRTUAL TABLE IF NOT EXISTS files_fts USING fts5(
                        name, content_preview, content='files', content_rowid='id'
                    )
                """)

                conn.commit()
                logger.info("Database initialized successfully")

        except sqlite3.Error as e:
            logger.error(f"Database setup failed: {e}")
            raise

    def process_file(self, file_path: str) -> Optional[DocumentInfo]:
        """
        Process a single file and extract its metadata and content.

        Args:
            file_path: Path to the file to process

        Returns:
            DocumentInfo object if successful, None if failed
        """
        try:
            path_obj = Path(file_path)

            if not path_obj.exists():
                logger.warning(f"File not found: {file_path}")
                return None

            if path_obj.suffix.lower() not in self.supported_extensions:
                logger.debug(f"Unsupported file type: {file_path}")
                return None

            # Get file statistics
            stat = path_obj.stat()
            modified_at = datetime.fromtimestamp(stat.st_mtime)

            # Extract content and generate hash
            content = self._extract_content(path_obj)
            content_hash = hashlib.sha256(content.encode('utf-8')).hexdigest()

            # Create preview (first 500 characters)
            content_preview = content[:500] + "..." if len(content) > 500 else content

            doc_info = DocumentInfo(
                id=None,
                path=str(path_obj.absolute()),
                name=path_obj.name,
                size=stat.st_size,
                modified_at=modified_at,
                content_hash=content_hash,
                content_preview=content_preview,
                file_type=path_obj.suffix.lower()
            )

            logger.info(f"Successfully processed file: {path_obj.name}")
            return doc_info

        except Exception as e:
            logger.error(f"Failed to process file {file_path}: {e}")
            return None

    def _extract_content(self, path: Path) -> str:
        """
        Extract text content from a file based on its type.

        Args:
            path: Path object for the file

        Returns:
            Extracted text content
        """
        try:
            # Handle different file types
            if path.suffix.lower() in ['.txt', '.md', '.py', '.js', '.ts', '.css', '.sql', '.sh', '.bat']:
                with open(path, 'r', encoding='utf-8', errors='ignore') as f:
                    return f.read()

            elif path.suffix.lower() in ['.json']:
                with open(path, 'r', encoding='utf-8') as f:
                    data = json.load(f)
                    return json.dumps(data, indent=2)

            elif path.suffix.lower() in ['.yaml', '.yml']:
                # For simplicity, treat YAML as text
                with open(path, 'r', encoding='utf-8', errors='ignore') as f:
                    return f.read()

            else:
                logger.warning(f"No specific handler for file type: {path.suffix}")
                with open(path, 'r', encoding='utf-8', errors='ignore') as f:
                    return f.read()

        except Exception as e:
            logger.error(f"Content extraction failed for {path}: {e}")
            return ""

    def index_directory(self, directory_path: str, recursive: bool = True) -> List[DocumentInfo]:
        """
        Index all supported files in a directory.

        Args:
            directory_path: Path to the directory to index
            recursive: Whether to index subdirectories

        Returns:
            List of processed DocumentInfo objects
        """
        processed_files = []
        directory = Path(directory_path)

        if not directory.exists() or not directory.is_dir():
            logger.error(f"Invalid directory: {directory_path}")
            return processed_files

        # Get file pattern based on recursive flag
        pattern = "**/*" if recursive else "*"

        logger.info(f"Starting indexing of directory: {directory_path}")

        for file_path in directory.glob(pattern):
            if file_path.is_file():
                doc_info = self.process_file(str(file_path))
                if doc_info:
                    processed_files.append(doc_info)
                    self._store_document(doc_info)

        logger.info(f"Indexed {len(processed_files)} files from {directory_path}")
        return processed_files

    def _store_document(self, doc_info: DocumentInfo) -> None:
        """Store document information in the database."""
        try:
            with sqlite3.connect(self.db_path) as conn:
                cursor = conn.cursor()

                # Insert or update file record
                cursor.execute("""
                    INSERT OR REPLACE INTO files
                    (path, name, size, modified_at, content_hash, content_preview, file_type)
                    VALUES (?, ?, ?, ?, ?, ?, ?)
                """, (
                    doc_info.path,
                    doc_info.name,
                    doc_info.size,
                    doc_info.modified_at.isoformat(),
                    doc_info.content_hash,
                    doc_info.content_preview,
                    doc_info.file_type
                ))

                # Update FTS index
                cursor.execute("""
                    INSERT OR REPLACE INTO files_fts (rowid, name, content_preview)
                    SELECT id, name, content_preview FROM files WHERE path = ?
                """, (doc_info.path,))

                conn.commit()

        except sqlite3.Error as e:
            logger.error(f"Failed to store document {doc_info.name}: {e}")

    def search_documents(self, query: str, limit: int = 20) -> List[Dict]:
        """
        Search documents using full-text search.

        Args:
            query: Search query string
            limit: Maximum number of results to return

        Returns:
            List of matching documents
        """
        try:
            with sqlite3.connect(self.db_path) as conn:
                cursor = conn.cursor()

                # Perform FTS search
                cursor.execute("""
                    SELECT f.id, f.path, f.name, f.size, f.modified_at,
                           f.content_preview, f.file_type,
                           highlight(files_fts, 0, '<mark>', '</mark>') as highlighted_name,
                           highlight(files_fts, 1, '<mark>', '</mark>') as highlighted_content
                    FROM files f
                    JOIN files_fts ON f.id = files_fts.rowid
                    WHERE files_fts MATCH ?
                    ORDER BY rank
                    LIMIT ?
                """, (query, limit))

                results = []
                for row in cursor.fetchall():
                    results.append({
                        'id': row[0],
                        'path': row[1],
                        'name': row[2],
                        'size': row[3],
                        'modified_at': row[4],
                        'content_preview': row[5],
                        'file_type': row[6],
                        'highlighted_name': row[7],
                        'highlighted_content': row[8]
                    })

                logger.info(f"Search for '{query}' returned {len(results)} results")
                return results

        except sqlite3.Error as e:
            logger.error(f"Search failed: {e}")
            return []

    def get_file_statistics(self) -> Dict[str, int]:
        """Get statistics about indexed files."""
        try:
            with sqlite3.connect(self.db_path) as conn:
                cursor = conn.cursor()

                cursor.execute("SELECT COUNT(*), SUM(size) FROM files")
                total_files, total_size = cursor.fetchone()

                cursor.execute("""
                    SELECT file_type, COUNT(*)
                    FROM files
                    GROUP BY file_type
                    ORDER BY COUNT(*) DESC
                """)
                file_types = dict(cursor.fetchall())

                return {
                    'total_files': total_files or 0,
                    'total_size': total_size or 0,
                    'file_types': file_types
                }

        except sqlite3.Error as e:
            logger.error(f"Failed to get statistics: {e}")
            return {'total_files': 0, 'total_size': 0, 'file_types': {}}


def main():
    """Example usage of the DocumentProcessor class."""
    processor = DocumentProcessor()

    # Example: Index current directory
    current_dir = os.getcwd()
    print(f"Indexing files in: {current_dir}")

    processed_files = processor.index_directory(current_dir, recursive=False)
    print(f"Processed {len(processed_files)} files")

    # Example: Search for documents
    search_results = processor.search_documents("python OR function")
    print(f"Found {len(search_results)} documents matching search")

    # Display statistics
    stats = processor.get_file_statistics()
    print(f"Database statistics: {stats}")


if __name__ == "__main__":
    main()