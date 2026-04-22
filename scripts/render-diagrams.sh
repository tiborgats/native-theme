#!/usr/bin/env bash
# Render Graphviz (.dot) sources in docs/assets/ to matching .svg files.
#
# Uses the `dot` binary from Graphviz — install via your package manager
# (e.g. `pacman -S graphviz`, `apt install graphviz`, `brew install graphviz`).
# Run this manually after editing any .dot file.
#
# `dot -Tsvg` writes text as <text font-family="…"> attributes, so viewers
# without the referenced font fall back to a generic face. "Fuzzy Bubbles"
# (the hand-drawn label on "Your app") is not a system font, so this script
# post-processes the SVG to embed a character-subsetted @font-face rule
# inline — keeping the output self-contained and identical everywhere.
#
# Dependencies:
#   - graphviz                          (`dot`, required)
#   - fontconfig                        (`fc-match`, required for embed)
#   - python-fonttools                  (`pyftsubset` + fontTools, required for embed)
#   - "Fuzzy Bubbles" font installed    (from Google Fonts, required for embed)
# Without the embed prerequisites the script still renders; the label just
# falls back to a sans-serif face in viewers that lack the font.
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

if ! command -v dot >/dev/null 2>&1; then
    echo "error: 'dot' not found — install Graphviz (https://graphviz.org/download/)" >&2
    exit 1
fi

shopt -s nullglob
dot_files=("$ASSETS_DIR"/*.dot)

if [ ${#dot_files[@]} -eq 0 ]; then
    echo "No .dot files in $ASSETS_DIR — nothing to render."
    exit 0
fi

embed_fuzzy_bubbles() {
    local svg="$1"
    grep -q 'font-family="Fuzzy Bubbles"' "$svg" || return 0

    if ! command -v python3 >/dev/null 2>&1 || ! python3 -c 'import fontTools' 2>/dev/null; then
        echo "  ! python-fonttools not available — 'Fuzzy Bubbles' will use viewer fallback" >&2
        return 0
    fi

    if ! command -v fc-match >/dev/null 2>&1; then
        echo "  ! fc-match not found — 'Fuzzy Bubbles' will use viewer fallback" >&2
        return 0
    fi

    local font family
    font=$(fc-match -f "%{file}" "Fuzzy Bubbles:style=Bold" 2>/dev/null || true)
    family=$(fc-match -f "%{family}" "Fuzzy Bubbles:style=Bold" 2>/dev/null || true)
    if [ "$family" != "Fuzzy Bubbles" ] || [ ! -f "$font" ]; then
        echo "  ! 'Fuzzy Bubbles Bold' not installed — viewer fallback will be used" >&2
        return 0
    fi

    python3 - "$svg" "$font" <<'PYEOF'
import base64, io, re, sys
from fontTools.subset import Subsetter
from fontTools.ttLib import TTFont

svg_path, font_path = sys.argv[1], sys.argv[2]
with open(svg_path) as f:
    svg = f.read()

chars = ''.join(re.findall(r'font-family="Fuzzy Bubbles"[^>]*>([^<]*)<', svg))
if not chars:
    sys.exit(0)

font = TTFont(font_path)
sub = Subsetter()
sub.populate(text=chars)
sub.subset(font)
font.flavor = 'woff'  # woff uses zlib, no brotli dep; woff2 would need brotli
buf = io.BytesIO()
font.save(buf)
b64 = base64.b64encode(buf.getvalue()).decode('ascii')

style = (
    "<defs><style>@font-face{"
    "font-family:'Fuzzy Bubbles';"
    "font-weight:bold;"
    f"src:url(data:font/woff;base64,{b64}) format('woff');"
    "}</style></defs>"
)
svg = re.sub(r'(<svg[^>]*>)', lambda m: m.group(1) + style, svg, count=1)
with open(svg_path, 'w') as f:
    f.write(svg)
PYEOF
    echo "  embedded 'Fuzzy Bubbles' subset"
}

for src in "${dot_files[@]}"; do
    out="${src%.dot}.svg"
    echo "→ rendering $(basename "$src") → $(basename "$out")"
    dot -Tsvg "$src" -o "$out"
    embed_fuzzy_bubbles "$out"
done

echo "Done."
