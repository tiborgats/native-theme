#!/usr/bin/env bash
# Strict panic lints: `cargo clippy` with the type-aware panic-prone lints
# promoted to errors. Covers every category of runtime panic the
# compiler/clippy can detect statically:
#
#   Explicit panics:    panic, todo, unimplemented, unreachable, manual_assert
#   Fallible unwrap:    unwrap_used, expect_used, unwrap_in_result
#   Slice/arr bounds:   indexing_slicing, string_slice
#   Integer overflow:   arithmetic_side_effects
#   Integer div/mod:    integer_division, modulo_arithmetic
#   Process exit:       exit
#   Result fn panics:   panic_in_result_fn
#
# The arguments are clippy's, before the `--`: the package, `--lib`, the
# features and, for the macOS and Windows code, the target.
#
#   scripts/check_strict_panic.sh -p native-theme-iced --lib
#   scripts/check_strict_panic.sh --target x86_64-apple-darwin -p native-theme --lib --features macos
#
# native-theme and the connectors deny `clippy::unwrap_used` and
# `clippy::expect_used` in their crate roots; the rest of the list is enforced
# only here. scripts/check_release.sh and ci.yml run it, so the list is kept in
# this one place.
#
# The results are kept apart from the ordinary clippy's (`-- -D warnings`).
# clippy-driver gets the lints in CLIPPY_ARGS, which every unit it lints
# records as an `# env-dep:` of its dep-info, and cargo tracks such a variable
# in the unit's fingerprint only, not in the metadata hash that names the
# unit's files (the table in cargo's src/compiler/fingerprint/mod.rs:67-90 and
# :193-197, at cargo 1.99.1). So the two lint sets shared one result per unit
# of a workspace crate and each replaced the other's: native-theme-derive and
# native-theme's build script were compiled again at every switch, and each
# workspace crate above them checked again, changed or not (cargo's log,
# `CARGO_LOG=cargo::compiler::fingerprint=info`: `ChangedEnv { var:
# "CLIPPY_ARGS", .. }`). The profile is in the metadata hash, so this run sets
# `debug = false` on each workspace crate (`[profile.dev.package.<crate>]`,
# clippy's dev profile, through `--config`): their units get files of their
# own beside the ordinary clippy's, in the same target directories. Debug
# info is code generation's, which comes after the lints, so no lint result
# changes; the dependencies, whose lints cargo caps, keep their one build. A
# named profile (`--profile strict-panic`) or a target directory of its own
# would keep the results apart too, but would check every dependency a second
# time, in CI's steps as well: the gpui connector's library alone took 52 s
# and 1.4 GB so (2026-10-24, at a load of 5).
#
# The separation holds for clippy's dev profile only. With `--release` or
# `--profile <name>` among the arguments, cargo builds another profile, which
# the `[profile.dev.package.<crate>]` settings do not reach (and cargo's
# release profile has `debug = false` already, so the same setting there would
# change no hash), so the two lint sets share one result
# per unit there again: each run is as correct, only slower. No caller passes
# either.
#
# Requires jq: the workspace crates are read from `cargo metadata`. A metadata
# that names no workspace crate is an error, not a run without the separation.
set -euo pipefail

if ! command -v jq &>/dev/null; then
    echo "check_strict_panic.sh: jq is required (it reads the workspace crates from cargo metadata); install jq and rerun" >&2
    exit 2
fi

ROOT=$(git rev-parse --show-toplevel 2>/dev/null || pwd)
cd "$ROOT"

WORKSPACE_CRATES=$(cargo metadata --no-deps --format-version 1 | jq -r '.packages[].name')
if [ -z "$WORKSPACE_CRATES" ]; then
    echo "check_strict_panic.sh: cargo metadata names no workspace crate, so the strict lints' results cannot be kept apart from the ordinary clippy's" >&2
    exit 1
fi
OWN_RESULTS=()
for crate in $WORKSPACE_CRATES; do
    OWN_RESULTS+=(--config "profile.dev.package.$crate.debug=false")
done

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

cargo clippy "$@" "${OWN_RESULTS[@]}" -- "${STRICT_PANIC_LINTS[@]}"
