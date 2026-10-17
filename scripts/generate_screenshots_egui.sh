#!/usr/bin/env bash
set -euo pipefail

# Screenshot automation for the native-theme egui showcase
# Captures Linux-native theme presets on the Basic tab using spectacle on KDE Wayland
#
# Uses spectacle for external window capture (same as gpui and iced) to include
# window decorations (title bar, buttons, borders) in screenshots.
#
# NOTE: On macOS/Windows, use the showcase's built-in self-capture, which
# writes the window with its title bar as a PNG, as gpui's and iced's do:
#   cargo run -p native-theme-egui --example showcase-egui --release --all-features -- \
#     --theme macos-monterey --variant light --icon-set system \
#     --tab basic --screenshot connectors/native-theme-egui/docs/assets/macos-macos-monterey-light.png
# This script uses spectacle for Linux (KDE Wayland) local captures.
#
# --all-features: the showcase is built with every connector feature, as the
# screenshot workflow builds it (the connector spec's Cargo.toml section).
#
# Usage: generate_screenshots_egui.sh [DUMP_DIR]. With DUMP_DIR (created if
# missing, a relative one taken from the directory the script is run from),
# each run also writes its layout dump, with the capture beside it, into
# DUMP_DIR (capture_showcases), for scripts/check_showcase_parity.py.

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
# capture_showcases runs the showcase once per entry of SCREENSHOT_ENTRIES and
# fails the script unless each capture is of its window at its default size.
source "$SCRIPT_DIR/capture_window.sh"
OUTPUT_DIR="$PROJECT_ROOT/connectors/native-theme-egui/docs/assets"

echo "=== Generating egui showcase screenshots ==="
echo "Presets: 3 (dark + light each)"
echo "Total screenshots: ${#SCREENSHOT_ENTRIES[@]}"
echo ""

mkdir -p "$OUTPUT_DIR"
trap stop_showcase EXIT
capture_showcases egui "--all-features" "$OUTPUT_DIR/linux-{theme}-{variant}.png" \
    "${1:-}" "${SCREENSHOT_ENTRIES[@]}"

echo ""
echo "=== Screenshot generation complete ==="
echo "Generated $(ls "$OUTPUT_DIR"/linux-*.png 2>/dev/null | wc -l) screenshots in $OUTPUT_DIR"
