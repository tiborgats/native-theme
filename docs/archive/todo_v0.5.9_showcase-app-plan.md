# The gpui showcase as an application — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Turn the gpui showcase into an application whose chrome is real gpui-component widgets, and whose Widget Info describes the widget instance under the pointer.

**Architecture:** The example becomes a module tree. A `WidgetInfo` value is built per widget instance by one function per widget kind, attached by a demo helper that builds the widget from the same arguments, and chosen at run time by a registry that shows the innermost hovered instance after a short settle time. The window gets a client-side TitleBar with menus, a toolbar sized by a new `geometry::toolbar`, a Sidebar for navigation, an `h_resizable` body (navigation | content | inspector) and a StatusBar. Gates move from demo blocks to instances.

**Tech Stack:** Rust 1.95, gpui-component / gpui-base 0.6.6, gpui-pre 0.3.6 (the `Cargo.lock` pins), the showcase's windowed test harness (`#[gpui::test]`, `VisualTestContext`), Python 3 for `scripts/check-widget-coverage.py`.

**Spec:** [`todo_v0.5.9_showcase-app-spec.md`](todo_v0.5.9_showcase-app-spec.md)
**Rationale:** [`todo_v0.5.9_showcase-app-rationale.md`](todo_v0.5.9_showcase-app-rationale.md)

## Global Constraints

- **NEVER LIE, NEVER INVENT.** A colour claim is read at the line it cites; a note about upstream cites `file.rs, Symbol`. A claim you cannot verify is a finding to report, never a sentence to write, and never a citation guessed to make a gate pass.
- No runtime panics, no `unsafe`. The repo's PreToolUse hook enforces this under `src/`, including tests; `#[gpui::test]` fns need `#[allow(clippy::unwrap_used, clippy::expect_used)]` on their module. The example's tests follow the example's existing test module conventions.
- No hardcoded theme values. Layout defaults the model does not state are named constants whose comment says so (spec §1.3).
- `native-theme-gpui` gains exactly `geometry::toolbar` (spec §9). Nothing else in `src/` becomes public.
- Every new gate ships a discrimination proof: seed the defect, watch it fail naming file and line, remove the seed.
- Every task ends with `CARGO_BUILD_JOBS=4 ./pre-release-check.sh` green and one commit of the named files. Never `git add -A`. No `Co-Authored-By` or AI-attribution lines. Push nothing.
- The Bash tool's shell is **fish**.
- Dispatch (CLAUDE.md): a task with a mechanical gate goes to the `implement` agent; claim derivation (Tasks 14–23) is judgment and stays inline.

## File Structure (after Task 2)

```
connectors/native-theme-gpui/examples/showcase-gpui/
  main.rs         entry point, CLI, screenshot capture, window options
  app.rs          Showcase state, theme switching, actions, top-level render
  chrome.rs       title bar, menus, toolbar, navigation, status bar, overlays
  inspector.rs    the inspector panel (Widget and Theme tabs)  — exempt from §10.1
  info/mod.rs     WidgetInfo, ColorClaim, Note, GEOMETRY_NOTES, native_info
  info/registry.rs InfoRegistry, InfoExt, INFO_SETTLE
  info/<page>.rs  per-kind info functions, one file per page
  demo.rs         demo helpers: build a widget and attach its info
  pages/<page>.rs one file per page: layout of demo helpers
  support.rs      layout helpers (with_gap, native_value, …), icon loading
  tests.rs        the windowed tests
```

`[[example]] name = "showcase-gpui"` gains `path = "examples/showcase-gpui/main.rs"`.

---

## Phase 0 — the scrollbar over the Settings rows

### Task 1: Settings rows keep off the page scrollbar

**Files:**
- Modify: `connectors/native-theme-gpui/examples/showcase-gpui.rs` (Settings demo, SettingGroup calls near :5673, :5699, :5717; test module)

**Interfaces:**
- Consumes: `geometry::scrollbar_gutter` (geometry.rs:557), `NativeStyled::native` (showcase-gpui.rs:740), the test helpers `open`, `use_preset`, `show`, `bounds_of`, `scrollbar_of`.
- Produces: `PROBE_SETTINGS_ROW` selector constant.

- [ ] **Step 1: Write the failing test**

Add a constant beside the other probes and a probe element inside the first SettingGroup of the Appearance page (an item whose field is an element: `SettingItem::new(…, SettingField::element(…))` — read `setting/fields/mod.rs:62-76` for the exact constructor, and use a `div().w_full().h(px(1.)).debug_selector(|| PROBE_SETTINGS_ROW.into())` so the probe spans the row's width). Then:

```rust
/// A Settings row keeps off the page's scrollbar.
///
/// The page body lays gpui-component's `ScrollbarLayer` over its right edge
/// (setting/page.rs:222-248, scroll/scrollable.rs:19-29) and reserves only its
/// own `px_4`; kde-breeze's groove is 21px, wider than that, so without a
/// gutter on the group the bar covers the rows.
#[gpui::test]
fn a_settings_row_keeps_off_the_page_scrollbar(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    use_preset(&mut cx, &showcase, "kde-breeze");
    show(&mut cx, &showcase, Tab::Layout);
    let (groove, overlay) = scrollbar_of(&mut cx, &showcase);
    assert!(!overlay && groove > px(0.), "kde-breeze must draw a non-overlay groove");
    let frame = bounds_of(&mut cx, "settings-frame");
    let row = bounds_of(&mut cx, PROBE_SETTINGS_ROW);
    assert!(
        frame.right() - row.right() >= groove,
        "the row ends {:?} before the frame's right edge; a {groove:?} groove covers it",
        frame.right() - row.right()
    );
}
```

Give the Settings demo's framing div `.debug_selector(|| "settings-frame".into())` (not a `tt-` id: that prefix still marks demo blocks until Task 24).

- [ ] **Step 2: Run it, expect FAIL**

Run: `CARGO_BUILD_JOBS=4 cargo test -p native-theme-gpui --example showcase-gpui a_settings_row_keeps_off_the_page_scrollbar`
Expected: FAIL, the row ending about 1rem (13.33px at kde-breeze's 10pt) before the frame edge, less than 21px.

- [ ] **Step 3: Apply the gutter to every SettingGroup**

On each `SettingGroup::new()` in the Settings demo add `.native(cx, geometry::scrollbar_gutter)`. `SettingGroup::render` applies the caller's refinement last on its own root (setting/group.rs:112), and `.py_4()` set by the page touches only the vertical padding, so the right padding survives.

- [ ] **Step 4: Run it, expect PASS**; also run `a_non_overlay_scrollbar_keeps_off_the_content`.

- [ ] **Step 5: Record the Tier U remainder**

Add to the Settings panel's "Not themeable": `("scrollbar", "gpui-component lays the page's scrollbar over the body's right edge (setting/page.rs, SettingPage) and reserves only 1rem; this demo pads each group by the platform's groove width instead (setting/group.rs, SettingGroup). The page body takes no refinement -- Tier U")`. Add an upstream PR candidate to docs/todo.md ("Upstream PR candidates from v0.5.8"): internal scrollbars reserve no gutter where the platform's scrollbars are not overlays — Settings page body, Dialog, Sheet, PopupMenu, MessageScroller, Sidebar, Tree (each attaches `.vertical_scrollbar`/`overflow_y_scrollbar`, scroll/scrollable.rs:19-29).

- [ ] **Step 6: pre-release-check, commit** `fix(showcase): Settings rows keep off an always-visible scrollbar`.

---

## Phase A — structure

### Task 2: The example becomes a module tree

**Files:**
- Create: the tree in "File Structure" above, by moving code out of `examples/showcase-gpui.rs` (which is deleted).
- Modify: `connectors/native-theme-gpui/Cargo.toml` (`path =` on the `[[example]]`), `connectors/native-theme-gpui/src/showcase.rs`, `scripts/check-widget-coverage.py`, `scripts/*.sh` that name the file (grep first).

**Interfaces:**
- Produces: `SHOWCASE_FILES: &[(&str, &str)]` in `src/showcase.rs` — `(relative path, include_str!(…))` for every `.rs` under the example directory; the per-file iteration every gate uses from here on.

This task moves code and changes no behaviour. Its gate is: every existing test passes unchanged in what it asserts, and the coverage script reports the same counts.

- [ ] **Step 1: Write the completeness test first** (in `src/showcase.rs`):

```rust
/// Every `.rs` file of the example is one the gates read.
#[test]
fn the_gates_read_every_showcase_file() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/showcase-gpui");
    let mut on_disk = Vec::new();
    collect_rs(&dir, &dir, &mut on_disk);
    on_disk.sort();
    let mut listed: Vec<String> = SHOWCASE_FILES.iter().map(|(p, _)| (*p).to_string()).collect();
    listed.sort();
    assert!(!on_disk.is_empty(), "no showcase files found under {}", dir.display());
    assert_eq!(listed, on_disk, "SHOWCASE_FILES and the example directory disagree");
}

fn collect_rs(root: &std::path::Path, dir: &std::path::Path, out: &mut Vec<String>) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_rs(root, &path, out);
        } else if path.extension().is_some_and(|e| e == "rs")
            && let Ok(rel) = path.strip_prefix(root)
        {
            out.push(rel.to_string_lossy().replace('\\', "/"));
        }
    }
}
```

- [ ] **Step 2: Move the code.** Split along the existing sections: `Tab`, constants, `CliArgs`, the capture functions and `main` → `main.rs`; `Showcase` struct, `new`, theme switching, `Render for Showcase` → `app.rs`; each `render_*_tab` method → `pages/<page>.rs` as `impl Showcase { … }` (fields become `pub(crate)`); `WidgetInfoPanel` → `inspector.rs`; `widget_tooltip*`, `hsla_to_hex`, `section`, `native_geometry`, `NativeStyled`, `refined`, `native_value`, `native_icon`, `with_gap`, `with_padding`, icon loading → `support.rs`; delegates → `support.rs`; the `#[cfg(test)]` module → `tests.rs`. `info/`, `demo.rs`, `chrome.rs` start empty (a module doc line each).

- [ ] **Step 3: Make the gates read files.** Replace `const SHOWCASE: &str = include_str!(…)` with `SHOWCASE_FILES`; every gate that scanned `SHOWCASE` iterates the files and reports `file:line`. `showcase_demos` (which cuts off the test module) is no longer needed — `tests.rs` is simply not a page file. Update `scripts/check-widget-coverage.py` to read every `.rs` under the directory (its `--showcase-gpui` default becomes the directory).

- [ ] **Step 4: Run everything.** `CARGO_BUILD_JOBS=4 cargo test -p native-theme-gpui --example showcase-gpui`, `cargo test -p native-theme-gpui --lib showcase`, `python3 scripts/check-widget-coverage.py`. Expected: all green; the coverage script prints the same counts as before (gpui 135 / 109 / 26 / 0).

- [ ] **Step 5: Discrimination proof.** Add an empty `pages/scratch.rs`; `the_gates_read_every_showcase_file` fails naming it. Remove it.

- [ ] **Step 6: pre-release-check, commit** `refactor(showcase): the example becomes a module tree the gates read whole`.

---

## Phase B — Widget Info core

### Task 3: The WidgetInfo model

**Files:**
- Create: `examples/showcase-gpui/info/mod.rs`
- Modify: `examples/showcase-gpui/tests.rs`

**Interfaces:**
- Produces:
  - `pub struct ColorClaim { pub role: &'static str, pub field: &'static str, pub value: Hsla, pub cited_at: &'static str }`
  - `pub fn claim(role: &'static str, field: &'static str, value: Hsla, cited_at: &'static str) -> ColorClaim`
  - `pub struct Note { pub what: &'static str, pub text: String }`
  - `pub struct WidgetInfo { pub kind: &'static str, pub variant: Option<String>, pub colors: Vec<ColorClaim>, pub config: Vec<Note>, pub not_themeable: Vec<Note>, pub instance: Vec<Note> }`
  - `impl WidgetInfo { pub fn new(kind) -> Self; pub fn variant(self, impl Into<String>) -> Self; pub fn color(self, ColorClaim) -> Self; pub fn config(self, &'static str, impl Into<String>) -> Self; pub fn not_themeable(self, &'static str, impl Into<String>) -> Self; pub fn instance(self, &'static str, impl Into<String>) -> Self; pub fn title(&self) -> String; pub fn to_text(&self) -> String }`
  - `pub fn hsla_to_hex(Hsla) -> String` (moved from support.rs)

- [ ] **Step 1: Write the failing tests** in `tests.rs`:

```rust
#[test]
fn a_widget_info_titles_itself_by_kind_and_variant() {
    let plain = WidgetInfo::new("Tag");
    assert_eq!(plain.title(), "Tag");
    let with = WidgetInfo::new("Tag").variant("Danger, outline");
    assert_eq!(with.title(), "Tag · Danger, outline");
}

#[test]
fn to_text_prints_the_four_sections_in_order() {
    let red = gpui::hsla(0.0, 1.0, 0.5, 1.0);
    let info = WidgetInfo::new("Tag")
        .variant("Danger")
        .color(claim("bg", "danger", red, "gpui-component/tag.rs:31"))
        .config("border-radius", "radius: 4px")
        .not_themeable("padding", "a rem literal")
        .instance("label", "Danger");
    let text = info.to_text();
    let order: Vec<usize> = ["Theme colors:", "Theme config:", "Not themeable:", "This instance:"]
        .iter()
        .map(|h| text.find(h).unwrap_or(usize::MAX))
        .collect();
    assert!(order.windows(2).all(|w| w[0] < w[1]), "sections out of order:\n{text}");
    assert!(text.starts_with("Tag · Danger\n"), "{text}");
    assert!(text.contains("  bg: danger #ff0000 (gpui-component/tag.rs:31)"), "{text}");
}

#[test]
fn an_empty_section_is_not_printed() {
    let text = WidgetInfo::new("Label").to_text();
    assert_eq!(text, "Label\n");
}
```

- [ ] **Step 2: Run, expect FAIL** (`WidgetInfo` not found).

- [ ] **Step 3: Implement** `info/mod.rs`:

```rust
//! What the inspector shows for one widget instance (spec §3).

use gpui::Hsla;

/// One colour a widget paints, the token it reads, and where upstream reads it.
#[derive(Clone, Debug, PartialEq)]
pub struct ColorClaim {
    pub role: &'static str,
    pub field: &'static str,
    pub value: Hsla,
    /// `<crate>/<path>.rs:<line>`, or `showcase` for a colour the showcase
    /// itself chooses. Read by `every_colour_claim_is_read_at_the_line_it_cites`.
    pub cited_at: &'static str,
}

/// The only way a claim is written, so the gates can find every one.
pub fn claim(role: &'static str, field: &'static str, value: Hsla, cited_at: &'static str) -> ColorClaim {
    ColorClaim { role, field, value, cited_at }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Note {
    pub what: &'static str,
    pub text: String,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct WidgetInfo {
    pub kind: &'static str,
    pub variant: Option<String>,
    pub colors: Vec<ColorClaim>,
    pub config: Vec<Note>,
    pub not_themeable: Vec<Note>,
    pub instance: Vec<Note>,
}

impl WidgetInfo {
    pub fn new(kind: &'static str) -> Self {
        Self { kind, ..Self::default() }
    }
    pub fn variant(mut self, variant: impl Into<String>) -> Self {
        self.variant = Some(variant.into());
        self
    }
    pub fn color(mut self, claim: ColorClaim) -> Self {
        self.colors.push(claim);
        self
    }
    pub fn config(mut self, what: &'static str, text: impl Into<String>) -> Self {
        self.config.push(Note { what, text: text.into() });
        self
    }
    pub fn not_themeable(mut self, what: &'static str, text: impl Into<String>) -> Self {
        self.not_themeable.push(Note { what, text: text.into() });
        self
    }
    pub fn instance(mut self, what: &'static str, text: impl Into<String>) -> Self {
        self.instance.push(Note { what, text: text.into() });
        self
    }
    pub fn title(&self) -> String {
        match &self.variant {
            Some(v) => format!("{} · {v}", self.kind),
            None => self.kind.to_string(),
        }
    }
    /// The plain-text form: the Copy button's payload and what tests read.
    pub fn to_text(&self) -> String {
        let mut s = format!("{}\n", self.title());
        if !self.colors.is_empty() {
            s.push_str("\nTheme colors:\n");
            for c in &self.colors {
                s.push_str(&format!("  {}: {} {} ({})\n", c.role, c.field, hsla_to_hex(c.value), c.cited_at));
            }
        }
        for (heading, notes) in [
            ("Theme config:", &self.config),
            ("Not themeable:", &self.not_themeable),
            ("This instance:", &self.instance),
        ] {
            if !notes.is_empty() {
                s.push_str(&format!("\n{heading}\n"));
                for n in notes {
                    s.push_str(&format!("  {}: {}\n", n.what, n.text));
                }
            }
        }
        s
    }
}
```

Move `hsla_to_hex` here unchanged (support.rs keeps a `pub use`).

- [ ] **Step 4: Run, expect PASS.**
- [ ] **Step 5: pre-release-check, commit** `feat(showcase): a WidgetInfo per widget instance`.

### Task 4: The registry — innermost wins, stale never wins, crossing is not hovering

**Files:**
- Create: `examples/showcase-gpui/info/registry.rs`
- Modify: `info/mod.rs` (`pub mod registry; pub use registry::*;`), `tests.rs`

**Interfaces:**
- Consumes: `WidgetInfo` (Task 3).
- Produces:
  - `pub const INFO_SETTLE: Duration` (250 ms)
  - `pub struct InfoRegistry` with `pub fn new() -> Self`, `pub fn bump_epoch(&mut self)`, `pub fn shown(&self) -> Option<&Rc<WidgetInfo>>`
  - `pub trait InfoExt: IntoElement + Sized { fn info(self, ui: &Entity<InfoRegistry>, id: impl Into<ElementId>, info: WidgetInfo) -> Stateful<Div>; }` implemented for every `IntoElement`
  - `pub fn epoch_marker(ui: &Entity<InfoRegistry>) -> impl IntoElement` — the root's first child

- [ ] **Step 1: Write the failing tests.** A test view with an outer 200×200 target "Outer" at (20,20) and, while `show_inner` is true, an inner 40×40 target "Inner" centred in it; the view's root's first child is `epoch_marker`. Helpers: `hover(cx, point)` = `cx.simulate_mouse_move(point, None, Modifiers::default()); cx.run_until_parked();` and `settle(cx)` = `cx.executor().advance_clock(INFO_SETTLE); cx.run_until_parked();`.

```rust
#[gpui::test] fn the_innermost_hovered_target_wins(cx: &mut TestAppContext)
// hover inner centre, settle → shown title "Inner"; hover (30,30) inside outer only, settle → "Outer".
#[gpui::test] fn leaving_every_target_keeps_what_is_shown(cx: &mut TestAppContext)
// hover inner, settle; hover (500,500) outside both, settle → still "Inner".
#[gpui::test] fn crossing_is_not_hovering(cx: &mut TestAppContext)
// hover outer-only, settle ("Outer"); hover inner, advance INFO_SETTLE / 2; hover outside; settle → still "Outer".
#[gpui::test] fn a_target_no_longer_drawn_never_wins(cx: &mut TestAppContext)
// hover inner, settle ("Inner"); set show_inner = false, notify, draw; hover inner centre again, settle → "Outer".
```

- [ ] **Step 2: Run, expect FAIL** (registry not found).

- [ ] **Step 3: Implement** `info/registry.rs`:

```rust
//! Which widget's info the inspector shows (spec §4).

use std::{collections::HashMap, rc::Rc, time::Duration};

use gpui::{
    canvas, div, App, Bounds, Context, Div, ElementId, Entity, InteractiveElement as _,
    IntoElement, ParentElement as _, Pixels, Stateful, Styled as _,
};

use super::WidgetInfo;

/// How long a new choice must stay the choice before it replaces what is
/// shown. The model states no hover delay (docs/todo.md, "A sidebar width
/// and a tooltip delay are missing from the model"); this is the showcase's.
pub const INFO_SETTLE: Duration = Duration::from_millis(250);

#[derive(Default)]
pub struct InfoRegistry {
    epoch: u64,
    bounds: HashMap<ElementId, (Bounds<Pixels>, u64)>,
    /// Hovered targets in the order they were entered.
    hovered: Vec<(ElementId, Rc<WidgetInfo>)>,
    shown: Option<(ElementId, Rc<WidgetInfo>)>,
    pending: u64,
}

impl InfoRegistry {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn bump_epoch(&mut self) {
        self.epoch = self.epoch.wrapping_add(1);
    }
    pub fn shown(&self) -> Option<&Rc<WidgetInfo>> {
        self.shown.as_ref().map(|(_, info)| info)
    }
    fn record_bounds(&mut self, id: ElementId, bounds: Bounds<Pixels>) {
        self.bounds.insert(id, (bounds, self.epoch));
    }
    /// The hovered target drawn in the latest frame with the smallest area;
    /// the most recently entered wins a tie.
    fn choose(&self) -> Option<(ElementId, Rc<WidgetInfo>)> {
        self.hovered
            .iter()
            .enumerate()
            .filter_map(|(ix, (id, info))| {
                let (b, epoch) = self.bounds.get(id)?;
                (*epoch == self.epoch).then(|| (b.size.width.as_f32() * b.size.height.as_f32(), ix, id, info))
            })
            .min_by(|a, b| a.0.total_cmp(&b.0).then(b.1.cmp(&a.1)))
            .map(|(_, _, id, info)| (id.clone(), info.clone()))
    }
    fn set_hovered(&mut self, id: ElementId, info: Rc<WidgetInfo>, hovered: bool, cx: &mut Context<Self>) {
        self.hovered.retain(|(h, _)| *h != id);
        if hovered {
            self.hovered.push((id, info));
        }
        self.reconsider(cx);
    }
    fn reconsider(&mut self, cx: &mut Context<Self>) {
        let Some((choice, _)) = self.choose() else { return };
        if self.shown.as_ref().is_some_and(|(id, _)| *id == choice) {
            return;
        }
        self.pending = self.pending.wrapping_add(1);
        let ticket = self.pending;
        cx.spawn(async move |this, cx| {
            cx.background_executor().timer(INFO_SETTLE).await;
            this.update(cx, |this, cx| {
                if this.pending != ticket {
                    return;
                }
                if let Some(choice) = this.choose() {
                    this.shown = Some(choice);
                    cx.notify();
                }
            })
            .ok();
        })
        .detach();
    }
}

/// The root's first child: bumps the epoch before any target records its
/// bounds in the same frame (spec §4.2).
pub fn epoch_marker(ui: &Entity<InfoRegistry>) -> impl IntoElement {
    let ui = ui.clone();
    canvas(move |_, _, cx: &mut App| ui.update(cx, |r, _| r.bump_epoch()), |_, _, _, _| {})
        .absolute()
        .size_0()
}

pub trait InfoExt: IntoElement + Sized {
    fn info(self, ui: &Entity<InfoRegistry>, id: impl Into<ElementId>, info: WidgetInfo) -> Stateful<Div> {
        let id: ElementId = id.into();
        let info = Rc::new(info);
        let (on_bounds, on_hover) = (ui.clone(), ui.clone());
        let (bounds_id, hover_id) = (id.clone(), id.clone());
        div()
            .id(id)
            .relative()
            .child(self)
            .child(
                canvas(
                    move |bounds, _, cx: &mut App| on_bounds.update(cx, |r, _| r.record_bounds(bounds_id, bounds)),
                    |_, _, _, _| {},
                )
                .absolute()
                .size_full(),
            )
            .on_hover(move |hovered: &bool, _, cx: &mut App| {
                let (id, info) = (hover_id.clone(), info.clone());
                on_hover.update(cx, |r, cx| r.set_hovered(id, info, *hovered, cx));
            })
    }
}

impl<E: IntoElement> InfoExt for E {}
```

(If `size_0` is not a method on `Canvas` in gpui-pre 0.3.6, use `.w(px(0.)).h(px(0.))`; the behaviour is what the tests pin.)

- [ ] **Step 4: Run, expect PASS.** If `a_target_no_longer_drawn_never_wins` fails, the epoch marker is not prepainted first: check it is the root's first child and the targets are its later siblings' descendants.
- [ ] **Step 5: pre-release-check, commit** `feat(showcase): the innermost hovered instance's info, after it settles`.

### Task 5: Geometry lines are generated

**Files:**
- Modify: `info/mod.rs`, `src/showcase.rs`

**Interfaces:**
- Produces:
  - `pub const GEOMETRY_NOTES: &[(&str, &str)]` — `(builder fn name, what it carries)`
  - `impl WidgetInfo { pub fn geometry(self, builder: &'static str) -> Self }` — appends `("geometry", "geometry::<builder>: <what>")`; an unknown name appends `("geometry", "geometry::<builder>: (no GEOMETRY_NOTES entry)")`, which the Task 5 gate prevents from reaching a commit.
  - `pub fn native_info<W: Styled>(w: W, cx: &App, build: fn(Native<'_>) -> StyleRefinement, name: &'static str, info: &mut WidgetInfo) -> W`

- [ ] **Step 1: Write the gate** in `src/showcase.rs`: `every_geometry_builder_has_a_note` — the set of `pub fn` names in `geometry.rs` (existing `public_fns`) equals the set of first elements of `GEOMETRY_NOTES` (parsed from `info/mod.rs` text: every `("name", "…")` line inside the `GEOMETRY_NOTES` array). And `native_info_names_the_builder_it_applies` — in every showcase file, for each `native_info(` call, the `geometry::X` argument and the `"Y"` argument satisfy X == Y.
- [ ] **Step 2: Run, expect FAIL** (table absent).
- [ ] **Step 3: Write the table.** One entry per `pub fn` of geometry.rs, the text taken from the audited "geometry" config lines the current panels carry (grep `("geometry", "geometry::` in the showcase) — for a builder no panel describes yet, read its doc comment and body in geometry.rs and write what it sets, field by field. `native_info` applies `native_geometry(cx, build)` with `refined` and calls `info.geometry(name)` only when the refinement was applied.
- [ ] **Step 4: Run, expect PASS. Discrimination proof:** delete one entry → `every_geometry_builder_has_a_note` fails naming it; write `native_info(w, cx, geometry::button, "input", …)` → the second gate fails naming the line. Revert both.
- [ ] **Step 5: pre-release-check, commit** `feat(showcase): geometry lines come from the builder applied`.

### Task 6: The citation gates read claims wherever they are written

**Files:**
- Modify: `src/showcase.rs`

**Interfaces:**
- Consumes: `SHOWCASE_FILES`; `claim(` calls (Task 3).
- Produces: `fn claims_in(file: &str) -> Vec<Claim>` covering both `hover_info(` arrays and `claim(` calls, until Task 24 removes the former.

- [ ] **Step 1:** Extend `panel_claims` so a `claim("role", "field", value, "cite")` call yields the same `Claim` a tuple in a `hover_info` array does; extend the prose collection to the string-literal second argument of `.config(`, `.not_themeable(` and `.instance(` calls.
- [ ] **Step 2: Discrimination proof:** write `claim("bg", "primary", t.primary, "gpui-component/tag.rs:30")` (line 30 reads `secondary`) in `info/mod.rs` test scaffolding → `every_colour_claim_is_read_at_the_line_it_cites` fails naming the file and line. Revert.
- [ ] **Step 3: pre-release-check, commit** `test(showcase): the citation gates read claim() calls too`.

---

## Phase C — the chrome

### Task 7: `geometry::toolbar`

**Files:**
- Modify: `connectors/native-theme-gpui/src/geometry.rs` (new builder, `icon_size_toolbar`, tests), `info/mod.rs` (`GEOMETRY_NOTES` entry)

**Interfaces:**
- Produces: `pub fn toolbar(n: Native<'_>) -> StyleRefinement`; `icon_size_toolbar` reading `n.resolved.toolbar.icon_size`.

- [ ] **Step 1: Check the source of every preset's toolbar values** against docs/platform-facts.md §2.13 before relying on them. §2.13 says KDE's toolbar has no bar height ("sizes to content") while `kde-breeze.toml` states `bar_height_px = 40.0`; find where that value came from (git log -S on the preset, the preset's comments, platform-facts). Record the answer in docs/todo.md. This is why the builder uses `min_h`, not `h`.
- [ ] **Step 2: Write the failing tests** in geometry.rs's test module, for every bundled preset and both modes, reading expectations from the resolved theme only:

```rust
#[test]
fn toolbar_carries_the_models_toolbar() {
    for (name, n) in every_preset_native() {
        let t = &n.resolved.toolbar;
        let r = toolbar(n);
        assert_eq!(r.min_size.height, Some(px(t.bar_height).into()), "{name}: bar height");
        assert_eq!(r.gap.width, Some(px(t.item_gap).into()), "{name}: item gap");
        assert_eq!(r.padding.left, Some(px(t.border.padding_horizontal).into()), "{name}: padding");
        assert_eq!(r.background, Some(rgba_to_hsla(t.background_color).into()), "{name}: background");
    }
}

#[test]
fn icon_size_toolbar_reads_the_toolbar() {
    for (name, n) in every_preset_native() {
        assert_eq!(icon_size_toolbar(n), Size::Size(px(n.resolved.toolbar.icon_size)), "{name}");
    }
}
```

(`every_preset_native` is the preset iterator the existing geometry tests use; if its name differs, use that one. Compare field shapes to how the existing tests compare `StyleRefinement` fields — they are the reference for the exact Option/length wrapping.)

- [ ] **Step 3: Run, expect FAIL.**
- [ ] **Step 4: Implement:**

```rust
/// An application-drawn toolbar row (docs/platform-facts.md §2.13):
/// `toolbar.bar_height` as its minimum height -- KDE's toolbar sizes to its
/// content, so a fixed height would be an invention there --,
/// `toolbar.item_gap` between items, `toolbar.border` padding,
/// `toolbar.background_color`, and `toolbar.font` size and weight. No edge:
/// §2.13 states none; an application that wants a rule draws a Separator.
#[must_use]
pub fn toolbar(n: Native<'_>) -> StyleRefinement {
    let t = &n.resolved.toolbar;
    with_text(StyleRefinement::default(), &t.font, n)
        .min_h(px(t.bar_height))
        .gap(px(t.item_gap))
        .px(px(t.border.padding_horizontal))
        .py(px(t.border.padding_vertical))
        .bg(rgba_to_hsla(t.background_color))
}

/// `Icon::with_size` for toolbar icons: `toolbar.icon_size`, which inherits
/// `defaults.icon_sizes.toolbar` where a platform states nothing narrower.
#[must_use]
pub fn icon_size_toolbar(n: Native<'_>) -> Size {
    Size::Size(px(n.resolved.toolbar.icon_size))
}
```

Update the old `icon_size_toolbar` test that asserted `defaults.icon_sizes.toolbar`. Add the `GEOMETRY_NOTES` entry `("toolbar", "toolbar.bar_height (minimum height), item_gap, border.padding_*, background_color, font size and weight")`.

- [ ] **Step 5: Run, expect PASS**; `cargo test -p native-theme-gpui` whole; the connector-parity agent is not needed (iced gains nothing).
- [ ] **Step 6: pre-release-check, commit** `feat(gpui): geometry::toolbar carries the model's toolbar`.

### Task 8: The window's own title bar and menus

**Files:**
- Modify: `main.rs` (window options), `app.rs` (actions, key bindings, render), `chrome.rs` (title bar, menus), `demo.rs`, `info/chrome.rs` (TitleBar and AppMenuBar info), `tests.rs`

**Interfaces:**
- Produces: `actions!(showcase, [Quit, ToggleSidebar, ToggleInspector, OpenCommandPalette, ReloadTheme, OpenPreferences, OpenAbout])`, `#[derive(Clone, PartialEq, Deserialize, JsonSchema, Action)] struct ShowPage(usize)` and `SetColorMode(AppColorMode)` (follow gpui-pre's action macro docs for payload actions); `pub fn title_bar(app: &Showcase, cx) -> impl IntoElement`; `pub fn menus() -> Vec<Menu>` (replaces `showcase_menus`).

- [ ] **Step 1: Failing test** `the_title_bar_is_the_top_of_the_window`: after `open`, the element tagged `CHROME_TITLE_BAR` has `top == 0` and spans the window width; `the_menus_run_actions`: dispatching `ShowPage(3)` makes `active_page == Page::Feedback`.
- [ ] **Step 2: Run, expect FAIL.**
- [ ] **Step 3: Implement.** `WindowOptions { window_bounds: Some(WindowBounds::Windowed(bounds)), window_decorations: Some(WindowDecorations::Client), ..TitleBar::window_options() }`. The title bar: `demo::title_bar(ui, cx, label, app_menu_bar)` builds `TitleBar::new()`, applies `geometry::title_bar` through `native_info`, adds the label and — `cfg!(not(target_os = "macos"))` — the AppMenuBar entity; `on_close_window` dispatches `Quit`. Menus per spec §2.2, given to `cx.set_menus` and to `GlobalState` (guarded, as now). Key bindings per spec §2.2. The TitleBar and AppMenuBar info functions take the audited content of today's TitleBar and AppMenuBar panels (the nested-bar and "this showcase gives both the same menus" notes become instance notes about the real bar).
- [ ] **Step 4: Verify the capture paths** still work: `--screenshot` on the current platform, and `scripts/generate_gpui_screenshots.sh` once (it drives the desktop — run only when the maintainer is not using it, or leave this sub-step to the maintainer and say so in the commit message).
- [ ] **Step 5: Run tests, pre-release-check, commit** `feat(showcase): the TitleBar is the window's title bar, with menus that act`.

### Task 9: The toolbar

**Files:**
- Modify: `chrome.rs`, `demo.rs`, `info/chrome.rs`, `app.rs` (the three Selects' state moves), `pages/inputs.rs` (its preset Combobox sample goes), `tests.rs`

- [ ] **Step 1: Failing tests** `the_toolbar_is_the_models_toolbar` (for kde-breeze and adwaita: the toolbar element's height ≥ `toolbar.bar_height`, its first two children's gap equals `toolbar.item_gap`) and `the_toolbar_switches_the_preset` (select "nord" in the toolbar Combobox → `current_theme_name == "nord"`).
- [ ] **Step 2: Run, expect FAIL.**
- [ ] **Step 3: Implement** per spec §2.3: a row refined by `geometry::toolbar` (via `native_info`), children built by `demo::` helpers — SidebarToggleButton, the preset Combobox (moved from the Inputs page), a ToggleGroup System/Light/Dark wired to `SetColorMode` (replacing the colour-mode Select), the icon-set Select, `Separator::vertical()`, three icon Buttons with Tooltips at `geometry::icon_size_toolbar`. Each carries its info. Remove the left column's selector blocks.
- [ ] **Step 4: Run, pre-release-check, commit** `feat(showcase): a real toolbar, sized by the platform's toolbar`.

### Task 10: Navigation, draggable panels, the inspector

**Files:**
- Modify: `app.rs`, `chrome.rs`, `inspector.rs`, `demo.rs`, `info/chrome.rs`, `tests.rs`; `main.rs` (`Tab` → `Page`, labels and icons)

**Interfaces:**
- Produces: `const NAV_WIDTH: Pixels`, `const INSPECTOR_WIDTH: Pixels` (named layout defaults, spec §1.3); `enum Page` (the ten pages, each with `label()` and `icon()`); `InspectorTab { Widget, Theme }`.

- [ ] **Step 1: Failing tests:**
  - `dragging_a_handle_resizes_its_neighbours`: `simulate_mouse_down` on the handle between content and inspector, move 40px left, up; the inspector's width grows by 40px ± 1px and the content's shrinks by the same.
  - `the_sidebar_navigates`: click the Sidebar item "Charts" → `active_page == Page::Charts` and `PAGE_ROOT` has a size.
  - `the_inspector_shows_the_settled_info`: hover a known instance (any demo helper's instance on the Buttons page carrying a probe), settle → the inspector's `INSPECTOR_TITLE` label reads that info's title.
  - `every_page_lays_out` (the renamed `every_tab_lays_out`).
- [ ] **Step 2: Run, expect FAIL.**
- [ ] **Step 3: Implement** per spec §1.1, §2.4–§2.6: `h_resizable("body")` with three `resizable_panel()`s sized `NAV_WIDTH`, flexible, `INSPECTOR_WIDTH`; the Sidebar with one `SidebarMenuItem` per `Page` (icon, `active`, `on_click` → `ShowPage`); the content panel's scrolled element with `geometry::scrollbar_gutter`; the inspector: a TabBar (built by `demo::tab_bar`, so it reports itself) over the Widget tab — sections rendered from `registry.shown()` with a swatch per claim (the existing `color_swatch`), token, hex, citation in muted text, and a Copy button writing `to_text()` via `cx.write_to_clipboard(ClipboardItem::new_string(…))` — and the Theme tab (today's `render_sidebar` markdown, rebuilt as rows, plus a "Window" section stating what `Root` draws: the WindowBorder facts from today's panel). The inspector observes the registry (`cx.observe`) to re-render. Remove the TabBar navigation and the old left column. The inspector's content module constructs widgets without `.info()` (spec §4.4).
- [ ] **Step 4: Run, pre-release-check, commit** `feat(showcase): Sidebar navigation, draggable panels, a structured inspector`.

### Task 11: The status bar

- [ ] **Step 1: Failing test** `the_status_bar_names_the_hovered_widget`: hover an instance, settle → the status bar's `STATUS_HOVERED` label reads its title; `the_status_bar_is_the_bottom_of_the_window`: its bottom equals the window's height.
- [ ] **Step 2: Run, expect FAIL.**
- [ ] **Step 3: Implement** per spec §2.7 in `chrome.rs`, styled by `geometry::status_bar` via `native_info`; desktop from `native_theme::detect` (the function the showcase already calls to decide the system theme), preset and mode from `Showcase`, font via `defined_size`, text scale and flags from the stored `AccessibilityPreferences`.
- [ ] **Step 4: pre-release-check, commit** `feat(showcase): a status bar that reports the environment and the hover`.

### Task 12: Overlays that do work

- [ ] **Step 1: Failing tests:** `ctrl_k_opens_the_command_palette` (simulate `ctrl-k`, a Dialog layer exists and contains the palette probe); `the_palette_switches_page` (type "Charts", enter → `Page::Charts`); `a_preference_reaches_apply_accessibility` (toggle "Reduce motion" in the Preferences sheet → `cx.reduce_motion()` is true); `a_theme_error_is_an_alert` (apply a nonexistent preset name → an element tagged `CONTENT_ALERT` exists).
- [ ] **Step 2: Run, expect FAIL.**
- [ ] **Step 3: Implement** per spec §2.8. Reuse the Command demo's construction for the palette (Overlays page) and the Settings demo's for Preferences; each gets `.native(cx, geometry::scrollbar_gutter)` on its SettingGroups (Task 1). Replace `error_message`'s banner with `demo::alert`.
- [ ] **Step 4: pre-release-check, commit** `feat(showcase): command palette, preferences, about, and errors as an Alert`.

### Task 13: The chrome samples leave the pages

- [ ] **Step 1:** Delete the samples of spec §7.1 from `pages/layout.rs`, `pages/overlays.rs`, `pages/inputs.rs`, with their panels and their `RESIZABLE_GROUPS` table and its test `resizable_groups_have_room_to_drag` (superseded by Task 10's drag test).
- [ ] **Step 2:** Run `python3 scripts/check-widget-coverage.py`: every widget still shown (the chrome constructs it). Update the `WindowBorder` exception's reason in `docs/showcase-exceptions.toml` to "drawn by `Root` around the window (client-side decorations since this change); its facts are in the inspector's Theme tab".
- [ ] **Step 3: pre-release-check, commit** `refactor(showcase): chrome is demonstrated by being the chrome`.

---

## Phase D — every instance reports itself

Tasks 14–23 have one shape. For page **P**:

1. Create `info/P.rs`. For every panel on the page, write `pub fn <kind>(t: &Theme, …) -> WidgetInfo` whose parameters are exactly what distinguishes the page's instances of that kind (spec §3.4).
2. **Move, do not rewrite, the audited content.** Every `("role", "field", t.x, "cite")` tuple becomes `.color(claim("role", "field", t.x, "cite"))`; every config tuple a `.config(…)` (geometry lines become `native_info` in the helper instead); every "Not themeable" entry that is a resolvability statement stays `.not_themeable(…)`; every API or demo fact becomes `.instance(…)`.
3. **Gallery panels split by variant.** A claim that holds for one variant goes into that variant's `match` arm; a claim that holds for all goes outside the match. Where the old panel had one claim for several variants, re-derive each variant's from upstream: open the widget's source at the variant's arm, cite the line that reads the token, and run the colour gate. A variant whose claim you cannot verify gets no claim and a finding in your report.
4. Create `demo::<kind>(ui, cx, id, …)` helpers building the widget and calling `.info(ui, id, info::P::<kind>(…))`; builders through `native_info`.
5. Rewrite `pages/P.rs` to call helpers only; delete its `hover_info` calls and `tt-` ids.
6. Run the page's windowed test (`every_page_lays_out`), the colour and prose gates, the coverage script; pre-release-check; commit `feat(showcase): <Page> reports every instance`.

**Worked example — the Tags gallery (Feedback page):**

```rust
// demo.rs -- the showcase's own list of the variants it builds, so the match
// below is exhaustive and the compiler rejects a variant without an arm.
#[derive(Clone, Copy)]
pub enum TagKind { Primary, Secondary, Danger, Success, Warning, Info }

// info/feedback.rs
pub fn tag(t: &Theme, kind: TagKind, outline: bool) -> WidgetInfo {
    // (name, fill, text, edge, text when outlined). Every line below was read
    // by the eighth audit pass; the outlined text of Secondary, Success,
    // Warning and Info was not, and the page never outlines them, so it is
    // `None` -- read `tag.rs`'s `fg` arms before ever passing `outline: true`.
    let (name, fill, text, edge, outlined_text) = match kind {
        TagKind::Primary => ("Primary",
            claim("bg", "primary", t.primary, "gpui-component/tag.rs:29"),
            claim("text", "primary_foreground", t.primary_foreground, "gpui-component/tag.rs:71"),
            claim("border", "primary", t.primary, "gpui-component/tag.rs:48"),
            Some(claim("text", "primary", t.primary, "gpui-component/tag.rs:69"))),
        TagKind::Secondary => ("Secondary",
            claim("bg", "secondary", t.secondary, "gpui-component/tag.rs:30"),
            claim("text", "secondary_foreground", t.secondary_foreground, "gpui-component/tag.rs:78"),
            claim("border", "border", t.border, "gpui-component/tag.rs:49"),
            None),
        TagKind::Danger => ("Danger",
            claim("bg", "danger", t.danger, "gpui-component/tag.rs:31"),
            claim("text", "danger_foreground", t.danger_foreground, "gpui-component/tag.rs:85"),
            claim("border", "danger", t.danger, "gpui-component/tag.rs:50"),
            Some(claim("text", "danger", t.danger, "gpui-component/tag.rs:83"))),
        TagKind::Success => ("Success",
            claim("bg", "success", t.success, "gpui-component/tag.rs:32"),
            claim("text", "success_foreground", t.success_foreground, "gpui-component/tag.rs:92"),
            claim("border", "success", t.success, "gpui-component/tag.rs:51"),
            None),
        TagKind::Warning => ("Warning",
            claim("bg", "warning", t.warning, "gpui-component/tag.rs:33"),
            claim("text", "warning_foreground", t.warning_foreground, "gpui-component/tag.rs:99"),
            claim("border", "warning", t.warning, "gpui-component/tag.rs:52"),
            None),
        TagKind::Info => ("Info",
            claim("bg", "info", t.info, "gpui-component/tag.rs:34"),
            claim("text", "info_foreground", t.info_foreground, "gpui-component/tag.rs:106"),
            claim("border", "info", t.info, "gpui-component/tag.rs:53"),
            None),
    };
    let info = WidgetInfo::new("Tag")
        .variant(if outline { format!("{name}, outline") } else { name.to_string() });
    let info = match (outline, outlined_text) {
        (true, Some(text)) => info
            .color(text)
            .color(edge)
            .not_themeable("outlined fill", "transparent_white(), a literal: an outlined Tag drops its variant background (tag.rs, Tag::render)"),
        _ => info.color(fill).color(text).color(edge),
    };
    info.config("border-radius", format!("radius: {}px", t.radius.as_f32()))
        .not_themeable("padding", "px_2p5 / py_1 at the default Size -- rems, so the platform's font -- and settable: Tag applies the caller's refinement last (tag.rs, Tag::render). native-theme states no tag widget. Our gap")
        .instance("label", name)
}
```

The `(true, None)` case falls to the unoutlined arm on purpose and is unreachable from the page; if a later page outlines another variant, its outlined text is read from `tag.rs` first and the arm gains `Some(…)`.

| Task | Page | Galleries to split (spec: rationale §1.1) |
|---|---|---|
| 14 | Buttons | Disabled Buttons, Buttons with Icons, Button Sizes, ButtonGroup, Toggle/ToggleGroup, Clipboard |
| 15 | Inputs | Checkbox (3 states), Radio group, Switch (2), Rating (2), Input (2 fields) |
| 16 | Data | Avatar / AvatarGroup, Pagination (2), Table, List, Tree |
| 17 | Feedback | Tags (8), Badge (count, count, dot), Bubble (7), Marker (5), Notification (4), Attachment (3), Progress, ProgressCircle, Spinner, Skeleton, ShimmerText |
| 18 | Typography | Label (3), Font Sizes, Headings, Link (2), Kbd |
| 19 | Layout | GroupBox (3), Separator (3), Stepper (2), Accordion, Collapsible, Carousel, Form, Settings |
| 20 | Overlays | Dialog, AlertDialog, Sheet, Popover, HoverCard, ContextMenu, PopupMenu |
| 21 | Charts | the five charts |
| 22 | Icons | Icon, the native icon grid, animated icons |
| 23 | Theme Map | the swatch table |

*As built:* Bubble (7) and Attachment (3) are on the Data page, not Feedback, so Task 16 migrated them with the Data page and Task 17 found them done.

**Task 22 also carries the paint-level spike** (rationale §3.6), time-boxed to one working session: determine whether `VisualTestContext` exposes the painted scene (search gpui-pre `test` and `scene` modules for a public accessor of the last frame's primitives). If it does, write one test that reads the fill painted inside a Tag instance's bounds and compares it with its `bg` claim's value; if it does not, record the finding in docs/todo.md under "The colour gate cannot tell whose line it is". Either outcome closes the spike.

---

## Phase E — gates and finish

### Task 24: The per-instance gates replace the block gates

**Files:**
- Modify: `src/showcase.rs`, `support.rs` (delete `hover_info`, `widget_tooltip*`), `the_showcase_hardcodes_no_style_values`

- [ ] **Step 1: Write** `every_widget_reports_itself` (spec §10.1): collect the names imported from `gpui_component` in each page and chrome file (the coverage script's `toolkit_roots` logic, reimplemented in Rust over `use` lines); fail on any `Name::new(`, `Name::<constructor>(` outside `demo.rs` and `chrome.rs`; fail on any `pub fn` in `demo.rs`/`chrome.rs` that constructs a widget and has no `.info(`; exempt `inspector.rs` by name.
- [ ] **Step 2: Discrimination proofs:** a `Tag::primary()` in `pages/feedback.rs` fails naming file and line; a `demo::` helper without `.info(` fails naming it. Revert.
- [ ] **Step 3: Delete** `every_demo_id_is_a_tt_id`, `every_demo_block_has_a_widget_info_panel`, `every_demo_names_the_builders_it_applies`, the `BLOCK_ID`/`PANEL_CALL`/`INDIRECT_BLOCK_ID` machinery and the `hover_info` array parsing from `claims_in`.
- [ ] **Step 4: Extend** `the_showcase_hardcodes_no_style_values` to `text_size(px(` literals; fix every hit in chrome and pages with a text-scale role or a rem (`text_sm`, `text_xs`) — run the gate first to list them; today they are `section()` (:714), the error banner (:7394), the inspector (:1463) and a few sidebar labels.
- [ ] **Step 5: pre-release-check, commit** `test(showcase): every widget reports itself; the block gates go`.

### Task 25: Documentation and archive

- [ ] **Step 1:** CHANGELOG `[Unreleased]`: `### Added` — `geometry::toolbar`; `### Changed` — the showcase is an application (chrome, per-instance Widget Info, This instance section), `icon_size_toolbar` reads `toolbar.icon_size`; `### Fixed` — Settings rows under the scrollbar.
- [ ] **Step 2:** docs/todo.md: close "The model's toolbar is read by nothing", "'Not themeable' has become a bucket"; update "The colour gate cannot tell whose line it is" with the spike's outcome; add "The iced showcase: per-instance Widget Info" as a follow-up; record Task 7 Step 1's finding.
- [ ] **Step 3:** Move the three `todo_v0.5.9_showcase-app-*.md` documents to `docs/archive/` and fix their relative links.
- [ ] **Step 4: pre-release-check, commit** `docs: the showcase-app plan is implemented and archived`.

---

## Self-review

- **Spec coverage:** §1 → Tasks 8, 10; §2.1–2.2 → 8; §2.3 → 9; §2.4–2.6 → 10; §2.7 → 11; §2.8 → 12; §3 → 3, 5, 14–23; §4 → 4, 10; §5 → 5, 14–23; §6 → 3, 14–23; §7 → 13; §8 → 1, 10, 12; §9 → 7; §10 → 2, 5, 6, 10, 24; §11 → 25.
- **Open by design, not placeholders:** the per-variant claims of Tasks 14–23 are derived during those tasks from upstream source under the colour gate; the plan names each gallery and the method, and writes out only claims already verified (the Tag arms above). Task 7 Step 1 and Task 22's spike are investigations whose outcomes are recorded, not assumed.
