# Extra Icon Sets — Runtime Download Feature

## Overview

The native-theme crate is licensed `MIT OR Apache-2.0 OR 0BSD`. Bundling icon sets
with copyleft or attribution-required licenses would impose those obligations on every
downstream user. To avoid this, extra icon sets are **not bundled** — they are
downloaded at runtime by the application, on first use, into an app-local cache
directory.

This keeps native-theme fully permissive. The licensing obligations of a downloaded
icon set apply to the **application developer** who opts in, not to native-theme
itself.

## How It Works

1. The application enables a cargo feature (e.g., `extra-icons`).
2. On first use, it calls a function like `download_icon_set("breeze")`.
3. The crate fetches a curated subset of icons (only those matching defined
   `IconRole` variants) from a known URL (release tarball or git archive).
4. Icons are cached in a platform-appropriate directory:
   - Linux: `$XDG_DATA_HOME/<app>/icons/` (or `~/.local/share/<app>/icons/`)
   - macOS: `~/Library/Application Support/<app>/icons/`
   - Windows: `%LOCALAPPDATA%\<app>\icons\`
5. Subsequent calls load from cache without network access.

## Available Icon Sets

### Freedesktop.org-Compliant (Linux Desktop Themes)

These follow the [freedesktop.org Icon Naming Specification][fd-spec] and integrate
with Linux desktop environments. All 42 `IconRole` variants can be mapped to standard
freedesktop names.

| Icon Set | License (SPDX) | Format | Icons (approx.) | Source |
|---|---|---|---|---|
| [Breeze][breeze] (KDE) | `LGPL-3.0-or-later` | SVG | ~7,100 unique | [KDE/breeze-icons][breeze] |
| [Adwaita][adwaita] (GNOME) | `LGPL-3.0-only` OR `CC-BY-SA-3.0` (dual, user's choice) | SVG | ~790 | [GNOME/adwaita-icon-theme][adwaita] |
| [Oxygen][oxygen] (KDE) | `LGPL-3.0-or-later` | SVG + PNG | ~5,300 SVG | [KDE/oxygen-icons][oxygen] |
| [Papirus][papirus] | `GPL-3.0-only` | SVG | ~5,000+ unique | [PapirusDevelopmentTeam/papirus-icon-theme][papirus] |
| [Tango][tango] | Public Domain | SVG + PNG | ~510 SVG | [freedesktop/tango-icon-theme][tango] |
| [elementary][elementary] (Pantheon) | `GPL-3.0-only` | SVG | ~2,700 | [elementary/icons][elementary] |
| [Numix][numix] | `GPL-3.0-only` | SVG | ~4,500+ unique | [numixproject/numix-icon-theme][numix] |

### General-Purpose Icon Libraries

These are designed for web and app UIs, not for desktop theme integration. They do
**not** follow the freedesktop naming spec. Mapping to `IconRole` requires the
existing name-translation functions (`lucide_name_for_gpui_icon`,
`material_name_for_gpui_icon`, or new equivalents).

| Icon Set | License (SPDX) | Format | Icons (approx.) | Source |
|---|---|---|---|---|
| [Lucide][lucide] | `ISC` | SVG | ~1,700 | [lucide-icons/lucide][lucide] |
| [Material Symbols][material] (Google) | `Apache-2.0` | SVG | ~4,200 | [google/material-design-icons][material] |
| [Phosphor][phosphor] | `MIT` | SVG | ~1,500 (x6 weights) | [phosphor-icons/core][phosphor] |
| [Tabler Icons][tabler] | `MIT` | SVG | ~5,000 | [tabler/tabler-icons][tabler] |
| [Bootstrap Icons][bootstrap] | `MIT` | SVG | ~2,000 | [twbs/icons][bootstrap] |
| [Heroicons][heroicons] | `MIT` | SVG | ~320 (x4 styles) | [tailwindlabs/heroicons][heroicons] |
| [Font Awesome Free][fa] | Icons: `CC-BY-4.0`, Fonts: `OFL-1.1`, Code: `MIT` | SVG | ~2,000 (free subset) | [FortAwesome/Font-Awesome][fa] |
| [Remix Icon][remix] | `Remix Icon License v1.0` (custom, not OSI-approved) | SVG | ~3,200 | [Remix-Design/RemixIcon][remix] |

## License Compatibility Notes

### Safe for runtime download (permissive or weak copyleft)

- **Tango** — Public Domain. No obligations at all. Smallest set but covers the
  basics. Was the reference implementation for the freedesktop icon naming spec.
- **Lucide, Phosphor, Tabler, Bootstrap, Heroicons** — MIT or ISC. Attribution in
  LICENSE file is the only requirement.
- **Material Symbols** — Apache-2.0. Attribution required, patent grant included.
- **Adwaita** — Dual-licensed. The app developer can choose CC-BY-SA-3.0
  (attribution + share-alike for the icons only) or LGPL-3.0.
- **Breeze, Oxygen** — LGPL-3.0 with an explicit artwork clarification stating that
  using icons in a GUI falls under LGPL Section 5 (analogous to dynamic linking).
  The application itself does not become LGPL.

### Requires caution

- **Papirus, elementary, Numix** — GPL-3.0 (strong copyleft). Using these icons
  in a proprietary application is legally uncertain. GPL-3.0 for artwork has no
  equivalent of the LGPL Section 5 linking exception. Best suited for GPL-licensed
  applications only.
- **Font Awesome Free** — The icons are CC-BY-4.0 (attribution required). The font
  files are OFL-1.1. Since we use SVGs, only CC-BY-4.0 applies.
- **Remix Icon** — Custom "Remix Icon License v1.0". Broadly permissive but
  prohibits: selling icons standalone, creating competing icon libraries, and using
  icons as logos/trademarks. Not OSI-approved. The icons must remain under this
  license even when integrated into projects using other licenses.

## App Developer Obligations

When an application opts into downloading an icon set, the developer is responsible
for:

1. **License file inclusion** — Ship the icon set's license text with the
   application (the download function should save it alongside the icons).
2. **Attribution** — Display appropriate credit as required by the license (e.g., in
   an About dialog or NOTICES file).
3. **User notification** — Inform users that icons will be downloaded on first run
   (good practice, may be legally required in some jurisdictions).
4. **Source availability** — For LGPL icon sets, the SVG source must be available.
   Since icons are stored as SVGs, this is satisfied automatically.

## Implementation Considerations

- **Subset extraction**: Only download icons matching the 42 `IconRole` variants,
  not the full theme. This reduces download size from megabytes to kilobytes.
- **Offline fallback**: If download fails, fall back to the platform's native icons
  or the already-mapped Lucide/Material icons in gpui-component.
- **Cache invalidation**: Store a version marker alongside cached icons. When the
  crate is updated with new `IconRole` variants, re-download to fill gaps.
- **Checksum verification**: Verify downloaded archives against known checksums to
  prevent tampering.
- **No network at build time**: The download happens at application runtime, never
  during `cargo build`. This avoids reproducibility and sandboxing issues.
- **crates.io size limit**: crates.io enforces a 10 MB per-crate limit, which is
  another reason not to bundle icon sets.

## Recommended Defaults

For applications that want a "just works" experience:

1. **Linux**: Use `system_icon_theme()` to detect the installed theme (Breeze,
   Adwaita, etc.) and load icons from the system's installed theme directly — no
   download needed. Only fall back to downloading if the system theme is missing or
   incomplete.
2. **macOS / Windows**: Platform-native icons (SF Symbols, Segoe Fluent) are the
   primary source. Extra icon sets serve as supplementary fallback for roles not
   covered by native icons.
3. **Cross-platform fallback**: Lucide (already mapped in gpui-component) or
   Material Symbols (Apache-2.0, comprehensive) are the safest choices for a
   downloaded fallback set.

[fd-spec]: https://specifications.freedesktop.org/icon-naming-spec/latest/
[breeze]: https://github.com/KDE/breeze-icons
[adwaita]: https://github.com/GNOME/adwaita-icon-theme
[oxygen]: https://github.com/KDE/oxygen-icons
[papirus]: https://github.com/PapirusDevelopmentTeam/papirus-icon-theme
[tango]: https://gitlab.freedesktop.org/tango/tango-icon-theme
[elementary]: https://github.com/elementary/icons
[numix]: https://github.com/numixproject/numix-icon-theme
[lucide]: https://github.com/lucide-icons/lucide
[material]: https://github.com/google/material-design-icons
[phosphor]: https://github.com/phosphor-icons/core
[tabler]: https://github.com/tabler/tabler-icons
[bootstrap]: https://github.com/twbs/icons
[heroicons]: https://github.com/tailwindlabs/heroicons
[fa]: https://github.com/FortAwesome/Font-Awesome
[remix]: https://github.com/Remix-Design/RemixIcon
