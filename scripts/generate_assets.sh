#!/usr/bin/env bash
set -euo pipefail

# Master orchestration script for all native-theme visual assets
# Generates spinner GIFs and showcase screenshots in one command

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

echo "=== Generating visual assets for native-theme ==="
echo ""

echo "--- Step 1: Generating spinner GIFs ---"
python3 "$SCRIPT_DIR/generate_gifs.py"
echo ""

echo "--- Step 2: Generating showcase screenshots ---"
bash "$SCRIPT_DIR/generate_screenshots.sh"
echo ""

echo "=== All visual assets generated ==="
echo "Output directory: $(dirname "$SCRIPT_DIR")/docs/assets/"
ls -la "$(dirname "$SCRIPT_DIR")/docs/assets/" | head -20
echo "... ($(ls "$(dirname "$SCRIPT_DIR")/docs/assets/" | wc -l) files total)"
