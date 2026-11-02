#!/bin/bash
# Runs inside the GNOME container (tools/linux-containers/gnome) under
# dbus-run-session, from the repository's root: builds gnome-vectors, starts
# mutter's headless display (as tools/gnome-host/run.sh does; GTK needs a
# display to style a widget), writes $1/adw_standalone.tsv, writes it a second
# time and checks that both are byte-identical (by sha256sum: the image has no
# diffutils). run.txt records each step's exit status; stderr.txt and
# stderr-again.txt each run's diagnostics.
set -u
out=$1
build=/tmp/gnome-vectors-build
mkdir -p "$out"
log="$out/run.txt"
note() { echo "$*" | tee -a "$log"; }

meson setup "$build" tools/gnome-vectors >"$out/build.log" 2>&1 && meson compile -C "$build" >>"$out/build.log" 2>&1
status=$?
note "build: exit $status"
if [ $status -ne 0 ]; then cat "$out/build.log"; exit 1; fi

XDG_RUNTIME_DIR=$(mktemp -d)
export XDG_RUNTIME_DIR
chmod 700 "$XDG_RUNTIME_DIR"
export GSETTINGS_BACKEND=memory
mutter --headless --wayland --no-x11 --virtual-monitor 1280x720 --wayland-display=nt-wayland \
  >"$out/display.log" 2>&1 &
server=$!
socket="$XDG_RUNTIME_DIR/nt-wayland"
export WAYLAND_DISPLAY=nt-wayland GDK_BACKEND=wayland
unset DISPLAY
for _ in $(seq 1 100); do
  [ -S "$socket" ] && break
  kill -0 "$server" 2>/dev/null || break
  sleep 0.1
done
if [ ! -S "$socket" ]; then
  note "display mutter: no socket $socket"
  cat "$out/display.log"
  exit 1
fi
note "display mutter: up ($socket)"

failed=0
timeout 120 "$build/gnome-vectors" "$out/adw_standalone.tsv" 2>"$out/stderr.txt"
status=$?
note "gnome-vectors: exit $status"
[ $status -eq 0 ] || failed=1
timeout 120 "$build/gnome-vectors" "$build/adw_standalone-again.tsv" 2>"$out/stderr-again.txt"
status=$?
note "gnome-vectors, second run: exit $status"
[ $status -eq 0 ] || failed=1
cat "$out/stderr.txt"
if [ -s "$out/adw_standalone.tsv" ] && \
   [ "$(sha256sum <"$out/adw_standalone.tsv")" = "$(sha256sum <"$build/adw_standalone-again.tsv")" ]; then
  note "two runs byte-identical"
else
  note "two runs differ"
  failed=1
fi
note "rows: $(grep -vc '^#' "$out/adw_standalone.tsv") (with the header line)"
sha256sum "$out/adw_standalone.tsv" | tee -a "$log"

kill "$server" 2>/dev/null
wait "$server" 2>/dev/null
exit $failed
