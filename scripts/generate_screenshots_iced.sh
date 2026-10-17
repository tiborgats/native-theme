#!/usr/bin/env bash
set -euo pipefail

# Screenshot automation for native-theme iced showcase
# Captures Linux-native theme presets on the Basic tab using spectacle on KDE Wayland
#
# Uses spectacle for external window capture (same as gpui) to include
# window decorations (title bar, buttons, borders) in screenshots.
#
# NOTE: On macOS/Windows, you can use the showcase's built-in self-capture:
#   cargo run -p native-theme-iced --example showcase-iced --release --features iced_aw -- \
#     --theme material-3 --variant dark --icon-set material \
#     --tab basic --screenshot connectors/native-theme-iced/docs/assets/macos-material-3-dark.png
# This script uses spectacle for Linux (KDE Wayland) local captures.

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
# capture_showcases runs the showcase once per entry of SCREENSHOT_ENTRIES and
# fails the script unless each capture is of its window at its default size.
source "$SCRIPT_DIR/capture_window.sh"
OUTPUT_DIR="$PROJECT_ROOT/connectors/native-theme-iced/docs/assets"

echo "=== Generating iced showcase screenshots ==="
echo "Presets: 3 (dark + light each)"
echo "Total screenshots: ${#SCREENSHOT_ENTRIES[@]}"
echo ""

cd "$PROJECT_ROOT"
mkdir -p "$OUTPUT_DIR"
trap stop_showcase EXIT
capture_showcases iced "--features iced_aw" "$OUTPUT_DIR/linux-{theme}-{variant}.png" \
    "${SCREENSHOT_ENTRIES[@]}"

echo ""
echo "=== Screenshot generation complete ==="
echo "Generated $(ls "$OUTPUT_DIR"/linux-*.png 2>/dev/null | wc -l) screenshots in $OUTPUT_DIR"
