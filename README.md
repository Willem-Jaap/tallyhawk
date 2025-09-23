# TallyHawk 🦅

A sharp-eyed CLI tool for gathering project statistics.

## Installation

```bash
cargo install tallyhawk
```

## Usage

```bash
# Analyze current directory
tallyhawk count

# Analyze specific path
tallyhawk count /path/to/project

# Different output formats
tallyhawk count --format json
tallyhawk count --format csv

# Include hidden files
tallyhawk count --all

# Include comments and blank lines in count
tallyhawk count --include-comments --include-blanks
```

## Features

- 📊 Comprehensive project statistics
- 🎨 Beautiful colored terminal output
- 📁 Support for multiple file types and languages
- 🔍 Respects `.gitignore` files
- 📤 Multiple output formats (table, JSON, CSV)
- ⚡ Fast scanning with parallel processing

## Development

```bash
# Build
cargo build

# Run
cargo run -- count

# Test
cargo test

# Release build
cargo build --release
```

## License

MIT License - see LICENSE file for details.