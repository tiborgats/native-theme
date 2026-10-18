#!/bin/bash

# Pre-release check script for native-theme workspace.
# Performs comprehensive checks before releasing a new version.
#
# Output design (glance-first):
#   * Each check renders a single-line status when done:
#         ✅ description    (2.3s)       — pass
#         ⚠  description    (9.2s) soft  — soft failure (a warning; only the
#                                          semver checks are soft)
#         ❌ description    (0.5s)       — hard failure (exits)
#   * Each command's stdout/stderr is captured, not streamed, and printed in
#     full only when its check fails: streamed output would interleave with
#     the status lines and bury the summary.
#   * Sections are separated by `── Section ──` dividers.
#   * A final summary banner reports total / pass / warn / fail / elapsed,
#     and reprints every warning and every failure that did not exit, so
#     nothing is buried above.
#
# Requires jq: the crate list, the panic scan's directories and the per-crate
# sections are read from `cargo metadata`. The script installs nothing: a
# missing cargo-audit, cargo-outdated or cargo-semver-checks fails the full run
# with its install command, and a cross-target check whose target or linker is missing is
# skipped with a warning the summary repeats.
#
# Modes:
#   (no argument)  the full run: every section below. Required on the last
#                  commit of every fix phase and before the release commit.
#   --commit       the per-commit subset: skips the release-only checks
#                  (the release-tag check, the gpui connector's tests on GPUI
#                  Fast, native-theme's ignored stem-index test, docs.rs
#                  emulation, packaging, semver checks, security audit,
#                  outdated dependencies, and the visual-asset and
#                  compatibility stamp checks), each printing "skipped
#                  (--commit mode)", and names them in the final summary.
#
# Formatting: `cargo fmt --all` formats the tree in place rather than checking
# it, by design: the per-commit gate formats what is about to be committed,
# and CI's `cargo fmt --all --check` refuses what was committed unformatted.

set -e  # Exit immediately if a command exits with a non-zero status
set -u  # Exit if an undefined variable is used

usage() {
    cat <<'EOF'
Usage: scripts/check_release.sh [--commit]

  (no argument)  the full pre-release check; required on the last commit of
                 every fix phase and before the release commit
  --commit       the per-commit subset: every lint, check, test, docs,
                 widget-coverage, showcase-parity and archive-index gate; skips
                 the release-tag check, the gpui connector's tests on GPUI
                 Fast, native-theme's ignored stem-index test, docs.rs
                 emulation, packaging, semver checks, the security audit,
                 outdated dependencies, and the visual-asset and
                 compatibility stamp checks
  -h, --help     print this help and exit
EOF
}

COMMIT_MODE=""
for arg in "$@"; do
    case "$arg" in
        --commit) COMMIT_MODE=1 ;;
        -h|--help) usage; exit 0 ;;
        *) printf "Unknown argument: %s\n\n" "$arg" >&2; usage >&2; exit 2 ;;
    esac
done

cd "$(dirname "$0")/.."  # every path below is relative to the repository root

# ─────────────────────────────────────────────────────────────────────────────
# Colour + icon vocabulary
# ─────────────────────────────────────────────────────────────────────────────
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
DIM='\033[2m'
BOLD='\033[1m'
NC='\033[0m' # No Color

ICON_OK='✅'
ICON_WARN='⚠️ '
ICON_FAIL='❌'
ICON_INFO='ℹ️ '
ICON_ROCKET='🚀'
ICON_PARTY='🎉'

# ─────────────────────────────────────────────────────────────────────────────
# Counters + warning and failure logs (re-printed in the final summary)
# ─────────────────────────────────────────────────────────────────────────────
PASS_COUNT=0
WARN_COUNT=0
FAIL_COUNT=0
WARNINGS=()
FAILURES=()
SKIPPED=()
SCRIPT_START=$SECONDS

# ─────────────────────────────────────────────────────────────────────────────
# Output primitives
# ─────────────────────────────────────────────────────────────────────────────
print_section() {
    # Compact section divider
    printf "\n${BOLD}${BLUE}── %s ──${NC}\n" "$1"
}

print_ok() {
    # $1 = description, $2 = optional elapsed (e.g. "2.3s")
    local elapsed="${2:-}"
    if [ -n "$elapsed" ]; then
        printf "${GREEN}${ICON_OK}${NC} %-48s ${DIM}%s${NC}\n" "$1" "$elapsed"
    else
        printf "${GREEN}${ICON_OK}${NC} %s\n" "$1"
    fi
    PASS_COUNT=$((PASS_COUNT + 1))
}

print_warn() {
    local elapsed="${2:-}"
    local suffix="${3:-}"  # e.g. "soft"
    if [ -n "$elapsed" ]; then
        printf "${YELLOW}${ICON_WARN}${NC} %-47s ${DIM}%s${NC} ${YELLOW}%s${NC}\n" "$1" "$elapsed" "$suffix"
    else
        printf "${YELLOW}${ICON_WARN}${NC} %s\n" "$1"
    fi
    WARNINGS+=("$1")
    WARN_COUNT=$((WARN_COUNT + 1))
}

print_fail() {
    local elapsed="${2:-}"
    if [ -n "$elapsed" ]; then
        printf "${RED}${ICON_FAIL}${NC} %-48s ${DIM}%s${NC}\n" "$1" "$elapsed"
    else
        printf "${RED}${ICON_FAIL}${NC} %s\n" "$1"
    fi
    FAILURES+=("$1")
    FAIL_COUNT=$((FAIL_COUNT + 1))
}

print_info() {
    printf "${BLUE}${ICON_INFO}${NC} %s\n" "$1"
}

print_skipped() {
    # $1 = section name, re-printed in the final summary
    printf "${DIM}   skipped (--commit mode)${NC}\n"
    SKIPPED+=("$1")
}

# Human-friendly elapsed time: "1m 23s" or "5s".
format_elapsed() {
    local secs="$1"
    if [ "$secs" -lt 60 ]; then
        printf "%ds" "$secs"
    else
        printf "%dm %02ds" "$((secs / 60))" "$((secs % 60))"
    fi
}

# ─────────────────────────────────────────────────────────────────────────────
# Check runners
#
# run_check       — hard fail (exits on command failure)
# run_check_soft  — soft fail (logs a warning, continues); the semver checks
#                   only, whose reported break asks for a version decision
# run_tests       — run_check for `cargo test`: filters the noise from its
#                   output and fails when the run passes no test (a filter or
#                   a cfg that matches nothing exits 0 with "0 passed")
# run_tests_min N — run_tests with N as the minimum of passed tests
# ─────────────────────────────────────────────────────────────────────────────

# Filter pattern for cargo test: drop individual "test NAME ... ok" lines while
# keeping FAILED, ignored, compile warnings/errors, the "running N tests"
# header, and the "test result:" per-binary summary.
TEST_OK_FILTER='^test .+ \.\.\. ok$'

# Stricter noise filter for `cargo test`: keeps only the pieces that matter —
# the "running N tests" header, the per-binary "test result: ..." summary,
# any FAILED lines, and any ignored-test markers. Suppresses blank lines,
# "Running unittests .../..." paths, "Doc-tests" headers, and "merged doctests
# compilation took ..." footers.
TEST_NOISE_FILTER='^test .+ \.\.\. ok$|^\s*$|^\s*Running (unittests|tests|benches)|^\s*Finished |^\s*Compiling |^\s+Doc-tests |^all doctests ran |^   Doc-tests |^\s*Generated |test .+ \.\.\. ignored$'

_run_check_impl() {
    # $1 = description, $2 = "hard"|"soft", $3... = command
    local description="$1"
    local mode="$2"
    shift 2
    local start=$SECONDS
    local output
    local status=0
    set +e
    output=$("$@" 2>&1)
    status=$?
    set -e
    local elapsed
    elapsed=$(format_elapsed "$((SECONDS - start))")

    if [ "$status" -eq 0 ]; then
        print_ok "$description" "$elapsed"
    else
        # Failure: dump full output so the user sees what broke.
        printf "%s\n" "$output" >&2
        if [ "$mode" = "soft" ]; then
            print_warn "$description" "$elapsed" "soft"
        else
            print_fail "$description" "$elapsed"
            printf "${RED}   Failed command:${NC} %s\n" "$*" >&2
            exit 1
        fi
    fi
}

run_check()      { _run_check_impl "$1" "hard" "${@:2}"; }
run_check_soft() { _run_check_impl "$1" "soft" "${@:2}"; }

_run_tests_impl() {
    # $1 = description, $2 = minimum passed, $3... = command
    local description="$1"
    local min_passed="$2"
    shift 2
    local start=$SECONDS
    local output
    local status=0
    set +e
    output=$("$@" 2>&1)
    status=$?
    set -e
    local elapsed
    elapsed=$(format_elapsed "$((SECONDS - start))")

    local total_passed=0
    if [ "$status" -eq 0 ]; then
        total_passed=$(printf "%s\n" "$output" \
            | grep -oE 'test result: ok\. [0-9]+ passed' \
            | awk '{sum += $4} END {print sum+0}')
    fi

    if [ "$status" -eq 0 ] && [ "$total_passed" -ge "$min_passed" ]; then
        # Successful run: print compact summary (sum passed across binaries).
        local total_ignored
        total_ignored=$(printf "%s\n" "$output" \
            | grep -oE '[0-9]+ ignored' \
            | awk '{sum += $1} END {print sum+0}')
        local detail=""
        if [ "$total_passed" -gt 0 ] || [ "$total_ignored" -gt 0 ]; then
            if [ "$total_ignored" -gt 0 ]; then
                detail=" ${total_passed} passed, ${total_ignored} ignored"
            else
                detail=" ${total_passed} passed"
            fi
        fi
        if [ -n "$detail" ]; then
            printf "${GREEN}${ICON_OK}${NC} %-36s ${DIM}%-9s${NC} ${DIM}%s${NC}\n" \
                "$description" "$elapsed" "·$detail"
        else
            print_ok "$description" "$elapsed"
            return
        fi
        PASS_COUNT=$((PASS_COUNT + 1))
    else
        # Failure: dump filtered output so the user sees what broke.
        printf "%s\n" "$output" | grep -v -E "$TEST_NOISE_FILTER" >&2 || true
        if [ "$status" -eq 0 ]; then
            printf "${RED}   %s passed, fewer than the minimum of %s:${NC} a filter or cfg that matches nothing still exits 0\n" \
                "$total_passed" "$min_passed" >&2
        fi
        print_fail "$description" "$elapsed"
        printf "${RED}   Failed command:${NC} %s\n" "$*" >&2
        exit 1
    fi
}

run_tests()      { _run_tests_impl "$1" 1 "${@:2}"; }
run_tests_min()  { _run_tests_impl "$2" "$1" "${@:3}"; }

# A cargo subcommand the full run needs. A missing one fails with its install
# command and its section does not run; it is never installed (a check
# installs nothing, and a prompt would end a run that has no terminal).
# $1 = tool, $2 = the section that needs it. Returns 1 when it is missing.
check_tool() {
    local tool_name="$1"
    local subcommand="${tool_name#cargo-}"

    if cargo "$subcommand" --help >/dev/null 2>&1; then
        print_ok "$tool_name installed"
        return 0
    fi
    print_fail "$tool_name not installed: $2 not run (cargo install $tool_name)"
    return 1
}

# ─────────────────────────────────────────────────────────────────────────────
# Preflight
# ─────────────────────────────────────────────────────────────────────────────
if [ ! -f "Cargo.toml" ]; then
    print_fail "Cargo.toml not found — run from project root"
    exit 1
fi

if ! command -v jq &>/dev/null; then
    print_fail "jq not found: the crate list is read from cargo metadata with jq; install jq and rerun"
    exit 1
fi

# The workspace members, read once: the panic scan's directories, the
# per-crate loops, the examples and the semver checks come from it. An empty
# list would skip every per-crate loop and still end green.
if ! WORKSPACE_METADATA=$(cargo metadata --no-deps --format-version 1); then
    print_fail "cargo metadata failed (its error is above)"
    exit 1
fi
WORKSPACE_CRATES=$(jq -r '.packages[].name' <<<"$WORKSPACE_METADATA")
if [ -z "$WORKSPACE_CRATES" ]; then
    print_fail "cargo metadata lists no workspace crate"
    exit 1
fi

CURRENT_VERSION=$(grep -E '^version\s*=' Cargo.toml | sed -E 's/.*"([^"]+)".*/\1/')

if [[ $COMMIT_MODE ]]; then
    printf "\n${BOLD}${ICON_ROCKET} native-theme v%s · per-commit checks (--commit)${NC}\n" "$CURRENT_VERSION"
else
    printf "\n${BOLD}${ICON_ROCKET} native-theme v%s · pre-release checks${NC}\n" "$CURRENT_VERSION"
fi

# ─────────────────────────────────────────────────────────────────────────────
# Section: environment
# ─────────────────────────────────────────────────────────────────────────────
print_section "Environment"

# `git rev-parse`, not a `.git` directory: in a worktree `.git` is a file.
# `git status --porcelain` counts staged and untracked files too.
if ! git rev-parse --is-inside-work-tree >/dev/null 2>&1; then
    print_warn "Not in a git work tree (the clean-tree and release-tag checks skipped)"
else
    if ! GIT_STATUS=$(git status --porcelain); then
        print_fail "git status failed (its error is above)"
    elif [ -n "$GIT_STATUS" ]; then
        print_info "Uncommitted changes present ($(printf "%s\n" "$GIT_STATUS" | wc -l) files)"
    else
        print_ok "Working tree clean"
    fi

    if [[ $COMMIT_MODE ]]; then
        print_skipped "Release tag check"
    elif git tag --list | grep -q "^v${CURRENT_VERSION}$"; then
        print_warn "Tag v${CURRENT_VERSION} already exists in git"
    else
        print_ok "Tag v${CURRENT_VERSION} not yet created"
    fi
fi

HAVE_AUDIT=""
HAVE_OUTDATED=""
HAVE_SEMVER_CHECKS=""
if [[ ! $COMMIT_MODE ]]; then
    check_tool cargo-audit "security audit" && HAVE_AUDIT=1
    check_tool cargo-outdated "outdated-dependency check" && HAVE_OUTDATED=1
    check_tool cargo-semver-checks "semver checks" && HAVE_SEMVER_CHECKS=1
fi

# ─────────────────────────────────────────────────────────────────────────────
# Section: source hygiene
# ─────────────────────────────────────────────────────────────────────────────
print_section "Source hygiene"

# TODO/FIXME scan
TODO_HITS=$(grep -rn --include="*.rs" --exclude-dir=target "TODO\|FIXME" . 2>/dev/null || true)
if [ -n "$TODO_HITS" ]; then
    TODO_COUNT=$(printf "%s\n" "$TODO_HITS" | wc -l)
    print_warn "$TODO_COUNT TODO/FIXME comment(s) in source"
    printf "${DIM}%s${NC}\n" "$TODO_HITS" | head -5
    if [ "$TODO_COUNT" -gt 5 ]; then
        printf "${DIM}    ... (%d more)${NC}\n" "$((TODO_COUNT - 5))"
    fi
else
    print_ok "No TODO/FIXME comments"
fi

# Panic-pattern scan (BLOCKING — runtime panics are forbidden in production)
#
# scripts/check_panic_patterns.py, over every workspace member's src/ (from
# `cargo metadata`, so a member added to the workspace is scanned without
# naming it here). It reads the code with comments and literals blanked, so a
# brace in a test's fixture string cannot hide the code after the test module,
# and it fails on a directory or file it cannot read; its docstring lists the
# patterns. A regex tripwire only: the authoritative panic check is the
# "Strict panic lints" section below, which runs clippy with the full
# panic-prone lint set using type-aware analysis. Its self-test runs the
# lexing and the test-module rules on a synthetic tree.
PANIC_SCAN_DIRS=$(jq -r '.workspace_root as $root | .packages[].manifest_path
    | ltrimstr($root + "/") | rtrimstr("Cargo.toml") + "src"' <<<"$WORKSPACE_METADATA")
if PANIC_HITS=$(python3 scripts/check_panic_patterns.py $PANIC_SCAN_DIRS 2>&1); then
    print_ok "No panic-inducing patterns"
else
    print_fail "Panic-inducing pattern(s) in production code, or an unreadable input"
    printf "${DIM}%s${NC}\n" "$PANIC_HITS"
    printf "${RED}   Runtime panics are FORBIDDEN. Replace with Result/Option or checked_* arithmetic.${NC}\n" >&2
    exit 1
fi
run_check "panic-pattern scan: self-test" python3 scripts/check_panic_patterns.py --self-test

# native-theme's Linux feature set: every feature with code on Linux
# (`macos` and `windows` add none there; `linux` is `kde` + `portal`).
# native-theme has no default features, so on Linux the tests of its
# feature-gated modules (kde, gnome, freedesktop, rasterize, spinners, fonts,
# the watchers) are run only by the test legs below that name this set. The
# clippy leg below that names it is the only lint of the Linux-only ones (kde,
# gnome, freedesktop, the KDE and GNOME watchers); the cross-target
# `--all-features` clippy runs also lint the tests that are not Linux-only
# (the rasterizer's and the spinners', for two). The strict panic lints name
# the set for the library.
NT_FEATURES="kde,portal,system-icons,material-icons,lucide-icons,watch,svg-rasterize,system-fonts"

# ─────────────────────────────────────────────────────────────────────────────
# Section: cargo check
# ─────────────────────────────────────────────────────────────────────────────
print_section "cargo check"
for crate in $WORKSPACE_CRATES; do
    run_check "check ($crate)" cargo check -p "$crate" --all-targets
done

# ─────────────────────────────────────────────────────────────────────────────
# Section: feature combinations
#
# scripts/check_features.sh checks every workspace crate's library with no
# default features, with each feature alone and with all features (the
# coverage of `cargo hack check --each-feature`), and fails on an error or a
# warning. The per-crate loops above build the default set only, and a crate
# that stops compiling without one of its default features is otherwise first
# seen by the user who turns it off.
# Hard failure. CI, the publish gate and the nightly canary run the same
# script.
# ─────────────────────────────────────────────────────────────────────────────
print_section "Feature combinations"
run_check "feature combinations (every crate)" bash scripts/check_features.sh

# ─────────────────────────────────────────────────────────────────────────────
# Section: system-icons on macOS and Windows
#
# Both connectors enable `system-icons` by default, and its icon code imports
# the platform crates, so the feature must build alone on each platform. The
# platform readers and icon loaders compile only on their own target, so
# native-theme with all features is checked there too, with warnings denied:
# dead code or an unchecked result in them is invisible to a Linux build. A
# cross-target `cargo check` needs the target's standard library (and, for
# windows-gnu, the MinGW linker toolchain); where one is missing the check is
# skipped with a warning that says why, and the summary repeats it. CI runs
# the same checks: its Windows and macOS test legs the first two, on the
# platform itself, and its clippy job the egui connector's macOS check.
# ─────────────────────────────────────────────────────────────────────────────
print_section "system-icons (cross-target)"
if command -v rustup &>/dev/null; then
    INSTALLED_TARGETS=$(rustup target list --installed 2>/dev/null || true)
else
    INSTALLED_TARGETS=""
fi
for target in x86_64-pc-windows-gnu x86_64-apple-darwin; do
    if ! printf "%s\n" "$INSTALLED_TARGETS" | grep -qx "$target"; then
        print_warn "skipped system-icons ($target): target not installed (rustup target add $target)"
        continue
    fi
    if [ "$target" = "x86_64-pc-windows-gnu" ] && ! command -v x86_64-w64-mingw32-gcc &>/dev/null; then
        print_warn "skipped system-icons ($target): x86_64-w64-mingw32-gcc not found"
        continue
    fi
    run_check "system-icons ($target)" \
        cargo check --target "$target" -p native-theme --no-default-features --features system-icons
    run_check "all features, -D warnings ($target)" \
        env RUSTFLAGS="-D warnings" cargo check --target "$target" -p native-theme --all-features
    if [ "$target" = "x86_64-apple-darwin" ]; then
        run_check "native-theme-egui, all targets ($target)" \
            cargo check --target "$target" -p native-theme-egui --all-targets --all-features
    fi
done

# clippy on the platform code: a Linux clippy run never sees the macOS and
# Windows branches, so lints in them (8 on macOS, 11 on Windows, found
# 2026-10-11) went unnoticed. Clippy builds no binary, so windows-msvc needs
# only its standard library, no linker. CI's clippy job runs the same four
# for both targets.
for target in x86_64-pc-windows-msvc x86_64-apple-darwin; do
    if ! printf "%s\n" "$INSTALLED_TARGETS" | grep -qx "$target"; then
        print_warn "skipped clippy ($target): target not installed (rustup target add $target)"
        continue
    fi
    run_check "clippy, all features ($target)" \
        cargo clippy --target "$target" -p native-theme --all-targets --all-features -- -D warnings
    run_check "clippy, no default features ($target)" \
        cargo clippy --target "$target" -p native-theme --all-targets --no-default-features -- -D warnings
    # The OS probe (tools/os-probe, never published) is all platform code;
    # its crate root denies the strict panic lints, so this run holds them.
    run_check "clippy, os-probe ($target)" \
        cargo clippy --target "$target" -p native-theme-os-probe --all-targets -- -D warnings
    # The showcases' shared crate (tools/showcase-support, never published):
    # its window capture and the Retina nudge are macOS and Windows code,
    # which no showcase's own cross-target build can lint from Linux.
    run_check "clippy, showcase-support ($target)" \
        cargo clippy --target "$target" -p native-theme-showcase-support --all-targets -- -D warnings
done

# ─────────────────────────────────────────────────────────────────────────────
# Section: formatting
# ─────────────────────────────────────────────────────────────────────────────
print_section "Formatting"
run_check "cargo fmt (all crates)" cargo fmt --all

# ─────────────────────────────────────────────────────────────────────────────
# Section: clippy
# ─────────────────────────────────────────────────────────────────────────────
print_section "clippy (-D warnings)"
# The gpui connector with `native` and `watch` on top of its defaults, so
# `watch.rs` and the showcase's `watch` code are linted too; each is additive,
# and `cargo check` above builds the defaults alone.
for crate in $WORKSPACE_CRATES; do
    if [ "$crate" = "native-theme-gpui" ]; then
        run_check "clippy ($crate, native, watch)" \
            cargo clippy -p "$crate" --all-targets --features native,watch -- -D warnings
    else
        run_check "clippy ($crate)" cargo clippy -p "$crate" --all-targets -- -D warnings
    fi
    # The Linux features too, so the reader, icon and watcher code and their
    # tests are linted; the run above keeps the no-feature cfg branches.
    if [ "$crate" = "native-theme" ]; then
        run_check "clippy ($crate, Linux features)" \
            cargo clippy -p "$crate" --all-targets --features "$NT_FEATURES" -- -D warnings
    fi
done
# The gpui connector on GPUI Fast (its `gpui-fast` feature), which the loop's
# features leave off, with `native` and `watch` as in the loop.
run_check "clippy (native-theme-gpui, gpui-fast, native, watch)" \
    cargo clippy -p native-theme-gpui --all-targets --features gpui-fast,native,watch -- -D warnings

# ─────────────────────────────────────────────────────────────────────────────
# Section: strict panic lints (library code only)
#
# scripts/check_strict_panic.sh runs clippy with the type-aware panic-prone
# lints promoted to errors; the lint list and what each lint catches are
# there, and CI's clippy job runs the same script.
#
# Scope: library targets only (--lib). Tests and examples are exempt because
# they legitimately use .unwrap() / asserts for failure clarity and demo brevity.
# Bin targets (if any in the future) should be added to this list.
#
# `native-theme` is run with the full Linux feature set ($NT_FEATURES) so
# feature-gated modules (kde, portal, system-icons, freedesktop, spinners,
# watch) are checked; plain `--lib` misses them. ci.yml names the same features.
# ─────────────────────────────────────────────────────────────────────────────
print_section "Strict panic lints (library code)"
run_check "strict-panic (native-theme, Linux features)" \
    bash scripts/check_strict_panic.sh -p native-theme --lib --features "$NT_FEATURES"
run_check "strict-panic (native-theme, no features)" \
    bash scripts/check_strict_panic.sh -p native-theme --lib --no-default-features
# The macOS and Windows code, with each platform's features, and the
# showcases' shared crate's capture code; CI's clippy job runs the same six.
# The targets are the cross-target clippy section's.
NT_PLATFORM_FEATURES="system-icons,material-icons,lucide-icons,watch,svg-rasterize,system-fonts"
for target in x86_64-apple-darwin x86_64-pc-windows-msvc; do
    if ! printf "%s\n" "$INSTALLED_TARGETS" | grep -qx "$target"; then
        print_warn "skipped strict-panic ($target): target not installed (rustup target add $target)"
        continue
    fi
    if [ "$target" = "x86_64-apple-darwin" ]; then
        platform_feature=macos
    else
        platform_feature=windows
    fi
    run_check "strict-panic (native-theme, $platform_feature features, $target)" \
        bash scripts/check_strict_panic.sh --target "$target" -p native-theme --lib --features "$platform_feature,$NT_PLATFORM_FEATURES"
    run_check "strict-panic (native-theme, no features, $target)" \
        bash scripts/check_strict_panic.sh --target "$target" -p native-theme --lib --no-default-features
    run_check "strict-panic (native-theme-showcase-support, $target)" \
        bash scripts/check_strict_panic.sh --target "$target" -p native-theme-showcase-support --lib
done
run_check "strict-panic (native-theme-derive)" \
    bash scripts/check_strict_panic.sh -p native-theme-derive --lib
run_check "strict-panic (native-theme-build)" \
    bash scripts/check_strict_panic.sh -p native-theme-build --lib
# With the optional features on top of the defaults, so `styles::aw` and
# `watch.rs` are linted too; nothing in the library is compiled only without one.
run_check "strict-panic (native-theme-iced, iced_aw, watch, native)" \
    bash scripts/check_strict_panic.sh -p native-theme-iced --lib --features iced_aw,watch,native
# The gpui connector with `native` and `watch` too, so `watch.rs` is linted.
run_check "strict-panic (native-theme-gpui, native, watch)" \
    bash scripts/check_strict_panic.sh -p native-theme-gpui --lib --features native,watch
run_check "strict-panic (native-theme-gpui, gpui-fast, native, watch)" \
    bash scripts/check_strict_panic.sh -p native-theme-gpui --lib --features gpui-fast,native,watch
run_check "strict-panic (native-theme-egui)" \
    bash scripts/check_strict_panic.sh -p native-theme-egui --lib --all-features
run_check "strict-panic (native-theme-egui-widgets)" \
    bash scripts/check_strict_panic.sh -p native-theme-egui-widgets --lib --all-features
# The OS probe is a binary; on Linux it is the stub, and its macOS and
# Windows code is held by the cross-target clippy runs above.
run_check "strict-panic (native-theme-os-probe)" \
    bash scripts/check_strict_panic.sh -p native-theme-os-probe --bins
# The test fixtures (tools/test-fixtures, never published) return each
# failure as an error, so a test fails on its own `?`.
run_check "strict-panic (native-theme-test-fixtures)" \
    bash scripts/check_strict_panic.sh -p native-theme-test-fixtures --lib
# The showcases' shared crate (tools/showcase-support, never published) is
# held to the library rule too: a showcase reports each error it returns.
run_check "strict-panic (native-theme-showcase-support)" \
    bash scripts/check_strict_panic.sh -p native-theme-showcase-support --lib

# ─────────────────────────────────────────────────────────────────────────────
# Section: tests
# ─────────────────────────────────────────────────────────────────────────────
print_section "Tests"
# The gpui connector with `native` and `watch` on top of its defaults:
# `on_theme_change`'s tests (they start no OS watcher) and the showcase's
# `watch` code. Each is additive, so the one leg covers the defaults too.
# The OS probe is a stub on Linux with no tests: it may pass none only while
# its test list is empty.
for crate in $WORKSPACE_CRATES; do
    if [ "$crate" = "native-theme-gpui" ]; then
        run_tests "test ($crate, native, watch)" cargo test -p "$crate" --features native,watch
    elif [ "$crate" = "native-theme-os-probe" ]; then
        probe_min=1
        if ! cargo test -p "$crate" -- --list 2>/dev/null | grep -q ': test$'; then
            probe_min=0
        fi
        run_tests_min "$probe_min" "test ($crate)" cargo test -p "$crate"
    else
        run_tests "test ($crate)" cargo test -p "$crate"
    fi
done
# native-theme with its Linux features ($NT_FEATURES): the loop above runs it
# with none, which compiles out the KDE and GNOME readers, the freedesktop
# icon loader, the rasterizer, the spinners, the font lookup and the watchers,
# and tests/reader_kde.rs. On a host with Breeze and Adwaita installed the
# tests that need them run too; elsewhere they skip themselves.
run_tests "test (native-theme, Linux features)" \
    cargo test -p native-theme --features "$NT_FEATURES"
# The freedesktop stem index against freedesktop-icons' unfiltered lookup over
# the host's installed Breeze and Adwaita themes: `#[ignore]`d for its run time
# (~30-50 s), so the per-commit subset leaves it to the full run. On a host
# with none of breeze, breeze-dark and Adwaita installed it checks nothing and
# passes (it prints "skipped" to its captured output). The freedesktop module
# exists on Linux only, where elsewhere the filter would match no test.
if [[ $COMMIT_MODE ]]; then
    printf "${DIM}test (native-theme, Linux features, stem index)${NC}\n"
    print_skipped "test (native-theme, Linux features, stem index)"
elif [ "$(uname -s)" != "Linux" ]; then
    print_info "skipped test (native-theme, Linux features, stem index): the freedesktop module is Linux-only"
else
    run_tests "test (native-theme, Linux features, stem index)" \
        cargo test -p native-theme --features "$NT_FEATURES" --lib -- --ignored \
        freedesktop::tests::indexed_lookup_matches_the_unfiltered_lookup_on_installed_themes
fi
# The loop above runs the gpui connector with its default features, which
# include `svg-rasterize`; its tests for the SVG path without rasterization only
# compile without it.
run_tests "test (native-theme-gpui, no features)" \
    cargo test -p native-theme-gpui --lib --no-default-features
# The gpui connector on GPUI Fast, with `native` and `watch` as above: a
# second GPUI core and platform layer to build, so the per-commit subset
# leaves it to the full run and CI. Doctests excluded: each is a crate of its
# own that names gpui-pre as `gpui`.
if [[ $COMMIT_MODE ]]; then
    printf "${DIM}test (native-theme-gpui, gpui-fast, native, watch)${NC}\n"
    print_skipped "test (native-theme-gpui, gpui-fast, native, watch)"
else
    run_tests "test (native-theme-gpui, gpui-fast, native, watch)" \
        cargo test -p native-theme-gpui --features gpui-fast,native,watch --all-targets
fi

# ─────────────────────────────────────────────────────────────────────────────
# Section: iced connector configurations
#
# The per-crate loops above run the connector with its default features only.
# Its other two supported configurations are gated here: `--no-default-features`
# (no icon or system-font support) and `--features iced_aw,watch,native`: the
# optional widget set the showcase's `iced_aw` tab renders, `theme_changes()`
# with its tests and the showcase's `watch` code, and the OS readers. Each is
# additive, so the one leg covers `iced_aw` alone too. The `watch` tests start
# no OS watcher, and on Linux the tests read the OS theme with or without
# `native` (the dev-dependencies turn on native-theme's `linux`), a read that
# cannot fail there: with no reader it is the platform preset. Same hard class
# as the connector's default test and clippy checks. .github/workflows/ci.yml
# runs the same two commands, so no configuration reaches a release that CI
# never saw.
# ─────────────────────────────────────────────────────────────────────────────
print_section "iced connector configurations"
run_tests "test (native-theme-iced, no features)" \
    cargo test -p native-theme-iced --no-default-features
run_tests "test (native-theme-iced, iced_aw, watch, native)" \
    cargo test -p native-theme-iced --features iced_aw,watch,native
run_check "clippy (native-theme-iced, iced_aw, watch, native)" \
    cargo clippy -p native-theme-iced --all-targets --features iced_aw,watch,native -- -D warnings

# ─────────────────────────────────────────────────────────────────────────────
# Section: egui connector configurations
#
# The per-crate loops above run the connector with its default features only.
# Its other two supported configurations are gated here: `--no-default-features`
# (no icon or system-font support) and `--all-features` (`svg-rasterize` and
# `watch` too) (spec §11); Task 37 adds the same two to CI. `cargo tree -d`
# must list no second `skrifa`: this crate names skrifa at epaint's own
# requirement so that one parser serves both (spec §11, §13 T7).
# ─────────────────────────────────────────────────────────────────────────────
print_section "egui connector configurations"
run_tests "test (native-theme-egui, no features)" \
    cargo test -p native-theme-egui --no-default-features
run_tests "test (native-theme-egui, all features)" \
    cargo test -p native-theme-egui --all-features
run_check "clippy (native-theme-egui, all features)" \
    cargo clippy -p native-theme-egui --all-targets --all-features -- -D warnings
run_check "one skrifa (native-theme-egui)" \
    bash -c 'out=$(cargo tree -p native-theme-egui --target all -d) && ! grep -q "^skrifa " <<<"$out"'
# The companion widget crate forwards the connector's features one to one, so it
# has the same two further configurations.
run_tests "test (native-theme-egui-widgets, no features)" \
    cargo test -p native-theme-egui-widgets --no-default-features
run_tests "test (native-theme-egui-widgets, all features)" \
    cargo test -p native-theme-egui-widgets --all-features
run_check "clippy (native-theme-egui-widgets, all features)" \
    cargo clippy -p native-theme-egui-widgets --all-targets --all-features -- -D warnings

# ─────────────────────────────────────────────────────────────────────────────
# Section: examples (only crates with an examples/ directory)
# ─────────────────────────────────────────────────────────────────────────────
EXAMPLES_PRINTED=0
for crate in $WORKSPACE_CRATES; do
    manifest=$(jq -r --arg name "$crate" '.packages[] | select(.name == $name) | .manifest_path' \
        <<<"$WORKSPACE_METADATA")
    if [ ! -d "$(dirname "$manifest")/examples" ]; then
        continue
    fi
    if [ "$EXAMPLES_PRINTED" -eq 0 ]; then
        print_section "Examples"
        EXAMPLES_PRINTED=1
    fi
    run_check "examples ($crate)" cargo build -p "$crate" --examples
done

# ─────────────────────────────────────────────────────────────────────────────
# Section: widget coverage
#
# scripts/check_widget_coverage.py reads the toolkits' own sources through
# `cargo metadata` (the iced manifest with `--features iced_aw`, so the
# optional dependency is in the graph) and fails when an upstream release adds
# a widget that no showcase renders or docs/showcase-exceptions.toml
# excepts, or when an exception has gone stale. Its `tomllib` import puts the
# floor at Python 3.11: a missing or older interpreter fails the check rather
# than skipping it, since a gate nobody runs is not a gate. CI and the nightly
# dependency canary run the same script.
#
# scripts/check_connector_parity.py, under the same floor, reads the three
# connectors' crate-root functions and those of the public modules they
# share, and fails on an accessor named unlike the `ResolvedTheme` leaf it
# reads, or a function of any class present in one connector and absent from
# a sibling or present there under another signature, unless
# scripts/connector_parity_exceptions.toml lists it with its reason, or on a
# stale exception. Its self-test plants each violation in a synthetic tree.
#
# scripts/check_readme_versions.py, under the same floor, fails on a
# `native-theme` or `native-theme-*` dependency a README states in a `toml`
# block that does not name the workspace version's `major.minor`; the release
# bump moves every README at once, and the check holds them there. It also
# fails on a crate a README's Rust code names that none of its `toml` blocks
# depends on: the doctests see the crate's own dependencies, a reader only
# those the README states. The same holds for the doc examples under each
# crate's src/, which may name only what its README states and the crate.
# ─────────────────────────────────────────────────────────────────────────────
print_section "Widget coverage"
if ! command -v python3 &>/dev/null; then
    print_fail "python3 not found — scripts/check_widget_coverage.py, scripts/check_connector_parity.py, scripts/check_readme_versions.py and scripts/check_wrong_safety_nets.py need Python 3.11+"
    exit 1
fi
if ! python3 -c 'import sys; sys.exit(0 if sys.version_info >= (3, 11) else 1)'; then
    PYTHON_VERSION=$(python3 -c 'import sys; print("%d.%d" % sys.version_info[:2])')
    print_fail "python3 is $PYTHON_VERSION — scripts/check_widget_coverage.py, scripts/check_connector_parity.py, scripts/check_readme_versions.py and scripts/check_wrong_safety_nets.py need 3.11+ (tomllib)"
    exit 1
fi
run_check "widget coverage (gpui · iced_widget · iced_aw · egui · egui_ui · egui_extras)" \
    python3 scripts/check_widget_coverage.py
run_check "widget coverage: self-test" \
    python3 scripts/check_widget_coverage.py --self-test
run_check "connector parity: names and signatures (iced · gpui · egui)" \
    python3 scripts/check_connector_parity.py
run_check "connector parity: self-test" \
    python3 scripts/check_connector_parity.py --self-test
run_check "README versions and crates: native-theme* dependencies name the workspace's minor, the code names only stated crates" \
    python3 scripts/check_readme_versions.py
run_check "README versions: self-test" \
    python3 scripts/check_readme_versions.py --self-test

# ─────────────────────────────────────────────────────────────────────────────
# Section: showcase parity (static modes)
#
# scripts/check_showcase_parity.py compares the three showcases' layout dumps
# and captures, which are taken on a display, so that comparison is not part
# of this script. Its two static modes need neither and run here: `--check-list`
# validates docs/showcase-elements.toml against docs/property-registry.toml
# and the `[parity]` exceptions against the list and the presets (an unknown
# element, property, preset or run, an overlapping scope, a group without a
# reason), and `--self-test` runs the comparator's own scenarios on synthetic
# dumps and captures, which needs Pillow. CI runs the same two.
# ─────────────────────────────────────────────────────────────────────────────
print_section "Showcase parity (static)"
run_check "showcase parity: element list and exceptions" \
    python3 scripts/check_showcase_parity.py --check-list
run_check "showcase parity: comparator self-test" \
    python3 scripts/check_showcase_parity.py --self-test

# ─────────────────────────────────────────────────────────────────────────────
# Section: data documents
#
# The four checks of the data documents, under the Python floor above
# (check_wrong_safety_nets.py and check_todo_citations.py import `tomllib`),
# each with its self-test:
# scripts/check_platform_facts_rows.py fails on a docs/property-registry.toml
# field with no docs/platform-facts.md row; scripts/check_wrong_safety_nets.py
# on a `# see wrong_safety_nets` mark in docs/inheritance-rules.toml without
# its [wrong_safety_nets] entry, an entry its [uniform] rule does not mark, or
# an entry no rule inherits; scripts/check_section_refs.py on a § reference in
# the registry, the inheritance rules, docs/showcase-elements.toml or
# docs/new-os-version-guide.md that names no platform-facts section;
# scripts/check_todo_citations.py on a preset key path in docs/todo.md that
# names no key of its preset, or a preset line citation no commit dates. CI
# runs the same eight.
# ─────────────────────────────────────────────────────────────────────────────
print_section "Data documents"
run_check "data documents: every registry field has a platform-facts row" \
    python3 scripts/check_platform_facts_rows.py
run_check "data documents: platform-facts rows self-test" \
    python3 scripts/check_platform_facts_rows.py --self-test
run_check "data documents: [wrong_safety_nets] and its [uniform] marks" \
    python3 scripts/check_wrong_safety_nets.py
run_check "data documents: wrong safety nets self-test" \
    python3 scripts/check_wrong_safety_nets.py --self-test
run_check "data documents: § references name platform-facts sections" \
    python3 scripts/check_section_refs.py
run_check "data documents: § references self-test" \
    python3 scripts/check_section_refs.py --self-test
run_check "data documents: docs/todo.md cites preset values by key path" \
    python3 scripts/check_todo_citations.py
run_check "data documents: todo citations self-test" \
    python3 scripts/check_todo_citations.py --self-test

# ─────────────────────────────────────────────────────────────────────────────
# Section: docs
# ─────────────────────────────────────────────────────────────────────────────
print_section "Docs"
# CI, the nightly canary and the publish gate all document with
# RUSTDOCFLAGS=-Dwarnings, so this gate does too: without it a broken
# intra-doc link is green here and red at tag time. Each crate is documented
# with its private items too (`--document-private-items`), so a broken link or
# a rustdoc warning in a private item's docs fails as well; a public item's
# link to a private one still fails, as `rustdoc::private_intra_doc_links`
# fires in both modes. One run per crate covers both: two configurations of
# one crate re-document each other on every run, as they share
# `target/doc/<crate>`. native-theme keeps two: its public API with no
# features (its default build), and its private items with its Linux
# features, as its test leg.
for crate in $WORKSPACE_CRATES; do
    if [ "$crate" = "native-theme" ]; then
        run_check "docs ($crate)" \
            env RUSTDOCFLAGS="-D warnings" cargo doc -p "$crate" --no-deps
        run_check "docs, private items ($crate, Linux features)" \
            env RUSTDOCFLAGS="-D warnings" \
            cargo doc -p "$crate" --no-deps --document-private-items --features "$NT_FEATURES"
    else
        run_check "docs, private items ($crate)" \
            env RUSTDOCFLAGS="-D warnings" cargo doc -p "$crate" --no-deps --document-private-items
    fi
done
# docs.rs builds native-theme-iced with every feature, and a doc item behind a
# feature gate can only break there. No other gate documents that configuration.
run_check "docs (native-theme-iced, all features)" \
    env RUSTDOCFLAGS="-D warnings" cargo doc -p native-theme-iced --no-deps --all-features
# docs.rs builds native-theme-egui with every feature too (its
# `[package.metadata.docs.rs]` says `all-features = true`).
run_check "docs (native-theme-egui, all features)" \
    env RUSTDOCFLAGS="-D warnings" cargo doc -p native-theme-egui --no-deps --all-features
run_check "docs (native-theme-egui-widgets, all features)" \
    env RUSTDOCFLAGS="-D warnings" cargo doc -p native-theme-egui-widgets --no-deps --all-features

# ─────────────────────────────────────────────────────────────────────────────
# Section: archive index
#
# An implemented plan moves to docs/archive/ with its rationale and spec, and
# the plan's last task adds them to the index table in docs/archive/README.md
# (CONTRIBUTING.md, "Where decisions live"). This lists every docs/archive/*.md
# the index does not name. The index writes a set as `name-{rationale,spec,plan}.md`,
# so its brace sets are expanded first.
# ─────────────────────────────────────────────────────────────────────────────
print_section "Archive index"
run_check "archive index (docs/archive/README.md)" python3 -c '
import glob, os, re, sys

with open("docs/archive/README.md") as f:
    index = f.read()
named = set()
for token in re.findall(r"[\w.{},-]+\.md", index):
    names = [token]
    while any("{" in n for n in names):
        expanded = []
        for n in names:
            m = re.search(r"\{([^{}]*)\}", n)
            if m:
                expanded.extend(n[:m.start()] + alt + n[m.end():] for alt in m.group(1).split(","))
            else:
                expanded.append(n)
        names = expanded
    named.update(names)
missing = sorted(
    name for name in (os.path.basename(p) for p in glob.glob("docs/archive/*.md"))
    if name != "README.md" and name not in named
)
for name in missing:
    print(f"docs/archive/{name} is not named in docs/archive/README.md")
sys.exit(1 if missing else 0)
'

# ─────────────────────────────────────────────────────────────────────────────
# Section: docs.rs emulation
#
# docs.rs documents a crate only after it is published, so a docs.rs failure
# would otherwise first be seen after the tag, with the version already on
# crates.io. scripts/check_docsrs.py documents each crate the way docs.rs will:
# nightly rustdoc on the library, with the features and targets of its
# `[package.metadata.docs.rs]`, `DOCS_RS=1` and `--cfg docsrs`, plus
# `-D warnings`. The section above documents with the stable toolchain and the
# workspace's own features, which is not that configuration. A missing nightly
# toolchain or target fails the check rather than skipping it: a gate nobody
# runs is not a gate.
# ─────────────────────────────────────────────────────────────────────────────
print_section "docs.rs emulation"
if [[ $COMMIT_MODE ]]; then
    print_skipped "docs.rs emulation"
else
    for crate in $WORKSPACE_CRATES; do
        run_check "docs.rs ($crate)" python3 -I scripts/check_docsrs.py "$crate"
    done
fi

# ─────────────────────────────────────────────────────────────────────────────
# Section: packaging
#
# `cargo package` builds each tarball and compiles it as if it had been
# downloaded from crates.io. The workspace-internal dependencies
# (native-theme-derive, native-theme) are passed in the same invocation so that
# compile resolves them from the freshly built tarballs rather than the
# registry; the version under test is therefore verifiable before it is
# published. `--allow-dirty` because this script runs on the working tree, not
# on a release commit. Publishing itself, in dependency order with index waits
# between crates, is done by .github/workflows/publish.yml on a `v*` tag.
# ─────────────────────────────────────────────────────────────────────────────
print_section "Packaging"
if [[ $COMMIT_MODE ]]; then
    print_skipped "Packaging"
else
    # cargo unpacks each packaged workspace crate into a local-registry source
    # cache (a `-<hash>` directory under $CARGO_HOME/registry/src, beside crates.io's
    # `index.crates.io-…`) and reuses that copy while the version is unchanged, so a
    # connector would be compiled against the native-theme of an earlier run and a
    # stale API would pass or fail for the wrong reason. Those copies of this
    # version's workspace crates are removed first; cargo unpacks the current
    # tarballs again. crates.io's own cache is not touched.
    WS_VERSION=$(cargo pkgid -p native-theme | sed 's/.*[@#]//')
    find "${CARGO_HOME:-$HOME/.cargo}/registry/src" -mindepth 2 -maxdepth 2 -type d \
        -path "*/registry/src/-*/native-theme*-$WS_VERSION" -exec rm -rf {} +
    # The packages are verified in a target directory of their own, where only the
    # workspace crates are cleaned before each run: cargo reuses a registry-sourced
    # build of the same name and version without looking at its source, and the
    # dependencies stay built between runs. The workspace's own target/ is untouched.
    PKG_TARGET="target/package-verify"
    cargo clean --target-dir "$PKG_TARGET" \
        -p native-theme-derive -p native-theme -p native-theme-build \
        -p native-theme-iced -p native-theme-gpui -p native-theme-egui -p native-theme-egui-widgets >/dev/null 2>&1 || true
    run_check "package (core: derive · native-theme · build)" \
        cargo package -p native-theme-derive -p native-theme -p native-theme-build --allow-dirty --target-dir "$PKG_TARGET"
    run_check "package (native-theme-iced)" \
        cargo package -p native-theme-derive -p native-theme -p native-theme-iced --allow-dirty --target-dir "$PKG_TARGET"
    run_check "package (native-theme-gpui)" \
        cargo package -p native-theme-derive -p native-theme -p native-theme-gpui --allow-dirty --target-dir "$PKG_TARGET"
    run_check "package (native-theme-egui)" \
        cargo package -p native-theme-derive -p native-theme -p native-theme-egui --allow-dirty --target-dir "$PKG_TARGET"
    run_check "package (native-theme-egui-widgets)" \
        cargo package -p native-theme-derive -p native-theme -p native-theme-egui -p native-theme-egui-widgets --allow-dirty --target-dir "$PKG_TARGET"
fi

# ─────────────────────────────────────────────────────────────────────────────
# Section: semver checks
#
# cargo-semver-checks compares each published library crate's public API with
# its newest release on crates.io and reports every change the version in
# Cargo.toml does not allow: a removed or renamed item, a new required trait
# method, a changed signature. It is the mechanical half of choosing the next
# version number. It downloads the baseline from crates.io and builds rustdoc
# JSON for it and for the working tree, under target/semver-checks; its first
# run on this workspace took 4m 18s for the six crates (2026-10-11), so it runs
# with the release-only sections. The only soft check: before a release the
# approved breaks are expected (the next version is the bump that allows
# them), and a reported break asks for a version decision (the bump, or the
# change undone), not a code fix. A missing tool is not soft: the checks did
# not run, so it fails in the Environment section with its install command.
# ─────────────────────────────────────────────────────────────────────────────
print_section "Semver checks"
if [[ $COMMIT_MODE ]]; then
    print_skipped "Semver checks"
elif [[ ! $HAVE_SEMVER_CHECKS ]]; then
    printf "${DIM}   not run: cargo-semver-checks is not installed (failed above)${NC}\n"
else
    # A proc-macro crate (native-theme-derive) has no library API to compare.
    SEMVER_CRATES=$(jq -r '.packages[] | select(.publish != []) | select(any(.targets[]; any(.kind[]; . == "lib"))) | .name' \
        <<<"$WORKSPACE_METADATA")
    for crate in $SEMVER_CRATES; do
        # The tool's own feature choice would turn on the gpui connector's
        # `gpui-fast`, which changes the GPUI core its API is written in; the
        # published API is the default features'. (`${a[@]+"${a[@]}"}`: an
        # empty array is unset to `set -u` before bash 4.4.)
        semver_features=()
        if [ "$crate" = "native-theme-gpui" ]; then
            semver_features=(--default-features)
        fi
        run_check_soft "semver-checks ($crate)" cargo semver-checks check-release -p "$crate" ${semver_features[@]+"${semver_features[@]}"}
    done
fi

# ─────────────────────────────────────────────────────────────────────────────
# Section: security & dependency freshness
#
# cargo audit checks Cargo.lock against the RustSec advisory database. Hard: a
# known advisory must not sit in the lock, and an audit that cannot run (the
# database fetch, a lockfile it cannot parse) checked nothing. .cargo/audit.toml
# denies the informational advisories too (unmaintained, unsound, yanked), and
# lets one pass only through an ignore entry that gives its reason.
# ─────────────────────────────────────────────────────────────────────────────
print_section "Security audit"
if [[ $COMMIT_MODE ]]; then
    print_skipped "Security audit"
elif [[ ! $HAVE_AUDIT ]]; then
    printf "${DIM}   not run: cargo-audit is not installed (failed above)${NC}\n"
else
    run_check "cargo audit (RustSec advisories, .cargo/audit.toml)" cargo audit
fi

print_section "Outdated dependencies"
if [[ $COMMIT_MODE ]]; then
    print_skipped "Outdated dependencies"
elif [[ ! $HAVE_OUTDATED ]]; then
    printf "${DIM}   not run: cargo-outdated is not installed (failed above)${NC}\n"
else
    set +e
    # --depth 1 restricts to direct workspace dependencies; without it, cargo
    # outdated walks transitive deps and reports confusing duplicate-version
    # artifacts (e.g. "Latest 0.12.3" for hashbrown 0.17.0) from unrelated
    # subgraphs.
    OUTDATED_OUT=$(cargo outdated --workspace --depth 1 2>&1)
    OUTDATED_STATUS=$?
    set -e
    # A row is outdated when Latest ($4) is a version other than Project ($2).
    # Compat ($3) is `---` when every newer release is semver-incompatible, as
    # each 0.x minor bump is, so it is not the column to read. `--workspace`
    # lists a dependency under every member that has it, so each name and
    # version is counted once.
    OUTDATED_COUNT=$(printf "%s\n" "$OUTDATED_OUT" \
        | awk '$1 ~ /^[a-z]/ && $2 ~ /^[0-9]/ && $4 ~ /^[0-9]/ && $2 != $4 && !seen[$1 " " $2]++ {c++} END {print c+0}')
    if [ "$OUTDATED_STATUS" -ne 0 ]; then
        # A failed run lists no rows, so its count says nothing.
        print_warn "cargo outdated failed (exit $OUTDATED_STATUS): dependencies not checked"
        printf "${DIM}%s${NC}\n" "$(printf '%s\n' "$OUTDATED_OUT" | grep -v '^[[:space:]]*$' | tail -5)"
    elif [ "$OUTDATED_COUNT" -eq 0 ]; then
        print_ok "All dependencies current"
    else
        print_warn "$OUTDATED_COUNT outdated dependency/dependencies"
        printf "${DIM}%s${NC}\n" "$OUTDATED_OUT" | head -20
    fi
fi

# ─────────────────────────────────────────────────────────────────────────────
# Section: visual assets
#
# scripts/generate_assets_release.sh captures the screenshots and GIFs and stamps
# docs/assets/PROVENANCE.toml with a hash over the sources they came from
# (scripts/update_provenance.sh). A mismatch at HEAD is a warning while the
# CHANGELOG entry for this version still says "Unreleased" and a hard failure
# once it carries a date: the release commit exists, so this tree is about to
# be tagged. The crates.io workflow's release gate runs the same check on the tag.
# ─────────────────────────────────────────────────────────────────────────────
print_section "Visual assets"
if [[ $COMMIT_MODE ]]; then
    print_skipped "Visual assets"
else
    set +e
    STAMP_MSG=$(bash scripts/update_provenance.sh check 2>&1)
    STAMP_STATUS=$?
    CLEAN_MSG=$(bash scripts/update_provenance.sh verify-clean 2>&1)
    CLEAN_STATUS=$?
    set -e
    if [ "$STAMP_STATUS" -eq 0 ]; then
        print_ok "$STAMP_MSG"
    elif grep -qE "^## \[${CURRENT_VERSION//./\\.}\] - [0-9]{4}-[0-9]{2}-[0-9]{2}" CHANGELOG.md; then
        print_fail "$STAMP_MSG"
        printf "${DIM}    CHANGELOG.md dates %s: release tree, regenerate the assets before tagging${NC}\n" "$CURRENT_VERSION"
    else
        print_warn "$STAMP_MSG"
    fi
    if [ "$CLEAN_STATUS" -ne 0 ]; then
        print_warn "asset sources have uncommitted changes; the stamp check compared HEAD"
        printf "${DIM}%s${NC}\n" "$(printf '%s\n' "$CLEAN_MSG" | tail -n +2 | head -5)"
    fi
fi

# ─────────────────────────────────────────────────────────────────────────────
# Section: compatibility claims
#
# Each connector's README states the upstream set it has been verified
# against. scripts/update_compatibility.sh run earns that line — it resolves the
# newest upstream release on a throwaway lockfile, runs the connector's gates
# on it and stamps docs/COMPATIBILITY.toml — and `check` says here whether the
# connector's sources are still the ones that run covered. Same class as the
# asset stamp above: a warning while the CHANGELOG entry for this version
# still says "Unreleased" and a hard failure once it carries a date, because a
# release must not state a compatibility nobody verified. `check` reads git
# and the stamp, never the registry.
# ─────────────────────────────────────────────────────────────────────────────
print_section "Compatibility claims"
if [[ $COMMIT_MODE ]]; then
    print_skipped "Compatibility claims"
else
    set +e
    COMPAT_MSG=$(bash scripts/update_compatibility.sh check 2>&1)
    COMPAT_STATUS=$?
    set -e
    if [ "$COMPAT_STATUS" -eq 0 ]; then
        print_ok "$COMPAT_MSG"
    elif grep -qE "^## \[${CURRENT_VERSION//./\\.}\] - [0-9]{4}-[0-9]{2}-[0-9]{2}" CHANGELOG.md; then
        print_fail "$COMPAT_MSG"
        printf "${DIM}    CHANGELOG.md dates %s: release tree, re-verify before tagging${NC}\n" "$CURRENT_VERSION"
    else
        print_warn "$COMPAT_MSG"
    fi
fi

# ─────────────────────────────────────────────────────────────────────────────
# Final summary
# ─────────────────────────────────────────────────────────────────────────────
TOTAL=$((PASS_COUNT + WARN_COUNT + FAIL_COUNT))
ELAPSED_TOTAL=$(format_elapsed "$((SECONDS - SCRIPT_START))")

printf "\n${BOLD}═══════════════════════════════════════════════════════════════════${NC}\n"

if [[ $COMMIT_MODE ]]; then
    if [ "$FAIL_COUNT" -ne 0 ]; then
        printf "${BOLD}${RED}${ICON_FAIL} FAILED${NC} · native-theme v%s per-commit subset\n" "$CURRENT_VERSION"
    elif [ "$WARN_COUNT" -eq 0 ]; then
        printf "${BOLD}${GREEN}${ICON_PARTY} ALL PASS${NC} · native-theme v%s per-commit subset (not a release check)\n" "$CURRENT_VERSION"
    else
        printf "${BOLD}${GREEN}${ICON_PARTY} PASS WITH WARNINGS${NC} · native-theme v%s per-commit subset (not a release check)\n" "$CURRENT_VERSION"
    fi
elif [ "$FAIL_COUNT" -eq 0 ]; then
    if [ "$WARN_COUNT" -eq 0 ]; then
        printf "${BOLD}${GREEN}${ICON_PARTY} ALL PASS${NC} · native-theme v%s ready for release\n" "$CURRENT_VERSION"
    else
        printf "${BOLD}${GREEN}${ICON_PARTY} PASS WITH WARNINGS${NC} · native-theme v%s ready for release\n" "$CURRENT_VERSION"
    fi
else
    printf "${BOLD}${RED}${ICON_FAIL} FAILED${NC} · native-theme v%s NOT ready for release\n" "$CURRENT_VERSION"
fi

printf "   ${DIM}%d checks · ${GREEN}%d %s${NC}${DIM} · ${YELLOW}%d %s${NC}${DIM} · ${RED}%d %s${NC}${DIM} · %s total${NC}\n" \
    "$TOTAL" "$PASS_COUNT" "$ICON_OK" "$WARN_COUNT" "$ICON_WARN" "$FAIL_COUNT" "$ICON_FAIL" "$ELAPSED_TOTAL"

if [ "$WARN_COUNT" -gt 0 ]; then
    printf "\n${BOLD}${YELLOW}Warnings (re-summarised):${NC}\n"
    for w in ${WARNINGS[@]+"${WARNINGS[@]}"}; do
        printf "   ${YELLOW}${ICON_WARN}${NC} %s\n" "$w"
    done
fi

if [ "$FAIL_COUNT" -gt 0 ]; then
    printf "\n${BOLD}${RED}Failures (re-summarised):${NC}\n"
    for f in ${FAILURES[@]+"${FAILURES[@]}"}; do
        printf "   ${RED}${ICON_FAIL}${NC} %s\n" "$f"
    done
fi

if [[ $COMMIT_MODE ]]; then
    printf "\n${BOLD}Skipped (--commit mode):${NC}\n"
    for s in ${SKIPPED[@]+"${SKIPPED[@]}"}; do
        printf "   ${DIM}%s${NC}\n" "$s"
    done
fi

printf "${BOLD}═══════════════════════════════════════════════════════════════════${NC}\n"

if [[ $COMMIT_MODE ]]; then
    if [ "$FAIL_COUNT" -eq 0 ]; then
        printf "\n   The full run (${DIM}./scripts/check_release.sh${NC}) is required on the last commit of\n"
        printf "   every fix phase and before the release commit.\n\n"
        exit 0
    fi
    exit 1
elif [ "$FAIL_COUNT" -eq 0 ]; then
    printf "\n${BOLD}${BLUE}Next steps:${NC}\n"
    printf "   1. Review the changes once more\n"
    printf "   2. Visual assets and compatibility: if either check above warned, push, run\n"
    printf "      ${DIM}./scripts/generate_assets_release.sh${NC} and commit the assets with docs/assets/PROVENANCE.toml,\n"
    printf "      docs/COMPATIBILITY.toml and the connector READMEs it rewrote\n"
    printf "   3. Date the CHANGELOG entry, set its compare link, commit ${DIM}chore(release): v%s${NC}\n" "$CURRENT_VERSION"
    printf "   4. Re-run this script on that commit (both stamp checks are hard once the entry is dated)\n"
    printf "   5. Fast-forward main to it and push; CI runs on the exact commit\n"
    printf "   6. Tag: ${DIM}git tag v%s${NC}\n" "$CURRENT_VERSION"
    printf "   7. Push: ${DIM}git push origin v%s${NC}\n" "$CURRENT_VERSION"
    printf "   8. The pushed tag triggers .github/workflows/publish.yml: its release gate re-checks\n"
    printf "      the tag name and both stamps, then CI runs on the tag's commit, every crate is\n"
    printf "      packaged and documented as docs.rs will, and only then are they published in\n"
    printf "      dependency order with index waits between crates. Manual fallback, same order:\n"
    printf "      ${DIM}cargo publish -p native-theme-derive${NC}\n"
    printf "      ${DIM}cargo publish -p native-theme${NC}\n"
    printf "      ${DIM}cargo publish -p native-theme-build${NC}\n"
    printf "      ${DIM}cargo publish -p native-theme-iced${NC}\n"
    printf "      ${DIM}cargo publish -p native-theme-gpui${NC}\n"
    printf "      ${DIM}cargo publish -p native-theme-egui${NC}\n"
    printf "      ${DIM}cargo publish -p native-theme-egui-widgets${NC}\n"
    echo
    exit 0
else
    exit 1
fi
