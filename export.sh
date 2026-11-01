#!/bin/bash
set -e

echo "[1/3] game-binaries..."
cargo build --release --no-default-features

echo "[2/3] Copying assets..."
rm -rf target/release/assets
cp -r assets target/release/

echo "[3/3] Done!"
echo "Game: target/release/ferro-engine"
echo "Start: ./target/release/ferro-engine assets/scenes/export.ron"
