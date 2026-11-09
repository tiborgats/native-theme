#!/usr/bin/env python3
"""The vector table of `win_ui` (specification S9.4's `WinUi`, S2.2 *Checked against their references*; ADD-A141),
by the `compiled_source` route: WinUI's `TextFormatting::GetScaledFontSize` at the pinned tag, its lines compiled
unchanged with a harness that calls it.

Writes `native-theme/tests/vectors/win_ui.tsv`: for each row, a size in logical px at a text scale factor, through
`GetScaledFontSize(factor)` of a `TextFormatting` holding that size with its text scale factor enabled
(`microsoft-ui-xaml@2.5.1 src/dxaml/xcp/core/text/common/TextFormatting.cpp:181-204`).

The inputs, all synthetic: every size of SIZES at every factor of FACTORS, then RANDOM_ROWS rows of a seeded random
size and factor each. FACTORS holds the factor 1, factors above it in small and large steps and factors below it;
SIZES sizes at and below zero, below 1 px (which the function scales from 1 px), common text sizes, large ones, and
the f32s around `e^(18/e)` px, where the curve's weight reaches zero. A size and a factor are f32s, passed to the
harness as the decimal they read back from.

The function's lines are read from the source cache (`scripts/update_sources.sh`) as the commit
`docs/facts/sources.toml` pins holds the file, through git's objects (`tools/pinned.py`), checked to be the function
from its signature to its closing brace, and compiled as they are after `harness.cpp`'s stub of what they read
(`XFLOAT` and `TextFormatting`'s two fields). The reference build takes `float log(float)` for the lines'
`log(inputFontSize)`, as MSVC's global overloads give it (`harness.cpp`), and targets the x86-64 baseline with no
contraction (`-O2 -march=x86-64 -mtune=generic -ffp-contract=off`); a second build calls the C library's
`double log(double)` instead, and the header counts the rows on which the two differ. The compiler and the harness
run under bubblewrap with no network, writing only the build folder.

Requires g++ and bubblewrap. Run from the repository root, the source cache's folder optional:

    python3 -I tools/winui-vectors/generate.py [source cache]
"""

import math
import random
import struct
import subprocess
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

SOURCE = "microsoft-ui-xaml@2.5.1"
FILE = "src/dxaml/xcp/core/text/common/TextFormatting.cpp"
# The function's lines, 1-based and inclusive, and what the first and the last hold.
LINES = (181, 204)
FIRST = "XFLOAT TextFormatting::GetScaledFontSize(XFLOAT fontScale) const"
LAST = "}"
VECTORS = ROOT / "native-theme" / "tests" / "vectors"
SEED = 20261109
REFERENCE_FLAGS = ["-O2", "-march=x86-64", "-mtune=generic", "-ffp-contract=off"]

FACTORS = [1.0, 1.05, 1.1, 1.15, 1.2, 1.25, 1.3, 1.4, 1.5, 1.75, 2.0, 2.25, 2.5, 3.0, 0.5, 0.75, 0.9, 0.0]
SIZES = [-14, -1, -0.5, 0, 0.001, 0.25, 0.5, 0.75, 0.999, 1, 1.001, 1.5, 2, 4, 6, 8, 9, 10, 11, 12, 13, 14, 15, 16,
         17, 18, 20, 22, 24, 26, 28, 32, 36, 40, 42, 46, 48, 56, 60, 64, 68, 72, 80, 96, 120, 128, 160, 200, 256, 300,
         400, 500, 600, 700, 1000, 2000]
RANDOM_ROWS = 300


def f32(x: float) -> float:
    """x rounded to the nearest f32, as a double."""
    return struct.unpack("<f", struct.pack("<f", x))[0]


def f32_bits(x: float) -> int:
    return struct.unpack("<I", struct.pack("<f", x))[0]


def f32_from_bits(b: int) -> float:
    return struct.unpack("<f", struct.pack("<I", b))[0]


def f32_text(x: float) -> str:
    """The shortest decimal that reads back to the f32 x, written without an exponent."""
    for digits in range(1, 10):
        text = f"{x:.{digits}g}"
        if f32(float(text)) == x:
            text = format(Decimal(text), "f")
            return text if "." in text else f"{text}.0"
    return format(Decimal(x), "f")


def crossover() -> list:
    """The f32s around `e^(18/e)`, where `18 − e·ln(m)` reaches zero: three below it and three above."""
    at = f32(math.exp(18 / math.e))
    bits = f32_bits(at)
    return [f32_from_bits(b) for b in range(bits - 3, bits + 4)]


def inputs() -> tuple:
    """The rows' inputs (size, factor), none listed twice, and the size of each group."""
    sizes = [f32(s) for s in SIZES] + crossover()
    out = []
    for factor in FACTORS:
        for size in sizes:
            row = (size, f32(factor))
            if row not in out:
                out.append(row)
    grid = len(out)
    rng = random.Random(SEED)
    while len(out) < grid + RANDOM_ROWS:
        row = (f32(rng.uniform(-5.0, 200.0)), f32(rng.uniform(0.5, 3.0)))
        if row not in out:
            out.append(row)
    return out, (grid, len(out) - grid)


def function_lines(cache: Path) -> bytes:
    """The function's lines as the pinned commit holds them, checked to be the whole function."""
    lines = pinned.pinned(cache, SOURCE, FILE).splitlines(keepends=True)
    first, last = LINES
    chosen = lines[first - 1:last]
    if len(chosen) != last - first + 1:
        sys.exit(f"{FILE}: fewer than {last} lines")
    if chosen[0].decode("utf-8").strip() != FIRST or chosen[-1].decode("utf-8").strip() != LAST:
        sys.exit(f"{FILE}:{first}-{last} is not GetScaledFontSize from its signature to its brace")
    return b"".join(chosen)


def compiler_version() -> str:
    ran = subprocess.run(["g++", "--version"], capture_output=True, text=True, check=False)
    lines = ran.stdout.splitlines()
    if ran.returncode != 0 or not lines:
        sys.exit("g++ not found")
    return lines[0].strip()


def build(work: Path, name: str, defines) -> Path:
    exe = work / name
    compiled = pinned.sandboxed(["g++", "-std=c++17", *REFERENCE_FLAGS, *defines, f"-I{work}", "-o", exe,
                                 HERE / "harness.cpp"], writable=[work], capture_output=True, text=True)
    if compiled.returncode != 0:
        sys.exit(f"g++ failed:\n{compiled.stdout}{compiled.stderr}")
    return exe


def run(exe: Path, rows: list) -> list:
    stdin = "".join(f"{f32_text(size)} {f32_text(factor)}\n" for size, factor in rows)
    ran = pinned.sandboxed([exe], readable=[exe.parent], input=stdin, capture_output=True, text=True)
    if ran.returncode != 0:
        sys.exit(f"the harness failed:\n{ran.stderr}")
    results = ran.stdout.split()
    if len(results) != len(rows):
        sys.exit(f"the harness answered {len(results)} rows of {len(rows)}")
    return [f32(float(result)) for result in results]


def main():
    if len(sys.argv) > 2:
        sys.exit(__doc__)
    cache = citations.cache_dir(sys.argv[1] if len(sys.argv) == 2 else None)
    rows, (grid, drawn) = inputs()
    compiler = compiler_version()
    with tempfile.TemporaryDirectory(prefix="winui-vectors-") as tmp:
        work = Path(tmp)
        (work / "GetScaledFontSize.inc").write_bytes(function_lines(cache))
        reference = run(build(work, "harness", []), rows)
        double_log = run(build(work, "harness-double-log", ["-DDOUBLE_LOG"]), rows)
    differing = sum(1 for a, b in zip(reference, double_log) if a != b)
    first, last = LINES
    lines = [
        "# function: win_ui",
        "# route: compiled_source",
        f"# source: {SOURCE}",
        "# generations: windows-11",
        "# generator: tools/winui-vectors",
        "# inputs: size:font_size factor:number",
        "# outputs: result:font_size",
        f"# written by tools/winui-vectors/generate.py with {compiler}; do not edit.",
        f"# Each row: TextFormatting::GetScaledFontSize(factor) ({FILE}:{first}-{last},",
        "# compiled unchanged) of a TextFormatting holding the size, its text scale factor enabled, in logical px.",
        "# The reference build takes `float log(float)`, as MSVC's global overloads do; a build calling",
        f"# `double log(double)` differs on {differing} of these rows. The inputs are synthetic.",
        f"# Group 1 ({grid} rows): every size of the grid at every factor of the grid (the factor 1, above and",
        "# below it; sizes at and below zero, below 1 px, common and large, and around e^(18/e) px).",
        f"# Group 2 ({drawn} rows): a seeded random size in -5..200 and factor in 0.5..3 each (seed {SEED}).",
        "size\tfactor\tresult",
    ]
    for (size, factor), result in zip(rows, reference):
        lines.append(f"{f32_text(size)}px\t{f32_text(factor)}\t{f32_text(result)}px")
    path = VECTORS / "win_ui.tsv"
    path.write_text("".join(line + "\n" for line in lines), encoding="utf-8")
    print(f"{path.name}: {len(rows)} rows, {differing} differing under the double log")


if __name__ == "__main__":
    main()
