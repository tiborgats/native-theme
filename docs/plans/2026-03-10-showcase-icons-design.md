# Showcase Icon Sets & Inspector Improvements

## Problem

The gpui showcase example has three issues:

1. The Theme Config Inspector section uses different styling (font size, spacing) than the Widget Info section — they should match.
2. There is no way to follow a theme's default icon set. When switching themes, the icon set stays fixed instead of following the theme's preference.
3. Two slightly different Lucide icon versions get mixed in the icon gallery — gpui-component's built-in Lucide and native-theme's bundled Lucide. They must be cleanly separated.

Additionally, native-theme's bundled icon sets (Lucide and Material) only cover the 42 IconRole variants. The gpui-component gallery shows 86 IconName variants. Both bundled sets must be expanded to cover all 86.

## Design

### 1. Theme Config Inspector Styling

Change `render_sidebar()` and `config_row()` to match `render_widget_info_panel()`:

- Outer gap: `gap_3` → `gap_0p5`
- Text size: `text_sm()` → `text_xs()` for both label and value in config rows
- Label keeps `font_semibold()`

### 2. "default (...)" Icon Set

Add a dynamic entry at the top of the icon set selector: `"default (freedesktop)"` (label updates based on current theme).

**State:** New field `use_default_icon_set: bool` on `Showcase`. Starts `true`.

**Resolving the default:** Read `variant.icon_theme` from the loaded theme preset. If `None`, use `system_icon_set().name()` (platform default). Helper method `resolve_default_icon_set(&self) -> String`.

**On theme change:** If `use_default_icon_set` is true, recompute the resolved icon set name, rebuild the selector items (to update the label), and reload all icons.

**On explicit icon set selection:** If user picks "default (...)", set `use_default_icon_set = true`. If user picks any other set, set `use_default_icon_set = false`.

### 3. Two Lucide Icon Sets

Replace the single "lucide" entry with two:

- **"gpui-component built-in (Lucide)"** — All icons rendered via `Icon::new(IconName::Foo)`. Zero native-theme loading. Both the native icons section (42 roles) and gpui-component icons section (86 names) use gpui-component's own SVGs. For the native section, `native_theme_gpui::icons::icon_name(role)` maps roles to IconName; unmapped roles show a gray placeholder.

- **"Lucide (bundled)"** — All icons rendered from native-theme's bundled Lucide SVGs (`native-theme/icons/lucide/`). For IconRole-based icons: `load_icon(role, "lucide")`. For name-based icons: `bundled_icon_by_name(Lucide, name)`. No gpui-component built-in icons used at all.

Internal sentinel: `icon_set_name = "gpui-builtin"` triggers the gpui-component rendering path.

### 4. Expand Bundled Icon Sets to 86

**native-theme core changes:**

Add to `bundled.rs`:
```rust
pub fn bundled_icon_by_name(set: IconSet, name: &str) -> Option<&'static [u8]>
```
Feature-gated on `material-icons` / `lucide-icons`. Dispatches to `material_svg_by_name(name)` and `lucide_svg_by_name(name)` — match tables mapping canonical icon names to `include_bytes!()`.

**SVG assets:**

Expand `native-theme/icons/lucide/` from 38 to 86+ SVG files. All from the latest Lucide release. Existing files updated to latest version too.

Expand `native-theme/icons/material/` from 38 to 86+ SVG files. All from latest Material Symbols.

For the 11 gpui-component icons that are custom (not standard Lucide names), map to closest Lucide/Material equivalents:

| gpui-component | Lucide equivalent | Material equivalent |
|---|---|---|
| Close | x | close |
| Dash | minus | remove |
| Delete | trash-2 | delete |
| WindowClose | x | close |
| WindowMinimize | minus | minimize |
| WindowMaximize | maximize | open_in_full |
| WindowRestore | minimize-2 | close_fullscreen |
| Inspector | inspect (or scan) | developer_mode |
| ResizeCorner | grip | drag_indicator |
| SortAscending | arrow-up-narrow-wide | sort (or arrow_upward) |
| SortDescending | arrow-down-wide-narrow | sort (or arrow_downward) |

**Connector changes (native-theme-gpui):**

Add mapping functions:
```rust
pub fn lucide_name_for_gpui_icon(gpui_name: &str) -> Option<&'static str>
pub fn material_name_for_gpui_icon(gpui_name: &str) -> Option<&'static str>
```

These map all 86 gpui-component icon name strings to their canonical Lucide/Material names.

**Size budgets:** Update test thresholds to 200KB (Lucide) and 400KB (Material).

### 5. Icon Set Selector Order

1. `default (freedesktop)` — dynamic label
2. `gpui-component built-in (Lucide)`
3. `Lucide (bundled)`
4. `material`
5. Platform-specific: `freedesktop` / `sf-symbols` / `segoe-fluent`

### Architecture Summary

```
Icon Set Selected        Native Icons (42 roles)              gpui-component Icons (86 names)
─────────────────        ──────────────────────               ───────────────────────────────
default (...)            Delegates to resolved set             Delegates to resolved set
gpui-builtin             icon_name(role) → Icon::new()        Icon::new(IconName)
Lucide (bundled)         load_icon(role, "lucide")            bundled_icon_by_name(Lucide, name)
material                 load_icon(role, "material")          bundled_icon_by_name(Material, name)
freedesktop              load_icon(role, "freedesktop")       bundled_icon_by_name(fallback set, name)
```

For system sets (freedesktop/sf-symbols/segoe-fluent), the gpui-component section falls back to whichever bundled set makes sense (or shows system icons where available + bundled fallback).
