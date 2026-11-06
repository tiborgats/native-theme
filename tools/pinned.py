"""What the vector generators (tools/kde-vectors, tools/motion-vectors) share: the pinned upstream files, read from the
source cache's git objects at the commit `docs/facts/sources.toml` pins, and a sandbox for the code built from them.

- `pinned(cache, sid, path)`: a file's bytes as the pinned commit's tree holds it, read with `git cat-file` from the
  entry's checkout in the source cache (`scripts/update_sources.sh` fills it), so git's own object hashes vouch for
  the content and an edited or stale checkout is not read; git never fetches a missing object here (no network).
- `materialise(cache, files, dest)`: those files written under dest/<id>/<path>, for a build to read.
- `sandboxed(command, writable)`: the command run under `bwrap --unshare-all` (no network, no other namespace shared),
  the filesystem read-only but a fresh `/tmp` and the `writable` folders; its output is what it prints and what it
  writes there. A missing `bwrap` is an error: nothing downloaded or built from downloads runs unconfined.
"""

import os
import shutil
import subprocess
import sys
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SOURCES = ROOT / "docs" / "facts" / "sources.toml"


def commits() -> dict:
    """{sources.toml id: pinned commit} of every git entry."""
    data = tomllib.loads(SOURCES.read_text(encoding="utf-8"))
    return {s["id"]: s["commit"] for s in data["source"] if s.get("kind") == "git"}


def pinned(cache: Path, sid: str, path: str, pins=None) -> bytes:
    """The bytes of `path` in the tree of `sid`'s pinned commit, from the cache's checkout's objects."""
    commit = (pins or commits()).get(sid)
    if commit is None:
        sys.exit(f"{SOURCES}: no git entry `{sid}`")
    repo = cache / sid
    env = dict(os.environ, GIT_NO_LAZY_FETCH="1", GIT_TERMINAL_PROMPT="0")
    out = subprocess.run(["git", "-C", str(repo), "cat-file", "blob", f"{commit}:{path}"], capture_output=True,
                         env=env, check=False)
    if out.returncode != 0:
        sys.exit(f"{repo}: no `{path}` at {commit} (scripts/update_sources.sh fills the cache): "
                 f"{out.stderr.decode(errors='replace').strip()}")
    return out.stdout


def materialise(cache: Path, files, dest: Path):
    """Each (id, path) of `files` written under dest/<id>/<path> as its pinned commit holds it."""
    pins = commits()
    for sid, path in files:
        target = dest / sid / path
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_bytes(pinned(cache, sid, path, pins))


def sandboxed(command, writable=(), readable=(), **run):
    """`subprocess.run(command, **run)` under bwrap: no network, the filesystem read-only, a fresh /tmp, the
    `readable` folders read-only and the `writable` folders writable (each re-bound after /tmp, so the build folder
    inside it stays visible)."""
    bwrap = shutil.which("bwrap")
    if bwrap is None:
        sys.exit("bwrap (bubblewrap) not found: the pinned code runs only sandboxed")
    jail = [bwrap, "--unshare-all", "--die-with-parent", "--new-session", "--ro-bind", "/", "/", "--dev", "/dev",
            "--proc", "/proc", "--tmpfs", "/tmp"]
    for folder in readable:
        jail += ["--ro-bind", str(folder), str(folder)]
    for folder in writable:
        jail += ["--bind", str(folder), str(folder)]
    return subprocess.run([*jail, "--", *map(str, command)], **run)
