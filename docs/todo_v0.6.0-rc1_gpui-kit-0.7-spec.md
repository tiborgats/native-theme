# v0.6.0-rc1 — gpui connector on GPUI Kit 0.7.0: Specification

Status: Design (2026-09-30), revised on 2026-09-30 and 2026-10-01 after an
independent fact-check, trial implementations and cold reviews (rationale
§3); nothing implemented
Target toolkit: **gpui-component 0.7.0**, **gpui-base 0.7.0**, GPUI as the
**`gpui-pre` 0.3.7** package; **gpui-kit 0.7.0** for the showcase and tests.
GPUI Kit 0.6.x is not supported afterwards.
Companion rationale:
[`todo_v0.6.0-rc1_gpui-kit-0.7-rationale.md`](todo_v0.6.0-rc1_gpui-kit-0.7-rationale.md)
(decisions K1–K17)
Companion plan:
[`todo_v0.6.0-rc1_gpui-kit-0.7-plan.md`](todo_v0.6.0-rc1_gpui-kit-0.7-plan.md)

This document amends the v0.5.8 and v0.5.9 specifications
(`archive/todo_v0.5.8_gpui-component-0.6-spec.md`,
`archive/todo_v0.5.9_gpui-kit-0.6.4-spec.md`); a section not mentioned here
is unchanged.

Upstream paths are relative to each crate's `src/` in
`~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/<crate>-<version>/`.
Connector paths are relative to `connectors/native-theme-gpui/` unless they
start with `docs/`, `scripts/` or `native-theme/`. Connector line numbers
are those of `v0.6.0-rc1` at `cc2f3b81`; earlier tasks shift them, so locate
by content.

---

## 0 -- Scope

### 0.1 What this delivers

1. The connector and its showcase build, lint and test clean on the 0.7.0 /
   0.3.7 stack, and require it (§2, §3).
2. The three new `IconName` variants map to genuine glyphs in all three sets
   (§3.2).
3. `ThemeColor::chart_grid` follows the theme, on the direct and the config
   path; the config tripwire can see a forgotten key (§4).
4. The showcase's actions and shortcuts work over dialogs and sheets again,
   and its capture pointer shield covers every top-level overlay — which it
   never did for dialogs, menus and popovers (§5; nested popups are a known
   limit, rationale K7).
5. Splitters: the connector documents which resizables its splitter colours
   reach, and how an application gets native ones (§6).
6. The connector's `Switch` draws the keyboard focus ring (§7).
7. The showcase shows every widget 0.7.0 added except `Questionnaire`, which
   cannot be built without a panic path and is excepted with that reason
   (§8.5), and its Widget Info is true for 0.7.0 (§8, §9).
8. Every upstream citation in the connector, README and showcase is true for
   0.7.0 / 0.3.7 (§9).
9. Documentation, changelog, compatibility stamp; a layout comparison proving
   the shared chrome and Basic page did not move (§10–§12).
10. These three documents are archived (§13).

### 0.2 Constraints

- No panics, no `unsafe`, no invented values, no hardcoded theme values, as
  in §0.2 (0.5.8). The repository's PreToolUse hooks apply.
- Public API: nothing added, removed or re-typed. Behaviour changes:
  `chart_grid`, `icon_name(ActionRefresh)`, the `Switch` focus ring,
  `geometry::toolbar` leaving the height to the content (§8.1).
- Work stays on branch `v0.6.0-rc1`; workspace version stays `0.6.0`.
- No release action (tag, push, upload, GitHub release, asset regeneration
  run) without the maintainer's explicit go.

### 0.3 Out of scope

Popover arrows (no model property yet), the toolbar's reload glyph across
the three showcases, a citation gate for `src/*.rs`, the latent base-palette
token drift, the `Settings` / dock splitter (upstream-owned). Each gets a
`docs/todo.md` entry (§10.3), as do `RadarChart` and `SankeyChart`, which
the coverage script cannot see (rationale K11). The time-enabled
`DatePicker` and a second chart with axes are rejected, not deferred
(rationale K11 / K12). Nothing in
`native-theme` core except three bundled SVGs (covered by the existing set
rules of `native-theme/icons/SOURCES.toml`); nothing in egui or iced.
The upstream citations in `docs/todo.md`, `ROADMAP.md`,
`docs/todo_gpui-full-theme.md` and `docs/todo_gpui-widgets-spec.md` keep their
0.6.6 numbers: a roadmap, a gap analysis and an implemented rc1 spec awaiting
archive, not the crate's documentation (as v0.5.9 §7 decided for the same
files); the egui and iced READMEs' mention of gpui-component 0.6.2 is
history.

---

## 1 -- Facts this specification rests on

Measured 2026-09-30 on a throwaway worktree of `v0.6.0-rc1` `cc2f3b81` ("the
probe") or read from the published sources.

| Fact | Evidence |
|---|---|
| Newest: gpui-kit / gpui-component / gpui-base / gpui-kit-assets 0.7.0, gpui-pre 0.3.7, all 2026-09-28 | crates.io API |
| gpui-component 0.7.0 requires `gpui-pre = "=0.3.7"`, `gpui-base = "0.7.0"` | published `Cargo.toml` |
| `cargo update -p gpui-component -p gpui-base -p gpui-kit` updates 27 packages, adds and removes none; no non-gpui package changes | probe; lockfile diff |
| Lib compile errors: `src/widgets/spinner.rs:240` (E0639, E0061), `src/icons.rs:158, 290, 439` (E0004: `Ban`, `CircleAlert`, `RefreshCw`) | probe pass 1 |
| Showcase compile errors: `app.rs:2197-2199` (`Root::render_*_layer`), `tests.rs:624, 627` (`Root::notification`) | probe pass 1 |
| With both patched: lib 271 pass / 10 fail, seams 13 / 0, showcase 172 / 2; the twelve failures are listed in rationale §1.3 | probe pass 2 |
| Rust 1.95.0 builds the library; 1.94.0 fails in gpui-pre 0.3.7 `profiler.rs:473, 494` (`cold_path`) | `cargo +1.95.0 check --lib --locked`; `cargo +1.94.0 … --ignore-rust-version` |
| `cargo audit`: 0 vulnerabilities, six allowed warnings (same as the branch) | probe |
| `check_widget_coverage.py`: 13 missing (`InputToken`, `Questionnaire`, `QuestionnaireActions`, `…Choice`, `…ChoiceDescription`, `…Choices`, `…Error`, `…Input`, `…Item`, `…Progress`, `TimeField`, `Toolbar`, `ToolbarGroup`), 1 stale exception (`Text`) | probe |
| `ThemeColor` has 139 fields (new `chart_grid`, `theme/theme_color.rs:147`); `ThemeConfigColors` serialises 140 keys: 128 public (new `chart_grid`, key `chart.grid`, `theme/schema.rs:386-388`) and 12 private `base.*` (`:643-677`); `Theme` gained none | tripwires; serde on the trial implementation |
| `ArcData::new(data, index, value, start_angle, end_angle)`, `pad_angle` 0; `Arc::paint(&self, &ArcData, fill, &Bounds, &mut Window)` | gpui-base `plot/shape/arc.rs:27-36, 243-254` |
| `default-icons.txt` 101 → 104: `ban`, `circle-alert`, `refresh-cw`; no removal; gpui-kit-assets' three SVGs are byte-identical to Lucide 1.41.0's | `diff`; `cmp` |
| Lucide 1.41.0 has `ban.svg`, `circle-alert.svg`; Material Symbols at `0cbb0881…` has `block`; the bundle has `refresh-cw.svg`, `error.svg`, `refresh.svg` and lacks the other three | HTTP 200 at the pinned refs; `ls native-theme/icons/*` |
| Overlays are mounted beside the view, not inside it; the view's action handlers are not on an overlay's dispatch path | gpui-base `root.rs:285-310`; gpui-component `root.rs:461-493`; gpui-pre `window.rs:2442-2450, 6283`; probe debug run |
| `RootPlugin::decorate(&self, surface, root: &Root, …)` wraps the finished surface; `Root::view()` returns the application view; plugins apply in registration order; `Root::register_plugin::<V>(cx, fn(&mut Window, &mut Context<V>) -> V)` | gpui-base `root.rs:26-80, 101-104, 165` |
| `gpui_kit::open_window(options, cx, build) -> Result<(AnyWindowHandle, Entity<V>)>` wraps the content in `gpui_base::Root` | gpui-kit `lib.rs:139-159` |
| gpui-base's `h_resizable` installs no renderer, so Base's built-in line is drawn in `ResizableTheme` colours; gpui-component's is gpui-base's plus its own renderer; 0.6.6's gpui-component re-exported gpui-base's; gpui-kit re-exports gpui-base as `gpui_kit::base` | gpui-base `resizable/resize_handle.rs:309-336`, `lib.rs:152-155`; gpui-component `resizable.rs:27-30`, 0.6.6 `lib.rs:107-110`; gpui-kit `lib.rs:103` |
| Dialogs, menus and popovers paint as `deferred` elements (dialog priority `10 + layer`), above any ordinary element — in 0.6.6 too | gpui-base 0.7.0 `dialog.rs:548, 643`, `popup.rs:190-198`; 0.6.6 `dialog.rs:541, 631` |
| gpui-base `Switch::track_focus(&FocusHandle)` exists (new in 0.7.0) | gpui-base `switch.rs:319-323` |
| `QuestionnaireState::new(items, cx) -> Result<Self, QuestionnaireSchemaError>` is its only constructor; no `Default`; gpui's `cx.new` takes an infallible builder | gpui-base `questionnaire/state.rs:39-45`; gpui-pre `app.rs:2971` |
| The existing Bar, Line and Area charts draw grids by default and the showcase does not turn them off | `bar_chart.rs:108`, `line_chart.rs:69`, `area_chart.rs:71`; showcase `demo.rs:7133-7171` |
| `chart_grid` is read by `chart/mod.rs:419`, `bar_chart.rs:857`, `radar_chart.rs:485`; `ThemeColor: Default` is transparent black | gpui-component 0.7.0; `theme_color.rs:58` |

---

## 2 -- Manifest and lockfile

`connectors/native-theme-gpui/Cargo.toml`:

```toml
# NOT inherited: the gpui-pre closure declares a higher floor than the
# workspace. Set to the lowest toolchain that compiles the library; measured
# 2026-09-30 on the 0.7.0 closure: 1.95.0 builds, 1.94.0 fails on
# std::hint::cold_path in gpui-pre 0.3.7 (src/profiler.rs).
rust-version = "1.95.0"
```

```toml
[dependencies]
# GPUI under the package name gpui-component uses, so both edges unify.
# gpui-component 0.7.0 depends on `gpui-pre = "=0.3.7"` (an exact pin), so
# 0.3.7 is the only version that resolves with it. gpui-component does not
# re-export gpui, so the connector names it. Caret: an exact pin in a library
# would conflict with any consumer whose other dependencies want a later
# snapshot.
gpui = { package = "gpui-pre", version = "0.3.7" }
# A hard floor: 0.7.0 moved Root and the plot primitives into gpui-base and
# added ThemeColor::chart_grid, which this crate writes.
gpui-component = "0.7.0"
# For gpui_base::Theme, ScrollbarTheme, ResizableTheme and
# apply_system_reduce_motion (not re-exported).
gpui-base = "0.7.0"
```

The `image` comment's "gpui-pre 0.3.6" becomes "0.3.7" (the version it pins,
0.25.10, is unchanged).

```toml
[dev-dependencies]
# Headless App for the apply/observer tests (#[gpui::test], TestAppContext).
gpui = { package = "gpui-pre", version = "0.3.7", features = ["test-support"] }
# The showcase uses the facade an application would: application(), init(),
# open_window(), and the Lucide assets under gpui_kit::assets.
# tree-sitter-rust: the grammar for the showcase's code editor section.
gpui-kit = { version = "0.7.0", features = ["tree-sitter-rust"] }
```

`Cargo.lock`: `cargo update -p gpui-component -p gpui-base -p gpui-kit`,
after the manifest edit; expected "Locking 27 packages", 27 package
`Updating` lines (plus cargo's "Updating crates.io index"), no `Adding` /
`Removing`.

---

## 3 -- Compile fixes

### 3.1 Spinner (K2)

`src/widgets/spinner.rs`, `ring`:

```rust
Arc::new()
    .inner_radius(inner.max(0.))
    .outer_radius(outer)
    .paint(
        &ArcData::new(&(), 0, end - start, start * TAU, end * TAU),
        look.color,
        &bounds,
        window,
    );
```

### 3.2 Icons (K3)

Bundle, from the repository root:

```sh
./scripts/update_icons.sh add lucide ban
./scripts/update_icons.sh add lucide circle-alert
./scripts/update_icons.sh add material block
```

The script writes the fetched bytes unchanged and needs no `SOURCES.toml`
edit (the set rules cover them). It needs the network, and `add` re-downloads
*every* bundled file from its pinned ref, so the expected result is exactly
three new files and no other change; `native-theme/tests/icon_sources.rs`
must still pass. If any other file changes, or a new one is missing or
contains no `<svg`, stop: never substitute a glyph.

Tables, `src/icons.rs` (one arm each, alphabetical position):

| `IconName` | `lucide_name_for_gpui_icon` | `material_name_for_gpui_icon` | `freedesktop_name_for_gpui_icon` |
|---|---|---|---|
| `Ban` | `"ban"` | `"block"` // exact | `"action-unavailable"` // exact |
| `CircleAlert` | `"circle-alert"` | `"error"` // exact | `"emblem-important"` // close: Adwaita a circle with "!", Breeze a bare "!" |
| `RefreshCw` | `"refresh-cw"` | `"refresh"` // close: one circular arrow, Lucide's has two | `"view-refresh"` // exact |

`icon_name`: `IconRole::ActionRefresh => IconName::RefreshCw` under
"Common Actions". Its doc: "Maps 31 of the 42 `IconRole` variants … The 11
unmapped roles (Shield, ActionSave, ActionPaste, ActionCut, ActionEdit,
ActionPrint, NavHome, TrashFull, DialogQuestion, Help, Lock) have no
corresponding icon in gpui-component 0.7." The tests
`icon_name_maps_exactly_30_roles` → 31 (renamed accordingly) and
`icon_name_data_driven` gain the new row.

`ALL_ICON_NAMES` gains `IconName::Ban`, `IconName::CircleAlert`,
`IconName::RefreshCw` in order; `all_icon_names_count_matches_gpui_component`
expects 104. The doc comments of all three tables (`src/icons.rs:153, 272,
416`: "… 101 … 0.6.6 …") become 104 / 0.7.0.

The showcase's gallery, `GPUI_ICONS` (`examples/showcase-gpui/support.rs:539-540`,
"The 101 gpui-component 0.6.6 IconName variants"), gains the same three
entries in order and says 104 / 0.7.0; its `role_for_gpui_icon` table
(`:503-506`, derived from `icon_name`) gains `"RefreshCw" =>
Some(IconRole::ActionRefresh)`.
`every_none_is_an_allowed_gap` and `every_some_resolves_in_its_bundle` must
pass unchanged (no new allowed gap).

### 3.3 Showcase compile

`examples/showcase-gpui/app.rs`: the three `.children(Root::render_*_layer(…))`
lines are deleted (§5 replaces what they did). `tests.rs`: the notification
count reads `cx.update(|window, cx| window.notifications(cx).len())`
(`WindowExt::notifications`, gpui-component `window_ext.rs:79, 197`).

---

## 4 -- `chart_grid` and the config export (K4, K5)

### 4.1 Direct path

`src/colors.rs`, beside `tc.table_row_border = c.list_grid;` (the one place
`list_grid` is consumed):

```rust
    // The grid of a chart (`chart/mod.rs:419`, `chart/bar_chart.rs:857`,
    // `chart/radar_chart.rs:485`): the model's grid-line colour, which
    // platform-facts §2.15 states as macOS `gridColor` and Material's
    // `outline-variant`, and which inherits `defaults.border.color` elsewhere.
    tc.chart_grid = c.list_grid;
```

### 4.2 Config path

`src/config.rs`, after `colors.chart_bearish = …`:
`colors.chart_grid = h(tc.chart_grid);`

### 4.3 Contract

`src/contract.rs`, after the `chart_bearish` row:

```rust
    Row {
        slot: "chart_grid",
        native: |r| r.list.grid_color,
        get: |tc| tc.chart_grid,
        exceptions: &[],
    },
```

### 4.4 Tripwires and counts

- `colors::tests::theme_color_field_count_tripwire`: comment "139 Hsla
  fields in gpui-component 0.7.0", `field_count, 139`.
- `no_theme_color_field_is_left_at_default`: `139`.
- `contract::every_theme_color_field_has_a_declared_source`: its literal
  `138` (`src/contract.rs:1276`) → `139`.
- `config::tests::theme_config_colors_cover_the_0_6_fields` →
  renamed `theme_config_colors_export_every_key`: its hand list of 15 keys
  asserted `is_some` (`src/config.rs:328-346`) and the literal `126` are
  replaced by one check of *which* keys stay unset, which asserts every other
  key exported (rationale K5):

  ```rust
  let object = value
      .as_object()
      .expect("ThemeConfigColors serialises as an object");
  let unset: Vec<&str> = object
      .iter()
      .filter(|(_, v)| v.is_null())
      .map(|(k, _)| k.as_str())
      .collect();
  // D43: the 12 private base-palette colours are not exported; and
  // group_box_title_foreground has no ThemeColor source.
  let expected_unset =
      |key: &str| key.starts_with("base.") || key == "group_box.title.foreground";
  let base = unset.iter().filter(|k| k.starts_with("base.")).count();
  assert_eq!(base, 12, "the base palette has 12 keys: {unset:?}");
  let forgotten: Vec<&&str> = unset.iter().filter(|k| !expected_unset(k)).collect();
  assert!(forgotten.is_empty(), "ThemeConfigColors keys not exported: {forgotten:?}");
  ```

  Measured on 0.7.0 (trial): 140 keys, 13 unset, 127 exported.
- Prose counts: `src/config.rs:5, 23, 93-99` (138 → 139 `ThemeColor`, 126 →
  127 exported, `ThemeConfigColors` 139 → 140), `src/lib.rs` doc table and
  `every_theme_field_is_named`'s comment ("the 138-field colour map" → 139),
  `README.md` wherever it counts the colour fields.

### 4.5 Tests

- `colors.rs`: `chart_grid_is_the_list_grid_colour` — over every preset of
  `Theme::list_presets()` in both variants,
  `to_theme_color(..).chart_grid == rgba_to_hsla(resolved.list.grid_color)`.
- `config.rs`: `the_config_carries_the_native_chart_grid` — `to_theme_config`
  carries `chart_grid == Some(hex of the direct path's chart_grid)`, so
  `Theme::change` cannot fall back to upstream's `border × 0.6`.
- Proof that `theme_config_colors_export_every_key` catches a forgotten key:
  with the `chart_grid` export removed, it fails naming `chart.grid`.

---

## 5 -- Overlay hosting in the showcase (K7, K8)

### 5.1 The plugin

New file `examples/showcase-gpui/host.rs` (module `host`), added to the
files the showcase's gates read (`SHOWCASE_FILES`, `src/showcase.rs`; the
gate `the_gates_read_every_showcase_file` fails otherwise):

- `pub(crate) struct ShowcaseHost;` implementing `Render` (renders an empty
  `div()`: it has no overlay of its own) and `gpui_base::RootPlugin` with only
  `decorate`.
- `decorate(&self, surface, root, window, cx) -> impl IntoElement`, returning
  `AnyElement` from both arms:
  1. `let Ok(showcase) = root.view().clone().downcast::<Showcase>() else {
     return surface; }` — any other root is passed through untouched.
  2. `div().relative().size_full()` + one `.on_action(…)` per action the
     view's `div` handled before (`ShowPage`, `SetColorMode`, `ReloadTheme`,
     `ToggleSidePanel`, `SetPreset`, `OpenCommandPalette`, `OpenPreferences`,
     `OpenAbout` — the list at `app.rs:2168-2175`), each forwarding through
     `showcase.update(cx, |this, cx| this.on_…(action, window, cx))`; the eight
     `Showcase::on_*` methods (`app.rs:1889-1976`) become `pub(crate)`;
     then `.child(surface)`; then `.into_any_element()`.
- `pub(crate) fn register(cx: &mut App)` calls
  `gpui_base::Root::register_plugin::<ShowcaseHost>(cx, |_, _| ShowcaseHost)`.

`app::init` calls `host::register(cx)` **after** `gpui_kit::init(cx)` has run
(main.rs and tests.rs both call `gpui_kit::init` first, then `app::init`, and
both before any window opens — a root instantiates the plugins registered
when it is created, gpui-base `root.rs:43-44, 147-162`), so the plugin wraps
outside gpui-component's `WindowState`.

The pointer shield stays in `Showcase::render` (§5.2).

### 5.2 The view

`Showcase::render`: the `.on_action(…)` calls are removed from the root
`div`; `.track_focus(&self.focus_handle)` stays. The pointer-shield child
stays the last child and becomes
`deferred(div().absolute().top_0().left_0().size_full().occlude()
.debug_selector(|| POINTER_SHIELD.into())).with_priority(usize::MAX)`: a
dialog, menu or popover is itself deferred (§1), no ordinary element is above
a deferred one, and a deferred element is painted by priority wherever it
sits in the tree (rationale K7). Its old comment ("Last, so it is the
topmost hitbox") becomes: the priority is what puts it above every
top-level overlay. A popup nested in a deferred overlay (a submenu, a
`Select` opened in a dialog) is prepainted in a later round and stays above
it — a known limit (rationale K7).
The comment block at `app.rs:2154-2161` is rewritten: overlays are drawn by
gpui-component's `WindowState` plugin beside the view
(gpui-base `root.rs:285-310`), and the showcase's actions are hosted
around both by `host::ShowcaseHost`; the shield, still in the view, is
deferred above every top-level overlay.

### 5.3 Opening the window

`main.rs` and `tests.rs` open the window with
`gpui_kit::open_window(options, cx, |window, cx| cx.new(|cx| …Showcase…))`,
dropping the `showcase_entity` / `view` cells. Nobody calls
`window_border()`. In `main.rs` the handle is an `AnyWindowHandle`: its four
`*window_handle` uses (among them `dump_layout`, `:1482-1489`) take it as it
is, the `dump_layout` guard and the macOS block lose their `Option`, and the
failure path (`eprintln!` and exit) is kept. `tests.rs`'s `open` / `open_with`
return `(Entity<Showcase>, AnyWindowHandle, VisualTestContext)`; call sites
binding `_root` need no change; `AnyWindowHandle` is imported, the
`RefCell` and `Deref` imports go, `Root` stays (`tests.rs:148` calls
`window.root::<Root>()`).

### 5.4 Tests (`tests.rs`)

- The existing `the_palette_switches_page`, `the_palette_installs_a_preset`
  pass unchanged.
- New `a_shortcut_works_while_the_palette_has_the_focus`: open the palette
  (`secondary-k`; the dialog takes the focus for its query, `chrome.rs:454`),
  press `secondary-b`; the side panel's visibility flips. On the branch
  before §5 this fails.
- New `the_pointer_shield_covers_an_open_dialog`: with the shield on (the
  mechanism `a_capture_hovers_nothing` uses, `tests.rs:4435`) and the About
  dialog open (`run_menu_item(…, "Help", "About")`), hovering the dialog's
  name (`settle_on(…, OVERLAY_ABOUT_NAME)`) shows no Widget Info. It fails
  with a non-deferred shield in any position (trial), so it pins the
  priority, not only the host.

### 5.5 Coverage rule and exemptions

`src/showcase.rs` `NOT_WIDGET_CONSTRUCTORS`: the three `Root::render_*_layer`
entries are removed; the `Root::new` entry is removed (the showcase no
longer calls it). Measured in the trial: `every_widget_reports_itself` then
passes without an entry for `gpui_kit::open_window` or
`Root::register_plugin`.

---

## 6 -- Splitters (K9)

No code. `src/base_layer.rs`'s `resizable_theme` doc, `src/lib.rs:72`'s
`splitter` row and `README.md:22-25, 141, 503-504` say which handles the
splitter colours reach, what they give, and how an application gets them:

- They reach every handle gpui-base draws with its built-in line: groups
  built with `gpui_base::h_resizable` / `v_resizable` (re-exported as
  `gpui_kit::base::h_resizable` / `v_resizable`), which install no renderer
  (gpui-base `resizable/resize_handle.rs:309-336`). That line is the
  splitter's *colours* at gpui-base's 1 px width (`HANDLE_SIZE`, `:13`), not
  the stated `splitter.divider_width`, and `splitter.hover_color` shows only
  while it is dragged (`src/base_layer.rs:121`) — as on 0.6.6.
- They do not reach groups built with `gpui_component::h_resizable` /
  `v_resizable`, which install gpui-component's renderer — a `border`
  hairline and a `muted_foreground` pill (gpui-component
  `resizable.rs:27-30, 93, 104-111`) — nor `Settings`' divider or the dock's
  edges, which install it internally (`setting/settings.rs:416`,
  `dock/dock.rs:143`).

Showcase: the `Settings` Widget Info (Layout page and Preferences sheet)
gains a not-themeable line for its divider whose citation is in the form
the prose gate reads (`file.rs, Symbol`):
"(setting/settings.rs, Settings; resizable.rs, resize_handle_appearance)".

---

## 7 -- `Switch` focus ring (K10)

`src/widgets/switch.rs`, `RenderOnce for Switch`, native branch:

- `let focus_handle = window.use_keyed_state(self.id.clone(), cx, |_, cx|
  cx.focus_handle()).read(cx).clone();` and `let focused =
  focus_handle.is_focused(window);` — the `Checkbox` pattern
  (`src/widgets/checkbox.rs:563-567`).
- `BaseSwitch::new(self.id)…​.track_focus(&focus_handle)` — gpui-base
  `switch.rs:319-323`.
- On the track: `.when(focused && !self.disabled, |track|
  track.focus_ring_style(window, cx))`, as gpui-component's switch does
  (`switch.rs:269-272`); `focus_ring_style` is a `ThemeStyled` method
  (gpui-component `styled.rs:127, 165-189`), imported as `ThemeStyled as _`.

The doc comment of `widgets::Switch` states the ring and its source
(`ThemeColor::ring`, from `defaults.focus_ring_color`, `src/colors.rs:197,
268`).

Tests (`src/widgets/tests.rs`):

- `a_switch_keeps_one_tab_stop`: in the `toggler` harness (the switch is the
  window's only focusable element), after `focus_next` the focused handle is
  recorded and `space` toggles; after a second `focus_next` the focused
  handle is the same one. It proves the change adds no second tab stop
  (measured in the trial: fails when a second handle is added to the
  track); it cannot prove that the base switch tracks the connector's
  handle, which only the manual check below shows.
- The existing `a_switch_toggles_on_click_and_from_the_keyboard` and
  `a_disabled_switch_is_inert` (which checks a click only) keep passing.
- The ring itself is not observable headlessly (gpui-component names its
  ring element only under its own `cfg(test)`, `styled.rs:264-267`); it is
  checked by hand: Tab to an *unchecked* switch on the Inputs page under a
  light and a dark preset and see the ring — the only proof that
  `track_focus` took effect, and the case where a borderless track's 50 %
  halo is least visible (rationale K10).

Showcase: `info/inputs.rs` gains a focus-ring colour claim on every
*enabled* Switch entry (a disabled switch takes no focus). Upstream's
switch: `ring`, from `t.ring`, citing `gpui-component/styled.rs:175-189`. The
connector's switch, whose entry has no `Theme` in hand (`native_switch(r,
…)`): `focus_ring_color`, from `stated(r.defaults.focus_ring_color)`, citing
the connector line that maps it (`native-theme-gpui/colors.rs:197`) — a
colour claim's value must read the field it names, or the gate rejects it.

README (`## Accessibility`): one line — the native `Switch` draws the
keyboard focus ring in `defaults.focus_ring_color`.

---

## 8 -- Showcase additions (K11, K12)

Each new section follows the showcase's existing rules: every widget is
built in a `demo.rs` helper that reports it (`.info(…)`; built directly in a
page it fails `every_widget_reports_itself`), every colour it claims is cited
at the line that reads it, geometry comes from `geometry::*` builders or is
stated as upstream's.

### 8.1 `geometry::toolbar` leaves the height to the content

`src/geometry.rs`, `toolbar`: the refinement starts with `.h_auto()` (before
the optional `min_h(bar_height)`), and the doc says so: the height is the
content's, at least `toolbar.bar_height` where the platform states one; an
application that wants a fixed height sets it after the refinement. Reason:
0.7.0's component `Toolbar` sets `h_8` at Small (`toolbar.rs:252-256`),
which the builder's `min_h` alone would leave in place on KDE and GNOME,
whose toolbars size to their content (`docs/platform-facts.md:1351`). Test
(`src/geometry.rs`, `toolbar_carries_the_models_toolbar`): the refinement's
`size.height` is `Some(Length::Auto)` for every preset. The chrome's `h_flex`
row is unaffected (§11 checks it).

The chrome toolbar itself stays an `h_flex` row (rationale K12).

### 8.2 Component `Toolbar` and `ToolbarGroup` (Buttons page)

A section "Toolbar", built by a `demo.rs` helper:
`gpui_component::toolbar::Toolbar::new("buttons-toolbar-row")` (its own id;
`"buttons-toolbar"` is the `.info` wrapper's), refined with `geometry::toolbar`
only (its padding and gap where stated; no fallback, no edge — the section
shows the widget, not the chrome row); two groups,
`ToolbarGroup::new("buttons-toolbar-edit").label("Edit")` holding Undo2,
Redo2, Copy and `ToolbarGroup::new("buttons-toolbar-view").label("View")`
holding Search, Maximize, Settings, each added with `content(…)` (with
`child(…)` a group, being `Sizable`, would be wrapped in an `input_h` box,
`toolbar.rs:29-35`); each group spaced by `toolbar.item_gap` where stated,
otherwise by upstream's own Small toolbar gap `gap_1` (`toolbar.rs:254`; a
`ToolbarGroup` spaces nothing itself), stated as such in Widget Info; its
ghost icon buttons also added with `content(…)`, built as
`demo::toolbar_button` builds its button (`demo.rs:1122-1160`) minus the
action and the tooltip, with the icon from the chosen icon theme and the
button's label as text where the theme lacks the glyph (as the chrome's
`tool_label` does), each reporting itself through a Widget Info entry of
its own (`info::buttons::toolbar_item`); a vertical `Separator`
(`separator::Separator::vertical()`, `separator.rs:28`) added with
`content(…)` between the groups. Widget Info: the Small-size literals
`h_8 p_1 gap_1 text_sm` (`toolbar.rs:252-256`) and that the refinement
(applied after them, `:258`) replaces them; that `child()` would force
`ghost().compact()` and an `input_h` wrapper of 1.5 rem at Small
(`toolbar.rs:17-37`, `button.rs:564-566`, `sizing.rs:273`), which is why the
showcase uses `content()`.

### 8.3 Inputs page

- **TimeField**: `TimeFieldState::new(window, cx)` (gpui-base
  `time_field.rs:308`) held on `Showcase` beside `date_picker_state`,
  `TimeField::new(&state)` in a `demo.rs` helper, next to the DatePicker.
  Widget Info colours: `input_background` and `foreground` through
  `input_style` (`input/input.rs:98-106`), `input` border, `danger` when
  invalid, `selection` on the active segment, focus ring
  (`time/time_field.rs:98-135`); geometry `radius`, `input_h`.
- **Input with a token**: an `Input` whose `InputState` (=
  `InputBaseState<InputMode>`, gpui-base `input/input/mod.rs:10`) holds one
  token, inserted once at construction inside its `cx.new` with
  `state.replace_with_token(InlineToken::new("mention", "@native-theme"), window, cx)`
  (gpui-base `input/base/inline_tokens.rs:19, 394-401`; re-exported as
  `gpui_component::input::InlineToken`). It returns
  `Result<(), InlineTokenError>`: on `Err` the field keeps its plain text and
  the showcase logs the error with `eprintln!` — no panic. Rendered in a
  `demo.rs` helper with `.token(|ctx, _, _| InputToken::new(ctx))`
  (`input/input.rs:178-186`). Measured in the trial: the token is accepted
  (value `"@native-theme"`, one token span) and drawn. A showcase test,
  `the_token_input_holds_its_token`, asserts one token span with text
  `"@native-theme"` (`InlineTokenSpan::range()` / `.token().text()`; its
  fields are private), so a refused token cannot go unnoticed behind the
  `eprintln!`. Widget Info colours:
  `radius`, `selection` (selected) / `muted` + `border`, `foreground`
  (`input/token.rs:44-56`).
- **Questionnaire**: not shown (§8.5, rationale K12). **DatePicker with a
  time**: not shown (rationale K12).

### 8.4 Charts page

No new chart (rationale K11).

- The existing Bar, Line and Area charts: their grid claims move from
  `border` to `chart_grid` (`chart/mod.rs:419`, `bar_chart.rs:857`);
  Candlestick keeps `border` (`candlestick_chart.rs:302-304`). The colour
  gate forces this as soon as it runs on 0.7.0 (the cited lines now read
  `chart_grid`), so it happens with the other colour claims (§9.3).
- All charts: `no_hover` is replaced by claims for the tooltip — surface
  `popover`, citing `gpui-component/styled.rs:197` (`popover_style`, which
  `plot/tooltip.rs:590` calls: the colour gate reads the cited line), label
  `muted_foreground` (`tooltip.rs:545`), and the guide line per chart: Line
  and Area a dashed hairline of `border` mixed toward `foreground`
  (`:125-127`); Bar and Candlestick a band of `foreground` at 8 % (`:128`,
  set by `.band(…)`, `bar_chart.rs:1109`, `candlestick_chart.rs:433`); Pie
  none — and the hover-dimming literals (`bar_chart.rs:26`,
  `pie_chart.rs:30`).

### 8.5 Coverage exceptions

`docs/showcase-exceptions.toml` `[gpui]`: remove `Text`; re-read every
remaining entry at gpui-component 0.7.0 and correct its line citations
(`Attachment*`, `Bubble*`, `Message*` moved; `WindowBorder`'s reason
rewritten: drawn by gpui-component's `WindowState::decorate`, `root.rs:450-458`);
the header's "gpui-component 0.6.6 (first read at 0.6.4 …)" becomes 0.7.0.

`Questionnaire`, `QuestionnaireActions`, `QuestionnaireChoice`,
`QuestionnaireChoices`, `QuestionnaireError`, `QuestionnaireInput`,
`QuestionnaireItem` and `QuestionnaireProgress` are excepted under a comment
that names the reason (it is none of the header's six kinds, and the header
gains it as a seventh: "a widget whose state the toolkit only builds
fallibly, which the showcase cannot hold without a panic path"). The reason
text, for `Questionnaire`: "it and its parts take an
`Entity<QuestionnaireState>`, whose only constructor returns `Result`
(gpui-base questionnaire/state.rs:40-45) while gpui builds entities
infallibly (gpui-pre app.rs:2971); no `Default` or unchecked constructor
exists, and upstream's own tests unwrap it (questionnaire/components.rs:1480).
Shown once upstream offers an infallible constructor (docs/todo.md)"; each of
the seven parts: "a part of `Questionnaire`, built from its state
(questionnaire/components.rs:N), excepted with it", N being the part's `new`
line in gpui-component 0.7.0: `QuestionnaireProgress` 377, `QuestionnaireItem`
536, `QuestionnaireChoices` 616, `QuestionnaireChoice` 700,
`QuestionnaireInput` 1089, `QuestionnaireError` 1160, `QuestionnaireActions`
1246 (`Questionnaire` itself: 308). `QuestionnaireChoiceDescription` needs no
state (`new() -> Self`, `:1032`) and is excepted as a composition slot, an
existing kind: "secondary text inside a `QuestionnaireChoice` card
(questionnaire/components.rs:1023-1032), shown where its parent is".
Gate: `python3 scripts/check_widget_coverage.py` prints "Every widget is shown
or excepted."

---

## 9 -- Citations and prose (K13, K14)

### 9.1 Library and README

Every entry of Appendix A is applied: the connector line's citation is
rewritten to the new line(s) **after opening the new line and confirming it
says what the prose claims**. Module `//!` headers that name the verified
stack say "gpui-component 0.7.0, gpui-base 0.7.0, gpui-pre 0.3.7".
References to gpui-component 0.5.1 are historical and stay.

### 9.2 Citations whose code was rewritten (re-read, not shifted)

| Connector | Cited | 0.7.0 |
|---|---|---|
| `src/contract.rs:1667, 1748, 1870, 1875` | `list/list_item.rs:209, 236-242, 237` | the idle / selected / secondary-selected branches at `:225-232, 257-275`; the right-clicked row's `selection` outline is new |
| `src/contract.rs:110-112` | "every reader of `selection` is a text selection" | false, and already in 0.6.6 (`table/state.rs:2212`); 0.7.0 readers: `input/input.rs:550`, `text/mod.rs:52`, `touch_selection/handle.rs:96`, `table/state.rs:2261`, `list/list_item.rs:272`, `input/token.rs:50-51`, `time/time_field.rs:135` |
| `src/lib.rs:199`, `src/geometry.rs:404`, `tests/seams.rs:106` | `root.rs:582, 590` | `WindowState::prepare` / `style`, gpui-component `root.rs:436, 444` |
| `src/lib.rs:915-917` | `theme/mod.rs:287-289` ("refreshes only the window passed") | false: `edit` refreshes every window (`:278-316`) |
| `src/lib.rs:995-999` | `theme/mod.rs:283-284` | `set_global` of the base theme now happens only in `sync_base` (`:453-458`) |
| `src/geometry.rs:455-459` | `dialog/dialog.rs:535` (`max_h` formula) | `:584`, `view − 2·margin − layer_offset` |
| `src/geometry.rs:559` | `group_box.rs:156-161` | `:172-177`, nested in a footer column |
| `src/icons.rs:76-79` | "ActionRefresh … no corresponding Lucide icon" | false (§3.2) |
| `README.md:41-48, 505, 520`, `src/icons.rs:153, 272, 416`, showcase `support.rs:539`, `info/icons.rs:49` | "101 variants", "101-icon gallery", "101 in gpui_component::IconName" (`README.md:45`'s "0.6.1 … keeps the same 101 variants" is history and stays) | 104 |
| `README.md:244` (`toolbar` row) | "gpui-component has no toolbar widget" | 0.7.0 has one; `geometry::toolbar` applies to it (the height is the content's, §8.1); add items with `content()` — `child()` makes buttons compact ghosts in a 1.5 rem box (`toolbar.rs:17-37`) |
| `README.md:520` | "138-field colour map" | 139 |
| `README.md:380` (Accessibility, `text_scaling_factor`) | "`Root` sets the window rem to `font_size`" | gpui-component's `WindowState` plugin does (`root.rs:434-436`) |
| `README.md` Quick start (`:96-110`) | "// open windows as usual" | `gpui_kit::open_window(…)` after `init`, and one sentence: a window opened before `gpui_kit::init` gets no `WindowState` — no dialogs, no rem, so text scaling fails silently (gpui-component `root.rs:434-436`; gpui-base `root.rs:43-44`) |
| `scripts/check_showcase_parity.py:184-185` | gpui-pre 0.3.6 `window.rs:4673` | 0.3.7 `:4665` (re-read) |
| `src/base_layer.rs:113-116` | the splitter colours reach resize handles | only handles gpui-base draws itself (§6) |
| `src/showcase.rs:1062-1089` | `Root::render_*_layer`, `Root::new` | removed (§5.5) |

### 9.3 Showcase

The gates drive it: `every_colour_claim_is_read_at_the_line_it_cites` (234
claims at first; 291 after §4 inserts lines into `src/colors.rs`, whose lines
the Theme Map's value claims cite — measured in the trial),
`every_prose_citation_still_exists` (`info/typography.rs:350`:
cite gpui-base `text/node.rs:3324-3334`, `BlockNode::Heading`),
`the_mirrored_handle_constants_match_gpui_base` (`demo.rs:2051, 2056` →
`resize_handle.rs:12, 13`), `every_theme_color_field_has_one_theme_token`
(a `ChartGrid` variant of the `ThemeToken` enum in `demo.rs`, and
`chart_grid` in the Chart group of `info/theme_map.rs` and
`pages/theme_map.rs`, its value claim citing the connector's own
`tc.chart_grid` line in `src/colors.rs`, as every Theme Map value claim does).

The gates cannot see these, which are rewritten by hand:

- Attachment: `info/data.rs:766, 778, 784, 789, 805-808, 834-838` per
  rationale §1.4(g).
- List/Tree right-click: `demo.rs:4144-4146`, `info/data.rs:437, 481`; split
  `ListRowState::RightClicked` (`demo.rs:4150-4156`) into right-clicked and
  selected-and-right-clicked, the latter claiming the `selection` outline
  (`list_item.rs:257-275`).
- Charts `no_hover`: §8.4 (the grid claims are the colour gate's, above).
- Textarea: `info/inputs.rs:152` — it now has a Size seam
  (`input/textarea.rs:161-188`).
- Popover: `info/overlays.rs:279` — cite the inline surface at
  `popover.rs:351-356`.
- Root prose: `info/buttons.rs:534`, `info/text.rs:79, 154`,
  `info/typography.rs:33, 220` → `WindowState::prepare` / `style`
  (gpui-component `root.rs:434-448`); `info/leaves.rs:690, 715` → `:444`;
  `tests.rs:3113` → `:436`; `app.rs:304`, `support.rs:903`,
  `chrome.rs:432, 454` (`Root::open_dialog` → `WindowState::open_dialog`,
  `root.rs:235-265`); `inspector.rs:266-270, 308`
  (`WindowState::decorate`, `root.rs:450-458`; the Server arm returns the
  content itself, `window_border.rs:134-139`; client inset `:155`).
- Stale line numbers in comments: `support.rs:967` (`table/state.rs:800`),
  `support.rs:1003-1004` (`:2009`), `demo.rs:6809` (`dialog.rs` ≈ `:678`).

---

## 10 -- Documentation

### 10.1 README (`connectors/native-theme-gpui/README.md`)

Compatibility table (`:63-66`) and Quick start (`:92`): 0.7.0 / 0.3.7 (the
`compat` tests enforce both). `:482`: 0.3.7. `:41-48`: 104 variants, audited
against 0.7.0's `default-icons.txt`. Splitter lines per §6. A line under
Accessibility: the native `Switch` draws the keyboard focus ring. The
`<!-- compat:begin -->` Verified line is written only by §12.

### 10.2 CHANGELOG (`## [Unreleased]`)

- Changed: **native-theme-gpui** requires gpui-component / gpui-base 0.7.0
  and gpui-pre 0.3.7 (gpui-kit 0.7.0 for the showcase).
- Added: icon mappings for `Ban`, `CircleAlert`, `RefreshCw`, and
  `icon_name(ActionRefresh)`.
- Changed: `chart_grid` from `list.grid_color`; the native `Switch` draws
  the focus ring; `geometry::toolbar` leaves the height to the content (at
  least `toolbar.bar_height`), so it fits gpui-component's new `Toolbar`.
- Documented: which resizables the splitter colours reach
  (`gpui_base::h_resizable` yes, at 1 px; gpui-component's, `Settings`, the
  dock no); that `gpui_kit::init` must precede the first window.
- Showcases (gpui): Toolbar and ToolbarGroup, TimeField, an input with an
  inline token; chart grids in `chart_grid`; the icon gallery's 104 icons.
- Fixed (gpui showcase): palette entries and shortcuts work over dialogs and
  sheets on 0.7.0; the capture pointer shield covers top-level dialogs,
  menus and popovers (it never covered them).

### 10.3 `docs/todo.md`

Append (never rewrite existing entries):

- `Settings` and dock splitters draw upstream's renderer; ask upstream for a
  handle-appearance setter on `Settings` (gpui-component
  `setting/settings.rs:416`, `dock/dock.rs:143`).
- Popover arrow: record per platform whether a popover has an arrow
  (`docs/platform-facts.md`, sources first), then model it.
- The toolbar's "Reload System Theme" glyph in all three showcases:
  `refresh-cw` / `refresh` / `view-refresh` instead of rotate-cw.
- Latent: the observer's base-palette repair writes 12 colours without their
  tokens (`src/lib.rs:1005-1020`); harmless while no widget reads
  `tokens.red…cyan_light`.
- A citation gate for `src/*.rs` doc comments.
- The showcases' splitter: move the gpui showcase's renderer, which paints
  `splitter.divider_width` / `divider_color` / `hover_color`
  (`examples/showcase-gpui/demo.rs:2087-2147`), into the connector as a
  public renderer; gpui-base's own line is 1 px.
- The theme-drawn widgets' focus ring (Checkbox, Slider, Switch) uses
  upstream's width and opacity; the model states
  `defaults.focus_ring_width` / `focus_ring_offset`.
- The connector's `Switch` lacks 0.7.0 upstream's `tab_stop`, `tab_index`
  and `focus_ring(bool)`.
- A toolbar role and arrow-key roving for the chrome toolbar of all three
  showcases (gpui-base `Toolbar` offers both; egui and iced need their own).
- `scripts/check_widget_coverage.py` discovers no chart (charts derive
  `IntoPlot`, not `IntoElement`); teach it `IntoPlot`, then show what it
  finds — today `RadarChart` and `SankeyChart`, never shown.
- Ask upstream for an infallible `QuestionnaireState` constructor (or schema
  validation at a later step), then show `Questionnaire` and drop its nine
  exceptions (`docs/showcase-exceptions.toml`).

---

## 11 -- Layout comparison and captures

The parity comparator and the captures cover only the shared chrome and the
Basic page (`docs/showcase-elements.toml`; the capture scripts pass
`--tab basic`). Nothing this work changes should move an element there, and
this is checked directly on the gpui showcase instead of re-running all
three showcases: layout dumps before the upgrade and after it, compared
element by element.

- **Before** (on the branch before any task, gpui-kit 0.6.6), and **after**
  (all tasks through §8 done): for each pairing of
  `scripts/generate_screenshots_gpui.sh` (`kde-breeze` dark/light with
  `--icon-set freedesktop --icon-theme breeze-dark`/`breeze`, `material`
  dark/light with `--icon-set material`, `catppuccin-mocha` dark/light with
  `--icon-set lucide`), run
  `cargo run -p native-theme-gpui --example showcase-gpui -- --theme T
  --variant V --icon-set S [--icon-theme I] --tab basic --dump-layout
  DIR/gpui-T-V.json` (without `--capture` the showcase quits after writing,
  `main.rs:693-697`), once plain into `rest/` and once with
  `--open-menu theme` into `menu/`. `DIR` is under the repository's
  `target/` (disk-backed).
- **Compare**: every element rectangle of every dump equal before and after,
  edges rounded to the pixel grid as the parity comparator rounds them
  (its R-snap rule, `scripts/check_showcase_parity.py` docstring). Expected: no
  difference. Any difference is investigated and reported with its cause in
  0.7.0 before anything else happens; if the maintainer accepts it, the
  three-way comparator (`scripts/check_showcase_parity.py`) and its
  `--merge --write` regeneration of `[parity]` are run as for any rc1 change.

The screenshots and the theme-switching GIF are regenerated by the
maintainer's `scripts/generate_assets_release.sh` run, as for every release,
not by this work.

---

## 12 -- Release gates

In order, from the repository root, after everything above is committed:

1. `CARGO_BUILD_JOBS=4 cargo clippy -p native-theme-gpui --all-targets
   --all-features --locked -- -D warnings` — clean.
2. `CARGO_BUILD_JOBS=4 cargo test -p native-theme-gpui --all-features
   --locked` and `… --lib --no-default-features` — green.
3. `CARGO_BUILD_JOBS=4 cargo test -p native-theme --test icon_sources` —
   green.
4. `RUSTDOCFLAGS="-D warnings" cargo doc -p native-theme-gpui --no-deps
   --all-features` — clean.
5. `python3 scripts/check_widget_coverage.py` — "Every widget is shown or
   excepted."
6. `cargo +1.95.0 check -p native-theme-gpui --lib --locked` — builds.
7. `./scripts/update_compatibility.sh run gpui` — writes the stamp and the
   README's Verified line (0.7.0 / 0.3.7); commit its output.
8. `./scripts/check_release.sh` — passes with two warnings, both expected
   while the CHANGELOG says Unreleased: the compatibility stamp, and the
   visual-assets stamp (`docs/assets/PROVENANCE.toml` hashes the connector's
   `src` and `examples`, `scripts/update_provenance.sh:29-44`, which this work
   changes; `check_release.sh:776-800` warns, and fails only once the
   CHANGELOG entry is dated). The maintainer's next
   `scripts/generate_assets_release.sh` run restamps it — including over an
   asset run captured before this work.

---

## 13 -- Archiving

When §12 passes, the three documents move to `docs/archive/` with
`git mv` — so they are committed first, before the first code change (plan
Task 1) — and any link to them from `docs/todo.md` is updated.

---

## 14 -- Acceptance

- The twelve probe failures of rationale §1.3 pass, and the tests of
  §4.5, §5.4 and §7 exist and pass.
- The coverage script is clean; the layout comparison of §11 shows no
  unexplained difference.
- The manual checks were done and reported: a focused unchecked switch
  draws the ring under a light and a dark preset (§7); the new Buttons and
  Inputs sections look right in a capture (plan Task 8); and the plan's
  Review Focus checks (a shortcut in the Preferences sheet, an open menu
  under the shield).
- `grep -rn '0\.6\.6\|0\.3\.6' connectors/native-theme-gpui/ --include='*.rs'
  --include='*.toml' --include='*.md'` finds only historical references, each
  marked as such.
- No `Root::render_` and no `Root::new(` remain in the showcase.

---

## Appendix A -- Library and README citations that moved

Mechanically mapped from 0.6.6 / 0.3.6 to 0.7.0 / 0.3.7 (203 entries), then
checked line by line in both versions: 185 cite identical code at the new
line; 12 are marked `CHANGED(…)` or `moved:` — see §9.2, never shift those
blindly; and 6 ranges start at the same code but their bodies changed, so
they too are re-read, not shifted:
`src/base_layer.rs:6` (`theme/mod.rs` — the projection gained `plot`),
`src/config.rs:94` (`theme_color.rs:59-343` — gained `chart_grid`),
`src/geometry.rs:176` (`button/button.rs:657-694` — the open state),
`src/geometry.rs:253` (`input/input.rs:579-593` — the editor padding cap),
`src/geometry.rs:255` (`input/textarea.rs:177-201` — tokens and size),
`src/geometry.rs:458` (`dialog/dialog.rs:584` — the `max_h` formula).
Format: `<connector file>:<line at cc2f3b81>  <crate> <file>:<old> -> <new>`.

```text
README.md:223  gpui-component input/input.rs:700-702 -> 762-764
src/base_layer.rs:6  gpui-component theme/mod.rs:294-347 -> 383-437
src/base_layer.rs:39  gpui-component theme/mod.rs:310-330 -> 399-419
src/base_layer.rs:47  gpui-component theme/mod.rs:309 -> 398
src/base_layer.rs:55  gpui-component theme/mod.rs:331-340 -> 420-429
src/base_layer.rs:115  gpui-component theme/mod.rs:342-344 -> 431-433
src/base_layer.rs:116  gpui-base resizable/resize_handle.rs:12 -> 13
src/base_layer.rs:128  gpui-base theme.rs:31-36 -> 32-37
src/colors.rs:129  gpui-component theme/theme_color.rs:178-179 -> 180-181
src/colors.rs:190  gpui-component theme/theme_color.rs:226 -> 228
src/colors.rs:190  gpui-component input/input.rs:502 -> 550
src/colors.rs:273  gpui-component button/button.rs:1139 -> 1170
src/colors.rs:273  gpui-component button/button.rs:1215 -> 1246
src/colors.rs:273  gpui-component button/button.rs:1257 -> 1288
src/colors.rs:273  gpui-component button/button.rs:1133 -> 1164
src/colors.rs:273  gpui-component button/button.rs:1210 -> 1241
src/colors.rs:273  gpui-component button/button.rs:1250 -> 1281
src/colors.rs:331  gpui-component button/button.rs:929-994 -> 960-1025
src/colors.rs:396  gpui-component table/state.rs:1528 -> 1579
src/colors.rs:396  gpui-component table/state.rs:1768 -> 1819
src/colors.rs:409  gpui-component table/state.rs:1769 -> 1820
src/colors.rs:410  gpui-component theme/schema.rs:1007 -> 1011
src/colors.rs:413  gpui-component table/state.rs:1424 -> 1475
src/colors.rs:511  gpui-component dialog/dialog.rs:277-279 -> 319-321
src/colors.rs:603  gpui-component theme/schema.rs:640-674 -> 643-677
src/colors.rs:605  gpui-component theme/schema.rs:688-696 -> 691-699
src/colors.rs:605  gpui-component theme/schema.rs:1075-1079 -> 1079-1083
src/colors.rs:1064  gpui-component theme/theme_color.rs:226 -> 228
src/config.rs:49  gpui-component theme/schema.rs:1066-1073 -> 1070-1077
src/config.rs:94  gpui-component theme/theme_color.rs:59-341 -> 59-343
src/config.rs:98  gpui-component theme/schema.rs:640-674 -> 643-677
src/config.rs:105  gpui-component theme/schema.rs:640-674 -> 643-677
src/config.rs:107  gpui-component theme/schema.rs:688-696 -> 691-699
src/config.rs:107  gpui-component theme/schema.rs:1075-1079 -> 1079-1083
src/contract.rs:110  gpui-component theme/theme_color.rs:226 -> 228
src/contract.rs:111  gpui-component input/input.rs:502 -> 550
src/contract.rs:111  gpui-component text/mod.rs:51 -> 52
src/contract.rs:123  gpui-component button/button.rs:993 -> 1024
src/contract.rs:133  gpui-component theme/theme_color.rs:178-179 -> 180-181
src/contract.rs:135  gpui-component button/button.rs:1139 -> 1170
src/contract.rs:135  gpui-component button/button.rs:1215 -> 1246
src/contract.rs:135  gpui-component button/button.rs:1257 -> 1288
src/contract.rs:136  gpui-component button/button.rs:1133 -> 1164
src/contract.rs:136  gpui-component button/button.rs:1210 -> 1241
src/contract.rs:136  gpui-component button/button.rs:1250 -> 1281
src/contract.rs:363  gpui-component table/state.rs:1528 -> 1579
src/contract.rs:363  gpui-component table/state.rs:1768 -> 1819
src/contract.rs:364  gpui-component theme/schema.rs:968 -> 972
src/contract.rs:364  gpui-component theme/schema.rs:1006 -> 1010
src/contract.rs:403  gpui-component table/state.rs:1769 -> 1820
src/contract.rs:411  gpui-component table/state.rs:1424 -> 1475
src/contract.rs:411  gpui-component table/state.rs:1527 -> 1578
src/contract.rs:411  gpui-component table/state.rs:1978 -> 2030
src/contract.rs:411  gpui-component table/state.rs:2233 -> 2289
src/contract.rs:906  gpui-component button/button.rs:1133 -> 1164
src/contract.rs:907  gpui-component button/button.rs:1210 -> 1241
src/contract.rs:907  gpui-component button/button.rs:1250 -> 1281
src/contract.rs:911  gpui-component theme/theme_color.rs:178-179 -> 180-181
src/contract.rs:967  gpui-component switch.rs:136-139 -> 172-175
src/contract.rs:968  gpui-component switch.rs:95 -> 102
src/contract.rs:1653  gpui-component table/state.rs:1768-1769 -> 1819-1820
src/contract.rs:1667  gpui-component list/list_item.rs:209 -> CHANGED(replace ~225-232)
src/contract.rs:1667  gpui-component list/list_item.rs:237 -> CHANGED(replace ~257-262)
src/contract.rs:1700  gpui-component button/button.rs:1133 -> 1164
src/contract.rs:1700  gpui-component button/button.rs:1210 -> 1241
src/contract.rs:1700  gpui-component button/button.rs:1250 -> 1281
src/contract.rs:1748  gpui-component list/list_item.rs:237 -> CHANGED(replace ~257-262)
src/contract.rs:1763  gpui-component input/input.rs:502 -> 550
src/contract.rs:1870  gpui-component list/list_item.rs:209 -> CHANGED(replace ~225-232)
src/contract.rs:1870  gpui-component list/list_item.rs:236-242 -> CHANGED(replace ~257-262)-CHANGED(replace ~264-274)
src/contract.rs:1871  gpui-component theme/schema.rs:967-968 -> 971-972
src/contract.rs:1871  gpui-component theme/schema.rs:1002 -> 1006
src/contract.rs:1875  gpui-component list/list_item.rs:209 -> CHANGED(replace ~225-232)
src/geometry.rs:36  gpui-component sizing.rs:236-237 -> 244-245
src/geometry.rs:37  gpui-component sizing.rs:261-264 -> 269-272
src/geometry.rs:157  gpui-component button/button.rs:689 -> 720
src/geometry.rs:157  gpui-component button/button.rs:690 -> 721
src/geometry.rs:157  gpui-component input/input.rs:699 -> 761
src/geometry.rs:157  gpui-component input/input.rs:719 -> 781
src/geometry.rs:158  gpui-component list/list_item.rs:182-193 -> 198-209
src/geometry.rs:158  gpui-component select.rs:535-546 -> 541-552
src/geometry.rs:176  gpui-component button/button.rs:626-663 -> 657-694
src/geometry.rs:176  gpui-component button/button.rs:690 -> 721
src/geometry.rs:177  gpui-component button/button.rs:698-706 -> 729-737
src/geometry.rs:189  gpui-component button/button.rs:690 -> 721
src/geometry.rs:192  gpui-component sizing.rs:319-325 -> 327-333
src/geometry.rs:201  gpui-component input/input.rs:704-714 -> 766-776
src/geometry.rs:201  gpui-component input/input.rs:719 -> 781
src/geometry.rs:205  gpui-component input/input.rs:706-709 -> 768-771
src/geometry.rs:208  gpui-component input/input.rs:256-260 -> 281-285
src/geometry.rs:210  gpui-component input/input.rs:708 -> 770
src/geometry.rs:210  gpui-component input/input.rs:719 -> 781
src/geometry.rs:212  gpui-component input/input.rs:700-702 -> 762-764
src/geometry.rs:217  gpui-component input/input.rs:701 -> 763
src/geometry.rs:218  gpui-component input/input.rs:719 -> 781
src/geometry.rs:221  gpui-component input/input.rs:736 -> 798
src/geometry.rs:227  gpui-component input/input.rs:700-702 -> 762-764
src/geometry.rs:241  gpui-component input/input.rs:704-714 -> 766-776
src/geometry.rs:242  gpui-component input/input.rs:719 -> 781
src/geometry.rs:253  gpui-component input/input.rs:531-541 -> 579-593
src/geometry.rs:254  gpui-component input/input.rs:700-702 -> 762-764
src/geometry.rs:255  gpui-component input/textarea.rs:141-158 -> 177-201
src/geometry.rs:309  gpui-component list/list_item.rs:185-187 -> 201-203
src/geometry.rs:309  gpui-component list/list_item.rs:193 -> 209
src/geometry.rs:312  gpui-component list/list_item.rs:189 -> 205
src/geometry.rs:393  gpui-pre elements/text.rs:649-656 -> 656-663
src/geometry.rs:404  gpui-component root.rs:582 -> moved: WindowState::prepare, root.rs:436 (0.7.0)
src/geometry.rs:427  gpui-component popover.rs:284 -> 308
src/geometry.rs:427  gpui-component popover.rs:312 -> 360
src/geometry.rs:428  gpui-component popover.rs:284 -> 308
src/geometry.rs:452  gpui-component dialog/dialog.rs:538-548 -> 587-597
src/geometry.rs:452  gpui-component dialog/dialog.rs:616-617 -> 672-673
src/geometry.rs:452  gpui-component dialog/dialog.rs:621 -> 677
src/geometry.rs:455  gpui-component dialog/dialog.rs:616 -> 672
src/geometry.rs:458  gpui-component dialog/dialog.rs:535 -> 584 (formula changed)
src/geometry.rs:459  gpui-component dialog/dialog.rs:631 -> 682
src/geometry.rs:465  gpui-component dialog/dialog.rs:540-552 -> 589-601
src/geometry.rs:467  gpui-component dialog/dialog.rs:620 -> 676
src/geometry.rs:468  gpui-component dialog/dialog.rs:656 -> 707
src/geometry.rs:485  gpui-component button/button.rs:593-595 -> 624-626
src/geometry.rs:559  gpui-component group_box.rs:105 -> 107
src/geometry.rs:559  gpui-component group_box.rs:156-161 -> 172-177 (now nested in a footer column)
src/geometry.rs:569  gpui-component accordion.rs:219 -> 220
src/geometry.rs:569  gpui-component accordion.rs:300-306 -> 301-307
src/geometry.rs:616  gpui-component sizing.rs:236-237 -> 244-245
src/geometry.rs:616  gpui-component sizing.rs:261-264 -> 269-272
src/geometry.rs:617  gpui-component select.rs:544 -> 550
src/geometry.rs:617  gpui-component select.rs:546 -> 552
src/geometry.rs:621  gpui-component select.rs:544 -> 550
src/geometry.rs:622  gpui-component select.rs:546 -> 552
src/geometry.rs:627  gpui-component select.rs:592-593 -> 598-599
src/geometry.rs:628  gpui-component select.rs:557-599 -> 563-605
src/geometry.rs:630  gpui-component select.rs:544 -> 550
src/geometry.rs:648  gpui-component select.rs:535-545 -> 541-551
src/geometry.rs:648  gpui-component select.rs:546 -> 552
src/geometry.rs:653  gpui-component select.rs:544 -> 550
src/geometry.rs:653  gpui-component select.rs:546 -> 552
src/geometry.rs:657  gpui-component select.rs:592-593 -> 598-599
src/geometry.rs:658  gpui-component select.rs:557-599 -> 563-605
src/geometry.rs:663  gpui-component select.rs:539 -> 545
src/geometry.rs:667  gpui-component select.rs:477-479 -> 483-485
src/geometry.rs:734  gpui-component icon.rs:182 -> 188
src/geometry.rs:735  gpui-component icon.rs:160 -> 166
src/geometry.rs:735  gpui-component icon.rs:189 -> 195
src/geometry.rs:804  gpui-component dialog/dialog.rs:419 -> 466
src/geometry.rs:815  gpui-component input/input.rs:719 -> 781
src/geometry.rs:828  gpui-component input/input.rs:639-650 -> 694-705
src/geometry.rs:829  gpui-component input/input.rs:711 -> 773
src/geometry.rs:829  gpui-component input/input.rs:719 -> 781
src/geometry.rs:837  gpui-component theme/mod.rs:379-385 -> 465-471
src/geometry.rs:838  gpui-component input/input.rs:646-650 -> 701-705
src/geometry.rs:882  gpui-component button/button.rs:1273-1319 -> 1304-1350
src/geometry.rs:882  gpui-component button/button.rs:1284 -> 1315
src/geometry.rs:884  gpui-component button/button.rs:754-760 -> 785-791
src/geometry.rs:885  gpui-component button/button.rs:690 -> 721
src/geometry.rs:950  gpui-component theme/mod.rs:482-484 -> 568-570
src/geometry.rs:951  gpui-component dialog/dialog.rs:528 -> 576
src/geometry.rs:1557  gpui-component list/list_item.rs:189 -> 205
src/geometry.rs:1557  gpui-component list/list_item.rs:193 -> 209
src/geometry.rs:1558  gpui-component select.rs:539 -> 545
src/geometry.rs:1558  gpui-component select.rs:546 -> 552
src/geometry.rs:1560  gpui-component select.rs:477-479 -> 483-485
src/icons.rs:901  gpui-pre app.rs:2841-2851 -> 2845-2855
src/icons.rs:902  gpui-pre window.rs:4997-5008 -> 4989-5000
src/icons.rs:1261  gpui-pre app.rs:2841-2851 -> 2845-2855
src/icons.rs:1262  gpui-pre window.rs:4997-5008 -> 4989-5000
src/lib.rs:199  gpui-component root.rs:582 -> moved: WindowState::prepare, root.rs:436 (0.7.0)
src/lib.rs:916  gpui-component theme/mod.rs:287-289 -> CHANGED(replace ~378-378)-CHANGED(replace ~378-378)
src/lib.rs:917  gpui-pre app.rs:1153 -> 1156
src/lib.rs:995  gpui-component theme/schema.rs:640-674 -> 643-677
src/lib.rs:997  gpui-component theme/schema.rs:688-696 -> 691-699
src/lib.rs:997  gpui-component theme/schema.rs:1075-1079 -> 1079-1083
src/lib.rs:998  gpui-component theme/mod.rs:283-284 -> CHANGED(replace ~378-378)-CHANGED(replace ~378-378)
src/lib.rs:999  gpui-component theme/mod.rs:367-371 -> 453-457
src/lib.rs:1024  gpui-pre app.rs:1767-1769 -> 1770-1772
src/lib.rs:1025  gpui-pre app.rs:1924-1928 -> 1927-1931
src/lib.rs:1028  gpui-pre app.rs:2189-2201 -> 2192-2204
src/showcase.rs:1847  gpui-component button/button.rs:993-994 -> 1024-1025
src/showcase.rs:1992  gpui-component theme/mod.rs:379-384 -> 465-470
src/variants.rs:6  gpui-component button/button.rs:1125-1132 -> 1156-1163
src/widgets/mod.rs:8  gpui-component switch.rs:150-157 -> 186-193
src/widgets/mod.rs:171  gpui-pre window.rs:4513-4521 -> 4505-4513
src/widgets/switch.rs:17  gpui-component switch.rs:197 -> 236
src/widgets/switch.rs:158  gpui-component switch.rs:169-178 -> 205-214
src/widgets/switch.rs:322  gpui-component switch.rs:237 -> 290
tests/seams.rs:106  gpui-component root.rs:582 -> moved: WindowState::prepare, root.rs:436 (0.7.0)
tests/seams.rs:106  gpui-component root.rs:590 -> moved: RootPlugin::style, root.rs:444 (0.7.0)
tests/seams.rs:278  gpui-pre app/test_context.rs:342-367 -> 347-372
tests/seams.rs:545  gpui-component input/input.rs:724 -> 786
tests/seams.rs:600  gpui-component select.rs:535 -> 541
tests/seams.rs:606  gpui-component input/input.rs:701 -> 763
tests/seams.rs:607  gpui-component input/input.rs:719 -> 781
tests/seams.rs:607  gpui-component select.rs:544 -> 550
tests/seams.rs:607  gpui-component select.rs:546 -> 552
tests/seams.rs:717  gpui-component select.rs:557-599 -> 563-605
tests/seams.rs:732  gpui-component select.rs:592-593 -> 598-599
tests/seams.rs:733  gpui-component select.rs:557-599 -> 563-605
tests/seams.rs:801  gpui-component input/textarea.rs:162-165 -> 205-208
tests/seams.rs:816  gpui-component input/input.rs:700-702 -> 762-764
tests/seams.rs:816  gpui-component input/input.rs:719 -> 781
tests/seams.rs:817  gpui-component input/input.rs:736 -> 798
tests/seams.rs:866  gpui-component input/input.rs:700-702 -> 762-764
tests/seams.rs:873  gpui-component input/input.rs:719 -> 781
```
