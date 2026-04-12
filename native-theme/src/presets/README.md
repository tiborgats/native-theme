# Bundled Presets

TOML theme specifications loaded via `Theme::preset("name")`. Each file
defines a complete or near-complete theme with light and dark variants.

## Two-tier system

### Regular presets

Complete, standalone theme specifications. They include **all** fields --
colors, geometry, fonts, and platform-specific values like `button_order`.
Any app can load a regular preset and get a fully resolved theme without
needing a platform reader.

Files: `kde-breeze.toml`, `macos-sonoma.toml`, `windows-11.toml`,
`adwaita.toml`, `material.toml`, `ios.toml`, and the community presets
(`catppuccin-*.toml`, `dracula.toml`, `gruvbox.toml`, `nord.toml`,
`one-dark.toml`, `solarized.toml`, `tokyo-night.toml`).

### Live presets (`*-live.toml`)

Base layers paired with platform readers (`from_kde`, `from_gnome`,
`from_macos`, `from_windows`). Live presets **omit** fields that the
reader provides at runtime from the operating system. The reader merges
live OS values onto the preset base, then the resolve pipeline fills any
remaining gaps.

Files: `kde-breeze-live.toml`, `macos-sonoma-live.toml`,
`windows-11-live.toml`, `adwaita-live.toml`.

## Preset file listing

| File | Description |
|------|-------------|
| `adwaita.toml` | GNOME/Adwaita standalone theme |
| `adwaita-live.toml` | GNOME/Adwaita base for `from_gnome` reader |
| `catppuccin-frappe.toml` | Catppuccin Frappe color scheme |
| `catppuccin-latte.toml` | Catppuccin Latte color scheme |
| `catppuccin-macchiato.toml` | Catppuccin Macchiato color scheme |
| `catppuccin-mocha.toml` | Catppuccin Mocha color scheme |
| `dracula.toml` | Dracula color scheme |
| `gruvbox.toml` | Gruvbox color scheme |
| `ios.toml` | iOS/iPadOS standalone theme |
| `kde-breeze.toml` | KDE Breeze standalone theme |
| `kde-breeze-live.toml` | KDE Breeze base for `from_kde` reader |
| `macos-sonoma.toml` | macOS Sonoma standalone theme |
| `macos-sonoma-live.toml` | macOS Sonoma base for `from_macos` reader |
| `material.toml` | Material Design standalone theme |
| `nord.toml` | Nord color scheme |
| `one-dark.toml` | One Dark color scheme |
| `solarized.toml` | Solarized color scheme |
| `tokyo-night.toml` | Tokyo Night color scheme |
| `windows-11.toml` | Windows 11 standalone theme |
| `windows-11-live.toml` | Windows 11 base for `from_windows` reader |

## Reader-provided fields

Live presets should **not** contain these fields. They are filled at
runtime by platform readers or `resolve_platform_defaults`:

- **`button_order`** -- filled by `resolve_platform_defaults` based on
  detected desktop environment
- **`icon_theme`** -- filled by platform reader or
  `resolve_platform_defaults` from system icon settings
- **`font_dpi`** -- filled by platform reader from OS display settings
- **`reduce_motion`**, **`high_contrast`**, **`reduce_transparency`**,
  **`text_scaling_factor`** -- filled by platform reader from OS
  accessibility settings
- **`icon_sizes`** -- filled by platform reader from filesystem lookup
