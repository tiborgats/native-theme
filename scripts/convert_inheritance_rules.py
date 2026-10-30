#!/usr/bin/env python3
"""Convert today's inheritance rules into S19.2's staged bases (specification S19.2; the 0.8.0 plan's *What carries
over*).

Today's inheritance rules of docs/inheritance-rules.toml (`[defaults_internal]`, `[border_inheritance]`,
`[font_inheritance]`, `[uniform]`, `[per_platform]`, `[text_scale_inheritance]`: S19.2 names `[uniform]` and
`[per_platform]` as its examples; the ledger's queue #31) become, per preset, each field the preset inherits stated as
an S2.2 reference over the new model's paths, in a staged input file (docs/facts/staged/inheritance-bases.toml,
STAGED) that each unit's preset step uses and the gate checks. The `[wrong_safety_nets]` are not converted (the facts
replace them, S19.2), nor `[no_inheritance]` and `[runtime_detection]`, which name no source. The file is removed at
the plan's M4 (`scripts/check_ledger.py --final --staged-file`), and this script with it.

Run from the repository root:

    python3 scripts/convert_inheritance_rules.py             # writes the staged file
    python3 scripts/convert_inheritance_rules.py --check     # exit 1 when it differs from a fresh run
    python3 scripts/convert_inheritance_rules.py --self-test

It prints the counts per preset. Exit 0 on success (with --check: the file equals a fresh run), 1 when --check finds
a difference, 2 on an input it cannot read or a wrong command line.

Rules
-----
The rules are read at commit fc6109d1 (SOURCE_PIN, the file in the tree; it is not edited before its removal). A
`[defaults_internal]` or `[uniform]` rule `"<field>" = "<source>"` holds on every preset; a `[per_platform]` rule's
`fallback` is what today's resolve() fills on every preset, its platform keys (`macos`, `windows`, `kde`, `gnome`)
documenting each platform's own source: the entry takes the `fallback` and names in its `note` the preset's platform
key where that differs. The generated sections expand into one rule per field: `[border_inheritance]` gives each
`widgets_with_border` widget's `border.<sub>` the source of its `"<sub>.inherits_from"` key, `corner_radius` taking
`defaults.border.corner_radius_lg` on the `corner_radius_lg_widgets`; `[font_inheritance]` gives each
`widgets_with_font` font's sub-fields (FONT_FIELDS: today's FontSpec, native-theme/src/model/font.rs:156-168, as the
section's comment lists them) the same sub-field of its `inherits_from`; `[text_scale_inheritance]` gives each
`entries` level's `text_scale.<level>.<sub>` its `"<sub>.inherits_from"` source. A field an explicit rule (`[uniform]`,
`[per_platform]`) names takes that rule only (`link.font.color` is `[uniform]`'s, as today's resolve() overrides the
font rule). A source that is no path (`defaults.line_height * size`) gives no value.

Presets
-------
The new model's platform presets that have one today (PRESETS: today's `native-theme/src/presets/<id>.toml`; `ios-26`
has none, and the community presets become palettes, which have no bases). A rule's field is inherited on a preset
when today's preset leaves it unset under a scheme (`[light.<widget>]`, `[dark.<widget>]`, the field's TOML key with or
without its unit suffix `_px`, `_ms`, `_pt`, `_deg`); `schemes` lists those. A rule whose field the preset states under
both schemes gives no entry.

Model paths
-----------
Today's field and today's source map to the new model's paths through the widget catalogue's *Today's fields*
(docs/todo_v0.7.1_model-completeness-spec-widgets.md, through scripts/spec_catalogue.py): a widget entry's item for a
widget's field, the `#### today's fields` table for a `defaults.` field. Nothing else is assumed:

- A widget item is split into clauses at `;` outside backticks and parentheses, each `<names> → <home>`. A clause
  maps one today's name (its unit suffix dropped) to one path when its home, without its parenthesised notes, is one
  backtick span and no note holds a backtick span (a note with a span may add a target or a preset's alternative). A
  font's colour is mapped by the clause's own `(its `color` → `<path>`)`, `(its colour → `<path>`)`, `(colour
  `<path>`)` or `, its colour `<path>``; failing those, a font whose home is one `<part>.text.font` or `<part>.text`
  path gives `<part>.text.fill` (the table's `color` -> `TextStyle.fill`). A clause naming several fields, or a
  `border.*` wildcard, maps none of them.
- A `defaults.` row maps its one field to its home's one span, or its fields pairwise to as many spans; a home with
  prose besides its spans (`no global: …`), a span that is no path (`<set>.border`) or a pattern (`{toolbar,…}`, `*`)
  maps nothing.
- A font's `family`, `size`, `weight` and `style`, where the font's clause maps it to one `<part>.text.font` or
  `<part>.text` path, map to `<part>.text.font.<sub>` (the table's `defaults.font` row: `font.family`, `font.size`,
  `font.weight`, `font.style`), as its `color` maps to `<part>.text.fill`.
- A widget path is checked against the catalogue: `<part>.<field>…` with the field one of the part's fields,
  `<widget>.<part>.<field>…` with the widget a catalogue entry, or `values.<name>` a widget value; a path that reads
  neither way, or both, maps nothing. A selector (` @ …`) is kept as the catalogue writes it. A `role.`, `token.` or
  `input.` path is taken as written.

`target` is today's field's path (`<widget>.<path>`), or `""` with a `note` where it has none. `value` is the S2.2
reference (`= <path>`): a role, token or input path as it stands; a path of the target's own widget as `= part.<field
path>` (the same part) or `= widget.<part>.<field path>`, or `= values.<name>`; `""` with a `note` where the source has
no path, is another widget's (S2.2 reaches no other widget), is a rule (a selector), or is the target's own field
(a rule restating it needs no base). `mapping` quotes each catalogue clause used, with its line.

Output
------
`[[base]]` entries, preset by preset (PRESETS' order), each preset's rules in the file's order (a generated section's
fields in its lists' order, each field's sub-fields in its keys' order): `preset`, `field`,
`schemes`, `rule`, `source` (`inheritance-rules.toml@fc6109d1:<line>`), `target`, `value`, `mapping`, and `note`
where a field is empty or the rule needs a word.
"""
import re
import sys
import tempfile
import tomllib
from pathlib import Path

import spec_catalogue

DOCS = Path("docs")
RULES = DOCS / "inheritance-rules.toml"
SOURCE_PIN = "fc6109d1"
CITE = f"inheritance-rules.toml@{SOURCE_PIN}"
CATALOGUE = DOCS / "todo_v0.7.1_model-completeness-spec-widgets.md"
TODAY_PRESETS = Path("native-theme/src/presets")
STAGED = DOCS / "facts" / "staged" / "inheritance-bases.toml"
# preset: its [per_platform] key (None: no key names its platform)
PRESETS = {
    "adwaita": "gnome",
    "kde-breeze": "kde",
    "macos-monterey": "macos",
    "macos-tahoe": "macos",
    "ios-13": None,
    "windows-11": "windows",
    "material-3": None,
}
SCHEMES = ("light", "dark")
# the converted sections, in the file's order
SECTIONS = ("defaults_internal", "border_inheritance", "font_inheritance", "uniform", "per_platform",
            "text_scale_inheritance")
SUFFIXES = ("", "_px", "_ms", "_pt", "_deg")
SUFFIX_RX = re.compile(r"_(px|ms|pt|deg)$")
MODEL_REF = re.compile(r"^(role|token|input)\.[a-z0-9_@.]+$")
PATH_RX = re.compile(r"^[a-z_][a-z0-9_]*(\.[a-z0-9_]+)*$")
FONT_FIELDS = ("family", "size", "weight", "style", "color")
FONT_SUBS = ("family", "size", "weight", "style")
COLOUR_RXS = (re.compile(r"\(its (?:`color`|colour) → `([^`]+)`\)"), re.compile(r"\(colour `([^`]+)`\)"),
              re.compile(r", its colour `([^`]+)`"))


class InputError(Exception):
    pass


def depth0_split(text: str, sep: str) -> list[str]:
    """text split at `sep` outside backticks and parentheses."""
    out, cur, depth, tick = [], [], 0, False
    i = 0
    while i < len(text):
        ch = text[i]
        if ch == "`":
            tick = not tick
        elif not tick and ch == "(":
            depth += 1
        elif not tick and ch == ")":
            depth = max(depth - 1, 0)
        if not tick and depth == 0 and text.startswith(sep, i):
            out.append("".join(cur))
            cur = []
            i += len(sep)
            continue
        cur.append(ch)
        i += 1
    out.append("".join(cur))
    return out


def strip_notes(text: str) -> tuple[str, list[str]]:
    """(text without its parenthesised groups outside backticks, the groups)."""
    out, notes, depth, tick, cur = [], [], 0, False, []
    for ch in text:
        if ch == "`" and depth == 0:
            tick = not tick
        if not tick and ch == "(":
            depth += 1
        if depth:
            cur.append(ch)
        else:
            out.append(ch)
        if not tick and ch == ")" and depth:
            depth -= 1
            if depth == 0:
                notes.append("".join(cur))
                cur = []
    return "".join(out), notes


def spans(text: str) -> list[str]:
    return re.findall(r"`([^`]*)`", text)


def only_spans(text: str) -> list[str] | None:
    """The spans when text is nothing but `a`, `b`, … (commas and spaces between), else None."""
    s = spans(text)
    rest = re.sub(r"`[^`]*`", "", text)
    return s if s and re.fullmatch(r"[\s,.]*", rest) else None


def today_name(name: str) -> str:
    return ".".join(SUFFIX_RX.sub("", seg) for seg in name.split("."))


class Catalogue:
    def __init__(self, text: str):
        self.entries = {e.name: e for e in spec_catalogue.entries(text)}
        self.widgets = {}
        for name, e in self.entries.items():
            item = e.item("Today's fields")
            if item is not None:
                self.widgets[name] = self.clauses(item)
        self.defaults = self.defaults_table(text)

    @staticmethod
    def clauses(item) -> dict:
        """{today's name: (path or None, quoted clause)} of one widget's Today's fields."""
        out = {}
        text = item.text
        offset = 0
        for clause in depth0_split(text, ";"):
            start = text.find(clause, offset)
            offset = start + len(clause)
            clause = clause.strip().rstrip(".").strip()
            parts = depth0_split(clause, " → ")
            if len(parts) != 2:
                continue
            quoted = f"spec-widgets.md:{item.line_at(max(start, 0))} {clause}"
            lhs, rhs = parts
            names = [today_name(n) for n in spans(re.sub(r"`[^`]*`'s", "", lhs))]
            colour = None
            for rx in COLOUR_RXS:
                m = rx.search(rhs)
                if m:
                    colour = m.group(1)
                    rhs = rhs[:m.start()] + rhs[m.end():]
                    break
            clean, notes = strip_notes(rhs)
            single = only_spans(clean)
            ok = (len(names) == 1 and single is not None and len(single) == 1
                  and not any(spans(n) for n in notes) and "*" not in names[0])
            for n in names:
                out[n] = (single[0] if ok else None, quoted)
            if colour and len(names) == 1:
                out[names[0] + ".color"] = (colour, quoted)
        return out

    @staticmethod
    def defaults_table(text: str) -> dict:
        """{defaults.<field>: (path or None, quoted row)}, and {pattern: (None, quoted row)} for its patterns."""
        out = {}
        lines = text.splitlines()
        start = next((i for i, l in enumerate(lines) if l.strip() == "#### today's fields"), None)
        if start is None:
            raise InputError(f"{CATALOGUE}: no `#### today's fields` section")
        for i in range(start + 1, len(lines)):
            line = lines[i]
            if line.startswith("#"):
                break
            if not line.startswith("|"):
                continue
            c = [x.strip() for x in line.strip().strip("|").split("|")]
            if len(c) != 2 or not c[0].startswith("`defaults."):
                continue
            quoted = f"spec-widgets.md:{i + 1} {c[0]} -> {c[1]}"
            lhs_clean, _ = strip_notes(c[0])
            names = []
            for k, s in enumerate(spans(lhs_clean)):
                names.append(s if s.startswith("defaults.") else "defaults." + s)
            names = [today_name(n) for n in names]
            clean, notes = strip_notes(c[1])
            homes = only_spans(clean)
            ok = (homes is not None and len(homes) == len(names)
                  and all(MODEL_REF.match(h) for h in homes))
            for k, n in enumerate(names):
                out[n] = (homes[k] if ok and not re.search(r"[{*]", n) else None, quoted)
        return out

    def widget_path(self, widget: str, path: str) -> str | None:
        """`<widget>.<path>` checked against the catalogue (the selector kept), or None."""
        bare, _, selector = path.partition(" @ ")
        if not PATH_RX.match(bare):
            return None
        segs = bare.split(".")
        found = []
        e = self.entries.get(widget)
        if e is not None:
            if segs[0] == "values" and len(segs) >= 2:
                if segs[1] in {SUFFIX_RX.sub("", v.name) for v in e.values}:
                    found.append(f"{widget}.{bare}")
            elif len(segs) >= 2 and any(segs[0] in p.names and segs[1] in p.fields for p in e.parts):
                found.append(f"{widget}.{bare}")
        other = self.entries.get(segs[0])
        if other is not None and len(segs) >= 3 and any(segs[1] in p.names and segs[2] in p.fields
                                                         for p in other.parts):
            found.append(bare)
        if len(found) != 1:
            return None
        return found[0] + (f" @ {selector}" if selector else "")

    def path(self, field: str) -> tuple[str | None, str | None, str]:
        """(the model path of today's field or None, the quoted clause or None, why not)."""
        if field.startswith("defaults."):
            hit = self.defaults.get(field)
            if hit is None:
                for pat, (_, quoted) in self.defaults.items():
                    if pattern_rx(pat).fullmatch(field):
                        return None, quoted, f"`{field}`: the catalogue's row names no one path"
                return None, None, f"`{field}`: the catalogue's today's-fields table has no row"
            if hit[0] is None:
                return None, hit[1], f"`{field}`: the catalogue's row names no one path"
            return hit[0], hit[1], ""
        widget, _, rest = field.partition(".")
        clauses = self.widgets.get(widget)
        if clauses is None:
            return None, None, f"`{field}`: the catalogue has no Today's fields for `{widget}`"
        hit = clauses.get(rest)
        head, _, sub = rest.rpartition(".")
        if hit is None and (sub == "color" or sub in FONT_SUBS):
            font = clauses.get(head)
            if font is not None and font[0] is not None:
                m = re.fullmatch(r"([a-z_]+)\.text(?:\.font)?", font[0])
                if m:
                    hit = (f"{m.group(1)}.text.fill" if sub == "color" else f"{m.group(1)}.text.font.{sub}", font[1])
        if hit is None:
            star = next((q for k, (_, q) in clauses.items() if k.endswith("*") and rest.startswith(k[:-1])), None)
            if star:
                return None, star, f"`{field}`: the catalogue's clause names no one path"
            return None, None, f"`{field}`: the catalogue's Today's fields of `{widget}` do not name it"
        if hit[0] is None:
            return None, hit[1], f"`{field}`: the catalogue's clause names no one path"
        full = self.widget_path(widget, hit[0])
        if full is None:
            return None, hit[1], f"`{field}`: `{hit[0]}` is no path of the catalogue's `{widget}`"
        return full, hit[1], ""


def pattern_rx(pattern: str):
    """A catalogue name with `*` (any name) or `{a,b}` (one of them) as a regular expression."""
    out = []
    for piece in re.split(r"(\*|\{[^}]*\})", pattern):
        if piece == "*":
            out.append("[a-z0-9_]+")
        elif piece.startswith("{") and piece.endswith("}"):
            out.append("(?:" + "|".join(re.escape(x.strip()) for x in piece[1:-1].split(",")) + ")")
        else:
            out.append(re.escape(piece))
    return re.compile("".join(out))


def reference(source: str, target: str | None, widget: str) -> tuple[str, str]:
    """(the S2.2 reference to `source` from a base of `target`, or "", why not); `widget` is today's field's, the
    target's own where the target has no path."""
    if MODEL_REF.match(source):
        return f"= {source}", ""
    if " @ " in source:
        return "", "the source is a rule (a selector), not a base"
    sw, sp, *srest = source.split(".")
    if target:
        widget = target.split(".")[0]
    if sw != widget:
        return "", f"the source `{source}` is another widget's: S2.2 has no reference to it"
    if sp == "values":
        return f"= values.{'.'.join(srest)}", ""
    if not target:
        return "", "the target has no path"
    tp = target.split(" @ ")[0].split(".")[1]
    if target.split(" @ ")[0] == source:
        return "", "the source is the target's own field: a rule restating it needs no base"
    if sp == tp:
        return f"= part.{'.'.join(srest)}", ""
    return f"= widget.{sp}.{'.'.join(srest)}", ""


def rule_lines(text: str) -> dict:
    """{(section, key): line} for every section's keys (quoted or bare) and [per_platform]'s tables (their
    `fallback` line)."""
    out, section, table = {}, None, None
    for n, line in enumerate(text.splitlines(), 1):
        m = re.match(r'^\[(\w+)(?:\."([^"]+)")?\]', line)
        if m:
            section, table = m.group(1), m.group(2)
            continue
        m = re.match(r'^(?:"([^"]+)"|([A-Za-z_][A-Za-z0-9_]*))\s*=', line)
        if section and not table and m:
            out[(section, m.group(1) or m.group(2))] = n
        if section == "per_platform" and table and re.match(r"^fallback\s*=", line):
            out[("per_platform", table)] = n
    return out


def generated_rules(rules: dict, lines: dict) -> dict:
    """{section: [(field, source, line, rule)]} of the generated sections, expanded per field."""
    def line(sec, key):
        if (sec, key) not in lines:
            raise InputError(f"{RULES}: no line for [{sec}] {key}")
        return lines[(sec, key)]

    def subs(table):
        return [(k[:-len(".inherits_from")], v) for k, v in table.items() if k.endswith(".inherits_from")]

    out = {"border_inheritance": [], "font_inheritance": [], "text_scale_inheritance": []}
    border = rules.get("border_inheritance", {})
    lg = border.get("corner_radius_lg_widgets", [])
    for w in border.get("widgets_with_border", []):
        for sub, source in subs(border):
            key = f"{sub}.inherits_from"
            if sub == "corner_radius" and w in lg:
                source, key = f"{source}_lg", "corner_radius_lg_widgets"
            out["border_inheritance"].append((f"{w}.border.{sub}", source, line("border_inheritance", key),
                                              f"[border_inheritance] {w}.border.{sub}: {key} -> {source}"))
    font = rules.get("font_inheritance", {})
    if font.get("widgets_with_font"):
        if not isinstance(font.get("inherits_from"), str):
            raise InputError(f"{RULES}: [font_inheritance] has no inherits_from")
        n = line("font_inheritance", "inherits_from")
        for f in font["widgets_with_font"]:
            for sub in FONT_FIELDS:
                source = f"{font['inherits_from']}.{sub}"
                out["font_inheritance"].append((f"{f}.{sub}", source, n,
                                                f"[font_inheritance] {f}.{sub}: inherits_from -> {source}"))
    scale = rules.get("text_scale_inheritance", {})
    for level in scale.get("entries", []):
        for sub, source in subs(scale):
            out["text_scale_inheritance"].append(
                (f"text_scale.{level}.{sub}", source, line("text_scale_inheritance", f"{sub}.inherits_from"),
                 f"[text_scale_inheritance] text_scale.{level}.{sub}: {sub}.inherits_from -> {source}"))
    return out


def stated(preset: dict, scheme: str, field: str) -> bool:
    node = preset.get(scheme, {})
    segs = field.split(".")
    for seg in segs[:-1]:
        node = node.get(seg) if isinstance(node, dict) else None
        if node is None:
            return False
    return isinstance(node, dict) and any(segs[-1] + s in node for s in SUFFIXES)


def build(root: Path) -> tuple[str, dict]:
    try:
        rules_text = (root / RULES).read_text(encoding="utf-8")
        rules = tomllib.loads(rules_text)
        cat = Catalogue((root / CATALOGUE).read_text(encoding="utf-8"))
        presets = {p: tomllib.loads((root / TODAY_PRESETS / f"{p}.toml").read_text(encoding="utf-8"))
                   for p in PRESETS}
    except (OSError, tomllib.TOMLDecodeError) as err:
        raise InputError(str(err)) from err
    lines = rule_lines(rules_text)
    wrong = rules.get("wrong_safety_nets", {})
    explicit = set(rules.get("uniform", {})) | set(rules.get("per_platform", {}))
    generated = generated_rules(rules, lines)
    todo = []   # (section, field, source, per_platform table, source line, rule)
    for sec in SECTIONS:
        if sec in generated:
            todo += [(sec, f, src, {}, n, rule) for f, src, n, rule in generated[sec] if f not in explicit]
            continue
        for field, value in rules.get(sec, {}).items():
            if sec == "per_platform":
                if "fallback" not in value:
                    raise InputError(f"{RULES}: [per_platform.\"{field}\"] has no fallback")
                source, rule = value["fallback"], f"[per_platform] {field}: fallback = {value['fallback']}"
            else:
                source, rule = value, f"[{sec}] {field} = {value}"
            if (sec, field) not in lines:
                raise InputError(f"{RULES}: no line for [{sec}] {field}")
            todo.append((sec, field, source, value if sec == "per_platform" else {}, lines[(sec, field)], rule))
    blocks, counts = [], {}
    for preset, platform in PRESETS.items():
        n = n_target = n_value = 0
        for sec, field, source, table, src_line, rule in todo:
            schemes = [s for s in SCHEMES if not stated(presets[preset], s, field)]
            if not schemes:
                continue
            target, t_quote, t_why = cat.path(field)
            spath, s_quote, s_why = cat.path(source) if PATH_RX.match(source) else (None, None, "")
            if not PATH_RX.match(source):
                value, v_why = "", f"the source `{source}` is no path (today's expression)"
            elif spath is None:
                value, v_why = "", f"the source has no path ({s_why})"
            else:
                value, v_why = reference(spath, target, field.split(".")[0])
            notes = [w for w in (t_why if target is None else "", v_why) if w]
            if sec == "per_platform":
                own = table.get(platform) if platform else None
                if own is None:
                    notes.append("no [per_platform] key names this preset's platform: today's fallback")
                elif own == "preset":
                    notes.append(f"[per_platform] names `preset` for {platform}: today's preset leaves it to the "
                                 f"fallback")
                elif own != source:
                    notes.append(f"[per_platform] names `{own}` for {platform}; today's resolve() fills the fallback")
            if field in wrong:
                notes.append(f"also a [wrong_safety_nets] entry (not converted): {wrong[field].get('why_wrong', '')}")
            mapping = list(dict.fromkeys(q for q in (t_quote, s_quote) if q))
            fields = [("preset", preset), ("field", field), ("schemes", schemes), ("rule", rule),
                      ("source", f"{CITE}:{src_line}"), ("target", target or ""), ("value", value),
                      ("mapping", mapping)]
            if notes:
                fields.append(("note", "; ".join(notes)))
            blocks.append("[[base]]\n" + "".join(f"{k} = {toml_value(v)}\n" for k, v in fields))
            n += 1
            n_target += bool(target)
            n_value += bool(value)
        counts[preset] = (n, n_target, n_value)
    return HEADER + "\n" + "\n".join(blocks), counts


HEADER = f"""\
# S19.2's staged inheritance bases, from {CITE} (its inheritance rules; not [wrong_safety_nets]): per
# preset, each field today's preset leaves to a rule, as the S2.2 reference that rule implies over the new model's
# paths (the widget catalogue's *Today's fields*). Input to each unit's preset step, which the gate checks; not facts. Written by
# scripts/convert_inheritance_rules.py, whose docstring states the rules; removed at the 0.8.0 plan's M4.
"""


def toml_string(s: str) -> str:
    out = []
    for ch in s:
        if ch == "\\":
            out.append("\\\\")
        elif ch == '"':
            out.append('\\"')
        elif ch == "\n":
            out.append("\\n")
        elif ch == "\t":
            out.append("\\t")
        elif ord(ch) < 0x20 or ord(ch) == 0x7F:
            out.append(f"\\u{ord(ch):04X}")
        else:
            out.append(ch)
    return '"' + "".join(out) + '"'


def toml_value(v) -> str:
    if isinstance(v, str):
        return toml_string(v)
    if isinstance(v, list):
        if not v:
            return "[]"
        return "[\n" + "".join(f"    {toml_string(x)},\n" for x in v) + "]"
    raise TypeError(v)


SELF_TEST_RULES = """\
[defaults_internal]
"defaults.selection_background" = "defaults.accent_color"

[uniform]
"button.background_color" = "defaults.background_color"    # see wrong_safety_nets
"button.hover_text_color" = "button.font.color"
"button.active_text_color" = "button.label_color"
"button.disabled_opacity" = "defaults.disabled_opacity"
"button.primary_text_color" = "defaults.accent_text_color"
"button.border.color" = "defaults.border.color"
"menu.icon_size" = "defaults.icon_sizes.toolbar"
"text_area.border.color" = "input.border.color"
"list.hover_background" = "list.background_color"
"sidebar.hover_background" = "sidebar.background_color"
"link.font.color" = "defaults.link_color"

[per_platform]

[per_platform."input.caret_color"]
fallback = "defaults.text_color"
macos    = "preset"
kde      = "defaults.focus_ring_color"
gnome    = "defaults.text_color"

[wrong_safety_nets]

[wrong_safety_nets."button.background_color"]
inherits_from = "defaults.background_color"
why_wrong     = "distinct"

[border_inheritance]
widgets_with_border = ["button", "list"]
"color.inherits_from"         = "defaults.border.color"
"corner_radius.inherits_from" = "defaults.border.corner_radius"
corner_radius_lg_widgets = ["list"]

[font_inheritance]
widgets_with_font = ["button.font", "link.font"]
inherits_from = "defaults.font"
sub_field_inheritance = true

[text_scale_inheritance]
entries = ["caption"]
"size.inherits_from"        = "defaults.font.size"
"line_height.inherits_from" = "defaults.line_height * size"
"""

SELF_TEST_CATALOGUE = """\
## Buttons and toggles

### button

- **Parts:**
  - `body` — the root: `fill`, `stroke`, `opacity`
  - `label` — the text: `text`
  - `icon` — the icon: `icon`
- **Widget values:** `icon_text_gap_px: Value<Px>`.
- **Today's fields:** `background_color` → `body.fill`; `font` → `label.text.font` (its `color` →
  `label.text.fill`); `hover_text_color` → `label.text.fill @ {hover}`; `disabled_opacity` → `body.opacity @
  {disabled}`; `primary_text_color` → `label.text.fill @ variant primary` (and `icon.icon.fill`); `label_color` →
  `values.icon_text_gap`; `border.color` / `border.line_width` → `body.stroke`.

### input

- **Parts:**
  - `body` — the root: `fill`, `stroke`
  - `caret` — the caret: `fill`
- **Today's fields:** `caret_color` → `caret.fill` (base `= token.caret.color`); `border.*` → `body.stroke`.

### menu

- **Parts:**
  - `body` — the root: `fill`
  - `item_icon` — the icon: `icon`
- **Today's fields:** `icon_size_px` → `item_icon.icon.size`.

## Containers

### list

- **Parts:**
  - `view` — the root: `fill`
  - `row_background` — a row: `fill`
- **Today's fields:** `background_color` → `view.fill`; `hover_background` → `row_background.fill @ {hover}`.

### text_area

- **Parts:**
  - `body` — the root: `stroke`
- **Today's fields:** `border.*` → `body.stroke`, `body.corners`.

### sidebar

- **Parts:**
  - `pane` — the root: `fill`
- **Today's fields:** `background_color` → `pane.fill`; `hover_background` → `list.row_background.fill @ {hover}
  variant sidebar`.

## Text ramp

#### today's fields

| Today's field | New home |
|---|---|
| `defaults.background_color` | `role.window.background` |
| `defaults.text_color` | `role.window.foreground` |
| `defaults.accent_color`, `accent_text_color` | `role.global.accent`, `role.global.accent_foreground` |
| `defaults.border.color`, `border.opacity` | `<set>.border`, the opacity folded in as `alpha(...)` |
| `defaults.disabled_opacity` | `token.params.opacity.disabled` |
| `defaults.icon_sizes.{toolbar,small}_px` | `token.icons.sizes.{toolbar,small}` (each an `IconStyle`) |
"""

SELF_TEST_PRESETS = {
    "adwaita": '[light.button]\nbackground_color = "#ffffff"\n[dark.button]\nbackground_color = "#000000"\n'
               '[light.input]\ncaret_color = "#000000"\n[dark.input]\ncaret_color = "#ffffff"\n'
               '[light.menu]\nicon_size_px = 16\n[dark.menu]\nicon_size_px = 16\n'
               '[light.button.border]\ncolor = "#000000"\n[dark.button.border]\ncolor = "#000000"\n',
    "kde-breeze": '[light.button]\nhover_text_color = "#000000"\n',
}

# (preset, field, key, the value wanted); None: no entry
SELF_TEST_WANT = [
    ("adwaita", "button.background_color", None, None),
    ("kde-breeze", "button.background_color", "target", "button.body.fill"),
    ("kde-breeze", "button.background_color", "value", "= role.window.background"),
    ("kde-breeze", "button.background_color", "note", "also a [wrong_safety_nets] entry (not converted): distinct"),
    ("kde-breeze", "button.background_color", "source", f"{CITE}:5"),
    ("kde-breeze", "button.background_color", "schemes", ["light", "dark"]),
    ("adwaita", "button.hover_text_color", "target", "button.label.text.fill @ {hover}"),
    ("adwaita", "button.hover_text_color", "value", ""),
    ("adwaita", "button.hover_text_color", "note",
     "the source is the target's own field: a rule restating it needs no base"),
    ("adwaita", "button.active_text_color", "target", ""),
    ("adwaita", "button.active_text_color", "value", "= values.icon_text_gap"),
    ("adwaita", "button.disabled_opacity", "target", "button.body.opacity @ {disabled}"),
    ("adwaita", "button.disabled_opacity", "value", "= token.params.opacity.disabled"),
    ("adwaita", "button.primary_text_color", "target", ""),
    ("adwaita", "button.primary_text_color", "value", "= role.global.accent_foreground"),
    ("adwaita", "button.border.color", None, None),
    ("windows-11", "button.border.color", "target", ""),
    ("windows-11", "button.border.color", "value", ""),
    ("adwaita", "menu.icon_size", None, None),
    ("ios-13", "menu.icon_size", "target", "menu.item_icon.icon.size"),
    ("ios-13", "menu.icon_size", "value", ""),
    ("ios-13", "text_area.border.color", "target", ""),
    ("ios-13", "list.hover_background", "target", "list.row_background.fill @ {hover}"),
    ("ios-13", "list.hover_background", "value", "= widget.view.fill"),
    ("ios-13", "sidebar.hover_background", "target", "list.row_background.fill @ {hover} variant sidebar"),
    ("ios-13", "sidebar.hover_background", "value", ""),
    ("ios-13", "sidebar.hover_background", "note",
     "the source `sidebar.pane.fill` is another widget's: S2.2 has no reference to it"),
    ("adwaita", "input.caret_color", None, None),
    ("kde-breeze", "input.caret_color", "target", ""),
    ("kde-breeze", "button.hover_text_color", "schemes", ["dark"]),
    ("kde-breeze", "input.caret_color", "value", "= role.window.foreground"),
    ("kde-breeze", "input.caret_color", "note", "`input.caret_color`: the catalogue's clause names no one path; "
                                                "[per_platform] names `defaults.focus_ring_color` for kde; "
                                                "today's resolve() fills the fallback"),
    ("macos-tahoe", "input.caret_color", "note",
     "`input.caret_color`: the catalogue's clause names no one path; "
     "[per_platform] names `preset` for macos: today's preset leaves it to the fallback"),
    ("material-3", "input.caret_color", "note",
     "`input.caret_color`: the catalogue's clause names no one path; "
     "no [per_platform] key names this preset's platform: today's fallback"),
    ("material-3", "input.caret_color", "source", f"{CITE}:20"),
    ("ios-13", "defaults.selection_background", "rule",
     "[defaults_internal] defaults.selection_background = defaults.accent_color"),
    ("ios-13", "defaults.selection_background", "value", "= role.global.accent"),
    ("ios-13", "defaults.selection_background", "source", f"{CITE}:2"),
    ("ios-13", "button.border.color", "rule", "[uniform] button.border.color = defaults.border.color"),
    ("ios-13", "button.border.corner_radius", "rule",
     "[border_inheritance] button.border.corner_radius: corner_radius.inherits_from -> defaults.border.corner_radius"),
    ("ios-13", "button.border.corner_radius", "source", f"{CITE}:34"),
    ("ios-13", "list.border.corner_radius", "rule",
     "[border_inheritance] list.border.corner_radius: corner_radius_lg_widgets -> defaults.border.corner_radius_lg"),
    ("ios-13", "list.border.corner_radius", "source", f"{CITE}:35"),
    ("ios-13", "list.border.color", "target", ""),
    ("ios-13", "button.font.family", "rule",
     "[font_inheritance] button.font.family: inherits_from -> defaults.font.family"),
    ("ios-13", "button.font.family", "source", f"{CITE}:39"),
    ("ios-13", "button.font.size", "target", "button.label.text.font.size"),
    ("ios-13", "button.font.color", "target", "button.label.text.fill"),
    ("ios-13", "link.font.color", "rule", "[uniform] link.font.color = defaults.link_color"),
    ("ios-13", "link.font.size", "rule", "[font_inheritance] link.font.size: inherits_from -> defaults.font.size"),
    ("ios-13", "text_scale.caption.size", "rule",
     "[text_scale_inheritance] text_scale.caption.size: size.inherits_from -> defaults.font.size"),
    ("ios-13", "text_scale.caption.line_height", "value", ""),
    ("ios-13", "text_scale.caption.line_height", "source", f"{CITE}:45"),
]


def self_test() -> int:
    failures = []
    with tempfile.TemporaryDirectory() as tmp:
        root = Path(tmp)
        (root / DOCS).mkdir(parents=True)
        (root / RULES).write_text(SELF_TEST_RULES, encoding="utf-8")
        (root / CATALOGUE).write_text(SELF_TEST_CATALOGUE, encoding="utf-8")
        (root / TODAY_PRESETS).mkdir(parents=True)
        for p in PRESETS:
            (root / TODAY_PRESETS / f"{p}.toml").write_text(SELF_TEST_PRESETS.get(p, ""), encoding="utf-8")
        text, counts = build(root)
        entries = {(b["preset"], b["field"]): b for b in tomllib.loads(text)["base"]}
        for preset, field, key, want in SELF_TEST_WANT:
            got = entries.get((preset, field))
            if key is None:
                if got is not None:
                    failures.append(f"{preset} {field}: an entry, want none")
            elif (got or {}).get(key) != want:
                failures.append(f"{preset} {field} `{key}`: got {(got or {}).get(key)!r}, want {want!r}")
        if counts["adwaita"] != (23, 9, 5) or counts["ios-13"] != (27, 11, 7):
            failures.append(f"counts: got {counts}")
        quoted = entries[("kde-breeze", "button.background_color")]["mapping"]
        if quoted != ["spec-widgets.md:10 `background_color` → `body.fill`",
                      "spec-widgets.md:57 `defaults.background_color` -> `role.window.background`"]:
            failures.append(f"mapping: got {quoted}")
        if not {k for k in entries if k[1] == "defaults.selection_background"}:
            failures.append("[defaults_internal] not converted")
        if {k for k in entries if k[0] == "ios-13" and k[1] == "link.font.color"} != {("ios-13", "link.font.color")}:
            failures.append("link.font.color: not one entry")
        order = [b["field"] for b in tomllib.loads(text)["base"] if b["preset"] == "ios-13"]
        if order[:2] != ["defaults.selection_background", "button.border.corner_radius"] or order[-1] != \
                "text_scale.caption.line_height":
            failures.append(f"order: got {order}")
        if write(root, text, check=False) != [f"{STAGED.as_posix()}: missing"] or write(root, text, check=True):
            failures.append("write, then check: not clean")
        (root / STAGED).write_text(text.replace("role.window.background", "role.view.background"), encoding="utf-8")
        if write(root, text, check=True) != [f"{STAGED.as_posix()}: differs from a fresh run"]:
            failures.append("check after an edit: no difference")
        (root / RULES).write_text(SELF_TEST_RULES.replace('fallback = "defaults.text_color"\n', ""),
                                  encoding="utf-8")
        try:
            build(root)
            failures.append("a [per_platform] table without fallback: accepted")
        except InputError:
            pass
    for failure in failures:
        print(f"self-test FAIL: {failure}", file=sys.stderr)
    if not failures:
        print(f"self-test ok ({len(SELF_TEST_WANT)} fields)")
    return 1 if failures else 0


def write(root: Path, text: str, check: bool) -> list[str]:
    p = root / STAGED
    old = p.read_text(encoding="utf-8") if p.is_file() else None
    if old == text:
        return []
    if not check:
        p.parent.mkdir(parents=True, exist_ok=True)
        p.write_text(text, encoding="utf-8")
    return [f"{STAGED.as_posix()}: " + ("missing" if old is None else "differs from a fresh run")]


def main(argv: list[str]) -> int:
    args = argv[1:]
    if args == ["--self-test"]:
        return self_test()
    if args not in ([], ["--check"]):
        print(__doc__, file=sys.stderr)
        return 2
    check = args == ["--check"]
    root = Path(".")
    try:
        text, counts = build(root)
        diffs = write(root, text, check)
    except (OSError, UnicodeDecodeError, InputError) as err:
        print(f"error: {err}", file=sys.stderr)
        return 2
    for preset, (n, with_target, with_value) in counts.items():
        print(f"{preset}: {n} inherited fields, {with_target} with a target, {n - with_target} without; "
              f"{with_value} with a value")
    if check:
        for d in diffs:
            print(d)
        return 1 if diffs else 0
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
