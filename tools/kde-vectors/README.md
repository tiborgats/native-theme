# kde-vectors

Generates the vector tables of the KDE platform functions (specification S2.2 *Checked against their references*)
by the `compiled_source` route, under `native-theme/tests/vectors/`: `kirigami_lerp.tsv`, `kde_shade.tsv`,
`kcu_mix.tsv`, `kcu_tint.tsv`, `kcu_shade.tsv`, `kcu_overlay.tsv`, `kirigami_tint.tsv`, `kde_effects.tsv` and
`kde_selection_inactive.tsv`; and, by the `public_api` route, the KDE reader's `reader.qfont_from_string.tsv`:
`QFont::fromString` called through the system's Qt 6.12.0 (`qfont.cpp`) on the strings Qt 6.12 writes for Plasma's
default fonts (`plasma-integration@6.8.2`) and typical choices, Qt 5's strings and each field's edge cases, each
result read through `QFont`'s getters and `resolveMask()` (its string cells JSON string literals); and, by the
`public_api` route through `readers.cpp`, the KDE reader's other two helpers' tables: `reader.kde_color_scheme.tsv`,
Plasma's dark-or-light test (kdeglobals' `[Colors:Window] BackgroundNormal` written by the system's KConfig 6.31,
`KColorScheme::createApplicationPalette` of it, Qt's `qGray` of the palette's window colour against the threshold
read from plasma-integration's and xdg-desktop-portal-kde's pinned files, each row checked against the installed
plasma-integration platform theme's `QStyleHints::colorScheme()`), and `reader.qxcb_xft_dpi.tsv`, the DPI Qt's xcb
plugin forces from the X resources (the system's Qt 6.12.0 on an Xvfb server whose root window's `RESOURCE_MANAGER`
holds them, read through `QXcbScreen::forcedDpi()`; each input also read back through `xrdb -query`).

By the `compiled_source` route through `accent.cpp`, the accent derivation's two tables, `kde_accent.tsv` and
`kde_accent_wm.tsv`: Plasma's colour scheme applier, `plasma-workspace@6.8.2 kcms/colors/colorsapplicator.cpp` and
`.h`, compiled unchanged and linked with the system's Qt 6.12.0, KConfig, KGuiAddons and KColorScheme 6.31, applied to
generated scheme files (Breeze 6.8.2's three and variants of them: tinting, the accent-coloured title bar, a `Header`
group or none, `[Inactive]` groups, missing groups and keys, translucent colours) and accents (Breeze's
`#00000000` among them), each case in a fresh kdeglobals that KColorScheme then reads back as the KDE reader reads
kdeglobals. Before writing them it re-runs the nine (scheme file, accent) pairs of `docs/facts/kde/accent.toml`
(SUSE-Cafe's file read from the cache, GPL-2.0, never written into a table) and stops on any fact that differs; it
also prints the constants of the two colour transforms the tint uses, as the system's Qt computed them
(`accent.cpp constants`), which the behavioural description quotes. Two runs write the same bytes.

By the `compiled_source` route through `icon.cpp`, `kirigami_label_icon_size.tsv`: Kirigami's
`IconSizes::sizeForLabels` and `roundedIconSize` (`kirigami@6.31.0 src/platform/units.cpp`, compiled unchanged with
the moc output of `units.h` under Kirigami's static-build define `KIRIGAMI_BUILD_TYPE_STATIC`, linked with the
system's Qt 6.12.0) on the offscreen platform at 96 DPI. A fractional height is the `QFontMetricsF` height of the
application font set to Noto Sans or Hack, Plasma's default and fixed-width fonts, at a point size (the Units object
follows the font through its `ApplicationFontChange` filter); an integer height is passed to `roundedIconSize`
directly, as `sizeForLabels` passes it. The rows: the facts' input first (Noto Sans 10 pt, 17.6875 px giving 16,
ADD-A141; the run stops if it differs), Plasma's other sizes, the integers at and beside each step boundary, zero,
negatives and large heights, each boundary's both sides from a sweep of both families, and seeded random fonts; each
font's result is checked against `roundedIconSize` of its truncated height. Two runs write the same bytes.

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
Config 6.31 headers and libraries; it refuses another Qt or KConfig. The readers' two tables also need the KF6
ColorScheme 6.31 headers and library (it refuses another), the plasma-integration platform theme, Qt's xcb plugin,
Xvfb and xrdb. The accent's two tables need the KF6 GuiAddons 6.31 and ColorScheme 6.31 headers and libraries and
QtDBus's headers (it refuses another KGuiAddons or KColorScheme). The icon table needs Qt's QML, Quick and Quick
Controls 2 headers and the Noto Sans and Hack fonts (it refuses a substituted family). Run from the repository root,
the source cache's folder optional (`--only-readers`: the readers' two tables alone; `--only-accent`: the accent's two
alone; `--only-icon`: the icon table alone):

    python3 -I tools/kde-vectors/generate.py [--only-readers | --only-accent | --only-icon] [source cache]
