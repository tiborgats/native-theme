#!/usr/bin/env bash
set -euo pipefail

# Screenshot automation for native-theme gpui showcase
# Captures 4 theme presets on the Buttons tab using spectacle on KDE Wayland
#
# Unlike iced (which has a built-in --screenshot flag), gpui has no
# programmatic screenshot API, so this script uses spectacle for external
# window capture.

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
OUTPUT_DIR="$PROJECT_ROOT/docs/assets"
DELAY=3

# Theme preset + variant pairings (matches iced screenshot set)
THEMES=("dracula:dark" "nord:light" "catppuccin-mocha:dark" "macos-sonoma:light")

echo "=== Generating gpui showcase screenshots ==="
echo "Presets: ${#THEMES[@]}"
echo "Total screenshots: ${#THEMES[@]}"
echo ""

# Pre-build showcase binary to avoid compile delays during capture loop
echo "--- Building showcase binary (release mode) ---"
cd "$PROJECT_ROOT"
cargo build -p native-theme-gpui --example showcase --release
echo ""

# Ensure output directory exists
mkdir -p "$OUTPUT_DIR"

# Kill any stale spectacle instances to avoid D-Bus singleton conflicts
pkill spectacle 2>/dev/null || true

# Clean up showcase process on exit
trap 'kill "$PID" 2>/dev/null || true' EXIT

echo "--- Capturing screenshots ---"
echo "WARNING: Do not interact with the desktop during capture."
echo ""

count=0
total=${#THEMES[@]}

for entry in "${THEMES[@]}"; do
    theme="${entry%%:*}"
    variant="${entry##*:}"
    output_file="$OUTPUT_DIR/gpui-linux-${theme}-${variant}.png"
    count=$((count + 1))
    echo "[$count/$total] Capturing: $theme $variant -> $(basename "$output_file")"

    cargo run -p native-theme-gpui --example showcase --release -- \
        --theme "$theme" --variant "$variant" --tab buttons &
    PID=$!

    sleep "$DELAY"

    spectacle -i -a -b -n -e -o "$output_file"
    sleep 1

    kill "$PID" 2>/dev/null || true
    wait "$PID" 2>/dev/null || true
done

echo ""
echo "=== Screenshot generation complete ==="
echo "Generated $(ls "$OUTPUT_DIR"/gpui-linux-*.png 2>/dev/null | wc -l) screenshots in $OUTPUT_DIR"
