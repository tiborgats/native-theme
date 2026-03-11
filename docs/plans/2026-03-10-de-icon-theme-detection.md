# DE-Aware Icon Theme Detection — Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Rename `icon_theme` → `icon_set` in the model, add cross-platform `system_icon_theme()` that detects the actual icon theme per desktop environment, and update the showcase to display it.

**Architecture:** The existing `ThemeVariant::icon_theme` field stores the icon *naming convention* ("freedesktop", "sf-symbols") but is misleadingly named. Rename it to `icon_set` (with serde alias for TOML backward compat). Add a new public function `system_icon_theme()` that detects the actual installed icon theme at runtime (e.g., "breeze-dark" on KDE, "Adwaita" on GNOME). On macOS/Windows where there's no user-configurable theme, it returns the icon set name. The Linux implementation uses `detect_linux_de()` to dispatch to the correct detection method per DE.

**Tech Stack:** Rust, configparser (for INI files), serde (alias), freedesktop-icons crate

---

## Task 1: Rename `ThemeVariant::icon_theme` → `icon_set`

**Files:**
- Modify: `native-theme/src/model/mod.rs:64-98` (struct field, merge, is_empty)
- Modify: `native-theme/src/model/mod.rs:413-464` (tests)

**Step 1: Rename the struct field with serde alias**

In `native-theme/src/model/mod.rs`, change the `icon_theme` field:

```rust
    /// Icon set / naming convention for this variant (e.g., "sf-symbols", "freedesktop").
    /// When None, resolved at runtime via system_icon_set().
    #[serde(default, skip_serializing_if = "Option::is_none", alias = "icon_theme")]
    pub icon_set: Option<String>,
```

Update `merge()` (line 86-88):
```rust
        if overlay.icon_set.is_some() {
            self.icon_set.clone_from(&overlay.icon_set);
        }
```

Update `is_empty()` (line 98):
```rust
            && self.icon_set.is_none()
```

**Step 2: Update the tests in the same file**

Rename all `icon_theme` references in tests (lines 413-464) to `icon_set`. The test for TOML round-trip should verify both `icon_set` (new) and `icon_theme` (alias) work:

```rust
    // === icon_set tests ===

    #[test]
    fn icon_set_default_is_none() {
        assert!(ThemeVariant::default().icon_set.is_none());
    }

    #[test]
    fn icon_set_merge_overlay() {
        let mut base = ThemeVariant::default();
        let mut overlay = ThemeVariant::default();
        overlay.icon_set = Some("material".into());
        base.merge(&overlay);
        assert_eq!(base.icon_set.as_deref(), Some("material"));
    }

    #[test]
    fn icon_set_merge_none_preserves() {
        let mut base = ThemeVariant::default();
        base.icon_set = Some("sf-symbols".into());
        let overlay = ThemeVariant::default();
        base.merge(&overlay);
        assert_eq!(base.icon_set.as_deref(), Some("sf-symbols"));
    }

    #[test]
    fn icon_set_is_empty_when_set() {
        let mut v = ThemeVariant::default();
        assert!(v.is_empty());
        v.icon_set = Some("material".into());
        assert!(!v.is_empty());
    }

    #[test]
    fn icon_set_toml_round_trip() {
        let mut variant = ThemeVariant::default();
        variant.icon_set = Some("material".into());
        let toml_str = toml::to_string(&variant).unwrap();
        assert!(toml_str.contains("icon_set"));
        let deserialized: ThemeVariant = toml::from_str(&toml_str).unwrap();
        assert_eq!(deserialized.icon_set.as_deref(), Some("material"));
    }

    #[test]
    fn icon_set_toml_alias_backward_compat() {
        // Old TOML files use "icon_theme" — verify the serde alias works
        let toml_str = r#"icon_theme = "freedesktop""#;
        let variant: ThemeVariant = toml::from_str(toml_str).unwrap();
        assert_eq!(variant.icon_set.as_deref(), Some("freedesktop"));
    }

    #[test]
    fn icon_set_toml_absent_deserializes_to_none() {
        let toml_str = r##"
[colors]
accent = "#ff0000"
"##;
        let variant: ThemeVariant = toml::from_str(toml_str).unwrap();
        assert!(variant.icon_set.is_none());
    }
```

**Step 3: Run tests to verify**

Run: `cargo test -p native-theme -- icon_set`
Expected: All new tests PASS.

**Step 4: Commit**

```
feat(model): rename icon_theme field to icon_set with serde alias
```

---

## Task 2: Rename `icon_theme` → `icon_set` in all TOML presets

**Files:**
- Modify: All 6 TOML presets that have `icon_theme`:
  - `native-theme/src/presets/kde-breeze.toml` (lines 8, 132)
  - `native-theme/src/presets/adwaita.toml` (lines 8, 128)
  - `native-theme/src/presets/windows-11.toml` (lines 8, 126)
  - `native-theme/src/presets/macos-sonoma.toml` (lines 8, 126)
  - `native-theme/src/presets/ios.toml` (lines 8, 126)
  - `native-theme/src/presets/material.toml` (lines 8, 128)

**Step 1: Replace `icon_theme` with `icon_set` in all presets**

In each file, change `icon_theme = "..."` to `icon_set = "..."`. Both `[light]` and `[dark]` sections.

**Step 2: Update preset tests**

In `native-theme/src/presets.rs` (lines 239-296), rename all `icon_theme` references to `icon_set`:

```rust
    // === icon_set preset tests ===

    #[test]
    fn icon_set_native_presets_have_correct_values() {
        let cases: &[(&str, &str)] = &[
            ("windows-11", "segoe-fluent"),
            ("macos-sonoma", "sf-symbols"),
            ("ios", "sf-symbols"),
            ("adwaita", "freedesktop"),
            ("kde-breeze", "freedesktop"),
            ("material", "material"),
        ];
        for (name, expected) in cases {
            let theme = preset(name).unwrap();
            let light = theme.light.as_ref().unwrap();
            assert_eq!(
                light.icon_set.as_deref(),
                Some(*expected),
                "preset '{name}' light.icon_set should be Some(\"{expected}\")"
            );
            let dark = theme.dark.as_ref().unwrap();
            assert_eq!(
                dark.icon_set.as_deref(),
                Some(*expected),
                "preset '{name}' dark.icon_set should be Some(\"{expected}\")"
            );
        }
    }

    #[test]
    fn icon_set_community_presets_are_none() {
        let community = &[
            "catppuccin-latte",
            "catppuccin-frappe",
            "catppuccin-macchiato",
            "catppuccin-mocha",
            "nord",
            "dracula",
            "gruvbox",
            "solarized",
            "tokyo-night",
            "one-dark",
            "default",
        ];
        for name in community {
            let theme = preset(name).unwrap();
            let light = theme.light.as_ref().unwrap();
            assert!(
                light.icon_set.is_none(),
                "preset '{name}' light.icon_set should be None"
            );
            let dark = theme.dark.as_ref().unwrap();
            assert!(
                dark.icon_set.is_none(),
                "preset '{name}' dark.icon_set should be None"
            );
        }
    }
```

**Step 3: Run tests**

Run: `cargo test -p native-theme -- icon_set`
Expected: All PASS.

**Step 4: Commit**

```
refactor(presets): rename icon_theme to icon_set in all TOML presets
```

---

## Task 3: Update all `icon_theme` references in platform readers and lib.rs

**Files:**
- Modify: `native-theme/src/kde/mod.rs:32` (`icon_theme: None` → `icon_set: Some("freedesktop".into())`)
- Modify: `native-theme/src/macos.rs:194,202` (`icon_theme: None` → `icon_set: Some("sf-symbols".into())`)
- Modify: `native-theme/src/windows.rs:274` (`icon_theme: None` → `icon_set: Some("segoe-fluent".into())`)
- Modify: `native-theme/src/lib.rs:290-318` (doc comments and `load_icon` parameter name)

**Step 1: Update KDE reader**

In `native-theme/src/kde/mod.rs:32`, change:
```rust
        icon_set: Some("freedesktop".into()),
```

**Step 2: Update macOS reader**

In `native-theme/src/macos.rs:194` and `:202`, change:
```rust
            icon_set: Some("sf-symbols".into()),
```

**Step 3: Update Windows reader**

In `native-theme/src/windows.rs:274`, change:
```rust
        icon_set: Some("segoe-fluent".into()),
```

**Step 4: Update `load_icon()` in lib.rs**

In `native-theme/src/lib.rs:290-318`, update the doc comment and parameter name from `icon_theme` to `icon_set`:

```rust
/// Load an icon for the given role using the specified icon set.
///
/// Resolves `icon_set` to an [`IconSet`] via [`IconSet::from_name()`],
/// falling back to [`system_icon_set()`] if the set string is not
/// recognized. Then dispatches to the appropriate platform loader or
/// bundled icon set.
///
/// # Fallback chain
///
/// 1. Parse `icon_set` to `IconSet` (unknown names fall back to system set)
/// 2. Platform loader (freedesktop/sf-symbols/segoe-fluent) when `system-icons` enabled
/// 3. Bundled SVGs (material/lucide) when the corresponding feature is enabled
/// 4. Wildcard: try bundled Material, else `None`
#[allow(unreachable_patterns, clippy::needless_return, unused_variables)]
pub fn load_icon(role: IconRole, icon_set: &str) -> Option<IconData> {
    let set = IconSet::from_name(icon_set).unwrap_or_else(system_icon_set);
```

**Step 5: Run full test suite**

Run: `cargo test -p native-theme`
Expected: All PASS. The serde alias ensures old TOML still works.

**Step 6: Commit**

```
refactor: update platform readers and load_icon to use icon_set naming
```

---

## Task 4: Expand `LinuxDesktop` enum and make `pub(crate)`

**Files:**
- Modify: `native-theme/src/lib.rs:128-149` (enum + detect function)
- Modify: `native-theme/src/lib.rs:365-405` (tests)
- Modify: `native-theme/src/gnome/mod.rs:325-336` (uses `super::LinuxDesktop`)

**Step 1: Write failing tests for new DEs**

Add to the `dispatch_tests` module in `native-theme/src/lib.rs`:

```rust
    #[test]
    fn detect_xfce() {
        assert_eq!(detect_linux_de("XFCE"), LinuxDesktop::Xfce);
    }

    #[test]
    fn detect_cinnamon() {
        assert_eq!(detect_linux_de("X-Cinnamon"), LinuxDesktop::Cinnamon);
    }

    #[test]
    fn detect_mate() {
        assert_eq!(detect_linux_de("MATE"), LinuxDesktop::Mate);
    }

    #[test]
    fn detect_lxqt() {
        assert_eq!(detect_linux_de("LXQt"), LinuxDesktop::LxQt);
    }

    #[test]
    fn detect_budgie() {
        assert_eq!(detect_linux_de("Budgie:GNOME"), LinuxDesktop::Budgie);
    }
```

**Step 2: Run tests to see them fail**

Run: `cargo test -p native-theme -- dispatch_tests`
Expected: FAIL — new variants don't exist yet.

**Step 3: Expand the enum and detection**

In `native-theme/src/lib.rs`, update the enum and function:

```rust
/// Desktop environments recognized on Linux.
#[cfg(target_os = "linux")]
#[derive(Debug, PartialEq)]
pub(crate) enum LinuxDesktop {
    Kde,
    Gnome,
    Xfce,
    Cinnamon,
    Mate,
    LxQt,
    Budgie,
    Unknown,
}

/// Parse `XDG_CURRENT_DESKTOP` (a colon-separated list) and return
/// the recognized desktop environment.
///
/// Checks components in order; first recognized DE wins. Budgie is checked
/// before GNOME because Budgie sets `Budgie:GNOME`.
#[cfg(target_os = "linux")]
pub(crate) fn detect_linux_de(xdg_current_desktop: &str) -> LinuxDesktop {
    for component in xdg_current_desktop.split(':') {
        match component {
            "KDE" => return LinuxDesktop::Kde,
            "Budgie" => return LinuxDesktop::Budgie,
            "GNOME" => return LinuxDesktop::Gnome,
            "XFCE" => return LinuxDesktop::Xfce,
            "X-Cinnamon" => return LinuxDesktop::Cinnamon,
            "Cinnamon" => return LinuxDesktop::Cinnamon,
            "MATE" => return LinuxDesktop::Mate,
            "LXQt" => return LinuxDesktop::LxQt,
            _ => {}
        }
    }
    LinuxDesktop::Unknown
}
```

**Step 4: Update existing tests**

Remove or update the tests that asserted XFCE/Cinnamon → `Unknown`:

- `detect_unknown_xfce`: change to verify `LinuxDesktop::Xfce`
- `detect_unknown_cinnamon`: change to verify `LinuxDesktop::Cinnamon`

**Step 5: Update `from_linux()` and `from_system_async()`**

In `from_linux()` (line 155-174), add match arms for new variants that fall back to Adwaita preset (same as current `Unknown` behavior — they'll get proper handling via icon theme detection later, not theme reading):

```rust
fn from_linux() -> crate::Result<NativeTheme> {
    let desktop = std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default();
    match detect_linux_de(&desktop) {
        #[cfg(feature = "kde")]
        LinuxDesktop::Kde => crate::kde::from_kde(),
        #[cfg(not(feature = "kde"))]
        LinuxDesktop::Kde => NativeTheme::preset("adwaita"),
        LinuxDesktop::Gnome | LinuxDesktop::Budgie => NativeTheme::preset("adwaita"),
        LinuxDesktop::Xfce | LinuxDesktop::Cinnamon
        | LinuxDesktop::Mate | LinuxDesktop::LxQt => NativeTheme::preset("adwaita"),
        LinuxDesktop::Unknown => {
            #[cfg(feature = "kde")]
            {
                let path = crate::kde::kdeglobals_path();
                if path.exists() {
                    return crate::kde::from_kde();
                }
            }
            NativeTheme::preset("adwaita")
        }
    }
}
```

Similarly update `from_system_async()` match arms.

**Step 6: Update gnome module**

In `native-theme/src/gnome/mod.rs:325`, the `detect_portal_backend` function only returns `Kde` or `Gnome`. No changes needed, but verify it still compiles with the `pub(crate)` visibility change.

**Step 7: Run tests**

Run: `cargo test -p native-theme -- dispatch_tests`
Expected: All PASS.

**Step 8: Commit**

```
feat(linux): expand LinuxDesktop enum with Xfce, Cinnamon, Mate, LxQt, Budgie
```

---

## Task 5: Add `system_icon_theme()` with DE-aware Linux detection

**Files:**
- Modify: `native-theme/src/model/icons.rs` (add `system_icon_theme()`)
- Modify: `native-theme/src/model/mod.rs:16` (re-export)
- Modify: `native-theme/src/lib.rs:92` (re-export)
- Modify: `native-theme/src/freedesktop.rs` (refactor `detect_theme()`)

**Step 1: Write tests for `system_icon_theme()`**

Add to `native-theme/src/model/icons.rs` test module:

```rust
    #[test]
    fn system_icon_theme_returns_non_empty() {
        let theme = system_icon_theme();
        assert!(!theme.is_empty(), "system_icon_theme() should return a non-empty string");
    }
```

**Step 2: Add `system_icon_theme()` in `native-theme/src/model/icons.rs`**

Add after the existing `system_icon_set()` function (after line 359):

```rust
/// Detect the icon theme name for the current platform.
///
/// Returns the name of the icon theme that provides the actual icon files:
/// - **macOS / iOS:** `"sf-symbols"` (no user-configurable icon theme)
/// - **Windows:** `"segoe-fluent"` (no user-configurable icon theme)
/// - **Linux:** DE-specific detection (e.g., `"breeze-dark"`, `"Adwaita"`)
/// - **Other:** `"material"` (bundled fallback)
///
/// On Linux, the detection method depends on the desktop environment:
/// - KDE: reads `[Icons] Theme` from `kdeglobals`
/// - GNOME/Budgie: `gsettings get org.gnome.desktop.interface icon-theme`
/// - Cinnamon: `gsettings get org.cinnamon.desktop.interface icon-theme`
/// - XFCE: `xfconf-query -c xsettings -p /Net/IconThemeName`
/// - MATE: `gsettings get org.mate.interface icon-theme`
/// - LXQt: reads `icon_theme` from `~/.config/lxqt/lxqt.conf`
/// - Unknown: tries KDE, then GNOME gsettings, then `"hicolor"`
///
/// # Examples
///
/// ```
/// use native_theme::system_icon_theme;
///
/// let theme = system_icon_theme();
/// // On a KDE system with Breeze Dark: "breeze-dark"
/// // On macOS: "sf-symbols"
/// ```
pub fn system_icon_theme() -> String {
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    { return "sf-symbols".to_string(); }

    #[cfg(target_os = "windows")]
    { return "segoe-fluent".to_string(); }

    #[cfg(target_os = "linux")]
    { return detect_linux_icon_theme(); }

    #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos", target_os = "ios")))]
    { "material".to_string() }
}

/// Linux icon theme detection, dispatched by desktop environment.
#[cfg(target_os = "linux")]
fn detect_linux_icon_theme() -> String {
    let desktop = std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default();
    let de = crate::detect_linux_de(&desktop);

    match de {
        crate::LinuxDesktop::Kde => detect_kde_icon_theme(),
        crate::LinuxDesktop::Gnome | crate::LinuxDesktop::Budgie => {
            gsettings_icon_theme("org.gnome.desktop.interface")
        }
        crate::LinuxDesktop::Cinnamon => {
            gsettings_icon_theme("org.cinnamon.desktop.interface")
        }
        crate::LinuxDesktop::Xfce => detect_xfce_icon_theme(),
        crate::LinuxDesktop::Mate => {
            gsettings_icon_theme("org.mate.interface")
        }
        crate::LinuxDesktop::LxQt => detect_lxqt_icon_theme(),
        crate::LinuxDesktop::Unknown => {
            // Try KDE first (kdeglobals may exist even without KDE in XDG_CURRENT_DESKTOP)
            let kde = detect_kde_icon_theme();
            if kde != "hicolor" {
                return kde;
            }
            // Try GNOME gsettings
            let gnome = gsettings_icon_theme("org.gnome.desktop.interface");
            if gnome != "hicolor" {
                return gnome;
            }
            "hicolor".to_string()
        }
    }
}

/// Read icon theme from KDE's kdeglobals INI file.
#[cfg(target_os = "linux")]
fn detect_kde_icon_theme() -> String {
    let path = crate::kde::kdeglobals_path();
    if let Ok(content) = std::fs::read_to_string(&path) {
        let mut ini = crate::kde::create_kde_parser();
        if ini.read(content).is_ok() {
            if let Some(theme) = ini.get("Icons", "Theme") {
                if !theme.is_empty() {
                    return theme;
                }
            }
        }
    }
    "hicolor".to_string()
}

/// Query gsettings for icon-theme with the given schema.
#[cfg(target_os = "linux")]
fn gsettings_icon_theme(schema: &str) -> String {
    std::process::Command::new("gsettings")
        .args(["get", schema, "icon-theme"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().trim_matches('\'').to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "hicolor".to_string())
}

/// Read icon theme from XFCE's xfconf-query.
#[cfg(target_os = "linux")]
fn detect_xfce_icon_theme() -> String {
    std::process::Command::new("xfconf-query")
        .args(["-c", "xsettings", "-p", "/Net/IconThemeName"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "hicolor".to_string())
}

/// Read icon theme from LXQt's config file.
#[cfg(target_os = "linux")]
fn detect_lxqt_icon_theme() -> String {
    let path = if let Ok(config_home) = std::env::var("XDG_CONFIG_HOME") {
        if !config_home.is_empty() {
            std::path::PathBuf::from(config_home)
        } else {
            dirs_fallback_config()
        }
    } else {
        dirs_fallback_config()
    }
    .join("lxqt")
    .join("lxqt.conf");

    if let Ok(content) = std::fs::read_to_string(&path) {
        for line in content.lines() {
            let trimmed = line.trim();
            if let Some(value) = trimmed.strip_prefix("icon_theme=") {
                let value = value.trim();
                if !value.is_empty() {
                    return value.to_string();
                }
            }
        }
    }
    "hicolor".to_string()
}

#[cfg(target_os = "linux")]
fn dirs_fallback_config() -> std::path::PathBuf {
    std::env::var("HOME")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| std::path::PathBuf::from("/tmp"))
        .join(".config")
}
```

**Step 3: Re-export from `native-theme/src/model/mod.rs`**

At line 16, add `system_icon_theme` to the re-export:

```rust
pub use icons::{IconData, IconRole, IconSet, icon_name, system_icon_set, system_icon_theme};
```

**Step 4: Re-export from `native-theme/src/lib.rs`**

At line 92, add `system_icon_theme`:

```rust
pub use model::icons::{icon_name, system_icon_set, system_icon_theme};
```

**Step 5: Refactor `freedesktop.rs` to use `system_icon_theme()`**

Replace the `detect_theme()` function in `native-theme/src/freedesktop.rs:11-17`:

```rust
/// Detect the active freedesktop icon theme.
///
/// Delegates to `system_icon_theme()` which handles DE-specific detection.
fn detect_theme() -> String {
    crate::system_icon_theme()
}
```

**Step 6: Make `kde::kdeglobals_path` and `kde::create_kde_parser` accessible**

These are already `pub(crate)` — verify in `native-theme/src/kde/mod.rs`. They are (line 67, 93). Good.

**Step 7: Run tests**

Run: `cargo test -p native-theme`
Expected: All PASS.

**Step 8: Commit**

```
feat(icons): add system_icon_theme() with DE-aware Linux detection
```

---

## Task 6: Update showcase to display icon theme name

**Files:**
- Modify: `connectors/native-theme-gpui/examples/showcase.rs`

**Step 1: Add import**

At line 49, add `system_icon_theme` to the import:

```rust
use native_theme::{icon_name as native_icon_name, load_icon, IconData, IconRole, IconSet, NativeTheme, system_icon_set, system_icon_theme, bundled_icon_by_name};
```

**Step 2: Rename `current_variant_icon_theme` → `current_variant_icon_set`**

Throughout the showcase file, rename the field that tracks the theme variant's icon set:
- Line 558-559: `current_variant_icon_theme` → `current_variant_icon_set`
- Line 603: in `resolve_default_icon_set()`
- Line 710-711: initialization
- Line 800: struct init
- Line 835: theme switch

**Step 3: Update `resolve_default_icon_set` to use `icon_set`**

```rust
    fn resolve_default_icon_set(&self) -> String {
        self.current_variant_icon_set
            .as_deref()
            .unwrap_or(system_icon_set().name())
            .to_string()
    }
```

**Step 4: Update theme switch code**

At line 835:
```rust
            self.current_variant_icon_set = variant.icon_set.clone();
```

At line 710-711:
```rust
        let current_variant_icon_set = pick_variant(&nt, is_dark)
            .and_then(|v| v.icon_set.clone());
```

**Step 5: Relabel "Icon Set" → "Icon Theme" in sidebar**

At line 2821:
```rust
                    .child(Label::new("Icon Theme").text_size(px(13.0)).font_semibold())
```

**Step 6: Show detected system icon theme in the icons section header**

In the native section title (around lines 2222-2240), add the detected theme name. Replace the native section title construction:

```rust
        let detected_theme = system_icon_theme();
        let is_system_set = matches!(self.icon_set_name.as_str(), "freedesktop" | "sf-symbols" | "segoe-fluent");
        let native_section_title = if is_system_set {
            format!(
                "Native Theme Icons: {} (theme: {}) [{}/{} loaded, {} system, {} fallback]{}",
                self.icon_set_name,
                detected_theme,
                loaded_count,
                self.loaded_icons.len(),
                system_count,
                fallback_count,
                fallback_label,
            )
        } else {
            format!(
                "Native Theme Icons: {} [{}/{} loaded]{}",
                self.icon_set_name,
                loaded_count,
                self.loaded_icons.len(),
                fallback_label,
            )
        };
```

**Step 7: Build and test visually**

Run: `cargo run -p native-theme-gpui --example showcase`
Expected: Sidebar shows "Icon Theme" label. Icons section header shows detected theme name (e.g., "freedesktop (theme: breeze-dark)").

**Step 8: Commit**

```
feat(showcase): display detected icon theme name, rename Icon Set to Icon Theme
```

---

## Task 7: Update docs references

**Files:**
- Modify: `docs/native-icons.md` — update all `icon_theme` field references to `icon_set`

**Step 1: Search and replace `icon_theme` → `icon_set` in docs**

Update the documentation to reflect the renamed field. Mention the serde alias for backward compatibility. Add a section about `system_icon_theme()`.

**Step 2: Commit**

```
docs: update native-icons.md for icon_set rename and system_icon_theme()
```

---

## Task 8: Final integration test

**Step 1: Run full test suite**

Run: `cargo test --all`
Expected: All PASS.

**Step 2: Run showcase and verify**

Run: `cargo run -p native-theme-gpui --example showcase`
Expected:
- Sidebar label says "Icon Theme"
- When using "default (freedesktop)" on KDE, header shows detected theme (e.g., "breeze-dark")
- Switching themes and icon sets works correctly
- All icons load as before

**Step 3: Final commit if any fixups needed**

---

## Summary of all commits

1. `feat(model): rename icon_theme field to icon_set with serde alias`
2. `refactor(presets): rename icon_theme to icon_set in all TOML presets`
3. `refactor: update platform readers and load_icon to use icon_set naming`
4. `feat(linux): expand LinuxDesktop enum with Xfce, Cinnamon, Mate, LxQt, Budgie`
5. `feat(icons): add system_icon_theme() with DE-aware Linux detection`
6. `feat(showcase): display detected icon theme name, rename Icon Set to Icon Theme`
7. `docs: update native-icons.md for icon_set rename and system_icon_theme()`
