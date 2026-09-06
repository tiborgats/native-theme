#!/usr/bin/env bash
# Re-download every bundled icon from native-theme/icons/SOURCES.toml, or add
# a new one:  scripts/refresh-icons.sh            (refresh all)
#             scripts/refresh-icons.sh add lucide battery
#             scripts/refresh-icons.sh add material battery_0_bar
# Requires python3 >= 3.11 (tomllib) and network access. Run from anywhere.
set -euo pipefail
cd "$(dirname "$0")/.."
MANIFEST=native-theme/icons/SOURCES.toml
command -v python3 >/dev/null || { echo "python3 is required" >&2; exit 1; }

if [ "${1:-}" = "add" ]; then
  [ $# -eq 3 ] || { echo "usage: $0 add <set> <name>" >&2; exit 1; }
  dir=$(python3 -c 'import sys,tomllib;m=tomllib.load(open(sys.argv[1],"rb"));print(next(s["dir"] for s in m["set"] if s["name"]==sys.argv[2]))' "$MANIFEST" "$2")
  : > "native-theme/icons/$dir/$3.svg"   # an empty file makes the refresh below fetch it
fi

python3 - "$MANIFEST" <<'PY'
import pathlib, sys, tomllib, urllib.request

manifest = pathlib.Path(sys.argv[1])
root = manifest.parent
data = tomllib.loads(manifest.read_text())
exceptions = {(f["set"], f["file"]): f for f in data.get("file", [])}

def raw(repo: str, ref: str, path: str) -> str:
    return repo.replace("https://github.com/", "https://raw.githubusercontent.com/", 1) + f"/{ref}/{path}"

failures = 0
for s in data["set"]:
    for svg in sorted((root / s["dir"]).glob("*.svg")):
        exc = exceptions.get((s["name"], svg.name))
        url = raw(s["repository"], exc["ref"], exc["path"]) if exc else raw(s["repository"], s["ref"], s["path"].replace("{name}", svg.stem))
        try:
            with urllib.request.urlopen(url) as r:
                body = r.read()
        except Exception as e:  # noqa: BLE001 - report and continue
            print(f"FAILED {svg}: {url}: {e}", file=sys.stderr)
            failures += 1
            continue
        if b"<svg" not in body:
            print(f"FAILED {svg}: {url}: not an SVG", file=sys.stderr)
            failures += 1
            continue
        svg.write_bytes(body)
        print(f"{svg} <- {url}")
sys.exit(1 if failures else 0)
PY
