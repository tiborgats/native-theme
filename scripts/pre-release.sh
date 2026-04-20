#!/usr/bin/env bash
set -euo pipefail

# Pre-release visual asset pipeline
#
# Triggers CI for macOS/Windows screenshots first, then generates local
# Linux assets in parallel while CI runs, and finally downloads the
# CI results into the per-connector docs/assets/ directories.
#
# Prerequisites: gh CLI authenticated, spectacle installed (KDE Wayland)
#
# Usage: bash scripts/pre-release.sh

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
ICED_DIR="$PROJECT_ROOT/connectors/native-theme-iced/docs/assets"
GPUI_DIR="$PROJECT_ROOT/connectors/native-theme-gpui/docs/assets"
NT_DIR="$PROJECT_ROOT/native-theme/docs/assets"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

ok()   { echo -e "${GREEN}✓${NC} $1"; }
fail() { echo -e "${RED}✗ $1${NC}"; exit 1; }
info() { echo -e "${YELLOW}→${NC} $1"; }

cd "$PROJECT_ROOT"

# ── Preflight checks ─────────────────────────────────────────────────

echo "=== Pre-release visual asset pipeline ==="
echo ""

info "Checking prerequisites..."
command -v gh >/dev/null 2>&1 || fail "gh CLI not found"
command -v python3 >/dev/null 2>&1 || fail "python3 not found"
command -v spectacle >/dev/null 2>&1 || fail "spectacle not found (needed for gpui captures)"
python3 -c "from PIL import Image" 2>/dev/null || fail "Pillow not installed (pip install Pillow)"
gh auth status >/dev/null 2>&1 || fail "gh CLI not authenticated"
ok "Prerequisites OK"
echo ""

# ── Step 1: Trigger CI early (runs in parallel with local work) ──────

echo "=== Step 1/5: Trigger macOS & Windows screenshots via CI ==="
echo ""

# Ensure local changes are pushed so CI uses the latest code
info "Checking for unpushed commits..."
LOCAL=$(git rev-parse HEAD)
REMOTE=$(git rev-parse @{u} 2>/dev/null || echo "none")

if [ "$LOCAL" != "$REMOTE" ]; then
    info "Unpushed commits detected. Push first, then re-run."
    fail "Push your changes before running this script (CI needs the latest code)"
fi
ok "Branch is up to date with remote"

info "Triggering screenshots workflow..."
BRANCH=$(git branch --show-current)
EXPECTED_SHA="$LOCAL"

gh workflow run screenshots.yml --ref "$BRANCH"

# Poll until the new run appears (the fixed-sleep approach races with GitHub's queue)
info "Waiting for workflow run to register..."
RUN_ID=""
for i in $(seq 1 30); do
    RUN_ID=$(gh run list --workflow=screenshots.yml --branch="$BRANCH" --limit 1 \
        --json databaseId,headSha --jq ".[] | select(.headSha == \"$EXPECTED_SHA\") | .databaseId")
    if [ -n "$RUN_ID" ]; then
        break
    fi
    sleep 2
done

if [ -z "$RUN_ID" ]; then
    fail "Timed out waiting for workflow run at $EXPECTED_SHA to appear"
fi

info "Workflow run: https://github.com/$(gh repo view --json nameWithOwner --jq '.nameWithOwner')/actions/runs/$RUN_ID"
ok "CI workflow triggered (running in background while we generate local assets)"
echo ""

# ── Step 2: Local Linux assets (while CI runs) ──────────────────────

echo "=== Step 2/5: Spinner GIFs ==="
python3 "$SCRIPT_DIR/generate_gifs.py"
ok "Spinner GIFs generated"
echo ""

echo "=== Step 3/5: Iced Linux screenshots ==="
bash "$SCRIPT_DIR/generate_screenshots.sh"
ok "Iced Linux screenshots generated"
echo ""

echo "=== Step 4/5: gpui Linux screenshots ==="
bash "$SCRIPT_DIR/generate_gpui_screenshots.sh"
ok "gpui Linux screenshots generated"
echo ""

echo "=== Step 5/5: Theme-switching GIFs (iced + gpui) ==="
bash "$SCRIPT_DIR/generate_theme_switching_gif.sh"
ok "Theme-switching GIFs generated"
echo ""

# ── Step 3: Wait for CI to complete ─────────────────────────────────

STATUS=$(gh run view "$RUN_ID" --json status,conclusion --jq '"\(.status) \(.conclusion)"')
read -r status conclusion <<< "$STATUS"

if [ "$status" = "completed" ]; then
    if [ "$conclusion" = "success" ]; then
        ok "CI screenshots workflow succeeded"
    else
        fail "CI screenshots workflow failed (conclusion: $conclusion). Check: gh run view $RUN_ID --log-failed"
    fi
else
    info "Waiting for CI to complete..."
    MAX_WAIT=180  # 30 minutes (180 iterations * 10s sleep)
    WAIT_COUNT=0
    while true; do
        sleep 10
        WAIT_COUNT=$((WAIT_COUNT + 1))
        if [ "$WAIT_COUNT" -ge "$MAX_WAIT" ]; then
            fail "Timed out waiting for CI after 30 minutes. Check: gh run view $RUN_ID"
        fi
        STATUS=$(gh run view "$RUN_ID" --json status,conclusion --jq '"\(.status) \(.conclusion)"')
        read -r status conclusion <<< "$STATUS"

        if [ "$status" = "completed" ]; then
            if [ "$conclusion" = "success" ]; then
                ok "CI screenshots workflow succeeded"
                break
            else
                echo ""
                fail "CI screenshots workflow failed (conclusion: $conclusion). Check: gh run view $RUN_ID --log-failed"
            fi
        fi
    done
fi

# Verify the completed run matches our commit before downloading
RUN_SHA=$(gh run view "$RUN_ID" --json headSha --jq '.headSha')
if [ "$RUN_SHA" != "$EXPECTED_SHA" ]; then
    fail "CI run built $RUN_SHA but expected $EXPECTED_SHA — refusing to download stale screenshots"
fi

# Download macOS and Windows screenshots
info "Downloading macOS and Windows screenshots..."
TMPDIR=$(mktemp -d)
gh run download "$RUN_ID" --dir "$TMPDIR"

DOWNLOADED=0
for artifact_dir in "$TMPDIR"/screenshots-iced-macos "$TMPDIR"/screenshots-gpui-macos "$TMPDIR"/screenshots-iced-windows "$TMPDIR"/screenshots-gpui-windows; do
    [ -d "$artifact_dir" ] || continue
    base=$(basename "$artifact_dir")   # e.g. screenshots-iced-macos
    case "$base" in
        screenshots-iced-*) dest="$ICED_DIR" ;;
        screenshots-gpui-*) dest="$GPUI_DIR" ;;
        *) fail "Unexpected artifact dir: $base" ;;
    esac
    mkdir -p "$dest"
    for f in "$artifact_dir"/*.png; do
        [ -f "$f" ] || continue
        cp "$f" "$dest/"
        DOWNLOADED=$((DOWNLOADED + 1))
        echo "  $(basename "$f") → $(realpath --relative-to="$PROJECT_ROOT" "$dest")"
    done
done

rm -rf "$TMPDIR"

if [ "$DOWNLOADED" -eq 0 ]; then
    fail "No screenshots downloaded from CI"
fi
ok "Downloaded $DOWNLOADED screenshots from CI"

# ── Summary ──────────────────────────────────────────────────────────

echo ""
echo "=== Pre-release assets complete ==="
echo ""
for dir in "$ICED_DIR" "$GPUI_DIR" "$NT_DIR"; do
    [ -d "$dir" ] || continue
    rel=$(realpath --relative-to="$PROJECT_ROOT" "$dir")
    count=$(find "$dir" -maxdepth 1 -type f \( -name '*.png' -o -name '*.gif' \) 2>/dev/null | wc -l)
    echo "$rel ($count files):"
    (cd "$dir" && ls -1 *.png *.gif 2>/dev/null) | while read -r f; do echo "  $f"; done
    echo ""
done

info "Review the assets, then commit: git add native-theme/docs/assets/ connectors/native-theme-*/docs/assets/ && git commit -m 'docs: update visual assets'"
