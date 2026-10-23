#!/usr/bin/env python3
"""List every `§` reference to docs/platform-facts.md that names no section of it.

The data documents point at platform-facts sections by number (`§2.21`,
`§2.1.9`, `§2 intro`). This script lists each such reference whose section
the facts file does not have.

Run from the repository root:

    python3 scripts/check_section_refs.py
    python3 scripts/check_section_refs.py --self-test

It prints one line per dangling reference, tab-separated: the file and line,
and the reference. Exit 0 when there is none, 1 when there are some, 2 on an
input it cannot read.

Rule
----
Files: docs/property-registry.toml, docs/inheritance-rules.toml,
docs/showcase-elements.toml and docs/new-os-version-guide.md.

References. A `§` followed by a number, `§N` or `§N.M` with any further
`.K` parts, is a platform-facts reference: a range (`§2.1–§2.29`) is two
references, and a trailing `.x` (`§2.x`) leaves the number before it. A `§`
followed by a letter (`§G6`) names another document's section and is skipped,
as is a numbered `§` whose line names another `.md` file closer before it than
platform-facts.md (`v0.5.7_gaps.md §20`).

Sections. `§N` needs a `## Chapter N` heading; `§N.M` a `### N.M` heading;
`§N.M.K` a `#### N.M.K` heading.
"""

from __future__ import annotations

import re
import sys
from pathlib import Path

FILES = [
    Path("docs/property-registry.toml"),
    Path("docs/inheritance-rules.toml"),
    Path("docs/showcase-elements.toml"),
    Path("docs/new-os-version-guide.md"),
]
FACTS = Path("docs/platform-facts.md")

REF = re.compile(r"§(\d+(?:\.\d+)*)")
MD_FILE = re.compile(r"[\w./-]+\.md\b")
CHAPTER = re.compile(r"##\s+Chapter\s+(\d+)\b")
SECTION = re.compile(r"#{3,4}\s+(\d+\.\d+(?:\.\d+)?)\s")


class InputError(Exception):
    """A facts file the rule cannot read."""


def facts_sections(text: str) -> set[str]:
    out: set[str] = set()
    for line in text.splitlines():
        m = CHAPTER.match(line) or SECTION.match(line)
        if m:
            out.add(m.group(1))
    if not any("." not in s for s in out) or not any("." in s for s in out):
        raise InputError("facts file has no chapters or sections")
    return out


def references(text: str) -> list[tuple[int, str]]:
    out = []
    for n, line in enumerate(text.splitlines(), 1):
        for m in REF.finditer(line):
            before = line[: m.start()]
            files = MD_FILE.findall(before)
            if files and not files[-1].endswith("platform-facts.md"):
                continue
            out.append((n, m.group(1)))
    return out


def dangling(docs: dict[str, str], facts_text: str) -> list[tuple[str, int, str]]:
    have = facts_sections(facts_text)
    out = []
    for name, text in docs.items():
        for line, ref in references(text):
            if ref not in have:
                out.append((name, line, ref))
    return out


def self_test() -> int:
    facts = """# Title
## Chapter 1: What OSes Provide
### 1.1 macOS
## Chapter 2: Mapping
### 2.1 Global Defaults
#### 2.1.8 Icon Sizes
### 2.29 Text Area
"""
    doc = """# §2.1 Global (§2.1–§2.29) and §2 intro, §1.1, §2.x tables
# §2.1.8 Icon Sizes
# §2.1.9 Icon Theme
# docs/archive/v0.5.7_gaps.md §G6 and §20 of it
# docs/platform-facts.md §2.30
# §3 nothing
"""
    want = [("doc", 3, "2.1.9"), ("doc", 5, "2.30"), ("doc", 6, "3")]
    failures = []
    got = dangling({"doc": doc}, facts)
    if got != want:
        failures.append(f"rule: got {got}, want {want}")
    try:
        dangling({"doc": doc}, "# nothing\n")
        failures.append("empty facts: accepted")
    except InputError:
        pass
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
        docs = {str(p): p.read_text(encoding="utf-8") for p in FILES}
        out = dangling(docs, FACTS.read_text(encoding="utf-8"))
    except (OSError, InputError) as err:
        print(f"error: {err}", file=sys.stderr)
        return 2
    for name, line, ref in out:
        print(f"{name}:{line}\t§{ref}")
    print(f"{len(out)} dangling § references", file=sys.stderr)
    return 1 if out else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
