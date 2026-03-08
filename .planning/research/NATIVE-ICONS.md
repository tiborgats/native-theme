# Native Icon Mapping for native-theme

## Purpose

Each supported OS uses its own native icon set for dialogs, alerts, window controls,
and common UI actions. To achieve true native look and feel, native-theme must map
a set of **semantic icon roles** to platform-specific icon identifiers.

Currently the GPUI showcase uses Lucide icons for everything — a Windows warning
dialog rendered with a Lucide triangle looks nothing like the real Windows experience.

---

## Semantic Icon Roles

These are the icon concepts that native-theme should define. Each platform maps
these to its own native icon identifiers.

### Dialog / Alert Icons (highest priority)

| Role            | Description                                    |
|-----------------|------------------------------------------------|
| `dialog-warning`  | Warning/caution dialog (yellow triangle + !)  |
| `dialog-error`    | Error/critical dialog (red circle + X)        |
| `dialog-info`     | Informational dialog (blue circle + i)        |
| `dialog-question` | Question/confirmation dialog (? mark)         |
| `dialog-success`  | Success/completion (green checkmark)          |
| `shield`          | Security/elevation prompt (UAC-style)         |

### Window Controls

| Role              | Description                                  |
|-------------------|----------------------------------------------|
| `window-close`      | Close window button                        |
| `window-minimize`   | Minimize window button                     |
| `window-maximize`   | Maximize window button                     |
| `window-restore`    | Restore from maximized                     |

### Common Actions

| Role            | Description                                    |
|-----------------|------------------------------------------------|
| `action-save`     | Save file                                    |
| `action-delete`   | Delete / trash                               |
| `action-copy`     | Copy to clipboard                            |
| `action-paste`    | Paste from clipboard                         |
| `action-cut`      | Cut to clipboard                             |
| `action-undo`     | Undo last action                             |
| `action-redo`     | Redo undone action                           |
| `action-search`   | Search / find                                |
| `action-settings` | Settings / preferences                       |
| `action-edit`     | Edit / pencil                                |
| `action-add`      | Add / create new                             |
| `action-remove`   | Remove / subtract                            |
| `action-refresh`  | Refresh / reload                             |
| `action-print`    | Print                                        |

### Navigation

| Role            | Description                                    |
|-----------------|------------------------------------------------|
| `nav-back`        | Navigate back / previous                     |
| `nav-forward`     | Navigate forward / next                      |
| `nav-up`          | Navigate up                                  |
| `nav-down`        | Navigate down                                |
| `nav-home`        | Go to home / start                           |
| `nav-menu`        | Hamburger / overflow menu                    |

### File Types / Places

| Role            | Description                                    |
|-----------------|------------------------------------------------|
| `file-generic`    | Generic file / document                      |
| `folder-closed`   | Closed folder                                |
| `folder-open`     | Open folder                                  |
| `trash-empty`     | Empty recycle bin / trash                     |
| `trash-full`      | Non-empty recycle bin / trash                 |

### Status

| Role            | Description                                    |
|-----------------|------------------------------------------------|
| `status-loading`  | Loading / spinner                            |
| `status-check`    | Completed / checkmark                        |
| `status-error`    | Failed / error indicator                     |

### System

| Role            | Description                                    |
|-----------------|------------------------------------------------|
| `user-account`    | User / account / person                      |
| `notification`    | Notification bell                            |
| `help`            | Help / question mark circle                  |
| `lock`            | Locked / security                            |

---

## Platform Mapping

### Windows 11 (Fluent Design)

**Icon system:** Segoe Fluent Icons font + SHSTOCKICONID shell stock icons + TaskDialog icons

**Dialog/Alert icons (SHSTOCKICONID via SHGetStockIconInfo):**

| Semantic Role     | Win32 Identifier         | SHSTOCKICONID   | Description                    |
|-------------------|--------------------------|-----------------|--------------------------------|
| `dialog-warning`  | `IDI_WARNING`            | `SIID_WARNING` (78)  | Yellow triangle + exclamation |
| `dialog-error`    | `IDI_ERROR`              | `SIID_ERROR` (80)    | Red circle + white X          |
| `dialog-info`     | `IDI_INFORMATION`        | `SIID_INFO` (79)     | Blue circle + white i         |
| `dialog-question` | `IDI_QUESTION`           | —                    | Blue circle + ?  (deprecated) |
| `shield`          | —                        | `SIID_SHIELD` (77)   | UAC shield icon               |

**TaskDialog constants:** `TD_WARNING_ICON`, `TD_ERROR_ICON`, `TD_INFORMATION_ICON`, `TD_SHIELD_ICON`

**File/System icons (SHSTOCKICONID):**

| Semantic Role   | SHSTOCKICONID              |
|-----------------|----------------------------|
| `file-generic`  | `SIID_DOCNOASSOC` (0)      |
| `folder-closed` | `SIID_FOLDER` (3)          |
| `folder-open`   | `SIID_FOLDEROPEN` (4)      |
| `trash-empty`   | `SIID_RECYCLER` (31)       |
| `trash-full`    | `SIID_RECYCLERFULL` (32)   |
| `action-delete` | `SIID_DELETE` (84)         |
| `action-search` | `SIID_FIND` (22)           |
| `action-settings` | `SIID_SETTINGS` (106)    |
| `help`          | `SIID_HELP` (23)           |
| `lock`          | `SIID_LOCK` (47)           |
| `user-account`  | `SIID_USERS` (96)          |

**Window controls:** Custom-drawn by DWM, not from icon font. Rendered via title bar painting.

**UI action icons (Segoe Fluent Icons / Symbol enum):**
Save, Delete, Find, Copy, Paste, Cut, Undo, Redo, Add, Remove, Refresh, Print,
Back, Forward, Up, Home, GlobalNavigationButton (menu).
All available as glyphs in the Segoe Fluent Icons font with specific codepoints.

---

### macOS / iOS (SF Symbols)

**Icon system:** SF Symbols (6,900+ symbols), NSImage system images

**Dialog/Alert icons:**

| Semantic Role     | SF Symbol Name                      | NSAlert Style          |
|-------------------|-------------------------------------|------------------------|
| `dialog-warning`  | `exclamationmark.triangle.fill`     | `.warning`             |
| `dialog-error`    | `xmark.circle.fill`                | `.critical`            |
| `dialog-info`     | `info.circle.fill`                 | `.informational`       |
| `dialog-question` | `questionmark.circle.fill`         | —                      |
| `dialog-success`  | `checkmark.circle.fill`            | —                      |
| `shield`          | `shield.fill`                      | —                      |

**NSImage.Name constants:** `NSImageNameCaution` (warning triangle), `NSImageNameInfo` (info)

**Common actions:**

| Semantic Role     | SF Symbol Name           |
|-------------------|--------------------------|
| `action-save`     | `square.and.arrow.down`  |
| `action-delete`   | `trash`                  |
| `action-copy`     | `doc.on.doc`             |
| `action-paste`    | `doc.on.clipboard`       |
| `action-cut`      | `scissors`               |
| `action-undo`     | `arrow.uturn.backward`   |
| `action-redo`     | `arrow.uturn.forward`    |
| `action-search`   | `magnifyingglass`        |
| `action-settings` | `gearshape`              |
| `action-edit`     | `pencil`                 |
| `action-add`      | `plus`                   |
| `action-remove`   | `minus`                  |
| `action-refresh`  | `arrow.clockwise`        |
| `action-print`    | `printer`                |

**Navigation:**

| Semantic Role  | SF Symbol Name        |
|----------------|-----------------------|
| `nav-back`     | `chevron.backward`    |
| `nav-forward`  | `chevron.forward`     |
| `nav-up`       | `chevron.up`          |
| `nav-down`     | `chevron.down`        |
| `nav-home`     | `house`               |
| `nav-menu`     | `line.3.horizontal`   |

**Window/System:**

| Semantic Role     | SF Symbol Name        |
|-------------------|-----------------------|
| `window-close`    | `xmark`               |
| `window-minimize` | `minus`               |
| `window-maximize` | `arrow.up.left.and.arrow.down.right` |
| `user-account`    | `person.fill`         |
| `notification`    | `bell.fill`           |
| `help`            | `questionmark.circle` |
| `lock`            | `lock.fill`           |
| `folder-closed`   | `folder.fill`         |
| `folder-open`     | `folder.badge.gearshape` |
| `file-generic`    | `doc`                 |
| `trash-empty`     | `trash`               |

**Naming convention:** Dot-separated with modifiers: `.fill`, `.circle`, `.square`, `.slash`, `.badge`

---

### GNOME / Adwaita

**Icon system:** freedesktop Icon Naming Specification + Adwaita icon theme, SVG format,
with `-symbolic` suffix variants for recolorable monochrome icons.

**Dialog/Alert icons:**

| Semantic Role     | freedesktop Name          | Symbolic Variant              |
|-------------------|---------------------------|-------------------------------|
| `dialog-warning`  | `dialog-warning`          | `dialog-warning-symbolic`     |
| `dialog-error`    | `dialog-error`            | `dialog-error-symbolic`       |
| `dialog-info`     | `dialog-information`      | `dialog-information-symbolic` |
| `dialog-question` | `dialog-question`         | `dialog-question-symbolic`    |
| `dialog-success`  | (no standard; use emblem) | `emblem-ok-symbolic`          |
| `shield`          | `security-high`           | `security-high-symbolic`      |

**Common actions:**

| Semantic Role     | freedesktop Name     |
|-------------------|----------------------|
| `action-save`     | `document-save`      |
| `action-delete`   | `edit-delete`        |
| `action-copy`     | `edit-copy`          |
| `action-paste`    | `edit-paste`         |
| `action-cut`      | `edit-cut`           |
| `action-undo`     | `edit-undo`          |
| `action-redo`     | `edit-redo`          |
| `action-search`   | `system-search`      |
| `action-settings` | `preferences-system` |
| `action-edit`     | `document-edit`      |
| `action-add`      | `list-add`           |
| `action-remove`   | `list-remove`        |
| `action-refresh`  | `view-refresh`       |
| `action-print`    | `document-print`     |

**Navigation:**

| Semantic Role  | freedesktop Name  |
|----------------|-------------------|
| `nav-back`     | `go-previous`     |
| `nav-forward`  | `go-next`         |
| `nav-up`       | `go-up`           |
| `nav-down`     | `go-down`         |
| `nav-home`     | `go-home`         |
| `nav-menu`     | `open-menu`       |

**Window/System:**

| Semantic Role     | freedesktop Name     |
|-------------------|----------------------|
| `window-close`    | `window-close`       |
| `window-minimize` | `window-minimize`    |
| `window-maximize` | `window-maximize`    |
| `window-restore`  | `window-restore`     |
| `user-account`    | `system-users`       |
| `notification`    | `preferences-system-notifications` |
| `help`            | `help-about`         |
| `lock`            | `system-lock-screen` |
| `folder-closed`   | `folder`             |
| `folder-open`     | `folder-open`        |
| `file-generic`    | `text-x-generic`     |
| `trash-empty`     | `user-trash`         |
| `trash-full`      | `user-trash-full`    |

**Naming convention:** Dash-separated, `-symbolic` suffix for monochrome variants.
Format: SVG in `/usr/share/icons/Adwaita/`. Sizes: scalable, 16, 24, 32, 48.

---

### KDE / Breeze

**Icon system:** Same freedesktop Icon Naming Specification as GNOME, different visual style.
All freedesktop names from the Adwaita table above also apply to Breeze.

Breeze provides both light and dark icon sets (`/usr/share/icons/breeze/` and
`/usr/share/icons/breeze-dark/`).

Breeze adds KDE-specific icons beyond the spec but the core dialog/action/navigation
icons use the same freedesktop names. Visual style is flatter and more geometric than
Adwaita.

---

### Material Design 3

**Icon system:** Material Symbols (variable font, 3,500+ icons).
Three styles: **Outlined** (default/recommended), Rounded, Sharp.

**Dialog/Alert icons:**

| Semantic Role     | Material Symbol Name |
|-------------------|----------------------|
| `dialog-warning`  | `warning`            |
| `dialog-error`    | `error`              |
| `dialog-info`     | `info`               |
| `dialog-question` | `help`               |
| `dialog-success`  | `check_circle`       |
| `shield`          | `shield`             |

**Common actions:**

| Semantic Role     | Material Symbol Name |
|-------------------|----------------------|
| `action-save`     | `save`               |
| `action-delete`   | `delete`             |
| `action-copy`     | `content_copy`       |
| `action-paste`    | `content_paste`      |
| `action-cut`      | `content_cut`        |
| `action-undo`     | `undo`               |
| `action-redo`     | `redo`               |
| `action-search`   | `search`             |
| `action-settings` | `settings`           |
| `action-edit`     | `edit`               |
| `action-add`      | `add`                |
| `action-remove`   | `remove`             |
| `action-refresh`  | `refresh`            |
| `action-print`    | `print`              |

**Navigation:**

| Semantic Role  | Material Symbol Name |
|----------------|----------------------|
| `nav-back`     | `arrow_back`         |
| `nav-forward`  | `arrow_forward`      |
| `nav-up`       | `arrow_upward`       |
| `nav-down`     | `arrow_downward`     |
| `nav-home`     | `home`               |
| `nav-menu`     | `menu`               |

**Window/System:**

| Semantic Role     | Material Symbol Name |
|-------------------|----------------------|
| `window-close`    | `close`              |
| `window-minimize` | `minimize`           |
| `window-maximize` | `open_in_full`       |
| `window-restore`  | `close_fullscreen`   |
| `user-account`    | `person`             |
| `notification`    | `notifications`      |
| `help`            | `help_outline`       |
| `lock`            | `lock`               |
| `folder-closed`   | `folder`             |
| `folder-open`     | `folder_open`        |
| `file-generic`    | `description`        |
| `trash-empty`     | `delete`             |

**Naming convention:** Underscore-separated, lowercase. Three style variants (outlined/rounded/sharp).

---

## Preset-to-Icon-System Mapping

| Preset              | Icon System           | Source                          |
|---------------------|-----------------------|---------------------------------|
| `windows-11`        | Segoe Fluent Icons    | Win32 SHSTOCKICONID + font      |
| `macos-sonoma`      | SF Symbols            | NSImage + systemName strings    |
| `ios`               | SF Symbols            | UIImage systemName strings      |
| `adwaita`           | freedesktop/Adwaita   | /usr/share/icons/Adwaita/       |
| `kde-breeze`        | freedesktop/Breeze    | /usr/share/icons/breeze/        |
| `material`          | Material Symbols      | Google Fonts variable font      |
| Community presets    | Fallback set          | Use freedesktop or Material     |

Community themes (catppuccin, nord, dracula, gruvbox, solarized, tokyo-night, one-dark)
don't have their own icon sets. They should use a sensible fallback — either the
freedesktop names (if on Linux) or a bundled default set.

---

## Implementation Considerations

### Option A: Icon Name Mapping (recommended for v0.2)

Add a `ThemeIcons` struct to `ThemeVariant` with semantic fields that resolve to
platform-specific icon identifier strings:

```rust
pub struct ThemeIcons {
    pub dialog_warning: Option<String>,  // e.g. "dialog-warning" or "exclamationmark.triangle.fill"
    pub dialog_error: Option<String>,
    pub dialog_info: Option<String>,
    // ...
}
```

Each preset TOML populates these with the platform-appropriate names.
Connectors translate these strings into toolkit-specific icon types.

### Option B: Bundled SVG Icons (future)

Ship platform-style SVG icon sets with native-theme. Each preset references
an icon theme directory. Heavier but ensures consistent rendering across platforms.

### Option C: Hybrid (long-term)

Icon name mapping for platforms that have native icon APIs (Windows, macOS, Linux),
with bundled SVG fallbacks for custom/community themes and cross-platform rendering.

---

## References

- [freedesktop Icon Naming Specification](https://specifications.freedesktop.org/icon-naming-spec/icon-naming-spec-latest.html)
- [SHSTOCKICONID (Win32)](https://learn.microsoft.com/en-us/windows/win32/api/shellapi/ne-shellapi-shstockiconid)
- [SF Symbols (Apple)](https://developer.apple.com/sf-symbols/)
- [Material Symbols (Google)](https://fonts.google.com/icons)
- [GNOME Themed Icons](https://developer.gnome.org/documentation/tutorials/themed-icons.html)
- [GNOME HIG - UI Icons](https://developer.gnome.org/hig/guidelines/ui-icons.html)
- [Breeze Icons (KDE)](https://invent.kde.org/frameworks/breeze-icons)
