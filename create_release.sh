#!/bin/bash

echo "📦 Creating WarpSeek Release Package"
echo "===================================="
echo ""

# Create release directory
RELEASE_DIR="warpseek-release-$(date +%Y%m%d)"
mkdir -p "$RELEASE_DIR"

echo "📁 Creating release directory: $RELEASE_DIR"

# Copy binaries
echo "📋 Copying binaries..."
cp target/release/warpseek "$RELEASE_DIR/"
cp target/release/ws "$RELEASE_DIR/"
cp target/release/warpseek-gui "$RELEASE_DIR/"

# Copy documentation
echo "📚 Copying documentation..."
cp README.md "$RELEASE_DIR/"
cp USAGE.md "$RELEASE_DIR/"
cp BUILD_SUCCESS.md "$RELEASE_DIR/"

# Copy HTML interface
echo "🌐 Copying GUI interface..."
cp -r dist "$RELEASE_DIR/"

# Copy icons
echo "🎨 Copying icons..."
cp -r icons "$RELEASE_DIR/"

# Copy scripts
echo "🔧 Copying scripts..."
cp build_all.sh "$RELEASE_DIR/"
cp install.sh "$RELEASE_DIR/"

# Create installation script
echo "📝 Creating installation script..."
cat > "$RELEASE_DIR/install.sh" << 'EOF'
#!/bin/bash

echo "Installing WarpSeek..."
echo "====================="
echo ""

# Check if running as root
if [ "$EUID" -eq 0 ]; then
    echo "Installing system-wide..."
    cp warpseek /usr/local/bin/
    cp ws /usr/local/bin/
    cp warpseek-gui /usr/local/bin/
    chmod +x /usr/local/bin/warpseek
    chmod +x /usr/local/bin/ws
    chmod +x /usr/local/bin/warpseek-gui
    echo "✅ Installed to /usr/local/bin/"
else
    echo "Installing to ~/.local/bin/..."
    mkdir -p ~/.local/bin
    cp warpseek ~/.local/bin/
    cp ws ~/.local/bin/
    cp warpseek-gui ~/.local/bin/
    chmod +x ~/.local/bin/warpseek
    chmod +x ~/.local/bin/ws
    chmod +x ~/.local/bin/warpseek-gui
    echo "✅ Installed to ~/.local/bin/"
    echo "Add ~/.local/bin to your PATH if not already there"
fi

echo ""
echo "🎉 Installation complete!"
echo ""
echo "Usage:"
echo "  ws init ~/Projects ~/Documents"
echo "  ws q \"search term\""
echo "  ws f \"filename\""
echo "  warpseek-gui  # Launch desktop GUI"
EOF

chmod +x "$RELEASE_DIR/install.sh"

# Create README for release
cat > "$RELEASE_DIR/RELEASE_README.md" << 'EOF'
# WarpSeek Release Package

## 🚀 Quick Start

### Install
```bash
./install.sh
```

### Terminal Usage
```bash
# Initialize with your folders
ws init ~/Projects ~/Documents

# Search files
ws q "search term"
ws f "filename"

# Get help
ws --help
```

### Desktop GUI
```bash
# Launch the desktop application
warpseek-gui
```

## 📦 What's Included

- **warpseek** - Full terminal command
- **ws** - Short terminal command  
- **warpseek-gui** - Desktop GUI application
- **dist/** - Web interface for GUI
- **icons/** - Application icons
- **Documentation** - README, USAGE, etc.

## 🎯 Features

- ✅ **Blazing Fast Search** - Terminal and desktop interfaces
- ✅ **Short Commands** - Use `ws` for quick access
- ✅ **Modern GUI** - Beautiful desktop interface
- ✅ **Cross-platform** - Works on macOS, Linux, Windows
- ✅ **Advanced Search** - Full-text, filename, regex, content
- ✅ **Smart Features** - History, bookmarks, filters

## 📊 File Sizes

- Terminal CLI: ~850KB
- Desktop GUI: ~6MB
- Total Package: ~10MB

## 🔧 System Requirements

- **Terminal**: Any modern terminal
- **Desktop GUI**: Modern web browser capabilities
- **OS**: macOS, Linux, Windows
- **Memory**: 50MB+ RAM
- **Storage**: 10MB+ free space

Enjoy your blazing-fast search experience! 🔍⚡
EOF

# Create archive
echo "📦 Creating release archive..."
tar -czf "${RELEASE_DIR}.tar.gz" "$RELEASE_DIR"

# Show results
echo ""
echo "✅ Release package created!"
echo ""
echo "📁 Release directory: $RELEASE_DIR"
echo "📦 Archive: ${RELEASE_DIR}.tar.gz"
echo ""
echo "📊 Package contents:"
echo "=========================="
ls -lh "$RELEASE_DIR"
echo ""
echo "📏 Archive size: $(du -h "${RELEASE_DIR}.tar.gz" | cut -f1)"
echo ""
echo "🚀 Ready for distribution!"
