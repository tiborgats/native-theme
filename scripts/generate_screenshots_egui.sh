#!/usr/bin/env bash
set -euo pipefail

# Screenshot automation for the native-theme egui showcase
# Captures Linux-native theme presets on the Buttons tab using spectacle on KDE Wayland
#
# Uses spectacle for external window capture (same as gpui and iced) to include
# window decorations (title bar, buttons, borders) in screenshots.
#
# NOTE: On macOS/Windows, use the showcase's built-in self-capture, which
# writes the frame egui rendered (ViewportCommand::Screenshot) as a PNG:
#   cargo run -p native-theme-egui --example showcase-egui --release --all-features -- \
#     --theme macos-sonoma --variant light --icon-set system \
#     --tab buttons --screenshot connectors/native-theme-egui/docs/assets/macos-macos-sonoma-light.png
# This script uses spectacle for Linux (KDE Wayland) local captures.
#
# --all-features: the showcase is built with every connector feature, as the
# screenshot workflow builds it (the connector spec's Cargo.toml section).

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
# `--capture` opens the showcase at its default size whatever the desktop
# remembers for its window; check_capture fails the script unless each capture
# is of a window at that size.
source "$SCRIPT_DIR/capture_size.sh"
OUTPUT_DIR="$PROJECT_ROOT/connectors/native-theme-egui/docs/assets"
DELAY=3

# Linux-native presets with matching icon sets (3 themes × dark+light).
# Format: theme:variant:icon-set. kde-breeze names its freedesktop theme per
# variant, breeze / breeze-dark (kde-breeze.toml:9, :317); `--icon-set <theme>`
# picks that installed theme, while `freedesktop` would mean the desktop's own.
THEMES=(
    "kde-breeze:dark:breeze-dark"
    "kde-breeze:light:breeze"
    "material:dark:material"
    "material:light:material"
    "catppuccin-mocha:dark:lucide"
    "catppuccin-mocha:light:lucide"
)

echo "=== Generating egui showcase screenshots ==="
echo "Presets: 3 (dark + light each)"
echo "Total screenshots: ${#THEMES[@]}"
echo ""

# Pre-build showcase binary to avoid compile delays during capture loop
echo "--- Building showcase binary (release mode) ---"
cd "$PROJECT_ROOT"
cargo build -p native-theme-egui --example showcase-egui --release --all-features
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
    output_file="$OUTPUT_DIR/linux-${theme}-${variant}.png"
    count=$((count + 1))
    echo "[$count/$total] Capturing: $theme $variant (icons: $icon_set) -> $(basename "$output_file")"

    cargo run -p native-theme-egui --example showcase-egui --release --all-features -- \
        --theme "$theme" --variant "$variant" --icon-set "$icon_set" \
        --tab buttons --capture &
    PID=$!

    sleep "$DELAY"

    spectacle -a -b -n -o "$output_file"
    sleep 1
    check_capture "$output_file"

    kill "$PID" 2>/dev/null || true
    wait "$PID" 2>/dev/null || true
done

echo ""
echo "=== Screenshot generation complete ==="
echo "Generated $(ls "$OUTPUT_DIR"/linux-*.png 2>/dev/null | wc -l) screenshots in $OUTPUT_DIR"
