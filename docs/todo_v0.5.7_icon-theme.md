# Icon Theme Dropdown: Bug Fix + Feature Enhancement

Two related issues in **both** the GPUI and iced showcase icon theme selectors.

---

## Issue 1: "system" selection reverts to "default" after ~1 second

### Symptom

On KDE (breeze-dark), select Theme: **Adwaita**, then select Icon Theme:
**"system (breeze-dark)"**.  After roughly 1 second the dropdown silently
reverts to **"default (Adwaita)"**.  Other icon themes (Lucide, Material) are
not affected.

Both showcases exhibit this bug, but through different code paths.

### Root cause: GPUI showcase

A single boolean `use_default_icon_set` (`showcase-gpui.rs:908`) conflates two
distinct user intents: "follow the preset's recommendation" and "use the OS
icon theme".

**The chain of events:**

1. User selects Theme: Adwaita.
   `apply_theme_by_name("Adwaita")` sets `has_toml_icon_theme = true`,
   `current_icon_theme = "Adwaita"`, `current_icon_set = Freedesktop` (lines
   1772-1793).  Because `use_default_icon_set` is `true` (initial value, line
   1687), the reapplication block at line 1802 fires and sets the dropdown to
   `default_icon_label()` = `"default (Adwaita)"`.

2. User selects Icon Theme: "system (breeze-dark)".
   `icon_set_internal_name()` returns `"system"` (line 1184).  Line 1483 sets
   `use_default_icon_set = internal == "default" || internal == "system"` ->
   **true**.  System icons are loaded correctly.  Dropdown shows "system
   (breeze-dark)".

3. ~500 ms later the KDE theme watcher fires.
   The inotify watcher (`watch/kde.rs`) monitors `~/.config/` for changes to
   `kdeglobals`/`kcmfontsrc`.  On a live KDE session these files are touched
   frequently by kded, plasmashell, etc.  The watcher sets `theme_change_flag`
   (line 1643).  The 500 ms polling loop (line 1839) picks it up and, because
   `color_mode == System` (the default, line 1228), sets
   `pending_system_theme_change = true` (line 1843).

4. On the next render (line 5327), `apply_theme_by_name("Adwaita")` runs again.
   The preset re-sets `current_icon_theme = "Adwaita"`, `has_toml_icon_theme =
   true`.  The block at line 1802 fires because `use_default_icon_set == true`.
   `default_icon_label()` (line 1137) sees the Adwaita freedesktop theme is
   installed, returns `"default (Adwaita)"`.  The dropdown is overwritten.

**Why other icon themes survive:** Lucide and Material set
`use_default_icon_set = false` (line 1483), so the block at 1802 is skipped
entirely on re-entry.

### Root cause: iced showcase

The iced showcase already has an `IconSetChoice` enum (`showcase-iced.rs:287`),
which is the right abstraction.  However, `rebuild_theme()` (line 854)
**unconditionally overwrites** the user's choice.

At lines 947-962, every call to `rebuild_theme()` runs:

```rust
let (new_choice, new_choices) = resolve_icon_choice(
    self.current_icon_set,
    &self.current_icon_theme,
    has_toml_icon_theme,
);
// ...
self.icon_set_choice = new_choice;   // <-- overwrites user's explicit pick
self.icon_set_choices = new_choices;
```

`resolve_icon_choice()` (line 333) always re-derives from the theme TOML.
When the user had explicitly selected `IconSetChoice::System`, this re-derivation
returns `IconSetChoice::Default("Adwaita")` (because the Adwaita TOML specifies
`icon_theme`), overwriting the user's intent.

The trigger is the same: the theme watcher fires `ThemeWatcherTick` (line 1328)
which calls `rebuild_theme()` (line 1331).

### Affected code locations

**GPUI showcase** (`showcase-gpui.rs`):

| What | Line(s) |
|---|---|
| `use_default_icon_set` field | 908 |
| Both intents collapsed into one bool | 1483 |
| Initial value `true` | 1687 |
| Reapplication block that overwrites | 1802-1823 |
| `default_icon_label()` always returns "default" | 1137-1154 |
| `resolve_default_icon_set()` ignores "system" | 1094-1109 |
| `resolve_default_icon_theme()` ignores "system" | 1116-1131 |
| Theme watcher polling loop | 1837-1852 |
| Render-time deferred reapplication | 5327-5332 |

**Iced showcase** (`showcase-iced.rs`):

| What | Line(s) |
|---|---|
| `IconSetChoice` enum (correct type, wrong usage) | 287-294 |
| `resolve_icon_choice()` ignores user's explicit pick | 333-360 |
| `icon_set_choice` field | 610 |
| `rebuild_theme()` unconditional overwrite | 947-962 |
| User's explicit selection handler | 1307-1327 |
| Theme watcher tick handler | 1328-1332 |

**Shared** (`watch/kde.rs`):

| What | Line(s) |
|---|---|
| KDE inotify watcher | 17-93 |

### Solution options

#### Option A: Library-level `IconSetChoice` enum + "respect user choice" pattern

Move `IconSetChoice` to the `native-theme` library (or introduce a new one that
covers all variants).  Both showcases import the same type.  The critical fix:
on theme re-application, only overwrite the icon choice when the current mode is
`Default` (meaning "follow the preset").  All other modes represent explicit
user intent and are preserved.

**Pros:**
- Models the domain correctly: there are distinct intents (default, system,
  explicit theme, bundled), not a boolean.
- Self-documenting: each variant's name says what it does.
- Impossible to create invalid state (unlike multiple bools).
- Each `match` arm has clear, auditable behaviour.
- Naturally extends to support Issue 2 (installed themes) via the
  `Freedesktop(String)` variant.
- Shared between both showcases — one fix, one type, consistent semantics.
- `System` variant can correctly respond to system icon theme changes at
  runtime (user switches breeze -> breeze-dark in KDE settings while the app
  is running — the "system" label updates, but the choice stays `System`).

**Cons:**
- Larger change surface: GPUI needs the bool replaced (6 sites), iced needs
  `rebuild_theme()` guarded (1 site) and its local enum replaced with the
  library import.
- The `Default(String)` / `Freedesktop(String)` variants allocate.  Negligible
  for a UI event.

#### Option B: Add a second boolean `user_chose_system: bool` (GPUI only)

Keep `use_default_icon_set` and add `user_chose_system: bool`.  The
reapplication block checks the second flag before overwriting.  Fix the iced
showcase separately.

**Pros:**
- Small, localised change for GPUI.

**Cons:**
- Two booleans create 4 states, only 3 are valid (`use_default_icon_set=false,
  user_chose_system=true` is meaningless).  Fragile invariant.
- Does not scale to Issue 2 (adding explicit freedesktop theme choices would
  need a third flag or a string).
- Easy to forget to set one flag when setting the other.
- Fixes each showcase independently — no shared API, duplicated semantics.
- Does not address the iced showcase at all (it already has an enum but doesn't
  respect it).

#### Option C: Track the user's last selected dropdown label

Store the raw display label (e.g. `"system (breeze-dark)"`) and on
reapplication, compare the candidate label against the stored one.  Skip the
update if they differ.

**Pros:**
- Conceptually simple.

**Cons:**
- String comparison against display labels is fragile (format changes break it).
- The label "system (breeze-dark)" changes if the system theme changes — should
  the stored value track the new one or stay on the old one?  Ambiguous
  semantics.
- Does not fix the icon *loading* part of the bug: the reapplication block also
  reloads icons via `resolve_default_icon_set()`, which still ignores the
  "system" intent.  Fixing the dropdown label without fixing the icon loading
  would show "system (breeze-dark)" but render Adwaita icons.
- No shared API.  Would need a parallel fix for Issue 2.

#### Option D: Only re-apply icons when the preset's icon info actually changes

Diff the old vs. new `current_icon_theme` / `current_icon_set`.  Skip the
reapplication block if they're identical.

**Pros:**
- Minimal code change.

**Cons:**
- Does not fix the root cause.  If the preset's icon theme *does* change (e.g.
  switching from Adwaita to Breeze theme), the user's "system" choice would
  still be overwritten.
- Masks the bug for the common case but fails for edge cases.
- The watcher path calls `invalidate_caches()` before reapplication, which
  could cause the preset to resolve differently even for the same theme name
  (e.g. dark/light flip).  Diffing becomes unreliable.
- No path to support Issue 2.

#### Option E: Remove the dropdown/choice update from the reapplication entirely

In GPUI: delete lines 1816-1822.  In iced: remove lines 962-963 from
`rebuild_theme()`.  Only reload icons, don't touch the selection state.

**Pros:**
- Simplest possible fix.

**Cons:**
- Breaks the "default" intent: when the user has "default" selected and
  switches from Adwaita to Breeze theme, the dropdown should update from
  "default (Adwaita)" to "default (breeze)" — but it would stay on the stale
  label.
- In GPUI, the dropdown items list is rebuilt to include the new theme's
  "default (X)" label, but the selection stays on a label that no longer exists
  in the list — undefined dropdown state.
- Does not address the icon-loading half of the reapplication block, which
  still uses preset-derived resolution instead of respecting "system".

### Verdict: Issue 1

**Option A (library-level enum) is the correct solution.**

It is the only option that:
1. Fixes the bug for all cases (not just the common one), in both showcases.
2. Models the actual domain semantics as a shared, reusable type.
3. Prevents invalid state at the type level.
4. Naturally supports Issue 2 without further refactoring.
5. Correctly handles the `System` variant's runtime re-derivation.

The extra change surface (GPUI: 6 sites, iced: 2 sites) is proportional to the
semantic improvement.  Options B-E are either fragile, incomplete, or break
other functionality.

---

## Issue 2: Only one system icon theme is available in the dropdown

### Symptom

Both showcases show only these entries (example for Adwaita + KDE breeze-dark):

```
default (Adwaita)
system (breeze-dark)
[Lucide / Material / gpui-builtin]
```

On a KDE system with breeze, breeze-dark, Adwaita, char-white, and other icon
themes installed, only the single current system theme is offered.  The user
cannot select an alternative installed freedesktop theme without using the
`--icon-theme` CLI flag.

### Affected code locations

**GPUI showcase** (`showcase-gpui.rs`):

| What | Line(s) |
|---|---|
| Dropdown items construction | 1157-1169 |
| Icon set selection handler | 1472-1518 |
| `icon_set_internal_name()` parsing | 1180-1195 |

**Iced showcase** (`showcase-iced.rs`):

| What | Line(s) |
|---|---|
| `IconSetChoice::choices()` builder | 312-321 |
| `IconSetChoice` enum (no `Freedesktop` variant) | 287-294 |
| Icon set selection handler | 1307-1327 |

**Library** (`icons.rs`):

| What | Line(s) |
|---|---|
| `is_freedesktop_theme_available()` | 244-277 |
| No `list_freedesktop_themes()` exists | — |

### Solution options for theme discovery

#### Option A: Add `list_freedesktop_themes()` to the library, populate dropdown

Add a public function to `icons.rs` that scans `$XDG_DATA_DIRS/icons/` and
`$XDG_DATA_HOME/icons/` for subdirectories containing `index.theme`.  Filter
out `hicolor` (mandatory fallback, not a user choice) and `default` (usually a
symlink).  Return a sorted `Vec<String>`.

Both showcases call this once and populate their dropdowns with the results.

**Pros:**
- Reusable library API: any consumer (not just showcases) can enumerate
  available icon themes.
- Follows the same pattern as the existing `is_freedesktop_theme_available()`.
- Users see all their installed themes without needing CLI flags.

**Cons:**
- Directory scanning is IO.  (Fast in practice: typically <50 entries, only
  checking directory existence + `index.theme`.)
- May list themes that have no application icons (e.g. cursor-only themes).
  The user would select one and see no icons — confusing.

#### Option B: Scan and filter by parsing `index.theme` files

Like Option A, but read each theme's `index.theme` to check whether it declares
`Directories=` entries with `Context=Actions`, `Context=Applications`, etc.
Filter out cursor-only themes and incomplete themes.

**Pros:**
- Cleaner list: only themes that actually have application icons are shown.
- Better user experience: every listed theme will produce visible results.

**Cons:**
- Significantly more IO and parsing: reading and parsing ~30-50 INI-like files
  on each scan.
- Fragile: `index.theme` format varies across themes.  Some themes use
  non-standard keys.  The parser would need to be robust.
- Complex filtering logic for diminishing returns (most installed themes are
  real icon themes, not cursor-only).
- Cursor-only themes are rare in `$XDG_DATA_DIRS/icons/`; most live in
  `$XDG_DATA_DIRS/cursors/` or `~/.local/share/icons/` with only a `cursors/`
  subdirectory.  A simple heuristic (Option C) handles them without full
  parsing.

#### Option C: Scan with lightweight heuristic filter

Like Option A, but apply a cheap heuristic to exclude non-icon themes: check
whether the theme directory contains *any* subdirectory other than `cursors/`.
Real icon themes have directories like `16x16/`, `22x22/`, `scalable/`,
`actions/`, etc.

**Pros:**
- Fast: one `read_dir` per theme, no file parsing.
- Filters out the most common false positive (cursor-only themes) without
  complex logic.
- Handles the practical reality: real icon themes always have size/category
  directories; cursor-only themes only have `cursors/`.

**Cons:**
- Heuristic can produce false positives for unusual theme layouts, but the
  consequence is just showing a theme that has few/no app icons — not a crash.
- Adds an extra `read_dir` per theme compared to Option A.

#### Option D: No scanning — only "system" and a text input

Keep the dropdown as-is (system + bundled) but add a free-text input field
where the user can type any freedesktop theme name (e.g. "char-white").

**Pros:**
- Zero IO overhead.
- No filtering question.
- Works for any theme, even non-standard ones.

**Cons:**
- Terrible discoverability: the user must already know the exact installed
  theme names.
- Typing the name is error-prone (case-sensitive, hyphens vs underscores).
- Breaks the dropdown-based UX pattern of the showcases.
- Does not solve the core user request: "show me what's available".

### Solution options for caching

#### Option E1: Scan on every dropdown rebuild

Call `list_freedesktop_themes()` every time the dropdown items list is rebuilt
(on theme change, color mode change, etc.).

**Pros:**
- Always up-to-date.

**Cons:**
- Repeated IO on every theme switch.  While fast, it's unnecessary since
  installed themes don't change during a session.

#### Option E2: Scan once at startup, cache the list

Scan during `Showcase::new()` (GPUI) or `State::default()` (iced) and store the
result.  Dropdown rebuilds use the cache.

**Pros:**
- No repeated IO.
- Startup cost is negligible (single `read_dir` per XDG data dir).

**Cons:**
- Does not pick up icon themes installed while the app is running (extremely
  rare; acceptable trade-off for a showcase app).

### Solution options for dropdown presentation

#### Option F: Flat list (no visual grouping)

Append all discovered themes to the existing flat list.

**Pros:**
- Simplest dropdown change: just push more items onto the vec.

**Cons:**
- Long list becomes hard to navigate.  On a system with 20+ icon themes, the
  user must scan through all entries with no visual separation.
- Unclear which items are installed freedesktop themes vs. bundled themes.

#### Option G: Grouped dropdown with section labels

Insert human-readable section separators (non-selectable items or visual
markers) between groups:

```
default (Adwaita)
system (breeze-dark)
-- Installed (freedesktop) --
Adwaita
breeze
breeze-dark
char-white
-- Bundled --
gpui-component built-in (Lucide)
Lucide (bundled)
Material (bundled)
```

**Pros:**
- Clear visual hierarchy.
- User immediately understands what each group means.
- Scales to large numbers of installed themes.

**Cons:**
- Requires the dropdown widget to support non-selectable separator items, or
  a workaround (e.g. items with a special prefix that the handler ignores).
- The GPUI `SearchableVec<SharedString>` delegate and the iced `pick_list`
  may not support heterogeneous items natively.

### Verdict: Issue 2

**Option C (heuristic filter) + Option E2 (cache at startup) + Option F (flat
list with clear naming).**

1. **Option C over A or B:** The heuristic filter (skip directories that
   *only* contain `cursors/`) catches the main false positive with negligible
   cost.  Full `index.theme` parsing (Option B) is over-engineered.

2. **Option E2 (cache):** Scan once at startup.  Icon themes are not installed
   while the app is running in any normal workflow.

3. **Option F over G (flat list):** The grouped dropdown (Option G) is better
   UX, but depends on widget support for non-selectable separators.  A flat
   list with clear naming conventions is acceptable and works with both GPUI
   and iced widgets as-is.  Grouping can be added later as a presentation
   improvement without changing the underlying data model.

4. **Option D (text input) is rejected:** It defeats the purpose.  The whole
   point is discoverability.

---

## Unified library API design

Both issues share the `IconSetChoice` enum as their core change.  This type
belongs in the `native-theme` library so both showcases (and any future
consumer) share the same semantics.

### Proposed type: `IconSetChoice`

Lives in `native-theme/src/icons.rs` (next to `is_freedesktop_theme_available`).

```rust
/// The user's icon set selection mode.
///
/// Represents the user's intent for which icons to display.  The key
/// invariant: only `Default` is re-derived on theme changes.  All other
/// variants represent an explicit user choice that is preserved across
/// theme re-applications.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IconSetChoice {
    /// Follow the theme preset's recommendation.
    ///
    /// The `String` is the preset's `icon_theme` name (e.g. "Adwaita"),
    /// used for display ("default (Adwaita)") and for loading when the
    /// icon set is `Freedesktop`.
    ///
    /// This is the ONLY variant that gets overwritten on theme change.
    Default(String),

    /// Use the OS-configured icon theme.
    ///
    /// Resolved at load time via `system_icon_set()`.  The display label
    /// ("system (breeze-dark)") is computed dynamically from
    /// `system_icon_theme()`, so it tracks runtime OS theme changes.
    System,

    /// User explicitly picked a specific installed freedesktop icon theme.
    ///
    /// The `String` is the theme directory name (e.g. "char-white",
    /// "breeze", "Papirus").  Loaded via `IconSet::Freedesktop` with
    /// `.theme(name)`.
    Freedesktop(String),

    /// Google Material Symbols (bundled).
    Material,

    /// Lucide Icons (bundled).
    Lucide,
}
```

### `Display` implementation

```rust
impl Display for IconSetChoice {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Default(name) => write!(f, "default ({name})"),
            Self::System => {
                let name = system_icon_theme();
                write!(f, "system ({name})")
            }
            Self::Freedesktop(name) => write!(f, "{name}"),
            Self::Material => write!(f, "Material (bundled)"),
            Self::Lucide => write!(f, "Lucide (bundled)"),
        }
    }
}
```

Note: the GPUI showcase has an additional "gpui-component built-in (Lucide)"
entry that is framework-specific.  This can be handled locally in the showcase
without polluting the library type.

### Helper methods

```rust
impl IconSetChoice {
    /// The effective `IconSet` loading mechanism for this choice.
    pub fn effective_icon_set(&self, theme_icon_set: IconSet) -> IconSet {
        match self {
            Self::Default(_) => theme_icon_set,
            Self::System => system_icon_set(),
            Self::Freedesktop(_) => IconSet::Freedesktop,
            Self::Material => IconSet::Material,
            Self::Lucide => IconSet::Lucide,
        }
    }

    /// The freedesktop theme name to pass to `IconLoader::theme()`, if any.
    ///
    /// Returns `Some` for `Default` (when the set is freedesktop) and
    /// `Freedesktop`.  Returns `None` for `System`, `Material`, `Lucide`.
    pub fn freedesktop_theme(&self) -> Option<&str> {
        match self {
            Self::Default(name) | Self::Freedesktop(name) => Some(name),
            _ => None,
        }
    }

    /// Whether this choice should be re-derived when the theme changes.
    ///
    /// Only `Default` follows the preset.  All others are explicit user
    /// choices that must be preserved.
    pub fn follows_preset(&self) -> bool {
        matches!(self, Self::Default(_))
    }
}
```

### Proposed function: `default_icon_choice`

Replaces both `resolve_icon_choice()` in iced and `resolve_default_icon_set()`
/ `resolve_default_icon_theme()` / `default_icon_label()` in GPUI.

```rust
/// Determine the default icon set choice for a theme.
///
/// When the TOML specifies `icon_theme` and the theme is available
/// (bundled sets are always available; freedesktop themes are checked
/// via `is_freedesktop_theme_available`), returns `Default(icon_theme)`.
///
/// When the TOML does not specify `icon_theme`, or the specified
/// freedesktop theme is not installed, returns `System`.
pub fn default_icon_choice(
    icon_set: IconSet,
    icon_theme: &str,
    has_toml_icon_theme: bool,
) -> IconSetChoice {
    if !has_toml_icon_theme {
        return IconSetChoice::System;
    }
    let available = match icon_set {
        IconSet::Material | IconSet::Lucide => true,
        IconSet::Freedesktop => is_freedesktop_theme_available(icon_theme),
        IconSet::SfSymbols | IconSet::SegoeIcons => true,
        _ => false,
    };
    if available {
        IconSetChoice::Default(icon_theme.to_string())
    } else {
        IconSetChoice::System
    }
}
```

### Proposed function: `list_freedesktop_themes`

```rust
/// List installed freedesktop icon themes.
///
/// Scans `$XDG_DATA_DIRS/icons/` and `$XDG_DATA_HOME/icons/` for
/// subdirectories containing `index.theme` and at least one non-cursor
/// subdirectory.  Excludes `hicolor` and `default`.  Returns a sorted,
/// deduplicated list of theme directory names.
///
/// Returns an empty `Vec` on non-Linux platforms.
pub fn list_freedesktop_themes() -> Vec<String>
```

### How showcases use the API

**Theme re-application pattern (both showcases):**

```rust
// After loading the new theme's icon_set / icon_theme / has_toml_icon_theme:
if self.icon_set_choice.follows_preset() {
    // User wants "follow the preset" — re-derive.
    self.icon_set_choice = default_icon_choice(icon_set, &icon_theme, has_toml);
    reload_icons(&self.icon_set_choice, ...);
    update_dropdown(&self.icon_set_choice, ...);
} else if matches!(self.icon_set_choice, IconSetChoice::System) {
    // User wants "system" — icons may need reloading if system theme changed,
    // but the choice stays System.
    reload_icons(&self.icon_set_choice, ...);
    update_dropdown_label(&self.icon_set_choice, ...);  // "system (Y)" may have changed
}
// For Freedesktop(_), Material, Lucide: do nothing.  User's choice is preserved.
```

**Dropdown construction pattern (both showcases):**

```rust
fn icon_set_dropdown_items(
    choice: &IconSetChoice,
    icon_set: IconSet,
    icon_theme: &str,
    has_toml: bool,
    installed_themes: &[String],  // cached from list_freedesktop_themes()
) -> Vec<IconSetChoice> {
    let mut items = Vec::new();

    // "default (X)" — only when TOML specifies an icon_theme
    if has_toml {
        items.push(default_icon_choice(icon_set, icon_theme, has_toml));
    }

    // "system (Y)" — always available
    items.push(IconSetChoice::System);

    // Installed freedesktop themes
    for name in installed_themes {
        items.push(IconSetChoice::Freedesktop(name.clone()));
    }

    // Bundled
    items.push(IconSetChoice::Material);
    items.push(IconSetChoice::Lucide);

    items
}
```

**Icon loading pattern (both showcases):**

```rust
let set = choice.effective_icon_set(theme_icon_set);
let theme = choice.freedesktop_theme();
// theme is Some("Adwaita") for Default, Some("char-white") for Freedesktop,
// None for System/Material/Lucide.
let data = IconLoader::new(role)
    .set(set)
    .theme_opt(theme)     // hypothetical; or conditional .theme() call
    .color_opt(fg)
    .load();
```

### What each showcase deletes

**GPUI showcase** — removes these functions entirely:
- `use_default_icon_set` field (replaced by `IconSetChoice`)
- `resolve_default_icon_set()` (replaced by `choice.effective_icon_set()`)
- `resolve_default_icon_theme()` (replaced by `choice.freedesktop_theme()`)
- `default_icon_label()` (replaced by `choice.to_string()` via `Display`)
- `icon_set_internal_name()` string parsing (replaced by enum matching)

**Iced showcase** — removes these functions entirely:
- Local `IconSetChoice` enum (replaced by library import)
- `resolve_icon_choice()` (replaced by `default_icon_choice()` + conditional
  overwrite)

---

## Combined implementation plan

1. **Library: add `IconSetChoice` enum** to `native-theme/src/icons.rs`
   with `Display`, `effective_icon_set()`, `freedesktop_theme()`, and
   `follows_preset()`.

2. **Library: add `default_icon_choice()`** to `native-theme/src/icons.rs`.

3. **Library: add `list_freedesktop_themes()`** to `native-theme/src/icons.rs`.

4. **Library: re-export** the new types from `native-theme/src/lib.rs` and
   connector crates as needed.

5. **Iced showcase:**
   - Replace local `IconSetChoice` with library import.
   - Remove local `resolve_icon_choice()`.
   - Guard `rebuild_theme()` icon block with `follows_preset()` check.
   - Add `installed_themes: Vec<String>` field, populated once at init.
   - Update `IconSetChoice::choices()` to include installed themes.
   - Handle `Freedesktop(name)` in the icon set selection message.

6. **GPUI showcase:**
   - Replace `use_default_icon_set: bool` with `IconSetChoice` from library.
   - Remove `resolve_default_icon_set()`, `resolve_default_icon_theme()`,
     `default_icon_label()`, and `icon_set_internal_name()`.
   - Rewrite the reapplication block at line 1802 to use `follows_preset()`.
   - Add `installed_themes: Vec<String>` field, populated once in `new()`.
   - Update `icon_set_dropdown_names()` to include installed themes.
   - Handle `Freedesktop(name)` in the icon set selection handler.

7. **Both showcases:** Verify that:
   - Selecting "system (breeze-dark)" with Adwaita theme no longer reverts.
   - Selecting a specific installed theme (e.g. "char-white") works and
     persists across theme watcher ticks.
   - Switching themes updates the "default (X)" label but preserves other
     selections.
   - The installed themes list shows all real icon themes (no cursor-only
     themes, no hicolor).
