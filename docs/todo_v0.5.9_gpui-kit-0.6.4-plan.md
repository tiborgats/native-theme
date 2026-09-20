# v0.5.9 — gpui connector on GPUI Kit 0.6.4: Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make `native-theme-gpui` build, behave and stay verifiably correct on gpui-component / gpui-base / gpui-kit 0.6.4 with gpui-pre 0.3.5, and prepare release v0.5.9.

**Architecture:** Move the dependency floors and the lockfile; delete the three uses of the upstream-removed `tiles` colour; name every `Theme` field in one test so the next upstream field change stops the build; replace the unconditional `set_reduce_motion` with a helper that undoes only the connector's own switch, so gpui-base's new OS reader keeps control; add headless rendered tests that measure six real widgets with and without the connector's geometry; refresh every upstream citation; show the new upstream components in the showcase; archive the implemented v0.5.8 design documents.

**Tech Stack:** Rust 2024, gpui-pre 0.3.5 (`#[gpui::test]`, `TestAppContext`, `debug_selector` / `debug_bounds`), gpui-component 0.6.4, gpui-base 0.6.4.

**Spec:** [`todo_v0.5.9_gpui-kit-0.6.4-spec.md`](todo_v0.5.9_gpui-kit-0.6.4-spec.md) — read it first. Reasons: [`todo_v0.5.9_gpui-kit-0.6.4-rationale.md`](todo_v0.5.9_gpui-kit-0.6.4-rationale.md).

## Global Constraints

- Floors, verbatim: `gpui = { package = "gpui-pre", version = "0.3.5" }`, `gpui-component = "0.6.4"`, `gpui-base = "0.6.4"`, dev `gpui-kit = { version = "0.6.4", features = ["tree-sitter-rust"] }`. Carets, never `=`.
- Connector `rust-version = "1.95.0"` (measured; do not change).
- No `unwrap` / `expect` / `panic!` / indexing / `unsafe` in non-test code. Repository PreToolUse hooks enforce this on every Write/Edit under `src/`, test modules included: keep `#[cfg(test)]` in the same edit as any `.expect`.
- No invented values: every expected number in a test comes from the resolved theme or from the refinement under test, never from a literal (the text scale `1.5` is the established test input).
- No public item added, removed or re-typed.
- No `Co-Authored-By` or other AI attribution in commits.
- All cargo commands: `CARGO_BUILD_JOBS=4` (the gpui stack OOMs this machine otherwise).
- Every command in this plan runs from the **repository root**, and every path is written from there. No step changes directory.
- The repository's PreToolUse panic hook (`.claude/hooks/no-runtime-panics.sh:47`) lets an edit through only when the text being written contains `#[test]`, `#[cfg(test)]` or `#[allow(clippy::unwrap_used` — `#[gpui::test]` does **not** match. An edit that adds `.expect(` to a file under `src/` must therefore carry one of those markers in the same edited region (`tests/` files are exempt by path, `:39`).
- Never tag, push a tag, publish or create a release. Task 8 stops before that.
- Work on branch `v0.5.9-gpui-kit-0.6.4`, created from `main`.
- If a step's expected output does not appear, stop and report; do not improvise around it.

## File map

| File | Responsibility in this plan |
|---|---|
| `connectors/native-theme-gpui/Cargo.toml` | floors, MSRV comment |
| `Cargo.lock` | 0.6.4 / 0.3.5 |
| `connectors/native-theme-gpui/src/colors.rs`, `src/config.rs` | `tiles` removal, counts |
| `connectors/native-theme-gpui/src/lib.rs` | `Theme` shape tripwire, `ReduceMotionRequest`, `forward_reduce_motion`, tests, docs |
| `connectors/native-theme-gpui/tests/seams.rs` (new) | rendered seam tests |
| `connectors/native-theme-gpui/src/{base_layer,geometry,icons}.rs`, `README.md`, `examples/showcase-gpui.rs` | citations, wording, swatch, five new showcase sections |
| `docs/archive/` | the three v0.5.8 design documents move here (and these three after the release) |
| `docs/todo.md`, `CHANGELOG.md`, workspace `Cargo.toml`, `native-theme/Cargo.toml` | follow-ups, notes, version |

---

### Task 1: Floors, lockfile, `tiles`

One task because none of the three compiles without the other two.

**Files:**
- Modify: `connectors/native-theme-gpui/Cargo.toml:6-10, 34-41, 54-58`
- Modify: `Cargo.lock`
- Modify: `connectors/native-theme-gpui/src/colors.rs:487, 1031-1057`, `src/config.rs:189, 344-354`
- Modify: `connectors/native-theme-gpui/src/lib.rs` (prose counts; the `Theme` shape tripwire in `mod tests`)
- Modify: `connectors/native-theme-gpui/examples/showcase-gpui.rs:5527-5528`

**Interfaces:** Consumes nothing. Produces a green build on the 0.6.4 stack that every later task needs.

- [ ] **Step 1: Branch**

```bash
git switch -c v0.5.9-gpui-kit-0.6.4
```

- [ ] **Step 2: Manifest.** Replace the three blocks with the text of spec §2 (the `rust-version` comment, the `[dependencies]` gpui / gpui-component / gpui-base block with its comments, the `gpui` and `gpui-kit` `[dev-dependencies]` entries with their comments). Keep every other line.

- [ ] **Step 3: Lockfile**

```bash
cargo update -p gpui-kit -p gpui-component -p gpui-base -p gpui-kit-assets -p gpui-pre
```
Expected: `Updating gpui-component v0.6.0 -> v0.6.4`, `Updating gpui-pre v0.3.3 -> v0.3.5`, 13 `Removing` lines, and exactly six `Adding` lines: `objc2-screen-capture-kit`, `tree-sitter`, `tree-sitter-json`, `tree-sitter-language` (cargo notes a newer 0.1.8 is available; the gpui stack pins 0.1.7), `tree-sitter-rust` and `streaming-iterator`. The last five come from the new dev feature, which is why this step runs after Step 2.

- [ ] **Step 4: Security advisory in the lockfile** (spec §2, E19). Not compatibility work; it is here because `cargo audit` is a hard CI job and the release commit has to pass it.

```bash
cargo audit ; echo "audit exit $?"
cargo update -p rustls
cargo audit ; echo "audit exit $?"
```
Expected: first run `error: 1 vulnerability found!` (`RUSTSEC-2026-0285`, `rustls 0.23.43`, present on `main` too) and exit 1; `Updating rustls v0.23.43 -> v0.23.45` with `Locking 1 package`; second run exit 0 with six allowed warnings. If the first run is already clean, upstream fixed it first: skip the update and say so.

- [ ] **Step 5: See the gate fail**

Run: `CARGO_BUILD_JOBS=4 cargo check -p native-theme-gpui --all-targets --all-features --locked`
Expected: exactly three `error[E0609]: no field \`tiles\`` (`src/colors.rs:487`, `src/config.rs:189` ×2), printed once for `lib` and once for `lib test`. The showcase's own `t.tiles` error does not appear yet, because the example cannot build before the library does.

- [ ] **Step 6: Delete the uses.**
  - `src/colors.rs`: delete the line `    tc.tiles = c.bg;` and the blank line after it.
  - `src/config.rs`: delete the line `    colors.tiles = h(tc.tiles);`.
  - `examples/showcase-gpui.rs`: the row ends

    ```rust
                    .child(color_swatch("window_border", t.window_border))
                    .child(color_swatch("tiles", t.tiles)),
    ```
    and becomes

    ```rust
                    .child(color_swatch("window_border", t.window_border)),
    ```

- [ ] **Step 7: See the tripwires fail**

Run: `CARGO_BUILD_JOBS=4 cargo test -p native-theme-gpui --all-features --locked --lib`
Expected: `187 passed; 3 failed` — `theme_color_field_count_tripwire`, `no_theme_color_field_is_left_at_default`, `theme_config_colors_cover_the_0_6_fields`. Any other failure: stop and report.

- [ ] **Step 8: Move the numbers.**
  - `src/colors.rs` `theme_color_field_count_tripwire`: comment `// ThemeColor has 138 Hsla fields in gpui-component 0.6.4 (each 16 bytes`; `field_count, 138,`.
  - `src/colors.rs` `no_theme_color_field_is_left_at_default`: `139,` → `138,`.
  - `src/config.rs`: comment `// Every field ThemeConfigColors exposes is exported: 138 ThemeColor`; `assert_eq!(exported, 126, "config colours exported");`.
  - Prose: `grep -n '139\|127' connectors/native-theme-gpui/src/*.rs connectors/native-theme-gpui/README.md connectors/native-theme-gpui/examples/showcase-gpui.rs` and change each hit that counts colour fields to 138 / 126 (`src/colors.rs:1, 3, 141`, `src/config.rs:5, 19, 87`, `src/lib.rs:49, 122, 128`, `README.md:13, 282, 306`, showcase `:21`). Do not touch unrelated numbers (line heights, pixel values).

- [ ] **Step 9: Guard `Theme`'s shape** (spec §3a). The three fields that broke this release were `Theme` fields the connector never named. Add this test to `mod tests` in `src/lib.rs`, directly above `fn to_theme_produces_valid_theme`:

```rust
    /// Compile-time tripwire: `gpui_component::Theme` has exactly these 20
    /// fields in 0.6.4. Upstream carried three `tile_*` fields through 0.6.0
    /// and 0.6.1 that this connector never set, and nothing noticed until
    /// 0.6.2 removed them again; a field *added* in a patch release would be
    /// left at upstream's default just as silently. Naming every field here
    /// turns either change into a compile error.
    #[test]
    fn every_theme_field_is_named() {
        let theme = to_theme(
            &test_resolved(),
            "Test",
            true,
            &AccessibilityPreferences::default(),
        );
        let GpuiTheme {
            // Set by `to_theme` from the resolved native theme.
            mode: _,
            font_family: _,
            font_size: _,
            mono_font_family: _,
            mono_font_size: _,
            radius: _,
            radius_lg: _,
            shadow: _,
            focus_ring: _,
            scrollbar_mode: _,
            highlight_theme: _,
            // `to_theme` fills the slot for `mode` only; the other keeps
            // `Theme::from(&ThemeColor)`'s empty `ThemeConfig::default()` until
            // a later `apply` stores the other variant (D34).
            light_theme: _,
            dark_theme: _,
            // Written by `Theme::from(&ThemeColor)`: the 138-field colour map,
            // the tokens upstream derives from it, and `transparent` (Issue 53).
            colors: _,
            tokens: _,
            transparent: _,
            // Left at upstream's default on purpose: native-theme models no
            // notification placement, list highlight default, sheet margin or
            // motion tokens (`ResolvedTheme` has no such field).
            notification: _,
            list: _,
            sheet: _,
            motion: _,
        } = theme;
    }
```

- [ ] **Step 10: Prove the guard can fail.** Delete the line `            motion: _,` from the pattern and run `CARGO_BUILD_JOBS=4 cargo test -p native-theme-gpui --all-features --locked --lib`.
Expected: `error[E0027]: pattern does not mention field \`motion\`` at that test. Restore the line.

- [ ] **Step 11: Green**

```bash
CARGO_BUILD_JOBS=4 cargo clippy -p native-theme-gpui --all-targets --all-features --locked -- -D warnings
CARGO_BUILD_JOBS=4 cargo test -p native-theme-gpui --all-features --locked
grep -rn 'tiles' connectors/native-theme-gpui/ ; echo "grep exit $?"
cargo tree -p native-theme-gpui -i gpui-pre | head -1
```
Expected: clippy clean; `191 passed; 0 failed` for the lib (190 + the shape tripwire); `grep exit 1`; `gpui-pre v0.3.5`.

- [ ] **Step 12: Commit**

```bash
git add Cargo.lock connectors/native-theme-gpui
git commit -m "fix(gpui): build on gpui-component 0.6.4 (upstream removed ThemeColor::tiles)"
```

---

### Task 2: Reduced motion — undo only the connector's own switch

Spec §5. The code below was run as written in the design probe (clippy clean, 194 library tests pass: 190 + the shape tripwire + these three). The `apply_system_reduce_motion` call is a no-op under GPUI's test scheduler, so the tests cover the connector's own writes only (spec §5.3).

**Files:**
- Modify: `connectors/native-theme-gpui/src/lib.rs` (helper directly above `fn apply_inner`; call sites `:697`, `:756`; `apply` docs step 6; tests in `mod apply_tests`)
- Modify: `connectors/native-theme-gpui/README.md:206`

**Interfaces:**
- Consumes: `gpui_base::apply_system_reduce_motion(cx: &mut App)` (public in gpui-base 0.6.4), `App::reduce_motion() -> bool`, `App::set_reduce_motion(bool)`, `App::default_global::<G: Global + Default>() -> &mut G`.
- Produces (private): `struct ReduceMotionRequest(bool)`, `fn forward_reduce_motion(reduce: bool, cx: &mut App)`.

- [ ] **Step 1: Write the tests** at the **top of `mod apply_tests`**, immediately after its `use super::*;` line. Anchor the edit on the module header (`src/lib.rs:1376-1379`, `#[cfg(test)]` / `#[allow(clippy::unwrap_used, clippy::expect_used)]` / `mod apply_tests {` / `use super::*;`) so that the text being written carries `#[cfg(test)]`: these tests use `.expect(`, and the panic hook does not recognise `#[gpui::test]` on its own (Global Constraints). The module already has `fn prefs(reduce_motion: bool)` and imports `TestAppContext`:

```rust
    /// Spec v0.5.9 §5: `false` is the absence of a request. gpui-base 0.6.2+
    /// (or the application) may have switched the flag on; a preset applied
    /// with default preferences must not switch it off.
    #[gpui::test]
    fn apply_with_default_prefs_keeps_a_flag_it_did_not_set(cx: &mut TestAppContext) {
        let prefs = AccessibilityPreferences::default();
        let (theme, resolved) =
            from_preset("catppuccin-mocha", true, &prefs).expect("preset should load");
        cx.update(|cx| {
            cx.set_reduce_motion(true); // stands for gpui-base's OS reading
            apply(theme, &resolved, &prefs, cx);
            assert!(cx.reduce_motion());
        });
    }

    #[gpui::test]
    fn releasing_a_request_keeps_a_flag_that_was_already_on(cx: &mut TestAppContext) {
        cx.update(|cx| {
            cx.set_reduce_motion(true);
            apply_accessibility(&prefs(true), cx);
            apply_accessibility(&prefs(false), cx);
            assert!(cx.reduce_motion(), "the connector never switched it on");
        });
    }

    #[gpui::test]
    fn a_repeated_request_is_released_once(cx: &mut TestAppContext) {
        cx.update(|cx| {
            apply_accessibility(&prefs(true), cx);
            apply_accessibility(&prefs(true), cx);
            assert!(cx.reduce_motion());
            apply_accessibility(&prefs(false), cx);
            assert!(!cx.reduce_motion());
        });
    }
```

- [ ] **Step 2: Run them**

Run: `CARGO_BUILD_JOBS=4 cargo test -p native-theme-gpui --all-features --locked --lib -- default_prefs_keeps releasing_a_request repeated_request`
Expected: the first two FAIL (today's code clears the flag); the third passes already and guards the implementation.

- [ ] **Step 3: Implement.** Add directly above `fn apply_inner`:

```rust
/// Whether the connector switched `App::reduce_motion` on and has not yet
/// switched it back (spec v0.5.9 §5).
#[derive(Default)]
struct ReduceMotionRequest(bool);

impl Global for ReduceMotionRequest {}

/// `true` is a request: switch the flag on, remembering that the connector did
/// so if it was off. `false` is the absence of a request: undo the connector's
/// own switch, if any, and ask gpui-base (which reads the OS preference itself
/// since 0.6.2) to re-read the system. gpui-base writes only a flag it still
/// owns, so a flag the application set is never cleared here.
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

Replace both `cx.set_reduce_motion(prefs.reduce_motion);` (in `apply_accessibility`'s `None` arm and in `apply_inner`) with `forward_reduce_motion(prefs.reduce_motion, cx);`. `Global` is already imported (`use gpui::{App, Global, SharedString, px};`).

- [ ] **Step 4: Run the whole lib suite**

Run: `CARGO_BUILD_JOBS=4 cargo test -p native-theme-gpui --all-features --locked --lib`
Expected: `194 passed; 0 failed` (191 + 3). The older `apply_accessibility_forwards_reduce_motion` passes unchanged.

- [ ] **Step 5: Docs.**
  - `apply` doc list, step 6 becomes:

    ```rust
    /// 6. forwards a reduced-motion *request*: `prefs.reduce_motion == true`
    ///    switches `App::reduce_motion` on if it is off; a later `false` undoes
    ///    only that switch, never clears a flag the connector did not set, and
    ///    asks gpui-base (which reads the OS preference itself since 0.6.2) to
    ///    re-read the system;
    ```
  - After the paragraph that starts "Call `gpui_component::init`", add a paragraph "Reduced motion:" with the three limits of spec §5.4, same meaning, as `///` prose.
  - `apply_accessibility` docs: "only `reduce_motion` is forwarded" → "only the reduced-motion request is forwarded".
  - `README.md:206`: the row text of spec §5.4.

- [ ] **Step 6: Lint and commit**

```bash
CARGO_BUILD_JOBS=4 cargo clippy -p native-theme-gpui --all-targets --all-features --locked -- -D warnings
CARGO_BUILD_JOBS=4 cargo doc -p native-theme-gpui --no-deps --all-features
git add connectors/native-theme-gpui
git commit -m "fix(gpui): a reduced-motion request no longer overwrites a flag the connector did not set"
```

---

### Task 3: Rendered seam tests

Spec §6. The file below was run as written in the design probe: 6 tests pass (unstyled → styled: button 32 → 40, input 32 → 34, select 32 → 34, combobox 32 → 34, list item 34 → 30, progress 8 → 6 px), clippy `-D warnings` and `cargo fmt --check` are clean, and removing a `refine_style` call makes that test fail (run on `input` and `combobox`). It resolves presets through `ResolutionContext::for_tests()`, not `from_preset`, so the expected heights do not depend on the machine's font DPI (spec §6.2).

**Files:**
- Create: `connectors/native-theme-gpui/tests/seams.rs`
- Modify: `connectors/native-theme-gpui/src/geometry.rs` (module docs, one sentence)

**Interfaces:** Consumes the public API only: `native_theme_gpui::{Native, apply, geometry, to_theme}` plus `native_theme::{AccessibilityPreferences, ResolutionContext, theme::{ColorMode, ResolvedTheme, Theme}}`.

- [ ] **Step 1: Create the file**

```rust
//! Rendered seam tests (spec v0.5.9 §6).
//!
//! Each `geometry` builder rests on one upstream behaviour: the widget applies
//! its own size defaults and *then* the caller's style. These tests lay real
//! gpui-component widgets out on GPUI's headless test platform, once without
//! and once with the builder's refinement, and compare the measured height
//! with the refinement's own field. When an upstream release stops honouring
//! a seam, this file fails (the nightly dependency canary runs it).
//!
//! Every test: `u` (unstyled) must differ from `e` (expected), otherwise the
//! input does not discriminate; then `s` (styled) must equal `e`. Text scale
//! 1.5 is the input that takes `control_height` past the presets' minimum.
//!
//! `button`, `input` and `progress` set the same `size.height` their widget
//! sets for itself, so they also prove the ordering of the two writes.
//! `select` and `combobox` set `min_size.height` and `list_item` a height the
//! widget leaves content-driven, so those three prove that the refinement
//! reaches the widget's root box and wins there, not an ordering claim.
//! Every other builder rests on the source citation in its doc comment.

use gpui::{
    AbsoluteLength, AnyElement, AppContext as _, Context, DefiniteLength, InteractiveElement as _,
    IntoElement, Length, ParentElement as _, Pixels, Render, SharedString, Size, StyleRefinement,
    Styled as _, TestAppContext, Window, div, px,
};
use gpui_component::{
    StyledExt as _,
    button::Button,
    combobox::{Combobox, ComboboxState},
    input::{Input, InputState},
    list::ListItem,
    progress::Progress,
    select::{SearchableVec, Select, SelectState},
};
use native_theme::theme::{ColorMode, ResolvedTheme, Theme};
use native_theme::{AccessibilityPreferences, ResolutionContext};
use native_theme_gpui::{Native, apply, geometry, to_theme};

fn scaled() -> AccessibilityPreferences {
    AccessibilityPreferences {
        text_scaling_factor: 1.5,
        ..AccessibilityPreferences::default()
    }
}

/// Resolve a preset without consulting the machine. `ResolutionContext::for_tests`
/// pins the font DPI at 96, so the point sizes the presets declare convert to
/// the same pixels everywhere; `from_preset` would resolve through
/// `ResolutionContext::from_system`, whose DPI is read from the desktop
/// (`native-theme/src/resolve/context.rs:62-83`) and would make the expected
/// height machine-dependent.
fn resolved(preset: &str) -> ResolvedTheme {
    Theme::preset(preset)
        .expect("preset loads")
        .into_variant(ColorMode::Light)
        .expect("light variant")
        .into_resolved(&ResolutionContext::for_tests())
        .expect("preset resolves")
}

type Build = fn(Option<&StyleRefinement>, &mut Window, &mut Context<Harness>) -> AnyElement;

struct Harness {
    style: Option<StyleRefinement>,
    build: Build,
}

impl Render for Harness {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let widget = (self.build)(self.style.as_ref(), window, cx);
        div().size(px(600.)).flex().items_start().child(
            div()
                .debug_selector(|| "probe".into())
                .flex_none()
                .child(widget),
        )
    }
}

fn native_style(preset: &str, build: fn(Native<'_>) -> StyleRefinement) -> StyleRefinement {
    let prefs = scaled();
    let resolved = resolved(preset);
    build(Native {
        resolved: &resolved,
        accessibility: &prefs,
    })
}

fn measure(
    cx: &mut TestAppContext,
    preset: &str,
    style: Option<StyleRefinement>,
    build: Build,
) -> Size<Pixels> {
    let prefs = scaled();
    let resolved = resolved(preset);
    let theme = to_theme(&resolved, preset, false, &prefs);
    cx.update(|cx| {
        gpui_component::init(cx);
        apply(theme, &resolved, &prefs, cx);
    });
    let (_, cx) = cx.add_window_view(|_, _| Harness { style, build });
    cx.update(|window, cx| window.draw(cx).clear(cx));
    cx.debug_bounds("probe").expect("probe was laid out").size
}

fn px_of(length: Option<Length>) -> Pixels {
    match length {
        Some(Length::Definite(DefiniteLength::Absolute(AbsoluteLength::Pixels(p)))) => p,
        other => panic!("the builder no longer sets an absolute length: {other:?}"),
    }
}

fn assert_seam(u: Pixels, s: Pixels, e: Pixels, what: &str) {
    assert_ne!(
        u, e,
        "{what}: the unstyled widget already measures the native value; this input proves nothing, pick another preset"
    );
    assert_eq!(
        s, e,
        "{what}: upstream no longer honours the caller's style"
    );
}

fn styled<T: gpui::Styled + IntoElement>(w: T, style: Option<&StyleRefinement>) -> AnyElement {
    match style {
        Some(style) => w.refine_style(style).into_any_element(),
        None => w.into_any_element(),
    }
}

fn button(s: Option<&StyleRefinement>, _: &mut Window, _: &mut Context<Harness>) -> AnyElement {
    styled(Button::new("b").label("OK"), s)
}
fn input(s: Option<&StyleRefinement>, w: &mut Window, cx: &mut Context<Harness>) -> AnyElement {
    let state = cx.new(|cx| InputState::new(w, cx));
    styled(Input::new(&state), s)
}
fn select(s: Option<&StyleRefinement>, w: &mut Window, cx: &mut Context<Harness>) -> AnyElement {
    let items: Vec<SharedString> = vec!["a".into(), "b".into()];
    let state = cx.new(|cx| SelectState::new(SearchableVec::new(items), None, w, cx));
    styled(Select::new(&state), s)
}
fn combobox(s: Option<&StyleRefinement>, w: &mut Window, cx: &mut Context<Harness>) -> AnyElement {
    let items: Vec<SharedString> = vec!["a".into(), "b".into()];
    let state = cx.new(|cx| ComboboxState::new(SearchableVec::new(items), vec![], w, cx));
    styled(Combobox::new(&state), s)
}
fn list_item(s: Option<&StyleRefinement>, _: &mut Window, _: &mut Context<Harness>) -> AnyElement {
    styled(ListItem::new("i").child("x"), s)
}
fn progress(s: Option<&StyleRefinement>, _: &mut Window, _: &mut Context<Harness>) -> AnyElement {
    styled(Progress::new("p"), s)
}

macro_rules! seam {
    ($name:ident, $preset:literal, $build:expr, $geom:expr, $field:ident . $sub:ident) => {
        #[gpui::test]
        fn $name(cx: &mut TestAppContext) {
            let style = native_style($preset, $geom);
            let e = px_of(style.$field.$sub);
            let u = measure(cx, $preset, None, $build).height;
            let s = measure(cx, $preset, Some(style), $build).height;
            assert_seam(u, s, e, stringify!($name));
        }
    };
}
seam!(
    button_takes_the_native_height,
    "kde-breeze",
    button,
    geometry::button,
    size.height
);
seam!(
    input_takes_the_native_height,
    "kde-breeze",
    input,
    geometry::input,
    size.height
);
// kde-breeze's combo box minimum equals upstream's own height; adwaita's is larger.
seam!(
    select_takes_the_native_min_height,
    "adwaita",
    select,
    geometry::select,
    min_size.height
);
// Same refinement as `select`, but a different upstream seam (`combobox.rs`).
seam!(
    combobox_takes_the_native_min_height,
    "adwaita",
    combobox,
    geometry::combobox,
    min_size.height
);
seam!(
    list_item_takes_the_native_height,
    "kde-breeze",
    list_item,
    geometry::list_item,
    size.height
);
seam!(
    progress_takes_the_native_height,
    "kde-breeze",
    progress,
    geometry::progress,
    size.height
);
```

- [ ] **Step 2: Run**

Run: `CARGO_BUILD_JOBS=4 cargo test -p native-theme-gpui --all-features --locked --test seams`
Expected: `6 passed; 0 failed`. A failure of the `assert_ne!` ("proves nothing") means a preset's value changed since 2026-09-19: report it, do not loosen the assertion.

- [ ] **Step 3: Prove a test can fail.** In `fn input`, temporarily replace `styled(Input::new(&state), s)` with `Input::new(&state).into_any_element()` (and `let _ = s;`). Run Step 2's command. Expected: `input_takes_the_native_height` FAILS with "upstream no longer honours the caller's style". Restore the line; run again; 6 pass.

- [ ] **Step 4: `src/geometry.rs` module docs.** Add one sentence: "`button`, `input`, `select`, `combobox`, `list_item` and `progress` are verified against real gpui-component widgets in `tests/seams.rs`; the other builders rest on the source citations in their doc comments."

- [ ] **Step 5: Lint and commit**

```bash
CARGO_BUILD_JOBS=4 cargo clippy -p native-theme-gpui --all-targets --all-features --locked -- -D warnings
git add connectors/native-theme-gpui
git commit -m "test(gpui): rendered seam tests for six height-setting geometry builders"
```

---

### Task 4: Citations and wording

Judgment work: no test can tell whether a cited line still *means* what the comment says. Do it by reading, inline, not by delegation.

**Files:** `connectors/native-theme-gpui/src/{lib,colors,config,base_layer,geometry,icons}.rs`, `examples/showcase-gpui.rs`, `README.md`, `docs/todo_gpui-full-theme.md:48`

- [ ] **Step 1: List the targets**

Run from the repository root, with spec §7's wider pattern — the narrow one misses six citations that continue on the next line:

```bash
grep -nE '0\.6\.0|0\.3\.3|[a-z_/]+\.rs:[0-9]|`:[0-9]+(-[0-9]+)?`|\(:[0-9]+' \
  connectors/native-theme-gpui/src/*.rs \
  connectors/native-theme-gpui/examples/*.rs \
  connectors/native-theme-gpui/README.md | grep -v 'freedesktop'
```
Expected: 75 lines (measured on `558da83`; a few are not citations, e.g. `src/lib.rs:137`). The narrower `grep -n '0\.6\.0\|0\.3\.3\|[a-z_/]*\.rs:[0-9]'` returns 69 and is not sufficient.

- [ ] **Step 2: For each line**, open the cited file in `~/.cargo/registry/src/*/gpui-component-0.6.4/`, `gpui-base-0.6.4/` or `gpui-pre-0.3.5/`; find the code the comment describes; rewrite the line numbers. The `refine_style` lines are already in spec §4, and the six bare-form citations with their new targets are in spec §7's table. If the cited code no longer does what the comment claims: **stop and report the claim**; do not reword to fit. Two known cases that are replacements, not shifts: `schema.rs:657-668` (three comments) becomes `:640-674`, and `base_layer.rs:29-31`'s explanation of `inset` must be re-read against 0.6.4's private `ThumbGeometry` (`gpui-base src/scrollbar.rs:1176-1231`, painted at `:1505-1510`), which now also clamps the inset to half the thumb length.
- [ ] **Step 3: Version wording** per spec §7: each module header states "verified against gpui-component 0.6.4, gpui-base 0.6.4, gpui-pre 0.3.5" once; inline citations lose the repeated version. `0.5.1` references stay. `src/icons.rs:138, 257, 401, 1694` and showcase `:488`: "0.6.0" → "0.6.4". README `:35` likewise, `:272` snippet → `0.3.5`. Showcase comments `:1366` ("0.6.0 added …") and `:5229` ("Button (0.6.0): …") are history and stay, as does `src/lib.rs:137` (native-theme's own v0.6.0).
- [ ] **Step 4: `menu_item` (spec §8, rationale §1.4i).** `src/geometry.rs:92` doc comment and `README.md:153` (receiver column): the text of spec §8's `menu_item` row. `docs/todo_gpui-full-theme.md:48`: in the v0.5.8 cell, "delivered (R) for application-built `MenuItem`" → "builder delivered for application-drawn rows (upstream's `MenuItemElement` is crate-private, corrected in v0.5.9)", and while the line is open fix its other citation: `popup_menu.rs:749` → `:775` (`PopupMenuItem::new` in 0.6.4). Leave the rest of the row. Do not change the function.
- [ ] **Step 5: `geometry::dialog` (spec §4, §8; rationale §1.4k, E18).** Do not change the function. Add to its doc comment: "0.6.4 clamps the dialog to what is left of the viewport *after* applying this style (`dialog/dialog.rs:631`), so `dialog.max_height` no longer reaches upstream's `Dialog`; `min_height` and the paddings still do, and the width goes through [`dialog_max_width`] and `Dialog::max_w`." Add the same sentence, shortened, to the `dialog` row of README's per-widget geometry table.
- [ ] **Step 6: README**, end of "Per-widget geometry": "The `button`, `input`, `select`, `combobox`, `list_item` and `progress` builders are verified against real gpui-component widgets in `tests/seams.rs`."
- [ ] **Step 7: Check and commit**

```bash
grep -rn '0\.6\.0\|0\.3\.3' connectors/native-theme-gpui/
CARGO_BUILD_JOBS=4 cargo doc -p native-theme-gpui --no-deps --all-features
git add connectors/native-theme-gpui docs/todo_gpui-full-theme.md
git commit -m "docs(gpui): upstream citations re-verified against 0.6.4 / gpui-pre 0.3.5"
```
Expected: the grep prints exactly the three lines spec §10 names (`src/lib.rs` "Planned for unification in v0.6.0", showcase "0.6.0 added `SliderEvent::Release`" and "Button (0.6.0):"); docs build without warnings.

---

### Task 5: Showcase — what 0.6.4 added or changed

Spec §8a, §8c. No test can judge a rendering; the gate is "compiles under clippy `-D warnings`, runs, and the maintainer looks at it" (Task 8).

**Files:**
- Modify: `connectors/native-theme-gpui/examples/showcase-gpui.rs`
- Modify: `connectors/native-theme-gpui/README.md` ("What gets mapped"), `docs/todo.md` ("Upstream PR candidates from v0.5.8 (Tier U)")

**Interfaces:** Consumes Task 1's manifest (the `tree-sitter-rust` dev feature).

- [ ] **Step 1: Learn the file's pattern.** `grep -n 'section("' connectors/native-theme-gpui/examples/showcase-gpui.rs` lists the sections; read one small one end to end (e.g. `"Kbd (keyboard shortcuts)"`) and the struct fields / `new` that hold widget state entities (`grep -n 'InputState::new' connectors/native-theme-gpui/examples/showcase-gpui.rs`).
- [ ] **Step 2: One section at a time, in this order, compiling after each:** `"InputGroup"` next to `"Text Input"`; `"Empty"` next to `"Skeleton Placeholders"`; `"Carousel"` after `"Collapsible"` (`:3947`; the second `"Accordion"` at `:5460` is the colour-map heading, not this one); `"Code editor (Rust)"` and `"Markdown"` after `"Muted & Monospace Text"`. Content per spec §8a's table. Constructors: read `src/input/group.rs`, `src/empty.rs`, `src/carousel/` in `~/.cargo/registry/src/*/gpui-component-0.6.4/`; `TextView::markdown(id, source)`; the editor is `Editor::new(&state)` over `cx.new(|cx| EditorState::new(window, cx).language("rust"))` (`gpui_component::input::{Editor, EditorState}`). State entities go where the existing ones live. Every colour, size and gap comes from `cx.theme()` / the resolved theme, as the neighbouring sections do; no literals beyond what neighbouring sections already use for layout.

  Run after each: `CARGO_BUILD_JOBS=4 cargo clippy -p native-theme-gpui --example showcase-gpui --all-features --locked -- -D warnings`
  Expected: clean.
- [ ] **Step 3: A widget that cannot be built as described** (missing public constructor, needs an asset): stop and report; do not substitute another widget.
- [ ] **Step 4: Ghost-hover note.** `README.md` ("What gets mapped") and `docs/todo.md` ("Upstream PR candidates") each gain the bullet of spec §8c, verbatim.
- [ ] **Step 5: Smoke run** (needs a display; skip and say so if there is none): `cargo run -p native-theme-gpui --example showcase-gpui`, open each new section once, confirm no panic in the terminal.
- [ ] **Step 6: Commit**

```bash
git add connectors/native-theme-gpui docs/todo.md
git commit -m "feat(showcase-gpui): InputGroup, Empty, Carousel, code editor and Markdown sections"
```

---

### Task 6: Archive the v0.5.8 design documents

- [ ] **Step 1**

```bash
git mv docs/todo_v0.5.8_gpui-component-0.6-rationale.md docs/todo_v0.5.8_gpui-component-0.6-spec.md docs/todo_v0.5.8_gpui-component-0.6-plan.md docs/archive/
grep -rn 'todo_v0.5.8_gpui-component' --include='*.md' --include='*.rs' --include='*.toml' --include='*.sh' . | grep -v '^./docs/archive/'
```
- [ ] **Step 2:** Rewrite **prose citations only**. The grep also matches the `git mv …` command and the grep pattern recorded in the v0.5.9 documents themselves — those record the move and must stay untouched. The real citation sites are `ROADMAP.md:78` and two absolute GitHub URLs in the connector, `README.md:175` and `src/lib.rs:85` (the crate-level rustdoc that ships to docs.rs): in both URLs `docs/` becomes `docs/archive/`. `docs/todo.md` and `docs/todo_gpui-full-theme.md` cite the v0.5.8 specification by section number only and need no edit. Links *inside* the three moved files to each other stay valid; links from them to `../` files need one more `../` (check with `grep -n '](\.\./\|](todo_' docs/archive/todo_v0.5.8_*`).
- [ ] **Step 3:** Re-run the grep: every remaining hit is either an `archive/` path or one of the recorded commands. Commit: `git add -A docs ROADMAP.md connectors/native-theme-gpui && git commit -m "docs: archive the implemented v0.5.8 gpui design documents"`.

---

### Task 7: Project documents and version

**Files:** `docs/todo.md`, `CHANGELOG.md`, `Cargo.toml:12, 22`, `native-theme/Cargo.toml:47`, `Cargo.lock`

- [ ] **Step 1: `docs/todo.md`** (append / amend, do not rewrite other items):
  - icon-table item (`:115-120`): replace "requires the connector's floor to move to gpui-component 0.6.1." with "the floor is 0.6.4 since v0.5.9, so nothing blocks this."
  - MSRV item (`:90`): "gpui-pre 0.3.3" → "gpui-pre 0.3.5 (re-measured 2026-09-19: 1.94.0 fails on `std::hint::cold_path`)".
  - under `#### Upstream PR candidates from v0.5.8 (Tier U)`, two new bullets: the ghost-hover token pair of spec §8c, and "a `Dialog::max_h` prop, or letting the caller's `max_h` win: 0.6.4 clamps the dialog to the viewport after `refine_style` (`dialog/dialog.rs:631`), so a themed `dialog.max_height` cannot reach it, while the width already has `Dialog::max_w`".
  - under `#### Research`, four new items:
    1. "gpui-component's `Theme::motion` (`MotionTokens`, eleven fields: four durations, three easings, two springs and two `Rems` travel distances, `src/theme/motion.rs:8-20`) stays at upstream's default, because `ResolvedTheme` has no motion field at all; KDE exposes `AnimationDurationFactor` and GNOME `enable-animations`, which native-theme reads only as the boolean `reduce_motion` (`native-theme/src/kde/mod.rs:42-52`, `src/detect.rs:674-685`). Decide whether native-theme should model motion — durations, easings, distances — which is a core-type change."
    2. "Preset font families that are not installed: GPUI resolves a named, missing family through its fallback stack on every text run (upstream's statement, gpui-component 0.6.4 `src/theme/system_font.rs:3-9`; not measured here). Decide whether `to_theme` should fall back to `.SystemUIFont` when `cx.text_system().all_font_names()` lacks the family, as upstream does for its own defaults."
    3. "Widen the captured screenshot tabs: every capture passes `--tab buttons` (`scripts/generate_gpui_screenshots.sh:70`, `screenshots.yml`), so the InputGroup, Empty, Carousel, code-editor and Markdown sections never appear in an artefact."
    4. "Re-verify the upstream `file:line` citations in `docs/todo.md`, `docs/todo_gpui-full-theme.md` and `ROADMAP.md`, which are still 0.6.0-era; four are known stale at 0.6.4 (`popup_menu.rs:749`, `button.rs:360`, `switch.rs:136-146`, `checkbox.rs:195-199`).
- [ ] **Step 2: `CHANGELOG.md`**: add the seven entries of spec §8 under the existing `## [Unreleased]` heading, alongside the canary / publish-job entries, using the file's own taxonomy: the one marked **Breaking Changes** (reduced motion) goes under a `### Breaking Changes` → `#### native-theme-gpui` group, as v0.5.7 and v0.5.8 filed theirs; the rest into `### Added`, `### Changed` and `### Fixed`. The raised floors are `### Changed`, not breaking: Cargo backtracks, so a pinned application keeps resolving to 0.5.8 (rationale §2.1). Do **not** date the heading or add a compare link: `pre-release-check.sh:593` makes the asset-stamp check a hard failure once `## [0.5.9] - <date>` exists; that edit belongs to the maintainer's release commit.
- [ ] **Step 3: Version.** `0.5.8` → `0.5.9` in `Cargo.toml:12`, `Cargo.toml:22`, `native-theme/Cargo.toml:47`; then `cargo check --workspace` once so `Cargo.lock` records the new workspace versions.
- [ ] **Step 4: Commit**

```bash
git add docs/todo.md CHANGELOG.md Cargo.toml native-theme/Cargo.toml Cargo.lock
git commit -m "chore: v0.5.9 version, changelog and follow-ups"
```

---

### Task 8: Verification and hand-over (stops before any release action)

- [ ] **Step 1: MSRV**

Run: `CARGO_BUILD_JOBS=4 cargo +1.95.0 check -p native-theme-gpui --lib --locked`
Expected: `Finished`.

- [ ] **Step 2: Full gate**

Run: `CARGO_BUILD_JOBS=4 ./pre-release-check.sh`, then `cargo audit`.
Expected: no failures, and one warning from the "Visual assets" section: the asset stamp no longer matches, because five stamped paths changed (`Cargo.lock`, the workspace and two crate manifests, the connector's `src/` and `examples/`; `scripts/asset-stamp.sh:28-50`). The warning is correct — it becomes a hard failure once the CHANGELOG heading is dated — and it is the reason for hand-over step 2. `cargo audit` must exit 0 (E19).

- [ ] **Step 3: Spec §10 acceptance list** — run each command, paste each result into the hand-over message.

- [ ] **Step 4: Hand over to the maintainer.** Push the branch (`git push -u origin v0.5.9-gpui-kit-0.6.4`) and open a PR only when the maintainer says so — hand-over step 2 needs the branch pushed. Report: test counts, any citation finding from Task 4, anything Task 5 could not build, and the remaining maintainer-only steps in order:
  1. run the showcase on the KDE desktop and look at it (`cargo run -p native-theme-gpui --example showcase-gpui`): scrollbars, resize handles, a mode switch, the icon gallery, and the five new sections — which no screenshot captures, every capture being `--tab buttons` (rationale §2.11). Then **ghost-button hover**: the "Button Variants" row, a dialog's close button, the tab bar, an `InputGroup` addon button. 0.6.4 paints the fill *and* the label from the accent pair, so the question is whether a selection-coloured pill is acceptable (rationale §2.10, §1.4g);
  2. push the branch (`scripts/pre-release.sh` refuses to run unless HEAD equals `@{u}`, and it triggers the screenshot workflow with `--ref <branch>`), then `./scripts/pre-release.sh` (screenshots + `docs/assets/PROVENANCE.toml`), commit;
  3. release commit `chore(release): v0.5.9`: rename `## [Unreleased]` to `## [0.5.9] - <date>` under a fresh empty `## [Unreleased]`, add `[0.5.9]: …/compare/v0.5.8...v0.5.9` above `CHANGELOG.md:998`; `./pre-release-check.sh` fully green on it (the asset check is hard now);
  4. fast-forward `main` and push; CI green on that commit; dispatch the dependency canary once on it; explicit go; `git tag -a v0.5.9` and `git push origin v0.5.9` by the maintainer; crates.io workflow; docs.rs; GitHub release page;
  5. after the release: `git mv docs/todo_v0.5.9_gpui-kit-0.6.4-{rationale,spec,plan}.md docs/todo_v0.5.9_theme-contracts-{rationale,spec,plan}.md docs/archive/` — all six, since both sets are implemented in this release — fix citing paths as in Task 6, commit (the maintainer's standing rule: implemented plans are archived).

**Do not perform any of 1–5.**

---

## Execution notes (model routing under the maintainer's dispatch policy)

Tasks 1, 2, 3, 6 and 7 have mechanical gates (compile, named failing tests, greps) and complete code or commands: suitable for the `implement` agent. Task 4 (does a cited line still mean what the comment says?) and Task 5 (a showcase has no gate beyond compiling) are judgment: orchestrator inline. Task 8 is orchestration.
