#!/usr/bin/env python3
"""Measures the macOS focus ring of the os-probe button passes: its dash, the curve of its outer corners and what it
draws outside itself, from the focus pass's layers, and its transition, from the focus-transition passes' screen
recordings. Raw measurements only: which `DashPattern`, `CornerCurve`, `Shadow` and `Transition` they are is the
facts' reading.

Run from the repository root (Python 3.11+, the standard library only):

    python3 docs/evidence/os-probe/<run>/analysis/measure_focus_ring.py docs/evidence/os-probe/<run>/os-probe-macos-26

It reads, under that image folder (the probe's own method texts are in each probe.toml, `[macos.button_pass]`
`method`, `layer_method` and, for the transition, `[macos.button_pass.focus_transition] method`):

- button-pass-focus/probe.toml: per appearance the push sheet's unfocused layer (`[…push.<appearance>.unfocused.layer]
  file`, `file_rect_px_xywh`) and per button its focused layer (`[…push.<appearance>.focused.<button>.layer] file`,
  `file_rect_px_xywh`, the button's region only) and the button's frame (`[….layer.buttons.<button>] frame_px_xywh`);
  the layers are each pixel's straight colour and alpha solved by the probe from captures over black and white;
- button-pass-focus-transition[-reduce-motion-off]/probe.toml (`[macos.button_pass.focus_transition]`: per appearance
  and button `crop_px_whxy`, `focus_at_ms`, `unfocus_at_ms`) and per recording the frames the workflow cut from it,
  `<appearance>/transition-<button>-frames/NNNNN.png`, with their times in `…-frames.csv` (ffprobe's pts_time, one a
  line, in frame order).

It writes `focus-ring.toml` and, where a transition pass is there, `focus-transition.toml`, into the image folder.

focus-ring.toml, per `<appearance>.<button>` of the push sheet. The ring's own alpha at a pixel is solved from the
focused layer F and the unfocused one U as a layer drawn over U: a_ring = (a_F - a_U) / (1 - a_U) (0 where a_U is 1);
a pixel is the ring's where F and U differ by more than 2/255 in a premultiplied channel or the alpha. Distances `d`
count pixels outward from the frame's edge: d = 1 is the first pixel outside it, d = 0 the last inside.

- `profile_<side>`: across the middle of each side (left, right: the frame's middle row; top, bottom: its middle
  column), from d = -4 to d = 6, `[d, a_ring, a_F, a_U, ring]` per pixel (ring: 1 where the pixel is the ring's);
- `band_d`: the outward distance of the band the dash is read along, the outermost d (of d >= 1, outside the frame) at
  which every profile has a_ring at least 0.9 of the largest a_ring outside the frame of its profile;
  `band_alpha_min`, `band_alpha_max`: the ring's alpha along that band, over the straight part of each side (from one
  corner's end to the other's: the largest `circle_radius_px` of the corners below, rounded up, in from each end of
  the band's side); `band_gaps`: how many pixels of it are not the ring's; `band_pixels`: how many it has;
- `beyond_ring_pixels`, `beyond_ring_alpha_max`: in the button's region, outside the frame grown by the ring's outer
  edge (`outer_d`, the largest d at which a profile has a ring pixel), the pixels with a_F of at least 3/255, and
  their largest a_F: what is drawn outside the ring (a shadow or glow would be);
- `corner_<corner>`: the ring's outer corner, in the box of the frame grown by the ring's outer edge: per row from the
  box's edge inward (up to 40 rows or half the box's height, ending after two rows under 0.05), the missing coverage
  (1 - a_F / the largest a_ring outside the frame of the profiles, `ring_alpha_max`, clamped to 0..1: the focused
  layer's own alpha, the ring and the bezel together, whose outer edge is the ring's) summed from the box's side
  inward to the first pixel at 0.95 coverage (the ring's outermost band pixel reads 0.97 of the largest: the outer
  edge lies a little inside it) (`rows`); `circle_radius_px`, `circle_rms_px`: the circle radius in 0.25 px steps
  whose missing area per row (integrated over the row, 16 samples) fits the rows best (least squares);
  `superellipse_a_px`, `superellipse_n`, `superellipse_rms_px`: the same for the corner |x/a|^n + |y/a|^n = 1 (a in
  0.25 px steps, n from 2 to 8 in steps of 0.1; n = 2 is the circle), as a continuous corner is closer to a
  superellipse than to a circle.

focus-transition.toml, per `<pass>.<appearance>.<button>` (pass: image, the image's Reduce Motion; reduce_motion_off):
each frame compared with the recording's first (the button unfocused): `changed` the pixels whose byte differs by
more than 8 in a channel, `diff` the sum of the absolute byte differences over r, g, b (the recording is H.264, so
a still screen's frames differ a little too); `plateau_diff`, the median `diff` of the frames whose `diff` is at least
half the largest (the ring drawn); `floor_diff`, the median `diff` of the frames before the first of those (the
encoder's noise, no ring); per frame `[t_s, share, changed, changed bounds x, y, w, h]` (`frames`), share = (diff -
floor_diff) / (plateau_diff - floor_diff); `focus_in`: from the last frame of share <= 0.05 before the first frame of
share >= 0.95 to that frame, their times, the span between them and the shares of the frames between
(`intermediate_shares`); `focus_out`: from the last frame of share >= 0.95 before the first frame of share <= 0.05
after the plateau to that frame; `frame_interval_s_median`: the median time between two frames (the recording keeps
a frame only where the screen changed).
"""

import struct
import sys
import tomllib
import zlib
from math import ceil, sqrt
from pathlib import Path


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
        kind = data[pos + 4:pos + 8]
        body = data[pos + 8:pos + 8 + length]
        pos += 12 + length
        if kind == b"IHDR":
            ihdr = struct.unpack(">IIBBBBB", body)
        elif kind == b"IDAT":
            idat += body
        elif kind == b"IEND":
            break
    if ihdr is None:
        raise Bad(f"{path}: no IHDR")
    width, height, depth, colour, _, _, interlace = ihdr
    if depth != 8 or colour not in (2, 6) or interlace != 0:
        raise Bad(f"{path}: depth {depth}, colour type {colour}, interlace {interlace}: not read here")
    bpp = 4 if colour == 6 else 3
    raw = zlib.decompress(idat)
    stride = width * bpp
    rows, prev, i = [], bytearray(stride), 0
    for _ in range(height):
        f = raw[i]
        line = bytearray(raw[i + 1:i + 1 + stride])
        i += 1 + stride
        if f == 1:
            for x in range(bpp, stride):
                line[x] = (line[x] + line[x - bpp]) & 0xFF
        elif f == 2:
            for x in range(stride):
                line[x] = (line[x] + prev[x]) & 0xFF
        elif f == 3:
            for x in range(stride):
                a = line[x - bpp] if x >= bpp else 0
                line[x] = (line[x] + (a + prev[x]) // 2) & 0xFF
        elif f == 4:
            for x in range(stride):
                a = line[x - bpp] if x >= bpp else 0
                b = prev[x]
                c = prev[x - bpp] if x >= bpp else 0
                p = a + b - c
                pa, pb, pc = abs(p - a), abs(p - b), abs(p - c)
                pred = a if pa <= pb and pa <= pc else (b if pb <= pc else c)
                line[x] = (line[x] + pred) & 0xFF
        elif f != 0:
            raise Bad(f"{path}: filter {f}")
        rows.append([tuple(line[x:x + bpp]) + ((255,) if bpp == 3 else ()) for x in range(0, stride, bpp)])
        prev = line
    return width, height, rows


class Layer:
    """A PNG placed at `origin` in window pixels; `at(x, y)` in window pixels, None outside."""

    def __init__(self, path: Path, origin=(0, 0)):
        self.w, self.h, self.rows = read_png(path)
        self.ox, self.oy = origin

    def at(self, x, y):
        x, y = x - self.ox, y - self.oy
        if 0 <= x < self.w and 0 <= y < self.h:
            return self.rows[y][x]
        return None


def premul(p):
    a = p[3] / 255
    return (p[0] / 255 * a, p[1] / 255 * a, p[2] / 255 * a, a)


def ring_pixel(f, u):
    """(a_ring, a_F, a_U, is_ring) of the focused pixel f over the unfocused u."""
    pf, pu = premul(f), premul(u)
    a_f, a_u = pf[3], pu[3]
    a_r = 0.0 if a_u >= 1.0 else max(0.0, min(1.0, (a_f - a_u) / (1.0 - a_u)))
    is_ring = any(abs(x - y) > 2 / 255 for x, y in zip(pf, pu))
    return a_r, a_f, a_u, is_ring


def r3(v):
    return round(v, 3)


SIDES = ("left", "right", "top", "bottom")


def side_pixel(fr, side, d, along):
    """The window pixel at outward distance d from `side` of frame fr, at `along` (a row for left/right, a column
    for top/bottom)."""
    x, y, w, h = fr
    if side == "left":
        return x - d, along
    if side == "right":
        return x + w - 1 + d, along
    if side == "top":
        return along, y - d
    return along, y + h - 1 + d


def circle_inset(r, y):
    """The circle corner's inset from the box's side at depth y (0..r) from its edge."""
    if y >= r:
        return 0.0
    return r - sqrt(max(0.0, r * r - (r - y) ** 2))


def superellipse_inset(a, n, y):
    if y >= a:
        return 0.0
    t = (a - y) / a
    return a - a * (max(0.0, 1.0 - t ** n)) ** (1.0 / n)


def row_area(inset, i):
    return sum(inset(i + (k + 0.5) / 16) for k in range(16)) / 16


def fit_rows(rows, inset_of, params):
    best = None
    for p in params:
        inset = inset_of(p)
        err = sum((row_area(inset, i) - v) ** 2 for i, v in enumerate(rows)) / max(1, len(rows))
        if best is None or err < best[1]:
            best = (p, err)
    return best[0], sqrt(best[1])


def corner_rows(get, outer, corner, ring_max):
    """Per row from the outer box's edge inward, the missing ring coverage summed from its side inward."""
    x, y, w, h = outer
    rows, quiet = [], 0
    for i in range(min(40, h // 2)):
        yy = y + i if corner.startswith("top") else y + h - 1 - i
        total = 0.0
        for j in range(w // 2):
            xx = x + j if corner.endswith("left") else x + w - 1 - j
            a = get(xx, yy)
            if a is None:
                break
            cov = max(0.0, min(1.0, a / ring_max)) if ring_max > 0 else 0.0
            if cov >= 0.95:
                break
            total += 1.0 - cov
        rows.append(total)
        quiet = quiet + 1 if total < 0.05 else 0
        if quiet >= 2:
            break
    while rows and rows[-1] < 0.05:
        rows.pop()
    return rows


def ring_toml(focus_dir: Path, probe: dict):
    sheets = probe["macos"]["button_pass"]["sheets"]["push"]
    out = {}
    inputs = []
    for appearance in ("aqua", "darkAqua"):
        run = sheets.get(appearance)
        if not isinstance(run, dict):
            continue
        ul = run["unfocused"]["layer"]
        ux, uy, _, _ = ul["file_rect_px_xywh"]
        unfocused = Layer(focus_dir / ul["file"], (ux, uy))
        inputs.append(ul["file"])
        per = {}
        for name, f in run["focused"].items():
            if not isinstance(f, dict) or "layer" not in f or "file" not in f["layer"]:
                continue
            fl = f["layer"]
            fx, fy, fw, fh = fl["file_rect_px_xywh"]
            focused = Layer(focus_dir / fl["file"], (fx, fy))
            inputs.append(fl["file"])
            fr = fl["buttons"][name]["frame_px_xywh"]

            def ring_at(x, y):
                fp, up = focused.at(x, y), unfocused.at(x, y)
                if fp is None or up is None:
                    return None
                return ring_pixel(fp, up)

            m = {}
            outer_d, band_d = None, None
            mids = {"left": fr[1] + fr[3] // 2, "right": fr[1] + fr[3] // 2,
                    "top": fr[0] + fr[2] // 2, "bottom": fr[0] + fr[2] // 2}
            band_ds = []
            ring_max = 0.0
            for side in SIDES:
                prof = []
                for d in range(-4, 7):
                    px = ring_at(*side_pixel(fr, side, d, mids[side]))
                    if px is None:
                        continue
                    a_r, a_f, a_u, is_ring = px
                    prof.append([d, r3(a_r), r3(a_f), r3(a_u), 1 if is_ring else 0])
                m[f"profile_{side}"] = prof
                ring_ds = [p[0] for p in prof if p[4]]
                if ring_ds:
                    outer_d = max(ring_ds) if outer_d is None else max(outer_d, max(ring_ds))
                outside = [p for p in prof if p[0] >= 1]
                top = max((p[1] for p in outside), default=0.0)
                ring_max = max(ring_max, top)
                strong = [p[0] for p in outside if top > 0 and p[1] >= 0.9 * top]
                if strong:
                    band_ds.append(max(strong))
            if band_ds:
                band_d = min(band_ds)
            if outer_d is not None:
                gx, gy, gw, gh = fr[0] - outer_d, fr[1] - outer_d, fr[2] + 2 * outer_d, fr[3] + 2 * outer_d
                beyond, beyond_max = 0, 0.0
                for yy in range(fy, fy + fh):
                    for xx in range(fx, fx + fw):
                        if gx <= xx < gx + gw and gy <= yy < gy + gh:
                            continue
                        p = focused.at(xx, yy)
                        if p is not None and p[3] >= 3:
                            beyond += 1
                            beyond_max = max(beyond_max, p[3] / 255)
                m["outer_d"] = outer_d
                m["beyond_ring_pixels"] = beyond
                m["beyond_ring_alpha_max"] = r3(beyond_max)
                outer = (gx, gy, gw, gh)

                def a_focused(x, y):
                    px = ring_at(x, y)
                    return None if px is None else px[1]

                for corner in ("top_left", "top_right", "bottom_right", "bottom_left"):
                    rows = corner_rows(a_focused, outer, corner, ring_max)
                    c = {"rows": [r3(v) for v in rows]}
                    if len(rows) >= 2:
                        radii = [k / 4 for k in range(2, 4 * 30)]
                        r, rms = fit_rows(rows, lambda r: (lambda y: circle_inset(r, y)), radii)
                        c["circle_radius_px"] = r
                        c["circle_rms_px"] = r3(rms)
                        params = [(a / 4, n / 10) for a in range(4, 4 * 40) for n in range(20, 81)]
                        (a, n), rms = fit_rows(rows, lambda p: (lambda y: superellipse_inset(p[0], p[1], y)), params)
                        c["superellipse_a_px"] = a
                        c["superellipse_n"] = n
                        c["superellipse_rms_px"] = r3(rms)
                    m[f"corner_{corner}"] = c
            radii = [c.get("circle_radius_px") for k, c in m.items() if k.startswith("corner_")]
            radii = [r for r in radii if r is not None]
            outer_r = max(radii) if radii else None
            if band_d is not None and outer_r is not None:
                skip = ceil(outer_r)
                alphas, gaps = [], 0
                for side in SIDES:
                    span = range(fr[1] + skip, fr[1] + fr[3] - skip) if side in ("left", "right") else \
                        range(fr[0] + skip, fr[0] + fr[2] - skip)
                    for along in span:
                        px = ring_at(*side_pixel(fr, side, band_d, along))
                        if px is None:
                            continue
                        alphas.append(px[0])
                        if not px[3]:
                            gaps += 1
                m["band_d"] = band_d
                m["band_pixels"] = len(alphas)
                m["band_gaps"] = gaps
                if alphas:
                    m["band_alpha_min"] = r3(min(alphas))
                    m["band_alpha_max"] = r3(max(alphas))
            else:
                m["band_d"] = "unavailable: no ring band, or no corner fit"
            m["ring_alpha_max"] = r3(ring_max)
            per[name] = m
        out[appearance] = per
    return out, inputs


def median(v):
    s = sorted(v)
    return s[len(s) // 2] if s else None


def transition_toml(image: Path):
    out, inputs = {}, []
    for folder, key in (("button-pass-focus-transition", "image"),
                        ("button-pass-focus-transition-reduce-motion-off", "reduce_motion_off")):
        probe_path = image / folder / "probe.toml"
        if not probe_path.exists():
            continue
        inputs.append(f"{folder}/probe.toml")
        probe = tomllib.loads(probe_path.read_text())
        ft = probe["macos"]["button_pass"].get("focus_transition", {})
        per_pass = {}
        for appearance in ("aqua", "darkAqua"):
            run = ft.get(appearance)
            if not isinstance(run, dict):
                continue
            per = {}
            for name, rec in run.items():
                if not isinstance(rec, dict):
                    continue
                stem = image / folder / appearance / f"transition-{name}"
                frames_dir = Path(f"{stem}-frames")
                csv = Path(f"{stem}-frames.csv")
                if not frames_dir.is_dir() or not csv.exists():
                    per[name] = {"error": f"unavailable: no {frames_dir.name}/ or {csv.name}"}
                    continue
                times = [float(line.split(",")[0]) for line in csv.read_text().split() if line.strip()]
                files = sorted(frames_dir.glob("*.png"))
                inputs.append(f"{folder}/{appearance}/{csv.name}")
                inputs.extend(f"{folder}/{appearance}/{frames_dir.name}/{p.name}" for p in files)
                if len(times) != len(files) or not files:
                    per[name] = {"error": f"unavailable: {len(files)} frames, {len(times)} times"}
                    continue
                first = read_png(files[0])
                w, h, base = first
                rows = []
                for t, p in zip(times, files):
                    fw, fh, px = read_png(p)
                    if (fw, fh) != (w, h):
                        rows.append([t, None, None, None])
                        continue
                    changed, diff = 0, 0
                    bx0, by0, bx1, by1 = w, h, -1, -1
                    for y in range(h):
                        for x in range(w):
                            a, b = px[y][x], base[y][x]
                            d = max(abs(a[0] - b[0]), abs(a[1] - b[1]), abs(a[2] - b[2]))
                            diff += abs(a[0] - b[0]) + abs(a[1] - b[1]) + abs(a[2] - b[2])
                            if d > 8:
                                changed += 1
                                bx0, by0, bx1, by1 = min(bx0, x), min(by0, y), max(bx1, x), max(by1, y)
                    bounds = [bx0, by0, bx1 - bx0 + 1, by1 - by0 + 1] if changed else []
                    rows.append([t, diff, changed, bounds])
                diffs = [r[1] for r in rows if r[1] is not None]
                top = max(diffs) if diffs else 0
                first_hi = next((i for i, r in enumerate(rows) if r[1] is not None and top and r[1] >= top / 2),
                                None)
                plateau = median([r[1] for r in rows if r[1] is not None and top and r[1] >= top / 2]) or 0
                floor = median([r[1] for r in rows[:first_hi] if r[1] is not None]) if first_hi else 0
                floor = floor or 0
                m = {"crop_px_whxy": rec.get("crop_px_whxy"), "focus_at_ms": rec.get("focus_at_ms"),
                     "unfocus_at_ms": rec.get("unfocus_at_ms"), "frame_count": len(rows),
                     "floor_diff": floor, "plateau_diff": plateau}
                span = plateau - floor
                share = [((r[1] - floor) / span) if span > 0 and r[1] is not None else None for r in rows]
                m["frames"] = [[r3(r[0]), r3(sh) if sh is not None else "n/a",
                                r[2] if r[2] is not None else "n/a", r[3] if r[3] else []]
                               for r, sh in zip(rows, share)]
                gaps = [b[0] - a[0] for a, b in zip(rows, rows[1:])]
                m["frame_interval_s_median"] = r3(median(gaps)) if gaps else "n/a"

                def ok(i, test):
                    return share[i] is not None and test(share[i])

                def bracket(lo, hi):
                    if lo is None or hi is None:
                        return {"error": "unavailable: no such frame"}
                    a, b = (lo, hi) if lo < hi else (hi, lo)
                    mid = [r3(share[i]) for i in range(a + 1, b) if share[i] is not None]
                    return {"from_frame": lo + 1, "from_time_s": r3(rows[lo][0]),
                            "to_frame": hi + 1, "to_time_s": r3(rows[hi][0]),
                            "span_s": r3(abs(rows[hi][0] - rows[lo][0])),
                            "intermediate_frames": len(mid), "intermediate_shares": mid}

                hi = next((i for i in range(len(rows)) if ok(i, lambda v: v >= 0.95)), None)
                lo = next((i for i in range(hi - 1, -1, -1) if ok(i, lambda v: v <= 0.05)), None) \
                    if hi is not None else None
                m["focus_in"] = bracket(lo, hi)
                lo2 = next((i for i in range(hi + 1, len(rows)) if ok(i, lambda v: v <= 0.05)), None) \
                    if hi is not None else None
                hi2 = next((i for i in range(lo2 - 1, hi - 1, -1) if ok(i, lambda v: v >= 0.95)), None) \
                    if lo2 is not None else None
                m["focus_out"] = bracket(hi2, lo2)
                per[name] = m
            per_pass[appearance] = per
        out[key] = per_pass
    return out, inputs


def toml_val(v):
    if isinstance(v, bool):
        return "true" if v else "false"
    if isinstance(v, (int, float)):
        return repr(v)
    if isinstance(v, str):
        return '"' + v.replace("\\", "\\\\").replace('"', '\\"') + '"'
    if isinstance(v, list):
        return "[" + ", ".join(toml_val(x) for x in v) + "]"
    raise Bad(f"no TOML for {v!r}")


def emit(lines, path, table):
    scalars = {k: v for k, v in table.items() if not isinstance(v, dict)}
    subs = {k: v for k, v in table.items() if isinstance(v, dict)}
    if scalars:
        lines.append(f"\n[{'.'.join(path)}]")
        for k, v in scalars.items():
            lines.append(f"{k} = {toml_val(v)}")
    for k, v in subs.items():
        emit(lines, path + [k], v)


def write(image: Path, name: str, root: str, table: dict, inputs):
    lines = [f"# Output of analysis/measure_focus_ring.py (its docstring is the method), file {name}.",
             "# input_image: every file this part reads, relative to the image folder.",
             "input_image = [" + ", ".join(toml_val(i) for i in inputs) + "]"]
    emit(lines, [root], table)
    (image / name).write_text("\n".join(lines) + "\n")
    tomllib.loads((image / name).read_text())


def main():
    if len(sys.argv) != 2:
        print(__doc__.split("\n\n")[1], file=sys.stderr)
        return 2
    image = Path(sys.argv[1])
    focus_dir = image / "button-pass-focus"
    probe = tomllib.loads((focus_dir / "probe.toml").read_text())
    ring, inputs = ring_toml(focus_dir, probe)
    write(image, "focus-ring.toml", "focus_ring", ring,
          ["button-pass-focus/probe.toml"] + [f"button-pass-focus/{i}" for i in inputs])
    transition, tin = transition_toml(image)
    if transition:
        write(image, "focus-transition.toml", "focus_transition", transition, tin)
    return 0


if __name__ == "__main__":
    sys.exit(main())
