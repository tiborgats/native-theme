#!/usr/bin/env bash
# Visual-asset provenance stamp: docs/assets/PROVENANCE.toml
#
# The screenshots and GIFs under native-theme/docs/assets and
# connectors/*/docs/assets are captured by scripts/pre-release.sh from a
# checked-out commit. This script records which sources they came from and
# lets the release gates verify that claim without any git history:
#
#   asset-stamp.sh write         write the stamp for HEAD (pre-release.sh
#                                calls it after the captures succeeded)
#   asset-stamp.sh check         exit 0 if HEAD's sources match the stamp;
#                                exit 1 with a message on stdout if the stamp
#                                is missing or the sources differ
#   asset-stamp.sh verify-clean  exit 1 if a stamped path has uncommitted
#                                changes (captures must run on HEAD's sources)
#   asset-stamp.sh hash          print the sources hash of HEAD
#
# The hash covers the git object ids of every path that feeds the showcases:
# crate manifests and sources, presets, icon bundles, the lockfile, the
# capture scripts and the screenshots workflow. `git rev-parse HEAD:<path>`
# yields a tree or blob id, so `check` works on a depth-1 checkout.
set -euo pipefail

ROOT=$(git rev-parse --show-toplevel)
STAMP_REL="docs/assets/PROVENANCE.toml"
STAMP="$ROOT/$STAMP_REL"

SOURCE_PATHS=(
    Cargo.toml
    Cargo.lock
    native-theme/Cargo.toml
    native-theme/build.rs
    native-theme/src
    native-theme/icons
    native-theme-derive/Cargo.toml
    native-theme-derive/src
    native-theme-build/Cargo.toml
    native-theme-build/src
    connectors/native-theme-gpui/Cargo.toml
    connectors/native-theme-gpui/src
    connectors/native-theme-gpui/examples
    connectors/native-theme-iced/Cargo.toml
    connectors/native-theme-iced/src
    connectors/native-theme-iced/examples
    scripts/generate_screenshots.sh
    scripts/generate_gpui_screenshots.sh
    scripts/generate_theme_switching_gif.sh
    scripts/generate_gifs.py
    .github/workflows/screenshots.yml
)

sources_hash() {
    local rev="${1:-HEAD}" p id ids=""
    for p in "${SOURCE_PATHS[@]}"; do
        id=$(git -C "$ROOT" rev-parse --verify --quiet "$rev:$p") || {
            echo "asset-stamp: $p not found at $rev" >&2
            return 1
        }
        ids+="$id"$'\n'
    done
    printf '%s' "$ids" | sha256sum | cut -d' ' -f1
}

stamp_field() {
    sed -n "s/^$1 = \"\(.*\)\"/\1/p" "$STAMP" | head -1
}

workspace_version() {
    grep -E '^version\s*=' "$ROOT/Cargo.toml" | head -1 | sed -E 's/.*"([^"]+)".*/\1/'
}

cmd_write() {
    local hash version commit today
    hash=$(sources_hash HEAD)
    version=$(workspace_version)
    commit=$(git -C "$ROOT" rev-parse HEAD)
    today=$(date -u +%Y-%m-%d)
    mkdir -p "$(dirname "$STAMP")"
    cat > "$STAMP" <<EOF
# Provenance of the visual assets (screenshots and GIFs under
# native-theme/docs/assets and connectors/*/docs/assets).
# Written by scripts/asset-stamp.sh at the end of scripts/pre-release.sh;
# read by pre-release-check.sh and the crates.io workflow's CI gate.
# Do not edit by hand.

# Workspace version and commit the assets were captured from.
version = "$version"
commit = "$commit"
generated = "$today"

# SHA-256 over the git object ids of every path that feeds the showcases
# (SOURCE_PATHS in scripts/asset-stamp.sh). The gates recompute it at HEAD;
# a difference means the assets were captured from other sources.
sources = "$hash"
EOF
    echo "wrote $STAMP_REL (version $version, commit ${commit:0:7}, sources ${hash:0:12})"
}

cmd_check() {
    if [ ! -f "$STAMP" ]; then
        echo "no $STAMP_REL: the visual assets carry no provenance; run ./scripts/pre-release.sh"
        return 1
    fi
    local recorded version commit generated current
    recorded=$(stamp_field sources)
    version=$(stamp_field version)
    commit=$(stamp_field commit)
    generated=$(stamp_field generated)
    current=$(sources_hash HEAD)
    if [ "$recorded" = "$current" ]; then
        echo "visual assets captured from HEAD's sources (v$version, ${commit:0:7}, $generated)"
        return 0
    fi
    echo "visual assets are stale: captured from other sources (v$version, ${commit:0:7}, $generated); run ./scripts/pre-release.sh"
    return 1
}

cmd_verify_clean() {
    local dirty
    dirty=$(git -C "$ROOT" status --porcelain -- "${SOURCE_PATHS[@]}")
    if [ -n "$dirty" ]; then
        echo "uncommitted changes in stamped source paths:"
        printf '%s\n' "$dirty" | head -10
        return 1
    fi
    echo "stamped source paths are clean"
}

case "${1:-}" in
    write)        cmd_write ;;
    check)        cmd_check ;;
    verify-clean) cmd_verify_clean ;;
    hash)         sources_hash HEAD ;;
    *)
        echo "usage: $0 {write|check|verify-clean|hash}" >&2
        exit 2
        ;;
esac
