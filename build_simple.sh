#!/bin/bash

echo "Building WarpSeek Terminal Version (Simple)..."

# Remove the build.rs file temporarily
mv build.rs build.rs.backup
echo 'fn main() {}' > build.rs

# Build the terminal version
cargo build --release --bin warpseek
cargo build --release --bin ws

# Restore build.rs
mv build.rs.backup build.rs

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
