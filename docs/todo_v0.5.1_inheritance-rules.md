# Inheritance Rules

Based on [todo_v0.5.1_theme-variant.md](todo_v0.5.1_theme-variant.md) struct definitions and [platform-facts.md](platform-facts.md) `←` arrows.

---

### Are inheritance rules the same on all platforms?

**The `resolve()` function is universal** — it uses the same derivation
rules on every platform. Platform differences are handled by the OS
readers, which populate different fields on each platform. The
universal `resolve()` rule only fires for fields the OS reader left
as `None`.

However, **7 fields** have `←` targets that genuinely differ depending
on the OS. In the OS-first model, these fields are typically provided
directly by the OS reader or the platform TOML — so the `←` targets
below serve as last-resort safety-net fallbacks in `resolve()`. The
universal `resolve()` picks one reasonable fallback for each; it may
not match every platform's native behavior, but that only matters if
both the OS reader and the platform TOML fail to provide the value.

### Platform-divergent inheritance

These 7 fields have different `←` targets depending on the OS
(verified against platform-facts §2 per-platform `←` arrows):

| Field | macOS | Windows | KDE | GNOME |
|-------|-------|---------|-----|-------|
| `input.caret` (§2.4) | preset¹ | `defaults.foreground` | `defaults.focus_ring_color` | `defaults.accent` |
| `scrollbar.track` (§2.8) | transparent | transparent | `defaults.background` | preset¹ |
| `spinner.fill` (§2.23) | preset¹ | `defaults.accent` | `defaults.foreground` | `defaults.foreground` |
| `popover.background` (§2.16) | `defaults.background` | `defaults.surface` | `defaults.background` | preset¹ |
| `list.background` (§2.15) | `defaults.background` | `defaults.background` | `defaults.surface` | preset¹ |
| `list.header_background` (§2.15) | `defaults.surface` | `defaults.background` | preset¹ | preset¹ |
| `input.background` (§2.4) | preset¹ | preset¹ | `defaults.surface` | preset¹ |

¹ "preset" = this platform uses a widget-specific API (⚙) for this value,
no `←` arrow in platform-facts. The OS reader provides it directly. The
platform TOML may also provide it as a non-⚙ design constant (e.g.,
GNOME's Adwaita CSS values). The `resolve()` fallback only fires if both
the OS reader and TOML leave it `None`.

**Why these differ:**
- `input.caret`: Windows derives from text color, KDE from focus
  decoration, GNOME from accent — three different design philosophies
  for caret visibility.
- `scrollbar.track`: macOS/Windows use transparent tracks (thumbs float);
  KDE draws a visible groove matching the background.
- `spinner.fill`: Windows spins in accent color; KDE/GNOME use foreground
  because their spinners are icon-based (process-working-symbolic).
- `popover.background`: Windows Fluent Flyout uses elevated surface;
  macOS/KDE use window background.
- `list.background`: KDE uses `[Colors:View]` group (= `defaults.surface`)
  for editable content areas including lists; macOS/Windows inherit
  window background.
- `list.header_background`: macOS headers are slightly elevated (≈ surface);
  Windows headers match window background.
- `input.background`: KDE's `[Colors:View] BackgroundNormal` is exactly
  `defaults.surface`; other platforms have distinct APIs.

### Fields that cannot be universally derived

These fields have no platform where the `←` arrow matches a useful
universal fallback. `resolve()` assigns a safety-net default, but it
will look wrong. The actual value must come from either:
- The **OS reader** (⚙ on macOS, Windows, KDE — provides the live value)
- The **platform TOML** (non-⚙ on GNOME — Adwaita CSS design constant)
- An explicit value in **cross-platform presets** (no OS reader)

| Field | resolve() safety net | Why the safety net is wrong |
|-------|---------------------|---------------------------|
| `button.background` | `defaults.background` | Button bg ≠ window bg on all 4 platforms (§2.3) |
| `button.foreground` | `defaults.foreground` | All platforms have distinct control text colors (§2.3) |
| `tooltip.background` | `defaults.background` | Tooltip bg is always visually distinct (§2.7) |
| `tooltip.foreground` | `defaults.foreground` | macOS tooltip fg is white, not label color (§2.7) |
| `sidebar.background` | `defaults.background` | All platforms have distinct sidebar bg (§2.12) |
| `list.alternate_row` | `defaults.background` | Alternate rows are always a tinted variant (§2.15) |
| `window.title_bar_background` | `defaults.surface` | Only macOS ≈ surface; Win/KDE/GNOME use distinct colors (§2.2) |

These are all ⚙ on macOS/Windows/KDE (OS reader provides them), but
non-⚙ on GNOME (Adwaita CSS values → must be in `adwaita.toml`).

### Uniform inheritance table

These rules apply identically on all 4 platforms. Where all platforms
agree on `← defaults.X`, the same fallback is correct everywhere.
Where no platform inherits (all widget-specific), the fallback is a
safety net — the OS reader or platform TOML always provides the
actual value.

| Widget field | Inherits from | All agree? |
|---|---|---|
| **window** (§2.2) | | |
| `window.background` | `defaults.background` | ✅ all `←` |
| `window.foreground` | `defaults.foreground` | ✅ all `←` |
| `window.border` | `defaults.border` | macOS/Win `←`, KDE/GNOME preset |
| `window.title_bar_background` | `defaults.surface` | no universal `←` (see above) |
| `window.title_bar_foreground` | `defaults.foreground` | all ⚙ / TOML |
| `window.inactive_title_bar_background` | `window.title_bar_background` | — |
| `window.inactive_title_bar_foreground` | `window.title_bar_foreground` | — |
| `window.title_bar_font` | `defaults.font` | all ⚙ / TOML |
| `window.radius` | `defaults.radius_lg` | Win/KDE/GNOME `←`, macOS preset |
| `window.shadow` | `defaults.shadow_enabled` | ✅ all `←` |
| **button** (§2.3) | | |
| `button.background` | `defaults.background` | no universal `←` (see above) |
| `button.foreground` | `defaults.foreground` | no universal `←` (see above) |
| `button.border` | `defaults.border` | ✅ all `←` |
| `button.font` | `defaults.font` | ✅ all `←` |
| `button.radius` | `defaults.radius` | ✅ all `←` |
| `button.disabled_opacity` | `defaults.disabled_opacity` | ✅ all `←` |
| `button.shadow` | `defaults.shadow_enabled` | ✅ all `←` |
| `button.primary_bg` | `defaults.accent` | ✅ all `←` |
| `button.primary_fg` | `defaults.accent_foreground` | ✅ all `←` |
| **input** (§2.4) | | |
| `input.background` | **per-platform** (see above) | |
| `input.foreground` | `defaults.foreground` | all ⚙ / TOML |
| `input.border` | `defaults.border` | ✅ all `←` |
| `input.placeholder` | `defaults.muted` | approximately ✅ |
| `input.caret` | **per-platform** (see above) | |
| `input.selection` | `defaults.selection` | ✅ all `←` |
| `input.selection_foreground` | `defaults.selection_foreground` | ✅ all `←` |
| `input.font` | `defaults.font` | ✅ all `←` |
| `input.radius` | `defaults.radius` | ✅ all `←` |
| `input.border_width` | `defaults.frame_width` | ✅ all `←` |
| **checkbox** (§2.5) | | |
| `checkbox.radius` | `defaults.radius` | ✅ all `←` |
| `checkbox.border_width` | `defaults.frame_width` | ✅ all `←` |
| `checkbox.checked_bg` | `defaults.accent` | ✅ all `←` |
| **menu** (§2.6) | | |
| `menu.background` | `defaults.background` | macOS/KDE ≈`←`, Win/GNOME preset |
| `menu.foreground` | `defaults.foreground` | macOS/KDE ≈`←`, Win/GNOME preset |
| `menu.separator` | `defaults.border` | macOS/Win/KDE `←`, GNOME preset |
| `menu.font` | `defaults.font` | GNOME `←`, macOS/Win/KDE widget-specific |
| **tooltip** (§2.7) | | |
| `tooltip.background` | `defaults.background` | no universal `←` (see above) |
| `tooltip.foreground` | `defaults.foreground` | no universal `←` (see above) |
| `tooltip.font` | `defaults.font` | Win/KDE/GNOME `←`, macOS widget-specific |
| `tooltip.radius` | `defaults.radius` | ✅ all `←` |
| **scrollbar** (§2.8) | | |
| `scrollbar.track` | **per-platform** (see above) | |
| `scrollbar.thumb` | `defaults.muted` | all ⚙ / TOML |
| `scrollbar.thumb_hover` | `defaults.muted` (darker) | all ⚙ / TOML |
| **slider** (§2.9) | | |
| `slider.fill` | `defaults.accent` | ✅ all `←` |
| `slider.track` | `defaults.muted` | ✅ all `←` |
| `slider.thumb` | `defaults.surface` | ✅ all `←` |
| **progress_bar** (§2.10) | | |
| `progress_bar.fill` | `defaults.accent` | ✅ all `←` |
| `progress_bar.track` | `defaults.muted` | ✅ all `←` |
| `progress_bar.radius` | `defaults.radius` | ✅ all `←` |
| **tab** (§2.11) | | |
| `tab.background` | `defaults.background` | ✅ all `←` |
| `tab.foreground` | `defaults.foreground` | ✅ all `←` |
| `tab.active_background` | `defaults.background` | ✅ all `←` |
| `tab.active_foreground` | `defaults.foreground` | ✅ all `←` |
| `tab.bar_background` | `defaults.background` | ✅ all `←` |
| **sidebar** (§2.12) | | |
| `sidebar.background` | `defaults.background` | no universal `←` (see above) |
| `sidebar.foreground` | `defaults.foreground` | macOS/Win `←`, KDE/GNOME preset |
| **toolbar** (§2.13) | | |
| `toolbar.font` | `defaults.font` | macOS/Win/GNOME `←`, KDE widget-specific |
| **status_bar** (§2.14) | | |
| `status_bar.font` | `defaults.font` | macOS/KDE/GNOME `←`, Win widget-specific |
| **list** (§2.15) | | |
| `list.background` | **per-platform** (see above) | |
| `list.foreground` | `defaults.foreground` | macOS/Win `←`, KDE/GNOME preset |
| `list.alternate_row` | `defaults.background` | no universal `←` (see above) |
| `list.selection` | `defaults.selection` | ✅ all `←` |
| `list.selection_foreground` | `defaults.selection_foreground` | ✅ all `←` |
| `list.header_background` | **per-platform** (see above) | |
| `list.header_foreground` | `defaults.foreground` | Win `←`, others preset |
| `list.grid_color` | `defaults.border` | macOS preset, others ≈`←` |
| **popover** (§2.16) | | |
| `popover.background` | **per-platform** (see above) | |
| `popover.foreground` | `defaults.foreground` | macOS/Win/KDE `←`, GNOME preset |
| `popover.border` | `defaults.border` | ✅ all `←` |
| `popover.radius` | `defaults.radius_lg` | ✅ all `←` |
| **splitter** (§2.17) | | |
| (sizing only — no inheritable fields) | | |
| **separator** (§2.18) | | |
| `separator.color` | `defaults.border` | macOS/Win/KDE `←`, GNOME preset |
| **switch** (§2.21) | | |
| `switch.checked_bg` | `defaults.accent` | ✅ all `←` |
| `switch.thumb_bg` | `defaults.surface` | all ⚙ / TOML |
| **dialog** (§2.22) | | |
| `dialog.title_font` | `defaults.font` | KDE `←`, others preset |
| `dialog.radius` | `defaults.radius_lg` | macOS/KDE `←`, Win/GNOME preset |
| **spinner** (§2.23) | | |
| `spinner.fill` | **per-platform** (see above) | |
| **combo_box** (§2.24) | | |
| `combo_box.radius` | `defaults.radius` | ✅ all `←` |
| **segmented_control** (§2.25) | | |
| `segmented_control.radius` | `defaults.radius` | ✅ all `←` |
| **card** (§2.26) | | |
| `card.background` | `defaults.surface` | Win/GNOME preset |
| `card.border` | `defaults.border` | Win/GNOME preset |
| `card.radius` | `defaults.radius_lg` | Win/GNOME preset |
| **expander** (§2.27) | | |
| `expander.radius` | `defaults.radius` | mixed |
| **link** (§2.28) | | |
| `link.color` | `defaults.link` | ✅ all `←` |
| **defaults** (global) | | |
| `defaults.selection_inactive` | `defaults.selection` | — |

### Sizing: no inheritance

Sizing properties (`min_width`, `min_height`, `max_width`, `max_height`,
`padding_*`, `height`, `width`, `diameter`, etc.) have **no inheritance** —
they are non-⚙ design constants and must be set in the platform TOML.
Missing sizing values cause validation errors.

### Fields with no default inheritance

The `switch.unchecked_bg` has **no default inheritance** — it must
come from the OS reader (⚙) or the platform TOML (non-⚙). It varies
too much across platforms to derive from a global default.

---

## Architecture: OS-First Theme Resolution

### Principle

**The OS is the primary source of truth.** When a user changes their
system font, accent color, or theme in OS settings, the application
must reflect that change.

The TOML is an **overlay on top of the native theme**, not the base.
This serves two use cases:

1. **Platform default TOMLs** (shipped with this crate): contain ONLY
   values that neither OS APIs nor inheritance can provide — design
   constants and non-⚙ colors. Size varies by platform: KDE/macOS
   TOMLs are very small (OS reads nearly everything), while
   GNOME/Windows TOMLs are larger (many Adwaita CSS / Fluent design
   token values are non-⚙).

2. **App developer TOMLs** (shipped with the application): can override
   ANY native value where the app's design requires it. An app that
   needs radius=8 buttons on all platforms sets `button.radius = 8.0`
   in its TOML, overriding both the OS and the inheritance.

The ⚙ and ↕ annotations in [platform-facts.md](platform-facts.md)
mark exactly which values are user-configurable (⚙) and which scale
with DPI (↕). Every ⚙ value should be read from the OS at runtime.
Non-⚙ values (HIG constants, Fluent design tokens, Breeze source
constants, Adwaita CSS geometry) are fixed design facts — these go
in the platform default TOML.

### Runtime flow

```
1. OS Reader (platform-specific Rust code)
   → Reads all ⚙ values the platform exposes
   → Returns partial ThemeVariant (None for fields
     OS does not provide)

2. resolve() — universal, shared Rust function
   → Fills remaining None fields from inheritance sources
   → accent → primary_bg, selection, focus_ring, checked_bg, fill
   → defaults.font → menu.font, tooltip.font, etc.
   → defaults.radius → button.radius, input.radius, etc.
   → Result: ThemeVariant with all OS-derivable fields populated,
     design constants still None

3. Platform default TOML — overlay on top
   → Fills the design-constant gaps (geometry, spacing, widget metrics)
   → Also fills non-⚙ colors the OS doesn't expose (e.g., Adwaita CSS
     colors on GNOME, Fluent design tokens on Windows)
   → Does NOT duplicate ⚙ values the OS reader already provided

4. App TOML (optional) — overlay on top
   → App developer overrides whatever they want
   → e.g., custom accent, larger buttons, tighter spacing

5. Final validate → ResolvedTheme (no Option fields)
   → If any field is still None: error listing missing fields
```

The merge direction is: **OS + inheritance form the base; TOML
overlays on top.** `Some` values in the TOML always win over the
base. This lets app developers override any native property.

### Where inheritance rules live

| Concern | Location | Rationale |
|---------|----------|-----------|
| Derivation sources (what each None field inherits FROM) | **Rust `resolve()`** | Universal, stable across all platforms and versions. `primary_bg ← accent` is true on macOS Sonoma, macOS Sequoia, Win10, Win11, KDE 5, KDE 6, GNOME 45, GNOME 46. |
| Which ⚙ values to read from the OS | **Rust OS readers** | Can't express API calls in TOML. Each reader knows its platform's APIs. |
| Design constants (geometry, spacing, widget metrics) | **TOML** | Version-specific, easy to update without code changes. |

### Why universal `resolve()` works

The OS reader determines which fields are populated before `resolve()`
runs. Different readers populate different fields. The universal rule
`tooltip.font ← defaults.font` fires on GNOME (no tooltip font
setting) but not on macOS (where the reader provided it from
`+toolTipsFontOfSize:`). Same rule, different outcome — because the
OS readers differ, not the rule.

| Field | KDE reader provides? | GNOME reader provides? | resolve() fills? |
|-------|---------------------|----------------------|-----------------|
| `defaults.font` | Yes (`[General] font`) | Yes (`font-name` gsetting) | No — already set |
| `menu.font` | Yes (`[General] menuFont`) | No (GNOME has no such setting) | KDE: no. GNOME: yes (← defaults.font) |
| `toolbar.font` | Yes (`[General] toolBarFont`) | No | KDE: no. GNOME: yes (← defaults.font) |
| `tooltip.font` | No (KDE has no tooltipFont) | No | Both: yes (← defaults.font) |
| `titlebar.font` | Yes (`[General] activeFont`) | Yes (`titlebar-font` gsetting) | No — already set |
| `status_bar.font` | No (KDE has no statusFont) | No | Both: yes (← defaults.font) |

---

## What Platform Default TOMLs Should Contain

Platform default TOMLs (shipped with this crate: `kde-breeze.toml`,
`adwaita.toml`, `windows-11.toml`, `macos-sonoma.toml`) should be
**minimal**. They supply ONLY values that neither the OS reader nor
`resolve()` can provide — the non-⚙ design constants.

### Include: design constants (non-⚙ values)

These are fixed values from design specifications that no OS API
exposes:

| Category | Examples | Source |
|----------|---------|--------|
| Geometry | `radius`, `radius_lg`, `frame_width`, `disabled_opacity`, `border_opacity` | HIG / Fluent / Breeze src / Adwaita CSS |
| Spacing | `xxs` through `xxl` | HIG / Fluent / Breeze src / Adwaita CSS |
| Widget sizing | `button.min_height`, `button.padding_horizontal`, `checkbox.indicator_size`, `scrollbar.width`, `tab.min_height` | Measured or from style engine source |
| Non-⚙ widget colors | `scrollbar.thumb`, `scrollbar.thumb_hover`, `shadow` | Measured / style engine / CSS |
| Scroll behavior | `scrollbar.overlay_mode` | Platform convention |
| Dialog layout | `dialog.button_order`, `dialog.content_padding`, `dialog.button_spacing` | Platform convention |

Example minimal `kde-breeze.toml`:

```toml
name = "KDE Breeze"

[light.geometry]
radius = 4.0
radius_lg = 8.0
frame_width = 1.0
disabled_opacity = 0.5
border_opacity = 0.2
scroll_width = 10.0
shadow = true

[light.spacing]
xxs = 2.0
xs = 4.0
s = 6.0
m = 8.0
l = 12.0
xl = 16.0
xxl = 24.0

[light.widget_metrics.button]
min_width = 80.0
padding_horizontal = 6.0
icon_spacing = 4.0

[light.widget_metrics.checkbox]
indicator_size = 20.0
spacing = 4.0

# ... remaining widget_metrics sections ...

[light.widget_metrics.splitter]
width = 1.0
```

Note what is **absent**: no `[light.colors]`, no `[light.fonts]`.
KDE's OS reader reads all 35 color roles and 6 font entries from
kdeglobals, so the TOML only needs design constants.

**This is KDE-specific.** GNOME's `adwaita.toml` is much larger
because GNOME only exposes accent + color-scheme + fonts via APIs.
All Adwaita CSS colors (button bg, tooltip bg, sidebar bg, etc.)
are non-⚙ design constants that must be in the TOML. Similarly,
`windows-11.toml` contains Fluent design token colors that the
Windows OS reader does not read.

### Exclude: ⚙ values and derived fields

**⚙ colors** — read from OS, not in platform TOML:
Don't include colors that the OS reader provides. On KDE this is
nearly all colors; on macOS it's all 36; on Windows it's UISettings
+ GetSysColor values; on GNOME it's only the portal accent.

**⚙ fonts** — read from OS:
The OS reader provides body font, mono font, and per-widget fonts
where the platform has separate settings. `resolve()` fills the
rest via inheritance. No font fields in any platform TOML.

**Derived fields** — filled by `resolve()`:
All `←` fields from the uniform inheritance table (above).
`resolve()` fills them from OS-provided sources. No derived fields
in any platform TOML.

**Non-⚙ colors** — these ARE in the platform TOML:
Colors from design guidelines / CSS / style engine source that
the OS does not expose via APIs. Examples: all Adwaita CSS colors
in `adwaita.toml`, Fluent design tokens in `windows-11.toml`,
scrollbar thumb colors (measured, non-⚙ on all platforms).

---

## What App Developer TOMLs Can Contain

An app developer ships a TOML with their application to **customize
the native look** where the app's design requires it. This TOML
overlays on top of the OS + resolve() + platform default TOML base.

Any field set in the app TOML wins. Examples:

```toml
# App wants a custom accent regardless of OS setting
[light.colors]
accent = "#e63946"

# App wants larger buttons than the platform default
[light.widget_metrics.button]
min_height = 40.0
padding_horizontal = 16.0

# App wants tighter spacing
[light.spacing]
s = 4.0
m = 6.0
```

If the app sets `accent` here, the accent-derived fields
(`primary_bg`, `slider.fill`, etc.) still hold the OS-resolved
values unless the app also overrides those. If the app wants
full accent propagation from a custom accent, it sets only
`accent` and relies on a second `resolve()` pass to propagate.
(Pipeline detail TBD in implementation.)

---

## Cross-Platform Presets (catppuccin, nord, etc.)

Cross-platform presets have **no OS reader**. They bypass the OS
entirely and must provide ALL values — colors, fonts, geometry,
spacing, everything:

```toml
name = "Catppuccin Mocha"

[dark.colors]
accent = "#cba6f7"
background = "#1e1e2e"
foreground = "#cdd6f4"
button = "#45475a"
button_foreground = "#cdd6f4"
tooltip = "#313244"
tooltip_foreground = "#cdd6f4"
sidebar = "#181825"
sidebar_foreground = "#cdd6f4"
# ... all direct color roles

[dark.fonts]
family = "Inter"
size = 14.0
mono_family = "JetBrains Mono"
mono_size = 14.0

[dark.geometry]
radius = 8.0
# ... all geometry

[dark.spacing]
# ... full spacing scale

[dark.widget_metrics.button]
# ... full widget metrics
```

Derived fields (`primary_bg`, `slider.fill`, `menu.font`, etc.)
should still be **absent** — `resolve()` fills them from the
preset's own `accent` and `font` values. This keeps the preset
DRY and ensures internal consistency.

---

## Summary

### TOML field disposition by preset type

| Disposition | In platform default TOML | In app developer TOML | In cross-platform preset |
|---|---|---|---|
| **Design constant** (non-⚙) | Yes — sole source | Optional override | Yes — sole source |
| **OS-readable** (⚙ colors, fonts) | **No** — OS reader provides | Optional override | Yes — no OS reader |
| **Derived** (`←` fields) | **No** — `resolve()` fills | Optional override | **No** — `resolve()` fills |

### Platform versioning

| What changed | What to do | Code change? |
|---|---|---|
| Design constants (new radii, spacing, colors) | Write new TOML (`windows-12.toml`) | No |
| New OS API available (e.g., toolbar font) | Add API call to OS reader | Yes (Rust) |
| Removed or deprecated API | Guard or remove API call in reader | Yes (Rust) |
| Different default values for existing APIs | New TOML with updated design constants | No |

New TOML files handle the common case (design constant updates)
without any Rust changes. `resolve()` is stable across versions
because the derivation sources are structural relationships that
don't change: `primary_bg ← accent` and `tooltip.font ← defaults.font`
are true on every platform version.
