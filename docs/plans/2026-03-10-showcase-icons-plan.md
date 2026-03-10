# Showcase Icon Sets & Inspector Improvements — Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Fix Theme Config Inspector styling, add "default (...)" icon set, split Lucide into two sets, and expand bundled icons to cover all 86 gpui-component IconName variants.

**Architecture:** Expand native-theme's bundled SVG system with a `bundled_icon_by_name(set, name)` function alongside the existing `bundled_icon_svg(set, role)`. Add connector mapping functions (`lucide_name_for_gpui_icon`, `material_name_for_gpui_icon`) in native-theme-gpui. Update the showcase to use the new icon set options.

**Tech Stack:** Rust, gpui, gpui-component 0.5, native-theme (workspace crate)

---

## Task 1: Theme Config Inspector Styling

**Files:**
- Modify: `connectors/native-theme-gpui/examples/showcase.rs:649-698`

**Step 1: Update `render_sidebar()` gap**

Change `gap_3` to `gap_0p5` at line 661:

```rust
// Before:
        v_flex()
            .gap_3()
            .p_3()
            .w_full()

// After:
        v_flex()
            .gap_0p5()
            .p_3()
            .w_full()
```

**Step 2: Update `config_row()` text sizes**

Change both labels from `text_sm` to `text_xs` at lines 695-697:

```rust
// Before:
    fn config_row(&self, label: &str, value: &str) -> impl IntoElement {
        let label_s: SharedString = label.to_string().into();
        let value_s: SharedString = value.to_string().into();
        v_flex()
            .gap_0p5()
            .child(Label::new(label_s).text_sm().font_semibold())
            .child(Label::new(value_s).text_sm())
    }

// After:
    fn config_row(&self, label: &str, value: &str) -> impl IntoElement {
        let label_s: SharedString = label.to_string().into();
        let value_s: SharedString = value.to_string().into();
        v_flex()
            .gap_0p5()
            .child(Label::new(label_s).text_xs().font_semibold())
            .child(Label::new(value_s).text_xs())
    }
```

**Step 3: Build to verify**

Run: `cargo build -p native-theme-gpui --example showcase`
Expected: compiles without errors.

**Step 4: Commit**

```
feat(showcase): match Theme Config Inspector style to Widget Info
```

---

## Task 2: Download and Bundle Latest Lucide SVGs (86 icons)

**Files:**
- Add: 48+ new SVG files to `native-theme/icons/lucide/`
- Replace: 38 existing SVG files with latest Lucide versions

**Step 1: Download the latest Lucide release**

Go to https://github.com/lucide-icons/lucide/releases and download the latest release SVG package. Extract the `icons/` directory.

**Step 2: Copy the 75 standard Lucide SVGs**

Copy these SVG files from the Lucide release into `native-theme/icons/lucide/`, replacing any existing files:

Already existing (update to latest version):
`bell.svg`, `check.svg`, `chevron-down.svg`, `chevron-left.svg`, `chevron-right.svg`, `chevron-up.svg`, `circle-check.svg`, `circle-x.svg`, `clipboard-paste.svg`, `copy.svg`, `file.svg`, `folder-closed.svg`, `folder-open.svg`, `house.svg`, `info.svg`, `loader.svg`, `lock.svg`, `maximize.svg`, `menu.svg`, `minimize-2.svg`, `minimize.svg`, `minus.svg`, `pencil.svg`, `plus.svg`, `printer.svg`, `redo-2.svg`, `refresh-cw.svg`, `save.svg`, `scissors.svg`, `search.svg`, `settings.svg`, `shield.svg`, `trash-2.svg`, `triangle-alert.svg`, `undo-2.svg`, `user.svg`, `x.svg`

New files to add:
`a-large-small.svg`, `arrow-down.svg`, `arrow-left.svg`, `arrow-right.svg`, `arrow-up.svg`, `asterisk.svg`, `book-open.svg`, `bot.svg`, `building-2.svg`, `calendar.svg`, `case-sensitive.svg`, `chart-pie.svg`, `chevrons-up-down.svg`, `circle-user.svg`, `ellipsis.svg`, `ellipsis-vertical.svg`, `external-link.svg`, `eye.svg`, `eye-off.svg`, `folder.svg`, `frame.svg`, `gallery-vertical-end.svg`, `github.svg`, `globe.svg`, `heart.svg`, `heart-off.svg`, `inbox.svg`, `layout-dashboard.svg`, `loader-circle.svg`, `map.svg`, `moon.svg`, `palette.svg`, `panel-bottom.svg`, `panel-bottom-open.svg`, `panel-left.svg`, `panel-left-close.svg`, `panel-left-open.svg`, `panel-right.svg`, `panel-right-close.svg`, `panel-right-open.svg`, `redo.svg`, `replace.svg`, `settings-2.svg`, `square-terminal.svg`, `star.svg`, `star-off.svg`, `sun.svg`, `thumbs-down.svg`, `thumbs-up.svg`, `undo.svg`

**Step 3: Add SVGs for the 11 custom gpui-component icons (closest Lucide equivalents)**

These gpui-component icons don't have exact Lucide names. Copy the closest Lucide match and rename:

| Target filename | Source from Lucide release |
|---|---|
| `close.svg` | Copy from `x.svg` (Lucide's close icon) |
| `dash.svg` | Copy from `minus.svg` |
| `delete.svg` | Copy from `trash-2.svg` |
| `window-close.svg` | Copy from `x.svg` |
| `window-minimize.svg` | Copy from `minus.svg` |
| `window-maximize.svg` | Copy from `maximize.svg` |
| `window-restore.svg` | Copy from `minimize-2.svg` |
| `inspect.svg` | Copy from `scan.svg` (or `search-code.svg`) |
| `resize-corner.svg` | Copy from `grip.svg` |
| `sort-ascending.svg` | Copy from `arrow-up-narrow-wide.svg` |
| `sort-descending.svg` | Copy from `arrow-down-wide-narrow.svg` |

Note: `circle-question-mark.svg` is used by IconRole but not in the 86 gpui-component icons. Keep it as-is (updated to latest).

**Step 4: Verify file count**

Run: `ls native-theme/icons/lucide/*.svg | wc -l`
Expected: 87 or more (75 standard + 11 custom-mapped + `circle-question-mark.svg` for IconRole)

**Step 5: Commit**

```
feat(icons): update bundled Lucide SVGs to latest release and expand to 86+ icons
```

---

## Task 3: Download and Bundle Latest Material SVGs (86 icons)

**Files:**
- Add: 48+ new SVG files to `native-theme/icons/material/`
- Replace: 38 existing SVG files with latest Material Symbols versions

**Step 1: Download Material Symbols SVGs**

Source: Google Material Symbols (https://fonts.google.com/icons). Use the "Outlined" style for consistency. Download the SVGs needed.

**Step 2: Copy existing icons (update to latest)**

Update all 38 existing files in `native-theme/icons/material/` with latest versions.

**Step 3: Add new Material SVGs for the remaining gpui-component icons**

Map each gpui-component IconName to a Material icon name and download:

| gpui-component name | Material icon file | Material icon name |
|---|---|---|
| ALargeSmall | font_size.svg | format_size |
| ArrowDown | arrow_downward.svg | (already exists) |
| ArrowLeft | arrow_back.svg | (already exists) |
| ArrowRight | arrow_forward.svg | (already exists) |
| ArrowUp | arrow_upward.svg | (already exists) |
| Asterisk | asterisk.svg | emergency |
| Bell | notifications.svg | (already exists) |
| BookOpen | book.svg | menu_book |
| Bot | smart_toy.svg | smart_toy |
| Building2 | apartment.svg | apartment |
| Calendar | calendar_today.svg | calendar_today |
| CaseSensitive | match_case.svg | match_case |
| ChartPie | pie_chart.svg | pie_chart |
| Check | check.svg | (already exists) |
| ChevronDown | (use expand_more.svg) | expand_more |
| ChevronLeft | (use chevron_left.svg) | chevron_left |
| ChevronRight | (use chevron_right.svg) | chevron_right |
| ChevronsUpDown | unfold_more.svg | unfold_more |
| ChevronUp | (use expand_less.svg) | expand_less |
| CircleCheck | check_circle.svg | (already exists) |
| CircleUser | account_circle.svg | account_circle |
| CircleX | cancel.svg | cancel |
| Close | close.svg | (already exists) |
| Copy | content_copy.svg | (already exists) |
| Dash | remove.svg | (already exists) |
| Delete | delete.svg | (already exists) |
| Ellipsis | more_horiz.svg | more_horiz |
| EllipsisVertical | more_vert.svg | more_vert |
| ExternalLink | open_in_new.svg | open_in_new |
| Eye | visibility.svg | visibility |
| EyeOff | visibility_off.svg | visibility_off |
| File | description.svg | (already exists) |
| Folder | folder.svg | (already exists) |
| FolderClosed | folder.svg | (reuse folder) |
| FolderOpen | folder_open.svg | (already exists) |
| Frame | crop_free.svg | crop_free |
| GalleryVerticalEnd | view_carousel.svg | view_carousel |
| GitHub | code.svg | code |
| Globe | language.svg | language |
| Heart | favorite.svg | favorite |
| HeartOff | heart_broken.svg | heart_broken |
| Inbox | inbox.svg | inbox |
| Info | info.svg | (already exists) |
| Inspector | developer_mode.svg | developer_mode |
| LayoutDashboard | dashboard.svg | dashboard |
| Loader | progress_activity.svg | (already exists) |
| LoaderCircle | autorenew.svg | autorenew |
| Map | map.svg | map |
| Maximize | open_in_full.svg | (already exists) |
| Menu | menu.svg | (already exists) |
| Minimize | minimize.svg | (already exists) |
| Minus | remove.svg | (already exists) |
| Moon | dark_mode.svg | dark_mode |
| Palette | palette.svg | palette |
| PanelBottom | dock_to_bottom.svg | dock_to_bottom |
| PanelBottomOpen | vertical_split.svg | (use web_asset or similar) |
| PanelLeft | side_navigation.svg | side_navigation |
| PanelLeftClose | left_panel_close.svg | left_panel_close |
| PanelLeftOpen | left_panel_open.svg | left_panel_open |
| PanelRight | right_panel_close.svg | right_panel_close |
| PanelRightClose | right_panel_close.svg | (reuse) |
| PanelRightOpen | right_panel_open.svg | right_panel_open |
| Plus | add.svg | (already exists) |
| Redo | redo.svg | (already exists) |
| Redo2 | redo.svg | (reuse) |
| Replace | find_replace.svg | find_replace |
| ResizeCorner | drag_indicator.svg | drag_indicator |
| Search | search.svg | (already exists) |
| Settings | settings.svg | (already exists) |
| Settings2 | tune.svg | tune |
| SortAscending | arrow_upward.svg | (reuse) |
| SortDescending | arrow_downward.svg | (reuse) |
| SquareTerminal | terminal.svg | terminal |
| Star | star.svg | star |
| StarOff | star_border.svg | star_border |
| Sun | light_mode.svg | light_mode |
| ThumbsDown | thumb_down.svg | thumb_down |
| ThumbsUp | thumb_up.svg | thumb_up |
| TriangleAlert | warning.svg | (already exists) |
| Undo | undo.svg | (already exists) |
| Undo2 | undo.svg | (reuse) |
| User | person.svg | (already exists) |
| WindowClose | close.svg | (reuse) |
| WindowMaximize | open_in_full.svg | (reuse) |
| WindowMinimize | minimize.svg | (reuse) |
| WindowRestore | close_fullscreen.svg | (reuse) |

Note: Some Material icons share SVG files (e.g., Redo and Redo2 both use redo.svg). That's fine — `bundled_icon_by_name` can point two names at the same include_bytes.

**Step 4: Verify file count**

Run: `ls native-theme/icons/material/*.svg | wc -l`
Expected: 60+ unique files (many icons reuse existing files)

**Step 5: Commit**

```
feat(icons): update bundled Material SVGs to latest and expand to 86+ icons
```

---

## Task 4: Add `bundled_icon_by_name()` to native-theme

**Files:**
- Modify: `native-theme/src/model/bundled.rs`
- Modify: `native-theme/src/model/mod.rs:15` (re-export)
- Modify: `native-theme/src/lib.rs:89` (re-export)

**Step 1: Write failing tests**

Add to bottom of `native-theme/src/model/bundled.rs` tests module:

```rust
    #[test]
    #[cfg(feature = "lucide-icons")]
    fn lucide_by_name_covers_gpui_icons() {
        // All 86 gpui-component icon names must resolve to a bundled SVG
        let names = [
            "a-large-small", "arrow-down", "arrow-left", "arrow-right",
            "arrow-up", "asterisk", "bell", "book-open", "bot", "building-2",
            "calendar", "case-sensitive", "chart-pie", "check", "chevron-down",
            "chevron-left", "chevron-right", "chevrons-up-down", "chevron-up",
            "circle-check", "circle-user", "circle-x", "close", "copy", "dash",
            "delete", "ellipsis", "ellipsis-vertical", "external-link", "eye",
            "eye-off", "file", "folder", "folder-closed", "folder-open", "frame",
            "gallery-vertical-end", "github", "globe", "heart", "heart-off",
            "inbox", "info", "inspect", "layout-dashboard", "loader",
            "loader-circle", "map", "maximize", "menu", "minimize", "minus",
            "moon", "palette", "panel-bottom", "panel-bottom-open", "panel-left",
            "panel-left-close", "panel-left-open", "panel-right",
            "panel-right-close", "panel-right-open", "plus", "redo", "redo-2",
            "replace", "resize-corner", "search", "settings", "settings-2",
            "sort-ascending", "sort-descending", "square-terminal", "star",
            "star-off", "sun", "thumbs-down", "thumbs-up", "triangle-alert",
            "undo", "undo-2", "user", "window-close", "window-maximize",
            "window-minimize", "window-restore",
        ];
        for name in names {
            let svg = bundled_icon_by_name(IconSet::Lucide, name);
            assert!(svg.is_some(), "Lucide by-name missing: {}", name);
            let bytes = svg.unwrap();
            let content = std::str::from_utf8(bytes).expect("SVG should be valid UTF-8");
            assert!(content.contains("<svg"), "Lucide {} does not contain <svg tag", name);
        }
    }

    #[test]
    #[cfg(feature = "material-icons")]
    fn material_by_name_covers_gpui_icons() {
        // All 86 gpui-component icon names must resolve to a bundled Material SVG
        let names = [
            "font_size", "arrow_downward", "arrow_back", "arrow_forward",
            "arrow_upward", "emergency", "notifications", "menu_book",
            "smart_toy", "apartment", "calendar_today", "match_case",
            "pie_chart", "check", "expand_more", "chevron_left",
            "chevron_right", "unfold_more", "expand_less", "check_circle",
            "account_circle", "cancel", "close", "content_copy", "remove",
            "delete", "more_horiz", "more_vert", "open_in_new", "visibility",
            "visibility_off", "description", "folder", "folder_open",
            "crop_free", "view_carousel", "code", "language", "favorite",
            "heart_broken", "inbox", "info", "developer_mode", "dashboard",
            "progress_activity", "autorenew", "map", "open_in_full", "menu",
            "minimize", "dark_mode", "palette", "dock_to_bottom",
            "web_asset", "side_navigation", "left_panel_close",
            "left_panel_open", "right_panel_close", "right_panel_open",
            "add", "redo", "find_replace", "drag_indicator", "search",
            "settings", "tune", "terminal", "star", "star_border",
            "light_mode", "thumb_down", "thumb_up", "warning", "undo",
            "person", "close_fullscreen",
        ];
        for name in names {
            let svg = bundled_icon_by_name(IconSet::Material, name);
            assert!(svg.is_some(), "Material by-name missing: {}", name);
            let bytes = svg.unwrap();
            let content = std::str::from_utf8(bytes).expect("SVG should be valid UTF-8");
            assert!(content.contains("<svg"), "Material {} does not contain <svg tag", name);
        }
    }

    #[test]
    fn by_name_non_bundled_sets_return_none() {
        assert!(bundled_icon_by_name(IconSet::SfSymbols, "check").is_none());
        assert!(bundled_icon_by_name(IconSet::Freedesktop, "check").is_none());
        assert!(bundled_icon_by_name(IconSet::SegoeIcons, "check").is_none());
    }

    #[test]
    fn by_name_unknown_name_returns_none() {
        // Even with features enabled, unknown names return None
        assert!(bundled_icon_by_name(IconSet::Lucide, "nonexistent-icon-xyz").is_none());
        assert!(bundled_icon_by_name(IconSet::Material, "nonexistent_icon_xyz").is_none());
    }
```

**Step 2: Run tests to verify they fail**

Run: `cargo test -p native-theme --features material-icons,lucide-icons -- bundled_icon_by_name`
Expected: compilation error (function doesn't exist yet)

**Step 3: Implement `bundled_icon_by_name`**

Add to `native-theme/src/model/bundled.rs` after the existing `bundled_icon_svg` function:

```rust
/// Returns raw SVG bytes for a bundled icon looked up by its canonical name
/// within the icon set.
///
/// Names use each set's canonical format:
/// - Lucide: kebab-case (e.g., `"arrow-down"`, `"circle-check"`)
/// - Material: snake_case (e.g., `"arrow_downward"`, `"check_circle"`)
///
/// Returns `None` for non-bundled sets, disabled features, or unknown names.
///
/// # Examples
///
/// ```
/// use native_theme::{IconSet, bundled_icon_by_name};
///
/// // Without features enabled, bundled icons return None
/// let result = bundled_icon_by_name(IconSet::SfSymbols, "check");
/// assert!(result.is_none());
/// ```
#[allow(unreachable_patterns, unused_variables)]
pub fn bundled_icon_by_name(set: IconSet, name: &str) -> Option<&'static [u8]> {
    match set {
        #[cfg(feature = "material-icons")]
        IconSet::Material => material_svg_by_name(name),

        #[cfg(feature = "lucide-icons")]
        IconSet::Lucide => lucide_svg_by_name(name),

        _ => None,
    }
}
```

Then add the two private match-table functions. For Lucide:

```rust
#[cfg(feature = "lucide-icons")]
#[allow(unreachable_patterns)]
fn lucide_svg_by_name(name: &str) -> Option<&'static [u8]> {
    Some(match name {
        "a-large-small" => include_bytes!("../../icons/lucide/a-large-small.svg"),
        "arrow-down" => include_bytes!("../../icons/lucide/arrow-down.svg"),
        "arrow-left" => include_bytes!("../../icons/lucide/arrow-left.svg"),
        "arrow-right" => include_bytes!("../../icons/lucide/arrow-right.svg"),
        "arrow-up" => include_bytes!("../../icons/lucide/arrow-up.svg"),
        "asterisk" => include_bytes!("../../icons/lucide/asterisk.svg"),
        "bell" => include_bytes!("../../icons/lucide/bell.svg"),
        "book-open" => include_bytes!("../../icons/lucide/book-open.svg"),
        "bot" => include_bytes!("../../icons/lucide/bot.svg"),
        "building-2" => include_bytes!("../../icons/lucide/building-2.svg"),
        "calendar" => include_bytes!("../../icons/lucide/calendar.svg"),
        "case-sensitive" => include_bytes!("../../icons/lucide/case-sensitive.svg"),
        "chart-pie" => include_bytes!("../../icons/lucide/chart-pie.svg"),
        "check" => include_bytes!("../../icons/lucide/check.svg"),
        "chevron-down" => include_bytes!("../../icons/lucide/chevron-down.svg"),
        "chevron-left" => include_bytes!("../../icons/lucide/chevron-left.svg"),
        "chevron-right" => include_bytes!("../../icons/lucide/chevron-right.svg"),
        "chevrons-up-down" => include_bytes!("../../icons/lucide/chevrons-up-down.svg"),
        "chevron-up" => include_bytes!("../../icons/lucide/chevron-up.svg"),
        "circle-check" => include_bytes!("../../icons/lucide/circle-check.svg"),
        "circle-question-mark" => include_bytes!("../../icons/lucide/circle-question-mark.svg"),
        "circle-user" => include_bytes!("../../icons/lucide/circle-user.svg"),
        "circle-x" => include_bytes!("../../icons/lucide/circle-x.svg"),
        "clipboard-paste" => include_bytes!("../../icons/lucide/clipboard-paste.svg"),
        "close" => include_bytes!("../../icons/lucide/close.svg"),
        "copy" => include_bytes!("../../icons/lucide/copy.svg"),
        "dash" => include_bytes!("../../icons/lucide/dash.svg"),
        "delete" => include_bytes!("../../icons/lucide/delete.svg"),
        "ellipsis" => include_bytes!("../../icons/lucide/ellipsis.svg"),
        "ellipsis-vertical" => include_bytes!("../../icons/lucide/ellipsis-vertical.svg"),
        "external-link" => include_bytes!("../../icons/lucide/external-link.svg"),
        "eye" => include_bytes!("../../icons/lucide/eye.svg"),
        "eye-off" => include_bytes!("../../icons/lucide/eye-off.svg"),
        "file" => include_bytes!("../../icons/lucide/file.svg"),
        "folder" => include_bytes!("../../icons/lucide/folder.svg"),
        "folder-closed" => include_bytes!("../../icons/lucide/folder-closed.svg"),
        "folder-open" => include_bytes!("../../icons/lucide/folder-open.svg"),
        "frame" => include_bytes!("../../icons/lucide/frame.svg"),
        "gallery-vertical-end" => include_bytes!("../../icons/lucide/gallery-vertical-end.svg"),
        "github" => include_bytes!("../../icons/lucide/github.svg"),
        "globe" => include_bytes!("../../icons/lucide/globe.svg"),
        "heart" => include_bytes!("../../icons/lucide/heart.svg"),
        "heart-off" => include_bytes!("../../icons/lucide/heart-off.svg"),
        "house" => include_bytes!("../../icons/lucide/house.svg"),
        "inbox" => include_bytes!("../../icons/lucide/inbox.svg"),
        "info" => include_bytes!("../../icons/lucide/info.svg"),
        "inspect" => include_bytes!("../../icons/lucide/inspect.svg"),
        "layout-dashboard" => include_bytes!("../../icons/lucide/layout-dashboard.svg"),
        "loader" => include_bytes!("../../icons/lucide/loader.svg"),
        "loader-circle" => include_bytes!("../../icons/lucide/loader-circle.svg"),
        "lock" => include_bytes!("../../icons/lucide/lock.svg"),
        "map" => include_bytes!("../../icons/lucide/map.svg"),
        "maximize" => include_bytes!("../../icons/lucide/maximize.svg"),
        "menu" => include_bytes!("../../icons/lucide/menu.svg"),
        "minimize" => include_bytes!("../../icons/lucide/minimize.svg"),
        "minimize-2" => include_bytes!("../../icons/lucide/minimize-2.svg"),
        "minus" => include_bytes!("../../icons/lucide/minus.svg"),
        "moon" => include_bytes!("../../icons/lucide/moon.svg"),
        "palette" => include_bytes!("../../icons/lucide/palette.svg"),
        "panel-bottom" => include_bytes!("../../icons/lucide/panel-bottom.svg"),
        "panel-bottom-open" => include_bytes!("../../icons/lucide/panel-bottom-open.svg"),
        "panel-left" => include_bytes!("../../icons/lucide/panel-left.svg"),
        "panel-left-close" => include_bytes!("../../icons/lucide/panel-left-close.svg"),
        "panel-left-open" => include_bytes!("../../icons/lucide/panel-left-open.svg"),
        "panel-right" => include_bytes!("../../icons/lucide/panel-right.svg"),
        "panel-right-close" => include_bytes!("../../icons/lucide/panel-right-close.svg"),
        "panel-right-open" => include_bytes!("../../icons/lucide/panel-right-open.svg"),
        "pencil" => include_bytes!("../../icons/lucide/pencil.svg"),
        "plus" => include_bytes!("../../icons/lucide/plus.svg"),
        "printer" => include_bytes!("../../icons/lucide/printer.svg"),
        "redo" => include_bytes!("../../icons/lucide/redo.svg"),
        "redo-2" => include_bytes!("../../icons/lucide/redo-2.svg"),
        "refresh-cw" => include_bytes!("../../icons/lucide/refresh-cw.svg"),
        "replace" => include_bytes!("../../icons/lucide/replace.svg"),
        "resize-corner" => include_bytes!("../../icons/lucide/resize-corner.svg"),
        "save" => include_bytes!("../../icons/lucide/save.svg"),
        "scissors" => include_bytes!("../../icons/lucide/scissors.svg"),
        "search" => include_bytes!("../../icons/lucide/search.svg"),
        "settings" => include_bytes!("../../icons/lucide/settings.svg"),
        "settings-2" => include_bytes!("../../icons/lucide/settings-2.svg"),
        "shield" => include_bytes!("../../icons/lucide/shield.svg"),
        "sort-ascending" => include_bytes!("../../icons/lucide/sort-ascending.svg"),
        "sort-descending" => include_bytes!("../../icons/lucide/sort-descending.svg"),
        "square-terminal" => include_bytes!("../../icons/lucide/square-terminal.svg"),
        "star" => include_bytes!("../../icons/lucide/star.svg"),
        "star-off" => include_bytes!("../../icons/lucide/star-off.svg"),
        "sun" => include_bytes!("../../icons/lucide/sun.svg"),
        "thumbs-down" => include_bytes!("../../icons/lucide/thumbs-down.svg"),
        "thumbs-up" => include_bytes!("../../icons/lucide/thumbs-up.svg"),
        "trash-2" => include_bytes!("../../icons/lucide/trash-2.svg"),
        "triangle-alert" => include_bytes!("../../icons/lucide/triangle-alert.svg"),
        "undo" => include_bytes!("../../icons/lucide/undo.svg"),
        "undo-2" => include_bytes!("../../icons/lucide/undo-2.svg"),
        "user" => include_bytes!("../../icons/lucide/user.svg"),
        "window-close" => include_bytes!("../../icons/lucide/window-close.svg"),
        "window-maximize" => include_bytes!("../../icons/lucide/window-maximize.svg"),
        "window-minimize" => include_bytes!("../../icons/lucide/window-minimize.svg"),
        "window-restore" => include_bytes!("../../icons/lucide/window-restore.svg"),
        "x" => include_bytes!("../../icons/lucide/x.svg"),
        _ => return None,
    })
}
```

For Material, same pattern with `material_svg_by_name(name)` using Material icon names and file paths. (Full match table analogous to Lucide table above, using the Material name → file mappings from Task 3.)

**Step 4: Add re-exports**

In `native-theme/src/model/mod.rs`, add after line 15:
```rust
pub use bundled::bundled_icon_by_name;
```

In `native-theme/src/lib.rs`, add `bundled_icon_by_name` to the re-export at line 89:
```rust
pub use model::{
    IconData, IconRole, IconSet, NativeTheme, ThemeColors, ThemeFonts, ThemeGeometry, ThemeSpacing,
    ThemeVariant, WidgetMetrics, bundled_icon_svg, bundled_icon_by_name,
};
```

**Step 5: Update size budget tests**

In the tests module of `bundled.rs`, update the size assertions:
- Lucide: change `100 * 1024` to `200 * 1024`
- Material: change `200 * 1024` to `400 * 1024`

**Step 6: Run tests**

Run: `cargo test -p native-theme --features material-icons,lucide-icons -- bundled`
Expected: all tests pass

**Step 7: Commit**

```
feat(icons): add bundled_icon_by_name() for name-based icon lookup
```

---

## Task 5: Add Connector Mapping Functions

**Files:**
- Modify: `connectors/native-theme-gpui/src/icons.rs`

**Step 1: Add `lucide_name_for_gpui_icon` function**

Add after the existing `icon_name()` function:

```rust
/// Map a gpui-component icon name string to its canonical Lucide icon name.
///
/// Returns the kebab-case Lucide name for use with
/// [`native_theme::bundled_icon_by_name`].
///
/// Covers all 86 gpui-component `IconName` variants.
pub fn lucide_name_for_gpui_icon(gpui_name: &str) -> Option<&'static str> {
    Some(match gpui_name {
        "ALargeSmall" => "a-large-small",
        "ArrowDown" => "arrow-down",
        "ArrowLeft" => "arrow-left",
        "ArrowRight" => "arrow-right",
        "ArrowUp" => "arrow-up",
        "Asterisk" => "asterisk",
        "Bell" => "bell",
        "BookOpen" => "book-open",
        "Bot" => "bot",
        "Building2" => "building-2",
        "Calendar" => "calendar",
        "CaseSensitive" => "case-sensitive",
        "ChartPie" => "chart-pie",
        "Check" => "check",
        "ChevronDown" => "chevron-down",
        "ChevronLeft" => "chevron-left",
        "ChevronRight" => "chevron-right",
        "ChevronsUpDown" => "chevrons-up-down",
        "ChevronUp" => "chevron-up",
        "CircleCheck" => "circle-check",
        "CircleUser" => "circle-user",
        "CircleX" => "circle-x",
        "Close" => "close",
        "Copy" => "copy",
        "Dash" => "dash",
        "Delete" => "delete",
        "Ellipsis" => "ellipsis",
        "EllipsisVertical" => "ellipsis-vertical",
        "ExternalLink" => "external-link",
        "Eye" => "eye",
        "EyeOff" => "eye-off",
        "File" => "file",
        "Folder" => "folder",
        "FolderClosed" => "folder-closed",
        "FolderOpen" => "folder-open",
        "Frame" => "frame",
        "GalleryVerticalEnd" => "gallery-vertical-end",
        "GitHub" => "github",
        "Globe" => "globe",
        "Heart" => "heart",
        "HeartOff" => "heart-off",
        "Inbox" => "inbox",
        "Info" => "info",
        "Inspector" => "inspect",
        "LayoutDashboard" => "layout-dashboard",
        "Loader" => "loader",
        "LoaderCircle" => "loader-circle",
        "Map" => "map",
        "Maximize" => "maximize",
        "Menu" => "menu",
        "Minimize" => "minimize",
        "Minus" => "minus",
        "Moon" => "moon",
        "Palette" => "palette",
        "PanelBottom" => "panel-bottom",
        "PanelBottomOpen" => "panel-bottom-open",
        "PanelLeft" => "panel-left",
        "PanelLeftClose" => "panel-left-close",
        "PanelLeftOpen" => "panel-left-open",
        "PanelRight" => "panel-right",
        "PanelRightClose" => "panel-right-close",
        "PanelRightOpen" => "panel-right-open",
        "Plus" => "plus",
        "Redo" => "redo",
        "Redo2" => "redo-2",
        "Replace" => "replace",
        "ResizeCorner" => "resize-corner",
        "Search" => "search",
        "Settings" => "settings",
        "Settings2" => "settings-2",
        "SortAscending" => "sort-ascending",
        "SortDescending" => "sort-descending",
        "SquareTerminal" => "square-terminal",
        "Star" => "star",
        "StarOff" => "star-off",
        "Sun" => "sun",
        "ThumbsDown" => "thumbs-down",
        "ThumbsUp" => "thumbs-up",
        "TriangleAlert" => "triangle-alert",
        "Undo" => "undo",
        "Undo2" => "undo-2",
        "User" => "user",
        "WindowClose" => "window-close",
        "WindowMaximize" => "window-maximize",
        "WindowMinimize" => "window-minimize",
        "WindowRestore" => "window-restore",
        _ => return None,
    })
}
```

**Step 2: Add `material_name_for_gpui_icon` function**

Same pattern, mapping to Material icon names (snake_case):

```rust
/// Map a gpui-component icon name string to its canonical Material icon name.
///
/// Returns the snake_case Material Symbols name for use with
/// [`native_theme::bundled_icon_by_name`].
///
/// Covers all 86 gpui-component `IconName` variants.
pub fn material_name_for_gpui_icon(gpui_name: &str) -> Option<&'static str> {
    Some(match gpui_name {
        "ALargeSmall" => "font_size",
        "ArrowDown" => "arrow_downward",
        "ArrowLeft" => "arrow_back",
        "ArrowRight" => "arrow_forward",
        "ArrowUp" => "arrow_upward",
        "Asterisk" => "emergency",
        "Bell" => "notifications",
        "BookOpen" => "menu_book",
        "Bot" => "smart_toy",
        "Building2" => "apartment",
        "Calendar" => "calendar_today",
        "CaseSensitive" => "match_case",
        "ChartPie" => "pie_chart",
        "Check" => "check",
        "ChevronDown" => "expand_more",
        "ChevronLeft" => "chevron_left",
        "ChevronRight" => "chevron_right",
        "ChevronsUpDown" => "unfold_more",
        "ChevronUp" => "expand_less",
        "CircleCheck" => "check_circle",
        "CircleUser" => "account_circle",
        "CircleX" => "cancel",
        "Close" => "close",
        "Copy" => "content_copy",
        "Dash" => "remove",
        "Delete" => "delete",
        "Ellipsis" => "more_horiz",
        "EllipsisVertical" => "more_vert",
        "ExternalLink" => "open_in_new",
        "Eye" => "visibility",
        "EyeOff" => "visibility_off",
        "File" => "description",
        "Folder" => "folder",
        "FolderClosed" => "folder",
        "FolderOpen" => "folder_open",
        "Frame" => "crop_free",
        "GalleryVerticalEnd" => "view_carousel",
        "GitHub" => "code",
        "Globe" => "language",
        "Heart" => "favorite",
        "HeartOff" => "heart_broken",
        "Inbox" => "inbox",
        "Info" => "info",
        "Inspector" => "developer_mode",
        "LayoutDashboard" => "dashboard",
        "Loader" => "progress_activity",
        "LoaderCircle" => "autorenew",
        "Map" => "map",
        "Maximize" => "open_in_full",
        "Menu" => "menu",
        "Minimize" => "minimize",
        "Minus" => "remove",
        "Moon" => "dark_mode",
        "Palette" => "palette",
        "PanelBottom" => "dock_to_bottom",
        "PanelBottomOpen" => "web_asset",
        "PanelLeft" => "side_navigation",
        "PanelLeftClose" => "left_panel_close",
        "PanelLeftOpen" => "left_panel_open",
        "PanelRight" => "right_panel_close",
        "PanelRightClose" => "right_panel_close",
        "PanelRightOpen" => "right_panel_open",
        "Plus" => "add",
        "Redo" => "redo",
        "Redo2" => "redo",
        "Replace" => "find_replace",
        "ResizeCorner" => "drag_indicator",
        "Search" => "search",
        "Settings" => "settings",
        "Settings2" => "tune",
        "SortAscending" => "arrow_upward",
        "SortDescending" => "arrow_downward",
        "SquareTerminal" => "terminal",
        "Star" => "star",
        "StarOff" => "star_border",
        "Sun" => "light_mode",
        "ThumbsDown" => "thumb_down",
        "ThumbsUp" => "thumb_up",
        "TriangleAlert" => "warning",
        "Undo" => "undo",
        "Undo2" => "undo",
        "User" => "person",
        "WindowClose" => "close",
        "WindowMaximize" => "open_in_full",
        "WindowMinimize" => "minimize",
        "WindowRestore" => "close_fullscreen",
        _ => return None,
    })
}
```

**Step 3: Build**

Run: `cargo build -p native-theme-gpui`
Expected: compiles

**Step 4: Commit**

```
feat(gpui): add lucide/material name mapping for all 86 gpui-component icons
```

---

## Task 6: Showcase — "default (...)" Icon Set + Two Lucide Sets

**Files:**
- Modify: `connectors/native-theme-gpui/examples/showcase.rs`

This is the largest showcase change. It modifies: imports, struct fields, icon set selector construction, theme change handler, icon loading, and icon rendering.

**Step 1: Add imports**

Add to the imports at the top of showcase.rs:

```rust
use native_theme::{icon_name as native_icon_name, load_icon, IconData, IconRole, IconSet, NativeTheme, system_icon_set, bundled_icon_by_name};
use native_theme_gpui::icons::{to_image_source, lucide_name_for_gpui_icon, material_name_for_gpui_icon};
```

(Replace the existing `native_theme` and `native_theme_gpui::icons` import lines.)

**Step 2: Add `use_default_icon_set` field and theme variant tracking**

Add to the `Showcase` struct:

```rust
    /// Whether the icon set follows the theme's default.
    use_default_icon_set: bool,
    /// The current theme's variant (for reading icon_theme).
    current_variant_icon_theme: Option<String>,
```

**Step 3: Add helper to resolve default icon set name**

Add method to `impl Showcase`:

```rust
    /// Resolve the effective icon set name for the current theme.
    fn resolve_default_icon_set(&self) -> String {
        self.current_variant_icon_theme
            .as_deref()
            .unwrap_or(system_icon_set().name())
            .to_string()
    }
```

**Step 4: Add helper to build icon set selector items**

```rust
    fn icon_set_names(&self) -> Vec<SharedString> {
        let default_resolved = self.resolve_default_icon_set();
        let mut names: Vec<SharedString> = vec![
            format!("default ({})", default_resolved).into(),
            "gpui-component built-in (Lucide)".into(),
            "Lucide (bundled)".into(),
            "material".into(),
        ];
        #[cfg(target_os = "linux")]
        names.push("freedesktop".into());
        #[cfg(target_os = "macos")]
        names.push("sf-symbols".into());
        #[cfg(target_os = "windows")]
        names.push("segoe-fluent".into());
        names
    }
```

**Step 5: Add helper to convert selector display name to internal name**

```rust
    /// Convert a display name from the icon set selector to the internal icon set name.
    fn icon_set_internal_name(display: &str) -> String {
        if display.starts_with("default (") {
            // "default" mode — actual set resolved elsewhere
            "default".to_string()
        } else if display == "gpui-component built-in (Lucide)" {
            "gpui-builtin".to_string()
        } else if display == "Lucide (bundled)" {
            "lucide".to_string()
        } else {
            display.to_string()
        }
    }
```

**Step 6: Rewrite `load_all_icons` to handle "gpui-builtin"**

Update the `load_all_icons` function. When `icon_set == "gpui-builtin"`, use `native_theme_gpui::icons::icon_name(role)` to get an `IconName` and render via `Icon::new()` later. For "gpui-builtin", the loaded_icons vec stores `None` for data (rendering is done differently in the render function).

Actually, simpler: `load_all_icons("gpui-builtin")` returns `IconSource::Bundled` with `None` data for roles that have a gpui-component mapping, and `IconSource::NotFound` with `None` data for roles that don't. The render function checks `icon_set_name == "gpui-builtin"` and uses `Icon::new()` instead of `img()`.

**Step 7: Rewrite `load_gpui_icons` to handle "gpui-builtin" and by-name lookup**

For "gpui-builtin": all icons get `data = None, source = Bundled` (rendered via `Icon::new()` in the render function).

For "lucide" or "material": use `bundled_icon_by_name()` with the appropriate name mapping function.

```rust
fn load_gpui_icons(
    icon_set: &str,
) -> Vec<(&'static str, IconName, Option<IconRole>, Option<IconData>, IconSource)> {
    if icon_set == "gpui-builtin" {
        // All icons rendered from gpui-component built-in; no native-theme data loaded
        return GPUI_ICONS
            .iter()
            .map(|(name, icon)| {
                let role = role_for_gpui_icon(name);
                (*name, icon.clone(), role, None, IconSource::Bundled)
            })
            .collect();
    }

    let is_system_set = matches!(icon_set, "freedesktop" | "sf-symbols" | "segoe-fluent");

    // Determine which name-mapping function to use for by-name lookups
    let icon_set_enum = IconSet::from_name(icon_set);

    GPUI_ICONS
        .iter()
        .map(|(name, icon)| {
            let role = role_for_gpui_icon(name);

            // Try loading by IconRole first (existing path)
            if let Some(r) = role {
                let data = load_icon(r, icon_set);
                let source = match &data {
                    None => IconSource::NotFound,
                    Some(_) if !is_system_set => IconSource::Bundled,
                    Some(IconData::Svg(loaded)) => {
                        let mat = load_icon(r, "material");
                        if let Some(IconData::Svg(mat_bytes)) = &mat {
                            if loaded == mat_bytes { IconSource::Fallback } else { IconSource::System }
                        } else { IconSource::System }
                    }
                    Some(_) => IconSource::System,
                };
                return (*name, icon.clone(), Some(r), data, source);
            }

            // No IconRole mapping — try by-name lookup
            if let Some(set) = icon_set_enum {
                let lookup_name = match set {
                    IconSet::Lucide => lucide_name_for_gpui_icon(name),
                    IconSet::Material => material_name_for_gpui_icon(name),
                    _ => None, // System sets: no by-name lookup available
                };
                if let Some(lname) = lookup_name {
                    if let Some(svg_bytes) = bundled_icon_by_name(set, lname) {
                        let data = Some(IconData::Svg(svg_bytes.to_vec()));
                        return (*name, icon.clone(), None, data, IconSource::Bundled);
                    }
                }
            }

            // Fallback: no icon data
            (*name, icon.clone(), None, None, IconSource::NotFound)
        })
        .collect()
}
```

**Step 8: Update icon set selector initialization in `Showcase::new`**

Replace the icon set selector construction (lines 520-554) to use the new `icon_set_names()` helper and set `use_default_icon_set = true` initially. The initial icon set resolves from the default theme's `icon_theme` field.

**Step 9: Update icon set selection handler**

```rust
cx.subscribe_in(
    &icon_set_select,
    window,
    |this: &mut Self, _entity, event: &SelectEvent<SearchableVec<SharedString>>, _window, cx| {
        if let SelectEvent::Confirm(Some(value)) = event {
            let display = value.to_string();
            let internal = Self::icon_set_internal_name(&display);
            this.use_default_icon_set = internal == "default";
            let effective = if this.use_default_icon_set {
                this.resolve_default_icon_set()
            } else {
                internal
            };
            this.icon_set_name = effective.clone();
            this.loaded_icons = load_all_icons(&effective);
            this.gpui_icons = load_gpui_icons(&effective);
            cx.notify();
        }
    },
)
.detach();
```

**Step 10: Update `apply_theme_by_name` to reload icons when in default mode**

After the existing theme application logic, add:

```rust
    // Track the current theme's icon_theme for default resolution
    self.current_variant_icon_theme = variant.icon_theme.clone();

    // If using default icon set, reload icons for the new theme's default
    if self.use_default_icon_set {
        let effective = self.resolve_default_icon_set();
        self.icon_set_name = effective.clone();
        self.loaded_icons = load_all_icons(&effective);
        self.gpui_icons = load_gpui_icons(&effective);
        // Rebuild icon set selector to update the "default (...)" label
        // (rebuild the SelectState delegate with new names)
    }
```

The selector label rebuild requires creating a new `SearchableVec` and updating the `icon_set_select` entity. Add a helper method `rebuild_icon_set_selector` that does this.

**Step 11: Update icon rendering in `render_icons_tab` for "gpui-builtin"**

In the native icons section, when `self.icon_set_name == "gpui-builtin"`, render using `Icon::new()` instead of `img()`:

```rust
let icon_element = if self.icon_set_name == "gpui-builtin" {
    // Use gpui-component's built-in Lucide icon
    if let Some(icon_name) = native_theme_gpui::icons::icon_name(*role) {
        div().child(Icon::new(icon_name).with_size(Size::Medium))
    } else {
        // No gpui-component mapping for this role — gray placeholder
        div()
            .w(px(20.0))
            .h(px(20.0))
            .bg(gpui::hsla(0.0, 0.0, 0.5, 0.2))
            .rounded(px(2.0))
    }
} else if let Some(icon_data) = data {
    // ... existing img() rendering
```

In the gpui-component icons section, when `self.icon_set_name == "gpui-builtin"`, always use `Icon::new()`:

```rust
let icon_element = if self.icon_set_name == "gpui-builtin" {
    div().child(Icon::new(icon.clone()).with_size(Size::Medium))
} else if let Some(icon_data) = data {
    let img_source = to_image_source(icon_data);
    div().child(gpui::img(img_source).w(px(20.0)).h(px(20.0)))
} else {
    // Gray placeholder for missing icons
    div()
        .w(px(20.0))
        .h(px(20.0))
        .bg(gpui::hsla(0.0, 0.0, 0.5, 0.2))
        .rounded(px(2.0))
};
```

**Step 12: Initialize new fields in `Showcase::new`**

Set `use_default_icon_set: true` and `current_variant_icon_theme` from the initial theme preset:

```rust
let current_variant_icon_theme = pick_variant(&nt, is_dark)
    .and_then(|v| v.icon_theme.clone());
```

And use the resolved default as the initial icon set instead of hardcoded `"material"`.

**Step 13: Build and test**

Run: `cargo build -p native-theme-gpui --example showcase`
Expected: compiles without errors.

Run: `cargo run -p native-theme-gpui --example showcase`
Expected: showcase opens. Verify:
- Theme Config Inspector uses same small font as Widget Info
- Icon set selector shows "default (...)" at top
- "gpui-component built-in (Lucide)" shows all icons via gpui-component
- "Lucide (bundled)" shows all icons from native-theme's bundled SVGs
- "material" shows all icons from native-theme's bundled Material SVGs
- Switching themes while "default" is selected changes the icon set

**Step 14: Commit**

```
feat(showcase): add default icon set, split Lucide into two sets, complete icon coverage
```

---

## Task Order & Dependencies

```
Task 1 (inspector styling)     — independent, do first
Task 2 (Lucide SVG download)   — independent, do in parallel with Task 1
Task 3 (Material SVG download) — independent, do in parallel
Task 4 (bundled_icon_by_name)  — depends on Tasks 2+3 (needs SVG files)
Task 5 (connector mappings)    — depends on Task 4 (uses the function)
Task 6 (showcase changes)      — depends on Tasks 4+5
```

Tasks 1, 2, 3 can all be done in parallel. Tasks 4 and 5 are sequential. Task 6 is last.
