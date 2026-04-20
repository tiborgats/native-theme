#!/usr/bin/env bash
# Render Mermaid (.mmd) sources in docs/assets/ to matching .svg files.
#
# Uses mermaid-cli via npx — no global install required.
# Run this manually after editing any .mmd file.
#
# Usage: ./scripts/render-diagrams.sh

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
ASSETS_DIR="$PROJECT_ROOT/docs/assets"

if [ ! -d "$ASSETS_DIR" ]; then
    echo "error: $ASSETS_DIR does not exist" >&2
    exit 1
fi

shopt -s nullglob
mmd_files=("$ASSETS_DIR"/*.mmd)

if [ ${#mmd_files[@]} -eq 0 ]; then
    echo "No .mmd files in $ASSETS_DIR — nothing to render."
    exit 0
fi

for src in "${mmd_files[@]}"; do
    out="${src%.mmd}.svg"
    echo "→ rendering $(basename "$src") → $(basename "$out")"
    npx --yes @mermaid-js/mermaid-cli \
        --input "$src" \
        --output "$out" \
        --backgroundColor transparent
done

echo "Done."
