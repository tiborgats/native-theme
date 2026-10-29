#!/bin/bash
# Records what Plasma's colour scheme applier writes into kdeglobals for each
# (scheme file, accent) pair: the accent run's measured facts (the 0.8.0 plan,
# D2; the specification's S17, "The accent run"). Runs inside the KDE
# container (tools/linux-containers/kde), from the repository's root; started
# outside a D-Bus session, it starts its own with dbus-run-session.
#
#   scripts/measure_kde_accent.sh <out-dir> <pairs.tsv>
#   scripts/measure_kde_accent.sh <out-dir> <scheme-file> <accent|-> [<scheme-file> <accent|-> ...]
#
# pairs.tsv holds one pair per line, tab-separated, `#` starting a comment:
#   id  scheme-file  accent  predecessor  same-as
# where scheme-file is a .colors path or the name of an installed scheme
# (/usr/share/color-schemes/<name>.colors); accent is any colour
# `plasma-apply-colorscheme --accent-color` takes, or `-` for none;
# predecessor is the scheme applied first, `-` for the default (BreezeDark,
# or BreezeClassic for BreezeDark itself) or `none`; same-as is `-` or the id
# of an earlier pair whose kdeglobals this pair's must equal byte for byte
# (a check that the predecessor leaves nothing behind). The second form
# measures each pair with the defaults, its id <scheme>.<accent or none>.
#
# Each pair runs in fresh XDG_CONFIG_HOME, XDG_DATA_HOME and XDG_CACHE_HOME
# directories, with QT_QPA_PLATFORM=offscreen:
#   1. `plasma-apply-colorscheme <predecessor>`: a fresh configuration
#      already names BreezeLight (the settings' default), and the tool writes
#      nothing for the scheme it names, so another scheme is applied first;
#   2. a check that kdeglobals stores no AccentColor, since a scheme applied
#      without an accent takes a stored one;
#   3. `plasma-apply-colorscheme <scheme>`;
#   4. for an accent, `plasma-apply-colorscheme --accent-color <accent>`: the
#      tool takes a scheme or an accent, not both, and applies the accent to
#      the scheme kdeglobals names.
# Each step must exit 0 and change kdeglobals; steps 1 and 3 must leave
# ColorScheme naming their scheme; step 3 must leave ColorSchemeHash equal to
# the scheme file's SHA-1. A scheme file outside /usr/share/color-schemes is
# copied into the pair's data directory under its own name.
#
# Writes into <out-dir>: <id>/<n>-<step>.kdeglobals after each step,
# <id>/kdeglobals (the result), <id>/<n>-<step>.out (the tool's output),
# manifest.toml (per pair: the inputs, each step's command, exit status and
# kdeglobals' SHA-256 before and after, the checks; the container's builds)
# and run.txt. Exits 1 when any check fails.
set -u

if [ -z "${DBUS_SESSION_BUS_ADDRESS:-}" ]; then
  exec dbus-run-session -- bash "$0" "$@"
fi

if [ $# -lt 2 ]; then
  echo "usage: $0 <out-dir> <pairs.tsv> | <out-dir> <scheme-file> <accent|-> ..." >&2
  exit 2
fi
out=$1
shift
mkdir -p "$out"
out=$(cd "$out" && pwd)
log="$out/run.txt"
manifest="$out/manifest.toml"
work=$(mktemp -d)
system_schemes=/usr/share/color-schemes
: >"$log"
note() { echo "$*" | tee -a "$log"; }
toml() { local s=${1//\\/\\\\}; s=${s//\"/\\\"}; printf '"%s"' "$s"; }
sha() { if [ -f "$1" ]; then sha256sum <"$1" | cut -d' ' -f1; else echo absent; fi; }

# The value of <key> in [<group>] of an INI file; fails when the key is absent.
ini_get() {
  [ -f "$1" ] || return 1
  awk -v group="[$2]" -v key="$3" '
    $0 == group { inside = 1; next }
    /^\[/ { inside = 0 }
    inside && index($0, key "=") == 1 { print substr($0, length(key) + 2); found = 1; exit }
    END { exit !found }' "$1"
}

pairs=()
if [ $# -eq 1 ]; then
  while IFS= read -r line || [ -n "$line" ]; do
    case "$line" in ''|'#'*) continue ;; esac
    pairs+=("$line")
  done <"$1"
  input="$1"
else
  if [ $(($# % 2)) -ne 0 ]; then
    echo "$0: scheme files and accents come in pairs" >&2
    exit 2
  fi
  while [ $# -gt 0 ]; do
    base=$(basename "$1" .colors)
    if [ "$2" = - ]; then suffix=none; else suffix=${2#\#}; fi
    pairs+=("$base.$suffix"$'\t'"$1"$'\t'"$2"$'\t-\t-')
    shift 2
  done
  input=arguments
fi

{
  echo "# Written by scripts/measure_kde_accent.sh: what plasma-apply-colorscheme wrote into"
  echo "# kdeglobals for each (scheme file, accent) pair, each pair in a fresh configuration."
  echo
  echo "[run]"
  echo "input = $(toml "$input")"
  echo "date = $(toml "$(date -u +%Y-%m-%dT%H:%M:%SZ)")"
  echo "snapshot = $(toml "$(cat /usr/share/native-theme/snapshot.txt 2>/dev/null || echo unknown)")"
  echo "environment = $(toml "QT_QPA_PLATFORM=offscreen; fresh XDG_CONFIG_HOME, XDG_DATA_HOME, XDG_CACHE_HOME per pair; dbus-run-session")"
  echo
  echo "[builds]"
  for package in plasma-workspace breeze kcolorscheme kconfig kconfigwidgets kguiaddons qt6-base; do
    version=$(pacman -Q "$package" 2>/dev/null | cut -d' ' -f2)
    echo "$package = $(toml "${version:-unknown}")"
  done
} >"$manifest"

failed=0
declare -A results
for line in "${pairs[@]}"; do
  IFS=$'\t' read -r id scheme_file accent predecessor same_as <<<"$line"
  accent=${accent:--}
  predecessor=${predecessor:--}
  same_as=${same_as:--}
  problems=()

  case "$scheme_file" in
    */*|*.colors) ;;
    *) scheme_file="$system_schemes/$scheme_file.colors" ;;
  esac
  scheme=$(basename "$scheme_file" .colors)
  if [ "$predecessor" = - ]; then
    if [ "$scheme" = BreezeDark ]; then predecessor=BreezeClassic; else predecessor=BreezeDark; fi
  fi

  dir="$out/$id"
  pair="$work/$id"
  mkdir -p "$dir" "$pair/config" "$pair/data/color-schemes" "$pair/cache"
  kdeglobals="$pair/config/kdeglobals"
  if [ ! -f "$scheme_file" ]; then
    problems+=("no scheme file $scheme_file")
  elif [ "$(dirname "$(readlink -f "$scheme_file")")" != "$system_schemes" ]; then
    cp "$scheme_file" "$pair/data/color-schemes/$scheme.colors"
  fi
  if [ "$predecessor" = "$scheme" ]; then
    problems+=("the predecessor is the scheme itself")
  fi
  scheme_sha1=$(sha1sum <"$scheme_file" 2>/dev/null | cut -d' ' -f1)

  {
    echo
    echo "[[pair]]"
    echo "id = $(toml "$id")"
    echo "scheme_file = $(toml "$scheme_file")"
    echo "scheme = $(toml "$scheme")"
    echo "scheme_sha256 = $(toml "$(sha "$scheme_file")")"
    echo "scheme_sha1 = $(toml "$scheme_sha1")"
    echo "accent = $(toml "$accent")"
    echo "predecessor = $(toml "$predecessor")"
    echo "same_as = $(toml "$same_as")"
  } >>"$manifest"

  steps=()
  [ "$predecessor" != none ] && steps+=("predecessor")
  steps+=("scheme")
  [ "$accent" != - ] && steps+=("accent")

  stored_before=false
  n=0
  step_toml=""
  for step in "${steps[@]}"; do
    n=$((n + 1))
    case "$step" in
      predecessor) args=("$predecessor"); expect=$predecessor ;;
      scheme) args=("$scheme"); expect=$scheme ;;
      accent) args=(--accent-color "$accent"); expect="" ;;
    esac
    if [ "$step" = scheme ] && ini_get "$kdeglobals" General AccentColor >/dev/null; then
      stored_before=true
      problems+=("AccentColor stored before the scheme's apply")
    fi
    before=$(sha "$kdeglobals")
    command=(env XDG_CONFIG_HOME="$pair/config" XDG_DATA_HOME="$pair/data" XDG_CACHE_HOME="$pair/cache"
             QT_QPA_PLATFORM=offscreen plasma-apply-colorscheme "${args[@]}")
    start=$(date +%s%N)
    "${command[@]}" >"$dir/$n-$step.out" 2>&1
    status=$?
    milliseconds=$((($(date +%s%N) - start) / 1000000))
    after=$(sha "$kdeglobals")
    [ -f "$kdeglobals" ] && cp "$kdeglobals" "$dir/$n-$step.kdeglobals"
    changed=false
    [ "$before" != "$after" ] && changed=true
    [ $status -eq 0 ] || problems+=("$step: exit $status")
    [ $changed = true ] || problems+=("$step: kdeglobals unchanged")
    if [ -n "$expect" ]; then
      named=$(ini_get "$kdeglobals" General ColorScheme || echo absent)
      [ "$named" = "$expect" ] || problems+=("$step: ColorScheme is $named, not $expect")
    fi
    if [ "$step" = scheme ]; then
      hash=$(ini_get "$kdeglobals" General ColorSchemeHash || echo absent)
      [ "$hash" = "$scheme_sha1" ] || problems+=("scheme: ColorSchemeHash $hash is not the file's SHA-1 $scheme_sha1")
    fi
    shown="plasma-apply-colorscheme ${args[*]}"
    step_toml+=$'\n'"[[pair.step]]"$'\n'
    step_toml+="name = $(toml "$step")"$'\n'
    step_toml+="command = $(toml "$shown")"$'\n'
    step_toml+="exit = $status"$'\n'
    step_toml+="milliseconds = $milliseconds"$'\n'
    step_toml+="output = $(toml "$id/$n-$step.out")"$'\n'
    step_toml+="kdeglobals = $(toml "$id/$n-$step.kdeglobals")"$'\n'
    step_toml+="sha256_before = $(toml "$before")"$'\n'
    step_toml+="sha256_after = $(toml "$after")"$'\n'
    step_toml+="changed = $changed"$'\n'
    note "$id: $shown: exit $status, kdeglobals changed: $changed"
  done

  result=$(sha "$kdeglobals")
  [ -f "$kdeglobals" ] && cp "$kdeglobals" "$dir/kdeglobals"
  stored_after=$(ini_get "$kdeglobals" General AccentColor || echo absent)
  if [ "$same_as" != - ]; then
    if [ -z "${results[$same_as]:-}" ]; then
      problems+=("same-as: no earlier pair $same_as")
    elif [ "${results[$same_as]}" != "$result" ]; then
      problems+=("kdeglobals differs from $same_as's")
    fi
  fi
  results[$id]=$result

  ok=true
  [ ${#problems[@]} -eq 0 ] || { ok=false; failed=1; }
  {
    echo "kdeglobals = $(toml "$id/kdeglobals")"
    echo "kdeglobals_sha256 = $(toml "$result")"
    echo "color_scheme_hash = $(toml "$(ini_get "$kdeglobals" General ColorSchemeHash || echo absent)")"
    echo "accent_stored_before_scheme_apply = $stored_before"
    echo "accent_color_after = $(toml "$stored_after")"
    echo "ok = $ok"
    printf 'problems = ['
    sep=""
    for problem in "${problems[@]}"; do printf '%s%s' "$sep" "$(toml "$problem")"; sep=", "; done
    printf ']\n'
    printf '%s' "$step_toml"
  } >>"$manifest"
  note "$id: ok = $ok${problems[0]:+ (${problems[*]})}"
done

rm -rf "$work"
note "pairs: ${#pairs[@]}, failed: $failed"
exit $failed
