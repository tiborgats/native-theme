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
#   fonts/                 KH-reads: fonts.py's fonts.toml at the platform's DPI and
#                          at 96, fontconfig.txt, font-files.sha256, fc-list.tsv,
#                          fc-match-sorted.tsv
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

# KH-reads (the 0.8.0 measurement plan): the fonts Qt resolves under the kde
# platform theme and their metrics (fonts.py; once at the platform's DPI and
# once with QT_FONT_DPI=96), the Noto Sans and Hack files with their SHA-256,
# and fontconfig's sorted match (fc-match -s, what Qt's family fallbacks
# follow) per family and per language, its first 20 fonts each.
fonts="$out/fonts"
mkdir -p "$fonts/default-dpi" "$fonts/dpi-96"
home=$(mktemp -d)
mkdir -p "$home/config" "$home/data" "$home/cache"
cp "$out/schemes/BreezeLight.none/kdeglobals" "$home/config/kdeglobals"
for dpi in default-dpi dpi-96; do
  extra=()
  [ "$dpi" = dpi-96 ] && extra=(QT_FONT_DPI=96)
  timeout 60 env XDG_CONFIG_HOME="$home/config" XDG_DATA_HOME="$home/data" XDG_CACHE_HOME="$home/cache" \
    QT_QPA_PLATFORM=offscreen QT_QPA_PLATFORMTHEME=kde QT_FORCE_STDERR_LOGGING=1 "${extra[@]}" \
    python3 tools/kde-host/fonts.py "$fonts/$dpi" >"$fonts/$dpi/stdout.txt" 2>"$fonts/$dpi/stderr.txt"
  status=$?
  note "fonts.py $dpi: exit $status"
  [ $status -eq 0 ] || failed=1
done
rm -rf "$home"
{
  echo "fc-match: $(fc-match --version 2>&1 | head -n 1)"
  for family in "Noto Sans" "Noto Sans:style=Bold" "Hack" "sans-serif" "serif" "monospace"; do
    echo "file $family: $(fc-match -f '%{file}' "$family")"
  done
} >"$fonts/fontconfig.txt"
for family in "Noto Sans" "Hack"; do
  file=$(fc-match -f '%{file}' "$family")
  [ -n "$file" ] && sha256sum "$file" >>"$fonts/font-files.sha256"
done
fc-list --format '%{family[0]}\t%{style[0]}\t%{file}\n' | sort >"$fonts/fc-list.tsv"
{
  printf 'pattern\trank\tfamily\tstyle\tfile\n'
  for pattern in "Noto Sans" "Hack" "sans-serif" "serif" "monospace" "emoji"; do
    fc-match -s -f '%{family[0]}\t%{style[0]}\t%{file}\n' "$pattern" | head -n 20 | awk -v p="$pattern" '{ printf "%s\t%d\t%s\n", p, NR, $0 }'
  done
  for lang in en ja ko zh-cn zh-tw zh-hk ar fa he hi bn ta th lo km my si hy ka am el ru; do
    for family in "Noto Sans" "Hack"; do
      pattern="$family:lang=$lang"
      fc-match -s -f '%{family[0]}\t%{style[0]}\t%{file}\n' "$pattern" | head -n 20 | awk -v p="$pattern" '{ printf "%s\t%d\t%s\n", p, NR, $0 }'
    done
  done
} >"$fonts/fc-match-sorted.tsv"
note "fc-match: $(wc -l <"$fonts/fc-match-sorted.tsv") lines, fc-list: $(wc -l <"$fonts/fc-list.tsv") fonts"

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
