#!/usr/bin/env python3
"""Reads the macOS pointer behaviour of the os-probe events pass (`--button-pass --events`,
`[macos.button_pass.events]`; U322, U329, U600, U609 of the 0.8.0 measurement plan) as the facts' values: the drag
threshold and earliest drag, the tool tip's delay, the spring-loading delay and the drop target's highlight colour.
The facts lane wrote it (2026-11-07, spec S17 *`evidence`*: an analysis output computed from values a harness read).

Run from the repository root (Python 3.11+, the standard library only), once per image folder:

    python3 scripts/analysis_macos_events.py docs/evidence/os-probe/<run>/<image>
    python3 scripts/analysis_macos_events.py --self-test

It reads `button-pass-events/probe.toml` under the image folder (the probe's `method` keys state how it posted and
read) and the drag-over window captures `button-pass-events/<appearance>/drag-over-table-on.png` and
`drag-over-table-above.png` (8-bit RGBA, one pixel a point: the screens are at a backing scale of 1,
`[macos.button_pass.events] mainScreen_backingScaleFactor`, which it requires to be 1). It writes `events.toml` into
the image folder:

- `[drag_start]`: `threshold_pt`, the smallest distance of `distance_after_hold."<n> pt"` (a press held 800 ms on the
  selected text, then moved n pt) whose `drag_pasteboard_changes` is at least 1, every smaller distance read having
  none (else no key); `distances_read`, the distances in order; `min_delay_ms`, the shortest hold of
  `hold_before_distance."<n> ms"` (a press moved 12 pt after n ms), where every hold read started a drag, so no
  drag waits longer than that hold (else no key); `holds_read`.
- `[tool_tip]`: `hover_time_ms`, the mean of the `cold` (1 s away first) and `rested` (5 s away) readings'
  `shown_after_ms` (from the cursor's arrival to the tool tip's window, polled), rounded to 0.1 ms, and
  `spread_ms`, their difference; `warm_shown_after_ms`, the `warm` reading (the cursor back 300 ms after a tip),
  for the record: the model holds one hover time.
- `[springing]`: `delay_s` and `enabled`, the user defaults `com.apple.springing.delay` and
  `com.apple.springing.enabled` as the pass read them (`[…events.defaults]`), and `spring_state_before`,
  `spring_state_after` of each appearance's drag over the spring-loaded button.
- `[drop_highlight.<appearance>]`: `on` and `above`, in each capture the most frequent straight colour (`#rrggbbaa`)
  among the table's pixels whose largest and smallest of r, g, b differ by more than 40 (the saturated pixels: the
  table's rows, header and text are grey), and `pixels`, how many pixels hold it. The table's rect is
  `frames_screen_xywh.table` (screen points, origin bottom left) in the window capture of `window_frame_xywh`, from
  the window's top left: x - window x, window y + window height - (y + height).

`input_image` names every file read, as repository paths. It exits 1, writing nothing, when a file it needs is
missing or not as expected.
"""

import struct
import sys
import tempfile
import tomllib
import zlib
from collections import Counter
from pathlib import Path

APPEARANCES = ("aqua", "darkAqua")


class Bad(Exception):
    pass


def read_png(path: Path):
    """(width, height, rows of (r, g, b, a)) of an 8-bit, non-interlaced RGB or RGBA PNG."""
    data = path.read_bytes()
    if data[:8] != b"\x89PNG\r\n\x1a\n":
        raise Bad(f"{path}: not a PNG")
    pos, idat, ihdr = 8, b"", None
    while pos < len(data):
        (length,) = struct.unpack(">I", data[pos:pos + 4])
        kind, body = data[pos + 4:pos + 8], data[pos + 8:pos + 8 + length]
        pos += 12 + length
        if kind == b"IHDR":
            ihdr = struct.unpack(">IIBBBBB", body)
        elif kind == b"IDAT":
            idat += body
    if ihdr is None:
        raise Bad(f"{path}: no IHDR")
    width, height, depth, colour, _, _, interlace = ihdr
    if depth != 8 or colour not in (2, 6) or interlace != 0:
        raise Bad(f"{path}: depth {depth}, colour type {colour}, interlace {interlace}: not read here")
    bpp = 4 if colour == 6 else 3
    raw, stride = zlib.decompress(idat), width * bpp
    rows, prev, i = [], bytearray(stride), 0
    for _ in range(height):
        f, line = raw[i], bytearray(raw[i + 1:i + 1 + stride])
        i += 1 + stride
        for x in range(stride):
            a = line[x - bpp] if x >= bpp else 0
            b = prev[x]
            c = prev[x - bpp] if x >= bpp else 0
            if f == 1:
                line[x] = (line[x] + a) & 0xFF
            elif f == 2:
                line[x] = (line[x] + b) & 0xFF
            elif f == 3:
                line[x] = (line[x] + (a + b) // 2) & 0xFF
            elif f == 4:
                p = a + b - c
                pa, pb, pc = abs(p - a), abs(p - b), abs(p - c)
                line[x] = (line[x] + (a if pa <= pb and pa <= pc else (b if pb <= pc else c))) & 0xFF
            elif f != 0:
                raise Bad(f"{path}: filter {f}")
        rows.append([tuple(line[x:x + bpp]) + ((255,) if bpp == 3 else ()) for x in range(0, stride, bpp)])
        prev = line
    return width, height, rows


def number_key(k: str) -> float:
    return float(k.split()[0])


def drag_start(ev: dict) -> dict:
    out = {}
    dist = ev["drag_start"]["distance_after_hold"]
    ds = sorted(dist, key=number_key)
    out["distances_read"] = [number_key(k) for k in ds]
    started = [k for k in ds if dist[k].get("drag_pasteboard_changes", 0) >= 1]
    if started:
        first = started[0]
        if all(dist[k].get("drag_pasteboard_changes", 0) == 0 for k in ds[:ds.index(first)]):
            out["threshold_pt"] = number_key(first)
    holds = ev["drag_start"]["hold_before_distance"]
    hs = sorted(holds, key=number_key)
    out["holds_read"] = [number_key(k) for k in hs]
    if hs and all(holds[k].get("drag_pasteboard_changes", 0) >= 1 for k in hs):
        out["min_delay_ms"] = number_key(hs[0])
    return out


def tool_tip(ev: dict) -> dict:
    t = ev["tool_tip"]
    cold, rested = t["cold"]["shown_after_ms"], t["rested"]["shown_after_ms"]
    if t["cold"].get("visible_before") or t["rested"].get("visible_before"):
        raise Bad("a tool tip was visible before the cold or rested reading")
    return {"hover_time_ms": round((cold + rested) / 2, 1), "spread_ms": round(abs(cold - rested), 1),
            "warm_shown_after_ms": round(t["warm"]["shown_after_ms"], 1)}


def table_rect(ev: dict):
    x, y, w, h = ev["frames_screen_xywh"]["table"]
    wx, wy, ww, wh = ev["window_frame_xywh"]
    return round(x - wx), round(wy + wh - (y + h)), round(w), round(h)


def saturated_mode(rows, rect):
    x0, y0, w, h = rect
    c = Counter()
    for y in range(y0, y0 + h):
        for x in range(x0, x0 + w):
            p = rows[y][x]
            if max(p[:3]) - min(p[:3]) > 40:
                c[p] += 1
    if not c:
        return None, 0
    p, n = c.most_common(1)[0]
    return "#" + "".join(f"{v:02x}" for v in p), n


def toml_val(v):
    if isinstance(v, bool):
        return "true" if v else "false"
    if isinstance(v, (int, float)):
        return repr(v)
    if isinstance(v, str):
        return '"' + v + '"'
    if isinstance(v, list):
        return "[" + ", ".join(toml_val(x) for x in v) + "]"
    raise Bad(f"value {v!r}")


def analyse(image: Path, rel_image: str) -> str:
    probe = image / "button-pass-events" / "probe.toml"
    ev = tomllib.loads(probe.read_text())["macos"]["button_pass"]["events"]
    if ev.get("mainScreen_backingScaleFactor") != 1.0:
        raise Bad("the screen's backing scale is not 1")
    inputs = [f"{rel_image}/button-pass-events/probe.toml"]
    tables = {"drag_start": drag_start(ev), "tool_tip": tool_tip(ev)}
    d = ev["defaults"]
    spring = {"delay_s": d["com.apple.springing.delay"], "enabled": d["com.apple.springing.enabled"]}
    for ap in APPEARANCES:
        dov = ev.get(ap, {}).get("drag-over", {})
        for k in ("spring_state_before", "spring_state_after"):
            if k in dov:
                spring[f"{k}_{ap}"] = dov[k]
    tables["springing"] = spring
    rect = table_rect(ev)
    for ap in APPEARANCES:
        hl = {}
        for name in ("on", "above"):
            png = image / "button-pass-events" / ap / f"drag-over-table-{name}.png"
            _, _, rows = read_png(png)
            inputs.append(f"{rel_image}/button-pass-events/{ap}/drag-over-table-{name}.png")
            colour, n = saturated_mode(rows, rect)
            if colour is not None:
                hl[name] = colour
                hl[f"{name}_pixels"] = n
        tables[f"drop_highlight.{ap}"] = hl
    lines = ["# Output of scripts/analysis_macos_events.py (its docstring is the method), file events.toml.",
             "# input_image: every file it reads, as a repository path (spec S17 *`evidence`*).",
             "input_image = " + toml_val(inputs)]
    for name, t in tables.items():
        lines.append(f"\n[{name}]")
        lines += [f"{k} = {toml_val(v)}" for k, v in t.items()]
    return "\n".join(lines) + "\n"


def write_png(path: Path, w, h, pixel):
    raw = b"".join(b"\x00" + b"".join(bytes(pixel(x, y)) for x in range(w)) for y in range(h))
    def chunk(k, b):
        return struct.pack(">I", len(b)) + k + b + struct.pack(">I", zlib.crc32(k + b) & 0xFFFFFFFF)
    path.write_bytes(b"\x89PNG\r\n\x1a\n" + chunk(b"IHDR", struct.pack(">IIBBBBB", w, h, 8, 6, 0, 0, 0))
                     + chunk(b"IDAT", zlib.compress(raw)) + chunk(b"IEND", b""))


def self_test() -> int:
    fails = []
    ev = {"drag_start": {"distance_after_hold": {"1 pt": {"drag_pasteboard_changes": 0},
                                                 "10 pt": {"drag_pasteboard_changes": 1},
                                                 "3 pt": {"drag_pasteboard_changes": 0},
                                                 "4 pt": {"drag_pasteboard_changes": 1}},
                         "hold_before_distance": {"50 ms": {"drag_pasteboard_changes": 1},
                                                  "0 ms": {"drag_pasteboard_changes": 1}}}}
    r = drag_start(ev)
    if r.get("threshold_pt") != 4.0 or r.get("min_delay_ms") != 0.0 or r["distances_read"] != [1.0, 3.0, 4.0, 10.0]:
        fails.append(f"drag_start {r}")
    ev["drag_start"]["distance_after_hold"]["1 pt"]["drag_pasteboard_changes"] = 1
    ev["drag_start"]["distance_after_hold"]["3 pt"]["drag_pasteboard_changes"] = 0
    if drag_start(ev).get("threshold_pt") != 1.0:
        fails.append("a first distance that starts a drag is the threshold")
    ev["drag_start"]["hold_before_distance"]["0 ms"]["drag_pasteboard_changes"] = 0
    if "min_delay_ms" in drag_start(ev):
        fails.append("a hold that started no drag leaves min_delay_ms out")
    t = tool_tip({"tool_tip": {"cold": {"shown_after_ms": 1500.0}, "rested": {"shown_after_ms": 1510.0},
                               "warm": {"shown_after_ms": 8.0}}})
    if t != {"hover_time_ms": 1505.0, "spread_ms": 10.0, "warm_shown_after_ms": 8.0}:
        fails.append(f"tool_tip {t}")
    if table_rect({"frames_screen_xywh": {"table": [500.0, 220.0, 240.0, 172.0]},
                   "window_frame_xywh": [200.0, 200.0, 560.0, 372.0]}) != (300, 180, 240, 172):
        fails.append("table_rect")
    with tempfile.TemporaryDirectory() as d:
        p = Path(d) / "t.png"
        write_png(p, 6, 4, lambda x, y: (0, 100, 225, 255) if y == 1 and x < 4 else (120, 120, 120, 255))
        _, _, rows = read_png(p)
        if saturated_mode(rows, (0, 0, 6, 4)) != ("#0064e1ff", 4):
            fails.append(f"saturated_mode {saturated_mode(rows, (0, 0, 6, 4))}")
        if saturated_mode(rows, (0, 2, 6, 2)) != (None, 0):
            fails.append("grey pixels give no highlight")
    for f in fails:
        print(f"self-test: {f}", file=sys.stderr)
    print(f"self-test: {'ok' if not fails else 'FAILED'}")
    return 1 if fails else 0


def main(argv) -> int:
    if argv[1:] == ["--self-test"]:
        return self_test()
    if len(argv) != 2:
        print(__doc__.split("\n\n")[2], file=sys.stderr)
        return 2
    image = Path(argv[1])
    try:
        text = analyse(image, image.as_posix().rstrip("/"))
    except (Bad, OSError, KeyError) as e:
        print(f"analysis_macos_events: {e}", file=sys.stderr)
        return 1
    out = image / "events.toml"
    out.write_text(text)
    tomllib.loads(out.read_text())
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
