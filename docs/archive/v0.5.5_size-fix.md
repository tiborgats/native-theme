# Size-Fix: Font pt/px confusion & missing widget metric passthrough

Discovered: 2026-04-08. Everything in the showcase looks ~25% too small compared
to native KDE System Settings, and widget geometry from `ResolvedThemeVariant` is
not applied to rendered widgets.

---

## Environment that exposed the bug

- KDE Plasma 6 on Wayland, Breeze Dark
- 4K display (3840x2160) at **Scale: 1** (no compositor scaling)
- `forceFontDPI` not set, `Xft.dpi = 96`
- No `QT_SCALE_FACTOR` or other overrides

---

## Issue 1 — Font sizes stored in POINTS, consumed as PIXELS

### Symptom

Fonts in the showcase are 25% smaller than in native KDE/GNOME apps.

### Root cause

Qt font strings (`"Noto Sans,10,..."`) and Pango font strings
(`"Cantarell 11"`) store the font size in **typographic points** (1 pt =
1/72 inch). At the standard screen reference of 96 DPI, the conversion is:

```
px = pt × 96/72 = pt × 4/3
```

So 10 pt = 13.33 px, and 11 pt = 14.67 px.

The KDE reader (`kde/fonts.rs:38`) and GNOME reader (`gnome/mod.rs:83`)
parse the point value and store it as-is in `FontSpec.size`. Every doc
comment from `FontSpec.size` (`model/font.rs:29`) through
`ResolvedFontSpec.size` (`model/font.rs:57`) and
`ResolvedTextScaleEntry.size` (`model/resolved.rs:34`) declares the unit
as "logical pixels", but the actual value is in **points**.

The gpui connector passes it directly to gpui:

```rust
// lib.rs:131 — treats font.size as pixels
theme.font_size = px(d.font.size);   // px(10.0) instead of px(13.33)
```

The iced connector does the same (`lib.rs:260`). Both connectors even have
comments saying "no pt-to-px conversion is applied" because the value is
"already in logical pixels" — this is incorrect.

### Affected code paths

| Location | What happens |
|---|---|
| `kde/fonts.rs:38` | Parses Qt field[1] (points) → `FontSpec.size` |
| `gnome/mod.rs:83` | Parses Pango trailing number (points) → `FontSpec.size` |
| `resolve/inheritance.rs:18-19` | Copies size as-is (no conversion) |
| `resolve/validate.rs` | Stores into `ResolvedFontSpec.size` (no conversion) |
| `connectors/native-theme-gpui/src/lib.rs:131` | `px(d.font.size)` — passes points as pixels |
| `connectors/native-theme-gpui/src/config.rs:50` | `font_size: Some(d.font.size)` — same |
| `connectors/native-theme-iced/src/lib.rs:260` | Returns `font.size` directly |

### Not affected (correct already)

| Platform | Why correct |
|---|---|
| **macOS** | `NSFont.pointSize()` returns Apple points. macOS uses 72 DPI as its coordinate base, so 1 Apple pt = 1 gpui logical pixel on macOS. No conversion needed. |
| **Windows** | The `windows-11.toml` preset stores 14.0 which matches WinUI3's 14 epx body text. However, the OS reader `logfont_to_fontspec_raw` (windows.rs:112-116) computes `|lfHeight| * 72 / dpi` which produces points — see Issue 1b. |

### Issue 1b — Windows OS reader also produces points

`logfont_to_fontspec_raw` (`windows.rs:112-116`) converts `lfHeight`
(device pixels at system DPI) to **points** via `|lfHeight| * 72 / dpi`.
The result (e.g. 12.0 pt for Segoe UI) is stored as `FontSpec.size` which
claims to be in logical pixels.

Correct conversion to 96-DPI logical pixels:

```
px = |lfHeight| * 96 / dpi
```

At 96 DPI: `|−16| * 96 / 96 = 16.0` logical px.
At 144 DPI (150%): `|−24| * 96 / 144 = 16.0` logical px.

Note: `lfHeight` is the character cell height (em square), not the visible
glyph height. The WinUI "14px body text" refers to a slightly different
metric. This warrants testing against real Windows rendering before
changing.

---

## Issue 2 — TOML preset font sizes are in points (not a bug, but needs conversion)

Platform presets store font sizes in their platform's native unit —
typographic points. This is correct and intentional: the values match
what the platform uses (KDE's "Noto Sans 10", GNOME's "Cantarell 11").

The conversion from pt to px is handled by the `font_dpi` field during
resolution. These presets do NOT need `font_dpi` in the TOML — the OS
reader provides it from system settings. See Fix 4 for how the pipeline
propagates the detected DPI to both variants.

### Platform presets (sizes in pt — converted by `font_dpi`)

| Preset | Font size (pt) | At 96 DPI → px | At 192 DPI → px |
|---|---|---|---|
| `kde-breeze.toml` | 10.0 | 13.33 | 26.67 |
| `adwaita.toml` | 11.0 | 14.67 | 29.33 |
| `macos-sonoma.toml` | 13.0 | 13.0 (72 DPI) | N/A |
| `ios.toml` | 17.0 | 17.0 (72 DPI) | N/A |

### Non-platform presets (sizes already in px — no `font_dpi`, no conversion)

| Preset | Size (px) | Status |
|---|---|---|
| `windows-11.toml` | 14.0 | WinUI epx, already px |
| `material.toml` | 14.0 | Design-system px |
| `catppuccin-*.toml` | 14.0 | Code-editor px |
| `dracula.toml` | 14.0 | Code-editor px |
| `one-dark.toml`, `nord.toml`, `gruvbox.toml`, `solarized.toml`, `tokyo-night.toml` | 14.0 | Code-editor px |

### text_scale and line_height

The text_scale entries in platform presets are also in points. The
`resolve_font_dpi_conversion` phase converts them together with font
sizes. Explicit `line_height` values in the preset are in points too
(they were computed as `size_pt × defaults.line_height`), so they get
the same conversion.

---

## Issue 3 — Widget metrics resolved but not applied

### Symptom

Buttons, comboboxes, inputs, and other widgets in the showcase use
gpui-component's built-in defaults instead of KDE Breeze's dimensions.

### Root cause

`to_theme()` (`native-theme-gpui/src/lib.rs:116-162`) and
`to_theme_config()` (`config.rs:28-68`) map only:

- `font_family`, `font_size`, `mono_font_family`, `mono_font_size`
- `radius`, `radius_lg`, `shadow`
- All 108 `ThemeColor` fields
- `scrollbar_show`, `highlight_theme`

None of the per-widget geometry is mapped:

| Resolved field | Value (KDE Breeze) | gpui-component equivalent | Status |
|---|---|---|---|
| `button.min_height` | 32.0 | No direct theme token | Not mapped |
| `button.min_width` | 80.0 | No direct theme token | Not mapped |
| `button.border.padding_horizontal` | 6.0 | No direct theme token | Not mapped |
| `button.border.padding_vertical` | 5.0 | No direct theme token | Not mapped |
| `input.min_height` | 32.0 | No direct theme token | Not mapped |
| `input.border.padding_horizontal` | 6.0 | No direct theme token | Not mapped |
| `combo_box.min_height` | 32.0 | No direct theme token | Not mapped |
| `combo_box.min_width` | 120.0 | No direct theme token | Not mapped |
| `menu.row_height` | 28.0 | No direct theme token | Not mapped |
| `tab.min_height` | 30.0 | No direct theme token | Not mapped |
| `scrollbar.thumb_width` | 8.0 | No direct theme token | Not mapped |
| `slider.thumb_diameter` | 20.0 | No direct theme token | Not mapped |
| `checkbox.indicator_width` | 20.0 | No direct theme token | Not mapped |
| `switch.track_width` | 36.0 | No direct theme token | Not mapped |
| `switch.track_height` | 18.0 | No direct theme token | Not mapped |
| `dialog.min_width` | 320.0 | No direct theme token | Not mapped |

This is fundamentally a **gpui-component upstream limitation** — the
`Theme` / `ThemeColor` / `ThemeConfig` structs don't expose per-widget
geometry tokens. The connector can only map what gpui-component accepts.

The iced connector has the same gap — Iced's `Theme`/`Palette` system
provides color-only theming; widget geometry is controlled per-widget via
style closures. See `docs/todo_v0.6.0_iced-full-theme-geometry.md`.

### Interim workaround

Users of `native-theme-gpui` and `native-theme-iced` can access the full
`ResolvedThemeVariant` directly and apply per-widget metrics via each
framework's style API. The `text_scaling_factor()` helper
(`native-theme-gpui/src/lib.rs:335`) exposes the accessibility scaling
preference.

---

## Issue 4 — `text_scaling_factor` computed but never applied

### Symptom

The `text_scaling_factor` field is correctly computed:

- KDE: `forceFontDPI / 96.0` (`kde/mod.rs:92`)
- GNOME: `text-scaling-factor` gsetting (`gnome/mod.rs:197`)
- Windows: `UISettings.TextScaleFactor` (`windows.rs:392`)
- macOS: `systemFontSize / 13.0` (`macos.rs:150`)

The resolved value is stored in `ResolvedThemeDefaults.text_scaling_factor`
and a helper function exposes it in the gpui connector
(`lib.rs:335-336`). But:

1. The gpui connector never multiplies `font.size` by it.
2. The iced connector never applies it either.
3. No documentation tells users they need to apply it themselves.

### Interaction with Issue 1

Once Issue 1 is fixed (pt→px conversion at the reader level), the
`text_scaling_factor` should NOT be baked into the font size at the core
level. It is an **accessibility preference** — the user wants text larger
than the OS default. The rendering framework's own DPI scaling already
handles display-level scaling.

The correct model:

- `FontSpec.size` = logical px (what the OS default is)
- `text_scaling_factor` = user preference multiplier (1.0 = no change)
- Final rendered size = `font.size × text_scaling_factor` (applied by the
  app, not the core crate)

---

## Issue 5 — Misleading doc comments

Several doc comments claim "logical pixels" where the actual unit is
platform-dependent:

| Location | Current doc | Actual value |
|---|---|---|
| `model/font.rs:29` (`FontSpec.size`) | "Font size in logical pixels" | points from KDE/GNOME readers |
| `model/font.rs:57` (`ResolvedFontSpec.size`) | "Font size in logical pixels" | points (after resolution, still unconverted) |
| `model/font.rs:74` (`TextScaleEntry.size`) | "Font size in logical pixels" | points from presets |
| `model/resolved.rs:34` (`ResolvedTextScaleEntry.size`) | "Font size in logical pixels" | points |
| `connectors/native-theme-gpui/src/config.rs:17` | "IMPORTANT: ResolvedFontSpec sizes are already in logical pixels" | Wrong for KDE/GNOME |
| `connectors/native-theme-gpui/src/config.rs:48` | "Font sizes are already in logical pixels (NOT points)" | Wrong for KDE/GNOME |
| `connectors/native-theme-iced/src/lib.rs:257` | "ResolvedFontSpec.size is already in logical pixels" | Wrong for KDE/GNOME |

---

## Proposed Solutions

### Fix 1 — Add `font_dpi` field to `ThemeDefaults` (core crate)

The DPI used for pt→px conversion must **not** be hardcoded. It varies by
platform and user configuration. Add it as a first-class field:

```rust
// model/defaults.rs — new field in ThemeDefaults
pub struct ThemeDefaults {
    // ...

    /// Font DPI: the dots-per-inch value used to convert typographic points
    /// to logical pixels.
    ///
    /// When `Some(dpi)`, font sizes (`FontSpec.size`, `TextScaleEntry.size`)
    /// in this variant are in **typographic points** and will be converted
    /// during resolution: `px = pt × font_dpi / 72`.
    ///
    /// When `None`, font sizes are already in **logical pixels** — no
    /// conversion is applied. This is the default for non-platform presets
    /// (catppuccin, dracula, material, etc.) where sizes are specified
    /// directly in px.
    ///
    /// OS readers auto-detect this from system settings. Users can override
    /// via TOML overlay or the Rust API to adjust font rendering.
    pub font_dpi: Option<f32>,

    // ...
}
```

Also add to `ResolvedThemeDefaults`:

```rust
pub struct ResolvedThemeDefaults {
    // ...
    /// Font DPI used for pt→px conversion. Defaults to 96.0 when not set.
    pub font_dpi: f32,
    // ...
}
```

### Fix 2 — OS readers detect `font_dpi` from system settings

DPI must be read from the system — never hardcoded in TOML presets.
Each OS reader detects `font_dpi` using a platform-appropriate fallback
chain.

**KDE** (`kde/mod.rs`) — detection chain:

1. `forceFontDPI` from kdeglobals `[General]` or kcmfontsrc
2. `Xft.dpi` from X resources (`xrdb -query` or reading `~/.Xresources`)
3. Physical DPI from monitor dimensions (if available from Wayland
   `wl_output` or XRandR: `resolution_px / (physical_mm / 25.4)`)
4. Fallback: `96.0`

```rust
fn detect_font_dpi(ini: &configparser::ini::Ini) -> f32 {
    // 1. forceFontDPI (explicit user preference)
    if let Some(dpi) = ini.get("General", "forceFontDPI")
        .or_else(|| read_kcmfontsrc_key("General", "forceFontDPI"))
        .and_then(|s| s.trim().parse::<f32>().ok())
        .filter(|&d| d > 0.0)
    {
        return dpi;
    }

    // 2. Xft.dpi from X resources
    if let Some(dpi) = read_xft_dpi() {
        return dpi;
    }

    // 3. Physical DPI from monitor (optional, platform-dependent)
    if let Some(dpi) = detect_physical_dpi() {
        return dpi;
    }

    // 4. Fallback
    96.0
}
```

Important: `forceFontDPI` is the font DPI value itself, NOT a scaling
factor. The current code that stores `forceFontDPI / 96` as
`text_scaling_factor` is **wrong** — it conflates font DPI with
accessibility text scaling. After this fix, `text_scaling_factor` on KDE
should only reflect a genuine accessibility preference (if KDE exposes one
separately), or be left as `None`.

**GNOME** (`gnome/mod.rs`) — detection chain:

1. `Xft.dpi` from X resources
2. Physical DPI from monitor dimensions
3. Fallback: `96.0`

Note: GNOME's `text-scaling-factor` gsetting is a separate accessibility
multiplier and must remain as `text_scaling_factor`, not `font_dpi`.

**macOS** (`macos.rs`): Apple uses 72 DPI as its coordinate base.
`NSFont.pointSize()` returns Apple points where 1pt = 1 logical pixel.
Set `font_dpi = Some(72.0)` so the formula `pt × 72/72 = pt` produces
the identity — no effective conversion, which is correct.

**Windows** (`windows.rs`): The reader already calls `GetDpiForSystem()`.
`logfont_to_fontspec_raw` converts lfHeight to points via
`|lfHeight| × 72 / system_dpi`. Set `font_dpi = Some(96.0)` so
resolution converts `pt × 96/72 = |lfHeight| × 72/system_dpi × 96/72
= |lfHeight| × 96/system_dpi` — the correct 96-DPI logical pixels.

### Fix 2b — `read_xft_dpi()` helper

Read `Xft.dpi` from X resources. On Wayland with XWayland, this is still
available. Example from the current system: `Xft.dpi: 96`.

```rust
/// Read Xft.dpi from X resources (available on X11 and XWayland).
fn read_xft_dpi() -> Option<f32> {
    let output = std::process::Command::new("xrdb")
        .arg("-query")
        .output()
        .ok()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        if let Some(value) = line.strip_prefix("Xft.dpi:") {
            return value.trim().parse::<f32>().ok().filter(|&d| d > 0.0);
        }
    }
    None
}
```

### Fix 2c — `detect_physical_dpi()` helper (optional)

Calculate DPI from monitor physical dimensions and resolution. This is a
last-resort fallback when no explicit DPI setting exists. Available from:

- **Wayland**: `wl_output` reports `physical_width` / `physical_height`
  in mm and resolution in pixels
- **X11/XRandR**: `xrandr` reports `connected ... WxH ... WIDTHmm x HEIGHTmm`
- **xdpyinfo**: reports `dimensions` and `resolution`

Example from the current system: 3840x2160 on a 700mm wide display →
`3840 / (700/25.4) = 139.4 DPI`.

Note: Physical DPI may be very high on HiDPI displays without compositor
scaling. Using raw physical DPI for font rendering can produce fonts much
larger than the user intended. Consider capping or rounding to standard
tiers (96, 120, 144, 168, 192) if the value seems like an unscaled HiDPI
display.

### Fix 3 — Resolution step: convert font sizes using `font_dpi`

Add a new resolution phase (after defaults-internal, before
widget-from-defaults) that converts all font sizes when `font_dpi` is set:

```rust
// resolve/inheritance.rs — new function
fn convert_pt_to_px(size: &mut Option<f32>, dpi: f32) {
    if let Some(s) = size {
        *s *= dpi / 72.0;
    }
}

// New resolution phase in ThemeVariant
pub(crate) fn resolve_font_dpi_conversion(&mut self) {
    let dpi = match self.defaults.font_dpi {
        Some(d) if d > 0.0 => d,
        Some(_) | None => return, // None = sizes already in px
    };

    // defaults.font, defaults.mono_font
    convert_pt_to_px(&mut self.defaults.font.size, dpi);
    convert_pt_to_px(&mut self.defaults.mono_font.size, dpi);

    // All per-widget fonts that have explicit sizes
    // (fonts with size=None will inherit converted defaults later)
    for widget_font in [
        &mut self.window.title_bar_font,
        &mut self.button.font,
        &mut self.input.font,
        &mut self.checkbox.font,
        &mut self.menu.font,
        &mut self.tooltip.font,
        &mut self.tab.font,
        &mut self.sidebar.font,
        &mut self.toolbar.font,
        &mut self.status_bar.font,
        &mut self.list.item_font,
        &mut self.list.header_font,
        &mut self.popover.font,
        &mut self.dialog.title_font,
        &mut self.dialog.body_font,
        &mut self.combo_box.font,
        &mut self.segmented_control.font,
        &mut self.expander.font,
        &mut self.link.font,
    ] {
        if let Some(font) = widget_font {
            convert_pt_to_px(&mut font.size, dpi);
        }
    }

    // text_scale entries
    for entry in [
        &mut self.text_scale.caption,
        &mut self.text_scale.section_heading,
        &mut self.text_scale.dialog_title,
        &mut self.text_scale.display,
    ] {
        if let Some(e) = entry {
            convert_pt_to_px(&mut e.size, dpi);
            // line_height values in the preset are also in points
            // if explicitly set; convert them too
            convert_pt_to_px(&mut e.line_height, dpi);
        }
    }

    // Clear font_dpi after conversion so it's not applied again
    // if resolve() is called multiple times
    self.defaults.font_dpi = None;
}
```

Call this in `resolve/mod.rs` as a new phase between Phase 1 (defaults
internal chains) and Phase 2 (safety nets):

```rust
pub fn resolve(&mut self) {
    self.resolve_defaults_internal();       // Phase 1
    self.resolve_font_dpi_conversion();     // Phase 1.5 (NEW)
    self.resolve_safety_nets();             // Phase 2
    self.resolve_widgets_from_defaults();   // Phase 3
    self.resolve_widget_to_widget();        // Phase 4
    self.resolve_icon_fallback();           // Phase 5
}
```

### Fix 4 — Pipeline: propagate `font_dpi` to inactive variant

TOML presets do NOT contain `font_dpi` — the DPI comes from the system
via the OS reader. But the current pipeline (`run_pipeline` in `lib.rs`)
only merges reader output into the **active** variant. The inactive
variant uses the full preset directly (`lib.rs:800,806`), with no reader
data — so it never receives the detected `font_dpi`.

Fix: after constructing both variants but before resolution, propagate
the reader's `font_dpi` to the inactive variant:

```rust
// run_pipeline() — after constructing light_variant and dark_variant,
// before calling into_resolved():

// Propagate font_dpi from the reader to both variants so the
// pt→px conversion uses the system-detected DPI for both.
if let Some(reader_dpi) = reader_output
    .light.as_ref().and_then(|v| v.defaults.font_dpi)
    .or_else(|| reader_output.dark.as_ref().and_then(|v| v.defaults.font_dpi))
{
    if light_variant.defaults.font_dpi.is_none() {
        light_variant.defaults.font_dpi = Some(reader_dpi);
    }
    if dark_variant.defaults.font_dpi.is_none() {
        dark_variant.defaults.font_dpi = Some(reader_dpi);
    }
}
```

This way:
- Active variant: gets `font_dpi` from the reader (via merge) ✓
- Inactive variant: gets `font_dpi` propagated from the reader ✓
- Standalone preset (no reader): no `font_dpi` set → no conversion
  (sizes assumed px). Users can set `font_dpi` via API/overlay.

### Fix 4b — TOML presets: keep font sizes in their natural platform unit

Platform presets keep sizes in typographic points — the unit their
platform natively uses. No `font_dpi` in the TOML. The conversion is
driven by the OS reader's detected DPI during the live pipeline.

**No TOML changes needed** for `kde-breeze.toml`, `kde-breeze-live.toml`,
`adwaita.toml`, `macos-sonoma.toml`, `windows-11.toml`, `ios.toml`.

Non-platform presets (`catppuccin-*.toml`, `dracula.toml`, `material.toml`,
etc.) already have sizes in px and no `font_dpi` → no conversion → correct
as-is.

### Fix 5 — Fix KDE `text_scaling_factor`

The current code (`kde/mod.rs:92`) sets:

```rust
variant.defaults.text_scaling_factor = Some(dpi / 96.0);
```

This is **wrong**. `forceFontDPI` is the font rendering DPI, not an
accessibility text scale preference. After adding `font_dpi`:

- `font_dpi` carries the DPI value (for pt→px conversion)
- `text_scaling_factor` should only carry a genuine accessibility
  multiplier

KDE does not have a separate text-scaling accessibility setting (unlike
GNOME's `text-scaling-factor` or Windows' `TextScaleFactor`). Remove the
incorrect derivation:

```rust
// REMOVE:
// variant.defaults.text_scaling_factor = Some(dpi / 96.0);

// font_dpi is now set in the new populate_font_dpi() function.
// text_scaling_factor is left as None for KDE (no separate
// accessibility text scale preference).
```

### Fix 6 — Widget metrics: document as intentional, track upstream

The widget metric gap requires gpui-component upstream changes (new theme
tokens for per-widget geometry). This is already tracked in
`docs/todo.md` line 9:

> `[ ] Map WidgetMetrics → gpui-component per-widget styling`

and the iced equivalent in `docs/todo_v0.6.0_iced-full-theme-geometry.md`.

No code change in native-theme — the resolved metrics ARE available in
`ResolvedThemeVariant` for direct access. The gap is in the downstream
toolkit connector surface area.

### Fix 7 — Update doc comments

After the fix, update docs to reflect the new semantics:

- `FontSpec.size`: "Font size. When `font_dpi` is set on `ThemeDefaults`,
  this is in typographic points and will be converted to logical pixels
  during resolution (`px = pt × font_dpi / 72`). When `font_dpi` is
  `None`, this is already in logical pixels."
- `ResolvedFontSpec.size`: "Font size in logical pixels. Converted from
  points during resolution if `font_dpi` was set."
- Connector comments: remove the "no pt-to-px conversion" phrasing;
  replace with "ResolvedFontSpec.size is in logical pixels (conversion
  from platform points is handled by the resolution step)."

### Fix 8 — Document `text_scaling_factor` usage

Add doc comment to `ResolvedThemeDefaults.text_scaling_factor` explaining
that connectors/apps should multiply `font.size` by this factor when the
user preference for larger text should be honored. This is an
accessibility multiplier, independent of `font_dpi`.

---

## Resolution phase order (after fix)

```
Phase 1:    resolve_defaults_internal()       — accent→selection, font.color chains
Phase 1.5:  resolve_font_dpi_conversion()     — pt→px using font_dpi  (NEW)
Phase 2:    resolve_safety_nets()             — platform fallbacks
Phase 3:    resolve_widgets_from_defaults()   — color/border/font inheritance
Phase 4:    resolve_widget_to_widget()        — inactive←active chains
Phase 5:    resolve_icon_fallback()           — icon set fallback
```

Font sizes must be converted to px BEFORE Phase 3 (font inheritance),
because Phase 3 copies `defaults.font.size` to per-widget fonts. If we
convert after inheritance, we'd have to visit every widget font again.

---

## User-override examples

### Change DPI via TOML overlay

```toml
# "I have a 4K display at scale=1 and want bigger text"
[defaults]
font_dpi = 144
```

Result: 10pt KDE font → `10 × 144/72 = 20px` instead of `13.33px`.

### Change DPI via Rust API

```rust
let system = native_theme::SystemTheme::from_system()?;
// Override before using the resolved theme
let mut overlay = native_theme::ThemeSpec::default();
overlay.dark_mut().defaults.font_dpi = Some(144.0);
let adjusted = system.overlay(&overlay)?;
```

### Disable conversion (sizes already in px)

```toml
# Custom theme with sizes in px — no conversion
[defaults]
font_dpi = 72   # 72/72 = 1.0, identity
```

Or simply omit `font_dpi` (None = no conversion).

---

## Test plan

1. **Unit test**: KDE reader with `forceFontDPI = 192` → `font_dpi = Some(192.0)`.
2. **Unit test**: KDE reader without `forceFontDPI` → falls back to
   `Xft.dpi` or 96.
3. **Unit test**: GNOME reader detects `font_dpi` from `Xft.dpi` or
   falls back to 96.
4. **Unit test**: `resolve_font_dpi_conversion` with `font_dpi=96`:
   `FontSpec.size = 10.0` → `13.33...` after resolution.
5. **Unit test**: `resolve_font_dpi_conversion` with `font_dpi=None`:
   `FontSpec.size = 14.0` → `14.0` unchanged.
6. **Unit test**: `resolve_font_dpi_conversion` with `font_dpi=72`:
   `FontSpec.size = 13.0` → `13.0` (macOS identity).
7. **Unit test**: text_scale entries and line_heights are also converted.
8. **Unit test**: `text_scaling_factor` on KDE is NOT derived from
   `forceFontDPI` (should be `None` unless a separate accessibility
   setting exists).
9. **Unit test**: `run_pipeline` propagates `font_dpi` to inactive
   variant.
10. **Preset validation**: All resolved preset font sizes > 0.
11. **Proptest**: Roundtrip still passes.
12. **Visual**: Showcase on KDE Breeze Dark — font size matches KDE
    System Settings.
13. **Visual**: Showcase on GNOME Adwaita — font size matches GNOME
    Settings.
14. **macOS**: No change in rendered font size (font_dpi=72, identity).
15. **DPI override**: User sets `font_dpi = 144` via overlay → fonts
    render ~50% larger than at 96 DPI.

---

## Summary

| # | Issue | Scope | Fix |
|---|---|---|---|
| 1 | Font pt→px conversion missing | Core crate | Add `font_dpi` field, convert during resolution (Fix 1, 3) |
| 2 | DPI not read from system | Core crate | OS readers auto-detect via fallback chain (Fix 2, 2b, 2c) |
| 3 | Pipeline doesn't propagate DPI to inactive variant | Core crate | Propagate reader's `font_dpi` in `run_pipeline` (Fix 4) |
| 4 | KDE `text_scaling_factor` conflates DPI with accessibility | Core crate | Remove `forceFontDPI / 96` derivation (Fix 5) |
| 5 | Widget metrics not mapped to connectors | Upstream | gpui-component / iced limitation (Fix 6, tracked) |
| 6 | `text_scaling_factor` unused by connectors | Documentation | Doc comments + connector docs (Fix 8) |
| 7 | Misleading doc comments | Core + connectors | Update after fix (Fix 7) |
