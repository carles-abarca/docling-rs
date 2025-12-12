#!/usr/bin/env python3
"""
Docling Python Wrapper

This script provides a CLI-like interface to the original Python docling library,
allowing comparison with docling-rs (Rust implementation).

Usage:
    python docling_python_wrapper.py <input_file> [options]

Options:
    --to <format>           Output format: markdown, json, text (default: markdown)
    --output-dir <dir>      Output directory (default: current directory)
    --chunk                 Enable document chunking
    --chunk-strategy <s>    Chunking strategy: hierarchical or hybrid (default: hierarchical)
    --chunk-max-tokens <n>  Maximum tokens per chunk for hybrid (default: 512)
    --chunk-merge-peers     Merge undersized peer chunks (default: true)

Requirements:
    pip install docling docling-core[chunking]
"""

import argparse
import json
import os
import sys
from pathlib import Path


def parse_args():
    parser = argparse.ArgumentParser(
        description="Docling Python wrapper for document conversion"
    )
    parser.add_argument("input", type=str, help="Input file path")
    parser.add_argument(
        "--to",
        dest="output_format",
        choices=["markdown", "json", "text"],
        default="markdown",
        help="Output format (default: markdown)",
    )
    parser.add_argument(
        "--output-dir",
        dest="output_dir",
        type=str,
        default=".",
        help="Output directory (default: current directory)",
    )
    parser.add_argument(
        "--chunk",
        action="store_true",
        help="Enable document chunking",
    )
    parser.add_argument(
        "--chunk-strategy",
        dest="chunk_strategy",
        choices=["hierarchical", "hybrid"],
        default="hierarchical",
        help="Chunking strategy (default: hierarchical)",
    )
    parser.add_argument(
        "--chunk-max-tokens",
        dest="chunk_max_tokens",
        type=int,
        default=512,
        help="Maximum tokens per chunk for hybrid strategy (default: 512)",
    )
    parser.add_argument(
        "--chunk-merge-peers",
        dest="chunk_merge_peers",
        type=str,
        default="true",
        help="Merge undersized peer chunks (default: true)",
    )
    return parser.parse_args()


def get_output_extension(output_format: str) -> str:
    extensions = {
        "markdown": "md",
        "json": "json",
        "text": "txt",
    }
    return extensions.get(output_format, "txt")


def convert_document(input_path: str):
    """Convert document using docling DocumentConverter."""
    from docling.document_converter import DocumentConverter

    converter = DocumentConverter()
    result = converter.convert(input_path)
    return result.document


def export_to_text(doc) -> str:
    """Export document to plain text."""
    # Try export_to_text first, fallback to export_to_markdown
    if hasattr(doc, "export_to_text"):
        return doc.export_to_text()
    # Fallback: use markdown and strip formatting
    md = doc.export_to_markdown()
    return md


def export_to_markdown(doc) -> str:
    """Export document to markdown."""
    return doc.export_to_markdown()


def export_to_json(doc) -> str:
    """Export document to JSON."""
    # Use model_dump_json if available (pydantic), otherwise try to_json
    if hasattr(doc, "model_dump_json"):
        return doc.model_dump_json(indent=2)
    elif hasattr(doc, "to_json"):
        return doc.to_json(indent=2)
    elif hasattr(doc, "dict"):
        return json.dumps(doc.dict(), indent=2, default=str)
    else:
        # Fallback: serialize what we can
        return json.dumps({"error": "Cannot serialize document"}, indent=2)


def chunk_hierarchical(doc) -> list:
    """Chunk document using HierarchicalChunker."""
    from docling_core.transforms.chunker import HierarchicalChunker

    chunker = HierarchicalChunker()
    chunks = list(chunker.chunk(doc))
    return chunks


def chunk_hybrid(doc, max_tokens: int = 512, merge_peers: bool = True) -> list:
    """Chunk document using HybridChunker."""
    from docling.chunking import HybridChunker

    chunker = HybridChunker(
        max_tokens=max_tokens,
        merge_peers=merge_peers,
    )
    chunks = list(chunker.chunk(dl_doc=doc))
    return chunks


def format_chunks_as_markdown(chunks: list, strategy: str) -> str:
    """Format chunks as markdown output similar to docling-rs."""
    lines = [f"# Chunking Results (strategy: {strategy})\n"]
    total = len(chunks)

    for i, chunk in enumerate(chunks, 1):
        text = chunk.text if hasattr(chunk, "text") else str(chunk)
        size = len(text)
        lines.append(f"## Chunk {i} of {total}")
        lines.append(f"Size: {size} characters\n")
        lines.append(text)
        lines.append("\n---\n")

    return "\n".join(lines)


def format_chunks_as_json(chunks: list, strategy: str) -> str:
    """Format chunks as JSON output similar to docling-rs."""
    chunk_list = []
    for i, chunk in enumerate(chunks, 1):
        text = chunk.text if hasattr(chunk, "text") else str(chunk)
        chunk_data = {
            "chunk_index": i,
            "text": text,
            "char_count": len(text),
        }
        # Add metadata if available
        if hasattr(chunk, "meta"):
            chunk_data["meta"] = {
                "headings": chunk.meta.headings if hasattr(chunk.meta, "headings") else [],
                "captions": chunk.meta.captions if hasattr(chunk.meta, "captions") else [],
            }
        chunk_list.append(chunk_data)

    result = {
        "strategy": strategy,
        "total_chunks": len(chunks),
        "chunks": chunk_list,
    }
    return json.dumps(result, indent=2, ensure_ascii=False)


def format_chunks_as_text(chunks: list, strategy: str) -> str:
    """Format chunks as plain text output."""
    lines = [f"Chunking Results (strategy: {strategy})\n"]
    lines.append(f"Total chunks: {len(chunks)}\n")
    lines.append("=" * 40 + "\n")

    for i, chunk in enumerate(chunks, 1):
        text = chunk.text if hasattr(chunk, "text") else str(chunk)
        lines.append(f"[Chunk {i}]")
        lines.append(text)
        lines.append("\n" + "-" * 40 + "\n")

    return "\n".join(lines)


def main():
    args = parse_args()

    # Validate input file
    input_path = Path(args.input)
    if not input_path.exists():
        print(f"Error: Input file not found: {args.input}", file=sys.stderr)
        sys.exit(1)

    # Create output directory
    output_dir = Path(args.output_dir)
    output_dir.mkdir(parents=True, exist_ok=True)

    # Determine output filename
    base_name = input_path.stem
    ext = get_output_extension(args.output_format)
    output_file = output_dir / f"{base_name}.{ext}"

    try:
        # Convert document
        doc = convert_document(str(input_path))

        if args.chunk:
            # Chunking mode
            merge_peers = args.chunk_merge_peers.lower() in ("true", "1", "yes")

            if args.chunk_strategy == "hierarchical":
                chunks = chunk_hierarchical(doc)
            else:
                chunks = chunk_hybrid(doc, args.chunk_max_tokens, merge_peers)

            # Format output based on format
            if args.output_format == "markdown":
                output_content = format_chunks_as_markdown(chunks, args.chunk_strategy)
            elif args.output_format == "json":
                output_content = format_chunks_as_json(chunks, args.chunk_strategy)
            else:
                output_content = format_chunks_as_text(chunks, args.chunk_strategy)
        else:
            # Regular conversion mode
            if args.output_format == "markdown":
                output_content = export_to_markdown(doc)
            elif args.output_format == "json":
                output_content = export_to_json(doc)
            else:
                output_content = export_to_text(doc)

        # Write output file
        with open(output_file, "w", encoding="utf-8") as f:
            f.write(output_content)

        # Print filename on success (like docling-rs)
        print(input_path.name)

    except ImportError as e:
        print(f"Error: Missing dependency - {e}", file=sys.stderr)
        print("Install with: pip install docling docling-core[chunking]", file=sys.stderr)
        sys.exit(2)
    except Exception as e:
        print(f"Error processing {input_path.name}: {e}", file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    main()
