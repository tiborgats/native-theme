#!/usr/bin/env bash
set -euo pipefail

# Capture theme-switching GIFs for the iced, gpui and egui showcases.
#
# Produces three GIFs:
#   connectors/native-theme-iced/docs/assets/theme-switching.gif  (via spectacle on KDE Wayland)
#   connectors/native-theme-gpui/docs/assets/theme-switching.gif  (via spectacle on KDE Wayland)
#   connectors/native-theme-egui/docs/assets/theme-switching.gif  (via spectacle on KDE Wayland)
#
# All three use spectacle for external window capture to include window
# decorations (title bar, buttons, borders) in the frames.
#
# Each GIF cycles through 4 Linux-native presets with matching icon sets.

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
# capture_showcases runs a showcase once per entry and fails the script unless
# each frame is of its window at its default size.
source "$SCRIPT_DIR/capture_window.sh"
ICED_OUTPUT_DIR="$PROJECT_ROOT/connectors/native-theme-iced/docs/assets"
GPUI_OUTPUT_DIR="$PROJECT_ROOT/connectors/native-theme-gpui/docs/assets"
EGUI_OUTPUT_DIR="$PROJECT_ROOT/connectors/native-theme-egui/docs/assets"
ICED_FRAME_DIR="$(mktemp -d)"
GPUI_FRAME_DIR="$(mktemp -d)"
EGUI_FRAME_DIR="$(mktemp -d)"
# The showcase and the frame directories go with the script, whether it ends,
# fails or is interrupted.
trap 'stop_showcase; rm -rf "$ICED_FRAME_DIR" "$GPUI_FRAME_DIR" "$EGUI_FRAME_DIR"' EXIT

# 4 visually distinct Linux-native presets with matching icon sets
# Format: theme:variant:icon-set:icon-theme
# Icon theme must match the UI theme.
THEMES=(
    "kde-breeze:dark:freedesktop:breeze-dark"
    "material-3:light:material:"
    "catppuccin-mocha:dark:lucide:"
    "kde-breeze:light:freedesktop:breeze"
)

mkdir -p "$ICED_OUTPUT_DIR" "$GPUI_OUTPUT_DIR" "$EGUI_OUTPUT_DIR"
cd "$PROJECT_ROOT"

# ── Iced GIF ──────────────────────────────────────────────────────────

echo "=== Generating iced theme-switching GIF ==="
echo ""
capture_showcases iced "--features iced_aw" "$ICED_FRAME_DIR/frame-{index}.png" "" "${THEMES[@]}"

echo ""
echo "--- Assembling iced GIF ---"
python3 "$SCRIPT_DIR/generate_gifs_spinners.py" \
    --theme-switching "$ICED_FRAME_DIR" \
    --theme-switching-output "$ICED_OUTPUT_DIR/theme-switching.gif"
echo ""
ls -lh "$ICED_OUTPUT_DIR/theme-switching.gif"

# ── gpui GIF ──────────────────────────────────────────────────────────

echo ""
echo "=== Generating gpui theme-switching GIF ==="
echo ""
capture_showcases gpui "" "$GPUI_FRAME_DIR/frame-{index}.png" "" "${THEMES[@]}"

echo ""
echo "--- Assembling gpui GIF ---"
python3 "$SCRIPT_DIR/generate_gifs_spinners.py" \
    --theme-switching "$GPUI_FRAME_DIR" \
    --theme-switching-output "$GPUI_OUTPUT_DIR/theme-switching.gif"
echo ""
ls -lh "$GPUI_OUTPUT_DIR/theme-switching.gif"

# ── egui GIF ──────────────────────────────────────────────────────────

echo ""
echo "=== Generating egui theme-switching GIF ==="
echo ""
capture_showcases egui "--all-features" "$EGUI_FRAME_DIR/frame-{index}.png" "" "${THEMES[@]}"

echo ""
echo "--- Assembling egui GIF ---"
python3 "$SCRIPT_DIR/generate_gifs_spinners.py" \
    --theme-switching "$EGUI_FRAME_DIR" \
    --theme-switching-output "$EGUI_OUTPUT_DIR/theme-switching.gif"
echo ""
ls -lh "$EGUI_OUTPUT_DIR/theme-switching.gif"

echo ""
echo "=== Done: all three theme-switching GIFs generated ==="
