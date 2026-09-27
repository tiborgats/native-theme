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
ICED_OUTPUT_DIR="$PROJECT_ROOT/connectors/native-theme-iced/docs/assets"
GPUI_OUTPUT_DIR="$PROJECT_ROOT/connectors/native-theme-gpui/docs/assets"
ICED_FRAME_DIR="$(mktemp -d)"
GPUI_FRAME_DIR="$(mktemp -d)"
EGUI_OUTPUT_DIR="$PROJECT_ROOT/connectors/native-theme-egui/docs/assets"
EGUI_FRAME_DIR="$(mktemp -d)"
DELAY=3

# 4 visually distinct Linux-native presets with matching icon sets
# Format: theme:variant:icon-set:icon-theme
# Icon theme must match the UI theme.
THEMES=(
    "kde-breeze:dark:freedesktop:breeze-dark"
    "material:light:material:"
    "catppuccin-mocha:dark:lucide:"
    "kde-breeze:light:freedesktop:breeze"
)

mkdir -p "$ICED_OUTPUT_DIR" "$GPUI_OUTPUT_DIR" "$EGUI_OUTPUT_DIR"
cd "$PROJECT_ROOT"

# Kill any stale spectacle instances to avoid D-Bus singleton conflicts
pkill spectacle 2>/dev/null || true

echo "WARNING: Do not interact with the desktop during capture."
echo ""

# ── Iced GIF ──────────────────────────────────────────────────────────

echo "=== Generating iced theme-switching GIF ==="
echo ""

echo "--- Building iced showcase binary (release mode) ---"
cargo build -p native-theme-iced --example showcase-iced --release --features iced_aw
echo ""

# Clean up showcase process on exit
trap 'kill "$PID" 2>/dev/null || true' EXIT

echo "--- Capturing iced frames ---"
for i in "${!THEMES[@]}"; do
    IFS=':' read -r theme variant icon_set icon_theme <<< "${THEMES[$i]}"
    frame_file="$ICED_FRAME_DIR/frame-$(printf '%02d' "$i").png"
    echo "[$((i + 1))/${#THEMES[@]}] $theme $variant (icons: $icon_set${icon_theme:+/$icon_theme})"

    cargo run -p native-theme-iced --example showcase-iced --release --features iced_aw -- \
        --theme "$theme" --variant "$variant" --icon-set "$icon_set" \
        --tab buttons &
    PID=$!

    sleep "$DELAY"

    spectacle -a -b -n -o "$frame_file"
    sleep 1

    kill "$PID" 2>/dev/null || true
    wait "$PID" 2>/dev/null || true
done

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

echo "--- Building gpui showcase binary (release mode) ---"
cargo build -p native-theme-gpui --example showcase-gpui --release
echo ""

echo "--- Capturing gpui frames ---"
for i in "${!THEMES[@]}"; do
    IFS=':' read -r theme variant icon_set icon_theme <<< "${THEMES[$i]}"
    frame_file="$GPUI_FRAME_DIR/frame-$(printf '%02d' "$i").png"
    echo "[$((i + 1))/${#THEMES[@]}] $theme $variant (icons: $icon_set${icon_theme:+/$icon_theme})"

    # Build CLI args
    cli_args=(--theme "$theme" --variant "$variant" --tab buttons --icon-set "$icon_set")
    if [ -n "$icon_theme" ]; then
        cli_args+=(--icon-theme "$icon_theme")
    fi

    cargo run -p native-theme-gpui --example showcase-gpui --release -- "${cli_args[@]}" &
    PID=$!

    sleep "$DELAY"

    spectacle -a -b -n -o "$frame_file"
    sleep 1

    kill "$PID" 2>/dev/null || true
    wait "$PID" 2>/dev/null || true
done

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

echo "--- Building egui showcase binary (release mode) ---"
cargo build -p native-theme-egui --example showcase-egui --release --all-features
echo ""

echo "--- Capturing egui frames ---"
for i in "${!THEMES[@]}"; do
    IFS=':' read -r theme variant icon_set icon_theme <<< "${THEMES[$i]}"
    frame_file="$EGUI_FRAME_DIR/frame-$(printf '%02d' "$i").png"
    echo "[$((i + 1))/${#THEMES[@]}] $theme $variant (icons: $icon_set${icon_theme:+/$icon_theme})"

    # The egui showcase takes the preset's own icon theme; no --icon-theme.
    cargo run -p native-theme-egui --example showcase-egui --release --all-features -- \
        --theme "$theme" --variant "$variant" --icon-set "$icon_set" \
        --tab buttons &
    PID=$!

    sleep "$DELAY"

    spectacle -a -b -n -o "$frame_file"
    sleep 1

    kill "$PID" 2>/dev/null || true
    wait "$PID" 2>/dev/null || true
done

echo ""
echo "--- Assembling egui GIF ---"
python3 "$SCRIPT_DIR/generate_gifs_spinners.py" \
    --theme-switching "$EGUI_FRAME_DIR" \
    --theme-switching-output "$EGUI_OUTPUT_DIR/theme-switching.gif"
echo ""
ls -lh "$EGUI_OUTPUT_DIR/theme-switching.gif"

# ── Cleanup ───────────────────────────────────────────────────────────

rm -rf "$ICED_FRAME_DIR" "$GPUI_FRAME_DIR" "$EGUI_FRAME_DIR"

echo ""
echo "=== Done: all three theme-switching GIFs generated ==="
