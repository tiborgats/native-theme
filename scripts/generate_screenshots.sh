#!/usr/bin/env bash
set -euo pipefail

# Screenshot automation for native-theme iced showcase
# Captures 4 theme presets on the Buttons tab using iced's built-in --screenshot flag

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
OUTPUT_DIR="$PROJECT_ROOT/docs/assets"

# Theme preset + variant pairings (matches CI workflow)
THEMES=("dracula:dark" "nord:light" "catppuccin-mocha:dark" "macos-sonoma:light")

echo "=== Generating iced showcase screenshots ==="
echo "Presets: ${#THEMES[@]}"
echo "Total screenshots: ${#THEMES[@]}"
echo ""

# Pre-build showcase binary to avoid compile delays during capture loop
echo "--- Building showcase binary (release mode) ---"
cd "$PROJECT_ROOT"
cargo build -p native-theme-iced --example showcase --release
echo ""

# Ensure output directory exists
mkdir -p "$OUTPUT_DIR"

echo "--- Capturing screenshots ---"
count=0
total=${#THEMES[@]}

for entry in "${THEMES[@]}"; do
    theme="${entry%%:*}"
    variant="${entry##*:}"
    output_file="$OUTPUT_DIR/iced-linux-${theme}-${variant}.png"
    count=$((count + 1))
    echo "[$count/$total] Capturing: $theme $variant -> $(basename "$output_file")"

    cargo run -p native-theme-iced --example showcase --release -- \
        --theme "$theme" --variant "$variant" --tab buttons \
        --screenshot "$output_file"
done

echo ""
echo "=== Screenshot generation complete ==="
echo "Generated $(ls "$OUTPUT_DIR"/iced-linux-*.png 2>/dev/null | wc -l) screenshots in $OUTPUT_DIR"
