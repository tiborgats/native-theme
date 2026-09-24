#!/usr/bin/env bash
# Feature-combination build check for every workspace crate.
#
# For each workspace member with a library target (from `cargo metadata
# --no-deps`), checks the library with no default features, with each feature
# of its `[features]` table alone (`--no-default-features --features F`,
# `default` excepted), and with all features: the coverage of `cargo hack
# check --each-feature` without a new tool. A combination fails when it does
# not build or when it emits a warning: cargo caps lints for dependencies, so
# every `warning:` line comes from a workspace crate. Prints one line per
# combination and exits non-zero when any fails, naming every failure at the
# end.
#
#   scripts/check-features.sh
#
# Requires jq. pre-release-check.sh, ci.yml, publish.yml and
# dependency-canary.yml run it.
set -euo pipefail

if ! command -v jq &>/dev/null; then
    echo "check-features.sh: jq is required (it reads the crates' features from cargo metadata); install jq and rerun" >&2
    exit 2
fi

ROOT=$(git rev-parse --show-toplevel 2>/dev/null || pwd)
cd "$ROOT"

# CI sets CARGO_TERM_COLOR=always; the excerpts and the warning scan read
# cargo's lines from their first character.
export CARGO_TERM_COLOR=never

METADATA=$(cargo metadata --no-deps --format-version 1)
CRATES=$(printf "%s" "$METADATA" | jq -r '
    .packages[]
    | select(any(.targets[]; any(.kind[]; . == "lib" or . == "proc-macro")))
    | .name')

FAILURES=()
TOTAL=0

check() {
    # $1 = crate, $2 = label, $3... = cargo check flags
    local crate="$1"
    local label="$2"
    shift 2
    local log
    local status=0
    local reason=""
    TOTAL=$((TOTAL + 1))
    log=$(cargo check -p "$crate" --lib "$@" 2>&1) || status=$?
    if [ "$status" -ne 0 ]; then
        reason="does not build"
    elif grep -q '^warning:' <<<"$log"; then
        reason="warns"
    fi
    if [ -z "$reason" ]; then
        printf "ok    %-22s %s\n" "$crate" "$label"
        return
    fi
    printf "FAIL  %-22s %s (%s)\n" "$crate" "$label" "$reason"
    { grep -E -A3 '^(error|warning)' <<<"$log" || true; } | sed -n '1,40p' | sed 's/^/        /'
    FAILURES+=("$crate: $label, $reason (cargo check -p $crate --lib $*)")
}

for crate in $CRATES; do
    check "$crate" "no default features" --no-default-features
    features=$(printf "%s" "$METADATA" \
        | jq -r --arg name "$crate" \
            '.packages[] | select(.name == $name) | .features | keys[] | select(. != "default")')
    for feature in $features; do
        check "$crate" "feature $feature alone" --no-default-features --features "$feature"
    done
    check "$crate" "all features" --all-features
done

echo
if [ "${#FAILURES[@]}" -eq 0 ]; then
    echo "All $TOTAL feature combinations build without warnings."
    exit 0
fi
echo "${#FAILURES[@]} of $TOTAL feature combinations fail:"
for failure in "${FAILURES[@]}"; do
    echo "  $failure"
done
exit 1
