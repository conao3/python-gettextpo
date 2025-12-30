# gettextpo

Python bindings for the GNU gettext PO file library, built with Rust and PyO3.

## Overview

gettextpo provides direct access to libgettextpo, the GNU gettext library for parsing and manipulating PO (Portable Object) translation files. This package enables you to read, write, and programmatically edit PO files with full access to message metadata, comments, and validation.

## Installation

```bash
pip install gettextpo
```

### Build from source

This package requires the gettext development libraries and Rust toolchain.

```bash
# Install dependencies (Debian/Ubuntu)
apt-get install libgettextpo-dev

# Install dependencies (macOS)
brew install gettext

# Build and install
pip install maturin
maturin develop
```

## Quick Start

```python
from gettextpo.lib import PoFile, PoHeader

# Read an existing PO file
po = PoFile.read("messages.po")

# Or load from a string
po = PoFile.load("""
msgid "Hello"
msgstr "Bonjour"
""")

# Access domains and headers
for domain in po.domains():
    header = po.domain_header(domain)
    po_header = PoHeader(header)
    print(f"Content-Type: {po_header['Content-Type']}")

# Iterate over messages
for domain in po.domains():
    for message in po.messages(domain):
        # Access message properties via the low-level API
        pass
```

## Features

- Read and write PO files
- Access and modify message headers
- Iterate over translation messages
- Support for plural forms
- Handle translator and extracted comments
- Track file positions for messages
- Manage obsolete and fuzzy message flags
- Format string validation

## API Reference

### PoFile

- `PoFile.create()` - Create a new empty PO file
- `PoFile.read(filename)` - Read a PO file from disk
- `PoFile.load(contents)` - Load a PO file from a string
- `po.domains()` - List all domains in the file
- `po.domain_header(domain)` - Get the header for a domain
- `po.messages(domain)` - Iterate over messages in a domain

### PoHeader

- `PoHeader(header)` - Create a header wrapper
- `header[field]` - Get a header field value
- `header[field] = value` - Set a header field value
- `header.render()` - Get the header as a string

## Requirements

- Python 3.12 or later
- libgettextpo (gettext development library)

## License

GPL-3.0-or-later
