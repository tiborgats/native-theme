#!/usr/bin/env bash
set -euo pipefail

# Screenshot automation for native-theme iced showcase
# Captures Linux-native theme presets on the Buttons tab using spectacle on KDE Wayland
#
# Uses spectacle for external window capture (same as gpui) to include
# window decorations (title bar, buttons, borders) in screenshots.
#
# NOTE: On macOS/Windows, you can use the showcase's built-in self-capture:
#   cargo run -p native-theme-iced --example showcase-iced --release -- \
#     --theme material --variant dark --icon-set material \
#     --screenshot docs/assets/iced-macos-material-dark.png
# This script uses spectacle for Linux (KDE Wayland) local captures.

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
OUTPUT_DIR="$PROJECT_ROOT/docs/assets"
DELAY=3

# Linux-native presets with matching icon sets (3 themes × dark+light)
# Format: theme:variant:icon-set
THEMES=(
    "kde-breeze:dark:freedesktop"
    "kde-breeze:light:freedesktop"
    "material:dark:material"
    "material:light:material"
    "catppuccin-mocha:dark:lucide"
    "catppuccin-mocha:light:lucide"
)

echo "=== Generating iced showcase screenshots ==="
echo "Presets: 3 (dark + light each)"
echo "Total screenshots: ${#THEMES[@]}"
echo ""

# Pre-build showcase binary to avoid compile delays during capture loop
echo "--- Building showcase binary (release mode) ---"
cd "$PROJECT_ROOT"
cargo build -p native-theme-iced --example showcase-iced --release
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
    IFS=':' read -r theme variant icon_set <<< "$entry"
    output_file="$OUTPUT_DIR/iced-linux-${theme}-${variant}.png"
    count=$((count + 1))
    echo "[$count/$total] Capturing: $theme $variant (icons: $icon_set) -> $(basename "$output_file")"

    cargo run -p native-theme-iced --example showcase-iced --release -- \
        --theme "$theme" --variant "$variant" --icon-set "$icon_set" \
        --tab buttons &
    PID=$!

    sleep "$DELAY"

    spectacle -a -b -n -o "$output_file"
    sleep 1

    kill "$PID" 2>/dev/null || true
    wait "$PID" 2>/dev/null || true
done

echo ""
echo "=== Screenshot generation complete ==="
echo "Generated $(ls "$OUTPUT_DIR"/iced-linux-*.png 2>/dev/null | wc -l) screenshots in $OUTPUT_DIR"
