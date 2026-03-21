#!/usr/bin/env bash
set -euo pipefail

# Capture individual theme frames from the iced showcase and assemble
# them into an animated theme-switching GIF for the root README hero section.
#
# Uses the iced showcase's --screenshot flag to capture one frame per
# theme preset, then calls generate_gifs.py --theme-switching for GIF assembly.

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
OUTPUT_DIR="$PROJECT_ROOT/docs/assets"
FRAME_DIR="$(mktemp -d)"

# 4 visually distinct Linux-native presets with matching icon sets
# Format: theme:variant:icon-set (icon theme must match UI theme)
THEMES=(
    "kde-breeze:dark:freedesktop"
    "material:light:material"
    "catppuccin-mocha:dark:lucide"
    "kde-breeze:light:freedesktop"
)

echo "=== Generating theme-switching GIF ==="
echo "Presets: ${#THEMES[@]}"
echo "Frame dir: $FRAME_DIR"
echo ""

# Pre-build showcase binary to avoid compile delays during capture loop
echo "--- Building showcase binary (release mode) ---"
cd "$PROJECT_ROOT"
cargo build -p native-theme-iced --example showcase --release
echo ""

# Ensure output directory exists
mkdir -p "$OUTPUT_DIR"

echo "--- Capturing theme frames ---"
for i in "${!THEMES[@]}"; do
    IFS=':' read -r theme variant icon_set <<< "${THEMES[$i]}"
    frame_file="$FRAME_DIR/frame-$(printf '%02d' "$i").png"
    echo "[$((i + 1))/${#THEMES[@]}] Capturing: $theme $variant (icons: $icon_set) -> $(basename "$frame_file")"

    cargo run -p native-theme-iced --example showcase --release -- \
        --theme "$theme" --variant "$variant" --icon-set "$icon_set" \
        --tab buttons --screenshot "$frame_file"
done

echo ""
echo "--- Assembling GIF ---"
python3 "$SCRIPT_DIR/generate_gifs.py" \
    --theme-switching "$FRAME_DIR" \
    --theme-switching-output "$OUTPUT_DIR/theme-switching.gif"

echo ""
echo "=== Theme-switching GIF generation complete ==="
ls -lh "$OUTPUT_DIR/theme-switching.gif"

# Clean up temp frames
rm -rf "$FRAME_DIR"
