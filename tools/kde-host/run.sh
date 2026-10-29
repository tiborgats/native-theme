#!/bin/bash
# Runs inside the KDE container (tools/linux-containers/kde) under
# dbus-run-session, from the repository's root: writes BreezeLight's and
# BreezeDark's kdeglobals with scripts/measure_kde_accent.sh (no accent), then
# renders both schemes with the QML host (buttons.qml, the `qml` runtime) and
# the Qt Widgets host (widgets.py, the container's PySide6) twice, and checks
# that the two runs wrote byte-identical files (sha256sum). Each render runs
# with QT_QPA_PLATFORM=offscreen, the `kde` platform theme (plasma-integration,
# which reads the palette and fonts from kdeglobals), Qt Quick's software
# renderer, and fresh XDG_CONFIG_HOME (the scheme's kdeglobals with
# [KDE] AnimationDurationFactor=0, KDE's "instant" animation speed, so Breeze
# draws each state at its end and not part way through a transition),
# XDG_DATA_HOME and XDG_CACHE_HOME, and Qt's messages on stderr (without a
# terminal, Qt sends them to the system log instead). Writes into $1:
#   schemes/               the accent script's output for the two schemes
#   run<N>/<scheme>/qml/   buttons.qml's PNGs, index.txt, its output
#   run<N>/<scheme>/widgets/  widgets.py's PNGs, index.toml, its output
#   run.txt (each step's exit status, the CPU, the tools), packages.txt,
#   sha256.txt
set -u
out=$1
mkdir -p "$out"
out=$(cd "$out" && pwd)
log="$out/run.txt"
: >"$log"
note() { echo "$*" | tee -a "$log"; }
manifest() { (cd "$1" && find . -type f ! -name stdout.txt ! -name stderr.txt -print | sort | xargs sha256sum); }

note "cpu: $(grep -m1 '^model name' /proc/cpuinfo | cut -d: -f2-)"
cp /usr/share/native-theme/packages.txt "$out/packages.txt"
qml=$(command -v qml6 || command -v qml || echo /usr/lib/qt6/bin/qml)
note "qml: $qml ($(QT_QPA_PLATFORM=offscreen "$qml" --version 2>&1 | head -n 1))"
QT_QPA_PLATFORM=offscreen "$qml" --help >"$out/qml-help.txt" 2>&1

bash scripts/measure_kde_accent.sh "$out/schemes" BreezeLight - BreezeDark - >/dev/null 2>&1
status=$?
note "measure_kde_accent.sh: exit $status"
if [ $status -ne 0 ]; then cat "$out/schemes/run.txt"; chmod -R a+rX "$out"; exit 1; fi

XDG_RUNTIME_DIR=$(mktemp -d)
export XDG_RUNTIME_DIR
chmod 700 "$XDG_RUNTIME_DIR"

failed=0
for run in 1 2; do
  for scheme in light dark; do
    case "$scheme" in
      light) kdeglobals="$out/schemes/BreezeLight.none/kdeglobals" ;;
      dark) kdeglobals="$out/schemes/BreezeDark.none/kdeglobals" ;;
    esac
    home=$(mktemp -d)
    mkdir -p "$home/config" "$home/data" "$home/cache"
    cp "$kdeglobals" "$home/config/kdeglobals"
    kwriteconfig6 --file "$home/config/kdeglobals" --group KDE --key AnimationDurationFactor 0
    note "kwriteconfig6 AnimationDurationFactor 0, run $run $scheme: exit $?"
    environment=(env XDG_CONFIG_HOME="$home/config" XDG_DATA_HOME="$home/data" XDG_CACHE_HOME="$home/cache"
                 QT_QPA_PLATFORM=offscreen QT_QPA_PLATFORMTHEME=kde QT_QUICK_CONTROLS_STYLE=org.kde.desktop
                 QT_QUICK_BACKEND=software QML_DISABLE_DISK_CACHE=1 QT_MESSAGE_PATTERN='%{message}'
                 QT_FORCE_STDERR_LOGGING=1)

    dir="$out/run$run/$scheme/qml"
    mkdir -p "$dir"
    timeout 120 "${environment[@]}" "$qml" --apptype widget tools/kde-host/buttons.qml -- "$dir" \
      >"$dir/stdout.txt" 2>"$dir/stderr.txt"
    status=$?
    grep -h '^index: ' "$dir/stdout.txt" "$dir/stderr.txt" | sed 's/^index: //' >"$dir/index.txt"
    note "buttons.qml run $run $scheme: exit $status, $(find "$dir" -name '*.png' | wc -l) PNGs"
    [ $status -eq 0 ] || failed=1

    dir="$out/run$run/$scheme/widgets"
    mkdir -p "$dir"
    timeout 120 "${environment[@]}" python3 tools/kde-host/widgets.py "$dir" \
      >"$dir/stdout.txt" 2>"$dir/stderr.txt"
    status=$?
    note "widgets.py run $run $scheme: exit $status, $(find "$dir" -name '*.png' | wc -l) PNGs"
    [ $status -eq 0 ] || failed=1
    rm -rf "$home"
  done
done

manifest "$out/run1" >"$out/run1.sha256"
manifest "$out/run2" >"$out/run2.sha256"
if [ -s "$out/run1.sha256" ] && [ "$(cat "$out/run1.sha256")" = "$(cat "$out/run2.sha256")" ]; then
  note "run 1 and run 2 byte-identical ($(wc -l <"$out/run1.sha256") files)"
else
  note "run 1 and run 2 differ"
  failed=1
fi
(cd "$out" && find . -name '*.png' -print | sort | xargs sha256sum) >"$out/sha256.txt"
chmod -R a+rX "$out"
exit $failed
