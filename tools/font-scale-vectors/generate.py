#!/usr/bin/env python3
"""The vector tables of `android_non_linear` and `android_non_linear_inverse` (specification S9.4's `AndroidNonLinear`,
S2.2 *Checked against their references*), by the `public_api` route: frameworks/base's FontScaleConverter at the
pinned tag, compiled unchanged and called through its public API.

Writes `native-theme/tests/vectors/android_non_linear.tsv` and `android_non_linear_inverse.tsv`: for each row, a
size in sp (in dp for the inverse) at a font scale, through `FontScaleConverter.forScale(fontScale)` and the
converter's `convertSpToDp` (`convertDpToSp`), or, where the scale has no converter, the size times (over) the
scale, as `TypedValue.applyDimension` (`deriveDimension`) converts sp at a density of one.

The inputs, all synthetic: every size of SIZES at every scale of SCALES, then RANDOM_ROWS rows of a seeded random
size and scale each. SCALES holds the factory's eight table scales, Android's font-size steps below them (0.85, 1),
scales just below and at the first curve's threshold, between two tables and beyond the last; SIZES the tables' sp,
sizes between and beyond them, Material 3's type scale's sizes, line heights and letter spacings (negative ones
among them) and zero. A size and a scale are f32s, passed to Java as the float literal they read back from.

Each Java file is read from the source cache (`scripts/update_sources.sh`) as the commit `docs/facts/sources.toml`
pins holds it, through git's objects (`tools/pinned.py`), and compiled as it is; `stubs/` stands in for the
annotations it imports, the aconfig-generated `Flags`, `android.os.Build` (an annotation's argument),
`android.graphics.Rect` (a method the harness never calls) and `ArrayUtils`' allocation helpers (as Ravenwood
allocates them on a host JVM), and `Harness.java` calls the API. The compiler and the harness run under bubblewrap
with no network, writing only the build folder (the harness nothing: it prints its results).

Requires a JDK (`javac` and `java`; the tables' header records the one that wrote them) and bubblewrap. Run from
the repository root, the source cache's folder optional:

    python3 -I tools/font-scale-vectors/generate.py [source cache]
"""

import random
import shutil
import struct
import sys
import tempfile
from decimal import Decimal
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(ROOT / "scripts"))
sys.path.insert(0, str(ROOT / "tools"))
import citations  # noqa: E402
import pinned  # noqa: E402

SOURCE = "frameworks-base@android-17.0.0_r1"
VECTORS = ROOT / "native-theme" / "tests" / "vectors"
SEED = 20261106
FUNCTIONS = ("android_non_linear", "android_non_linear_inverse")

# The Java files the harness's classes need (javac's `-verbose` over `Harness.java` with the tree on its source path).
FILES = [f"core/java/{path}" for path in [
    "android/content/res/FontScaleConverter.java",
    "android/content/res/FontScaleConverterFactory.java",
    "android/content/res/FontScaleConverterImpl.java",
    "android/util/ContainerHelpers.java",
    "android/util/EmptyArray.java",
    "android/util/MathUtils.java",
    "android/util/SparseArray.java",
    "android/util/UtilConfig.java",
    "com/android/internal/util/GrowingArrayUtils.java",
]]

# The factory's table scales; Android's font-size steps below them; scales below, at and just above the threshold
# the factory derives from its first table's key (1.03); between tables; beyond the last; small.
SCALES = [1.05, 1.1, 1.15, 1.2, 1.3, 1.5, 1.8, 2.0, 0.85, 1.0, 0.5, 1.02, 1.025, 1.03, 1.035, 1.04, 1.045, 1.06,
          1.25, 1.4, 1.65, 1.9, 1.95, 2.5, 3.0]
# The tables' sp; between, below and beyond them; Material 3's type scale (sizes, line heights, letter spacings);
# zero.
SIZES = [8, 10, 12, 14, 18, 20, 24, 30, 100, 1, 4, 6, 9, 11, 13, 15, 17, 19, 21, 27, 33, 50, 120, 250, 16, 22, 28,
         32, 36, 45, 57, 40, 44, 52, 64, 0.1, 0.15, 0.25, 0.4, 0.5, -0.25, -0.5, -1.5, -14, 0]
RANDOM_ROWS = 300


def f32(x: float) -> float:
    """x rounded to the nearest f32, as a double."""
    return struct.unpack("<f", struct.pack("<f", x))[0]


def f32_text(x: float) -> str:
    """The shortest decimal that reads back to the f32 x, written without an exponent."""
    for digits in range(1, 10):
        text = f"{x:.{digits}g}"
        if f32(float(text)) == x:
            text = format(Decimal(text), "f")
            return text if "." in text else f"{text}.0"
    return format(Decimal(x), "f")


def inputs() -> tuple:
    """The rows' inputs (size, scale), none listed twice, and the size of each group."""
    out = []
    for scale in SCALES:
        for size in SIZES:
            row = (f32(size), f32(scale))
            if row not in out:
                out.append(row)
    grid = len(out)
    rng = random.Random(SEED)
    while len(out) < grid + RANDOM_ROWS:
        row = (f32(rng.uniform(-20.0, 150.0)), f32(rng.uniform(0.5, 2.6)))
        if row not in out:
            out.append(row)
    return out, (grid, len(out) - grid)


def java_version() -> str:
    """The first line `java -version` prints."""
    for tool in ("javac", "java"):
        if shutil.which(tool) is None:
            sys.exit(f"{tool} not found: a JDK is required")
    ran = pinned.sandboxed(["java", "-version"], capture_output=True, text=True)
    lines = (ran.stderr or ran.stdout).splitlines()
    if ran.returncode != 0 or not lines:
        sys.exit(f"java -version failed:\n{ran.stderr}")
    return lines[0].strip()


def run_harness(cache: Path, calls: list) -> list:
    """The harness's results, one float per call (function, size, scale)."""
    with tempfile.TemporaryDirectory(prefix="font-scale-vectors-") as tmp:
        build = Path(tmp)
        sources = build / "src"
        pinned.materialise(cache, [(SOURCE, path) for path in FILES], sources)
        java_files = [str(sources / SOURCE / path) for path in FILES]
        stubs = sorted(str(p) for p in (HERE / "stubs").rglob("*.java"))
        classes = build / "classes"
        classes.mkdir()
        compiled = pinned.sandboxed(["javac", "-nowarn", "-encoding", "UTF-8", "-d", classes, *java_files, *stubs,
                                     HERE / "Harness.java"], writable=[build], capture_output=True, text=True)
        if compiled.returncode != 0:
            sys.exit(f"javac failed:\n{compiled.stdout}{compiled.stderr}")
        stdin = "".join(f"{fn}\t{f32_text(size)}\t{f32_text(scale)}\n" for fn, size, scale in calls)
        ran = pinned.sandboxed(["java", "-cp", classes, "Harness"], readable=[build], input=stdin,
                               capture_output=True, text=True)
        if ran.returncode != 0:
            sys.exit(f"the harness failed:\n{ran.stderr}")
    results = ran.stdout.split()
    if len(results) != len(calls):
        sys.exit(f"the harness answered {len(results)} rows of {len(calls)}")
    return [f32(float(result)) for result in results]


def main():
    if len(sys.argv) > 2:
        sys.exit(__doc__)
    cache = citations.cache_dir(sys.argv[1] if len(sys.argv) == 2 else None)
    rows, (grid, drawn) = inputs()
    java = java_version()
    calls = [(fn, size, scale) for fn in FUNCTIONS for size, scale in rows]
    results = iter(run_harness(cache, calls))
    for fn in FUNCTIONS:
        sp, dp = ("sp", "dp") if fn == FUNCTIONS[0] else ("dp", "sp")
        method = "convertSpToDp" if fn == FUNCTIONS[0] else "convertDpToSp"
        times = "times" if fn == FUNCTIONS[0] else "over"
        lines = [
            f"# function: {fn}",
            "# route: public_api",
            f"# source: {SOURCE}",
            "# generations: material-3",
            "# generator: tools/font-scale-vectors",
            "# inputs: size:font_size font_scale:number",
            "# outputs: result:font_size",
            f"# written by tools/font-scale-vectors/generate.py on {java}; do not edit.",
            f"# Each row: the size in {sp} at the font scale, `FontScaleConverter.forScale(font_scale)` and the",
            f"# converter's `{method}`, else the size {times} the scale, in {dp}, as `Float.toString` writes it;",
            "# the factory's tables restored before each row. The inputs are synthetic.",
            f"# Group 1 ({grid} rows): every size of the grid at every scale of the grid (the factory's table",
            "# scales, Android's 0.85 and 1, around the threshold, between the tables and beyond the last).",
            f"# Group 2 ({drawn} rows): a seeded random size in -20..150 and scale in 0.5..2.6 each (seed {SEED}).",
            "size\tfont_scale\tresult",
        ]
        for size, scale in rows:
            lines.append(f"{f32_text(size)}px\t{f32_text(scale)}\t{f32_text(next(results))}px")
        out = VECTORS / f"{fn}.tsv"
        out.write_text("\n".join(lines) + "\n", encoding="utf-8")
        print(f"{out.relative_to(ROOT)}: {len(rows)} rows")


if __name__ == "__main__":
    main()
