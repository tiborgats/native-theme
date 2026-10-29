#!/usr/bin/env bash
# The source cache (plan D1): every git entry of docs/facts/sources.toml checked out at its commit.
#
#   update_sources.sh [--seed DIR]... [--jobs N]   fetch each entry missing from the cache or not at its commit
#   update_sources.sh --verify                     offline: exit 1 unless every entry's checkout is at its commit,
#                                                  with no change to a tracked file
#
# Each entry goes to <cache>/<id>/: a shallow `git init` + `git fetch --depth 1 origin <commit>` + checkout, into
# <cache>/<id>.tmp first and renamed when its HEAD is the commit, so an interrupted run leaves no half checkout under
# the id. An entry with `sparse` folders is a partial clone (blobs fetched for those folders only, cone mode); a
# server that serves no partial clone (code.qt.io) sends the whole tree, which git accepts with a warning. A server
# that refuses a fetch by SHA is asked for the entry's tag instead, and the tag must peel to the commit. An entry
# whose checkout is already at its commit is skipped, so a run after an edit of sources.toml fetches only what
# changed. A folder in the cache that no entry names is reported, not removed.
#
# --seed DIR (repeatable): before fetching an entry without sparse folders, a clean, complete checkout at its commit
# in DIR/<repository name> is copied instead (its origin set to the entry's repository). --jobs N: fetches at once
# (default 4).
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
        --jobs) (($# >= 2)) || usage; JOBS="$2"; shift ;;
        *) usage ;;
    esac
    shift
done

# id, repository, commit, tag, sparse folders (space-separated) of every git entry
ENTRIES=$(python3 - "$SOURCES" <<'EOF'
import sys, tomllib
with open(sys.argv[1], "rb") as f:
    data = tomllib.load(f)
for e in data.get("source", []):
    if e.get("kind") == "git":
        print("\t".join([e["id"], e["repository"], e["commit"], e["tag"], " ".join(e.get("sparse", []))]))
EOF
)

head_of() {
    git -C "$1" rev-parse --verify -q HEAD 2>/dev/null || true
}

verify() {
    local id url commit tag sparse dir head bad=0 n=0
    while IFS=$'\t' read -r id url commit tag sparse; do
        n=$((n + 1))
        dir="$CACHE/$id"
        if [[ ! -d "$dir/.git" ]]; then
            echo "$id: no checkout at $dir"
            bad=$((bad + 1))
            continue
        fi
        head=$(head_of "$dir")
        if [[ "$head" != "$commit" ]]; then
            echo "$id: HEAD ${head:-none}, want $commit"
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
        if ! cut -f1 <<<"$ENTRIES" | grep -qxF "$name"; then
            echo "# not in sources.toml (left in place): $d"
        fi
    done
}

seed_from() {
    # seed_from <id> <url> <commit> <tmp>: copy a matching checkout from a --seed folder into <tmp>
    local id="$1" url="$2" commit="$3" tmp="$4" name s cand
    name=$(basename "$url" .git)
    for s in "${SEEDS[@]}"; do
        cand="$s/$name"
        [[ -d "$cand/.git" ]] || continue
        [[ "$(head_of "$cand")" == "$commit" ]] || continue
        [[ "$(git -C "$cand" config --bool core.sparseCheckout || true)" != "true" ]] || continue
        [[ -z "$(git -C "$cand" config remote.origin.promisor || true)" ]] || continue
        [[ -z "$(git -C "$cand" status --porcelain --untracked-files=no)" ]] || continue
        cp -a "$cand" "$tmp"
        git -C "$tmp" remote set-url origin "$url" 2>/dev/null || git -C "$tmp" remote add origin "$url"
        echo "$id: seeded from $cand"
        return 0
    done
    return 1
}

fetch_one() {
    local id="$1" url="$2" commit="$3" tag="$4" sparse="$5"
    local dir="$CACHE/$id" tmp="$CACHE/$id.tmp" log="$CACHE/$id.log" filter=()
    if [[ -d "$dir/.git" && "$(head_of "$dir")" == "$commit" ]]; then
        echo "$id: at $commit"
        return 0
    fi
    rm -rf "$tmp"
    if [[ -z "$sparse" && ${#SEEDS[@]} -gt 0 ]] && seed_from "$id" "$url" "$commit" "$tmp"; then
        :
    else
        rm -rf "$tmp"
        mkdir -p "$tmp"
        git -C "$tmp" init -q
        git -C "$tmp" remote add origin "$url"
        if [[ -n "$sparse" ]]; then
            # shellcheck disable=SC2086 # the folders are space-separated on purpose
            git -C "$tmp" sparse-checkout set --cone $sparse
            filter=(--filter=blob:none)
        fi
        if ! git -C "$tmp" fetch -q --depth 1 "${filter[@]}" origin "$commit" >"$log" 2>&1; then
            if [[ "$tag" == "commit" ]] ||
                ! git -C "$tmp" fetch -q --depth 1 "${filter[@]}" origin "refs/tags/$tag" >>"$log" 2>&1; then
                echo "$id: fetch failed (see $log)"
                return 1
            fi
        fi
        if ! git -C "$tmp" -c advice.detachedHead=false checkout -q --detach "$commit" >>"$log" 2>&1; then
            echo "$id: checkout of $commit failed (see $log)"
            return 1
        fi
    fi
    if [[ "$(head_of "$tmp")" != "$commit" ]]; then
        echo "$id: HEAD $(head_of "$tmp"), want $commit"
        return 1
    fi
    rm -rf "$dir"
    mv "$tmp" "$dir"
    rm -f "$log"
    echo "$id: fetched $commit"
}

if ((VERIFY)); then
    if [[ ! -d "$CACHE" ]]; then
        echo "no source cache at $CACHE" >&2
        exit 1
    fi
    verify
    exit
fi

mkdir -p "$CACHE"
RESULTS=$(mktemp -d "$CACHE/.run.XXXXXX")
trap 'rm -rf "$RESULTS"' EXIT
while IFS=$'\t' read -r id url commit tag sparse; do
    while (($(jobs -rp | wc -l) >= JOBS)); do
        wait -n || true
    done
    (fetch_one "$id" "$url" "$commit" "$tag" "$sparse" && touch "$RESULTS/ok.$id" || touch "$RESULTS/fail.$id") \
        </dev/null &
done <<<"$ENTRIES"
wait
stale
OK=$(find "$RESULTS" -name 'ok.*' | wc -l)
FAIL=$(find "$RESULTS" -name 'fail.*' | wc -l)
echo "# $OK entries at their commit, $FAIL failed"
((FAIL == 0))
