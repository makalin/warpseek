# WarpSeek

A blazing-fast search tool with both terminal and desktop GUI interfaces that beats Spotlight for developers:

**Quick Start**: Use `ws` for the short command or `warpseek` for the full name.

## Features

### Core Search Capabilities
- **Instant fuzzy filename search** - Find files by name with intelligent matching
- **Full-text search** - Search across file contents with advanced query support
- **Regex search** - Powerful pattern matching with regular expressions
- **Content-only search** - Search only within file contents
- **File type filtering** - Filter by specific file extensions
- **Case-sensitive search** - Toggle case sensitivity for precise matching

### Advanced Features
- **Search history** - Keep track of previous searches
- **Bookmarks** - Save important files and folders
- **Smart filters** - Advanced filtering by file size, modification date, and more
- **Real-time indexing** - Watch mode keeps index fresh automatically
- **Parallel processing** - Multi-threaded indexing for maximum speed
- **Respects `.gitignore`** - Automatically excludes version control files

### User Interfaces
- **Terminal CLI** - Fast command-line interface for power users
- **Desktop GUI** - Modern web-based interface with menu system
- **Cross-platform** - Works on macOS, Linux, and Windows

## Installation

### From Source
```bash
git clone https://github.com/makalin/warpseek
cd warpseek
cargo build --release
```

### Install Binary
```bash
cargo install --path .
# or from repo:
# cargo install --git https://github.com/makalin/warpseek
```

## Usage

### Terminal Interface
```bash
# 1) Initialize with folders to index
ws init ~/Projects ~/Public ~/Documents

# 2) Full-text search
ws q "jwt AND secret" --top 100
ws q "Public" --names-only

# 3) Fuzzy filename search
ws f "invoice"

# 4) Rebuild / watch
ws index
ws watch

# 5) Manage paths
ws add ~/Downloads
ws remove ~/Downloads

# 6) Purge index
ws purge
```

**Note**: You can use either `warpseek` or `ws` as the command. The `ws` command is shorter and faster to type.

### Desktop GUI
```bash
# Launch the desktop application
warpseek-gui
```

The desktop interface provides:
- **Menu Bar** - File, Edit, View, Tools, and Help menus
- **Search Interface** - Modern search box with real-time results
- **Advanced Options** - Search type, filters, and preferences
- **Results Panel** - Rich display of search results with previews
- **Keyboard Shortcuts** - Fast navigation and search

### Menu System
- **File Menu**: Open folders, manage index, export results
- **Edit Menu**: Copy, select, clear, undo operations
- **View Menu**: Toggle preview, filters, history, bookmarks
- **Tools Menu**: Search filters, history, bookmarks, statistics, settings
- **Help Menu**: About, shortcuts, documentation

## Configuration

### Index Location
- **macOS**: `~/Library/Application Support/warpseek/index`
- **Linux**: `~/.local/share/warpseek/index`
- **Windows**: `%APPDATA%/warpseek/index`

### Search Types
1. **Full Text** - Search both filenames and content
2. **Filename** - Search only filenames
3. **Content** - Search only file contents
4. **Regex** - Use regular expressions for pattern matching

### File Filters
- **File Extensions** - Filter by specific file types
- **Size Limits** - Set minimum and maximum file sizes
- **Date Ranges** - Filter by modification date
- **Exclude Patterns** - Skip specific directories or files

## Advanced Usage

### Search Query Syntax
```bash
# Boolean operators
ws q "javascript AND react"
ws q "python OR ruby"
ws q "NOT node_modules"

# Phrase search
ws q "\"exact phrase\""

# Wildcards
ws q "test*.js"
```

### Regex Search
```bash
# Find email addresses
ws q "[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}" --regex

# Find TODO comments
ws q "TODO|FIXME|HACK" --regex
```

### Performance Tips
- Use `--names-only` for faster filename searches
- Limit results with `--top N` for better performance
- Use file type filters to reduce search scope
- Regular index rebuilding keeps performance optimal

## Development

### Building
```bash
# Build terminal version (both warpseek and ws commands)
cargo build --bin warpseek
cargo build --bin ws

# Build desktop GUI
cargo build --bin warpseek-gui

# Build all
cargo build
```

### Dependencies
- **Tantivy** - Full-text search engine
- **FST** - Finite state transducer for fuzzy matching
- **Tauri** - Desktop application framework
- **Rayon** - Parallel processing
- **Notify** - File system watching

## Notes
- Index is stored under OS data directories
- Skips files >20MB and binary blobs by heuristic
- Watch mode automatically rebuilds index on file changes
- Desktop GUI requires modern web browser capabilities