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

echo "--- Step 3: Generating gpui showcase screenshots ---"
bash "$SCRIPT_DIR/generate_gpui_screenshots.sh"
echo ""

echo "--- Step 4: Generating theme-switching GIFs (iced + gpui) ---"
bash "$SCRIPT_DIR/generate_theme_switching_gif.sh"
echo ""

PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

echo "=== All visual assets generated ==="
for dir in \
    "$PROJECT_ROOT/native-theme/docs/assets" \
    "$PROJECT_ROOT/connectors/native-theme-gpui/docs/assets" \
    "$PROJECT_ROOT/connectors/native-theme-iced/docs/assets" \
    "$PROJECT_ROOT/docs/assets"; do
    if [ -d "$dir" ]; then
        count=$(find "$dir" -maxdepth 1 -type f | wc -l)
        echo "  $dir ($count files)"
    fi
done
