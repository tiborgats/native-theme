#!/usr/bin/env python3
"""The verifiers' tooling (the 0.8.0 plan's *Where the work happens* and *How values are checked*, "Facts,
independently"): the units of a delta, the verifier's brief, the comparison of a lane's facts with its verifier's,
and the audit sample. Offline but for `git`; the brief of a measured unit reads the evidence checkout.

Run from the repository root:

    python3 scripts/verify_facts.py units [--since [PLATFORM=]COMMIT ...] [--rev REV] [--out FILE]
    python3 scripts/verify_facts.py brief <unit> <out-dir> --lane-brief FILE [--units FILE] [--no-combinations]
                                          [--cache DIR]
    python3 scripts/verify_facts.py compare <unit> <verifier-facts.toml> [--units FILE] [--out DIR]
    python3 scripts/verify_facts.py sample <unit> --compared DIR --seed N [--rate 0.05] [--min 10]
    python3 scripts/verify_facts.py --self-test

The facts are read as scripts/check_facts.py reads them (items_of, canonical_key, canonical_when; the fact files of
`docs/facts/<platform>/`, `state-models.toml` left out), at a commit, through `git show`.

A fact's identity (S17 *Which fact applies*): its key read as a TOML dotted key (check_facts.canonical_key), its
`when` with empty lists and tables left out and its name lists read as sets (check_facts.canonical_when), its
`scheme_file` and its `accent` (a colour, spelled as NORMALISATIONS' colour rules spell it). Within a unit a fact is also named by
its file. Where one file states an identity twice, the last fact applies and is the one used. A fact's kind:
`pending` (`pending = "U<n>"`, no citation, nothing to verify), `measured` (a `kind = "measured"` citation), `cited`
(only `published` and `read` citations).

`units`: the delta of each platform since its baseline, BASELINE (the M1a tie-break commits, the verified state of
each platform: the ledger's *Verification rates*) unless `--since` names another (`--since kde=<commit>`, or one
commit for every platform), against `--rev` (HEAD): each identity of the revision whose facts differ from the
baseline's in any field but `rows`, or that the baseline lacks, non-pending, is to be verified. Per platform and kind
they are split by key prefix (a prefix holding more than MAX_FACTS facts is split by its next key segment, a whole key
holding more one fact at a time), the pieces, in key order, packed into units of at most MAX_FACTS and near equal
size, and a unit under MIN_FACTS merged into a neighbour where the two hold at most MAX_FACTS. A unit's id is
`<platform code>-<C|M><n>` (CODES; C cited, M measured). It writes the units file (TOML, `--out`, else stdout): the
revision, the baselines, per platform and kind the new, changed and removed facts (pending ones included), and per
unit its id, platform, kind, presets, counts, `scope` (per file and key family: the keys' common prefix and the
count) and `identities` (file, line, key, `when`, `scheme_file`, `accent`, new or changed). It is deterministic: the
same commits give the same file. `brief`, `compare` and `sample` read it (`--units`), or compute it as `units` with
no option does.

`brief <unit> <out-dir>`: the verifier's folder (the plan: the unit's keys, the pins, the catalogue's state-model
source lines and, for `measured` facts, copies of the cited evidence and the method's script), every file of it
read at the units' revision:

- `README.md`: the task, what to read (this folder and the source cache only) and what to write (`state-models.toml`
  and `facts.toml` in the lanes' format);
- `keys.toml`: the unit, its presets and its keys; `combinations.toml`: the combinations to state, each a key with its
  `when`, `scheme_file` and `accent` as the lane wrote them (`--no-combinations` leaves the file out, for the plan's
  first step: the verifier derives the state model before it sees the lane's combinations);
- `pins.toml`: the `docs/facts/sources.toml` entries serving a preset of the unit's platform (all of them, so the
  list does not say which the lane cited; the evidence repository's entry left out), each with its checkout's path in
  the source cache (`--cache`, else citations.cache_dir);
- `catalogue.md`: for each widget the keys name (a catalogue entry, `component.<key>` too), its catalogue items
  CATALOGUE_ITEMS as the catalogue writes them, with their lines;
- `facts-format.md`: the lane brief's FORMAT_SECTIONS (`--lane-brief`, the facts lanes' brief, which lives outside
  the repository);
- for a measured unit, `docs/evidence/…`: each evidence file the unit's facts cite, and each cited run's `run.toml`
  and `analysis/` scripts (the method), copied from the evidence root (citations.evidence_root: a clean checkout at
  the pin). A file it cannot find stops the brief.

No lane value or citation is written. The leak check proves it, on the folder written: each string of the unit's
facts (every leaf of a value, as TOML spells a number or a bool; every citation's `source`, `evidence`, `quote` and
`absent` name) is looked for in every file but the evidence copies (which hold the measured values by design), a
string starting or ending in a letter or digit only where no letter, digit or `_` continues it. Each file is written
from named inputs alone: `keys.toml` and `combinations.toml` from the unit's identities, `pins.toml` from
sources.toml, `catalogue.md` from the catalogue, `facts-format.md` from the lane brief, `README.md` from the unit's
id, platform, kind, presets and file list, each with the file's own headings. A string found is a collision:
`trivial` where it is shorter than FLOOR characters or a keyword or bool (KEYWORDS); else, where the file's inputs
hold it too, unavoidable, classed by the file (LEAK_CLASSES: `identity`, the verifier is told what to state;
`specification`, the texts every lane and verifier reads; `sources`; `template`); else a `leak`, a string that can
only have come from the lane's facts. It prints the counts and each collision but the trivial ones (to stdout, never
into the folder); a leak is exit 1.

`compare <unit> <verifier-facts.toml>`: the lane's facts of the unit against the verifier's file, by identity. The
values are compared after NORMALISATIONS, each listed in the output with the facts whose spelling it changed and
the pairs that agree only through it. Counts: agreed, disagreed, lane-only (the unit's identities the verifier does
not state), verifier-only (the verifier's identities the lane states nowhere on the platform) and outside the unit
(the verifier's identities the lane states outside the unit: unchanged since the baseline, or pending). It writes
into `--out` (the verifier file's folder): `tiebreak.toml`, the summary, the normalisations, a summary line in the
ledger's *Verification rates* table's form, each disagreement with both values and both citations, each one-sided
fact with its value and citations, and each fact outside the unit whose values differ (a lane's pending fact among
them) with both; `agreed.toml`, the agreed facts with both citations, which `sample` reads.

`sample <unit> --compared DIR --seed N`: the audit sample of the agreed facts of `DIR/agreed.toml` (the plan: 5 %
per lane and unit, at least ten), `max(--min, ceil(--rate × agreed))` of them, never more than there are: the agreed
facts ordered by the SHA-256 of `<seed>` and the identity, the first ones taken, so a seed gives the same sample on
any machine and Python. It writes `DIR/sample.toml` with the seed, the rate, the minimum, the population and each
sampled fact with both values and citations.

Exit 0, 1 (`brief`: a leak), or 2 on an input it cannot read, an unknown unit or a wrong command line.

    python3 scripts/verify_facts.py --self-test   # the units, a normalisation, an identity mismatch, one-sided facts,
                                                  # the brief's leak check and the sample's determinism, on fixtures
"""
import argparse
import hashlib
import json
import math
import re
import shutil
import subprocess
import sys
import tempfile
import tomllib
from dataclasses import dataclass
from fractions import Fraction
from pathlib import Path

import check_fact_citations
import check_facts
import citations
import spec_catalogue
from generate_platform_facts import toml_inline, toml_string

FACTS = check_facts.FACTS
PLATFORMS = check_facts.PLATFORMS
SOURCES = check_facts.SOURCES
CATALOGUE = check_facts.CATALOGUE
STATE_MODELS = check_facts.STATE_MODELS

# The verified state of each platform: the M1a tie-break commits (the ledger's *Verification rates*).
BASELINE = {"gnome": "f34baa5b", "kde": "cc4f5dd9", "windows": "e93901ab", "material": "983dcc4f",
            "ios": "d2178ac2", "macos": "5d48e80f"}
CODES = {"gnome": "GN", "kde": "KD", "windows": "WN", "material": "MT", "ios": "IO", "macos": "MC"}
KIND_CODES = {"cited": "C", "measured": "M"}
MIN_FACTS = 300
MAX_FACTS = 1000

CATALOGUE_ITEMS = ("Variants", "Sizes and densities", "Flags", "Exclusive groups", "State-model source")
FORMAT_SECTIONS = ("## The format", "## Canonical forms")
FLOOR = 4
KEYWORDS = {"true", "false", "auto", "absent", "none"}
EVIDENCE_DIR = "docs/evidence"
FORMAT_HEADER = "# The facts format (the facts lanes' brief, its format sections)"

NORMALISATIONS = {
    "key_quoting": "keys compared as TOML dotted keys: a quoted segment TOML can write bare is that segment "
                   "(check_facts.canonical_key)",
    "empty_selectors": "an empty list or table in a `when` is left out, and an empty `when` is none",
    "selector_sets": "a `when`'s name lists compared as sets (check_facts.canonical_when)",
    "colour_case": "a hex colour's digits in lower case",
    "opaque_alpha": "`#rrggbbff` is `#rrggbb`, `srgb(r, g, b, 1)` is `srgb(r, g, b)`",
    "srgb_hex": "`srgb(…)` whose channels are whole 8-bit values is its hex literal (the canonical forms: hex where "
                "the colour is exact in 8-bit sRGB)",
    "number_format": "a number in one spelling (`1.0` is `1`, `0.50` is `0.5`, `4.0px` is `4px`), in a value, a unit "
                     "literal, a colour function and an expression",
    "expression_spacing": "an expression's whitespace collapsed, and none around `(`, `)`, `,` and `=`",
}
IDENTITY_NORMALISATIONS = ("key_quoting", "empty_selectors", "selector_sets")


class InputError(Exception):
    pass


# --------------------------------------------------------------------------------------------- the commits

class Tree:
    """The repository's files at a commit, read through git."""

    def __init__(self, root, rev):
        self.root = Path(root)
        self.rev = resolve(root, rev)

    def git(self, *args):
        try:
            got = subprocess.run(["git", "-C", str(self.root), *args], capture_output=True, text=True)
        except OSError as err:
            raise InputError(f"git: {err}") from err
        return got

    def files(self, prefix):
        got = self.git("ls-tree", "-r", "--name-only", self.rev, "--", prefix)
        if got.returncode != 0:
            raise InputError(f"git ls-tree {self.rev} {prefix}: {got.stderr.strip()}")
        return sorted(got.stdout.split("\n")[:-1])

    def text(self, path):
        got = self.git("show", f"{self.rev}:{path}")
        if got.returncode != 0:
            raise InputError(f"{path} at {self.rev}: {got.stderr.strip()}")
        return got.stdout


def resolve(root, rev):
    try:
        got = subprocess.run(["git", "-C", str(root), "rev-parse", "--verify", "-q", f"{rev}^{{commit}}"],
                             capture_output=True, text=True)
    except OSError as err:
        raise InputError(f"git: {err}") from err
    if got.returncode != 0:
        raise InputError(f"no commit {rev!r} in {root}")
    return got.stdout.strip()


# ----------------------------------------------------------------------------------------------- the facts

@dataclass
class Fact:
    file: str      # the fact file's name in its platform's directory
    line: int
    data: dict
    cites: list    # the citations' tables
    identity: tuple

    @property
    def kind(self):
        return fact_kind(self.data)


def fact_kind(data):
    if "pending" in data:
        return "pending"
    cites = data.get("cite") if isinstance(data.get("cite"), list) else []
    return "measured" if any(isinstance(c, dict) and c.get("kind") == "measured" for c in cites) else "cited"


def read_facts(file, platform, text):
    """The facts of one fact file's text, as check_facts.items_of reads them."""
    try:
        data = tomllib.loads(text)
    except tomllib.TOMLDecodeError as err:
        raise InputError(f"{file}: not valid TOML: {err}") from err
    out = []
    for it in check_facts.items_of(file, platform, text, data, "fact"):
        out.append(Fact(Path(file).name, it.line, it.data, [c.data for c in it.cites], identity_of(it.data)))
    return out


def platform_facts(tree, platform):
    """Every fact of the platform's directory at the tree's commit, in file and source order."""
    out = []
    for path in tree.files(f"{FACTS.as_posix()}/{platform}/"):
        if path.endswith(".toml") and Path(path).name != STATE_MODELS and Path(path).parent.name == platform:
            out += read_facts(path, platform, tree.text(path))
    return out


def plain(data):
    """A fact's identity fields as it writes them."""
    return {k: data[k] for k in ("key", "when", "scheme_file", "accent") if k in data}


def last_by(facts, key):
    """{key(fact): the last fact with it} (S17 *Which fact applies*: the last matching fact applies)."""
    out = {}
    for f in facts:
        out[key(f)] = f
    return out


# ------------------------------------------------------------------------------------- the normalisations

def num_text(x):
    x = float(x)
    if x.is_integer() and abs(x) < 1e15:
        return str(int(x))
    return repr(x)


def colour_text(raw, on, used):
    """A colour literal spelled as the normalisations `on` spell it, the ones that changed it added to `used`."""
    lits = check_fact_citations.string_literals(raw)
    if len(lits) != 1 or lits[0].kind != "colour":
        return raw
    if raw.startswith("#"):
        out = raw
        if "colour_case" in on and out != out.lower():
            out = out.lower()
            used.add("colour_case")
        if "opaque_alpha" in on and len(out) == 9 and out[7:].lower() == "ff":
            out = out[:7]
            used.add("opaque_alpha")
        return out
    m = check_facts.FUNC_COLOUR.match(raw)
    if not m:
        return raw
    name, args = m.group(1), [a for a in m.groups()[1:] if a is not None]
    if "opaque_alpha" in on and len(args) == 4 and float(args[3]) == 1:
        args = args[:3]
        used.add("opaque_alpha")
    channels = lits[0].value
    if (name == "srgb" and "srgb_hex" in on
            and all(abs(c - round(c)) < 1e-6 for c in channels)):
        used.add("srgb_hex")
        r, g, b, a = (round(c) for c in channels)
        return f"#{r:02x}{g:02x}{b:02x}" + (f"{a:02x}" if len(args) == 4 else "")
    if "number_format" in on:
        out = f"{name}({', '.join(num_text(a) for a in args)})"
        if out != raw:
            used.add("number_format")
        return out
    return raw


def expression_text(s, on, used):
    def token(m):
        t = m.group(0)
        if m.lastgroup in ("colour", "hex"):
            return colour_text(t, on, used)
        if m.lastgroup == "num" and "number_format" in on:
            u = re.search(r"(px|ms|deg|pt)$", t)
            out = num_text(t[:u.start()] if u else t) + (u.group(1) if u else "")
            if out != t:
                used.add("number_format")
            return out
        return t
    out = check_fact_citations.VALUE_TOKEN_RX.sub(token, s)
    if "expression_spacing" in on:
        spaced = re.sub(r"\s*([(),=])\s*", r"\1", " ".join(out.split()))
        if spaced != out:
            used.add("expression_spacing")
        out = spaced
    return out


def string_text(s, on, used):
    if check_facts.HEX_LITERAL.match(s) or check_facts.FUNC_COLOUR.match(s):
        return colour_text(s, on, used)
    m = check_facts.UNIT_LITERAL.match(s)
    if m:
        if "number_format" not in on:
            return s
        out = num_text(m.group(1)) + m.group(2)
        if out != s:
            used.add("number_format")
        return out
    if s.lstrip().startswith("="):
        return expression_text(s, on, used)
    return s


def norm_value(v, on, used):
    """A value as a comparable tree: a bool, a number, a string, a list and a table each tagged (so `true` is no `1`),
    spelled as the normalisations `on` spell it."""
    if isinstance(v, bool):
        return ("bool", v)
    if isinstance(v, (int, float)):
        if "number_format" in on:
            if isinstance(v, float) and num_text(v) != repr(v):
                used.add("number_format")
            return ("number", num_text(v))
        return ("number", repr(v))
    if isinstance(v, str):
        return ("string", string_text(v, on, used))
    if isinstance(v, list):
        return ("list",) + tuple(norm_value(x, on, used) for x in v)
    if isinstance(v, dict):
        return ("table",) + tuple(sorted((k, norm_value(x, on, used)) for k, x in v.items()))
    return ("other", repr(v))


def fact_value(data, on, used):
    if "pending" in data:
        return ("pending", data.get("pending"))
    return norm_value(data.get("value"), on, used)


def strip_empty(v):
    if isinstance(v, dict):
        out = {k: strip_empty(x) for k, x in v.items()}
        return {k: x for k, x in out.items() if x not in ([], {}, None)}
    if isinstance(v, list):
        return [strip_empty(x) for x in v]
    return v


def ordered(v):
    """A `when` as a comparable value with its lists in their written order."""
    if isinstance(v, dict):
        return tuple(sorted((k, ordered(x)) for k, x in v.items()))
    if isinstance(v, list):
        return ("list",) + tuple(ordered(x) for x in v)
    return v


def norm_when(when, on, used):
    w = when
    if "empty_selectors" in on:
        s = strip_empty(w) if isinstance(w, dict) else w
        s = s or None
        if s != w:
            used.add("empty_selectors")
        w = s
    if "selector_sets" in on:
        c = check_facts.canonical_when(w)
        if ordered(w) != ordered_sets(w):
            used.add("selector_sets")
        return c
    return ordered(w)


def ordered_sets(v):
    """ordered(v) with each name list sorted and without repeats: what selector_sets changes, to tell whether it
    did."""
    if isinstance(v, dict):
        return tuple(sorted((k, ordered_sets(x)) for k, x in v.items()))
    if isinstance(v, list):
        if check_facts.str_list(v):
            return ("list",) + tuple(sorted(set(v)))
        return ("list",) + tuple(ordered_sets(x) for x in v)
    return v


def norm_key(key, on, used):
    if "key_quoting" not in on or not isinstance(key, str):
        return key
    segs = check_facts.canonical_key(key)
    if isinstance(segs, tuple) and ".".join(check_fact_citations.toml_key(s) for s in segs) != key:
        used.add("key_quoting")
    return segs


def identity_of(data, on=tuple(NORMALISATIONS), used=None):
    used = set() if used is None else used
    accent = data.get("accent")
    scheme = data.get("scheme_file")
    return (norm_key(data.get("key"), on, used), norm_when(data.get("when"), on, used),
            scheme.strip() if isinstance(scheme, str) else scheme,
            colour_text(accent, on, used) if isinstance(accent, str) else accent)


# ----------------------------------------------------------------------------------------------- the units

@dataclass
class Entry:
    fact: Fact
    status: str   # "new" or "changed"
    segs: tuple


def without_rows(data):
    return {k: v for k, v in data.items() if k != "rows"}


def delta(base, head):
    """(the head's identities to verify as Entries, {(kind, status): count}), each identity named with its file; an
    identity is new where the base lacks it, changed where its facts differ in any field but `rows`."""
    def groups(facts):
        out = {}
        for f in facts:
            out.setdefault((f.file, f.identity), []).append(f)
        return out
    b, h = groups(base), groups(head)
    counts, entries = {}, []
    for ident, facts in h.items():
        last = facts[-1]
        if ident not in b:
            status = "new"
        elif [without_rows(f.data) for f in b[ident]] != [without_rows(f.data) for f in facts]:
            status = "changed"
        else:
            continue
        counts[(last.kind, status)] = counts.get((last.kind, status), 0) + 1
        if last.kind != "pending":
            segs = last.identity[0] if isinstance(last.identity[0], tuple) else (str(last.identity[0]),)
            entries.append(Entry(last, status, segs))
    for ident, facts in b.items():
        if ident not in h:
            counts[(facts[-1].kind, "removed")] = counts.get((facts[-1].kind, "removed"), 0) + 1
    return entries, counts


def split(entries, depth, hi):
    """The entries (sharing their first `depth` key segments) in pieces of at most `hi`: by the next key segment, a
    whole key's facts one by one (pack then fills units with them in order)."""
    if len(entries) <= hi:
        return [entries]
    if all(len(e.segs) <= depth for e in entries):
        return [[e] for e in entries]
    groups = {}
    for e in entries:
        groups.setdefault(e.segs[:depth + 1], []).append(e)
    return [p for g in groups.values() for p in split(g, depth + 1, hi)]


def pack(pieces, lo, hi):
    """The pieces, in order, packed into units of at most `hi` and near equal size; a unit under `lo` merged into the
    smaller neighbour with which it holds at most `hi`."""
    total = sum(len(p) for p in pieces)
    if not total:
        return []
    target = total / math.ceil(total / hi)
    units, cur = [], []
    for p in pieces:
        if cur and len(cur) + len(p) > hi:
            units.append(cur)
            cur = []
        cur.extend(p)
        if len(cur) >= target:
            units.append(cur)
            cur = []
    if cur:
        units.append(cur)
    i = 0
    while i < len(units):
        if len(units[i]) < lo:
            near = [j for j in (i - 1, i + 1) if 0 <= j < len(units) and len(units[i]) + len(units[j]) <= hi]
            if near:
                j = min(near, key=lambda k: (len(units[k]), k))
                a, b = sorted((i, j))
                units[a:b + 1] = [units[a] + units[b]]
                i = 0
                continue
        i += 1
    return units


def family(segs):
    return segs[:2] if segs[:1] == ("component",) else segs[:1]


def dotted(segs):
    return ".".join(check_fact_citations.toml_key(s) for s in segs)


def common_prefix(seqs):
    first = seqs[0]
    n = len(first)
    for s in seqs[1:]:
        n = min(n, len(s))
        for i in range(n):
            if s[i] != first[i]:
                n = i
                break
    return first[:n]


def make_units(platform, entries, lo=MIN_FACTS, hi=MAX_FACTS):
    """The platform's units: per kind, its entries in key order split and packed."""
    out = []
    order = {f: i for i, f in enumerate(sorted({e.fact.file for e in entries}))}
    for kind in KIND_CODES:
        mine = sorted((e for e in entries if e.fact.kind == kind),
                      key=lambda e: (family(e.segs), order[e.fact.file], e.segs, e.fact.line))
        for n, unit in enumerate(pack(split(mine, 0, hi), lo, hi), 1):
            out.append(Unit(f"{CODES.get(platform, platform.upper())}-{KIND_CODES[kind]}{n}", platform, kind, unit))
    return out


@dataclass
class Unit:
    id: str
    platform: str
    kind: str
    entries: list

    def scope(self):
        groups = {}
        for e in self.entries:
            groups.setdefault((e.fact.file, family(e.segs)), []).append(e)
        return [{"file": f, "prefix": dotted(common_prefix([e.segs for e in es])), "facts": len(es)}
                for (f, _), es in groups.items()]


def compute_units(root, rev="HEAD", since=None, lo=MIN_FACTS, hi=MAX_FACTS):
    """(the revision's commit, {platform: baseline commit}, {platform: counts}, [Unit])."""
    head = Tree(root, rev)
    bases, counts, units = {}, {}, []
    for platform in PLATFORMS:
        base = Tree(root, (since or {}).get(platform, BASELINE[platform]))
        bases[platform] = base.rev
        entries, counts[platform] = delta(platform_facts(base, platform), platform_facts(head, platform))
        units += make_units(platform, entries, lo, hi)
    return head.rev, bases, counts, units


def units_text(rev, bases, counts, units, lo=MIN_FACTS, hi=MAX_FACTS):
    out = ["# The verifiers' units (the 0.8.0 plan's \"Facts, independently\"), written by scripts/verify_facts.py "
           "units:", "# per platform, the facts to verify since its baseline, split by key prefix and packed by "
           "kind. Generated: do not edit.", "",
           f"rev = {toml_string(rev)}", f"min_facts = {lo}", f"max_facts = {hi}", "", "[since]"]
    out += [f"{p} = {toml_string(c)}" for p, c in bases.items()]
    for p, c in counts.items():
        for kind in ("pending", "measured", "cited"):
            n = {s: c.get((kind, s), 0) for s in ("new", "changed", "removed")}
            out += ["", "[[count]]", f"platform = {toml_string(p)}", f"kind = {toml_string(kind)}"]
            out += [f"{s} = {v}" for s, v in n.items()]
    for u in units:
        out += ["", "[[unit]]", f"id = {toml_string(u.id)}", f"platform = {toml_string(u.platform)}",
                f"kind = {toml_string(u.kind)}", f"presets = {toml_inline(list(PLATFORMS.get(u.platform, ())))}",
                f"facts = {len(u.entries)}",
                f"new = {sum(1 for e in u.entries if e.status == 'new')}",
                f"changed = {sum(1 for e in u.entries if e.status == 'changed')}",
                "scope = [" + ", ".join(toml_inline(s) for s in u.scope()) + "]", "identities = ["]
        for e in u.entries:
            d = e.fact.data
            ident = {"file": e.fact.file, "line": e.fact.line, "key": d.get("key")}
            for k in ("when", "scheme_file", "accent"):
                if k in d:
                    ident[k] = d[k]
            ident["status"] = e.status
            out.append(f"  {toml_inline(ident)},")
        out.append("]")
    return "\n".join(out) + "\n"


def read_units(root, path):
    """The units file's table, or one computed as `units` with no option writes it."""
    if path:
        try:
            return tomllib.loads(Path(path).read_text(encoding="utf-8"))
        except (OSError, UnicodeDecodeError, tomllib.TOMLDecodeError) as err:
            raise InputError(f"{path}: {err}") from err
    print("# no --units: computing the units at HEAD since the built-in baselines", file=sys.stderr)
    return tomllib.loads(units_text(*compute_units(root)))


def find_unit(doc, unit_id):
    for u in doc.get("unit", []):
        if u.get("id") == unit_id:
            return u
    ids = ", ".join(str(u.get("id")) for u in doc.get("unit", []))
    raise InputError(f"no unit {unit_id!r} in the units (one of {ids})")


def unit_facts(root, doc, unit):
    """(the unit's facts at the units' revision, every fact of its platform there)."""
    tree = Tree(root, doc["rev"])
    facts = platform_facts(tree, unit["platform"])
    by = last_by(facts, lambda f: (f.file, f.identity))
    out = []
    for ident in unit["identities"]:
        k = (ident["file"], identity_of(ident))
        if k not in by:
            raise InputError(f"{unit['id']}: {ident['file']}: no fact {ident.get('key')!r} with that `when` at "
                             f"{doc['rev']}")
        out.append(by[k])
    return out, facts


# ----------------------------------------------------------------------------------------------- the brief

def widget_of(segs, entries):
    """The catalogue entry a key's segments name, or None (a colour role, a token, an input)."""
    if segs[:1] == ("component",) and len(segs) > 1:
        name = f"component.{segs[1]}"
        return name if name in entries else None
    if not segs or segs[0] not in entries:
        return None
    if segs[0] == "input":
        parts = {n for p in entries["input"].parts for n in p.names}
        return "input" if len(segs) > 1 and (segs[1] == "values" or segs[1] in parts) else None
    return segs[0]


def catalogue_text(cat_text, widgets):
    """(the text, the lines it adds to the catalogue's): each widget's CATALOGUE_ITEMS lines, as the catalogue
    writes them."""
    entries = {e.name: e for e in spec_catalogue.entries(cat_text)}
    lines = cat_text.splitlines()
    out = ["# The catalogue's lines for the unit's widgets", "",
           f"From `{CATALOGUE.as_posix()}`: each widget's {', '.join(CATALOGUE_ITEMS)} items, as the catalogue writes "
           "them. The state-model source names where the combinations come from; the other items name what the "
           "model can express (S10, S17)."]
    if not widgets:
        out += ["", "The unit's keys name no widget: they are colour roles, tokens or inputs, whose facts take "
                    "conditions only (S7)."]
    added = list(out)
    for w in sorted(widgets):
        e = entries[w]
        nums = set()
        for it in e.items:
            if it.label in CATALOGUE_ITEMS:
                span = list(range(it.line, it.line + len(it.lines)))
                while span and not lines[span[-1] - 1].strip():
                    span.pop()
                nums |= set(span)
        runs = []
        for n in sorted(nums):
            if runs and n == runs[-1][-1] + 1:
                runs[-1].append(n)
            else:
                runs.append([n])
        head = f"## {w} (catalogue line {e.line})"
        out += ["", head]
        added.append(head)
        for r in runs:
            out += ["", f"Lines {r[0]}-{r[-1]}:", ""] + [lines[n - 1] for n in r]
            added.append(f"Lines {r[0]}-{r[-1]}:")
    return "\n".join(out) + "\n", "\n".join(added)


def format_text(lane_brief):
    lines = lane_brief.splitlines()
    out, keep = [FORMAT_HEADER, ""], False
    for line in lines:
        if line.startswith("## "):
            keep = line.startswith(FORMAT_SECTIONS)
        if keep:
            out.append(line)
    if len(out) == 2:
        raise InputError(f"the lane brief has none of the sections {', '.join(FORMAT_SECTIONS)}")
    return "\n".join(out).rstrip() + "\n"


def pins_text(sources_data, platform, cache):
    """(the text, the lines it adds to the entries': its comments and each entry's cache path)."""
    presets = set(PLATFORMS.get(platform, ()))
    out = ["# The pinned sources serving the unit's presets (docs/facts/sources.toml), each with its checkout in the "
           "source cache.", "# Cite a source as `source = \"<id> <path>:<lines>\"`, the lines at the pin.", ""]
    added = list(out)
    for e in sources_data.get("source", []):
        if not isinstance(e, dict) or e.get("evidence") is True:
            continue
        gens = e.get("generations") if isinstance(e.get("generations"), list) else []
        if not presets & set(gens):
            continue
        out.append("[[source]]")
        out += [f"{k} = {toml_inline(v)}" for k, v in e.items()]
        if e.get("kind") in ("git", "snapshot", "sdk"):
            out.append(f"cache = {toml_string(str(Path(cache) / e['id']))}")
            added.append(out[-1])
        out.append("")
    return "\n".join(out), "\n".join(added)


def readme_text(unit, files, cache):
    measured = unit["kind"] == "measured"
    out = [f"# Verifier's brief: unit {unit['id']} ({unit['platform']}, {unit['kind']})", "",
           f"You verify the {unit['facts']} facts of this unit for the presets {', '.join(unit['presets'])}, "
           "independently of the facts lane that wrote them: you never see the lane's values or citations.", "",
           "## What you read", "",
           "This folder, and the pinned sources in the source cache "
           f"(`{cache}`, one checkout per `pins.toml` entry, at the pin). Nothing else: not the project, its gap "
           "maps or `platform-facts.md`, not the evidence repository's checkout.", ""]
    out += [f"- `{f}`: {what}" for f, what in files]
    out += ["", "## What you write", "",
            "1. `state-models.toml`: for each widget of the keys, the `[[model]]` entry derived from the sources (the "
            "catalogue's state-model source names where to look): the states, conditions, contexts, variants, sizes "
            "and densities the platform's drawing distinguishes, the exclusive groups, the combinations it does not "
            "draw, each with its citations.",
            "2. `facts.toml`: a `[[fact]]` for each key and combination, in the format of `facts-format.md`: `key`, "
            "`when`, `scheme_file` and `accent` as `combinations.toml` writes them, `value`, `rows = []` and the "
            "citations. A combination the platform draws that the list lacks gets a fact too; one you cannot source "
            "gets none, and goes in your report with what you tried.",
            "   " + ("Each value with a `measured` citation: `evidence = \"docs/evidence/<harness>/<run>/<image>/"
                     "<file>#<path>\"` (the copies under `docs/evidence/`, the run's `analysis/` scripts its method), "
                     "and its `tolerance`; check the reading, the method and the conversion."
                     if measured else
                     "Each value with a `published` (or `read`) citation: `source = \"<id> <path>:<lines>\"` from "
                     "`pins.toml` and the `quote` found there."),
            "3. Your report: the counts, the combinations you could not source, and anything the catalogue cannot "
            "express."]
    return "\n".join(out) + "\n"


def brief(root, doc, unit_id, out, lane_brief, cache, evidence_root=None, combinations=True):
    """Write the unit's brief into `out` (a new or empty folder); return (the leak check's collisions, {file: the
    text it was written from})."""
    unit = find_unit(doc, unit_id)
    tree = Tree(root, doc["rev"])
    facts, _ = unit_facts(root, doc, unit)
    out = Path(out)
    if out.exists() and any(out.iterdir()):
        raise InputError(f"{out} is not empty")
    out.mkdir(parents=True, exist_ok=True)
    cat = tree.text(CATALOGUE.as_posix())
    sources_text = tree.text(SOURCES.as_posix())
    try:
        sources_data = tomllib.loads(sources_text)
    except tomllib.TOMLDecodeError as err:
        raise InputError(f"{SOURCES}: {err}") from err
    entries = {e.name: e for e in spec_catalogue.entries(cat)}
    keys = sorted({f.data["key"] for f in facts}, key=lambda k: (str(check_facts.canonical_key(k)), k))
    widgets = {w for f in facts
               if (w := widget_of(f.identity[0] if isinstance(f.identity[0], tuple) else (), entries))}
    header = ["[unit]", f"id = {toml_string(unit['id'])}", f"platform = {toml_string(unit['platform'])}",
              f"kind = {toml_string(unit['kind'])}", f"presets = {toml_inline(unit['presets'])}",
              f"facts = {unit['facts']}"]
    keys_head = ["# The unit's keys (S17 *Keys and values*): state each under every combination the platform draws.",
                 ""] + header
    combos_head = ["# The combinations to state: each key with its `when`, `scheme_file` and `accent`, as the lane's "
                   "facts name them.", ""] + header
    ident = identity_text(facts)
    write = {"keys.toml": ("\n".join(keys_head + ["", "keys = ["] + [f"  {toml_string(k)}," for k in keys] + ["]"])
                           + "\n", "\n".join(keys_head) + "\n" + ident)}
    files = [("keys.toml", "the unit and its keys")]
    if combinations:
        rows = list(combos_head)
        for f in facts:
            rows += ["", "[[combination]]"] + [f"{k} = {toml_inline(v)}" for k, v in plain(f.data).items()]
        write["combinations.toml"] = ("\n".join(rows) + "\n", "\n".join(combos_head) + "\n" + ident)
        files.append(("combinations.toml", "the combinations to state, one fact each"))
    pins, pins_added = pins_text(sources_data, unit["platform"], cache)
    write["pins.toml"] = (pins, sources_text + "\n" + pins_added)
    files.append(("pins.toml", "the pinned sources and their checkouts"))
    catalogue, cat_added = catalogue_text(cat, widgets)
    write["catalogue.md"] = (catalogue, cat + "\n" + cat_added)
    files.append(("catalogue.md", "the catalogue's lines for the unit's widgets"))
    write["facts-format.md"] = (format_text(lane_brief), lane_brief + "\n" + FORMAT_HEADER)
    files.append(("facts-format.md", "the facts format"))
    if unit["kind"] == "measured":
        if evidence_root is None:
            try:
                evidence_root = citations.evidence_root(sources_data, cache)
            except citations.EvidenceRootError as err:
                raise InputError(str(err)) from err
        n = copy_evidence(facts, Path(evidence_root), out)
        files.append((f"{EVIDENCE_DIR}/", f"the {n} evidence files the unit's facts cite, each run's run.toml and "
                                          f"analysis/ scripts"))
    readme = readme_text(unit, files, cache)
    write["README.md"] = (readme, readme)
    for name, (text, _) in write.items():
        (out / name).write_text(text, encoding="utf-8")
    inputs = {name: provenance for name, (_, provenance) in write.items()}
    return leak_check(out, facts, inputs), inputs


def copy_evidence(facts, root, out):
    """Copy each cited evidence file, and each cited run's run.toml and analysis/ files, from the evidence root."""
    wanted = set()
    for f in facts:
        for c in f.cites:
            ev = check_facts.parse_evidence(c.get("evidence")) if c.get("kind") == "measured" else None
            if not ev:
                continue
            file, _, harness, run, _ = ev
            wanted.add(file)
            run_dir = f"{EVIDENCE_DIR}/{harness}/{run}"
            wanted.add(f"{run_dir}/run.toml")
            analysis = root / run_dir / check_fact_citations.ANALYSIS
            if analysis.is_dir():
                wanted |= {p.relative_to(root).as_posix() for p in analysis.rglob("*") if p.is_file()}
    for rel in sorted(wanted):
        src = root / rel
        if not src.is_file():
            raise InputError(f"no evidence file {rel} under {root}")
        dst = out / rel
        dst.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(src, dst)
    return len(wanted)


# ------------------------------------------------------------------------------------------ the leak check

def value_strings(v):
    if isinstance(v, bool):
        return ["true" if v else "false"]
    if isinstance(v, (int, float)):
        return [toml_inline(v)]
    if isinstance(v, str):
        return [v]
    if isinstance(v, list):
        return [s for x in v for s in value_strings(x)]
    if isinstance(v, dict):
        return [s for x in v.values() for s in value_strings(x)]
    return []


def needles(facts):
    """{string: what it is} of the facts' values and citations."""
    out = {}
    for f in facts:
        for s in value_strings(f.data.get("value")):
            out.setdefault(s, f"a value of {f.data.get('key')}")
        for c in f.cites:
            for k in ("source", "evidence", "quote"):
                if isinstance(c.get(k), str):
                    out.setdefault(" ".join(c[k].split()) if k == "quote" else c[k], f"a citation's {k}")
            for name in c.get("absent", []) if isinstance(c.get("absent"), list) else []:
                if isinstance(name, str):
                    out.setdefault(name, "a citation's absent name")
    out.pop("", None)
    return out


def needle_rx(s):
    left = r"(?<![A-Za-z0-9_])" if s[:1].isalnum() else ""
    right = r"(?![A-Za-z0-9_])" if s[-1:].isalnum() else ""
    return re.compile(left + re.escape(s) + right)


def identity_text(facts):
    """The text of the facts' identities: their keys, `when`s, scheme files and accents, as TOML writes them."""
    return "\n".join(f"{k} = {toml_inline(v)}" for f in facts for k, v in plain(f.data).items())


LEAK_CLASSES = {"keys.toml": "identity", "combinations.toml": "identity", "catalogue.md": "specification",
                "facts-format.md": "specification", "pins.toml": "sources", "README.md": "template"}


def leak_check(out, facts, inputs):
    """[(class, string, what, file, line)] of every string of the facts' values and citations found in the folder's
    files (the evidence copies left out): `trivial` (under FLOOR characters, or a keyword or bool), else the class of
    the file (LEAK_CLASSES) where the text the file was written from holds it too (`inputs`: {file: that text}), else
    `leak`, a string that can only have come from the lane's facts."""
    found = []
    texts = {}
    for p in sorted(Path(out).rglob("*")):
        rel = p.relative_to(out).as_posix()
        if p.is_file() and not rel.startswith(EVIDENCE_DIR + "/"):
            texts[rel] = p.read_text(encoding="utf-8")
    for s, what in sorted(needles(facts).items()):
        rx = needle_rx(s)
        for rel, text in texts.items():
            for m in rx.finditer(text):
                line = text.count("\n", 0, m.start()) + 1
                if len(s) < FLOOR or s in KEYWORDS:
                    cls = "trivial"
                elif rel in inputs and rx.search(inputs[rel]):
                    cls = LEAK_CLASSES.get(rel, "leak")
                else:
                    cls = "leak"
                found.append((cls, s, what, rel, line))
    return found


def leak_report(found):
    names = ("leak", "identity", "specification", "sources", "template", "trivial")
    counts = {c: sum(1 for f in found if f[0] == c) for c in names}
    lines = [f"# leak check: {counts['leak']} leaks; unavoidable collisions: {counts['identity']} in the identities, "
             f"{counts['specification']} in the specification's text (the catalogue, the lane brief's format), "
             f"{counts['sources']} in sources.toml's entries, {counts['template']} in the README's own text, "
             f"{counts['trivial']} trivial (under {FLOOR} characters, or a keyword or bool)"]
    groups = {}
    for cls, s, what, rel, line in found:
        if cls != "trivial":
            groups.setdefault((cls, rel, s, what), []).append(line)
    for (cls, rel, s, what), at in sorted(groups.items(), key=lambda g: (names.index(g[0][0]), g[0][1:3])):
        more = f" (+{len(at) - 5} more)" if len(at) > 5 else ""
        lines.append(f"{cls}\t{rel}:{','.join(map(str, at[:5]))}{more}\t{what}\t{s!r}")
    return counts, "\n".join(lines)


# --------------------------------------------------------------------------------------------- the compare

def side(f):
    """A fact's file, line, value (or pending id) and citations, for the tie-breaker."""
    d = {"file": f.file, "line": f.line}
    if "pending" in f.data:
        d["pending"] = f.data["pending"]
    else:
        d["value"] = f.data.get("value")
    d["cite"] = f.cites
    return d


def compare(root, doc, unit_id, verifier_path, out=None):
    """Write tiebreak.toml and agreed.toml into `out`; return the summary table."""
    unit = find_unit(doc, unit_id)
    lane_unit, lane_all = unit_facts(root, doc, unit)
    vpath = Path(verifier_path)
    try:
        vtext = vpath.read_text(encoding="utf-8")
    except (OSError, UnicodeDecodeError) as err:
        raise InputError(f"{vpath}: {err}") from err
    vfacts = read_facts(vpath.as_posix(), unit["platform"], vtext)
    lane = {f.identity: f for f in lane_unit}
    lane_any = last_by(lane_all, lambda f: f.identity)
    ver = last_by(vfacts, lambda f: f.identity)
    all_on = tuple(NORMALISATIONS)
    stats = {n: {"changed": 0, "decisive": 0} for n in NORMALISATIONS}
    agreed, disagreed, lane_only, verifier_only, outside = [], [], [], [], []
    before = outside_agreed = 0
    for ident, lf in lane.items():
        vf = ver.get(ident)
        if vf is None:
            lane_only.append(lf)
            continue
        used = set()
        identity_of(lf.data, all_on, used)
        identity_of(vf.data, all_on, used)
        a, b = fact_value(lf.data, all_on, used), fact_value(vf.data, all_on, used)
        for n in used:
            stats[n]["changed"] += 1
        if fact_value(lf.data, (), set()) == fact_value(vf.data, (), set()):
            before += 1
        if a == b:
            agreed.append((lf, vf))
            for n in NORMALISATIONS:
                rest = tuple(x for x in all_on if x != n)
                if n in IDENTITY_NORMALISATIONS:
                    if identity_of(lf.data, rest) != identity_of(vf.data, rest):
                        stats[n]["decisive"] += 1
                elif fact_value(lf.data, rest, set()) != fact_value(vf.data, rest, set()):
                    stats[n]["decisive"] += 1
        else:
            disagreed.append((lf, vf))
    for ident, vf in ver.items():
        if ident in lane:
            continue
        if ident in lane_any:
            lf = lane_any[ident]
            if fact_value(lf.data, all_on, set()) == fact_value(vf.data, all_on, set()):
                outside_agreed += 1
            else:
                outside.append((lf, vf))
        else:
            verifier_only.append(vf)
    compared = len(agreed) + len(disagreed)
    summary = {
        "unit": unit["id"], "platform": unit["platform"], "kind": unit["kind"], "rev": doc["rev"],
        "verifier_file": vpath.as_posix(), "lane_facts": len(lane), "verifier_facts": len(ver),
        "verifier_repeated": len(vfacts) - len(ver), "compared": compared, "agreed": len(agreed),
        "agreed_before_normalising": before, "disagreed": len(disagreed), "lane_only": len(lane_only),
        "verifier_only": len(verifier_only), "outside_unit": len(outside) + outside_agreed,
        "outside_unit_disagreed": len(outside),
    }
    summary["ledger"] = ledger_line(summary)
    out = Path(out) if out else vpath.parent
    out.mkdir(parents=True, exist_ok=True)
    t = ["# The comparison of a lane's facts with its verifier's (scripts/verify_facts.py compare): the tie-breaker's "
         "input.", "", "[summary]"] + [f"{k} = {toml_inline(v)}" for k, v in summary.items()]
    for n, what in NORMALISATIONS.items():
        t += ["", "[[normalisation]]", f"name = {toml_string(n)}", f"what = {toml_string(what)}",
              f"changed = {stats[n]['changed']}", f"decisive = {stats[n]['decisive']}"]
    for table, pairs in (("disagreed", disagreed), ("outside_unit", outside)):
        for lf, vf in pairs:
            t += ["", f"[[{table}]]"] + [f"{k} = {toml_inline(v)}" for k, v in plain(lf.data).items()]
            t += [f"lane = {toml_inline(side(lf))}", f"verifier = {toml_inline(side(vf))}"]
    for table, facts, who in (("lane_only", lane_only, "lane"), ("verifier_only", verifier_only, "verifier")):
        for f in facts:
            t += ["", f"[[{table}]]"] + [f"{k} = {toml_inline(v)}" for k, v in plain(f.data).items()]
            t.append(f"{who} = {toml_inline(side(f))}")
    (out / "tiebreak.toml").write_text("\n".join(t) + "\n", encoding="utf-8")
    g = ["# The agreed facts (scripts/verify_facts.py compare), the audit sample's population.", "", "[summary]",
         f"unit = {toml_string(unit['id'])}", f"rev = {toml_string(doc['rev'])}",
         f"verifier_file = {toml_string(vpath.as_posix())}", f"agreed = {len(agreed)}"]
    for lf, vf in agreed:
        g += ["", "[[agreed]]"] + [f"{k} = {toml_inline(v)}" for k, v in plain(lf.data).items()]
        g += [f"lane = {toml_inline(side(lf))}", f"verifier = {toml_inline(side(vf))}"]
    (out / "agreed.toml").write_text("\n".join(g) + "\n", encoding="utf-8")
    return summary


def sample_size(n, rate, least):
    return min(n, max(least, math.ceil(Fraction(str(rate)) * n)))


def ledger_line(s):
    c = s["compared"]
    pct = f"{100 * s['agreed'] / c:.1f} %; {100 * s['agreed_before_normalising'] / c:.1f} % before" if c else "none"
    return (f"| {s['unit']} | {s['platform']} | lane {s['lane_facts']:,}, verifier {s['verifier_facts']:,} | "
            f"{s['agreed']:,} of {c:,} compared combinations after normalising spellings ({pct}) | to tie-break: "
            f"{s['disagreed']:,} disagreed, {s['lane_only']:,} lane-only, {s['verifier_only']:,} verifier-only | "
            f"{sample_size(s['agreed'], 0.05, 10)} | |")


# ---------------------------------------------------------------------------------------------- the sample

def sample(unit_id, compared, seed, rate=0.05, least=10):
    """Write DIR/sample.toml; return the sampled entries."""
    path = Path(compared) / "agreed.toml"
    try:
        doc = tomllib.loads(path.read_text(encoding="utf-8"))
    except (OSError, UnicodeDecodeError, tomllib.TOMLDecodeError) as err:
        raise InputError(f"{path}: {err}") from err
    if doc.get("summary", {}).get("unit") != unit_id:
        raise InputError(f"{path} is the comparison of unit {doc.get('summary', {}).get('unit')!r}, not {unit_id!r}")
    population = doc.get("agreed", [])

    def order(e):
        ident = json.dumps([e.get(k) for k in ("key", "when", "scheme_file", "accent")], sort_keys=True,
                           ensure_ascii=False)
        return hashlib.sha256(f"{seed}\n{ident}".encode("utf-8")).hexdigest()

    k = sample_size(len(population), rate, least)
    chosen = sorted(sorted(range(len(population)), key=lambda i: order(population[i]))[:k])
    picked = [population[i] for i in chosen]
    s = ["# The audit sample of the agreed facts (scripts/verify_facts.py sample): the tie-breaker checks each.", "",
         "[sample]", f"unit = {toml_string(unit_id)}", f"rev = {toml_string(doc['summary'].get('rev', ''))}",
         f"seed = {toml_string(str(seed))}", f"rate = {toml_inline(float(rate))}", f"min = {least}",
         f"population = {len(population)}", f"size = {k}",
         "order = \"the agreed facts by the SHA-256 of the seed, a newline and the identity's JSON "
         "([key, when, scheme_file, accent], keys sorted), the first `size` taken\""]
    for e in picked:
        s += ["", "[[fact]]"] + [f"{key} = {toml_inline(v)}" for key, v in e.items()]
    (Path(compared) / "sample.toml").write_text("\n".join(s) + "\n", encoding="utf-8")
    return picked


# ------------------------------------------------------------------------------------------------ the CLI

def parse_since(values):
    out = {}
    for v in values or []:
        if "=" in v:
            p, c = v.split("=", 1)
            if p not in PLATFORMS:
                raise InputError(f"--since {v}: no platform {p!r} (one of {', '.join(PLATFORMS)})")
            out[p] = c
        else:
            out.update({p: v for p in PLATFORMS})
    return out


def counts_report(bases, counts, units):
    lines = ["# platform\tbaseline\tkind\tnew\tchanged\tremoved\tto verify"]
    for p, c in counts.items():
        for kind in ("pending", "measured", "cited"):
            n = [c.get((kind, s), 0) for s in ("new", "changed", "removed")]
            todo = 0 if kind == "pending" else n[0] + n[1]
            lines.append(f"# {p}\t{bases[p][:8]}\t{kind}\t{n[0]}\t{n[1]}\t{n[2]}\t{todo}")
    for u in units:
        lines.append(f"# {u.id}\t{u.platform}\t{u.kind}\t{len(u.entries)}\t"
                     + "; ".join(f"{s['file']} {s['prefix']} {s['facts']}" for s in u.scope()))
    return "\n".join(lines)


def main(argv) -> int:
    if argv[1:] == ["--self-test"]:
        return self_test()
    ap = argparse.ArgumentParser(prog="verify_facts.py", description=__doc__.split("\n\n")[0])
    sub = ap.add_subparsers(dest="cmd", required=True)
    u = sub.add_parser("units")
    u.add_argument("--since", action="append")
    u.add_argument("--rev", default="HEAD")
    u.add_argument("--out")
    b = sub.add_parser("brief")
    b.add_argument("unit")
    b.add_argument("out_dir")
    b.add_argument("--lane-brief", required=True)
    b.add_argument("--units")
    b.add_argument("--cache")
    b.add_argument("--no-combinations", action="store_true")
    c = sub.add_parser("compare")
    c.add_argument("unit")
    c.add_argument("verifier_facts")
    c.add_argument("--units")
    c.add_argument("--out")
    s = sub.add_parser("sample")
    s.add_argument("unit")
    s.add_argument("--compared", required=True)
    s.add_argument("--seed", required=True)
    s.add_argument("--rate", type=float, default=0.05)
    s.add_argument("--min", type=int, default=10)
    try:
        args = ap.parse_args(argv[1:])
    except SystemExit as err:
        return 2 if err.code else 0
    root = Path(".")
    try:
        if args.cmd == "units":
            rev, bases, counts, units = compute_units(root, args.rev, parse_since(args.since))
            text = units_text(rev, bases, counts, units)
            if args.out:
                Path(args.out).write_text(text, encoding="utf-8")
            else:
                sys.stdout.write(text)
            print(counts_report(bases, counts, units), file=sys.stderr)
            return 0
        if args.cmd == "brief":
            try:
                lane_brief = Path(args.lane_brief).read_text(encoding="utf-8")
            except (OSError, UnicodeDecodeError) as err:
                raise InputError(f"--lane-brief {args.lane_brief}: {err}") from err
            found, _ = brief(root, read_units(root, args.units), args.unit, args.out_dir, lane_brief,
                          citations.cache_dir(args.cache), combinations=not args.no_combinations)
            counts, report = leak_report(found)
            print(report)
            return 1 if counts["leak"] else 0
        if args.cmd == "compare":
            summary = compare(root, read_units(root, args.units), args.unit, args.verifier_facts, args.out)
            print(f"# {summary['unit']}: agreed {summary['agreed']}, disagreed {summary['disagreed']}, lane-only "
                  f"{summary['lane_only']}, verifier-only {summary['verifier_only']}, outside the unit "
                  f"{summary['outside_unit']} ({summary['outside_unit_disagreed']} differing)")
            print(summary["ledger"])
            return 0
        picked = sample(args.unit, args.compared, args.seed, args.rate, args.min)
        print(f"# {args.unit}: {len(picked)} agreed facts sampled with seed {args.seed} "
              f"({Path(args.compared) / 'sample.toml'})")
        return 0
    except InputError as err:
        print(f"error: {err}", file=sys.stderr)
        return 2


# --------------------------------------------------------------------------------------------- self-test

SELF_TEST_CATALOGUE = '''# catalogue

## Buttons and toggles

### button

- **Kind:** typed widget.
- **Parts:**
  - `body` — the root: `fill`, `corners`.
- **Variants:** typed `flat`; keyed `@pill`.
- **Flags:** keyed `@busy`.
- **Exclusive groups:** none; precedence: state models (S17).
- **State-model source:**
  - `adwaita`: libadwaita's selectors in `_buttons.scss` (`border-radius: 6px;`).
- **Notes:** a note naming #ffffff.
'''

SELF_TEST_SOURCES = '''
[[source]]
id = "libadwaita@1.10.0"
kind = "git"
generations = ["adwaita"]

[[source]]
id = "breeze@6.8.2"
kind = "git"
generations = ["kde-breeze"]

[[source]]
id = "native-theme-evidence"
kind = "git"
evidence = true
generations = []
'''

SELF_TEST_LANE_BRIEF = '''# Facts lane brief

## Hard rules
- Never invent a value.

## The format (normative)
- One directory per platform.

## Method
1. State model first.

## Canonical forms (binding)
- **Paint:** a single colour layer is the colour itself (`value = "#ffffff"`).
'''

_CITE = 'kind = "published"\nsource = "libadwaita@1.10.0 src/stylesheet/widgets/_buttons.scss:{n}"\nquote = "{q}"\n'
_EV = "docs/evidence/gnome-host/2026-10-30-run-1/gnome-51/nodes.toml#button.{n}.radius"


def _fact(key, value, when=None, n=1, q="color: #ffffff;", measured=False, pending=None):
    out = f"[[fact]]\nkey = '{key}'\n" if '"' in key else f'[[fact]]\nkey = "{key}"\n'
    if when:
        out += f"when = {when}\n"
    if pending:
        return out + f'pending = "{pending}"\nrows = []\n\n'
    out += f"value = {value}\nrows = []\n\n[[fact.cite]]\n"
    if measured:
        return out + f'kind = "measured"\nevidence = "{_EV.format(n=n)}"\ntolerance = 0\n\n'
    return out + _CITE.format(n=n, q=q) + "\n"


def _repo(base_files, head_files):
    tmp = Path(tempfile.mkdtemp(prefix="verify-facts-"))

    def git(*a):
        subprocess.run(["git", "-C", str(tmp), "-c", "user.name=t", "-c", "user.email=t@t", "-c",
                        "commit.gpgsign=false", *a], check=True, capture_output=True)
    git("init", "-q")
    for files in (base_files, head_files):
        for p in tmp.joinpath("docs").glob("**/*.toml"):
            p.unlink()
        for rel, text in files.items():
            p = tmp / rel
            p.parent.mkdir(parents=True, exist_ok=True)
            p.write_text(text, encoding="utf-8")
        git("add", "-A")
        git("commit", "-q", "-m", "fixture", "--allow-empty")
    return tmp


def self_test() -> int:
    failures = []

    def check(label, got, want):
        if got != want:
            failures.append(f"{label}: got {got!r}, want {want!r}")

    common = {SOURCES.as_posix(): SELF_TEST_SOURCES, CATALOGUE.as_posix(): SELF_TEST_CATALOGUE}
    base_button = "".join([
        _fact("button.body.fill", '"#ffffff"', '{ states = ["hover"] }', 1),
        _fact("button.body.fill", '"#eeeeee"', '{ states = ["pressed"] }', 2, q="color: #eeeeee;"),
        _fact("button.body.corners.top_left", '"6px"', None, 3, q="border-radius: 6px;"),
        _fact("button.body.fill", None, '{ states = ["focused"] }', pending="U1"),
    ])
    head_button = "".join([
        _fact("button.body.fill", '"#ffffff"', '{ states = ["hover"] }', 1),           # same
        _fact("button.body.fill", '"#dddddd"', '{ states = ["pressed"] }', 2, q="color: #dddddd;"),   # changed
        _fact("button.body.fill", None, '{ states = ["focused"] }', pending="U1"),   # same, pending
        _fact("button.body.fill", None, '{ states = ["disabled"] }', pending="U2"),  # new, pending
        _fact("button.body.corners.top_right", '"6px"', None, 4, q="border-radius: 6px;"),   # new
        _fact("button.body.corners.bottom_left", '"6px"', None, 5, q="border-radius: 6px;"),   # new
        _fact("button.body.corners.bottom_right", '"6px"', '{ states = ["hover", "pressed"] }', 6,
              q="border-radius: 6px;"),   # new
        _fact("button.body.opacity", "0.5", '{ states = ["disabled"] }', 7, q="opacity: 0.5;"),   # new
        _fact("button.body.corners.top_left", '"8px"', '{ states = ["hover"] }', 8, measured=True),   # new, measured
    ])
    head_tokens = _fact("token.focus.width", '"2px"', None, 9, q="outline-width: 2px;")   # new
    repo = _repo({**common, "docs/facts/gnome/button.toml": base_button},
                 {**common, "docs/facts/gnome/button.toml": head_button, "docs/facts/gnome/tokens.toml": head_tokens})
    out = Path(tempfile.mkdtemp(prefix="verify-facts-out-"))
    try:
        since = {p: "HEAD~1" for p in PLATFORMS}
        rev, bases, counts, units = compute_units(repo, "HEAD", since, lo=2, hi=3)
        g = counts["gnome"]
        check("units: cited new", g.get(("cited", "new")), 5)
        check("units: cited changed", g.get(("cited", "changed")), 1)
        check("units: cited removed (top_left without a when)", g.get(("cited", "removed")), 1)
        check("units: measured new", g.get(("measured", "new")), 1)
        check("units: pending new", g.get(("pending", "new")), 1)
        cited = [u for u in units if u.platform == "gnome" and u.kind == "cited"]
        check("units: cited split into pieces of at most 3", [len(u.entries) for u in cited], [3, 3])
        check("units: ids", [u.id for u in units if u.platform == "gnome"], ["GN-C1", "GN-C2", "GN-M1"])
        check("units: a unit's scope", cited[1].scope(),
              [{"file": "button.toml", "prefix": "button.body", "facts": 2},
               {"file": "tokens.toml", "prefix": "token.focus.width", "facts": 1}])
        text = units_text(rev, bases, counts, units, 2, 3)
        check("units: deterministic", units_text(*compute_units(repo, "HEAD", since, lo=2, hi=3), 2, 3), text)
        doc = tomllib.loads(text)
        check("units: file names every to-verify fact once",
              sum(u["facts"] for u in doc["unit"] if u["platform"] == "gnome"), 7)
        check("pack: a small rest merged", [len(x) for x in pack([[1] * 3, [1] * 3, [1]], 2, 4)], [3, 4])
        check("split: a whole key's facts one by one", [len(x) for x in split(
            [Entry(None, "new", ("a", "b")) for _ in range(5)], 0, 2)], [1, 1, 1, 1, 1])
        check("pack: a whole key's facts fill the units", [len(x) for x in pack([[1]] * 5 + [[1] * 2], 1, 3)],
              [3, 2, 2])

        # the brief and its leak check
        lane_brief = SELF_TEST_LANE_BRIEF
        unit_id = cited[0].id
        found, inputs = brief(repo, doc, unit_id, out / "brief", lane_brief, Path("/cache"))
        classes = {f[0] for f in found}
        check("brief: no leak", [f for f in found if f[0] == "leak"], [])
        check("brief: files", sorted(p.name for p in (out / "brief").iterdir()),
              ["README.md", "catalogue.md", "combinations.toml", "facts-format.md", "keys.toml", "pins.toml"])
        check("brief: the format excerpt's sections",
              [x for x in (out / "brief" / "facts-format.md").read_text().splitlines() if x.startswith("## ")],
              ["## The format (normative)", "## Canonical forms (binding)"])
        pins = tomllib.loads((out / "brief" / "pins.toml").read_text())
        check("brief: the platform's pins only, no evidence entry", [e["id"] for e in pins["source"]],
              ["libadwaita@1.10.0"])
        cat = (out / "brief" / "catalogue.md").read_text()
        check("brief: the catalogue's items, not its notes", ("State-model source" in cat, "Notes" in cat),
              (True, False))
        check("brief: a quote the catalogue's lines hold is the specification's",
              [(f[0], f[3]) for f in found if f[1] == "border-radius: 6px;" and f[0] != "trivial"],
              [("specification", "catalogue.md")])
        combos = tomllib.loads((out / "brief" / "combinations.toml").read_text())
        check("brief: no value or citation in the combinations",
              any("value" in c or "cite" in c for c in combos["combination"]), False)
        facts1 = unit_facts(repo, doc, find_unit(doc, unit_id))[0]
        readme = out / "brief" / "README.md"
        readme.write_text(readme.read_text() + "\nlibadwaita@1.10.0 src/stylesheet/widgets/_buttons.scss:4\n")
        combos_file = out / "brief" / "combinations.toml"
        combos_file.write_text(combos_file.read_text() + '\n# border-radius: 6px;\n')
        planted = leak_check(out / "brief", facts1, inputs)
        check("leak check: a planted citation and quote are leaks",
              sorted((f[3], f[2]) for f in planted if f[0] == "leak"),
              [("README.md", "a citation's source"), ("combinations.toml", "a citation's quote")])
        trivial = leak_check(out / "brief", [Fact("x.toml", 1, {"key": "k", "value": 0}, [], ())], {})
        check("leak check: `0` is trivial", {f[0] for f in trivial}, {"trivial"})
        nocombo, _ = brief(repo, doc, unit_id, out / "brief2", lane_brief, Path("/cache"), combinations=False)
        check("brief: --no-combinations", (out / "brief2" / "combinations.toml").exists(), False)
        check("brief: --no-combinations, no leak", [f for f in nocombo if f[0] == "leak"], [])
        ev_root = out / "evidence"
        ev = ev_root / "docs/evidence/gnome-host/2026-10-30-run-1"
        (ev / "gnome-51").mkdir(parents=True)
        (ev / "gnome-51" / "nodes.toml").write_text("[button.8]\nradius = 8\n")
        (ev / "run.toml").write_text('[images.gnome-51]\npreset = "adwaita"\n')
        (ev / "analysis").mkdir()
        (ev / "analysis" / "read_nodes.py").write_text("print('method')\n")
        mid = next(u.id for u in units if u.kind == "measured")
        mfound, _ = brief(repo, doc, mid, out / "brief3", lane_brief, Path("/cache"), evidence_root=ev_root)
        check("brief: a measured unit's evidence, run.toml and method",
              sorted(p.relative_to(out / "brief3").as_posix() for p in (out / "brief3" / "docs").rglob("*")
                     if p.is_file()),
              ["docs/evidence/gnome-host/2026-10-30-run-1/analysis/read_nodes.py",
               "docs/evidence/gnome-host/2026-10-30-run-1/gnome-51/nodes.toml",
               "docs/evidence/gnome-host/2026-10-30-run-1/run.toml"])
        check("brief: the evidence copies are not scanned", [f for f in mfound if f[0] == "leak"], [])

        # the compare: normalisations, an identity mismatch, one-sided facts
        units1 = {"rev": doc["rev"], "unit": [{
            "id": "GN-T1", "platform": "gnome", "kind": "cited", "presets": ["adwaita"], "facts": 6,
            "identities": [{"file": "button.toml", "key": k, **({"when": w} if w else {})} for k, w in (
                ("button.body.fill", {"states": ["hover"]}),
                ("button.body.fill", {"states": ["pressed"]}),
                ("button.body.corners.top_right", None),
                ("button.body.corners.bottom_left", None),
                ("button.body.corners.bottom_right", {"states": ["hover", "pressed"]}),
                ("button.body.opacity", {"states": ["disabled"]}))]}]}
        vfile = out / "verifier" / "facts.toml"
        vfile.parent.mkdir()
        vfile.write_text("".join([
            _fact("button.body.fill", '"#FFFFFF"', '{ states = ["hover"], not_states = [] }', 10),   # case, empty
            _fact("button.body.fill", '"srgb(0.8, 0.8, 0.8)"', '{ states = ["pressed"] }', 11),   # disagrees
            _fact("button.body.corners.top_right", '"6.0px"', None, 12),               # number format
            _fact('button.body."corners".bottom_left', '"6px"', None, 13),            # key quoting
            _fact("button.body.corners.bottom_right", '"6px"', '{ states = ["pressed", "hover"] }', 14),   # sets
            _fact("button.body.opacity", "0.5", '{ states = ["hover"] }', 15),         # identity mismatch
            _fact("button.body.fill", '"#cccccc"', '{ states = ["focused"] }', 16),    # lane pending: outside
            _fact("token.focus.width", '"srgb(1, 1, 1)"', None, 17),                    # lane outside the unit
            _fact("button.body.stroke.width", '"1px"', None, 18),                     # verifier-only
            _fact("button.body.corners.top_left", '"8.0px"', '{ states = ["hover"] }', 19),   # outside, agreed
        ]))
        summary = compare(repo, units1, "GN-T1", vfile)
        check("compare: counts", {k: summary[k] for k in ("agreed", "disagreed", "lane_only", "verifier_only",
                                                           "outside_unit", "outside_unit_disagreed",
                                                           "agreed_before_normalising")},
              {"agreed": 4, "disagreed": 1, "lane_only": 1, "verifier_only": 2, "outside_unit": 3,
               "outside_unit_disagreed": 2, "agreed_before_normalising": 2})
        tb = tomllib.loads((out / "verifier" / "tiebreak.toml").read_text())
        norms = {n["name"]: (n["changed"], n["decisive"]) for n in tb["normalisation"]}
        check("compare: colour case decisive", norms["colour_case"], (1, 1))
        check("compare: empty selectors decisive", norms["empty_selectors"], (1, 1))
        check("compare: number format decisive", norms["number_format"], (1, 1))
        check("compare: key quoting decisive", norms["key_quoting"], (1, 1))
        check("compare: selector sets decisive", norms["selector_sets"], (1, 1))
        check("compare: the disagreement's values and citations",
              (tb["disagreed"][0]["lane"]["value"], tb["disagreed"][0]["verifier"]["value"],
               tb["disagreed"][0]["lane"]["cite"][0]["source"][-2:], tb["disagreed"][0]["verifier"]["line"] > 0),
              ("#dddddd", "srgb(0.8, 0.8, 0.8)", ":2", True))
        check("compare: the identity mismatch is one-sided both ways",
              (tb["lane_only"][0]["when"], sorted(v["key"] for v in tb["verifier_only"])),
              ({"states": ["disabled"]}, ["button.body.opacity", "button.body.stroke.width"]))
        check("compare: the lane's pending fact outside the unit",
              sorted(o["lane"].get("pending", o["lane"].get("value")) for o in tb["outside_unit"]), ["2px", "U1"])
        check("compare: the ledger line", summary["ledger"].startswith(
            "| GN-T1 | gnome | lane 6, verifier 10 | 4 of 5 compared combinations after normalising spellings "
            "(80.0 %; 40.0 % before) | to tie-break: 1 disagreed, 1 lane-only, 2 verifier-only | 4 | |"), True)
        check("normalise: srgb is its hex", norm_value("srgb(1, 1, 1)", tuple(NORMALISATIONS), set()),
              ("string", "#ffffff"))
        check("normalise: an expression", norm_value("= alpha( surface.foreground , 0.50 )", tuple(NORMALISATIONS),
                                                      set()), ("string", "=alpha(surface.foreground,0.5)"))
        check("normalise: true is no 1", norm_value(True, tuple(NORMALISATIONS), set()) ==
              norm_value(1, tuple(NORMALISATIONS), set()), False)

        # the sample: deterministic, recorded with its seed, its size by the rule
        agreed_doc = out / "many"
        agreed_doc.mkdir()
        rows = ["[summary]", 'unit = "GN-T1"', 'rev = "x"', "agreed = 300"]
        for i in range(300):
            rows += ["", "[[agreed]]", f'key = "button.body.k{i}"', 'lane = { value = "1px" }']
        (agreed_doc / "agreed.toml").write_text("\n".join(rows) + "\n")
        a = [e["key"] for e in sample("GN-T1", agreed_doc, 7)]
        first = (agreed_doc / "sample.toml").read_text()
        b = [e["key"] for e in sample("GN-T1", agreed_doc, 7)]
        check("sample: the same seed, the same sample", (a, first), (b, (agreed_doc / "sample.toml").read_text()))
        c = [e["key"] for e in sample("GN-T1", agreed_doc, 8)]
        check("sample: another seed, another sample", a != c, True)
        check("sample: max(min, ceil(rate x agreed))", (len(a), sample_size(200, 0.05, 10), sample_size(5, 0.05, 10),
                                                        sample_size(499, 0.05, 10)), (15, 10, 5, 25))
        rec = tomllib.loads((agreed_doc / "sample.toml").read_text())["sample"]
        check("sample: recorded with its seed", (rec["seed"], rec["population"], rec["size"]), ("8", 300, 15))
        try:
            sample("GN-T2", agreed_doc, 7)
            failures.append("sample: another unit's comparison accepted")
        except InputError:
            pass
    except (InputError, OSError, subprocess.CalledProcessError, tomllib.TOMLDecodeError, KeyError) as err:
        failures.append(f"fixture run failed: {type(err).__name__}: {err}")
    finally:
        shutil.rmtree(repo, ignore_errors=True)
        shutil.rmtree(out, ignore_errors=True)
    for f in failures:
        print(f"self-test FAIL: {f}", file=sys.stderr)
    if not failures:
        print("self-test ok (units, brief and leak check, compare and its normalisations, sample)")
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
