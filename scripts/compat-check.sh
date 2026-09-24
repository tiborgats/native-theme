#!/usr/bin/env bash
# Tested-compatible upstream versions: docs/COMPATIBILITY.toml
#
# Each connector's README states the upstream set it has been verified
# against. The claim is only worth what the run behind it was, so the run is
# here:
#
#   compat-check.sh run [gpui|iced]  resolve the newest upstream release on a
#                                    throwaway lockfile, run that connector's
#                                    gates on it, and -- only if they all pass
#                                    -- stamp docs/COMPATIBILITY.toml with the
#                                    versions the lockfile ended up with and
#                                    rewrite the README's Verified line from it
#   compat-check.sh check            exit 0 if the stamp exists, is
#                                    well-formed, and each connector's sources
#                                    are the ones it was verified from; exit 1
#                                    with a message on stdout otherwise
#   compat-check.sh hash <connector> print a connector's sources hash at HEAD
#
# The hash covers the git object ids of that connector's Cargo.toml, src,
# examples and tests: a connector that has changed has not been verified, the
# rule scripts/asset-stamp.sh applies to the sources its screenshots came
# from. `git rev-parse HEAD:<path>` yields a tree or blob id, so `check` works
# on a depth-1 checkout and needs no network.
#
# There is deliberately no `write` verb. A stamp written without the run it
# names would be exactly the unchecked claim this script exists to remove.
#
# `run` needs the network and mutates Cargo.lock; the committed lockfile pins
# the floors, which is what CI tests with --locked, so it is restored
# unconditionally on the way out. Neither CI nor the nightly dependency canary
# runs `run` -- the canary tests the newest upstream its own way.
set -euo pipefail

ROOT=$(git rev-parse --show-toplevel)
STAMP_REL="docs/COMPATIBILITY.toml"
STAMP="$ROOT/$STAMP_REL"
CONNECTORS=(gpui iced)

cd "$ROOT"

# The crate a connector shorthand names, and the order the stamp's tables keep.
crate_of() {
    case "$1" in
        gpui) echo native-theme-gpui ;;
        iced) echo native-theme-iced ;;
        *) return 1 ;;
    esac
}

# The upstream family whose newest release the run resolves: every crate that
# a break in this connector could come from, including the ones it reaches
# through another (gpui-kit-assets through gpui-kit, iced_core through iced).
family_of() {
    case "$1" in
        gpui) echo "gpui-base gpui-component gpui-kit gpui-kit-assets gpui-pre" ;;
        iced) echo "iced iced_aw iced_core iced_test iced_widget" ;;
        *) return 1 ;;
    esac
}

# The paths whose content the verdict is about.
source_paths() {
    local dir="connectors/$(crate_of "$1")"
    printf '%s\n' "$dir/Cargo.toml" "$dir/src" "$dir/examples" "$dir/tests"
}

readme_of() {
    echo "connectors/$(crate_of "$1")/README.md"
}

sources_hash() {
    local connector="$1" rev="${2:-HEAD}" p id ids=""
    while read -r p; do
        id=$(git -C "$ROOT" rev-parse --verify --quiet "$rev:$p") || {
            echo "compat-check: $p not found at $rev" >&2
            return 1
        }
        ids+="$id"$'\n'
    done < <(source_paths "$connector")
    printf '%s' "$ids" | sha256sum | cut -d' ' -f1
}

# ── The stamp ────────────────────────────────────────────────────────────────

# One table of the stamp, its sub-tables included, as it stands on disk.
stamp_block() {
    [ -f "$STAMP" ] || return 0
    awk -v head="[$1]" -v prefix="[$1." '
        substr($0, 1, 1) == "[" { inside = ($0 == head || index($0, prefix) == 1) }
        inside { print }
    ' "$STAMP"
}

stamp_field() {
    stamp_block "$1" | sed -n "s/^$2 = \"\(.*\)\"/\1/p" | head -1
}

# `Verified against a 1, b 2 and c 3 on <date>.` -- the sentence the README
# carries between its compat markers, and the one the connectors' own
# `src/compat.rs` renders from this stamp to check that it has not been
# edited by hand.
verified_sentence() {
    printf '%s' "$1" | awk -v date="$2" '
        {
            gsub(/"/, "")
            sub(/ = /, " ")
            pairs[n++] = $0
        }
        END {
            for (i = 0; i < n; i++) {
                if (i == 0)          out = pairs[i]
                else if (i == n - 1) out = out " and " pairs[i]
                else                 out = out ", " pairs[i]
            }
            printf "Verified against %s on %s.\n", out, date
        }'
}

write_stamp() {
    local crate="$1" commit="$2" today="$3" hash="$4" versions="$5"
    local fresh blocks=() name block
    fresh=$(printf '[%s]\ncommit = "%s"\ngenerated = "%s"\nsources = "%s"\n\n[%s.verified]\n%s' \
        "$crate" "$commit" "$today" "$hash" "$crate" "$versions")
    # Read the other connector's block before the redirection below truncates
    # the file: a run is about one connector and leaves the other's claim as
    # it found it.
    for name in native-theme-gpui native-theme-iced; do
        if [ "$name" = "$crate" ]; then
            blocks+=("$fresh")
            continue
        fi
        block=$(stamp_block "$name")
        if [ -n "$block" ]; then
            blocks+=("$block")
        fi
    done
    mkdir -p "$(dirname "$STAMP")"
    {
        cat <<'EOF'
# Upstream versions each connector has been verified against.
#
# Written by `scripts/compat-check.sh run`, which resolves the newest upstream
# release of a connector's family on a throwaway lockfile, runs that
# connector's tests, clippy, documentation and the widget-coverage script on
# it, and records what the lockfile resolved only when every one of them
# passed. Read by pre-release-check.sh and by each connector's own
# `src/compat.rs`, which requires the README's Verified line to be this file's.
#
# `sources` is a SHA-256 over the git object ids of that connector's
# Cargo.toml, src, examples and tests. A connector whose sources have changed
# has not been verified, whatever this file says about the set.
# Do not edit by hand.
EOF
        for block in "${blocks[@]}"; do
            printf '\n%s\n' "$block"
        done
    } >"$STAMP"
}

write_readme() {
    local connector="$1" versions="$2" today="$3"
    local file sentence
    file="$ROOT/$(readme_of "$connector")"
    if ! grep -q '^<!-- compat:begin -->$' "$file" ||
        ! grep -q '^<!-- compat:end -->$' "$file"; then
        echo "compat-check: $(readme_of "$connector") carries no <!-- compat:begin --> / <!-- compat:end --> markers to write the Verified line between" >&2
        exit 1
    fi
    sentence=$(verified_sentence "$versions" "$today")
    awk -v line="$sentence" '
        $0 == "<!-- compat:begin -->" { print; print line; inside = 1; next }
        $0 == "<!-- compat:end -->"   { inside = 0 }
        !inside { print }
    ' "$file" >"$file.new"
    mv "$file.new" "$file"
}

# ── The run ──────────────────────────────────────────────────────────────────

LOCK_BACKUP=""
CURRENT_CRATE=""

restore_lock() {
    if [ -n "$LOCK_BACKUP" ] && [ -f "$LOCK_BACKUP" ]; then
        cp "$LOCK_BACKUP" "$ROOT/Cargo.lock"
        rm -f "$LOCK_BACKUP"
        LOCK_BACKUP=""
    fi
}

run_gate() {
    local label="$1"
    shift
    printf '  → %s\n' "$label"
    if ! "$@"; then
        echo "compat-check: $CURRENT_CRATE is NOT verified: $label failed on the updated lockfile" >&2
        exit 1
    fi
}

verify_clean() {
    local connector="$1" paths=() p dirty
    while read -r p; do paths+=("$p"); done < <(source_paths "$connector")
    dirty=$(git -C "$ROOT" status --porcelain -- "${paths[@]}")
    if [ -n "$dirty" ]; then
        echo "compat-check: uncommitted changes in $(crate_of "$connector")'s sources; the stamp records what HEAD holds, so commit them first:" >&2
        printf '%s\n' "$dirty" | head -10 >&2
        exit 1
    fi
}

# The version the lockfile resolved for a crate.
lock_version() {
    awk -F' = ' -v want="\"$1\"" '
        $1 == "name"                      { package = $2 }
        $1 == "version" && package == want { gsub(/"/, "", $2); print $2; exit }
    ' "$ROOT/Cargo.lock"
}

gates_gpui() {
    run_gate "tests" cargo test -p native-theme-gpui --locked
    run_gate "clippy" cargo clippy -p native-theme-gpui --all-targets --locked -- -D warnings
    run_gate "documentation" env RUSTDOCFLAGS="-D warnings" \
        cargo doc -p native-theme-gpui --no-deps --locked
    run_gate "widget coverage" python3 scripts/check-widget-coverage.py
}

gates_iced() {
    run_gate "tests" cargo test -p native-theme-iced --locked
    run_gate "tests (no default features)" \
        cargo test -p native-theme-iced --locked --no-default-features
    run_gate "tests (iced_aw)" cargo test -p native-theme-iced --locked --features iced_aw
    run_gate "clippy" \
        cargo clippy -p native-theme-iced --all-targets --all-features --locked -- -D warnings
    run_gate "documentation" env RUSTDOCFLAGS="-D warnings" \
        cargo doc -p native-theme-iced --no-deps --locked
    run_gate "documentation (all features)" env RUSTDOCFLAGS="-D warnings" \
        cargo doc -p native-theme-iced --no-deps --locked --all-features
    run_gate "widget coverage" python3 scripts/check-widget-coverage.py
}

run_connector() {
    local connector="$1" crate update_args=() c version versions="" hash commit today
    crate=$(crate_of "$connector")
    CURRENT_CRATE="$crate"
    verify_clean "$connector"

    printf '== %s: the newest %s upstream ==\n' "$crate" "$connector"
    LOCK_BACKUP=$(mktemp)
    cp "$ROOT/Cargo.lock" "$LOCK_BACKUP"

    for c in $(family_of "$connector"); do update_args+=(-p "$c"); done
    if ! cargo update "${update_args[@]}"; then
        echo "compat-check: cargo update failed. The run resolves the newest upstream release and needs the registry; it is never skipped, because a claim made offline would be about the versions already in the lockfile." >&2
        exit 1
    fi

    "gates_$connector"

    # The versions the gates actually ran against, read out of the throwaway
    # lockfile -- never from the registry, and never typed.
    for c in $(family_of "$connector"); do
        version=$(lock_version "$c")
        if [ -z "$version" ]; then
            echo "compat-check: $c is not in Cargo.lock, so the run cannot say which version it verified" >&2
            exit 1
        fi
        versions+="$c = \"$version\""$'\n'
    done

    hash=$(sources_hash "$connector" HEAD)
    commit=$(git -C "$ROOT" rev-parse HEAD)
    today=$(date -u +%Y-%m-%d)
    write_stamp "$crate" "$commit" "$today" "$hash" "$versions"
    write_readme "$connector" "$versions" "$today"
    restore_lock

    printf '%s verified: %s\n' "$crate" "$(verified_sentence "$versions" "$today")"
    printf 'wrote %s and %s (commit %s, sources %s)\n\n' \
        "$STAMP_REL" "$(readme_of "$connector")" "${commit:0:7}" "${hash:0:12}"
}

cmd_run() {
    local requested=("$@") connector
    if [ ${#requested[@]} -eq 0 ]; then
        requested=("${CONNECTORS[@]}")
    fi
    for connector in "${requested[@]}"; do
        if ! crate_of "$connector" >/dev/null; then
            echo "compat-check: unknown connector '$connector' (gpui, iced)" >&2
            exit 2
        fi
    done
    trap restore_lock EXIT INT TERM
    for connector in "${requested[@]}"; do
        run_connector "$connector"
    done
}

cmd_check() {
    if [ ! -f "$STAMP" ]; then
        echo "no $STAMP_REL: neither connector states an upstream set it has been verified against; run ./scripts/compat-check.sh run"
        return 1
    fi
    local connector crate recorded generated pairs current stale=() verified=()
    for connector in "${CONNECTORS[@]}"; do
        crate=$(crate_of "$connector")
        recorded=$(stamp_field "$crate" sources)
        generated=$(stamp_field "$crate" generated)
        pairs=$(stamp_block "$crate.verified" | grep -c ' = "' || true)
        if [ -z "$recorded" ] || [ -z "$generated" ] || [ "$pairs" -eq 0 ]; then
            stale+=("$crate has no verified set in $STAMP_REL")
            continue
        fi
        current=$(sources_hash "$connector" HEAD)
        if [ "$recorded" != "$current" ]; then
            stale+=("$crate has changed since it was verified on $generated")
            continue
        fi
        verified+=("$crate $generated")
    done
    if [ ${#stale[@]} -eq 0 ]; then
        echo "connectors verified against their upstream sets (${verified[*]})"
        return 0
    fi
    local joined
    joined=$(printf '%s; ' "${stale[@]}")
    echo "compatibility claims are stale: ${joined%; }. ./scripts/compat-check.sh run re-verifies and refreshes them"
    return 1
}

cmd_hash() {
    if ! crate_of "${1:-}" >/dev/null 2>&1; then
        echo "usage: $0 hash <gpui|iced>" >&2
        exit 2
    fi
    sources_hash "$1" HEAD
}

case "${1:-}" in
    run)
        shift
        cmd_run "$@"
        ;;
    check) cmd_check ;;
    hash)
        shift
        cmd_hash "${1:-}"
        ;;
    *)
        echo "usage: $0 {run [gpui|iced] | check | hash <gpui|iced>}" >&2
        exit 2
        ;;
esac
