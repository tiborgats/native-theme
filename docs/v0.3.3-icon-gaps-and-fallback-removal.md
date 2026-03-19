# Icon Gaps and Fallback Removal

## Current State After Lazy Fixes

Three mappings were added (commit `69c5650`):
- SF Symbols `WindowRestore` -> `"arrow.down.right.and.arrow.up.left"`
- SF Symbols `TrashFull` -> `"trash.fill"`
- Segoe `DialogSuccess` -> `"CheckMark"` (0xE73E)

## Remaining Gaps

After the fixes, the mapping coverage is:

| Icon Set | Coverage | Missing Roles |
|----------|----------|---------------|
| SF Symbols | 40/42 | `FolderOpen`, `StatusLoading` |
| Segoe Icons | 41/42 | `StatusLoading` |
| Freedesktop | 41/42 | `Notification` |
| Material | 41/42 | `TrashFull` |
| Lucide | 41/42 | `TrashFull` |

---

## 1. SF Symbols `FolderOpen`

**Problem:** SF Symbols has no "open folder" concept. Apple's icon vocabulary treats folders as a single state.

**Options:**

| Option | Symbol | Pros | Cons |
|--------|--------|------|------|
| A | `"folder.fill"` | Exists, visually distinct from `"folder"` | "Filled" means shading style, not "open" — semantically wrong |
| B | `"folder.badge.minus"` | Exists, visually distinct | Implies "remove from folder", not "open folder" |
| C | Map to `None` (keep as-is) | Honest — SF Symbols genuinely lacks this concept | Gap remains |
| D | Remove `FolderOpen` from `IconRole` | Eliminates the gap entirely | Breaking change; freedesktop and Windows DO have it |

**Proposal:** Option C. SF Symbols genuinely doesn't have an open folder icon. Mapping to a semantically wrong symbol is worse than returning None. The app can decide how to handle the absence (e.g. use `FolderClosed` as fallback, or skip the icon).

---

## 2. SF Symbols `StatusLoading`

**Problem:** Loading indicators are animated on Apple platforms. SF Symbols has no static "loading" glyph — the entire concept is an animated view (`ProgressView`), not an icon.

**Options:**

| Option | Symbol | Pros | Cons |
|--------|--------|------|------|
| A | `"arrow.triangle.2.circlepath"` | Conveys rotation/refresh concept | Not really "loading" — more "sync/refresh" |
| B | `"circle.dashed"` | Conveys incompleteness/waiting | Obscure, users won't recognize it as loading |
| C | `"ellipsis.circle"` | Activity indicator metaphor | Tiny dots, not a spinner |
| D | Map to `None` (keep as-is) | Honest — loading is animated, not static | Gap remains |

**Proposal:** Option D. A static glyph for loading is misleading. The purpose of a loading icon is to convey ongoing activity, which requires animation. The app should use its toolkit's native progress indicator, not a static icon.

---

## 3. Segoe `StatusLoading`

**Problem:** Same as SF Symbols — Windows progress rings are animated controls, not static icons.

**Options:**

| Option | Glyph | Pros | Cons |
|--------|-------|------|------|
| A | `"Sync"` (0xE72D) | Exists in Segoe Fluent | Means "sync", not "loading" |
| B | Map to `None` (keep as-is) | Honest | Gap remains |

**Proposal:** Option B. Same reasoning as SF Symbols. Loading is not a static icon on Windows.

---

## 4. Freedesktop `Notification`

**Problem:** The freedesktop icon naming specification does not include a notification/bell icon. `notification-active` exists in KDE themes (Breeze, Oxygen) but is a KDE convention, not in the spec. GNOME/Adwaita does not ship it.

**Options:**

| Option | Name | Pros | Cons |
|--------|------|------|------|
| A | `"notification-active"` | Works on KDE (Breeze, Oxygen) | Fails on GNOME/Adwaita/Papirus — returns no icon |
| B | DE-aware dispatch: `{ kde = "notification-active", default = ... }` | Correct per-DE | No standard default exists for non-KDE DEs |
| C | `"preferences-desktop-notification"` | In some themes as an app/category icon | It's a settings panel icon, not a notification bell |
| D | Map to `None` (keep as-is) | Honest — no universal freedesktop bell | Gap remains |
| E | Remove `Notification` from system icon loading for freedesktop, always use bundled for this role | Explicit about the gap | Would need the bundled-per-role mechanism from Chapter 6 |

**Proposal:** Option A with a caveat. `notification-active` is the closest match and ships in the most popular KDE themes. On GNOME, the freedesktop lookup will fail (icon not found in theme) and `load_freedesktop_icon` will return `None` — which is the correct behavior when a theme doesn't have an icon. The app can then decide what to do. This is strictly better than the current `None` because it works on KDE (the most common freedesktop case) and gracefully degrades on GNOME.

**Alternative proposal:** Option D if we want to be purist about spec compliance. `notification-active` is NOT in the spec, and themes are not required to ship it.

---

## 5. Material `TrashFull`

**Problem:** Material Symbols has `delete` (trash can) but no variant for "trash with contents". The bundled SVGs already map both `TrashEmpty` and `TrashFull` to the same `delete.svg`.

**Options:**

| Option | Approach | Pros | Cons |
|--------|----------|------|------|
| A | Map `TrashFull` to `"delete"` (same as `TrashEmpty`) | 42/42 coverage, consistent with bundled SVGs | Two roles map to the same icon — no visual distinction |
| B | Map `TrashFull` to `"delete_forever"` | Visually distinct (has an X on the trash) | Semantically wrong — "delete forever" ≠ "trash is full" |
| C | Keep as `None` | Honest | Gap in `icon_name()`, though `bundled_icon_svg()` handles it |

**Proposal:** Option A. Material already reuses `delete.svg` for both in the bundled set. Making `icon_name()` consistent with `bundled_icon_svg()` eliminates the discrepancy. The lack of visual distinction is a Material design choice, not our problem.

---

## 6. Lucide `TrashFull`

**Problem:** Same as Material. Lucide has `trash-2` but no full variant. Bundled SVGs already map both to `trash-2.svg`.

**Options:** Same as Material.

**Proposal:** Option A — map to `"trash-2"` (same as `TrashEmpty`), matching what `bundled_icon_svg()` already does.

---

## 7. Removing the Cross-Set Fallback Pattern

### Where it exists today

Four locations silently substitute Material SVGs when the requested set fails:

| File | Line | Code |
|------|------|------|
| `freedesktop.rs` | 71-72 | `bundled_icon_svg(IconSet::Material, role).map(...)` |
| `sficons.rs` | 158-159 | `bundled_icon_svg(IconSet::Material, role).map(...)` |
| `winicons.rs` | 401-402 | `bundled_icon_svg(IconSet::Material, role).map(...)` |
| `lib.rs` | 424-429 | wildcard `_` branch in `load_icon()` falls back to Material |

### What the fix looks like

Each platform loader should return `None` when it can't find the icon:

```rust
// freedesktop.rs — BEFORE
pub fn load_freedesktop_icon(role: IconRole) -> Option<IconData> {
    if let Some(name) = icon_name(IconSet::Freedesktop, role)
        && let Some(path) = find_icon(name, &theme, 24)
        && let Ok(bytes) = std::fs::read(&path)
    {
        return Some(IconData::Svg(bytes));
    }
    // Bundled Material SVG fallback
    bundled_icon_svg(IconSet::Material, role).map(|bytes| IconData::Svg(bytes.to_vec()))
}

// freedesktop.rs — AFTER
pub fn load_freedesktop_icon(role: IconRole) -> Option<IconData> {
    let name = icon_name(IconSet::Freedesktop, role)?;
    let path = find_icon(name, &theme, 24)?;
    let bytes = std::fs::read(&path).ok()?;
    Some(IconData::Svg(bytes))
}
```

Same pattern for `sficons.rs` and `winicons.rs` — remove the `bundled_icon_svg` fallback line, return `None`.

For `lib.rs` `load_icon()`, the wildcard branch becomes:

```rust
// BEFORE
_ => {
    #[cfg(feature = "material-icons")]
    {
        return bundled_icon_svg(IconSet::Material, role)
            .map(|b| IconData::Svg(b.to_vec()));
    }
    #[cfg(not(feature = "material-icons"))]
    { None }
}

// AFTER
_ => None,
```

### Tests that need updating

These tests explicitly assert the fallback behavior and must be removed or rewritten:

| File | Test | Current assertion |
|------|------|-------------------|
| `freedesktop.rs` | `load_icon_notification_uses_bundled_fallback` | Asserts Notification returns Some (Material SVG) |
| `sficons.rs` | `fallback_for_unmapped_role` | Asserts WindowRestore returns Some (Material SVG) |
| `winicons.rs` | `fallback_for_unmapped_role` | Asserts DialogSuccess returns Some (Material SVG) |

After the mapping fixes, `WindowRestore` and `DialogSuccess` are no longer unmapped, so those two tests become wrong regardless. The freedesktop `Notification` test should assert `None` (or `Some` if we go with Option A above).

### Doc comments to update

All three platform loaders have doc comments describing the "Fallback chain" with "Bundled Material SVGs" as tier 2. These must be updated to say the function returns `None` when the icon isn't available.

---

## 8. Build-Time Coverage Checks

### Problem

Currently, icon mapping gaps are only discoverable at runtime — an app calls `load_icon(IconRole::Notification, "freedesktop")` and gets `None`. The developer might not notice until a user reports a missing icon in production.

### Options

#### Option A: Compile-time `const` assertion in native-theme itself

Add a `const` function that verifies every `IconRole` variant has a mapping for every `IconSet`:

```rust
#[cfg(test)]
mod coverage_tests {
    use super::*;

    #[test]
    fn all_roles_mapped_for_all_system_sets() {
        let sets = [IconSet::SfSymbols, IconSet::SegoeIcons, IconSet::Freedesktop];
        for &set in &sets {
            for &role in IconRole::ALL.iter() {
                assert!(
                    icon_name(set, role).is_some(),
                    "{role:?} has no mapping for {set:?}"
                );
            }
        }
    }
}
```

**Pros:** Catches gaps immediately when a new `IconRole` is added. CI fails if coverage is incomplete.
**Cons:** Forces 100% coverage, which may not be achievable for genuinely missing icons (StatusLoading). Requires `#[allow]` annotations or an explicit exceptions list.

#### Option B: Coverage test with explicit exception list

```rust
#[cfg(test)]
fn known_gaps() -> &'static [(IconSet, IconRole)] {
    &[
        (IconSet::SfSymbols, IconRole::FolderOpen),
        (IconSet::SfSymbols, IconRole::StatusLoading),
        (IconSet::SegoeIcons, IconRole::StatusLoading),
        // no more — all others must be mapped
    ]
}

#[test]
fn no_unexpected_icon_gaps() {
    let gaps = known_gaps();
    let sets = [IconSet::SfSymbols, IconSet::SegoeIcons, IconSet::Freedesktop];
    for &set in &sets {
        for &role in IconRole::ALL.iter() {
            let is_known_gap = gaps.contains(&(set, role));
            let is_mapped = icon_name(set, role).is_some();
            if !is_known_gap {
                assert!(
                    is_mapped,
                    "{role:?} has no mapping for {set:?} and is not in known_gaps()"
                );
            }
        }
    }
}
```

**Pros:** Allows genuine gaps while catching accidental omissions. Adding a new `IconRole` variant without updating all mapping functions fails the test. The exception list is reviewable and must be explicitly maintained.
**Cons:** Someone could add gaps to the exception list to silence the test.

#### Option C: Build-time check in native-theme-build for custom icons

The build crate (`native-theme-build`) already validates that every role in a custom icon TOML has a mapping. Extend this to also check that every `IconRole` variant has a bundled SVG in `bundled_icon_svg()`:

```rust
// In native-theme-build or as a test in native-theme
#[test]
fn all_roles_have_bundled_svg() {
    for &role in IconRole::ALL.iter() {
        for set in [IconSet::Material, IconSet::Lucide] {
            assert!(
                bundled_icon_svg(set, role).is_some(),
                "{role:?} has no bundled SVG for {set:?}"
            );
        }
    }
}
```

This ensures that even if a system icon is missing, the app can always load the bundled version explicitly (if it wants to).

#### Option D: App-side build.rs check

Apps that use `native-theme` can add a build-time check in their `build.rs`:

```rust
// In the app's build.rs
fn main() {
    // Verify all icon roles we use have mappings
    let roles = native_theme::IconRole::ALL;
    let set = native_theme::system_icon_set();
    for role in roles {
        if native_theme::icon_name(set, *role).is_none() {
            panic!("IconRole::{role:?} has no mapping for {set:?}");
        }
    }
}
```

**Pros:** App controls which gaps are acceptable. Different apps may have different requirements.
**Cons:** Requires every app to set this up. Easy to forget.

### Proposal

**Option B** for native-theme itself (catches regressions at CI time with explicit exception list), plus **Option C** for bundled SVG completeness (ensures bundled sets are always 42/42). Together they guarantee:

1. Every new `IconRole` variant must be added to ALL five mapping functions or explicitly listed as a known gap
2. Every `IconRole` has a bundled SVG in both Material and Lucide sets (even if it reuses another icon like TrashFull -> delete.svg)
3. The known gaps list is small, visible, and reviewed in PRs

This shifts the "missing icon" discovery from runtime to CI, and makes it impossible to ship a new role without consciously deciding its mapping for every platform.
