#!/usr/bin/env python3
"""Check every name the v0.7.1 specification's catalogue and traceability use.

It reads the core (S…), builds the vocabulary it defines (state flags, shared flags, conditions, colour
roles, tokens, element data, part-style fields), reads the widget catalogue and builds, per typed widget
and keyed component, the parts, widget values, variants and flags its entry lists (the names in backticks
inside its **Parts**, **Widget values**, **Variants** and **Flags** items). Then it extracts every
reference from the trace targets and from the catalogue's own backtick spans and reports each name
neither defines, plus notation the specification forbids.

    python3 scripts/check_spec_names.py            # report, exit 1 if anything is unknown
    python3 scripts/check_spec_names.py --index    # also write index.json (the catalogue's names per widget)
    python3 scripts/check_spec_names.py --only trace-macos   # one file
    python3 scripts/check_spec_names.py --cat cat-assembled.md --part cat/cat-01-A.md   # index the whole catalogue, check one chunk
    python3 scripts/check_spec_names.py --cat copy/widgets.md --traces copy/   # check copies (trace-<platform>.md in copy/)
    python3 scripts/check_spec_names.py --self-test   # the rules on built-in fixtures

Output: one line per problem, tab-separated: file, line, kind, name, context.
"""
import json
import re
import sys
from collections import defaultdict
from pathlib import Path

from spec_catalogue import catalogue_index

DOCS = Path("docs")  # run from the repository root
CORE = DOCS / "todo_v0.7.1_model-completeness-spec.md"
CAT = DOCS / "todo_v0.7.1_model-completeness-spec-widgets.md"
PLATS = ["macos", "windows", "kde", "gnome", "material", "ios"]
TRACES = {f"trace-{p}": DOCS / f"todo_v0.7.1_model-completeness-spec-trace-{p}.md" for p in PLATS}
HERE = Path(__file__).resolve().parent

IDENT = r"[a-z_][a-z0-9_]*"

# The brief's decided shared-flag list (section 1); the core's SharedFlag is updated from it at the end. Set to None
# once the core carries it.
# The core's SharedFlag (S7.2) plus the flags the catalogue's *Shared flags* table added after cebfe6df, which the
# core's enum must mirror (listed as a needed core change). Set to None once the core carries them.
SHARED_OVERRIDE = ["empty", "in_fill", "today", "other_month", "current", "multiline", "filled", "half", "preview",
                   "placeholder", "pulling", "refreshing", "placed_above", "placed_below", "route_active", "editing",
                   "swiped", "context_menu_open", "sorted", "descending", "overwrite", "composing", "minimized",
                   "text_selected",
                   # the sixth check: the catalogue's drop-operation flags (a needed core change: SharedFlag)
                   "drop_copy", "drop_move"]


def snake(name: str) -> str:
    s = re.sub(r"(?<=[a-z0-9])([A-Z])", r"_\1", name)
    s = re.sub(r"(?<=[A-Z])([A-Z][a-z])", r"_\1", s)
    return s.lower()


# ---------------------------------------------------------------------------------------------- core

def strip_comments(code: str) -> str:
    return re.sub(r"//[^\n]*", "", code)


def rust_blocks(text: str) -> str:
    return "\n".join(m.group(1) for m in re.finditer(r"```rust\n(.*?)```", text, re.S))


def body_of(code: str, kind: str, name: str) -> str:
    m = re.search(rf"pub {kind} {name}\b[^{{]*\{{", code)
    if not m:
        raise SystemExit(f"core: no {kind} {name}")
    i, depth = m.end(), 1
    while depth:
        c = code[i]
        depth += c == "{"
        depth -= c == "}"
        i += 1
    return code[m.end(): i - 1]


def enum_variants(code: str, name: str) -> list[str]:
    b = body_of(code, "enum", name)
    out, depth, tok = [], 0, ""
    for c in b:
        if c in "({<":
            if depth == 0 and tok.strip():
                out.append(tok.strip())
                tok = ""
            depth += 1
        elif c in ")}>":
            depth -= 1
        elif depth == 0:
            if c == ",":
                if tok.strip():
                    out.append(tok.strip())
                tok = ""
            else:
                tok += c
    if tok.strip():
        out.append(tok.strip())
    return [v for v in out if re.fullmatch(r"[A-Z]\w*", v)]


def struct_fields(code: str, name: str) -> list[str]:
    return re.findall(r"pub (\w+):", body_of(code, "struct", name))


def core_vocab(text=None) -> dict:
    text = CORE.read_text() if text is None else text
    code = strip_comments(rust_blocks(text))
    v = {}
    sf = enum_variants(code, "StateFlag")
    v["state_flags"] = {snake(x) for x in sf if x not in ("Shared", "Widget")}
    v["shared_flags"] = {snake(x) for x in enum_variants(code, "SharedFlag")}
    if SHARED_OVERRIDE:
        v["shared_flags"] = set(SHARED_OVERRIDE)
    v["conditions"] = {snake(x) for x in enum_variants(code, "Condition")}
    v["schemes"] = {snake(x) for x in enum_variants(code, "Scheme")}
    v["input_kinds"] = {snake(x) for x in enum_variants(code, "InputKind")}
    v["idioms"] = {snake(x) for x in enum_variants(code, "Idiom")}
    v["postures"] = {snake(x) for x in enum_variants(code, "Posture")}
    v["sizes"] = {snake(x) for x in enum_variants(code, "ControlSize")}
    v["densities"] = {snake(x) for x in enum_variants(code, "Density") if x != "Keyed"}
    v["size_classes"] = {snake(x) for x in enum_variants(code, "SizeClass")}
    v["surface_sets"] = {snake(x) for x in enum_variants(code, "SurfaceSet")}
    v["surface_roles"] = set(struct_fields(code, "SurfaceRoles"))
    v["global_roles"] = set(struct_fields(code, "GlobalRoles"))
    v["tokens"] = set(struct_fields(code, "Tokens"))
    v["part_fields"] = set(struct_fields(code, "PartStyle"))
    v["element_data"] = set(struct_fields(code, "ElementData"))
    # the expression grammar's element operands (S2.2), font metrics included (char_width, font_x_height, …)
    gm = re.search(r'"element\." data(.*?)\| "part\."', text, re.S)
    if gm:
        v["element_data"] |= set(re.findall(r"\b([a-z][a-z_]+)\b", gm.group(1))) - {"the", "s", "part", "font",
                                                                               "measured", "through", "element", "data"}
    v["text_roles"] = set(struct_fields(code, "TextRoles"))
    v["text_tokens"] = set(struct_fields(code, "TextTokens"))   # roles, keyed, fallbacks, rendering, scaling, font_dpi
    v["text_style"] = set(struct_fields(code, "TextStyle"))
    v["font"] = set(struct_fields(code, "Font"))
    v["icon_style"] = set(struct_fields(code, "IconStyle"))
    v["glyph"] = set(struct_fields(code, "Glyph"))   # stroke_width, line_cap, line_join went into GlyphStroke (6c857ce6)
    v["glyph_stroke"] = set(struct_fields(code, "GlyphStroke"))
    v["focus"] = set(struct_fields(code, "FocusIndicator"))
    v["corners"] = set(struct_fields(code, "Corners"))
    v["stroke"] = set(struct_fields(code, "Stroke")) | {"width", "paint", "dash", "snap"}
    v["transform"] = set(struct_fields(code, "Transform"))
    v["caret"] = set(struct_fields(code, "Caret"))
    v["behaviour"] = set(struct_fields(code, "Behavior"))
    v["scroll"] = set(struct_fields(code, "ScrollPhysics"))
    v["touch"] = set(struct_fields(code, "TouchBehavior"))
    v["fallbacks"] = set(struct_fields(code, "FontFallbacks"))
    v["rendering"] = set(struct_fields(code, "TextRendering"))
    v["pointer"] = {"theme", "size", "color", "drag_cursors"}
    v["kde"] = set(struct_fields(code, "KdeTokens"))
    v["kde_effects"] = set(struct_fields(code, "KdeEffects"))
    v["material"] = set(struct_fields(code, "MaterialScheme"))
    v["material_paint"] = set(struct_fields(code, "Material"))
    v["status_names"] = {"success", "warning", "error", "info", "neutral", "destructive"}
    v["status_fields"] = {"fill", "foreground", "standalone", "background", "background_solid"}
    v["placements"] = {x for x in enum_variants(code, "PlacementStrategy")}
    v["presentation_events"] = {snake(x) for x in enum_variants(code, "PresentationEvent") if x != "Keyed"}
    v["presentation_kinds"] = {x for x in enum_variants(code, "PresentationKind")}
    v["focus_visible_when"] = {x for x in enum_variants(code, "FocusVisibleWhen")}
    return v


# ----------------------------------------------------------------------------------------- checking

def norm_value(n: str) -> str:
    return re.sub(r"_(px|ms|deg)$", "", n)


class Checker:
    def __init__(self, cat=CAT, core_text=None, cat_text=None):
        self.v = core_vocab(core_text)
        self.cat_text = Path(cat).read_text() if cat_text is None else cat_text
        self.w = catalogue_index(self.cat_text)
        self.all_variants = set().union(*(x["variants"] for x in self.w.values()))
        self.all_flags = set().union(*(x["flags"] for x in self.w.values()))
        self.problems = []
        self.unknown_widgets = []
        self.conds = self.v["conditions"]

    def add(self, src, ln, kind, name, ctx):
        self.problems.append((src, ln, kind, name, ctx.strip()[:160]))

    # -- vocabulary checks
    def flag_ok(self, f, widgets):
        f = f.strip().lstrip("!").strip()
        if not f:
            return True
        if f.startswith("@"):
            if not widgets:
                return f in self.all_flags
            return any(f in self.w[w]["flags"] for w in widgets if w in self.w) or f in self.all_flags
        return f in self.v["state_flags"] or f in self.v["shared_flags"]

    def variant_ok(self, var, widgets):
        var = var.strip().lstrip("!").strip()
        if not var:
            return True
        pool = set()
        for w in widgets:
            if w in self.w:
                pool |= self.w[w]["variants"]
        if var in pool:
            return True
        return var in self.all_variants and not widgets

    def cond_ok(self, c):
        c = c.strip().lstrip("!")
        if c in self.conds:
            return True
        if re.fullmatch(r"container_size:[a-z_]+:(?:[a-z0-9_]+|<key>)", c):
            return True
        if ":" in c:
            head, arg = c.split(":", 1)
            if head == "scheme":
                return arg in self.v["schemes"]
            if head == "input":
                return arg in self.v["input_kinds"]
            if head == "idiom":
                return arg in self.v["idioms"]
            if head == "posture":
                return arg in self.v["postures"]
            if head == "device_model":
                return bool(re.fullmatch(r"[a-z0-9_]+|<key>", arg))
            if head in ("flavor", "preset", "system_setting"):
                return bool(re.fullmatch(r"[a-z0-9_-]+", arg))
            if head == "tiled":
                return all(x in ("top", "right", "bottom", "left") for x in arg.split(","))
            return False
        if re.fullmatch(r"container_size:[a-z_]+:(?:[a-z0-9_]+|<key>)", c):
            return True
        return False

    def role_ok(self, path):
        seg = path.split(".")
        if seg[0].startswith("@"):
            return True
        if seg[0] == "global":
            if len(seg) < 2 or seg[1] not in self.v["global_roles"]:
                return False
            if seg[1] == "status":
                return len(seg) == 2 or (seg[2] in self.v["status_names"] or seg[2] in ("*", "<s>")) and (
                    len(seg) < 4 or seg[3] in self.v["status_fields"] or seg[3] == "*")
            return True
        if seg[0] in self.v["surface_sets"] or seg[0] in ("<set>", "*"):
            if len(seg) < 2:
                return True
            if seg[1] == "disabled":
                return len(seg) < 3 or seg[2] in self.v["surface_roles"] or seg[2] == "*"
            return seg[1] in self.v["surface_roles"] or seg[1] in ("*", "<role>")
        return False

    def token_ok(self, path):
        seg = [norm_value(s) for s in path.split(".") if s]
        if not seg:
            return False
        t = seg[0]
        if t not in self.v["tokens"]:
            return False
        if len(seg) == 1:
            return True
        s1 = seg[1]
        if s1 in ("*", "<key>", "x") or s1.startswith("<"):
            return True
        if t == "motion":
            return s1 in ("durations", "timings", "speed_factor")
        if t == "text":
            if s1 == "roles":
                return len(seg) < 3 or seg[2] in self.v["text_roles"] or seg[2] == "*"
            if s1 == "fallbacks":
                return len(seg) < 3 or seg[2] in self.v["fallbacks"]
            if s1 == "rendering":
                return len(seg) < 3 or seg[2] in self.v["rendering"]
            if s1 in ("scaling", "font_dpi"):
                return len(seg) == 2 and s1 in self.v["text_tokens"]
            return s1 == "keyed"
        if t == "icons":
            if s1 == "sizes":
                return len(seg) < 3 or seg[2] in ("toolbar", "small", "large", "dialog", "panel", "keyed", "*") \
                    or seg[2].startswith("<")
            return s1 in ("set", "theme", "theme_fallback", "mirroring")
        if t == "focus":
            return s1 in self.v["focus"]
        if t == "caret":
            return s1 in self.v["caret"]
        if t == "behavior":
            if s1 == "scroll":
                return len(seg) < 3 or seg[2] in self.v["scroll"]
            if s1 == "touch":
                return len(seg) < 3 or seg[2] in self.v["touch"]
            return s1 in self.v["behaviour"]
        if t == "pointer":
            return s1 in self.v["pointer"]
        if t == "kde":
            if s1 in ("disabled", "inactive"):
                return len(seg) < 3 or seg[2] in self.v["kde_effects"]
            return s1 in self.v["kde"]
        if t == "material":
            return s1 in self.v["material"]
        return True  # params.<group>.<key>, elevation.<key>, theme_change, size_classes, icon_mirroring

    def field_ok(self, segs):
        """A part-style field path after the part: fill, text.font.size, …"""
        if not segs:
            return True
        f = norm_value(segs[0])
        pf = self.v["part_fields"] | {"min_width", "min_height", "max_width", "max_height"}
        if f in ("*",) or f.startswith("<"):
            return True
        if f.endswith(("_px", "_ms")):
            f = norm_value(f)
        if f == "append":  # fill.append
            return True
        if f not in pf:
            return False
        if len(segs) == 1:
            return True
        s = segs[1]
        if s in ("append", "*") or s.startswith("<"):
            return True
        sub = {"text": self.v["text_style"], "icon": self.v["icon_style"], "glyph": self.v["glyph"],
               "focus": self.v["focus"], "corners": self.v["corners"], "stroke": self.v["stroke"],
               "transform": self.v["transform"]}.get(f)
        if sub is None:
            return True
        if s not in sub:
            return False
        if f == "text" and s == "font" and len(segs) > 2:
            return segs[2] in self.v["font"]
        if f == "glyph" and s == "stroke" and len(segs) > 2 and not segs[2].startswith(("<", "*")):
            return segs[2] in self.v["glyph_stroke"]
        return True

    # -- extraction
    def widget_paths(self, s):
        """(widget, [segments]) for each dotted path starting at a known widget or component."""
        out = []
        for m in re.finditer(r"(?<![\w.@:<>\-/])(component\.[a-z_][a-z0-9_]*|[a-z_][a-z0-9_]*)"
                             r"((?:\.(?:\{[^}]*\}|\*|[a-z_][a-z0-9_]*))+)", s):
            w = m.group(1)
            if w not in self.w:
                if w.startswith("component.") or w in OLD_WIDGETS:
                    self.unknown_widgets.append(w)
                continue
            out.append((w, m.group(2), m.start()))
        return out

    def expand(self, segs: str):
        """`.a.{b,c}.d` -> [['a','b','d'], ['a','c','d']] (braces may hold dotted paths)."""
        parts = re.findall(r"\.(\{[^}]*\}|\*|[a-z_][a-z0-9_]*)", segs)
        paths = [[]]
        for p in parts:
            if p.startswith("{"):
                alts = [a.strip() for a in p[1:-1].split(",") if a.strip()]
                paths = [x + a.split(".") for x in paths for a in alts]
            else:
                paths = [x + [p] for x in paths]
        return paths

    def check_path(self, src, ln, w, segs, ctx):
        info = self.w[w]
        for path in self.expand(segs):
            if not path:
                continue
            p0 = path[0]
            if p0 == "*":
                continue
            if p0 == "values":
                if len(path) < 2 or path[1] == "*":
                    continue
                val = norm_value(path[1])
                if val not in {norm_value(x) for x in info["values"]} and path[1] != "presentations":
                    self.add(src, ln, "value", f"{w}.values.{path[1]}", ctx)
                elif path[1] == "presentations" and "presentations" not in info["values"]:
                    self.add(src, ln, "value", f"{w}.values.presentations", ctx)
                continue
            if w == "input" and p0 in ("accent", "accent_ramp", "highlight", "kde_scheme", "inactive", "seed", "tint"):
                continue  # an S9 input path
            if p0 not in info["parts"]:
                self.add(src, ln, "part", f"{w}.{p0}", ctx)
                continue
            if not self.field_ok(path[1:]):
                self.add(src, ln, "field", f"{w}.{'.'.join(path)}", ctx)

    def check_string(self, src, ln, s, ctx=None, kind=None):
        ctx = ctx if ctx is not None else s
        widgets = []
        self.unknown_widgets = []
        for w, segs, _ in self.widget_paths(s):
            widgets.append(w)
            if kind == "input" and w == "input":
                continue
            self.check_path(src, ln, w, segs, ctx)
        for w in self.unknown_widgets:
            self.add(src, ln, "widget", w, ctx)
        for m in re.finditer(r"(?:context|not_context)\s+(?:any\s+)?(component\.[a-z_][a-z0-9_]*|[a-z_][a-z0-9_]*)", s):
            nm = m.group(1)
            if (nm.startswith("component.") or nm in OLD_WIDGETS) and nm not in self.w:
                self.add(src, ln, "widget", nm, ctx)
        # bare widget names used as context or targets
        for m in re.finditer(r"(?:context|not_context)\s+(?:any\s+|nearest\s*\{[^}]*\}\s*=\s*)?\(?\s*"
                             r"(component\.[a-z_][a-z0-9_]*|[a-z_][a-z0-9_]*)", s):
            nm = m.group(1)
            if nm in self.w:
                widgets.append(nm)
        any_widget = bool(re.search(r"context any variant", s))
        mm = re.match(r"\s*(component\.[a-z_][a-z0-9_]*|[a-z_][a-z0-9_]*)\b", s)
        if mm and mm.group(1) in self.w:
            widgets.append(mm.group(1))
            # `widget.<part>.<field>` (S2.2) names a part of the target's own widget
            for mw in re.finditer(r"(?<![\w.])widget\.([a-z_][a-z0-9_]*)\.", s):
                if mw.group(1) not in self.w[mm.group(1)]["parts"] and mw.group(1) not in ("<part>", "part"):
                    self.add(src, ln, "widget-ref", f"{mm.group(1)}: widget.{mw.group(1)}", ctx)
        # roles, tokens, surface, element, part
        for m in re.finditer(r"(?<![\w.])role\.((?:@?[a-z_*<>][a-z0-9_*<>]*)(?:\.[a-z_*<>][a-z0-9_*<>]*)*)", s):
            if not self.role_ok(m.group(1)):
                self.add(src, ln, "role", "role." + m.group(1), ctx)
        for m in re.finditer(r"(?<![\w.])token\.((?:[a-z_*<>][a-z0-9_*<>]*)(?:\.(?:\{[^}]*\}|[a-z_*<>][a-z0-9_*<>]*))*)", s):
            for path in self.expand("." + m.group(1)):
                if not self.token_ok(".".join(path)):
                    self.add(src, ln, "token", "token." + ".".join(path), ctx)
        for m in re.finditer(r"(?<![\w.])surface\.([a-z_]+)", s):
            if m.group(1) not in self.v["surface_roles"]:
                self.add(src, ln, "surface", "surface." + m.group(1), ctx)
        for m in re.finditer(r"(?<![\w.])element\.([a-z_]+)", s):
            if m.group(1) not in self.v["element_data"]:
                self.add(src, ln, "element", "element." + m.group(1), ctx)
        # selectors
        sel = s
        for m in re.finditer(r"(?<![.\w\]])!?\{([^{}]*)\}", sel):
            body = m.group(1)
            before = sel[max(0, m.start() - 12):m.start()]
            if re.search(r"(nearest|variants|of)\s*$", before):
                continue
            if re.match(r"\s*system_setting", body):
                self.add(src, ln, "notation", "{system_setting x} (write `cond system_setting:x`)", ctx)
                continue
            if "|" in body:
                self.add(src, ln, "notation", "{a|b}", ctx)
                continue
            if re.search(r"[A-Z]", body) or "=" in body or "." in body:
                continue  # a type sketch, not a state set
            for f in body.split(","):
                f = f.strip()
                if not f or f in ("…", "...") or " " in f.strip("! "):
                    continue
                if not self.flag_ok(f, widgets):
                    self.add(src, ln, "flag", f, ctx)
        for m in re.finditer(r"(?<![\w])!?variants?\s+(\{[^}]*\}|@?[a-z_][a-z0-9_]*(?:\s*,\s*@[a-z_][a-z0-9_]*)*)", sel):
            body = m.group(1).strip("{}")
            for var in re.split(r"[,\s]+", body):
                var = var.strip()
                if not var or var in ("of", "and", "or", "per", "the", "a", "on"):
                    continue
                if not self.variant_ok(var, widgets) and not (any_widget and var in self.all_variants):
                    self.add(src, ln, "variant", var, ctx)
        for m in re.finditer(r"(?<![\w])[a-z_][a-z0-9_]*\.[a-z_][a-z0-9_]*\[([^\]]*)\]", sel):
            for var in m.group(1).split(","):
                ms = re.fullmatch(r"\s*size\s+([a-z_]+)\s*", var)
                if ms:  # w.part[size s]: the ancestor's control size (ContextMatch.sizes)
                    if ms.group(1) not in self.v["sizes"] and ms.group(1) != "s":
                        self.add(src, ln, "size", ms.group(1), ctx)
                    continue
                if var.strip() and not self.variant_ok(var.strip(), widgets):
                    self.add(src, ln, "variant", var.strip(), ctx)
        if re.search(r"container_size\(", sel):  # S11 spells it container_size:<widget>:<key>
            self.add(src, ln, "notation", "container_size(w, k) (write `container_size:w:k`)", ctx)
        for m in re.finditer(r"(?<![\w])!?cond\s+(container_size:[a-z_]+:(?:[a-z0-9_]+|<key>)|[a-z_][a-z0-9_]*(?::[a-z0-9_,-]+)?)", sel):
            if not self.cond_ok(m.group(1)):
                self.add(src, ln, "condition", m.group(1), ctx)
        if re.search(r"\bcondition[: ]", sel):
            self.add(src, ln, "notation", "condition (write `cond <name>`)", ctx)
        if re.search(r"(?<![\w])ctx\b", sel):
            self.add(src, ln, "notation", "ctx (write `context`)", ctx)
        if re.search(r"(?<![\w])not \{", sel):
            self.add(src, ln, "notation", "not {a} (write `!{a}`)", ctx)
        if re.search(r"\b(pos|position)\s+(odd|even)\b", sel):
            self.add(src, ln, "notation", "pos odd (write `pos parity odd`)", ctx)
        for m in re.finditer(r"(?<![\w])size\s+([a-z_]+)", sel):
            if m.group(1) not in self.v["sizes"] and m.group(1) not in ("s", "class", "classes", "per", "of"):
                self.add(src, ln, "size", m.group(1), ctx)
        for m in re.finditer(r"(?<![\w])[hv]?_?size_class\s+([a-z_]+)", sel):
            if m.group(1) not in self.v["size_classes"] and m.group(1) not in ("c",):
                self.add(src, ln, "size_class", m.group(1), ctx)
        for m in re.finditer(r"FocusVisibleWhen::(\w+)|focus_visible_when\s*(?:=|:)\s*`?(\w+)", s):
            nm = m.group(1) or m.group(2)
            if nm and nm[0].isupper() and nm not in self.v["focus_visible_when"]:
                self.add(src, ln, "focus_visible_when", nm, ctx)
        for m in re.finditer(r"PlacementStrategy::(\w+)", s):
            if m.group(1) not in self.v["placements"]:
                self.add(src, ln, "placement", m.group(1), ctx)

    # -- files
    def check_trace(self, key, path):
        self.check_trace_text(key, path.read_text())

    def check_trace_text(self, key, text):
        for i, line in enumerate(text.splitlines(), 1):
            if not line.startswith("| ") or line.startswith("| Row") or line.startswith("|---"):
                continue
            cells = [c.strip() for c in re.split(r"(?<!\\)\|", line.strip().strip("|"))]
            if len(cells) != 3:
                continue
            target, kind = cells[1], cells[2]
            t = target.strip("`")
            if t.startswith("-"):
                continue
            if kind == "reader" or t.startswith("reader:"):
                continue
            self.check_string(key, i, t, kind=kind)
            for old in OLD_NAMES:
                if re.search(old, t):
                    self.add(key, i, "old-name", old, t)
            if "FollowPart" in t and not re.search(FOLLOW_PART_OK, t):
                self.add(key, i, "follow-part", "FollowPart outside a focus indicator", t)
            if menu_selected_problem(t):
                self.add(key, i, "menu-selected", "{selected} on a menu item outside context combo_box (S7.2)", t)

    def check_catalogue(self, part=None):
        lines = (Path(part).read_text() if part else self.cat_text).splitlines()
        in_rename = False
        in_mba = False
        for i, line in enumerate(lines, 1):
            if line.startswith("- **may_be_absent"):
                in_mba = True
            elif line.startswith(("- **", "#")) or not line.strip():
                in_mba = False
            if in_mba:
                for m in re.finditer(r"`([^`]*(?:^|[.{,\s])(?:min|max)_(?:width|height))[^`]*`\s*\(\s*(?:`(?:adwaita|kde-breeze|macos|windows|ios|material)|all but|every preset|the value phase)", line):
                    self.add("catalogue", i, "absent-size-limit", m.group(1),
                             "a size limit with none is `auto`, never Absent (S2.11, S6.3)")
            if line.startswith("## "):
                # the rename table lists old names; the notation table spells placeholders
                in_rename = line.startswith(("## Rename table", "## Selector notation"))
            if in_rename or line.startswith("| Spelling") or "Selector notation" in line:
                continue
            for span in re.findall(r"`([^`]+)`", line):
                self.check_string("catalogue", i, span, ctx=line)
            for old in OLD_NAMES:
                if re.search(old, line):
                    self.add("catalogue", i, "old-name", old, line)
            if "FollowPart" in line and not re.search(FOLLOW_PART_OK, line):
                self.add("catalogue", i, "follow-part", "FollowPart outside a focus indicator", line)


# widgets the check pass removed or renamed (a path through one is an unknown widget)
OLD_WIDGETS = {"wrap_box", "colour_button", "colour_picker", "navigation_bar", "drag_preview", "scroll_area",
               "scroll_edge_effect", "spin_button", "stack", "toolbar_customization",
               "writing_tools", "drawer", "main_window", "button_group"}

OLD_NAMES = [
    r"selection_text_inactive", r"selection_background_inactive", r"role\.global\.selection_text\b(?!_)",
    r"token\.colour_effects", r"\bcolour_effects\b", r"\bTextSlot\b", r"stroke_ratio", r"IconStyle\.variant\b",
    r"\bicon\.variant\b", r"symbol_variant\b", r"KeyboardOnly|AfterKeyboard|WithFullKeyboardAccess",
    r"PresentationKind::None", r"TransientTouch|transient_touch", r"scrollbar\.values\.overlay\b",
    r"PlacementStrategy::Keyed", r"\btoolbar\.item\b", r"\btoolbar\.item_(?!group)", r"\bminimise\b|\bmaximise\b",
    r"role\.status\.", r"role\.scrim\b", r"tokens\.(focus|icon_sizes)", r"\bfocus_visible\b(?!_when)(?=\s*\()",
    r"\bPresentations\b", r"\bEffect\b \(CONTRA-4\)|animation Effect",
    r"may_be_absent_parts", r"\bAncestor\b", r"Element\.ancestors", r"\bcolour_roles\b",
    r"\bFlavour\b rule", r"CaptionButton \{",
    # removed by the core's third check (cebfe6df)
    r"\bkde_(?:intensity|color|contrast)\b", r"token\.icon_sizes", r"token\.icon_mirroring", r"\bicon_mirroring\b",
    r"FocusCorners", r"hides_while_selecting", r"overwrite_shape", r"\bblock_text\b", r"@overwrite\b", r"@composing\b",
    r"\[preset\.", r"PropertyGroup::Focus",
    # removed by the core's fourth check (1e4d15a2)
    r"\bFocusRing\b", r"FocusIndicator\.secondary", r"token\.focus\.secondary", r"\.focus\.secondary",
    r"Timing::Keyed", r"\bKeyedCurve\b", r"secondary (?:focus )?ring",
    r"sf_tracking\(font_size, design\)", r"name_index\([^)]*@", r"name_index\([^)]*<method>",
    r"kde_effects\((?:[^(),]|\([^()]*\))*,(?:[^(),]|\([^()]*\))*,(?:[^(),]|\([^()]*\))*\)",
]

# removed by the core's fifth check (6c857ce6)
OLD_NAMES += [
    r"glyph\b[^`|]*?\b(?:stroke_width|line_cap|line_join)\b",      # Glyph.stroke: GlyphStroke { paint, width, cap, join }
    r"\bLineHeight\b",                                              # line_height: Value<FontSize>, `auto`
    r"Breakpoint\s*\{[^}]*\bmin_(?:width|height)\b",                # Breakpoint { measured, width, height, combine, variant }
    r"breakpoints\s*\([^)]*\bmin_(?:width|height)\b",
    r"(?:element|ElementData|Element\.data)\.display_corner_radius",  # the root's container: RootInputs (S4.6, S8)
    r"selected \(keyboard selection\)",
    r"@thumb_move\b",                                                 # a Geometry transition of the thumb (S2.7)
    r"Comparison\s*\{\s*AtLeast,\s*Below\s*\}",
]

# removed by the core's sixth check (f0854b14, 5f30ba68)
OLD_NAMES += [
    r"\bCaretShape\b",                                               # the caret parts' rules carry the shape
    r"\bcaret\.(?:\{[^}]*\b)?shape\b",                               # Caret.shape is gone
    r"\bTarget::",                                                   # RuleBody replaces target + patch
    r"kde_accent\(\s*[^(),]+,\s*[^(),]+\)",                          # kde_accent(set, key, group)
    r"\bKeyframes\b",                                                # Keyframe { at, part, timing, patch }
]

# removed by the core's seventh check (b1460c39, 0d511dc5)
OLD_NAMES += [
    r'shape\s*=\s*"?@',                                             # no keyed shapes: ShapeKind { Material(..) } in code
    r"\bkeyed shapes?\b", r"declare\.shapes\b", r"\bdeclared shapes\b", r"\bshape library\b",
    r"WinUiFlyout\(",                                                 # side and align are PlacementParams; WinUiFlyoutFull
    r"m3_role\((?:[^(),]|\([^()]*\))*,",                              # m3_role(role) reads its inputs itself
    r"Path\s*\{(?![^}]*view_box)",                                   # GlyphSource::Path { data, view_box }
    r"\bhas_arrow\b",                                                 # the arrow's absence by rules (review 7, SF-8)
]


# removed or renamed by the core's eighth check (5c7f16b7)
OLD_NAMES += [
    r"display\.corner_radius(?!_(?:\{top|top|bottom)_?(?:left|right)?)",     # four keys: corner_radius_top_left, …
    r"set_accent\(Accent\)|apply_palette\(Palette\)",               # set_accent(Some(accent)), apply_palette(Some(…))
    r"which the function applies itself",                             # kde_selection_inactive + kde_effects over it
    r"\bFramesData\b|\bFrameList\b|\bTransformData\b|\bTransformAnimation\b|EmptyFrameListError",
    r"Theme::from_(?:toml|file)",                                     # gone (S16, S19.3): Layer::from_toml
    r"High's base applies",                                           # M-4: the contrasts exclude each other
]


# removed or renamed by the core's ninth check (04b25608, core-changes-r9.txt)
OLD_NAMES += [
    r"\bbody_line_height\b",                                        # body_font_height (ascent plus descent, Q1.1)
    r"font.s line height[^|]*?(?:label|Q1\.1)|label[- ]icon[^|]*?font.s line height",   # Kirigami: the font height
    r"Radius::Full\b(?!\s*\{)",                                      # Radius::Full { maximum } (S2.4)
    r"font\.axes\b[^|;]*?\b(?:wght|wdth)\b",                         # one route: font.weight, font.width (S2.8, S12)
    r"font\.features\b[^|;]*?\bsmcp\b",                             # TextCase::SmallCaps (S12)
    r"IconStyle\.axes\b[^|;]*?\bwght\b",                             # IconStyle.weight
    r"corners\s*=\s*\"full\"|corners\s*=\s*\{\s*(?:full|concentric)\b|=\s*\"full\"",   # corners_px only (S2.4, S11)
    r"\bComponent\s*\{\s*key\b|a `Component` the preset declares",   # ComponentBase / PartRef::Component (S5.1)
    r"KColorUtils::tint\([^)]*\b0\.\d",                               # the amount is the implementation's
    r"(?:intensity|color|contrast)_amount[^|]*Fraction",              # KdeEffects amounts: Value<f32>
]


# removed or renamed by the core's tenth check (160c5b23, core-changes-r10.txt)
OLD_NAMES += [
    r"\bkey_repeat_(?:delay|interval)\b(?!_ms)(?<!stepper\.values\.key_repeat_interval)",   # Behavior.key_repeat: Option<KeyRepeat>
    r"Fade\s*\{[^}]*\brelative\b|\brelative:\s*bool",                 # Fade { start, end: Value<FadeLength>, under }
    r"from_port\(\s*file\s*\)",                                   # from_port(file, scheme)
    r"\bOutline::Corners\(Corners\)",                              # Outline::Corners(OutlineCorners)
    r"view_box:\s*Rect\b",                                          # ViewBox { x, y, width, height }
    r"FocusIndicator.s `dash`, `shadow`",                             # FocusIndicator.dash is never absent
    r"\bfrom_system_with_preset\b|\bResolutionContext\b|\bto_toml\b|\blint_toml\b|\bPresetInfo\b",
    r"\bsystem_is_dark\b|\bdetect_is_dark\b|\bprefers_reduced_motion\b|\bicon_theme_for\b",
]

# removed or renamed by the core's eleventh check (0984c4a2, core-changes-r11.txt)
OLD_NAMES += [
    r"caret\.width\s*/\s*shape|shape and blink come from the caret token|caret token[^|`]*\bshape\b",  # S4.2: the caret parts' rules
    r"\bIconRole additions?\b|\bicon[- ]role list\b|\bby its icon role\b",   # R11: UIKit's glyphs are parts' glyphs
    r"\btype_icons\b|standard_button_icons:\s*Vec<\(ActionRole,\s*Key\)>",  # S-2: icon names are strings, one home
    r"text_area\.values\.resize\b|\bResize\s*\{\s*None",                   # S-4: R1.6, Compose draws no resize handle
    r"S10 has no declaration of a typed widget's value keys",                # S10's `keys = { … }` (ADD-A107)
    r"S6\.3: `max_width`, `max_height` on every",                            # the core's marks include min_* (S2.11)
    r"preset's (?:own )?bases[^|]*?kde_accent|kde_accent\([^)]*\)[^|]*?over Breeze's scheme inputs",  # ADD-A109
    r"\b(?:Absent|absent) minimums?\b|\bminimum[^|`]{0,20}\bAbsent\b",       # minimums are `auto` (S2.11)
    r"(?:min|max)_(?:width|height)(?:_px)?`?\s*(?:=\s*\"?absent|\(absent\b|absent = true)",
]

# `{selected}` on a menu item outside `context combo_box`: a menu item's highlight is `focused` or `hover` (S7.2)
MENU_ITEM = r"\bmenu\.(?:\{[^}]*\bitem|item)"


def menu_selected_problem(t):
    if not re.search(MENU_ITEM, t):
        return False
    alts = t.split(" / ")
    tail = alts[-1]
    for alt in alts:
        for m in re.finditer(r"(?<![!\w])\{([^{}]*)\}", alt):
            flags = [x.strip() for x in m.group(1).split(",")]
            if "selected" in flags and "context combo_box" not in alt and "context combo_box" not in tail:
                return True
    return False


# `Radius::FollowPart` is a focus indicator's corner only (S2.4, S2.10)
FOLLOW_PART_OK = r"focus|FocusIndicator"


NAMED_VALUES = (r"\b(?:bold|DemiBold|SemiBold|semibold|demibold)\b|\b(?:In|Out|InOut|OutIn)(?:Quad|Cubic|Quart|Quint|Sine|Expo|Circ"
                r"|Back|Elastic|Bounce)\b|\bone grid unit\b|\bone point larger\b")


def platform_values(text):
    out = []
    for i, line in enumerate(text.splitlines(), 1):
        for m in re.finditer(r"(?<![<\w])\d+(?:\.\d+)?\s?(?:px|ms|pt|dp|sp)\b|#[0-9a-fA-F]{6}\b|" + NAMED_VALUES, line):
            out.append((i, m.group(0), line.strip()[:140]))
    return out


SELF_TEST_CORE = """
```rust
pub enum StateFlag { Hover, Pressed, Selected, Disabled, Shared(SharedFlag), Widget(Key) }
pub enum SharedFlag { Empty }
pub enum Condition { WindowInactive, ContrastHigh }
pub enum Scheme { Light, Dark }
pub enum InputKind { Touch, Mouse }
pub enum Idiom { Phone, Pad }
pub enum Posture { Tabletop }
pub enum ControlSize { Small, Regular }
pub enum Density { Compact, Keyed(Key) }
pub enum SizeClass { Compact, Regular }
pub enum SurfaceSet { View, Header }
pub struct SurfaceRoles { pub background: Paint, pub foreground: Paint }
pub struct GlobalRoles { pub accent: Paint, pub status: Status }
pub struct Tokens { pub text: TextTokens, pub kde: KdeTokens, pub motion: Motion }
pub struct PartStyle { pub fill: X, pub stroke: X, pub corners: X, pub text: X, pub focus: X, pub padding: X }
pub struct ElementData { pub progress: f32 }
pub struct TextRoles { pub body: X }
pub struct TextTokens { pub roles: X, pub keyed: X, pub scaling: X }
pub struct TextStyle { pub font: Font, pub fill: X }
pub struct Font { pub size: X, pub weight: X }
pub struct IconStyle { pub size: X }
pub struct Glyph { pub stroke: X }
pub struct GlyphStroke { pub width: X }
pub struct FocusIndicator { pub width: X, pub corners: X }
pub struct Corners { pub radius: X }
pub struct Stroke { pub top: X }
pub struct Transform { pub scale: X }
pub struct Caret { pub width: X }
pub struct Behavior { pub focus_visible_when: X }
pub struct ScrollPhysics { pub friction: X }
pub struct TouchBehavior { pub slop: X }
pub struct FontFallbacks { pub emoji: X }
pub struct TextRendering { pub hinting: X }
pub struct KdeTokens { pub disabled: KdeEffects, pub frame_width: X }
pub struct KdeEffects { pub intensity_amount: X }
pub struct MaterialScheme { pub primary: X }
pub struct Material { pub blur: X }
pub enum PlacementStrategy { Below, Above }
pub enum PresentationEvent { Appear, Keyed(Key) }
pub enum PresentationKind { Fade }
pub enum FocusVisibleWhen { Always, Keyboard }
```
"""

SELF_TEST_CATALOGUE = """
## Buttons and toggles

### button

- **Parts:**
  - `body` — the root: `fill`, `stroke`, `corners`, `focus`.
  - `label` — the text: `text`.
- **Widget values:** `icon_text_gap_px: Value<Px>`.
- **Variants:** typed `flat`; keyed `@osd`.
- **Flags:** keyed `@busy`.

## Overlays

### menu

- **Parts:**
  - `body` — the root: `fill`.
  - `item` *(element)* — one item: `fill`.

## Keyed components

#### component.led (kde)

- **Parts:**
  - `body` — the root: `fill`.
"""

# (string, the problems check_string reports as (kind, name)): a passing and a failing case per rule
SELF_TEST_STRINGS = [
    ("button.body.fill @ {hover}", []),
    ("button.values.icon_text_gap", []),
    ("button.values.nope", [("value", "button.values.nope")]),
    ("button.label.text", []),
    ("button.nope.fill", [("part", "button.nope")]),
    ("button.label.text.font.size", []),
    ("button.body.nope", [("field", "button.body.nope")]),
    ("button.label.text.font.nope", [("field", "button.label.text.font.nope")]),
    ("component.led.body.fill", []),
    ("component.ghost.body.fill", [("widget", "component.ghost")]),
    ("wrap_box.body.fill", [("widget", "wrap_box")]),
    ("button.body.fill @ context component.led.body", []),
    ("button.body.fill @ context component.ghost", [("widget", "component.ghost")]),
    ("button.body.corners = widget.body.corners", []),
    ("button.body.corners = widget.nope.corners", [("widget-ref", "button: widget.nope")]),
    ("role.view.background role.global.status.error.fill", []),
    ("role.view.nope", [("role", "role.view.nope")]),
    ("role.nope", [("role", "role.nope")]),
    ("token.text.roles.body token.kde.disabled.intensity_amount", []),
    ("token.nope", [("token", "token.nope")]),
    ("token.text.roles.nope", [("token", "token.text.roles.nope")]),
    ("surface.background", []),
    ("surface.nope", [("surface", "surface.nope")]),
    ("element.progress", []),
    ("element.nope", [("element", "element.nope")]),
    ("button.body.fill @ {@busy, empty, !disabled}", []),
    ("button.body.fill @ {nope}", [("flag", "nope")]),
    ("button.body.fill @ {@nope}", [("flag", "@nope")]),
    ("button.body.fill @ variant flat", []),
    ("button.body.fill @ variant @osd", []),
    ("button.body.fill @ variant nope", [("variant", "nope")]),
    ("menu.item.fill @ variant flat", [("variant", "flat")]),
    ("button.body.fill @ cond scheme:dark", []),
    ("button.body.fill @ cond container_size:sidebar:small", []),
    ("button.body.fill @ cond nope", [("condition", "nope")]),
    ("button.body.fill @ cond scheme:nope", [("condition", "scheme:nope")]),
    ("button.body.fill @ size small", []),
    ("button.body.fill @ size nope", [("size", "nope")]),
    ("button.body.fill @ h_size_class compact", []),
    ("button.body.fill @ h_size_class nope", [("size_class", "nope")]),
    ("FocusVisibleWhen::Keyboard", []),
    ("FocusVisibleWhen::Nope", [("focus_visible_when", "Nope")]),
    ("PlacementStrategy::Below", []),
    ("PlacementStrategy::Nope", [("placement", "Nope")]),
    ("button.body.fill @ cond system_setting:reduce_motion", []),
    ("button.body.fill @ {system_setting reduce_motion}",
     [("notation", "{system_setting x} (write `cond system_setting:x`)")]),
    ("button.body.fill @ {hover|pressed}", [("notation", "{a|b}")]),
    ("button.body.fill @ container_size(sidebar, small)",
     [("notation", "container_size(w, k) (write `container_size:w:k`)")]),
    ("button.body.fill @ condition window_inactive", [("notation", "condition (write `cond <name>`)")]),
    ("button.body.fill @ ctx menu", [("notation", "ctx (write `context`)")]),
    ("button.body.fill @ !{hover}", []),
    ("button.body.fill @ not {hover}", [("notation", "not {a} (write `!{a}`)")]),
    ("button.body.fill @ pos parity odd", []),
    ("button.body.fill @ pos odd", [("notation", "pos odd (write `pos parity odd`)")]),
]

SELF_TEST_TRACE = """
| Row | Target | Kind |
|---|---|---|
| R-1 | `button.body.fill @ {hover}` | rule |
| R-2 | `button.label.text (TextSlot)` | field |
| R-3 | `button.body.corners = Radius::FollowPart` | field |
| R-4 | `button.body.focus.corners = Radius::FollowPart` | field |
| R-5 | `menu.item.fill @ {selected}` | rule |
| R-6 | `menu.item.fill @ {selected} context combo_box` | rule |
| R-7 | `reader:kde:nope.nope` | reader |
| R-8 | - (nope.nope) | out (R1.1) |
| R-9 | `button.nope` | field |
"""

SELF_TEST_BAD_CATALOGUE = """
## Notes

- **may_be_absent:** fields: `body.min_width` (`adwaita`: none), `body.fill` (`adwaita`).
- Old: `TextSlot`.
- `button.body.corners = Radius::FollowPart`.
- `button.body.focus.corners = Radius::FollowPart`.

## Rename table

| `TextSlot` | `button.nope` |
"""


def self_test() -> int:
    failures = []
    ck = Checker(core_text=SELF_TEST_CORE, cat_text=SELF_TEST_CATALOGUE)
    if sorted(ck.w) != ["button", "component.led", "menu"]:
        failures.append(f"index: got {sorted(ck.w)}")
    if ck.v["state_flags"] != {"hover", "pressed", "selected", "disabled"} or ck.v["densities"] != {"compact"}:
        failures.append(f"core vocabulary: got {ck.v['state_flags']}, {ck.v['densities']}")
    for s, want in SELF_TEST_STRINGS:
        ck.problems = []
        ck.check_string("t", 1, s)
        got = sorted((p[2], p[3]) for p in ck.problems)
        if got != sorted(want):
            failures.append(f"{s!r}: got {got}, want {sorted(want)}")
    ck.problems = []
    ck.check_catalogue()
    if ck.problems:
        failures.append(f"clean catalogue: got {ck.problems}")
    ck.problems = []
    ck.check_trace_text("trace", SELF_TEST_TRACE)
    got = [(p[1], p[2], p[3]) for p in ck.problems]
    want = [(5, "old-name", r"\bTextSlot\b"), (6, "follow-part", "FollowPart outside a focus indicator"),
            (8, "menu-selected", "{selected} on a menu item outside context combo_box (S7.2)"),
            (12, "part", "button.nope")]
    if got != want:
        failures.append(f"trace: got {got}, want {want}")
    bad = Checker(core_text=SELF_TEST_CORE, cat_text=SELF_TEST_CATALOGUE + SELF_TEST_BAD_CATALOGUE)
    bad.check_catalogue()
    got = [(p[1], p[2], p[3]) for p in bad.problems]
    n = SELF_TEST_CATALOGUE.count("\n")
    want = [(n + 4, "absent-size-limit", "body.min_width"), (n + 5, "old-name", r"\bTextSlot\b"),
            (n + 6, "follow-part", "FollowPart outside a focus indicator")]
    if got != want:
        failures.append(f"catalogue: got {got}, want {want}")
    got = [(i, v) for i, v, _ in platform_values("a 12px gap, 250 ms\nS2.4, <8px, #a1b2c3, bold, InOutCubic")]
    want = [(1, "12px"), (1, "250 ms"), (2, "#a1b2c3"), (2, "bold"), (2, "InOutCubic")]
    if got != want:
        failures.append(f"platform values: got {got}, want {want}")
    for failure in failures:
        print(f"self-test FAIL: {failure}", file=sys.stderr)
    if not failures:
        print("self-test ok")
    return 1 if failures else 0


def main():
    args = sys.argv[1:]
    if args == ["--self-test"]:
        sys.exit(self_test())
    cat = CAT
    if "--cat" in args:
        cat = Path(args[args.index("--cat") + 1])
    if "--traces" in args:   # a folder holding trace-<platform>.md copies; the others are read from docs
        tdir = Path(args[args.index("--traces") + 1])
        for key in list(TRACES):
            if (tdir / f"{key}.md").exists():
                TRACES[key] = tdir / f"{key}.md"
    ck = Checker(cat)
    only = None
    if "--only" in args:
        only = args[args.index("--only") + 1]
    part = None
    if "--part" in args:
        part = args[args.index("--part") + 1]
        only = "catalogue"
    if not only or only == "catalogue":
        ck.check_catalogue(part)
    for key, path in TRACES.items():
        if not only or only == key:
            ck.check_trace(key, path)
    if "--index" in args:
        idx = {w: {k: sorted(v) if isinstance(v, set) else v for k, v in d.items()} for w, d in ck.w.items()}
        (HERE / "index.json").write_text(json.dumps(idx, indent=1))
        vocab = {k: sorted(v) for k, v in ck.v.items()}
        (HERE / "vocab.json").write_text(json.dumps(vocab, indent=1))
    for p in ck.problems:
        print("\t".join(str(x) for x in p))
    counts = defaultdict(int)
    for p in ck.problems:
        counts[(p[0], p[2])] += 1
    print(f"# {len(ck.problems)} unknown names or notation problems", file=sys.stderr)
    for k in sorted(counts):
        print(f"#   {k[0]:16} {k[1]:20} {counts[k]}", file=sys.stderr)
    pv = []
    files = [("catalogue", Path(part) if part else cat)] + list(TRACES.items())
    for key, path in files:
        if only and key != only:
            continue
        for i, val, ctx in platform_values(path.read_text()):
            pv.append((key, i, val, ctx))
    for x in pv:
        print("VALUE\t" + "\t".join(str(y) for y in x))
    print(f"# {len(pv)} platform values", file=sys.stderr)
    sys.exit(1 if ck.problems or pv else 0)


if __name__ == "__main__":
    main()
