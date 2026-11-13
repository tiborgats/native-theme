# gnome-reader-vectors

Generates the vector tables of the GNOME reader's two LGPL-derived helpers (the 0.8.0 plan's D10), under
`native-theme/tests/vectors/`:

- `reader.pango_font_description.tsv`, by the `public_api` route: Pango's `pango_font_description_from_string`, called
  through the system's Pango 1.58.2 (`pango@1.58.2`, the pin; it refuses another) on the GNOME font settings'
  defaults (read from `gsettings-desktop-schemas@51.0`), typical values and the parser's edge cases, each result read
  through Pango's getters (`pango.c`).
- `reader.adw_accent_nearest.tsv`, by the `compiled_source` route: libadwaita's
  `adw_accent_color_nearest_from_rgba`, from `adw-accent-color.c` and `adw-color-utils.c` read from the source cache
  as the commit `docs/facts/sources.toml` pins holds them (`tools/pinned.py`), compiled unchanged with `accent.c`,
  which calls it. `stubs/config.h` stands in for the header libadwaita's build generates. The inputs: the nine
  accents, Plasma's accent palette (`plasma-workspace@6.8.2`), typical colours, adjacent `f32` colours across each
  hue and chroma threshold (found by bisection with the harness) and seeded random colours. The reference build
  targets the x86-64 baseline (no fused multiply-add); a second build with `-mfma` counts the rows it snaps
  differently.

A string cell of these tables is a JSON string literal (the `string` column type the tables propose, so an input may
be empty or hold surrounding whitespace). The compiler and the harnesses run under bubblewrap with no network,
writing only the build folder.

Requires gcc, bubblewrap, `pkg-config`, Pango 1.58.2's and GTK 4's development headers. Run from the repository root,
the source cache's folder optional:

    python3 -I tools/gnome-reader-vectors/generate.py [source cache]
