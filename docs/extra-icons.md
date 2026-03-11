# Extra Icons Strategy

## The Problem

The native-theme crate defines 42 `IconRole` variants (dialog, window, action,
navigation, file, status, system icons). These map to native icons on every
platform: freedesktop on Linux, SF Symbols on macOS, Segoe Fluent on Windows.

But GUI toolkits need more icons than these 42 semantic roles. gpui-component,
for example, uses 86 `IconName` variants — panel layouts, disclosure chevrons,
sort indicators, visibility toggles, VCS indicators, and other app-UI concepts
that go beyond what a theme's semantic roles cover.

Of those 86, 28 overlap with existing `IconRole` mappings (e.g., `Copy` =
`ActionCopy`, `Search` = `ActionSearch`). The remaining **58 need freedesktop
name mappings** so they can also be loaded from the user's installed icon theme.

Every desktop icon theme contains equivalents for most of these 58 icons — just
under different names depending on the desktop environment's naming convention.
The challenge is mapping them per DE, and respecting licenses that prevent
bundling.

## Strategy: Three Layers

### Layer 1 — System icons for 42 `IconRole` variants (done)

`load_icon(role, icon_set)` already loads native icons from the system. On
Linux this uses the `freedesktop-icons` crate to read SVGs from the installed
theme (e.g., `/usr/share/icons/breeze/`). macOS and Windows use their own APIs.
Material SVGs serve as bundled fallback.

### Layer 2 — System theme lookup for toolkit extras (next step)

The user's icon theme is **already installed on disk**. No download is needed.
A new mapping function extends the existing freedesktop loader to cover all 86
gpui-component icons, not just the 42 `IconRole` variants. The connector tries
the system theme first; if the icon is missing, it falls back to bundled
Lucide/Material.

Because different desktop environments use different icon names for the same
concept, the mapping must be **DE-aware** — trying names in the order that
matches the detected DE (see "Naming Conventions" section below).

This is the primary path for achieving native look and feel for all icons.

### Layer 3 — Bundled permissive fallback (done)

Lucide (`ISC`) and Material Symbols (`Apache-2.0`) are compiled into the binary
via `include_bytes!` when their cargo features are enabled. The
`to_image_source_colored()` function tints monochrome SVGs to match the active
theme's foreground color. This provides acceptable visuals on any platform
without network access.

### Layer 4 — Runtime download (future, for edge cases)

Runtime download becomes relevant only when the system theme is not installed:

- Cross-platform development (macOS/Windows developer wanting Breeze-style icons)
- Linux user explicitly selects a theme that is not installed
- App developer wants a complete third-party icon library (Phosphor, Tabler, etc.)

See "Runtime Download" section below for details.

## Naming Conventions Across Desktop Environments

The freedesktop Icon Naming Specification defines a core set of standard names.
Beyond that core, each DE ecosystem has developed its own extensions. Icon themes
within the same DE share these naming conventions.

### Three naming layers

1. **freedesktop standard** — defined in the [specification][fd-spec]. Present
   in virtually all themes. Examples: `edit-copy`, `go-down`, `list-add`,
   `view-fullscreen`, `folder`, `edit-find-replace`.

2. **KDE convention** — extensions shared by Breeze, Oxygen, Papirus (colored
   layer), and partially Numix. These cover developer tools, panel layouts, VCS,
   and visibility toggles. Examples: `view-visible`, `configure`, `tab-close`,
   `code-context`, `view-left-close`.

3. **GNOME convention** — extensions used by Adwaita, elementary, and Papirus
   (symbolic layer). GNOME uses a smaller, curated set with different names for
   some of the same concepts. Examples: `view-reveal`, `view-more`,
   `sidebar-show`, `list-drag-handle`.

### Where concepts differ by DE

| Concept | KDE name | GNOME name |
|---|---|---|
| Show/hide toggle | `view-visible` / `view-hidden` | `view-reveal` / `view-conceal` |
| Overflow menu | `overflow-menu` | `view-more-horizontal` |
| Left sidebar | `sidebar-expand-left`, `view-left-close` | `sidebar-show` |
| Right sidebar | `view-right-close`, `view-right-new` | `sidebar-show-right` |
| Drag handle | `drag-handle` | `list-drag-handle` |
| Favorite | `emblem-favorite` | `starred` |

### Cinnamon

Linux Mint's Cinnamon desktop has its own panel icons (`view-left-pane-symbolic`,
`view-right-pane-symbolic`) but provides very few action icons. Most requests
fall through to Adwaita via theme inheritance.

### XFCE, LXQt, Mate

These DEs ship minimal icon themes and rely on the user's chosen theme (often
Papirus, Adwaita, or a KDE theme). No DE-specific icon names to map.

### Theme coverage summary

| Name category | Breeze | Oxygen | Papirus | Adwaita | elementary | Numix |
|---|---|---|---|---|---|---|
| freedesktop standard | all | all | all | all | all | all |
| KDE convention | all | most | all (colored) | — | — | partial |
| GNOME convention | — | — | all (symbolic) | all | partial | partial |

Papirus is uniquely **bilingual** — it implements KDE names in its colored icon
layers and GNOME names in its symbolic layer.

## Lookup Algorithm

For each gpui icon on Linux:

```
1. Detect DE via LinuxDesktop enum (already implemented)
2. Select name list based on DE:
   - KDE:      try KDE name → freedesktop name → bundled fallback
   - GNOME:    try GNOME name → freedesktop name → bundled fallback
   - Cinnamon: try Cinnamon name → GNOME name → freedesktop name → bundled
   - Other:    try freedesktop name → bundled fallback
3. For each candidate name:
   a. Try exact name via freedesktop-icons lookup
   b. Try with -symbolic suffix
   c. Next candidate
4. Fall back to bundled Lucide/Material (tinted to theme foreground)
```

## Complete gpui-component → freedesktop Mapping

All 86 gpui-component `IconName` variants with their icon names per desktop
environment convention. Verified against installed Breeze and Adwaita themes,
and cross-referenced with Oxygen, Papirus, elementary, and Numix repos.

**Columns:**
- **fd** = freedesktop standard name (works in all themes)
- **KDE** = KDE convention name (Breeze, Oxygen, Papirus colored)
- **GNOME** = GNOME convention name (Adwaita, elementary, Papirus symbolic)

A dash (—) means no equivalent exists in that convention; the loader skips it
and tries the next column.

### 28 icons covered by existing `IconRole` mappings

These already get native icons through the `IconRole` path in `load_icon()`.

| gpui `IconName` | `IconRole` | fd | KDE | GNOME |
|---|---|---|---|---|
| `Bell` | `Notification` | — | `notification-active` | — |
| `Check` | `StatusCheck` | — | `emblem-default` | — |
| `ChevronDown` | `NavDown` | `go-down` | | |
| `ChevronLeft` | `NavBack` | `go-previous` | | |
| `ChevronRight` | `NavForward` | `go-next` | | |
| `ChevronUp` | `NavUp` | `go-up` | | |
| `CircleCheck` | `DialogSuccess` | — | `emblem-ok-symbolic` | — |
| `CircleX` | `DialogError` | `dialog-error` | | |
| `Copy` | `ActionCopy` | `edit-copy` | | |
| `Delete` | `ActionDelete` | `edit-delete` | | |
| `File` | `FileGeneric` | `text-x-generic` | | |
| `FolderClosed` | `FolderClosed` | `folder` | | |
| `FolderOpen` | `FolderOpen` | `folder-open` | | |
| `Info` | `DialogInfo` | `dialog-information` | | |
| `Loader` | `StatusLoading` | — | `process-working` | — |
| `Menu` | `NavMenu` | `open-menu` | | |
| `Minus` | `ActionRemove` | `list-remove` | | |
| `Plus` | `ActionAdd` | `list-add` | | |
| `Redo2` | `ActionRedo` | `edit-redo` | | |
| `Search` | `ActionSearch` | `edit-find` | | |
| `Settings` | `ActionSettings` | `preferences-system` | | |
| `TriangleAlert` | `DialogWarning` | `dialog-warning` | | |
| `Undo2` | `ActionUndo` | `edit-undo` | | |
| `User` | `UserAccount` | `system-users` | | |
| `WindowClose` | `WindowClose` | `window-close` | | |
| `WindowMaximize` | `WindowMaximize` | `window-maximize` | | |
| `WindowMinimize` | `WindowMinimize` | `window-minimize` | | |
| `WindowRestore` | `WindowRestore` | `window-restore` | | |

Empty KDE/GNOME cells mean the freedesktop standard name is used by all DEs.

Note: `Bell`/`Notification` has no freedesktop name in the current
`freedesktop_name()` function (returns `None`). The mapping to
`notification-active` is new.

### 58 icons needing new mappings

| gpui `IconName` | fd | KDE | GNOME | Notes |
|---|---|---|---|---|
| `ALargeSmall` | — | `format-font-size-more` | — | |
| `ArrowDown` | — | `go-down-skip` | — | Full arrow (vs chevron) |
| `ArrowLeft` | — | `go-previous-skip` | — | |
| `ArrowRight` | — | `go-next-skip` | — | |
| `ArrowUp` | — | `go-up-skip` | — | |
| `Asterisk` | — | `rating` | — | Star/rating indicator |
| `BookOpen` | `help-contents` | | | |
| `Bot` | `face-smile` | | | No robot/AI icon in any theme |
| `Building2` | — | `applications-office` | — | Category icon, closest match |
| `Calendar` | — | `view-calendar` | — | |
| `CaseSensitive` | — | `format-text-uppercase` | — | |
| `ChartPie` | — | `office-chart-pie` | — | |
| `ChevronsUpDown` | — | `handle-sort` | — | Sort/unfold indicator |
| `CircleUser` | — | `user-identity` | — | |
| `Close` | — | `tab-close` | — | Generic close (vs `window-close`) |
| `Dash` | `list-remove` | | | Same as `Minus` |
| `Ellipsis` | — | `overflow-menu` | `view-more-horizontal` | |
| `EllipsisVertical` | — | `overflow-menu` | `view-more` | |
| `ExternalLink` | — | `external-link` | — | |
| `Eye` | — | `view-visible` | `view-reveal` | |
| `EyeOff` | — | `view-hidden` | `view-conceal` | |
| `Folder` | `folder` | | | Same as `FolderClosed` |
| `Frame` | — | `select-rectangular` | — | |
| `GalleryVerticalEnd` | — | `view-list-icons` | — | |
| `GitHub` | — | `vcs-branch` | — | VCS branch as substitute |
| `Globe` | — | `globe` | — | |
| `Heart` | — | `emblem-favorite` | `starred` | |
| `HeartOff` | `non-starred` | | | Un-favorite semantics |
| `Inbox` | — | `mail-folder-inbox` | — | |
| `Inspector` | — | `code-context` | — | |
| `LayoutDashboard` | `view-grid` | | | |
| `LoaderCircle` | — | `process-working` | — | Same as `Loader` |
| `Map` | `find-location` | | | |
| `Maximize` | `view-fullscreen` | | | UI maximize (vs `window-maximize`) |
| `Minimize` | `window-minimize` | | | |
| `Moon` | `weather-clear-night` | | | Dark mode toggle |
| `Palette` | — | `palette` | — | |
| `PanelBottom` | — | `view-split-top-bottom` | — | |
| `PanelBottomOpen` | — | `view-split-top-bottom` | — | No separate "open" variant |
| `PanelLeft` | — | `sidebar-expand-left` | `sidebar-show` | |
| `PanelLeftClose` | — | `view-left-close` | `sidebar-show` | |
| `PanelLeftOpen` | — | `view-left-new` | `sidebar-show` | |
| `PanelRight` | — | `view-right-new` | `sidebar-show-right` | |
| `PanelRightClose` | — | `view-right-close` | `sidebar-show-right` | |
| `PanelRightOpen` | — | `view-right-new` | `sidebar-show-right` | |
| `Redo` | `edit-redo` | | | Same as `Redo2` |
| `Replace` | `edit-find-replace` | | | |
| `ResizeCorner` | — | `drag-handle` | `list-drag-handle` | |
| `Settings2` | — | `configure` | — | Alternate settings/tuning |
| `SortAscending` | `view-sort-ascending` | | | |
| `SortDescending` | `view-sort-descending` | | | |
| `SquareTerminal` | `utilities-terminal` | | | |
| `Star` | `starred` | | | |
| `StarOff` | `non-starred` | | | |
| `Sun` | `weather-clear` | | | Light mode toggle |
| `ThumbsDown` | — | `rating-unrated` | — | Closest negative feedback icon |
| `ThumbsUp` | — | `approved` | — | |
| `Undo` | `edit-undo` | | | Same as `Undo2` |

Empty cells mean the freedesktop standard name (fd column) is used by that DE.
A dash (—) means no name exists in that convention.

### Coverage expectations by DE

**KDE** (Breeze, breeze-dark): All 58 icons will resolve from the system theme.
Breeze is the most comprehensive icon set with ~4,400 unique names.

**GNOME** (Adwaita): ~20 icons resolve via freedesktop standard names. ~8 more
resolve via GNOME-specific names (view-reveal, view-conceal, view-more,
sidebar-show, list-drag-handle, starred). The remaining ~30 (KDE-specific names
like `code-context`, `vcs-branch`, `format-text-uppercase`, all panel variants
beyond `sidebar-show`) will fall back to bundled Lucide/Material.

**Papirus**: Nearly all icons resolve. Papirus implements both KDE names (in its
colored layers) and GNOME names (in its symbolic layer).

**Cinnamon**: Falls through to Adwaita for most icons. Has unique
`view-left-pane-symbolic` / `view-right-pane-symbolic` for panel toggles.

**XFCE, LXQt, Mate**: Depend on the user's chosen icon theme. If the user
selects Papirus or Breeze, full coverage. If Adwaita, partial coverage with
bundled fallback.

## Licensing Constraints

The native-theme crate is licensed `MIT OR Apache-2.0 OR 0BSD`. Bundling icon
sets with copyleft or attribution-required licenses would impose obligations on
every downstream user.

**Why we cannot bundle Breeze/Adwaita**: Both are `LGPL-3.0`. While the LGPL
artwork clarification states that *using* icons in a GUI is permitted (analogous
to dynamic linking), *distributing* them inside a crate changes the calculus —
the crate itself would carry LGPL obligations, which conflicts with the 0BSD
license option.

**What we can bundle**: Lucide (`ISC`) and Material Symbols (`Apache-2.0`) are
permissive enough. Both are already bundled via cargo features.

**Reading from the installed system theme** avoids all licensing issues — the
icons are already on the user's system, installed and licensed through their
package manager.

## Available Icon Sets (Reference)

### Freedesktop.org-Compliant (Linux Desktop Themes)

| Icon Set | License (SPDX) | Icons (approx.) | Source |
|---|---|---|---|
| [Breeze][breeze] (KDE) | `LGPL-3.0-or-later` | ~7,100 unique | [KDE/breeze-icons][breeze] |
| [Adwaita][adwaita] (GNOME) | `LGPL-3.0-only` OR `CC-BY-SA-3.0` | ~790 | [GNOME/adwaita-icon-theme][adwaita] |
| [Oxygen][oxygen] (KDE) | `LGPL-3.0-or-later` | ~5,300 SVG | [KDE/oxygen-icons][oxygen] |
| [Papirus][papirus] | `GPL-3.0-only` | ~5,000+ unique | [PapirusDevelopmentTeam/papirus-icon-theme][papirus] |
| [Tango][tango] | Public Domain | ~510 SVG | [freedesktop/tango-icon-theme][tango] |
| [elementary][elementary] (Pantheon) | `GPL-3.0-only` | ~2,700 | [elementary/icons][elementary] |
| [Numix][numix] | `GPL-3.0-only` | ~4,500+ unique | [numixproject/numix-icon-theme][numix] |

### General-Purpose Icon Libraries

| Icon Set | License (SPDX) | Icons (approx.) | Source |
|---|---|---|---|
| [Lucide][lucide] | `ISC` | ~1,700 | [lucide-icons/lucide][lucide] |
| [Material Symbols][material] | `Apache-2.0` | ~4,200 | [google/material-design-icons][material] |
| [Phosphor][phosphor] | `MIT` | ~1,500 (x6 weights) | [phosphor-icons/core][phosphor] |
| [Tabler Icons][tabler] | `MIT` | ~5,000 | [tabler/tabler-icons][tabler] |
| [Bootstrap Icons][bootstrap] | `MIT` | ~2,000 | [twbs/icons][bootstrap] |
| [Heroicons][heroicons] | `MIT` | ~320 (x4 styles) | [tailwindlabs/heroicons][heroicons] |
| [Font Awesome Free][fa] | Icons: `CC-BY-4.0` | ~2,000 (free) | [FortAwesome/Font-Awesome][fa] |

### License Compatibility

**Safe to read from system / download at runtime:**

- **Tango** — Public Domain.
- **Lucide, Phosphor, Tabler, Bootstrap, Heroicons** — MIT or ISC.
- **Material Symbols** — Apache-2.0.
- **Adwaita** — Dual-licensed LGPL-3.0 / CC-BY-SA-3.0 (app developer's choice).
- **Breeze, Oxygen** — LGPL-3.0 with artwork clarification (using icons in a GUI
  is permitted; distributing SVG sources satisfies the LGPL requirement).

**Requires caution:**

- **Papirus, elementary, Numix** — GPL-3.0 (strong copyleft). No linking
  exception for artwork. Best suited for GPL-licensed applications.
- **Font Awesome Free** — CC-BY-4.0 for SVG icons (attribution required).

## Runtime Download (Layer 4 — Future)

For cases where the system theme is not installed, a runtime download feature
can fetch a curated subset of icons on first use.

### When needed

- macOS / Windows developer wanting a specific Linux theme's visual style
- Linux user selecting a theme not installed via their package manager
- App developer wanting a complete third-party icon library as their UI language

### How it would work

1. Application enables a cargo feature (e.g., `extra-icons`).
2. On first use, calls `download_icon_set("breeze")` or similar.
3. The crate fetches only the icons matching `IconRole` variants + the 58 extra
   mappings (not the full theme) from a known URL.
4. Icons are cached in a platform-appropriate directory:
   - Linux: `$XDG_DATA_HOME/<app>/icons/`
   - macOS: `~/Library/Application Support/<app>/icons/`
   - Windows: `%LOCALAPPDATA%\<app>\icons\`
5. Subsequent calls load from cache without network access.

### App developer obligations (for downloaded icon sets)

1. **License file** — Ship the icon set's license with the application (the
   download function saves it alongside the icons).
2. **Attribution** — Display credit as required (About dialog or NOTICES file).
3. **User notification** — Inform users that icons will be downloaded on first
   run.
4. **Source availability** — For LGPL icon sets, SVG sources must be available.
   Since icons are stored as SVGs, this is satisfied automatically.

### Implementation considerations

- **Subset extraction**: Only the ~100 mapped icons, not the full theme.
- **Offline fallback**: Fall back to bundled Lucide/Material if download fails.
- **Checksum verification**: Verify archives against known checksums.
- **No network at build time**: Downloads happen at application runtime only.
- **crates.io size limit**: 10 MB per crate — another reason to not bundle.

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
