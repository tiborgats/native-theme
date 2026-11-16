# CI's MSRV job (.github/workflows/ci.yml, `msrv`), sourced by
# scripts/check_release.sh: a lint can fire at the MSRV and not on stable, so
# the job denies warnings, and it runs every command below, each on its
# toolchain: the workspace's rust-version, then the connectors' (the job's
# second toolchain, read from ci.yml). --commit and --quick run them, --quick
# the two at the workspace's rust-version only where anything their key
# follows changed (their crates' reach and included files, the files outside
# the crates but QUICK_UNREAD, the tools, the environment, their toolchain);
# the full run leaves them to --commit and CI.
#
# Sourcing this file defines the six checks' arguments and the functions
# below and runs nothing. msrv_toolchains reads the two toolchains;
# msrv_ci_check checks that the six checks are the job's, so a change of
# ci.yml this file does not follow fails locally; msrv_checks runs the
# section, with check_release.sh's run_check and print_fail.
#
# The workspace's rust-version builds in a folder of its own: Cargo at that
# version reads no `build.build-dir`, so its two checks get `--target-dir
# <build_directory>/msrv-<major.minor>` (target/msrv-<major.minor> where
# cargo metadata reports no build_directory), the one argument this file adds
# to the job's. An EXIT trap, set when the section starts, prints the
# folder's `du -sb` and deletes it however the run ends; a folder a killed run
# left behind is removed before the first check.

# The six checks' arguments after `cargo <toolchain> check`, in the job's
# order: the first two on the workspace's rust-version, the other four on
# the connectors' toolchain.
MSRV_CHECK_ARGS_1=(--workspace --exclude native-theme-gpui --exclude native-theme-egui --exclude native-theme-egui-widgets --all-features --locked)
MSRV_CHECK_ARGS_2=(-p native-theme --locked)
MSRV_CHECK_ARGS_3=(-p native-theme-egui -p native-theme-egui-widgets --all-features --locked)
MSRV_CHECK_ARGS_4=(-p native-theme-gpui --locked)
MSRV_CHECK_ARGS_5=(-p native-theme-gpui --features native,watch --locked)
MSRV_CHECK_ARGS_6=(-p native-theme-gpui --features gpui-fast,native,watch --locked)

# Set only while a --quick-keyed check runs (msrv_check). One the environment
# passes in would make every step before the first check an MSRV one to
# quick_key_kind, and, exported, put the checks' own setting of it into the
# environment their --quick key hashes: it is unset, export and all.
unset MSRV_QUICK_TOOLCHAIN
MSRV_QUICK_TOOLCHAIN=""

# The workspace's rust-version, from the root Cargo.toml.
msrv_rust_version() {
    grep -E '^rust-version\s*=' Cargo.toml | sed -E 's/.*"([^"]+)".*/\1/'
}

# The `msrv` job of the ci.yml at $1: its header line up to, not including,
# the next job's header line, or to the file's end.
msrv_job() {
    awk '/^  msrv:$/{f=1;print;next} f&&/^  [a-z-]*:$/{exit} f' "$1"
}

# Sets MSRV_TOOLCHAIN and CONNECTOR_TOOLCHAIN ("+<version>"). Where a version
# reads empty, sets MSRV_TOOLCHAINS_ERROR, naming the file, and returns 1.
msrv_toolchains() {
    local version
    MSRV_TOOLCHAINS_ERROR=""
    version=$(msrv_rust_version)
    if [ -z "$version" ]; then
        MSRV_TOOLCHAINS_ERROR="Cargo.toml: no rust-version read"
        return 1
    fi
    MSRV_TOOLCHAIN="+$version"
    version=$(msrv_job .github/workflows/ci.yml \
        | sed -n 's/^      - uses: dtolnay\/rust-toolchain@//p' | sed -n 2p)
    if [ -z "$version" ]; then
        MSRV_TOOLCHAINS_ERROR=".github/workflows/ci.yml: no version of the msrv job's second rust-toolchain read"
        return 1
    fi
    CONNECTOR_TOOLCHAIN="+$version"
}

# The lines given, each quoted, comma-separated; "nothing" where none is.
msrv_lines() {
    if [ "$#" -eq 0 ]; then
        printf 'nothing'
        return
    fi
    local first=1 line
    for line in "$@"; do
        [ "$first" -eq 1 ] || printf ', '
        printf "'%s'" "$line"
        first=0
    done
}

# Checks the ci.yml at $1: its `msrv` job runs exactly the six checks above,
# each on its toolchain (the first from Cargo.toml's rust-version, the second
# read from the job), under RUSTFLAGS=-Dwarnings and nothing else that would
# change them. The job accepts only the line shapes it has: runs-on, a
# job-level env holding RUSTFLAGS alone, steps, the checkout, the toolchain
# steps, `cargo check` steps, the system-dependency step, comments and blank
# lines. The file's top-level keys are name, on, env and jobs, and its env
# holds CARGO_TERM_COLOR alone. Returns 0, or prints each difference and
# returns 1.
msrv_ci_check() {
    local file="$1"
    local -a problems=()
    local rust_version
    rust_version=$(msrv_rust_version)
    if [ -z "$rust_version" ]; then
        problems+=("Cargo.toml: no rust-version read")
    fi

    local line key
    local -a top_keys=()
    local in_env=0
    local -a workflow_env=()
    while IFS= read -r line || [ -n "$line" ]; do
        if [[ $line =~ ^([^[:space:]#][^:]*): ]]; then
            key="${BASH_REMATCH[1]}"
            top_keys+=("$key")
            in_env=0
            [ "$key" = "env" ] && in_env=1
        elif [ "$in_env" -eq 1 ] && [[ $line =~ ^[[:space:]]+[^[:space:]#] ]]; then
            workflow_env+=("$line")
        fi
    done <"$file"
    if [ "${top_keys[*]-}" != "name on env jobs" ]; then
        problems+=("$file: the top-level keys are '${top_keys[*]-}', not 'name on env jobs'")
    fi
    if [ "${#workflow_env[@]}" -ne 1 ] || [ "${workflow_env[0]}" != "  CARGO_TERM_COLOR: always" ]; then
        problems+=("$file: the workflow's env holds $(msrv_lines ${workflow_env[@]+"${workflow_env[@]}"}), not '  CARGO_TERM_COLOR: always' alone")
    fi

    local job
    job=$(msrv_job "$file")
    if [ -z "$job" ]; then
        problems+=("$file: no msrv job (a line '  msrv:')")
        printf "%s\n" "${problems[@]}"
        return 1
    fi

    local -a toolchains=()
    local -a runs=()
    local -a env_entries=()
    local -a words=()
    local toolchain="" previous="" job_env=0 n=0
    while IFS= read -r line; do
        n=$((n + 1))
        if [ "$n" -eq 1 ]; then
            continue
        fi
        if [ "$previous" = "dependencies" ]; then
            previous=""
            if [[ $line =~ ^\ {8}run:\ sudo\ apt-get\  ]]; then
                continue
            fi
            problems+=("$file: the msrv job's system-dependency step is not followed by its 'run: sudo apt-get …' line: '$line'")
            continue
        fi
        if [ "$previous" = "env" ]; then
            if [[ $line =~ ^\ {6}[^[:space:]#-] ]]; then
                env_entries+=("$line")
                continue
            fi
            previous=""
        fi
        if [[ $line =~ ^[[:space:]]*$ ]] || [[ $line =~ ^[[:space:]]*# ]]; then
            continue
        elif [ "$line" = "    runs-on: ubuntu-latest" ] || [ "$line" = "    steps:" ]; then
            continue
        elif [ "$line" = "    env:" ]; then
            job_env=$((job_env + 1))
            previous="env"
        elif [[ $line =~ ^\ {6}-\ uses:\ actions/checkout@[^[:space:]]+$ ]]; then
            continue
        elif [[ $line =~ ^\ {6}-\ uses:\ dtolnay/rust-toolchain@(.*)$ ]]; then
            toolchain="${BASH_REMATCH[1]}"
            toolchains+=("$toolchain")
            if [ -z "$toolchain" ]; then
                problems+=("$file: the msrv job's toolchain step names no version: '$line'")
            fi
        elif [[ $line =~ ^\ {6}-\ run:\ cargo\ check\ (.*)$ ]]; then
            read -ra words <<<"${BASH_REMATCH[1]}"
            runs+=("+$toolchain check ${words[*]}")
        elif [ "$line" = "      - name: Install system dependencies (gpui)" ]; then
            previous="dependencies"
        else
            problems+=("$file: the msrv job's line '$line' is of no shape the job is checked for")
        fi
    done <<<"$job"
    if [ "$previous" = "dependencies" ]; then
        problems+=("$file: the msrv job's system-dependency step has no 'run: sudo apt-get …' line")
    fi
    if [ "$job_env" -ne 1 ] || [ "${#env_entries[@]}" -ne 1 ] \
        || [ "${env_entries[0]-}" != "      RUSTFLAGS: -Dwarnings" ]; then
        problems+=("$file: the msrv job's env holds $(msrv_lines ${env_entries[@]+"${env_entries[@]}"}), not '      RUSTFLAGS: -Dwarnings' alone")
    fi

    local connector="${toolchains[1]-}"
    if [ -z "$connector" ]; then
        problems+=("$file: the msrv job's second toolchain has no version")
    fi
    local -a expected=(
        "+$rust_version check ${MSRV_CHECK_ARGS_1[*]}"
        "+$rust_version check ${MSRV_CHECK_ARGS_2[*]}"
        "+$connector check ${MSRV_CHECK_ARGS_3[*]}"
        "+$connector check ${MSRV_CHECK_ARGS_4[*]}"
        "+$connector check ${MSRV_CHECK_ARGS_5[*]}"
        "+$connector check ${MSRV_CHECK_ARGS_6[*]}"
    )
    if [ "${#runs[@]}" -ne "${#expected[@]}" ]; then
        problems+=("$file: the msrv job runs ${#runs[@]} cargo check line(s), scripts/msrv_checks.sh ${#expected[@]}")
    fi
    local i
    for i in "${!expected[@]}"; do
        if [ "${runs[$i]-}" != "${expected[$i]}" ]; then
            problems+=("$file: the msrv job's cargo check $((i + 1)) is 'cargo ${runs[$i]-(none)}', scripts/msrv_checks.sh runs 'cargo ${expected[$i]}'")
        fi
    done
    for ((i = ${#expected[@]}; i < ${#runs[@]}; i++)); do
        problems+=("$file: the msrv job's cargo check $((i + 1)), 'cargo ${runs[$i]}', is not one of scripts/msrv_checks.sh's")
    done

    if [ "${#problems[@]}" -gt 0 ]; then
        printf "%s\n" "${problems[@]}"
        return 1
    fi
}

# Reads `rustup toolchain list` once, and the host from `rustc -vV`, for
# msrv_toolchain_installed. Where either fails, MSRV_TOOLCHAIN_LIST_ERROR
# holds the reason, which the first check then fails with.
msrv_toolchain_list() {
    MSRV_TOOLCHAIN_LIST=""
    MSRV_TOOLCHAIN_LIST_ERROR=""
    MSRV_HOST=""
    local listing
    if ! command -v rustup >/dev/null 2>&1; then
        MSRV_TOOLCHAIN_LIST_ERROR="rustup not found: the MSRV toolchains cannot be checked"
        return 0
    fi
    MSRV_HOST=$(rustc -vV 2>/dev/null | sed -n 's/^host: //p')
    if [ -z "$MSRV_HOST" ]; then
        MSRV_TOOLCHAIN_LIST_ERROR="rustc -vV named no host: the MSRV toolchains cannot be checked"
        return 0
    fi
    if ! listing=$(rustup toolchain list 2>&1); then
        MSRV_TOOLCHAIN_LIST_ERROR="rustup toolchain list failed: $listing"
        return 0
    fi
    MSRV_TOOLCHAIN_LIST=$(awk '{print $1}' <<<"$listing")
}

# Returns 0 where the toolchain $1 ("+<version>") is installed: the first
# field of a line of the listing is exactly <version>-<host> (the listing
# also holds names a prefix match would confuse, such as 1.95-<host> beside
# 1.95.0-<host>). Otherwise prints the reason and returns 1.
msrv_toolchain_installed() {
    local version="${1#+}"
    if [ -n "$MSRV_TOOLCHAIN_LIST_ERROR" ]; then
        printf "%s\n" "$MSRV_TOOLCHAIN_LIST_ERROR"
        return 1
    fi
    if ! grep -qxF "$version-$MSRV_HOST" <<<"$MSRV_TOOLCHAIN_LIST"; then
        printf "toolchain %s is not installed (rustup toolchain install %s)\n" "$version" "$version"
        return 1
    fi
}

# One check: $1 = toolchain, $2 = description, $3... = the arguments after
# `check`; `--quick-keyed` first marks a check --quick keys (the two at the
# workspace's rust-version): MSRV_QUICK_TOOLCHAIN holds its toolchain while
# it runs, which check_release.sh's quick_key_kind and quick_step_base read.
# A toolchain the listing lacks fails it without running cargo, and before
# run_check computes a rust-version check's --quick key, which runs `rustc
# <toolchain> -vV` (quick_step_base): rustup may install a toolchain it is
# asked for. The check denies warnings through RUSTFLAGS, as the job does,
# with CARGO_ENCODED_RUSTFLAGS unset, which Cargo would read in its place.
msrv_check() {
    local quick_keyed=""
    if [ "$1" = "--quick-keyed" ]; then
        quick_keyed=1
        shift
    fi
    local toolchain="$1"
    local description="$2"
    shift 2
    local reason
    if ! reason=$(msrv_toolchain_installed "$toolchain"); then
        print_fail "$description"
        printf "${RED}   %s${NC}\n" "$reason" >&2
        exit 1
    fi
    MSRV_QUICK_TOOLCHAIN=""
    if [[ $quick_keyed ]]; then
        MSRV_QUICK_TOOLCHAIN="$toolchain"
    fi
    run_check "$description" env -u CARGO_ENCODED_RUSTFLAGS RUSTFLAGS=-Dwarnings cargo "$toolchain" check "$@"
    MSRV_QUICK_TOOLCHAIN=""
}

# The section: the toolchains, the folder and its trap, the check against
# ci.yml, then the six checks.
msrv_checks() {
    if ! msrv_toolchains; then
        print_fail "MSRV toolchains not read: $MSRV_TOOLCHAINS_ERROR"
        exit 1
    fi
    local build_directory
    build_directory=$(jq -r '.build_directory // empty' <<<"$WORKSPACE_METADATA")
    MSRV_TARGET_DIR="${build_directory:-target}/msrv-$(cut -d. -f1-2 <<<"${MSRV_TOOLCHAIN#+}")"
    local quoted
    quoted=$(printf %q "$MSRV_TARGET_DIR")
    # shellcheck disable=SC2064  # the path is fixed when the trap is set
    trap "[ -d $quoted ] && du -sb $quoted || true; rm -rf $quoted || true" EXIT
    if ! rm -rf "$MSRV_TARGET_DIR"; then
        print_fail "MSRV: the leftover $MSRV_TARGET_DIR could not be removed"
        exit 1
    fi

    run_check "MSRV: the six checks are ci.yml's msrv job" msrv_ci_check .github/workflows/ci.yml
    msrv_toolchain_list
    msrv_check --quick-keyed "$MSRV_TOOLCHAIN" \
        "MSRV $MSRV_TOOLCHAIN, -D warnings (the workspace but the gpui and egui crates, all features)" \
        "${MSRV_CHECK_ARGS_1[@]}" --target-dir "$MSRV_TARGET_DIR"
    msrv_check --quick-keyed "$MSRV_TOOLCHAIN" "MSRV $MSRV_TOOLCHAIN, -D warnings (native-theme, default features)" \
        "${MSRV_CHECK_ARGS_2[@]}" --target-dir "$MSRV_TARGET_DIR"
    msrv_check "$CONNECTOR_TOOLCHAIN" "MSRV $CONNECTOR_TOOLCHAIN, -D warnings (the egui crates, all features)" \
        "${MSRV_CHECK_ARGS_3[@]}"
    msrv_check "$CONNECTOR_TOOLCHAIN" "MSRV $CONNECTOR_TOOLCHAIN, -D warnings (native-theme-gpui)" \
        "${MSRV_CHECK_ARGS_4[@]}"
    msrv_check "$CONNECTOR_TOOLCHAIN" "MSRV $CONNECTOR_TOOLCHAIN, -D warnings (native-theme-gpui, native, watch)" \
        "${MSRV_CHECK_ARGS_5[@]}"
    msrv_check "$CONNECTOR_TOOLCHAIN" "MSRV $CONNECTOR_TOOLCHAIN, -D warnings (native-theme-gpui, gpui-fast, native, watch)" \
        "${MSRV_CHECK_ARGS_6[@]}"
}
