---
phase: 75-non-exhaustive-compile-gate-iconset-default
verified: 2026-04-12T16:00:00Z
status: passed
score: 10/10
overrides_applied: 0
---

# Phase 75: non-exhaustive + compile-gate + IconSet::default removal — Verification Report

**Phase Goal:** Unit 6 part B: LAYOUT-02 + WATCH-03 + ICON-05 (non_exhaustive + compile-gate + IconSet::default removal)
**Verified:** 2026-04-12T16:00:00Z
**Status:** passed
**Re-verification:** No — initial verification

---

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| 1 | LinuxDesktop enum has #[non_exhaustive] attribute | VERIFIED | detect.rs line 5: `#[non_exhaustive]` present above `pub enum LinuxDesktop` |
| 2 | LinuxDesktop has Hyprland, Sway, River, Niri, CosmicDe variants | VERIFIED | detect.rs lines 22-31: all five variants with doc comments |
| 3 | detect_linux_de() maps XDG_CURRENT_DESKTOP values to the new variants | VERIFIED | detect.rs lines 60-64: `"Hyprland" => Hyprland`, `"sway" => Sway`, `"river" => River`, `"niri" => Niri`, `"COSMIC" => CosmicDe` |
| 4 | Matching LinuxDesktop without a wildcard arm produces a non-exhaustive compile error (external crates) | VERIFIED | #[non_exhaustive] on LinuxDesktop enforces this for any external crate; crate-internal matches list all variants exhaustively (confirmed by clippy -D warnings passing) |
| 5 | All new compositor variants map to the adwaita preset in pipeline dispatch | VERIFIED | pipeline.rs lines 308-316 (from_linux) and 407-415 (from_system_async_inner): all five new variants in the Xfce/Cinnamon/Mate/LxQt adwaita arm |
| 6 | watch/mod.rs match arms handle the new variants correctly | VERIFIED | watch/mod.rs line 202: `_` wildcard arm returns `WatchUnavailable` — correct since no watch backend exists for Wayland compositors |
| 7 | IconSet::default() no longer exists — calling it produces a compile error | VERIFIED | model/icons.rs line 274: derive line is `#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]` — no `Default`; no `#[default]` attribute on any variant |
| 8 | Removing Default from IconSet does not break serde deserialization | VERIFIED | model/mod.rs line 174: `pub icon_set: Option<IconSet>` — defaults to None via Option; test `icon_set_default_is_none` passes (ThemeVariant::default().icon_set is None) |
| 9 | on_theme_change is compile-gated behind #[cfg(feature = "watch")] | VERIFIED | lib.rs line 116: `#[cfg(feature = "watch")] pub mod watch;` and line 185: `#[cfg(feature = "watch")] pub use watch::{ThemeChangeEvent, ThemeWatcher, on_theme_change};` — without the feature the symbols are absent, producing a compile error |
| 10 | CHANGELOG documents IconSet::default() removal with system_icon_set() migration guidance | VERIFIED | CHANGELOG.md line 14: `- \`IconSet::default()\` -- use \`system_icon_set()\` for the platform-appropriate icon set` under [0.5.7] Removed / native-theme (core) |

**Score:** 10/10 truths verified

---

### Required Artifacts

| Artifact | Expected | Status | Details |
|----------|----------|--------|---------|
| `native-theme/src/detect.rs` | LinuxDesktop enum with #[non_exhaustive] and five Wayland compositor variants | VERIFIED | #[non_exhaustive] at line 5; Hyprland, Sway, River, Niri, CosmicDe at lines 22-31; detection mappings at lines 60-64 |
| `native-theme/src/pipeline.rs` | Preset mapping and pipeline dispatch for new LinuxDesktop variants | VERIFIED | New variants in from_linux() (lines 312-316) and from_system_async_inner() (lines 411-415); 6 detection tests at lines 582-613 |
| `native-theme/src/model/icons.rs` | IconSet enum without Default derive | VERIFIED | derive macro at line 274 has no Default; no #[default] attribute on any variant; five variants remain intact |
| `native-theme/src/lib.rs` | watch module and re-exports compile-gated behind watch feature | VERIFIED | Lines 116-117 and 185-186 gate module and re-exports |
| `native-theme/src/watch/mod.rs` | Match handles new variants via wildcard arm | VERIFIED | Wildcard at line 202 returns WatchUnavailable for unrecognized DEs including new compositor variants |
| `CHANGELOG.md` | Migration note for IconSet::default() removal | VERIFIED | Line 14 under [0.5.7] Removed section |

---

### Key Link Verification

| From | To | Via | Status | Details |
|------|----|-----|--------|---------|
| native-theme/src/detect.rs | native-theme/src/pipeline.rs | LinuxDesktop variants used in match arms | WIRED | Pattern `LinuxDesktop::(Hyprland\|Sway\|River\|Niri\|CosmicDe)` found at pipeline.rs lines 312-316 and 411-415 |
| native-theme/src/model/icons.rs | native-theme/src/model/mod.rs | icon_set: Option<IconSet> field defaults to None via Option | WIRED | mod.rs line 174: `pub icon_set: Option<IconSet>` — serde defaults to None, not IconSet::default() |
| native-theme/src/model/icons.rs | native-theme/src/model/icons.rs (detect_linux_icon_theme) | New variants handled in icon theme match | WIRED | icons.rs lines 573-577: Hyprland, Sway, River, Niri, CosmicDe added to GNOME/Budgie gsettings arm |

---

### Data-Flow Trace (Level 4)

Not applicable — this phase modifies enum definitions, detection logic, and compile-time gates. No new data-rendering components were introduced.

---

### Behavioral Spot-Checks

| Behavior | Command | Result | Status |
|----------|---------|--------|--------|
| cargo check compiles cleanly | `cargo check -p native-theme` | Finished dev profile in 0.15s | PASS |
| All 6 detection tests pass | `cargo test -p native-theme -- detect_hyprland detect_sway detect_cosmic detect_river detect_niri detect_cosmic_full_desktop` | 6 passed, 0 failed | PASS |
| All native-theme tests pass | `cargo test -p native-theme` | 38 passed (+ 7 ignored), 0 failed across all test suites | PASS |
| clippy with -D warnings | `cargo clippy -p native-theme -- -D warnings` | Finished dev profile in 1.95s, zero warnings | PASS |
| No Default in IconSet derive | grep for `Default` in model/icons.rs | No matches found | PASS |
| CHANGELOG entry present | grep for `IconSet::default` in CHANGELOG.md | Line 14: entry with system_icon_set() guidance | PASS |

---

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
|-------------|------------|-------------|--------|----------|
| LAYOUT-02 | 75-01-PLAN.md | LinuxDesktop #[non_exhaustive] with new Wayland compositor variants | SATISFIED | detect.rs has #[non_exhaustive], five new variants, detect_linux_de() mappings; pipeline dispatch updated |
| WATCH-03 | 75-02-PLAN.md | on_theme_change compile-gated behind watch feature | SATISFIED | lib.rs lines 116 and 185 gate module declaration and re-exports behind #[cfg(feature = "watch")] |
| ICON-05 | 75-02-PLAN.md | IconSet::default() removed | SATISFIED | Default removed from derive; no #[default] attribute; validate.rs updated to avoid Default trait bound |

---

### Anti-Patterns Found

No blockers or warnings. The `// Placeholder: never used because validate() short-circuits on missing` comment in validate.rs line 427 refers to a `IconSet::Freedesktop` value used only as a structural placeholder in a None branch that always short-circuits before the value is used — this is an intentional, documented pattern, not a stub.

---

### Human Verification Required

None. All phase deliverables are statically verifiable:
- Attribute presence on enum
- Variant membership
- Match arm coverage
- Derive macro contents
- Feature gating via #[cfg]
- Test execution results
- CHANGELOG content

---

## Gaps Summary

No gaps. All 10 observable truths are verified against the actual codebase. The phase goal is fully achieved:

- `LinuxDesktop` is `#[non_exhaustive]` with five new Wayland compositor variants
- All pipeline dispatch paths handle the new variants correctly (adwaita preset)
- `IconSet::default()` is a compile error — Default removed from the derive
- `on_theme_change`, `ThemeWatcher`, and `ThemeChangeEvent` are absent without the `watch` feature, making any call a compile error
- CHANGELOG documents the API removal with migration guidance

### Notable Deviation (informational, no impact)

Plan 75-01 Task 2 omitted the suggested wildcard arms in `from_linux()` and `from_system_async_inner()`. This was correct: `#[non_exhaustive]` does not require wildcard arms in same-crate matches, and adding them would have produced unreachable-pattern warnings (clippy -D warnings enforced). The variant list is fully explicit and exhaustive within the crate.

Plan 75-02 Task 1 inlined icon_set extraction in validate.rs instead of using the generic `require()` helper, because `require<T: Clone + Default>` required `Default` on IconSet. The inline match is functionally equivalent and correctly documented.

---

_Verified: 2026-04-12T16:00:00Z_
_Verifier: Claude (gsd-verifier)_
