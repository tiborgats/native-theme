#!/usr/bin/env python3
"""Check that every Widget Info colour claim is read at the line it cites.

A panel in the gpui showcase says which theme field a widget reads. That is a
statement about a *dependency's* source, so nothing in this repository can
keep it true on its own (widget-info spec section 4). Each claim therefore
carries the location it was read at, and this script opens that location and
requires the named field to be there.

Run from the repository root:

    python3 scripts/check-widget-citations.py

Exit 0 only when every cited claim checks out and no claim is left uncited
once `--require-citations` is given. Until the citation pass is finished the
script reports the uncited count without failing on it, so each page's commit
stays green while the count falls (spec section 6.6).

Why a line and not a symbol
---------------------------
`file.rs, Symbol` is the form the prose notes already use, and it does not
work for this. `ButtonVariant::text_color` (button.rs:947-996) holds
`Self::Link => cx.theme().link` and
`Self::Text => cx.theme().foreground.opacity(0.9)` two lines apart, so a
symbol-scoped check would find `foreground` inside that function and pass
Button (Link)'s claim of it. Only a line separates variants that share a
function.

The cost of a line is drift: an insertion earlier in a file moves every later
line, so a claim can fail although nothing about it changed. The failure
therefore prints what the cited line *now* says, which is what tells the two
cases apart at a glance.

Where a citation points
-----------------------
`<file>:<line>` or `<file>:<from>-<to>`. A citation gives a path, not a
crate, and four are searched: `gpui-component`, `gpui-base`, `gpui-pre` and
this connector's own `src/`. That breadth is needed --  `TITLE_BAR_HEIGHT` is
gpui-component's, `PANEL_MIN_SIZE` is gpui-base's, and the flat button
variant is ours -- and it is also why a claim holds when **any** candidate
bears it out: `geometry.rs` names a file in this connector and another in
gpui-pre, and an existence check does not need to know which one a note
meant.

Each crate's source directory comes from `cargo metadata` only, never from a
registry path or a version literal, as `check-widget-coverage.py` does. A
dependency missing from the metadata is a hard error, never a silently
skipped check.

Two things are checked
----------------------
A **colour claim** carries `<file>:<line>` and names a theme field; the field
must be on that line. A **"Not themeable" note** may carry `file.rs, Symbol`,
and then the symbol must still exist -- existence only, never semantics. What
a note says about upstream is a human's to keep true; a note pointing at
something upstream has *deleted* is one a machine should catch, and this
project has been bitten by exactly that when `ThemeColor::tiles` went away in
a patch release. A note with no citation is not required to gain one: many
state an absence, with no symbol to point at, and demanding a citation there
would invite an invented one.
"""

import argparse
import json
import os
import re
import subprocess
import sys

SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
PROJECT_ROOT = os.path.dirname(SCRIPT_DIR)

GPUI_MANIFEST = os.path.join(
    PROJECT_ROOT, "connectors", "native-theme-gpui", "Cargo.toml"
)
CONNECTOR_SRC = os.path.join(PROJECT_ROOT, "connectors", "native-theme-gpui", "src")
SHOWCASE = os.path.join(
    PROJECT_ROOT, "connectors", "native-theme-gpui", "examples", "showcase-gpui.rs"
)

CITATION = re.compile(r"^([A-Za-z0-9_./-]+\.rs):(\d+)(?:-(\d+))?$")


class Failure(Exception):
    """A check could not be carried out, or did not hold."""


# --------------------------------------------------------------------------
# Locating the vendored source
# --------------------------------------------------------------------------


def cargo_metadata():
    cmd = [
        "cargo",
        "metadata",
        "--format-version",
        "1",
        "--manifest-path",
        GPUI_MANIFEST,
    ]
    try:
        done = subprocess.run(
            cmd, cwd=PROJECT_ROOT, capture_output=True, text=True, check=False
        )
    except OSError as err:
        raise Failure(f"could not run `cargo metadata`: {err}") from err
    if done.returncode != 0:
        raise Failure("`cargo metadata` failed:\n" + " ".join(cmd) + "\n" + done.stderr.strip())
    try:
        return json.loads(done.stdout)
    except json.JSONDecodeError as err:
        raise Failure(f"`cargo metadata` produced no usable JSON: {err}") from err


def source_dir(meta, name):
    for pkg in meta.get("packages", []):
        if pkg.get("name") == name:
            src = os.path.join(os.path.dirname(pkg["manifest_path"]), "src")
            if not os.path.isdir(src):
                raise Failure(f"`{name}` has no src directory at {src}")
            return src
    raise Failure(
        f"`{name}` is not in the cargo metadata, so its source cannot be read; "
        "a missing dependency is a hard error, never a skipped check"
    )


# --------------------------------------------------------------------------
# Reading the claims out of the showcase
# --------------------------------------------------------------------------


def scan_balanced(text, start):
    """Index just past the `)` matching the `(` at `start`, strings honoured."""
    depth = 0
    i = start
    n = len(text)
    while i < n:
        c = text[i]
        if c == '"':
            j = i - 1
            hashes = 0
            while j >= 0 and text[j] == "#":
                hashes += 1
                j -= 1
            if hashes and j >= 0 and text[j] == "r":
                end = text.find('"' + "#" * hashes, i + 1)
                i = (end + 1 + hashes) if end != -1 else n
                continue
            i += 1
            while i < n:
                if text[i] == "\\":
                    i += 2
                    continue
                if text[i] == '"':
                    i += 1
                    break
                i += 1
            continue
        if c == "'":
            if i + 2 < n and text[i + 1] == "\\":
                k = text.find("'", i + 2)
                i = (k + 1) if k != -1 else i + 1
                continue
            if i + 2 < n and text[i + 2] == "'":
                i += 3
                continue
            i += 1
            continue
        if c == "/" and text[i : i + 2] == "//":
            k = text.find("\n", i)
            i = (k + 1) if k != -1 else n
            continue
        if c == "(":
            depth += 1
        elif c == ")":
            depth -= 1
            if depth == 0:
                return i + 1
        i += 1
    raise Failure(f"unbalanced parentheses from offset {start} in the showcase")


def split_top(body):
    """Split at depth-0 commas, strings and char literals honoured."""
    out, depth, cur, i, n = [], 0, [], 0, len(body)
    while i < n:
        c = body[i]
        if c == '"':
            start = i
            i += 1
            while i < n:
                if body[i] == "\\":
                    i += 2
                    continue
                if body[i] == '"':
                    i += 1
                    break
                i += 1
            cur.append(body[start:i])
            continue
        if c == "'":
            if i + 2 < n and body[i + 1] == "\\":
                k = body.find("'", i + 2)
                stop = (k + 1) if k != -1 else i + 1
            elif i + 2 < n and body[i + 2] == "'":
                stop = i + 3
            else:
                stop = i + 1
            cur.append(body[i:stop])
            i = stop
            continue
        if c == "/" and body[i : i + 2] == "//":
            k = body.find("\n", i)
            stop = (k + 1) if k != -1 else n
            cur.append(body[i:stop])
            i = stop
            continue
        if c in "([{":
            depth += 1
        elif c in ")]}":
            depth -= 1
        elif c == "," and depth == 0:
            out.append("".join(cur))
            cur = []
            i += 1
            continue
        cur.append(c)
        i += 1
    if "".join(cur).strip():
        out.append("".join(cur))
    return out


def unquote(text):
    """The contents of a Rust string literal, or None."""
    t = text.strip()
    if len(t) >= 2 and t[0] == '"' and t[-1] == '"':
        return t[1:-1]
    return None


def claims(source):
    """Every colour claim: (line, widget, role, field, citation)."""
    out = []
    for m in re.finditer(r"\.hover_info\s*\(", source):
        open_paren = m.end() - 1
        end = scan_balanced(source, open_paren)
        args = split_top(source[open_paren + 1 : end - 1])
        if len(args) != 5:
            raise Failure(
                f"hover_info at line {source.count(chr(10), 0, m.start()) + 1} "
                f"takes {len(args)} arguments, not 5"
            )
        widget = unquote(args[1]) or "?"
        body = args[2].strip()
        if not body.startswith("&["):
            raise Failure(f"{widget}: the colours argument is not an array literal")
        inner = body[2:].rstrip()
        inner = inner[:-1] if inner.endswith("]") else inner
        if not inner.strip():
            continue
        for tup in split_top(inner):
            t = tup.strip()
            if not t.startswith("("):
                continue
            parts = split_top(t[1:-1])
            if len(parts) != 4:
                raise Failure(
                    f"{widget}: a colour claim has {len(parts)} parts, not 4: "
                    f"{t[:70]}"
                )
            line = source.count("\n", 0, open_paren + 1 + tup.find(t[0])) + 1
            out.append(
                (
                    line,
                    widget,
                    unquote(parts[0]) or "?",
                    unquote(parts[1]) or "?",
                    unquote(parts[3]),
                )
            )
    return out


# --------------------------------------------------------------------------
# Checking a citation
# --------------------------------------------------------------------------


def candidates(citation, roots):
    """Every file a citation could name, across the crates searched.

    A citation gives a path, not a crate: `button.rs` is gpui-component's
    `button/button.rs`, `resizable/mod.rs` is gpui-base's, and `geometry.rs`
    is this connector's own -- but gpui-pre has a `geometry.rs` too. Rather
    than guess, every candidate is returned and the claim holds if **any** of
    them bears it out. An existence check does not need to know which crate a
    note meant; it needs to know the thing is still somewhere.
    """
    out = []
    for root in roots:
        path = os.path.join(root, citation)
        if os.path.isfile(path):
            out.append(path)
    # A bare name may sit in a subdirectory: button.rs is button/button.rs.
    base = os.path.basename(citation)
    for root in roots:
        for dirpath, _dirs, files in os.walk(root):
            if base in files:
                path = os.path.join(dirpath, base)
                if path not in out:
                    out.append(path)
    return out


def reads_field(text, field):
    """Whether `text` uses `field` as an identifier.

    `tokens.x` satisfies a claim of `x`: `ThemeTokens` is a mechanical
    projection of `ThemeColor`, so the two are the same value.
    """
    return re.search(r"(?<![A-Za-z0-9_])" + re.escape(field) + r"(?![A-Za-z0-9_])", text)


PROSE_CITATION = re.compile(
    r"([A-Za-z0-9_/]+\.rs),\s*([A-Za-z_][A-Za-z0-9_]*(?:::[A-Za-z_][A-Za-z0-9_]*)*)"
)


def prose_citations(source):
    """Every `file.rs, Symbol` a "Not themeable" note cites.

    Yields (line, widget, file, symbol). Only the entries that already cite
    something are returned: a note without a citation is not required to gain
    one (spec section 4.6). Many state an absence -- "the model carries no
    circular-progress diameter" -- with no symbol to point at, and demanding a
    citation there would invite an invented one.
    """
    out = []
    for m in re.finditer(r"\.hover_info\s*\(", source):
        open_paren = m.end() - 1
        end = scan_balanced(source, open_paren)
        args = split_top(source[open_paren + 1 : end - 1])
        if len(args) != 5:
            continue
        widget = unquote(args[1]) or "?"
        line = source.count("\n", 0, m.start()) + 1
        for hit in PROSE_CITATION.finditer(args[4]):
            out.append((line, widget, hit.group(1), hit.group(2)))
    return out


def check_prose(roots):
    """Spec section 4.6: a cited symbol must still exist.

    Existence, never semantics. What a note *says* about upstream is a human's
    to keep true; a note pointing at something upstream has **deleted** is one
    a machine should catch, and this project has been bitten by exactly that --
    `ThemeColor::tiles` went away in a patch release.

    The symbol's last path segment is looked for as an identifier anywhere in
    the file, not as a declaration: a method reached through a trait is not
    declared in the file that uses it, and failing those would teach everyone
    to ignore this check.
    """
    bad = []
    cited = prose_citations(open(SHOWCASE, encoding="utf-8").read())
    for line, widget, path_part, symbol in cited:
        paths = candidates(path_part, roots)
        if not paths:
            bad.append(f"{widget} line {line}: cites {path_part}, which does not exist")
            continue
        leaf = symbol.split("::")[-1]
        if not any(
            reads_field(open(p, encoding="utf-8").read(), leaf) for p in paths
        ):
            bad.append(
                f"{widget} line {line}: cites {path_part}, {symbol} — `{leaf}` is "
                f"in none of {', '.join(os.path.relpath(p) for p in paths)}"
            )
    return len(cited), bad


def check(require_citations):
    meta = cargo_metadata()
    # A note cites whichever crate states the fact: `TITLE_BAR_HEIGHT` is
    # gpui-component's, `PANEL_MIN_SIZE` is gpui-base's, and the flat button
    # variant is this connector's own. All three are searched, in the order a
    # citation is most likely to mean.
    roots = [
        source_dir(meta, "gpui-component"),
        source_dir(meta, "gpui-base"),
        source_dir(meta, "gpui-pre"),
        CONNECTOR_SRC,
    ]

    source = open(SHOWCASE, encoding="utf-8").read()
    found = claims(source)
    if not found:
        raise Failure("no colour claims found in the showcase; the check would pass vacuously")

    uncited, bad = [], []
    checked = 0
    for line, widget, role, field, citation in found:
        if not citation:
            uncited.append((line, widget, role, field))
            continue
        m = CITATION.match(citation)
        if not m:
            bad.append(f"{widget} [{role}] line {line}: `{citation}` is not <file>:<line>")
            continue
        paths = candidates(m.group(1), roots)
        if not paths:
            bad.append(
                f"{widget} [{role}] line {line}: cites {m.group(1)}, which does not exist"
            )
            continue
        first = int(m.group(2))
        last = int(m.group(3) or m.group(2))
        checked += 1
        shown = None
        held = False
        for path in paths:
            lines = open(path, encoding="utf-8").read().split("\n")
            if first < 1 or last > len(lines) or first > last:
                continue
            span = "\n".join(lines[first - 1 : last])
            if shown is None:
                shown = span.strip().splitlines()
            if reads_field(span, field):
                held = True
                break
        if not held:
            now = shown[0].strip() if shown else "(no such line in that file)"
            bad.append(
                f"{widget} [{role}] line {line}: claims `{field}` at {citation}\n"
                f"      that line now reads: {now}"
            )

    prose_total, prose_bad = check_prose(roots)

    print(
        f"colour claims {len(found)}   cited {len(found) - len(uncited)}   "
        f"verified {checked}   uncited {len(uncited)}   wrong {len(bad)}"
    )
    print(f"prose citations {prose_total}   missing {len(prose_bad)}")
    if bad:
        print("\ncitations that do not hold:")
        for b in bad:
            print(f"  {b}")
    if prose_bad:
        print("\nprose citations pointing at something that is gone:")
        for b in prose_bad:
            print(f"  {b}")
        bad = bad + prose_bad
    if uncited and require_citations:
        print("\nclaims with no citation:")
        for line, widget, role, field in uncited[:40]:
            print(f"  {widget} [{role}] line {line}: {field}")
        if len(uncited) > 40:
            print(f"  … and {len(uncited) - 40} more")
    if bad:
        return 1
    if uncited and require_citations:
        return 1
    return 0


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument(
        "--require-citations",
        action="store_true",
        help="fail when a claim has no citation, not only when one is wrong",
    )
    args = parser.parse_args()
    try:
        return check(args.require_citations)
    except Failure as err:
        print(f"error: {err}", file=sys.stderr)
        return 2


if __name__ == "__main__":
    sys.exit(main())
