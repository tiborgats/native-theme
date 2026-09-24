# Bundled Presets

TOML theme specifications loaded via `Theme::preset("name")`. Each file
defines a complete or near-complete theme with light and dark variants.

## Two-tier system

### Regular presets

Standalone theme specifications: colors, fonts, geometry and
platform-specific values like `button_order`. Any app can load a regular
preset and get a resolved theme without needing a platform reader.

Gated sizes (padding sides, menu and list row heights, toolbar `bar_height`
and `item_gap`, combobox `arrow_area_width`): a native preset states one only
where `docs/platform-facts.md` gives it for its platform; the color-scheme
presets, `material` and `ios`, which have no platform-facts column, state none
(`documented_sizes.rs` checks both). An unstated gated size resolves to `None`
and the toolkit's own applies. The presets' other sizes (`min_height_px`,
`max_width_px`, …) are stated, often without a source, and are still being
audited (`docs/todo.md`, Table B); the color-scheme presets state many of them
too.

Files: `kde-breeze.toml`, `macos-sonoma.toml`, `windows-11.toml`,
`adwaita.toml`, `material.toml`, `ios.toml`, and the community presets
(`catppuccin-*.toml`, `dracula.toml`, `gruvbox.toml`, `nord.toml`,
`one-dark.toml`, `solarized.toml`, `tokyo-night.toml`).

### Live presets (`*-live.toml`)

Base layers paired with the platform readers, the crate-internal
`ThemeReader` implementations `KdeReader`, `GnomeReader` (and
`GnomePortalKdeReader`), `MacosReader` and `WindowsReader`, which
`SystemTheme::from_system()` selects. Live presets **omit** fields that the
reader provides at runtime from the operating system. The reader merges
live OS values onto the preset base, then the resolve pipeline fills any
remaining gaps.

Files: `kde-breeze-live.toml`, `macos-sonoma-live.toml`,
`windows-11-live.toml`, `adwaita-live.toml`.

## Preset file listing

| File | Description |
|------|-------------|
| `adwaita.toml` | GNOME/Adwaita standalone theme |
| `adwaita-live.toml` | GNOME/Adwaita base for the GNOME reader |
| `catppuccin-frappe.toml` | Catppuccin Frappe color scheme |
| `catppuccin-latte.toml` | Catppuccin Latte color scheme |
| `catppuccin-macchiato.toml` | Catppuccin Macchiato color scheme |
| `catppuccin-mocha.toml` | Catppuccin Mocha color scheme |
| `dracula.toml` | Dracula color scheme |
| `gruvbox.toml` | Gruvbox color scheme |
| `ios.toml` | iOS/iPadOS standalone theme |
| `kde-breeze.toml` | KDE Breeze standalone theme |
| `kde-breeze-live.toml` | KDE Breeze base for the KDE reader |
| `macos-sonoma.toml` | macOS Sonoma standalone theme |
| `macos-sonoma-live.toml` | macOS Sonoma base for the macOS reader |
| `material.toml` | Material Design standalone theme |
| `nord.toml` | Nord color scheme |
| `one-dark.toml` | One Dark color scheme |
| `solarized.toml` | Solarized color scheme |
| `tokyo-night.toml` | Tokyo Night color scheme |
| `windows-11.toml` | Windows 11 standalone theme |
| `windows-11-live.toml` | Windows 11 base for the Windows reader |

## Reader-provided fields

Live presets should **not** contain these fields. They are filled at
runtime by platform readers or `resolve_platform_defaults`:

- **`button_order`** -- filled by `resolve_platform_defaults` based on
  detected desktop environment
- **`icon_theme`** -- filled by platform reader or
  `resolve_platform_defaults` from system icon settings
- **`font_dpi`** -- filled by the platform reader: KDE's from `forceFontDPI`,
  `Xft.dpi` or the display, GNOME's from `Xft.dpi` or the display, macOS's a
  fixed 72, and Windows' a fixed 96, since it reads its fonts at 96 DPI so
  that they resolve to logical pixels
- **`reduce_motion`**, **`high_contrast`**, **`reduce_transparency`**,
  **`text_scaling_factor`** -- filled by platform reader from OS
  accessibility settings (the macOS reader reports no text-scaling factor:
  its text size setting reaches only a few Apple apps,
  `docs/platform-facts.md` §2.1.7)
- **`icon_sizes`** -- filled by platform reader from filesystem lookup
