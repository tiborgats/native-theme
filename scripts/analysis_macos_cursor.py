#!/usr/bin/env python3
"""Reads whether macOS hides the pointer while typing, and how it goes, from the os-probe events pass's
`typing-pointer.mov` recordings (`[macos.button_pass.events.<appearance>.typing-pointer]`; U320, U607, U329's and
U609's `hide_pointer_fade` of the 0.8.0 measurement plan): the cursor drawn over a key text view, keys typed into it,
the mouse moved 1 pt, keys typed again. The facts lane wrote it (2026-11-07, spec S17 *`evidence`*: an analysis
output computed from a recording's frames).

Run from the repository root (Python 3.11+, the standard library and ffmpeg on the path):

    python3 scripts/analysis_macos_cursor.py docs/evidence/os-probe/<run>/<image>
    python3 scripts/analysis_macos_cursor.py --self-test

Per appearance it reads `button-pass-events/probe.toml` (the recording's `crop_quartz_xywh`, the screen rect the
recording was cropped to, and its `posted` events: the move onto the text view, `posted at (x, y)` in the same
Quartz coordinates, gives the cursor's hot spot), `<appearance>/typing-pointer.crop` (the crop ffmpeg applies,
`w:h:x:y`), `<appearance>/typing-pointer.mov` and `<appearance>/typing-pointer-frames.csv` (each frame's time, in
seconds from the recording's start). It cuts the recording into frames as the workflow's step *Cut the motion and
events recordings into frames* does (`ffmpeg -i <mov> -vf crop=<crop> -fps_mode passthrough <dir>/%05d.png`, into a
temporary folder, not kept) and requires as many frames as the CSV lists. Per frame, with luma
L = (r + g + b) / 3 of a pixel:

- `cursor`: the cursor's mark, the sum over the box from the hot spot's x - 10 to x + 9 and y - 14 to y + 13 (crop
  pixels) of max(0, |L - L0| - 8), L0 the pixel's luma in the first frame (before the cursor reaches the text view,
  it is elsewhere; a dark cursor on a light view and a light one on a dark view alike); `drawn` where it is above a
  fifth of the largest frame's;
- `text`: the text's ink, the count of pixels of the text's polarity (L below 110 where the region's median luma in
  the first frame is at least 128, a light view with dark text; L above 160 otherwise, a dark view with light text;
  a focus ring's or selection's mid tones are neither), in every row from x 30 to the box's left edge: typed
  characters add ink there, the text view's text being left of the cursor (the crop's left 30 columns, the window's
  edge, and the cursor's box left out; the rest of what is drawn there is static but for the text's caret blinking,
  a few pixels).

It writes `cursor.toml` into the image folder, `[typing.<appearance>]`: `frames`; `first_key_frame`, the first frame
whose text ink exceeds the first frame's by more than 20 (a typed character drawn); `drawn_before_key`, whether the
cursor is drawn in each of the frames from the fifth to the third before it (not in the two frames just before it:
macOS hides the pointer on the key-down, which the recording shows a frame or two before the typed character's ink,
so the cursor is gone there; run 42328425189's recordings hide it one frame before the ink, 1-based frames 110 and
133 against 111 and 134, the cursor fully drawn through 109 and 132); `hidden_from_frame`, the first frame from 3
before `first_key_frame` on in which it is not drawn; `hides_on_key`, true where it is drawn before the key and hidden
within 2 frames of the key's first frame; `shown_again_frame`, the first later frame in which it is drawn again (after the
1 pt move), and `hidden_again_frame`, the first later frame in which it is hidden again; `fade_frames`, the frames
between the last frame drawn at more than 80 % of the drawn darkness and `hidden_from_frame` (0: from fully drawn to
hidden in one frame); `hide_interval_ms`, the time from the last drawn frame to `hidden_from_frame` (the frames'
CSV times), the most a fade could last unseen; and `hide_pointer_while_typing` (= `hides_on_key`).

`input_image` names every file read, as repository paths. It exits 1, writing nothing, when a file it needs is
missing or not as expected.
"""

import csv
import re
import shutil
import subprocess
import struct
import sys
import tempfile
import tomllib
import zlib
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
        rows.append([tuple(line[x:x + bpp]) for x in range(0, stride, bpp)])
        prev = line
    return width, height, rows


def luma(p):
    return (p[0] + p[1] + p[2]) / 3


def hot_spot(rec: dict):
    cx, cy = rec["crop_quartz_xywh"][:2]
    for p in rec["posted"]:
        if "move onto the text view" in p:
            m = re.search(r"posted at \(([\d.]+), ([\d.]+)\)", p)
            if m:
                return round(float(m.group(1)) - cx), round(float(m.group(2)) - cy)
    raise Bad("no move onto the text view among the posted events")


def measures(frames, spot):
    """Per frame (cursor darkness, text ink)."""
    hx, hy = spot
    w0, h0, first = frames[0]
    lumas = sorted(luma(first[y][x]) for y in range(h0) for x in range(30, max(31, hx - 10)))
    light = lumas[len(lumas) // 2] >= 128   # the view's background: light, its text dark; dark, its text light
    out = []
    for w, h, rows in frames:
        ys = range(max(0, hy - 14), min(h, hy + 14))
        dark = 0.0
        for y in ys:
            for x in range(max(0, hx - 10), min(w, hx + 10)):
                dark += max(0.0, abs(luma(first[y][x]) - luma(rows[y][x])) - 8)
        ink = sum(1 for y in range(h) for x in range(30, max(30, hx - 10))
                  if (luma(rows[y][x]) < 110 if light else luma(rows[y][x]) > 160))
        out.append((dark, ink))
    return out


def decide(ms, times):
    top = max(d for d, _ in ms)
    if top <= 0:
        raise Bad("the cursor is never drawn in its box")
    drawn = [d > top / 5 for d, _ in ms]
    ink0 = ms[0][1]
    key = next((i for i, (_, ink) in enumerate(ms) if ink > ink0 + 20), None)
    if key is None:
        raise Bad("no typed character appears")
    res = {"frames": len(ms), "first_key_frame": key + 1}
    res["drawn_before_key"] = all(drawn[max(0, key - 5):key - 2]) and key >= 5
    hidden = next((i for i in range(max(0, key - 3), len(ms)) if not drawn[i]), None)
    if hidden is None:
        res["hides_on_key"] = False
        res["hide_pointer_while_typing"] = False
        return res
    res["hidden_from_frame"] = hidden + 1
    res["hides_on_key"] = bool(res["drawn_before_key"] and abs(hidden - key) <= 2)
    again = next((i for i in range(hidden, len(ms)) if drawn[i]), None)
    if again is not None:
        res["shown_again_frame"] = again + 1
        gone = next((i for i in range(again, len(ms)) if not drawn[i]), None)
        if gone is not None:
            res["hidden_again_frame"] = gone + 1
    steady = max(d for d, _ in ms[max(0, key - 5):hidden]) if hidden > 0 else 0
    last_full = max((i for i in range(0, hidden) if ms[i][0] > 0.8 * steady), default=None)
    if last_full is not None:
        res["fade_frames"] = hidden - last_full - 1
    if hidden > 0:
        res["hide_interval_ms"] = round(1000 * (times[hidden] - times[hidden - 1]), 1)
    res["hide_pointer_while_typing"] = res["hides_on_key"]
    return res


def cut(mov: Path, crop: str, out: Path):
    if shutil.which("ffmpeg") is None:
        raise Bad("ffmpeg is not on the path")
    r = subprocess.run(["ffmpeg", "-v", "error", "-i", str(mov), "-vf", f"crop={crop}", "-fps_mode", "passthrough",
                        str(out / "%05d.png")], capture_output=True, text=True)
    if r.returncode != 0:
        raise Bad(f"ffmpeg: {r.stderr.strip()}")
    return sorted(out.glob("*.png"))


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


def analyse(image: Path) -> str:
    rel = image.as_posix().rstrip("/")
    ev = tomllib.loads((image / "button-pass-events" / "probe.toml").read_text())["macos"]["button_pass"]["events"]
    inputs = [f"{rel}/button-pass-events/probe.toml"]
    out = {}
    for ap in APPEARANCES:
        rec = ev[ap]["typing-pointer"]
        base = image / "button-pass-events" / ap
        crop = (base / "typing-pointer.crop").read_text().strip()
        times = [float(r[0]) for r in csv.reader((base / "typing-pointer-frames.csv").read_text().splitlines())
                 if r and re.match(r"^[\d.]+$", r[0])]
        inputs += [f"{rel}/button-pass-events/{ap}/typing-pointer.{x}" for x in ("mov", "crop")]
        inputs.append(f"{rel}/button-pass-events/{ap}/typing-pointer-frames.csv")
        with tempfile.TemporaryDirectory() as d:
            pngs = cut(base / "typing-pointer.mov", crop, Path(d))
            if len(pngs) != len(times):
                raise Bad(f"{ap}: {len(pngs)} frames cut, {len(times)} times listed")
            frames = [read_png(p) for p in pngs]
        out[ap] = decide(measures(frames, hot_spot(rec)), times)
    lines = ["# Output of scripts/analysis_macos_cursor.py (its docstring is the method), file cursor.toml.",
             "# input_image: every file it reads, as a repository path (spec S17 *`evidence`*).",
             "input_image = " + toml_val(inputs)]
    for ap, t in out.items():
        lines.append(f"\n[typing.{ap}]")
        lines += [f"{k} = {toml_val(v)}" for k, v in t.items()]
    return "\n".join(lines) + "\n"


def self_test() -> int:
    fails = []
    rec = {"crop_quartz_xywh": [180.0, 348.0, 300.0, 140.0],
           "posted": ["2230.8 ms (planned 2200 ms): move onto the text view: posted at (330.0, 418.0)"]}
    if hot_spot(rec) != (150, 70):
        fails.append(f"hot_spot {hot_spot(rec)}")
    times = [i / 60 for i in range(20)]
    # the cursor drawn from frame 3, the key's ink at frame 10, the cursor gone at 10, back at 15, gone at 18
    ms = [(0.0, 100)] * 3 + [(500.0, 100)] * 7 + [(0.0, 150)] * 5 + [(500.0, 150)] * 3 + [(0.0, 170)] * 2
    r = decide(ms, times)
    want = {"frames": 20, "first_key_frame": 11, "drawn_before_key": True, "hidden_from_frame": 11, "hides_on_key": True,
            "shown_again_frame": 16, "hidden_again_frame": 19, "fade_frames": 0, "hide_interval_ms": 16.7,
            "hide_pointer_while_typing": True}
    if r != want:
        fails.append(f"decide {r}")
    ms2 = [(0.0, 100)] * 3 + [(500.0, 100)] * 7 + [(500.0, 150)] * 10
    if decide(ms2, times)["hide_pointer_while_typing"]:
        fails.append("a cursor that stays drawn does not hide")
    ms3 = [(0.0, 100)] * 3 + [(500.0, 100)] * 6 + [(250.0, 100)] + [(0.0, 150)] * 10
    if decide(ms3, times).get("fade_frames") != 1:
        fails.append(f"a half-drawn frame is a fade frame: {decide(ms3, times)}")
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
        text = analyse(image)
    except (Bad, OSError, KeyError) as e:
        print(f"analysis_macos_cursor: {e}", file=sys.stderr)
        return 1
    out = image / "cursor.toml"
    out.write_text(text)
    tomllib.loads(out.read_text())
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
