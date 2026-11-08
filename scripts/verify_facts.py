#!/usr/bin/env python3
"""The verifiers' tooling (the 0.8.0 plan's *Where the work happens* and *How values are checked*, "Facts,
independently"): the units of a delta, the verifier's brief, the comparison of the state models and of the facts of a
lane and its verifier, and the audit sample. Offline but for `git`; the brief of a measured unit reads the evidence
checkout.

Run from the repository root:

    python3 scripts/verify_facts.py units [--since [PLATFORM=]COMMIT ...] [--rev REV] [--out FILE]
    python3 scripts/verify_facts.py brief <unit> <out-dir> --lane-brief FILE [--units FILE] [--cache DIR]
                                          [--no-combinations | --combinations-from MODEL-COMPARE]
    python3 scripts/verify_facts.py compare-model <unit> <verifier-state-models.toml> [--units FILE] [--out DIR]
    python3 scripts/verify_facts.py compare <unit> <verifier-facts.toml> [--units FILE] [--out DIR]
                                            [--combinations-from MODEL-COMPARE]
    python3 scripts/verify_facts.py sample <unit> --compared DIR --seed N [--rate 0.05] [--min 10]
    python3 scripts/verify_facts.py --self-test

The flow per unit (the plan: the state model first, compared mechanically, then the values of the agreed
combinations):

    1. units --out units.toml
    2. brief <unit> <A> --no-combinations           the verifier writes <A>/state-models.toml
    3. compare-model <unit> <A>/state-models.toml   writes <A>/model-compare.toml (its differences to the tie-breaker)
    4. brief <unit> <B> --combinations-from <A>/model-compare.toml    the verifier writes <B>/facts.toml
    5. compare <unit> <B>/facts.toml --combinations-from <A>/model-compare.toml    writes <B>/tiebreak.toml, agreed.toml
    6. sample <unit> --compared <B> --seed N        writes <B>/sample.toml

(`brief` with neither option gives one folder for both steps, and `compare` without the option compares every
combination of the unit.)

The facts and state models are read as scripts/check_facts.py reads them (items_of, canonical_key, canonical_when; the
fact files and `state-models.toml` of `docs/facts/<platform>/`), at a commit, through `git show`.

A fact's identity (S17 *Which fact applies*): its key read as a TOML dotted key (check_facts.canonical_key), its
`when` with empty lists and tables left out and its name lists read as sets (check_facts.canonical_when), its
`scheme_file` and its `accent` (a colour, spelled as NORMALISATIONS' colour rules spell it). Within a unit a fact
is also named by its file. Where one file states an identity twice, the last fact applies and is the one used. A
fact's kind: `pending` (`pending = "U<n>"`, no citation, nothing to verify), `measured` (a `kind = "measured"`
citation), `cited` (only `published` and `read` citations).

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
same commits give the same file. The other subcommands read it (`--units`), or compute it as `units` with no option
does.

`brief <unit> <out-dir>`: the verifier's folder (the plan: the unit's keys, the pins, the catalogue's state-model
source lines and, for `measured` facts, copies of the cited evidence and the method's script), every file of it
read at the units' revision:

- `README.md`: the task, what to read (this folder and the source cache only) and what to write: `state-models.toml`
  (with `--no-combinations`), `facts.toml` (with `--combinations-from`), or both;
- `keys.toml`: the unit, its presets and its keys; `combinations.toml`: the combinations to state, each a key with its
  `when`, `scheme_file` and `accent` as the lane wrote them: every one, or with `--combinations-from` those a
  compare-model result agrees on; `--no-combinations` leaves the file out;
- `pins.toml`: the `docs/facts/sources.toml` entries serving a preset of the unit's platform (all of them, so the
  list does not say which the lane cited; the evidence repository's entry left out), each with its checkout's path in
  the source cache (`--cache`, else citations.cache_dir);
- `catalogue.md`: the catalogue's CATALOGUE_SECTIONS (what an entry's names mean) and, for each widget the keys
  name (a catalogue entry, `component.<key>` too), its whole entry, as the catalogue writes them, with their lines;
- `spec.md`: the specification's normative sections, SPEC_SECTIONS (S1 to S21: the parts, fields and their types,
  the roles, tokens, surfaces and bindings, the expressions, references, functions and absence, the declarations,
  the preset spelling and, in S17, the facts and state models; not the progress table nor S22, the additions
  register; the traces, gap maps and `platform-facts.md`, which state platform values, are never copied), each
  found by its heading, from it to the next heading of its level or above (headings inside code blocks not
  counted); a section it cannot find stops the brief;
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
sources.toml, `catalogue.md` from the catalogue, `spec.md` from the specification, `facts-format.md` from the lane
brief, `README.md` from the unit's id, platform, kind, presets and file list, each with the file's own headings. A
string found is a collision: `trivial` where it is shorter than FLOOR characters or a keyword or bool (KEYWORDS);
else, where the file's inputs hold it too, unavoidable, classed by the file (LEAK_CLASSES: `identity`, the verifier
is told what to state; `specification`, the model's vocabulary in the texts every lane and verifier reads;
`sources`; `template`), but `normative-value` where the string is a platform value (platform_value: a citation, a
value holding a literal, a text that is no name) in normative text, a leak to report; else a `leak`, a string that
can only have come from the lane's facts. It prints the counts and each collision but the trivial ones (to stdout,
never into the folder); a leak or a normative value is exit 1. So the brief passes it, `brief` elides first: in its
copies of the normative text (REDACTED_FILES), each platform value and citation of the unit (redact: no trivial
string, none its identities hold) is replaced by an ELIDED marker, the text around it kept, and `README.md` lists
each elision by file, line and class (a value or a citation), never the string; the repository's documents are not
changed.

`compare-model <unit> <verifier-state-models.toml>`: the lane's state-model entries of the unit's widgets against the
verifier's, an entry named by its widget and its `when` (its `preset:` or `flavor:` conditions). Per entry both
state: each declaration (`available`, `variants`, `sizes`, `densities`, `axes.states`, `axes.conditions`,
`axes.contexts`, `exclusive`, `not_drawn`, `flags`, `default_size`, `requires_variants`, `sizes_by_variant`,
`sizes_by_context`, `exclusive_variants`, `variant_sets`, `variant_products`, `recency`) whose members differ, with
the members only each side states (name lists as sets; an exclusive group and a recency set in their order, which
is their precedence); and the combinations check_facts.py's expansion gives each side and not the other: the state
combinations (check_facts.state_combinations) and the variant sets (check_facts.drawn_set_lists). Then each
combination of the unit (its facts' `when`s) is agreed or refused: refused where the verifier's entries draw none of
the combinations it holds in and the lane's may (entry_refusal: no entry applies, check_facts.applicable_entries;
check_facts.model_never; a state neither in `axes.states` nor in `flags`, or no state combination holding it; a
variant, size, density or condition the entry does not declare, or no variant set holding its variants; a context
no `axes.contexts` chain matches, check_facts.selector_matches; a `not_drawn` entry whose lists it implies). A
combination the lane's own model refuses too is kept, marked `lane_model_refuses`. A key that names no widget (a
colour role, a token, an input) is agreed. It writes `model-compare.toml` into `--out` (the verifier file's folder):
the summary, each entry with its differences, the agreed combinations (`[[combination]]`, which `brief` and
`compare` read with `--combinations-from`) and the refused ones with the reasons.

`compare <unit> <verifier-facts.toml>`: the lane's facts of the unit (with `--combinations-from`, those of the agreed
combinations) against the verifier's file, by identity, each pair by the values that apply in its combination
(compare_pair). The combination is the lane fact's `when` under each world it holds in (worlds: its conditions, with
each scheme and preset it leaves open; explore() opens every other condition it leaves open that a fact or a
reference the comparison reads names, splitting the world, the condition held and not, as for the schemes, a
one-valued condition's other values and CONTRAST's other member refused where one is held), in its minimal form
(generate_preset.minimal: its names and nothing else, the size an element holds by the lane's state models). In each
world each side's value is the one its facts give the combination (S17 *Which fact applies*: the last fact of the
key and run in file order whose `when` holds there after the exclusive groups' drop, generate_preset.holds, the
preset generator's round-trip matcher; the pair's own where none does or no element holds the combination), so a
later general fact over an earlier specific one is seen; a world where an opened condition hands either side's
combination to another fact than the world without it does is that fact's combination, compared under its own
identity (or one-sided), not this pair's. The values are compared after NORMALISATIONS, each listed
in the output with the facts whose spelling it changed and the pairs that agree only through it; a pair equal in
every world is agreed. Where the values still differ in a world and either holds an expression or a reference, they
are resolved there (Resolver): references read from the lane's facts of the platform and S2.2's arithmetic,
`alpha`, `with_alpha`, `mix` and `pick` evaluated; a pair equal in every world, by resolution in one at least, is
`equivalent by resolution`, a class of its own, never agreed nor in the audit sample's population, listed with its
values per world; any other stays disagreed, with the values per world, the fact that applies where it is not the
pair's own, or why they did not resolve (`absent` is no colour, so it never equals a transparent one), a plain
disagreement (no resolution, the pair's own facts in every world) without them. Counts: agreed, equivalent by
resolution, disagreed, lane-only (the compared identities the verifier does not state), verifier-only (the
verifier's identities the lane states nowhere on the platform), outside the compared combinations (the verifier's
identities the lane states elsewhere: unchanged since the baseline, pending, or refused by the model comparison) and
refused by the model. It writes into `--out` (the verifier file's
folder): `tiebreak.toml`, the summary, the normalisations, a summary line in the ledger's *Verification rates*
table's form, each equivalent and each disagreement with both values, both citations and the resolution, each
one-sided fact with its value and citations, and each fact outside the compared combinations whose values differ (a
lane's pending fact among them) with both; `agreed.toml`, the agreed facts with both citations, which `sample` reads.

`sample <unit> --compared DIR --seed N`: the audit sample of the agreed facts of `DIR/agreed.toml` (the plan: 5 %
per lane and unit, at least ten), `max(--min, ceil(--rate × agreed))` of them, never more than there are: the agreed
facts ordered by the SHA-256 of `<seed>` and the identity, the first ones taken, so a seed gives the same sample on
any machine and Python. It writes `DIR/sample.toml` with the seed, the rate, the minimum, the population and each
sampled fact with both values and citations.

Exit 0, 1 (`brief`: a leak), or 2 on an input it cannot read, an unknown unit or a wrong command line.

    python3 scripts/verify_facts.py --self-test   # the units, the brief and its leak check, the specification's
                                                  # sections, the model comparison and the two-step flow, the
                                                  # normalisations, an identity mismatch, one-sided facts and the
                                                  # sample's determinism, on fixtures
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
import generate_preset
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

# The catalogue's sections that define what an entry's names mean, before the entries.
# The catalogue sections every brief needs, and those copied where the catalogue has them (the colour roles',
# tokens' and keyed components' definitions, which a unit of roles or tokens reads).
CATALOGUE_REQUIRED = ("Entry format", "Shared value types", "Shared flags", "Selector notation")
CATALOGUE_SECTIONS = CATALOGUE_REQUIRED + ("Nested controls", "Keyed components", "Colour roles and text ramp",
                                           "Tokens, conditions, readers, placement")
FORMAT_SECTIONS = ("## The format", "## Canonical forms")
SPEC = check_facts.CORE
# The specification's normative sections: S1 to S21, which define the keys (parts, fields and their types, roles,
# tokens, surfaces, bindings), the value grammar (expressions, references, functions, absence) and the facts; not its
# progress table nor S22, the additions register.
SPEC_SECTIONS = tuple(f"S{n}" for n in range(1, 22))
SPEC_HEADING = re.compile(r"^(#{2,4}) (S\d+(?:\.\d+)*)[.\s]")
SPEC_HEADER = "# The specification's sections the facts format reads"
FLOOR = 4
KEYWORDS = {"true", "false", "auto", "absent", "none"}
EVIDENCE_DIR = "docs/evidence"
ELIDED = "⟦elided: {} of this unit⟧"
REDACTED_FILES = ("spec.md", "catalogue.md", "facts-format.md")
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
    "omitted_option": "an `Option` field inside a list item or an enum variant's payload stated `\"none\"` is left "
                      "out, as S11 reads one left out there (`None`; the core's types, check_facts.key_info)",
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


def fact_value(data, on, used, types=None):
    if "pending" in data:
        return ("pending", data.get("pending"))
    return norm_value(stated_value(data, on, used, types), on, used)


def stated_value(data, on, used, types):
    """A fact's value with the `Option` members S11 reads as `None` when left out (inside a list item or an enum
    variant's payload) left out where they state `"none"` (omitted_option; `types`, a check_facts.Context, gives the
    core's types)."""
    v = data.get("value")
    if "omitted_option" not in on or types is None or not isinstance(v, (list, dict)):
        return v
    typ = check_facts.key_info(types, data.get("key"))[4] if isinstance(data.get("key"), str) else None
    out = without_none_options(types, v, typ)
    if out != v:
        used.add("omitted_option")
    return out


def without_none_options(types, v, typ, inside=False):
    """`v` under the core's type `typ` without the `Option` members stated `"none"` inside a list item or an enum
    variant's payload (S11: "an `Option` inside a list item or an enum variant's payload left out is `None`"); an
    `Option` field elsewhere, left out, is missing, and is kept."""
    if isinstance(v, list):
        item = check_facts.item_type(typ) if typ else None
        return [without_none_options(types, x, item, True) for x in v]
    if not isinstance(v, dict):
        return v
    variant = bool(typ) and check_facts.strip_type(typ) in types.enums
    out = {}
    for k, x in v.items():
        t = check_facts.child_type(types, typ, k) if typ else None
        if inside and not variant and x == "none" and t and t.strip().startswith("Option<"):
            continue
        out[k] = without_none_options(types, x, t, inside or variant)
    return out


def type_context(root, rev):
    """The core's and the catalogue's types at a commit, as check_facts reads them (key_info, child_type)."""
    tree = Tree(root, rev)
    try:
        return check_facts.Context(root, core_text=tree.text(SPEC.as_posix()),
                                   cat_text=tree.text(CATALOGUE.as_posix()),
                                   sources_text=tree.text(SOURCES.as_posix()), presets=set(), rows=set())
    except check_facts.InputError as err:
        raise InputError(f"the core's types at {rev}: {err}") from err


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


def heading_spans(text, level_of):
    """[(first line, last line, heading's key)] of the text's sections whose heading `level_of` keys (returning
    (level, key) or None), each up to the next heading of its level or above; headings inside code blocks are not
    headings."""
    lines = text.splitlines()
    heads, fence = [], False
    for n, line in enumerate(lines, 1):
        if line.startswith("```"):
            fence = not fence
            continue
        m = re.match(r"^(#{1,6}) ", line) if not fence else None
        if m:
            heads.append((n, len(m.group(1)), level_of(line)))
    out = []
    for i, (n, level, key) in enumerate(heads):
        if key is None:
            continue
        end = next((m - 1 for m, lv, _ in heads[i + 1:] if lv <= level), len(lines))
        while end > n and not lines[end - 1].strip():
            end -= 1
        out.append((n, end, key))
    return out


def catalogue_text(cat_text, widgets):
    """(the text, the lines it adds to the catalogue's): the catalogue's CATALOGUE_SECTIONS and each widget's whole
    entry, as the catalogue writes them."""
    lines = cat_text.splitlines()
    names = {e.name for e in spec_catalogue.entries(cat_text)}

    def key(line):
        m = re.match(r"^## (.+?)\s*$", line)
        if m and m.group(1) in CATALOGUE_SECTIONS:
            return m.group(1)
        m = spec_catalogue.TYPED_HEADING.match(line) or spec_catalogue.COMPONENT_HEADING.match(line)
        if m:
            name = m.group(1) if line.startswith("### ") else f"component.{m.group(1)}"
            return name if name in widgets and name in names else None
        return None

    out = ["# The catalogue's sections for the unit's widgets", "",
           f"From `{CATALOGUE.as_posix()}`: {', '.join(CATALOGUE_SECTIONS)}, and each widget's whole entry, as the "
           "catalogue writes them."]
    if not widgets:
        out += ["", "The unit's keys name no widget: they are colour roles, tokens or inputs, whose facts take "
                    "conditions only (S7)."]
    added = list(out)
    spans = heading_spans(cat_text, key)
    found = {k for _, _, k in spans}
    missing = sorted((set(CATALOGUE_REQUIRED) | set(widgets)) - found)
    if missing:
        raise InputError(f"{CATALOGUE}: no section {', '.join(missing)}")
    for a, b, _ in spans:
        mark = f"<!-- the catalogue's lines {a}-{b} -->"
        out += ["", mark, ""] + lines[a - 1:b]
        added.append(mark)
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


def spec_text(spec, ids=SPEC_SECTIONS):
    """(the text, the lines it adds to the specification's): the sections `ids`, each from its heading to the next
    heading of its level or above (headings inside code blocks not counted), in the specification's order."""
    lines = spec.splitlines()

    def key(line):
        m = SPEC_HEADING.match(line)
        return m.group(2) if m and m.group(2) in ids else None

    out = [SPEC_HEADER, "", f"From `{SPEC.as_posix()}`: {ids[0]} to {ids[-1]}." if len(ids) > 1 else
           f"From `{SPEC.as_posix()}`: {ids[0]}."]
    added = list(out)
    spans = heading_spans(spec, key)
    for a, b, _ in spans:
        mark = f"<!-- the specification's lines {a}-{b} -->"
        out += ["", mark, ""] + lines[a - 1:b]
        added.append(mark)
    missing = [x for x in ids if x not in {k for _, _, k in spans}]
    if missing:
        raise InputError(f"{SPEC}: no section {', '.join(missing)}")
    return "\n".join(out).rstrip() + "\n", "\n".join(added)


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


def readme_text(unit, files, cache, phase, redactions=()):
    """The README; `phase`: "model" (no combinations: the state models alone), "facts" (the combinations both state
    models draw: the facts alone) or "both"; `redactions`: (file, line, class) of each elided string."""
    measured = unit["kind"] == "measured"
    out = [f"# Verifier's brief: unit {unit['id']} ({unit['platform']}, {unit['kind']})", "",
           f"You verify the {unit['facts']} facts of this unit for the presets {', '.join(unit['presets'])}, "
           "independently of the facts lane that wrote them: you never see the lane's values or citations.", "",
           "## What you read", "",
           "This folder, and the pinned sources in the source cache "
           f"(`{cache}`, one checkout per `pins.toml` entry, at the pin). Nothing else: not the project, its gap "
           "maps or `platform-facts.md`, not the evidence repository's checkout.", ""]
    out += [f"- `{f}`: {what}" for f, what in files]
    model = ("`state-models.toml`: for each widget of the keys, the `[[model]]` entry derived from the sources (the "
             "catalogue's state-model source names where to look; the format is S17's *State models* in "
             "`spec.md`): the states, conditions, contexts, variants, sizes and densities the platform's drawing "
             "distinguishes, the exclusive groups, the combinations it does not draw, each with its citations.")
    facts = ("`facts.toml`: a `[[fact]]` for each key and combination, in the format of `facts-format.md` and "
             "`spec.md`: `key`, `when`, `scheme_file` and `accent` as `combinations.toml` writes them, `value`, "
             "`rows = []` and the citations. A combination the platform draws that the list lacks gets a fact too; one "
             "you cannot source gets none, and goes in your report with what you tried.")
    how = ("Each value with a `measured` citation: `evidence = "
           "\"docs/evidence/<harness>/<run>/<image>/<file>#<path>\"` (the copies under `docs/evidence/`, the run's "
           "`analysis/` scripts its method), and its `tolerance`; check the reading, the method and the conversion."
           if measured else
           "Each value with a `published` (or `read`) citation: `source = \"<id> <path>:<lines>\"` from `pins.toml` "
           "and the `quote` found there.")
    steps = {"model": [model + " This brief holds no combinations: they follow once your state models are compared "
                               "with the lane's."],
             "facts": [facts + " The combinations are those your state models and the lane's both draw.", "   " + how],
             "both": [model, facts, "   " + how]}[phase]
    out += ["", "## What you write", ""] + [f"{i}. {x}" if not x.startswith("   ") else x for i, x in
                                            enumerate(steps, 1)]
    out.append(f"{sum(1 for x in steps if not x.startswith('   ')) + 1}. Your report: the counts, the combinations you "
               f"could not source, and anything the catalogue cannot express.")
    if redactions:
        out += ["", "## Elided text", "",
                f"The copied normative text held {len(redactions)} of this unit's values or citations; each is "
                f"replaced by `{ELIDED.format('a value')}` or `{ELIDED.format('a citation')}`, the text around it "
                "kept as written:", ""]
        out += [f"- `{f}` line {n}: {cls}" for f, n, cls in redactions]
    return "\n".join(out) + "\n"


def redact(text, facts):
    """(the text with each platform value and citation of the facts (platform_value; not a trivial string, nor one
    their identities hold, which the verifier is given) replaced by an ELIDED marker, [(line, "value" or
    "citation")]): one pass, the longest string first where two start at one place, so the text around each is kept
    exactly."""
    ident = identity_text(facts)
    wanted = {n: w for n, w in needles(facts).items()
              if len(n) >= FLOOR and n not in KEYWORDS and platform_value(n, w) and n in text
              and not (n in ident and needle_rx(n).search(ident))}
    if not wanted:
        return text, []
    order = sorted(wanted, key=lambda n: (-len(n), n))
    rx = re.compile("|".join(f"(?:{needle_rx(n).pattern})" for n in order))
    found = []

    def mark(m):
        what = wanted.get(m.group(0))
        cls = "citation" if what is None or what.startswith("a citation's") else "value"
        found.append((text.count("\n", 0, m.start()) + 1, cls))
        return ELIDED.format(f"a {cls}")

    return rx.sub(mark, text), found


def brief(root, doc, unit_id, out, lane_brief, cache, evidence_root=None, combinations=True, agreed=None):
    """Write the unit's brief into `out` (a new or empty folder); return (the leak check's collisions, {file: the
    text it was written from}). `agreed`: a compare-model result, whose agreed combinations alone are written."""
    unit = find_unit(doc, unit_id)
    tree = Tree(root, doc["rev"])
    facts, _ = unit_facts(root, doc, unit)
    stated = facts if agreed is None else agreed_facts(agreed, doc, unit, facts)
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
                   "facts name them" + ("" if agreed is None else ", those both state models draw") + ".", ""] + header
    ident = identity_text(facts)
    write = {"keys.toml": ("\n".join(keys_head + ["", "keys = ["] + [f"  {toml_string(k)}," for k in keys] + ["]"])
                           + "\n", "\n".join(keys_head) + "\n" + ident)}
    files = [("keys.toml", "the unit and its keys")]
    if combinations:
        rows = list(combos_head)
        for f in stated:
            rows += ["", "[[combination]]"] + [f"{k} = {toml_inline(v)}" for k, v in plain(f.data).items()]
        write["combinations.toml"] = ("\n".join(rows) + "\n", "\n".join(combos_head) + "\n" + ident)
        files.append(("combinations.toml", "the combinations to state, one fact each"))
    pins, pins_added = pins_text(sources_data, unit["platform"], cache)
    write["pins.toml"] = (pins, sources_text + "\n" + pins_added)
    files.append(("pins.toml", "the pinned sources and their checkouts"))
    catalogue, cat_added = catalogue_text(cat, widgets)
    write["catalogue.md"] = (catalogue, cat + "\n" + cat_added)
    files.append(("catalogue.md", "the catalogue's shared definitions and the unit's widgets' entries"))
    write["facts-format.md"] = (format_text(lane_brief), lane_brief + "\n" + FORMAT_HEADER)
    files.append(("facts-format.md", "the facts format"))
    spec = tree.text(SPEC.as_posix())
    spec_out, spec_added = spec_text(spec)
    write["spec.md"] = (spec_out, spec + "\n" + spec_added)
    files.append(("spec.md", f"the specification's normative sections, {SPEC_SECTIONS[0]} to {SPEC_SECTIONS[-1]} "
                             f"(the model's blocks, roles, tokens, selectors, declarations, preset spelling and, in "
                             f"S17, the facts and state models)"))
    if unit["kind"] == "measured":
        if evidence_root is None:
            try:
                evidence_root = citations.evidence_root(sources_data, cache)
            except citations.EvidenceRootError as err:
                raise InputError(str(err)) from err
        n = copy_evidence(facts, Path(evidence_root), out)
        files.append((f"{EVIDENCE_DIR}/", f"the {n} evidence files the unit's facts cite, each run's run.toml and "
                                          f"analysis/ scripts"))
    redactions = []
    for name in REDACTED_FILES:
        text, provenance = write[name]
        text, found = redact(text, facts)
        write[name] = (text, provenance)
        redactions += [(name, n, cls) for n, cls in found]
    readme = readme_text(unit, files, cache, "model" if not combinations else "both" if agreed is None else "facts",
                         redactions)
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
                "facts-format.md": "specification", "spec.md": "specification", "pins.toml": "sources",
                "README.md": "template"}


def platform_value(s, what):
    """Whether a lane string is a platform value rather than the model's vocabulary: a citation's source, quote or
    evidence, or a value holding a literal (S2.2: a number, a colour, a length…) or a text that is no snake_case name
    (a font family, an icon name); a name (`circular`, `@card`) and an expression or reference without a literal
    (`= role.global.accent`) are vocabulary, as is an `absent` name."""
    if what.startswith("a citation's"):
        return what != "a citation's absent name"
    if s.lstrip().startswith("="):
        return bool(check_fact_citations.string_literals(s.lstrip()[1:]))
    return not check_facts.NAME.match(s) or bool(check_fact_citations.string_literals(s))


def leak_check(out, facts, inputs):
    """[(class, string, what, file, line)] of every string of the facts' values and citations found in the folder's
    files (the evidence copies left out): `trivial` (under FLOOR characters, or a keyword or bool), else, where the
    text the file was written from holds it too (`inputs`: {file: that text}), the class of the file (LEAK_CLASSES),
    but `normative-value` where that class is `specification` and the string a platform value (platform_value: a
    platform value in normative text, a leak to report; `identity` where the facts' identities hold it), else
    `leak`, a string that can only have come from the lane's facts."""
    found = []
    ident = identity_text(facts)
    texts = {}
    for p in sorted(Path(out).rglob("*")):
        rel = p.relative_to(out).as_posix()
        if p.is_file() and not rel.startswith(EVIDENCE_DIR + "/"):
            texts[rel] = p.read_text(encoding="utf-8")
    for s, what in sorted(needles(facts).items()):
        rx = needle_rx(s)
        for rel, text in texts.items():
            if s not in text:
                continue
            for m in rx.finditer(text):
                line = text.count("\n", 0, m.start()) + 1
                if len(s) < FLOOR or s in KEYWORDS:
                    cls = "trivial"
                elif rel in inputs and s in inputs[rel] and rx.search(inputs[rel]):
                    cls = LEAK_CLASSES.get(rel, "leak")
                    if cls == "specification" and platform_value(s, what):
                        cls = "identity" if s in ident and rx.search(ident) else "normative-value"
                else:
                    cls = "leak"
                found.append((cls, s, what, rel, line))
    return found


def leak_report(found):
    names = ("leak", "normative-value", "identity", "specification", "sources", "template", "trivial")
    counts = {c: sum(1 for f in found if f[0] == c) for c in names}
    lines = [f"# leak check: {counts['leak']} leaks, {counts['normative-value']} platform values in normative text "
             f"(the catalogue, the specification, the lane brief's format: leaks to report); unavoidable collisions: "
             f"{counts['identity']} in the identities, {counts['specification']} of the model's vocabulary in "
             f"normative text, {counts['sources']} in sources.toml's entries, {counts['template']} in the README's "
             f"own text, {counts['trivial']} trivial (under {FLOOR} characters, or a keyword or bool)"]
    groups = {}
    for cls, s, what, rel, line in found:
        if cls != "trivial":
            groups.setdefault((cls, rel, s, what), []).append(line)
    for (cls, rel, s, what), at in sorted(groups.items(), key=lambda g: (names.index(g[0][0]), g[0][1:3])):
        more = f" (+{len(at) - 5} more)" if len(at) > 5 else ""
        lines.append(f"{cls}\t{rel}:{','.join(map(str, at[:5]))}{more}\t{what}\t{s!r}")
    return counts, "\n".join(lines)


# ------------------------------------------------------------------------------------------- resolution

class Unresolved(Exception):
    """A value the resolver cannot evaluate (a reference no lane fact states, a platform function, an operand of
    the wrong kind); its message names why."""


EXPR_TOKEN = re.compile(r"\s*(?:(?P<colour>\b(?:srgb|display_p3)\([^()]*\))|(?P<hex>#[0-9a-zA-Z]+)"
                        r"|(?P<num>-?\d+(?:\.\d+)?(?:px|ms|deg|pt)?)(?![\w.])"
                        r"|(?P<name>@?[a-z_][a-z0-9_]*(?:\.@?[a-z0-9_][a-z0-9_]*)*)|(?P<p>[(),]))")
UNIT_SUFFIXES = {"_px": "px", "_ms": "ms", "_deg": "deg", "_pt": "pt"}
REF_HEADS = ("role", "token", "input", "part", "widget", "values")
NUM_EQ = 1e-4      # check_fact_citations.NUMBER_EQ: relative
CHANNEL_EQ = 0.5   # half an 8-bit step, on the 0-255 scale


def parse_expression(text):
    """An S2.2 expression's tree: ("lit", text), ("ref", path) or ("call", name, [args])."""
    toks, i = [], 0
    while i < len(text):
        if not text[i:].strip():
            break
        m = EXPR_TOKEN.match(text, i)
        if not m or m.end() == i:
            raise Unresolved(f"cannot read {text[i:]!r}")
        toks.append((m.lastgroup, m.group(m.lastgroup)))
        i = m.end()
    pos = 0

    def node():
        nonlocal pos
        if pos >= len(toks):
            raise Unresolved(f"{text!r} ends early")
        kind, t = toks[pos]
        pos += 1
        if kind == "name" and pos < len(toks) and toks[pos] == ("p", "("):
            pos += 1
            args = [node()]
            while pos < len(toks) and toks[pos] == ("p", ","):
                pos += 1
                args.append(node())
            if pos >= len(toks) or toks[pos] != ("p", ")"):
                raise Unresolved(f"{text!r}: a call is not closed")
            pos += 1
            return ("call", t, args)
        if kind == "name":
            return ("ref", t)
        if kind == "p":
            raise Unresolved(f"{text!r}: {t!r} out of place")
        return ("lit", t)

    tree = node()
    if pos != len(toks):
        raise Unresolved(f"{text!r}: more after the expression")
    return tree


def literal(t, unit=None):
    """A literal's resolved value: ("num", x, unit), ("colour", channels 0-255, space), ("keyword", …), ("bool", …)
    or ("name", …)."""
    if t in KEYWORDS - {"true", "false"}:
        return ("keyword", t)
    if t in ("true", "false"):
        return ("bool", t == "true")
    if check_facts.HEX_LITERAL.match(t) or check_facts.FUNC_COLOUR.match(t):
        lits = check_fact_citations.string_literals(t)
        if len(lits) == 1 and lits[0].kind == "colour":
            return ("colour", tuple(lits[0].value), lits[0].dim)
    m = check_facts.UNIT_LITERAL.match(t)
    if m:
        return ("num", float(m.group(1)), m.group(2))
    if re.fullmatch(r"-?\d+(?:\.\d+)?", t):
        return ("num", float(t), unit)
    return ("name", t)


def num(v, what):
    if v[0] != "num":
        raise Unresolved(f"{what}: not a number ({render(v)})")
    return v[1], v[2]


def colour(v, what):
    if v[0] != "colour":
        raise Unresolved(f"{what}: not a colour ({render(v)})")
    return v[1], v[2]


def arith(name, args):
    if name in ("floor", "round"):
        x, u = num(args[0], name)
        return ("num", float(math.floor(x) if name == "floor" else math.copysign(math.floor(abs(x) + 0.5), x)), u)
    (x, u), (y, w) = num(args[0], name), num(args[1], name)
    if name in ("add", "sub", "min", "max"):
        if u and w and u != w:
            raise Unresolved(f"{name}: {u} and {w}")
        r = {"add": x + y, "sub": x - y, "min": min(x, y), "max": max(x, y)}[name]
        return ("num", r, u or w)
    if name == "mul":
        if u and w:
            raise Unresolved(f"mul: {u} times {w}")
        return ("num", x * y, u or w)
    if y == 0:
        raise Unresolved("div: by zero")
    return ("num", x / y, None if u and u == w else u)


def call(name, args):
    if name in ("add", "sub", "mul", "div", "min", "max") and len(args) == 2 or name in ("floor", "round") \
            and len(args) == 1:
        return arith(name, args)
    if name in ("alpha", "with_alpha") and len(args) == 2:
        (c, space), (a, _) = colour(args[0], name), num(args[1], name)
        return ("colour", c[:3] + ((c[3] * a) if name == "alpha" else a * 255,), space)
    if name == "mix" and len(args) == 3:
        (a, sa), (b, sb), (t, _) = colour(args[0], name), colour(args[1], name), num(args[2], name)
        if sa != sb:
            raise Unresolved("mix: two colour spaces")
        alpha = a[3] * (1 - t) + b[3] * t
        rgb = tuple((x * a[3] * (1 - t) + y * b[3] * t) / alpha if alpha else 0.0 for x, y in zip(a[:3], b[:3]))
        return ("colour", rgb + (alpha,), sa)
    if name == "pick" and len(args) >= 3:
        i, _ = num(args[0], name)
        if not float(i).is_integer() or not 0 <= int(i) < len(args) - 1:
            raise Unresolved(f"pick: no colour at {i}")
        return args[1 + int(i)]
    raise Unresolved(f"the function {name}() is not evaluated here")


def render(v):
    if v[0] == "num":
        return num_text(round(v[1], 6)) + (v[2] or "")
    if v[0] == "colour":
        ch = v[1]
        if v[2] == "srgb" and all(abs(c - round(c)) < 1e-6 for c in ch):
            r, g, b, a = (round(c) for c in ch)
            return f"#{r:02x}{g:02x}{b:02x}" + ("" if a == 255 else f"{a:02x}")
        name = "srgb" if v[2] == "srgb" else "display_p3"
        return f"{name}({', '.join(num_text(round(c / 255, 6)) for c in (ch[:3] if ch[3] == 255 else ch))})"
    if v[0] in ("list", "table"):
        inner = ", ".join(render(x) if v[0] == "list" else f"{x[0]} = {render(x[1])}" for x in v[1])
        return f"[{inner}]" if v[0] == "list" else f"{{ {inner} }}"
    if v[0] == "bool":
        return "true" if v[1] else "false"
    return str(v[1])


def same(a, b):
    """Two resolved values equal: numbers within NUM_EQ (relative) and of one unit where both state one, colours of
    one space within CHANNEL_EQ per channel, lists and tables member by member."""
    if a[0] != b[0]:
        return False
    if a[0] == "num":
        return (not a[2] or not b[2] or a[2] == b[2]) and abs(a[1] - b[1]) <= NUM_EQ * max(1.0, abs(a[1]), abs(b[1]))
    if a[0] == "colour":
        return a[2] == b[2] and all(abs(x - y) <= CHANNEL_EQ for x, y in zip(a[1], b[1]))
    if a[0] == "list":
        return len(a[1]) == len(b[1]) and all(same(x, y) for x, y in zip(a[1], b[1]))
    if a[0] == "table":
        return [k for k, _ in a[1]] == [k for k, _ in b[1]] and all(same(x[1], y[1]) for x, y in zip(a[1], b[1]))
    return a[1] == b[1]


def has_expression(v):
    return any(isinstance(x, str) and x.lstrip().startswith("=") for x in value_strings(v))


class Resolver:
    """Evaluates values in one combination: the compared fact's `when` (the element's states, variants, sizes and
    context) under one world (its conditions, with a scheme and a preset where the `when` fixes none). A reference
    reads the lane's facts of the platform: `role.`, `token.` and `input.` by the facts whose conditions the world
    holds (of the compared fact's accent run where the referenced facts have runs), `part.`, `widget.<part>.` and
    `values.` by the facts of the compared fact's widget whose `when` the combination implies; the last such fact
    applies (S17), a whole block's fact read at its leaf. A fact naming a condition the world leaves open
    (condition_open: `decided` holds those the compared `when` names and the splits so far) raises Branch, where the
    rest of its `when` holds, so explore() splits the world on it. `surface.`, `element.`, `base`, `font_size`,
    `line_height`, `body_font_height`, `text_scale` and the platform functions are not evaluated."""

    def __init__(self, facts, fact, world, decided=frozenset()):
        self.by_key = {}
        for f in facts:
            if isinstance(f.identity[0], tuple):
                self.by_key.setdefault(f.identity[0], []).append(f)
        self.fact = fact
        self.when = fact.data.get("when") if isinstance(fact.data.get("when"), dict) else {}
        self.world = world
        self.decided = frozenset(decided)
        self.segs = fact.identity[0] if isinstance(fact.identity[0], tuple) else ()
        self.depth = 0

    def holds_world(self, f):
        w = f.data.get("when") if isinstance(f.data.get("when"), dict) else {}
        return conditions_hold(check_facts.when_lists(w, "conditions"), check_facts.when_lists(w, "not_conditions"),
                               self.world, self.decided)

    def implied(self, f):
        """The fact's `when` holds wherever the combination's does: each list a subset of the combination's (its
        conditions held by the world), its context, position and `latest` the combination's own."""
        w = f.data.get("when") if isinstance(f.data.get("when"), dict) else {}
        for k, v in w.items():
            if k in ("conditions", "not_conditions"):
                continue
            if check_facts.str_list(v):
                if not set(v) <= check_facts.when_lists(self.when, k):
                    return False
            elif check_facts.canonical_when(v) != check_facts.canonical_when(self.when.get(k)):
                return False
        return self.holds_world(f)

    def run_of(self, cands):
        mine = (self.fact.data.get("scheme_file"), self.fact.data.get("accent"))
        own = [f for f in cands if (f.data.get("scheme_file"), f.data.get("accent")) == mine]
        return own or [f for f in cands if "scheme_file" not in f.data and "accent" not in f.data]

    def lookup(self, segs, test, name):
        for k in range(len(segs), 0, -1):
            f = next((x for x in reversed(self.run_of(self.by_key.get(segs[:k], []))) if test(x)), None)
            if f is None:
                continue
            if "pending" in f.data:
                raise Unresolved(f"{name}: pending in the lane's facts ({f.data['pending']})")
            v = self.value(f.data.get("value"))
            for seg in segs[k:]:
                if v[0] != "table":
                    raise Unresolved(f"{name}: {seg!r} inside a {v[0]}")
                members = dict(v[1])
                hit = next((members[x] for x in [seg] + [seg + suf for suf in UNIT_SUFFIXES] if x in members), None)
                if hit is None:
                    raise Unresolved(f"{name}: no {seg!r} in the lane's fact")
                v = hit
            return v
        raise Unresolved(f"{name}: no lane fact applies")

    def ref(self, path):
        if path in ("true", "false"):
            return ("bool", path == "true")
        if path == "scheme":
            scheme = next((c[7:] for c in self.world if c.startswith("scheme:")), None)
            if scheme is None:
                raise Unresolved("scheme: no scheme in the world")
            return ("name", scheme)
        parsed = check_facts.key_segments(path)
        segs = tuple(parsed[0]) if isinstance(parsed, tuple) else ()
        head = segs[0] if segs else path
        if head not in REF_HEADS:
            if "." not in path:
                return ("name", path)
            raise Unresolved(f"{path}: not evaluated here")
        if head in ("role", "token", "input"):
            return self.lookup(segs, self.holds_world, path)
        widget = self.segs[:2] if self.segs[:1] == ("component",) else self.segs[:1]
        if head == "part":
            target = widget + self.segs[len(widget):len(widget) + 1] + segs[1:]
        elif head == "widget":
            target = widget + segs[1:]
        else:
            target = widget + ("values",) + segs[1:]
        return self.lookup(target, self.implied, path)

    def expr(self, tree, unit=None):
        self.depth += 1
        try:
            if self.depth > 64:
                raise Unresolved("a chain of references too long")
            if tree[0] == "lit":
                return literal(tree[1], unit)
            if tree[0] == "ref":
                return self.ref(tree[1])
            if tree[1] == "active" and len(tree[2]) == 1 and tree[2][0][0] == "ref":
                saved = self.world, self.decided
                self.world, self.decided = self.world - {"window_inactive"}, self.decided | {"window_inactive"}
                try:
                    return self.ref(tree[2][0][1])
                finally:
                    self.world, self.decided = saved
            return call(tree[1], [self.expr(a) for a in tree[2]])
        finally:
            self.depth -= 1

    def value(self, v, unit=None):
        if isinstance(v, bool):
            return ("bool", v)
        if isinstance(v, (int, float)):
            return ("num", float(v), unit)
        if isinstance(v, str):
            if v.lstrip().startswith("="):
                return self.expr(parse_expression(v.lstrip()[1:]), unit)
            return literal(v, unit)
        if isinstance(v, list):
            return ("list", tuple(self.value(x) for x in v))
        if isinstance(v, dict):
            return ("table", tuple(sorted((k, self.value(x, next((u for suf, u in UNIT_SUFFIXES.items()
                                                                  if k.endswith(suf)), None)))
                                          for k, x in v.items())))
        raise Unresolved(f"a value of type {type(v).__name__}")


def worlds(fact, platform):
    """The worlds a fact's combination holds in: its `when`'s conditions, with each scheme and each preset of the
    platform where the `when` fixes none (and does not refuse it). explore() opens the other conditions."""
    w = fact.data.get("when") if isinstance(fact.data.get("when"), dict) else {}
    base = check_facts.when_lists(w, "conditions")
    nots = check_facts.when_lists(w, "not_conditions")
    axes = [[f"scheme:{x}" for x in ("light", "dark")], [f"preset:{x}" for x in PLATFORMS.get(platform, ())]]
    out = [frozenset(base)]
    for axis in axes:
        values = [c for c in axis if c not in nots]
        if values and not any(c in base for c in axis):
            out = [x | {c} for x in out for c in values]
    return out


def world_label(world, platform):
    """A world as the tie-breaker reads it: the conditions it holds, a platform's one preset left out."""
    only = PLATFORMS.get(platform, ())
    return ", ".join(sorted(c for c in world if len(only) > 1 or c != f"preset:{only[0]}")) or "no condition"


class Branch(Exception):
    """A condition an evaluation reads that its world leaves open (condition_open): explore() splits the world on
    it."""

    def __init__(self, condition):
        super().__init__(condition)
        self.condition = condition


AXES = ("scheme", "preset")   # each world fixes one of each (worlds())


class Closed:
    """The `decided` of a world that opens nothing: every condition it does not hold is refused."""

    def __contains__(self, c):
        return True


CLOSED = Closed()


def condition_open(c, world, decided):
    """Whether a world leaves a condition open: not decided (named by the compared `when`, or split on before), no
    scheme or preset (the world fixes them), and not refused by another value of a one-valued condition the world
    holds (check_facts.ONE_VALUED, one container's `container_size:`, CONTRAST; S7.1, S12)."""
    if c in decided or c in world:
        return False
    head, _, arg = c.partition(":")
    if head in AXES:
        return False
    if head in check_facts.ONE_VALUED:
        return not any(x.partition(":")[0] == head for x in world)
    if head == "container_size":
        return not any(x.startswith(f"container_size:{arg.partition(':')[0]}:") for x in world)
    if c in check_facts.CONTRAST:
        return not any(x in check_facts.CONTRAST for x in world)
    return True


def conditions_hold(conds, nots, world, decided):
    """Whether a `when`'s conditions hold in a world: False where a condition the world decides refuses it, else
    Branch on the first it leaves open, else True."""
    if any(c not in world and not condition_open(c, world, decided) for c in conds) or (nots & world):
        return False
    opened = sorted(c for c in conds | nots if condition_open(c, world, decided))
    if opened:
        raise Branch(opened[0])
    return True


def explore(fact, platform, evaluate):
    """[(world, result)]: `evaluate(world, decided)` under each world the fact's combination holds in (worlds()), a
    world split, the condition held and not, on each condition an evaluation reads that the world leaves open
    (Branch), so every condition the result depends on is opened, as the schemes are."""
    w = fact.data.get("when") if isinstance(fact.data.get("when"), dict) else {}
    named = frozenset(check_facts.when_lists(w, "conditions") | check_facts.when_lists(w, "not_conditions"))
    stack = [(world, named) for world in reversed(worlds(fact, platform))]
    out = []
    while stack:
        world, decided = stack.pop()
        try:
            out.append((world, evaluate(world, decided)))
        except Branch as b:
            stack.append((world, decided | {b.condition}))
            stack.append((world | {b.condition}, decided | {b.condition}))
    return out


# ----------------------------------------------------------------------------------------- effective values

class Facts:
    """One side's facts as the effective values read them (S17 *Which fact applies*): per key and run (`scheme_file`
    and `accent`), in file order; each fact's `when` for a preset (generate_preset.preset_when), None where it does
    not hold for the preset (generate_preset.holds_for)."""

    def __init__(self, facts):
        self.by = {}
        for f in facts:
            self.by.setdefault((f.identity[0], f.identity[2], f.identity[3]), []).append(f)
        self.whens = {}

    def of(self, ident):
        return self.by.get((ident[0], ident[2], ident[3]), [])

    def when_for(self, f, preset):
        k = (id(f), preset)
        if k not in self.whens:
            self.whens[k] = (generate_preset.preset_when(f.data.get("when"), preset)
                             if generate_preset.holds_for(f, preset) else None)
        return self.whens[k]


def preset_models(items, preset):
    """{(widget, flavour or None): the state-model entry in force for the preset}, as the preset generator picks them
    (generate_preset.declarations: the last entry naming the preset, else the last shared one, and every flavour's)."""
    entries = {}
    for it in items:
        d = it.data
        if not isinstance(d.get("widget"), str):
            continue
        conds = sorted(check_facts.when_lists(d.get("when"), "conditions"))
        named = [c[7:] for c in conds if c.startswith("preset:")]
        if named and named != [preset]:
            continue
        flavor = next((c[7:] for c in conds if c.startswith("flavor:")), None)
        entries.setdefault(d["widget"], []).append((d, bool(named), flavor))
    models = {}
    for widget, es in entries.items():
        plain = [e for e in es if e[2] is None]
        chosen = [e for e in plain if e[1]] or plain
        for d, _, flavor in chosen[-1:] + [e for e in es if e[2] is not None]:
            models[(widget, flavor)] = d
    return models


def effective(facts, ident, combo, world, decided, preset, groups, recency):
    """The fact of `facts` that applies in a combination (S17 *Which fact applies*: the last in file order of the
    identity's key and run whose `when` holds, after the exclusive groups' drop; generate_preset.holds, the round
    trip's matcher), or None; Branch on a condition the world leaves open where the rest of a `when` holds."""
    states = generate_preset.reduced(combo["states"], groups)
    for f in reversed(facts.of(ident)):
        w = facts.when_for(f, preset)
        if w is None:
            continue
        conds, nots = check_facts.when_lists(w, "conditions"), check_facts.when_lists(w, "not_conditions")
        opened = sorted(c for c in conds | nots if condition_open(c, world, decided))
        if opened:
            rest = {**w, "conditions": sorted(conds - set(opened)), "not_conditions": sorted(nots - set(opened))}
            if generate_preset.holds(rest, combo, groups, recency, states):
                raise Branch(opened[0])
            continue
        if generate_preset.holds(w, combo, groups, recency, states):
            return f
    return None


@dataclass
class Sides:
    """What the comparison of a pair reads: both sides' facts, the lane's facts of the platform (the references'),
    the lane's state models per preset (the combination's element) and the catalogue's entries (its widget)."""
    lane: Facts
    verifier: Facts
    lane_all: list
    platform: str
    models: dict
    entries: dict
    types: object


def combination(sides, lf, world):
    """(the compared `when`'s minimal combination in a world, its exclusive groups, its recency sets), or None where
    no element holds it: generate_preset.minimal (its names and nothing else, the size an element holds by the
    lane's state model), its conditions the world's."""
    preset = next((c[7:] for c in world if c.startswith("preset:")), None)
    scheme = next((c[7:] for c in world if c.startswith("scheme:")), None)
    if preset is None or scheme is None:
        return None
    w = generate_preset.preset_when(lf.data.get("when"), preset)
    widget = widget_of(lf.identity[0] if isinstance(lf.identity[0], tuple) else (), sides.entries)
    model = generate_preset.model_of(sides.models.get(preset, {}), widget, w)
    groups = [g for g in model.get("exclusive", []) if isinstance(g, list)] if model else []
    recency = check_facts.recency_of(model) if model else {}
    combo = generate_preset.minimal(w, scheme, groups, recency, model)
    if combo is None:
        return None
    combo["conditions"] |= {c for c in world if c.partition(":")[0] not in AXES}
    return combo, groups, recency, preset


def value_text(f):
    return f"pending {f.data['pending']}" if "pending" in f.data else toml_inline(f.data.get("value"))


def compare_pair(sides, lf, vf, used):
    """(class, notes, [(the lane's fact, the verifier's) that apply in each world]) of a compared pair: in every world
    the lane fact's combination holds in (explore(): the schemes, the presets and each condition the facts or the
    references read), the fact of each side that applies there (effective(); the pair's own where none does or no
    element holds the combination), their values compared after the normalisations (`used` collects those that
    changed a spelling), else, where either holds an expression or a reference, resolved (Resolver). `agreed` where
    they are equal in every world; `equivalent` where each world is equal or resolves to one value, one at least by
    resolution; `disagreed` otherwise. A world where a condition explore() opened hands either side's combination to
    another fact than in the world without the opened conditions is that fact's combination, compared under its own
    identity where both sides state it (or one-sided), and not here. The notes give each world's values and the fact
    that applies where it is not the pair's own; none for a plain disagreement (no resolution, the pair's own facts in
    every world)."""
    on = tuple(NORMALISATIONS)
    when = lf.data.get("when") if isinstance(lf.data.get("when"), dict) else {}
    named = check_facts.when_lists(when, "conditions")

    def applying(world, decided):
        got = combination(sides, lf, world)
        if got is None:
            return lf, vf
        combo, groups, recency, preset = got
        return (effective(sides.lane, lf.identity, combo, world, decided, preset, groups, recency) or lf,
                effective(sides.verifier, vf.identity, combo, world, decided, preset, groups, recency) or vf)

    def evaluate(world, decided):
        lw, vw = applying(world, decided)
        base = frozenset(c for c in world if c in named or c.partition(":")[0] in AXES)
        if base != world:
            lw0, vw0 = applying(base, CLOSED)
            if lw is not lw0 or vw is not vw0:
                return "elsewhere", lw, vw, None
        if fact_value(lw.data, on, used, sides.types) == fact_value(vw.data, on, used, sides.types):
            return "equal", lw, vw, None
        if "pending" in lw.data or "pending" in vw.data or not (
                has_expression(lw.data.get("value")) or has_expression(vw.data.get("value"))):
            return "differ", lw, vw, None
        r = Resolver(sides.lane_all, lf, set(world), decided)
        try:
            a = r.value(stated_value(lw.data, on, set(), sides.types))
            b = r.value(stated_value(vw.data, on, set(), sides.types))
        except Unresolved as err:
            return "unresolved", lw, vw, f"unresolved: {err}"
        if same(a, b):
            return "same", lw, vw, f"both {render(a)}"
        return "differ", lw, vw, f"lane {render(a)}, verifier {render(b)}"

    results = explore(lf, sides.platform, evaluate)
    kinds = {r[0] for _, r in results} - {"elsewhere"}
    cls = "agreed" if kinds == {"equal"} else "equivalent" if kinds <= {"equal", "same"} else "disagreed"
    pairs = [(lw, vw) for _, (kind, lw, vw, _) in results if kind != "elsewhere"]
    if cls != "equivalent" and all(lw is lf and vw is vf and note is None for _, (_, lw, vw, note) in results):
        return cls, [], pairs
    notes = []
    for world, (kind, lw, vw, note) in results:
        text = note or ("not compared, another fact's combination" if kind == "elsewhere" else
                        f"both {value_text(lw)}" if kind == "equal" else
                        f"lane {value_text(lw)}, verifier {value_text(vw)}")
        applies = [f"the {who}'s {g.file}:{g.line} applies" for who, g, own in (("lane", lw, lf), ("verifier", vw, vf))
                   if g is not own]
        notes.append(f"{world_label(world, sides.platform)}: {text}" + (f" ({'; '.join(applies)})" if applies else ""))
    return cls, notes, pairs


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


def compare(root, doc, unit_id, verifier_path, out=None, agreed_models=None, types=None):
    """Write tiebreak.toml and agreed.toml into `out`; return the summary table. `agreed_models`: a compare-model
    result, whose agreed combinations alone are compared (the others were the model comparison's). `types`: the
    core's types the normalisations read (type_context(), at the units' revision by default)."""
    unit = find_unit(doc, unit_id)
    types = type_context(root, doc["rev"]) if types is None else types
    lane_unit, lane_all = unit_facts(root, doc, unit)
    refused = 0
    if agreed_models is not None:
        kept = agreed_facts(agreed_models, doc, unit, lane_unit)
        refused, lane_unit = len(lane_unit) - len(kept), kept
    vpath = Path(verifier_path)
    try:
        vtext = vpath.read_text(encoding="utf-8")
    except (OSError, UnicodeDecodeError) as err:
        raise InputError(f"{vpath}: {err}") from err
    vfacts = read_facts(vpath.as_posix(), unit["platform"], vtext)
    tree = Tree(root, doc["rev"])
    items = platform_models(tree, unit["platform"])
    sides = Sides(Facts(lane_all), Facts(vfacts), lane_all, unit["platform"],
                  {p: preset_models(items, p) for p in PLATFORMS.get(unit["platform"], ())},
                  {e.name: e for e in spec_catalogue.entries(tree.text(CATALOGUE.as_posix()))}, types)
    lane = {f.identity: f for f in lane_unit}
    lane_any = last_by(lane_all, lambda f: f.identity)
    ver = last_by(vfacts, lambda f: f.identity)
    all_on = tuple(NORMALISATIONS)
    stats = {n: {"changed": 0, "decisive": 0} for n in NORMALISATIONS}
    agreed, equivalent, disagreed, lane_only, verifier_only, outside = [], [], [], [], [], []
    before = outside_agreed = 0
    for ident, lf in lane.items():
        vf = ver.get(ident)
        if vf is None:
            lane_only.append(lf)
            continue
        used = set()
        identity_of(lf.data, all_on, used)
        identity_of(vf.data, all_on, used)
        cls, notes, pairs = compare_pair(sides, lf, vf, used)
        for n in used:
            stats[n]["changed"] += 1
        if all(fact_value(a.data, (), set()) == fact_value(b.data, (), set()) for a, b in pairs):
            before += 1
        if cls == "agreed":
            agreed.append((lf, vf))
            for n in NORMALISATIONS:
                rest = tuple(x for x in all_on if x != n)
                if n in IDENTITY_NORMALISATIONS:
                    if identity_of(lf.data, rest) != identity_of(vf.data, rest):
                        stats[n]["decisive"] += 1
                elif any(fact_value(a.data, rest, set(), types) != fact_value(b.data, rest, set(), types)
                         for a, b in pairs):
                    stats[n]["decisive"] += 1
        else:
            (equivalent if cls == "equivalent" else disagreed).append((lf, vf, notes))
    for ident, vf in ver.items():
        if ident in lane:
            continue
        if ident in lane_any:
            lf = lane_any[ident]
            if fact_value(lf.data, all_on, set(), types) == fact_value(vf.data, all_on, set(), types):
                outside_agreed += 1
            else:
                outside.append((lf, vf))
        else:
            verifier_only.append(vf)
    compared = len(agreed) + len(equivalent) + len(disagreed)
    summary = {
        "unit": unit["id"], "platform": unit["platform"], "kind": unit["kind"], "rev": doc["rev"],
        "verifier_file": vpath.as_posix(), "lane_facts": len(lane), "verifier_facts": len(ver),
        "verifier_repeated": len(vfacts) - len(ver), "compared": compared, "agreed": len(agreed),
        "agreed_before_normalising": before, "equivalent_by_resolution": len(equivalent),
        "disagreed": len(disagreed), "lane_only": len(lane_only),
        "verifier_only": len(verifier_only), "outside_unit": len(outside) + outside_agreed,
        "outside_unit_disagreed": len(outside), "refused_by_model": refused,
    }
    summary["ledger"] = ledger_line(summary)
    out = Path(out) if out else vpath.parent
    out.mkdir(parents=True, exist_ok=True)
    t = ["# The comparison of a lane's facts with its verifier's (scripts/verify_facts.py compare): the tie-breaker's "
         "input.", "", "[summary]"] + [f"{k} = {toml_inline(v)}" for k, v in summary.items()]
    for n, what in NORMALISATIONS.items():
        t += ["", "[[normalisation]]", f"name = {toml_string(n)}", f"what = {toml_string(what)}",
              f"changed = {stats[n]['changed']}", f"decisive = {stats[n]['decisive']}"]
    for table, pairs in (("equivalent", equivalent), ("disagreed", disagreed),
                         ("outside_unit", [(lf, vf, []) for lf, vf in outside])):
        for lf, vf, notes in pairs:
            t += ["", f"[[{table}]]"] + [f"{k} = {toml_inline(v)}" for k, v in plain(lf.data).items()]
            t += [f"lane = {toml_inline(side(lf))}", f"verifier = {toml_inline(side(vf))}"]
            if notes:
                t.append(f"resolved = {toml_inline(notes)}")
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
    eq = s.get("equivalent_by_resolution", 0)
    return (f"| {s['unit']} | {s['platform']} | lane {s['lane_facts']:,}, verifier {s['verifier_facts']:,} | "
            f"{s['agreed']:,} of {c:,} compared combinations after normalising spellings ({pct})"
            + (f"; {eq:,} more equivalent by resolution" if eq else "") + " | to tie-break: "
            f"{s['disagreed']:,} disagreed, {s['lane_only']:,} lane-only, {s['verifier_only']:,} verifier-only"
            + (f", {s['refused_by_model']:,} refused by the verifier's state model"
               if s.get("refused_by_model") else "")
            + f" | {sample_size(s['agreed'], 0.05, 10)} | |")


# ------------------------------------------------------------------------------------- the state models

MODEL_SKIP = {"widget", "when", "rows", "cite"}
ORDERED_FIELDS = {"exclusive", "recency"}   # an exclusive group's order is its precedence, a recency set's its order


def read_models(file, platform, text):
    """The `[[model]]` entries of a state-models file's text, as check_facts.items_of reads them."""
    try:
        data = tomllib.loads(text)
    except tomllib.TOMLDecodeError as err:
        raise InputError(f"{file}: not valid TOML: {err}") from err
    return check_facts.items_of(file, platform, text, data, "model")


def platform_models(tree, platform):
    path = f"{FACTS.as_posix()}/{platform}/{STATE_MODELS}"
    return read_models(path, platform, tree.text(path)) if path in tree.files(path) else []


def fact_widgets(tree, facts):
    """[the catalogue entry each fact's key names, or None]."""
    entries = {e.name: e for e in spec_catalogue.entries(tree.text(CATALOGUE.as_posix()))}
    return [widget_of(f.identity[0] if isinstance(f.identity[0], tuple) else (), entries) for f in facts]


def entry_id(it):
    when = it.data.get("when")
    return it.data.get("widget"), check_facts.canonical_when(strip_empty(when) if isinstance(when, dict) else when)


def canon(field, x):
    """A declaration's item as a comparable value: name lists as sets (in ORDERED_FIELDS, in order), tables without
    their empty members."""
    if isinstance(x, list):
        if check_facts.str_list(x):
            return ("list",) + (tuple(x) if field in ORDERED_FIELDS else tuple(sorted(set(x))))
        return ("list",) + tuple(canon(field, v) for v in x)
    if isinstance(x, dict):
        return ("table",) + tuple(sorted((k, canon(field, v)) for k, v in x.items() if v not in ([], {}, None)))
    return x


def declarations(d):
    """{field: {comparable item: the item as written}} of an entry: each list field's members, each table field's
    entries (`recency`), a scalar as its one item; `axes` by its three lists."""
    out = {}
    for k, v in d.items():
        if k in MODEL_SKIP:
            continue
        if k == "axes" and isinstance(v, dict):
            for a, x in v.items():
                out[f"axes.{a}"] = {canon(a, i): i for i in x} if isinstance(x, list) else {canon(a, x): x}
        elif isinstance(v, list):
            out[k] = {canon(k, i): i for i in v}
        elif isinstance(v, dict):
            out[k] = {(n, canon(k, i)): {n: i} for n, i in v.items()}
        else:
            out[k] = {canon(k, v): v}
    return out


def entry_combinations(d):
    """(the state combinations, the variant sets) of an available entry, as check_facts.py expands them."""
    if d.get("available") is not True:
        return set(), set()
    listed, products = check_facts.drawn_set_lists(d)
    return check_facts.state_combinations(d), listed | products


def covers(nd, when, states):
    """A `not_drawn` entry stating only name lists that the `when` implies (each list a subset of the `when`'s): every
    combination the `when` holds in is one it removes."""
    if not isinstance(nd, dict) or not nd or nd.keys() - {"states", "not_states", "variants", "not_variants",
                                                          "conditions", "not_conditions", "sizes", "densities"}:
        return False
    for k, v in nd.items():
        have = states if k == "states" else check_facts.when_lists(when, k)
        if not check_facts.str_list(v) or not set(v) <= have:
            return False
    return True


def entry_refusal(when, e):
    """Why one entry draws no combination the `when` holds in ([] where it may draw one)."""
    d = e.data
    if d.get("available") is not True:
        return ["the entry declares the widget not available"]
    lists = check_facts.when_lists
    out = list(check_facts.model_never(when, e))
    axes = d.get("axes") if isinstance(d.get("axes"), dict) else {}
    latest = when.get("latest") if isinstance(when.get("latest"), dict) else {}
    latest = {v for v in latest.values() if isinstance(v, str)}
    states = lists(when, "states") | latest
    on_axes = set(axes.get("states", [])) if check_facts.str_list(axes.get("states", [])) else set()
    flags = set(d.get("flags", [])) if check_facts.str_list(d.get("flags", [])) else set()
    combined = states & on_axes
    if states - on_axes - flags:
        out.append(f"states {sorted(states - on_axes - flags)} are neither in axes.states nor in flags")
    elif not any(combined <= k and not lists(when, "not_states") & k for k in check_facts.state_combinations(d)):
        out.append(f"no state combination holds {sorted(combined)} without {sorted(lists(when, 'not_states'))}")
    variants = lists(when, "variants")
    known = set(d.get("variants", [])) if check_facts.str_list(d.get("variants", [])) else set()
    if variants - known:
        out.append(f"variants {sorted(variants - known)} are not in variants")
    elif len(variants) > 1 and not any(variants <= x and not lists(when, "not_variants") & x
                                       for x in entry_combinations(d)[1]):
        out.append(f"no variant set holds {sorted(variants)}")
    for k in ("sizes", "densities"):
        have = set(d.get(k, [])) if check_facts.str_list(d.get(k, [])) else set()
        if k == "sizes":
            for by in ("sizes_by_variant", "sizes_by_context"):
                for e in d.get(by, []) if isinstance(d.get(by), list) else []:
                    if isinstance(e, dict) and check_facts.str_list(e.get("sizes", [])):
                        have |= set(e.get("sizes", []))
        if lists(when, k) - have:
            out.append(f"{k} {sorted(lists(when, k) - have)} are not in {k}")
    conds = {c for c in lists(when, "conditions") if not c.startswith(("preset:", "flavor:"))}
    have = set(axes.get("conditions", [])) if check_facts.str_list(axes.get("conditions", [])) else set()
    if conds - have:
        out.append(f"conditions {sorted(conds - have)} are not in axes.conditions")
    chains = check_facts.axes_chains(d)
    for sel in when.get("context", []) if isinstance(when.get("context"), list) else []:
        if not any(check_facts.selector_matches(sel, c) is not False for c in chains):
            out.append(f"no axes.contexts chain matches the context {toml_inline(sel)}")
    for nd in d.get("not_drawn", []) if isinstance(d.get("not_drawn"), list) else []:
        if covers(nd, when, states):
            out.append(f"not_drawn {toml_inline(nd)} removes it")
    return out


def model_refusal(when, items):
    """Why the widget's entries draw no combination the `when` holds in: none applies (check_facts.applicable_entries),
    or each that applies refuses it; [] where one may draw it."""
    when = when if isinstance(when, dict) else {}
    apps = check_facts.applicable_entries(items, when)
    if not apps:
        return ["no state-model entry of the widget applies"]
    reasons = []
    for e in apps:
        r = entry_refusal(when, e)
        if not r:
            return []
        reasons += r
    return reasons


def flag_lists(sets):
    return sorted(sorted(x) for x in sets)


def compare_model(root, doc, unit_id, verifier_path, out=None):
    """Write model-compare.toml into `out`; return its summary."""
    unit = find_unit(doc, unit_id)
    tree = Tree(root, doc["rev"])
    facts, _ = unit_facts(root, doc, unit)
    widgets_of = fact_widgets(tree, facts)
    widgets = {w for w in widgets_of if w}
    vpath = Path(verifier_path)
    try:
        vtext = vpath.read_text(encoding="utf-8")
    except (OSError, UnicodeDecodeError) as err:
        raise InputError(f"{vpath}: {err}") from err
    lane = [it for it in platform_models(tree, unit["platform"]) if it.data.get("widget") in widgets]
    ver_all = read_models(vpath.as_posix(), unit["platform"], vtext)
    ver = [it for it in ver_all if it.data.get("widget") in widgets]
    lane_by, ver_by = last_by(lane, entry_id), last_by(ver, entry_id)
    t, counts = [], {"declarations": 0, "states_lane": 0, "states_verifier": 0, "sets_lane": 0, "sets_verifier": 0}
    for eid in sorted(lane_by.keys() | ver_by.keys(), key=repr):
        l, v = lane_by.get(eid), ver_by.get(eid)
        it = l or v
        head = ["", "[[entry]]", f"widget = {toml_string(eid[0])}"]
        if "when" in it.data:
            head.append(f"when = {toml_inline(it.data['when'])}")
        if not (l and v):
            t += head + [f"only = {toml_string('lane' if l else 'verifier')}", f"line = {it.line}"]
            continue
        t += head + [f"lane_line = {l.line}", f"verifier_line = {v.line}"]
        a, b = declarations(l.data), declarations(v.data)
        diffs = []
        for field in sorted(a.keys() | b.keys()):
            x, y = a.get(field, {}), b.get(field, {})
            lo, vo = [x[k] for k in x if k not in y], [y[k] for k in y if k not in x]
            if lo or vo:
                counts["declarations"] += 1
                diffs += ["", "[[entry.difference]]", f"field = {toml_string(field)}", f"lane_only = {toml_inline(lo)}",
                          f"verifier_only = {toml_inline(vo)}"]
        (ls, lv), (vs, vv) = entry_combinations(l.data), entry_combinations(v.data)
        counts["states_lane"] += len(ls - vs)
        counts["states_verifier"] += len(vs - ls)
        counts["sets_lane"] += len(lv - vv)
        counts["sets_verifier"] += len(vv - lv)
        t += [f"state_combinations = {{ lane = {len(ls)}, verifier = {len(vs)} }}",
              f"state_combinations_lane_only = {toml_inline(flag_lists(ls - vs))}",
              f"state_combinations_verifier_only = {toml_inline(flag_lists(vs - ls))}",
              f"variant_sets_lane_only = {toml_inline(flag_lists(lv - vv))}",
              f"variant_sets_verifier_only = {toml_inline(flag_lists(vv - lv))}"] + diffs
    agreed, refused = [], []
    for f, w in zip(facts, widgets_of):
        when = f.data.get("when")
        if w is None:
            agreed.append((f, [], []))
            continue
        r = model_refusal(when, [it for it in ver if it.data.get("widget") == w])
        mine = model_refusal(when, [it for it in lane if it.data.get("widget") == w])
        if r and not mine:
            refused.append((f, r))
        else:
            agreed.append((f, mine, r))
    summary = {
        "unit": unit["id"], "platform": unit["platform"], "rev": doc["rev"], "verifier_file": vpath.as_posix(),
        "widgets": sorted(widgets), "lane_entries": len(lane_by), "verifier_entries": len(ver_by),
        "verifier_entries_of_other_widgets": len(ver_all) - len(ver),
        "matched_entries": len(lane_by.keys() & ver_by.keys()),
        "lane_only_entries": len(lane_by.keys() - ver_by.keys()),
        "verifier_only_entries": len(ver_by.keys() - lane_by.keys()),
        "declaration_differences": counts["declarations"],
        "state_combinations_lane_only": counts["states_lane"],
        "state_combinations_verifier_only": counts["states_verifier"],
        "variant_sets_lane_only": counts["sets_lane"], "variant_sets_verifier_only": counts["sets_verifier"],
        "combinations": len(facts), "agreed_combinations": len(agreed), "refused_combinations": len(refused),
        "lane_model_refuses": sum(1 for _, mine, _ in agreed if mine),
    }
    rows = ["# The comparison of a lane's state models with its verifier's (scripts/verify_facts.py compare-model): "
            "the tie-breaker's input, and the combinations the second brief states (brief --combinations-from).",
            "", "[summary]"] + [f"{k} = {toml_inline(v)}" for k, v in summary.items()] + t
    for f, mine, theirs in agreed:
        rows += ["", "[[combination]]", f"file = {toml_string(f.file)}"]
        rows += [f"{k} = {toml_inline(v)}" for k, v in plain(f.data).items()]
        if mine:
            rows += [f"lane_model_refuses = {toml_inline(mine)}", f"verifier_model_refuses = {toml_inline(theirs)}"]
    for f, why in refused:
        rows += ["", "[[refused]]", f"file = {toml_string(f.file)}"]
        rows += [f"{k} = {toml_inline(v)}" for k, v in plain(f.data).items()]
        rows.append(f"reasons = {toml_inline(why)}")
    out = Path(out) if out else vpath.parent
    out.mkdir(parents=True, exist_ok=True)
    (out / "model-compare.toml").write_text("\n".join(rows) + "\n", encoding="utf-8")
    return summary


def read_toml(path):
    try:
        return tomllib.loads(Path(path).read_text(encoding="utf-8"))
    except (OSError, UnicodeDecodeError, tomllib.TOMLDecodeError) as err:
        raise InputError(f"{path}: {err}") from err


def agreed_facts(result, doc, unit, facts):
    """The unit's facts whose combinations a compare-model result (its table) agrees on."""
    head = result.get("summary", {})
    if head.get("unit") != unit["id"] or head.get("rev") != doc["rev"]:
        raise InputError(f"the compare-model result is of unit {head.get('unit')!r} at {head.get('rev')!r}, not "
                         f"{unit['id']!r} at {doc['rev']!r}")
    wanted = {(c.get("file"), identity_of(c)) for c in result.get("combination", [])}
    got = [f for f in facts if (f.file, f.identity) in wanted]
    if len(got) != len(wanted):
        raise InputError(f"the compare-model result names {len(wanted) - len(got)} combinations unit {unit['id']} "
                         f"does not hold")
    return got


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
    b.add_argument("--combinations-from")
    m = sub.add_parser("compare-model")
    m.add_argument("unit")
    m.add_argument("verifier_models")
    m.add_argument("--units")
    m.add_argument("--out")
    c = sub.add_parser("compare")
    c.add_argument("unit")
    c.add_argument("verifier_facts")
    c.add_argument("--units")
    c.add_argument("--out")
    c.add_argument("--combinations-from")
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
            if args.no_combinations and args.combinations_from:
                raise InputError("--no-combinations and --combinations-from exclude each other")
            agreed = read_toml(args.combinations_from) if args.combinations_from else None
            found, _ = brief(root, read_units(root, args.units), args.unit, args.out_dir, lane_brief,
                             citations.cache_dir(args.cache), combinations=not args.no_combinations, agreed=agreed)
            counts, report = leak_report(found)
            print(report)
            return 1 if counts["leak"] or counts["normative-value"] else 0
        if args.cmd == "compare-model":
            summary = compare_model(root, read_units(root, args.units), args.unit, args.verifier_models, args.out)
            print(f"# {summary['unit']}: {summary['matched_entries']} entries matched, {summary['lane_only_entries']} "
                  f"lane-only, {summary['verifier_only_entries']} verifier-only; {summary['declaration_differences']} "
                  f"declarations differ; state combinations {summary['state_combinations_lane_only']} lane-only, "
                  f"{summary['state_combinations_verifier_only']} verifier-only; variant sets "
                  f"{summary['variant_sets_lane_only']} lane-only, {summary['variant_sets_verifier_only']} "
                  f"verifier-only; combinations {summary['agreed_combinations']} agreed, "
                  f"{summary['refused_combinations']} refused by the verifier's model")
            return 0
        if args.cmd == "compare":
            agreed = read_toml(args.combinations_from) if args.combinations_from else None
            summary = compare(root, read_units(root, args.units), args.unit, args.verifier_facts, args.out, agreed)
            print(f"# {summary['unit']}: agreed {summary['agreed']}, equivalent by resolution "
                  f"{summary['equivalent_by_resolution']}, disagreed {summary['disagreed']}, lane-only "
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

## What this is

Not normative here.

## Entry format

- **Parts:** each part's fields.

## Shared value types

| `ArrowStyle` | none, chevron |

## Shared flags

`drop_target`.

## Selector notation

`{checked}`.

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
- **Notes:** a note naming the card.

### link

- **Kind:** typed widget, not the unit's.

## Tokens, conditions, readers, placement

Copied: the tokens' definitions.
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

SELF_TEST_SPEC = '''# spec

## 0. Progress

Not normative.

## S1. Section 1

## S2. Blocks

### S2.2 Values and expressions

Literals.

```toml
## S99. inside a code block
```

## S3. Section 3

## S4. Section 4

## S5. Section 5

## S6. Section 6

## S7. Section 7

## S8. Section 8

## S9. Section 9

## S10. Declarations

## S11. Preset files (TOML)

## S12. Section 12

## S13. Section 13

## S14. Section 14

## S15. Section 15

## S16. Section 16

## S17. Facts as data and the gate (R4, R22)

### State models

`[[model]]`.

## S18. Section 18

## S19. Section 19

## S20. Section 20

## S21. Section 21

## S22. Additions register

ADD-A1: a record.
'''

SELF_TEST_SKETCH = '''
```rust
pub struct SketchPaint { pub layers: Vec<SketchLayer>, pub kind: SketchKind, pub inset: Option<Value<Px>> }
pub struct SketchLayer { pub kind: SketchKind }
pub enum SketchKind { Plain, Linear { angle: Deg, offset: Option<Value<Px>> } }
```
'''   # synthetic types: an `Option` in a list item, in an enum variant's payload and in neither

SELF_TEST_MODELS = '''
[[model]]
widget = "button"
available = true
variants = ["flat"]
sizes = []
densities = []
axes = { states = ["hover", "pressed", "disabled"], conditions = [], contexts = [] }
exclusive = []
not_drawn = [{ states = ["disabled", "pressed"] }]
rows = []

[[model.cite]]
kind = "published"
source = "libadwaita@1.10.0 src/stylesheet/widgets/_buttons.scss:1"
quote = "button:hover"
'''

SELF_TEST_VERIFIER_MODELS = '''
[[model]]
widget = "button"
available = true
variants = ["flat", "@pill"]
sizes = []
densities = []
axes = { states = ["hover", "pressed"], conditions = [], contexts = [] }
exclusive = [["pressed", "hover"]]
not_drawn = []
rows = []

[[model.cite]]
kind = "published"
source = "libadwaita@1.10.0 src/stylesheet/widgets/_buttons.scss:9"
quote = "button:active"
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


_TRANSITION = ('{ properties = ["fill"], kind = "interpolate", duration_ms = 100, delay_ms = 0, timing = "linear", '
               'engine = "win_ui" }')
RESOLUTION_LANE = {   # the lane's facts the resolution cases read (windows: one preset, the worlds are the schemes)
    "docs/facts/windows/roles.toml": _fact("role.window.foreground", '"#000000"', '{ conditions = ["scheme:light"] }')
    + _fact("role.window.foreground", '"#ffffff"', '{ conditions = ["scheme:dark"] }'),
    "docs/facts/windows/tokens.toml": _fact("token.focus.width", '"2px"')
    + _fact("token.motion.durations.button", '"200ms"')
    + _fact("token.motion.durations.button", '"0ms"', '{ conditions = ["animations_off"] }'),
    "docs/facts/windows/button.toml": "".join([
        _fact("button.body.padding.top", '"3px"'), _fact("button.body.padding.bottom", '"3px"'),
        _fact("button.body.padding.top", '"5px"', '{ variants = ["flat"] }'),
        _fact("button.body.min_height", '"= add(24px, add(part.padding.top, part.padding.bottom))"'),
        _fact("button.body.stroke.width", '"= token.focus.width"'),
        _fact("button.body.fill", '"= alpha(role.window.foreground, 0.5)"', '{ states = ["checked"] }'),
        _fact("button.body.stroke.color", '"absent"'),
        _fact("button.body.fill", '"#000000"', '{ states = ["hover"] }'),
        _fact("button.body.fill", '"= m3_role(primary)"', '{ states = ["pressed"] }'),
        _fact("button.body.transitions", f"[{_TRANSITION}]"),
        _fact("button.body.focus.transition", _TRANSITION),
        _fact("button.values.press", '"= token.motion.durations.button"'),
        _fact("button.values.release", '"= token.motion.durations.button"'),
        _fact("button.values.hold", '"#000000"'),
        _fact("button.values.hold", '"#ffffff"', '{ conditions = ["contrast_high"] }'),
        _fact("button.body.opacity", "0.5", '{ states = ["hover"] }'),
        _fact("button.body.opacity", "1"),   # later and general: it applies under hover too (S17)
    ]),
}
RESOLUTION_VERIFIER = "".join([
    _fact("button.body.min_height", '"30px"'),                                     # equivalent: 24 + 3 + 3
    _fact("button.body.stroke.width", '"2px"'),                                    # equivalent: the token
    _fact("button.body.fill", '"#00000080"', '{ states = ["checked"] }'),          # equal in light only
    _fact("button.body.stroke.color", '"= alpha(role.window.foreground, 0)"'),     # absent is no transparent colour
    _fact("button.body.fill", '"= role.window.foreground"', '{ states = ["hover"] }'),   # equal in light only
    _fact("button.body.fill", '"#6750a4"', '{ states = ["pressed"] }'),            # a platform function: unresolved
])


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

    common = {SOURCES.as_posix(): SELF_TEST_SOURCES, CATALOGUE.as_posix(): SELF_TEST_CATALOGUE,
              SPEC.as_posix(): SELF_TEST_SPEC}
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
                 {**common, "docs/facts/gnome/button.toml": head_button, "docs/facts/gnome/tokens.toml": head_tokens,
                  "docs/facts/gnome/state-models.toml": SELF_TEST_MODELS, **RESOLUTION_LANE})
    out = Path(tempfile.mkdtemp(prefix="verify-facts-out-"))
    types = check_facts.Context(repo, core_text=check_facts.SELF_TEST_CORE + SELF_TEST_SKETCH,
                                cat_text=check_facts.SELF_TEST_CATALOGUE, sources_text=check_facts.SELF_TEST_SOURCES,
                                presets=set(), rows=set())
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
              ["README.md", "catalogue.md", "combinations.toml", "facts-format.md", "keys.toml", "pins.toml",
               "spec.md"])
        spec_md = (out / "brief" / "spec.md").read_text()
        heads = [x for x in spec_md.splitlines() if x.startswith("## ")]
        check("brief: the specification's S1 to S21, S17's heading among them, not its progress nor S22",
              ("## S17. Facts as data and the gate (R4, R22)" in heads, heads[0], heads[-1], len(heads),
               "ADD-A1" in spec_md, "Not normative." in spec_md, "### State models" in spec_md),
              (True, "## S1. Section 1", "## S21. Section 21", 22, False, False, True))
        try:
            spec_text(SELF_TEST_SPEC.replace("## S17.", "## S16."))
            failures.append("spec_text: a missing S17 accepted")
        except InputError:
            pass
        check("brief: the format excerpt's sections",
              [x for x in (out / "brief" / "facts-format.md").read_text().splitlines() if x.startswith("## ")],
              ["## The format (normative)", "## Canonical forms (binding)"])
        pins = tomllib.loads((out / "brief" / "pins.toml").read_text())
        check("brief: the platform's pins only, no evidence entry", [e["id"] for e in pins["source"]],
              ["libadwaita@1.10.0"])
        cat = (out / "brief" / "catalogue.md").read_text()
        check("brief: the catalogue's shared sections and the widget's whole entry, nothing else",
              ("## Entry format" in cat, "## Shared value types" in cat, "## Shared flags" in cat,
               "## Selector notation" in cat, "- **Notes:** a note naming the card." in cat, "### link" in cat,
               "Copied: the tokens' definitions." in cat, "Not normative here." in cat),
              (True,) * 5 + (False, True, False))
        check("brief: a citation in the copied normative text is elided, the text around it kept",
              ([f for f in found if f[0] in ("leak", "normative-value")], "border-radius: 6px;" in cat,
               "libadwaita's selectors in `_buttons.scss` (`⟦elided: a citation of this unit⟧`)." in cat),
              ([], False, True))
        readme_md = (out / "brief" / "README.md").read_text()
        cat_line = next(n for n, x in enumerate(cat.splitlines(), 1) if "⟦elided" in x)
        check("brief: the README lists each elision by file, line and class, never the string",
              (f"- `catalogue.md` line {cat_line}: citation" in readme_md, "border-radius: 6px;" in readme_md),
              (True, False))
        spec_file = out / "brief" / "spec.md"
        saved = spec_file.read_text()
        spec_file.write_text(saved + "\nborder-radius: 6px;\n")
        planted = leak_check(out / "brief", unit_facts(repo, doc, find_unit(doc, unit_id))[0],
                             {**inputs, "spec.md": inputs["spec.md"] + "\nborder-radius: 6px;\n"})
        check("leak check: an unelided platform value in normative text is reported",
              [(f[0], f[3]) for f in planted if f[1] == "border-radius: 6px;"], [("normative-value", "spec.md")])
        spec_file.write_text(saved)
        check("redact: a string the identities hold is kept",
              redact("accent #e5a50a", [Fact("f.toml", 1, {"key": "k", "value": "#e5a50a", "accent": "#e5a50a"}, [],
                                             ())]), ("accent #e5a50a", []))
        check("redact: exact, the longest first, trivial strings kept",
              redact("a 'x.c:1-5' b x.c:1 c 6px", [Fact("f.toml", 1, {"key": "k", "value": "6px"}, [
                  {"source": "x.c:1-5"}, {"source": "x.c:1"}], ())]),
              ("a '⟦elided: a citation of this unit⟧' b ⟦elided: a citation of this unit⟧ c 6px",
               [(1, "citation"), (1, "citation")]))
        check("leak check: the model's vocabulary is no platform value, a literal or a text is",
              [platform_value(x, "a value of k") for x in ("circular", "@card", "= role.global.accent",
                                                           "= alpha(role.x, 0.5)", "#ffffff", "0.3", "Cantarell",
                                                           "adw-tab-counter-symbolic")]
              + [platform_value("x.c:1", "a citation's source"), platform_value("font", "a citation's absent name")],
              [False, False, False, True, True, True, True, True, True, False])
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
        summary = compare(repo, units1, "GN-T1", vfile, types=types)
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
        # the two-phase flow: the model comparison, the second brief, the comparison of its combinations
        vmodels = out / "verifier" / "state-models.toml"
        vmodels.write_text(SELF_TEST_VERIFIER_MODELS)
        ms = compare_model(repo, units1, "GN-T1", vmodels)
        check("compare-model: counts", {k: ms[k] for k in (
            "matched_entries", "declaration_differences", "state_combinations_lane_only",
            "state_combinations_verifier_only", "agreed_combinations", "refused_combinations")},
              {"matched_entries": 1, "declaration_differences": 4, "state_combinations_lane_only": 3,
               "state_combinations_verifier_only": 0, "agreed_combinations": 4, "refused_combinations": 2})
        mc = tomllib.loads((out / "verifier" / "model-compare.toml").read_text())
        check("compare-model: each difference listed", {d["field"]: (d["lane_only"], d["verifier_only"])
                                                         for d in mc["entry"][0]["difference"]},
              {"axes.states": (["disabled"], []), "exclusive": ([], [["pressed", "hover"]]),
               "not_drawn": ([{"states": ["disabled", "pressed"]}], []), "variants": ([], ["@pill"])})
        check("compare-model: the combinations only the lane's model gives",
              mc["entry"][0]["state_combinations_lane_only"], [["disabled"], ["disabled", "hover"],
                                                                ["hover", "pressed"]])
        check("compare-model: the refused combinations and why",
              [(r["key"], r["reasons"][0][:24]) for r in mc["refused"]],
              [("button.body.corners.bottom_right", "['hover', 'pressed'] are"),
               ("button.body.opacity", "states ['disabled'] are ")])
        found2, _ = brief(repo, units1, "GN-T1", out / "brief4", lane_brief, Path("/cache"), agreed=mc)
        combos2 = tomllib.loads((out / "brief4" / "combinations.toml").read_text())["combination"]
        check("brief --combinations-from: the agreed combinations only", len(combos2), 4)
        check("brief --combinations-from: no leak", [f for f in found2 if f[0] == "leak"], [])
        s2 = compare(repo, units1, "GN-T1", vfile, out / "second", mc, types)
        check("compare --combinations-from: counts", {k: s2[k] for k in ("lane_facts", "agreed", "lane_only",
                                                                         "refused_by_model")},
              {"lane_facts": 4, "agreed": 3, "lane_only": 0, "refused_by_model": 2})
        check("compare --combinations-from: the ledger line names the refused", "2 refused by the verifier's state "
              "model" in s2["ledger"], True)
        try:
            compare(repo, units1, "GN-T1", vfile, out / "second", {**mc, "summary": {**mc["summary"], "unit": "X"}},
                    types)
            failures.append("compare: another unit's model comparison accepted")
        except InputError:
            pass
        # resolution: a reference or expression against a literal, evaluated in the combination
        units3 = {"rev": doc["rev"], "unit": [{
            "id": "WN-T1", "platform": "windows", "kind": "cited", "presets": ["windows-11"], "facts": 6,
            "identities": [{"file": "button.toml", "key": k, **({"when": w} if w else {})} for k, w in (
                ("button.body.min_height", None), ("button.body.stroke.width", None),
                ("button.body.fill", {"states": ["checked"]}), ("button.body.stroke.color", None),
                ("button.body.fill", {"states": ["hover"]}), ("button.body.fill", {"states": ["pressed"]}))]}]}
        rfile = out / "resolution" / "facts.toml"
        rfile.parent.mkdir()
        rfile.write_text(RESOLUTION_VERIFIER)
        s3 = compare(repo, units3, "WN-T1", rfile, types=types)
        check("resolution: counts, the equivalent never agreed", {k: s3[k] for k in (
            "agreed", "equivalent_by_resolution", "disagreed")},
              {"agreed": 0, "equivalent_by_resolution": 2, "disagreed": 4})
        rt = tomllib.loads((out / "resolution" / "tiebreak.toml").read_text())
        check("resolution: each equivalent item listed with its worlds",
              sorted((e["key"], tuple(e["resolved"])) for e in rt["equivalent"]),
              [("button.body.min_height", ("scheme:light: both 30px", "scheme:dark: both 30px")),
               ("button.body.stroke.width", ("scheme:light: both 2px", "scheme:dark: both 2px"))])
        notes = {(d["key"], str(d.get("when"))): d.get("resolved", []) for d in rt["disagreed"]}
        check("resolution: equal in one world only stays a disagreement",
              notes[("button.body.fill", "{'states': ['checked']}")],
              ["scheme:light: both srgb(0, 0, 0, 0.5)",
               "scheme:dark: lane srgb(1, 1, 1, 0.5), verifier #00000080"])
        check("resolution: absent against a transparent colour stays a disagreement",
              notes[("button.body.stroke.color", "None")],
              ["scheme:light: lane absent, verifier #00000000", "scheme:dark: lane absent, verifier #ffffff00"])
        check("resolution: a platform function is unresolved",
              notes[("button.body.fill", "{'states': ['pressed']}")][0].startswith(
                  "scheme:light: unresolved: the function m3_role()"), True)
        check("resolution: no equivalent in the audit sample's population",
              tomllib.loads((out / "resolution" / "agreed.toml").read_text()).get("agreed", []), [])
        check("resolution: the ledger line", "; 2 more equivalent by resolution |" in s3["ledger"], True)
        check("resolution: the evaluator", [render(Resolver([], Fact("x.toml", 1, {"key": "button.body.x"}, [],
                                                                       identity_of({"key": "button.body.x"})),
                                                                  set()).value(v)) for v in (
            "= mix(#000000, #ffffff, 0.5)", "= max(sub(4px, 6px), div(10px, 4))", "= with_alpha(#ff0000, 0.25)",
            "= round(2.5)", "= pick(1, #000000, #ffffff)")],
              ["srgb(0.5, 0.5, 0.5)", "2.5px", "srgb(1, 0, 0, 0.25)", "3", "#ffffff"])
        # an Option left out of a list item is None (S11), so it agrees with a stated "none"; in a single
        # transition, where `from` is stated, it stays a difference
        units4 = {"rev": doc["rev"], "unit": [{
            "id": "WN-T2", "platform": "windows", "kind": "cited", "presets": ["windows-11"], "facts": 2,
            "identities": [{"file": "button.toml", "key": k} for k in ("button.body.transitions",
                                                                         "button.body.focus.transition")]}]}
        ofile = out / "omitted" / "facts.toml"
        ofile.parent.mkdir()
        with_none = _TRANSITION.replace(' }', ', from = "none" }')
        ofile.write_text(_fact("button.body.transitions", f"[{with_none}]")
                         + _fact("button.body.focus.transition", with_none))
        s4 = compare(repo, units4, "WN-T2", ofile, types=types)
        ot = tomllib.loads((out / "omitted" / "tiebreak.toml").read_text())
        check("omitted option: a list item's `none` agrees with one left out, a single transition's does not",
              (s4["agreed"], s4["disagreed"], [d["key"] for d in ot.get("disagreed", [])],
               {n["name"]: (n["changed"], n["decisive"]) for n in ot["normalisation"]}["omitted_option"]),
              (1, 1, ["button.body.focus.transition"], (1, 1)))
        check("omitted option: an enum variant's payload too, never outside one",
              [without_none_options(types, v, "SketchPaint") for v in (
                  {"layers": [{"kind": {"linear": {"angle_deg": 0, "offset_px": "none"}}}], "inset": "none"},
                  {"kind": {"linear": {"angle_deg": 0, "offset_px": "none"}}, "inset": "none"})],
              [{"layers": [{"kind": {"linear": {"angle_deg": 0}}}], "inset": "none"},
               {"kind": {"linear": {"angle_deg": 0}}, "inset": "none"}])
        # the conditions a combination leaves open, as the schemes: a reference that differs under one is no
        # equivalent; each side's value is the one its facts give the combination, a later general fact over an
        # earlier specific one (S17 *Which fact applies*); and a world where an opened condition hands either side's
        # combination to another fact is that fact's, not compared here
        units5 = {"rev": doc["rev"], "unit": [{
            "id": "WN-T3", "platform": "windows", "kind": "cited", "presets": ["windows-11"], "facts": 5,
            "identities": [{"file": "button.toml", "key": k, **({"when": w} if w else {})} for k, w in (
                ("button.values.press", None), ("button.values.release", None), ("button.values.hold", None),
                ("button.body.opacity", {"states": ["hover"]}), ("button.body.opacity", None))]}]}
        efile = out / "effective" / "facts.toml"
        efile.parent.mkdir()
        efile.write_text("".join([
            _fact("button.values.press", '"200ms"'),                                       # not under animations_off
            _fact("button.values.release", '"200ms"'),
            _fact("button.values.release", '"0ms"', '{ conditions = ["animations_off"] }'),   # its own combination
            _fact("button.values.hold", '"#000000"'),        # agreed: the lane's contrast_high fact is another's
            _fact("button.body.opacity", "1"),
            _fact("button.body.opacity", "0.5", '{ states = ["hover"] }'),   # the lane's own, which never applies there
        ]))
        line_of = {(f.file, f.data["key"], str(f.data.get("when"))): f.line for f in read_facts(
            "docs/facts/windows/button.toml", "windows", RESOLUTION_LANE["docs/facts/windows/button.toml"])
            + read_facts(efile.as_posix(), "windows", efile.read_text())}
        s5 = compare(repo, units5, "WN-T3", efile, types=types)
        et = tomllib.loads((out / "effective" / "tiebreak.toml").read_text())
        check("open conditions and effective values: counts", {k: s5[k] for k in (
            "agreed", "equivalent_by_resolution", "disagreed", "verifier_only")},
              {"agreed": 2, "equivalent_by_resolution": 1, "disagreed": 2, "verifier_only": 1})
        enotes = {(d["key"], str(d.get("when"))): d.get("resolved") for d in et["disagreed"] + et["equivalent"]}
        check("open conditions: a reference differing under animations_off stays a disagreement",
              enotes[("button.values.press", "None")],
              ["animations_off, scheme:light: lane 0ms, verifier 200ms", "scheme:light: both 200ms",
               "animations_off, scheme:dark: lane 0ms, verifier 200ms", "scheme:dark: both 200ms"])
        check("open conditions: a world another fact of either side takes is not compared",
              enotes[("button.values.release", "None")][:2],
              ["animations_off, scheme:light: not compared, another fact's combination (the verifier's facts.toml:"
               f"{line_of[('facts.toml', 'button.values.release', str({'conditions': ['animations_off']}))]} applies)",
               "scheme:light: both 200ms"])
        check("effective values: the lane's later general fact applies over its own specific one",
              enotes[("button.body.opacity", "{'states': ['hover']}")],
              [f"scheme:{x}: lane 1, verifier 0.5 (the lane's button.toml:"
               f"{line_of[('button.toml', 'button.body.opacity', 'None')]} applies)" for x in ("light", "dark")])
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
        print("self-test ok (units, brief and leak check, specification sections, compare-model and the two-step "
              "flow, compare and its normalisations, sample)")
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
