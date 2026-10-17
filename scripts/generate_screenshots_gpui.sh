#!/usr/bin/env bash
set -euo pipefail

# Screenshot automation for native-theme gpui showcase
# Captures Linux-native theme presets on the Basic tab using spectacle on KDE Wayland
#
# Both gpui and iced now support built-in --screenshot self-capture on macOS
# (via screencapture -l). This script uses spectacle for Linux (KDE Wayland)
# local captures where self-capture is not available.
#
# NOTE: On macOS, you can use the showcase's built-in self-capture:
#   cargo run -p native-theme-gpui --example showcase-gpui --release -- \
#     --theme macos-monterey --variant light --icon-set system \
#     --tab basic --screenshot connectors/native-theme-gpui/docs/assets/macos-macos-monterey-light.png
#
# Adwaita needs a GNOME environment (requires adwaita icon theme).

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
# capture_showcases runs the showcase once per entry of SCREENSHOT_ENTRIES and
# fails the script unless each capture is of its window at its default size.
source "$SCRIPT_DIR/capture_window.sh"
OUTPUT_DIR="$PROJECT_ROOT/connectors/native-theme-gpui/docs/assets"

echo "=== Generating gpui showcase screenshots ==="
echo "Presets: 3 (dark + light each)"
echo "Total screenshots: ${#SCREENSHOT_ENTRIES[@]}"
echo ""

cd "$PROJECT_ROOT"
mkdir -p "$OUTPUT_DIR"
trap stop_showcase EXIT
capture_showcases gpui "" "$OUTPUT_DIR/linux-{theme}-{variant}.png" \
    "${SCREENSHOT_ENTRIES[@]}"

echo ""
echo "=== Screenshot generation complete ==="
echo "Generated $(ls "$OUTPUT_DIR"/linux-*.png 2>/dev/null | wc -l) screenshots in $OUTPUT_DIR"
