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
#     Paths are sorted bytewise (LC_ALL=C), whatever the locale.
# A failed step records nothing; a step whose inputs change while it runs
# records nothing either. A step that declares no inputs always runs. Its
# records are the worktree's own (check_release.sh: target/check-release-cache).
#
# A shared step (check_release.sh's CACHE_SHARED: the facts gate's parts) is
# keyed narrowly instead, so that its result is reused wherever what decides
# its verdict is the same, in any worktree of the repository:
#   * the shared base (step_cache_init_shared): the tool versions as above;
#     the environment variables that can change a verdict alone
#     (step_cache_verdict_environment, whose comment lists each rule and its
#     reason); the text of the cache's own key functions; and what the
#     caller adds (check_release.sh: the text of its cache functions). The
#     scripts' hashes and the rest of the environment are not in it, so an
#     unrelated edit of a script, another working directory, a thread count
#     or an unrelated variable keeps the record;
#   * the step's description and command, as above;
#   * the text of the functions that wrap it between the caller's call and
#     the command's start (`declare -f`), each named where it is declared;
#   * its declared inputs' content, as above, each path under the worktree's
#     root written relative to it, and a missing path outside the root left
#     out (Cargo's configuration files of the root's ancestors: which of them
#     are looked for depends on where the worktree is, not on what it holds;
#     one that exists is hashed by path and content).
# Its records live in the repository's common git directory
# (step_cache_shared_dir: <git-common-dir>/check-release-cache), one for every
# worktree, and each names the commit and worktree it passed in and when.
#
# Usage (sourced): step_cache_init <dir> <label> <base text>, and for shared
# steps step_cache_init_shared <dir> <root> <label> <base text>; then per step
# step_cache_key [--shared <functions>] <description> <inputs...> --
# <command...> (prints the key), step_cache_hit [--shared] <description>
# <key> (prints the label recorded, returns 1 on a miss) and
# step_cache_record [--shared] <description> <key>.
#
#   scripts/step_cache.sh --self-test
#
# runs the self-test: a step re-runs after any declared input, the base (the
# script's own hash among it), the command or the environment changes, and a
# failed step records nothing; a file an input `!<pattern>` leaves out
# changes nothing, its neighbours still do. A shared step is reused after an
# unrelated edit of the calling script, in another worktree with the same
# content (its record found there), under another thread count, locale,
# evidence path or unrelated variable, and re-runs after a declared input, a
# verdict variable, the command, a wrapping function's text or a tool version
# changes; a step that is not shared re-runs after each of these.

STEP_CACHE_DIR=""
STEP_CACHE_LABEL=""
STEP_CACHE_BASE=""
STEP_CACHE_SHARED_DIR=""
STEP_CACHE_SHARED_LABEL=""
STEP_CACHE_SHARED_BASE=""
STEP_CACHE_ROOT=""

# $1 = the cache directory, $2 = the label a record states ("<commit>"),
# $3 = the base text (tool versions, scripts, environment).
step_cache_init() {
    STEP_CACHE_DIR="$1"
    STEP_CACHE_LABEL="$2"
    STEP_CACHE_BASE=$(printf "%s" "$3" | sha256sum | cut -d' ' -f1)
    mkdir -p "$STEP_CACHE_DIR"
}

# The shared steps' cache: $1 = its directory (step_cache_shared_dir), $2 =
# the worktree's root, $3 = the label a record states ("<commit> in
# <worktree>"; the record adds when), $4 = the base text (tool versions,
# step_cache_verdict_environment, the caller's cache functions). The cache's
# own key functions join the base.
step_cache_init_shared() {
    STEP_CACHE_SHARED_DIR="$1"
    STEP_CACHE_ROOT="$2"
    STEP_CACHE_SHARED_LABEL="$3"
    STEP_CACHE_SHARED_BASE=$(
        {
            printf "%s\n" "$4"
            step_cache_functions step_cache_key step_cache_inputs step_cache_listing \
                step_cache_excluding step_cache_relative step_cache_functions \
                step_cache_verdict_environment step_cache_directory_content
        } | sha256sum | cut -d' ' -f1
    )
}

# The shared steps' cache directory of the worktree whose root is $1: the
# folder check-release-cache in the repository's common git directory, the
# same from every worktree.
step_cache_shared_dir() {
    local common
    common=$(git -C "$1" rev-parse --path-format=absolute --git-common-dir 2>/dev/null) || return 1
    [ -n "$common" ] || return 1
    printf "%s/check-release-cache" "$common"
}

# The environment as part of a base: every variable but those bash sets per
# process, sorted, NUL-separated entries hashed.
step_cache_environment() {
    env -0 | grep -zvE '^(_|SHLVL|OLDPWD)=' | LC_ALL=C sort -z | sha256sum | cut -d' ' -f1
}

# The environment as part of the shared base: the variables that can change
# a verdict of the facts gate or of its build, sorted, NUL-separated entries
# hashed. In:
#   * NT_* (the gate's NT_FACTS_GATE_PRESETS and NT_FACTS_GATE_SHARD select
#     what it checks; the project's other NT_ variables, current and future,
#     too) and FACTS_GATE_RELEASE (the release's stricter gate);
#     NT_FACTS_GATE_MERGE, a path, by the content of the directory it names
#     (step_cache_directory_content): it makes the gate check the shards'
#     listed paths there instead of gating;
#   * NATIVE_THEME_* (the project's own; the test fixtures' root among them);
#   * CARGO_* (Cargo's configuration: profiles, CARGO_TARGET_<triple>_*,
#     rustflags, the target, unstable flags, aliases, registries) and the
#     CARGO_CFG_*/CARGO_FEATURE_* build scripts read;
#   * RUSTFLAGS, RUSTDOCFLAGS, RUSTC, RUSTC_* (RUSTC_WRAPPER,
#     RUSTC_WORKSPACE_WRAPPER, RUSTC_BOOTSTRAP: a wrapper is a program run in
#     rustc's place), RUSTDOC, RUSTDOC_*, RUSTFMT, RUSTUP_* (the toolchain
#     chosen) and RUST_* (RUST_MIN_STACK decides whether a test overflows its
#     stack; RUST_TEST_* other than the two below);
#   * NEXTEST_* (a test step run by nextest: its profile, its retries);
#   * GIT_* (GIT_DIR, GIT_WORK_TREE, GIT_CEILING_DIRECTORIES, GIT_CONFIG_*:
#     where the git the gate runs on the evidence and the source cache
#     looks).
# Out, each named below:
#   * how fast it runs: NT_FACTS_GATE_THREADS, CARGO_BUILD_JOBS,
#     CARGO_INCREMENTAL, CARGO_CACHE_RUSTC_INFO, RUST_TEST_THREADS,
#     NEXTEST_TEST_THREADS;
#   * what it prints: NT_FACTS_GATE_TIMING, CARGO_TERM_*, CARGO_LOG,
#     RUST_BACKTRACE, RUST_LIB_BACKTRACE, RUST_LOG, RUSTC_LOG,
#     RUST_TEST_NOCAPTURE, NEXTEST's status, output and progress settings;
#   * where it builds: CARGO_HOME (its configuration files are inputs, by
#     path and content), CARGO_TARGET_DIR, CARGO_BUILD_TARGET_DIR,
#     CARGO_BUILD_BUILD_DIR;
#   * where it writes, never reads: NT_FACTS_GATE_LISTED (a shard's listed
#     paths; check_release.sh runs no shard);
#   * paths to pinned data: NATIVE_THEME_EVIDENCE and NATIVE_THEME_SOURCES
#     (the evidence root and the source cache): the gate refuses either
#     unless it is a clean checkout at the commit docs/facts/sources.toml
#     pins, an input of every gate step, so the pin is the data's identity;
#   * git's editors, pager, prompts, tracing, identity and transport
#     (GIT_EDITOR, GIT_SEQUENCE_EDITOR, GIT_PAGER, GIT_ASKPASS,
#     GIT_TERMINAL_PROMPT, GIT_TRACE*, GIT_AUTHOR_*, GIT_COMMITTER_*,
#     GIT_SSH*): the gate's git commands read none of them;
#   * every other variable: neither the gate (native-theme/tests/facts_gate.rs
#     and facts_gate/), native-theme's build script nor the build scripts its
#     tests build read it. PATH and HOME among them: the programs PATH
#     selects that compile or run the gate are keyed by their versions, and
#     the Cargo configuration under HOME by path and content; git, which the
#     gate asks whether the pinned checkouts are at their pins, fails the
#     gate when it cannot answer. The locale (LANG, LC_*, LANGUAGE): the gate
#     compares no localised text (git's porcelain and plumbing output is not
#     translated), and the keys sort bytewise.
step_cache_verdict_environment() {
    local entry name
    while IFS= read -r -d '' entry; do
        name="${entry%%=*}"
        case "$name" in
            NT_FACTS_GATE_THREADS | NT_FACTS_GATE_TIMING | NT_FACTS_GATE_LISTED) continue ;;
            NT_FACTS_GATE_MERGE) entry="$name=$(step_cache_directory_content "${entry#*=}")" ;;
            NATIVE_THEME_EVIDENCE | NATIVE_THEME_SOURCES) continue ;;
            CARGO_BUILD_JOBS | CARGO_INCREMENTAL | CARGO_CACHE_RUSTC_INFO | CARGO_TERM_* | CARGO_LOG | \
                CARGO_HOME | CARGO_TARGET_DIR | CARGO_BUILD_TARGET_DIR | CARGO_BUILD_BUILD_DIR) continue ;;
            RUST_TEST_THREADS | RUST_BACKTRACE | RUST_LIB_BACKTRACE | RUST_LOG | RUSTC_LOG | \
                RUST_TEST_NOCAPTURE) continue ;;
            NEXTEST_TEST_THREADS | NEXTEST_STATUS_LEVEL | NEXTEST_FINAL_STATUS_LEVEL | \
                NEXTEST_FAILURE_OUTPUT | NEXTEST_SUCCESS_OUTPUT | NEXTEST_HIDE_PROGRESS_BAR | \
                NEXTEST_SHOW_PROGRESS | NEXTEST_NO_INPUT_HANDLER) continue ;;
            GIT_EDITOR | GIT_SEQUENCE_EDITOR | GIT_PAGER | GIT_ASKPASS | GIT_TERMINAL_PROMPT | \
                GIT_TRACE* | GIT_AUTHOR_* | GIT_COMMITTER_* | GIT_SSH*) continue ;;
            NT_* | FACTS_GATE_RELEASE | NATIVE_THEME_* | CARGO_* | RUSTFLAGS | RUSTDOCFLAGS | RUSTC | \
                RUSTC_* | RUSTDOC | RUSTDOC_* | RUSTFMT | RUSTUP_* | RUST_* | NEXTEST_* | GIT_*) ;;
            *) continue ;;
        esac
        printf '%s\0' "$entry"
    done < <(env -0) | LC_ALL=C sort -z | sha256sum | cut -d' ' -f1
}

# The identity of the directory $1 names, as a variable's value: its files'
# paths (relative to it) and content, or `missing`. A relative path is the
# gate's, which cargo runs in its package's directory (native-theme/).
step_cache_directory_content() {
    local directory="$1"
    if [[ "$directory" != /* ]]; then
        directory="native-theme/$directory"
    fi
    if [ ! -d "$directory" ]; then
        printf "missing"
        return
    fi
    (cd "$directory" && find -L . -type f -print0 | LC_ALL=C sort -z | xargs -0 -r sha256sum --) \
        | sha256sum | cut -d' ' -f1
}

# The text of the shell functions $@, each by `declare -f`; a function not
# defined by name.
step_cache_functions() {
    local name
    for name in "$@"; do
        declare -f "$name" || printf "no function %s\n" "$name"
    done
}

# The content hash of the inputs $@ (after `--relative <root>`, each path
# under <root> written relative to it, step_cache_relative): a directory's
# files (symbolic links followed) by path and content, a file by path and
# content, a glob's matches, a missing input by name; each file an input
# `!<pattern>` matches left out.
step_cache_inputs() {
    local root=""
    if [ "${1:-}" = "--relative" ]; then
        root="$2"
        shift 2
    fi
    local input
    local excluded=()
    for input in "$@"; do
        if [[ "$input" == '!'* ]]; then
            excluded+=("${input#!}")
        fi
    done
    if [ -n "$root" ]; then
        step_cache_listing "$@" | step_cache_excluding ${excluded[@]+"${excluded[@]}"} \
            | step_cache_relative "$root" | sha256sum | cut -d' ' -f1
    else
        step_cache_listing "$@" | step_cache_excluding ${excluded[@]+"${excluded[@]}"} \
            | sha256sum | cut -d' ' -f1
    fi
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
            mapfile -t matches < <(compgen -G "$input" | LC_ALL=C sort)
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
                    | LC_ALL=C sort -z | xargs -0 -r sha256sum --
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

# Standard input's listing lines (step_cache_listing) with each path under
# the root $1 written relative to it, and each `missing` line of an absolute
# path outside it left out.
step_cache_relative() {
    local root="$1"
    local line path
    while IFS= read -r line; do
        if [[ "$line" =~ ^([0-9a-f]{64}\ [\ *])(.*)$ ]]; then
            path="${BASH_REMATCH[2]}"
            line="${BASH_REMATCH[1]}${path#"$root"/}"
        elif [[ "$line" == "directory "* ]]; then
            path="${line#directory }"
            line="directory ${path#"$root"/}"
        elif [[ "$line" == "missing "* ]]; then
            path="${line#missing }"
            if [[ "$path" == /* && "$path" != "$root"/* ]]; then
                continue
            fi
            line="missing ${path#"$root"/}"
        fi
        printf '%s\n' "$line"
    done
}

# Prints the key of the step $1 with the inputs before `--` and the command
# after it. With `--shared <functions>` first, the shared key: the shared
# base, the text of the functions (space-separated names) that wrap the
# step, and paths under the worktree's root relative to it, in the inputs
# and in the command's arguments.
step_cache_key() {
    local shared="" functions=""
    if [ "${1:-}" = "--shared" ]; then
        shared=1
        functions="$2"
        shift 2
    fi
    local description="$1"
    shift
    local inputs=()
    while [ "$#" -gt 0 ] && [ "$1" != "--" ]; do
        inputs+=("$1")
        shift
    done
    [ "$#" -gt 0 ] && shift
    if [[ ! $shared ]]; then
        {
            printf 'base %s\n' "$STEP_CACHE_BASE"
            printf 'step %s\n' "$description"
            printf 'command'
            printf ' %q' "$@"
            printf '\n'
            printf 'inputs %s\n' "$(step_cache_inputs ${inputs[@]+"${inputs[@]}"})"
        } | sha256sum | cut -d' ' -f1
        return
    fi
    local argument function_names
    read -ra function_names <<<"$functions"
    {
        printf 'shared base %s\n' "$STEP_CACHE_SHARED_BASE"
        printf 'step %s\n' "$description"
        printf 'command'
        for argument in "$@"; do
            if [[ "$argument" == "$STEP_CACHE_ROOT"/* ]]; then
                argument="./${argument#"$STEP_CACHE_ROOT"/}"
            fi
            printf ' %q' "$argument"
        done
        printf '\n'
        printf 'functions %s\n' "$(step_cache_functions ${function_names[@]+"${function_names[@]}"} \
            | sha256sum | cut -d' ' -f1)"
        printf 'inputs %s\n' "$(step_cache_inputs --relative "$STEP_CACHE_ROOT" ${inputs[@]+"${inputs[@]}"})"
    } | sha256sum | cut -d' ' -f1
}

# The record file of the step $1 (after `--shared`, in the shared cache).
step_cache_file() {
    local dir="$STEP_CACHE_DIR"
    if [ "$1" = "--shared" ]; then
        dir="$STEP_CACHE_SHARED_DIR"
        shift
    fi
    printf "%s/%s" "$dir" "$(printf "%s" "$1" | sha256sum | cut -d' ' -f1)"
}

# Returns 0 and prints the recorded label when the step $1 last passed with
# the key $2; 1 otherwise. `--shared` first: in the shared cache.
step_cache_hit() {
    local scope=()
    if [ "$1" = "--shared" ]; then
        scope=(--shared)
        shift
    fi
    local file key label
    file=$(step_cache_file ${scope[@]+"${scope[@]}"} "$1")
    [ -f "$file" ] || return 1
    { IFS= read -r key && IFS= read -r label; } <"$file" || return 1
    [ "$key" = "$2" ] || return 1
    printf "%s" "$label"
}

# Records that the step $1 passed with the key $2. `--shared` first: in the
# shared cache, its label followed by the date and time; written to a file
# of its own, then moved, as another worktree may record the same step.
step_cache_record() {
    local file label tmp
    if [ "$1" = "--shared" ]; then
        shift
        mkdir -p "$STEP_CACHE_SHARED_DIR" || return 1
        file=$(step_cache_file --shared "$1")
        label="$STEP_CACHE_SHARED_LABEL, $(date '+%Y-%m-%d %H:%M')"
        tmp=$(mktemp "$file.XXXXXX") || return 1
        printf "%s\n%s\n%s\n" "$2" "$label" "$1" >"$tmp" && mv "$tmp" "$file"
        return
    fi
    file=$(step_cache_file "$1")
    printf "%s\n%s\n%s\n" "$2" "$STEP_CACHE_LABEL" "$1" >"$file.tmp" && mv "$file.tmp" "$file"
}

# The self-test: a synthetic step over a synthetic tree in a temporary
# directory, then a shared and a worktree step over two worktrees of a
# synthetic repository.
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
        else
            printf "  ok  %s: %s\n" "$1" "$got"
        fi
    }
    printf "step cache self-test, a worktree step:\n"
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
    step_cache_self_test_shared "$dir" || failures=$((failures + 1))
    rm -rf "$dir"
    if [ "$failures" -ne 0 ]; then
        return 1
    fi
    printf "step cache self-test: every case passed\n"
}

# The shared cache's part of the self-test, in the directory $1: a
# repository with two worktrees, `one` and `two`, each holding the same
# synthetic tree and a calling script that defines the step's wrapping
# function; the same step run shared and per worktree. Returns 1 on a
# failed case.
step_cache_self_test_shared() {
    local dir="$1" failures=0 tools=1 root shared_dir
    local git_test=(git -c user.name=step-cache -c user.email=step-cache@invalid -c commit.gpgsign=false
        -c core.hooksPath=/dev/null)
    "${git_test[@]}" init -q "$dir/one" || return 1
    "${git_test[@]}" -C "$dir/one" commit -q --allow-empty -m "step cache self-test" || return 1
    "${git_test[@]}" -C "$dir/one" worktree add -q --detach "$dir/two" || return 1
    for root in "$dir/one" "$dir/two"; do
        mkdir -p "$root/inputs/sub"
        printf 'a\n' >"$root/file.txt"
        printf 'b\n' >"$root/inputs/sub/nested.txt"
        printf 'upper\n' >"$root/inputs/B.txt"
        printf 'lower\n' >"$root/inputs/a.txt"
        printf 'c\n' >"$root/one.glob"
        printf '#!/bin/bash\nstep_cache_self_test_wrapper() { "$@"; }\n' >"$root/script.sh"
    done
    shared_dir=$(step_cache_shared_dir "$dir/one") || return 1
    if [ "$shared_dir" != "$(step_cache_shared_dir "$dir/two")" ] || [[ "$shared_dir" != "$dir/one/.git/"* ]]; then
        printf "step cache self-test: the worktrees' shared directories differ or lie outside the common git directory: %s, %s\n" \
            "$shared_dir" "$(step_cache_shared_dir "$dir/two")" >&2
        return 1
    fi
    # One run of the step in the worktree $2 ("one" or "two"), shared or per
    # worktree ($1 "shared" or "worktree"), as check_release.sh runs it from
    # the worktree's root, its script sourced; $3 = what the command exits
    # with, $4... more arguments of it. Prints "skipped" or "ran", and the
    # label of the record it found.
    run_shared() {
        local scope="$1" name="$2" root="$dir/$2" status="${3:-0}" key label
        shift 3
        cd "$root" || return 1
        # shellcheck source=/dev/null  # the synthetic calling script
        source "$root/script.sh"
        # Cargo's configuration files of the root's ancestors stand for the
        # missing paths outside it, which differ between the worktrees.
        local step_inputs=("$root/file.txt" "inputs" "$root/*.glob" "$dir/absent-$name/config.toml")
        if [ "$scope" = "shared" ]; then
            step_cache_init_shared "$shared_dir" "$root" "test-commit in $name" \
                "$(printf "tools %s\n%s" "$tools" "$(step_cache_verdict_environment)")"
            key=$(step_cache_key --shared step_cache_self_test_wrapper "a step" "${step_inputs[@]}" \
                -- step_cache_self_test_wrapper true "$@")
            if label=$(step_cache_hit --shared "a step" "$key"); then
                printf "skipped %s" "$label"
                return
            fi
            printf "ran"
            if [ "$status" -eq 0 ]; then
                step_cache_record --shared "a step" "$key"
            fi
            return
        fi
        step_cache_init "$root/worktree-cache" "test-commit" \
            "$(printf "tools %s\n%s\n%s" "$tools" "$(sha256sum <"$root/script.sh")" "$(step_cache_environment)")"
        key=$(step_cache_key "a step" "${step_inputs[@]}" -- step_cache_self_test_wrapper true "$@")
        if label=$(step_cache_hit "a step" "$key"); then
            printf "skipped %s" "$label"
            return
        fi
        printf "ran"
        if [ "$status" -eq 0 ]; then
            step_cache_record "a step" "$key"
        fi
    }
    # $1 = the case, $2 = the worktree, $3 = what the shared step does, $4 =
    # what the worktree step does ("ran" or "skipped"), $5... = run_shared's
    # status and arguments.
    expect_both() {
        local scope expected got
        for scope in shared worktree; do
            expected="$3"
            [ "$scope" = "worktree" ] && expected="$4"
            got=$(run_shared "$scope" "$2" "${@:5}")
            if [ "${got%% *}" != "$expected" ]; then
                printf "step cache self-test: %s (%s step, worktree %s): expected %s, got %s\n" \
                    "$1" "$scope" "$2" "$expected" "$got" >&2
                failures=$((failures + 1))
            else
                printf "  ok  %s (%s step, worktree %s): %s\n" "$1" "$scope" "$2" "$got"
            fi
        done
    }
    # $1 = the variable, $2 = its value or nothing to unset it.
    set_variable() {
        if [ "$#" -gt 1 ]; then
            export "$1=$2"
        else
            unset "$1"
        fi
    }
    local rustflags=("RUSTFLAGS") locale=("LC_ALL")
    [[ -n "${RUSTFLAGS+set}" ]] && rustflags+=("$RUSTFLAGS")
    [[ -n "${LC_ALL+set}" ]] && locale+=("$LC_ALL")
    printf "step cache self-test, a shared step and a worktree step:\n"
    expect_both "first run" one ran ran 0
    expect_both "unchanged" one skipped skipped 0
    printf '# an unrelated edit\n' >>"$dir/one/script.sh"
    expect_both "(a) an unrelated edit of the calling script" one skipped ran 0
    printf '# an unrelated edit\n' >>"$dir/two/script.sh"
    expect_both "(b) another worktree (another PWD), the same content" two skipped ran 0
    local found
    found=$(run_shared shared two 0)
    if [[ "$found" != "skipped test-commit in one, "* ]]; then
        printf "step cache self-test: the shared record found from two is not the one written in one: %s\n" "$found" >&2
        failures=$((failures + 1))
    else
        printf "  ok  the shared record found from two names one: %s\n" "$found"
    fi
    # The negative control: the shared step in two, its cache in a directory
    # of two's own, does not find one's record, so (b) found it through the
    # common git directory alone.
    local common_dir="$shared_dir"
    shared_dir="$dir/two/own-cache"
    found=$(run_shared shared two 0)
    shared_dir="$common_dir"
    if [ "$found" != "ran" ]; then
        printf "step cache self-test: negative control: a cache of two's own found a record: %s\n" "$found" >&2
        failures=$((failures + 1))
    else
        printf "  ok  negative control: from a cache of two's own, the step ran\n"
    fi
    export NT_FACTS_GATE_THREADS=3 CARGO_BUILD_JOBS=2 NEXTEST_TEST_THREADS=2 RUST_TEST_THREADS=2
    expect_both "(c) another thread count" one skipped ran 0
    unset NT_FACTS_GATE_THREADS CARGO_BUILD_JOBS NEXTEST_TEST_THREADS RUST_TEST_THREADS
    export STEP_CACHE_SELF_TEST_VARIABLE=1
    expect_both "(d) an unrelated new variable" one skipped ran 0
    unset STEP_CACHE_SELF_TEST_VARIABLE
    export LC_ALL=C
    expect_both "another locale (C)" one skipped ran 0
    export LC_ALL=en_US.UTF-8
    expect_both "another locale (en_US.UTF-8, B.txt and a.txt sorted otherwise)" one skipped ran 0
    set_variable "${locale[@]}"
    export NATIVE_THEME_EVIDENCE="$dir/evidence-elsewhere" NATIVE_THEME_SOURCES="$dir/sources-elsewhere"
    expect_both "another evidence root and source cache path (the pin unchanged)" one skipped ran 0
    unset NATIVE_THEME_EVIDENCE NATIVE_THEME_SOURCES
    mkdir -p "$dir/merge-a" "$dir/merge-b"
    printf 'listed\n' >"$dir/merge-a/listed-1-of-2.tsv"
    printf 'listed\n' >"$dir/merge-b/listed-1-of-2.tsv"
    export NT_FACTS_GATE_MERGE="$dir/merge-a"
    expect_both "NT_FACTS_GATE_MERGE set" one ran ran 0
    export NT_FACTS_GATE_MERGE="$dir/merge-b"
    expect_both "NT_FACTS_GATE_MERGE at another path, the same content" one skipped ran 0
    printf 'listed 2\n' >"$dir/merge-b/listed-1-of-2.tsv"
    # The worktree step's base holds the variable's text, as it always has.
    expect_both "NT_FACTS_GATE_MERGE's content edited" one ran skipped 0
    unset NT_FACTS_GATE_MERGE
    expect_both "NT_FACTS_GATE_MERGE unset (the record holds the last pass)" one ran ran 0
    expect_both "unchanged" one skipped skipped 0
    printf 'a2\n' >"$dir/one/file.txt"
    expect_both "(e) a declared input edited" one ran ran 0
    printf 'b2\n' >"$dir/one/inputs/sub/nested.txt"
    expect_both "(e) a declared directory's file edited" one ran ran 0
    export RUSTFLAGS="${RUSTFLAGS:-} --cfg step_cache_self_test"
    expect_both "(f) RUSTFLAGS changed" one ran ran 0
    set_variable "${rustflags[@]}"
    expect_both "(f) RUSTFLAGS back (the record holds the last pass)" one ran ran 0
    export FACTS_GATE_RELEASE=1
    expect_both "(f) FACTS_GATE_RELEASE set" one ran ran 0
    unset FACTS_GATE_RELEASE
    expect_both "(f) FACTS_GATE_RELEASE unset" one ran ran 0
    expect_both "(g) the command changed" one ran ran 0 --flag
    expect_both "(g) the command back" one ran ran 0
    printf '#!/bin/bash\nstep_cache_self_test_wrapper() { "$@" || return; }\n' >"$dir/one/script.sh"
    expect_both "(h) the wrapping function's text changed" one ran ran 0
    tools=2
    expect_both "(i) a tool version changed" one ran ran 0
    expect_both "unchanged" one skipped skipped 0
    printf 'a3\n' >"$dir/one/file.txt"
    expect_both "failed" one ran ran 1
    expect_both "a failure recorded nothing" one ran ran 1
    expect_both "passed" one ran ran 0
    expect_both "after the pass" one skipped skipped 0
    [ "$failures" -eq 0 ]
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
