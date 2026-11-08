#!/usr/bin/env python3
"""List every citation of today's platform-facts.md or inheritance-rules.toml by line that names no commit.

The 0.8.0 switch replaced the hand-written docs/platform-facts.md with the document
scripts/generate_platform_facts.py writes from the verified facts, and deleted
docs/inheritance-rules.toml. A citation of either by line reads the file as commit fc6109d1 holds it,
`platform-facts.md@fc6109d1:N`, `inheritance-rules.toml@fc6109d1:N` (the 0.8.0 plan's *Citations of
today's platform-facts.md by line*): a line number without its commit names a line of another file, or
of none.

Run from the repository root:

    python3 scripts/check_pinned_citations.py
    python3 scripts/check_pinned_citations.py --self-test

It prints one line per finding, tab-separated: the file and line, and the citation. Exit 0 when there is
none, 1 when there are some, 2 on a wrong command line or when git cannot list the files.

Rule
----
Files: every file git tracks or would track (`git ls-files --cached --others --exclude-standard`), read
as UTF-8 (a file that is not is skipped), except EXEMPT: `docs/archive/`, which records what was true
when it was written, and the scripts that define and convert the citation format, whose patterns and
fixtures name the unpinned form. The evidence, whose recorded files keep the citations of their run's
commit, lives in the evidence repository (spec S17 *The evidence root*), not here.

Citations. `platform-facts.md:` or `inheritance-rules.toml:` followed by a digit, with or without a
leading `docs/`. A pinned citation, `platform-facts.md@<sha>:N`, has `@` where the rule wants `:`, so
it is never a finding; a `:N` continuation after a pinned citation inherits its commit. A mention of the
file with no line (`docs/platform-facts.md`, `§2.21`) is not a line citation.
"""

from __future__ import annotations

import re
import subprocess
import sys

EXEMPT_DIRS = ("docs/archive/",)
EXEMPT_FILES = (
    "scripts/check_pinned_citations.py",
    "scripts/citations.py",
    "scripts/convert_inheritance_rules.py",
    "scripts/import_platform_facts.py",
    "scripts/repin_citations.py",
)

UNPINNED = re.compile(r"(?:platform-facts\.md|inheritance-rules\.toml):\d+(?:[-–]\d+)?")


def exempt(path: str) -> bool:
    return path.startswith(EXEMPT_DIRS) or path in EXEMPT_FILES


def findings(texts: dict[str, str]) -> list[tuple[str, int, str]]:
    out = []
    for path in sorted(texts):
        if exempt(path):
            continue
        for n, line in enumerate(texts[path].splitlines(), 1):
            for m in UNPINNED.finditer(line):
                out.append((path, n, m.group(0)))
    return out


def tracked() -> list[str]:
    res = subprocess.run(["git", "ls-files", "-z", "--cached", "--others", "--exclude-standard"],
                         capture_output=True, check=True)
    return [p for p in res.stdout.decode("utf-8").split("\0") if p]


def read_texts(paths: list[str]) -> dict[str, str]:
    texts = {}
    for path in paths:
        if exempt(path):
            continue
        try:
            with open(path, encoding="utf-8") as f:
                texts[path] = f.read()
        except (OSError, UnicodeDecodeError):
            continue
    return texts


def self_test() -> int:
    texts = {
        "native-theme/src/a.rs": "/// 82% (`docs/platform-facts.md:748`), and :755\n"
                                 "/// pinned (`docs/platform-facts.md@fc6109d1:746-755`, :760)\n",
        "docs/todo.md": "a rule (`docs/inheritance-rules.toml:211`) and one at the pin "
                        "(`inheritance-rules.toml@fc6109d1:205`)\n"
                        "a range platform-facts.md:358-368, a section `docs/platform-facts.md` §2.21\n",
        "CHANGELOG.md": "no line: docs/platform-facts.md and docs/inheritance-rules.toml\n",
        "docs/archive/old.md": "platform-facts.md:12\n",
        "scripts/citations.py": "platform-facts.md:LINE platform-facts.md:1061\n",
    }
    want = [
        ("docs/todo.md", 1, "inheritance-rules.toml:211"),
        ("docs/todo.md", 2, "platform-facts.md:358-368"),
        ("native-theme/src/a.rs", 1, "platform-facts.md:748"),
    ]
    failures = []
    got = findings(texts)
    if got != want:
        failures.append(f"rule: got {got}, want {want}")
    if not exempt("docs/archive/x.md") or exempt("docs/archived.md") or exempt("scripts/check_facts.py"):
        failures.append("exempt: wrong paths")
    for failure in failures:
        print(f"self-test FAIL: {failure}", file=sys.stderr)
    if not failures:
        print("self-test ok")
    return 1 if failures else 0


def main(argv: list[str]) -> int:
    if argv[1:] == ["--self-test"]:
        return self_test()
    if argv[1:]:
        print(__doc__, file=sys.stderr)
        return 2
    try:
        texts = read_texts(tracked())
    except (OSError, subprocess.CalledProcessError, UnicodeDecodeError) as err:
        print(f"error: cannot list the files: {err}", file=sys.stderr)
        return 2
    out = findings(texts)
    for path, line, cite in out:
        print(f"{path}:{line}\t{cite}")
    print(f"{len(out)} unpinned line citations in {len(texts)} files", file=sys.stderr)
    return 1 if out else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
