#!/usr/bin/env python3
"""The vector table of `m3_role` (specification S2.2 *Checked against their references*), by the `public_api` route:
material-color-utilities' Java code at the pinned commit, compiled unchanged and called through its public API.

Writes `native-theme/tests/vectors/m3_role.tsv`: for each row, the role `MaterialDynamicColors` names
(`allDynamicColors()`, each `DynamicColor` by its `name`) resolved by `DynamicColor.getArgb` in `new
SchemeTonalSpot(Hct.fromInt(seed), isDark, contrastLevel, SpecVersion.SPEC_2021, Platform.PHONE)`, the scheme the
Material facts' `token.material` states (`variant = "tonal_spot"`, `spec_version = "spec_2021"`), at the facts'
contrast levels and others.

The inputs: every role the Material facts call `m3_role` with (`docs/facts/material/`), each over a set of synthetic
seeds (the sRGB primaries, a yellow, a grey and a violet of middle lightness) in both schemes at the standard,
medium and high levels (0, 0.5, 1) and two of them at the reduced levels (-1, -0.5); then rows of a seeded random
role, seed, scheme and level each. A seed is a byte colour (the model's `input.accent` enters the scheme as the
source's ARGB integer), a level an f32 (the model's number), passed to Java as the double it widens to.

Each Java file is read from the source cache (`scripts/update_sources.sh`) as the commit `docs/facts/sources.toml`
pins holds it, through git's objects (`tools/pinned.py`), and compiled as it is; `stubs/` stands in for the
annotations it imports (AndroidX's and Error Prone's), and `Harness.java` calls the API. The compiler and the
harness run under bubblewrap with no network, writing only the build folder (the harness nothing: it prints its
results).

Requires a JDK (`javac` and `java`; the table's header records the one that wrote it) and bubblewrap. Run from the
repository root, the source cache's folder optional:

    python3 -I tools/m3-vectors/generate.py [source cache]
"""

import random
import re
import shutil
import struct
import sys
import tempfile
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(ROOT / "scripts"))
sys.path.insert(0, str(ROOT / "tools"))
import citations  # noqa: E402
import pinned  # noqa: E402

SOURCE = "material-color-utilities@bd811e4c"
VECTORS = ROOT / "native-theme" / "tests" / "vectors"
FACTS = ROOT / "docs" / "facts" / "material"
SEED = 20261106

# The Java files the harness's classes need (javac's `-verbose` over `Harness.java` with the tree on its source path).
FILES = [f"java/{path}" for path in [
    "contrast/Contrast.java",
    "dislike/DislikeAnalyzer.java",
    "dynamiccolor/ColorSpec.java",
    "dynamiccolor/ColorSpec2021.java",
    "dynamiccolor/ColorSpec2025.java",
    "dynamiccolor/ColorSpec2026.java",
    "dynamiccolor/ColorSpecs.java",
    "dynamiccolor/ContrastCurve.java",
    "dynamiccolor/DynamicColor.java",
    "dynamiccolor/DynamicScheme.java",
    "dynamiccolor/MaterialDynamicColors.java",
    "dynamiccolor/ToneDeltaPair.java",
    "dynamiccolor/TonePolarity.java",
    "dynamiccolor/Variant.java",
    "hct/Cam16.java",
    "hct/Hct.java",
    "hct/HctSolver.java",
    "hct/ViewingConditions.java",
    "palettes/TonalPalette.java",
    "scheme/SchemeTonalSpot.java",
    "temperature/TemperatureCache.java",
    "utils/ColorUtils.java",
    "utils/MathUtils.java",
]]

# Synthetic seeds (0xRRGGBB): the sRGB primaries, a yellow, a grey and a violet of middle lightness.
FIXED_SEEDS = [0xFF0000, 0x00FF00, 0x0000FF, 0xFFFF00, 0x808080, 0x6750A4]
STANDARD_LEVELS = [0.0, 0.5, 1.0]
REDUCED_SEEDS = [0x0000FF, 0x6750A4]
REDUCED_LEVELS = [-1.0, -0.5]
RANDOM_ROWS = 400
SCHEMES = ["light", "dark"]


def roles() -> list:
    """The roles the Material facts call `m3_role` with, sorted."""
    out = set()
    for path in sorted(FACTS.glob("*.toml")):
        data = tomllib.loads(path.read_text(encoding="utf-8"))
        for fact in data.get("fact", []):
            value = fact.get("value")
            if isinstance(value, str):
                out.update(re.findall(r"\bm3_role\(@([a-z0-9_]+)\)", value))
    if not out:
        sys.exit(f"{FACTS}: no m3_role call")
    return sorted(out)


def f32(x: float) -> float:
    """x rounded to the nearest f32, as a double."""
    return struct.unpack("<f", struct.pack("<f", x))[0]


def f32_text(x: float) -> str:
    """The shortest decimal that reads back to the f32 x."""
    for digits in range(1, 10):
        text = f"{x:.{digits}g}"
        if f32(float(text)) == x:
            return text if "e" in text or "." in text else f"{text}.0"
    return repr(x)


def inputs(names: list) -> tuple:
    """The rows' inputs (role, seed, level, scheme), none listed twice, and the size of each group."""
    out = []

    def add(role, seed, level, scheme):
        if (role, seed, level, scheme) not in out:
            out.append((role, seed, level, scheme))

    for seed in FIXED_SEEDS:
        for scheme in SCHEMES:
            for level in STANDARD_LEVELS:
                for role in names:
                    add(role, seed, level, scheme)
    fixed = len(out)
    for seed in REDUCED_SEEDS:
        for scheme in SCHEMES:
            for level in REDUCED_LEVELS:
                for role in names:
                    add(role, seed, level, scheme)
    reduced = len(out) - fixed
    rng = random.Random(SEED)
    for _ in range(RANDOM_ROWS):
        add(rng.choice(names), rng.randrange(0x1000000), f32(rng.uniform(-1.0, 1.0)), rng.choice(SCHEMES))
    return out, (fixed, reduced, len(out) - fixed - reduced)


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


def run_harness(cache: Path, rows: list) -> list:
    """The harness's results, one ARGB integer per row."""
    with tempfile.TemporaryDirectory(prefix="m3-vectors-") as tmp:
        build = Path(tmp)
        sources = build / "src"
        pinned.materialise(cache, [(SOURCE, path) for path in FILES], sources)
        tree = sources / SOURCE / "java"
        java_files = [str(tree / path.removeprefix("java/")) for path in FILES]
        stubs = sorted(str(p) for p in (HERE / "stubs").rglob("*.java"))
        classes = build / "classes"
        classes.mkdir()
        compiled = pinned.sandboxed(["javac", "-nowarn", "-encoding", "UTF-8", "-d", classes, *java_files, *stubs,
                                     HERE / "Harness.java"], writable=[build], capture_output=True, text=True)
        if compiled.returncode != 0:
            sys.exit(f"javac failed:\n{compiled.stdout}{compiled.stderr}")
        stdin = "".join(f"{role}\t{seed:06x}\t{level!r}\t{scheme}\n" for role, seed, level, scheme in rows)
        ran = pinned.sandboxed(["java", "-cp", classes, "Harness"], readable=[build], input=stdin,
                               capture_output=True, text=True)
        if ran.returncode != 0:
            sys.exit(f"the harness failed:\n{ran.stderr}")
    results = ran.stdout.split()
    if len(results) != len(rows):
        sys.exit(f"the harness answered {len(results)} rows of {len(rows)}")
    return [int(argb, 16) for argb in results]


def colour(argb: int) -> str:
    """An ARGB integer as `srgb(r, g, b, a)`, each channel its byte over 255."""
    channels = [(argb >> shift) & 0xFF for shift in (16, 8, 0, 24)]
    return "srgb(" + ", ".join(repr(c / 255) for c in channels) + ")"


def main():
    if len(sys.argv) > 2:
        sys.exit(__doc__)
    cache = citations.cache_dir(sys.argv[1] if len(sys.argv) == 2 else None)
    names = roles()
    rows, (fixed, reduced, drawn) = inputs(names)
    java = java_version()
    results = run_harness(cache, rows)
    lines = [
        "# function: m3_role",
        "# route: public_api",
        f"# source: {SOURCE}",
        "# generations: material-3",
        "# generator: tools/m3-vectors",
        "# inputs: role:name accent:colour variant:enum<MaterialVariant> spec_version:name contrast_level:number"
        " scheme:enum<Scheme>",
        "# outputs: result:colour",
        f"# written by tools/m3-vectors/generate.py on {java}; do not edit.",
        "# Each row: `MaterialDynamicColors`' role of that name,",
        "# `DynamicColor.getArgb` in `new SchemeTonalSpot(Hct.fromInt(accent), scheme == dark, contrast_level,",
        "# SpecVersion.SPEC_2021, Platform.PHONE)`, its bytes over 255. The operands after the role are those m3_role",
        "# reads itself (input.accent, token.material, the scheme). The seeds and levels are synthetic; the roles are",
        "# those the Material facts call. A row already listed is not repeated.",
        f"# Group 1 ({fixed} rows): every role over {len(FIXED_SEEDS)} seeds (the sRGB primaries, a yellow, a grey, a",
        "# violet of middle lightness) in both schemes at the levels 0, 0.5 and 1.",
        f"# Group 2 ({reduced} rows): every role over the blue and the violet in both schemes at the levels -1 and -0.5.",
        f"# Group 3 ({drawn} rows): a seeded random role, seed, scheme and f32 level in -1..1 each (seed {SEED}).",
        "role\taccent\tvariant\tspec_version\tcontrast_level\tscheme\tresult",
    ]
    for (role, seed, level, scheme), argb in zip(rows, results):
        lines.append("\t".join([f"@{role}", f"#{seed:06x}", "tonal_spot", "spec_2021", f32_text(level), scheme,
                                colour(argb)]))
    out = VECTORS / "m3_role.tsv"
    out.write_text("\n".join(lines) + "\n", encoding="utf-8")
    print(f"{out.relative_to(ROOT)}: {len(rows)} rows")


if __name__ == "__main__":
    main()
