#!/usr/bin/env python3
"""The tie-break's measured derivations of the os-probe button pass of run 41006842699 (macOS 26.6.2): the passes of
the accent-following button looks (S17 *A measured derivation*) that docs/facts/macos/button.toml states as
expressions over `input.accent`, beside measure_button_layers.py's three (`derivations.toml`).

Run from the repository root (Python 3.11+, the standard library only):

    python3 docs/evidence/os-probe/2026-10-30-run-41006842699/analysis/tiebreak_derivations.py

It reads, under os-probe-macos-26/ of the run, the button passes `button-pass` (the image's accent) and
`button-pass-accent-red` (AppleAccentColor 0): their probe.toml (each button's frame and cell rects) and the layers
`<appearance>/[styles_b/]<state>-layer.png` of the push and styles_b sheets in the states normal, pressed and disabled;
and the accents, controlAccentColor of each appearance in `probe.toml` (the main pass, the image's accent) and
`accent-red/probe.toml`. It measures each button with measure_button_layers.py's own `measure_button` (imported from
this folder: the fill, the most frequent straight colour and alpha of the drawn box inset by 2 px; the label solved
over the fill), so the two passes are read by one method, and writes `tiebreak-derivations.toml`:
`[[derivation.<name>_<scheme>_<size>]]` (the size as the model names it: `extra_large` for `extraLarge`), one table per
pass, with `"input.accent"` (the pass's controlAccentColor,
"#rrggbb"), `result` (the measured value, "#rrggbbaa", or "#rrggbb" where opaque) and `conditions` (the scheme), for
each name:

- `default_fill` (push `<size>-default`, normal), `default_pressed_fill` (pressed);
- `checked_fill` (push `<size>-toggle_on`, normal), `checked_pressed_fill`, `checked_disabled_fill`;
- `primary_fill` (styles_b `<size>-primary`, normal), `primary_pressed_fill`, `primary_disabled_fill`;
- `tinted_fill` (styles_b `<size>-tinted`, normal), `tinted_pressed_fill`, `tinted_disabled_fill`, `tinted_label`,
  `tinted_pressed_label`, `tinted_disabled_label`;
- `primary_disabled_label` and `checked_disabled_label`, the labels of those disabled accent bezels;

and `[[derivation.<name>_<size>]]`, the same passes of both schemes in one list, for a fact that holds in both; and
`[[derivation.caret_color_dark]]`: controlAccentColor and textInsertionPointColor of darkAqua in the two probe.toml
files (no image: the dark caret captures show the caret only while it fades), the colour NSTextInsertionIndicator
draws by default (`NSTextInsertionIndicator.h:30-32`).

It exits 1, writing nothing, when a file it needs is missing or not as expected.
"""

import sys
import tomllib
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import measure_button_layers as m  # noqa: E402  (the lane's method, reused as is)

RUN = Path(m.RUN)
PASSES = {"key": "button-pass", "accent_red": "button-pass-accent-red"}
ACCENT_PROBES = {"key": RUN / "probe.toml", "accent_red": RUN / "accent-red" / "probe.toml"}
APPEARANCES = {"aqua": "light", "darkAqua": "dark"}
SIZES = {"mini": "mini", "small": "small", "regular": "regular", "large": "large", "extraLarge": "extra_large"}
CASES = {
    "default_fill": ("push", "default", "normal", "fill"),
    "default_pressed_fill": ("push", "default", "pressed", "fill"),
    "checked_fill": ("push", "toggle_on", "normal", "fill"),
    "checked_pressed_fill": ("push", "toggle_on", "pressed", "fill"),
    "checked_disabled_fill": ("push", "toggle_on", "disabled", "fill"),
    "checked_disabled_label": ("push", "toggle_on", "disabled", "label"),
    "primary_fill": ("styles_b", "primary", "normal", "fill"),
    "primary_pressed_fill": ("styles_b", "primary", "pressed", "fill"),
    "primary_disabled_fill": ("styles_b", "primary", "disabled", "fill"),
    "primary_disabled_label": ("styles_b", "primary", "disabled", "label"),
    "tinted_fill": ("styles_b", "tinted", "normal", "fill"),
    "tinted_pressed_fill": ("styles_b", "tinted", "pressed", "fill"),
    "tinted_label": ("styles_b", "tinted", "normal", "label"),
    "tinted_pressed_label": ("styles_b", "tinted", "pressed", "label"),
    "tinted_disabled_fill": ("styles_b", "tinted", "disabled", "fill"),
    "tinted_disabled_label": ("styles_b", "tinted", "disabled", "label"),
}


def spell(c: str) -> str:
    return c[:7] if c.endswith("ff") else c


def main() -> int:
    images, lines = [], []
    try:
        results = {}
        for pname, pdir in PASSES.items():
            probe_path = RUN / pdir / "probe.toml"
            images.append(str(probe_path))
            sheets = tomllib.loads(probe_path.read_text())["macos"]["button_pass"]["sheets"]
            for sheet in ("push", "styles_b"):
                s = sheets[sheet]
                for ap in APPEARANCES:
                    for st in ("normal", "pressed", "disabled"):
                        lay = s[ap][st]["layer"]
                        path = RUN / pdir / lay["file"]
                        images.append(str(path))
                        layer = m.Layer(path)
                        for b, bt in lay["buttons"].items():
                            results[(pname, sheet, ap, st, b)] = m.measure_button(
                                layer, bt["frame_px_xywh"], s["buttons"][b], None)
        accents = {}
        for pname, path in ACCENT_PROBES.items():
            images.append(str(path))
            colors = tomllib.loads(path.read_text())["macos"]["colors"]
            accents[pname] = {ap: colors[ap]["controlAccentColor"][:7] for ap in APPEARANCES}
            carets = {ap: colors[ap]["textInsertionPointColor"] for ap in APPEARANCES}
            lines += ["[[derivation.caret_color_dark]]", f'"input.accent" = "{accents[pname]["darkAqua"]}"',
                      f'result = "{spell(carets["darkAqua"])}"', 'conditions = ["scheme:dark"]', ""]
        for name, (sheet, kind, st, field) in CASES.items():
            both = {size: [] for size in SIZES}
            for ap, scheme in APPEARANCES.items():
                for size in SIZES:
                    for pname in PASSES:
                        r = results[(pname, sheet, ap, st, f"{size}-{kind}")].get(field)
                        if not isinstance(r, str) or not r.startswith("#"):
                            raise m.Bad(f"{name} {pname} {ap} {size}: {r!r}")
                        table = [f'"input.accent" = "{accents[pname][ap]}"', f'result = "{spell(r)}"',
                                 f'conditions = ["scheme:{scheme}"]', ""]
                        lines += [f"[[derivation.{name}_{scheme}_{SIZES[size]}]]"] + table
                        both[size].append(table)
            for size, tables in both.items():
                for table in tables:
                    lines += [f"[[derivation.{name}_{SIZES[size]}]]"] + table
    except (m.Bad, OSError, KeyError) as e:
        print(f"tiebreak_derivations: {e}", file=sys.stderr)
        return 1
    head = ["# Written by analysis/tiebreak_derivations.py (its docstring is the method); do not edit.", "",
            "input_image = ["] + [f'  "{p}",' for p in dict.fromkeys(images)] + ["]", ""]
    (RUN / "tiebreak-derivations.toml").write_text("\n".join(head + lines))
    return 0


if __name__ == "__main__":
    sys.exit(main())
