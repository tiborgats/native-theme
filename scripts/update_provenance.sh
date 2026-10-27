#!/usr/bin/env bash
# Visual-asset provenance stamp: docs/assets/PROVENANCE.toml
#
# The screenshots and GIFs under native-theme/docs/assets and
# connectors/*/docs/assets are captured by scripts/generate_assets_release.sh from a
# checked-out commit. This script records which sources they came from and
# lets the release gates verify that claim without any git history:
#
#   update_provenance.sh write [DIR]   write the stamp for HEAD (generate_assets_release.sh
#                                calls it after the captures succeeded); DIR holds
#                                the CI captures' OS-version sidecars
#                                (<toolkit>-macos.txt, <toolkit>-windows.txt)
#   update_provenance.sh check         exit 0 if HEAD's sources match the stamp;
#                                exit 1 with a message on stdout if the stamp
#                                is missing or the sources differ
#   update_provenance.sh verify-clean  exit 1 if a stamped path has uncommitted
#                                changes (captures must run on HEAD's sources)
#   update_provenance.sh hash          print the sources hash of HEAD
#
# The hash covers the git object ids of every path that feeds the showcases:
# crate manifests and sources, presets, icon bundles, the egui connector's
# mapping.toml, the showcases' shared crate (tools/showcase-support), the
# lockfile, the capture scripts and the screenshots workflow. `git rev-parse HEAD:<path>` yields a tree or blob id, so `check`
# works on a depth-1 checkout.
#
# The stamp also records the OS versions of the CI runners that captured the
# macOS and Windows screenshots (`macos_version`, `windows_version`), each
# from the sidecars the screenshots workflow uploads with its captures:
# `sw_vers -productVersion` on macOS, `[System.Environment]::OSVersion.Version`
# on Windows. One job per toolkit captures each platform, so a field lists
# every distinct version its jobs ran on; it is empty when no sidecar was
# given. No gate compares them with a preset's range yet.
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
    connectors/native-theme-egui/Cargo.toml
    connectors/native-theme-egui/src
    connectors/native-theme-egui/examples
    connectors/native-theme-egui/mapping.toml
    connectors/native-theme-egui-widgets/Cargo.toml
    connectors/native-theme-egui-widgets/src
    tools/showcase-support/Cargo.toml
    tools/showcase-support/src
    scripts/generate_screenshots_iced.sh
    scripts/generate_screenshots_gpui.sh
    scripts/generate_screenshots_egui.sh
    scripts/generate_gifs_theme_switching.sh
    scripts/capture_window.sh
    scripts/generate_gifs_spinners.py
    .github/workflows/screenshots.yml
)

sources_hash() {
    local rev="${1:-HEAD}" p id ids=""
    for p in "${SOURCE_PATHS[@]}"; do
        id=$(git -C "$ROOT" rev-parse --verify --quiet "$rev:$p") || {
            echo "update_provenance: $p not found at $rev" >&2
            return 1
        }
        ids+="$id"$'\n'
    done
    printf '%s' "$ids" | sha256sum | cut -d' ' -f1
}

stamp_field() {
    sed -n "s/^$1 = \"\(.*\)\"/\1/p" "$STAMP" | head -1
}

# The distinct OS versions in DIR's sidecars for one platform, comma-separated;
# nothing when DIR is empty or holds none. A line that is not a dotted version
# is reported on stderr and left out.
os_versions() {
    local dir="$1" platform="$2" f line
    local -a versions=()
    [ -n "$dir" ] || return 0
    for f in "$dir"/*-"$platform".txt; do
        [ -f "$f" ] || continue
        while IFS= read -r line || [ -n "$line" ]; do
            line="${line%$'\r'}"
            [ -n "$line" ] || continue
            if [[ "$line" =~ ^[0-9]+(\.[0-9]+)*$ ]]; then
                versions+=("$line")
            else
                echo "update_provenance: $f: not an OS version: $line" >&2
            fi
        done < "$f"
    done
    [ "${#versions[@]}" -gt 0 ] || return 0
    printf '%s\n' "${versions[@]}" | sort -uV | paste -sd ',' - | sed 's/,/, /g'
}

workspace_version() {
    grep -E '^version\s*=' "$ROOT/Cargo.toml" | head -1 | sed -E 's/.*"([^"]+)".*/\1/'
}

cmd_write() {
    local sidecars="${1:-}" hash version commit today macos_version windows_version
    if [ -n "$sidecars" ] && [ ! -d "$sidecars" ]; then
        echo "update_provenance: $sidecars is not a directory" >&2
        return 1
    fi
    macos_version=$(os_versions "$sidecars" macos)
    windows_version=$(os_versions "$sidecars" windows)
    hash=$(sources_hash HEAD)
    version=$(workspace_version)
    commit=$(git -C "$ROOT" rev-parse HEAD)
    today=$(date -u +%Y-%m-%d)
    mkdir -p "$(dirname "$STAMP")"
    cat > "$STAMP" <<EOF
# Provenance of the visual assets (screenshots and GIFs under
# native-theme/docs/assets and connectors/*/docs/assets).
# Written by scripts/update_provenance.sh at the end of scripts/generate_assets_release.sh;
# read by scripts/check_release.sh and the crates.io workflow's release gate.
# Do not edit by hand.

# Workspace version and commit the assets were captured from.
version = "$version"
commit = "$commit"
generated = "$today"

# OS versions of the CI runners that captured the macOS and Windows
# screenshots, from the screenshots workflow's sidecars; empty when the run
# recorded none.
macos_version = "$macos_version"
windows_version = "$windows_version"

# SHA-256 over the git object ids of every path that feeds the showcases
# (SOURCE_PATHS in scripts/update_provenance.sh). The gates recompute it at HEAD;
# a difference means the assets were captured from other sources.
sources = "$hash"
EOF
    echo "wrote $STAMP_REL (version $version, commit ${commit:0:7}, sources ${hash:0:12}, macOS ${macos_version:-unrecorded}, Windows ${windows_version:-unrecorded})"
}

cmd_check() {
    if [ ! -f "$STAMP" ]; then
        echo "no $STAMP_REL: the visual assets carry no provenance; run ./scripts/generate_assets_release.sh"
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
    echo "visual assets are stale: captured from other sources (v$version, ${commit:0:7}, $generated); run ./scripts/generate_assets_release.sh"
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
    write)        cmd_write "${2:-}" ;;
    check)        cmd_check ;;
    verify-clean) cmd_verify_clean ;;
    hash)         sources_hash HEAD ;;
    *)
        echo "usage: $0 {write [DIR]|check|verify-clean|hash}" >&2
        exit 2
        ;;
esac
