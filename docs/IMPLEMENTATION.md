# native-theme: Implementation Specification

An independent, toolkit-agnostic Rust crate that provides a unified theme data model,
TOML-serializable preset theme files for major desktop and mobile platforms, and optional
runtime OS theme reading behind feature flags.

This document consolidates all research, design decisions, and implementation details needed
to build the crate from scratch.

---

## Table of Contents

1. [Executive Summary](#1-executive-summary)
2. [Motivation & Ecosystem Gap](#2-motivation--ecosystem-gap)
3. [Strategic Decision: Why an Independent Crate](#3-strategic-decision-why-an-independent-crate)
4. [Crate Identity](#4-crate-identity)
5. [Platform Capabilities Matrix](#5-platform-capabilities-matrix)
6. [Prior Art & Lessons Learned](#6-prior-art--lessons-learned)
7. [Available Rust Dependencies](#7-available-rust-dependencies)
8. [Data Model](#8-data-model)
9. [TOML Serialization Format](#9-toml-serialization-format)
10. [Bundled Presets](#10-bundled-presets)
11. [Crate Structure](#11-crate-structure)
12. [Feature Flags & Dependencies](#12-feature-flags--dependencies)
13. [Platform Reader Design](#13-platform-reader-design)
14. [Error Handling](#14-error-handling)
15. [Change Notification](#15-change-notification)
16. [Adapter Pattern](#16-adapter-pattern)
17. [Design Decisions & Rationale](#17-design-decisions--rationale)
18. [Implementation Phases](#18-implementation-phases)
19. [Design Questions (All Resolved)](#19-design-questions-all-resolved)
20. [Sources & References](#20-sources--references)

---

## 1. Executive Summary

Every non-native Rust GUI app (egui, iced, gpui, slint, dioxus, tauri) ships hardcoded
colors and spacing that don't match the host OS theme. The only thing any of them detect
today is dark/light mode. No unified, toolkit-agnostic, cross-platform theme data crate
exists in the Rust ecosystem.

`native-theme` fills this gap. It provides:

- A **data model** (36 semantic color roles, fonts, geometry, spacing) as plain Rust types.
- **TOML serialization** for human-readable, human-editable theme files.
- **Bundled presets** extracted from authoritative platform sources (Breeze, Adwaita,
  Windows 11, macOS Sonoma, Material 3, iOS).
- **Runtime OS readers** behind feature flags (Phase 3+) that populate the model from
  live OS settings.
- **Zero GUI toolkit dependencies.** Each toolkit writes a thin adapter (~50 lines) in
  app code.

The MVP (Phase 1) delivers the data model, TOML serde, and 3 presets (`default`,
`kde-breeze`, `adwaita` -- each with light and dark variants). This is immediately usable
in any Rust GUI app.

---

## 2. Motivation & Ecosystem Gap

### Current state

gpui (0.2.2) and gpui-component (0.5.1) illustrate the problem:

- **Dark/Light detection**: `window.observe_window_appearance()` reads the OS preference.
- **Binary theme switching**: `Theme::change(ThemeMode::Light | Dark, None, cx)`.
- **Dynamic sync**: `Theme::sync_system_appearance()` updates when the OS toggles.

Everything else -- colors, spacing, border radius, widget styles -- is hardcoded in two
palettes (Light and Dark). No accent color, no per-platform semantic colors, no font size
inheritance, no border radius from the OS.

This is not specific to gpui. Every Rust GUI toolkit has the same limitation.

### The gap

The theme data exists but is scattered across incompatible sources:

| Source | What it has | Format | Accessible? |
|---|---|---|---|
| KDE `~/.config/kdeglobals` | 60+ color roles, fonts | INI | Readable at runtime |
| KDE Breeze `breezemetrics.h` | ~80 spacing/geometry constants | C++ constants | No API; compiled into libbreeze |
| Adwaita CSS variables | Colors, fonts, `--window-radius`, opacity | CSS | Readable only from inside GTK |
| Adwaita SCSS source | Spacing, padding, all geometry | SCSS | Not exposed; internal build artifact |
| COSMIC `cosmic-theme` | Full model: colors, spacing, corner radii, density, fonts | RON | Rust crate, but coupled to iced/COSMIC |
| Freedesktop portal | Accent color, dark/light, contrast, reduced-motion | D-Bus | Via `ashpd`; 4 values only |
| Windows UISettings | Accent + 6 shades, background, foreground | COM API | Via `windows` crate; colors only |
| macOS NSColor | ~40 semantic colors | ObjC API | Via `objc2-app-kit`; colors only |
| iOS UIColor | ~30 semantic colors + Dynamic Type | ObjC API | Via `objc2-ui-kit`; colors only |
| Android Material You | Dynamic tonal palettes (5x13 = 65 color values, API 31+) | JNI API | Via `jni`; requires Android context |
| `system-theme` 0.3.0 | Dark/light + accent on desktop | Rust | Coupled to iced; not agnostic |

No crate unifies this data into a common, toolkit-agnostic format.

### Why this matters

The audience is large: egui (~15k GitHub stars), iced (~25k), gpui, slint, dioxus, tauri --
all benefit equally from a shared theme data format. The preset files are pure data; anyone
can contribute new presets (Catppuccin, Dracula, Nord, etc.) without writing Rust code.

---

## 3. Strategic Decision: Why an Independent Crate

Three options were evaluated:

### Option A: Independent toolkit-agnostic crate (CHOSEN)

An independent `native-theme` crate on crates.io with a generic data model, TOML serde,
bundled presets, and feature-gated platform readers. Each GUI toolkit writes a thin adapter
(~50 lines) in app code.

**Strengths:**
- Fills a genuine ecosystem gap with no competition.
- Benefits all Rust GUI toolkits equally.
- Presets are pure data -- anyone can contribute.
- Clean separation: data model is stable even if toolkits change APIs.
- TOML format can become a de facto standard for theme exchange.
- The extra effort over a gpui-specific approach is ~1-2 days; the benefit is 1000x audience.

**Weaknesses:**
- Model must be general enough for all toolkits (toolkit-specific tokens like `chart_1`
  are excluded by design; adapters derive them from the closest semantic role).
- Preset maintenance as OS themes evolve (Breeze/Adwaita evolve slowly).

### Option B: gpui-focused crate (REJECTED)

Maps 1:1 to gpui-component's `ThemeConfig`. Uses hex strings, gpui-component field names.

**Why rejected:** Audience of ~10 developers. Couples to gpui-component's internal schema.
The extra cost of generality is trivial.

### Option C: Contribute to gpui-component upstream (REJECTED)

Add TOML theme loading as a PR to gpui-component.

**Why rejected:** Platform-specific theme reading (ashpd, kdeglobals, Windows UISettings,
macOS NSColor) is out of scope for a widget library. The maintainers would reasonably
reject D-Bus and ObjC dependencies. Preset files are opinionated data. Blocks on upstream
review cycles. Helps nobody outside gpui.

**What could still go upstream:** A small PR adding TOML as an alternative input format for
`ThemeConfig` (alongside existing JSON). But presets, OS readers, and selection UI belong
in userland.

---

## 4. Crate Identity

### Name: `native-theme`

**Main type:** `NativeTheme` (self-documenting even without crate namespace).

**Rationale:**
- "Native theme" is the most natural phrase for what the crate provides: the theme that
  makes apps look native on the current OS.
- Matches Electron's `nativeTheme` API -- the most widely known cross-platform precedent
  for this exact concept.
- Works equally for desktop (KDE, GNOME, Windows, macOS) and mobile (Android, iOS).
- Available on crates.io (verified March 2026).
- Short, memorable, unambiguous in context.

**Alternatives considered:**

| Name | Verdict | Reason |
|---|---|---|
| `system-theme` | Taken | 0.3.0, coupled to iced |
| `os-theme` | Available but weak | `os` prefix overloaded in Rust; terse |
| `platform-theme` | Available but vague | "Platform" is too broad (web, consoles) |
| `host-theme` | Available but confusing | Sounds like server/hosting infrastructure |
| `sys-theme` | Available but wrong connotation | `-sys` prefix implies FFI binding crate |

**Prior art naming:**

| Project | Name pattern |
|---|---|
| COSMIC | `cosmic-theme` (coupled to iced/COSMIC) |
| Electron | `nativeTheme` (closest conceptual match) |
| Flutter | `ThemeMode.system` |
| SwiftUI | `.preferredColorScheme` |
| CSS | `prefers-color-scheme` |

---

## 5. Platform Capabilities Matrix

Legend:
- **dynamic** -- readable from OS at runtime via an API or config file; updates when the
  user changes it.
- **static** -- values exist in toolkit source code (C++, SCSS, etc.) or design specs.
  Must be manually extracted and bundled as preset data.
- **n/s** -- not supported / not exposed by the platform.

Source key:
- KDE: `kdeglobals` INI, freedesktop portal via `ashpd`, `breezemetrics.h` C++ source
- GNOME: freedesktop portal via `ashpd`, `gsettings`/dconf, Adwaita CSS variables,
  Adwaita SCSS source
- Windows: `GetSystemMetrics`, `SystemParametersInfo`, `UISettings` COM, registry
- macOS: `NSColor`, `NSFont`, `NSAppearance`, `NSWorkspace` accessibility
- iOS: `UIColor`, `UIFont`, `UITraitCollection`, `UIAccessibility`
- Android: `Configuration`, `Resources`, `Settings.Global`, Material You dynamic colors
  (API 31+)

### 5.1 Mode and Accessibility

| Property | KDE | GNOME | Windows 11 | macOS | iOS | Android |
|---|---|---|---|---|---|---|
| Dark/light preference | dynamic: portal `color-scheme` | dynamic: portal `color-scheme` | dynamic: registry `AppsUseLightTheme` | dynamic: `NSAppearance` | dynamic: `UITraitCollection.userInterfaceStyle` | dynamic: `Configuration.uiMode` |
| High contrast | dynamic: portal `contrast` | dynamic: portal `contrast` | dynamic: `SystemParametersInfo(SPI_GETHIGHCONTRAST)` | dynamic: `accessibilityDisplayShouldIncreaseContrast` | dynamic: `UIAccessibility.isDarkerSystemColorsEnabled` | dynamic: `fontWeightAdjustment` (API 31+) |
| Reduced motion | dynamic: portal `reduce-motion` (Plasma 6.6+) | dynamic: portal `reduce-motion` | dynamic: `SystemParametersInfo(SPI_GETCLIENTAREAANIMATION)` | dynamic: `accessibilityDisplayShouldReduceMotion` | dynamic: `UIAccessibility.isReduceMotionEnabled` | dynamic: `Settings.Global.ANIMATOR_DURATION_SCALE == 0` |

**Design note:** Mode and accessibility flags are environment signals that inform which
theme variant to use. They are NOT part of the `NativeTheme` data model; they are detected
by the consuming app (which already detects dark/light via platform-specific APIs) or
queried via a separate reader API.

### 5.2 Colors -- Core

| Property | KDE | GNOME | Windows 11 | macOS | iOS | Android |
|---|---|---|---|---|---|---|
| Accent color | dynamic: kdeglobals `[Colors:View] DecorationFocus` / portal `accent-color` | dynamic: portal `accent-color` (GNOME 47+) | dynamic: `UISettings.GetColorValue(Accent)` | dynamic: `NSColor.controlAccentColor` | dynamic: app `tintColor` (per-app) | dynamic: `system_accent1_500` (API 31+) |
| Accent shades | n/s | n/s | dynamic: `UISettings` `AccentLight1-3`, `AccentDark1-3` | n/s | n/s | dynamic: `system_accent1_0` through `system_accent1_1000` (13 tones) |
| Window background | dynamic: kdeglobals `[Colors:Window] BackgroundNormal` | static: Adwaita CSS `--window-bg-color` | dynamic: `UISettings.GetColorValue(Background)` | dynamic: `NSColor.windowBackgroundColor` | dynamic: `UIColor.systemBackground` | dynamic: `system_neutral1_10`/`_900` |
| Window foreground | dynamic: kdeglobals `[Colors:Window] ForegroundNormal` | static: Adwaita CSS `--window-fg-color` | dynamic: `UISettings.GetColorValue(Foreground)` | dynamic: `NSColor.labelColor` | dynamic: `UIColor.label` | dynamic: `system_neutral1_900`/`_10` |
| View/content background | dynamic: kdeglobals `[Colors:View] BackgroundNormal` | static: Adwaita CSS `--view-bg-color` | n/s | dynamic: `NSColor.controlBackgroundColor` | dynamic: `UIColor.secondarySystemBackground` | dynamic: `?attr/colorSurface` |
| View/content foreground | dynamic: kdeglobals `[Colors:View] ForegroundNormal` | static: Adwaita CSS `--view-fg-color` | n/s | dynamic: `NSColor.textColor` | dynamic: `UIColor.label` | dynamic: `?attr/colorOnSurface` |
| Border color | n/s (no dedicated border color in kdeglobals; `DecorationHover` is accent-like) | static: Adwaita CSS `--border-color` | n/s | dynamic: `NSColor.separatorColor` | dynamic: `UIColor.separator` | n/s |
| Shadow color | n/s | n/s | n/s | dynamic: `NSColor.shadowColor` | n/s | n/s |

### 5.3 Colors -- Semantic Status

| Property | KDE | GNOME | Windows 11 | macOS | iOS | Android |
|---|---|---|---|---|---|---|
| Error/danger | dynamic: kdeglobals `[Colors:View] ForegroundNegative` | static: Adwaita CSS `--error-bg-color` | n/s | dynamic: `NSColor.systemRedColor` | dynamic: `UIColor.systemRed` | dynamic: `?attr/colorError` |
| Warning | dynamic: kdeglobals `[Colors:View] ForegroundNeutral` | static: Adwaita CSS `--warning-bg-color` | n/s | dynamic: `NSColor.systemOrangeColor` | dynamic: `UIColor.systemOrange` | n/s (not in Material 3) |
| Success | dynamic: kdeglobals `[Colors:View] ForegroundPositive` | static: Adwaita CSS `--success-bg-color` | n/s | dynamic: `NSColor.systemGreenColor` | dynamic: `UIColor.systemGreen` | n/s (not in Material 3) |
| Info | dynamic: kdeglobals `[Colors:View] ForegroundActive` | n/s | n/s | dynamic: `NSColor.systemBlueColor` | dynamic: `UIColor.systemBlue` | n/s |
| Link | dynamic: kdeglobals `[Colors:View] ForegroundLink` | n/s | n/s | dynamic: `NSColor.linkColor` | dynamic: `UIColor.link` | n/s |
| Destructive action | n/s | static: Adwaita CSS `--destructive-bg-color` | n/s | n/s | dynamic: `UIColor.systemRed` | dynamic: `?attr/colorError` |

### 5.4 Colors -- Component

| Property | KDE | GNOME | Windows 11 | macOS | iOS | Android |
|---|---|---|---|---|---|---|
| Button background | dynamic: kdeglobals `[Colors:Button] BackgroundNormal` | n/s | n/s | dynamic: `NSColor.controlColor` | dynamic: `UIColor.systemFill` | dynamic: `?attr/colorPrimaryContainer` |
| Button foreground | dynamic: kdeglobals `[Colors:Button] ForegroundNormal` | n/s | n/s | dynamic: `NSColor.controlTextColor` | n/s | dynamic: `?attr/colorOnPrimaryContainer` |
| Selection background | dynamic: kdeglobals `[Colors:Selection] BackgroundNormal` | n/s | n/s (accent-derived) | dynamic: `NSColor.selectedContentBackgroundColor` | n/s (handles-based) | n/s (accent-derived) |
| Selection foreground | dynamic: kdeglobals `[Colors:Selection] ForegroundNormal` | n/s | n/s | dynamic: `NSColor.selectedTextColor` | n/s | n/s |
| Tooltip background | dynamic: kdeglobals `[Colors:Tooltip] BackgroundNormal` | n/s | n/s | n/s | n/s | n/s |
| Tooltip foreground | dynamic: kdeglobals `[Colors:Tooltip] ForegroundNormal` | n/s | n/s | n/s | n/s | n/s |
| Sidebar background | n/s | static: Adwaita CSS `--sidebar-bg-color` | n/s | dynamic: `NSColor.underPageBackgroundColor` | dynamic: `UIColor.secondarySystemBackground` | dynamic: `?attr/colorSurfaceVariant` |
| Sidebar foreground | n/s | static: Adwaita CSS `--sidebar-fg-color` | n/s | n/s | n/s | dynamic: `?attr/colorOnSurfaceVariant` |
| Header bar background | n/s | static: Adwaita CSS `--headerbar-bg-color` | n/s | n/s | n/s | n/s |
| Header bar foreground | n/s | static: Adwaita CSS `--headerbar-fg-color` | n/s | dynamic: `NSColor.windowFrameTextColor` | n/s | n/s |
| Card/surface background | n/s | static: Adwaita CSS `--card-bg-color` | n/s | n/s | dynamic: `UIColor.secondarySystemGroupedBackground` | dynamic: `?attr/colorSurfaceContainer` |
| Popover background | n/s | static: Adwaita CSS `--popover-bg-color` | n/s | n/s | n/s | n/s |
| Dialog background | n/s | static: Adwaita CSS `--dialog-bg-color` | n/s | n/s | n/s | n/s |
| Disabled text | dynamic: kdeglobals `[Colors:View] ForegroundInactive` | n/s (uses `--disabled-opacity`) | n/s | dynamic: `NSColor.disabledControlTextColor` | dynamic: `UIColor.tertiaryLabel` | n/s (uses opacity) |
| Highlight / find match | n/s | n/s | n/s | dynamic: `NSColor.findHighlightColor` | n/s | n/s |
| Focus ring | dynamic: kdeglobals `[Colors:View] DecorationFocus` | n/s | n/s | dynamic: `NSColor.keyboardFocusIndicatorColor` | n/s | n/s |
| Grid/table lines | n/s | n/s | n/s | dynamic: `NSColor.gridColor` | n/s | n/s |
| Separator/divider | n/s | n/s | n/s | dynamic: `NSColor.separatorColor` | dynamic: `UIColor.separator` | dynamic: `?attr/colorOutlineVariant` |
| Complementary background | dynamic: kdeglobals `[Colors:Complementary] BackgroundNormal` | n/s | n/s | n/s | n/s | n/s |
| Complementary foreground | dynamic: kdeglobals `[Colors:Complementary] ForegroundNormal` | n/s | n/s | n/s | n/s | n/s |
| Alternate row background | dynamic: kdeglobals `[Colors:View] BackgroundAlternate` | n/s | n/s | dynamic: `NSColor.alternatingContentBackgroundColors` | n/s | n/s |
| Menu item selected text | n/s | n/s | n/s | dynamic: `NSColor.selectedMenuItemTextColor` | n/s | n/s |
| Label (secondary) | n/s | n/s | n/s | dynamic: `NSColor.secondaryLabelColor` | dynamic: `UIColor.secondaryLabel` | n/s |
| Label (tertiary) | n/s | n/s | n/s | dynamic: `NSColor.tertiaryLabelColor` | dynamic: `UIColor.tertiaryLabel` | n/s |
| Label (quaternary) | n/s | n/s | n/s | dynamic: `NSColor.quaternaryLabelColor` | dynamic: `UIColor.quaternaryLabel` | n/s |
| Placeholder text | n/s | n/s | n/s | dynamic: `NSColor.placeholderTextColor` | dynamic: `UIColor.placeholderText` | n/s |

### 5.5 Colors -- Palette / Named

| Property | KDE | GNOME | Windows 11 | macOS | iOS | Android |
|---|---|---|---|---|---|---|
| Red | n/s | static: `--red-1`..`--red-5` | n/s | dynamic: `systemRedColor` | dynamic: `systemRed` | n/s |
| Orange | n/s | static: `--orange-1`..`--orange-5` | n/s | dynamic: `systemOrangeColor` | dynamic: `systemOrange` | n/s |
| Yellow | n/s | static: `--yellow-1`..`--yellow-5` | n/s | dynamic: `systemYellowColor` | dynamic: `systemYellow` | n/s |
| Green | n/s | static: `--green-1`..`--green-5` | n/s | dynamic: `systemGreenColor` | dynamic: `systemGreen` | n/s |
| Blue | n/s | static: `--blue-1`..`--blue-5` | n/s | dynamic: `systemBlueColor` | dynamic: `systemBlue` | n/s |
| Purple | n/s | static: `--purple-1`..`--purple-5` | n/s | dynamic: `systemPurpleColor` | dynamic: `systemPurple` | n/s |
| Brown | n/s | static: `--brown-1`..`--brown-5` | n/s | dynamic: `systemBrownColor` | dynamic: `systemBrown` | n/s |
| Pink | n/s | n/s | n/s | dynamic: `systemPinkColor` | dynamic: `systemPink` | n/s |
| Teal | n/s | static: `--teal-1`..`--teal-5` | n/s | dynamic: `systemTealColor` | dynamic: `systemTeal` | n/s |
| Indigo | n/s | n/s | n/s | dynamic: `systemIndigoColor` | dynamic: `systemIndigo` | n/s |
| Mint | n/s | n/s | n/s | n/s | dynamic: `systemMint` | n/s |
| Cyan | n/s | n/s | n/s | dynamic: `systemCyanColor` | dynamic: `systemCyan` | n/s |
| Gray | n/s | static: `--light-1`..`--dark-5` | n/s | dynamic: `systemGrayColor` | dynamic: `systemGray`..`systemGray6` | n/s |

**Design note:** Named palette colors are NOT included in the `NativeTheme` data model.
They are too platform-specific and toolkit-specific to standardize. Instead, adapters map
semantic status colors (danger, warning, success, info) from the model, which covers the
practical use cases. Named palette colors remain available through the platform reader APIs
if needed.

### 5.6 Fonts

| Property | KDE | GNOME | Windows 11 | macOS | iOS | Android |
|---|---|---|---|---|---|---|
| UI font family | dynamic: kdeglobals `[General] font` | dynamic: gsettings `font-name` | dynamic: `SPI_GETNONCLIENTMETRICS` | dynamic: `NSFont.systemFont` | dynamic: `UIFont.systemFont` | static: Roboto |
| UI font size | dynamic: kdeglobals `[General] font` (encoded) | dynamic: gsettings `font-name` (encoded) | dynamic: `NONCLIENTMETRICS.lfMessageFont.lfHeight` | dynamic: `NSFont.systemFontSize` | dynamic: Dynamic Type | dynamic: `fontScale` |
| Monospace font family | dynamic: kdeglobals `[General] fixed` | dynamic: gsettings `monospace-font-name` | static: registry `HKCU\Console\FaceName` | dynamic: `NSFont.monospacedSystemFont` | dynamic: `UIFont.monospacedSystemFont` | static: Droid Sans Mono |
| Monospace font size | dynamic: kdeglobals `[General] fixed` (encoded) | dynamic: gsettings `monospace-font-name` (encoded) | static: registry `HKCU\Console\FontSize` | dynamic: via font size parameter | dynamic: via font size parameter | dynamic: `fontScale` applied |
| Document font family | n/s | static: Adwaita CSS `--document-font-family` | n/s | n/s | n/s | n/s |
| Document font size | n/s | static: Adwaita CSS `--document-font-size` | n/s | n/s | n/s | n/s |

### 5.7 Geometry -- Global

| Property | KDE | GNOME | Windows 11 | macOS | iOS | Android |
|---|---|---|---|---|---|---|
| Border radius (general) | static: `breezemetrics.h` `Frame_FrameRadius` = 5 | static: Adwaita SCSS `$corner-radius` = 12px | static: WinUI3 source (~4px) | static: AppKit (~5px) | static: HIG (~10px) | static: Material 3 medium = 12dp |
| Border radius (large) | n/s | static: `--window-radius` = 15px | n/s | n/s | n/s | static: Material 3 large = 16dp, XL = 28dp |
| Frame/border width | static: `Frame_FrameWidth` = 2 | static: Adwaita SCSS (~1px) | dynamic: `SM_CXBORDER` | n/s | n/s | n/s |
| 3D edge width | n/s | n/s | dynamic: `SM_CXEDGE` | n/s | n/s | n/s |
| Focus border width | n/s | n/s | dynamic: `SM_CXFOCUSBORDER` | n/s | n/s | n/s |
| Drop shadow | static: Breeze source (enabled) | static: Adwaita source (enabled) | n/s | n/s | static: HIG elevation | static: Material 3 elevation |
| Disabled opacity | n/s | static: `--disabled-opacity` | n/s | n/s | n/s | static: Material 3 (0.38) |
| Border opacity | n/s | static: `--border-opacity` | n/s | n/s | n/s | n/s |

### 5.8 Geometry -- Layout Spacing

| Property | KDE | GNOME | Windows 11 | macOS | iOS | Android |
|---|---|---|---|---|---|---|
| Default spacing | static: `Layout_DefaultSpacing` = 6 | static: Adwaita SCSS (~8px) | n/s | n/s | static: HIG (~8pt) | static: Material 3 (8dp grid) |
| Top-level margin | static: `Layout_TopLevelMarginWidth` = 10 | static: Adwaita SCSS | n/s | n/s | static: HIG (~16pt) | static: Material 3 (16dp) |
| Child margin | static: `Layout_ChildMarginWidth` = 6 | static: Adwaita SCSS | n/s | n/s | n/s | n/s |

**Critical observation:** No platform exposes spacing, padding, or border-radius values
through a portable runtime API. These are baked into each toolkit (Qt/Breeze, GTK/Adwaita,
UIKit, Material). The values in presets are **editorial choices** -- measured or approximated
from design specs and toolkit source code. This must be documented clearly so users
understand they are sensible approximations, not pixel-perfect OS data.

### 5.9 Widget Metrics (Desktop Only, Deferred)

Widget-level metrics (~80 constants from KDE `breezemetrics.h`, ~30 from Adwaita SCSS,
~20 dynamic from Windows `GetSystemMetrics`) are comprehensively documented in the original
spec but **deferred to post-1.0**. See Section 8.6 for the deferred data model.

Rationale for deferral:
1. Most toolkit adapters cannot consume per-widget metrics today (gpui hardcodes `.px_N()`;
   egui's `Spacing` covers only a fraction).
2. KDE's ~80 constants are comprehensive but no other platform exposes comparable data,
   making the model asymmetric.
3. Tier 1 (colors, fonts, geometry, spacing) delivers ~90% of the visual integration value.

The full widget metrics tables (window/title bar, button, checkbox/radio, text input,
combobox, spin box, slider, progress bar, scrollbar, menu bar, menu item, tab bar, tooltip,
toolbar, list/tree/table, header, group box, tool box, splitter, dialog, icons/cursors,
misc rendering) are preserved in the appendix for future reference.

### 5.10 Coverage Summary

| Category | KDE | GNOME | Windows | macOS | iOS | Android |
|---|---|---|---|---|---|---|
| Mode / accessibility | 3 dynamic | 3 dynamic | 3 dynamic | 3 dynamic | 3 dynamic | 3 dynamic |
| Colors (core) | 5 dynamic | 5 static, 1 dynamic | 4 dynamic | 7 dynamic | 6 dynamic | 6 dynamic |
| Colors (semantic) | 5 dynamic | 4 static | 0 | 5 dynamic | 6 dynamic | 2 dynamic |
| Colors (component) | 11 dynamic | 7 static | 0 | 17 dynamic | 9 dynamic | 5 dynamic |
| Colors (palette) | 0 | 11 static | 0 | 11 dynamic | 13 dynamic | 0 |
| Fonts | 4 dynamic | 4 dynamic, 2 static | 2 dynamic, 2 static | 4 dynamic | 4 dynamic | 1 static, 1 dynamic |
| Geometry (global) | 3 static | 4 static | 3 dynamic | 0 | 1 static | 2 static |
| Geometry (layout) | 3 static | 3 static | 0 | 0 | 1 static | 1 static |
| Widget metrics | ~80 static | ~30 static | ~20 dynamic | 0 | 0 | 0 |

**Key observations:**

- **KDE** is the richest source: colors are fully dynamic (kdeglobals); geometry is fully
  static but comprehensively documented in `breezemetrics.h`.
- **GNOME/Adwaita** colors are partially exposed through CSS variables (mostly static from
  an external app's perspective). Accent color became dynamic via portal in GNOME 47+.
  libadwaita CSS variables include `--accent-bg-color`, `--accent-fg-color`,
  `--accent-color` (standalone), `--dim-opacity`, `--disabled-opacity`, `--border-opacity`,
  and `--window-radius` (15px default). Notably, `--window-fg-color` uses alpha:
  `rgb(0 0 6 / 80%)` in light mode -- another data point supporting `Rgba` over RGB-only.
- **Windows** has excellent dynamic support for a narrow set: accent + 6 shades, bg/fg,
  some system metrics, and fonts. Widget-internal geometry is not exposed.
- **macOS** has the most dynamic color APIs (~40 semantic NSColor properties) but exposes
  almost zero geometry. Consistency comes from strict HIG enforcement.
- **iOS** mirrors macOS for colors (UIColor parallels NSColor) and adds Dynamic Type for
  font sizes. No geometry exposed.
- **Android** provides the richest dynamic accent system (Material You: 5 palettes x
  13 tones = 65 dynamic values on API 31+). Semantic status colors (warning, success) are
  NOT part of Material 3. No geometry or widget metrics are exposed.

---

## 6. Prior Art & Lessons Learned

### 6.1 `system-theme` 0.3.0

[`system-theme`](https://github.com/danielstuart14/system-theme) (0.3.0, Dec 2025,
Apache-2.0/MIT) is the closest prior art. It provides runtime OS theme reading (dark/light,
contrast, accent color) on Linux, macOS, and Windows, with hardcoded palettes for Fluent,
Aqua, Adwaita, and Breeze (6 color roles each), plus optional iced integration.

We are NOT using it as a dependency because it solves a different problem (runtime reader
with minimal data) than `native-theme` (comprehensive data model + editable presets +
runtime readers). However, its platform reader implementations are well-written and inform
our design.

#### Patterns to adopt

**Error taxonomy.** Distinguishes `Unsupported` (platform can't do this), `Unavailable`
(data exists but can't be read now), `MainThreadRequired` (macOS), and
`Platform(Box<dyn Error>)` (wrapped OS error). This is a good taxonomy. Our error type
adopts a similar structure (see Section 14), but omits `MainThreadRequired` since
NSColor is thread-safe and we handle appearance resolution internally.

**GTK vs Qt detection on Linux.** Checks whether
`org.freedesktop.impl.portal.desktop.gtk` has a D-Bus owner. If yes -> GTK/GNOME; if
no -> Qt/KDE. This is a cheap, reliable heuristic (no file parsing, no env var sniffing).
We replicate this in `from_system()` to decide whether to read kdeglobals or use
portal-only colors.

**Extended DE detection (our improvement):** Also check for:
- `org.freedesktop.impl.portal.desktop.kde` -> KDE
- `org.freedesktop.impl.portal.desktop.cosmic` -> COSMIC
- Fall back to `XDG_CURRENT_DESKTOP` env var if no portal backend is running.

**macOS observer pattern.** Uses `objc2::define_class!` to create a `ThemeObserver`
registering for three notifications:
1. KVO on `NSApplication.effectiveAppearance` -> dark/light changes
2. `NSSystemColorsDidChangeNotification` -> accent color changes
3. `NSWorkspaceAccessibilityDisplayOptionsDidChangeNotification` -> contrast changes

Observer correctly unregisters in `Drop`. This is the right approach for macOS live theme
tracking and we use the same three notification sources.

**Windows API capability checks.** Before calling `UISettings.GetColorValue`, checks
`ApiInformation::IsMethodPresent`. Before reading `AccessibilitySettings.HighContrast`,
checks `ApiInformation::IsPropertyPresent`. This graceful degradation on older Windows
versions is important and we replicate it.

**Signal-based Linux change notification.** Subscribes to the portal's `SettingChanged`
D-Bus signal in a background thread, then broadcasts via `tokio::sync::Notify`. The pattern
of one background watcher + broadcast for fan-out is clean.

#### Pitfalls to avoid

**Hard `tokio` dependency.** Requires `tokio` even for synchronous `get_theme()` calls.
For `native-theme`, async support and change notifications are behind a feature flag. The
core crate (data model + presets + sync `from_*()` readers) must NOT pull in an async
runtime.

**No kdeglobals fallback.** Only queries the freedesktop portal, which provides 4 values
(color-scheme, accent-color, contrast, reduced-motion). Misses the 60+ color roles in
`~/.config/kdeglobals`. Our Linux reader tries the portal first for live values it supports,
then falls back to kdeglobals parsing for the full palette.

**Minimal palette (6 colors).** `ThemePalette` has only `background`, `foreground`,
`accent`, `success`, `warning`, `danger`. Mapping only 6 fields leaves ~30 theme fields
at defaults, producing a "tinted but still generic" look. Our model needs 36 semantic roles
to meaningfully close the gap.

**No alpha channel.** `ThemeColor` is RGB-only (`f32` components). Several platform tokens
have meaningful transparency (macOS `NSColor.shadowColor`, GNOME `--border-opacity`, Material
3 disabled states at 38%). Adding alpha later is a breaking change. We use `Rgba` (8-bit per
channel with alpha) from the start.

**No serialization / no user-editable themes.** Palettes are `const` Rust structs. Users
cannot load, save, or edit themes. Community presets (Catppuccin, Nord, etc.) are impossible
without code changes. TOML serialization is a core value proposition of `native-theme`.

**Toolkit integration built in.** The iced adapter lives inside `system-theme` behind a
feature flag, coupling the crate to iced's version. Our approach: adapters live in the
consuming app (~50 lines each), keeping `native-theme` fully toolkit-agnostic.

### 6.2 COSMIC `cosmic-theme`

[`cosmic-theme`](https://github.com/pop-os/libcosmic/tree/master/cosmic-theme) is a
comprehensive theme system (colors, spacing scale, corner radii, density, fonts) serialized
as RON. It proves the model works but is locked to iced/COSMIC's ecosystem. Not reusable
outside COSMIC applications.

### 6.3 `dark-light` 2.0

Provides `detect() -> Result<Mode, Error>` where `Mode` is `Dark | Light | Unspecified`,
on all desktop platforms. Solves only dark/light detection -- no colors, no fonts, no
geometry. Too minimal for our needs, but validates that dark/light detection across
platforms is well-understood.

---

## 7. Available Rust Dependencies

### 7.1 Core (always required)

| Crate | Version | Why |
|---|---|---|
| `serde` | 1.x | Serialization derive macros |
| `toml` | 1.0.x | TOML parsing and serialization |

These are the ONLY mandatory dependencies. With no feature flags enabled, `native-theme`
depends only on `serde` + `toml`.

### 7.2 Linux (feature-gated)

| Crate | Version | Feature | Why |
|---|---|---|---|
| `ashpd` | 0.13.x | `portal` | XDG Desktop Portal D-Bus wrapper. Reads `color-scheme`, `accent-color`, `contrast`, `reduce-motion`. Supports change notifications via async stream. Works on KDE Plasma 6 and GNOME 47+. **ashpd is async-only** (built on zbus 5.x); the `portal` feature therefore requires an async runtime. **ashpd's default features include `tokio`**, so enabling the `portal` feature automatically pulls in tokio. For async-std, use `default-features = false, features = ["async-std"]`. |
| `configparser` | 3.1.x | `kde` | Zero-dependency INI parser for `~/.config/kdeglobals`. `Ini::load()` accepts `AsRef<Path>`, so `PathBuf` works directly. Stable KDE format. Gives full color palette (60+ roles). Fully synchronous. |

#### ashpd API for settings reading

ashpd 0.13 provides typed constants and enums for the freedesktop appearance namespace:

```rust
use ashpd::desktop::settings::{
    Settings, ColorScheme, Contrast,
    APPEARANCE_NAMESPACE, COLOR_SCHEME_KEY, ACCENT_COLOR_SCHEME_KEY, CONTRAST_KEY,
};

let settings = Settings::new().await?;

// Dark/light preference (typed enum)
let scheme = settings
    .read::<ColorScheme>(APPEARANCE_NAMESPACE, COLOR_SCHEME_KEY)
    .await?;
// ColorScheme::NoPreference | PreferDark | PreferLight

// High contrast (typed enum)
let contrast = settings
    .read::<Contrast>(APPEARANCE_NAMESPACE, CONTRAST_KEY)
    .await?;

// Accent color via generic read (D-Bus wire type is `(ddd)`)
// Out-of-range values should be treated as "unset"
let (r, g, b) = settings
    .read::<(f64, f64, f64)>(APPEARANCE_NAMESPACE, ACCENT_COLOR_SCHEME_KEY)
    .await?;
// Or use the convenience method which returns ashpd::desktop::Color (f64 RGB):
// let color = settings.accent_color().await?;
// color.red(), color.green(), color.blue() -> f64 in [0.0, 1.0]

// Listen for live changes (returns a Stream<Item = Setting>)
let mut stream = settings.receive_setting_changed().await?;
while let Some(change) = stream.next().await {
    // change.namespace(), change.key(), change.value()
}

// Convenience methods also available for common settings:
// settings.color_scheme().await?        -> ColorScheme
// settings.accent_color().await?        -> Color
// settings.contrast().await?            -> Contrast
// settings.receive_accent_color_changed().await?  -> Stream<Item = Color>
```

**Important:** The `accent-color` key is defined in the
`org.freedesktop.appearance` namespace of the XDG Desktop Portal Settings interface. The
underlying D-Bus value is a struct of three doubles (RGB in [0.0, 1.0] sRGB). Out-of-range
values signal "no accent color set." We convert to `Rgba` by clamping and scaling:
`(v.clamp(0.0, 1.0) * 255.0).round() as u8`.

**Async caveat:** All ashpd calls are async (zbus is async-only). This means portal
reading requires an async runtime. To keep the core crate sync-friendly, portal reading
is behind a separate `portal` feature flag (see Section 12). The `kde` feature (kdeglobals
parsing) is fully synchronous and does NOT depend on ashpd.

#### KDE kdeglobals parsing

`~/.config/kdeglobals` is an INI file with sections `[Colors:Window]`, `[Colors:View]`,
`[Colors:Button]`, `[Colors:Selection]`, `[Colors:Tooltip]`, `[Colors:Complementary]`,
and `[Colors:Header]` (with `[Colors:Header][Inactive]` sub-section). Each contains:

- `BackgroundNormal`, `BackgroundAlternate` (R,G,B integers)
- `ForegroundNormal`, `ForegroundActive`, `ForegroundInactive`, `ForegroundLink`,
  `ForegroundVisited`
- `ForegroundNegative`, `ForegroundNeutral`, `ForegroundPositive`
- `DecorationFocus`, `DecorationHover`

```rust
use configparser::ini::Ini;

fn parse_kde_rgb(s: &str) -> Option<Rgba> {
    let parts: Vec<&str> = s.split(',').collect();
    if parts.len() < 3 { return None; }
    Some(Rgba::rgb(
        parts[0].trim().parse().ok()?,
        parts[1].trim().parse().ok()?,
        parts[2].trim().parse().ok()?,
    ))
}

// NOTE: configparser lowercases section and key names by default (Ini::new()).
// Use Ini::new_cs() for case-sensitive mode, which is needed here since
// kdeglobals keys are CamelCase (e.g., "BackgroundNormal", "ForegroundNormal").
let mut config = Ini::new_cs();
// load() returns Result<_, String> -- map the error for our Error type
let config_dir = dirs::config_dir()
    .ok_or_else(|| Error::Unavailable("no config directory found".into()))?;
config.load(config_dir.join("kdeglobals"))
    .map_err(|e| Error::Unavailable(e))?;

let bg = config.get("Colors:Window", "BackgroundNormal")
    .and_then(|v| parse_kde_rgb(&v));
let fg = config.get("Colors:Window", "ForegroundNormal")
    .and_then(|v| parse_kde_rgb(&v));
let accent = config.get("Colors:View", "DecorationFocus")
    .and_then(|v| parse_kde_rgb(&v));
```

### 7.3 Windows (feature-gated)

| Crate | Version | Feature | Why |
|---|---|---|---|
| `windows` | 0.62.x | `windows` | Official Microsoft crate. `UISettings::GetColorValue(UIColorType)` for accent + 6 shades, bg, fg. Also `GetSystemMetrics` for system metrics. |

```rust
use windows::UI::ViewManagement::{UISettings, UIColorType};

let settings = UISettings::new()?;
let accent = settings.GetColorValue(UIColorType::Accent)?;
// accent.R, accent.G, accent.B, accent.A (all u8)
let bg = settings.GetColorValue(UIColorType::Background)?;
let is_dark = bg.R == 0 && bg.G == 0 && bg.B == 0;
```

UIColorType variants: `Background`, `Foreground`, `Accent`, `AccentDark1`..`AccentDark3`,
`AccentLight1`..`AccentLight3`. (`Complement` also exists but is unsupported by Microsoft.)

**Important:** Before calling `UISettings.GetColorValue`, check
`ApiInformation::IsMethodPresent` for graceful degradation on older Windows versions.

### 7.4 macOS (feature-gated)

| Crate | Version | Feature | Why |
|---|---|---|---|
| `objc2-app-kit` | 0.3.x | `macos` | Low-level NSColor access. `controlAccentColor`, `windowBackgroundColor`, `labelColor`, `systemRedColor`, etc. Requires unsafe Obj-C interop. |

**Thread safety:** NSColor objects are immutable once created, and immutable Foundation
objects are generally thread-safe per Apple's threading guidelines. NSColor does NOT
require the main thread for reading component values from a concrete color. However,
**appearance resolution** of dynamic/semantic colors (e.g., `labelColor`, which changes
between light and dark mode) depends on `NSAppearance.current` being set correctly.
The reader should resolve the appearance context on the main thread (via
`NSAppearance.performAsCurrentDrawingAppearance` on macOS 11+, or by setting
`NSAppearance.current` directly) before extracting color components. Once colors are
resolved to concrete sRGB values, the resulting data is safe to use from any thread.

**Alternative considered:** `cacao` 0.3.2 wraps 39 NSColor semantic variants but is
missing `controlAccentColor`. Not sufficient for our needs.

### 7.5 iOS (feature-gated)

| Crate | Version | Feature | Why |
|---|---|---|---|
| `objc2-ui-kit` | 0.3.x | `ios` | UIColor semantic colors, UIFont with Dynamic Type, UITraitCollection for appearance. Same objc2 ecosystem as macOS. |

### 7.6 Android (feature-gated)

| Crate | Version | Feature | Why |
|---|---|---|---|
| `jni` | 0.22.x | `android` | JNI bridge to `Resources.getColor(android.R.color.system_accent1_*)` for Material You dynamic colors (API 31+). Verbose. Requires Android JNI context. |
| `ndk` | 0.9.x | `android` | Android NDK bindings. `Configuration` access for `uiMode` (dark/light), font scale. |

**Note:** Android theme reading from Rust is immature. No crate wraps Material You dynamic
colors ergonomically. The Android reader would need JNI calls into
`android.content.res.Resources` and `android.provider.Settings`. Deferred to Phase 7.

### 7.7 Optional utilities

| Crate | Purpose | When |
|---|---|---|
| `notify` 8.x (latest stable: 8.2.0; 9.0.0-rc.2 available) | File system watching | Feature `watch` -- watch `~/.config/kdeglobals` for changes |
| `dirs` 6.x | XDG/platform config directories | Feature `kde` -- find `~/.config/kdeglobals` |

---

## 8. Data Model

The model covers the **union** of what all platforms provide, organized into tiers.
**MVP (Phase 1) includes Tier 1 only.**

All types are `Send + Sync` (they are plain data). All color values are in sRGB color space.
Font sizes and geometry values are in **logical pixels** (CSS-like); the consuming toolkit
handles DPI scaling.

All public structs (`NativeTheme`, `ThemeVariant`, `ThemeColors`, `ThemeFonts`,
`ThemeGeometry`, `ThemeSpacing`) use `#[non_exhaustive]` to allow adding new fields in
minor versions without breaking downstream code. Constructors are provided via `Default`
and serde deserialization.

### 8.1 Top-Level: `NativeTheme`

```rust
/// A complete theme with optional light and dark variants.
///
/// TOML preset files typically define both variants. Runtime readers populate
/// the variant matching the current OS mode; the other variant is `None`.
/// The consuming app picks the right variant based on OS dark/light preference
/// (detected separately, NOT part of this struct).
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct NativeTheme {
    pub name: String,
    pub light: Option<ThemeVariant>,
    pub dark: Option<ThemeVariant>,
}
```

```rust
impl NativeTheme {
    /// Create a new empty theme with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            light: None,
            dark: None,
        }
    }
}
```

**Design decision: No `version` field.** TOML format versioning is handled by serde
compatibility (new optional fields are backward-compatible). If a breaking change is ever
needed, the crate major version bumps. Adding a `version` field to the struct would require
validation logic and error paths that provide little value given serde's natural evolution.

**Design decision: No `is_dark` / `active` field.** The theme IS the data; mode detection
is handled separately by the consuming app. Runtime readers populate whichever variant
matches the current OS mode. The consuming app already knows which mode is active (via
`window.observe_window_appearance()` in gpui, or equivalent in other toolkits).

### 8.2 `ThemeVariant`

```rust
/// A single theme variant (light or dark).
///
/// All sub-structs derive Default (all fields None), and #[serde(default)]
/// ensures missing TOML sections deserialize as the default rather than
/// causing an error. For example, a TOML file with [light.colors] but no
/// [light.fonts] will produce ThemeFonts { family: None, size: None, ... }.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct ThemeVariant {
    pub colors: ThemeColors,
    pub fonts: ThemeFonts,
    pub geometry: ThemeGeometry,
    pub spacing: ThemeSpacing,
    /// Per-widget sizing and spacing metrics (v0.2+).
    /// Optional because not all themes or presets provide widget metrics.
    /// When merging, if both base and overlay have widget metrics they are
    /// merged recursively; if only the overlay has them they are cloned.
    pub widget_metrics: Option<WidgetMetrics>,
}
```

### 8.3 Color type: `Rgba`

```rust
/// 8-bit-per-channel sRGB color with alpha.
///
/// Alpha is included because several platform tokens have meaningful transparency:
/// - macOS `NSColor.shadowColor` (has alpha)
/// - GNOME `--border-opacity` (applied to border color)
/// - Material 3 disabled states (38% opacity)
/// - Various overlay and scrim colors
///
/// Using Rgba from the start avoids a painful retrofit. system-theme's RGB-only
/// `ThemeColor` is a known limitation we deliberately avoid.
///
/// Hex serialization: `#RRGGBB` (alpha defaults to FF) or `#RRGGBBAA`.
/// This follows the CSS convention.
///
/// All values are in sRGB color space. macOS P3 display colors are converted
/// to sRGB on read (NSColor provides colorUsingColorSpace: for this).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Rgba {
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}
```

**Custom serde:** `Rgba` uses custom `Serialize`/`Deserialize` to read/write hex strings.
6-digit `#RRGGBB` implies `a = 0xFF`. 8-digit `#RRGGBBAA` includes explicit alpha.

`Rgba` also implements `Display` (outputs `#rrggbb` or `#rrggbbaa`) and
`FromStr` (parses the same formats) for convenience outside serde contexts.

Implementation approach: implement `serde::Serialize` and `serde::Deserialize` manually
(~30 lines each) rather than pulling in a dependency like `hex_color` or `serde-hex`. The
logic is trivial and avoids an extra dependency in the always-on path.

```rust
// Serialize (via serde::Serializer::serialize_str):
let hex = if self.a == 255 {
    format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
} else {
    format!("#{:02x}{:02x}{:02x}{:02x}", self.r, self.g, self.b, self.a)
};
serializer.serialize_str(&hex)

// Deserialize: strip leading '#', then parse 6-char or 8-char hex string.
// 6-char: RRGGBB (alpha = 0xFF). 8-char: RRGGBBAA.
// Reject other lengths with a descriptive error.
```

**Why u8 instead of f32:** All platform sources are 8-bit or can be losslessly converted.
KDE kdeglobals uses integer R,G,B. Windows UISettings returns u8 Color components. macOS
NSColor can be read via `getRed:green:blue:alpha:` (CGFloat -> u8 with no visible precision
loss; 256 levels per channel exceeds human perception). The freedesktop portal returns
`(f64, f64, f64)` in [0.0, 1.0] which converts to u8 by `(v * 255.0).round() as u8`.

### 8.4 Tier 1a: `ThemeColors` (36 semantic roles)

```rust
/// All theme colors as a flat set of 36 semantic color roles.
///
/// Organized into logical groups (core, primary, secondary, status,
/// interactive, panel, component) but stored as direct fields for
/// simpler access and flatter TOML serialization.
///
/// All fields are `Option<T>`:
/// - Different platforms expose different subsets (see coverage matrix).
/// - Preset files only specify known values.
/// - Unspecified values are left to the consuming toolkit's defaults.
/// - This enables layering: load a base preset, overlay with user overrides.
///
/// Uses `#[serde_with::skip_serializing_none]` to produce clean TOML output
/// (only specified values are written).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct ThemeColors {
    // Core (7)
    pub accent: Option<Rgba>,
    pub background: Option<Rgba>,       // window/app background
    pub foreground: Option<Rgba>,       // primary text
    pub surface: Option<Rgba>,          // card/view/content background
    pub border: Option<Rgba>,
    pub muted: Option<Rgba>,            // secondary/inactive text
    pub shadow: Option<Rgba>,           // often has meaningful alpha

    // Primary (2) -- v0.2: renamed from primary/primary_foreground
    pub primary_background: Option<Rgba>,   // primary action background
    pub primary_foreground: Option<Rgba>,

    // Secondary (2) -- v0.2: renamed from secondary/secondary_foreground
    pub secondary_background: Option<Rgba>, // secondary action background
    pub secondary_foreground: Option<Rgba>,

    // Status (8)
    pub danger: Option<Rgba>,
    pub danger_foreground: Option<Rgba>,
    pub warning: Option<Rgba>,
    pub warning_foreground: Option<Rgba>,
    pub success: Option<Rgba>,
    pub success_foreground: Option<Rgba>,
    pub info: Option<Rgba>,
    pub info_foreground: Option<Rgba>,

    // Interactive (4)
    pub selection: Option<Rgba>,
    pub selection_foreground: Option<Rgba>,
    pub link: Option<Rgba>,
    pub focus_ring: Option<Rgba>,

    // Panel (6)
    pub sidebar: Option<Rgba>,
    pub sidebar_foreground: Option<Rgba>,
    pub tooltip: Option<Rgba>,
    pub tooltip_foreground: Option<Rgba>,
    pub popover: Option<Rgba>,
    pub popover_foreground: Option<Rgba>,

    // Component (7)
    pub button: Option<Rgba>,
    pub button_foreground: Option<Rgba>,
    pub input: Option<Rgba>,
    pub input_foreground: Option<Rgba>,
    pub disabled: Option<Rgba>,
    pub separator: Option<Rgba>,
    pub alternate_row: Option<Rgba>,
}
```

**v0.2 change:** The primary and secondary fields were renamed from `primary`/`secondary`
to `primary_background`/`secondary_background` for clarity and consistency with the
`*_foreground` naming pattern throughout the struct.

**Platform mapping for each color role:**

| `ThemeColors` field | KDE source | GNOME source | Windows source | macOS source | iOS source | Android source |
|---|---|---|---|---|---|---|
| `accent` | `[Colors:View] DecorationFocus` / portal | portal `accent-color` | `UIColorType::Accent` | `controlAccentColor` | `tintColor` | `system_accent1_500` |
| `background` | `[Colors:Window] BackgroundNormal` | `--window-bg-color` | `UIColorType::Background` | `windowBackgroundColor` | `systemBackground` | `system_neutral1_10/900` |
| `foreground` | `[Colors:Window] ForegroundNormal` | `--window-fg-color` | `UIColorType::Foreground` | `labelColor` | `label` | `system_neutral1_900/10` |
| `surface` | `[Colors:View] BackgroundNormal` | `--view-bg-color` | n/s | `controlBackgroundColor` | `secondarySystemBackground` | `?attr/colorSurface` |
| `border` | n/s (editorial approximation) | `--border-color` | n/s | `separatorColor` | `separator` | n/s |
| `muted` | `[Colors:View] ForegroundInactive` | n/s | n/s | `secondaryLabelColor` | `secondaryLabel` | n/s |
| `shadow` | n/s | n/s | n/s | `shadowColor` | n/s | n/s |
| `primary_background` | `[Colors:View] DecorationFocus` | `--accent-bg-color` | `Accent` | `controlAccentColor` | `tintColor` | `?attr/colorPrimary` |
| `primary_foreground` | `[Colors:Selection] ForegroundNormal` | `--accent-fg-color` | derived white/black | derived | n/s | `?attr/colorOnPrimary` |
| `secondary_background` | `[Colors:Button] BackgroundNormal` | `--headerbar-bg-color` | n/s | `controlColor` | `systemFill` | `?attr/colorSecondaryContainer` |
| `secondary_foreground` | `[Colors:Button] ForegroundNormal` | `--headerbar-fg-color` | n/s | `controlTextColor` | n/s | `?attr/colorOnSecondaryContainer` |
| `danger` | `[Colors:View] ForegroundNegative` | `--error-bg-color` | n/s | `systemRedColor` | `systemRed` | `?attr/colorError` |
| `warning` | `[Colors:View] ForegroundNeutral` | `--warning-bg-color` | n/s | `systemOrangeColor` | `systemOrange` | n/s |
| `success` | `[Colors:View] ForegroundPositive` | `--success-bg-color` | n/s | `systemGreenColor` | `systemGreen` | n/s |
| `info` | `[Colors:View] ForegroundActive` | n/s | n/s | `systemBlueColor` | `systemBlue` | n/s |
| `link` | `[Colors:View] ForegroundLink` | n/s | n/s | `linkColor` | `link` | n/s |
| `selection` | `[Colors:Selection] BackgroundNormal` | n/s | accent-derived | `selectedContentBackgroundColor` | n/s | accent-derived |
| `selection_foreground` | `[Colors:Selection] ForegroundNormal` | n/s | n/s | `selectedTextColor` | n/s | n/s |
| `focus_ring` | `[Colors:View] DecorationFocus` | n/s | n/s | `keyboardFocusIndicatorColor` | n/s | n/s |
| `sidebar` | n/s | `--sidebar-bg-color` | n/s | `underPageBackgroundColor` | `secondarySystemBackground` | `?attr/colorSurfaceVariant` |
| `tooltip` | `[Colors:Tooltip] BackgroundNormal` | n/s | n/s | n/s | n/s | n/s |
| `button` | `[Colors:Button] BackgroundNormal` | n/s | n/s | `controlColor` | `systemFill` | `?attr/colorPrimaryContainer` |
| `button_foreground` | `[Colors:Button] ForegroundNormal` | n/s | n/s | `controlTextColor` | n/s | `?attr/colorOnPrimaryContainer` |
| `input` | `[Colors:View] BackgroundNormal` | `--view-bg-color` | n/s | `textBackgroundColor` | `systemBackground` | `?attr/colorSurfaceContainerHighest` |
| `input_foreground` | `[Colors:View] ForegroundNormal` | `--view-fg-color` | n/s | `textColor` | `label` | `?attr/colorOnSurface` |
| `disabled` | `[Colors:View] ForegroundInactive` | computed (fg * `--disabled-opacity`) | n/s | `disabledControlTextColor` | `tertiaryLabel` | computed (fg * 0.38 opacity) |
| `separator` | n/s | n/s | n/s | `separatorColor` | `separator` | `?attr/colorOutlineVariant` |
| `alternate_row` | `[Colors:View] BackgroundAlternate` | n/s | n/s | `alternatingContentBackgroundColors` | n/s | n/s |

**Note on `disabled`:** GNOME Adwaita and Android Material 3 handle disabled states via
opacity (Adwaita: `--disabled-opacity` = 0.5; Material 3: 0.38) rather than a dedicated color. The
reader should compute the actual disabled color by applying the opacity to the foreground
color: `disabled = foreground * opacity + background * (1 - opacity)`. Platforms that
provide a dedicated disabled color (KDE, macOS, iOS) use it directly.

### 8.5 Tier 1b-1d: Fonts, Geometry, Spacing

```rust
/// Font settings. Sizes are in logical pixels (the toolkit handles DPI scaling).
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ThemeFonts {
    pub family: Option<String>,      // UI font family name
    pub size: Option<f32>,           // UI font size in px
    pub mono_family: Option<String>, // Monospace font family name
    pub mono_size: Option<f32>,      // Monospace font size in px
}

/// Geometric properties for UI elements.
///
/// Controls border radius, frame widths, and opacity values
/// that vary across platform themes.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct ThemeGeometry {
    pub radius: Option<f32>,            // corner radius for rounded elements (px)
    pub radius_lg: Option<f32>,         // larger radius for dialogs, cards, panels (px) [v0.2]
    pub frame_width: Option<f32>,       // window/widget frame border width (px)
    pub disabled_opacity: Option<f32>,  // opacity for disabled elements (0.0-1.0)
    pub border_opacity: Option<f32>,    // opacity for borders (0.0-1.0)
    pub scroll_width: Option<f32>,      // scrollbar track width (px)
    pub shadow: Option<bool>,           // whether drop shadows are used [v0.2]
}

/// Named spacing scale in logical pixels.
///
/// NOTE: No platform exposes a spacing scale at runtime. Values in presets are
/// editorial choices -- measured or approximated from design specs and toolkit
/// source code (breezemetrics.h, Adwaita SCSS, Material Design guidelines,
/// Apple HIG). This is documented clearly so users understand they are sensible
/// approximations, not pixel-perfect OS data.
///
/// Different toolkits handle spacing differently:
/// - gpui: hardcoded in render methods (.px_2(), .gap_1()) -- NOT theme-driven
/// - egui: Spacing struct with item_spacing, button_padding, indent, etc.
/// - iced: per-widget padding/spacing values
/// - COSMIC: 10-level named scale (space_none through space_xxxl)
///
/// A generic spacing model is included for toolkits that can consume it and
/// for forward-compatibility. Document that not all toolkits use it today.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ThemeSpacing {
    pub xxs: Option<f32>,   // ~2px
    pub xs: Option<f32>,    // ~4px
    pub s: Option<f32>,     // ~6px
    pub m: Option<f32>,     // ~8px
    pub l: Option<f32>,     // ~12px
    pub xl: Option<f32>,    // ~16px
    pub xxl: Option<f32>,   // ~24px
}
```

### 8.6 `WidgetMetrics` (v0.2)

Per-widget sizing and spacing metrics, implemented in v0.2. Contains 12 sub-structs for
specific widget types. Sub-structs use bare fields (not `Option`-wrapped) with
`skip_serializing_if = "is_empty"` to omit empty sub-structs from TOML output.

`WidgetMetrics` is held as `Option<WidgetMetrics>` in `ThemeVariant` for backward
compatibility -- presets without widget metrics simply omit the field.

```rust
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct WidgetMetrics {
    pub button: ButtonMetrics,
    pub checkbox: CheckboxMetrics,
    pub input: InputMetrics,
    pub scrollbar: ScrollbarMetrics,
    pub slider: SliderMetrics,
    pub progress_bar: ProgressBarMetrics,
    pub tab: TabMetrics,
    pub menu_item: MenuItemMetrics,
    pub tooltip: TooltipMetrics,
    pub list_item: ListItemMetrics,
    pub toolbar: ToolbarMetrics,
    pub splitter: SplitterMetrics,
}
```

Each sub-struct has all `Option<f32>` fields. Examples:

```rust
pub struct ButtonMetrics {
    pub min_width: Option<f32>,
    pub min_height: Option<f32>,
    pub padding_horizontal: Option<f32>,
    pub padding_vertical: Option<f32>,
    pub icon_spacing: Option<f32>,
}

pub struct CheckboxMetrics {
    pub indicator_size: Option<f32>,
    pub spacing: Option<f32>,
}

pub struct ScrollbarMetrics {
    pub width: Option<f32>,
    pub min_thumb_height: Option<f32>,
    pub slider_width: Option<f32>,
}

pub struct SliderMetrics {
    pub track_height: Option<f32>,
    pub thumb_size: Option<f32>,
    pub tick_length: Option<f32>,
}
```

Other sub-structs follow the same pattern: `InputMetrics` (min_height, padding_horizontal,
padding_vertical), `ProgressBarMetrics` (height, min_width), `TabMetrics` (min_width,
min_height, padding_horizontal, padding_vertical), `MenuItemMetrics` (height,
padding_horizontal, padding_vertical, icon_spacing), `TooltipMetrics` (padding, max_width),
`ListItemMetrics` (height, padding_horizontal, padding_vertical), `ToolbarMetrics` (height,
item_spacing, padding), `SplitterMetrics` (width).

Platform reader functions populate widget metrics from compile-time constants:
- KDE: `breeze_widget_metrics()` sourced from `breezemetrics.h`
- macOS: `macos_widget_metrics()` sourced from Apple HIG
- GNOME: `adwaita_widget_metrics()` sourced from libadwaita/GTK4
- Windows: DPI-aware metrics from `GetSystemMetricsForDpi` + WinUI3 Fluent defaults

All 17 bundled presets include widget_metrics data. Community color themes use generic
defaults (not platform-specific values). Widget sizing is mode-independent, so light and
dark variants receive identical widget_metrics.

### 8.7 Tier 3: Platform Readers (feature-gated)

```rust
// Sync readers (no async runtime required)

/// Feature: "kde". Reads ~/.config/kdeglobals. Fully synchronous.
pub fn from_kde() -> Result<NativeTheme, Error>;

/// Feature: "windows". Reads UISettings + GetSystemMetrics. Synchronous.
pub fn from_windows() -> Result<NativeTheme, Error>;

/// Feature: "macos". Reads NSColor + NSFont. Synchronous.
/// NSColor is thread-safe; only NSAppearance.currentAppearance resolution
/// needs the main thread (handled internally by the reader).
pub fn from_macos() -> Result<NativeTheme, Error>;

// Async readers (require an async runtime)

/// Feature: "portal". Reads freedesktop portal (accent, scheme, contrast).
/// Returns Adwaita defaults overlaid with portal values.
pub async fn from_gnome() -> Result<NativeTheme, Error>;

/// Feature: "ios". Deferred to Phase 7.
pub fn from_ios() -> Result<NativeTheme, Error>;

/// Feature: "android". Deferred to Phase 7.
pub fn from_android() -> Result<NativeTheme, Error>;

/// Auto-detect platform and read the current OS theme.
///
/// On Linux, uses DE detection heuristic (see Section 13) to decide
/// between from_kde() and loading an Adwaita preset.
///
/// Returns a NativeTheme with the active variant (light or dark) populated.
/// The inactive variant is None. The consuming app detects which mode is
/// active via its own platform APIs.
pub fn from_system() -> Result<NativeTheme, Error>;
```

---

## 9. TOML Serialization Format

TOML keys map 1:1 to struct field names. All fields are `Option<T>` -- a theme file only
needs to specify the values it wants to override. Unspecified values are left to the
consuming toolkit's defaults.

Color format: `#RRGGBB` (6-digit, alpha defaults to FF) or `#RRGGBBAA` (8-digit, explicit
alpha). Follows the CSS convention.

### Example: KDE Breeze preset (Plasma 6 / current)

```toml
name = "KDE Breeze"

[light]

[light.fonts]
family = "Noto Sans"
size = 10.0
mono_family = "Hack"
mono_size = 10.0

[light.geometry]
radius = 5.0
radius_lg = 7.0
shadow = true               # Breeze enables drop shadows (compositor-drawn)
frame_width = 2.0

[light.spacing]
xxs = 2.0
xs = 4.0
s = 6.0
m = 8.0
l = 12.0
xl = 16.0
xxl = 24.0

[light.colors]
accent = "#3daee9"          # [Colors:View] DecorationFocus = 61,174,233
background = "#eff0f1"      # [Colors:Window] BackgroundNormal = 239,240,241
foreground = "#232629"      # [Colors:Window] ForegroundNormal = 35,38,41
surface = "#ffffff"          # [Colors:View] BackgroundNormal = 255,255,255
border = "#bdc3c7"          # editorial: KDE has no border color; approximated
muted = "#707d8a"           # [Colors:View] ForegroundInactive = 112,125,138
primary = "#3daee9"          # = accent
primary_foreground = "#ffffff" # [Colors:Selection] ForegroundNormal = 255,255,255
secondary = "#fcfcfc"       # [Colors:Button] BackgroundNormal = 252,252,252
secondary_foreground = "#232629" # [Colors:Button] ForegroundNormal = 35,38,41
danger = "#da4453"          # [Colors:View] ForegroundNegative = 218,68,83
danger_foreground = "#ffffff"
warning = "#f67400"          # [Colors:View] ForegroundNeutral = 246,116,0
warning_foreground = "#ffffff"
success = "#27ae60"          # [Colors:View] ForegroundPositive = 39,174,96
success_foreground = "#ffffff"
info = "#3daee9"            # [Colors:View] ForegroundActive = 61,174,233
info_foreground = "#ffffff"
selection = "#3daee9"        # [Colors:Selection] BackgroundNormal = 61,174,233
selection_foreground = "#ffffff" # [Colors:Selection] ForegroundNormal = 255,255,255
link = "#2980b9"            # [Colors:View] ForegroundLink = 41,128,185
shadow = "#00000033"        # editorial
focus_ring = "#3daee9"      # [Colors:View] DecorationFocus
tooltip = "#f7f7f7"          # [Colors:Tooltip] BackgroundNormal = 247,247,247
tooltip_foreground = "#232629" # [Colors:Tooltip] ForegroundNormal = 35,38,41
alternate_row = "#f7f7f7"    # [Colors:View] BackgroundAlternate = 247,247,247
disabled = "#707d8a"        # [Colors:View] ForegroundInactive = 112,125,138

[dark]
[dark.fonts]
family = "Noto Sans"
size = 10.0
mono_family = "Hack"
mono_size = 10.0

[dark.geometry]
radius = 5.0
radius_lg = 7.0
shadow = true               # Breeze enables drop shadows (compositor-drawn)
frame_width = 2.0

[dark.spacing]
xxs = 2.0
xs = 4.0
s = 6.0
m = 8.0
l = 12.0
xl = 16.0
xxl = 24.0

[dark.colors]
accent = "#3daee9"          # [Colors:View] DecorationFocus = 61,174,233
background = "#202326"      # [Colors:Window] BackgroundNormal = 32,35,38
foreground = "#fcfcfc"      # [Colors:Window] ForegroundNormal = 252,252,252
surface = "#141618"          # [Colors:View] BackgroundNormal = 20,22,24
border = "#4d4d4d"          # editorial: KDE has no border color; approximated
muted = "#a1a9b1"           # [Colors:View] ForegroundInactive = 161,169,177
primary = "#3daee9"          # = accent
primary_foreground = "#fcfcfc" # [Colors:Selection] ForegroundNormal = 252,252,252
secondary = "#292c30"       # [Colors:Button] BackgroundNormal = 41,44,48
secondary_foreground = "#fcfcfc" # [Colors:Button] ForegroundNormal = 252,252,252
danger = "#da4453"          # [Colors:View] ForegroundNegative = 218,68,83
danger_foreground = "#fcfcfc"
warning = "#f67400"          # [Colors:View] ForegroundNeutral = 246,116,0
warning_foreground = "#fcfcfc"
success = "#27ae60"          # [Colors:View] ForegroundPositive = 39,174,96
success_foreground = "#fcfcfc"
info = "#3daee9"            # [Colors:View] ForegroundActive = 61,174,233
info_foreground = "#fcfcfc"
selection = "#3daee9"        # [Colors:Selection] BackgroundNormal = 61,174,233
selection_foreground = "#fcfcfc" # [Colors:Selection] ForegroundNormal = 252,252,252
link = "#1d99f3"            # [Colors:View] ForegroundLink = 29,153,243
shadow = "#00000066"        # editorial
focus_ring = "#3daee9"      # [Colors:View] DecorationFocus
tooltip = "#292c30"          # [Colors:Tooltip] BackgroundNormal = 41,44,48
tooltip_foreground = "#fcfcfc" # [Colors:Tooltip] ForegroundNormal = 252,252,252
alternate_row = "#1d1f22"    # [Colors:View] BackgroundAlternate = 29,31,34
disabled = "#a1a9b1"        # [Colors:View] ForegroundInactive = 161,169,177
```

### Theme layering

Because all fields are `Option<T>`, themes can be layered: load a base preset, then
overlay with user overrides. A user file might contain only:

```toml
name = "My Custom"

[light.colors]
accent = "#e91e63"
primary = "#e91e63"
```

The consumer loads the base preset, then merges the overlay (replacing `Some` values,
keeping base values for `None` fields). The merge function is straightforward:

```rust
impl ThemeVariant {
    /// Merge an overlay variant into this variant. `Some` values in the overlay
    /// replace the corresponding values in self; `None` values are left unchanged.
    pub fn merge(&mut self, overlay: &ThemeVariant) {
        self.colors.merge(&overlay.colors);
        self.fonts.merge(&overlay.fonts);
        self.geometry.merge(&overlay.geometry);
        self.spacing.merge(&overlay.spacing);
    }
}

impl ThemeColors {
    pub fn merge(&mut self, overlay: &ThemeColors) {
        if overlay.accent.is_some() { self.accent = overlay.accent; }
        if overlay.background.is_some() { self.background = overlay.background; }
        // ... for all 36 fields
    }
}

// ThemeFonts, ThemeGeometry, ThemeSpacing follow the same pattern.
```

This can be generated with a derive macro or written manually for each struct.

---

## 10. Bundled Presets

Presets are TOML files embedded in the binary via `include_str!()` and parsed at first
access. They are loaded via `NativeTheme::preset("kde-breeze")` or
`NativeTheme::list_presets()`.

Each preset is a single TOML file with `[light]` and `[dark]` sections. The preset name
refers to the whole file (not a single variant). `NativeTheme::preset("kde-breeze")` returns
a `NativeTheme` with both `light` and `dark` populated.

**17 bundled presets** (as of v0.2), all including widget_metrics data:

| Preset name | TOML file | Source | Completeness |
|---|---|---|---|
| `default` | `default.toml` | Neutral defaults (gpui-component-like) | All fields + widget_metrics |
| `kde-breeze` | `kde-breeze.toml` | kdeglobals defaults + `breezemetrics.h` | All fields + widget_metrics |
| `adwaita` | `adwaita.toml` | Adwaita CSS variables + SCSS source | All fields + widget_metrics |
| `windows-11` | `windows-11.toml` | UISettings defaults + WinUI3 Fluent specs | All fields + widget_metrics |
| `macos-sonoma` | `macos-sonoma.toml` | NSColor catalog dumps + HIG | All fields + widget_metrics |
| `material` | `material.toml` | Material Design 3 baseline theme (purple) | All fields + widget_metrics |
| `ios` | `ios.toml` | UIColor defaults + HIG measurements | All fields + widget_metrics |
| `catppuccin-latte` | `catppuccin-latte.toml` | Catppuccin palette (light) | Colors + widget_metrics |
| `catppuccin-frappe` | `catppuccin-frappe.toml` | Catppuccin palette | Colors + widget_metrics |
| `catppuccin-macchiato` | `catppuccin-macchiato.toml` | Catppuccin palette | Colors + widget_metrics |
| `catppuccin-mocha` | `catppuccin-mocha.toml` | Catppuccin palette (dark) | Colors + widget_metrics |
| `nord` | `nord.toml` | Nord palette | Colors + widget_metrics |
| `dracula` | `dracula.toml` | Dracula palette | Colors + widget_metrics |
| `gruvbox` | `gruvbox.toml` | Gruvbox palette | Colors + widget_metrics |
| `solarized` | `solarized.toml` | Solarized palette | Colors + widget_metrics |
| `tokyo-night` | `tokyo-night.toml` | Tokyo Night palette | Colors + widget_metrics |
| `one-dark` | `one-dark.toml` | One Dark palette | Colors + widget_metrics |

Community color themes use generic widget_metrics defaults (not platform-specific values).

### Preset loading API

```rust
impl NativeTheme {
    /// Load a bundled preset by name.
    pub fn preset(name: &str) -> Option<NativeTheme> { ... }

    /// List all available bundled preset names.
    pub fn list_presets() -> &'static [&'static str] { ... }

    /// Load from a TOML string.
    pub fn from_toml(toml_str: &str) -> Result<NativeTheme, toml::de::Error> { ... }

    /// Load from a TOML file path.
    pub fn from_file(path: &Path) -> Result<NativeTheme, Error> { ... }

    /// Serialize to a TOML string.
    pub fn to_toml(&self) -> Result<String, Error> { ... }
}
```

---

## 11. Crate Structure

The project uses a Cargo workspace with resolver v3:

```
native-theme/                     (repository root)
  Cargo.toml                      (workspace definition)
  Cargo.lock
  README.md

  native-theme/                   (core crate: native-theme 0.2.0)
    Cargo.toml
    README.md
    src/
      lib.rs                      # Public API, re-exports, from_system(), from_system_async()
      color.rs                    # Rgba struct with custom hex serde
      error.rs                    # Error enum
      presets.rs                  # preset(), list_presets(), from_toml(), from_file(), to_toml()
      model/
        mod.rs                    # NativeTheme, ThemeVariant
        colors.rs                 # ThemeColors (36 flat fields)
        fonts.rs                  # ThemeFonts
        geometry.rs               # ThemeGeometry (7 fields including radius_lg, shadow)
        spacing.rs                # ThemeSpacing
        widget_metrics.rs         # WidgetMetrics + 12 per-widget sub-structs
      kde/                        # Feature: "kde" (sync)
        mod.rs                    # from_kde() -- kdeglobals parsing
        colors.rs                 # KDE color section mapping
        fonts.rs                  # Qt font string parsing
        metrics.rs                # breeze_widget_metrics()
      gnome/                      # Feature: "portal" (async)
        mod.rs                    # from_gnome(), from_kde_with_portal(), adwaita_widget_metrics()
      macos.rs                    # from_macos(), macos_widget_metrics()
      windows.rs                  # from_windows(), WinUI3 spacing, DPI-aware metrics
      presets/                    # 17 TOML preset files
        default.toml
        kde-breeze.toml
        adwaita.toml
        windows-11.toml
        macos-sonoma.toml
        material.toml
        ios.toml
        catppuccin-{latte,frappe,macchiato,mocha}.toml
        nord.toml, dracula.toml, gruvbox.toml
        solarized.toml, tokyo-night.toml, one-dark.toml

  connectors/
    native-theme-iced/            (iced connector crate)
      Cargo.toml
      src/lib.rs                  # iced_core::theme::Catalog + widget metric helpers
    native-theme-gpui/            (gpui connector crate)
      Cargo.toml
      src/lib.rs                  # gpui-component ThemeColor mapping
```

Workspace dependencies (`serde`, `serde_with`, `toml`) are inherited via
`[workspace.dependencies]` in the root `Cargo.toml`.

---

## 12. Feature Flags & Dependencies

```toml
[package]
name = "native-theme"
version = "0.1.0"
edition = "2024"
license = "MIT OR Apache-2.0"
description = "Toolkit-agnostic OS theme data model with presets and runtime readers"
keywords = ["theme", "native", "gui", "colors", "desktop"]
categories = ["gui", "config"]

[dependencies]
serde = { version = "1", features = ["derive"] }
toml = "1.0"

# Linux (ashpd defaults to tokio; for async-std use default-features = false, features = ["async-std"])
ashpd = { version = "0.13", optional = true }
configparser = { version = "3.1", optional = true }
dirs = { version = "6", optional = true }

# Windows
[target.'cfg(target_os = "windows")'.dependencies]
windows = { version = "0.62", optional = true, features = [
    "UI_ViewManagement",              # UISettings, UIColorType
    "Win32_UI_WindowsAndMessaging",   # GetSystemMetrics, SystemParametersInfo
] }

# macOS
[target.'cfg(target_os = "macos")'.dependencies]
objc2-app-kit = { version = "0.3", optional = true, features = ["NSColor", "NSFont", "NSAppearance", "NSColorSpace"] }

# iOS
[target.'cfg(target_os = "ios")'.dependencies]
objc2-ui-kit = { version = "0.3", optional = true, features = ["UIColor", "UIFont", "UITraitCollection"] }

# Android
[target.'cfg(target_os = "android")'.dependencies]
jni = { version = "0.22", optional = true }
ndk = { version = "0.9", optional = true }

# Optional utilities
notify = { version = "8", optional = true }

[features]
default = []

# Sync readers (no async runtime required)
kde = ["dep:configparser", "dep:dirs"]  # kdeglobals parsing only (sync)

# Async readers (requires an async runtime like tokio or async-std)
portal = ["dep:ashpd"]                 # freedesktop portal (accent, scheme, contrast)

# Platform readers
windows = ["dep:windows"]
macos = ["dep:objc2-app-kit"]
ios = ["dep:objc2-ui-kit"]
android = ["dep:jni", "dep:ndk"]

# File watching
watch = ["dep:notify"]
```

**Key design:** The `kde` feature is **synchronous** (only `configparser` + `dirs`). The
`portal` feature pulls in `ashpd` which is **async** (built on zbus). This separation
ensures that apps needing only static kdeglobals reading never pay the cost of an async
runtime. Apps wanting live portal data (accent color, color scheme changes) enable `portal`
and provide their own async runtime.

**Note on Linux deps:** `ashpd`, `configparser`, and `dirs` are listed under `[dependencies]`
(not `[target.'cfg(target_os = "linux")'.dependencies]`) because Cargo feature unification
can cause issues with platform-specific optional deps. The feature flags (`kde`, `portal`)
gate all code paths, so these deps compile but are unused on non-Linux targets. This is a
minor trade-off for simpler feature flag management.

**With no features enabled:** only `serde` + `toml` as dependencies. This is the Phase 1
MVP configuration.

---

## 13. Platform Reader Design

### 13.1 Linux: `from_kde()` (sync, feature `kde`)

**Strategy:** Read kdeglobals for the full palette. Purely synchronous, no D-Bus.

1. Parse `~/.config/kdeglobals` via `configparser::ini::Ini`:
   - Map all `[Colors:*]` sections to `ThemeColors` fields (see mapping table in 8.4)
   - Parse `[General] font` and `[General] fixed` for font family + size (Qt
     `QFont::toString()` format: `"Noto Sans,10,-1,5,50,0,0,0,0,0"` -- comma-separated,
     field 0 = family, field 1 = point size as float)
2. Determine light/dark from the `[General] ColorScheme` key or by heuristic (compare
   background luminance against a threshold).
3. Populate `ThemeGeometry` and `ThemeSpacing` from breezemetrics.h values (hardcoded
   constants -- these never change at runtime).
4. Populate `WidgetMetrics` from `breeze_widget_metrics()` -- compile-time constants
   sourced from `breezemetrics.h`.

When the `portal` feature is also enabled, `from_kde_with_portal()` reads KDE kdeglobals
as a base and overlays portal accent-color (portal is more current when the user changes
accent in System Settings). This extension requires async.

### 13.2 Linux: `from_gnome()` (async, feature `portal`)

**Strategy:** Portal overlay pattern -- read native Adwaita config as base, overlay
portal accent color.

1. Build base theme from hardcoded Adwaita CSS variable defaults (these are **static**
   from an external app's perspective -- Adwaita CSS variables are only accessible from
   inside GTK).
2. Read portal `accent-color` and `color-scheme` via ashpd (async, using re-exported zbus).
3. Override `accent`/`primary_background` from portal if available (GNOME 47+).
4. Populate `ThemeGeometry` and `ThemeSpacing` from Adwaita SCSS defaults (hardcoded).
5. Populate `ThemeFonts` from hardcoded defaults: `"Adwaita Sans"` 11pt (the default
   since GNOME 48; previously `"Cantarell"` through GNOME 47), monospace `"Adwaita Mono"`
   11pt (since GNOME 48; previously `"Source Code Pro"`). Note: GNOME fonts are configurable via `gsettings` / dconf
   (`org.gnome.desktop.interface font-name`), but reading dconf requires either a D-Bus
   call or GIO bindings -- neither is included as a dependency. The hardcoded defaults
   are a reasonable starting point; a future enhancement could add optional dconf reading.
6. Populate `WidgetMetrics` from `adwaita_widget_metrics()` -- compile-time constants
   sourced from libadwaita/GTK4 source code.

Also provides `from_kde_with_portal()` which reads KDE kdeglobals as a base and overlays
portal accent color. The `detect_portal_backend()` function is `pub(crate) async` for
use by the async `from_system_async()` dispatch.

Note: without the `portal` feature, there is no GNOME reader -- use the `adwaita` preset
instead. GNOME has no equivalent to `~/.config/kdeglobals` that can be parsed synchronously
for the full color palette.

**Why not read Adwaita CSS at runtime?** Adwaita CSS variables (`--window-bg-color`, etc.)
are resolved inside GTK's CSS engine. External apps cannot query them. The values are
documented and stable across Adwaita versions, so hardcoding them in the preset/reader
is equivalent. When Adwaita changes, we update the preset data.

### 13.3 `from_system()` and `from_system_async()` -- cross-platform dispatch

Two dispatch functions are provided:

- **`from_system()`** (sync) -- dispatches to platform-specific sync readers. On Linux
  GNOME/GTK desktops, falls back to the bundled Adwaita preset since GNOME has no sync
  reader.
- **`from_system_async()`** (async) -- mirrors `from_system()` but adds portal detection
  for Unknown DE on Linux. When the `portal` feature is enabled, it uses
  `detect_portal_backend()` to determine if GNOME portal is available, enabling live
  accent color reading for GNOME desktops.

```rust
pub fn from_system() -> Result<NativeTheme, Error> {
    #[cfg(target_os = "macos")]
    return from_macos();

    #[cfg(target_os = "windows")]
    return from_windows();

    #[cfg(target_os = "linux")]
    return from_linux();

    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    Err(Error::Unsupported)
}

pub async fn from_system_async() -> Result<NativeTheme, Error> {
    // Same as from_system(), but on Linux with portal feature:
    // detects Unknown DE and tries from_gnome() before falling back.
}
```

### 13.4 Windows: `from_windows()`

1. Read `UISettings.GetColorValue` for: `Accent`, `AccentDark1-3`, `AccentLight1-3`,
   `Background`, `Foreground`.
2. Map accent shades to colors: `AccentDark1` maps to light variant `primary_background`,
   `AccentLight1` maps to dark variant `primary_background`. System `Foreground` maps to
   `primary_foreground`.
3. Read `SystemParametersInfo(SPI_GETNONCLIENTMETRICS)` for font name and size.
4. Read `GetSystemMetricsForDpi` for DPI-aware geometry: scrollbar width, border width,
   etc. The reader uses `read_geometry_dpi_aware()` which returns `(ThemeGeometry, u32)`
   to share the DPI value with font size conversion.
5. Populate spacing from WinUI3 Fluent Design constants (hardcoded as pure constants).
6. Populate `WidgetMetrics` using DPI-aware `GetSystemMetricsForDpi` values for scrollbar,
   button, and menu metrics; WinUI3 Fluent defaults for remaining widgets. Non-Windows
   builds use a fallback with WinUI3 Fluent defaults.
7. Both light and dark variants are always populated (dual-variant pattern).

### 13.5 macOS: `from_macos()`

The macOS module is unconditionally compiled (not behind `cfg(feature)`) -- only the
actual OS FFI calls are gated behind `cfg(target_os = "macos")`. This allows tests to
run cross-platform.

**Thread note:** NSColor component reading is thread-safe, but resolving dynamic/semantic
colors (like `labelColor`) requires the correct `NSAppearance.current` context. The reader
resolves the appearance on the main thread (via `performAsCurrentDrawingAppearance` on
macOS 11+), then extracts color components to `Rgba` values which are plain data.

1. Read `NSAppearance.currentAppearance` to determine light/dark.
2. Read NSColor semantic colors:
   - `controlAccentColor` -> accent, primary_background
   - `windowBackgroundColor` -> background
   - `labelColor` -> foreground
   - `separatorColor` -> border, separator
   - `secondaryLabelColor` -> muted
   - `shadowColor` -> shadow
   - `systemRedColor` -> danger
   - `systemOrangeColor` -> warning
   - `systemGreenColor` -> success
   - `systemBlueColor` -> info
   - `linkColor` -> link
   - `selectedContentBackgroundColor` -> selection
   - `selectedTextColor` -> selection_foreground
   - `keyboardFocusIndicatorColor` -> focus_ring
   - `controlColor` -> button, secondary_background
   - `controlTextColor` -> button_foreground
   - `underPageBackgroundColor` -> sidebar
   - `textBackgroundColor` -> input
   - `textColor` -> input_foreground
   - `disabledControlTextColor` -> disabled
   - `alternatingContentBackgroundColors[1]` -> alternate_row
   - `controlBackgroundColor` -> surface
3. Read `NSFont.systemFont` for family and size.
4. Read `NSFont.monospacedSystemFont` for mono family and size.
5. Geometry: hardcoded from AppKit (~5px radius). macOS exposes no geometry APIs.
6. Widget metrics: `macos_widget_metrics()` provides compile-time constants sourced from
   Apple HIG measurements. Both light and dark variants are always populated.

**Color space:** macOS may return P3 colors. Convert to sRGB via
`color.colorUsingColorSpace(&NSColorSpace::sRGBColorSpace())` (objc2 Rust) before
extracting components.

### 13.6 iOS: `from_ios()` (deferred to Phase 7)

Similar to macOS but using `UIColor` equivalents and `UIFont.preferredFont(forTextStyle:)`
for Dynamic Type font sizes.

### 13.7 Android: `from_android()` (deferred to Phase 7)

JNI calls into `Resources.getColor(android.R.color.system_accent1_500)` etc. for Material
You colors. `Configuration.uiMode` for dark/light. `Configuration.fontScale` for font
scaling. Requires Android JNI context. No ergonomic Rust wrapper exists.

---

## 14. Error Handling

Following `system-theme`'s proven error taxonomy:

```rust
#[derive(Debug)]
pub enum Error {
    /// This operation is not supported on the current platform.
    /// E.g., calling from_kde() on macOS.
    Unsupported,

    /// The data exists but cannot be read right now.
    /// E.g., D-Bus is not running, kdeglobals file not found.
    Unavailable(String),

    /// TOML parsing or file I/O error.
    Format(String),

    /// Wrapped platform-specific error.
    Platform(Box<dyn std::error::Error + Send + Sync>),
}

impl std::fmt::Display for Error { ... }
impl std::error::Error for Error { ... }

// Convenience conversions
impl From<toml::de::Error> for Error { ... }
impl From<toml::ser::Error> for Error { ... }
impl From<std::io::Error> for Error { ... }
```

---

## 15. Change Notification

Change notification is a post-MVP feature (Phase 3+), behind a feature flag.

### Linux (ashpd)

Subscribe to portal `SettingChanged` D-Bus signal:

```rust
let mut stream = settings.receive_setting_changed().await?;
while let Some(change) = stream.next().await {
    // Re-read affected values, rebuild NativeTheme
}
```

Additionally, use `notify` crate to watch `~/.config/kdeglobals` for file changes (KDE
updates this file when the user changes theme in System Settings).

### macOS

Register three Objective-C notification observers (as `system-theme` does):
1. KVO on `NSApplication.effectiveAppearance` -> dark/light changes
2. `NSSystemColorsDidChangeNotification` -> accent/color changes
3. `NSWorkspaceAccessibilityDisplayOptionsDidChangeNotification` -> contrast changes

Unregister in `Drop`.

### Windows

Use `UISettings.ColorValuesChanged` event.

### Design: No mandatory async runtime

The core crate does NOT depend on tokio or any async runtime. The feature split makes this
explicit:

- `kde` feature: **sync** (configparser + dirs). No async runtime needed.
- `portal` feature: **async** (ashpd/zbus). Requires an async runtime.
- `watch` feature: **sync** (notify crate, OS-level file watching).
- `windows`, `macos` features: **sync** (platform APIs are synchronous).

Sync readers (`from_kde()`, `from_windows()`, `from_macos()`) work without any async
runtime. Only portal reading (`from_gnome()`, portal overlay in `from_kde_with_portal()`)
and portal change streaming require async.

---

## 16. Connector Crates (v0.2)

In v0.2, the adapter pattern evolved from code snippets into first-party **connector
crates** that live in the workspace. Each connector translates `NativeTheme` data into
toolkit-native types, including widget metric helpers.

### `native-theme-iced` (connector for iced)

Crate: `connectors/native-theme-iced/`

Provides `iced_core::theme::Catalog` integration -- the standard iced theming mechanism.
Widget metric helpers are free functions (not Catalog impls) per iced architecture.
Uses `iced_core 0.14` (not full `iced`) to avoid winit windowing dependency in the library.

### `native-theme-gpui` (connector for gpui)

Crate: `connectors/native-theme-gpui/`

Provides `gpui-component::ThemeColor` mapping from `NativeTheme`. Uses the Colorize trait
for lighten/darken (multiplicative lightness). Matches `apply_config` fallback logic for
hover/active state derivation. Maps all 108 `ThemeColor` fields via grouped helper functions.

### Writing a new connector

For other toolkits (egui, slint, dioxus), the adapter pattern still applies. A connector
typically:

1. Takes a `&ThemeVariant` and maps its 36 color roles to toolkit-native color types.
2. Maps `ThemeGeometry` values (radius, shadow, etc.) to toolkit-native spacing/styling.
3. Optionally maps `WidgetMetrics` to toolkit-native per-widget sizing.
4. Derives toolkit-specific tokens from the closest semantic role (e.g.,
   `hyperlink_color = link.unwrap_or(accent)`).

---

## 17. Design Decisions & Rationale

### 17.1 Option<T> for all fields

Every field in `ThemeColors`, `ThemeFonts`, `ThemeGeometry`, `ThemeSpacing` is `Option<T>`.

**Why:**
- Different platforms expose different subsets (the coverage matrix makes this clear).
- Preset files should only specify values they have authoritative data for.
- The consuming toolkit has its own defaults for unspecified values.
- This enables natural theme layering (base preset + user overrides).
- Avoids forcing preset authors to fabricate values they don't have.

### 17.2 Rgba with u8 components

**Why u8 over f32:**
- TOML hex strings map naturally to u8.
- CSS/web colors are u8-based.
- KDE kdeglobals uses integer R,G,B.
- Windows UISettings returns Color with u8 components.
- macOS NSColor can be read as u8 via `getRed:green:blue:alpha:` (no visible precision
  loss; 256 levels exceed human perception for color discrimination).
- Freedesktop portal returns `(f64, f64, f64)` in [0.0, 1.0] -> `(v * 255.0).round() as u8`.

### 17.3 Included alpha channel from day one

`system-theme`'s RGB-only `ThemeColor` is a known limitation. Several platform tokens have
meaningful transparency:
- macOS `NSColor.shadowColor` (has alpha)
- GNOME `--border-opacity` and `--disabled-opacity`
- Material 3 disabled states (38% opacity)
- Various overlay, scrim, and sheet colors

Adding alpha later is a **breaking change** (struct layout, serde format). Including it
from the start costs nothing and prevents a painful retrofit.

### 17.4 36 semantic roles (not 6, not 100)

**Why not 6 (like system-theme)?** Mapping only `background`, `foreground`, `accent`,
`danger`, `warning`, `success` leaves ~20 theme fields at defaults. The result is "tinted
but still generic" -- not enough to feel native.

**Why not 100+ (all platform tokens)?** Many tokens are too platform-specific or
toolkit-specific to standardize. macOS has `findHighlightColor` and `gridColor`; KDE has
`ComplementaryBackground`; iOS has `systemGray2` through `systemGray6`. Including all of
these would make the model unwieldy and asymmetric.

**The sweet spot (36)** covers the roles that every GUI toolkit can map to something
meaningful: backgrounds, foregrounds, accent, primary/secondary actions, status colors,
interactive states, and surface variants.

### 17.5 Named spacing scale (not raw values, not multiplier)

**Why not raw pixel values?** Too platform-specific. KDE's `Layout_DefaultSpacing = 6`
is not meaningful in isolation.

**Why not a single multiplier?** Too coarse. A 1.5x multiplier affects all spacing
equally, but platforms differ in how they scale different spacing levels.

**Why a named scale (xxs through xxl)?** Provides a consistent vocabulary that adapters
can map to their toolkit's spacing system (egui's `Spacing`, iced's widget parameters,
etc.). The 7 levels (2, 4, 6, 8, 12, 16, 24px) cover the common spacing values across
all platform design systems.

**Important caveat:** These are editorial values, not runtime-readable OS data. Documented
clearly.

### 17.6 Separate light/dark variants at the top level

**Why?** Most platforms define separate color sets for light and dark mode. A TOML file
for "KDE Breeze" naturally has both variants. The consuming app picks the right variant
based on OS dark/light preference.

**Runtime readers** populate only the active variant (the one matching the current OS mode).
The other variant is `None`. If the app needs both, it loads the corresponding preset.

**Color-only community presets** (e.g., Catppuccin Mocha) may define only `dark`.

### 17.7 No accessibility flags in the data model

Mode detection (dark/light, high contrast, reduced motion) is handled **separately** from
theme data. These are environment signals that inform which variant to use or how to modify
the theme, but they are not visual tokens.

For runtime readers, the returned colors already incorporate the user's current accessibility
state (e.g., macOS semantic colors automatically adapt to high contrast mode). No separate
adjustment is needed.

For presets, there is no accessibility adjustment -- the preset represents a specific visual
configuration.

### 17.8 sRGB color space

All `Rgba` values are in sRGB. macOS may internally use P3 display color space; the reader
converts via `NSColor.colorUsingColorSpace:` (Objective-C) / `colorUsingColorSpace(_:)`
(Swift) with the sRGB color space before extracting components. In Rust with objc2, this
would be `color.colorUsingColorSpace(&NSColorSpace::sRGBColorSpace())`. This ensures
consistent behavior across platforms.

### 17.9 Logical pixels for sizes

All font sizes, geometry values, and spacing values are in **logical pixels** (like CSS px).
The consuming toolkit handles DPI scaling. This matches how all platforms define their
design tokens:
- KDE: breezemetrics.h values are logical px
- Adwaita: CSS px (logical)
- Material: dp (density-independent pixels, equivalent to logical px)
- iOS HIG: pt (points, equivalent to logical px)
- Windows: DIP (device-independent pixels, equivalent to logical px)

### 17.10 `impl_merge!` macro for merge

Theme layering uses an `impl_merge!` declarative macro that generates `merge()` and
`is_empty()` methods for all model structs. The macro handles two field types: `option`
fields (Option values replaced if Some) and `nested` fields (sub-structs merged
recursively). `ThemeVariant` uses a manual `merge()` implementation to handle the
`Option<WidgetMetrics>` recursive merge case.

---

## 18. Implementation Phases

### v0.1 Phases (1-8)

| Phase | Scope | Delivers |
|---|---|---|
| **1** | Data model (colors, fonts, geometry, spacing) + TOML serde + Rgba custom serialization + presets (Default, Breeze light/dark, Adwaita light/dark) + preset loading API | Crate MVP, usable in any Rust GUI app |
| **2** | Linux runtime readers: `from_kde()` (sync, `kde` feature), `from_gnome()` (async, `portal` feature), `from_system()` | Live OS theme sync on Linux |
| **3** | Publish to crates.io, documentation, README with examples for egui/iced/slint | Ecosystem contribution |
| **4** | Windows + macOS readers + remaining presets (Windows 11, macOS Sonoma, Material, iOS) + community presets (Catppuccin, Nord, Dracula, Gruvbox, Solarized, Tokyo Night, One Dark) | Cross-platform desktop + community |
| **5-8** | Additional presets, refinements, testing | Pre-v0.2 completion |

### v0.2 Phases (9-15) -- Completed

| Phase | Scope | Delivers |
|---|---|---|
| **9** | Cargo workspace restructuring: virtual workspace with resolver v3, workspace dep inheritance, connector stubs | Clean multi-crate layout |
| **10** | API breaking changes: flat ThemeColors (36 direct fields, removed nested sub-structs), NativeTheme associated methods (preset, from_toml, from_file, list_presets, to_toml), ThemeGeometry extended with radius_lg and shadow | v0.2 public API |
| **11** | Platform readers: macOS reader (from_macos()), Windows accent shades + DPI-aware geometry, Linux portal overlay pattern, from_system_async() | Cross-platform runtime readers |
| **12** | Widget metrics: WidgetMetrics with 12 per-widget sub-structs, platform-specific metric functions (breeze_widget_metrics, macos_widget_metrics, adwaita_widget_metrics, Windows DPI-aware), all 17 presets updated | Fine-grained per-widget sizing |
| **13** | CI pipeline: GitHub Actions matrix (7 entries), fmt/clippy/test gates, semver-checks baseline | Automated quality assurance |
| **14** | Toolkit connectors: native-theme-iced (iced_core Catalog integration), native-theme-gpui (gpui-component ThemeColor mapping), demo apps | First-party toolkit adapters |
| **15** | Publishing prep: documentation, IMPLEMENTATION.md update, new-OS-version guide | Publication readiness |

---

## 19. Design Questions (All Resolved)

1. **Async KDE reader:** RESOLVED -- `from_kde()` is sync (kdeglobals only, `kde` feature).
   Portal reading is behind a separate `portal` feature (async). This cleanly separates
   concerns and avoids forcing an async runtime on users who only need static palette data.

2. **Preset file format:** RESOLVED -- one TOML file with `[light]` and `[dark]` sections.
   Simpler (fewer files) and matches conceptual organization.

3. **`from_system()` behavior:** RESOLVED -- returns the first successful reader based on
   platform/DE detection. Falls back to bundled preset if no reader succeeds. Never panics.

4. **KDE font parsing format:** RESOLVED -- The `font=` value in kdeglobals follows Qt's
   `QFont::toString()` format. Qt 4 uses 10 fields:
   `"Family,PointSizeF,PixelSize,StyleHint,Weight,Style,Underline,StrikeOut,FixedPitch,
   RawMode"`. Qt 5/6 uses 16 fields (appends Capitalization, LetterSpacing, WordSpacing,
   Stretch, StyleStrategy, FontStyle; field 10 changed from RawMode to "Always 0").
   Field 0 is family name, field 1 is point size (float). Field 5 is `Style`
   (int: 0=Normal, 1=Italic, 2=Oblique). KDE Plasma 6 uses the 16-field Qt 6 format.
   Robust parsing: split by comma, take field 0 (family) and field 1 (point size),
   ignore remaining fields.

5. **ashpd accent-color deserialization type:** RESOLVED -- ashpd provides a convenience
   method `settings.accent_color().await?` that returns `ashpd::desktop::Color` (f64 RGB
   in [0.0, 1.0]). The generic `settings.read::<(f64, f64, f64)>(APPEARANCE_NAMESPACE,
   ACCENT_COLOR_SCHEME_KEY)` also works. Prefer the convenience method.

6. **Rust edition:** RESOLVED -- Use `edition = "2024"`. Edition 2024 was stabilized in
   Rust 1.85 (February 2025), over a year ago. All supported platforms have access to it.
   As a new crate with no existing users, there is no compatibility concern.

7. **Theme merging API:** RESOLVED -- `merge()` is provided on `ThemeVariant`, `ThemeColors`,
   `ThemeFonts`, `ThemeGeometry`, and `ThemeSpacing`. It's a natural operation on the data
   model and doesn't require toolkit knowledge. See Section 9 (Theme layering).

8. **`disabled` field semantics:** RESOLVED -- Reader computes the blended color internally.
   On platforms using opacity (GNOME: `--disabled-opacity` = 0.5; Material 3: 0.38), the
   reader blends the foreground and background colors: `disabled = fg * opacity + bg *
   (1 - opacity)`. Platforms with a dedicated disabled color (KDE, macOS, iOS) use it
   directly. No helper function is exposed by the crate.

---

## 20. Sources & References

### Platform Documentation

- [KDE/breeze - breezemetrics.h](https://github.com/KDE/breeze/blob/master/kstyle/breezemetrics.h)
- [libadwaita CSS Variables](https://gnome.pages.gitlab.gnome.org/libadwaita/doc/main/css-variables.html) (also: [v1.2 reference](https://gnome.pages.gitlab.gnome.org/libadwaita/doc/1.2/css-variables.html))
- [XDG Desktop Portal Settings](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Settings.html)
- [Accent color portal PR](https://github.com/flatpak/xdg-desktop-portal/pull/815)
- [GetSystemMetrics (Microsoft)](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsystemmetrics)
- [NSColor (Apple)](https://developer.apple.com/documentation/appkit/nscolor)
- [UIColor (Apple)](https://developer.apple.com/documentation/uikit/uicolor)
- [NSColor catalog dump (macOS Sonoma)](https://gist.github.com/martinhoeller/38509f37d42814526a9aecbb24928f46)
- [Material Design 3 Color System](https://m3.material.io/styles/color/system/overview)
- [Material Design 3 Color Roles](https://m3.material.io/styles/color/roles)
- [Material Design 3 Shape System](https://m3.material.io/styles/shape/overview)
- [Material You Dynamic Color](https://developer.android.com/develop/ui/views/theming/dynamic-colors)
- [Apple Human Interface Guidelines](https://developer.apple.com/design/human-interface-guidelines/)

### Rust Crates

- [ashpd](https://crates.io/crates/ashpd) 0.13 -- XDG Desktop Portal ([docs](https://docs.rs/ashpd), [source](https://github.com/bilelmoussaoui/ashpd))
- [configparser](https://crates.io/crates/configparser) 3.1 -- INI parser ([lib.rs](https://lib.rs/crates/configparser))
- [windows](https://crates.io/crates/windows) 0.62 -- Windows API
- [objc2-app-kit](https://crates.io/crates/objc2-app-kit) 0.3 -- macOS AppKit bindings ([source](https://github.com/madsmtm/objc2))
- [objc2-ui-kit](https://crates.io/crates/objc2-ui-kit) 0.3 -- iOS UIKit bindings
- [jni](https://crates.io/crates/jni) 0.22 -- Android JNI bridge
- [ndk](https://crates.io/crates/ndk) 0.9 -- Android NDK bindings
- [serde](https://crates.io/crates/serde) 1.x -- Serialization
- [toml](https://crates.io/crates/toml) 1.0 -- TOML parser
- [notify](https://crates.io/crates/notify) 8.2 -- File system watcher
- [dirs](https://crates.io/crates/dirs) 6.x -- Platform config directories

### Prior Art

- [COSMIC cosmic-theme](https://github.com/pop-os/libcosmic/tree/master/cosmic-theme) -- comprehensive but iced-coupled
- [system-theme 0.3.0](https://crates.io/crates/system-theme) -- closest prior art, iced-coupled
- [dark-light 2.0](https://crates.io/crates/dark-light) -- dark/light detection only
- [catppuccin/palette](https://github.com/catppuccin/palette) -- color-only presets
- Electron [nativeTheme](https://www.electronjs.org/docs/latest/api/native-theme) -- closest conceptual match

---

## Appendix A: Widget Metrics Platform Reference

The full widget metrics tables from the platform coverage matrix. These were used as
source data when implementing `WidgetMetrics` in v0.2 (Phase 12). They cover:

- **Window / Title Bar:** title bar height, button width/height, small caption, sizing
  frame, fixed frame, padded border (KDE: `TitleBar_MarginWidth` = 4; Windows: ~8 dynamic
  `GetSystemMetrics` calls)
- **Button:** padding, min width/height, icon spacing, radius (KDE: 5 constants; GNOME:
  from SCSS)
- **Tool Button:** padding, item spacing, inline indicator width (KDE: 3 constants)
- **Checkbox / Radio:** indicator size, radius, focus margin, label spacing (KDE: 4
  constants; GNOME: from SCSS; Windows: `SM_CXMENUCHECK`)
- **Text Input / Line Edit:** padding, caret width (KDE: 1 constant; Windows: `SPI_GETCARETWIDTH`)
- **Combobox / Select:** padding, item margin (KDE: 2 constants)
- **Spin Box:** padding, arrow button width (KDE: 2 constants)
- **Slider:** groove thickness, control thickness, tick length/margin (KDE: 4 constants)
- **Progress Bar:** track thickness, busy indicator size, item spacing (KDE: 3 constants)
- **Scrollbar:** width, thumb width, min thumb height, no-button/single-button heights
  (KDE: 6 constants; Windows: ~7 dynamic `GetSystemMetrics` calls)
- **Menu Bar:** height, item padding, button dimensions (KDE: 2 constants; Windows: 3
  dynamic metrics)
- **Menu Item / Context Menu:** frame width, padding, spacing, text margins, highlight
  gap, accelerator space, indicator width (KDE: 9 constants)
- **Tab Bar / Tabs:** padding, min dimensions, item spacing, overlap, active effect,
  widget margin (KDE: 9 constants)
- **Tooltip:** padding (KDE: 1 constant)
- **Toolbar:** frame width, handle extent/width, separator width, extension width, item
  margin/spacing, separator margin (KDE: 8 constants)
- **List / Tree / Table:** item margins (4-sided), item padding, first item margin, arrow
  size, side panel margin (KDE: 9 constants)
- **Header (Column Headers):** margin, item spacing, arrow size (KDE: 3 constants)
- **Group Box / Section:** title margin (KDE: 1 constant)
- **Tool Box:** tab min width, item spacing, margin (KDE: 3 constants)
- **Splitter:** handle width (KDE: 1 constant)
- **Dialog:** content margin (KDE: uses `Layout_TopLevelMarginWidth`)
- **Icons and Cursors:** icon sizes large/small, cursor size, arrow sizes (Windows: 3
  dynamic; KDE: 2 constants)
- **Misc Rendering:** shadow overlap, blend value, pen widths for symbol/frame/shadow
  (KDE: 5 constants)

Total: ~80 KDE static constants, ~30 Adwaita SCSS values, ~20 Windows dynamic metrics,
~0 macOS/iOS/Android (these platforms achieve consistency through HIG/Material enforcement
rather than exposable metrics).

---

## Appendix B: Mobile Platform Mapping

The data model covers mobile without mobile-specific extensions:

| `ThemeColors` field | Android Material You | iOS UIColor |
|---|---|---|
| `accent` | `system_accent1_500` | `tintColor` |
| `background` | `?attr/colorSurface` | `systemBackground` |
| `foreground` | `?attr/colorOnSurface` | `label` |
| `primary_background` | `?attr/colorPrimary` | `tintColor` |
| `primary_foreground` | `?attr/colorOnPrimary` | derived |
| `danger` | `?attr/colorError` | `systemRed` |
| `success` | n/s (not in Material 3) | `systemGreen` |
| `warning` | n/s (not in Material 3) | `systemOrange` |
| `surface` | `?attr/colorSurface` | `secondarySystemBackground` |
| `sidebar` | `?attr/colorSurfaceVariant` | `secondarySystemBackground` |
| `button` | `?attr/colorPrimaryContainer` | `systemFill` |
| `separator` | `?attr/colorOutlineVariant` | `separator` |

| `ThemeGeometry` field | Android Material 3 | iOS HIG |
|---|---|---|
| `radius` | 12dp (medium shape) | ~10pt |
| `radius_lg` | 16dp (large shape) / 28dp (XL) | n/s |
| `disabled_opacity` | 0.38 | n/s |

| `ThemeSpacing` field | Android Material 3 | iOS HIG |
|---|---|---|
| `xxs` | 2dp | ~2pt |
| `xs` | 4dp | ~4pt |
| `s` | 6dp | n/s |
| `m` | 8dp | ~8pt |
| `l` | 12dp | ~12pt |
| `xl` | 16dp | ~16pt |
| `xxl` | 24dp | ~24pt |

Material baseline uses a purple accent. On API 31+ devices, `from_android()` reads the
user's actual dynamic colors. The static preset is a fallback for older devices or
non-Android targets that want Material aesthetics.
