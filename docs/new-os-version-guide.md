# Updating Platform Constants for New OS Versions

native-theme ships platform-specific constants extracted from authoritative sources
(KDE Breeze metrics, Apple HIG measurements, WinUI3 Fluent specs, libadwaita/GTK4
defaults). When a new OS version ships (e.g., KDE Plasma 7, Windows 12, macOS 16,
GNOME 48), these constants may need updating.

This guide covers the update procedure for each platform.

---

## KDE (Breeze)

**Source:** `breezemetrics.h` from the KDE Breeze repository
(<https://invent.kde.org/plasma/breeze>)

**Files to update:**

- `native-theme/src/kde/metrics.rs` -- the `populate_widget_sizing()` function
  (button sizes, checkbox dimensions, scrollbar widths, etc.)
- `native-theme/src/kde/mod.rs` -- KDE reader logic (rarely changes between versions)
- `native-theme/src/presets/kde-breeze.toml` -- bundled preset data

**What to look for:**

- Changed default values in `breezemetrics.h` for button height, checkbox size,
  scrollbar width, slider groove thickness, menu item height, etc.
- New constants added to `breezemetrics.h` that map to existing per-widget sizing fields.
- Changes to color group names or key names in `kdeglobals` format.

**Process:**

1. Clone or pull the latest Breeze source from <https://invent.kde.org/plasma/breeze>.
2. Compare the new `kstyle/breezemetrics.h` with the values in `populate_widget_sizing()`.
3. Update any changed constants in `native-theme/src/kde/metrics.rs`.
4. Regenerate the preset TOML if widget_metrics values changed:
   `cargo test -p native-theme --features kde` to verify.
5. Update `native-theme/src/presets/kde-breeze.toml` if colors, geometry, or spacing
   defaults changed.
6. Run the full test suite: `cargo test -p native-theme --features kde`.

---

## Windows

**Source:** Windows SDK headers, `GetSystemMetricsForDpi` API documentation,
WinUI3 Fluent Design specifications

**Files to update:**

- `native-theme/src/windows.rs` -- WinUI3 spacing constants, system metric mappings,
  `winui3_spacing()`, DPI-aware geometry reader
- `native-theme/src/presets/windows-11.toml` -- bundled preset data

**What to look for:**

- New system metric indices added to `GetSystemMetricsForDpi`.
- Changed WinUI3 Fluent Design spacing values (padding, margins, control sizes).
- New `UIColorType` variants for additional system colors.
- Changes to default font (Segoe UI Variable) or font size.
- Changes to default corner radius values.

**Process:**

1. Review Windows SDK release notes for new `GetSystemMetrics` indices.
2. Review WinUI3 Fluent Design specs for updated spacing/sizing values.
3. Update constants in `native-theme/src/windows.rs`.
4. Update `native-theme/src/presets/windows-11.toml` (or create a new preset, e.g.,
   `windows-12.toml`, if the visual language changes significantly).
5. Test on the target Windows version: `cargo test -p native-theme --features windows`.

**Note:** Widget metrics in the Windows reader use `GetSystemMetricsForDpi` for
DPI-aware values (scrollbar width, button sizes, menu metrics). Verify these still
return correct values on the new Windows version.

---

## macOS

**Source:** Apple Human Interface Guidelines (HIG), AppKit release notes,
Xcode Interface Builder measurements

**Files to update:**

- `native-theme/src/macos.rs` -- the `macos_widget_metrics()` function and
  `from_macos()` reader
- `native-theme/src/presets/macos-sonoma.toml` -- bundled preset data (or create a
  new preset for the new macOS version)

**What to look for:**

- Changed default sizes in HIG (button heights, control padding, font sizes).
- New or renamed `NSColor` semantic color names.
- Changes to default system font family or size.
- Changes to default corner radius (currently ~5px for controls).
- New accessibility or Dynamic Type behaviors.

**Process:**

1. Review Apple HIG for the new macOS version at
   <https://developer.apple.com/design/human-interface-guidelines/>.
2. Review AppKit release notes for changed/deprecated `NSColor` names.
3. Measure updated control sizes in Xcode Interface Builder if HIG does not
   provide exact values.
4. Update constants in `macos_widget_metrics()` in `native-theme/src/macos.rs`.
5. If semantic color names changed, update the `from_macos()` function's
   NSColor mappings.
6. Update or create the preset TOML file.
7. Test on the target macOS version: `cargo test -p native-theme --features macos`.

**Note:** The macOS module is unconditionally compiled. The `build_theme` tests run
cross-platform using hardcoded fallback values, so basic testing works on any OS.

---

## GNOME (Adwaita / libadwaita)

**Source:** libadwaita source code (<https://gitlab.gnome.org/GNOME/libadwaita>),
GTK4 source, freedesktop portal specification

**Files to update:**

- `native-theme/src/gnome/mod.rs` -- the `adwaita_widget_metrics()` function,
  hardcoded Adwaita color/geometry/spacing defaults, `from_gnome()` reader
- `native-theme/src/presets/adwaita.toml` -- bundled preset data

**What to look for:**

- Changed default values in libadwaita CSS or SCSS source files (colors, padding,
  corner radius, font sizes).
- Changed default font family (GNOME 48+ uses "Adwaita Sans" / "Adwaita Mono";
  earlier versions used "Cantarell" / "Source Code Pro").
- New CSS custom properties (`--window-bg-color`, `--accent-bg-color`, etc.).
- New freedesktop portal settings (accent-color, color-scheme, contrast).

**Process:**

1. Clone or pull latest libadwaita from <https://gitlab.gnome.org/GNOME/libadwaita>.
2. Compare CSS variable defaults with the hardcoded values in `gnome/mod.rs`.
3. Compare widget sizing values with those in `adwaita_widget_metrics()`.
4. Update any changed constants.
5. Update `native-theme/src/presets/adwaita.toml`.
6. Run the test suite: `cargo test -p native-theme --features portal`.

---

## Updating Preset TOML Files

After updating reader constants, also update the corresponding preset TOML files in
`native-theme/src/presets/`. Presets should reflect the platform's default appearance.

Steps:

1. Update color values in the `[light.colors]` and `[dark.colors]` sections.
2. Update geometry values (`radius`, `radius_lg`, `shadow`, etc.).
3. Update `[light.widget_metrics]` / `[dark.widget_metrics]` sections if widget
   sizing changed.
4. Run the full test suite: `cargo test -p native-theme` (no feature flags needed
   for preset-only changes).

Community color presets (Catppuccin, Nord, Dracula, etc.) use generic widget_metrics
defaults and are not affected by platform-specific changes.

---

## Adding a New Platform

To add support for a new platform (e.g., a new Linux desktop environment):

1. **Feature flag:** Add a new feature in `native-theme/Cargo.toml` with any
   required dependencies.
2. **Reader module:** Create a new module (e.g., `native-theme/src/cosmic.rs`)
   with a `from_cosmic()` function that returns `Result<ThemeSpec>`.
3. **Widget metrics:** Add a `cosmic_widget_metrics()` function if the platform
   has well-defined widget sizing constants.
4. **Preset file:** Create `native-theme/src/presets/cosmic.toml` with default
   light and dark variants.
5. **Register preset:** Add the preset name to the `preset()` and `list_presets()`
   functions in `native-theme/src/presets.rs`.
6. **Update dispatch:** Add the platform to `from_system()` and
   `from_system_async()` in `native-theme/src/lib.rs`.
7. **Tests:** Add tests for the new reader and preset.
8. **CI:** Add the platform to the CI matrix in `.github/workflows/ci.yml` if
   a runner is available.
