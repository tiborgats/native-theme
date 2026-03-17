#!/usr/bin/env bash
#
# dev-env-update.sh — Update development tools and dependencies for reasoning-engine
# Supports: Arch Linux (CachyOS, Garuda, etc.), Debian/Ubuntu, Fedora
#
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
LOG_FILE="/dev/null"

# --- Helpers ---------------------------------------------------------------

info()    { printf '\033[1;34m⟳\033[0m %s\n' "$*"; }
# "checking" prints an in-progress line that will be overwritten by the result
checking() { printf '\033[1;34m⟳\033[0m %s ...\r' "$*"; }
updated()  { printf '\033[2K\033[1;32m✓\033[0m %s  \033[2m%s → %s\033[0m\n' "$1" "$2" "$3"; }
current()  { printf '\033[2K\033[0;32m✓\033[0m %s  \033[2m(%s)\033[0m\n' "$1" "$2"; }
skipped()  { printf '\033[2K\033[0;33m–\033[0m %s  \033[2m%s\033[0m\n' "$1" "$2"; }
failed()   { printf '\033[2K\033[1;31m✗\033[0m %s  \033[2m%s\033[0m\n' "$1" "$2"; }

need_cmd() { command -v "$1" &>/dev/null; }

detect_distro() {
    if [ -f /etc/os-release ]; then
        # shellcheck source=/dev/null
        . /etc/os-release
        case "${ID:-}" in
            cachyos|garuda|arch|endeavouros|manjaro) echo "arch" ;;
            ubuntu|debian|linuxmint|pop)             echo "debian" ;;
            fedora|rhel|centos|rocky|alma)           echo "fedora" ;;
            *)
                case "${ID_LIKE:-}" in
                    *arch*)              echo "arch" ;;
                    *debian*|*ubuntu*)   echo "debian" ;;
                    *fedora*|*rhel*)     echo "fedora" ;;
                    *)                   echo "unknown" ;;
                esac
                ;;
        esac
    else
        echo "unknown"
    fi
}

DISTRO=$(detect_distro)

# Install or update a system package via the distro package manager.
# Usage: pkg_update <arch-name> <debian-name> [fedora-name]
pkg_update() {
    local arch_pkg="$1" deb_pkg="$2" fed_pkg="${3:-$2}"
    case "$DISTRO" in
        arch)
            if need_cmd paru; then
                paru -S --needed --noconfirm "$arch_pkg" >>"$LOG_FILE" 2>&1
            elif need_cmd yay; then
                yay -S --needed --noconfirm "$arch_pkg" >>"$LOG_FILE" 2>&1
            else
                sudo pacman -S --needed --noconfirm "$arch_pkg" >>"$LOG_FILE" 2>&1
            fi
            ;;
        debian)
            sudo apt-get update -qq >>"$LOG_FILE" 2>&1
            sudo apt-get install --only-upgrade -y "$deb_pkg" >>"$LOG_FILE" 2>&1
            ;;
        fedora)
            sudo dnf upgrade -y "$fed_pkg" >>"$LOG_FILE" 2>&1
            ;;
        *)
            skipped "$deb_pkg" "unsupported distro — update manually"
            return 1
            ;;
    esac
}

# Get the short version string for a command.
# Usage: get_version <command> [args...]
get_version() {
    local cmd="$1"; shift
    if ! need_cmd "$cmd"; then echo "not installed"; return; fi
    "$cmd" "$@" 2>/dev/null | head -1 | grep -oP '[0-9]+\.[0-9]+[0-9.a-zA-Z_-]*' | head -1 || echo "unknown"
}

echo
printf '\033[1m  Development Environment Update\033[0m\n'
printf '  %s\n\n' "$(date '+%Y-%m-%d %H:%M')"

# --- Rust toolchain --------------------------------------------------------

checking "Rust toolchain"
if need_cmd rustup; then
    v_before=$(get_version rustc --version)
    rustup update >>"$LOG_FILE" 2>&1
    v_after=$(get_version rustc --version)
    if [ "$v_before" = "$v_after" ]; then
        current "Rust toolchain" "$v_after"
    else
        updated "Rust toolchain" "$v_before" "$v_after"
    fi
else
    skipped "Rust toolchain" "rustup not found"
fi

# --- Cargo tools ------------------------------------------------------------

CARGO_TOOLS=("cargo-geiger" "mdbook" "mdbook-pdf")

# Pre-fetch installed versions and latest versions in one go
installed_list=$(cargo install --list 2>/dev/null || true)

for tool in "${CARGO_TOOLS[@]}"; do
    checking "$tool"

    # Installed version from cargo install --list
    v_installed=$(echo "$installed_list" | grep "^${tool} " | grep -oP 'v\K[0-9]+\.[0-9]+[0-9.]*' | head -1 || echo "")

    # Latest version from crates.io registry
    v_latest=$(cargo search "$tool" --limit 1 2>/dev/null \
        | grep "^${tool} " | grep -oP '"[0-9]+\.[0-9]+[0-9.]*"' | tr -d '"' || echo "")

    if [ -n "$v_installed" ] && [ "$v_installed" = "$v_latest" ]; then
        current "$tool" "$v_installed"
    else
        v_from="${v_installed:-not installed}"
        v_to="${v_latest:-latest}"
        printf '\033[2K\033[1;34m⟳\033[0m %s  %s → %s\n' "$tool" "$v_from" "$v_to"
        # Stream build output live so the user can see compilation progress
        if cargo install "$tool" 2>&1 | tee -a "$LOG_FILE"; then
            v_after=$(cargo install --list 2>/dev/null | grep "^${tool} " | grep -oP 'v\K[0-9]+\.[0-9]+[0-9.]*' | head -1 || echo "$v_to")
            updated "$tool" "$v_from" "$v_after"
        else
            failed "$tool" "install failed — see output above"
        fi
    fi
done

# --- Cargo dependencies -----------------------------------------------------

checking "Cargo.lock dependencies"
lock_before=""
if [ -f "$SCRIPT_DIR/Cargo.lock" ]; then
    lock_before=$(md5sum "$SCRIPT_DIR/Cargo.lock" | cut -d' ' -f1)
fi

# cargo update prints "Updating crate_name v0.1 -> v0.2" lines — show them live
update_output=$(cd "$SCRIPT_DIR" && cargo update 2>&1) || true
echo "$update_output" >> "$LOG_FILE"

lock_after=""
if [ -f "$SCRIPT_DIR/Cargo.lock" ]; then
    lock_after=$(md5sum "$SCRIPT_DIR/Cargo.lock" | cut -d' ' -f1)
fi

if [ "$lock_before" = "$lock_after" ]; then
    current "Cargo.lock" "no changes"
else
    # Show which crates were updated
    n_updated=$(echo "$update_output" | grep -c "Updating\|Removing\|Adding" || echo "0")
    printf '\033[2K\033[1;32m✓\033[0m Cargo.lock  \033[2m%s crate(s) changed:\033[0m\n' "$n_updated"
    # Print the update/add/remove lines indented
    echo "$update_output" | grep "Updating\|Removing\|Adding" | sed 's/^/    /' || true
fi

# --- AI tools ---------------------------------------------------------------

# Helper: check an npm global tool — quick version compare, live output only on update
# Usage: check_npm_tool <display-name> <npm-package> <binary> <version-args...>
check_npm_tool() {
    local name="$1" pkg="$2" bin="$3"; shift 3
    if need_cmd "$bin"; then
        checking "$name"
        v_before=$(get_version "$bin" "$@")
        v_latest=$(npm show "$pkg" version 2>/dev/null || echo "")

        if [ -n "$v_latest" ] && [ "$v_before" = "$v_latest" ]; then
            current "$name" "$v_before"
        else
            printf '\033[2K\033[1;34m⟳\033[0m %s  %s → %s\n' "$name" "$v_before" "${v_latest:-latest}"
            if npm install -g "${pkg}@latest" 2>&1 | tee -a "$LOG_FILE"; then
                v_after=$(get_version "$bin" "$@")
                updated "$name" "$v_before" "$v_after"
            else
                failed "$name" "update failed — see output above"
            fi
        fi
    else
        skipped "$name" "not installed"
    fi
}

check_npm_tool "Claude Code" "@anthropic-ai/claude-code" "claude" "--version"
check_npm_tool "Gemini CLI" "@google/gemini-cli" "gemini" "--version"

# OpenCode
if need_cmd opencode; then
    checking "OpenCode"
    v_before=$(get_version opencode --version)
    # No registry to query — run installer quietly, compare versions
    if curl -fsSL https://opencode.ai/install 2>/dev/null | bash >/dev/null 2>&1; then
        v_after=$(get_version opencode --version)
        if [ "$v_before" = "$v_after" ]; then
            current "OpenCode" "$v_before"
        else
            updated "OpenCode" "$v_before" "$v_after"
        fi
    else
        failed "OpenCode" "update failed"
    fi
else
    skipped "OpenCode" "not installed"
fi

# Antigravity
if need_cmd antigravity || need_cmd agy; then
    checking "Antigravity"
    v_before=$(get_version antigravity --version 2>/dev/null || get_version agy --version)
    if pkg_update antigravity antigravity antigravity; then
        v_after=$(get_version antigravity --version 2>/dev/null || get_version agy --version)
        if [ "$v_before" = "$v_after" ]; then
            current "Antigravity" "$v_before"
        else
            updated "Antigravity" "$v_before" "$v_after"
        fi
    else
        failed "Antigravity" "update failed — see output above"
    fi
else
    skipped "Antigravity" "not installed"
fi

# get-shit-done-cc (GSD)
if need_cmd npm && need_cmd npx; then
    checking "get-shit-done-cc"
    cached_pkg=$(ls ~/.npm/_npx/*/node_modules/get-shit-done-cc/package.json 2>/dev/null | head -1 || echo "")
    if [ -n "$cached_pkg" ] && [ -f "$cached_pkg" ]; then
        v_before=$(grep '"version"' "$cached_pkg" | head -1 | grep -oP '[0-9]+\.[0-9]+\.[0-9]+' || echo "unknown")
    else
        v_before="not installed"
    fi

    v_latest=$(npm show get-shit-done-cc version 2>/dev/null || echo "")
    if [ -z "$v_latest" ]; then
        failed "get-shit-done-cc" "could not fetch latest version"
    elif [ "$v_before" = "$v_latest" ]; then
        current "get-shit-done-cc" "$v_before"
    else
        printf '\033[2K\033[1;34m⟳\033[0m get-shit-done-cc  %s → %s\n' "$v_before" "$v_latest"
        if npx get-shit-done-cc@latest 2>&1 | tee -a "$LOG_FILE"; then
            updated "get-shit-done-cc" "$v_before" "$v_latest"
        else
            failed "get-shit-done-cc" "update failed — see output above"
        fi
    fi
else
    skipped "get-shit-done-cc" "npm/npx not found"
fi

# --- Done -------------------------------------------------------------------

echo
printf '\033[1;32m  Done.\033[0m\n\n'
