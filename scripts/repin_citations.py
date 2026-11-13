#!/usr/bin/env python3
"""Re-pin the 0.8.0 planning documents' citations of pinned platform sources to the release set (plan M0, *Re-pin*).

Scope: the citations scripts/citations.py finds in the core, the catalogue, the six traces and the gap maps
(citations.REPIN_SCOPE) that cite code of a project docs/facts/sources.toml pins. Out of scope: documentation pages,
issue trackers, this repository's own files, the toolkits, the projects sources.toml lists as not pinned, and the seeds
file's quotes of platform-facts cells.

For each citation in scope:

1. The "from" ref, the commit the citation was read at: the ref citations.py resolves for it (an abbreviation's as its
   definition in the documents states it now, citations.definitions()). Where the citation states none (a bare
   `file:L`), names several projects, or names none, the ref comes from the gap-map row it belongs to: the row IDs in
   the same table row or sentence (`GN-…` GNOME, `K1.2`/`Q1.2`/`K-…` KDE, `W1-…`/`W-…` Windows, `MAT-…` Material,
   `MAC-…` macOS) name the platform's gap map, and when that row's citations, else the whole file's, cite the file
   name from one project at one ref, or (for a citation naming one project) the file cites that project at one ref
   only, that is the ref. Otherwise the citation is **unmappable**; a file mention (no line numbers) of one pinned
   project needs no from ref and is still checked at the pin.
2. The "to" pin: the sources.toml entry of the same project; where a project has several (GTK 4 and GTK 3), the one
   whose commit or tag the from ref names, else the one of the same major version, else the one serving the citing
   document's platform generation.
3. The same commit (or tag) at both ends: **unchanged**. A citation an earlier `--apply` re-pinned names its pin (or
   its definition does), so it is unchanged: a second run over re-pinned documents changes nothing.
4. Otherwise the cited file is read at both commits (the from commit from a checkout under `--from`, the pin from the
   source cache) and its lines mapped by difflib's opcodes over the whole file. A file missing at the pin is looked
   for by its name (a rename or a move); when the citation's own path text no longer names it, the citation is
   **changed**. Per cited range: **moved** when every line maps to a line with the same text and the range stays
   contiguous; **vanished** when no line maps; **changed** otherwise (a line whose text differs, a line gone while
   others stay, lines inserted inside the range). A citation's status is that of its ranges taken together. A citation
   without line numbers (a file mention, a Material token file) is **present** when the file exists at the pin (and
   each backticked token name right after it, `(`name`)`, occurs in it) and **vanished** otherwise.
   **error**: the from commit has no checkout, or the file is not in it, or its content is not available offline.
   A folder that moved between the commits while the citations' paths still name the files (microsoft-ui-xaml's
   `controls/` under `src/`) is listed in the summary: the definition naming the old folder needs the new one.

`--apply` rewrites, in the documents, the moved, unchanged and present citations only: the line numbers in place
(`path:L`, `:L` continuations, a link's text and `#L` anchor), and the ref where the citation carries it (`name@sha`,
`file@sha`, a link's or URL's ref, `qtbase v6.10.1`, `at <ref>`), each in the citation's own syntax (a SHA keeps its
length). A citation whose ref comes from a definition (an abbreviation, a document's citation base) has that definition
rewritten instead, at the lines citations.ABBR names: the from SHA becomes the pin's commit and the from tag the pin's
tag, or the pin's tag is added after the SHA. A carried path or a `:L` continuation follows its head citation; a head
that carries its own ref and its followers are rewritten together or not at all (a changed follower keeps the head's old
ref, and the report lists the group as held). A citation whose ref the document leaves implicit (a bare file name) gets
its line numbers only, and only when the citations its ref was taken from (its row's, or its file's) are rewritten too,
so that a later run takes the pin from them. Changed, vanished, unmappable and error citations are left as they are and
listed; a definition rewritten for some citations leaves the others under it with their old lines (the report says so).
After `--apply`, the summary lists the lines that still name a re-pinned from commit.

    python3 scripts/repin_citations.py [--docs DIR] [--from DIR ...] [--cache DIR] [--sources FILE]
                                       [--report PATH.tsv] [--apply]
    python3 scripts/repin_citations.py --self-test

`--docs DIR` reads (and with `--apply` writes) the documents in DIR (default docs/), by their file names. `--from DIR`
(repeatable) names folders whose git checkouts (the folder itself, or its direct subfolders) are at the cited commits;
a checkout's project is the sources.toml entry whose repository has the same name. `--cache` is the source cache
(default as citations.py, `NATIVE_THEME_SOURCES` overrides it). Git runs offline (GIT_NO_LAZY_FETCH): a partial
clone's missing file is an error, never a fetch.

The report (TSV) has one row per citation in scope: document, line, citation, project, from, to, status, old lines,
new lines, note, file (the repository path at the from commit, `-> new` where it moved), apply (what `--apply` does or
did with it). Exit: 0 when no changed, vanished or error citation remains among the mapped ones, 1 otherwise, 2 on
unreadable input (sources.toml, a document, the cache).
"""
import argparse
import collections
import csv
import difflib
import os
import re
import shutil
import subprocess
import sys
import tempfile
import tomllib
from dataclasses import dataclass, field
from pathlib import Path

import citations

GIT_ENV = dict(os.environ, GIT_NO_LAZY_FETCH="1", GIT_TERMINAL_PROMPT="0")
MAPPED = ("unchanged", "moved", "changed", "vanished", "present")
STATUSES = MAPPED + ("unmappable", "error")
REWRITE = ("unchanged", "moved", "present")
PLATFORM_DOC = {"gnome": "gap-map-gnome.md", "kde": "gap-map-kde.md", "windows": "gap-map-windows.md",
                "material": "gap-map-material.md", "macos": "gap-map-macos.md", "ios": "gap-map-ios.md"}
GENERATION = {"gnome": "adwaita", "kde": "kde-breeze", "windows": "windows-11", "material": "material-3",
              "macos": "macos-tahoe", "ios": "ios-26"}
RE_ROW_FULL = re.compile(r"(?<![\w-])((?:GN|K|W\d?|MAT|MAC)-[A-Z]*\d+(?:[.-][A-Z]*\d+)*|[KQ]\d+\.\d+)")
ROW_PLATFORM = (("GN-", "gnome"), ("K", "kde"), ("Q", "kde"), ("W", "windows"), ("MAT-", "material"),
                ("MAC-", "macos"))
RE_SHA = re.compile(r"\b[0-9a-f]{7,40}\b")
RE_TAG = re.compile(r"(?<![\w.])v?\d+\.\d+(?:\.\d+)*(?![\w.]*\d)")
RE_RANGE = re.compile(r"(\d+)(?:([-–])L?(\d+))?")
RE_TOKEN_NAME = re.compile(r"[a-z][a-z0-9.-]*\*?")
FIELDS = ["document", "line", "citation", "project", "from", "to", "status", "old_lines", "new_lines", "note", "file",
          "apply"]


# -- checkouts --------------------------------------------------------------------------------------------------------

def git(path, *args):
    return subprocess.run(["git", "-C", str(path), *args], capture_output=True, text=True, env=GIT_ENV)


class Checkout:
    """A git checkout at one commit: its file list from the commit's tree, its files read offline."""

    def __init__(self, path, head, tags=(), name=""):
        self.path, self.head, self.tags, self.name = Path(path), head, set(tags), name
        self.project = ""
        self._files = None
        self._names = None
        self._proc = None

    @classmethod
    def open(cls, path):
        r = git(path, "rev-parse", "HEAD")
        if r.returncode:
            return None
        tags = git(path, "tag", "--points-at", "HEAD").stdout.split()
        url = git(path, "config", "--get", "remote.origin.url").stdout.strip()
        name = re.sub(r"\.git$", "", url.rstrip("/")).rsplit("/", 1)[-1].lower() if url else ""
        return cls(path, r.stdout.strip(), tags, name)

    @property
    def files(self):
        if self._files is None:
            out = git(self.path, "ls-tree", "-r", "--name-only", "HEAD").stdout
            self._files = set(f for f in out.split("\n") if f)
        return self._files

    def by_name(self, name):
        if self._names is None:
            self._names = collections.defaultdict(list)
            for f in self.files:
                self._names[f.rsplit("/", 1)[-1]].append(f)
        return self._names.get(name, [])

    def find(self, path):
        """The repository paths a cited path names: itself, else the files it is a suffix of."""
        path = path.strip("/")
        if path in self.files:
            return [path]
        return sorted(f for f in self.by_name(path.rsplit("/", 1)[-1]) if f.endswith("/" + path))

    def read(self, path):
        """The file's text at the commit, or None when its content is not available offline."""
        if self._proc is None:
            self._proc = subprocess.Popen(["git", "-C", str(self.path), "cat-file", "--batch"], stdin=subprocess.PIPE,
                                          stdout=subprocess.PIPE, stderr=subprocess.DEVNULL, env=GIT_ENV)
        self._proc.stdin.write(f"{self.head}:{path}\n".encode())
        self._proc.stdin.flush()
        header = self._proc.stdout.readline().decode().split()
        if len(header) != 3 or header[1] != "blob":
            return None
        data = self._proc.stdout.read(int(header[2]))
        self._proc.stdout.read(1)
        return data.decode("utf-8", "replace")

    def close(self):
        if self._proc is not None:
            self._proc.stdin.close()
            self._proc.wait()
            self._proc = None


@dataclass
class Pin:
    id: str
    project: str
    tag: str
    commit: str
    generations: list
    repo_name: str
    checkout: Checkout = None


def load_pins(sources, cache):
    data = tomllib.loads(Path(sources).read_text(encoding="utf-8"))
    pins = []
    entries = data.get("source", [])
    for e in entries if isinstance(entries, list) else []:
        if not isinstance(e, dict) or e.get("kind") != "git" or not e.get("project"):
            continue
        bad = [k for k in ("id", "project", "repository", "tag", "commit") if not isinstance(e.get(k), str)]
        if bad:
            raise ValueError(f"{sources}: [[source]] {e.get('id')!r}: {', '.join(bad)} not a string "
                             f"(check_sources.py)")
        name = re.sub(r"\.git$", "", e["repository"].rstrip("/")).rsplit("/", 1)[-1].lower()
        gens = e.get("generations") if isinstance(e.get("generations"), list) else []
        pin = Pin(e["id"], e["project"], e["tag"], e["commit"], gens, name)
        co = Checkout.open(Path(cache) / e["id"]) if (Path(cache) / e["id"]).is_dir() else None
        if co is not None and co.head == e["commit"]:
            pin.checkout = co
        pins.append(pin)
    not_pinned = {n["project"] for n in data.get("not_pinned", []) if isinstance(n, dict)
                  and isinstance(n.get("project"), str)}
    sites = {s["project"] for s in data.get("site", []) if isinstance(s, dict) and isinstance(s.get("project"), str)}
    return pins, not_pinned, sites


def load_from_checkouts(dirs, pins):
    """Every checkout under the --from folders, with its project (the pinned repository of the same name)."""
    by_name = {}
    for p in pins:
        by_name.setdefault(p.repo_name, p.project)
    out = []
    for d in dirs:
        d = Path(d)
        cands = [d] if (d / ".git").exists() else sorted(x for x in d.iterdir() if (x / ".git").exists()) \
            if d.is_dir() else []
        for c in cands:
            co = Checkout.open(c)
            if co is not None:
                co.project = by_name.get(co.name, "")
                out.append(co)
    return out


# -- refs -------------------------------------------------------------------------------------------------------------

def ref_parts(ref):
    """(SHAs, tags) a citation's ref names: `1.10.0 (c9ab1186)` -> (["c9ab1186"], ["1.10.0"])."""
    shas = RE_SHA.findall(ref or "")
    tags = [t for t in RE_TAG.findall(ref or "") if not any(t in s for s in shas)]
    return shas, tags


def same_tag(a, b):
    return a.lstrip("v") == b.lstrip("v")


@dataclass
class From:
    """A resolved from ref: the pin itself (unchanged) or a checkout at another commit."""
    kind: str            # "pin" | "checkout" | "error" | "unmappable"
    label: str           # the ref as the report prints it
    checkout: Checkout = None
    pin: Pin = None
    note: str = ""

    @property
    def key(self):
        if self.kind == "pin":
            return ("pin", self.pin.id)
        if self.kind == "checkout":
            return ("commit", self.checkout.head)
        return (self.kind, self.label)


class Resolver:
    def __init__(self, pins, checkouts):
        self.pins = collections.defaultdict(list)
        for p in pins:
            self.pins[p.project].append(p)
        self.checkouts = checkouts

    def from_ref(self, project, ref):
        if not ref:
            return From("unmappable", "", note="no commit stated")
        if "|" in ref:
            return From("unmappable", ref, note="ref depends on a section the citation is outside of")
        if citations.refkind(ref) == "branch":
            return From("unmappable", ref, note=f"a branch ({ref}), no commit")
        shas, tags = ref_parts(ref)
        if not shas and not tags:
            return From("unmappable", ref, note="ref is neither a commit nor a tag")
        for p in self.pins.get(project, []):
            if any(p.commit.startswith(s) for s in shas) or (not shas and any(same_tag(t, p.tag) for t in tags)):
                return From("pin", ref, pin=p)
        for co in self.checkouts:
            if shas and any(co.head.startswith(s) for s in shas):
                return From("checkout", ref, checkout=co)
        for co in self.checkouts:
            if not shas and co.project == project and any(same_tag(t, x) for t in tags for x in co.tags):
                return From("checkout", ref, checkout=co)
        return From("error", ref, note=f"no checkout at {ref} under --from")

    def to_pin(self, project, frm, platform):
        cands = self.pins.get(project, [])
        if frm.kind == "pin":
            return frm.pin, ""
        if not cands:
            return None, "project not pinned"
        if len(cands) == 1:
            return cands[0], ""
        _, tags = ref_parts(frm.label)
        tags += sorted(frm.checkout.tags) if frm.checkout else []
        major = {t.lstrip("v").split(".")[0] for t in tags}
        same = [p for p in cands if p.tag.lstrip("v").split(".")[0] in major]
        if len(same) == 1:
            return same[0], ""
        gen = [p for p in cands if GENERATION.get(platform) in p.generations]
        if len(gen) == 1:
            return gen[0], ""
        return None, f"several pins ({', '.join(p.id for p in cands)})"


# -- line mapping -----------------------------------------------------------------------------------------------------

def parse_ranges(lines):
    """[(first, last, span in the lines text)] of `678-695`, `38-39, :41`, `L71-L73`."""
    return [(int(m.group(1)), int(m.group(3) or m.group(1)), m.span()) for m in RE_RANGE.finditer(lines or "")]


class LineMap:
    def __init__(self, old, new):
        a, b = old.splitlines(), new.splitlines()
        self.n = len(a)
        self.map = {}          # old line (1-based) -> (new line, same text)
        sm = difflib.SequenceMatcher(None, a, b, autojunk=False)
        for tag, i1, i2, j1, j2 in sm.get_opcodes():
            for i in range(i1, i2):
                if tag == "equal":
                    self.map[i + 1] = (j1 + (i - i1) + 1, True)
                elif tag == "replace":
                    self.map[i + 1] = (min(j1 + (i - i1), j2 - 1) + 1, False)

    def range(self, first, last):
        """(status, new first, new last, note) for the old lines first..last."""
        if first < 1 or last < first or last > self.n:
            return "vanished", None, None, f"lines {first}-{last} beyond the from file's {self.n}"
        got = [self.map.get(i) for i in range(first, last + 1)]
        hit = [g for g in got if g]
        if not hit:
            return "vanished", None, None, "every cited line deleted"
        lo, hi = min(g[0] for g in hit), max(g[0] for g in hit)
        if all(g and g[1] for g in got):
            if hi - lo == last - first:
                return "moved", lo, hi, ""
            return "changed", lo, hi, "lines inserted inside the range"
        if any(g is None for g in got):
            return "changed", lo, hi, "some cited lines deleted"
        return "changed", lo, hi, "cited text differs"


def format_lines(text, new):
    """The lines text with each range's numbers replaced: new is [(first, last)] in range order."""
    out, pos = [], 0
    for (m, (lo, hi)) in zip(RE_RANGE.finditer(text), new):
        out.append(text[pos:m.start()])
        s = m.group(0)
        if m.group(3):
            s = s[:m.start(1) - m.start()] + str(lo) + s[m.end(1) - m.start():m.start(3) - m.start()] + str(hi)
        else:
            s = str(lo)
        out.append(s)
        pos = m.end()
    out.append(text[pos:])
    return "".join(out)


# -- the run ----------------------------------------------------------------------------------------------------------

@dataclass
class Result:
    c: citations.Citation
    idx: int
    status: str = ""
    project: str = ""
    frm: From = None
    pin: Pin = None
    old_path: str = ""
    new_path: str = ""
    new_ranges: list = field(default_factory=list)
    note: str = ""
    cls: str = ""          # the unmappable or error class
    anchor: str = ""       # inline | definition | head | none
    head: int = -1
    deps: list = field(default_factory=list)   # the records an implicit ref was taken from (from_row)
    moved_dir: tuple = ()  # (from folder, to folder) when the file moved
    apply: str = ""
    applied: bool = False


def row_platform(row_id):
    return next((p for prefix, p in ROW_PLATFORM if row_id.startswith(prefix)), "")


def platform_of(doc):
    for p, d in PLATFORM_DOC.items():
        if doc == d or doc == f"spec-trace-{p}.md":
            return p
    return ""


def def_locations(c):
    """The (document, line) pairs of the definition a citation's ref comes from, or []."""
    m = c.method
    if "ref by section heading" in m:
        s = re.search(r"seeds :(\d+)", c.note)
        return [("gap-map-seeds.md", int(s.group(1)))] if s else []
    if not re.match(r"(path-prefix abbreviation|line abbreviation|word abbreviation|project word; commit from|"
                    r"document citation base)", m):
        return []
    return [(doc, i) for doc, a, b in citations.locations(m[m.find("(") + 1:]) for i in range(a, b + 1)]


def cited_rel(c):
    """The path the citation's text states, without its abbreviation: what must still name the file at the pin."""
    base = c.path.strip("/")
    if base in citations.LINE_ABBR:
        return citations.LINE_ABBR[base][4]
    for a in citations.PATH_ABBR:
        if base.startswith(a[0]):
            return base[len(a[0]):]
    return base


def anchor_of(c, line_text):
    """inline (the citation carries its ref), definition, head (carried or continued) or none (implicit)."""
    s, m = c.syntax, c.method
    if s.startswith(":L continuation") or m.startswith("carried"):
        return "head"
    if m in ("URL", "host path", "file@sha", "inline project@sha prefix") or s.startswith("file:L at"):
        return "inline"
    if m.startswith("project word + tag"):
        return "inline"
    if m.startswith("word abbreviation"):
        w = m[m.find("(") + 1:].split(",")[0]
        if re.search(r"(?<![\w-])" + re.escape(w) + r" v?\d+\.\d+(?:\.\d+)?(?: tree)?\s*`?" + re.escape(c.path),
                     line_text):
            return "inline"
    if def_locations(c):
        return "definition"
    return "none"


def sentence_of(lines, ln, needle):
    """A table row's whole line; in prose, the sentence of the paragraph that holds needle on line ln."""
    text = lines[ln - 1]
    if text.lstrip().startswith("|"):
        return text

    def item(s):
        return bool(re.match(r"\s*(?:[-*]|\d+\.)\s", s))

    def brk(s):
        return not s.strip() or s.startswith(("#", "|"))

    a = ln - 1
    while a > 0 and not item(lines[a]) and not brk(lines[a - 1]):
        a -= 1
    b = ln
    while b < len(lines) and not brk(lines[b]) and not item(lines[b]):
        b += 1
    para = " ".join(lines[a:b])
    pos = sum(len(x) + 1 for x in lines[a:ln - 1]) + max(0, text.find(needle))
    starts = [0] + [m.end() for m in re.finditer(r"[.!?;]\s+(?=[A-Z*(`])", para)]
    st = max(s for s in starts if s <= pos)
    en = min([s for s in starts if s > pos] + [len(para)])
    return para[st:en]


class Run:
    def __init__(self, docs, pins, not_pinned, sites, checkouts, index):
        self.docs = docs                      # short name -> text
        self.lines = {d: t.split("\n") for d, t in docs.items()}
        self.not_pinned, self.sites = not_pinned, sites
        self.resolver = Resolver(pins, checkouts)
        self.maps = {}
        self.records = []
        defs = citations.definitions(docs)
        for d, t in docs.items():
            self.records += citations.scan(t, d, index, defs)
        self.results = []
        self.platform_refs = {}

    # in scope: a citation of code a pin covers (or could: no project or several)
    def in_scope(self, c):
        if c.quote:
            return False
        if c.kind not in ("code repo", "") or (c.kind == "" and not c.syntax.startswith(":L")):
            return False
        projs = citations.projects(c)
        if not projs:
            return True
        pinned = set(self.resolver.pins)
        return any(p in pinned for p in projs) or not all(
            p in self.not_pinned or p in self.sites or p.startswith((citations.OWN, "toolkit")) for p in projs)

    def platform_sources(self, doc):
        """For a platform gap map: project -> from key -> (from, records), file name -> (project, from key) ->
        (project, from, records), and per row ID the (project, from, file name, record) of the citations on its table
        row; records are indices into self.records."""
        if doc in self.platform_refs:
            return self.platform_refs[doc]
        by_proj, by_name = collections.defaultdict(dict), collections.defaultdict(dict)
        by_line = collections.defaultdict(list)
        for i, c in enumerate(self.records):
            if c.doc != doc or c.quote or "|" in c.project or not c.project or not c.ref:
                continue
            if c.status not in ("resolved", "heuristic") or c.kind != "code repo":
                continue
            f = self.resolver.from_ref(c.project, c.ref)
            if f.kind not in ("pin", "checkout"):
                continue
            by_proj[c.project].setdefault(f.key, (f, []))[1].append(i)
            name = c.file.rsplit("/", 1)[-1] if c.file else ""
            if name:
                by_name[name].setdefault((c.project, f.key), (c.project, f, []))[2].append(i)
            by_line[c.line].append((c.project, f, name, i))
        rows = {}
        for ln, s in enumerate(self.lines[doc], 1):
            m = re.match(r"\|\s*`?([^|`†]+?)†?`?\s*\|", s)
            if m and RE_ROW_FULL.fullmatch(m.group(1).strip()):
                rows.setdefault(m.group(1).strip(), by_line.get(ln, []))
        self.platform_refs[doc] = (by_proj, by_name, rows)
        return self.platform_refs[doc]

    def from_row(self, c):
        """(project, From, note, records) from the row's platform gap map, or (None, None, why, []): the row's own
        citations of the file name (or of the project) first, then the whole file's; records are the citations the
        ref was taken from."""
        lines = self.lines[c.doc]
        needle = c.path.rsplit("/", 1)[-1] if c.path else (":" + c.lines)
        sent = sentence_of(lines, c.line, needle)
        ids = collections.defaultdict(set)
        for m in RE_ROW_FULL.finditer(sent):
            ids[row_platform(m.group(1))].add(m.group(1))
        ids.pop("", None)
        if not ids:
            return None, None, "no row ID in its table row or sentence", []
        name = c.file.rsplit("/", 1)[-1] if c.file else ""
        projs = set(citations.projects(c))
        answers = {}
        for p, rowids in sorted(ids.items()):
            doc = PLATFORM_DOC[p]
            if doc not in self.docs:
                continue
            by_proj, by_name, rows = self.platform_sources(doc)
            on_rows = [x for i in sorted(rowids) for x in rows.get(i, [])]
            hits = {}
            for pr, f, n, i in on_rows:
                if (name and n == name or not name) and (not projs or pr in projs):
                    hits.setdefault((pr, f.key), (pr, f, []))[2].append(i)
            where = f"{doc} row {', '.join(sorted(rowids))}"
            if len(hits) != 1:
                hits = {k: v for k, v in by_name.get(name, {}).items() if not projs or v[0] in projs} if name else {}
                where = doc
            if len(hits) == 1:
                proj, f, recs = next(iter(hits.values()))
                answers[(proj, f.key)] = (proj, f, f"ref from {where}, which cites {name or proj} at that ref only",
                                          recs)
            elif len(projs) == 1 and len(by_proj.get(next(iter(projs)), {})) == 1:
                proj = next(iter(projs))
                f, recs = next(iter(by_proj[proj].values()))
                answers[(proj, f.key)] = (proj, f, f"ref from {doc}, which cites {proj} at that ref only", recs)
        if len(answers) == 1:
            return next(iter(answers.values()))
        if not answers:
            return None, None, f"the row's platform file ({', '.join(PLATFORM_DOC[p] for p in sorted(ids))}) " \
                               f"names no single source for it", []
        return None, None, "the row IDs' platform files disagree", []

    def lmap(self, frm, fpath, pin, tpath):
        key = (frm.head, fpath, pin.commit, tpath)
        if key not in self.maps:
            a, b = frm.read(fpath), pin.checkout.read(tpath)
            self.maps[key] = None if a is None or b is None else LineMap(a, b)
        return self.maps[key]

    def classify(self, r):
        c = r.c
        projs = citations.projects(c)
        frm, project = None, ""
        if not c.file and not c.lines and len(projs) != 1 or c.syntax.startswith(("brace-list", "elided")):
            r.status, r.project, r.note = "unmappable", c.project, "the citation gives no file path"
            r.cls = "brace list" if c.syntax.startswith("brace-list") else "elided path" \
                if c.syntax.startswith("elided") else "no path"
            return
        if c.syntax.startswith(":L") and not projs:
            r.status, r.cls, r.note = "unmappable", "continuation without a head", "no citation before it to continue"
            return
        if len(projs) == 1 and c.ref:
            project = projs[0]
            frm = self.resolver.from_ref(project, c.ref)
        if frm is None or (frm.kind == "unmappable" and "section" in frm.note):
            p2, f2, why, deps = self.from_row(c)
            if f2 is not None:
                project, frm = p2, f2
                r.note, r.deps = why, deps
            elif frm is None:
                r.status, r.project = "unmappable", c.project
                r.cls = ("no project" if not projs else "several projects" if len(projs) > 1 else "no commit")
                r.note = why
                if not c.lines and len(projs) == 1:
                    self.presence_without_from(r, projs[0])
                return
            else:
                r.note = why
        r.project, r.frm = project, frm
        if frm.kind == "unmappable":
            r.status, r.cls, r.note = "unmappable", frm.note.split(" (")[0], frm.note
            return
        if frm.kind == "error":
            r.status, r.cls, r.note = "error", "no from checkout", frm.note
            return
        pin, why = self.resolver.to_pin(project, frm, platform_of(c.doc))
        if pin is None:
            r.status, r.cls, r.note = "unmappable", why.split(" (")[0], why
            return
        r.pin = pin
        if not c.file:
            r.status, r.cls, r.note = "unmappable", "no path", "the citation gives no file path (elided)"
            return
        if frm.kind == "pin":
            r.status = "unchanged"
            r.old_path = r.new_path = c.file
            if pin.checkout is not None and not pin.checkout.find(c.file):
                r.note = (r.note + "; " if r.note else "") + "path not found at the pin"
            return
        if pin.checkout is None:
            r.status, r.cls, r.note = "error", "no pin checkout", f"{pin.id} not in the source cache at its commit"
            return
        co = frm.checkout
        found = co.find(c.file)
        if not found and c.path != c.file:
            found = co.find(c.path)
        if len(found) > 1:
            r.status, r.cls = "unmappable", "file name names several files"
            r.note = f"{c.file} names {len(found)} files at {co.head[:8]}"
            return
        if not found:
            r.status, r.cls, r.note = "error", "path not in the from commit", f"{c.file} not at {co.head[:8]}"
            return
        r.old_path = found[0]
        tp = self.to_path(r, co, pin)
        if tp is None:
            r.status, r.note = "vanished", (r.note + "; " if r.note else "") + f"{r.old_path} gone at {pin.id}"
            return
        r.new_path = tp
        if not c.lines:
            self.presence(r, pin)
            return
        lm = self.lmap(co, r.old_path, pin, tp)
        if lm is None:
            r.status, r.cls, r.note = "error", "content not available", f"{r.old_path} not readable offline"
            return
        sts, notes = [], []
        for first, last, _ in parse_ranges(c.lines):
            st, lo, hi, n = lm.range(first, last)
            sts.append(st)
            r.new_ranges.append((lo, hi))
            if n:
                notes.append(f"{first}-{last}: {n}" if first != last else f"{first}: {n}")
        if not sts:
            r.status, r.cls, r.note = "unmappable", "no line numbers", "line text not understood"
            return
        r.status = "moved" if all(s == "moved" for s in sts) else "vanished" if all(s == "vanished" for s in sts) \
            else "changed"
        if tp != r.old_path:
            notes.append(f"file now {tp}")
            a, b = r.old_path.split("/"), tp.split("/")
            k = 0
            while k < min(len(a), len(b)) and a[-1 - k] == b[-1 - k]:
                k += 1
            r.moved_dir = ("/".join(a[:len(a) - k]) + "/", "/".join(b[:len(b) - k]) + "/")
            if not tp.endswith(cited_rel(c)):
                if r.status == "moved":
                    r.status = "changed"
                notes.append("the citation's path text no longer names it")
        r.note = "; ".join(x for x in [r.note] + notes if x)

    def to_path(self, r, co, pin):
        p = r.old_path
        if p in pin.checkout.files:
            return p
        cands = pin.checkout.by_name(p.rsplit("/", 1)[-1])
        if not cands:
            return None
        if len(cands) == 1:
            return cands[0]
        old = co.read(p) or ""
        best = max(cands, key=lambda f: difflib.SequenceMatcher(None, old, pin.checkout.read(f) or "").quick_ratio())
        ratio = difflib.SequenceMatcher(None, old, pin.checkout.read(best) or "").ratio()
        return best if ratio >= 0.5 else None

    def presence(self, r, pin):
        text = pin.checkout.read(r.new_path)
        missing = [t for t in self.token_names(r.c) if text is not None and t.rstrip("*").replace(".", "-") not in
                   text and t.rstrip("*") not in text]
        if missing:
            r.status, r.note = "vanished", f"token {', '.join(missing)} not in {r.new_path} at {pin.id}"
        else:
            r.status = "present"

    def presence_without_from(self, r, project):
        """A file mention needs no from commit: present / vanished at the project's one pin."""
        cands = self.resolver.pins.get(project, [])
        if len(cands) != 1 or cands[0].checkout is None or not r.c.file:
            return
        pin = cands[0]
        found = pin.checkout.find(r.c.file)
        r.project, r.pin, r.cls = project, pin, ""
        r.frm = From("unmappable", "", note="no commit stated")
        r.note = "no from commit (a file mention needs none)"
        if not found:
            r.status = "vanished"
            r.note += f"; {r.c.file} not at {pin.id}"
            return
        r.old_path = r.new_path = found[0]
        self.presence(r, pin)

    def token_names(self, c):
        text = self.lines[c.doc][c.line - 1]
        m = re.search(re.escape(c.path) + r"`?\s*\(((?:`[^`]+`(?:,\s*)?)+)\)", text)
        if not m or not c.path.endswith(".scss"):
            return []
        return [t for t in re.findall(r"`([^`]+)`", m.group(1)) if RE_TOKEN_NAME.fullmatch(t)]

    def run(self):
        for i, c in enumerate(self.records):
            if self.in_scope(c):
                r = Result(c, i)
                self.classify(r)
                r.anchor = anchor_of(c, self.lines[c.doc][c.line - 1])
                self.results.append(r)
        self.link_heads()
        self.plan_apply()
        return self.results

    def link_heads(self):
        last = {}
        by_idx = {}
        for r in self.results:
            by_idx[r.idx] = r
        for i, c in enumerate(self.records):
            r = by_idx.get(i)
            key = (c.doc, c.project, c.ref)
            if r is not None and r.anchor == "head":
                h = last.get(key)
                if h is not None:
                    r.head = h
                    hr = by_idx.get(h)
                    if hr is not None and hr.anchor == "head" and hr.head >= 0:
                        r.head = hr.head
            elif c.project and c.ref and not c.syntax.startswith(":L") and not c.method.startswith("carried"):
                last[key] = i
        self.by_idx = by_idx

    def plan_apply(self):
        # a citation that carries its ref and those that follow it are rewritten together or not at all
        groups = collections.defaultdict(list)
        for r in self.results:
            if r.anchor == "head" and r.head in self.by_idx:
                groups[r.head].append(r)
        blocked = {}
        for h, members in groups.items():
            head = self.by_idx[h]
            if head.anchor != "inline":
                continue
            bad = [m for m in [head] + members if m.status not in REWRITE or
                   (m.frm is not None and head.frm is not None and m.frm.key != head.frm.key)]
            if bad:
                why = f"keep: shares its ref with a {bad[0].status} citation (line {bad[0].c.line})"
                for m in [head] + members:
                    blocked[m.idx] = why
        for r in self.results:
            if r.status not in REWRITE:
                r.apply = "keep"
                continue
            if r.status == "unchanged":
                r.apply = "nothing to rewrite (same commit)"
                continue
            if r.idx in blocked:
                r.apply = blocked[r.idx]
                continue
            if r.anchor == "head":
                h = self.by_idx.get(r.head)
                r.apply = "lines (implicit ref)" if h is None else "lines (follows its head)"
                if r.status != "moved":
                    r.apply = "nothing to rewrite"
            elif r.anchor == "inline":
                r.apply = "lines and ref" if r.status == "moved" else "ref"
            elif r.anchor == "definition":
                r.apply = ("lines; " if r.status == "moved" else "") + "definition " + \
                    ", ".join(f"{d}:{n}" for d, n in sorted(set(def_locations(r.c)))[:3])
            else:
                r.apply = "lines (implicit ref)" if r.status == "moved" else "nothing to rewrite"


# -- --apply ----------------------------------------------------------------------------------------------------------

def masked_spans(text):
    spans = [m.span() for m in citations.RE_MDLINK.finditer(text)]
    spans += [m.span() for m in citations.RE_URL.finditer(text)]
    return spans


def inside(span, spans):
    return any(a <= span[0] and span[1] <= b for a, b in spans)


def new_ref(old, pin):
    """The pin in the old ref's form: a SHA of the same length, or the tag with the old one's `v` style."""
    if re.fullmatch(r"[0-9a-f]{7,40}", old):
        return pin.commit[:len(old)]
    if pin.tag == "commit":
        return pin.commit[:8]
    tag = pin.tag.lstrip("v") if "/" not in pin.tag else pin.tag
    return ("v" + tag if old.startswith("v") and "/" not in pin.tag else tag)


class Editor:
    def __init__(self, run):
        self.run = run
        self.edits = collections.defaultdict(list)    # (doc, line) -> [(start, end, text)]
        self.used = collections.Counter()             # (doc, line, kind, key) occurrences consumed

    def add(self, doc, ln, start, end, text):
        lst = self.edits[(doc, ln)]
        if any(not (end <= a or start >= b) for a, b, _ in lst):
            return False
        lst.append((start, end, text))
        return True

    def nth(self, doc, ln, kind, key, matches):
        k = self.used[(doc, ln, kind, key)]
        self.used[(doc, ln, kind, key)] += 1
        return matches[k] if k < len(matches) else None

    def token_matches(self, text, c):
        tail = r":(?P<lines>" + re.escape(c.lines) + r")(?!\d|[-–]\d)" if c.lines else r"(?P<lines>)(?![\w/.:@-])"
        rx = re.compile(r"(?<![\w/.@#-])" + re.escape(c.path) + r"(?P<glued>@[0-9a-f]{7,40})?" + tail)
        masks = masked_spans(text)
        return [m for m in rx.finditer(text) if not inside(m.span(), masks)]

    def cont_matches(self, text, c):
        masks = masked_spans(text)
        for m in citations.RE_TOKEN.finditer(text):
            if m.group("lines") and ("/" in m.group("path") or "." in m.group("path") or
                                     m.group("path") in citations.LINE_ABBR):
                masks.append(m.span())
        return [m for m in citations.RE_CONT.finditer(text) if m.group("lines").lstrip(":") == c.lines
                and not inside(m.span(), masks)]

    def url_matches(self, text, c, md):
        out = []
        if md:
            for m in citations.RE_MDLINK.finditer(text):
                k = citations.classify_url(m.group(2))
                if k[0] == "code" and (k[1], k[2], k[3], k[4]) == (c.project, c.ref, c.path, c.lines):
                    out.append(m)
        else:
            mds = [m.span() for m in citations.RE_MDLINK.finditer(text)]
            for m in citations.RE_URL.finditer(text):
                url = m.group(0).rstrip(".,;:")
                k = citations.classify_url(url)
                if not inside(m.span(), mds) and k[0] == "code" and (k[1], k[2], k[3], k[4]) == \
                        (c.project, c.ref, c.path, c.lines):
                    out.append(m)
        return out

    def citation(self, r):
        """Rewrite one citation's lines and inline ref; False when it cannot be found in its line."""
        c, doc, ln = r.c, r.c.doc, r.c.line
        text = self.run.lines[doc][ln - 1]
        lines_new = format_lines(c.lines, r.new_ranges) if r.status == "moved" else c.lines
        do_ref = r.anchor == "inline"
        if c.method in ("URL", "host path"):
            ms = self.url_matches(text, c, c.syntax.startswith("md-link"))
            m = self.nth(doc, ln, "url", (c.project, c.ref, c.path, c.lines, c.syntax[:6]), ms)
            if m is None:
                return False
            s = m.group(0)
            url_at = s.find("://") if c.syntax.startswith("md-link") else 0
            url_at = s.rfind("(", 0, url_at) + 1 if c.syntax.startswith("md-link") else 0
            url = s[url_at:]
            head, sep, anchor = url.partition("#")
            if do_ref and c.ref:
                head = head.replace("/" + c.ref + "/", "/" + new_ref(c.ref, r.pin) + "/", 1)
            if anchor and r.status == "moved":
                anchor = format_lines(anchor, r.new_ranges)
            label = s[:url_at]
            if r.status == "moved" and c.syntax.startswith("md-link"):
                plain = c.lines.replace("L", "")
                lm = re.search(r":(" + re.escape(plain) + r")\]\($", label)
                if lm:
                    label = label[:lm.start(1)] + format_lines(plain, r.new_ranges) + label[lm.end(1):]
            return self.add(doc, ln, m.start(), m.end(), label + head + sep + anchor)
        if c.syntax.startswith(":L"):
            m = self.nth(doc, ln, "cont", c.lines, self.cont_matches(text, c))
            if m is None:
                return False
            a, b = m.span("lines")
            return self.add(doc, ln, a + 1, b, lines_new)
        m = self.nth(doc, ln, "tok", (c.path, c.lines), self.token_matches(text, c))
        if m is None:
            return False
        ok = True
        if r.status == "moved":
            ok = self.add(doc, ln, m.start("lines"), m.end("lines"), lines_new)
        if do_ref and ok:
            ok = self.inline_ref(text, doc, ln, m, c, r)
        return ok

    def inline_ref(self, text, doc, ln, m, c, r):
        if m.group("glued"):
            a, b = m.span("glued")
            return self.add(doc, ln, a + 1, b, new_ref(m.group("glued")[1:], r.pin))
        start = m.start()
        before = text[max(0, start - 80):start]
        off = max(0, start - 80)
        if c.method == "inline project@sha prefix":
            mm = None
            for x in citations.RE_PROJSHA.finditer(before):
                mm = x
            if mm and before[mm.end():].strip(" `") == "":
                return self.add(doc, ln, off + mm.start("sha"), off + mm.end("sha"), new_ref(mm.group("sha"), r.pin))
            return False
        if c.syntax.startswith("file:L at"):
            after = text[m.end():m.end() + 80]
            t = citations.RE_TRAILING_REF.match(after)
            if t:
                g = 1 if t.group(1) else 2
                return self.add(doc, ln, m.end() + t.start(g), m.end() + t.end(g), new_ref(t.group(g), r.pin))
            return False
        wm = citations.RE_WORD.search(before)
        if wm and wm.group("ver"):
            v = wm.group("ver").strip()
            a = wm.start("ver") + wm.group("ver").find(v)
            return self.add(doc, ln, off + a, off + a + len(v), new_ref(v, r.pin))
        return False

    def definition(self, doc, ln, frm, pin):
        """Rewrite the from ref at a definition line: SHA prefixes and tags; add the tag after a lone SHA."""
        if doc not in self.run.lines or ln > len(self.run.lines[doc]):
            return 0
        text = self.run.lines[doc][ln - 1]
        full = frm.checkout.head
        tags = [t for t in ref_parts(frm.label)[1]] + sorted(frm.checkout.tags)
        n = 0
        last_sha_end = None
        for m in RE_SHA.finditer(text):
            if full.startswith(m.group(0)):
                if self.add(doc, ln, m.start(), m.end(), pin.commit[:len(m.group(0))]):
                    n += 1
                    last_sha_end = m.end() + (1 if text[m.end():m.end() + 1] == "`" else 0)
        tag_hit = False
        for t in set(tags):
            for m in re.finditer(r"(?<![\w.])" + re.escape(t) + r"(?![\w]|\.\d)", text):
                if self.add(doc, ln, m.start(), m.end(), new_ref(t, pin)):
                    n += 1
                    tag_hit = True
        if n and not tag_hit and last_sha_end is not None and pin.tag not in text and pin.tag != "commit":
            if self.add(doc, ln, last_sha_end, last_sha_end, f" (tag `{pin.tag}`)"):
                n += 1
        return n

    def write(self):
        out = {}
        for doc, lines in self.run.lines.items():
            new = list(lines)
            for (d, ln), lst in self.edits.items():
                if d != doc:
                    continue
                s = new[ln - 1]
                for a, b, t in sorted(lst, key=lambda e: (-e[0], -e[1])):
                    s = s[:a] + t + s[b:]
                new[ln - 1] = s
            out[doc] = "\n".join(new)
        return out


def definition_of(run, r):
    """The definition lines a citation's ref comes from: its own, or its head's for a carried path or continuation."""
    if r.anchor == "head":
        h = run.by_idx.get(r.head)
        return def_locations(h.c) if h is not None and h.anchor == "definition" else []
    return def_locations(r.c) if r.anchor == "definition" else []


def apply(run):
    """Plan every edit; returns the documents' new texts and the definitions rewritten."""
    ed = Editor(run)
    defs = collections.defaultdict(dict)   # (doc, line) -> {(from key, pin id): (from, pin)}
    # a citation whose implicit ref comes from other citations (from_row) goes last: it is rewritten only when they
    # are, so that a later run reads its ref at the pin too
    for r in sorted(run.results, key=lambda r: r.anchor == "none" and bool(r.deps)):
        if r.status not in REWRITE or r.apply.startswith(("keep", "nothing")):
            r.applied = False
            continue
        h = run.by_idx.get(r.head) if r.anchor == "head" else None
        if h is not None and h.anchor == "inline" and not h.applied:
            r.apply = f"keep: its head (line {h.c.line}) was not rewritten"
            continue
        held = [d for d in r.deps if r.anchor == "none" and r.frm is not None and r.frm.kind == "checkout"
                and not (run.by_idx.get(d) is not None and run.by_idx[d].applied)]
        if held:
            src = run.records[held[0]]
            r.apply = f"keep: {src.doc}:{src.line}, which its ref comes from, was not rewritten"
            continue
        ok = ed.citation(r) if (r.status == "moved" or r.anchor == "inline") else True
        if not ok:
            r.apply = "keep: not found in its line"
            continue
        r.applied = True
        if r.frm is not None and r.frm.kind == "checkout":
            for loc in definition_of(run, r):
                defs[loc][(r.frm.key, r.pin.id)] = (r.frm, r.pin)
    done = {}
    for (doc, ln), pairs in sorted(defs.items(), key=lambda x: x[0]):
        for frm, pin in pairs.values():
            n = ed.definition(doc, ln, frm, pin)
            if n:
                done[(doc, ln)] = done.get((doc, ln), 0) + n
    for r in run.results:
        if r.status in ("changed", "vanished") and r.frm is not None and \
                any(loc in done for loc in definition_of(run, r)):
            r.apply = "keep; its definition now names the pin, its lines are still the from commit's"
    return ed.write(), done


def remaining_mentions(texts, results):
    """Lines that still name a from commit some citation was re-pinned from."""
    shas = {r.frm.checkout.head for r in results if r.applied and r.frm is not None and r.frm.kind == "checkout"}
    out = collections.Counter()
    for doc, text in texts.items():
        for ln, s in enumerate(text.split("\n"), 1):
            for m in RE_SHA.finditer(s):
                if any(h.startswith(m.group(0)) for h in shas):
                    out[(doc, ln)] += 1
    return out


# -- output -----------------------------------------------------------------------------------------------------------

def fmt_ranges(r):
    return ", ".join("-" if lo is None else (f"{lo}-{hi}" if hi != lo else str(lo)) for lo, hi in r.new_ranges)


def write_report(path, results):
    with open(path, "w", newline="", encoding="utf-8") as f:
        w = csv.writer(f, delimiter="\t", quoting=csv.QUOTE_NONE, escapechar="\\", lineterminator="\n")
        w.writerow(FIELDS)
        for r in results:
            c = r.c
            frm = r.frm.label if r.frm else c.ref
            if r.frm is not None and r.frm.kind == "checkout":
                frm = f"{frm} ({r.frm.checkout.head[:8]})" if r.frm.checkout.head[:8] not in frm else frm
            note = r.note + (f" [{r.cls}]" if r.cls else "")
            w.writerow([c.doc, c.line, c.raw.replace("\t", " "), r.project or c.project, frm, r.pin.id if r.pin else "",
                        r.status, c.lines, fmt_ranges(r) if r.status in ("moved", "changed") else "", note,
                        r.old_path + (f" -> {r.new_path}" if r.new_path and r.new_path != r.old_path else ""), r.apply])


def summary(results, out_of_scope, applied=None, defs=None, remaining=None):
    docs = sorted({r.c.doc for r in results})
    print(f"# {len(results)} citations in scope, {out_of_scope} out of scope (documentation, own, toolkit, "
          f"not pinned, quotes)")
    head = "document".ljust(24) + "".join(s[:10].rjust(11) for s in STATUSES)
    print(head)
    tot = collections.Counter()
    for d in docs:
        cnt = collections.Counter(r.status for r in results if r.c.doc == d)
        tot.update(cnt)
        print(d.ljust(24) + "".join(str(cnt.get(s, 0)).rjust(11) for s in STATUSES))
    print("all".ljust(24) + "".join(str(tot.get(s, 0)).rjust(11) for s in STATUSES))
    for st in ("unmappable", "error"):
        cls = collections.Counter(r.cls for r in results if r.status == st)
        if cls:
            print(f"# {st} by class: " + "; ".join(f"{k} {v}" for k, v in cls.most_common()))
    moved = collections.Counter((r.project, r.moved_dir) for r in results if r.moved_dir and r.status == "moved")
    for (proj, (a, b)), n in sorted(moved.items(), key=lambda x: -x[1]):
        print(f"# folder moved at the pin: {proj} {a if a != '/' else '(root)'} -> {b if b != '/' else '(root)'} "
              f"({n} moved citations; a definition naming the old folder needs the new one)")
    held = sum(1 for r in results if r.status in REWRITE and r.apply.startswith("keep"))
    if held:
        print(f"# held (mapped, not rewritten): {held}")
    if applied is not None:
        print(f"# --apply: {applied} citations rewritten, {len(defs)} definition lines rewritten:")
        for d, ln in sorted(defs):
            print(f"#   {d}:{ln}")
        if remaining:
            per = collections.Counter(d for d, _ in remaining)
            print(f"# lines still naming a re-pinned from commit: {len(remaining)} ("
                  + ", ".join(f"{d} {n}" for d, n in sorted(per.items())) + ")")
            for (d, ln), _ in sorted(remaining.items())[:60]:
                print(f"#   {d}:{ln}")


def exit_code(results):
    return 1 if any(r.status in ("changed", "vanished", "error") for r in results) else 0


def build_index(pins, checkouts, repo=citations.REPO):
    index = citations.Index.of_repo(repo)
    for p in pins:
        if p.checkout is not None:
            index.add(p.project, p.checkout.files, p.commit)
    for co in checkouts:
        if co.project:
            index.add(co.project, co.files)
    return index


def main(argv):
    ap = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    ap.add_argument("--self-test", action="store_true")
    ap.add_argument("--docs", default=None)
    ap.add_argument("--from", dest="from_dirs", action="append", default=[])
    ap.add_argument("--cache", default=None)
    ap.add_argument("--sources", default=None)
    ap.add_argument("--report", default=None)
    ap.add_argument("--apply", action="store_true")
    a = ap.parse_args(argv[1:])
    if a.self_test:
        return self_test()
    repo = citations.REPO
    cache = citations.cache_dir(a.cache)
    sources = Path(a.sources) if a.sources else repo / citations.SOURCES_TOML
    docs_dir = Path(a.docs) if a.docs else repo / "docs"
    try:
        if not cache.is_dir():
            raise OSError(f"{cache}: no source cache")
        pins, not_pinned, sites = load_pins(sources, cache)
        docs = {}
        for k in citations.REPIN_SCOPE:
            p = docs_dir / Path(citations.DOCUMENTS[k]).name
            docs[citations.short_name(p)] = p.read_text(encoding="utf-8")
    except (OSError, UnicodeDecodeError, ValueError) as e:
        print(f"repin_citations: {e}", file=sys.stderr)
        return 2
    checkouts = load_from_checkouts(a.from_dirs, pins)
    run = Run(docs, pins, not_pinned, sites, checkouts, build_index(pins, checkouts, repo))
    results = run.run()
    out_of_scope = len(run.records) - len(results)
    applied = defs = remaining = None
    if a.apply:
        texts, defs = apply(run)
        applied = sum(1 for r in results if r.applied)
        for k in citations.REPIN_SCOPE:
            p = docs_dir / Path(citations.DOCUMENTS[k]).name
            new = texts[citations.short_name(p)]
            if new != docs[citations.short_name(p)]:
                p.write_text(new, encoding="utf-8")
        remaining = remaining_mentions(texts, results)
    if a.report:
        write_report(a.report, results)
    summary(results, out_of_scope, applied, defs, remaining)
    for co in checkouts + [p.checkout for p in pins if p.checkout]:
        co.close()
    return exit_code(results)


# -- self-test --------------------------------------------------------------------------------------------------------

OLD = [f"line {i}" for i in range(1, 13)]
NEW = ["new a", "new b"] + OLD[:6] + ["line 7 edited"] + OLD[7:]          # +2; line 7 changed
GONE = ["x 1", "x 2", "x 3"]
TOKENS = ["$tokens: (", "  'container-height': 40px,", "  'label-text-font': x,", ");"]


def make_repo(path, files_a, files_b):
    """A repository with two commits; returns their SHAs."""
    path.mkdir(parents=True)
    env = dict(GIT_ENV, GIT_AUTHOR_NAME="t", GIT_AUTHOR_EMAIL="t@t", GIT_COMMITTER_NAME="t", GIT_COMMITTER_EMAIL="t@t",
               GIT_AUTHOR_DATE="2026-01-01T00:00:00", GIT_COMMITTER_DATE="2026-01-01T00:00:00")

    def g(*args):
        subprocess.run(["git", "-C", str(path), "-c", "commit.gpgsign=false", "-c", "core.hooksPath=/dev/null", *args],
                       check=True, capture_output=True, env=env)

    def put(files):
        for f in path.glob("*"):
            if f.name != ".git":
                shutil.rmtree(f) if f.is_dir() else f.unlink()
        for name, lines in files.items():
            (path / name).parent.mkdir(parents=True, exist_ok=True)
            (path / name).write_text("\n".join(lines) + "\n")
        g("add", "-A")
        g("commit", "-q", "-m", "c")
        return subprocess.run(["git", "-C", str(path), "rev-parse", "HEAD"], capture_output=True, text=True,
                              env=env).stdout.strip()

    g("init", "-q")
    g("remote", "add", "origin", "https://github.com/KDE/breeze.git")
    return put(files_a), put(files_b)


SELF_TEST_DOCS = {
    # the KDE gap map: line 3 is the definition table row of the fixture's abbreviation `zz`
    "gap-map-kde.md": """## Sources

| `zz` | `kstyle/a.cpp` | KDE/breeze @ `{A8}` |

| ID | Source |
|---|---|
| K1.1 | `breeze@{A8} kstyle/a.cpp:3-4` |
| K1.2 | `breeze@{A8} kstyle/a.cpp:7` |
| K1.3 | `breeze@{A8} kstyle/gone.cpp:2` |
| K1.4 | `breeze@{A8} kstyle/a.cpp` |
| K1.5 | `breeze@{A8} kstyle/gone.cpp` |
| K1.6 | [a.cpp:3](https://github.com/KDE/breeze/blob/{A40}/kstyle/a.cpp#L3) |
| K1.7 | `breeze@{A8} kstyle/a.cpp:1`, `:2` |
| K1.8 | `zz:5` and `zz:7` |
| K1.9 | `breeze@{A8} kstyle/a.cpp:6-8` |
| K1.10 | `breeze@{A8} kstyle/a.cpp:9`; `kstyle/b.cpp:2` |
| K1.11 | [a.cpp:3](https://github.com/KDE/breeze/blob/master/kstyle/a.cpp#L3) |
| K1.12 | `breeze@deadbeef kstyle/a.cpp:3` |
| K1.13 | `breeze@{A8} kstyle/m.scss` (`container-height`) |
| K1.14 | `breeze@{A8} kstyle/m.scss` (`container-width`) |
| K1.15 | `breeze@{A8} kstyle/a.cpp:3-4`, `kstyle/a.cpp:3` |
| K1.16 | `zz:9` |
""",
    "gap-map.md": "Here `breeze@{B8} kstyle/a.cpp:5` is at the pin.\n",
    "spec-widgets.md": """| entry | source |
|---|---|
| x | K1.1: `a.cpp:10-11` |
| y | no row: `a.cpp:4` |
| z | `nosuch.cpp:3` |
| w | K1.3: `b.cpp:3` |
| v | K1.10: `a.cpp:5` |
""",
}

SELF_TEST_WANT = [
    # (document, line, status, new lines)
    ("gap-map-kde.md", 3, "present", ""),
    ("gap-map-kde.md", 7, "moved", "5-6"),
    ("gap-map-kde.md", 8, "changed", "9"),
    ("gap-map-kde.md", 9, "vanished", ""),
    ("gap-map-kde.md", 10, "present", ""),
    ("gap-map-kde.md", 11, "vanished", ""),
    ("gap-map-kde.md", 12, "moved", "5"),
    ("gap-map-kde.md", 13, "moved", "3"),
    ("gap-map-kde.md", 13, "moved", "4"),
    ("gap-map-kde.md", 14, "moved", "7"),
    ("gap-map-kde.md", 14, "changed", "9"),
    ("gap-map-kde.md", 15, "changed", "8-10"),
    ("gap-map-kde.md", 16, "moved", "11"),
    ("gap-map-kde.md", 16, "changed", "2"),
    ("gap-map-kde.md", 17, "unmappable", ""),
    ("gap-map-kde.md", 18, "error", ""),
    ("gap-map-kde.md", 19, "present", ""),
    ("gap-map-kde.md", 20, "vanished", ""),
    ("gap-map-kde.md", 21, "moved", "5-6"),
    ("gap-map-kde.md", 21, "moved", "5"),
    ("gap-map-kde.md", 22, "moved", "11"),
    ("gap-map.md", 1, "unchanged", ""),
    ("spec-widgets.md", 3, "moved", "12-13"),
    ("spec-widgets.md", 4, "unmappable", ""),
    ("spec-widgets.md", 5, "unmappable", ""),
    ("spec-widgets.md", 6, "moved", "3"),
    ("spec-widgets.md", 7, "moved", "7"),
]

SELF_TEST_APPLIED = {
    # (document, line): the line after --apply
    ("gap-map-kde.md", 3): "| `zz` | `kstyle/a.cpp` | KDE/breeze @ `{B8}` (tag `v9.9.9`) |",
    ("gap-map-kde.md", 7): "| K1.1 | `breeze@{B8} kstyle/a.cpp:5-6` |",
    ("gap-map-kde.md", 8): "| K1.2 | `breeze@{A8} kstyle/a.cpp:7` |",
    ("gap-map-kde.md", 10): "| K1.4 | `breeze@{B8} kstyle/a.cpp` |",
    ("gap-map-kde.md", 12): "| K1.6 | [a.cpp:5](https://github.com/KDE/breeze/blob/{B40}/kstyle/a.cpp#L5) |",
    ("gap-map-kde.md", 13): "| K1.7 | `breeze@{B8} kstyle/a.cpp:3`, `:4` |",
    ("gap-map-kde.md", 14): "| K1.8 | `zz:7` and `zz:7` |",
    ("gap-map-kde.md", 16): "| K1.10 | `breeze@{A8} kstyle/a.cpp:9`; `kstyle/b.cpp:2` |",
    ("gap-map-kde.md", 21): "| K1.15 | `breeze@{B8} kstyle/a.cpp:5-6`, `kstyle/a.cpp:5` |",
    ("gap-map-kde.md", 22): "| K1.16 | `zz:11` |",
    ("spec-widgets.md", 3): "| x | K1.1: `a.cpp:12-13` |",
    ("spec-widgets.md", 7): "| v | K1.10: `a.cpp:5` |",
}


def self_test():
    failures = []
    saved = dict(citations.LINE_ABBR)
    with tempfile.TemporaryDirectory() as tmp:
        tmp = Path(tmp)
        files_a = {"kstyle/a.cpp": OLD, "kstyle/gone.cpp": GONE, "kstyle/b.cpp": ["b 1", "b 2", "b 3"],
                   "kstyle/m.scss": TOKENS}
        files_b = {"kstyle/a.cpp": NEW, "kstyle/b.cpp": ["b 1", "b two", "b 3"], "kstyle/m.scss": TOKENS}
        a, b = make_repo(tmp / "repo", files_a, files_b)
        for name, sha in (("from", a), ("cache/breeze@9.9.9", b)):
            subprocess.run(["git", "clone", "-q", str(tmp / "repo"), str(tmp / name)], check=True, capture_output=True)
            subprocess.run(["git", "-C", str(tmp / name), "checkout", "-q", sha], check=True, capture_output=True)
            subprocess.run(["git", "-C", str(tmp / name), "remote", "set-url", "origin",
                            "https://github.com/KDE/breeze.git"], check=True, capture_output=True)
        (tmp / "sources.toml").write_text(
            f'[[source]]\nid = "breeze@9.9.9"\nkind = "git"\nproject = "KDE/breeze"\n'
            f'repository = "https://invent.kde.org/plasma/breeze.git"\ntag = "v9.9.9"\ncommit = "{b}"\n'
            f'generations = ["kde-breeze"]\n')
        fmt = dict(A8=a[:8], A40=a, B8=b[:8], B40=b)
        docs = {d: t.format(**fmt) for d, t in SELF_TEST_DOCS.items()}
        citations.LINE_ABBR["zz"] = ("zz", "line", "KDE/breeze", a[:8], "kstyle/a.cpp", "gap-map-kde.md:3")
        try:
            pins, not_pinned, sites = load_pins(tmp / "sources.toml", tmp / "cache")
            checkouts = load_from_checkouts([tmp / "from"], pins)
            index = citations.Index((), {})
            for p in pins:
                index.add(p.project, p.checkout.files, p.commit)
            for co in checkouts:
                index.add(co.project, co.files)
            run = Run(docs, pins, not_pinned, sites, checkouts, index)
            results = run.run()
            got = [(r.c.doc, r.c.line, r.status, fmt_ranges(r) if r.status in ("moved", "changed") else "")
                   for r in results]
            for w in SELF_TEST_WANT:
                if w not in got:
                    failures.append(f"want {w}")
            for g in got:
                if g not in SELF_TEST_WANT:
                    failures.append(f"unexpected {g}")
            if exit_code(results) != 1:
                failures.append("exit code: want 1 (changed, vanished and error citations)")
            texts, _ = apply(run)
            for (d, ln), want in SELF_TEST_APPLIED.items():
                have = texts[d].split("\n")[ln - 1]
                if have != want.format(**fmt):
                    failures.append(f"--apply {d}:{ln}: got {have!r}, want {want.format(**fmt)!r}")
            # a second run over the rewritten documents finds every rewritten citation at its pin, and changes nothing
            again = Run(texts, pins, not_pinned, sites, checkouts, index)
            again.run()
            texts2, _ = apply(again)
            for d in texts:
                for ln, (x, y) in enumerate(zip(texts[d].split("\n"), texts2[d].split("\n")), 1):
                    if x != y:
                        failures.append(f"second --apply {d}:{ln}: {x!r} became {y!r}")
            done = {(r.c.doc, r.c.line, r.c.raw) for r in results if r.applied}
            for r in again.results:
                if (r.c.doc, r.c.line, r.c.raw) in done and r.status not in ("unchanged", "present"):
                    failures.append(f"second run {r.c.doc}:{r.c.line}: {r.status}, not at its pin")
            for co in checkouts + [p.checkout for p in pins]:
                co.close()
        finally:
            citations.LINE_ABBR.clear()
            citations.LINE_ABBR.update(saved)
    for f in failures:
        print(f"self-test FAIL: {f}", file=sys.stderr)
    if not failures:
        print(f"self-test ok ({len(SELF_TEST_WANT)} citations, {len(SELF_TEST_APPLIED)} rewritten lines)")
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
