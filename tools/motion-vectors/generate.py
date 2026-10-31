#!/usr/bin/env python3
"""The vectors of `timing::evaluate`'s cubic Bézier, steps and spring readings and of `ripple::progress`,
`ripple::phase` and `ripple::shade` (specification S2.7, S2.12), from the sources' own code. Offline.

`timing`: libadwaita's `ease_cubic_bezier` with its `x_for_t`, `y_for_t` and `t_for_x` (`adw-easing.c:477-541`) and
`AdwSpringAnimation`'s `oscillate`, `get_first_zero` and `calculate_duration` (`adw-spring-animation.c:101-243`), copied
from the source cache at `libadwaita@1.10.0` unchanged and compiled with `timing.c`, which states the struct and the
spring parameters' getters those functions read (`adw-spring-params.c:85-96`'s damping from the ratio) and calls them
as `adw_spring_animation_real_calculate_value` does (`:265-282`); CSS's step easing function (CSS Easing Functions
Level 1, section 2.3.2), which no source code states, is `timing.c`'s transcription of the specification's steps.

`ripple`: `RippleShader.java`'s uniform setters (`setRadius`, `setNoisePhase`, `setResolution`, `:138-184`, `:194-198`) copied
from `frameworks-base@android-17.0.0_r1` unchanged into `Ripple.java`, which records the uniforms; the shader's AGSL
(`:42-122`) transcribed there statement by statement in `float`, a GLSL builtin as GLSL defines it; the lifecycle's
constants and curves from `RippleAnimationSession.java:42-48`, `:113-139`, `:150-220`; the (0.4, 0, 0.2, 1) curve,
which `PathInterpolator` approximates through the native `Path.approximate` outside the cache, solved exactly by
bisection to the double's precision.

Requires gcc, `pkg-config` with GLib's headers, and a JDK (javac, java). Prints the rows, which the tests in
`native-theme/src/next/blocks/part_tests.rs` (`timing_*`) and `native-theme/src/next/ripple.rs` (`ripple_*`) hold.
Run from the repository root:

    python3 -I tools/motion-vectors/generate.py [source cache]
"""

import os
import re
import subprocess
import sys
import tempfile
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE.parent.parent / "scripts"))
import citations  # noqa: E402


def lines(path: Path, first: int, last: int) -> str:
    return "\n".join(path.read_text(encoding="utf-8").splitlines()[first - 1:last]) + "\n"


def run_timing(cache: Path, work: Path) -> str:
    src = cache / "libadwaita@1.10.0" / "src"
    easing = lines(src / "adw-easing.c", 477, 541)
    spring = lines(src / "adw-spring-animation.c", 101, 243)
    harness = (HERE / "timing.c").read_text(encoding="utf-8")
    program = harness.replace("/* ADW_EASING */", easing).replace("/* ADW_SPRING */", spring)
    c = work / "timing.c"
    c.write_text(program, encoding="utf-8")
    flags = subprocess.run(["pkg-config", "--cflags", "glib-2.0"], check=True, capture_output=True,
                           text=True).stdout.split()
    libs = subprocess.run(["pkg-config", "--libs", "glib-2.0"], check=True, capture_output=True,
                          text=True).stdout.split()
    exe = work / "timing"
    subprocess.run(["gcc", "-O2", "-march=x86-64", "-mtune=generic", *flags, str(c), "-o", str(exe), *libs, "-lm"],
                   check=True)
    return subprocess.run([str(exe)], check=True, capture_output=True, text=True).stdout


def run_ripple(cache: Path, work: Path) -> str:
    shader = cache / "frameworks-base@android-17.0.0_r1" / "graphics/java/android/graphics/drawable/RippleShader.java"
    setters = lines(shader, 124, 125) + lines(shader, 138, 184) + lines(shader, 194, 198)
    harness = (HERE / "Ripple.java").read_text(encoding="utf-8")
    java = work / "Ripple.java"
    java.write_text(harness.replace("    /* RIPPLE_SHADER */", setters), encoding="utf-8")
    subprocess.run(["javac", "-d", str(work), str(java)], check=True)
    return subprocess.run(["java", "-cp", str(work), "Ripple"], check=True, capture_output=True,
                          text=True).stdout


def main() -> int:
    cache = citations.cache_dir(sys.argv[1] if len(sys.argv) > 1 else None)
    with tempfile.TemporaryDirectory() as tmp:
        work = Path(tmp)
        sys.stdout.write(run_timing(cache, work))
        sys.stdout.write(run_ripple(cache, work))
    return 0


if __name__ == "__main__":
    sys.exit(main())
