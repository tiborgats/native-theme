# freedesktop name mapping for gpui-component icons — Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Map all 86 gpui-component `IconName` variants to freedesktop icon names with DE-aware selection, so the GPUI connector can load native-styled icons from the user's installed Linux icon theme.

**Architecture:** Two changes: (1) make `LinuxDesktop` + `detect_linux_de` public in native-theme and add `load_freedesktop_icon_by_name` to look up arbitrary freedesktop names; (2) add `freedesktop_name_for_gpui_icon` in the GPUI connector with per-entry confidence annotations.

**Tech Stack:** Rust, native-theme crate, native-theme-gpui connector, freedesktop-icons crate.

---

### Task 1: Make `LinuxDesktop` and `detect_linux_de` public

**Files:**
- Modify: `native-theme/src/lib.rs:131` (`pub(crate) enum` → `pub enum`)
- Modify: `native-theme/src/lib.rs:148` (`pub(crate) fn` → `pub fn`)

**Step 1: Change visibility**

In `native-theme/src/lib.rs`, line 131:
```rust
// Before:
pub(crate) enum LinuxDesktop {
// After:
pub enum LinuxDesktop {
```

Line 148:
```rust
// Before:
pub(crate) fn detect_linux_de(xdg_current_desktop: &str) -> LinuxDesktop {
// After:
pub fn detect_linux_de(xdg_current_desktop: &str) -> LinuxDesktop {
```

**Step 2: Re-export from the crate root**

Add to the `pub use` block near line 92:
```rust
pub use crate::LinuxDesktop;
pub use crate::detect_linux_de;
```

Note: both are `#[cfg(target_os = "linux")]` — the re-exports need the same gate.

**Step 3: Run tests**

Run: `cargo test -p native-theme`
Expected: PASS (no behavior change, only visibility)

**Step 4: Commit**

```
feat: make LinuxDesktop and detect_linux_de public

The GPUI connector needs DE detection to select the correct
freedesktop icon naming convention (KDE vs GNOME vs generic).
```

---

### Task 2: Add `load_freedesktop_icon_by_name` to native-theme

**Files:**
- Modify: `native-theme/src/freedesktop.rs` (add new public function)
- Modify: `native-theme/src/lib.rs` (re-export)

**Step 1: Write the test**

Add to `native-theme/src/freedesktop.rs` in the existing `mod tests` block:

```rust
#[test]
fn load_icon_by_name_finds_edit_copy() {
    let theme = detect_theme();
    let result = load_freedesktop_icon_by_name("edit-copy", &theme);
    assert!(result.is_some(), "edit-copy should be found in system theme");
    assert!(matches!(result.unwrap(), IconData::Svg(_)));
}

#[test]
fn load_icon_by_name_returns_none_for_nonexistent() {
    let result = load_freedesktop_icon_by_name("zzz-nonexistent-icon", "hicolor");
    assert!(result.is_none());
}
```

**Step 2: Run tests to verify they fail**

Run: `cargo test -p native-theme load_icon_by_name`
Expected: FAIL (function does not exist)

**Step 3: Implement the function**

In `native-theme/src/freedesktop.rs`, add after `load_freedesktop_icon`:

```rust
/// Load a freedesktop icon by name from the given theme.
///
/// Looks up the name in the specified theme directory (with `-symbolic`
/// suffix fallback for Adwaita-style themes), reads the SVG file, and
/// returns it as `IconData::Svg`.
///
/// Unlike [`load_freedesktop_icon`] which takes an `IconRole`, this
/// function takes an arbitrary freedesktop icon name string. This is
/// used by connectors to load toolkit-specific icons beyond the 42
/// `IconRole` variants.
///
/// Returns `None` if the icon is not found in the theme.
pub fn load_freedesktop_icon_by_name(name: &str, theme: &str) -> Option<IconData> {
    let path = find_icon(name, theme, 24)?;
    let bytes = std::fs::read(&path).ok()?;
    Some(IconData::Svg(bytes))
}
```

**Step 4: Re-export from crate root**

In `native-theme/src/lib.rs`, add alongside the existing freedesktop re-export (line 117):

```rust
#[cfg(all(target_os = "linux", feature = "system-icons"))]
pub use freedesktop::load_freedesktop_icon_by_name;
```

**Step 5: Run tests**

Run: `cargo test -p native-theme load_icon_by_name`
Expected: PASS

**Step 6: Commit**

```
feat: add load_freedesktop_icon_by_name for arbitrary icon lookups

Connectors need to load icons by freedesktop name string (not just
IconRole) to support toolkit-specific icons beyond the 42 roles.
```

---

### Task 3: Add `freedesktop_name_for_gpui_icon` to the GPUI connector

**Files:**
- Modify: `connectors/native-theme-gpui/src/icons.rs` (add the mapping function)

This is the main mapping function. Each entry has a confidence comment.
The function takes the detected `LinuxDesktop` to select the right naming
convention when KDE and GNOME diverge.

**Step 1: Write the test**

Add to the bottom of `connectors/native-theme-gpui/src/icons.rs`:

```rust
#[cfg(test)]
mod freedesktop_mapping_tests {
    use super::*;
    use native_theme::LinuxDesktop;

    #[test]
    fn all_86_gpui_icons_have_mapping_on_kde() {
        let all_names = [
            "ALargeSmall", "ArrowDown", "ArrowLeft", "ArrowRight", "ArrowUp",
            "Asterisk", "Bell", "BookOpen", "Bot", "Building2",
            "Calendar", "CaseSensitive", "ChartPie", "Check", "ChevronDown",
            "ChevronLeft", "ChevronRight", "ChevronsUpDown", "ChevronUp",
            "CircleCheck", "CircleUser", "CircleX", "Close", "Copy",
            "Dash", "Delete", "Ellipsis", "EllipsisVertical", "ExternalLink",
            "Eye", "EyeOff", "File", "Folder", "FolderClosed",
            "FolderOpen", "Frame", "GalleryVerticalEnd", "GitHub", "Globe",
            "Heart", "HeartOff", "Inbox", "Info", "Inspector",
            "LayoutDashboard", "Loader", "LoaderCircle", "Map", "Maximize",
            "Menu", "Minimize", "Minus", "Moon", "Palette",
            "PanelBottom", "PanelBottomOpen", "PanelLeft", "PanelLeftClose",
            "PanelLeftOpen", "PanelRight", "PanelRightClose", "PanelRightOpen",
            "Plus", "Redo", "Redo2", "Replace", "ResizeCorner",
            "Search", "Settings", "Settings2", "SortAscending", "SortDescending",
            "SquareTerminal", "Star", "StarOff", "Sun", "ThumbsDown",
            "ThumbsUp", "TriangleAlert", "Undo", "Undo2", "User",
            "WindowClose", "WindowMaximize", "WindowMinimize", "WindowRestore",
        ];
        let mut missing = Vec::new();
        for name in &all_names {
            if freedesktop_name_for_gpui_icon(name, LinuxDesktop::Kde).is_none() {
                missing.push(*name);
            }
        }
        assert!(
            missing.is_empty(),
            "Missing KDE freedesktop mappings for: {:?}",
            missing,
        );
    }

    #[test]
    fn eye_differs_by_de() {
        assert_eq!(
            freedesktop_name_for_gpui_icon("Eye", LinuxDesktop::Kde),
            Some("view-visible"),
        );
        assert_eq!(
            freedesktop_name_for_gpui_icon("Eye", LinuxDesktop::Gnome),
            Some("view-reveal"),
        );
    }

    #[test]
    fn freedesktop_standard_ignores_de() {
        // edit-copy is freedesktop standard — same for all DEs
        assert_eq!(
            freedesktop_name_for_gpui_icon("Copy", LinuxDesktop::Kde),
            freedesktop_name_for_gpui_icon("Copy", LinuxDesktop::Gnome),
        );
    }

    #[test]
    fn unknown_name_returns_none() {
        assert!(freedesktop_name_for_gpui_icon("NotARealIcon", LinuxDesktop::Kde).is_none());
    }
}
```

**Step 2: Run tests to verify they fail**

Run: `cargo test -p native-theme-gpui freedesktop_mapping`
Expected: FAIL (function does not exist)

**Step 3: Implement the function**

Add to `connectors/native-theme-gpui/src/icons.rs`, after `material_name_for_gpui_icon`:

```rust
/// Map a gpui-component icon name to its freedesktop icon name for the
/// given desktop environment.
///
/// Returns the best freedesktop name for the detected DE's naming
/// convention. When KDE and GNOME use different names for the same
/// concept, the DE parameter selects the right one. For freedesktop
/// standard names (present in all themes), the DE is ignored.
///
/// ## Confidence levels
///
/// Each mapping is annotated with a confidence level:
/// - `exact`: the freedesktop icon is semantically identical
/// - `close`: same concept, minor visual difference
/// - `approximate`: best available match, different metaphor
///
/// Covers all 86 gpui-component `IconName` variants.
#[cfg(target_os = "linux")]
pub fn freedesktop_name_for_gpui_icon(
    gpui_name: &str,
    de: native_theme::LinuxDesktop,
) -> Option<&'static str> {
    use native_theme::LinuxDesktop;

    let is_gnome = matches!(de, LinuxDesktop::Gnome | LinuxDesktop::Budgie
        | LinuxDesktop::Cinnamon | LinuxDesktop::Mate);

    Some(match gpui_name {
        // --- Icons with freedesktop standard names (all DEs) ---
        "BookOpen"         => "help-contents",          // close
        "Bot"              => "face-smile",             // approximate
        "ChevronDown"      => "go-down",                // close: full nav arrow, not disclosure chevron
        "ChevronLeft"      => "go-previous",            // close
        "ChevronRight"     => "go-next",                // close
        "ChevronUp"        => "go-up",                  // close
        "CircleX"          => "dialog-error",           // close
        "Copy"             => "edit-copy",              // exact
        "Dash"             => "list-remove",            // exact
        "Delete"           => "edit-delete",            // exact
        "File"             => "text-x-generic",         // exact
        "Folder"           => "folder",                 // exact
        "FolderClosed"     => "folder",                 // exact
        "FolderOpen"       => "folder-open",            // exact
        "HeartOff"         => "non-starred",            // close: un-favorite semantics
        "Info"             => "dialog-information",     // exact
        "LayoutDashboard"  => "view-grid",              // close
        "Map"              => "find-location",          // close
        "Maximize"         => "view-fullscreen",        // exact
        "Menu"             => "open-menu",              // exact
        "Minimize"         => "window-minimize",        // exact
        "Minus"            => "list-remove",            // exact
        "Moon"             => "weather-clear-night",    // close: dark mode toggle
        "Plus"             => "list-add",               // exact
        "Redo"             => "edit-redo",              // exact
        "Redo2"            => "edit-redo",              // exact
        "Replace"          => "edit-find-replace",      // exact
        "Search"           => "edit-find",              // exact
        "Settings"         => "preferences-system",     // exact
        "SortAscending"    => "view-sort-ascending",    // exact
        "SortDescending"   => "view-sort-descending",   // exact
        "SquareTerminal"   => "utilities-terminal",     // close
        "Star"             => "starred",                // exact
        "StarOff"          => "non-starred",            // exact
        "Sun"              => "weather-clear",          // close: light mode toggle
        "TriangleAlert"    => "dialog-warning",         // exact
        "Undo"             => "edit-undo",              // exact
        "Undo2"            => "edit-undo",              // exact
        "User"             => "system-users",           // exact
        "WindowClose"      => "window-close",           // exact
        "WindowMaximize"   => "window-maximize",        // exact
        "WindowMinimize"   => "window-minimize",        // exact
        "WindowRestore"    => "window-restore",         // exact

        // --- Icons where KDE and GNOME diverge ---
        "Ellipsis"         => if is_gnome { "view-more-horizontal" } else { "overflow-menu" },  // exact
        "EllipsisVertical" => if is_gnome { "view-more" } else { "overflow-menu" },             // close: no vertical variant in KDE
        "Eye"              => if is_gnome { "view-reveal" } else { "view-visible" },            // exact
        "EyeOff"           => if is_gnome { "view-conceal" } else { "view-hidden" },            // exact
        "Heart"            => if is_gnome { "starred" } else { "emblem-favorite" },             // close
        "PanelLeft"        => if is_gnome { "sidebar-show" } else { "sidebar-expand-left" },    // close
        "PanelLeftClose"   => if is_gnome { "sidebar-show" } else { "view-left-close" },        // close
        "PanelLeftOpen"    => if is_gnome { "sidebar-show" } else { "view-left-new" },          // close
        "PanelRight"       => if is_gnome { "sidebar-show-right" } else { "view-right-new" },   // close
        "PanelRightClose"  => if is_gnome { "sidebar-show-right" } else { "view-right-close" }, // close
        "PanelRightOpen"   => if is_gnome { "sidebar-show-right" } else { "view-right-new" },   // close
        "ResizeCorner"     => if is_gnome { "list-drag-handle" } else { "drag-handle" },        // close

        // --- KDE-only names (no GNOME equivalent — GNOME falls back to bundled) ---
        "ALargeSmall"      => "format-font-size-more",  // close
        "ArrowDown"        => "go-down-skip",           // close: full arrow vs nav arrow
        "ArrowLeft"        => "go-previous-skip",       // close
        "ArrowRight"       => "go-next-skip",           // close
        "ArrowUp"          => "go-up-skip",             // close
        "Asterisk"         => "rating",                 // approximate
        "Bell"             => "notification-active",    // close
        "Building2"        => "applications-office",    // approximate
        "Calendar"         => "view-calendar",          // exact
        "CaseSensitive"    => "format-text-uppercase",  // close
        "ChartPie"         => "office-chart-pie",       // exact
        "Check"            => "dialog-ok",              // close: checkmark vs OK button
        "ChevronsUpDown"   => "handle-sort",            // close
        "CircleCheck"      => "emblem-ok-symbolic",     // close
        "CircleUser"       => "user-identity",          // close
        "Close"            => "tab-close",              // exact
        "ExternalLink"     => "external-link",          // exact
        "Frame"            => "select-rectangular",     // close
        "GalleryVerticalEnd" => "view-list-icons",      // approximate
        "GitHub"           => "vcs-branch",             // approximate: VCS branch as substitute
        "Globe"            => "globe",                  // exact
        "Inbox"            => "mail-folder-inbox",      // exact
        "Inspector"        => "code-context",           // close
        "Loader"           => "process-working",        // exact
        "LoaderCircle"     => "process-working",        // exact
        "Palette"          => "palette",                // exact
        "PanelBottom"      => "view-split-top-bottom",  // close
        "PanelBottomOpen"  => "view-split-top-bottom",  // close: no separate open variant
        "Settings2"        => "configure",              // exact
        "ThumbsDown"       => "rating-unrated",         // approximate
        "ThumbsUp"         => "approved",               // approximate

        _ => return None,
    })
}
```

**Step 4: Run tests**

Run: `cargo test -p native-theme-gpui freedesktop_mapping`
Expected: PASS

**Step 5: Commit**

```
feat(gpui): add freedesktop_name_for_gpui_icon with DE-aware mapping

Maps all 86 gpui-component IconName variants to freedesktop icon
names. The DE parameter selects between KDE and GNOME naming
conventions where they diverge (view-visible vs view-reveal, etc.).

Each mapping is annotated with confidence: exact, close, or
approximate. See docs/extra-icons.md for the full analysis.
```

---

### Task 4: Verify mappings resolve against installed Breeze theme

**Files:**
- Modify: `connectors/native-theme-gpui/src/icons.rs` (add integration test)

**Step 1: Write the integration test**

Add to the `freedesktop_mapping_tests` module:

```rust
#[test]
#[cfg(feature = "system-icons")]
fn all_kde_names_resolve_in_breeze() {
    let theme = native_theme::system_icon_theme();
    // Only meaningful on a KDE system with Breeze installed
    if !theme.contains("breeze") {
        eprintln!("Skipping: system theme is '{}', not Breeze", theme);
        return;
    }

    let all_names = [
        "ALargeSmall", "ArrowDown", "ArrowLeft", "ArrowRight", "ArrowUp",
        "Asterisk", "Bell", "BookOpen", "Bot", "Building2",
        "Calendar", "CaseSensitive", "ChartPie", "Check", "ChevronDown",
        "ChevronLeft", "ChevronRight", "ChevronsUpDown", "ChevronUp",
        "CircleCheck", "CircleUser", "CircleX", "Close", "Copy",
        "Dash", "Delete", "Ellipsis", "EllipsisVertical", "ExternalLink",
        "Eye", "EyeOff", "File", "Folder", "FolderClosed",
        "FolderOpen", "Frame", "GalleryVerticalEnd", "GitHub", "Globe",
        "Heart", "HeartOff", "Inbox", "Info", "Inspector",
        "LayoutDashboard", "Loader", "LoaderCircle", "Map", "Maximize",
        "Menu", "Minimize", "Minus", "Moon", "Palette",
        "PanelBottom", "PanelBottomOpen", "PanelLeft", "PanelLeftClose",
        "PanelLeftOpen", "PanelRight", "PanelRightClose", "PanelRightOpen",
        "Plus", "Redo", "Redo2", "Replace", "ResizeCorner",
        "Search", "Settings", "Settings2", "SortAscending", "SortDescending",
        "SquareTerminal", "Star", "StarOff", "Sun", "ThumbsDown",
        "ThumbsUp", "TriangleAlert", "Undo", "Undo2", "User",
        "WindowClose", "WindowMaximize", "WindowMinimize", "WindowRestore",
    ];

    let mut missing = Vec::new();
    for name in &all_names {
        let fd_name = freedesktop_name_for_gpui_icon(name, LinuxDesktop::Kde)
            .expect(&format!("{} has no KDE mapping", name));
        if native_theme::load_freedesktop_icon_by_name(fd_name, &theme).is_none() {
            missing.push(format!("{} → {}", name, fd_name));
        }
    }
    assert!(
        missing.is_empty(),
        "These gpui icons did not resolve in Breeze:\n  {}",
        missing.join("\n  "),
    );
}
```

**Step 2: Run the integration test**

Run: `cargo test -p native-theme-gpui all_kde_names_resolve_in_breeze`
Expected: PASS (on KDE system with Breeze) or skip message (on other DEs)

**Step 3: Commit**

```
test(gpui): verify all freedesktop mappings resolve in Breeze

Integration test that checks every gpui→freedesktop mapping
actually finds an SVG in the installed Breeze theme. Skips
gracefully on non-KDE systems.
```

---
