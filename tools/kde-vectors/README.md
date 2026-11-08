# kde-vectors

Generates the vector tables of the KDE platform functions (specification S2.2 *Checked against their references*)
by the `compiled_source` route, under `native-theme/tests/vectors/`: `kirigami_lerp.tsv`, `kde_shade.tsv`,
`kcu_mix.tsv`, `kcu_tint.tsv`, `kcu_shade.tsv`, `kcu_overlay.tsv`, `kirigami_tint.tsv`, `kde_effects.tsv` and
`kde_selection_inactive.tsv`; and, by the `public_api` route, the KDE reader's `reader.qfont_from_string.tsv`:
`QFont::fromString` called through the system's Qt 6.12.0 (`qfont.cpp`) on the strings Qt 6.12 writes for Plasma's
default fonts (`plasma-integration@6.8.2`) and typical choices, Qt 5's strings and each field's edge cases, each
result read through `QFont`'s getters and `resolveMask()` (its string cells JSON string literals).

`generate.py` reads each upstream file from the source cache (`scripts/update_sources.sh`) as the commit
`docs/facts/sources.toml` pins holds it, through git's objects (`tools/pinned.py`): Qt's `qcolor.cpp`, Kirigami's
`colorutils.cpp`, KColorScheme's `kcolorscheme.cpp`, KGuiAddons' `kcolorutils.cpp` and `kcolorspaces.cpp`, with the
headers beside them, and Breeze's `BreezeLight.colors` and `BreezeDark.colors` for the inputs. It compiles the code
unchanged with `harness.cpp`, which calls the functions, and writes each result unrounded. `KColorUtils::overlayColors`
paints with the system's Qt 6.12.0 `QPainter`, whose raster engine is not compiled from the pin. KColorScheme's effects
and its Selection set's inactive start are called through an in-memory KConfig holding the tokens and colours. `stubs/` stands in for the headers the projects' own builds generate (export and logging macros). `moc`,
the compiler, the linker and the harnesses run under bubblewrap with no network, writing only the build folder.

The reference build targets the x86-64 baseline (`-O2 -march=x86-64 -mtune=generic`), the target of the x86-64
packages `sources.toml`'s `arch` fields name, which has no fused multiply-add; a second build with `-mfma` finds the
inputs on which a build fusing `QColor::toRgb`'s `1.0f - (s * f)` gives another colour, and the table holds some of
them.

Requires g++, bubblewrap, `pkg-config`, Qt 6.12.0's development headers (private ones included) and `moc`, and the KF6
Config 6.31 headers and libraries; it refuses another Qt or KConfig. Run from the repository root, the source cache's
folder optional:

    python3 -I tools/kde-vectors/generate.py [source cache]
