#!/usr/bin/env bash
# The narrow per-task gate of the 0.8.0 work (plan, section 1, rule 7): the
# core crate's own steps, with the commands, flags and feature lists
# scripts/check_release.sh uses for them, for a task whose diff lies wholly in
# native-theme/src/next/, native-theme/tests/next_* and
# native-theme/tests/{vectors,snapshots}/.
#
#   scripts/check_next.sh [--target windows|macos]
#
# Steps: cargo fmt --all (formats in place, as check_release.sh does); clippy
# on native-theme's targets without features and with the Linux features,
# -D warnings; the strict panic lints on its library with both; its tests with
# both (cargo nextest where installed, then the doctests through cargo test
# --doc, as check_release.sh's cargo_tests); its two rustdoc runs,
# -D warnings; the panic-pattern scan of native-theme/src and the README
# versions check. `--target windows` or `--target macos` adds check_release.sh's
# cross-target clippy and strict-panic steps of native-theme for that target,
# whose cfg-gated code no host step compiles.
#
# It leaves to `check_release.sh --commit` the MSRV checks, the feature
# combinations and every other crate. Each step prints its name, result and
# wall time; the first failure prints the step's output and stops the run
# with a non-zero status; a total time ends a passing run.
#
# Scaffolding of the 0.8.0 work: deleted at the 0.8.0 cut.

set -uo pipefail

ROOT=$(git rev-parse --show-toplevel 2>/dev/null || pwd)
cd "$ROOT" || exit 1

TARGET=""
while [ $# -gt 0 ]; do
    case "$1" in
        --target)
            case "${2:-}" in
                windows) TARGET=x86_64-pc-windows-msvc ;;
                macos) TARGET=x86_64-apple-darwin ;;
                *)
                    echo "check_next.sh: --target takes windows or macos" >&2
                    exit 2
                    ;;
            esac
            shift 2
            ;;
        -h | --help)
            sed -n '2,25p' "$0" | sed 's/^# \{0,1\}//'
            exit 0
            ;;
        *)
            echo "check_next.sh: unknown argument: $1" >&2
            exit 2
            ;;
    esac
done

# check_release.sh's NT_FEATURES and NT_PLATFORM_FEATURES.
NT_FEATURES="kde,portal,system-icons,material-icons,lucide-icons,watch,svg-rasterize,system-fonts"
NT_PLATFORM_FEATURES="system-icons,material-icons,lucide-icons,watch,svg-rasterize,system-fonts"

HAVE_NEXTEST=""
if cargo nextest --version >/dev/null 2>&1; then
    HAVE_NEXTEST=1
fi

format_elapsed() {
    local s=$1
    if [ "$s" -ge 60 ]; then
        printf "%dm%02ds" $((s / 60)) $((s % 60))
    else
        printf "%ds" "$s"
    fi
}

# step <name> <command...>: runs the command with its output captured, prints
# one status line, and on failure the output and the command, then exits.
step() {
    local name="$1"
    shift
    local start=$SECONDS
    local output status
    output=$("$@" 2>&1)
    status=$?
    local elapsed
    elapsed=$(format_elapsed $((SECONDS - start)))
    if [ "$status" -eq 0 ]; then
        printf "ok    %-58s %s\n" "$name" "$elapsed"
    else
        printf "%s\n" "$output" >&2
        printf "FAIL  %-58s %s\n" "$name" "$elapsed"
        printf "      failed command: %s\n" "$*" >&2
        printf "check_next.sh: stopped at the first failure (total %s)\n" \
            "$(format_elapsed $((SECONDS - TOTAL_START)))" >&2
        exit 1
    fi
}

# check_release.sh's cargo_tests: cargo nextest where installed, then the
# doctests through cargo test --doc; cargo test where it is not.
tests() {
    if [[ ! $HAVE_NEXTEST ]]; then
        cargo test "$@"
        return
    fi
    local threads=()
    if [ -n "${RUST_TEST_THREADS:-}" ] && [ -z "${NEXTEST_TEST_THREADS:-}" ]; then
        threads=(--test-threads "$RUST_TEST_THREADS")
    fi
    cargo nextest run --no-tests=pass --status-level fail --final-status-level fail \
        --color never ${threads[@]+"${threads[@]}"} "$@" || return
    cargo test "$@" --doc
}

TOTAL_START=$SECONDS

step "cargo fmt (all crates)" cargo fmt --all
step "clippy (native-theme)" \
    cargo clippy -p native-theme --all-targets -- -D warnings
step "clippy (native-theme, Linux features)" \
    cargo clippy -p native-theme --all-targets --features "$NT_FEATURES" -- -D warnings
step "strict-panic (native-theme, Linux features)" \
    bash scripts/check_strict_panic.sh -p native-theme --lib --features "$NT_FEATURES"
step "strict-panic (native-theme, no features)" \
    bash scripts/check_strict_panic.sh -p native-theme --lib --no-default-features
if [ -n "$TARGET" ]; then
    if [ "$TARGET" = "x86_64-apple-darwin" ]; then
        platform_feature=macos
    else
        platform_feature=windows
    fi
    step "clippy, all features ($TARGET)" \
        cargo clippy --target "$TARGET" -p native-theme --all-targets --all-features -- -D warnings
    step "clippy, no default features ($TARGET)" \
        cargo clippy --target "$TARGET" -p native-theme --all-targets --no-default-features -- -D warnings
    step "strict-panic (native-theme, $platform_feature features, $TARGET)" \
        bash scripts/check_strict_panic.sh --target "$TARGET" -p native-theme --lib \
        --features "$platform_feature,$NT_PLATFORM_FEATURES"
    step "strict-panic (native-theme, no features, $TARGET)" \
        bash scripts/check_strict_panic.sh --target "$TARGET" -p native-theme --lib --no-default-features
fi
step "test (native-theme)" tests -p native-theme
step "test (native-theme, Linux features)" tests -p native-theme --features "$NT_FEATURES"
step "docs (native-theme)" \
    env RUSTDOCFLAGS="-D warnings" cargo doc -p native-theme --no-deps
step "docs, private items (native-theme, Linux features)" \
    env RUSTDOCFLAGS="-D warnings" \
    cargo doc -p native-theme --no-deps --document-private-items --features "$NT_FEATURES"
step "panic-pattern scan (native-theme/src)" \
    python3 scripts/check_panic_patterns.py native-theme/src
step "README versions and crates" python3 scripts/check_readme_versions.py

printf "check_next.sh: all steps passed (total %s)\n" "$(format_elapsed $((SECONDS - TOTAL_START)))"
