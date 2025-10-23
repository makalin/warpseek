#!/bin/bash

echo "Building WarpSeek Terminal Version..."

# Create a temporary Cargo.toml without GUI dependencies
cp Cargo.toml Cargo.toml.backup

# Remove GUI dependencies temporarily
sed -i '' '/# GUI dependencies/,/uuid = { version = "1.6", features = \["v4", "serde"\] }/d' Cargo.toml
sed -i '' '/\[build-dependencies\]/,/tauri-build = "1.5"/d' Cargo.toml

# Build the terminal version
cargo build --release --bin warpseek
cargo build --release --bin ws

# Restore original Cargo.toml
mv Cargo.toml.backup Cargo.toml

echo "Build complete!"
echo ""
echo "Binaries available:"
echo "  target/release/warpseek"
echo "  target/release/ws"
echo ""
echo "Usage:"
echo "  ./target/release/ws init ~/Projects ~/Documents"
echo "  ./target/release/ws q \"search term\""
echo "  ./target/release/ws f \"filename\""
