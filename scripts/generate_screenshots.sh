#!/usr/bin/env bash
set -euo pipefail

# Screenshot automation for native-theme showcases
# Captures all 17 themes x 2 variants x 2 toolkits = 68 screenshots

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
OUTPUT_DIR="$PROJECT_ROOT/docs/assets"

THEMES=(
    default kde-breeze adwaita windows-11 macos-sonoma material ios
    catppuccin-latte catppuccin-frappe catppuccin-macchiato catppuccin-mocha
    nord dracula gruvbox solarized tokyo-night one-dark
)
VARIANTS=( light dark )
TOOLKITS=( gpui iced )

# Consistent icon theme across all captures
ICON_SET="material"

# Seconds to wait for window rendering before capture
DELAY=3

echo "=== Generating showcase screenshots ==="
echo "Themes: ${#THEMES[@]}, Variants: ${#VARIANTS[@]}, Toolkits: ${#TOOLKITS[@]}"
echo "Total screenshots: $(( ${#THEMES[@]} * ${#VARIANTS[@]} * ${#TOOLKITS[@]} ))"
echo ""

# Pre-build both showcase binaries to avoid compile delays during capture loop
echo "--- Building showcase binaries (release mode) ---"
cd "$PROJECT_ROOT"
cargo build -p native-theme-gpui --example showcase --release
cargo build -p native-theme-iced --example showcase --release
echo ""

# Ensure output directory exists
mkdir -p "$OUTPUT_DIR"

# Kill any existing spectacle instances to avoid D-Bus singleton issues
pkill spectacle 2>/dev/null || true
sleep 1

echo "--- Capturing screenshots ---"
count=0
total=$(( ${#THEMES[@]} * ${#VARIANTS[@]} * ${#TOOLKITS[@]} ))

for toolkit in "${TOOLKITS[@]}"; do
    for theme in "${THEMES[@]}"; do
        for variant in "${VARIANTS[@]}"; do
            output_file="$OUTPUT_DIR/${toolkit}-${theme}-${variant}.png"
            count=$((count + 1))
            echo "[$count/$total] Capturing: $toolkit $theme $variant -> $(basename "$output_file")"

            # Launch showcase with CLI args
            cargo run -p "native-theme-${toolkit}" --example showcase --release -- \
                --theme "$theme" --variant "$variant" --icon-set "$ICON_SET" &
            PID=$!

            # Wait for window to render
            sleep "$DELAY"

            # Capture active window
            spectacle -i -a -b -n -e -o "$output_file"

            # Brief pause for spectacle to finish writing
            sleep 1

            # Kill the showcase
            kill "$PID" 2>/dev/null || true
            wait "$PID" 2>/dev/null || true
        done
    done
done

echo ""
echo "=== Screenshot generation complete ==="
echo "Generated $(ls "$OUTPUT_DIR"/*.png 2>/dev/null | wc -l) screenshots in $OUTPUT_DIR"
