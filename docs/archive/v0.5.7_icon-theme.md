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
   installed, returns `"default (Adwaita)"`.  **Both the dropdown label and
   the loaded icons** are overwritten — the icons switch from breeze-dark back
   to Adwaita.

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
which calls `rebuild_theme()` (line 1331).  The iced subscription is gated on
`color_mode == System` (line 3146), so this only fires in System color mode —
the same condition as the GPUI showcase.

### Affected code locations

**GPUI showcase** (`showcase-gpui.rs`):

| What | Line(s) |
|---|---|
| `use_default_icon_set` field | 908 |
| Both intents collapsed into one bool | 1483 |
| Initial value `true` | 1687 |
| Reapplication block (overwrites choice AND icons) | 1802-1823 |
| `default_icon_label()` returns preset-derived label | 1137-1154 |
| `resolve_default_icon_set()` resolves from preset, ignores user selection | 1094-1109 |
| `resolve_default_icon_theme()` resolves from preset, ignores user selection | 1116-1131 |
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
| Watcher subscription gated on `color_mode == System` | 3146 |

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
user intent and are preserved.  Icons are always reloaded regardless of mode
(text color may change on dark/light switch).

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
- Helper methods (`effective_icon_set()`, `freedesktop_theme()`,
  `follows_preset()`) eliminate ad-hoc string parsing and boolean checks.
- The user explicitly asked for a "user-friendly API" — a shared library type
  is the direct answer.

**Cons:**
- Larger change surface: GPUI needs the bool replaced (6 sites), iced needs
  `rebuild_theme()` guarded (1 site) and its local enum replaced with the
  library import.
- The `Default(String)` / `Freedesktop(String)` variants allocate a `String`.
  Negligible for a UI event that happens on user interaction.
- Adds public API surface to the `native-theme` library.  However, the type is
  self-contained and unlikely to need breaking changes.

#### Option B: Add a second boolean `user_chose_system: bool` (GPUI only)

Keep `use_default_icon_set` and add `user_chose_system: bool`.  The
reapplication block checks the second flag before overwriting.  Fix the iced
showcase separately.

**Pros:**
- Small, localised change for GPUI.
- Easy to understand in isolation.

**Cons:**
- Two booleans create 4 states, only 3 are valid (`use_default_icon_set=false,
  user_chose_system=true` is meaningless).  Fragile invariant.
- Does not scale to Issue 2 (adding explicit freedesktop theme choices would
  need a third flag or a string).
- Easy to forget to set one flag when setting the other.
- Fixes each showcase independently — no shared API, duplicated semantics.
- Does not address the iced showcase at all (it already has an enum but doesn't
  respect it — this option only covers GPUI).

#### Option C: Track the user's last selected dropdown label

Store the raw display label (e.g. `"system (breeze-dark)"`) and on
reapplication, compare the candidate label against the stored one.  Skip the
update if they differ.

**Pros:**
- Conceptually simple.
- Small change.

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
- Masks the bug for the common case (same theme, same icon info) but fails for
  edge cases (dark/light toggle that changes icon theme, theme switch).
- The watcher path calls `invalidate_caches()` before reapplication, which
  could cause the preset to resolve differently even for the same theme name
  (e.g. dark/light flip).  Diffing becomes unreliable.
- Would also incorrectly skip icon reloading when only text color changed
  (dark ↔ light), breaking icon recoloring.
- No path to support Issue 2.

#### Option E: Remove the dropdown/choice update from the reapplication entirely

In GPUI: delete lines 1816-1822.  In iced: remove lines 962-963 from
`rebuild_theme()`.  Only reload icons, don't touch the selection state.

**Pros:**
- Simplest possible fix: delete a few lines.

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
  The user would see "system (breeze-dark)" in the dropdown but the actual
  icons rendered would be Adwaita's.

#### Option F: Fix each showcase locally, no library changes

For iced: keep the existing local `IconSetChoice` enum, add a
`Freedesktop(String)` variant, add a `follows_preset()` guard in
`rebuild_theme()`.

For GPUI: introduce a local `IconSetMode` enum replacing the
`use_default_icon_set: bool`.

Each showcase implements its own enum and guard logic independently.  No changes
to the `native-theme` library.

**Pros:**
- No library API surface change.
- Each showcase can evolve independently.
- Smaller PRs — showcase changes don't require a library release.

**Cons:**
- Duplicated type and semantics between two showcases.  Both define a nearly
  identical enum with identical invariants — a textbook duplication smell.
- `list_freedesktop_themes()` (needed for Issue 2) must either go in the
  library anyway or be duplicated in each showcase.  If it goes in the library,
  the enum should go there too for consistency.
- If a third consumer (or a user's own app) needs the same pattern, they must
  reinvent it.
- Divergence risk: the enums may evolve differently, confusing contributors
  who work in both showcases.
- The user explicitly requested a "user-friendly API," which implies a shared,
  reusable type.

### Verdict: Issue 1

**Option A (library-level enum) is the correct solution.**

It is the only option that:
1. Fixes the bug for all cases (not just the common one), in both showcases.
2. Models the actual domain semantics as a shared, reusable type.
3. Prevents invalid state at the type level.
4. Naturally supports Issue 2 without further refactoring.
5. Correctly handles the `System` variant's runtime re-derivation.
6. Satisfies the "user-friendly API" requirement.

Options B-E are either fragile, incomplete, or break other functionality.
Option F is viable but wastes the opportunity to provide a clean shared API
and forces duplication.

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

#### Option A: Add `list_freedesktop_themes()` with `index.theme` existence check only

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
- Simplest scanning logic: just check if `index.theme` exists.

**Cons:**
- Includes cursor-only themes.  On a typical KDE system, `/usr/share/icons/`
  contains cursor-only themes such as `breeze_cursors`, `Breeze_Light`, and
  `capitaine-cursors` — all with an `index.theme` but no application icons.
  Selecting one would show zero icons, confusing the user.

#### Option B: Scan and filter by full `index.theme` parsing

Like Option A, but read each theme's `index.theme` and parse it to check
whether it declares `Directories=` entries with `Context=Actions`,
`Context=Applications`, etc.  Filter out cursor-only themes and incomplete
themes.

**Pros:**
- Cleanest list: only themes that actually have application icons are shown.
- Better user experience: every listed theme will produce visible results.

**Cons:**
- Significantly more IO and parsing: reading and parsing ~30-50 INI-like files,
  iterating their `Directories=` list, and checking `Context=` per entry.
- Fragile: `index.theme` format varies across themes.  Some themes use
  non-standard keys.  The parser would need to be robust.
- Complex filtering logic for diminishing returns.
- Overkill: the distinction between "has any icons" and "cursor-only" can be
  made with a much simpler check (Option C).

#### Option C: Scan with `Directories=` line check

Like Option A, but after confirming `index.theme` exists, read its first few
lines to check whether it contains a `Directories=` line.  The freedesktop Icon
Theme Specification requires `Directories=` in the `[Icon Theme]` group for
real icon themes.  Cursor-only themes (e.g. `breeze_cursors`, `Breeze_Light`,
`capitaine-cursors`) omit it — they only provide a `[Icon Theme]` header with
`Name=` and inherit cursor data.

Verified on the actual system:
- `breeze_cursors/index.theme`: no `Directories=` line (cursor-only)
- `Breeze_Light/index.theme`: no `Directories=` line (cursor-only)
- `breeze/index.theme`: has `Directories=actions/12,actions/16,...` (real theme)

**Pros:**
- Simple: one line scan per theme file — no full INI parsing needed.
- Spec-conformant: based on the freedesktop Icon Theme Specification, not
  filesystem layout heuristics.
- Reliable: correctly filters out all cursor-only themes tested
  (`breeze_cursors`, `Breeze_Light`, `capitaine-cursors`,
  `capitaine-cursors-light`) while keeping all real icon themes.
- Fast: reads only until the `Directories=` line is found (typically within
  the first 20 lines) or EOF.

**Cons:**
- Slightly more IO than Option A (reads a few lines of each `index.theme`
  instead of just checking its existence), but still fast.
- Theoretically, a cursor-only theme could include a `Directories=` line
  pointing only to cursor directories.  Extremely unlikely in practice.

#### Option D: Heuristic subdirectory check (no file reading)

Instead of reading `index.theme`, check whether the theme directory contains
known icon subdirectories such as `actions/`, `apps/`, or `mimetypes/`.

**Pros:**
- No file reading at all — only directory existence checks.
- Fast: a few `Path::is_dir()` calls per theme.

**Cons:**
- Not spec-conformant: theme authors can use arbitrary directory structures.
  The heuristic is based on common convention, not the freedesktop spec.
- False positives: a cursor-only theme could have leftover or unrelated
  subdirectories that happen to match.
- False negatives: a valid icon theme with non-standard directory names would
  be incorrectly filtered out.
- Strictly worse than Option C in every dimension — same IO class (filesystem
  metadata), less reliable, less principled.

#### Option E: No scanning — only "system" and a text input

Keep the dropdown as-is (system + bundled) but add a free-text input field
where the user can type any freedesktop theme name (e.g. "char-white").

**Pros:**
- Zero IO overhead.
- No filtering question.
- Works for any theme, even non-standard ones not in XDG paths.

**Cons:**
- Terrible discoverability: the user must already know the exact installed
  theme names.
- Typing the name is error-prone (case-sensitive, hyphens vs underscores).
- Breaks the dropdown-based UX pattern of the showcases.
- Does not solve the core user request: "show me what's available".

### Solution options for caching

#### Option F1: Scan on every dropdown rebuild

Call `list_freedesktop_themes()` every time the dropdown items list is rebuilt
(on theme change, color mode change, etc.).

**Pros:**
- Always up-to-date.

**Cons:**
- Repeated IO on every theme switch.  While fast, it's unnecessary since
  installed themes rarely change during a session.

#### Option F2: Scan once at startup, cache the list

Scan during `Showcase::new()` (GPUI) or `State::default()` (iced) and store the
result.  Dropdown rebuilds use the cache.

**Pros:**
- No repeated IO.
- Startup cost is negligible.

**Cons:**
- Does not pick up icon themes installed while the app is running (extremely
  rare; acceptable trade-off for a showcase app).

### Solution options for dropdown presentation

#### Option G: Flat list (no visual grouping)

Append all discovered themes to the existing flat list.

**Pros:**
- Simplest dropdown change: just push more items onto the vec.
- Works with both GPUI's `SearchableVec<SharedString>` and iced's `pick_list`
  without widget modifications.
- The `Display` impl on `IconSetChoice` provides visual distinction through
  naming conventions: installed themes show bare names ("breeze", "char-white"),
  while bundled themes show "Material (bundled)", "Lucide (bundled)", and
  meta-selections show "default (...)", "system (...)".

**Cons:**
- On systems with many installed themes (20+), the list is long.
- No visual grouping between installed and bundled entries.

#### Option H: Grouped dropdown with section labels

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
  may not support heterogeneous items natively.  Would require widget-level
  changes or a custom delegate — scope creep beyond the icon theme fix.

### Verdict: Issue 2

**Option C (Directories= check) + Option F2 (cache at startup) + Option G
(flat list).**

1. **Option C over A, B, or D:** The `Directories=` line check is the sweet
   spot — spec-conformant, simple (one line scan, no parsing), and verified to
   correctly filter out all cursor-only themes on the actual system.  Option A
   produces false positives (cursor-only themes).  Option B is overkill (full
   INI parsing for diminishing returns).  Option D is strictly worse than C
   (heuristic, not spec-based, same IO class).

2. **Option F2 (cache):** Scan once at startup.  Icon themes are not installed
   while the app is running in any normal workflow.

3. **Option G over H (flat list):** The grouped dropdown (Option H) is better
   UX, but requires widget support for non-selectable separators that neither
   GPUI nor iced provides out of the box.  A flat list works with current
   widgets and the `Display` naming convention (`"breeze"` vs
   `"Material (bundled)"`) provides sufficient visual distinction.  Grouping
   can be added later as a presentation improvement without changing the
   underlying data model or API.

4. **Option E (text input) is rejected:** It defeats the purpose.  The whole
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
    /// It is only constructed via `default_icon_choice()`, which
    /// guarantees the theme is available (bundled sets are always
    /// available; freedesktop themes are checked via
    /// `is_freedesktop_theme_available` before returning this variant).
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

Note: the `System` variant calls `system_icon_theme()` on each display.  This
function is **cached** — it reads system configuration once and returns the
cached result on subsequent calls.  When the theme watcher fires, the
reapplication path calls `invalidate_caches()` before reloading, which clears
the cache so the next `system_icon_theme()` call re-detects.  This means the
`System` label automatically updates when the user changes their OS icon theme
mid-session (e.g. `breeze` → `breeze-dark` in KDE System Settings).

The GPUI showcase has an additional "gpui-component built-in (Lucide)" entry
that is framework-specific.  This can be handled locally in the showcase
without polluting the library type.

### Helper methods

```rust
impl IconSetChoice {
    /// The effective `IconSet` loading mechanism for this choice.
    ///
    /// For `Default`, returns the theme's icon set (caller passes it in).
    /// For `Freedesktop`, always returns `IconSet::Freedesktop`.
    /// For others, returns the corresponding bundled or system set.
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
    /// Returns `Some(name)` for `Default` and `Freedesktop` variants.
    /// Returns `None` for `System`, `Material`, `Lucide`.
    ///
    /// The caller is responsible for only passing the result to
    /// `IconLoader::theme()` when the effective icon set is
    /// `IconSet::Freedesktop`.  For bundled sets (Material, Lucide),
    /// the theme name is not used by the loader.
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

The caller passes `variant.defaults.icon_theme.as_deref()` — when the TOML has
no `icon_theme` key, the caller passes `None` and the function returns `System`.
This eliminates the separate `has_toml_icon_theme: bool` parameter and makes
invalid calls (meaningless `icon_set`/`icon_theme` with `false`) unrepresentable.

```rust
/// Determine the default icon set choice for a theme.
///
/// When the TOML specifies `icon_theme` (`Some`) and the theme is
/// available (bundled sets are always available; freedesktop themes are
/// checked via `is_freedesktop_theme_available`), returns
/// `Default(icon_theme)`.
///
/// When the TOML does not specify `icon_theme` (`None`), or the
/// specified freedesktop theme is not installed, returns `System`.
pub fn default_icon_choice(
    icon_set: IconSet,
    icon_theme: Option<&str>,
) -> IconSetChoice {
    let Some(icon_theme) = icon_theme else {
        return IconSetChoice::System;
    };
    let available = match icon_set {
        IconSet::Material | IconSet::Lucide => true,
        IconSet::Freedesktop => is_freedesktop_theme_available(icon_theme),
        IconSet::SfSymbols | IconSet::SegoeIcons => true,
    };
    if available {
        IconSetChoice::Default(icon_theme.to_string())
    } else {
        IconSetChoice::System
    }
}
```

Note: the `match` has no `_ => false` catch-all.  All five current `IconSet`
variants are listed explicitly so that adding a new variant produces a compiler
error, forcing the author to decide whether the new set's theme is "always
available" or needs a runtime check.

### Proposed function: `list_freedesktop_themes`

```rust
/// List installed freedesktop icon themes.
///
/// Scans `$XDG_DATA_DIRS/icons/` and `$XDG_DATA_HOME/icons/` for
/// subdirectories containing an `index.theme` file with a `Directories=`
/// line (per the freedesktop Icon Theme Specification).  This filters
/// out cursor-only themes that lack application icons.
///
/// Excludes `hicolor` (mandatory fallback) and `default` (typically a
/// symlink).  Returns a sorted, deduplicated list of theme directory
/// names.
///
/// Silently skips entries on IO errors (e.g. permission denied).
/// Returns an empty `Vec` on non-Linux platforms.
///
/// Note: themes installed via Flatpak or Snap may reside outside
/// standard XDG paths and will not be discovered.  This is an
/// acceptable limitation for showcase apps.
pub fn list_freedesktop_themes() -> Vec<String>
```

### How showcases use the API

**Theme re-application pattern (both showcases):**

```rust
// After loading the new theme's icon_set / icon_theme (Option<&str>):

// 1. Only re-derive the choice when the user is in "follow preset" mode.
if self.icon_set_choice.follows_preset() {
    self.icon_set_choice = default_icon_choice(icon_set, icon_theme.as_deref());
    update_dropdown(&self.icon_set_choice, ...);
}

// 2. ALWAYS reload icons — text color changes on dark/light switch affect
//    icon recoloring (fg_rgb parameter), even for explicit user choices.
reload_icons(&self.icon_set_choice, ...);
```

This is the critical fix: step 1 is conditional (only `Default`), step 2 is
unconditional (all modes).  The existing code does both unconditionally,
causing the overwrite bug.

**Dropdown construction pattern (both showcases):**

```rust
fn icon_set_dropdown_items(
    icon_set: IconSet,
    icon_theme: Option<&str>,
    installed_themes: &[String],  // cached from list_freedesktop_themes()
) -> Vec<IconSetChoice> {
    let mut items = Vec::new();

    // "default (X)" — only when TOML specifies an icon_theme and it's available.
    // When icon_theme is None, default_icon_choice() returns System, so we skip
    // adding a redundant Default entry.
    if let choice @ IconSetChoice::Default(_) = default_icon_choice(icon_set, icon_theme) {
        items.push(choice);
    }

    // "system (Y)" — always available
    items.push(IconSetChoice::System);

    // Installed freedesktop themes.
    // Note: if the preset's icon_theme (e.g. "Adwaita") also appears here,
    // both "default (Adwaita)" and "Adwaita" are shown intentionally — they
    // have different semantics.  "default" follows the preset and is re-derived
    // on theme change; the explicit entry is preserved across theme changes.
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
let data = if let Some(theme) = theme.filter(|_| set == IconSet::Freedesktop) {
    IconLoader::new(role).set(set).theme(theme).color_opt(fg).load()
} else {
    IconLoader::new(role).set(set).color_opt(fg).load()
};
```

Note: the `--icon-theme` CLI override is orthogonal to `IconSetChoice`.  It is
passed separately to the icon loader and takes priority within the freedesktop
set, regardless of the user's dropdown choice.

### What each showcase deletes

**GPUI showcase** — removes these items entirely:
- `use_default_icon_set` field (replaced by `IconSetChoice`)
- `resolve_default_icon_set()` (replaced by `choice.effective_icon_set()`)
- `resolve_default_icon_theme()` (replaced by `choice.freedesktop_theme()`)
- `default_icon_label()` (replaced by `choice.to_string()` via `Display`)
- `icon_set_internal_name()` string parsing (replaced by enum matching)

**Iced showcase** — removes these items entirely:
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
   - Guard `rebuild_theme()`: only overwrite `icon_set_choice` when
     `follows_preset()` is true.  Always reload icons unconditionally.
   - Add `installed_themes: Vec<String>` field, populated once at init.
   - Update `IconSetChoice::choices()` to include installed themes.
   - Handle `Freedesktop(name)` in the icon set selection message.

6. **GPUI showcase:**
   - Replace `use_default_icon_set: bool` with `IconSetChoice` from library.
   - Remove `resolve_default_icon_set()`, `resolve_default_icon_theme()`,
     `default_icon_label()`, and `icon_set_internal_name()`.
   - Rewrite the reapplication block at line 1802: only update choice and
     dropdown when `follows_preset()`.  Always reload icons.
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
   - Dark/light toggle recolors icons correctly for all choices (the icon
     reload is unconditional, only the choice update is conditional).
