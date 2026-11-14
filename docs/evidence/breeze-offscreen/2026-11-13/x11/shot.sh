#!/usr/bin/env bash
# usage: shot.sh <showcase-binary> <Xft.dpi> <out.png>   (private Xvfb on :97; nothing on the real desktop)
set -u
S=$(dirname "$0"); BIN=$1; DPI=$2; OUT=$3
Xvfb :97 -screen 0 1920x1080x24 -nolisten tcp -noreset >/dev/null 2>&1 & XP=$!
sleep 1
printf 'Xft.dpi: %s\n' "$DPI" > "$S/res"; DISPLAY=:97 xrdb -nocpp -merge "$S/res"
env -u WAYLAND_DISPLAY -u WAYLAND_SOCKET DISPLAY=:97 XDG_SESSION_TYPE=x11 XDG_CONFIG_HOME="$S/cfg" XDG_CONFIG_DIRS="$S/empty" \
  "$BIN" --theme kde-breeze --variant dark --capture > "$OUT.log" 2>&1 & AP=$!
sleep 20
DISPLAY=:97 import -window root "$OUT"
kill $AP 2>/dev/null; wait $AP 2>/dev/null
kill $XP 2>/dev/null; wait $XP 2>/dev/null
