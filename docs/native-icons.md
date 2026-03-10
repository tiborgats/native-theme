# Native Icons

Each OS platform has its own icon set for dialogs, window controls, and common
UI actions. To achieve native look and feel, native-theme provides platform
icon loading as a core feature — on the same level as colors, fonts, and
geometry.

This document is the implementation specification for the complete icon
handling system.

## Architecture

### Data flow

```
┌─────────────────────────────────────────────────────┐
│  native-theme crate                                 │
│                                                     │
│  ThemeVariant                                       │
│  ├── colors, fonts, geometry, spacing, ...          │
│  └── icon_set: Option<String>    ◄── "sf-symbols"   │
│                                                     │
│  load_icon("sf-symbols", DialogWarning, 24)         │
│      │                                              │
│      ├─ macOS:  NSImage(systemSymbolName:) → RGBA   │
│      ├─ Windows: SHGetStockIconInfo → RGBA          │
│      ├─ Linux:  /usr/share/icons/…/…svg → SVG      │
│      └─ fallback: bundled SVGs → SVG                │
│      │                                              │
│      ▼                                              │
│  IconData::Svg(bytes) or IconData::Rgba { w,h,data }│
└──────────────────┬──────────────────────────────────┘
                   │
                   ▼
┌──────────────────────────────────────────────────────┐
│  connector (e.g. native-theme-gpui)                  │
│                                                      │
│  IconData → gpui::RenderImage or svg rendering       │
│  (trivial format conversion — no platform logic)     │
└──────────────────────────────────────────────────────┘
```

### Types

```rust
/// Semantic icon role — a platform-agnostic icon concept.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IconRole {
    // Dialog / Alert
    DialogWarning,
    DialogError,
    DialogInfo,
    DialogQuestion,
    DialogSuccess,
    Shield,

    // Window controls
    WindowClose,
    WindowMinimize,
    WindowMaximize,
    WindowRestore,

    // Common actions
    ActionSave,
    ActionDelete,
    ActionCopy,
    ActionPaste,
    ActionCut,
    ActionUndo,
    ActionRedo,
    ActionSearch,
    ActionSettings,
    ActionEdit,
    ActionAdd,
    ActionRemove,
    ActionRefresh,
    ActionPrint,

    // Navigation
    NavBack,
    NavForward,
    NavUp,
    NavDown,
    NavHome,
    NavMenu,

    // Files / Places
    FileGeneric,
    FolderClosed,
    FolderOpen,
    TrashEmpty,
    TrashFull,

    // Status
    StatusLoading,
    StatusCheck,
    StatusError,

    // System
    UserAccount,
    Notification,
    Help,
    Lock,
}

/// Icon data returned by the loading function.
pub enum IconData {
    /// SVG content (from freedesktop themes, bundled icon sets).
    Svg(Vec<u8>),
    /// Rasterized RGBA pixels (from macOS/Windows system APIs).
    Rgba {
        width: u32,
        height: u32,
        data: Vec<u8>,
    },
}
```

### ThemeVariant integration

```rust
pub struct ThemeVariant {
    pub colors: Colors,
    pub fonts: Fonts,
    pub geometry: Geometry,
    pub spacing: Spacing,
    pub widget_metrics: WidgetMetrics,
    pub icon_set: Option<String>,    // NEW
}
```

The `icon_set` field is set in each preset TOML (serialized as `icon_set`,
with `icon_theme` accepted as a backward-compatible serde alias). When
`None`, the crate resolves it at runtime to the host OS native icon set
(see Default Assignments below).

### Public API

```rust
/// Load an icon for the given role.
///
/// Returns the icon as SVG bytes or rasterized RGBA pixels depending on
/// the platform and icon set.  Returns `None` if the icon set does not
/// cover the requested role.
///
/// `icon_set` values: "sf-symbols", "segoe-fluent", "freedesktop",
/// "material", "lucide", "system"
///
/// `size` is in pixels. On HiDPI displays, callers should multiply the
/// desired point size by the scale factor (e.g. 24pt × 2 = 48px).
pub fn load_icon(role: IconRole, icon_set: &str) -> Option<IconData>

/// Look up the identifier string for a given icon set and role.
///
/// Useful when the connector can handle the icon lookup itself
/// (e.g. gpui-component already has Lucide icons loaded).
pub fn icon_name(icon_set: &str, role: IconRole) -> Option<&'static str>

/// Resolve "system" to the actual icon set for the current OS.
///
/// - macOS / iOS  → IconSet::SfSymbols
/// - Windows      → IconSet::SegoeFluent
/// - Linux        → IconSet::Freedesktop
/// - other        → IconSet::Material
pub fn system_icon_set() -> IconSet

/// Detect the actual installed icon theme name on the current system.
///
/// Returns a theme name like "breeze-dark", "Adwaita", "Papirus", etc.
/// On non-Linux platforms, returns the canonical icon set name.
/// Useful when you need the real theme name (e.g. for freedesktop
/// icon directory lookup) rather than the abstract icon set category.
pub fn system_icon_theme() -> String
```

### Platform loading

| Platform | `load_icon` implementation |
|----------|--------------------------|
| macOS | `NSImage(systemSymbolName:)` → rasterize at requested size → `IconData::Rgba` |
| iOS | `UIImage(systemName:)` → rasterize → `IconData::Rgba` |
| Windows | `SHGetStockIconInfo(SIID_*)` for stock icons; load Segoe Fluent Icons font and render glyph for UI icons → `IconData::Rgba` |
| Linux | Look up SVG in the active icon theme following the [Icon Theme Specification](https://specifications.freedesktop.org/icon-theme/latest/); subdirectory layout is defined by each theme's `index.theme` (conventionally `{size}x{size}/{context}/{name}.svg`). Search order: `$HOME/.icons`, `$XDG_DATA_DIRS/icons`, `/usr/share/pixmaps`. Fall back to `hicolor` → `IconData::Svg` |
| Fallback | Return bundled SVG from the Material or Lucide set (if enabled) → `IconData::Svg` |

Platform-specific code lives behind `#[cfg(target_os = "...")]` in the
native-theme crate. Connectors never contain platform logic.

---

## Icon Sets

### Native icon sets

These are the platform-provided icon systems. They cannot be redistributed
but can be loaded at runtime through OS APIs.

| Icon Set | Platforms | License | Bundleable? | Access method |
|----------|----------|---------|-------------|---------------|
| SF Symbols | macOS, iOS | Apple proprietary | No | `NSImage` / `UIImage` API |
| Segoe Fluent Icons | Windows | Microsoft proprietary | No | `SHGetStockIconInfo` + system font |
| freedesktop (Adwaita) | Linux (GNOME) | LGPL-3.0-or-later / CC-BY-SA-3.0-US | Yes | SVG file lookup |
| freedesktop (Breeze) | Linux (KDE) | LGPL-3.0 | Yes | SVG file lookup |

### Bundled icon sets

These ship with the native-theme crate (controlled by feature flags) and
serve as cross-platform fallbacks.

| Icon Set | License | Icons | Notes |
|----------|---------|-------|-------|
| Material Symbols | Apache 2.0 | 3,800+ | Recommended cross-platform default. Three styles (Outlined, Rounded, Sharp). |
| Lucide | ISC | 1,700+ | Lightweight, clean line icons. Already used by gpui-component (87 icon subset). |

Future candidates (not in scope for initial implementation):

| Icon Set | License | Icons | Notes |
|----------|---------|-------|-------|
| Phosphor | MIT | 9,000+ | Six weights (Thin → Fill). Very comprehensive. |
| Tabler | MIT | 5,900+ | Consistent stroke width. |

### Feature flags

```toml
[features]
default = ["system-icons", "material-icons"]
system-icons = []      # Platform-native icon loading (macOS/Windows/Linux APIs)
material-icons = []    # Bundle Material Symbols SVGs as cross-platform fallback
lucide-icons = []      # Bundle Lucide SVGs as optional icon set
```

---

## Default Assignments

Each preset specifies which icon set to use. Native theme presets use
their platform's icon set. Community themes detect the host OS at runtime.

### Native theme presets

| Preset | `icon_set` | Rationale |
|--------|-----------|-----------|
| `windows-11` | `"segoe-fluent"` | Windows Fluent Design icons |
| `macos-sonoma` | `"sf-symbols"` | Apple SF Symbols |
| `ios` | `"sf-symbols"` | Same SF Symbols as macOS |
| `adwaita` | `"freedesktop"` | GNOME Adwaita icon theme |
| `kde-breeze` | `"freedesktop"` | KDE Breeze icon theme |
| `material` | `"material"` | Material Design's own icon system |

### Community theme presets

| Preset | `icon_set` | Resolved at runtime via `system_icon_set()` |
|--------|-----------|---------------------------------------------|
| `catppuccin-latte` | `None` (system) | macOS → SF Symbols, Linux → freedesktop, Windows → Segoe Fluent |
| `catppuccin-frappe` | `None` (system) | same |
| `catppuccin-macchiato` | `None` (system) | same |
| `catppuccin-mocha` | `None` (system) | same |
| `nord` | `None` (system) | same |
| `dracula` | `None` (system) | same |
| `gruvbox` | `None` (system) | same |
| `solarized` | `None` (system) | same |
| `tokyo-night` | `None` (system) | same |
| `one-dark` | `None` (system) | same |
| `default` | `None` (system) | same |

Community themes are color-only — they have no opinion on icons. Using the
host OS native icons makes them feel native on every platform. The user
can override `icon_set` to `"material"` or `"lucide"` for a consistent
cross-platform look.

### Override examples

```rust
// Use native icons (automatic per-OS)
let variant = pick_variant(&theme, false).unwrap();
// variant.icon_set is None → resolved to system_icon_set()

// Force Material icons everywhere
let mut variant = pick_variant(&theme, false).unwrap().clone();
variant.icon_set = Some("material".into());

// Force Lucide icons (e.g. for gpui-component consistency)
variant.icon_set = Some("lucide".into());
```

---

## Availability Matrix

Columns are grouped by icon system since macOS and iOS share SF Symbols, and
Adwaita and KDE Breeze share freedesktop icon names.

- **✓** = standard named icon exists
- **—** = no standard icon for this role

### Dialog / Alert

| Role | Windows 11 | macOS / iOS | Adwaita / Breeze | Material | Lucide |
|------|-----------|-------------|-----------------|----------|--------|
| `dialog-warning` | ✓ | ✓ | ✓ | ✓ | ✓ |
| `dialog-error` | ✓ | ✓ | ✓ | ✓ | ✓ |
| `dialog-info` | ✓ | ✓ | ✓ | ✓ | ✓ |
| `dialog-question` | ✓ ^1^ | ✓ | ✓ | ✓ | ✓ |
| `dialog-success` | — | ✓ | ✓ ^2^ | ✓ | ✓ |
| `shield` | ✓ | ✓ | ✓ | ✓ | ✓ |

^1^ `MB_ICONQUESTION` / `IDI_QUESTION` is no longer recommended by Microsoft
UX guidelines. Microsoft recommends using `dialog-info` instead.\
^2^ freedesktop spec has no `dialog-success`. Adwaita provides `emblem-ok-symbolic`.

### Window Controls

| Role | Windows 11 | macOS / iOS | Adwaita / Breeze | Material | Lucide |
|------|-----------|-------------|-----------------|----------|--------|
| `window-close` | ✓ ^3^ | ✓ ^4^ | ✓ | ✓ | ✓ |
| `window-minimize` | ✓ ^3^ | ✓ ^4^ | ✓ ^14^ | ✓ | ✓ |
| `window-maximize` | ✓ ^3^ | ✓ ^4^ | ✓ ^14^ | ✓ | ✓ |
| `window-restore` | ✓ ^3^ | — | ✓ ^14^ | ✓ | ✓ |

^3^ Windows draws title-bar buttons via DWM, not from icons. Segoe Fluent
Icons provides matching glyphs (`ChromeClose`, `ChromeMinimize`, etc.).\
^4^ macOS uses system-drawn colored circles (traffic lights). SF Symbols
provides general-purpose equivalents (`xmark`, `minus`,
`arrow.up.left.and.arrow.down.right`), not dedicated window-control icons.
iOS does not have windowed UI.

### Common Actions

| Role | Windows 11 | macOS / iOS | Adwaita / Breeze | Material | Lucide |
|------|-----------|-------------|-----------------|----------|--------|
| `action-save` | ✓ ^5^ | ✓ | ✓ | ✓ | ✓ |
| `action-delete` | ✓ | ✓ | ✓ | ✓ | ✓ |
| `action-copy` | ✓ ^5^ | ✓ | ✓ | ✓ | ✓ |
| `action-paste` | ✓ ^5^ | ✓ | ✓ | ✓ | ✓ |
| `action-cut` | ✓ ^5^ | ✓ | ✓ | ✓ | ✓ |
| `action-undo` | ✓ ^5^ | ✓ | ✓ | ✓ | ✓ |
| `action-redo` | ✓ ^5^ | ✓ | ✓ | ✓ | ✓ |
| `action-search` | ✓ | ✓ | ✓ | ✓ | ✓ |
| `action-settings` | ✓ | ✓ | ✓ | ✓ | ✓ |
| `action-edit` | ✓ ^5^ | ✓ | ✓ ^6^ | ✓ | ✓ |
| `action-add` | ✓ ^5^ | ✓ | ✓ | ✓ | ✓ |
| `action-remove` | ✓ ^5^ | ✓ | ✓ | ✓ | ✓ |
| `action-refresh` | ✓ ^5^ | ✓ | ✓ | ✓ | ✓ |
| `action-print` | ✓ | ✓ | ✓ | ✓ | ✓ |

^5^ Available as Segoe Fluent Icons glyphs, not as SHSTOCKICONID stock icons.\
^6^ `document-edit` is not in the freedesktop spec but is shipped by Adwaita.

### Navigation

| Role | Windows 11 | macOS / iOS | Adwaita / Breeze | Material | Lucide |
|------|-----------|-------------|-----------------|----------|--------|
| `nav-back` | ✓ ^5^ | ✓ | ✓ | ✓ | ✓ |
| `nav-forward` | ✓ ^5^ | ✓ | ✓ | ✓ | ✓ |
| `nav-up` | ✓ ^5^ | ✓ | ✓ | ✓ | ✓ |
| `nav-down` | ✓ ^5^ | ✓ | ✓ | ✓ | ✓ |
| `nav-home` | ✓ ^5^ | ✓ | ✓ | ✓ | ✓ |
| `nav-menu` | ✓ ^5^ | ✓ | ✓ ^7^ | ✓ | ✓ |

^7^ `open-menu` is not in the freedesktop spec but is shipped by GNOME/Adwaita
as the standard hamburger menu icon.

### Files / Places

| Role | Windows 11 | macOS / iOS | Adwaita / Breeze | Material | Lucide |
|------|-----------|-------------|-----------------|----------|--------|
| `file-generic` | ✓ | ✓ | ✓ | ✓ | ✓ |
| `folder-closed` | ✓ | ✓ | ✓ | ✓ | ✓ |
| `folder-open` | ✓ | — ^8^ | ✓ | ✓ | ✓ |
| `trash-empty` | ✓ | ✓ | ✓ | ✓ ^9^ | ✓ ^9^ |
| `trash-full` | ✓ | — | ✓ | — | — |

^8^ SF Symbols has no open-folder variant. The `folder` family includes
`folder`, `folder.fill`, and badge variants, but no `folder.open`.\
^9^ Material and Lucide have no separate trash icon; `delete` / `trash-2`
serves both roles.

### Status

| Role | Windows 11 | macOS / iOS | Adwaita / Breeze | Material | Lucide |
|------|-----------|-------------|-----------------|----------|--------|
| `status-loading` | — ^10^ | — ^10^ | ✓ | ✓ | ✓ |
| `status-check` | ✓ ^5^ | ✓ | ✓ ^11^ | ✓ | ✓ |
| `status-error` | ✓ | ✓ | ✓ | ✓ | ✓ |

^10^ Loading is typically an animated widget (progress ring / spinner), not a
static icon.\
^11^ Adwaita uses `emblem-default-symbolic` (checkmark emblem).

### System

| Role | Windows 11 | macOS / iOS | Adwaita / Breeze | Material | Lucide |
|------|-----------|-------------|-----------------|----------|--------|
| `user-account` | ✓ | ✓ | ✓ ^12^ | ✓ | ✓ |
| `notification` | ✓ ^5^ | ✓ | — ^13^ | ✓ | ✓ |
| `help` | ✓ | ✓ | ✓ | ✓ | ✓ |
| `lock` | ✓ | ✓ | ✓ | ✓ | ✓ |

^12^ `system-users` is not in the freedesktop spec but is shipped by Adwaita.\
^13^ The freedesktop spec does not define a notification bell icon. Linux
desktops handle notifications through system services, not standalone icons.\
^14^ `window-minimize`, `window-maximize`, and `window-restore` are not in the
freedesktop Icon Naming Specification. Both Adwaita and Breeze ship these icons
as de facto standard names.

---

## Icon Identifiers

### Windows 11

Two sources: **SHSTOCKICONID** values retrieved via `SHGetStockIconInfo`, and
**Segoe Fluent Icons** font glyphs.

#### SHSTOCKICONID (stock system icons)

| Role | Constant | Value |
|------|----------|-------|
| `dialog-warning` | `SIID_WARNING` | 78 |
| `dialog-error` | `SIID_ERROR` | 80 |
| `dialog-info` | `SIID_INFO` | 79 |
| `dialog-question` | `IDI_QUESTION` | (not recommended) |
| `shield` | `SIID_SHIELD` | 77 |
| `file-generic` | `SIID_DOCNOASSOC` | 0 |
| `folder-closed` | `SIID_FOLDER` | 3 |
| `folder-open` | `SIID_FOLDEROPEN` | 4 |
| `trash-empty` | `SIID_RECYCLER` | 31 |
| `trash-full` | `SIID_RECYCLERFULL` | 32 |
| `action-delete` | `SIID_DELETE` | 84 |
| `action-search` | `SIID_FIND` | 22 |
| `action-settings` | `SIID_SETTINGS` | 106 |
| `action-print` | `SIID_PRINTER` | 16 |
| `user-account` | `SIID_USERS` | 96 |
| `help` | `SIID_HELP` | 23 |
| `lock` | `SIID_LOCK` | 47 |
| `status-error` | `SIID_ERROR` | 80 |

TaskDialog also defines: `TD_WARNING_ICON`, `TD_ERROR_ICON`,
`TD_INFORMATION_ICON`, `TD_SHIELD_ICON`.

#### Segoe Fluent Icons (UI glyphs)

| Role | Glyph name | Codepoint |
|------|-----------|-----------|
| `action-save` | Save | U+E74E |
| `action-copy` | Copy | U+E8C8 |
| `action-paste` | Paste | U+E77F |
| `action-cut` | Cut | U+E8C6 |
| `action-undo` | Undo | U+E7A7 |
| `action-redo` | Redo | U+E7A6 |
| `action-edit` | Edit | U+E70F |
| `action-add` | Add | U+E710 |
| `action-remove` | Remove | U+E738 |
| `action-refresh` | Refresh | U+E72C |
| `nav-back` | Back | U+E72B |
| `nav-forward` | Forward | U+E72A |
| `nav-up` | Up | U+E74A |
| `nav-down` | Down | U+E74B |
| `nav-home` | Home | U+E80F |
| `nav-menu` | GlobalNavigationButton | U+E700 |
| `notification` | Ringer | U+EA8F |
| `status-check` | CheckMark | U+E73E |
| `window-close` | ChromeClose | U+E8BB |
| `window-minimize` | ChromeMinimize | U+E921 |
| `window-maximize` | ChromeMaximize | U+E922 |
| `window-restore` | ChromeRestore | U+E923 |

### macOS / iOS (SF Symbols)

All icons use Apple's SF Symbols `systemName` strings. Append `.fill` for filled
variants.

#### Dialog / Alert

| Role | SF Symbol |
|------|----------|
| `dialog-warning` | `exclamationmark.triangle.fill` |
| `dialog-error` | `xmark.circle.fill` |
| `dialog-info` | `info.circle.fill` |
| `dialog-question` | `questionmark.circle.fill` |
| `dialog-success` | `checkmark.circle.fill` |
| `shield` | `shield.fill` |

NSAlert styles: `.warning`, `.critical`, `.informational`.
NSImage.Name constants: `NSImageNameCaution` (warning), `NSImageNameInfo` (info).

#### Common Actions

| Role | SF Symbol |
|------|----------|
| `action-save` | `square.and.arrow.down` ^a^ |
| `action-delete` | `trash` |
| `action-copy` | `doc.on.doc` |
| `action-paste` | `doc.on.clipboard` |
| `action-cut` | `scissors` |
| `action-undo` | `arrow.uturn.backward` |
| `action-redo` | `arrow.uturn.forward` |
| `action-search` | `magnifyingglass` |
| `action-settings` | `gearshape` |
| `action-edit` | `pencil` |
| `action-add` | `plus` |
| `action-remove` | `minus` |
| `action-refresh` | `arrow.clockwise` |
| `action-print` | `printer` |

^a^ `square.and.arrow.down` is Apple's standard download/import icon. SF
Symbols has no dedicated "save" icon (macOS apps auto-save). This is the
closest available equivalent.

Note: `gear` (Apple's stylized settings icon) and `gearshape` (conventional
gear shape) both exist. `gearshape` is the conventional choice.

#### Navigation

| Role | SF Symbol |
|------|----------|
| `nav-back` | `chevron.backward` |
| `nav-forward` | `chevron.forward` |
| `nav-up` | `chevron.up` |
| `nav-down` | `chevron.down` |
| `nav-home` | `house` |
| `nav-menu` | `line.horizontal.3` |

#### Window / System

| Role | SF Symbol |
|------|----------|
| `window-close` | `xmark` |
| `window-minimize` | `minus` |
| `window-maximize` | `arrow.up.left.and.arrow.down.right` |
| `file-generic` | `doc` |
| `folder-closed` | `folder` |
| `trash-empty` | `trash` |
| `user-account` | `person.fill` |
| `notification` | `bell.fill` |
| `help` | `questionmark.circle` |
| `lock` | `lock.fill` |
| `status-check` | `checkmark` |
| `status-error` | `xmark.circle.fill` |

### Adwaita / KDE Breeze (freedesktop)

Both themes implement the freedesktop Icon Naming Specification plus
additional de facto standard names (noted where applicable). Append
`-symbolic` for monochrome, recolorable SVG variants.

Breeze provides light and dark icon sets (`breeze/` and `breeze-dark/`).
Visual style differs but names are the same.

#### Dialog / Alert

| Role | Icon name |
|------|----------|
| `dialog-warning` | `dialog-warning` |
| `dialog-error` | `dialog-error` |
| `dialog-info` | `dialog-information` |
| `dialog-question` | `dialog-question` |
| `dialog-success` | `emblem-ok-symbolic` |
| `shield` | `security-high` |

#### Common Actions

| Role | Icon name |
|------|----------|
| `action-save` | `document-save` |
| `action-delete` | `edit-delete` |
| `action-copy` | `edit-copy` |
| `action-paste` | `edit-paste` |
| `action-cut` | `edit-cut` |
| `action-undo` | `edit-undo` |
| `action-redo` | `edit-redo` |
| `action-search` | `edit-find` |
| `action-settings` | `preferences-system` |
| `action-edit` | `document-edit` |
| `action-add` | `list-add` |
| `action-remove` | `list-remove` |
| `action-refresh` | `view-refresh` |
| `action-print` | `document-print` |

#### Navigation

| Role | Icon name |
|------|----------|
| `nav-back` | `go-previous` |
| `nav-forward` | `go-next` |
| `nav-up` | `go-up` |
| `nav-down` | `go-down` |
| `nav-home` | `go-home` |
| `nav-menu` | `open-menu` |

#### Window / Files / System

| Role | Icon name |
|------|----------|
| `window-close` | `window-close` |
| `window-minimize` | `window-minimize` ^14^ |
| `window-maximize` | `window-maximize` ^14^ |
| `window-restore` | `window-restore` ^14^ |
| `file-generic` | `text-x-generic` |
| `folder-closed` | `folder` |
| `folder-open` | `folder-open` |
| `trash-empty` | `user-trash` |
| `trash-full` | `user-trash-full` |
| `user-account` | `system-users` |
| `help` | `help-browser` |
| `lock` | `system-lock-screen` |
| `status-loading` | `process-working` |
| `status-check` | `emblem-default` |
| `status-error` | `dialog-error` |

### Material Design (Material Symbols)

Three styles: **Outlined** (default), Rounded, Sharp. Fill, weight, grade, and
optical size are controlled via font variation axes, not icon name suffixes.

#### Dialog / Alert

| Role | Symbol name |
|------|------------|
| `dialog-warning` | `warning` |
| `dialog-error` | `error` |
| `dialog-info` | `info` |
| `dialog-question` | `help` |
| `dialog-success` | `check_circle` |
| `shield` | `shield` |

#### Common Actions

| Role | Symbol name |
|------|------------|
| `action-save` | `save` |
| `action-delete` | `delete` |
| `action-copy` | `content_copy` |
| `action-paste` | `content_paste` |
| `action-cut` | `content_cut` |
| `action-undo` | `undo` |
| `action-redo` | `redo` |
| `action-search` | `search` |
| `action-settings` | `settings` |
| `action-edit` | `edit` |
| `action-add` | `add` |
| `action-remove` | `remove` |
| `action-refresh` | `refresh` |
| `action-print` | `print` |

#### Navigation

| Role | Symbol name |
|------|------------|
| `nav-back` | `arrow_back` |
| `nav-forward` | `arrow_forward` |
| `nav-up` | `arrow_upward` |
| `nav-down` | `arrow_downward` |
| `nav-home` | `home` |
| `nav-menu` | `menu` |

#### Window / Files / System

| Role | Symbol name |
|------|------------|
| `window-close` | `close` |
| `window-minimize` | `minimize` |
| `window-maximize` | `open_in_full` |
| `window-restore` | `close_fullscreen` |
| `file-generic` | `description` |
| `folder-closed` | `folder` |
| `folder-open` | `folder_open` |
| `trash-empty` | `delete` |
| `user-account` | `person` |
| `notification` | `notifications` |
| `help` | `help` |
| `lock` | `lock` |
| `status-loading` | `progress_activity` |
| `status-check` | `check` |
| `status-error` | `error` |

### Lucide

Lucide icons use kebab-case names. The full library has 1,700+ icons. The
gpui-component crate (v0.5) bundles a curated subset of 87 as the `IconName`
enum. The table below shows both the full Lucide name and the gpui-component
`IconName` variant where available.

#### Dialog / Alert

| Role | Lucide name | gpui-component `IconName` |
|------|------------|--------------------------|
| `dialog-warning` | `triangle-alert` | `TriangleAlert` |
| `dialog-error` | `circle-x` | `CircleX` |
| `dialog-info` | `info` | `Info` |
| `dialog-question` | `circle-question-mark` | — |
| `dialog-success` | `circle-check` | `CircleCheck` |
| `shield` | `shield` | — |

#### Common Actions

| Role | Lucide name | gpui-component `IconName` |
|------|------------|--------------------------|
| `action-save` | `save` | — |
| `action-delete` | `trash-2` | `Delete` |
| `action-copy` | `copy` | `Copy` |
| `action-paste` | `clipboard-paste` | — |
| `action-cut` | `scissors` | — |
| `action-undo` | `undo-2` | `Undo2` |
| `action-redo` | `redo-2` | `Redo2` |
| `action-search` | `search` | `Search` |
| `action-settings` | `settings` | `Settings` |
| `action-edit` | `pencil` | — |
| `action-add` | `plus` | `Plus` |
| `action-remove` | `minus` | `Minus` |
| `action-refresh` | `refresh-cw` | — |
| `action-print` | `printer` | — |

#### Navigation

| Role | Lucide name | gpui-component `IconName` |
|------|------------|--------------------------|
| `nav-back` | `chevron-left` | `ChevronLeft` |
| `nav-forward` | `chevron-right` | `ChevronRight` |
| `nav-up` | `chevron-up` | `ChevronUp` |
| `nav-down` | `chevron-down` | `ChevronDown` |
| `nav-home` | `house` | — |
| `nav-menu` | `menu` | `Menu` |

#### Window / Files / System

| Role | Lucide name | gpui-component `IconName` |
|------|------------|--------------------------|
| `window-close` | `x` | `Close` |
| `window-minimize` | `minimize` | `WindowMinimize` |
| `window-maximize` | `maximize` | `WindowMaximize` |
| `window-restore` | `minimize-2` | `WindowRestore` |
| `file-generic` | `file` | `File` |
| `folder-closed` | `folder-closed` | `FolderClosed` |
| `folder-open` | `folder-open` | `FolderOpen` |
| `trash-empty` | `trash-2` | `Delete` |
| `user-account` | `user` | `User` |
| `notification` | `bell` | `Bell` |
| `help` | `circle-question-mark` | — |
| `lock` | `lock` | — |
| `status-loading` | `loader` | `Loader` |
| `status-check` | `check` | `Check` |
| `status-error` | `circle-x` | `CircleX` |

#### gpui-component coverage

gpui-component's 87-icon subset covers **27 of 42** icon roles. Roles without
a matching `IconName` variant (marked — above) would need either:
- The full Lucide SVG library bundled via the `lucide-icons` feature
- A fallback to Material or the native icon set

---

## Connector Interface

The connector's job is trivial — convert `IconData` to the toolkit's image
type. No platform logic, no icon lookups.

### gpui connector example

```rust
use gpui::RenderImage;
use native_theme::{IconData, IconRole};

/// Convert native-theme IconData to a gpui-renderable image.
pub fn load_icon(variant: &ThemeVariant, role: IconRole, size: u32) -> Option<RenderImage> {
    let icon_set = variant.icon_set.as_deref()
        .unwrap_or_else(|| native_theme::system_icon_set().name());

    let icon_data = native_theme::load_icon(role, icon_set)?;

    match icon_data {
        IconData::Rgba { width, height, data } => {
            // Create gpui image from raw RGBA pixels
            Some(RenderImage::from_rgba(width, height, data))
        }
        IconData::Svg(bytes) => {
            // Parse SVG and render to gpui
            Some(render_svg_to_image(&bytes, size))
        }
    }
}
```

### Lucide shortcut for gpui

When `icon_set` is `"lucide"`, the gpui connector can skip `load_icon`
entirely and map `IconRole` directly to gpui-component's `IconName` enum,
since those icons are already loaded:

```rust
pub fn icon_name_for_role(role: IconRole) -> Option<IconName> {
    match role {
        IconRole::DialogWarning => Some(IconName::TriangleAlert),
        IconRole::DialogError => Some(IconName::CircleX),
        IconRole::ActionSearch => Some(IconName::Search),
        // ... see Lucide table above
        _ => None, // role not in gpui-component's 87-icon subset
    }
}
```

---

## References

- [SHSTOCKICONID (shellapi.h)](https://learn.microsoft.com/en-us/windows/win32/api/shellapi/ne-shellapi-shstockiconid) - Win32 stock icon enum
- [Segoe Fluent Icons](https://learn.microsoft.com/en-us/windows/apps/design/style/segoe-fluent-icons-font) - Windows 11 system icon font
- [SF Symbols](https://developer.apple.com/sf-symbols/) - Apple icon library
- [freedesktop Icon Naming Specification](https://specifications.freedesktop.org/icon-naming-spec/latest/) - Linux desktop standard
- [freedesktop Icon Theme Specification](https://specifications.freedesktop.org/icon-theme/latest/) - Icon theme directory lookup
- [Adwaita Icon Theme](https://gitlab.gnome.org/GNOME/adwaita-icon-theme) - GNOME icon set (LGPL-3.0-or-later / CC-BY-SA-3.0-US)
- [Breeze Icons](https://invent.kde.org/frameworks/breeze-icons) - KDE icon set (LGPL-3.0)
- [Material Symbols](https://fonts.google.com/icons) - Google icon library (Apache 2.0)
- [Lucide Icons](https://lucide.dev/) - Fork of Feather Icons (ISC license)
- [Phosphor Icons](https://phosphoricons.com/) - Flexible icon family (MIT)
