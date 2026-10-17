#!/usr/bin/env bash
# Sourced by the Linux capture scripts (generate_screenshots_{iced,gpui,egui}.sh,
# generate_gifs_theme_switching.sh): `capture_showcases` runs a showcase once
# per entry and captures its own window with spectacle (`capture_showcase`), and
# fails unless each capture is of that window at the showcases' default size.
#
# spectacle captures the active window, and the window a script launched need
# not be the active one: KWin's focus stealing prevention can keep a new window
# behind the one the user works in. So before capturing, a KWin script makes
# the showcase's window the active one, found by its app id: a showcase run with
# `--capture` takes the app id `showcase-{egui,gpui,iced}-capture-<pid>`, which
# no other window has.
#
# spectacle captures the active window with the frame KWin draws around it,
# whose size is the decoration theme's, so the check takes a second capture of
# the same window without its decoration and shadow (`-e -S`): that one is the
# window's content, and must be exactly the default size times the scale factor
# of the output the window is on, and must appear pixel for pixel inside the
# first capture, which is larger in both directions since it holds the frame
# (only the pixels KWin's rounded corners blend differently in the two are not
# compared); the showcase is stopped during both captures, so they hold the
# same frame. A capture of another window, or of the showcase's window in
# another state, fails the script, with both files and their sizes.
#
# Requires: spectacle, kscreen-doctor, qdbus6 (KDE Plasma 6), dbus-monitor and
# dbus-send (D-Bus), python3 with Pillow.

# The showcases' default window size in logical pixels: `WINDOW_SIZE` in
# connectors/native-theme-egui/examples/showcase-egui/main.rs and
# connectors/native-theme-iced/examples/showcase-iced.rs, `WINDOW_SIZE_PX` in
# connectors/native-theme-gpui/examples/showcase-gpui/main.rs.
SHOWCASE_WINDOW_WIDTH=1280
SHOWCASE_WINDOW_HEIGHT=720

# How long, in seconds, a showcase's window may take to appear and become the
# active window.
SHOWCASE_WINDOW_TIMEOUT=20

# The D-Bus interface of the KWin script's report. Nothing implements it: the
# script's call goes to the bus itself, which answers it with an error, and
# kwin_probe reads the call's argument off `dbus-monitor`.
KWIN_PROBE_INTERFACE=org.nativetheme.CaptureProbe

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

# kwin_probe APP_ID ACTIVATE: runs a KWin script that counts the windows whose
# app id is APP_ID and, when ACTIVATE is `true` and there is exactly one, makes
# it the active window. Prints "COUNT ACTIVE CLASS": COUNT the windows found,
# ACTIVE 1 when the active window is one of them (0 otherwise), CLASS the active
# window's resource class.
#
# The script is loaded, run and unloaded through KWin's scripting D-Bus
# interface: org.kde.kwin.Scripting.loadScript(filePath, pluginName) and
# unloadScript(pluginName) at /Scripting, org.kde.kwin.Script.run() at
# /Scripting/Script<id>, whose reply comes once the script has run
# (https://invent.kde.org/plasma/kwin/-/blob/master/src/scripting/scripting.h,
# https://invent.kde.org/plasma/kwin/-/blob/master/src/scripting/scripting.cpp).
# The script uses the scripting API's workspace.stackingOrder,
# workspace.activeWindow (read-write), Window.resourceClass,
# Window.desktopFileName and callDBus
# (https://develop.kde.org/docs/plasma/kwin/api/). On Wayland KWin sets a
# window's resourceClass and desktopFileName to its xdg-toplevel app id.
kwin_probe() {
    local app_id="$1" activate="$2" token dir log monitor id report i
    if [[ ! "$app_id" =~ ^[A-Za-z0-9._-]+$ ]]; then
        echo "ERROR: kwin_probe: app id '$app_id' holds characters it does not quote" >&2
        return 1
    fi
    token="native-theme-capture-$$-$RANDOM$RANDOM"
    dir="$(mktemp -d)"
    log="$dir/monitor.log"
    dbus-monitor --session "type='method_call',interface='$KWIN_PROBE_INTERFACE'" \
        > "$log" 2>&1 &
    monitor=$!
    # The monitor is listening once it logs a call of our own.
    for i in $(seq 50); do
        dbus-send --session --type=method_call --dest=org.freedesktop.DBus \
            /org/freedesktop/DBus "$KWIN_PROBE_INTERFACE.Report" \
            "string:$token ready" 2>/dev/null || true
        grep -qF "\"$token ready\"" "$log" && break
        sleep 0.1
    done
    if ! grep -qF "\"$token ready\"" "$log"; then
        kill "$monitor" 2>/dev/null || true
        rm -rf "$dir"
        echo "ERROR: dbus-monitor logs no call on the session bus" >&2
        return 1
    fi
    cat > "$dir/probe.js" <<EOF
const appId = "$app_id";
const found = workspace.stackingOrder.filter(
    (w) => w.resourceClass === appId || w.desktopFileName === appId);
if ($activate && found.length === 1) {
    workspace.activeWindow = found[0];
}
const active = workspace.activeWindow;
callDBus("org.freedesktop.DBus", "/org/freedesktop/DBus",
    "$KWIN_PROBE_INTERFACE", "Report",
    "$token " + found.length + " " + (found.includes(active) ? 1 : 0)
        + " " + (active ? active.resourceClass : "(none)"));
EOF
    report=""
    if id="$(qdbus6 org.kde.KWin /Scripting org.kde.kwin.Scripting.loadScript \
            "$dir/probe.js" "$token")" && [ "$id" -ge 0 ] 2>/dev/null \
        && qdbus6 org.kde.KWin "/Scripting/Script$id" org.kde.kwin.Script.run >/dev/null; then
        for i in $(seq 50); do
            report="$(sed -n "s/^ *string \"$token \([0-9]* [01] .*\)\"\$/\1/p" "$log")"
            [ -n "$report" ] && break
            sleep 0.1
        done
    fi
    qdbus6 org.kde.KWin /Scripting org.kde.kwin.Scripting.unloadScript "$token" >/dev/null || true
    kill "$monitor" 2>/dev/null || true
    wait "$monitor" 2>/dev/null || true
    rm -rf "$dir"
    if [ -z "$report" ]; then
        echo "ERROR: the KWin script that looks for the window '$app_id' did not report" >&2
        return 1
    fi
    echo "$report"
}

# activate_window PID APP_ID: makes the one window whose app id is APP_ID, of
# the process PID, the active window, and waits until it is. Fails when PID
# exits, when no such window appears, when there are several, or when it does
# not become active, within SHOWCASE_WINDOW_TIMEOUT seconds.
activate_window() {
    local pid="$1" app_id="$2" count=0 active=0 class="(none)" deadline report
    deadline=$((SECONDS + SHOWCASE_WINDOW_TIMEOUT))
    while :; do
        if ! kill -0 "$pid" 2>/dev/null; then
            echo "ERROR: the showcase (pid $pid) exited before its window '$app_id' could be captured" >&2
            return 1
        fi
        report="$(kwin_probe "$app_id" true)" || return 1
        read -r count active class <<< "$report"
        if [ "$count" -gt 1 ]; then
            echo "ERROR: $count windows have the app id '$app_id'; the capture needs exactly one" >&2
            return 1
        fi
        if [ "$count" -eq 1 ] && [ "$active" -eq 1 ]; then
            return 0
        fi
        if [ "$SECONDS" -ge "$deadline" ]; then
            break
        fi
        sleep 0.5
    done
    if [ "$count" -eq 0 ]; then
        echo "ERROR: no window with the app id '$app_id' appeared within ${SHOWCASE_WINDOW_TIMEOUT} s" >&2
    else
        echo "ERROR: the window '$app_id' did not become the active window within ${SHOWCASE_WINDOW_TIMEOUT} s (the active window is '$class')" >&2
    fi
    return 1
}

# require_active APP_ID: fails unless the window whose app id is APP_ID is still
# the active window.
require_active() {
    local app_id="$1" count active class report
    report="$(kwin_probe "$app_id" false)" || return 1
    read -r count active class <<< "$report"
    if [ "$count" -ne 1 ] || [ "$active" -ne 1 ]; then
        echo "ERROR: the window '$app_id' is no longer the active window (the active window is '$class'): another window took the focus during the capture" >&2
        return 1
    fi
}

# check_capture CAPTURE CONTENT: fails unless CONTENT, a capture of a window
# without its decoration and shadow, is the showcases' default size at the
# active output's scale, and appears pixel for pixel inside CAPTURE, a capture
# of the same window with its frame.
check_capture() {
    local capture="$1" content="$2" scale
    scale="$(output_scale)"
    python3 - "$capture" "$content" "$scale" \
        "$SHOWCASE_WINDOW_WIDTH" "$SHOWCASE_WINDOW_HEIGHT" <<'EOF'
import sys
from PIL import Image

capture, content, scale, width, height = sys.argv[1:]
scale = float(scale)
framed = Image.open(capture).convert("RGBA")
inner = Image.open(content).convert("RGBA")
w, h = framed.size
cw, ch = inner.size
# Half away from zero, as the showcases' Rust `f64::round` rounds.
ew, eh = int(int(width) * scale + 0.5), int(int(height) * scale + 0.5)
files = f"{capture} ({w}x{h} px) and {content} ({cw}x{ch} px)"
if (cw, ch) != (ew, eh):
    sys.exit(
        f"ERROR: {files}: the window's content is {cw}x{ch} px, expected {ew}x{eh} "
        f"({width}x{height} at scale {scale}): the window did not open at its default size, "
        "or the capture is of another window"
    )
if w <= cw or h <= ch:
    sys.exit(f"ERROR: {files}: the capture is no larger than the content: it holds no frame")

framed_bytes, inner_bytes = framed.tobytes(), inner.tobytes()
stride, inner_stride = w * 4, cw * 4


def inner_row(y):
    return inner_bytes[y * inner_stride:(y + 1) * inner_stride]


# KWin rounds the window's corners, and the two captures blend the pixels the
# rounding cuts differently: the content capture leaves them translucent, the
# framed capture blends them with the frame. So a pixel within a corner square
# (a twentieth of the content's longer side) that is translucent in either
# capture is not compared; every other pixel must be equal, and the content
# must be opaque outside its corners.
corner = max(cw, ch) // 20
alpha = inner.getchannel("A")
for box in (
    (0, 0, corner, corner),
    (cw - corner, 0, cw, corner),
    (0, ch - corner, corner, ch),
    (cw - corner, ch - corner, cw, ch),
):
    alpha.paste(255, box)
stray = sum(alpha.histogram()[:255])
if stray:
    x0, y0, x1, y1 = alpha.point(lambda v: 255 if v < 255 else 0).getbbox()
    sys.exit(
        f"ERROR: {files}: the content has {stray} translucent pixels outside its corners "
        f"(within {x0},{y0} to {x1 - 1},{y1 - 1}): it is not the showcase's opaque window"
    )
corner_columns = [x for x in range(cw) if x < corner or x >= cw - corner]
skipped = 0


def matches_at(start):
    global skipped
    skipped = 0
    for y in range(ch):
        at = start + y * stride
        framed_row = framed_bytes[at:at + inner_stride]
        row = inner_row(y)
        if corner <= y < ch - corner:
            if framed_row != row:
                return False
            continue
        middle = slice(corner * 4, (cw - corner) * 4)
        if framed_row[middle] != row[middle]:
            return False
        for x in corner_columns:
            a, b = framed_row[x * 4:x * 4 + 4], row[x * 4:x * 4 + 4]
            if a[3] != 255 or b[3] != 255:
                skipped += 1
            elif a != b:
                return False
    return True


# Every offset (dx, dy) at which the content fits inside the capture is a
# candidate; the search looks for one row of the content, clear of the corners,
# in the capture's rows, then compares every row at each place it is found.
# The row is the one with the most distinct pixels, so it is found in few
# places.
def distinct_pixels(y):
    row = inner_row(y)
    return len({row[x:x + 4] for x in range(0, inner_stride, 4)})


probe = max(range(corner, ch - corner, max(1, ch // 64)), key=distinct_pixels)
needle = inner_row(probe)
found = []
for dy in range(h - ch + 1):
    row = framed_bytes[(dy + probe) * stride:(dy + probe + 1) * stride]
    at = row.find(needle)
    while at != -1 and at <= stride - inner_stride:
        if at % 4 == 0 and matches_at(dy * stride + at):
            found.append((at // 4, dy))
        at = row.find(needle, at + 1)
if not found:
    sys.exit(
        f"ERROR: {files}: the content does not appear pixel for pixel inside the capture: "
        "the two are not of the same window in the same state"
    )
dx, dy = found[0]
matches_at(dy * stride + dx * 4)
print(
    f"  identity check: {w}x{h} px holds the {cw}x{ch} px content at +{dx}+{dy} "
    f"(scale {scale}; {skipped} translucent corner pixels not compared)"
)
EOF
}

# capture_showcase KIND PID CAPTURE: captures the window of the showcase KIND
# (egui, gpui or iced) run with `--capture` as process PID, with its frame, to
# CAPTURE; fails unless check_capture passes and the window stayed the active
# one throughout. On Unix `cargo run` replaces its own process with the program
# it runs, so the PID of a `cargo run … &` is the showcase's.
#
# The showcase is stopped (SIGSTOP) during the two captures, so both hold the
# same frame of its window: a window that animates (gpui's loading button
# spins) would otherwise differ between them. KWin keeps showing the last frame
# the window committed. stop_showcase, which a caller's EXIT trap calls, sends
# the showcase SIGCONT before it kills it: a script interrupted between the two
# signals would otherwise leave it stopped, where SIGTERM waits until it runs
# again.
capture_showcase() {
    local kind="$1" pid="$2" capture="$3" app_id content status=0
    app_id="showcase-$kind-capture-$pid"
    activate_window "$pid" "$app_id" || return 1
    sleep 1
    content="$(mktemp --suffix=.png)"
    if ! kill -STOP "$pid"; then
        echo "ERROR: the showcase (pid $pid) exited before its window '$app_id' could be captured" >&2
        return 1
    fi
    spectacle -a -b -n -o "$capture" || status=1
    sleep 1
    if [ "$status" -eq 0 ]; then
        spectacle -a -b -n -e -S -o "$content" || status=1
        sleep 1
    fi
    kill -CONT "$pid" || status=1
    if [ "$status" -ne 0 ] || ! require_active "$app_id"; then
        rm -f "$content"
        return 1
    fi
    if ! check_capture "$capture" "$content"; then
        echo "  (the content capture $content is kept for inspection)" >&2
        return 1
    fi
    rm -f "$content"
}

# How long, in seconds, capture_showcases lets a showcase run before it makes
# its window the active one and captures it: for the window to open and the
# showcase to lay out its page and load its fonts and icons.
SHOWCASE_START_DELAY=3

# The presets the Linux screenshots capture (generate_screenshots_*.sh), which
# the release asset run's parity step also compares at rest: three, dark and
# light, each with its icon set, as `theme:variant:icon-set:icon-theme`.
# kde-breeze names its freedesktop icon theme per variant, breeze and
# breeze-dark (native-theme/src/presets/kde-breeze.toml:16, :478).
SCREENSHOT_ENTRIES=(
    "kde-breeze:dark:freedesktop:breeze-dark"
    "kde-breeze:light:freedesktop:breeze"
    "material-3:dark:material:"
    "material-3:light:material:"
    "catppuccin-mocha:dark:lucide:"
    "catppuccin-mocha:light:lucide:"
)

# The showcase capture_showcases started, while it runs.
SHOWCASE_PID=""

# stop_showcase: stops the showcase capture_showcases started, if one runs. It
# is continued first: capture_showcase stops it (SIGSTOP) during its captures,
# and SIGTERM waits until a stopped process runs again. A caller's EXIT trap
# calls it, so a script that fails or is interrupted leaves no showcase behind;
# `set -e` holds inside a trap too, so each kill of a showcase that already
# exited needs its `|| true`.
stop_showcase() {
    if [ -n "$SHOWCASE_PID" ]; then
        kill -CONT "$SHOWCASE_PID" 2>/dev/null || true
        kill "$SHOWCASE_PID" 2>/dev/null || true
        wait "$SHOWCASE_PID" 2>/dev/null || true
        SHOWCASE_PID=""
    fi
}

# wait_for_dump PID FILE: waits until FILE, the layout dump of the showcase run
# as process PID (`--dump-layout`), holds a whole JSON object. Fails when PID
# exits first, or when none is written within SHOWCASE_WINDOW_TIMEOUT seconds.
wait_for_dump() {
    local pid="$1" file="$2" deadline
    deadline=$((SECONDS + SHOWCASE_WINDOW_TIMEOUT))
    until python3 -c 'import json, sys; json.load(open(sys.argv[1]))' "$file" 2>/dev/null; do
        if ! kill -0 "$pid" 2>/dev/null; then
            echo "ERROR: the showcase (pid $pid) exited before it wrote its layout dump $file" >&2
            return 1
        fi
        if [ "$SECONDS" -ge "$deadline" ]; then
            echo "ERROR: the showcase (pid $pid) wrote no layout dump $file within ${SHOWCASE_WINDOW_TIMEOUT} s" >&2
            return 1
        fi
        sleep 0.5
    done
}

# capture_showcases KIND FEATURES OUTPUT DUMPS ENTRY...: builds the showcase
# KIND (egui, gpui or iced) in release mode with the cargo flags FEATURES, then
# runs it once per ENTRY, `theme:variant:icon-set:icon-theme` (the icon theme
# may be empty), on its Basic page with `--capture`, and captures its window to
# OUTPUT (capture_showcase), in which `{theme}`, `{variant}` and `{index}` (the
# entry's, from 00) stand for the entry's. The caller fails on the first
# capture that fails. `--capture` opens the showcase at its default size
# whatever the desktop remembers for its window, with an app id of its own.
#
# The gpui showcase takes a freedesktop icon theme as `--icon-theme`; the iced
# and egui showcases have no such flag and take an installed theme's name as
# their `--icon-set`, so an entry's icon theme, where it has one, is theirs.
# FEATURES differ by showcase, as the callers and the screenshot workflow build
# them: iced with `--features iced_aw` (its Extra page), egui with
# `--all-features`, gpui with its defaults ("").
#
# With a directory DUMPS ("" for none), each run also writes its layout dump
# (`--dump-layout`, the chrome at rest wherever the pointer is) to
# DUMPS/KIND-THEME-VARIANT.json, and the capture and the content's offset in it
# (the identity check's `+dx+dy`) beside it as .png and .offset, the files
# scripts/check_showcase_parity.py reads from a run's directory.
capture_showcases() {
    local kind="$1" features="$2" output="$3" dumps="$4"
    shift 4
    local crate="native-theme-$kind" example="showcase-$kind" total=$# index=0
    local feature_args=() args entry theme variant icon_set icon_theme file name dump report offset
    read -ra feature_args <<< "$features"

    echo "--- Building the $kind showcase (release mode) ---"
    cargo build -p "$crate" --example "$example" --release "${feature_args[@]}"
    echo ""

    # A spectacle left running would answer the captures' D-Bus calls itself.
    pkill spectacle 2>/dev/null || true

    echo "--- Capturing the $kind showcase ---"
    echo "WARNING: Do not interact with the desktop during capture."
    echo ""
    for entry in "$@"; do
        IFS=':' read -r theme variant icon_set icon_theme <<< "$entry"
        file="${output//\{theme\}/$theme}"
        file="${file//\{variant\}/$variant}"
        file="${file//\{index\}/$(printf '%02d' "$index")}"
        index=$((index + 1))
        echo "[$index/$total] $theme $variant (icons: $icon_set${icon_theme:+/$icon_theme}) -> $(basename "$file")"

        args=(--theme "$theme" --variant "$variant" --tab basic --capture)
        if [ "$kind" = gpui ]; then
            args+=(--icon-set "$icon_set")
            if [ -n "$icon_theme" ]; then
                args+=(--icon-theme "$icon_theme")
            fi
        else
            args+=(--icon-set "${icon_theme:-$icon_set}")
        fi
        dump=""
        if [ -n "$dumps" ]; then
            name="$kind-$theme-$variant"
            dump="$dumps/$name.json"
            rm -f "$dump" "$dumps/$name.png" "$dumps/$name.offset"
            args+=(--dump-layout "$dump")
        fi

        cargo run -p "$crate" --example "$example" --release "${feature_args[@]}" -- "${args[@]}" &
        SHOWCASE_PID=$!
        sleep "$SHOWCASE_START_DELAY"
        report="$(capture_showcase "$kind" "$SHOWCASE_PID" "$file")" || return 1
        printf '%s\n' "$report"
        if [ -n "$dump" ]; then
            wait_for_dump "$SHOWCASE_PID" "$dump" || return 1
            offset="$(printf '%s\n' "$report" | sed -n 's/.* content at \(+[0-9]*+[0-9]*\) (.*/\1/p')"
            if [ -z "$offset" ]; then
                echo "ERROR: the identity check of $file printed no offset of the content" >&2
                return 1
            fi
            cp "$file" "$dumps/$name.png"
            printf '%s\n' "$offset" > "$dumps/$name.offset"
        fi
        stop_showcase
    done
}
