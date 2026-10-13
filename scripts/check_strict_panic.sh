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
set -euo pipefail

ROOT=$(git rev-parse --show-toplevel 2>/dev/null || pwd)
cd "$ROOT"

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

cargo clippy "$@" -- "${STRICT_PANIC_LINTS[@]}"
