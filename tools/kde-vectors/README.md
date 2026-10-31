# kde-vectors

Generates the vector tables of the KDE platform functions (specification S2.2 *Checked against their references*)
by the `compiled_source` route: `native-theme/tests/vectors/kirigami_lerp.tsv` and `kde_shade.tsv`.

`generate.py` reads each upstream file from the source cache (`scripts/update_sources.sh`) as the commit
`docs/facts/sources.toml` pins holds it, through git's objects (`tools/pinned.py`): Qt's `qcolor.cpp`, Kirigami's
`colorutils.cpp`, KColorScheme's `kcolorscheme.cpp`, KGuiAddons' `kcolorutils.cpp` and `kcolorspaces.cpp`, with the
headers beside them. It compiles them unchanged with `harness.cpp`, which calls the functions, and writes each result
unrounded. `stubs/` stands in for the headers the projects' own builds generate (export and logging macros). `moc`,
the compiler, the linker and the harnesses run under bubblewrap with no network, writing only the build folder.

The reference build targets the x86-64 baseline (`-O2 -march=x86-64 -mtune=generic`), the target of the x86-64
packages `sources.toml`'s `arch` fields name, which has no fused multiply-add; a second build with `-mfma` finds the
inputs on which a build fusing `QColor::toRgb`'s `1.0f - (s * f)` gives another colour, and the table holds some of
them.

Requires g++, bubblewrap, `pkg-config`, Qt 6.12.0's development headers (private ones included) and `moc`, and the KF6
Config 6.31 headers and libraries; it refuses another Qt or KConfig. Run from the repository root, the source cache's
folder optional:

    python3 -I tools/kde-vectors/generate.py [source cache]
