#!/bin/bash

# Pre-release check script for native-theme workspace.
# Performs comprehensive checks before releasing a new version.
#
# Output design (glance-first):
#   * Each check renders a single-line status when done:
#         ✅ description    (2.3s)       — pass
#         ⚠  description    (9.2s) soft  — soft-failure (warning, non-blocking)
#         ❌ description    (0.5s)       — hard failure (exits)
#   * During the check, the command's own stdout/stderr streams normally so
#     long-running work still shows progress.
#   * Sections are separated by `── Section ──` dividers.
#   * A final summary banner reports total / pass / warn / fail / elapsed,
#     and reprints every warning so nothing is buried above.

set -e  # Exit immediately if a command exits with a non-zero status
set -u  # Exit if an undefined variable is used

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
# Counters + warning log (re-printed in the final summary)
# ─────────────────────────────────────────────────────────────────────────────
PASS_COUNT=0
WARN_COUNT=0
FAIL_COUNT=0
WARNINGS=()
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
    FAIL_COUNT=$((FAIL_COUNT + 1))
}

print_info() {
    printf "${BLUE}${ICON_INFO}${NC} %s\n" "$1"
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
# run_check_soft  — soft fail (logs a warning, continues)
# run_tests[_soft] — variants that filter noise from `cargo test` output
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
        # Successful run: print compact summary (sum passed across binaries).
        local total_passed total_ignored
        total_passed=$(printf "%s\n" "$output" \
            | grep -oE 'test result: ok\. [0-9]+ passed' \
            | awk '{sum += $4} END {print sum+0}')
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
        if [ "$mode" = "soft" ]; then
            print_warn "$description" "$elapsed" "soft"
        else
            print_fail "$description" "$elapsed"
            printf "${RED}   Failed command:${NC} %s\n" "$*" >&2
            exit 1
        fi
    fi
}

run_tests()      { _run_tests_impl "$1" "hard" "${@:2}"; }
run_tests_soft() { _run_tests_impl "$1" "soft" "${@:2}"; }

# Optional-tool installer: prompts user if a cargo subcommand is missing.
check_and_install_tool() {
    local tool_name="$1"
    local package_name="$2"
    local description="$3"
    local subcommand="${tool_name#cargo-}"

    if cargo "$subcommand" --help >/dev/null 2>&1; then
        print_ok "$tool_name installed"
        return 0
    fi
    printf "${YELLOW}${ICON_WARN}${NC} %s is not installed — %s\n" "$tool_name" "$description"
    printf "${BLUE}    Install now? [Y/n]: ${NC}"
    read -r response
    if [[ -z "$response" || "$response" =~ ^[Yy]$ ]]; then
        if cargo install "$package_name" >/dev/null 2>&1; then
            print_ok "$package_name installed"
        else
            print_fail "$package_name install failed"
            exit 1
        fi
    else
        print_warn "$package_name skipped (optional)"
    fi
}

# ─────────────────────────────────────────────────────────────────────────────
# Preflight
# ─────────────────────────────────────────────────────────────────────────────
if [ ! -f "Cargo.toml" ]; then
    print_fail "Cargo.toml not found — run from project root"
    exit 1
fi

CURRENT_VERSION=$(grep -E '^version\s*=' Cargo.toml | sed -E 's/.*"([^"]+)".*/\1/')

printf "\n${BOLD}${ICON_ROCKET} native-theme v%s · pre-release checks${NC}\n" "$CURRENT_VERSION"

# ─────────────────────────────────────────────────────────────────────────────
# Section: environment
# ─────────────────────────────────────────────────────────────────────────────
print_section "Environment"

if [ ! -d ".git" ]; then
    print_warn "Not in a git repository (some checks skipped)"
else
    if ! git diff --quiet; then
        local_count=$(git status --porcelain | wc -l)
        print_info "Uncommitted changes present ($local_count files)"
    else
        print_ok "Working tree clean"
    fi

    if git tag --list | grep -q "^v${CURRENT_VERSION}$"; then
        print_warn "Tag v${CURRENT_VERSION} already exists in git"
    else
        print_ok "Tag v${CURRENT_VERSION} not yet created"
    fi
fi

check_and_install_tool "cargo-audit" "cargo-audit" "Security vulnerability scanner"
check_and_install_tool "cargo-outdated" "cargo-outdated" "Outdated-dependency checker"

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
# Detected patterns (outside #[cfg(test)] blocks):
#   .unwrap() · .expect(...) · panic!(...) · todo!(...) · unimplemented!(...)
#   unreachable!(...) · Instant::now() + / - (use checked_add/checked_sub)
#
# Regex-based tripwire only. The authoritative panic check is the
# "Strict panic lints" section below, which runs clippy with the full
# panic-prone lint set using type-aware analysis.
PANIC_FOUND=0
PANIC_HITS_ALL=""
for src_dir in native-theme/src connectors/native-theme-gpui/src connectors/native-theme-iced/src; do
    if [ -d "$src_dir" ]; then
        HITS=$(python3 -c "
import sys, re, glob

def scan_file(path):
    issues = []
    try:
        with open(path) as f:
            lines = f.readlines()
    except:
        return issues
    in_test = 0
    brace_depth = 0
    test_active = False
    for i, line in enumerate(lines, 1):
        s = line.strip()
        if re.search(r'#\[cfg\((all\()?test[,\)]', s):
            test_active = True
            continue
        opens = line.count('{')
        closes = line.count('}')
        if test_active and opens > 0:
            in_test = brace_depth + 1
            test_active = False
        brace_depth += opens - closes
        if in_test > 0 and brace_depth < in_test:
            in_test = 0
        if in_test > 0:
            continue
        if s.startswith('//'):
            continue
        if '.unwrap()' in line:
            issues.append(f'{path}:{i}: [.unwrap()] {s}')
        elif re.search(r'\.expect\s*\(', line):
            issues.append(f'{path}:{i}: [.expect()] {s}')
        elif re.search(r'\bpanic!\s*\(', line):
            issues.append(f'{path}:{i}: [panic!] {s}')
        elif re.search(r'\btodo!\s*\(', line):
            issues.append(f'{path}:{i}: [todo!] {s}')
        elif re.search(r'\bunimplemented!\s*\(', line):
            issues.append(f'{path}:{i}: [unimplemented!] {s}')
        elif re.search(r'\bunreachable!\s*\(', line):
            issues.append(f'{path}:{i}: [unreachable!] {s}')
        elif re.search(r'Instant::now\(\)\s*[-+]', line):
            issues.append(f'{path}:{i}: [Instant::now() arithmetic; use checked_add/checked_sub] {s}')
    return issues

issues = []
for f in glob.glob('${src_dir}/**/*.rs', recursive=True):
    if f.endswith('/tests.rs'):
        continue
    issues.extend(scan_file(f))
for issue in issues:
    print(issue)
sys.exit(0 if not issues else 1)
" 2>&1) || true
        if [ -n "$HITS" ]; then
            PANIC_HITS_ALL+="$HITS"$'\n'
            PANIC_FOUND=1
        fi
    fi
done

if [ "$PANIC_FOUND" -eq 1 ]; then
    print_fail "Panic-inducing pattern(s) in production code"
    printf "${DIM}%s${NC}" "$PANIC_HITS_ALL"
    printf "${RED}   Runtime panics are FORBIDDEN. Replace with Result/Option or checked_* arithmetic.${NC}\n" >&2
    exit 1
else
    print_ok "No panic-inducing patterns"
fi

# ─────────────────────────────────────────────────────────────────────────────
# Per-crate discovery
# ─────────────────────────────────────────────────────────────────────────────
if command -v jq &>/dev/null; then
    WORKSPACE_CRATES=$(cargo metadata --no-deps --format-version 1 2>/dev/null \
        | jq -r '.packages[].name')
else
    WORKSPACE_CRATES=$(cargo metadata --no-deps --format-version 1 2>/dev/null \
        | grep -oP '"workspace_members":\[[^\]]*\]' | grep -oP '[^/]+(?=#)')
fi

# ─────────────────────────────────────────────────────────────────────────────
# Section: cargo check
# ─────────────────────────────────────────────────────────────────────────────
print_section "cargo check"
for crate in $WORKSPACE_CRATES; do
    if [ "$crate" = "native-theme-gpui" ]; then
        run_check_soft "check ($crate)" cargo check -p "$crate" --all-targets
    else
        run_check "check ($crate)" cargo check -p "$crate" --all-targets
    fi
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
for crate in $WORKSPACE_CRATES; do
    if [ "$crate" = "native-theme-gpui" ]; then
        run_check_soft "clippy ($crate)" cargo clippy -p "$crate" --all-targets -- -D warnings
    else
        run_check "clippy ($crate)" cargo clippy -p "$crate" --all-targets -- -D warnings
    fi
done

# ─────────────────────────────────────────────────────────────────────────────
# Section: strict panic lints (library code only)
#
# Runs clippy with type-aware panic-prone lints promoted to errors. Covers
# every category of runtime panic the compiler/clippy can detect statically:
#
#   Explicit panics:    panic, todo, unimplemented, unreachable, manual_assert
#   Fallible unwrap:    unwrap_used, expect_used, unwrap_in_result
#   Slice/arr bounds:   indexing_slicing, string_slice
#   Integer overflow:   arithmetic_side_effects
#   Integer div/mod:    integer_division, modulo_arithmetic
#   Process exit:       exit
#   Result fn panics:   panic_in_result_fn
#
# Scope: library targets only (--lib). Tests and examples are exempt because
# they legitimately use .unwrap() / asserts for failure clarity and demo brevity.
# Bin targets (if any in the future) should be added to this list.
#
# `native-theme` is run with the full Linux feature set so feature-gated
# modules (kde, portal, system-icons, freedesktop, spinners, watch) are
# checked; plain `--lib` misses them.
# ─────────────────────────────────────────────────────────────────────────────
STRICT_PANIC_LINTS=(
    -D clippy::unwrap_used
    -D clippy::expect_used
    -D clippy::unwrap_in_result
    -D clippy::panic
    -D clippy::panic_in_result_fn
    -D clippy::todo
    -D clippy::unimplemented
    -D clippy::unreachable
    -D clippy::manual_assert
    -D clippy::indexing_slicing
    -D clippy::string_slice
    -D clippy::arithmetic_side_effects
    -D clippy::integer_division
    -D clippy::modulo_arithmetic
    -D clippy::exit
)
NT_FEATURES="kde,portal,system-icons,material-icons,lucide-icons,watch,svg-rasterize"

print_section "Strict panic lints (library code)"
run_check "strict-panic (native-theme, Linux features)" \
    cargo clippy -p native-theme --lib --features "$NT_FEATURES" -- "${STRICT_PANIC_LINTS[@]}"
run_check "strict-panic (native-theme, no features)" \
    cargo clippy -p native-theme --lib --no-default-features -- "${STRICT_PANIC_LINTS[@]}"
run_check "strict-panic (native-theme-derive)" \
    cargo clippy -p native-theme-derive --lib -- "${STRICT_PANIC_LINTS[@]}"
run_check "strict-panic (native-theme-build)" \
    cargo clippy -p native-theme-build --lib -- "${STRICT_PANIC_LINTS[@]}"
run_check "strict-panic (native-theme-iced)" \
    cargo clippy -p native-theme-iced --lib -- "${STRICT_PANIC_LINTS[@]}"
run_check_soft "strict-panic (native-theme-gpui)" \
    cargo clippy -p native-theme-gpui --lib -- "${STRICT_PANIC_LINTS[@]}"

# ─────────────────────────────────────────────────────────────────────────────
# Section: tests
# ─────────────────────────────────────────────────────────────────────────────
print_section "Tests"
for crate in $WORKSPACE_CRATES; do
    if [ "$crate" = "native-theme-gpui" ]; then
        run_tests_soft "test ($crate)" cargo test -p "$crate"
    else
        run_tests "test ($crate)" cargo test -p "$crate"
    fi
done

# ─────────────────────────────────────────────────────────────────────────────
# Section: examples (only crates with an examples/ directory)
# ─────────────────────────────────────────────────────────────────────────────
EXAMPLES_PRINTED=0
for crate in $WORKSPACE_CRATES; do
    crate_dir=$(cargo metadata --no-deps --format-version 1 2>/dev/null \
        | jq -r --arg name "$crate" '.packages[] | select(.name == $name) | .manifest_path' \
        | xargs dirname)
    if [ ! -d "$crate_dir/examples" ]; then
        continue
    fi
    if [ "$EXAMPLES_PRINTED" -eq 0 ]; then
        print_section "Examples"
        EXAMPLES_PRINTED=1
    fi
    if [ "$crate" = "native-theme-gpui" ]; then
        run_check_soft "examples ($crate)" cargo build -p "$crate" --examples
    else
        run_check "examples ($crate)" cargo build -p "$crate" --examples
    fi
done

# ─────────────────────────────────────────────────────────────────────────────
# Section: docs
# ─────────────────────────────────────────────────────────────────────────────
print_section "Docs"
for crate in $WORKSPACE_CRATES; do
    if [ "$crate" = "native-theme-gpui" ]; then
        run_check_soft "docs ($crate)" cargo doc -p "$crate" --no-deps
    else
        run_check "docs ($crate)" cargo doc -p "$crate" --no-deps
    fi
done

# ─────────────────────────────────────────────────────────────────────────────
# Section: packaging
#
# --no-verify rationale: cargo package verification compiles each tarball as if
# it were downloaded from crates.io. For first-ever publication of this
# workspace, the internal proc-macro crate native-theme-derive is not yet on
# the registry, so tarball verification cannot resolve the workspace-internal
# dep. The real tarball compilation check happens during `cargo publish`
# itself (not run here). See RELEASING.md for the ordered publish workflow.
#
# Once native-theme-derive 0.5.7 is published to crates.io, remove --no-verify
# from these three lines to restore full tarball-compile verification.
# ─────────────────────────────────────────────────────────────────────────────
print_section "Packaging"
run_check "package (core: derive · native-theme · build)" \
    cargo package --no-verify -p native-theme-derive -p native-theme -p native-theme-build --allow-dirty
run_check_soft "package (native-theme-iced)" \
    cargo package --no-verify -p native-theme-derive -p native-theme -p native-theme-iced --allow-dirty
run_check_soft "package (native-theme-gpui)" \
    cargo package --no-verify -p native-theme-derive -p native-theme -p native-theme-gpui --allow-dirty

# ─────────────────────────────────────────────────────────────────────────────
# Section: security & dependency freshness
# ─────────────────────────────────────────────────────────────────────────────
print_section "Security audit"
set +e
AUDIT_OUT=$(cargo audit 2>&1)
AUDIT_STATUS=$?
set -e
if [ "$AUDIT_STATUS" -eq 0 ]; then
    print_ok "cargo audit — no advisories"
else
    # Extract "Warning:" or "Error:" lines for compact warning surfacing.
    AUDIT_SUMMARY=$(printf "%s\n" "$AUDIT_OUT" | grep -E '^\s*(Crate|Title|ID):\s' | head -12 || true)
    print_warn "cargo audit reported findings"
    if [ -n "$AUDIT_SUMMARY" ]; then
        printf "${DIM}%s${NC}\n" "$AUDIT_SUMMARY"
    fi
fi

print_section "Outdated dependencies"
set +e
# --depth 1 restricts to direct workspace dependencies; without it, cargo
# outdated walks transitive deps and reports confusing duplicate-version
# artifacts (e.g. "Latest 0.12.3" for hashbrown 0.17.0) from unrelated
# subgraphs.
OUTDATED_OUT=$(cargo outdated --workspace --depth 1 2>&1)
set -e
OUTDATED_COUNT=$(printf "%s\n" "$OUTDATED_OUT" \
    | awk '$1 ~ /^[a-z]/ && $2 ~ /^[0-9]/ && $3 ~ /^[0-9]/ && $2 != $3 {c++} END {print c+0}')
if [ "$OUTDATED_COUNT" -eq 0 ]; then
    print_ok "All dependencies current"
else
    print_warn "$OUTDATED_COUNT outdated dependency/dependencies"
    printf "${DIM}%s${NC}\n" "$OUTDATED_OUT" | head -20
fi

# ─────────────────────────────────────────────────────────────────────────────
# Final summary
# ─────────────────────────────────────────────────────────────────────────────
TOTAL=$((PASS_COUNT + WARN_COUNT + FAIL_COUNT))
ELAPSED_TOTAL=$(format_elapsed "$((SECONDS - SCRIPT_START))")

printf "\n${BOLD}═══════════════════════════════════════════════════════════════════${NC}\n"

if [ "$FAIL_COUNT" -eq 0 ]; then
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
    for w in "${WARNINGS[@]}"; do
        printf "   ${YELLOW}${ICON_WARN}${NC} %s\n" "$w"
    done
fi

printf "${BOLD}═══════════════════════════════════════════════════════════════════${NC}\n"

if [ "$FAIL_COUNT" -eq 0 ]; then
    printf "\n${BOLD}${BLUE}Next steps:${NC}\n"
    printf "   1. Review the changes once more\n"
    printf "   2. Update CHANGELOG.md if needed\n"
    printf "   3. Commit any final changes\n"
    printf "   4. Tag: ${DIM}git tag v%s${NC}\n" "$CURRENT_VERSION"
    printf "   5. Push: ${DIM}git push origin v%s${NC}\n" "$CURRENT_VERSION"
    printf "   6. Publish in order (see RELEASING.md):\n"
    printf "      ${DIM}cargo publish -p native-theme-derive${NC}\n"
    printf "      ${DIM}cargo publish -p native-theme${NC}\n"
    printf "      ${DIM}cargo publish -p native-theme-build${NC}\n"
    printf "      ${DIM}cargo publish -p native-theme-iced${NC}\n"
    printf "      ${DIM}cargo publish -p native-theme-gpui${NC}\n"
    echo
    exit 0
else
    exit 1
fi
