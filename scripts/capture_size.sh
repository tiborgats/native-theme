#!/usr/bin/env bash
# Sourced by the Linux capture scripts (generate_screenshots_{iced,gpui,egui}.sh,
# generate_gifs_theme_switching.sh): checks that a spectacle capture of a
# showcase's window is of a window at the showcases' default size.
#
# spectacle captures the active window with the frame KWin draws around it,
# whose size is the decoration theme's, so the check takes a second capture of
# the same window without its decoration and shadow (`-e -S`): that one is the
# window's content, and must be exactly the default size times the scale factor
# of the output the window is on. The first capture must be larger in both
# directions, since it holds the frame. Any other size fails the script, with
# the measured and the expected sizes.
#
# Requires: spectacle, kscreen-doctor, qdbus6 (KDE Plasma 6), python3.

# The showcases' default window size in logical pixels: `WINDOW_SIZE` in
# connectors/native-theme-egui/examples/showcase-egui/main.rs and
# connectors/native-theme-iced/examples/showcase-iced.rs, `WINDOW_SIZE_PX` in
# connectors/native-theme-gpui/examples/showcase-gpui/main.rs.
SHOWCASE_WINDOW_WIDTH=1280
SHOWCASE_WINDOW_HEIGHT=720

# png_size FILE: prints "WIDTH HEIGHT", read from the PNG's IHDR chunk.
png_size() {
    python3 - "$1" <<'EOF'
import struct, sys
with open(sys.argv[1], "rb") as f:
    head = f.read(24)
if len(head) < 24 or head[:8] != b"\x89PNG\r\n\x1a\n" or head[12:16] != b"IHDR":
    sys.exit(f"{sys.argv[1]}: not a PNG")
print(*struct.unpack(">II", head[16:24]))
EOF
}

# output_scale: prints the scale factor of KWin's active output.
output_scale() {
    local output
    output="$(qdbus6 org.kde.KWin /KWin org.kde.KWin.activeOutputName)"
    kscreen-doctor -j | python3 -c '
import json, sys
name = sys.argv[1]
scales = [o["scale"] for o in json.load(sys.stdin)["outputs"] if o["name"] == name]
if len(scales) != 1:
    sys.exit(f"kscreen-doctor lists no single output named {name!r}")
print(scales[0])
' "$output"
}

# check_capture CAPTURE: fails unless CAPTURE, a `spectacle -a` capture of the
# active window, is of a window whose content is the default size. Takes the
# content capture itself, so the window must still be the active one.
check_capture() {
    local capture="$1" content scale size content_size
    content="$(mktemp --suffix=.png)"
    spectacle -a -b -n -e -S -o "$content"
    sleep 1
    scale="$(output_scale)"
    size="$(png_size "$capture")"
    content_size="$(png_size "$content")"
    rm -f "$content"
    python3 - "$capture" "$size" "$content_size" "$scale" \
        "$SHOWCASE_WINDOW_WIDTH" "$SHOWCASE_WINDOW_HEIGHT" <<'EOF'
import sys
capture, size, content, scale, width, height = sys.argv[1:]
w, h = map(int, size.split())
cw, ch = map(int, content.split())
scale = float(scale)
# Half away from zero, as the showcases' Rust `f64::round` rounds.
ew, eh = int(int(width) * scale + 0.5), int(int(height) * scale + 0.5)
errors = []
if (cw, ch) != (ew, eh):
    errors.append(
        f"the window's content is {cw}x{ch} px, expected {ew}x{eh} "
        f"({width}x{height} at scale {scale}): the window did not open at its default size"
    )
if w <= cw or h <= ch:
    errors.append(f"the capture is {w}x{h} px, no larger than the content: it holds no frame")
if errors:
    sys.exit(f"ERROR: {capture}: " + "; ".join(errors))
print(f"  size check: {w}x{h} px around a {cw}x{ch} content at scale {scale}")
EOF
}
