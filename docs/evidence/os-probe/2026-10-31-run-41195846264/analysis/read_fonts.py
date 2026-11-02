#!/usr/bin/env python3
"""Reads the macOS fonts of the os-probe passes P1 (font descriptors) and P2 (line heights) as the facts' values:
each font's `font.style`, `font.axes`, `font.features` and its `line_height`, where the probe's reads decide them.
The facts lane wrote it (2026-11-02, spec S17 *`evidence`*: an analysis output for a value computed from read values).

Run from the repository root (Python 3.11+, the standard library only):

    python3 docs/evidence/os-probe/<run>/analysis/read_fonts.py docs/evidence/os-probe/<run>/os-probe-macos-26

It reads, under that image folder, `probe.toml`, `reduce-transparency-off/probe.toml` and
`increase-contrast-on/probe.toml` (`[macos.fonts]`: the text styles, their emphasized fonts and the named fonts, each
with its `descriptor` table; `[macos.line_height.fonts]`: the same fonts and the system font per control size, plain)
and `button-pass/probe.toml` (`[macos.button_pass.sheets.<sheet>.buttons.<button>.font]`, each with its `descriptor`).
The probe's methods are `[macos.fonts] descriptor_method` and `[macos.line_height] method`. It writes `fonts.toml`
into the image folder: per pass (`main`, `reduce_transparency_off`, `increase_contrast_on`, `button_pass`) and font,
under the font's own table path, the keys below, each written only where its rule holds (no key: not decided here).

- `style = "normal"`: the descriptor's `italicAngle` is 0, its `NSFontSlantTrait` is 0, and `symbolicTraits_set`
  holds no `TraitItalic` (NSFontDescriptor.h's italic trait).
- `axes`, a table: `opsz = "follows_size"` where `kCTFontOpticalSizeAttribute` is `"auto"` (Core Text sizes the
  optical axis to the point size) and `CTFontCopyVariation`'s `opsz` (tag 1869640570) equals the point size clamped
  to the `opsz` axis's minimum and maximum (`CTFontCopyVariationAxes`) at the font's size and at twice it
  (`at_second_size`), within 0.01; the variation holding no axis but `opsz` and `wght` (2003265652; the weight is
  `font.weight`'s route and never an axis, spec S2.8, S12). The table is written only where every axis of the
  variation is so accounted for.
- `features`, an empty table: the descriptor's `NSFontFeatureSettingsAttribute` is unavailable (the OS answered no
  attribute) and `CTFontCopyFeatureSettings` is unavailable (NULL): no feature beyond the font's defaults is set.
- `line_height` (line heights, `[macos.line_height.fonts]`, plain fonts only): the laid-out line fragment heights of
  "Ag\\nAg" (`laid_out.first_line_height`, `second_line_height`) and the advance between them (`line_advance`)
  all equal. Where they equal the font's own line spacing, `ascender - descender + leading` of the same table, within
  0.001, `line_height = "auto"` (spec S2.8: the font's own line spacing); otherwise the length in points (a logical
  pixel on AppKit, spec S17), `line_height = <n>`, the laid-out value.
- `button_pass` fonts carry `style`, `axes` and `features` as above, and `line_height` where the line-height pass
  read a font with the same `fontName` and `pointSize` (the control size's system font): that font's
  `line_height`, with `line_height_from` naming its table.

`input_image` names every file read, as repository paths.
"""

import sys
import tomllib
from pathlib import Path

OPSZ = "1869640570"
WGHT = "2003265652"
PASSES = (("main", "probe.toml"), ("reduce_transparency_off", "reduce-transparency-off/probe.toml"),
          ("increase_contrast_on", "increase-contrast-on/probe.toml"))


def read_descriptor(font: dict) -> dict:
    out = {}
    d = font.get("descriptor")
    if not isinstance(d, dict):
        return out
    traits = d.get("symbolicTraits_set", [])
    if d.get("italicAngle") == 0.0 and d.get("NSFontSlantTrait") == 0.0 and "TraitItalic" not in traits:
        out["style"] = "normal"
    var = d.get("CTFontCopyVariation")
    second = d.get("at_second_size", {})
    var2 = second.get("CTFontCopyVariation") if isinstance(second, dict) else None
    axes = {a.get("NSCTVariationAxisIdentifier"): a for a in d.get("CTFontCopyVariationAxes", []) if isinstance(a, dict)}
    opsz_axis = axes.get(float(OPSZ))
    if (d.get("kCTFontOpticalSizeAttribute") == "auto" and isinstance(var, dict) and isinstance(var2, dict)
            and opsz_axis is not None and set(var) <= {OPSZ, WGHT} and set(var2) <= {OPSZ, WGHT}
            and OPSZ in var and OPSZ in var2):
        lo, hi = opsz_axis["NSCTVariationAxisMinimumValue"], opsz_axis["NSCTVariationAxisMaximumValue"]
        ok = True
        for size, v in ((font.get("pointSize"), var), (second.get("pointSize"), var2)):
            if not isinstance(size, float) or abs(v[OPSZ] - min(hi, max(lo, size))) > 0.01:
                ok = False
        if ok:
            out["axes"] = {"opsz": "follows_size"}
    if (str(d.get("NSFontFeatureSettingsAttribute", "")).startswith("unavailable")
            and str(d.get("CTFontCopyFeatureSettings", "")).startswith("unavailable")):
        out["features"] = {}
    return out


def read_line_height(t: dict):
    lo = t.get("laid_out")
    if not isinstance(lo, dict):
        return None
    hs = [lo.get("first_line_height"), lo.get("second_line_height"), lo.get("line_advance")]
    if not all(isinstance(h, float) for h in hs) or max(hs) - min(hs) > 0.001:
        return None
    own = t["ascender"] - t["descender"] + t["leading"]
    return "auto" if abs(hs[0] - own) <= 0.001 else hs[0]


def walk_fonts(table, path, fn, out):
    """Calls fn on every font table (one with `pointSize` and `descriptor`) under `table`."""
    if isinstance(table, dict):
        if "pointSize" in table and "descriptor" in table:
            r = fn(table)
            if r:
                out[".".join(path)] = r
            return
        for k, v in table.items():
            walk_fonts(v, path + [k], fn, out)


def toml_val(v):
    if isinstance(v, bool):
        return "true" if v else "false"
    if isinstance(v, (int, float)):
        return repr(v)
    if isinstance(v, str):
        return '"' + v.replace("\\", "\\\\").replace('"', '\\"') + '"'
    if isinstance(v, list):
        return "[" + ", ".join(toml_val(x) for x in v) + "]"
    raise ValueError(v)


def key(k):
    return k if k.replace("_", "").replace("-", "").isalnum() else toml_val(k)


def main():
    if len(sys.argv) != 2:
        print(__doc__.split("\n\n")[1], file=sys.stderr)
        return 2
    image = Path(sys.argv[1])
    inputs, result = [], {}
    line_heights = {}
    for name, rel in PASSES:
        inputs.append(rel)
        macos = tomllib.loads((image / rel).read_text())["macos"]
        fonts = {}
        walk_fonts(macos["fonts"], [], read_descriptor, fonts)
        lh = {}
        for path, t in _plain(macos["line_height"]["fonts"], []):
            v = read_line_height(t)
            if v is not None:
                lh[path] = v
                if name == "main":
                    line_heights[(t["fontName"], t["pointSize"])] = (path, v)
        per = {}
        for path, r in fonts.items():
            per.setdefault(path, {}).update(r)
        for path, v in lh.items():
            per.setdefault(path, {})["line_height"] = v
        result[name] = per
    rel = "button-pass/probe.toml"
    inputs.append(rel)
    bp = tomllib.loads((image / rel).read_text())["macos"]["button_pass"]["sheets"]
    per = {}
    for sheet, s in bp.items():
        buttons = s.get("buttons") if isinstance(s, dict) else None
        if not isinstance(buttons, dict):
            continue
        for button, b in buttons.items():
            font = b.get("font") if isinstance(b, dict) else None
            if not isinstance(font, dict):
                continue
            r = read_descriptor(font)
            same = line_heights.get((font.get("fontName"), font.get("pointSize")))
            if same is not None:
                r["line_height"] = same[1]
                r["line_height_from"] = "main." + same[0]
            if r:
                per[f"{sheet}.{button}"] = r
    result["button_pass"] = per
    lines = ["# Output of analysis/read_fonts.py (its docstring is the method), file fonts.toml.",
             "# input_image: every file it reads, as a repository path (spec S17 *`evidence`*).",
             "input_image = [" + ", ".join(toml_val(f"{image.as_posix()}/{i}") for i in inputs) + "]"]
    for pname, per in result.items():
        for path, r in per.items():
            head = ".".join(["fonts", pname] + [key(p) for p in path.split(".")])
            lines.append(f"\n[{head}]")
            for k, v in r.items():
                if not isinstance(v, dict):
                    lines.append(f"{k} = {toml_val(v)}")
            for k, v in r.items():
                if isinstance(v, dict):
                    lines.append(f"\n[{head}.{k}]")
                    for kk, vv in v.items():
                        lines.append(f"{kk} = {toml_val(vv)}")
    out = image / "fonts.toml"
    out.write_text("\n".join(lines) + "\n")
    tomllib.loads(out.read_text())
    return 0


def _plain(table, path):
    """(path, table) of every `plain` line-height table under `table`, the path naming the font."""
    if isinstance(table, dict):
        if isinstance(table.get("plain"), dict) and "laid_out" in table["plain"]:
            yield ".".join(path), table["plain"]
            return
        for k, v in table.items():
            yield from _plain(v, path + [k])


if __name__ == "__main__":
    sys.exit(main())
