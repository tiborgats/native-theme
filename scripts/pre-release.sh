#!/usr/bin/env bash
set -euo pipefail

# Pre-release visual asset pipeline
#
# Triggers CI for macOS/Windows screenshots first, then generates local
# Linux assets in parallel while CI runs, and finally downloads the
# CI results into docs/assets/.
#
# Prerequisites: gh CLI authenticated, spectacle installed (KDE Wayland)
#
# Usage: bash scripts/pre-release.sh

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
OUTPUT_DIR="$PROJECT_ROOT/docs/assets"

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
    while true; do
        sleep 10
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
    if [ -d "$artifact_dir" ]; then
        for f in "$artifact_dir"/*.png; do
            [ -f "$f" ] || continue
            cp "$f" "$OUTPUT_DIR/"
            DOWNLOADED=$((DOWNLOADED + 1))
            echo "  $(basename "$f")"
        done
    fi
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
TOTAL=$(ls "$OUTPUT_DIR"/*.png "$OUTPUT_DIR"/*.gif 2>/dev/null | wc -l)
echo "Total assets in docs/assets/: $TOTAL files"
echo ""
echo "Linux screenshots:"
ls -1 "$OUTPUT_DIR"/iced-linux-*.png "$OUTPUT_DIR"/gpui-linux-*.png 2>/dev/null | while read -r f; do echo "  $(basename "$f")"; done
echo ""
echo "macOS screenshots:"
ls -1 "$OUTPUT_DIR"/iced-macos-*.png "$OUTPUT_DIR"/gpui-macos-*.png 2>/dev/null | while read -r f; do echo "  $(basename "$f")"; done
echo ""
echo "Windows screenshots:"
ls -1 "$OUTPUT_DIR"/iced-windows-*.png "$OUTPUT_DIR"/gpui-windows-*.png 2>/dev/null | while read -r f; do echo "  $(basename "$f")"; done
echo ""
echo "GIFs:"
ls -1 "$OUTPUT_DIR"/*.gif 2>/dev/null | while read -r f; do echo "  $(basename "$f")"; done
echo ""
info "Review the assets, then commit: git add docs/assets/ && git commit -m 'docs: update visual assets'"
