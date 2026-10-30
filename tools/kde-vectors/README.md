# kde-vectors

Generates the vector tables of the KDE platform functions (specification S2.2 *Checked against their references*)
by the `compiled_source` route: `native-theme/tests/vectors/kirigami_lerp.tsv` and `kde_shade.tsv`.

`generate.py` fetches each upstream file at the commit `docs/facts/sources.toml` pins (Qt's `qcolor.cpp`, Kirigami's
`colorutils.cpp`, KColorScheme's `kcolorscheme.cpp`, KGuiAddons' `kcolorutils.cpp` and `kcolorspaces.cpp`, with the
headers beside them), compiles them unchanged with `harness.cpp`, which calls the functions, and writes each result
unrounded. `stubs/` stands in for the headers the projects' own builds generate (export and logging macros).

The sources are built with Arch Linux's x86-64 flags (`-O2 -march=x86-64 -mtune=generic`), the build `sources.toml`'s
`arch` fields name; a second build with `-mfma` finds the inputs on which a build fusing `QColor::toRgb`'s
`1.0f - (s * f)` gives another colour, and the table holds some of them.

Requires g++, `pkg-config`, Qt 6.12.0's development headers (private ones included) and `moc`, and the KF6 Config
6.31 headers and libraries; it refuses another Qt. Run from the repository root:

    python3 -I tools/kde-vectors/generate.py
