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
# records are the worktree's own (check_release.sh: target/check-release-cache),
# one per step.
#
# A shared step (check_release.sh's CACHE_SHARED: the facts gate's parts) is
# keyed narrowly instead, so that its result is reused wherever what decides
# its verdict is the same, in any worktree of the repository:
#   * the shared base, which step_cache_start_shared builds for every caller:
#     the caller's fixed text (check_release.sh: the tool versions as above
#     and the settings that decide a step's inputs), the text of the
#     caller's functions it names (check_release.sh: those that key a step,
#     look it up and record it) and of this file's key, lookup and record
#     functions. The scripts' hashes and the environment are not in it, so
#     an unrelated edit of a script, another working directory or an
#     unrelated variable keeps the record;
#   * the environment variables that can change a verdict, read for each key
#     (step_cache_verdict_environment, whose comment lists each rule and its
#     reason);
#   * the step's description and command, as above;
#   * the text of the functions that wrap it between the caller's call and
#     the command's start (`declare -f`), each named where it is declared; a
#     named function that is not defined is an error, the step not keyed;
#   * its declared inputs' content, as above, each path under the worktree's
#     root written relative to it, and a missing path outside the root left
#     out (Cargo's configuration files of the root's ancestors: which of them
#     are looked for depends on where the worktree is, not on what it holds;
#     one that exists is hashed by path and content).
# Its records live in the repository's common git directory
# (step_cache_shared_dir: <git-common-dir>/check-release-cache), shared by
# every worktree: one file per step and key, so each worktree's passed state
# keeps its own record, naming the commit and worktree it passed in and
# when. A record found is touched; one not written or found for 30 days is
# removed when the cache starts. Where the folder cannot be created or
# written, the shared cache is off (step_cache_start_shared says why) and
# the shared steps run; a record that cannot be written returns 1, which
# check_release.sh reports as a warning.
#
# Usage (sourced): step_cache_init <dir> <label> <base text>, and for shared
# steps step_cache_start_shared <root> <label> <fixed text> <functions...>;
# then per step step_cache_key [--shared <functions>] <description>
# <inputs...> -- <command...> (prints the key), step_cache_hit [--shared]
# <description> <key> (prints the label recorded, returns 1 on a miss) and
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
# verdict variable, the command, a wrapping or caller function's text or a
# tool version changes; a step that is not shared re-runs after each of
# these. Two worktrees' records stand side by side; an undefined wrapper is
# an error; a shared folder that cannot be written turns the shared cache
# off; and git's variables of the caller (a hook's GIT_DIR) reach no
# repository the self-test makes.

STEP_CACHE_DIR=""
STEP_CACHE_LABEL=""
STEP_CACHE_BASE=""
STEP_CACHE_SHARED_DIR=""
STEP_CACHE_SHARED_LABEL=""
STEP_CACHE_SHARED_BASE=""
STEP_CACHE_SHARED_NOTE=""
STEP_CACHE_ROOT=""

# The age, in minutes, past which a shared record is removed: 30 days.
STEP_CACHE_SHARED_AGE=$((30 * 24 * 60))

# $1 = the cache directory, $2 = the label a record states ("<commit>"),
# $3 = the base text (tool versions, scripts, environment).
step_cache_init() {
    STEP_CACHE_DIR="$1"
    STEP_CACHE_LABEL="$2"
    STEP_CACHE_BASE=$(printf "%s" "$3" | sha256sum | cut -d' ' -f1)
    mkdir -p "$STEP_CACHE_DIR"
}

# Starts the shared steps' cache of the worktree whose root is $1: $2 = the
# label a record states ("<commit> in <worktree>"; the record adds when),
# $3 = the caller's fixed text (tool versions, settings), $4... = the
# caller's functions whose text the shared base holds. Builds the shared
# base (the fixed text, those functions' text and this file's key, lookup
# and record functions'), creates the folder, checks that it can be
# written and removes the records older than STEP_CACHE_SHARED_AGE.
# Returns 1, the reason in STEP_CACHE_SHARED_NOTE, where there is no git
# directory, the folder cannot be written or a function named is not
# defined.
step_cache_start_shared() {
    local root="$1" label="$2" fixed="$3"
    shift 3
    local dir probe functions
    STEP_CACHE_SHARED_DIR=""
    STEP_CACHE_SHARED_NOTE=""
    if ! dir=$(step_cache_shared_dir "$root"); then
        STEP_CACHE_SHARED_NOTE="no git directory found for $root"
        return 1
    fi
    if ! functions=$(step_cache_defined_functions "$@" 2>&1); then
        STEP_CACHE_SHARED_NOTE="$functions"
        return 1
    fi
    if ! mkdir -p "$dir" 2>/dev/null || ! probe=$(mktemp "$dir/.write-test.XXXXXX" 2>/dev/null) \
        || ! rm -f "$probe"; then
        STEP_CACHE_SHARED_NOTE="$dir cannot be written"
        return 1
    fi
    find "$dir" -maxdepth 1 -type f -mmin "+$STEP_CACHE_SHARED_AGE" -delete 2>/dev/null || true
    STEP_CACHE_SHARED_DIR="$dir"
    STEP_CACHE_ROOT="$root"
    STEP_CACHE_SHARED_LABEL="$label"
    STEP_CACHE_SHARED_BASE=$(
        {
            printf "%s\n" "$fixed"
            printf "%s\n" "$functions"
            step_cache_functions step_cache_start_shared step_cache_key step_cache_inputs \
                step_cache_listing step_cache_excluding step_cache_relative step_cache_functions \
                step_cache_defined_functions step_cache_verdict_environment step_cache_file \
                step_cache_hit step_cache_record
        } | sha256sum | cut -d' ' -f1
    )
}

# git, its arguments $@, with every GIT_* variable of the environment
# removed: a caller's GIT_DIR (a git hook's) would point it at another
# repository.
step_cache_git() {
    local name
    local unset_arguments=()
    while IFS= read -r name; do
        unset_arguments+=(-u "$name")
    done < <(compgen -e | grep '^GIT_' || true)
    env ${unset_arguments[@]+"${unset_arguments[@]}"} git "$@"
}

# The shared steps' cache directory of the worktree whose root is $1: the
# folder check-release-cache in the repository's common git directory, the
# same from every worktree.
step_cache_shared_dir() {
    local common
    common=$(cd "$1" && step_cache_git rev-parse --git-common-dir 2>/dev/null) || return 1
    [ -n "$common" ] || return 1
    common=$(cd "$1" && cd "$common" && pwd) || return 1
    printf "%s/check-release-cache" "$common"
}

# The environment as part of a base: every variable but those bash sets per
# process, sorted, NUL-separated entries hashed.
step_cache_environment() {
    env -0 | grep -zvE '^(_|SHLVL|OLDPWD)=' | LC_ALL=C sort -z | sha256sum | cut -d' ' -f1
}

# The environment as part of a shared key: the variables that can change a
# verdict of the facts gate or of its build, sorted, NUL-separated entries
# hashed. In:
#   * NT_* (the gate's NT_FACTS_GATE_PRESETS and NT_FACTS_GATE_SHARD select
#     what it checks, NT_FACTS_GATE_MERGE makes it check shards' listed
#     paths instead; check_release.sh unsets all three and
#     NT_FACTS_GATE_LISTED; the project's other NT_ variables, current and
#     future, are in too) and FACTS_GATE_RELEASE (the release's stricter
#     gate);
#   * NATIVE_THEME_* (the project's own; the test fixtures' root among them);
#   * CARGO_* (Cargo's configuration: profiles, CARGO_TARGET_<triple>_*,
#     rustflags, the target, unstable flags, aliases, registries) and the
#     CARGO_CFG_*/CARGO_FEATURE_* build scripts read;
#   * RUSTFLAGS, RUSTDOCFLAGS, RUSTC, RUSTC_* (RUSTC_WRAPPER,
#     RUSTC_WORKSPACE_WRAPPER, RUSTC_BOOTSTRAP: a wrapper is a program run in
#     rustc's place), RUSTDOC, RUSTDOC_*, RUSTFMT, RUSTUP_* (the toolchain
#     chosen) and RUST_* (RUST_MIN_STACK decides whether a test overflows its
#     stack; RUST_TEST_* other than those below);
#   * NEXTEST_* (a test step run by nextest: its profile, its retries);
#   * GIT_* (GIT_DIR, GIT_WORK_TREE, GIT_CEILING_DIRECTORIES, GIT_CONFIG_*:
#     where the git the gate runs on the evidence and the source cache
#     looks).
# By their form alone, the thread and job counts, which set how fast it
# runs: NT_FACTS_GATE_THREADS, RUST_TEST_THREADS, CARGO_BUILD_JOBS and
# NEXTEST_TEST_THREADS. A plain positive number of up to nine digits, which
# each of the gate, libtest, Cargo and nextest takes, is left out like the
# variable unset (the default count); any other value, which one of them
# may refuse, is keyed by its text.
# Out, each named below:
#   * how fast it runs: CARGO_INCREMENTAL, CARGO_CACHE_RUSTC_INFO;
#   * what it prints: NT_FACTS_GATE_TIMING, CARGO_TERM_*, CARGO_LOG,
#     RUST_BACKTRACE, RUST_LIB_BACKTRACE, RUST_LOG, RUSTC_LOG,
#     RUST_TEST_NOCAPTURE, NEXTEST's status, output and progress settings;
#   * where it builds: CARGO_HOME (its configuration files are inputs, by
#     path and content), CARGO_TARGET_DIR, CARGO_BUILD_TARGET_DIR,
#     CARGO_BUILD_BUILD_DIR;
#   * where it writes, never reads: NT_FACTS_GATE_LISTED (a shard's listed
#     paths; check_release.sh unsets it and runs no shard);
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
            NT_FACTS_GATE_THREADS | RUST_TEST_THREADS | CARGO_BUILD_JOBS | NEXTEST_TEST_THREADS)
                if [[ "${entry#*=}" =~ ^[1-9][0-9]{0,8}$ ]]; then
                    continue
                fi
                ;;
            NT_FACTS_GATE_TIMING | NT_FACTS_GATE_LISTED) continue ;;
            NATIVE_THEME_EVIDENCE | NATIVE_THEME_SOURCES) continue ;;
            CARGO_INCREMENTAL | CARGO_CACHE_RUSTC_INFO | CARGO_TERM_* | CARGO_LOG | \
                CARGO_HOME | CARGO_TARGET_DIR | CARGO_BUILD_TARGET_DIR | CARGO_BUILD_BUILD_DIR) continue ;;
            RUST_BACKTRACE | RUST_LIB_BACKTRACE | RUST_LOG | RUSTC_LOG | RUST_TEST_NOCAPTURE) continue ;;
            NEXTEST_STATUS_LEVEL | NEXTEST_FINAL_STATUS_LEVEL | NEXTEST_FAILURE_OUTPUT | \
                NEXTEST_SUCCESS_OUTPUT | NEXTEST_HIDE_PROGRESS_BAR | NEXTEST_SHOW_PROGRESS | \
                NEXTEST_NO_INPUT_HANDLER) continue ;;
            GIT_EDITOR | GIT_SEQUENCE_EDITOR | GIT_PAGER | GIT_ASKPASS | GIT_TERMINAL_PROMPT | \
                GIT_TRACE* | GIT_AUTHOR_* | GIT_COMMITTER_* | GIT_SSH*) continue ;;
            NT_* | FACTS_GATE_RELEASE | NATIVE_THEME_* | CARGO_* | RUSTFLAGS | RUSTDOCFLAGS | RUSTC | \
                RUSTC_* | RUSTDOC | RUSTDOC_* | RUSTFMT | RUSTUP_* | RUST_* | NEXTEST_* | GIT_*) ;;
            *) continue ;;
        esac
        printf '%s\0' "$entry"
    done < <(env -0) | LC_ALL=C sort -z | sha256sum | cut -d' ' -f1
}

# The text of the shell functions $@, each by `declare -f`; a function not
# defined by name. For this file's own functions, which are always defined.
step_cache_functions() {
    local name
    for name in "$@"; do
        declare -f "$name" || printf "no function %s\n" "$name"
    done
}

# The text of the shell functions $@, each by `declare -f`; returns 1, an
# error on standard error, where one is not defined.
step_cache_defined_functions() {
    local name
    for name in "$@"; do
        if ! declare -f "$name"; then
            printf "step cache: no function %s, which a shared step's key names\n" "$name" >&2
            return 1
        fi
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
# base, the verdict's variables (step_cache_verdict_environment), the text
# of the functions (space-separated names) that wrap the step, and paths
# under the worktree's root relative to it, in the inputs and in the
# command's arguments; returns 1, printing nothing, where one of those
# functions is not defined.
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
    local argument function_names function_text
    read -ra function_names <<<"$functions"
    function_text=$(step_cache_defined_functions ${function_names[@]+"${function_names[@]}"}) || return 1
    {
        printf 'shared base %s\n' "$STEP_CACHE_SHARED_BASE"
        printf 'environment %s\n' "$(step_cache_verdict_environment)"
        printf 'step %s\n' "$description"
        printf 'command'
        for argument in "$@"; do
            if [[ "$argument" == "$STEP_CACHE_ROOT"/* ]]; then
                argument="./${argument#"$STEP_CACHE_ROOT"/}"
            fi
            printf ' %q' "$argument"
        done
        printf '\n'
        printf 'functions %s\n' "$(printf "%s" "$function_text" | sha256sum | cut -d' ' -f1)"
        printf 'inputs %s\n' "$(step_cache_inputs --relative "$STEP_CACHE_ROOT" ${inputs[@]+"${inputs[@]}"})"
    } | sha256sum | cut -d' ' -f1
}

# The record file of the step $1. `--shared <key>` first: in the shared
# cache, named by the step and the key, one file for each key passed.
step_cache_file() {
    if [ "$1" = "--shared" ]; then
        printf "%s/%s" "$STEP_CACHE_SHARED_DIR" "$(printf "%s\n%s" "$3" "$2" | sha256sum | cut -d' ' -f1)"
        return
    fi
    printf "%s/%s" "$STEP_CACHE_DIR" "$(printf "%s" "$1" | sha256sum | cut -d' ' -f1)"
}

# Returns 0 and prints the recorded label when the step $1 last passed with
# the key $2; 1 otherwise. `--shared` first: in the shared cache, the record
# of that key, touched so that its age counts from this use.
step_cache_hit() {
    local file key label shared=""
    if [ "$1" = "--shared" ]; then
        shared=1
        file=$(step_cache_file --shared "$3" "$2")
        shift
    else
        file=$(step_cache_file "$1")
    fi
    [ -f "$file" ] || return 1
    { IFS= read -r key && IFS= read -r label; } <"$file" || return 1
    [ "$key" = "$2" ] || return 1
    if [[ $shared ]]; then
        touch -- "$file" 2>/dev/null || true
    fi
    printf "%s" "$label"
}

# Records that the step $1 passed with the key $2; returns 1 where the
# record cannot be written. `--shared` first: in the shared cache, its label
# followed by the date and time; written to a file of its own, then moved,
# as another worktree may record the same step.
step_cache_record() {
    local file label tmp
    if [ "$1" = "--shared" ]; then
        shift
        file=$(step_cache_file --shared "$2" "$1")
        label="$STEP_CACHE_SHARED_LABEL, $(date '+%Y-%m-%d %H:%M')"
        tmp=$(mktemp "$file.XXXXXX" 2>/dev/null) || return 1
        if ! printf "%s\n%s\n%s\n" "$2" "$label" "$1" >"$tmp" || ! mv "$tmp" "$file"; then
            rm -f "$tmp"
            return 1
        fi
        return 0
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
    chmod -R u+w "$dir" 2>/dev/null || true
    rm -rf "$dir"
    if [ "$failures" -ne 0 ]; then
        return 1
    fi
    printf "step cache self-test: every case passed\n"
}

# The shared cache's part of the self-test, in the directory $1: a
# repository with two worktrees, `one` and `two`, each holding the same
# synthetic tree and a calling script that defines the step's wrapping
# function and a function the shared base holds; the same step run shared
# and per worktree. The whole part runs with GIT_DIR and GIT_WORK_TREE
# naming a repository `outer`, as in a git hook, which must stay untouched.
# Returns 1 on a failed case.
step_cache_self_test_shared() {
    local dir="$1" failures=0 tools=1 root shared_dir outer_head
    local git_test=(step_cache_git -c user.name=step-cache -c user.email=step-cache@invalid
        -c commit.gpgsign=false -c core.hooksPath=/dev/null)
    "${git_test[@]}" init -q "$dir/outer" || return 1
    "${git_test[@]}" -C "$dir/outer" commit -q --allow-empty -m "outer" || return 1
    outer_head=$("${git_test[@]}" -C "$dir/outer" rev-parse HEAD) || return 1
    export GIT_DIR="$dir/outer/.git" GIT_WORK_TREE="$dir/outer"
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
        printf '#!/bin/bash\nstep_cache_self_test_wrapper() { "$@"; }\nstep_cache_self_test_keyer() { :; }\n' \
            >"$root/script.sh"
    done
    shared_dir=$(step_cache_shared_dir "$dir/one") || return 1
    if [ "$shared_dir" != "$(step_cache_shared_dir "$dir/two")" ] || [[ "$shared_dir" != "$dir/one/.git/"* ]]; then
        printf "step cache self-test: the worktrees' shared directories differ or lie outside the common git directory: %s, %s\n" \
            "$shared_dir" "$(step_cache_shared_dir "$dir/two")" >&2
        return 1
    fi
    # The worktree whose own cache the worktree step uses (by default its
    # own: check_release.sh's target/check-release-cache), and a folder the
    # shared step uses in place of the common git directory's (by default
    # none).
    local cache_of="" shared_of=""
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
            if ! step_cache_start_shared "$root" "test-commit in $name" "tools $tools" step_cache_self_test_keyer; then
                printf "uncached %s" "$STEP_CACHE_SHARED_NOTE"
                return
            fi
            if [ -n "$shared_of" ]; then
                STEP_CACHE_SHARED_DIR="$shared_of"
                mkdir -p "$shared_of"
            fi
            key=$(step_cache_key --shared step_cache_self_test_wrapper "a step" "${step_inputs[@]}" \
                -- step_cache_self_test_wrapper true "$@") || return 1
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
        step_cache_init "$dir/${cache_of:-$name}/worktree-cache" "test-commit" \
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
    # $1 = the case, $2 = what it found, $3 = what was expected (a prefix).
    check() {
        if [[ "$2" != "$3"* ]]; then
            printf "step cache self-test: %s: expected %s, got %s\n" "$1" "$3" "$2" >&2
            failures=$((failures + 1))
        else
            printf "  ok  %s: %s\n" "$1" "$2"
        fi
    }
    # $1 = the case, $2 = the worktree, $3 = what the shared step does, $4 =
    # what the worktree step does ("ran" or "skipped"), $5... = run_shared's
    # status and arguments.
    expect_both() {
        check "$1 (shared step, worktree $2)" "$(run_shared shared "$2" "${@:5}")" "$3"
        check "$1 (worktree step, worktree $2)" "$(run_shared worktree "$2" "${@:5}")" "$4"
    }
    # $1 = the variable, $2 = its value or nothing to unset it.
    set_variable() {
        if [ "$#" -gt 1 ]; then
            export "$1=$2"
        else
            unset "$1"
        fi
    }
    local rustflags=("RUSTFLAGS") locale=("LC_ALL") found
    [[ -n "${RUSTFLAGS+set}" ]] && rustflags+=("$RUSTFLAGS")
    [[ -n "${LC_ALL+set}" ]] && locale+=("$LC_ALL")
    printf "step cache self-test, a shared step and a worktree step:\n"
    expect_both "first run" one ran ran 0
    expect_both "unchanged" one skipped skipped 0
    printf '# an unrelated edit\n' >>"$dir/one/script.sh"
    expect_both "(a) an unrelated edit of the calling script" one skipped ran 0
    printf '# an unrelated edit\n' >>"$dir/two/script.sh"
    check "(b) another worktree (another PWD), the same content (shared step, worktree two)" \
        "$(run_shared shared two 0)" "skipped test-commit in one, "
    # The worktree step in two looks in one's cache, which holds one's
    # record: only its key (the working directory, the absolute paths) can
    # make it run.
    cache_of=one
    check "(b) another worktree (another PWD), the same content (worktree step, worktree two, one's cache)" \
        "$(run_shared worktree two 0)" "ran"
    cache_of=""
    # The negative control: the shared step in two, its cache in a folder
    # of two's own, does not find one's record, so (b) found it through the
    # common git directory alone.
    shared_of="$dir/two/own-cache"
    check "negative control: from a cache of two's own (shared step, worktree two)" \
        "$(run_shared shared two 0)" "ran"
    shared_of=""
    export NT_FACTS_GATE_THREADS=3 CARGO_BUILD_JOBS=2 NEXTEST_TEST_THREADS=2 RUST_TEST_THREADS=2
    expect_both "(c) another thread count" one skipped ran 0
    export NT_FACTS_GATE_THREADS=0
    expect_both "(c) a thread count the gate refuses (0)" one ran ran 0
    unset NT_FACTS_GATE_THREADS CARGO_BUILD_JOBS NEXTEST_TEST_THREADS RUST_TEST_THREADS
    export STEP_CACHE_SELF_TEST_VARIABLE=1
    expect_both "(d) an unrelated new variable" one skipped ran 0
    unset STEP_CACHE_SELF_TEST_VARIABLE
    export LC_ALL=C
    expect_both "another locale (C)" one skipped ran 0
    if locale -a 2>/dev/null | grep -qixE 'en_US\.utf-?8'; then
        export LC_ALL=en_US.UTF-8
        expect_both "another locale (en_US.UTF-8, B.txt and a.txt sorted otherwise)" one skipped ran 0
    else
        printf "  skip  another locale (en_US.UTF-8): not installed (locale -a)\n"
    fi
    set_variable "${locale[@]}"
    export NATIVE_THEME_EVIDENCE="$dir/evidence-elsewhere" NATIVE_THEME_SOURCES="$dir/sources-elsewhere"
    expect_both "another evidence root and source cache path (the pin unchanged)" one skipped ran 0
    unset NATIVE_THEME_EVIDENCE NATIVE_THEME_SOURCES
    export NT_FACTS_GATE_MERGE="$dir/merge"
    expect_both "NT_FACTS_GATE_MERGE set (keyed by its text)" one ran ran 0
    unset NT_FACTS_GATE_MERGE
    expect_both "NT_FACTS_GATE_MERGE unset (each key's own record)" one skipped ran 0
    expect_both "unchanged" one skipped skipped 0
    printf 'a2\n' >"$dir/one/file.txt"
    expect_both "(e) a declared input edited" one ran ran 0
    printf 'b2\n' >"$dir/one/inputs/sub/nested.txt"
    expect_both "(e) a declared directory's file edited" one ran ran 0
    export RUSTFLAGS="${RUSTFLAGS:-} --cfg step_cache_self_test"
    expect_both "(f) RUSTFLAGS changed" one ran ran 0
    set_variable "${rustflags[@]}"
    expect_both "(f) RUSTFLAGS back (each key's own record)" one skipped ran 0
    export FACTS_GATE_RELEASE=1
    expect_both "(f) FACTS_GATE_RELEASE set" one ran ran 0
    unset FACTS_GATE_RELEASE
    expect_both "(f) FACTS_GATE_RELEASE unset" one skipped ran 0
    expect_both "(g) the command changed" one ran ran 0 --flag
    expect_both "(g) the command back" one skipped ran 0
    printf '#!/bin/bash\nstep_cache_self_test_wrapper() { "$@" || return; }\nstep_cache_self_test_keyer() { :; }\n' \
        >"$dir/one/script.sh"
    expect_both "(h) the wrapping function's text changed" one ran ran 0
    printf '#!/bin/bash\nstep_cache_self_test_wrapper() { "$@" || return; }\nstep_cache_self_test_keyer() { true; }\n' \
        >"$dir/one/script.sh"
    expect_both "(h) a function of the caller's shared base changed" one ran ran 0
    tools=2
    expect_both "(i) a tool version changed" one ran ran 0
    expect_both "unchanged" one skipped skipped 0
    printf 'a3\n' >"$dir/one/file.txt"
    expect_both "failed" one ran ran 1
    expect_both "a failure recorded nothing" one ran ran 1
    expect_both "passed" one ran ran 0
    expect_both "after the pass" one skipped skipped 0
    # Two worktrees at different content: each record stays beside the other.
    printf 'two only\n' >"$dir/two/file.txt"
    check "two worktrees' records: two at its own content (shared step)" "$(run_shared shared two 0)" "ran"
    check "two worktrees' records: one, unchanged, finds its own (shared step)" \
        "$(run_shared shared one 0)" "skipped test-commit in one, "
    check "two worktrees' records: two, unchanged, finds its own (shared step)" \
        "$(run_shared shared two 0)" "skipped test-commit in two, "
    # A wrapper that is not defined is an error: no key.
    local status=0
    found=$(cd "$dir/one" && step_cache_key --shared step_cache_self_test_undefined "a step" file.txt -- true \
        2>/dev/null) || status=$?
    check "an undefined wrapper: the key's status and output" "$status ${found:-none}" "1 none"
    # A shared folder that cannot be written: the shared cache is off, with
    # its reason; a record there fails without ending the caller.
    chmod a-w "$shared_dir"
    if [ -w "$shared_dir" ]; then
        printf "  skip  a shared folder that cannot be written: this user writes it all the same\n"
    else
        check "a shared folder that cannot be written (shared step)" "$(run_shared shared one 0)" \
            "uncached $shared_dir cannot be written"
        found=$(set -e; STEP_CACHE_SHARED_DIR="$shared_dir"; step_cache_record --shared "a step" "0" \
            || printf "returned %s, " "$?"; printf "went on")
        check "a record that cannot be written" "$found" "returned 1, went on"
    fi
    chmod u+w "$shared_dir"
    unset GIT_DIR GIT_WORK_TREE
    found="$("${git_test[@]}" -C "$dir/outer" rev-parse HEAD) $("${git_test[@]}" -C "$dir/outer" rev-list --count HEAD)"
    check "git's variables of the caller (GIT_DIR, GIT_WORK_TREE): the outer repository untouched" \
        "$found" "$outer_head 1"
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
