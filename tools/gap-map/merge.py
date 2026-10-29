"""Build the gap map's cross-platform tables from the six tags-*.tsv files.

Usage: python3 -I merge.py <scratchpad-dir>  -> prints markdown on stdout,
and a consistency report on stderr.
"""
import collections
import re
import sys
from pathlib import Path

PLATFORMS = [
    ("MACOS", "macOS", "macos"),
    ("WINDOWS", "Windows", "windows"),
    ("KDE", "KDE", "kde"),
    ("GNOME", "GNOME", "gnome"),
    ("MATERIAL", "Material", "material"),
    ("IOS", "iOS", "ios"),
]
CAPS = [f"C{i:02d}" for i in range(1, 29)]


def load(d):
    rows = []
    for key, _, slug in PLATFORMS:
        p = d / f"tags-{slug}.tsv"
        for n, line in enumerate(p.read_text().splitlines(), 1):
            if not line.strip():
                continue
            f = line.split("\t")
            if len(f) != 7:
                print(f"{p.name}:{n}: {len(f)} fields", file=sys.stderr)
                continue
            plat, rid, sec, elem, caps, src, summ = (x.strip() for x in f)
            if plat != key:
                print(f"{p.name}:{n}: platform {plat!r}", file=sys.stderr)
            cl = [c.strip() for c in caps.split(",") if c.strip()]
            bad = [c for c in cl if c not in CAPS]
            if bad or not cl:
                print(f"{p.name}:{n}: caps {caps!r}", file=sys.stderr)
            if sec not in ("4.1", "4.2", "4.3"):
                print(f"{p.name}:{n}: section {sec!r}", file=sys.stderr)
            rows.append(dict(plat=key, id=rid, sec=sec, elem=elem, caps=cl, src=src, summ=summ))
    return rows


def ids(lst):
    return ", ".join(sorted(lst, key=lambda s: [int(t) if t.isdigit() else t for t in re.split(r"(\d+)", s)]))


def main():
    d = Path(sys.argv[1])
    rows = load(d)
    names = {k: n for k, n, _ in PLATFORMS}
    out = []
    # per-platform totals
    tot = collections.Counter((r["plat"], r["sec"]) for r in rows)
    out.append("| Platform | §4.1 | §4.2 | §4.3 | Total |")
    out.append("|---|---:|---:|---:|---:|")
    for k, n, _ in PLATFORMS:
        a, b, c = (tot[(k, s)] for s in ("4.1", "4.2", "4.3"))
        out.append(f"| {n} | {a} | {b} | {c} | {a + b + c} |")
    out.append(f"| **All** | {sum(tot[(k, '4.1')] for k, *_ in PLATFORMS)} | "
               f"{sum(tot[(k, '4.2')] for k, *_ in PLATFORMS)} | "
               f"{sum(tot[(k, '4.3')] for k, *_ in PLATFORMS)} | {len(rows)} |")
    out.append("")
    # capability matrix
    cell = collections.defaultdict(list)
    for r in rows:
        for c in r["caps"]:
            cell[(c, r["plat"])].append(r["id"])
    out.append("CAPMATRIX")
    out.append("| Capability | " + " | ".join(n for _, n, _ in PLATFORMS) + " |")
    out.append("|---|" + "---:|" * len(PLATFORMS))
    for c in CAPS:
        out.append(f"| {c} | " + " | ".join(str(len(cell[(c, k)])) for k, *_ in PLATFORMS) + " |")
    out.append("")
    out.append("CAPROWS")
    for c in CAPS:
        out.append(f"#### {c}")
        out.append("")
        for k, n, _ in PLATFORMS:
            if cell[(c, k)]:
                out.append(f"- {n}: {ids(cell[(c, k)])}")
        out.append("")
    # elements of 4.1
    el = collections.defaultdict(list)
    for r in rows:
        if r["sec"] == "4.1":
            el[(r["elem"], r["plat"])].append(r["id"])
    elems = sorted({e for e, _ in el})
    out.append("ELEMENTS")
    out.append("| Model section | " + " | ".join(n for _, n, _ in PLATFORMS) + " |")
    out.append("|---|" + "---:|" * len(PLATFORMS))
    for e in elems:
        out.append(f"| `{e}` | " + " | ".join(str(len(el[(e, k)])) for k, *_ in PLATFORMS) + " |")
    out.append("")
    # controls of 4.2
    ct = collections.defaultdict(list)
    label = {}
    for r in rows:
        if r["sec"] == "4.2":
            key = r["elem"].lower()
            label.setdefault(key, r["elem"])
            ct[(key, r["plat"])].append(r["id"])
    ctl = sorted({e for e, _ in ct}, key=lambda e: (-sum(1 for k, *_ in PLATFORMS if ct[(e, k)]), e))
    out.append("CONTROLS")
    out.append("| Control | " + " | ".join(n for _, n, _ in PLATFORMS) + " |")
    out.append("|---|" + "---|" * len(PLATFORMS))
    for e in ctl:
        out.append(f"| {label[e]} | " + " | ".join(ids(ct[(e, k)]) or "–" for k, *_ in PLATFORMS) + " |")
    print("\n".join(out))


main()
