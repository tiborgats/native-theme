# Bundled Presets

TOML theme specifications loaded via `Theme::preset("name")`. Each file
defines a complete or near-complete theme with light and dark variants.

## Presets and platform readers

Each preset is a standalone theme specification: colors, fonts, geometry and
platform-specific values like `button_order`. Any app can load one and get a
resolved theme without needing a platform reader. `SystemTheme::from_system()`
selects a platform reader, one of the crate-internal `ThemeReader`
implementations `KdeReader`, `GnomeReader` (and `GnomePortalKdeReader`),
`MacosReader` and `WindowsReader`, and merges the values it reads from the
operating system over its platform preset (`kde-breeze`, `adwaita`,
`macos-sonoma`, `windows-11`); a field the reader does not provide keeps the
preset's value, and the resolve pipeline fills any remaining gaps.

Gated sizes (padding sides, menu and list row heights, toolbar `bar_height`
and `item_gap`, combobox `arrow_area_width`): a native preset states one only
where `docs/platform-facts.md` gives it for its platform: KDE, GNOME, macOS,
Windows or Material, whose column counts a material-web or Compose
implementation constant cited at a pinned line as documented, as the KDE and
GNOME columns count a Breeze or libadwaita source constant. The color-scheme
presets, and the `ios` platform preset, which has no platform-facts column,
state none (`documented_sizes.rs` checks both). An
unstated gated size resolves to `None` and the toolkit's own applies. The
presets' other sizes (`min_height_px`, `max_width_px`, …) are stated,
often without a source, and are still being audited (`docs/todo.md`,
Table B); the color-scheme presets state many of them too.

Files: `kde-breeze.toml`, `macos-sonoma.toml`, `windows-11.toml`,
`adwaita.toml`, `material-3.toml`, `ios.toml`, and the community presets
(`catppuccin-*.toml`, `dracula.toml`, `gruvbox.toml`, `nord.toml`,
`one-dark.toml`, `solarized.toml`, `tokyo-night.toml`).

## Preset file listing

| File | Description |
|------|-------------|
| `adwaita.toml` | GNOME/Adwaita standalone theme |
| `catppuccin-frappe.toml` | Catppuccin Frappe color scheme |
| `catppuccin-latte.toml` | Catppuccin Latte color scheme |
| `catppuccin-macchiato.toml` | Catppuccin Macchiato color scheme |
| `catppuccin-mocha.toml` | Catppuccin Mocha color scheme |
| `dracula.toml` | Dracula color scheme |
| `gruvbox.toml` | Gruvbox color scheme |
| `ios.toml` | iOS/iPadOS standalone theme |
| `kde-breeze.toml` | KDE Breeze standalone theme |
| `macos-sonoma.toml` | macOS Sonoma standalone theme |
| `material-3.toml` | Material Design standalone theme |
| `nord.toml` | Nord color scheme |
| `one-dark.toml` | One Dark color scheme |
| `solarized.toml` | Solarized color scheme |
| `tokyo-night.toml` | Tokyo Night color scheme |
| `windows-11.toml` | Windows 11 standalone theme |

## What each reader supplies

The platform readers, or `resolve_platform_defaults`, supply these fields
over the platform preset:

- **`button_order`** -- filled by `resolve_platform_defaults` based on
  detected desktop environment
- **`icon_theme`** -- stated at the Theme level by the adwaita, macOS and
  Windows presets; KDE's reader supplies it per variant
- **`font_dpi`** -- filled by the platform reader: KDE's from `forceFontDPI`,
  `Xft.dpi` or the display, GNOME's from `Xft.dpi` or the display, macOS's a
  fixed 72, and Windows' a fixed 96, since it reads its fonts at 96 DPI so
  that they resolve to logical pixels
- **`reduce_motion`**, **`high_contrast`**, **`reduce_transparency`**,
  **`text_scaling_factor`** -- filled by platform reader from OS
  accessibility settings (the macOS reader reports no text-scaling factor:
  its text size setting reaches only a few Apple apps,
  `docs/platform-facts.md` §2.1.7)
- **`icon_sizes`** -- stated by every platform preset; the KDE reader replaces
  them from the icon theme's `index.theme`
