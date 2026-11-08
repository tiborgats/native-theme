#!/bin/bash

# The step cache of scripts/check_release.sh's --quick and --commit modes,
# sourced by it; the full run never uses it, and --no-cache turns it off.
#
# A step is skipped only when the key of its run equals the key recorded when
# it last passed. The key is a SHA-256 over:
#   * the base: the tool versions and environment step_cache_init is given
#     (check_release.sh passes `rustc -vV`, `cargo -vV`, clippy's, rustdoc's,
#     rustfmt's, Python's and nextest's versions, the files of the scripts
#     themselves, and the environment but `_`, `SHLVL` and `OLDPWD`, which
#     bash sets per process);
#   * the step's description and its command, each argument;
#   * the content of its declared inputs: each file's path and bytes, every
#     file under a declared directory, a glob's matches, and a missing input
#     by its name, so an input added, removed, renamed or edited changes it.
#     A directory's `__pycache__` folders are left out: Python writes them
#     while a check runs, from the sources beside them, which are hashed.
#     An input `!<pattern>` leaves out every file the other inputs name
#     whose path the bash pattern matches (a step that reads one bundled
#     preset leaves out the others, which the crate's directory holds).
# A failed step records nothing; a step whose inputs change while it runs
# records nothing either. A step that declares no inputs always runs.
#
# Usage (sourced): step_cache_init <dir> <label> <base text>, then per step
# step_cache_key <description> <inputs...> -- <command...> (prints the key),
# step_cache_hit <description> <key> (prints the label recorded, returns 1
# on a miss) and step_cache_record <description> <key>.
#
#   scripts/step_cache.sh --self-test
#
# runs the self-test: a step re-runs after any declared input, the base (the
# script's own hash among it), the command or the environment changes, and a
# failed step records nothing; a file an input `!<pattern>` leaves out
# changes nothing, its neighbours still do.

STEP_CACHE_DIR=""
STEP_CACHE_LABEL=""
STEP_CACHE_BASE=""

# $1 = the cache directory, $2 = the label a record states ("<commit>"),
# $3 = the base text (tool versions, scripts, environment).
step_cache_init() {
    STEP_CACHE_DIR="$1"
    STEP_CACHE_LABEL="$2"
    STEP_CACHE_BASE=$(printf "%s" "$3" | sha256sum | cut -d' ' -f1)
    mkdir -p "$STEP_CACHE_DIR"
}

# The environment as part of a base: every variable but those bash sets per
# process, sorted, NUL-separated entries hashed.
step_cache_environment() {
    env -0 | grep -zvE '^(_|SHLVL|OLDPWD)=' | sort -z | sha256sum | cut -d' ' -f1
}

# The content hash of the inputs $@: a directory's files (symbolic links
# followed) by path and content, a file by path and content, a glob's
# matches, a missing input by name; each file an input `!<pattern>` matches
# left out.
step_cache_inputs() {
    local input
    local excluded=()
    for input in "$@"; do
        if [[ "$input" == '!'* ]]; then
            excluded+=("${input#!}")
        fi
    done
    step_cache_listing "$@" | step_cache_excluding ${excluded[@]+"${excluded[@]}"} | sha256sum | cut -d' ' -f1
}

# The lines step_cache_inputs hashes for the inputs $@, the `!<pattern>`
# inputs skipped: `sha256sum` lines, `directory <path>` and
# `missing <input>`.
step_cache_listing() {
    local input match matches
    for input in "$@"; do
        if [[ "$input" == '!'* ]]; then
            continue
        fi
        if [[ "$input" == *[*?[]* ]]; then
            mapfile -t matches < <(compgen -G "$input" | sort)
            if [ "${#matches[@]}" -eq 0 ]; then
                printf 'missing %s\n' "$input"
            fi
        else
            matches=("$input")
        fi
        for match in ${matches[@]+"${matches[@]}"}; do
            if [ -d "$match" ]; then
                printf 'directory %s\n' "$match"
                find -L "$match" -type d -name __pycache__ -prune -o -type f -print0 \
                    | sort -z | xargs -0 -r sha256sum --
            elif [ -e "$match" ]; then
                sha256sum -- "$match"
            else
                printf 'missing %s\n' "$match"
            fi
        done
    done
}

# Standard input's lines but each `sha256sum` line whose path one of the
# patterns $@ matches.
step_cache_excluding() {
    local line path pattern
    while IFS= read -r line; do
        if [[ "$line" =~ ^[0-9a-f]{64}\ [\ *](.*)$ ]]; then
            path="${BASH_REMATCH[1]}"
            for pattern in "$@"; do
                # shellcheck disable=SC2053  # the pattern matches as a glob
                if [[ "$path" == $pattern ]]; then
                    continue 2
                fi
            done
        fi
        printf '%s\n' "$line"
    done
}

# Prints the key of the step $1 with the inputs before `--` and the command
# after it.
step_cache_key() {
    local description="$1"
    shift
    local inputs=()
    while [ "$#" -gt 0 ] && [ "$1" != "--" ]; do
        inputs+=("$1")
        shift
    done
    [ "$#" -gt 0 ] && shift
    {
        printf 'base %s\n' "$STEP_CACHE_BASE"
        printf 'step %s\n' "$description"
        printf 'command'
        printf ' %q' "$@"
        printf '\n'
        printf 'inputs %s\n' "$(step_cache_inputs ${inputs[@]+"${inputs[@]}"})"
    } | sha256sum | cut -d' ' -f1
}

# The record file of the step $1.
step_cache_file() {
    printf "%s/%s" "$STEP_CACHE_DIR" "$(printf "%s" "$1" | sha256sum | cut -d' ' -f1)"
}

# Returns 0 and prints the recorded label when the step $1 last passed with
# the key $2; 1 otherwise.
step_cache_hit() {
    local file key label
    file=$(step_cache_file "$1")
    [ -f "$file" ] || return 1
    { IFS= read -r key && IFS= read -r label; } <"$file" || return 1
    [ "$key" = "$2" ] || return 1
    printf "%s" "$label"
}

# Records that the step $1 passed with the key $2.
step_cache_record() {
    local file
    file=$(step_cache_file "$1")
    printf "%s\n%s\n%s\n" "$2" "$STEP_CACHE_LABEL" "$1" >"$file.tmp" && mv "$file.tmp" "$file"
}

# The self-test: a synthetic step over a synthetic tree in a temporary
# directory.
step_cache_self_test() {
    local dir failures=0
    mkdir -p "$STEP_CACHE_SELF_TEST_ROOT"
    dir=$(mktemp -d "$STEP_CACHE_SELF_TEST_ROOT/step-cache-self-test.XXXXXX") || return 1
    mkdir -p "$dir/tree/inputs/sub" "$dir/cache"
    printf 'a\n' >"$dir/tree/file.txt"
    printf 'b\n' >"$dir/tree/inputs/sub/nested.txt"
    printf 'c\n' >"$dir/tree/one.glob"
    mkdir -p "$dir/tree/inputs/__pycache__"
    printf '#!/bin/bash\n' >"$dir/tree/script.sh"
    local inputs=("$dir/tree/file.txt" "$dir/tree/inputs" "$dir/tree/*.glob")
    local base_of
    base_of() { printf "tools 1\n%s\n%s" "$(sha256sum <"$dir/tree/script.sh")" "$(step_cache_environment)"; }
    # One run of the synthetic step as check_release.sh runs it; $1 = what
    # the command exits with. Prints "skipped" or "ran".
    run_once() {
        local key
        step_cache_init "$dir/cache" "test-label" "$(base_of)"
        key=$(step_cache_key "a step" "${inputs[@]}" -- true "$@")
        if step_cache_hit "a step" "$key" >/dev/null; then
            printf "skipped"
            return
        fi
        printf "ran"
        if [ "${1:-0}" -eq 0 ]; then
            step_cache_record "a step" "$key"
        fi
    }
    # $1 = the case, $2 = "ran" or "skipped", $3... = run_once's arguments.
    expect() {
        local got
        got=$(run_once "${@:3}")
        if [ "$got" != "$2" ]; then
            printf "step cache self-test: %s: expected %s, got %s\n" "$1" "$2" "$got" >&2
            failures=$((failures + 1))
        fi
    }
    expect "first run" ran
    expect "unchanged" skipped
    printf 'a2\n' >"$dir/tree/file.txt"
    expect "a file edited" ran
    expect "after the edit, unchanged" skipped
    printf 'b2\n' >"$dir/tree/inputs/sub/nested.txt"
    expect "a file in a directory edited" ran
    printf 'x\n' >"$dir/tree/inputs/__pycache__/module.pyc"
    expect "a __pycache__ file written" skipped
    printf 'd\n' >"$dir/tree/inputs/added.txt"
    expect "a file added to a directory" ran
    mv "$dir/tree/inputs/added.txt" "$dir/tree/inputs/renamed.txt"
    expect "a file renamed" ran
    rm "$dir/tree/inputs/renamed.txt"
    expect "a file removed" ran
    printf 'e\n' >"$dir/tree/two.glob"
    expect "a glob's new match" ran
    printf 'c2\n' >"$dir/tree/one.glob"
    expect "a glob's match edited" ran
    rm "$dir/tree/file.txt"
    expect "a file deleted" ran
    printf 'a\n' >"$dir/tree/file.txt"
    expect "the file restored" ran
    expect "restored, unchanged" skipped
    inputs+=("!$dir/tree/inputs/sub/*.left")
    expect "an exclusion that matches nothing" skipped
    printf 'f\n' >"$dir/tree/inputs/sub/out.left"
    expect "a file an exclusion leaves out added" skipped
    printf 'f2\n' >"$dir/tree/inputs/sub/out.left"
    expect "a file an exclusion leaves out edited" skipped
    printf 'b3\n' >"$dir/tree/inputs/sub/nested.txt"
    expect "its neighbour edited" ran
    rm "$dir/tree/inputs/sub/out.left"
    expect "a file an exclusion leaves out removed" skipped
    printf '#!/bin/bash\n# edited\n' >"$dir/tree/script.sh"
    expect "the script edited" ran
    expect "the command's arguments changed" ran 0 --flag
    export STEP_CACHE_SELF_TEST_VARIABLE=1
    expect "the environment changed" ran
    expect "the environment, unchanged" skipped
    unset STEP_CACHE_SELF_TEST_VARIABLE
    expect "the environment back (the record holds the last pass)" ran
    printf 'a3\n' >"$dir/tree/file.txt"
    expect "failed" ran 1
    expect "a failure recorded nothing" ran 1
    expect "passed" ran
    expect "after the pass" skipped
    rm -rf "$dir"
    if [ "$failures" -ne 0 ]; then
        return 1
    fi
    printf "step cache self-test: every case passed\n"
}

# The self-test's temporary directories: under the repository's target/.
STEP_CACHE_SELF_TEST_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)/target"

if [[ "${BASH_SOURCE[0]}" == "$0" ]]; then
    set -euo pipefail
    case "${1:-}" in
        --self-test) step_cache_self_test ;;
        *) printf "usage: scripts/step_cache.sh --self-test   (sourced by scripts/check_release.sh)\n" >&2; exit 2 ;;
    esac
fi
