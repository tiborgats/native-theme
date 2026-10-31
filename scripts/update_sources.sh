#!/usr/bin/env bash
# The source cache (plan D1): every git entry of docs/facts/sources.toml checked out at its commit.
#
#   update_sources.sh [--seed DIR]... [--jobs N]   fetch each entry missing from the cache or not at its commit
#   update_sources.sh --verify                     offline: exit 1 unless every entry's checkout is at its commit,
#                                                  with its sparse folders and no change to a tracked file
#
# sources.toml is checked first with scripts/check_sources.py's form rules (not its citation completeness): on any
# problem nothing is fetched, verified or removed. Every path the script writes or removes is resolved and must lie
# directly inside the cache folder.
#
# Each entry goes to <cache>/<id>/: a shallow `git init` + `git fetch --depth 1 origin <commit>` + checkout, into
# <cache>/<id>.tmp first and renamed when its HEAD is the commit, so an interrupted run leaves no half checkout under
# the id. An entry with `sparse` folders is a partial clone (blobs fetched for those folders only, cone mode); a
# server that serves no partial clone (code.qt.io) sends the whole tree, which git accepts with a warning. A server
# that refuses a fetch by SHA is asked for the entry's tag instead, and the tag must peel to the commit. An entry
# whose checkout is already at its commit with the entry's sparse folders is skipped (a changed tracked file in it is
# reset to the commit first), so a run after an edit of sources.toml fetches only what changed. A folder in the cache
# that no entry names is reported, not removed.
#
# --seed DIR (repeatable): an entry without sparse folders whose commit is the HEAD of DIR/<repository name> is
# fetched from that checkout instead of its repository (its tracked files only; origin is still the entry's
# repository). --jobs N: fetches at once (default 4, at least 1).
#
# The SDK and distribution entries are not fetched: an SDK exists only on its runner image, a distribution snapshot
# is installed by the harness that uses it. The documentation snapshots, once sources.toml lists them, are fetched
# and their SHA-256 verified here too.
#
# Requires git and python3 (3.11, for tomllib); network access unless --verify.
set -euo pipefail

CACHE="${NATIVE_THEME_SOURCES:-/run/media/tibi/work/cargo-build/native-theme-sources}"
ROOT=$(git rev-parse --show-toplevel)
SOURCES="$ROOT/docs/facts/sources.toml"

usage() {
    sed -n '4,6p' "$0" | sed 's/^# \{0,1\}//' >&2
    exit 2
}

VERIFY=0
JOBS=4
SEEDS=()
while (($#)); do
    case "$1" in
        --verify) VERIFY=1 ;;
        --seed) (($# >= 2)) || usage; SEEDS+=("$2"); shift ;;
        --jobs) (($# >= 2)) && [[ "$2" =~ ^[1-9][0-9]*$ ]] || usage; JOBS="$2"; shift ;;
        *) usage ;;
    esac
    shift
done

# id, repository, commit, tag, sparse folders (space-separated) of every git entry, once the whole file passes
# check_sources.py's form rules (they keep tabs, newlines and spaces out of every field printed here)
if ! ENTRIES=$(python3 -I - "$ROOT/scripts" "$SOURCES" <<'EOF'
import sys, tomllib
sys.path.insert(0, sys.argv[1])
import check_sources
try:
    with open(sys.argv[2], "rb") as f:
        data = tomllib.load(f)
except (OSError, tomllib.TOMLDecodeError) as e:
    sys.exit(f"{sys.argv[2]}: {e}")
problems, _ = check_sources.check_form(data)
if problems:
    print("\n".join(problems), file=sys.stderr)
    sys.exit(f"{sys.argv[2]}: {len(problems)} problems (scripts/check_sources.py); nothing done")
for e in data.get("source", []):
    if e.get("kind") == "git":
        print("\t".join([e["id"], e["repository"], e["commit"], e["tag"], " ".join(e.get("sparse", []))]))
EOF
); then
    exit 2
fi

head_of() {
    git -C "$1" rev-parse --verify -q HEAD 2>/dev/null || true
}

# The sparse folders of a checkout, space-separated and sorted; empty for a complete one.
sparse_of() {
    if [[ "$(git -C "$1" config --bool core.sparseCheckout 2>/dev/null || true)" == "true" ]]; then
        git -C "$1" sparse-checkout list 2>/dev/null | sort | paste -sd' ' -
    fi
}

sorted_words() {
    tr ' ' '\n' <<<"$1" | sed '/^$/d' | sort | paste -sd' ' -
}

# Exits unless $1 resolves to a path directly inside the cache folder.
inside_cache() {
    local p
    p=$(realpath -m -- "$1")
    if [[ "$(dirname -- "$p")" != "$CACHE_REAL" || "$p" == "$CACHE_REAL" ]]; then
        echo "refused: $1 is not directly inside $CACHE_REAL" >&2
        return 1
    fi
}

verify() {
    local id url commit tag sparse dir head want got bad=0 n=0
    while IFS=$'\t' read -r id url commit tag sparse; do
        [[ -n "$id" ]] || continue
        n=$((n + 1))
        dir="$CACHE/$id"
        if ! inside_cache "$dir"; then
            bad=$((bad + 1))
            continue
        fi
        if [[ ! -d "$dir/.git" ]]; then
            echo "$id: no checkout at $dir"
            bad=$((bad + 1))
            continue
        fi
        head=$(head_of "$dir")
        want=$(sorted_words "$sparse")
        got=$(sparse_of "$dir")
        if [[ "$head" != "$commit" ]]; then
            echo "$id: HEAD ${head:-none}, want $commit"
            bad=$((bad + 1))
        elif [[ "$got" != "$want" ]]; then
            echo "$id: sparse folders '${got:-none (complete)}', want '${want:-none (complete)}'"
            bad=$((bad + 1))
        elif [[ -n "$(git -C "$dir" status --porcelain --untracked-files=no)" ]]; then
            echo "$id: tracked files changed in $dir"
            bad=$((bad + 1))
        fi
    done <<<"$ENTRIES"
    stale
    echo "# $n checkouts verified in $CACHE"
    echo "# $bad problems"
    ((bad == 0))
}

stale() {
    local d name
    for d in "$CACHE"/*/; do
        [[ -d "$d" ]] || continue
        name=$(basename "$d")
        if ! cut -f1 <<<"$ENTRIES" | grep -qxF -- "$name"; then
            echo "# not in sources.toml (left in place): $d"
        fi
    done
}

# seed_for <url> <commit>: prints a --seed checkout whose HEAD is <commit>, complete and not a partial clone.
seed_for() {
    local url="$1" commit="$2" name s cand
    name=$(basename "$url" .git)
    for s in "${SEEDS[@]}"; do
        cand=$(realpath -m -- "$s/$name")
        [[ -d "$cand/.git" ]] || continue
        [[ "$(head_of "$cand")" == "$commit" ]] || continue
        [[ "$(git -C "$cand" config --bool core.sparseCheckout || true)" != "true" ]] || continue
        [[ -z "$(git -C "$cand" config remote.origin.promisor || true)" ]] || continue
        printf "%s\n" "$cand"
        return 0
    done
    return 1
}

fetch_one() {
    local id="$1" url="$2" commit="$3" tag="$4" sparse="$5"
    local dir="$CACHE/$id" tmp="$CACHE/$id.tmp" log="$CACHE/$id.log" filter=() seed=""
    inside_cache "$dir" && inside_cache "$tmp" && inside_cache "$log" || return 1
    if [[ -d "$dir/.git" && "$(head_of "$dir")" == "$commit" &&
        "$(sparse_of "$dir")" == "$(sorted_words "$sparse")" ]]; then
        if [[ -n "$(git -C "$dir" status --porcelain --untracked-files=no)" ]]; then
            if git -C "$dir" reset -q --hard "$commit" >"$log" 2>&1 &&
                [[ -z "$(git -C "$dir" status --porcelain --untracked-files=no)" ]]; then
                rm -f -- "$log"
                echo "$id: tracked files reset to $commit"
                return 0
            fi
            echo "$id: tracked files changed and not reset (see $log); fetching again"
        else
            echo "$id: at $commit"
            return 0
        fi
    fi
    rm -rf -- "$tmp" || { echo "$id: cannot remove $tmp"; return 1; }
    mkdir -p -- "$tmp" || { echo "$id: cannot create $tmp"; return 1; }
    {
        git -C "$tmp" init -q &&
            git -C "$tmp" remote add --end-of-options origin "$url"
    } >"$log" 2>&1 || { echo "$id: git init failed (see $log)"; return 1; }
    if [[ -n "$sparse" ]]; then
        # shellcheck disable=SC2086 # the folders are space-separated on purpose (check_sources.py allows no spaces)
        git -C "$tmp" sparse-checkout set --cone --end-of-options $sparse >>"$log" 2>&1 ||
            { echo "$id: sparse-checkout failed (see $log)"; return 1; }
        filter=(--filter=blob:none)
    elif ((${#SEEDS[@]})) && seed=$(seed_for "$url" "$commit"); then
        if git -C "$tmp" fetch -q --depth 1 --end-of-options "file://$seed" "$commit" >>"$log" 2>&1; then
            echo "$id: seeded from $seed"
        else
            echo "$id: seed $seed failed, fetching from $url"
            seed=""
        fi
    fi
    if [[ -z "$seed" ]] &&
        ! git -C "$tmp" fetch -q --depth 1 "${filter[@]}" --end-of-options origin "$commit" >>"$log" 2>&1; then
        if [[ "$tag" == "commit" ]] ||
            ! git -C "$tmp" fetch -q --depth 1 "${filter[@]}" --end-of-options origin "refs/tags/$tag" >>"$log" 2>&1
        then
            echo "$id: fetch failed (see $log)"
            return 1
        fi
    fi
    if ! git -C "$tmp" -c advice.detachedHead=false checkout -q --detach "$commit" >>"$log" 2>&1; then
        echo "$id: checkout of $commit failed (see $log)"
        return 1
    fi
    if [[ "$(head_of "$tmp")" != "$commit" ]]; then
        echo "$id: HEAD $(head_of "$tmp"), want $commit"
        return 1
    fi
    rm -rf -- "$dir" || { echo "$id: cannot remove the old $dir"; return 1; }
    mv -- "$tmp" "$dir" || { echo "$id: cannot rename $tmp to $dir"; return 1; }
    rm -f -- "$log"
    echo "$id: fetched $commit"
}

if ((VERIFY)); then
    if [[ ! -d "$CACHE" ]]; then
        echo "no source cache at $CACHE" >&2
        exit 1
    fi
    CACHE_REAL=$(realpath -- "$CACHE")
    verify
    exit
fi

mkdir -p -- "$CACHE"
CACHE_REAL=$(realpath -- "$CACHE")
if [[ "$CACHE_REAL" == "/" ]]; then
    echo "refused: the cache folder is /" >&2
    exit 2
fi
RESULTS=$(mktemp -d "$CACHE/.run.XXXXXX")
trap 'rm -rf -- "$RESULTS"' EXIT
n=0
while IFS=$'\t' read -r id url commit tag sparse; do
    [[ -n "$id" ]] || continue
    n=$((n + 1))
    while (($(jobs -rp | wc -l) >= JOBS)); do
        wait -n || true
    done
    (fetch_one "$id" "$url" "$commit" "$tag" "$sparse" && touch "$RESULTS/ok.$n" || touch "$RESULTS/fail.$n") \
        </dev/null &
done <<<"$ENTRIES"
wait
stale
OK=$(find "$RESULTS" -name 'ok.*' | wc -l)
FAIL=$(find "$RESULTS" -name 'fail.*' | wc -l)
echo "# $OK entries at their commit, $FAIL failed"
((FAIL == 0))
