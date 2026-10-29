#!/bin/bash
# Runs inside the GNOME container (tools/linux-containers/gnome) under
# dbus-run-session, from the repository's root: builds gnome-host, starts the
# headless display named by $1 (mutter or xvfb), renders both schemes with
# each renderer in GNOME_HOST_RENDERERS (default "cairo gl") twice, and
# checks that the two runs wrote byte-identical files (the coreutils' sha256sum
# compares them: the image has no diffutils). Writes into $2:
#   <renderer>/run<N>/<scheme>/  gnome-host's PNGs, index.toml, its output
#   run.txt (each step's exit status, the CPU), build.log, display.log,
#   sha256.txt
set -u
display=$1
out=$2
renderers=${GNOME_HOST_RENDERERS:-"cairo gl"}
build=/tmp/gnome-host-build
mkdir -p "$out"
log="$out/run.txt"
note() { echo "$*" | tee -a "$log"; }
manifest() { (cd "$1" && find . -type f ! -name stdout.txt ! -name stderr.txt -print | sort | xargs sha256sum); }

note "cpu: $(grep -m1 '^model name' /proc/cpuinfo | cut -d: -f2-)"
note "cpu flags: $(grep -m1 '^flags' /proc/cpuinfo | tr ' ' '\n' | grep -E '^(avx|avx2|avx512f|fma|sse4_2)$' | tr '\n' ' ')"

meson setup "$build" tools/gnome-host >"$out/build.log" 2>&1 && meson compile -C "$build" >>"$out/build.log" 2>&1
status=$?
note "build: exit $status"
if [ $status -ne 0 ]; then cat "$out/build.log"; exit 1; fi

XDG_RUNTIME_DIR=$(mktemp -d)
export XDG_RUNTIME_DIR
chmod 700 "$XDG_RUNTIME_DIR"
export GSETTINGS_BACKEND=memory

case "$display" in
  mutter)
    mutter --help >"$out/mutter-help.txt" 2>&1
    mutter --headless --wayland --no-x11 --virtual-monitor 1280x720 --wayland-display=nt-wayland \
      >"$out/display.log" 2>&1 &
    server=$!
    socket="$XDG_RUNTIME_DIR/nt-wayland"
    export WAYLAND_DISPLAY=nt-wayland GDK_BACKEND=wayland
    unset DISPLAY
    ;;
  xvfb)
    Xvfb :99 -screen 0 1280x720x24 -nolisten tcp >"$out/display.log" 2>&1 &
    server=$!
    socket=/tmp/.X11-unix/X99
    export DISPLAY=:99 GDK_BACKEND=x11
    unset WAYLAND_DISPLAY
    ;;
  *)
    note "unknown display $display"
    exit 1
    ;;
esac
for _ in $(seq 1 100); do
  [ -S "$socket" ] && break
  kill -0 "$server" 2>/dev/null || break
  sleep 0.1
done
if [ ! -S "$socket" ]; then
  note "display $display: no socket $socket"
  cat "$out/display.log"
  exit 1
fi
note "display $display: up ($socket)"

failed=0
for run in 1 2; do
  for renderer in $renderers; do
    for scheme in light dark; do
      dir="$out/$renderer/run$run/$scheme"
      mkdir -p "$dir"
      timeout 120 /tmp/gnome-host-build/gnome-host --scheme "$scheme" --renderer "$renderer" --out "$dir" \
        >"$dir/stdout.txt" 2>"$dir/stderr.txt"
      status=$?
      note "gnome-host $renderer run $run $scheme: exit $status"
      [ $status -eq 0 ] || failed=1
    done
  done
done

for renderer in $renderers; do
  manifest "$out/$renderer/run1" >"$out/$renderer/run1.sha256"
  manifest "$out/$renderer/run2" >"$out/$renderer/run2.sha256"
  if [ -s "$out/$renderer/run1.sha256" ] && \
     [ "$(cat "$out/$renderer/run1.sha256")" = "$(cat "$out/$renderer/run2.sha256")" ]; then
    note "$renderer: run 1 and run 2 byte-identical ($(wc -l <"$out/$renderer/run1.sha256") files)"
  else
    note "$renderer: run 1 and run 2 differ"
    failed=1
  fi
done
(cd "$out" && find . \( -name '*.png' -o -name index.toml \) -print | sort | xargs sha256sum) >"$out/sha256.txt"

kill "$server" 2>/dev/null
wait "$server" 2>/dev/null
exit $failed
