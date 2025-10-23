#!/bin/bash

echo "Installing WarpSeek..."
echo ""

# Build the project
echo "Building WarpSeek..."
cargo build --release

if [ $? -eq 0 ]; then
    echo "✅ Build successful!"
    echo ""
    
    # Install binaries
    echo "Installing binaries..."
    cp target/release/warpseek /usr/local/bin/ 2>/dev/null || echo "Note: Could not install to /usr/local/bin (try with sudo)"
    cp target/release/ws /usr/local/bin/ 2>/dev/null || echo "Note: Could not install ws to /usr/local/bin (try with sudo)"
    
    echo ""
    echo "🎉 Installation complete!"
    echo ""
    echo "Usage:"
    echo "  ws init ~/Projects ~/Documents    # Initialize index"
    echo "  ws q \"search term\"              # Search files"
    echo "  ws f \"filename\"                 # Fuzzy filename search"
    echo "  ws index                         # Rebuild index"
    echo "  ws watch                         # Watch for changes"
    echo ""
    echo "Or use the full command:"
    echo "  warpseek init ~/Projects ~/Documents"
    echo "  warpseek q \"search term\""
    echo ""
    echo "Desktop GUI:"
    echo "  warpseek-gui                     # Launch desktop application"
    echo ""
else
    echo "❌ Build failed. Please check the errors above."
    exit 1
fi
