#!/bin/sh

set -e

BIN_NAME="adn"
VERSION=$(grep '^version' Cargo.toml | head -n1 | cut -d'"' -f2)
DIST_DIR="dist"
TAR_NAME="${BIN_NAME}-${VERSION}-linux-x86_64.tar.gz"

echo "🔧 Building release binary..."
cargo build --release

echo "📁 Preparing dist directory..."
rm -rf "$DIST_DIR"
mkdir -p "$DIST_DIR"

echo "📦 Copying binary and installer..."
cp "target/release/$BIN_NAME" "$DIST_DIR/"
cp "install.sh" "$DIST_DIR/"

echo "📦 Creating tarball..."
tar -czf "$DIST_DIR/$TAR_NAME" -C "$DIST_DIR" "$BIN_NAME" install.sh

rm "$DIST_DIR/$BIN_NAME" "$DIST_DIR/install.sh"

echo "✅ Package created at: $DIST_DIR/$TAR_NAME"

