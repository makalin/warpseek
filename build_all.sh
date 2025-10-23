#!/bin/bash

echo "🚀 Building WarpSeek - Complete Release"
echo "======================================"
echo ""

# Build terminal versions
echo "📱 Building Terminal CLI..."
cargo build --release --bin warpseek
cargo build --release --bin ws

# Build desktop GUI
echo "🖥️  Building Desktop GUI..."
cargo build --release --bin warpseek-gui

echo ""
echo "✅ Build Complete!"
echo ""

# Show build results
echo "📦 Built Binaries:"
echo "=================="
ls -lh target/release/warpseek*
echo ""

echo "📊 Binary Sizes:"
echo "================"
echo "Terminal CLI (warpseek): $(du -h target/release/warpseek | cut -f1)"
echo "Short Command (ws):     $(du -h target/release/ws | cut -f1)"
echo "Desktop GUI:            $(du -h target/release/warpseek-gui | cut -f1)"
echo ""

echo "🎯 Usage:"
echo "========"
echo ""
echo "Terminal Commands:"
echo "  ./target/release/warpseek --help"
echo "  ./target/release/ws --help"
echo "  ./target/release/ws init ~/Projects ~/Documents"
echo "  ./target/release/ws q \"search term\""
echo "  ./target/release/ws f \"filename\""
echo ""
echo "Desktop GUI:"
echo "  ./target/release/warpseek-gui"
echo ""

echo "🔧 Installation:"
echo "==============="
echo "To install system-wide:"
echo "  sudo cp target/release/warpseek /usr/local/bin/"
echo "  sudo cp target/release/ws /usr/local/bin/"
echo "  sudo cp target/release/warpseek-gui /usr/local/bin/"
echo ""

echo "🎉 WarpSeek is ready for release!"
echo ""
echo "Features:"
echo "  ✅ Terminal CLI with 'ws' short command"
echo "  ✅ Desktop GUI with modern interface"
echo "  ✅ Cross-platform support"
echo "  ✅ Advanced search capabilities"
echo "  ✅ Menu system and user-friendly interface"
