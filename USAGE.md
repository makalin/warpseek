# WarpSeek Usage Guide

## Quick Start with `ws` Command

WarpSeek provides a short `ws` command for faster typing:

### Basic Commands

```bash
# Initialize with your project folders
ws init ~/Projects ~/Documents ~/Code

# Search for files containing "react"
ws q "react"

# Search only filenames
ws f "component"

# Search with options
ws q "javascript AND typescript" --top 20
ws q "TODO" --names-only

# Rebuild the index
ws index

# Watch for file changes
ws watch

# Add more folders
ws add ~/Downloads

# Remove folders
ws remove ~/Downloads

# Show statistics
ws stats

# Clear everything
ws purge
```

### Search Types

1. **Full Text Search** (`ws q`): Searches both filenames and content
2. **Filename Search** (`ws f`): Fast fuzzy filename matching
3. **Content Only**: Use `--names-only` flag to search only filenames

### Advanced Search

```bash
# Boolean operators
ws q "javascript AND react"
ws q "python OR ruby"
ws q "NOT node_modules"

# Phrase search
ws q "\"exact phrase\""

# Regex search
ws q "TODO|FIXME|HACK" --regex

# Case sensitive
ws q "React" --case-sensitive

# Limit results
ws q "test" --top 10
```

### Performance Tips

- Use `ws f` for filename-only searches (fastest)
- Use `--names-only` for content searches when you only need filenames
- Limit results with `--top N` for better performance
- Use `ws index` to rebuild when you add many files

### Desktop GUI

For a graphical interface:

```bash
warpseek-gui
```

This opens a modern desktop application with:
- Menu bar with File, Edit, View, Tools, Help
- Real-time search with instant results
- Advanced filtering options
- Search history and bookmarks
- File preview and quick open

### Command Aliases

You can use either:
- `ws` - Short and fast
- `warpseek` - Full name

Both commands are identical in functionality.

### Examples

```bash
# Find all JavaScript files
ws f "*.js"

# Find files containing "authentication"
ws q "authentication"

# Find TODO comments in code
ws q "TODO|FIXME" --regex

# Find large files (if you have file size in metadata)
ws q "size:>1MB"

# Search in specific file types
ws q "function" --names-only
```

### Troubleshooting

If search is slow:
1. Rebuild the index: `ws index`
2. Check if you have too many files indexed
3. Use `--top` to limit results
4. Use filename search (`ws f`) instead of content search

If results are outdated:
1. Run `ws index` to rebuild
2. Use `ws watch` to keep index fresh
3. Check if your folders are still being monitored
