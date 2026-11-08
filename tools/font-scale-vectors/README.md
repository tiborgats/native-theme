# font-scale-vectors

Generates the vector tables of `android_non_linear` and `android_non_linear_inverse` (specification S9.4's
`AndroidNonLinear`, S2.2 *Checked against their references*) by the `public_api` route, under
`native-theme/tests/vectors/`: `android_non_linear.tsv` and `android_non_linear_inverse.tsv`, Android's non-linear
font scaling as frameworks/base's code at the pinned tag gives it.

`generate.py` reads each Java file from the source cache (`scripts/update_sources.sh`) as the commit
`docs/facts/sources.toml` pins holds it, through git's objects (`tools/pinned.py`), and compiles the code unchanged
with `Harness.java`, which calls the public API as `android.util.TypedValue` does: `FontScaleConverter.forScale(
fontScale)`, then the converter's `convertSpToDp` or `convertDpToSp`, or the size times or over the scale where the
scale has no converter. The factory caches the converters it builds; the harness restores its tables before each
row, so a row is what a process asking for that scale first gets. The sizes and scales are synthetic. `stubs/`
stands in for what the code imports from outside the files compiled: the annotations, the `Flags` class aconfig
generates, `android.os.Build` (an annotation's argument), `android.graphics.Rect` (a method the harness never calls)
and `ArrayUtils`' allocation helpers, as Ravenwood allocates them on a host JVM. The compiler and the harness run
under bubblewrap with no network, writing only the build folder.

Requires a JDK (`javac` and `java`, 21 or later) and bubblewrap. Run from the repository root, the source cache's
folder optional:

    python3 -I tools/font-scale-vectors/generate.py [source cache]
