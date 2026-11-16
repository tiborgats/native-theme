#!/bin/bash

# The step cache of scripts/check_release.sh's --commit and --quick modes,
# sourced by it; the full run never uses it, and --no-cache turns it off.
#
# A step is skipped only when the key of its run equals the key recorded when
# it last passed. The strict key is a SHA-256 over:
#   * the base: the tool versions and environment step_cache_init is given
#     (check_release.sh passes `rustc -vV`, `cargo -vV`, clippy's, rustdoc's,
#     rustfmt's, Python's and nextest's versions, the files of the gate's
#     scripts themselves, and the environment but `_`, `SHLVL` and `OLDPWD`,
#     which bash sets per process);
#   * the step's description and its command, each argument;
#   * the content of its declared inputs: each file's path and bytes, every
#     file under a declared directory, a glob's matches, and a missing input
#     by its name, so an input added, removed, renamed or edited changes it.
#     A directory's `__pycache__` folders are left out: Python writes them
#     while a check runs, from the sources beside them, which are hashed.
#     Paths are sorted bytewise (LC_ALL=C), whatever the locale.
# A failed step records nothing; a step whose inputs change while it runs
# records nothing either. A step that declares no inputs always runs. Its
# records are the worktree's own (check_release.sh: target/check-release-cache),
# one per step.
#
# A cargo step's inputs (`@rust:<crates>`, cache_inputs) are what it reads
# besides its command: Cargo's and the tools' configuration
# (rust_config_inputs) and the crates' reach (rust_reach), the directories of
# the workspace packages `cargo tree` lists for them with every feature on,
# their dev-dependencies and the build dependencies, on this host. Registry
# packages are pinned by Cargo.lock; their build scripts and procedural
# macros run code at compile time, so a reach is refused (the step runs, the
# reason printed once per crate set) where it holds a registry package that
# probes the host (CACHE_HOST_PROBES), a registry package with a build script
# or a procedural macro that CACHE_REVIEWED_REGISTRY_CODE does not list by
# name and version (read from the full metadata, step_cache_full_metadata), a
# path package outside the workspace, or a workspace package with a build
# script or a procedural macro that CACHE_TRUSTED_BUILD_CODE does not list.
# A full metadata that cannot be read, or lists no package, keys no step.
#
# The included files: a reached package's code may include files from
# outside it (include_str!, include_bytes!, include!, #[path]), which its
# directory does not hold. step_cache_includes runs
# scripts/step_cache_includes.py on the reached packages' directories, which
# reads their code with comments and literals blanked and prints each such
# target, and the reach adds them to the inputs, with each reached package's
# target sources outside its directory (the workspace metadata's src_path);
# an include the helper cannot resolve refuses the reach (its docstring has
# the rules).
#
# The --quick key (step_cache_key --quick) is the key of a test or docs step
# the strict key does not decide, and of the two MSRV checks at the
# workspace's rust-version; --quick skips such a step where it equals the key
# recorded when the step last passed in --commit or --quick. Its records are
# a set of their own, <the cache directory>/quick/ (step_cache_init_quick).
# It is the strict key's form with another base and another reach:
#   * the base (step_cache_quick_base): the tool versions, the environment but
#     a deny-list of what cannot change a verdict (step_cache_quick_environment:
#     the thread and job counts as plain numbers, the output, location and
#     speed settings, and the variables a new terminal or session gives each
#     call), and every file git lists, tracked or untracked and not ignored,
#     outside the workspace packages' directories but those QUICK_UNREAD
#     matches, by path and content: the manifests, the lock, Cargo's
#     configuration, docs/*.toml and every script, the gate's own among them;
#   * the reach (rust_reach --quick): the strict one's, without its refusals
#     for registry code that reads the host (CACHE_HOST_PROBES,
#     CACHE_REVIEWED_REGISTRY_CODE), which --quick leaves to --commit; a path
#     package outside the workspace, an unreviewed workspace build script or
#     procedural macro and an include the helper refuses still refuse it.
# It follows nothing outside the repository but the environment and the
# tools: the host's desktop files, fonts, icon themes and libraries, the
# network and the clock can change a skipped test's verdict, which the next
# --commit checks.
#
# Every function here that pipes sees a failure inside the pipe, so that a
# file that cannot be read, or a failing find or sha256sum, fails the key
# instead of hashing a short listing: `local -` and `set -o pipefail` in the
# function, never a shell-global pipefail, which would change the caller's
# other pipes; and `|| return 1` on each command of a loop that feeds a pipe,
# since pipefail sees only a loop's last command.
#
# Usage (sourced): step_cache_init <dir> <label> <base text>, then
# step_cache_full_metadata (or step_cache_set_full_metadata <json>), and
# step_cache_init_quick <the base step_cache_quick_base <tool versions>
# prints>; per step, cache_inputs <description> <array name> (the inputs
# CACHE_INPUTS declares, returns 1 where a reach is refused), or for the
# --quick key step_cache_command_crates <command...> (its crates) and
# step_cache_quick_inputs <crates> <array name>; step_cache_key [--quick]
# <description> <inputs...> -- <command...> (prints the key), step_cache_hit
# [--quick] <description> <key> (prints the label recorded, returns 1 on a
# miss) and step_cache_record [--quick] <description> <key> (returns 1 where
# the record cannot be written). The reach's refusals are queued in
# STEP_CACHE_NOTES for the caller to print, each text once a run.
#
#   scripts/step_cache.sh --self-test
#
# runs the self-test: a step re-runs after any declared input, the base (the
# script's own hash among it), the command or the environment changes, and a
# failed step records nothing; a record that cannot be written returns 1 and
# the caller goes on; the locale does not change a key; a file that cannot be
# read fails the inputs and the key, in the gate's own shell options; the
# --quick environment follows every variable but the deny-list's, split at
# NUL; the include helper's self-test; on a synthetic workspace, an include
# outside its package is hashed in both keys, a non-literal one, a failing
# `cargo tree`, an unreviewed workspace build script and a path package
# outside the workspace refuse both reaches, a host probe and an unlisted
# registry build script the strict one alone, the --quick base follows the
# files outside the packages but QUICK_UNREAD's and fails where git cannot
# list them, the two record sets are apart, a step whose input changes while
# it runs records nothing in either, a full metadata that fails or lists no
# package keys no step, and a command's crates are its -p crates or the
# workspace's members but its --exclude ones.

STEP_CACHE_DIR=""
STEP_CACHE_LABEL=""
STEP_CACHE_BASE=""
# 1 once step_cache_init has succeeded and while the full metadata is read;
# empty otherwise, when no key is computed. STEP_CACHE_OFF_REASON says why.
STEP_CACHE_STRICT=""
STEP_CACHE_OFF_REASON=""
STEP_CACHE_FULL_METADATA=""
# The registry packages with a build script or a procedural macro, a line
# "<name> v<version>" each, from the full metadata.
STEP_CACHE_REGISTRY_BUILD_CODE=""
STEP_CACHE_NOTES=()
declare -gA STEP_CACHE_NOTED=()
# 1 once step_cache_init_quick has succeeded; empty otherwise, when no
# --quick key is computed. STEP_CACHE_QUICK_OFF_REASON says why.
STEP_CACHE_QUICK=""
STEP_CACHE_QUICK_BASE=""
STEP_CACHE_QUICK_OFF_REASON=""
# The cargo every call here runs (the self-test puts a stub in its place).
STEP_CACHE_CARGO=cargo
declare -gA RUST_REACH=()
declare -gA RUST_REACH_QUICK=()
declare -gA STEP_CACHE_TREE=()
# The files outside the workspace packages that no crate reads, which the
# --quick base leaves out: each pattern is matched against a path `git
# ls-files` prints with bash's [[ $path == $pattern ]], where `*` matches `/`
# too, so `docs/archive/*` holds every file under docs/archive/. A test or a
# build script reads none of them (searched: include*!, #[path], file reads,
# `..` and CARGO_MANIFEST_DIR joins, 2026-11-15), but docs/platform-facts.md,
# which native-theme includes: an include target stays in the key of the
# reach that includes it (step_cache_includes), whatever this list says.
QUICK_UNREAD=('*.md' 'docs/archive/*' 'docs/issues/*' 'docs/assets/*' 'docs/evidence/*' '.github/*'
    'LICENSE-*' '.gitignore' '.gitattributes')

# The repository root, from this file's place: the include helper and the
# self-test's temporary directories are found from it, wherever it is run.
STEP_CACHE_ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
STEP_CACHE_SELF="$STEP_CACHE_ROOT_DIR/scripts/$(basename "${BASH_SOURCE[0]}")"
STEP_CACHE_INCLUDES="$STEP_CACHE_ROOT_DIR/scripts/step_cache_includes.py"

# $1 = the cache directory, $2 = the label a record states ("<commit>"),
# $3 = the base text (tool versions, scripts, environment; empty where the
# caller could not compute it). Returns 1, the reason in
# STEP_CACHE_OFF_REASON, where the base is empty or cannot be hashed or the
# directory cannot be created; the directory and the label are set either
# way, for step_cache_init_quick.
step_cache_init() {
    local -
    set -o pipefail
    local base
    STEP_CACHE_STRICT=""
    STEP_CACHE_DIR="$1"
    STEP_CACHE_LABEL="$2"
    if [ -z "$3" ]; then
        STEP_CACHE_OFF_REASON="its base could not be computed"
        return 1
    fi
    if ! base=$(printf "%s" "$3" | sha256sum | cut -d' ' -f1) || [ -z "$base" ]; then
        STEP_CACHE_OFF_REASON="its base could not be hashed"
        return 1
    fi
    if ! mkdir -p "$STEP_CACHE_DIR" 2>/dev/null; then
        STEP_CACHE_OFF_REASON="$STEP_CACHE_DIR could not be created"
        return 1
    fi
    STEP_CACHE_BASE="$base"
    STEP_CACHE_STRICT=1
}

# $1 = the --quick base (step_cache_quick_base). The --quick set's records are
# in <the directory step_cache_init was given>/quick/; called after it,
# whether or not the strict set is on. Returns 1, the reason in
# STEP_CACHE_QUICK_OFF_REASON, where the base is empty, no directory was
# given or quick/ cannot be created.
step_cache_init_quick() {
    STEP_CACHE_QUICK=""
    if [ -z "$1" ]; then
        STEP_CACHE_QUICK_OFF_REASON="its base could not be computed"
        return 1
    fi
    if [ -z "$STEP_CACHE_DIR" ]; then
        STEP_CACHE_QUICK_OFF_REASON="no cache directory was given (step_cache_init)"
        return 1
    fi
    if ! mkdir -p "$STEP_CACHE_DIR/quick" 2>/dev/null; then
        STEP_CACHE_QUICK_OFF_REASON="$STEP_CACHE_DIR/quick could not be created"
        return 1
    fi
    STEP_CACHE_QUICK_BASE="$1"
    STEP_CACHE_QUICK=1
}

# Cargo, its arguments $@, as STEP_CACHE_CARGO names it.
step_cache_cargo() {
    "$STEP_CACHE_CARGO" "$@"
}

# Reads the full metadata (cargo metadata --format-version 1 --locked, with
# the dependencies) once, for the registry list (step_cache_set_full_metadata).
# Returns 1, the strict cache off for the run, where it fails.
step_cache_full_metadata() {
    local metadata
    if ! metadata=$(step_cache_cargo metadata --format-version 1 --locked 2>/dev/null); then
        STEP_CACHE_STRICT=""
        STEP_CACHE_OFF_REASON="cargo metadata --format-version 1 --locked failed"
        return 1
    fi
    step_cache_set_full_metadata "$metadata"
}

# Takes $1 as the full metadata and reads its registry packages with a build
# script or a procedural macro. Returns 1, STEP_CACHE_STRICT emptied and the
# reason in STEP_CACHE_OFF_REASON, where it lists no package or jq cannot
# read it.
step_cache_set_full_metadata() {
    local build_code
    STEP_CACHE_FULL_METADATA=""
    STEP_CACHE_REGISTRY_BUILD_CODE=""
    if ! jq -e '(.packages | length) > 0' <<<"$1" >/dev/null 2>&1; then
        STEP_CACHE_STRICT=""
        STEP_CACHE_OFF_REASON="the full metadata (cargo metadata --locked) lists no package"
        return 1
    fi
    if ! build_code=$(jq -r '.packages[] | select(.source != null)
        | select(any(.targets[]; any(.kind[]; . == "custom-build" or . == "proc-macro")))
        | "\(.name) v\(.version)"' <<<"$1"); then
        STEP_CACHE_STRICT=""
        STEP_CACHE_OFF_REASON="the full metadata's registry packages could not be read"
        return 1
    fi
    STEP_CACHE_FULL_METADATA="$1"
    STEP_CACHE_REGISTRY_BUILD_CODE="$build_code"
}

# The environment as part of a base: every variable but those bash sets per
# process, sorted, NUL-separated entries hashed.
step_cache_environment() {
    local -
    set -o pipefail
    env -0 | grep -zvE '^(_|SHLVL|OLDPWD)=' | LC_ALL=C sort -z | sha256sum | cut -d' ' -f1
}

# The environment as part of the --quick base: every variable, sorted,
# NUL-separated entries hashed, but a deny-list of what cannot change a
# verdict, so that a variable no list foresaw makes the step run:
#   * those bash sets per process: _, SHLVL, OLDPWD;
#   * the thread and job counts, while a plain positive number (keyed by
#     their text otherwise);
#   * the output settings (LIBC_BUILD_VERBOSE makes libc's build script
#     print); the build locations (CARGO_HOME's configuration files are
#     inputs, rust_config_inputs); the speed settings;
#   * the variables a new session, terminal or service gives each call, none
#     of them read by the workspace's code or a registry crate's (searched,
#     2026-11-15); CLAUDE_CODE_SESSION_ATTENDED and AI_AGENT name only who
#     runs the gate.
# The entries are split at NUL, so a value holding a newline and an `=`
# cannot read as another variable.
step_cache_quick_environment() {
    local -
    set -o pipefail
    local entry
    env -0 | while IFS= read -r -d '' entry; do
        case "${entry%%=*}" in
            RUST_TEST_THREADS | CARGO_BUILD_JOBS | NEXTEST_TEST_THREADS)
                [[ ${entry#*=} =~ ^[1-9][0-9]{0,8}$ ]] && continue ;;
            _ | SHLVL | OLDPWD) continue ;;
            CARGO_TERM_* | CARGO_LOG | RUST_BACKTRACE | RUST_LIB_BACKTRACE | RUST_LOG | RUSTC_LOG | \
                RUST_TEST_NOCAPTURE | NEXTEST_STATUS_LEVEL | NEXTEST_FINAL_STATUS_LEVEL | \
                NEXTEST_FAILURE_OUTPUT | NEXTEST_SUCCESS_OUTPUT | NEXTEST_HIDE_PROGRESS_BAR | \
                NEXTEST_SHOW_PROGRESS | NEXTEST_NO_INPUT_HANDLER | LIBC_BUILD_VERBOSE) continue ;;
            CARGO_HOME | CARGO_TARGET_DIR | CARGO_BUILD_TARGET_DIR | CARGO_BUILD_BUILD_DIR) continue ;;
            CARGO_INCREMENTAL | CARGO_CACHE_RUSTC_INFO) continue ;;
            CLAUDE_CODE_SESSION_ID | CLAUDE_PID | CLAUDE_CODE_MESSAGING_TOKEN | CLAUDE_CODE_MESSAGING_SOCKET | \
                CLAUDE_CODE_CHILD_SESSION | CLAUDE_EFFORT | WINDOWID | KONSOLE_DBUS_SESSION | \
                KONSOLE_DBUS_WINDOW | SHELL_SESSION_ID | INVOCATION_ID | JOURNAL_STREAM | SYSTEMD_EXEC_PID | \
                MANAGERPID | MANAGERPIDFDID | MEMORY_PRESSURE_WATCH | MEMORY_PRESSURE_WRITE | \
                KONSOLE_DBUS_SERVICE | KONSOLE_DBUS_ACTIVATION_COOKIE | CLAUDE_CODE_EXECPATH | \
                CLAUDE_CODE_SESSION_ATTENDED | AI_AGENT) continue ;;
        esac
        printf '%s\0' "$entry"
    done | LC_ALL=C sort -z | sha256sum | cut -d' ' -f1
}

# The --quick base: a SHA-256 over the tool versions $1, the environment
# (step_cache_quick_environment) and the content of every file `git ls-files
# -co --exclude-standard` lists outside the workspace packages' directories
# (WORKSPACE_METADATA) but those QUICK_UNREAD matches, by path and content
# (step_cache_inputs), run from the repository's root. Where git cannot list
# them (no git work tree), lists a path it has to quote (one holding a
# control character, a quote or a backslash, which no file is read by), or a
# file cannot be read, prints the reason and returns 1.
step_cache_quick_base() {
    local -
    set -o pipefail
    local listing package_dirs path dir pattern environment files_hash base
    local -a dirs=() files=()
    if ! listing=$(git -c core.quotePath=off ls-files -co --exclude-standard 2>/dev/null); then
        printf "git ls-files failed (no git work tree?)"
        return 1
    fi
    if ! package_dirs=$(jq -r --arg root "$PWD/" \
        '.packages[].manifest_path | ltrimstr($root) | rtrimstr("/Cargo.toml")' <<<"$WORKSPACE_METADATA" 2>/dev/null) \
        || [ -z "$package_dirs" ]; then
        printf "the workspace packages' directories could not be read"
        return 1
    fi
    mapfile -t dirs <<<"$package_dirs"
    if [ -n "$listing" ]; then
        while IFS= read -r path; do
            if [[ $path == \"* ]]; then
                printf "git ls-files lists a path it quotes: %s" "$path"
                return 1
            fi
            for pattern in "${QUICK_UNREAD[@]}"; do
                # shellcheck disable=SC2053  # the pattern is a glob
                [[ $path == $pattern ]] && continue 2
            done
            for dir in "${dirs[@]}"; do
                [[ $path == "$dir"/* ]] && continue 2
            done
            files+=("$path")
        done <<<"$listing"
    fi
    if ! environment=$(step_cache_quick_environment); then
        printf "the environment could not be hashed"
        return 1
    fi
    if ! files_hash=$(step_cache_inputs ${files[@]+"${files[@]}"}); then
        printf "a file outside the workspace packages could not be read"
        return 1
    fi
    if ! base=$({
        printf 'tools\n%s\n' "$1"
        printf 'environment %s\n' "$environment"
        printf 'files %s\n' "$files_hash"
    } | sha256sum | cut -d' ' -f1) || [ -z "$base" ]; then
        printf "its hash could not be computed"
        return 1
    fi
    printf "%s" "$base"
}

# The content hash of the inputs $@: a directory's files (symbolic links
# followed) by path and content, a file by path and content, a glob's
# matches, a missing input by name. Returns 1 where an input cannot be read.
step_cache_inputs() {
    local -
    set -o pipefail
    local input match matches sorted
    for input in "$@"; do
        if [[ "$input" == *[*?[]* ]]; then
            mapfile -t matches < <(compgen -G "$input")
            if [ "${#matches[@]}" -eq 0 ]; then
                printf 'missing %s\n' "$input"
            else
                sorted=$(printf '%s\n' "${matches[@]}" | LC_ALL=C sort) || return 1
                mapfile -t matches <<<"$sorted"
            fi
        else
            matches=("$input")
        fi
        for match in ${matches[@]+"${matches[@]}"}; do
            if [ -d "$match" ]; then
                printf 'directory %s\n' "$match"
                find -L "$match" -type d -name __pycache__ -prune -o -type f -print0 \
                    | LC_ALL=C sort -z | xargs -0 -r sha256sum -- || return 1
            elif [ -e "$match" ]; then
                sha256sum -- "$match" || return 1
            else
                printf 'missing %s\n' "$match"
            fi
        done
    done | sha256sum | cut -d' ' -f1
}

# Prints the key of the step $1 with the inputs before `--` and the command
# after it. `--quick` first: its --quick key, over STEP_CACHE_QUICK_BASE in
# place of the strict base. Returns 1, printing nothing, where that set is off
# or an input cannot be read.
step_cache_key() {
    local -
    set -o pipefail
    local base_line
    if [ "${1:-}" = "--quick" ]; then
        shift
        [[ $STEP_CACHE_QUICK ]] || return 1
        base_line="quick base $STEP_CACHE_QUICK_BASE"
    else
        [[ $STEP_CACHE_STRICT ]] || return 1
        base_line="base $STEP_CACHE_BASE"
    fi
    local description="$1"
    shift
    local inputs=()
    while [ "$#" -gt 0 ] && [ "$1" != "--" ]; do
        inputs+=("$1")
        shift
    done
    [ "$#" -gt 0 ] && shift
    local inputs_hash
    inputs_hash=$(step_cache_inputs ${inputs[@]+"${inputs[@]}"}) || return 1
    {
        printf '%s\n' "$base_line"
        printf 'step %s\n' "$description"
        printf 'command'
        printf ' %q' "$@"
        printf '\n'
        printf 'inputs %s\n' "$inputs_hash"
    } | sha256sum | cut -d' ' -f1
}

# The record file of the step $1; `--quick` first: in the --quick set.
step_cache_file() {
    local -
    set -o pipefail
    local name dir="$STEP_CACHE_DIR"
    if [ "$1" = "--quick" ]; then
        dir="$STEP_CACHE_DIR/quick"
        shift
    fi
    name=$(printf "%s" "$1" | sha256sum | cut -d' ' -f1) || return 1
    printf "%s/%s" "$dir" "$name"
}

# Returns 0 and prints the recorded label when the step $1 last passed with
# the key $2; 1 otherwise. `--quick` first: in the --quick set.
step_cache_hit() {
    local file key label quick=()
    if [ "$1" = "--quick" ]; then
        quick=(--quick)
        shift
    fi
    file=$(step_cache_file ${quick[@]+"${quick[@]}"} "$1") || return 1
    [ -f "$file" ] || return 1
    { IFS= read -r key && IFS= read -r label; } 2>/dev/null <"$file" || return 1
    [ "$key" = "$2" ] || return 1
    printf "%s" "$label"
}

# Records that the step $1 passed with the key $2; returns 1 where the
# record cannot be written. `--quick` first: in the --quick set.
step_cache_record() {
    local file quick=()
    if [ "$1" = "--quick" ]; then
        quick=(--quick)
        shift
    fi
    file=$(step_cache_file ${quick[@]+"${quick[@]}"} "$1") || return 1
    if ! { printf "%s\n%s\n%s\n" "$2" "$STEP_CACHE_LABEL" "$1" >"$file.tmp"; } 2>/dev/null \
        || ! mv -f "$file.tmp" "$file" 2>/dev/null; then
        rm -f "$file.tmp" 2>/dev/null
        return 1
    fi
}

# The inputs CACHE_INPUTS declares for the step $1, into the array named $2:
# its paths and globs, each `@rust:<crate>[,<crate>...]` standing for what a
# cargo step on those crates reads (rust_reach). Returns 1 where a reach is
# refused or the cache is off.
cache_inputs() {
    local -n cache_inputs_out="$2"
    local token tokens crates
    read -ra tokens <<<"${CACHE_INPUTS[$1]}"
    cache_inputs_out=()
    for token in ${tokens[@]+"${tokens[@]}"}; do
        if [[ "$token" == @rust:* ]]; then
            crates="${token#@rust:}"
            rust_reach "$crates" || return 1
            mapfile -t -O "${#cache_inputs_out[@]}" cache_inputs_out <<<"${RUST_REACH[$crates]}"
        else
            cache_inputs_out+=("$token")
        fi
    done
}

# Queues the note $1 in STEP_CACHE_NOTES, unless it was queued before in this
# run: a crate set's reason is printed once, whichever key met it first.
step_cache_note() {
    if [[ -z "${STEP_CACHE_NOTED[$1]+set}" ]]; then
        STEP_CACHE_NOTED[$1]=1
        STEP_CACHE_NOTES+=("$1")
    fi
}

# What a cargo step on the crates $1 (comma-separated) reads besides its
# command, into RUST_REACH[$1], once per run (rust_reach_resolve). Returns 1
# where the reach is refused, queuing the reason in STEP_CACHE_NOTES the
# first time, or where the cache is off: no `cargo tree` runs then.
# `--quick` first: the reach in the --quick key's form (rust_reach_resolve
# --quick), into RUST_REACH_QUICK[$1], where the --quick set is on; a crate
# set whose strict reach was accepted has the same inputs in that form.
rust_reach() {
    if [ "$1" = "--quick" ]; then
        shift
        [[ $STEP_CACHE_QUICK ]] || return 1
        if [[ -z "${RUST_REACH_QUICK[$1]+set}" ]]; then
            local quick_reach
            if [[ -n "${RUST_REACH[$1]+set}" && "${RUST_REACH[$1]}" != "!" ]]; then
                RUST_REACH_QUICK[$1]="${RUST_REACH[$1]}"
            elif ! step_cache_cargo_tree "$1"; then
                RUST_REACH_QUICK[$1]="!"
                step_cache_note "$1: not keyed, its steps run: cargo tree failed"
            elif quick_reach=$(rust_reach_resolve --quick "$1"); then
                RUST_REACH_QUICK[$1]="$quick_reach"
            else
                RUST_REACH_QUICK[$1]="!"
                step_cache_note "$1: not keyed, its steps run: ${quick_reach:-its reach could not be resolved}"
            fi
        fi
        [ "${RUST_REACH_QUICK[$1]}" != "!" ]
        return
    fi
    [[ $STEP_CACHE_STRICT ]] || return 1
    if [[ -z "${RUST_REACH[$1]+set}" ]]; then
        local reach
        if ! step_cache_cargo_tree "$1"; then
            RUST_REACH[$1]="!"
            step_cache_note "$1: not keyed, its steps run: cargo tree failed"
        elif reach=$(rust_reach_resolve "$1"); then
            RUST_REACH[$1]="$reach"
        else
            RUST_REACH[$1]="!"
            step_cache_note "$1: not keyed, its steps run: ${reach:-its reach could not be resolved}"
        fi
    fi
    [ "${RUST_REACH[$1]}" != "!" ]
}

# The inputs of a --quick key over the crates $1 (comma-separated,
# step_cache_command_crates), its reach in the --quick form, into the array
# named $2. Returns 1 where that reach is refused or the --quick set is off.
step_cache_quick_inputs() {
    local -n quick_inputs_out="$2"
    quick_inputs_out=()
    rust_reach --quick "$1" || return 1
    mapfile -t quick_inputs_out <<<"${RUST_REACH_QUICK[$1]}"
}

# The crates of the cargo command $@, comma-separated and sorted: those its
# -p (--package) options name, or, where it has --workspace, every workspace
# member (WORKSPACE_CRATES) but those its --exclude options name. Arguments
# after `--` are not read. Returns 1, printing nothing, where an --exclude
# names no member, or the command has neither -p nor --workspace.
step_cache_command_crates() {
    local -
    set -o pipefail
    local argument previous="" workspace="" member crates
    local -a packages=() excluded=() members=() selected=()
    local -A is_member=() is_excluded=()
    for argument in "$@"; do
        [ "$argument" = "--" ] && break
        case "$previous" in
            -p | --package) packages+=("$argument") ;;
            --exclude) excluded+=("$argument") ;;
        esac
        case "$argument" in
            --package=*) packages+=("${argument#--package=}") ;;
            --exclude=*) excluded+=("${argument#--exclude=}") ;;
            --workspace) workspace=1 ;;
        esac
        previous="$argument"
    done
    if [ -n "${WORKSPACE_CRATES:-}" ]; then
        mapfile -t members <<<"$WORKSPACE_CRATES"
    fi
    for member in ${members[@]+"${members[@]}"}; do
        [ -n "$member" ] && is_member[$member]=1
    done
    for argument in ${excluded[@]+"${excluded[@]}"}; do
        [ -n "$argument" ] && [[ -n "${is_member[$argument]+set}" ]] || return 1
        is_excluded[$argument]=1
    done
    if [[ $workspace ]]; then
        for member in ${members[@]+"${members[@]}"}; do
            [[ -n "${is_excluded[$member]+set}" ]] || selected+=("$member")
        done
    else
        selected=(${packages[@]+"${packages[@]}"})
    fi
    [ "${#selected[@]}" -gt 0 ] || return 1
    crates=$(printf '%s\n' "${selected[@]}" | LC_ALL=C sort -u | paste -sd,) || return 1
    printf "%s" "$crates"
}

# `cargo tree` of the crates $1 (comma-separated) with every feature of
# theirs on, their dev-dependencies and the build dependencies, on this host,
# a package a line ("<name> v<version>", a path package's directory after
# it), into STEP_CACHE_TREE[$1], once per crate set. Returns 1 where it fails.
step_cache_cargo_tree() {
    if [[ -z "${STEP_CACHE_TREE[$1]+set}" ]]; then
        local crates crate tree
        local args=()
        IFS=, read -ra crates <<<"$1"
        for crate in "${crates[@]}"; do
            args+=(-p "$crate")
        done
        if tree=$(step_cache_cargo tree "${args[@]}" --all-features -e normal,build,dev --prefix none \
            --format '{p}' --locked 2>/dev/null); then
            STEP_CACHE_TREE[$1]="+$tree"
        else
            STEP_CACHE_TREE[$1]="!"
        fi
    fi
    [ "${STEP_CACHE_TREE[$1]}" != "!" ]
}

# Prints the inputs of the crates $1's reach, one per line: the
# configuration (rust_config_inputs), the reached workspace packages'
# directories, the files they include from outside them
# (step_cache_includes) and their target sources outside their directories.
# Refused (prints the reason, returns 1) where the tree (step_cache_cargo_tree)
# holds a host probe, an unlisted registry build script or procedural macro,
# a path package outside the workspace or an untrusted workspace build
# script or procedural macro, or where an include cannot be resolved.
# `--quick` first: the --quick key's form, which leaves the host probes and
# the registry build code to --commit and refuses only for the others.
rust_reach_resolve() {
    local -
    set -o pipefail
    local quick=""
    if [ "$1" = "--quick" ]; then
        quick=1
        shift
    fi
    local tree="${STEP_CACHE_TREE[$1]#+}"
    local found locals name path relative sources source includes
    local dirs=() helper_args=()
    if [[ ! $quick ]]; then
        if ! found=$(awk -v list="$CACHE_HOST_PROBES" \
            'BEGIN {n = split(list, a, " "); for (i = 1; i <= n; i++) probe[a[i]] = 1}
            ($1 in probe) && !seen[$1]++ {printf "%s%s", (c++ ? " " : ""), $1}' <<<"$tree"); then
            printf "its tree could not be read"
            return 1
        fi
        if [ -n "$found" ]; then
            printf "its reach holds the host probe(s) %s (CACHE_HOST_PROBES)" "$found"
            return 1
        fi
        if ! found=$(awk -v code="$STEP_CACHE_REGISTRY_BUILD_CODE" \
            -v reviewed="$(printf '%s\n' ${CACHE_REVIEWED_REGISTRY_CODE[@]+"${CACHE_REVIEWED_REGISTRY_CODE[@]}"})" \
            'BEGIN {n = split(code, a, "\n"); for (i = 1; i <= n; i++) built[a[i]] = 1
                    n = split(reviewed, b, "\n"); for (i = 1; i <= n; i++) listed[b[i]] = 1}
            {p = $1 " " $2} (p in built) && !(p in listed) && !seen[p]++ {printf "%s%s", (c++ ? ", " : ""), p}' \
            <<<"$tree"); then
            printf "its tree could not be read"
            return 1
        fi
        if [ -n "$found" ]; then
            printf "its reach holds registry build code CACHE_REVIEWED_REGISTRY_CODE does not list: %s" "$found"
            return 1
        fi
    fi
    if ! locals=$(awk '{sub(/ \(\*\)$/, "")}
        match($0, / \(\/[^)]*\)$/) {p = substr($0, RSTART + 2, RLENGTH - 3); if (!seen[p]++) print $1 " " p}' \
        <<<"$tree"); then
        printf "its tree could not be read"
        return 1
    fi
    if [ -z "$locals" ]; then
        printf "cargo tree lists no workspace package"
        return 1
    fi
    while read -r name path; do
        if ! jq -e --arg name "$name" --arg path "$path/Cargo.toml" \
            'any(.packages[]; .name == $name and .manifest_path == $path)' \
            <<<"$WORKSPACE_METADATA" >/dev/null; then
            printf "its reach holds the path package %s (%s), outside the workspace" "$name" "$path"
            return 1
        fi
        relative="${path#"$PWD"/}"
        if [[ " $CACHE_TRUSTED_BUILD_CODE " == *" $name "* ]]; then
            helper_args+=(--trusted "$relative")
        elif jq -e --arg name "$name" \
            'any(.packages[] | select(.name == $name) | .targets[]; any(.kind[]; . == "custom-build" or . == "proc-macro"))' \
            <<<"$WORKSPACE_METADATA" >/dev/null; then
            printf "its reach holds %s, whose build script or procedural macro CACHE_TRUSTED_BUILD_CODE does not list" \
                "$name"
            return 1
        else
            helper_args+=("$relative")
        fi
        dirs+=("$relative")
        if ! sources=$(jq -r --arg name "$name" --arg dir "$path/" \
            '.packages[] | select(.name == $name) | .targets[].src_path | select(startswith($dir) | not)' \
            <<<"$WORKSPACE_METADATA"); then
            printf "the target sources of %s could not be read" "$name"
            return 1
        fi
        if [ -n "$sources" ]; then
            while IFS= read -r source; do
                dirs+=("${source#"$PWD"/}")
            done <<<"$sources"
        fi
    done <<<"$locals"
    if ! includes=$(step_cache_includes "${helper_args[@]}"); then
        printf "%s" "$includes"
        return 1
    fi
    rust_config_inputs "${dirs[@]}" || return 1
    printf "%s\n" "${dirs[@]}"
    if [ -n "$includes" ]; then
        printf "%s\n" "$includes"
    fi
}

# The files the code under the package directories $@ includes from outside
# them (scripts/step_cache_includes.py; `--trusted <dir>` for a package
# CACHE_TRUSTED_BUILD_CODE lists), one per line. Returns 1, the helper's
# first refusal printed, where it refuses or fails.
step_cache_includes() {
    local output target where
    if ! output=$(python3 "$STEP_CACHE_INCLUDES" "$@"); then
        printf "the include scan refused it: %s" "${output%%$'\n'*}"
        return 1
    fi
    [ -n "$output" ] || return 0
    while IFS=$'\t' read -r target where; do
        printf "%s\n" "$target"
    done <<<"$output"
}

# What every cargo step reads besides the packages, one per line: the
# workspace's manifest, lock and tool configuration, nextest's, every Cargo
# configuration file Cargo would read here (each ancestor's .cargo/ and
# CARGO_HOME's), and clippy's in each of the package directories $@ and each
# of their ancestors up to /, as clippy looks it up from the manifest's
# directory upward; a missing one hashed as missing.
rust_config_inputs() {
    local -A seen=()
    local entry dir
    local entries=(Cargo.toml Cargo.lock .cargo .config/nextest.toml rust-toolchain rust-toolchain.toml
        rustfmt.toml .rustfmt.toml clippy.toml .clippy.toml)
    dir=$(dirname "$PWD")
    while :; do
        entries+=("$dir/.cargo/config" "$dir/.cargo/config.toml")
        [ "$dir" = "/" ] && break
        dir=$(dirname "$dir")
    done
    entries+=("${CARGO_HOME:-$HOME/.cargo}/config" "${CARGO_HOME:-$HOME/.cargo}/config.toml")
    for dir in "$@"; do
        [ -d "$dir" ] || continue
        [[ "$dir" == /* ]] || dir="$PWD/$dir"
        while :; do
            entries+=("${dir%/}/clippy.toml" "${dir%/}/.clippy.toml")
            [ "$dir" = "/" ] && break
            dir=$(dirname "$dir")
        done
    done
    for entry in "${entries[@]}"; do
        entry="${entry#"$PWD"/}"
        if [[ -z "${seen[$entry]+set}" ]]; then
            seen[$entry]=1
            printf "%s\n" "$entry"
        fi
    done
}

# The self-test: a synthetic step over a synthetic tree in a temporary
# directory, then the records, the locale and the pipe rule, the include
# helper's self-test, and the reach over a synthetic workspace. Every
# permission a case removes is given back by the trap on its exit, before
# the directory is removed.
step_cache_self_test() {
    local dir failures=0
    mkdir -p "$STEP_CACHE_SELF_TEST_ROOT"
    dir=$(mktemp -d "$STEP_CACHE_SELF_TEST_ROOT/step-cache-self-test.XXXXXX") || return 1
    # shellcheck disable=SC2064  # the directory is fixed when the trap is set
    trap "chmod -R u+rwx $(printf %q "$dir") 2>/dev/null; rm -rf $(printf %q "$dir")" EXIT
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
    # $1 = the case, $2 = what it found, $3 = what was expected.
    check() {
        if [ "$2" != "$3" ]; then
            printf "step cache self-test: %s: expected %s, got %s\n" "$1" "$3" "$2" >&2
            failures=$((failures + 1))
        else
            printf "  ok  %s: %s\n" "$1" "$4"
        fi
    }
    # $1 = the case, $2 = "ran" or "skipped", $3... = run_once's arguments.
    expect() {
        local got
        got=$(run_once "${@:3}")
        check "$1" "$got" "$2" "$got"
    }
    printf "step cache self-test, a step:\n"
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

    local found
    printf "step cache self-test, the records and the inputs:\n"
    # 1. A cache folder that cannot be written: the record returns 1 and
    # the caller goes on.
    mkdir -p "$dir/locked-cache"
    chmod a-w "$dir/locked-cache"
    if [ -w "$dir/locked-cache" ]; then
        printf "  skip  a cache folder that cannot be written: this user writes it all the same\n"
    else
        found=$(set -e; STEP_CACHE_DIR="$dir/locked-cache"; step_cache_record "a step" "0" \
            || printf "returned %s, " "$?"; printf "went on")
        check "a cache folder that cannot be written: the record" "$found" "returned 1, went on" ran
    fi
    chmod u+w "$dir/locked-cache"
    # 2. The locale: B.txt and a.txt, which en_US.UTF-8 sorts otherwise than
    # C, give one hash under both.
    mkdir -p "$dir/locale"
    printf 'upper\n' >"$dir/locale/B.txt"
    printf 'lower\n' >"$dir/locale/a.txt"
    local locales
    locales=$(locale -a 2>/dev/null) || locales=""
    if grep -qixE 'en_US\.utf-?8' <<<"$locales"; then
        local hash_c hash_us
        hash_c=$(LC_ALL=C step_cache_inputs "$dir/locale") || hash_c="failed"
        hash_us=$(LC_ALL=en_US.UTF-8 step_cache_inputs "$dir/locale") || hash_us="failed"
        check "the locale (B.txt and a.txt under C and en_US.UTF-8)" "$hash_us" "$hash_c" ran
    else
        printf "  skip  the locale (en_US.UTF-8): not installed (locale -a)\n"
    fi
    # 3. A pipe failure: the first of two inputs unreadable, as a file and
    # inside a directory, fails the inputs and the key, in the gate's own
    # shell options (set -e and set -u, no pipefail), each call a condition.
    mkdir -p "$dir/pipe/folder"
    printf 'bad\n' >"$dir/pipe/bad.txt"
    printf 'ok\n' >"$dir/pipe/ok.txt"
    printf 'bad\n' >"$dir/pipe/folder/bad.txt"
    chmod 000 "$dir/pipe/bad.txt" "$dir/pipe/folder/bad.txt"
    if [ -r "$dir/pipe/bad.txt" ]; then
        printf "  skip  a pipe failure (an unreadable input): this user reads it all the same\n"
    else
        found=$(bash -c 'set -e; set -u; set +o pipefail; source "$1"
            step_cache_init "$2/pipe-cache" "test-label" "base"
            for given in "$2/bad.txt" "$2/folder"; do
                if ! h=$(step_cache_inputs "$given" "$2/ok.txt"); then printf "inputs failed; "; else printf "inputs %s; " "$h"; fi
                if ! k=$(step_cache_key "a step" "$given" "$2/ok.txt" -- true); then printf "key failed; "; else printf "key %s; " "$k"; fi
            done' _ "$STEP_CACHE_SELF" "$dir/pipe" 2>/dev/null) || found="the shell ended: $found"
        check "a pipe failure: an unreadable file, then inside a directory, fails the inputs and the key" \
            "$found" "inputs failed; key failed; inputs failed; key failed; " ran
    fi
    chmod u+rw "$dir/pipe/bad.txt" "$dir/pipe/folder/bad.txt"

    printf "step cache self-test, the --quick environment:\n"
    step_cache_self_test_quick_environment || failures=$((failures + 1))

    printf "step cache self-test, the include helper:\n"
    if found=$(python3 "$STEP_CACHE_INCLUDES" --self-test 2>&1); then
        printf "  ok  the include helper's self-test: ran\n"
    else
        printf "%s\n" "$found" >&2
        printf "step cache self-test: the include helper's self-test failed\n" >&2
        failures=$((failures + 1))
    fi

    printf "step cache self-test, the reach over a synthetic workspace:\n"
    step_cache_self_test_reach "$dir" || failures=$((failures + 1))

    chmod -R u+rwx "$dir" 2>/dev/null || true
    rm -rf "$dir"
    trap - EXIT
    if [ "$failures" -ne 0 ]; then
        return 1
    fi
    printf "step cache self-test: every case passed\n"
}

# 4. The --quick environment (step_cache_quick_environment), each hash taken
# in a shell of its own whose environment is a baseline that sets every
# per-session variable of the deny-list, with one change: a variable outside
# the deny-list changed or added changes it, a denied one changed or unset
# leaves it, a thread count that is no positive number changes it, the names
# next to the deny-list's change it, and a denied variable whose value holds
# a newline and an `=` leaves it, which shows the entries split at NUL (split
# at newlines, the value would read as a RUSTFLAGS, which that case's baseline
# lacks). Returns 1 on a failed case.
step_cache_self_test_quick_environment() {
    local failures=0 name
    local per_session=(CLAUDE_CODE_SESSION_ID CLAUDE_PID CLAUDE_CODE_MESSAGING_TOKEN CLAUDE_CODE_MESSAGING_SOCKET
        CLAUDE_CODE_CHILD_SESSION CLAUDE_EFFORT WINDOWID KONSOLE_DBUS_SESSION KONSOLE_DBUS_WINDOW SHELL_SESSION_ID
        INVOCATION_ID JOURNAL_STREAM SYSTEMD_EXEC_PID MANAGERPID MANAGERPIDFDID MEMORY_PRESSURE_WATCH
        MEMORY_PRESSURE_WRITE KONSOLE_DBUS_SERVICE KONSOLE_DBUS_ACTIVATION_COOKIE CLAUDE_CODE_EXECPATH
        CLAUDE_CODE_SESSION_ATTENDED AI_AGENT)
    local baseline=("PATH=$PATH" "HOME=/home/step-cache-self-test" "LANG=C.UTF-8" "RUSTFLAGS=-Cdebuginfo=0"
        "DBUS_SESSION_BUS_ADDRESS=unix:path=/run/step-cache-self-test" "CARGO_BUILD_JOBS=16"
        "RUST_TEST_THREADS=16" "NEXTEST_TEST_THREADS=1" "CARGO_TERM_COLOR=never"
        "CARGO_TARGET_DIR=/step-cache-self-test/target")
    for name in "${per_session[@]}"; do
        baseline+=("$name=baseline")
    done
    # The hash with the changes $@ made to the baseline: NAME=value sets or
    # adds NAME, -NAME removes it.
    quick_environment_with() {
        local change entry target
        local entries=("${baseline[@]}") kept=()
        for change in "$@"; do
            target="${change#-}"
            target="${target%%=*}"
            kept=()
            for entry in "${entries[@]}"; do
                [ "${entry%%=*}" = "$target" ] || kept+=("$entry")
            done
            entries=("${kept[@]}")
            [[ $change == -* ]] || entries+=("$change")
        done
        env -i "${entries[@]}" bash -c 'source "$1" && step_cache_quick_environment' _ "$STEP_CACHE_SELF"
    }
    # $1 = the reference's changes (one word, "" for none), $2... = the
    # changes, one per case; prints "<change> equal" or "<change> changed"
    # for each, ";"-separated.
    compare_to() {
        local reference change got sep=""
        local -a reference_changes=()
        [ -n "$1" ] && reference_changes=("$1")
        reference=$(quick_environment_with ${reference_changes[@]+"${reference_changes[@]}"}) || reference="failed"
        shift
        for change in "$@"; do
            got=$(quick_environment_with ${reference_changes[@]+"${reference_changes[@]}"} "$change") || got="failed"
            if [ "$got" = failed ] || [ "$reference" = failed ]; then
                printf '%s%s failed' "$sep" "${change%%$'\n'*}"
            elif [ "$got" = "$reference" ]; then
                printf '%s%s equal' "$sep" "${change%%$'\n'*}"
            else
                printf '%s%s changed' "$sep" "${change%%$'\n'*}"
            fi
            sep="; "
        done
    }
    # $1 = the case, $2 = the expected word ("equal" or "changed"), $3 = the
    # reference's change, $4... = the changes.
    expect_environment() {
        local case_name="$1" word="$2" reference="$3" change expected="" sep=""
        shift 3
        for change in "$@"; do
            expected="$expected$sep${change%%$'\n'*} $word"
            sep="; "
        done
        check "$case_name" "$(compare_to "$reference" "$@")" "$expected" ran
    }
    expect_environment "the --quick environment: a variable outside the deny-list changed or added changes it" \
        changed "" RUSTFLAGS=-Dwarnings HOME=/home/step-cache-other PATH="$PATH:/step-cache-self-test" \
        LANG=en_US.UTF-8 DBUS_SESSION_BUS_ADDRESS=unix:path=/run/step-cache-other STEP_CACHE_SELF_TEST_NEW=1
    expect_environment "the --quick environment: the thread and job counts, the output and the location changed leave it" \
        equal "" CARGO_BUILD_JOBS=8 RUST_TEST_THREADS=4 NEXTEST_TEST_THREADS=2 CARGO_TERM_COLOR=always \
        CARGO_TARGET_DIR=/step-cache-self-test/other
    expect_environment "the --quick environment: RUST_TEST_THREADS=0, no positive number, changes it" \
        changed "" RUST_TEST_THREADS=0
    expect_environment "the --quick environment: the names next to the deny-list's change it" \
        changed "" CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUNNER=valgrind \
        CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUSTFLAGS=-Dwarnings CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=clang \
        CARGO_BUILD_TARGET=x86_64-unknown-linux-musl CARGO_BUILD_RUSTFLAGS=-Dwarnings RUST_LOG_STYLE=never
    local changes=()
    for name in "${per_session[@]}"; do
        changes+=("$name=other" "-$name")
    done
    expect_environment "the --quick environment: every per-session variable changed or unset leaves it" \
        equal "" "${changes[@]}"
    expect_environment "the --quick environment: a denied variable holding a newline and an = leaves it (split at NUL)" \
        equal -RUSTFLAGS "CARGO_TERM_COLOR=always"$'\n'"RUSTFLAGS=-Dwarnings"
    [ "$failures" -eq 0 ]
}

# The reach's part of the self-test, in the directory $1: a synthetic
# workspace (ws/) whose packages each stand for one case, its workspace and
# full metadata, and a cargo stub that prints each package's tree from trees/
# and the full metadata from the file metadata-file names, fails while
# tree-fail or metadata-fail exists, and logs each call; ws/ is a synthetic
# git tree (git with the caller's GIT_* removed), from whose index each case
# restores what it edits. Each case runs in a subshell from ws/, as
# check_release.sh runs from the repository's root. Returns 1 on a failed
# case.
step_cache_self_test_reach() {
    local dir="$1" failures=0 ws="$1/ws" package members="" full
    mkdir -p "$ws/docs" "$dir/trees" "$dir/outside/g/src"
    for package in a b c d e f; do
        mkdir -p "$ws/$package/src"
        printf '[package]\nname = "%s"\n' "$package" >"$ws/$package/Cargo.toml"
        printf '\n' >"$ws/$package/src/lib.rs"
    done
    printf 'const NOTES: &str = include_str!("../../docs/notes.md");\n' >"$ws/a/src/lib.rs"
    printf 'notes\n' >"$ws/docs/notes.md"
    mkdir -p "$ws/docs/archive/sub"
    printf 'archived\n' >"$ws/docs/archive/sub/notes.txt"
    printf 'data = 1\n' >"$ws/docs/data.toml"
    printf 'const NOTES: &str = include_str!(NOTES_PATH);\n' >"$ws/b/src/lib.rs"
    printf 'fn main() {}\n' >"$ws/c/build.rs"
    printf '[workspace]\nmembers = ["a", "b", "c", "d", "e", "f"]\n' >"$ws/Cargo.toml"
    printf '\n' >"$ws/Cargo.lock"
    printf '[package]\nname = "g"\n' >"$dir/outside/g/Cargo.toml"
    # The trees: a (an include outside it, a listed registry build script),
    # b (a non-literal include), c (its own build script, untrusted), d (the
    # probe cc), e (an unlisted registry build script), f (a path package
    # outside the workspace).
    printf 'a v0.1.0 (%s/a)\nlisted v1.0.0\n' "$ws" >"$dir/trees/a"
    printf 'b v0.1.0 (%s/b)\n' "$ws" >"$dir/trees/b"
    printf 'c v0.1.0 (%s/c)\n' "$ws" >"$dir/trees/c"
    printf 'd v0.1.0 (%s/d)\ncc v1.0.0\n' "$ws" >"$dir/trees/d"
    printf 'e v0.1.0 (%s/e)\nunlisted v1.0.0\n' "$ws" >"$dir/trees/e"
    printf 'f v0.1.0 (%s/f)\ng v0.1.0 (%s/outside/g)\n' "$ws" "$dir" >"$dir/trees/f"
    for package in a b c d e f; do
        members="$members${members:+, }{\"name\": \"$package\", \"version\": \"0.1.0\", \"source\": null,"
        members="$members \"manifest_path\": \"$ws/$package/Cargo.toml\","
        members="$members \"targets\": [{\"kind\": [\"lib\"], \"src_path\": \"$ws/$package/src/lib.rs\"}"
        if [ "$package" = c ]; then
            members="$members, {\"kind\": [\"custom-build\"], \"src_path\": \"$ws/c/build.rs\"}"
        fi
        members="$members]}"
    done
    printf '{"packages": [%s]}\n' "$members" >"$dir/workspace.json"
    full=$(printf '{"packages": [%s, %s, %s, %s]}\n' "$members" \
        '{"name": "listed", "version": "1.0.0", "source": "registry+https://example.invalid", "targets": [{"kind": ["custom-build"]}]}' \
        '{"name": "unlisted", "version": "1.0.0", "source": "registry+https://example.invalid", "targets": [{"kind": ["custom-build"]}]}' \
        '{"name": "cc", "version": "1.0.0", "source": "registry+https://example.invalid", "targets": [{"kind": ["lib"]}]}')
    printf '%s\n' "$full" >"$dir/full.json"
    printf '{"packages": []}\n' >"$dir/empty.json"
    printf 'full.json\n' >"$dir/metadata-file"
    cat >"$dir/cargo-stub" <<'STUB'
#!/bin/bash
here=$(dirname "$0")
printf '%s\n' "$*" >>"$here/cargo.log"
case "$1" in
    tree)
        [ -e "$here/tree-fail" ] && exit 1
        shift
        while [ "$#" -gt 0 ]; do
            if [ "$1" = -p ]; then
                cat "$here/trees/$2" || exit 1
                shift
            fi
            shift
        done
        ;;
    metadata) [ -e "$here/metadata-fail" ] && exit 1; cat "$here/$(cat "$here/metadata-file")" ;;
    *) exit 1 ;;
esac
STUB
    chmod +x "$dir/cargo-stub"
    local git_test=(step_cache_git -c user.name=step-cache -c user.email=step-cache@invalid
        -c commit.gpgsign=false -c core.hooksPath=/dev/null)
    "${git_test[@]}" init -q "$ws" && "${git_test[@]}" -C "$ws" add -A \
        && "${git_test[@]}" -C "$ws" commit -q -m "step cache self-test" || return 1

    # One case in a subshell from ws/, the gate's variables set for the
    # synthetic workspace: the commands $1, eval'd, with key_of <crate>, the
    # key of the step "check (<crate>)" or "none: " and the queued notes, and
    # key_of_quick <crate>, its --quick key, or the same.
    reach_case() {
        (
            cd "$ws" || exit 1
            # shellcheck disable=SC2034  # read by the functions under test
            WORKSPACE_METADATA=$(cat "$dir/workspace.json")
            # shellcheck disable=SC2034
            CACHE_HOST_PROBES="pkg-config cc cmake bindgen system-deps vcpkg metadeps"
            # shellcheck disable=SC2034
            CACHE_TRUSTED_BUILD_CODE="a"
            # shellcheck disable=SC2034
            CACHE_REVIEWED_REGISTRY_CODE=("listed v1.0.0")
            declare -A CACHE_INPUTS=()
            for package in a b c d e f; do
                CACHE_INPUTS["check ($package)"]="@rust:$package"
            done
            STEP_CACHE_CARGO="$dir/cargo-stub"
            RUST_REACH=()
            RUST_REACH_QUICK=()
            STEP_CACHE_TREE=()
            STEP_CACHE_NOTES=()
            STEP_CACHE_NOTED=()
            step_cache_init "$dir/reach-cache" "test-label" "tools 1" || exit 1
            step_cache_set_full_metadata "$full" || exit 1
            step_cache_init_quick "quick base 1" || exit 1
            key_of() {
                local step_inputs=() k
                if cache_inputs "check ($1)" step_inputs && k=$(step_cache_key "check ($1)" "${step_inputs[@]}" -- true); then
                    printf "%s" "$k"
                else
                    printf "none: %s" "${STEP_CACHE_NOTES[*]-}"
                fi
            }
            key_of_quick() {
                local step_inputs=() k
                if step_cache_quick_inputs "$1" step_inputs \
                    && k=$(step_cache_key --quick "check ($1)" "${step_inputs[@]}" -- true); then
                    printf "%s" "$k"
                else
                    printf "none: %s" "${STEP_CACHE_NOTES[*]-}"
                fi
            }
            eval "$1"
        )
    }
    # $1 = the case, $2 = the commands, $3 = what they must print.
    check_reach() {
        local got
        got=$(reach_case "$2" 2>&1) || got="$got (exit $?)"
        if [ "$got" != "$3" ]; then
            printf "step cache self-test: %s: expected %s, got %s\n" "$1" "$3" "$got" >&2
            failures=$((failures + 1))
        else
            printf "  ok  %s: ran\n" "$1"
        fi
    }
    # shellcheck disable=SC2016  # each case's commands run in reach_case
    {
        # 6. An include outside its package is hashed: editing its target
        # changes the key; a non-literal include refuses the reach; the cargo
        # tree stub failing gives no key.
        check_reach "an include outside its package: in the reach's inputs" \
            'rust_reach a && grep -cxF docs/notes.md <<<"${RUST_REACH[a]}"' "1"
        check_reach "an include outside its package: its target edited changes the key" '
            first=$(key_of a); printf "edited\n" >>docs/notes.md; second=$(key_of a)
            step_cache_git checkout -q -- docs/notes.md
            if [[ $first != none* && $second != none* && $first != "$second" ]]; then printf "changed"
            else printf "%s, %s" "$first" "$second"; fi' "changed"
        check_reach "a non-literal include refuses the reach" '
            k=$(key_of b)
            if [[ $k == "none: b: not keyed, its steps run: the include scan refused it: "*"not a string literal"* ]]
            then printf "refused"; else printf "%s" "$k"; fi' "refused"
        check_reach "the cargo tree stub failing gives no key" '
            : >../tree-fail; k=$(key_of a); rm -f ../tree-fail
            printf "%s" "$k"' "none: a: not keyed, its steps run: cargo tree failed"
        # The same in the --quick form: the include target, a .md file that
        # QUICK_UNREAD matches, stays in the key of the reach that includes it.
        check_reach "an include outside its package: its target edited changes the --quick key" '
            first=$(key_of_quick a); printf "edited\n" >>docs/notes.md; second=$(key_of_quick a)
            step_cache_git checkout -q -- docs/notes.md
            if [[ $first != none* && $second != none* && $first != "$second" ]]; then printf "changed"
            else printf "%s, %s" "$first" "$second"; fi' "changed"
        check_reach "a non-literal include refuses the --quick reach" '
            k=$(key_of_quick b)
            if [[ $k == "none: b: not keyed, its steps run: the include scan refused it: "*"not a string literal"* ]]
            then printf "refused"; else printf "%s" "$k"; fi' "refused"
        check_reach "the cargo tree stub failing gives no --quick key" '
            : >../tree-fail; k=$(key_of_quick a); rm -f ../tree-fail
            printf "%s" "$k"' "none: a: not keyed, its steps run: cargo tree failed"
        # 7. The refusals, each naming what it found: a probe (cc), a
        # registry build script the list does not name, an unreviewed
        # workspace build script, a path package outside the workspace.
        check_reach "a reach holding a probe (cc) refused" 'key_of d' \
            "none: d: not keyed, its steps run: its reach holds the host probe(s) cc (CACHE_HOST_PROBES)"
        check_reach "a reach holding a registry build script CACHE_REVIEWED_REGISTRY_CODE does not list refused" \
            'key_of e' \
            "none: e: not keyed, its steps run: its reach holds registry build code CACHE_REVIEWED_REGISTRY_CODE does not list: unlisted v1.0.0"
        check_reach "a reach holding an unreviewed workspace build script refused" 'key_of c' \
            "none: c: not keyed, its steps run: its reach holds c, whose build script or procedural macro CACHE_TRUSTED_BUILD_CODE does not list"
        check_reach "a reach holding a path package outside the workspace refused" 'key_of f' \
            "none: f: not keyed, its steps run: its reach holds the path package g ($dir/outside/g), outside the workspace"
        # In the --quick form the probe and the unlisted registry build
        # script are accepted (left to --commit), the other two refused.
        check_reach "the --quick reach: a probe (cc) and an unlisted registry build script accepted" '
            d=$(key_of_quick d); e=$(key_of_quick e)
            if [[ $d != none* && $e != none* ]]; then printf "accepted"; else printf "%s, %s" "$d" "$e"; fi' "accepted"
        check_reach "the --quick reach: an unreviewed workspace build script refused" 'key_of_quick c' \
            "none: c: not keyed, its steps run: its reach holds c, whose build script or procedural macro CACHE_TRUSTED_BUILD_CODE does not list"
        check_reach "the --quick reach: a path package outside the workspace refused" 'key_of_quick f' \
            "none: f: not keyed, its steps run: its reach holds the path package g ($dir/outside/g), outside the workspace"
        # 8. The two sets: a step recorded in one is found in it and not in
        # the other.
        check_reach "the two sets: a strict record is no --quick one, a --quick record no strict one" '
            step_cache_record "check (a)" "k1"; step_cache_record --quick "check (b)" "k2"
            for found in "check (a)|k1|" "check (a)|k1|--quick" "check (b)|k2|--quick" "check (b)|k2|"; do
                IFS="|" read -r step k quick <<<"$found"
                if step_cache_hit $quick "$step" "$k" >/dev/null; then printf "%s %s found; " "$step" "${quick:-strict}"
                else printf "%s %s not found; " "$step" "${quick:-strict}"; fi
            done' \
            "check (a) strict found; check (a) --quick not found; check (b) --quick found; check (b) strict not found; "
        # 9. A step whose input changes while it runs records nothing: its
        # key taken before and after it ran, as check_release.sh's
        # step_passed does, the record written only where the two agree.
        check_reach "a step whose input changes while it runs records nothing" '
            before=$(key_of a); printf "// edited while the step ran\n" >>a/src/lib.rs; after=$(key_of a)
            if [ "$before" = "$after" ]; then step_cache_record "check (a)" "$before"; fi
            step_cache_git checkout -q -- a/src/lib.rs
            if step_cache_hit "check (a)" "$(key_of a)" >/dev/null; then printf "recorded"; else printf "not recorded"; fi' \
            "not recorded"
        check_reach "a step whose input changes while it runs records nothing in the --quick set" '
            before=$(key_of_quick a); printf "// edited while the step ran\n" >>a/src/lib.rs; after=$(key_of_quick a)
            if [ "$before" = "$after" ]; then step_cache_record --quick "check (a)" "$before"; fi
            step_cache_git checkout -q -- a/src/lib.rs
            if step_cache_hit --quick "check (a)" "$(key_of_quick a)" >/dev/null; then printf "recorded"
            else printf "not recorded"; fi' \
            "not recorded"
        # 10. The full metadata: a read that fails, or one that lists no
        # package, turns the cache off with its reason, and no key is
        # computed: no cargo tree runs.
        check_reach "a full metadata read that fails: the cache off, no key, no cargo tree" '
            : >../metadata-fail; : >../cargo.log
            status=0; step_cache_full_metadata || status=$?; rm -f ../metadata-fail
            printf "%s; %s; %s; " "$status" "$STEP_CACHE_OFF_REASON" "$(key_of a)"
            grep -c "^tree" ../cargo.log || true' \
            "1; cargo metadata --format-version 1 --locked failed; none: ; 0"
        check_reach "a full metadata that lists no package: the cache off, no key, no cargo tree" '
            printf "empty.json\n" >../metadata-file; : >../cargo.log
            status=0; step_cache_full_metadata || status=$?; printf "full.json\n" >../metadata-file
            printf "%s; %s; %s; " "$status" "$STEP_CACHE_OFF_REASON" "$(key_of a)"
            grep -c "^tree" ../cargo.log || true' \
            "1; the full metadata (cargo metadata --locked) lists no package; none: ; 0"
    }

    # 5. The --quick base, in the gate's own shell options (set -e and set
    # -u, no pipefail), each call a condition, as check_release.sh makes it:
    # an edit of a nested file QUICK_UNREAD matches, or of a file inside a
    # package, leaves it; an edit of a file outside the packages, or a new
    # untracked one there, changes it; where git ls-files fails (the tree's
    # .git unreadable, GIT_CEILING_DIRECTORIES keeping git from the
    # repositories above it), there is no base.
    local found
    # shellcheck disable=SC2016  # the commands run in the child shell
    found=$(step_cache_without_git_environment bash -c 'set -e; set -u; set +o pipefail; source "$1"; cd "$2"
        WORKSPACE_METADATA=$(cat "$3")
        base_of() { if ! b=$(step_cache_quick_base "tools 1"); then printf "none: %s" "$b"; else printf "%s" "$b"; fi; }
        compared() { if [[ $1 == none* || $2 == none* ]]; then printf "%s, %s" "$1" "$2"
            elif [ "$1" = "$2" ]; then printf "equal"; else printf "changed"; fi; }
        first=$(base_of)
        printf "x\n" >>docs/archive/sub/notes.txt; second=$(base_of); git checkout -q -- docs/archive/sub/notes.txt
        printf "an unread file edited %s; " "$(compared "$first" "$second")"
        printf "// x\n" >>a/src/lib.rs; second=$(base_of); git checkout -q -- a/src/lib.rs
        printf "a package file edited %s; " "$(compared "$first" "$second")"
        printf "x = 2\n" >>docs/data.toml; second=$(base_of); git checkout -q -- docs/data.toml
        printf "a file outside the packages edited %s; " "$(compared "$first" "$second")"
        printf "y = 1\n" >docs/new.toml; second=$(base_of); rm -f docs/new.toml
        printf "a new untracked file %s; " "$(compared "$first" "$second")"
        second=$(base_of)
        printf "restored %s" "$(compared "$first" "$second")"' _ "$STEP_CACHE_SELF" "$ws" "$dir/workspace.json" 2>&1) \
        || found="the shell ended: $found"
    check "the --quick base: an unread or a package file edited leaves it, a file outside the packages edited or added changes it" \
        "$found" "an unread file edited equal; a package file edited equal; a file outside the packages edited changed; a new untracked file changed; restored equal" ran
    chmod 000 "$ws/.git"
    if [ -r "$ws/.git" ] || [ -x "$ws/.git" ]; then
        printf "  skip  the --quick base where git ls-files fails (an unreadable .git): this user reads it all the same\n"
    else
        # shellcheck disable=SC2016
        found=$(step_cache_without_git_environment env GIT_CEILING_DIRECTORIES="$dir" \
            bash -c 'set -e; set -u; set +o pipefail; source "$1"; cd "$2"
            WORKSPACE_METADATA=$(cat "$3")
            if ! b=$(step_cache_quick_base "tools 1"); then printf "none: %s" "$b"; else printf "base %s" "$b"; fi' \
            _ "$STEP_CACHE_SELF" "$ws" "$dir/workspace.json" 2>&1) || found="the shell ended: $found"
        check "the --quick base where git ls-files fails (an unreadable .git): none" \
            "$found" "none: git ls-files failed (no git work tree?)" ran
    fi
    chmod u+rwx "$ws/.git"

    # 11. The command's crates (step_cache_command_crates, which keys the
    # rust-version MSRV checks), on a synthetic workspace a, b, c of its own,
    # each reaching itself alone: `check --workspace --exclude c` keys a and
    # b, so an edit of c leaves its key and one of b changes it; with
    # `--exclude b` its crates are a and c and its key another; an --exclude
    # naming no member gives no key; `-p a -p c` keys a and c.
    local ws11="$dir/c11/ws" members11=""
    mkdir -p "$dir/c11/trees"
    for package in a b c; do
        mkdir -p "$ws11/$package/src"
        printf '[package]\nname = "%s"\n' "$package" >"$ws11/$package/Cargo.toml"
        printf '\n' >"$ws11/$package/src/lib.rs"
        printf '%s v0.1.0 (%s/%s)\n' "$package" "$ws11" "$package" >"$dir/c11/trees/$package"
        members11="$members11${members11:+, }{\"name\": \"$package\", \"version\": \"0.1.0\", \"source\": null,"
        members11="$members11 \"manifest_path\": \"$ws11/$package/Cargo.toml\","
        members11="$members11 \"targets\": [{\"kind\": [\"lib\"], \"src_path\": \"$ws11/$package/src/lib.rs\"}]}"
    done
    printf '[workspace]\nmembers = ["a", "b", "c"]\n' >"$ws11/Cargo.toml"
    printf '\n' >"$ws11/Cargo.lock"
    printf '{"packages": [%s]}\n' "$members11" >"$dir/c11/workspace.json"
    cp "$dir/cargo-stub" "$dir/c11/cargo-stub"
    found=$(
        exec 2>&1
        cd "$ws11" || exit 1
        # shellcheck disable=SC2034  # read by the functions under test
        WORKSPACE_METADATA=$(cat "$dir/c11/workspace.json")
        # shellcheck disable=SC2034
        WORKSPACE_CRATES=$'a\nb\nc'
        # shellcheck disable=SC2034
        CACHE_TRUSTED_BUILD_CODE=""
        STEP_CACHE_CARGO="$dir/c11/cargo-stub"
        RUST_REACH=()
        RUST_REACH_QUICK=()
        STEP_CACHE_TREE=()
        STEP_CACHE_NOTES=()
        STEP_CACHE_NOTED=()
        step_cache_init "$dir/c11/cache" "test-label" "tools 1" || exit 1
        step_cache_init_quick "quick base 1" || exit 1
        # The --quick key of a step running the command $@, as check_release.sh
        # takes a rust-version MSRV check's, or "none".
        command_key() {
            local crates step_inputs=() k
            if crates=$(step_cache_command_crates "$@") && step_cache_quick_inputs "$crates" step_inputs \
                && k=$(step_cache_key --quick "MSRV" "${step_inputs[@]}" -- "$@"); then
                printf "%s" "$k"
            else
                printf "none"
            fi
        }
        crates_of() { step_cache_command_crates "$@" || printf "none"; }
        compared() { if [[ $1 == none || $2 == none ]]; then printf "%s, %s" "$1" "$2"
            elif [ "$1" = "$2" ]; then printf "the same key"; else printf "another key"; fi; }
        first=$(command_key check --workspace --exclude c)
        printf "crates %s; " "$(crates_of check --workspace --exclude c)"
        printf "// edited\n" >>c/src/lib.rs
        printf "c edited: %s; " "$(compared "$first" "$(command_key check --workspace --exclude c)")"
        printf '\n' >c/src/lib.rs
        printf "// edited\n" >>b/src/lib.rs
        printf "b edited: %s; " "$(compared "$first" "$(command_key check --workspace --exclude c)")"
        printf '\n' >b/src/lib.rs
        printf -- "--exclude b: crates %s, %s; " "$(crates_of check --workspace --exclude b)" \
            "$(compared "$first" "$(command_key check --workspace --exclude b)")"
        printf -- "--exclude d: crates %s, key %s; " "$(crates_of check --workspace --exclude d)" \
            "$(command_key check --workspace --exclude d)"
        printf -- "-p a -p c: crates %s" "$(crates_of check -p a -p c)"
    ) || found="the shell ended: $found"
    check "the command's crates: --workspace but its --exclude ones, or its -p ones" "$found" \
        "crates a,b; c edited: the same key; b edited: another key; --exclude b: crates a,c, another key; --exclude d: crates none, key none; -p a -p c: crates a,c" ran
    [ "$failures" -eq 0 ]
}

# The command $@ with every GIT_* variable of the environment removed: a
# caller's GIT_DIR (a git hook's) would point git at another repository.
step_cache_without_git_environment() {
    local name
    local unset_arguments=()
    for name in $(compgen -e); do
        if [[ $name == GIT_* ]]; then
            unset_arguments+=(-u "$name")
        fi
    done
    env ${unset_arguments[@]+"${unset_arguments[@]}"} "$@"
}

# git, its arguments $@, without the caller's GIT_* variables.
step_cache_git() {
    step_cache_without_git_environment git "$@"
}

# The self-test's temporary directories: under the repository's target/.
STEP_CACHE_SELF_TEST_ROOT="$STEP_CACHE_ROOT_DIR/target"

if [[ "${BASH_SOURCE[0]}" == "$0" ]]; then
    set -euo pipefail
    case "${1:-}" in
        --self-test) step_cache_self_test ;;
        *) printf "usage: scripts/step_cache.sh --self-test   (sourced by scripts/check_release.sh)\n" >&2; exit 2 ;;
    esac
fi
