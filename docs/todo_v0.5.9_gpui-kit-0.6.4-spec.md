# v0.5.9 — gpui connector on GPUI Kit 0.6.4: Specification

Status: Design (2026-09-19); nothing implemented
Target toolkit: **gpui-component 0.6.4**, **gpui-base 0.6.4**, GPUI as the
**`gpui-pre` 0.3.5** package; **gpui-kit 0.6.4** for the showcase and tests.
Older GPUI Kit versions are not supported.
Companion rationale:
[`todo_v0.5.9_gpui-kit-0.6.4-rationale.md`](todo_v0.5.9_gpui-kit-0.6.4-rationale.md)
(decisions E1–E21)
Companion plan:
[`todo_v0.5.9_gpui-kit-0.6.4-plan.md`](todo_v0.5.9_gpui-kit-0.6.4-plan.md)

This document amends the v0.5.8 specification
(`archive/todo_v0.5.8_gpui-component-0.6-spec.md`); a section not mentioned here is
unchanged. "§N (0.5.8)" refers to that document.

---

## 0 -- Scope

### 0.1 What v0.5.9 delivers

1. The connector builds, lints and tests clean against the 0.6.4 stack, and
   requires it (§2, §3).
2. Reduced motion cooperates with gpui-base's own OS reader instead of
   overwriting it (§5).
3. Two mechanical guards replace claims that only a reader could check:
   rendered seam tests for the six builders that set a widget's height (§6),
   and one test that names every field of `Theme` (§3a).
4. Every upstream citation in the connector is true for 0.6.4 / 0.3.5 (§7).
5. Documentation, changelog, lockfile, version 0.5.9 (§8, §9).
6. The showcase renders what 0.6.4 added or changed: three new components,
   a code editor, a Markdown view (§8a).
7. The v0.5.8 design documents are archived (§8b).

### 0.2 Constraints

- No panics, no `unsafe`, no invented values, no hardcoded theme values: as
  in §0.2 (0.5.8). The repository hooks apply to test code under `src/`;
  `tests/` files may use `expect` in test functions.
- Two public items are added: the module `variants` with `ghost_button`
  (§8c), and `geometry::input_group_button` (rationale §1.4n, E21).
  Nothing is removed or re-typed. Two behaviour changes: reduced motion (§5)
  and the source of the `accent` and `sidebar_accent` pairs (rationale §1.4n,
  E20; two tests in `src/colors.rs` on the presets where old and new differ).
- No release action (tag, upload, GitHub release) without the maintainer's
  explicit go.

### 0.3 Out of scope

Icon tables keyed by `gpui_kit_assets::IconName`; Lucide bundle refresh;
installed-font checks for preset families; exact version pins; anything in
`native-theme` core or the iced connector (rationale §2.5, §2.8, §4).

---

## 1 -- Facts this specification rests on

Measured 2026-09-19 on a throwaway worktree of `main` `9360978` ("the probe")
or read from the published crate sources.

| Fact | Evidence |
|---|---|
| Newest: gpui-kit / gpui-component / gpui-base / gpui-kit-assets 0.6.4 (2026-09-18), gpui-pre 0.3.5 (2026-09-14) | crates.io API |
| gpui-component 0.6.4 requires `gpui-pre ^0.3.5`, `gpui-base ^0.6.4` | published `Cargo.toml:277-282` |
| `ThemeColor::tiles`, `ThemeConfigColors::tiles`, `Theme::{tile_grid_size, tile_shadow, tile_radius}` exist in 0.6.1 and not in 0.6.2 / 0.6.4 | `src/theme/theme_color.rs`, `schema.rs`, `mod.rs` of each version |
| With the lockfile on 0.6.4 / 0.3.5 the connector fails with exactly three `E0609 no field tiles` errors (`src/colors.rs:487`, `src/config.rs:189` twice) | probe pass 1; canary run 35393538690 |
| With those lines and the showcase swatch removed: clippy `--all-targets --all-features -D warnings` clean; 187 tests pass; the 3 failures are `theme_color_field_count_tripwire`, `no_theme_color_field_is_left_at_default` (139 expected) and `theme_config_colors_cover_the_0_6_fields` (127 expected) | probe pass 2 |
| `ThemeColor` has 138 fields (139 − `tiles`); the connector exports 126 config colours (127 − `tiles`); no field was added | the three tripwires fail at exactly these numbers and pass once they are changed (probe) |
| Rust 1.95.0 builds the library on the new closure; 1.94.0 does not (`std::hint::cold_path` unstable, gpui-pre 0.3.5 `src/profiler.rs:473, 494`) | `cargo +1.95.0 check --lib --locked`; `cargo +1.94.0 check --lib --locked --ignore-rust-version` |
| The lockfile loses 13 packages and gains six: `objc2-screen-capture-kit` and, through the new dev feature, `tree-sitter`, `tree-sitter-json`, `tree-sitter-language`, `tree-sitter-rust`, `streaming-iterator`. No `-sys` crate is added | `cargo update` output, and `diff` of the `name =` lines of both lockfiles |
| `Theme` has 20 public fields and is not `#[non_exhaustive]`; between 0.6.0 and 0.6.4 it lost the three `tile_*` fields and gained none; `ThemeConfig` is field-identical; `ThemeTokens` is generated from `ThemeColor`'s name list by `define_theme_tokens!` and lost the `tiles` token with it (139 → 138) | field lists of both versions, compared; `theme/theme_color.rs:343-374` |
| 0.6.4 applies `.max_h(<viewport remainder>)` to the dialog *after* `refine_style`, so `geometry::dialog`'s `max_h` no longer reaches the widget; `min_h_24()` still runs before it, so `min_h` does | gpui-component 0.6.4 `src/dialog/dialog.rs:535, 621, 631`; 0.6.0 has no such line |
| `ThemeColor::list_active_border` is still a field but no 0.6.4 widget reads it (0.6.0 drew it as the selected-row outline, `list/list_item.rs:249-255`) | `grep -rl list_active_border` in both trees |
| `cargo audit` on the moved lockfile reports `RUSTSEC-2026-0285` (`rustls 0.23.43`), which `main` has too; `cargo update -p rustls` → 0.23.45 clears it and locks one package | `cargo audit` before and after, on the probe |
| `to_theme` leaves `Theme::{notification, list, sheet, motion}` at `Theme::from(&ThemeColor)`'s defaults; `ResolvedTheme` has no field any of them could be mapped from | `src/lib.rs:154-198`; gpui-component `theme/mod.rs:659-682`; `native-theme/src/resolve/` |
| gpui-base 0.6.2+ reads the OS reduced-motion preference in `init` and owns `App::reduce_motion` while the flag equals its last write; `apply_system_reduce_motion` is public and is a no-op under GPUI's test scheduler | gpui-base 0.6.4 `src/reduce_motion.rs:48-50, 65-75, 79-85, 89-101`; `src/lib.rs` re-export |
| `Theme::change` probes fonts only when the family is upstream's default name | gpui-component 0.6.4 `src/theme/mod.rs:279-280`, `system_font.rs:33-36`, `mono_font.rs` `resolve_default_mono_font` |
| The component `IconName` set is the 101 names of 0.6.1, which v0.5.8 verified equal to 0.6.0 | `gpui-kit-assets/default-icons.txt` identical in 0.6.1 and 0.6.4 |
| Headless rendered measurements at text scale 1.5, unstyled → with the builder's refinement: `Button` 32 → 40, `Input` 32 → 34, `ListItem` 34 → 30, `Progress` 8 → 6 px (kde-breeze, light); `Select` and `Combobox` 32 → 34 px (adwaita, light; with kde-breeze `Select` measures 32 both ways). With the `refine_style` call removed the test fails (run on `input` and `combobox`) | probe, the file of §6 run as written |
| `MenuItemElement` is `pub(crate)` in a private module in 0.6.0 and 0.6.4; an application cannot construct one | gpui-component `src/menu/menu_item.rs:10-11`, `src/menu/mod.rs:6` |
| `apply_config` (hence `Theme::change`) rebuilds `Theme::tokens` from the config's colours; `tokens.accent` is read by the ghost button, menu items, toggles and table cells alike | gpui-component 0.6.4 `src/theme/schema.rs:1103`; `button/button.rs:1126`, `menu/menu_item.rs:117, 121`, `button/toggle.rs:155, 202`, `table/state.rs:2198` |
| gpui-base `src/theme.rs`, `src/styled.rs`, `src/resizable/resize_handle.rs`, gpui-pre `src/color.rs`, `src/style.rs`: byte-identical 0.6.0 → 0.6.4 / 0.3.3 → 0.3.5. gpui-base `src/scrollbar.rs`: no `pub` line changed | `cmp`; `diff \| grep pub` |

---

## 2 -- Manifest and lockfile

`connectors/native-theme-gpui/Cargo.toml`:

```toml
# NOT inherited: the gpui-pre closure declares a higher floor than the
# workspace. Set to the lowest toolchain that compiles the library; measured
# 2026-09-19 on the 0.6.4 closure: 1.95.0 builds, 1.94.0 fails on
# std::hint::cold_path in gpui-pre 0.3.5 (src/profiler.rs).
rust-version = "1.95.0"
```

```toml
[dependencies]
# GPUI under the package name gpui-component uses, so both edges unify; 0.3.5
# is the minimum gpui-component 0.6.4 accepts. gpui-component does not
# re-export gpui, so the connector names it. Caret: an exact pin in a library
# would conflict with any consumer whose other dependencies want a later
# snapshot.
gpui = { package = "gpui-pre", version = "0.3.5" }
# A hard floor: 0.6.2 removed ThemeColor::tiles, which older connector code
# wrote, and added the reduced-motion reader this crate cooperates with.
gpui-component = "0.6.4"
# For gpui_base::Theme, ScrollbarTheme, ResizableTheme and
# apply_system_reduce_motion (not re-exported).
gpui-base = "0.6.4"
```

```toml
[dev-dependencies]
# Headless App for the apply/observer tests (#[gpui::test], TestAppContext).
gpui = { package = "gpui-pre", version = "0.3.5", features = ["test-support"] }
# The showcase uses the facade an application would: application(), init(),
# and the Lucide assets under gpui_kit::assets. tree-sitter-rust: the grammar
# for the showcase's code editor section.
gpui-kit = { version = "0.6.4", features = ["tree-sitter-rust"] }
```

Only the version numbers, the feature list and the comment lines shown change;
the two existing dev-dependency comments stay.

Everything else in the manifest is unchanged (features, docs.rs metadata, the
showcase example entry).

`Cargo.lock`: `cargo update -p gpui-kit -p gpui-component -p gpui-base
-p gpui-kit-assets -p gpui-pre`, run after the manifest edit so that the
tree-sitter crates the new dev feature needs are added in the same step;
committed. One further package is updated, for a reason that is not
compatibility: `cargo update -p rustls` (0.23.43 → 0.23.45) clears
`RUSTSEC-2026-0285`, which `main` already carries and which would fail
`ci.yml:117-123`'s hard `cargo audit` job on the release commit (E19).
Nothing else is updated. `Cargo.lock` is a stamped path, so
the asset stamp gate applies (§9).

Workspace `Cargo.toml`: `[workspace.package] version = "0.5.9"` and every
in-workspace dependency requirement that names `0.5.8` (found with
`grep -rn '0\.5\.8' --include=Cargo.toml .`).

---

## 3 -- `tiles`

| File | Change |
|---|---|
| `src/colors.rs:487` | delete `tc.tiles = c.bg;` and the blank line that separated it |
| `src/config.rs:189` | delete `colors.tiles = h(tc.tiles);` |
| `examples/showcase-gpui.rs:5528` | delete the `tiles` swatch; the `window_border` swatch becomes the last child of its row |
| `src/colors.rs` tests (`:1031-1057`) | 139 → 138, comment names 0.6.4 |
| `src/config.rs` tests (`:344-354`) | 127 → 126; comment arithmetic `138 − 12` |
| Every prose "139" / "127" in `src/colors.rs:1-3, 141`, `src/config.rs:5, 19, 87`, `src/lib.rs:49, 122, 128`, `README.md:13, 282, 306`, showcase `:21` | 138 / 126 |

The tripwire tests are the gate: they fail before the number changes and pass
after. `no_theme_color_field_is_left_at_default` additionally proves that no
*new* `ThemeColor` field appeared that the mapping leaves at zero.

## 3a -- A guard for `Theme`'s own shape

The removed fields were `Theme` fields, not `ThemeColor` fields, and nothing in
the connector ever mentioned them (rationale §1.4j, §2.12). One test in
`src/lib.rs`'s existing test module destructures a `Theme` exhaustively, so the
field list the connector was written against is checked by the compiler:

- every field is bound to `_`, grouped by four comments — set by `to_theme`;
  the `light_theme` / `dark_theme` pair, of which `to_theme` fills only the
  slot for `mode` and leaves the other at `ThemeConfig::default()` until a
  later `apply` stores it (D34); written by `Theme::from(&ThemeColor)`; and
  deliberately left at upstream's default (`notification`, `list`, `sheet`,
  `motion`: no native-theme receiver);
- an upstream field *added* in a patch release becomes
  `error[E0027]: pattern does not mention field …` (measured on the probe by
  deleting `motion: _`), a field *removed* becomes an unknown-field error;
- the test has no runtime assertion and no `expect`: the compiler is the gate.

The complete test is in the plan, Task 1.

---

## 4 -- Styling seams in 0.6.4

The `geometry` builders (§9 (0.5.8)) are unchanged. Their upstream evidence
moves; every widget still applies the caller's style after its own defaults:

| Builder | File (gpui-component `src/`) | `refine_style` line 0.6.0 → 0.6.4 |
|---|---|---|
| `button` | `button/button.rs` | 650 → 690 (icon/label variants 712, 720 → 752, 760; new crate-private `content_style` at 712 is not reachable) |
| `input`, `input_height` | `input/input.rs` | 587 → 719 |
| `menu_item` | `menu/menu_item.rs` | 109 → 111 |
| `list_item` | `list/list_item.rs` | 196 → 193 |
| `status_bar` | `status_bar.rs` | 95 → 96 |
| `dialog` | `dialog/dialog.rs` | 582 → 621; **but** `:631` re-applies `.max_h(<viewport remainder>)` after the refinement (computed at `:535`, new in 0.6.4), so only `min_h`, `px` and `py` still reach the widget (rationale §1.4k, E18) |
| `dialog_footer` | `dialog/footer.rs` | 51 → 56 |
| `dialog_title` | `dialog/title.rs` | 45 → 45 |
| `group_box_content` | `group_box.rs` | 161 → 161 |
| `checkbox` | `checkbox.rs` | 271 → 286 |
| `radio` | `radio.rs` | 211 → 226 |
| `select` | `select.rs` | 490 → 546 |
| `combobox` | `combobox.rs` | 992 → 997 |
| `title_bar` | `title_bar.rs` | 342 → 343 |
| rem root (§3.4 (0.5.8)) | `root.rs` | `set_rem_size` 579 → 582 |
| `tooltip`, `popover`, `dialog_description`, `table`, `progress`, `accordion_title` | | file byte-identical, citations stand |
| `spinner_size` | `spinner.rs` (byte-identical) **plus `icon.rs`** | `spinner.rs:53-65` stands; its second citation `icon.rs:160, 189` does not — `icon.rs` was rewritten in 0.6.4 (the enum now comes from gpui-kit-assets) and the two size arms became one at `:182`. Re-read under §7 |

The *defaults* line ranges each doc comment cites before the arrow
(e.g. `:587-624` for `Button`) are re-read and rewritten during §7; this table
fixes only what was verified while designing.

Noted, no action: upstream moved every control label to a 1.25 line box in
0.6.4 — `Checkbox` / `Radio` 1.2 → 1.25 (`checkbox.rs:329`, `radio.rs:250`,
with the inner `line_height(relative(1.))` on the label removed) and 1.0 →
1.25 on the `Button` root (`button/button.rs:689`), `DialogTitle`
(`dialog/title.rs:44`), `DialogFooter`, the `GroupBox` title, `Tag`,
`ToggleButton` and `Tab`. The connector sets text size, never line height, on
any of them, so the mapping does not change; the intrinsic height of these
rows grows slightly wherever the connector sets no height of its own.

---

## 5 -- Reduced motion

### 5.1 Semantics

`AccessibilityPreferences::reduce_motion` is a *request*, not the state of
the flag:

| `prefs.reduce_motion` | `App::reduce_motion` / connector state | Effect |
|---|---|---|
| `true` | flag off | switch the flag on; remember that the connector did |
| `true` | flag already on (whoever set it) | nothing |
| `false` | the connector had switched it on | switch the flag off, forget, then call `gpui_base::apply_system_reduce_motion(cx)` |
| `false` | otherwise | call `gpui_base::apply_system_reduce_motion(cx)` only: the connector writes nothing; gpui-base re-reads the OS and writes only if it still owns the flag |

Applies identically to `apply`, `apply_system_theme` and both branches of
`apply_accessibility`, through one private function.

### 5.2 Shape

```rust
/// Whether the connector switched `App::reduce_motion` on and has not yet
/// switched it back (spec v0.5.9 §5).
#[derive(Default)]
struct ReduceMotionRequest(bool);

impl Global for ReduceMotionRequest {}

fn forward_reduce_motion(reduce: bool, cx: &mut App) {
    if reduce {
        if !cx.reduce_motion() {
            cx.default_global::<ReduceMotionRequest>().0 = true;
            cx.set_reduce_motion(true);
        }
        return;
    }
    if std::mem::take(&mut cx.default_global::<ReduceMotionRequest>().0) {
        cx.set_reduce_motion(false);
    }
    gpui_base::apply_system_reduce_motion(cx);
}
```

Private; `default_global` for every access (no panicking accessor). The two
`cx.set_reduce_motion(prefs.reduce_motion)` calls (`src/lib.rs:697, 756`)
become `forward_reduce_motion(prefs.reduce_motion, cx)`. Run in the probe as
written: clippy clean, all tests pass.

### 5.3 Tests (in `src/lib.rs`, `mod apply_tests`, `#[gpui::test]`)

| Test | Asserts |
|---|---|
| `apply_with_default_prefs_keeps_a_flag_it_did_not_set` | `cx.set_reduce_motion(true)`; `apply(.., &default(), cx)`; flag still `true`. Fails on today's code. |
| `releasing_a_request_keeps_a_flag_that_was_already_on` | flag `true`; `apply_accessibility(true)` then `(false)` → still `true`. Fails on today's code. |
| `a_repeated_request_is_released_once` | request `true` twice → `true`; `false` → `false`. |
| (not testable) the re-read | `apply_system_reduce_motion` returns at once under GPUI's test scheduler, so no headless test can observe it; it rests on upstream's contract (`reduce_motion.rs:52-64`) and upstream's own tests |
| existing `apply_accessibility_forwards_reduce_motion`, `apply_accessibility_rescales_from_the_stored_variant`, and the `assert!(cx.reduce_motion())` in the first `apply` test (`:1451`) | unchanged and still pass |

### 5.4 Documented limits (doc comment of `apply`, README "Accessibility")

- A flag that is already on is left to whoever switched it on (gpui-base
  following the OS, or the application). Every `false` asks gpui-base to
  re-read the OS, so on macOS and Windows, which gpui-base does not follow
  live, re-applying the theme picks up a changed OS setting. gpui-base decides
  by value, not by provenance (`reduce_motion.rs:93-100`): an application flag
  that happens to equal gpui-base's own last reading is taken for gpui-base's
  own and is replaced when the OS reading changes.
- While the connector's request stands, gpui-base discards OS readings (the
  flag differs from its last write), including the Linux portal's first answer
  if `apply` ran before it arrived. After the request is withdrawn, macOS and
  Windows are re-read at once; Linux follows again from the portal's next
  change signal.
- To force the flag either way regardless of the OS, call
  `cx.set_reduce_motion` after `apply`.

README row (`README.md:206`) becomes: "`reduce_motion` | `true` switches
`App::reduce_motion` on if it is off; a later `false` undoes only that, then
gpui-base (which reads the OS preference itself since 0.6.2) re-reads the OS and decides. GPUI's
animations and gpui-component's spinner, shimmer, progress and marker honour
the flag".

---

## 6 -- Rendered seam tests

### 6.1 What is tested, and why only this

Six builders write a height (or minimum height) on a widget's root, where a
reordered upstream render would win silently. They get a rendered test:
`button`, `input`, `select`, `combobox` (same refinement as `select`,
different upstream seam), `list_item`, `progress`.

What each proves is not the same, and the file's header says so: `button`,
`input` and `progress` write the same `size.height` their widget writes for
itself, so they fail if upstream ever applies the caller's style first;
`select` and `combobox` write `min_size.height`, and `list_item` a height the
widget leaves content-driven, so those three prove the refinement reaches the
root box and wins there.

Not rendered, although they set a height: `accordion_title` (goes to an inner
title element with no selector; `accordion.rs` is byte-identical in 0.6.4),
`dialog` (`min_h` of an overlay that must be open; its `max_h` no longer
reaches the widget at all, §4), `menu_item` (no upstream receiver, §8). The
rest set text size, padding, gaps or radii, except the widths `tooltip`
(`max_w`) and `button`, `progress`, `select` (`min_w`) set, against which
upstream writes nothing that could compete. All of these keep the source
citation in their doc comment as evidence, and nothing more is claimed for
them.

### 6.2 Harness

New file `connectors/native-theme-gpui/tests/seams.rs` (integration test,
public API only; the complete file is in the plan, Task 3, and was run in the
probe as written). One view renders

```
div().size(px(600.)).flex().items_start()
  └ div().debug_selector(|| "probe".into()).flex_none()
      └ <the widget, with or without refine_style(&builder(native))>
```

in `cx.add_window_view`, draws once, and reads `cx.debug_bounds("probe")`.

Setup per measurement: resolve the preset through
`ResolutionContext::for_tests()` (`native-theme/src/resolve/context.rs:77-83`),
build the theme with `to_theme` and install it with `native_theme_gpui::apply`,
after `gpui_component::init(cx)`, so both measurements run under the native
theme. The test must **not** use `from_preset`, which resolves through
`ResolutionContext::from_system()` (`:62-72`): that reads the machine's font
DPI (KDE `forceFontDPI`, `Xft.dpi`, xrandr, else 96), the presets declare font
sizes in points, and the expected height is computed from the resolved font —
so on a 72-DPI session `control_height` would fall back to the preset minimum,
the expected value would equal the unstyled one and the `assert_ne!` guard
would fire for a reason that has nothing to do with upstream. With the pinned
context the measurements are the same on every machine.

No display is needed (GPUI test platform) and no new dependency
(`gpui-pre/test-support` is already a dev-dependency).

### 6.3 The rule every test obeys

1. measure the widget **without** the refinement → `u`;
2. measure it **with** the refinement → `s`;
3. take the expected value `e` from the refinement's own field
   (`size.height`, or `min_size.height` for `select` and `combobox`), never
   from a literal;
4. assert `u != e`: the input discriminates. If a preset change ever makes
   them equal the test fails and says so, instead of passing vacuously;
5. assert `s == e`.

Inputs: text scale 1.5 (the v0.5.8 test input that takes `control_height`
past a preset's minimum), light variant; preset `kde-breeze`, except `select`
and `combobox`, which use `adwaita` because Breeze's combo-box minimum equals
upstream's own height.

The measured numbers do not depend on the machine's fonts: each unstyled value
is a rem multiple or a line height, and GPUI's default line height is the
constant φ (`gpui-pre 0.3.5 src/style.rs:494`), not a font metric. The harness
renders no `Root`, so the rem size is GPUI's own 16 px rather than the theme's
font size; that changes only `u`, never the refinement's `e`.

### 6.4 Where it runs

`cargo test -p native-theme-gpui` picks the file up, so CI (`ci.yml:85-101`),
the publish gate (`publish.yml:74`), the nightly canary
(`dependency-canary.yml:85`) and `pre-release-check.sh:478` all run it with no
workflow change, and all four run it with default features, which is how the probe ran it
(194 library tests, 6 seam tests). In `pre-release-check.sh` the connector's
tests are a *soft* check (`:477-478`), so a failure warns there; in the other
three it fails the run.

---

## 7 -- Citations

Every upstream `file:line` reference in `connectors/native-theme-gpui/src/*.rs`,
`examples/showcase-gpui.rs` and `README.md` is opened in the 0.6.4 / 0.3.5
sources. The search must catch two forms: a citation naming its file, and a
citation continued on the next line in the bare form `` `:658-666` ``. The
first `grep -n '0\.6\.0\|0\.3\.3\|[a-z_/]*\.rs:[0-9]'` matches 69 lines and
misses six of the second kind, five of whose ranges are already wrong;
`grep -nE '0\.6\.0|0\.3\.3|[a-z_/]+\.rs:[0-9]|`:[0-9]+(-[0-9]+)?`|\(:[0-9]+'`
matches 75, the same 69 plus exactly those six (the freedesktop table excluded
in both). Each is then:

- rewritten to the new line(s) if the cited code still says what the comment
  claims;
- if it does not, the *claim* is the finding: stop and report, do not reword
  the comment to fit.

Version wording: each module's `//!` header names the stack once
("verified against gpui-component 0.6.4, gpui-base 0.6.4, gpui-pre 0.3.5");
inline citations drop the repeated version. References to gpui-component
**0.5.1** (`src/colors.rs:300`, the origin of the button mapping) are
historical and stay. `src/lib.rs:137` ("Planned for unification in v0.6.0")
is not an upstream citation and stays.

The six bare-form citations, with what they must become:

| Connector line | Cites | 0.6.4 / 0.3.5 |
|---|---|---|
| `src/geometry.rs:64` | `button/button.rs:658-666` (the label's text size) | `:698-706` |
| `src/lib.rs:842` | `theme/mod.rs:321-325` (`Theme::sync_base`) | `:367-371` (`:321` now holds a scrollbar closure) |
| `src/colors.rs:541`, `src/config.rs:97`, `src/lib.rs:840` | `schema.rs:687-695` (`apply_color!`) | `:688-696` |
| `src/config.rs:97`, `src/lib.rs:840` | `schema.rs:1074-1078` (the default-colour branch) | `:1075-1079` |
| `src/lib.rs:868` | gpui-pre `app.rs:1817-1821` (`apply_notify_global_observers_effect`) | 0.3.5 `:1860-1864` |

Known targets beyond §4: `schema.rs:1066-1073` (highlight fallback),
`schema.rs:640-674` (the 12 private palette fields — the range three connector
comments give, `:657-668`, was never right and must be replaced, not shifted),
`schema.rs:357`
(`group_box_title_foreground`), `theme/mod.rs:268-300, 283-298, 247-256`
(base projection, now carrying the mobile branches), gpui-base
`scrollbar.rs:589, 597-646, 616, 655, 1391-1400` (the last one is the thumb
anchor, which 0.6.4 moved into the private `ThumbGeometry`,
`:1176-1231`, painting at `:1505-1510`; `base_layer.rs:29-31`'s explanation of
`inset` must be re-read against it, including the new
`inset.clamp(0, logical_length / 2)`), `theme.rs:31-36`; gpui-pre
`app.rs:1074, 1662-1664, 2087-2099`, `color.rs:224-262`; gpui-component
`tab/tab.rs:801-808`, `button/button.rs:884-949`, `icon.rs:160, 189`.

Out of scope, stated so it is not mistaken for an oversight: the upstream
citations in `docs/todo.md`, `docs/todo_gpui-full-theme.md` and `ROADMAP.md`
keep their 0.6.0 line numbers (four are already stale). They are a roadmap and
a gap analysis, not the crate's documentation; re-verifying them is a separate
pass (rationale §4). The one line Task 4 edits there is corrected in passing.

---

## 8 -- Documentation

| File | Change |
|---|---|
| `connectors/native-theme-gpui/README.md` | dependency snippet `0.3.3` → `0.3.5` (`:272`); "0.6.0's variant set" → 0.6.4 (`:35`); reduced-motion row (§5.4); counts (§3); a sentence at the end of "Per-widget geometry" pointing at `tests/seams.rs`; the ghost-hover bullet in "What gets mapped" (§8c) |
| `src/lib.rs` crate docs and `apply` docs | step 6 of `apply` describes §5.1; limits of §5.4 |
| `src/geometry.rs` module docs | which six builders are mechanically verified (§6.1) |
| `src/geometry.rs` `dialog` doc, `README.md` per-widget geometry row | the `max_height` sentence of §4: 0.6.4 clamps the dialog to the viewport after the refinement, so `dialog.max_height` no longer reaches `Dialog`; `min_height`, padding and `dialog_max_width` (through `Dialog::max_w`) still do. The builder is unchanged (E18) |
| `src/geometry.rs:92` (`menu_item` doc), `README.md:153`, `docs/todo_gpui-full-theme.md:48` (whose `popup_menu.rs:749` citation moves to `:775` while the line is open) | "application-built `MenuItem`" → "a menu row the application draws with its own elements; upstream's `MenuItemElement` is crate-private and `PopupMenu` builds its own rows, so no gpui-component widget takes this style". The function is unchanged. |
| `src/icons.rs:138, 257, 401, 1694`, showcase `:488` | "0.6.0" → "0.6.4" (same 101 variants) |
| `docs/todo.md` | icon-table item: precondition met (floor is 0.6.4); new Research item: installed-font check for preset families, citing gpui-component 0.6.4 `src/theme/system_font.rs:3-9`; MSRV note refreshed to 0.3.5 |
| `CHANGELOG.md` | see below |

`CHANGELOG.md`, under the existing `## [Unreleased]` heading, next to the
canary and publish-job entries already there. The heading stays undated:
`pre-release-check.sh:593` treats a dated `## [0.5.9] - …` heading as "release
tree" and turns the asset-stamp warning into a failure, so dating it is part
of the maintainer's release commit (§9), not of the implementation:

- **Fixed** — native-theme-gpui 0.5.8 no longer compiled on a fresh
  dependency resolution: gpui-component 0.6.2 (2026-09-18) removed
  `ThemeColor::tiles`, which the connector wrote, in a patch release that the
  connector's caret requirement admits. The nightly dependency canary reported
  it the same evening.
- **Changed** — requires gpui-component / gpui-base 0.6.4 and gpui-pre 0.3.5;
  older GPUI Kit versions are not supported. Not filed as breaking: Cargo
  backtracks, so an application pinning below the floors keeps resolving to
  0.5.8, which still builds there (measured, rationale §2.1). v0.5.8's move
  from gpui-component 0.5 to 0.6 was filed as breaking because those versions
  are semver-incompatible and the types stop unifying; this is not that.
- **Changed** — `ThemeColor` mapping 139 → 138 fields, config export 127 → 126
  (`tiles` no longer exists upstream).
- **Breaking Changes → native-theme-gpui** — reduced motion: `reduce_motion: false` no longer clears a flag
  the connector did not set, so gpui-base's new OS reader (0.6.2) and the
  application keep control; `true` then `false` undoes only the connector's
  own switch, and every `false` asks gpui-base to re-read the OS setting.
  Callers that relied on `apply(.., &default(), ..)` to switch motion back on
  call `cx.set_reduce_motion(false)` themselves. (Breaking because it changes
  what an existing call does; pre-1.0 policy allows no migration document, so
  the changelog is the only notice a consumer gets. The file's own taxonomy
  puts it under `### Breaking Changes` → `#### native-theme-gpui`, as
  `CHANGELOG.md` did for v0.5.7 and v0.5.8.)
- **Added** — rendered seam tests (`tests/seams.rs`): real gpui-component
  widgets are laid out headlessly with and without the connector's geometry.
- **Added** — showcase sections for `InputGroup`, `Empty`, `Carousel`, a Rust
  code editor and a Markdown view.
- **Fixed** — documentation of `geometry::menu_item`: gpui-component has no
  public menu-item element to apply it to; it styles application-drawn rows.
- **Fixed** — documentation of `geometry::dialog`: gpui-component 0.6.4 clamps
  the dialog to the viewport after applying the caller's style, so the theme's
  `dialog.max_height` no longer reaches it.

No migration section (pre-1.0 policy).

## 8a -- Showcase additions

New sections, each following the file's existing `section("…")` pattern and
placed in the tab whose widgets they belong with; every colour, size and
spacing comes from the active theme (no literals):

| Section | Content | What it lets a human verify |
|---|---|---|
| `InputGroup` | one inline-addon group, one with a trailing `InputGroupButton`, one `InputGroupTextarea` | `input`, `ring`, `radius`, `muted*`, the internal ghost button (§8c) |
| `Empty` | `Empty` with media icon, title, description and one action button | `muted`, `muted_foreground`, `border`, `radius_tokens` |
| `Carousel` | three slides with pagination | `radius`, reduced motion (§5) |
| `Code editor` | `Editor::new(&state)` over `EditorState::new(window, cx).language("rust")` (gpui-component 0.6.4 `src/input/editor.rs:39`; gpui-base 0.6.4 `src/input/base/state.rs:9219-9246`: line numbers and search are on by default) holding a dozen lines of Rust | mono font family and size, caret, selection, highlighter style per mode (D41), search panel |
| `Markdown` | a `TextView::markdown(id, source)` (gpui-component 0.6.4 `src/text/compat.rs:56`) with heading, paragraph with inline code and a link, fenced code block, quote, small table | `link`, `muted`, `border`, mono font |

Syntax colours need a grammar: the dev-dependency becomes
`gpui-kit = { version = "0.6.4", features = ["tree-sitter-rust"] }`
(gpui-kit 0.6.4 `Cargo.toml:174-177`; dev builds only, the published library
is unaffected). Without it the editor shows unhighlighted text and D41's
per-mode highlighter style stays unseen, which is the point of the section.
Constructors not cited above (`Carousel`, `Empty*`, `InputGroup*`) are read
from `src/carousel/`, `src/empty.rs`, `src/input/group.rs` at implementation
time; nothing else is added to the dependency list.

## 8b -- Archiving

`git mv docs/todo_v0.5.8_gpui-component-0.6-{rationale,spec,plan}.md docs/archive/`
and update every path that cites them: `grep -rn 'todo_v0.5.8_gpui-component'
--include='*.md' --include='*.rs' --include='*.toml' --include='*.sh' .`
Today that grep matches, outside the moved files themselves: the v0.5.9
documents, `ROADMAP.md:78`, and two absolute GitHub URLs in the connector —
`README.md:175` and `src/lib.rs:85`, the latter in the crate-level rustdoc that
ships to docs.rs, where a stale link 404s. In both URLs `docs/` becomes
`docs/archive/`. `docs/todo.md` and `docs/todo_gpui-full-theme.md` cite the
v0.5.8 specification by section number only and need no edit. The literal
`git mv` command and the grep pattern recorded in these v0.5.9 documents are
not citations and stay as they are.
The three v0.5.9 documents are archived the same way after the release; that
is the plan's last task.

## 8c -- Ghost-button hover (known visual change, no code in v0.5.9)

Upstream 0.6.4 fills a hovered ghost button with `tokens.accent`
(`src/button/button.rs:1125-1132`); the connector maps `accent` to the
platform accent colour, while platform-facts `:1168` records a subtle fill
for button hover. 0.6.4 also paints the hovered label with `accent_foreground`
(`:1141`), so the hovered ghost button is a selection-coloured pill, not just a
fill. `README.md` ("What gets mapped") and `docs/todo.md` ("Upstream PR
candidates") gain: "a `ghost_hover` / `ghost_hover_foreground` token pair
separate from `accent` / `accent_foreground` (0.6.4 hovers ghost buttons with
the accent pair, which native themes map to the platform's selection colours)".
The visual check (2026-09-20) found the same defect in the new `InputGroup`
addon button, from a different wrong token (`muted`, rationale §1.4m), and
settled the question: the connector ships the application-side remedy.

`src/variants.rs`, public:

```rust
#[must_use]
pub fn ghost_button(cx: &App) -> ButtonCustomVariant {
    let theme = cx.theme();
    ButtonCustomVariant::new(cx)
        .foreground(theme.secondary_foreground)
        .hover(theme.secondary_hover)
        .active(theme.secondary_active)
}
```

Transparent when idle, as upstream's `.ghost()` is (`ButtonCustomVariant::new`
starts from `theme.transparent`; a non-outline custom border is the same
colour, `button/button.rs:1033-1039`). Applied with `ButtonVariants::custom`,
it replaces `.ghost()` on a `Button` and makes `InputGroupButton` skip its
in-group repaint (`input/group.rs:544-545`). Two tests in the module: the
hover equals the resolved `button.hover_background` and differs from both
`accent` and `muted` (kde-breeze, where all three differ); and the variant
follows a light → dark `apply`. The showcase uses it for the InputGroup's Copy
button and shows a "Ghost (native)" button beside upstream's ghost. README
gains a "Flat buttons" subsection under "Per-widget geometry".

---

## 9 -- Release gates

Unchanged from v0.5.8 and the order `pre-release-check.sh:634-647` prints,
listed so the plan can cite them:

1. `./pre-release-check.sh` without failures (run with `CARGO_BUILD_JOBS=4`).
   The asset-stamp check *warns* at this point: five stamped paths change in
   this release (`Cargo.lock`, the workspace and two crate `Cargo.toml`s, the
   connector's `src/` and `examples/`; `scripts/asset-stamp.sh:28-50`). It
   becomes a hard failure once the CHANGELOG heading is dated, and the stamp
   must therefore be rewritten *after* the version bump, i.e. gate 2 comes
   after plan Task 7.
2. Push the branch first: `scripts/pre-release.sh` refuses to run unless HEAD
   equals `@{u}` and triggers `gh workflow run screenshots.yml --ref <branch>`,
   which needs the branch on origin. Then `./scripts/pre-release.sh` on the
   maintainer's KDE desktop regenerates the screenshots; assets and
   `docs/assets/PROVENANCE.toml` committed. This is also the visual check of
   the showcase on 0.6.4, which no test replaces — including the five new
   sections, which no screenshot captures (rationale §2.11).
3. The release commit (`chore(release): v0.5.9`) dates the CHANGELOG heading
   and adds the `v0.5.8...v0.5.9` compare link; `./pre-release-check.sh` fully
   green on it; CI green on that exact commit; canary dispatched once by hand
   on it and green.
4. Maintainer's explicit go → tag `v0.5.9` pushed by name by the maintainer →
   crates.io workflow → docs.rs check → GitHub release page.

---

## 10 -- Acceptance

- [ ] `cargo clippy --workspace --all-targets --all-features -- -D warnings` clean on the 0.6.4 lockfile.
- [ ] `cargo test -p native-theme-gpui --all-features`: all pass, including the three re-numbered tripwires, the three §5.3 tests, the §3a shape tripwire, the two §8c variant tests and the six §6 tests: 200 in the library and 6 in `tests/seams.rs`.
- [ ] `cargo +1.95.0 check -p native-theme-gpui --lib --locked` passes.
- [ ] `cargo audit` exits 0 (the six allowed warnings for unmaintained crates remain; `RUSTSEC-2026-0285` is gone, E19).
- [ ] `grep -rn 'max_h' connectors/native-theme-gpui/src/geometry.rs` still finds the dialog line, and its doc comment says the value does not reach upstream's `Dialog` (E18).
- [ ] `grep -rn 'tiles' connectors/native-theme-gpui/` finds exactly one line: the `Cargo.toml` comment recording why the floor is hard ("0.6.2 removed ThemeColor::tiles"). No `.rs`, `.md` or example file mentions it.
- [ ] `grep -rn '0\.6\.0\|0\.3\.3' connectors/native-theme-gpui/` finds no *citation*: every remaining hit states history or native-theme's own version — `src/lib.rs:137` ("Planned for unification in v0.6.0"), the `Theme`-tripwire and `spinner_size` comments that say what 0.6.0 did, and the two showcase comments ("0.6.0 added `SliderEvent::Release`", "Button (0.6.0):").
- [ ] `cargo tree -p native-theme-gpui -i gpui-pre` shows one gpui-pre, 0.3.5.
- [ ] `cargo doc -p native-theme-gpui --no-deps --all-features` builds without warnings (the form `pre-release-check.sh:510-516` and CI use, plus the features docs.rs enables).
- [ ] The showcase builds and shows the five §8a sections; `docs/todo_v0.5.8_*` no longer exists outside `docs/archive/`.
- [ ] §9 gate 1 by the implementer; gates 2–4 are the maintainer's.
