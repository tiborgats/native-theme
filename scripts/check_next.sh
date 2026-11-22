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
# -D warnings; the strict panic lints on its library with both; the two MSRV
# checks of native-theme in check_release.sh's MSRV section
# (scripts/msrv_checks.sh: cargo check on the workspace's rust-version,
# RUSTFLAGS=-Dwarnings, --locked, in its own build folder, which is deleted
# when the run ends), with its default features and with all its features;
# its tests with both feature sets (cargo nextest where installed, then the
# doctests through cargo test --doc, as check_release.sh's cargo_tests; each
# run fails below run_tests' minimum of one passed test); from the plan's M0,
# step 4 on, cargo check of the gpui connector's targets and its parity test
# (next::parity), with the features check_release.sh tests it with; the core
# crate's two rustdoc runs, -D warnings; the panic-pattern scan of
# native-theme/src and the README versions check. `--target windows` or
# `--target macos` adds check_release.sh's cross-target clippy and
# strict-panic steps of native-theme for that target, whose cfg-gated code no
# host step compiles.
#
# It leaves to `check_release.sh --commit` the other MSRV checks, the feature
# combinations and every other crate's steps. Each step prints its name,
# result and wall time; the first failure prints the step's output and stops
# the run with a non-zero status; a total time ends a passing run.
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
            sed -n '2,33p' "$0" | sed 's/^# \{0,1\}//'
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

# test_step <name> <command...>: step for a test run, with the minimum of
# check_release.sh's run_tests: it fails, too, where the command exits 0 with
# fewer than one test passed (cargo test's "test result: ok. N passed" per
# binary and nextest's "N tests run: N passed", summed), since a filter or cfg
# that matches nothing still exits 0.
test_step() {
    local name="$1"
    shift
    local min_passed=1
    local start=$SECONDS
    local output status
    output=$("$@" 2>&1)
    status=$?
    local total_passed=0
    if [ "$status" -eq 0 ]; then
        total_passed=$(printf "%s\n" "$output" |
            grep -oE 'test result: ok\. [0-9]+ passed|[0-9]+ tests? run: [0-9]+ passed' |
            awk '{sum += $(NF - 1)} END {print sum+0}')
    fi
    local elapsed
    elapsed=$(format_elapsed $((SECONDS - start)))
    if [ "$status" -eq 0 ] && [ "$total_passed" -ge "$min_passed" ]; then
        printf "ok    %-58s %s\n" "$name ($total_passed passed)" "$elapsed"
    else
        printf "%s\n" "$output" >&2
        if [ "$status" -eq 0 ]; then
            printf "      %s passed, fewer than the minimum of %s: a filter or cfg that matches nothing still exits 0\n" \
                "$total_passed" "$min_passed" >&2
        fi
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

# The gpui connector's parity test: tests' runner without its doctest run,
# which cargo refuses beside --lib and a test filter.
parity_tests() {
    if [[ ! $HAVE_NEXTEST ]]; then
        cargo test "$@"
        return
    fi
    local threads=()
    if [ -n "${RUST_TEST_THREADS:-}" ] && [ -z "${NEXTEST_TEST_THREADS:-}" ]; then
        threads=(--test-threads "$RUST_TEST_THREADS")
    fi
    cargo nextest run --no-tests=pass --status-level fail --final-status-level fail \
        --color never ${threads[@]+"${threads[@]}"} "$@"
}

# check_release.sh's MSRV section, scripts/msrv_checks.sh, sourced for the
# toolchain, the check that it is installed (rustup may install a toolchain
# it is asked for) and the arguments; it runs nothing when sourced.
# shellcheck source=scripts/msrv_checks.sh
source scripts/msrv_checks.sh

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
# The MSRV checks of native-theme, as msrv_checks runs them: the workspace's
# rust-version, warnings denied through RUSTFLAGS (CARGO_ENCODED_RUSTFLAGS
# unset, which Cargo would read in its place), in the folder
# <build_directory>/msrv-<major.minor>, which an EXIT trap deletes however the
# run ends (Cargo at that version reads no build.build-dir), a leftover first.
if ! msrv_toolchains; then
    echo "check_next.sh: MSRV toolchains not read: $MSRV_TOOLCHAINS_ERROR" >&2
    exit 1
fi
msrv_toolchain_list
if ! reason=$(msrv_toolchain_installed "$MSRV_TOOLCHAIN"); then
    echo "check_next.sh: MSRV: $reason" >&2
    exit 1
fi
build_directory=$(cargo metadata --format-version 1 --no-deps | jq -r '.build_directory // empty')
MSRV_TARGET_DIR="${build_directory:-target}/msrv-$(cut -d. -f1-2 <<<"${MSRV_TOOLCHAIN#+}")"
quoted=$(printf %q "$MSRV_TARGET_DIR")
# shellcheck disable=SC2064  # the path is fixed when the trap is set
trap "[ -d $quoted ] && du -sb $quoted || true; rm -rf $quoted || true" EXIT
if ! rm -rf "$MSRV_TARGET_DIR"; then
    echo "check_next.sh: MSRV: the leftover $MSRV_TARGET_DIR could not be removed" >&2
    exit 1
fi
step "MSRV $MSRV_TOOLCHAIN, -D warnings (native-theme, default features)" \
    env -u CARGO_ENCODED_RUSTFLAGS RUSTFLAGS=-Dwarnings \
    cargo "$MSRV_TOOLCHAIN" check "${MSRV_CHECK_ARGS_2[@]}" --target-dir "$MSRV_TARGET_DIR"
step "MSRV $MSRV_TOOLCHAIN, -D warnings (native-theme, all features)" \
    env -u CARGO_ENCODED_RUSTFLAGS RUSTFLAGS=-Dwarnings \
    cargo "$MSRV_TOOLCHAIN" check -p native-theme --all-features --locked --target-dir "$MSRV_TARGET_DIR"
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
test_step "test (native-theme)" tests -p native-theme
test_step "test (native-theme, Linux features)" tests -p native-theme --features "$NT_FEATURES"
# The gpui connector's 0.8.0 mapping, from the plan's M0, step 4 on (rule 7),
# with the features check_release.sh tests the connector with.
GPUI_FEATURES="native,watch"
step "check (native-theme-gpui, all targets, native, watch)" \
    cargo check -p native-theme-gpui --all-targets --features "$GPUI_FEATURES"
test_step "connector parity (native-theme-gpui, native, watch)" \
    parity_tests -p native-theme-gpui --features "$GPUI_FEATURES" --lib next::parity
step "docs (native-theme)" \
    env RUSTDOCFLAGS="-D warnings" cargo doc -p native-theme --no-deps
step "docs, private items (native-theme, Linux features)" \
    env RUSTDOCFLAGS="-D warnings" \
    cargo doc -p native-theme --no-deps --document-private-items --features "$NT_FEATURES"
step "panic-pattern scan (native-theme/src)" \
    python3 scripts/check_panic_patterns.py native-theme/src
step "README versions and crates" python3 scripts/check_readme_versions.py

printf "check_next.sh: all steps passed (total %s)\n" "$(format_elapsed $((SECONDS - TOTAL_START)))"
