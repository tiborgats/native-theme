# v0.6.1 — gpui connector on GPUI Kit 0.7.1: Specification

**Status:** design, not implemented. Reasons for every decision:
[`todo_v0.6.1_gpui-kit-0.7.1-rationale.md`](todo_v0.6.1_gpui-kit-0.7.1-rationale.md)
(L1–L15). Tasks and gates:
[`todo_v0.6.1_gpui-kit-0.7.1-plan.md`](todo_v0.6.1_gpui-kit-0.7.1-plan.md).

Abbreviations: `GC` = gpui-component 0.7.1 `src/`, `GB` = gpui-base 0.7.1
`src/`, `GP` = gpui-pre 0.3.8 `src/`, `CN` = `connectors/native-theme-gpui/`,
`SC` = `CN/examples/showcase-gpui/`. Upstream line numbers are of 0.7.1 /
0.3.8; connector line numbers are of `942302d6` (tag `v0.6.0`) — locate by
content, they drift as tasks land.

---

## 0 -- Scope

### 0.1 What this delivers

1. native-theme-gpui builds on gpui-component / gpui-base / gpui-kit 0.7.1 and
   gpui-pre 0.3.8, and its icon tables no longer break downstream builds when
   upstream adds an icon (L1–L3).
2. The platform's font reaches a Button's label, a Toggle's text and DataTable
   cells again, through three new `geometry` items (L4–L6).
3. The showcase shows ColorSelect and SpeechWaveform, excepts SpeechButton,
   and its Basic page lays out exactly as it did on 0.7.0 (L7, L8, L12).
4. Every upstream citation and claim is true for 0.7.1 (L9–L11).
5. Version 0.6.1, CHANGELOG, ROADMAP, todo items, compatibility stamp (L13).

### 0.2 Constraints

- No `unwrap` / `expect` / `panic!` / `unreachable!` / slice indexing /
  `unsafe` / unchecked integer arithmetic outside test code, showcase included.
- No invented values: every colour and size is a resolved-theme field, an
  upstream literal cited at its line, or (§7.2 only) demonstration data that
  no style setter receives.
- Never mix icon sets: an unmapped `IconName` returns `None`.
- Additive public API only: `geometry::button_label`, `geometry::toggle`,
  `geometry::data_table_size`. Nothing removed or re-typed.
- No source file cites these three documents by path: they move to
  `docs/archive/` in the last task, after the compatibility stamp has hashed
  the connector's sources.

### 0.3 Out of scope

Rationale L15: a checker for library prose citations, `list.header_font` on a
DataTable header, a compile-time guard for the showcase's `GPUI_ICONS`,
`PlotAppearScope` in the showcase, a `speech` feature, new `IconRole`s, filing
upstream PRs.

---

## 1 -- Facts this specification rests on

| # | Fact | Source |
|---|---|---|
| F1 | 0.7.1 crates pin `gpui-pre = "=0.3.8"` | upstream workspace `Cargo.toml:69` |
| F2 | `IconName` gains `Mic`, `Square` (104 → 106) | gpui-kit-assets 0.7.1 `default-icons.txt`; `GC/icon.rs:13-19` |
| F3 | Nothing else breaks the build; two tests fail | probe, rationale §1.2–1.3 |
| F4 | Medium `button_text_size` is `text_sm` | `GC/sizing.rs:337-343` |
| F5 | The Button label size is on an inner element; children share it, and a child that states a size wins | `GC/button/button.rs:749, :763-772`; `SC/demo.rs` `tool_label`, `listed_label` |
| F6 | Toggle sets `text_sm` before the caller's refinement | `GC/button/toggle.rs:174-179, :214` |
| F7 | `Size::Size` table cells set no text size; the row height is verbatim | `GC/sizing.rs:57-64, :323-333` |
| F8 | `list.row_height` is `Option<f32>` when resolved (`soft_option`) | `native-theme/src/model/widgets/mod.rs:604-606`; `CN/src/geometry.rs:42-43` |
| F9 | ColorSelect's refinement lands on the wrapper, not the field | `GC/color_picker.rs:488, :762-813` |
| F10 | Speech widgets and traits exist without the `speech` feature | `GC/lib.rs:80`, `GC/speech/mod.rs:13-36` |
| F11 | `SpeechState::start` needs a recognizer *and* an input; `stop` drops the capture | `GC/speech/state.rs:190-193, :244` |
| F12 | Charts draw in over 1000 ms; reduced motion skips it | `GC/theme/mod.rs:69, :77-79, :125`; `GB/plot/appear.rs:245`; `GB/motion/presence.rs:61` |
| F13 | MSRV 1.95.0 builds the 0.7.1 closure; 1.94 cannot (`cold_path`) | measured 2026-10-06; `GP/profiler.rs:473` |
| F14 | The README's Required table and Quick-start lines are test-checked against `Cargo.toml` | `CN/src/compat.rs` (`the_readme_states_the_manifest_floors`, `the_quick_start_states_the_manifest_floors`) |
| F15 | A wildcard arm under `#[allow(unreachable_patterns)]` is warning-free; a macro-generated match without one is exhaustive | toy crate under `#![deny(warnings)]`, rationale §3 |

---

## 2 -- Manifest, lockfile, README floors (L1)

### 2.1 `CN/Cargo.toml`

Replace, keeping every other line:

```toml
# NOT inherited: the gpui-pre closure declares a higher floor than the
# workspace. Set to the lowest toolchain that compiles the library; measured
# 2026-10-06 on the 0.7.1 closure: 1.95.0 builds, 1.94.0 fails on
# std::hint::cold_path in gpui-pre 0.3.8 (src/profiler.rs:473).
rust-version = "1.95.0"
```

```toml
# GPUI under the package name gpui-component uses, so both edges unify.
# gpui-component 0.7.1 depends on `gpui-pre = "=0.3.8"` (an exact pin), so
# 0.3.8 is the only version that resolves with it. gpui-component does not
# re-export gpui, so the connector names it. Caret: an exact pin in a library
# would conflict with any consumer whose other dependencies want a later
# snapshot.
gpui = { package = "gpui-pre", version = "0.3.8" }
# A hard floor: 0.7.1 added IconName::Mic and IconName::Square, which the
# icon tables name.
gpui-component = "0.7.1"
# For gpui_base::Theme, ScrollbarTheme, ResizableTheme and
# apply_system_reduce_motion (not re-exported), and the headless primitives
# the `widgets` feature builds on (Tabs, the slider and progress parts).
gpui-base = "0.7.1"
```

In the `image` comment: "Pinned to the version gpui-pre 0.3.7 resolves" →
"0.3.8" (still `0.25.10`: the probe lockfile resolves it).
Dev-dependencies: `gpui = { package = "gpui-pre", version = "0.3.8", features =
["test-support"] }`, `gpui-kit = { version = "0.7.1", features =
["tree-sitter-rust"] }` — **no** `speech` feature (L8).

### 2.2 Lockfile

```bash
cargo update -p gpui-component -p gpui-base -p gpui-kit -p gpui-pre
```

Expected (probe): gpui-base, gpui-component, gpui-component-macros, gpui-kit,
gpui-kit-assets 0.7.0 → 0.7.1; every `gpui-pre*` 0.3.7 → 0.3.8; `notify` 7,
`inotify` 0.10, `borsh`, `filetime`, `x11-clipboard` 0.9 removed. Nothing
outside the gpui closure changes version.

### 2.3 MSRV

`rust-version` stays `1.95.0` (F13). The gate re-runs it:
`CARGO_BUILD_JOBS=4 cargo +1.95.0 check -p native-theme-gpui --lib --locked`.

### 2.4 README floors — same commit as the manifest (F14)

`CN/README.md`:

- the **Required** table (`:71-74`): `gpui-component` 0.7.1, `gpui-base` 0.7.1,
  `gpui-pre` 0.3.8, `gpui-kit` 0.7.1 (`rust-version` unchanged);
- Quick start (`:100`): `gpui-kit = "0.7.1"`;
- "GPUI as `gpui-pre`" (`:512`): `version = "0.3.8"`;
- the paragraph under the table (`:77-80`) gains one sentence: "gpui-component
  0.7.1 did it again by adding two `IconName` variants, which the published
  0.6.0 matched exhaustively; since 0.6.1 an `IconName` this crate does not
  know maps to no icon instead of stopping the build."

The **Verified** line (`:87`) is not touched by hand (§12.5).

---

## 3 -- Icons (L2, L3)

### 3.1 Bundled glyphs

```bash
scripts/update_icons.sh add lucide mic
scripts/update_icons.sh add lucide square
scripts/update_icons.sh add material mic
scripts/update_icons.sh add material square
```

Each prints `native-theme/icons/<set>/<name>.svg <- <url>`. `cmp` the two
Lucide files against gpui-kit-assets 0.7.1's `assets/icons/{mic,square}.svg`
(registry source): identical. `SOURCES.toml` is not edited (both `[[set]]`
rules cover the new files); `native-theme/tests/icon_sources.rs` must still
pass.

### 3.2 The three tables (`CN/src/icons.rs`)

Insert in each table's order (`Mic` after `Menu`; `Square` before
`SquareTerminal`):

```rust
// lucide_name_for_gpui_icon
        IconName::Mic => "mic",
        IconName::Square => "square",
// material_name_for_gpui_icon
        IconName::Mic => "mic",       // exact
        IconName::Square => "square", // exact
// freedesktop_name_for_gpui_icon (the "standard names, all DEs" group)
        IconName::Mic => "audio-input-microphone", // exact (Icon Naming Spec, Devices)
        IconName::Square => "media-playback-stop", // close: upstream draws it only as SpeechButton's stop glyph (speech/button.rs:96-100); Breeze and Adwaita fill the square
```

and, as the **last** arm of each of the three `match icon { … }` blocks:

```rust
        // An `IconName` newer than this table (upstream adds icons in patch
        // releases) draws nothing from this set rather than another set's
        // glyph. The test module's `variant_name` is the match without a
        // wildcard that makes the dependency canary report the newcomer.
        #[allow(unreachable_patterns)]
        _ => return None,
```

Doc comments: the three "Covers all 104 gpui-component 0.7.0 `IconName`
variants" (`:155, :277, :424`) become "Covers all 106 gpui-component 0.7.1
`IconName` variants; a newer variant returns `None`". The freedesktop table's
"today every variant has one" stays true.

### 3.3 Tests (`CN/src/icons.rs`, test module)

Replace the `ALL_ICON_NAMES` constant (`:1406` to its closing `];`) with:

```rust
    /// Every `IconName`, once. The macro writes the list and, from the same
    /// identifiers, a `match` with no wildcard arm, so a variant upstream adds
    /// stops this test module compiling — which is how the dependency canary
    /// reports it — while the library's tables, which end in a wildcard, keep
    /// building for applications. A variant listed twice is an unreachable
    /// pattern, which `clippy -D warnings` rejects.
    macro_rules! all_icon_names {
        ($($variant:ident),* $(,)?) => {
            pub(super) const ALL_ICON_NAMES: &[IconName] = &[$(IconName::$variant),*];

            /// The variant's identifier, for failure messages (`IconName`
            /// has no `Debug`).
            pub(super) fn variant_name(icon: &IconName) -> &'static str {
                match icon {
                    $(IconName::$variant => stringify!($variant),)*
                }
            }
        };
    }
    all_icon_names!(
        ALargeSmall,
        ArrowDown,
        // … every entry of today's list as a bare identifier, in today's
        // order, with `Mic` after `Menu` and `Square` after `SortDescending` …
    );
```

(The comment inside the invocation is this document's ellipsis: the
implementation lists all 106 identifiers.)

- `all_icon_names_count_matches_gpui_component` (`:1749-1762`) stays, with
  `106` and this comment in place of the "Issue 41" one: "The count the
  README, the table docs and the showcase's gallery state. `variant_name`
  already stops this module compiling when upstream adds a variant; this
  fails when the list changes, as the reminder to update those counts."
- `every_none_is_an_allowed_gap` (`:1528-1548`): its two messages name the
  icon — `"{} has no Lucide mapping and is not in LUCIDE_NONE_ALLOWED",
  variant_name(icon)` and the Material twin.
- New, in `mod tests`:

```rust
    #[test]
    fn mic_and_square_map_to_the_sets_own_glyphs() {
        assert_eq!(lucide_name_for_gpui_icon(IconName::Mic), Some("mic"));
        assert_eq!(lucide_name_for_gpui_icon(IconName::Square), Some("square"));
        assert_eq!(material_name_for_gpui_icon(IconName::Mic), Some("mic"));
        assert_eq!(material_name_for_gpui_icon(IconName::Square), Some("square"));
    }
```

- New, in `mod freedesktop_mapping_tests` (`:2500`), beside `eye_differs_by_de`:

```rust
    #[test]
    fn mic_and_square_take_standard_names_on_every_desktop() {
        for de in [LinuxDesktop::Kde, LinuxDesktop::Gnome] {
            assert_eq!(
                freedesktop_name_for_gpui_icon(IconName::Mic, de),
                Some("audio-input-microphone"),
            );
            assert_eq!(
                freedesktop_name_for_gpui_icon(IconName::Square, de),
                Some("media-playback-stop"),
            );
        }
    }
```

  (If `LinuxDesktop` is not `Copy`, write the two desktops out.)

### 3.4 Showcase gallery

`SC/support.rs:540-541`: "The 106 gpui-component 0.7.1 IconName variants shown
in the gallery."; `GPUI_ICONS` gains `("Mic", IconName::Mic)` and
`("Square", IconName::Square)` in order. `role_for_gpui_icon` is unchanged (no
`IconRole` for either). `SC/info/icons.rs:49`: "104" → "106".

### 3.5 README icons paragraph

`CN/README.md:47-57` (the **Icons** bullet), `:537`, `:553`: "104" → "106".
The audit sentence at `:55-57` continues: "0.7.1 added `mic` and `square`.
Since 0.6.1 the tables end in a wildcard: an `IconName` newer than this crate
maps to no icon, and the test module's exhaustive list is what reports it."

---

## 4 -- The Button label (L4)

### 4.1 `geometry::button_label` (`CN/src/geometry.rs`, after `button`)

```rust
/// The text of a `Button`'s label, passed as the button's child: `button.font`'s
/// size and weight, and the platform's line height.
///
/// gpui-component sets the label's size from the `Size` enum on the content
/// element (`button_text_size`, `src/sizing.rs:337-343`, applied at
/// `src/button/button.rs:749`) after the button's own refinement (`:733`), so
/// a size given to [`button`] would be shadowed; since 0.7.1 that size is
/// `text_sm` at `Size::Medium`, 0.875 of the platform font. The content
/// element's children are the one public route under it (`:772`): a label
/// passed as a child, built like upstream's own label element (`:763-771`)
/// and refined with this. The name upstream derives from `.label()`
/// (`:737-740`) is then given explicitly:
///
/// ```ignore
/// use gpui::{ParentElement, Styled, div};
/// use gpui_component::{StyledExt, button::Button};
/// use native_theme_gpui::geometry;
///
/// Button::new("save")
///     .refine_style(&geometry::button(n))
///     .child(
///         div().min_w_0().whitespace_nowrap().text_ellipsis()
///             .refine_style(&geometry::button_label(n))
///             .child("Save"),
///     )
///     .accessibility_label("Save");
/// ```
///
/// It carries the line height [`button`] gives the button, so it is complete
/// on a Button that does not take [`button`] (a `ButtonGroup`'s child). No
/// colour: the label takes the button's state colours (idle, hovered,
/// pressed, disabled, selected) from the root, as `.label()`'s does. Labels of
/// Buttons that gpui-component's own widgets build are out of reach.
#[must_use]
pub fn button_label(n: Native<'_>) -> StyleRefinement {
    let line = StyleRefinement::default().line_height(relative(n.resolved.defaults.line_height));
    with_text(line, &n.resolved.button.font, n)
}
```

`button`'s comment (`:186-198`) keeps its reasoning and gains: "Since 0.7.1
the size it would shadow is `text_sm` at Medium (`sizing.rs:337-343`);
[`button_label`] gives a child label the platform's size." Its
`sizing.rs:327-333` citation moves with that sentence.

### 4.2 Showcase: one label helper

`SC/demo.rs` has two child-label helpers today: `listed_label` + `labelled`
(`:7882-7916`), which record a listed label's bounds, and `tool_label`
(`:1165-1192`), which draws a chrome button's text in `button.font` by hand.

- `labelled` takes `cx` and absorbs `listed_label`:

```rust
/// `button` labelled `label`. Where a native theme is installed, or the label
/// is an element of docs/showcase-elements.toml, the label is the Button's
/// child, in the box upstream puts its own `label` in (button/button.rs,
/// `RenderOnce for Button`: `min_w_0`, `whitespace_nowrap`, `text_ellipsis`):
/// in `button.font` through `geometry::button_label`, because upstream sizes
/// a `.label()` from the `Size` enum on an element no style reaches, and
/// recording its bounds where it is listed. The Button is named by the label
/// as `Button::label` would name it. Otherwise `Button::label`.
fn labelled(
    ui: &Entity<InfoRegistry>,
    cx: &App,
    button: Button,
    id: &str,
    label: impl Into<SharedString>,
) -> Button {
    let label = label.into();
    let text = native_geometry(cx, geometry::button_label);
    let listed = elements::label_of(id);
    if text.is_none() && listed.is_none() {
        return button.label(label);
    }
    let boxed = div().relative().min_w_0().whitespace_nowrap().text_ellipsis();
    button.accessibility_label(label.clone()).child(
        refined(boxed, text.as_ref())
            .child(label)
            .when_some(listed, |this, listed| this.child(elements::record(ui, listed))),
    )
}
```

  (Adapt types to the compiler; the behaviour is the contract.)
- `tool_label` goes. It is `labelled` without the listed-label record and
  without upstream's `min_w_0` / `text_ellipsis` box, with the same three
  values written by hand (`text_size_of`, `line_height_of`, the weight;
  `line_height_of` is the scaled size × `defaults.line_height`, which is what
  `relative(defaults.line_height)` resolves to). Its three callers (`:1152`,
  `:1280`, `:1383`, each with `ui` in scope) call `labelled`; the comment that
  names it (`SC/tests.rs:3724`) names `demo::labelled`. `text_size_of` and
  `line_height_of` stay (four other callers).
- Every `gpui_component::button::Button` the showcase gives a text label goes
  through `labelled`: the sites that do today (`:2937`, `:5307`, `:8011`), the
  tool-button fallbacks (`tool_label`'s three, and `:8051`, which calls
  `button.label(name)` directly), and the direct `.label(…)` sites on Buttons —
  `:2995` (the `ButtonGroup`'s children), `:3011` (the `DropdownButton`'s
  button), `:4707`, `:5181`, `:5275`, `:5346`, `:6453` (the collapse toggle),
  `:6858`, `:6923`, `:7059`, `:7119`, `:7192`. **Not** `sized_button`
  (`:2962-2969`, the "Button Sizes" panel shows upstream's scale), and not the
  `.label(…)` of other widgets (`Tab`, `Toggle`, `Checkbox`, `Radio`, `Switch`,
  `InputGroupButton`, `CommandGroup`, `Separator`, form fields, …). After the
  change, `grep -n '\.label(' SC/*.rs SC/pages/*.rs` lists no `Button` but
  `sized_button`'s and `labelled`'s own fallback.
- `SC/support.rs:184-188` (the list of exceptions to "every widget takes its
  builder") gains: "`ButtonGroup` and `DropdownButton` children still take
  `geometry::button_label` on their label; it is their frame the group
  manages."

### 4.3 Widget Info

- `SC/info/leaves.rs:354`: "gpui: `geometry::button_label` gives the label,
  the Button's child, `button.font`'s size and weight (demo::labelled); a
  `.label()` would be upstream's `text_sm` at Medium (sizing.rs:337-343)".
- `SC/info/buttons.rs:512` (font-weight) and `:535` (label size), in
  `info::buttons::button`: where `styled` — `config("label size",
  "button.font.size through geometry::button_label on the label, which the
  showcase passes as the Button's child: upstream sizes a .label() from the
  Size enum on the content element (sizing.rs, button_text_size;
  button/button.rs, Button::render), which a style on the Button does not
  reach and a child's own size overrules")`; where not — the existing
  `not_themeable("label size", …)` with its ladder corrected to 0.7.1
  (`text_xs`, `text_sm` for Small and Medium, `text_base` for Large and
  `Size::Size`).
- The "Button Sizes" panel's notes (`:580-581`) are unchanged and stay true.

### 4.4 Gates

- Unit (`CN/src/geometry.rs`): §11.
- Seam (`CN/tests/seams.rs`): under every native preset at factors 1.0 and
  1.5, with `Build` functions `fn(Option<&StyleRefinement>, …)` in the file's
  form — `u` = the width of the text probe as a Medium Button's child with no
  style, `s` = the same with `geometry::button_label` on the probe, `e` = the
  probe with `geometry::button_label` in a bare `div()`;
  `assert_seam(u, s, e, …)` (which requires `u ≠ e`).
- `--dump-layout` of the Basic page under the six pairings equals the 0.7.0
  dumps element for element (`target/layout-071/before/`, taken from `main`).
- `a_page_sample_icon_the_set_lacks_is_absent` passes, unedited (L12).

---

## 5 -- Toggle (L5)

### 5.1 Builder (`CN/src/geometry.rs`, after `button_label`)

```rust
/// `Toggle` (`src/button/toggle.rs:174-179` → `:214`): [`button_label`]'s
/// text — `button.font`'s size and weight and the platform's line height.
///
/// gpui-component sets `text_sm` on a `Size::Medium` Toggle's root before
/// the caller's refinement (0.875 of the platform font since 0.7.1), so this
/// size wins. Only the text: a `Toggle` is `Ghost` by default (`:15-16`), a
/// flat button, and the model states no flat-button frame, so upstream's own
/// stands. The platform's segmented control is the segmented `TabBar`, not a
/// `ToggleGroup`.
#[must_use]
pub fn toggle(n: Native<'_>) -> StyleRefinement {
    button_label(n)
}
```

### 5.2 Showcase

Each `Toggle` the showcase builds takes it the way its neighbours take their
builders (`native_info` / `.native`): `demo::toggle` (`SC/demo.rs:3026-3052`)
and the `ToggleGroup`'s children (`:3055-3065`). `toggle_notes`
(`SC/info/buttons.rs:653-659`): the "size" note becomes "min_w_8 / h_8 at the
default Size, upstream's own: the model states no flat-button frame
(button/toggle.rs, Toggle::render)", and a `config("text", "button.font's size
and weight through geometry::toggle, which lands after the text_sm a Medium
Toggle sets on itself (button/toggle.rs, Toggle::render)")` joins it where a
native theme is installed — `toggle_notes` and its two callers
(`info::buttons::toggle`, `toggle_group`) take the `styled: bool`
`info::buttons::button` takes, computed as `demo::button` computes it
(`SC/demo.rs:2882`). The sentence about `segmented_control` goes.

### 5.3 Gates

Unit: §11. Seam: under every native preset, `u` = the probe's width as an
unrefined Medium `Toggle`'s child, `s` = with the Toggle refined by
`geometry::toggle`, `e` = the probe refined by `geometry::toggle` in a bare
`div()`; `assert_seam`.

---

## 6 -- DataTable (L6)

### 6.1 Builder (`CN/src/geometry.rs`, beside `spinner_size`)

```rust
/// `DataTable::with_size`: `list.row_height` as `Size::Size`, or upstream's own
/// Medium row height where the theme states none.
///
/// Under `Size::Size` gpui-component sets the row height verbatim
/// (`src/sizing.rs:57-64`) and the cells no text size (`:323-333`), so they
/// inherit — refine the table's container with [`table`] and they take
/// `list.item_font`. The Medium cell padding stands (`:89-94`). At
/// `Size::Medium` the cells are `text_sm` since 0.7.1: 0.875 of the platform
/// font.
#[must_use]
pub fn data_table_size(n: Native<'_>) -> Size {
    Size::Size(
        n.resolved
            .list
            .row_height
            .map_or_else(|| Size::Medium.table_row_height(), px),
    )
}
```

### 6.2 Showcase

`demo::data_table` (`SC/demo.rs:4021-4041`):
`DataTable::new(state).with_size(native_value(cx, geometry::data_table_size)
.unwrap_or_default())` (`Size`'s default is `Medium`), and the container the
`.info(…)` call returns takes `.native(cx, geometry::table)`, the builder the
declarative `Table` takes (`:4169`). In `info::data::data_table` (`SC/info/data.rs:51-75`), which gains the
`styled: bool` parameter `info::buttons::button` already takes,
the "row height … nothing applies it here yet" note becomes two `config`
lines where `styled` — "row height: list.row_height
through geometry::data_table_size as Size::Size, which table_row_height
returns verbatim; upstream's Medium 32px where the theme states none
(sizing.rs, table_row_height)" and "text: list.item_font through
geometry::table on the table's container; a Size::Size cell sets no size of
its own and inherits it (sizing.rs, table_cell_size), where a Medium cell is
text_sm" — and a line for the new state: "keyboard focus: a Tab-focused
DataTable takes `ring` and the focus ring (table/data_table.rs:176-179)".
The "geometry" note ("DataTable is not Styled …") stays.

### 6.3 Gates

Unit: §11. Seam: a one-column, one-row `TableDelegate` (modelled on the
showcase's `SampleTableDelegate`, `SC/support.rs`) whose cell is the text
probe, in a fixed-size container refined by `geometry::table`: `u` = the
probe's width at `Size::Medium`, `s` = at `geometry::data_table_size` (read
from the installed native theme inside the `Build` function), `e` = the probe
refined by `geometry::table` in a bare `div()`; `assert_seam`, under every
native preset.

---

## 7 -- New widgets in the showcase (L7, L8)

### 7.1 ColorSelect

- State: `color_select_state: Entity<ColorPickerState>` on `Showcase`, beside
  `color_picker_state` (`SC/app.rs:375, :1357-1358`), built with
  `ColorPickerState::new(window, cx)` and **no** default value. Empty, the
  field draws only theme colours — the placeholder in `muted_foreground` and
  the swatch edged in `input` (`GC/color_picker.rs:786`, `:806`); once a
  colour is picked, the swatch is that colour edged in it darkened by 30%
  (`:794-796`) and the hex is in `foreground`, demonstration data.
- `demo::color_select(ui, cx, id, state)`, next to `demo::color_picker`
  (`SC/demo.rs:3940-3951`): `ColorSelect::new(state).placeholder("Pick a
  color").info(ui, id, info::inputs::color_select(cx.theme(),
  state.read(cx).value().is_some()))`. No geometry builder (L7). gpui redraws
  the page when the state changes (it re-renders a window whose last draw
  read an entity that notifies, `GP/window.rs:3460-3480`), so the panel
  follows a pick without a subscription.
- Inputs page (`SC/pages/inputs.rs`, after the ColorPicker section):
  heading "ColorSelect" (`inputs-heading-color-select`) and the select
  (`inputs-color-select`, `.self_start()`).
- `info::inputs::color_select(t, picked: bool)` — each colour claim cites the
  line that holds the token, so `every_colour_claim_is_read_at_the_line_it_cites`
  checks it: the fill through the existing `input_background(t)` helper;
  `border` = `input` (`gpui-component/color_picker.rs:773`); `caret` =
  `muted_foreground` (`:812`); `focused border` = `ring` (`:778`); and by
  state — empty: `placeholder` = `muted_foreground` (`:806`), `swatch edge` =
  `input` (`:786`); picked: `text` = `foreground`
  (`gpui-component/input/input.rs:105`) and `instance("swatch", "the picked
  colour, edged in it darkened by 30% (color_picker.rs,
  ColorPickerButton::render_field)")`.
  `not_themeable("frame", "the field is an inner element that sets its own
  height, padding and text size from the Input ladder (color_picker.rs,
  ColorPickerButton::render_field); a refinement lands on the outer wrapper
  and reaches only its width, so no geometry builder is applied")`, and
  `not_themeable("own icons", super::own_icons("the caret's ChevronDown
  (select.rs, Caret)"))`.
- `a_widgets_own_icons_are_named_gpui_components` (`SC/tests.rs:6191`): the
  list and its doc comment gain `("ColorSelect", inputs::color_select(&t,
  false))`.

### 7.2 SpeechWaveform

New file `SC/speech.rs` (`mod speech;` in `main.rs`, beside the others at
`:41-49`):

```rust
//! The inputs the showcase's `SpeechState` runs on: a synthetic level
//! envelope and a recognizer that hears nothing. No microphone, no network.
//! gpui-component documents this route ("implement this trait to feed audio
//! from elsewhere", speech/recognizer.rs, AudioInput).

use std::{cell::Cell, rc::Rc, time::Duration};

use gpui::{App, Subscription};
use gpui_component::speech::{
    AudioFormat, AudioInput, AudioSink, RecognitionSession, SpeechError, SpeechRecognizer,
    SpeechSink,
};

/// How often the synthetic input delivers a block: on a timer, as a device
/// hands over its buffers, because the waveform paces its scroll by when
/// levels arrive (speech/level.rs, the playhead).
const BLOCK: Duration = Duration::from_millis(40);

/// The block amplitudes, cycled: demonstration data for the waveform, not a
/// style value.
const ENVELOPE: [f32; 12] = [0.05, 0.2, 0.45, 0.7, 0.5, 0.3, 0.6, 0.85, 0.4, 0.15, 0.3, 0.1];

/// Feeds a `SpeechState` the envelope. `blocks` counts the blocks delivered,
/// for the test that the timer stops with the session.
#[derive(Clone, Default)]
pub(crate) struct SyntheticAudio {
    pub(crate) blocks: Rc<Cell<usize>>,
}

impl AudioInput for SyntheticAudio {
    fn start(
        &self,
        format: AudioFormat,
        sink: AudioSink,
        cx: &mut App,
    ) -> Result<Subscription, SpeechError> {
        // Saturating: nothing here may overflow, whatever format a
        // recognizer asks for.
        let per_block = u64::from(format.sample_rate())
            .saturating_mul(u64::from(format.channels()))
            .saturating_mul(u64::try_from(BLOCK.as_millis()).unwrap_or(u64::MAX))
            / 1000;
        let per_block = usize::try_from(per_block).unwrap_or(usize::MAX);
        let blocks = self.blocks.clone();
        let task = cx.spawn(async move |cx| {
            for amplitude in ENVELOPE.iter().copied().cycle() {
                cx.background_executor().timer(BLOCK).await;
                let peak = (f32::from(i16::MAX) * amplitude) as i16;
                let samples: Vec<i16> = (0..per_block)
                    .map(|i| if i % 2 == 0 { peak } else { peak.saturating_neg() })
                    .collect();
                blocks.set(blocks.get().saturating_add(1));
                cx.update(|cx| sink.push(samples, cx));
            }
        });
        Ok(Subscription::new(move || drop(task)))
    }
}

/// Opens a session that reports itself ready, discards the audio and
/// recognises nothing.
pub(crate) struct SilentRecognizer;

struct SilentSession(SpeechSink);

impl SpeechRecognizer for SilentRecognizer {
    fn start(
        &self,
        sink: SpeechSink,
        cx: &mut App,
    ) -> Result<Box<dyn RecognitionSession>, SpeechError> {
        sink.ready(cx);
        Ok(Box::new(SilentSession(sink)))
    }
}

impl RecognitionSession for SilentSession {
    fn push_audio(&mut self, _: &[i16], _: &mut App) {}

    fn finish(&mut self, cx: &mut App) {
        self.0.finish(cx);
    }
}
```

Adapt to the compiler where gpui-pre 0.3.8's async signatures differ
(`SC/app.rs:155-164` and `GC/speech/state.rs:249-256` are the shapes to
follow). The float→integer `as` cast saturates and cannot panic; `unwrap_or`
is the non-panicking fallback, not `unwrap`.

- State: `speech_state: Entity<SpeechState>` on `Showcase`, built with
  `cx.new(|cx| SpeechState::new(cx).recognizer(SilentRecognizer)
  .input(SyntheticAudio::default()).system_fallback(false))`. No
  subscription: the page reads the state while it renders, and gpui redraws
  a window whose last draw read an entity that notifies
  (`GP/window.rs:3460-3480`), so the Button's label and the waveform follow
  each new level; the waveform asks for animation frames while it scrolls
  (`GC/speech/waveform.rs:89-91`). The session is not stopped when the page
  changes: like an application's, it runs until Stop.
- Inputs page: heading "SpeechWaveform" (`inputs-heading-speech-waveform`),
  then a row: a Button (`inputs-speech-toggle`, built and labelled like the
  page's other Buttons — `geometry::button`, `labelled` — reading "Stop"
  while `state.status().is_capturing()`, else "Listen"; `on_click` →
  `speech_state.update(cx, |s, cx| s.toggle(cx))`) and
  `SpeechWaveform::new(&speech_state)` (`inputs-speech-waveform`).
- `info::inputs::speech_waveform(t)`: `bars, capturing` = `primary`
  (`gpui-component/speech/waveform.rs:93`); `bars, idle` =
  `muted_foreground` (`:95`); `instance("signal", "the showcase's own
  synthetic level envelope (speech.rs), no microphone; nothing is
  recognised")`; `not_themeable("size", "96px wide and 12 / 16 / 20 / 24px
  tall per Size, literals (speech/waveform.rs, DEFAULT_WIDTH and
  SpeechWaveform::height)")`; `not_themeable("motion", "one bar per 80ms of
  audio, scrolling while capturing; the bars stand still under reduced
  motion (speech/level.rs, LEVEL_INTERVAL; speech/waveform.rs,
  SpeechWaveform::render)")`.

### 7.3 SpeechButton — exception

`docs/showcase-exceptions.toml`, `[gpui]`, beside `SidebarToggleButton`:

```toml
SpeechButton = "it would mix icon themes: it draws gpui-component's own `Mic` or `Square`, chosen as it renders with no setter (speech/button.rs:96-100; read in gpui-component 0.7.1), so under any other chosen icon theme it would show another icon theme's icon; without a recognizer it renders an empty element (:78-79). The Inputs page drives its SpeechState from a Button of the showcase's own"
```

The showcase's sources name `SpeechButton` nowhere, comments included (the
coverage script would count it as shown and call the exception stale).

### 7.4 Coverage

`python3 scripts/check_widget_coverage.py` prints "Every widget is shown or
excepted."

---

## 8 -- Charts (L9)

The chart panels' shared `hover` note (`SC/info/charts.rs:57-60`, which all
five chart infos call: `:135, :186, :240, :291, :366`) gains:
`not_themeable("draw-in", "on first paint the data draws in over 1000ms on
easeOutQuart (theme/mod.rs, PLOT_APPEAR and plot_appear_easing), and the
tooltip waits for it; skipped under reduced motion, which this connector
forwards from the platform (gpui-base plot/appear.rs and motion/presence.rs);
the page replays it on every visit")`.

The doc comment at `:57` ("gpui-component 0.7.0's charts are interactive by
default") names 0.7.1 after confirming it still holds.

---

## 9 -- Citations and prose (L10, L11)

### 9.1 Renumbering — one commit, nothing but digits

Apply Appendix A (436 lines: `CN` library, tests, README, showcase) and
Appendix B's shifted entries (`docs/showcase-exceptions.toml`, 33 lines):
each `:<old>` becomes `:<new>` at the connector line named (located by
content after Task 1's edits). By hand or by a throwaway script; either way
the commit must pass:

```bash
bash <<'EOF'
paths=(connectors/native-theme-gpui docs/showcase-exceptions.toml)
side() { git diff -U0 -- "${paths[@]}" | grep -E "^$1" | grep -vE '^(---|\+\+\+) (a/|b/|/dev/null)' \
         | cut -c2- | sed -E 's/[0-9]+//g' | sort; }
diff <(side -) <(side '\+') && echo DIGITS-ONLY
EOF
```

Expected: `DIGITS-ONLY` — with every digit stripped, the removed lines and the
added lines are the same multiset, so only numbers changed. Run it after
`cargo fmt`: should rustfmt reflow a line whose citation grew by a digit, the
diff shows exactly that line, and the commit message says so. In the same commit, the module headers
naming the verified stack (`src/base_layer.rs:11-12`, `src/colors.rs:8-9`,
`src/config.rs:8-9`, `src/geometry.rs:89-90`, `src/variants.rs:23`,
`src/widgets/mod.rs:44-45`, `SC/info/leaves.rs:8-9`) say "gpui-component
0.7.1, gpui-base 0.7.1 and gpui-pre 0.3.8".

### 9.2 Citations whose code changed (read, not shifted)

| Connector | Cited (0.7.0) | 0.7.1 — rewrite the prose to this |
|---|---|---|
| `SC/info/charts.rs:301` | `chart/candlestick_chart.rs:343` | `:368`, the same `is_bullish`, now inside the draw-in reveal closure (`:340-342`) |
| `SC/info/inputs.rs:449, :774, :1226`, `SC/info/layout.rs:749` | `styled.rs:189` | `:261`, `theme.ring.alpha(FOCUS_RING_OPACITY)` inside `focus_style` |
| `SC/info/layout.rs:736` | `styled.rs:182-184` | `:257-271`: with the ring off, a bordered element's border is tinted (`:269-271`); a borderless one gets a 1 px `ring` line child (`:273-297`) — the Carousel's no-ring branch says which applies to it |
| `SC/info/layout.rs:737` | `styled.rs:186-190` | `:258-262`, the same `focus_ring(…)` call inside `focus_style` (`:250-298`) |
| `SC/info/layout.rs:743` | `styled.rs:187` | `:259` |
| `SC/info/layout.rs:756` | `styled.rs:183` | `:270`, only when the element has a border (`:269`) |
| `SC/info/leaves.rs:665` | `styled.rs:175-190` | `:179-184` + `:250-298`, `focus_ring_style` → `focus_style` |
| `SC/info/leaves.rs:354`, `src/geometry.rs:193` | `sizing.rs:327-333` | `:337-343`, Medium is `text_sm` — rewritten by §4.1 and §4.3, not here |
| `SC/tests.rs:3676` | `sizing.rs:330` | `:340`, `Size::Small \| Size::Medium => text_sm` |
| `src/geometry.rs:593` | `checkbox.rs:270-285` | `:270-282`, refined at `:283`; the size match is now `input_text_size` (XS xs, S/M sm, L base) |
| `src/geometry.rs:608` | `radio.rs:210-225` | `:210-222`, refined at `:223`; same `input_text_size` change |
| `src/widgets/checkbox.rs:585` | `checkbox.rs:280` | `:279` `.rounded(cx.theme().radius * 0.5)` (also wrong at 0.7.0, where it was `:282`) |
| `src/widgets/mod.rs:8` | `switch.rs:186-193` | `:186-195`, a `Size::Large` arm added (track 44 × 24, thumb 20) |
| `docs/showcase-exceptions.toml` (Appendix B's 3 `CHANGED`) | see Appendix B | re-read each (§9.3) |

Other claims rewritten (L10): `SC/info/feedback.rs:422` (Shimmer: when the
text's lightness is within 0.1 of the sweep's target the sweep heads for the
other end, `shimmer.rs:14, :448-454` — the dark Default and Reverse kinds),
and the Questionnaire exception's citations
(`gpui-base questionnaire/state.rs:40-45` → `:48-51`;
`gpui-pre app.rs:2971` → `:3109`, confirm).

### 9.3 Citations already wrong at 0.7.0 (found in this review)

Fix while there: `SC/info/typography.rs:245` cites gpui-base
`input/base/element.rs:2449` (maps to `:2544`, a bare `(`) — the prose means
`let fg = dim(text_style.color)`, find its 0.7.1 line; `SC/tests.rs:81` cites
`gpui-pre-0.3.5/src/app/test_context.rs:132-136` — re-point to 0.3.8;
`docs/showcase-exceptions.toml:591, :874, :882` cite the 0.6.6 ranges
`input/input.rs:531-541` / `input/textarea.rs:141-158`, and `:170` cites
gpui-pre `app.rs:1536`, a lone `}` — re-read each and cite what the prose
means. `src/lib.rs:483` cites gpui-pre-macos 0.3.7 `src/text_system.rs` line
282: the 0.3.8 macOS crate is not in the local registry; fetch it and check,
or write "read at 0.3.7" — never guess the line.

### 9.4 Prose that names the stack

Where a sentence states what the *verified* stack does (`src/geometry.rs:1`,
`:1696`, `:1704`, `src/colors.rs:1204`, `src/config.rs:93`,
`src/contract.rs:374, :1436, :1635, :1868, :1891, :2054`, `src/lib.rs:480-504`,
`src/compat.rs:105-106`, `SC/demo.rs:224, :1207`, `SC/host.rs:3`,
`SC/main.rs:731, :1452, :1513`, `SC/tests.rs:6180`, `SC/info/chrome.rs:400`,
`CN/README.md:29`, and `ROADMAP.md:30, :88, :98`), re-verify against 0.7.1 and
write 0.7.1 / 0.3.8. Where it states what a version *introduced*
(`src/base_layer.rs:8`, `src/icons.rs:1769`, `src/lib.rs:1086`,
`CN/README.md:55, :415`, `SC/info/inputs.rs:440`), it is history: keep it.
Paths that embed a registry directory (`src/contract.rs:1365, :1377`
`gpui-component-0.7.0/src/…`, `tests/seams.rs:449` `gpui-pre-0.3.7/src/…`)
take the new directory name with the mapped line. The counts 139 / 140 / 127
/ 12 hold (`theme_color.rs` and `schema.rs` are byte-identical in 0.7.1).

---

## 10 -- Upstream changes accepted without code (L14)

Read, no connector change (rationale §1.4e–g): the borderless focus line
(`GC/styled.rs:273-297`; every bundled preset states a ring width of 1, 2 or
3), single-line Input no longer clipped (#3343), Command item radius
(`GC/command/state.rs:707-720`), popup-menu single highlight
(`GC/menu/menu_item.rs:116`), RadioGroup disabled items
(`GC/radio.rs:406-408`), Slider touch drag (inherited through `SliderTrack`,
`GB/slider.rs:607-670`), Clipboard accessible name (`GC/clipboard.rs:57`),
Questionnaire confirm-on-choose (still excepted), scroll bounce (#3316),
Textarea rows (#3330), soft wrap vs. horizontal scroll (#3358), OtpInput /
Accordion / Tag ladders (`GC/input/otp_input.rs:117-120`; no claim states the
size; the showcase uses Medium), Large Switch track (`GC/switch.rs:187,
:192`), dropdown row inset (`GC/sizing.rs:173-179`, Tier U), a
non-interactive chart keeping its id (`GC/chart/mod.rs:690-699`; unused),
a `TextView` following its container's text colour (#3329,
`GB/text/text_view.rs:650-664`; the showcase's Markdown sits under the root's
`text_color(theme.foreground)`, `SC/app.rs:2210`, its style's own colour),
scrollable code blocks (#3322; the showcase's `TextView` sets no style),
theme-file watching (#3320; no `ThemeRegistry` in `CN`), Kit's hidden `gpui`
re-export (`gpui-kit/src/lib.rs:100`; unused).

---

## 11 -- Tests

New or changed; all must pass.

| Test | Where | Pins |
|---|---|---|
| `all_icon_names!` / `variant_name` | `CN/src/icons.rs` tests | §3.3: an upstream variant the list lacks is a compile error of the test module |
| `all_icon_names_count_matches_gpui_component` (106) | same | the counts the prose states |
| `mic_and_square_map_to_the_sets_own_glyphs`, `mic_and_square_take_standard_names_on_every_desktop` | same | §3.2 |
| `button_label_carries_the_button_font` | `CN/src/geometry.rs` tests | below |
| `button_label_follows_the_button_font_not_the_body_font` | same | below |
| `toggle_is_the_button_labels_text` | same | `toggle(n) == button_label(n)` in every case; no height, padding, radius, border |
| `data_table_size_falls_back_to_upstream_medium` | same | below |
| seams: Button child label, Toggle, DataTable cell | `CN/tests/seams.rs` | §4.4, §5.3, §6.3 |
| `the_synthetic_input_feeds_levels_and_stops_with_the_session` | `SC/tests.rs` | §7.2, below |
| `a_widgets_own_icons_are_named_gpui_components` | `SC/tests.rs:6191` | + ColorSelect, empty (§7.1) |
| `every_colour_claim_is_read_at_the_line_it_cites` | `CN/src/showcase.rs` | §7, §9 (163 failures → 0) |
| `a_page_sample_icon_the_set_lacks_is_absent` | `SC/tests.rs:6034` | passes unedited after §4 |

On the module's own helpers (`for_each_case`: every case at factors 1.0, 1.5
and 0.8; `assert_text`: size × s and weight; `resolved`, `Native::unscaled`,
`abs`):

```rust
    #[test]
    fn button_label_carries_the_button_font() {
        for_each_case(|r, s, n| {
            let out = button_label(n);
            assert_text(&out, &r.button.font, s);
            assert_eq!(out.text.line_height, Some(relative(r.defaults.line_height)));
            assert_eq!(out.text.color, None, "the label takes the button's state colours");
        });
    }

    #[test]
    fn button_label_follows_the_button_font_not_the_body_font() {
        let mut r = resolved("kde-breeze", ColorMode::Light);
        r.button.font.size = r.defaults.font.size + 3.0;
        let out = button_label(Native::unscaled(&r));
        assert_eq!(out.text.font_size, abs(r.button.font.size));
    }

    #[test]
    fn toggle_is_the_button_labels_text() {
        for_each_case(|_, _, n| {
            let out = toggle(n);
            assert_eq!(out, button_label(n));
            assert_eq!(out.size.height, None);
            assert_eq!(out.padding, Default::default());
            assert_eq!(out.corner_radii, Default::default());
            assert_eq!(out.border_widths, Default::default());
        });
    }

    #[test]
    fn data_table_size_falls_back_to_upstream_medium() {
        let mut stated = 0;
        let mut unstated = 0;
        for_each_case(|r, _, n| {
            let expected = match r.list.row_height {
                Some(h) => {
                    stated += 1;
                    px(h)
                }
                None => {
                    unstated += 1;
                    Size::Medium.table_row_height()
                }
            };
            assert!(matches!(data_table_size(n), Size::Size(h) if h == expected));
        });
        assert!(stated > 0 && unstated > 0, "both branches must be exercised");
    }
```

(If `StyleRefinement` is not `PartialEq`, compare `toggle`'s `text` fields
with `button_label`'s. Field names per gpui-pre 0.3.8; keep each assertion's
meaning. `kde-breeze` is in the module's `CASES` and states no row height;
if no case states one, add a preset that does to the local loop.)

In the showcase's test module, headless and without a window:

```rust
#[gpui::test]
fn the_synthetic_input_feeds_levels_and_stops_with_the_session(cx: &mut TestAppContext) {
    use crate::speech::{SilentRecognizer, SyntheticAudio};
    use gpui_component::speech::{SpeechState, SpeechStatus};

    let audio = SyntheticAudio::default();
    let blocks = audio.blocks.clone();
    let state = cx.update(|cx| {
        cx.new(|cx| {
            SpeechState::new(cx)
                .recognizer(SilentRecognizer)
                .input(audio)
                .system_fallback(false)
        })
    });

    cx.update(|cx| state.update(cx, |s, cx| s.start(cx)));
    cx.executor().advance_clock(Duration::from_secs(1));
    cx.run_until_parked();
    cx.read(|cx| {
        let s = state.read(cx);
        assert!(s.status().is_capturing(), "the silent recognizer reports ready");
        assert!(s.levels().next().is_some(), "no level in a second of synthetic audio");
    });

    cx.update(|cx| state.update(cx, |s, cx| s.stop(cx)));
    cx.run_until_parked();
    assert_eq!(cx.read(|cx| state.read(cx).status()), SpeechStatus::Idle);
    let delivered = blocks.get();
    cx.executor().advance_clock(Duration::from_secs(1));
    cx.run_until_parked();
    assert_eq!(blocks.get(), delivered, "the synthetic input kept running after stop");
}
```

---

## 12 -- Project documents (L13)

### 12.1 CHANGELOG (`CHANGELOG.md`)

A new `## [Unreleased]` above `## [0.6.0] - 2026-10-01`, in the file's own
taxonomy and crate grouping:

- **Fixed:** native-theme-gpui 0.6.0 does not build against gpui-component
  0.7.1, which added `IconName::Mic` and `IconName::Square`; 0.6.1 does. A
  Button's label, a Toggle's text and DataTable cells can be drawn at the
  platform's font size again, which gpui-component 0.7.1 shrank to 0.875 of it
  at `Size::Medium` (through the builders below).
- **Changed:** requires gpui-component, gpui-base and gpui-kit 0.7.1 and
  gpui-pre 0.3.8 (MSRV unchanged, 1.95.0). The three `IconName` tables return
  `None` for a variant newer than the connector, instead of failing to compile.
- **Added:** `geometry::button_label`, `geometry::toggle`,
  `geometry::data_table_size`; `Mic` and `Square` in the Lucide, Material and
  freedesktop tables; native-theme bundles Lucide and Material `mic` and
  `square`. Showcase: ColorSelect, SpeechWaveform.

Do not date the heading or add a compare link (the release commit's).

### 12.2 `docs/todo.md` — append or amend in place, never rewrite

- The open item "**A `DataTable` can take the platform's row height today.**"
  (`:1625-1635`): tick it and append "Done in v0.6.1:
  `geometry::data_table_size`. Since gpui-component 0.7.1 the same `Size::Size`
  is also what keeps the cells from `text_sm` (`sizing.rs:323-333`)."
- The item "**`Size::Size(px)` is an escape hatch, but read the target
  first.**" (`:1704-1718`) and the note above it that a Button label "is
  already the platform's size": append "0.7.1 (2026-10): `button_text_size`
  is `text_sm` at Medium (`sizing.rs:337-343`), so a `.label()` is 0.875 of the
  platform font; `geometry::button_label` on a child label carries
  `button.font` (v0.6.1)."
- Under `#### Upstream PRs to gpui-component` (`:2412`), three entries:
  (1) "Button: let the caller's text size reach the label — `button_text_size`
  on the content element (`button/button.rs:749`) shadows a size set on the
  button; honouring the instance style's text size there, or a public
  `content_style`, would retire the child-label pattern of
  `geometry::button_label`." (2) "ColorSelect: apply the caller's refinement
  to the framed field (`color_picker.rs:488` lands on the wrapper; the field is
  `render_field`, `:762-813`)." (3) "DataTable: `Styled`, or a text-size
  setter, so the cell size need not go through `Size::Size`."
- In the existing icon-provider PR entry (`:2472-2480`), append to its list:
  "`SpeechButton`'s Mic / Square (`src/speech/button.rs:96-100`, 0.7.1)".
- Under `#### Research` (`:1236`): "A mechanical check of the library's prose
  `file:line` citations against the locked registry sources (the showcase's
  colour claims have one, `every_colour_claim_is_read_at_the_line_it_cites`);
  the 0.7.1 review mapped 1237 of them by script (v0.6.1 spec, Appendix A)."

### 12.3 `ROADMAP.md` and the gap table

- A delivered entry before the next milestone: "## v0.6.1 — gpui connector on
  GPUI Kit 0.7.1" — two sentences (the build fix and floors; the label /
  toggle / table builders and the two new showcase widgets) and a link to
  `docs/archive/todo_v0.6.1_gpui-kit-0.7.1-spec.md` (the path it has after the
  last task).
- `## v0.6.1 — Full theme geometry in the iced connector` (`:33`) →
  `## v0.6.2 — …`; `## v0.6.2 — Upstream receivers for the gpui connector`
  (`:74`) → `## v0.6.3 — …`, and "v0.6.2 is the set of PRs" (`:83`) → "v0.6.3".
- In that list (`:96`): "button label text size independent of rem" → "a
  Button label that takes the caller's text size (`button_text_size` on the
  content element shadows it; since 0.7.1 the Medium label is `text_sm`)";
  two bullets added: "`ColorSelect` applying the caller's refinement to its
  framed field" and "`DataTable` as `Styled`, or with a text-size setter".
- `:30`, `:88`, `:98`: per §9.4.
- `docs/todo_gpui-full-theme.md`: `:72` "the v0.6.2 PR list" → "v0.6.3"; and
  below the table a dated note (append): "v0.6.1 (gpui-component 0.7.1): the
  button's label size reaches a Button through a child label
  (`geometry::button_label`), a `Toggle`'s text through `geometry::toggle`, and
  a `DataTable`'s row height and cell text through
  `geometry::data_table_size` + `geometry::table`."

### 12.4 Connector README

Beyond §2.4 and §3.5: the geometry table (`:246-271`) — the `button` row's
"the label size is set on an inner element" continues "— `button_label`
reaches it on a child label"; new rows `button_label` (`button.font` size and
weight, `defaults.line_height` → the label passed as a `Button`'s child),
`toggle` (the same → `Toggle`), `data_table_size` (`list.row_height`, or
upstream's Medium → `DataTable::with_size`; with `table` on the container for
the cell text). The example at `:194` shows §4.1's pattern in place of
`.label("Save")`.

### 12.5 Version and stamp

- `0.6.0` → `0.6.1`: `Cargo.toml:14` and `:24`, `native-theme/Cargo.toml:53`
  (`native-theme-derive`), `connectors/native-theme-egui-widgets/Cargo.toml:38`
  (`native-theme-egui`); `cargo check --workspace` once so `Cargo.lock`
  records it.
- `scripts/update_compatibility.sh run gpui` writes `docs/COMPATIBILITY.toml`
  and the README's Verified line, only if its gates pass on the newest
  upstream it resolves. It needs the connector's paths clean, so it runs after
  the last source commit. The maintainer's release asset run repeats it for
  all three connectors (the version bump stales the egui stamp).

---

## 13 -- Acceptance

Every command from the repository root with `CARGO_BUILD_JOBS=4`.

| # | Command | Expected |
|---|---|---|
| A1 | `cargo build -p native-theme-gpui --all-targets --all-features --locked` | `Finished`, no warning |
| A2 | `cargo test -p native-theme-gpui --all-features` | all pass |
| A3 | `cargo test -p native-theme` | all pass (icon sources) |
| A4 | `cargo +1.95.0 check -p native-theme-gpui --lib --locked` | `Finished` |
| A5 | `cargo clippy -p native-theme-gpui --all-targets --all-features -- -D warnings` | clean |
| A6 | `python3 scripts/check_widget_coverage.py` | "Every widget is shown or excepted." |
| A7 | layout dumps after (§4.4) vs `target/layout-071/before/` | 0 differences |
| A8 | `scripts/update_compatibility.sh run gpui`, then `check` | the gpui stamp names gpui-kit 0.7.1 / gpui-pre 0.3.8 (or newer, if upstream released and the gates pass); `check` reports the gpui connector verified |
| A9 | `./scripts/check_release.sh` | no failure; warnings only for the asset stamp and the egui compatibility stamp (the version bump touches `native-theme-egui-widgets/Cargo.toml`) |
| A10 | `grep -rn "0\.7\.0\|0\.3\.7" connectors/native-theme-gpui/{src,tests,examples,README.md,Cargo.toml} ROADMAP.md` | only §9.4's history sentences |

---

## Appendix A -- Library, test, README and showcase citations that moved

Mechanically mapped by GNU-diff hunks between the registry sources, then every
old line re-read byte-identical at its new line (436 entries). The 17 changed
sites are in §9.2 and are **not** here. Format: `<connector file>:<line at
942302d6>  <crate> <file>:<old> -> <new>`; paths relative to `CN/` and to each
crate's `src/`.

```text
README.md:247  gpui-component input/input.rs:762-764 -> 774-776
examples/showcase-gpui/app.rs:304  gpui-pre window.rs:6251-6259 -> 6325-6333
examples/showcase-gpui/demo.rs:194  gpui-component button/button.rs:660-662 -> 672-674
examples/showcase-gpui/demo.rs:202  gpui-component menu/popup_menu.rs:1222 -> 1221
examples/showcase-gpui/demo.rs:207  gpui-component menu/popup_menu.rs:1483-1485 -> 1487-1489
examples/showcase-gpui/demo.rs:208  gpui-component menu/popup_menu.rs:1321 -> 1325
examples/showcase-gpui/demo.rs:216  gpui-component menu/popup_menu.rs:1252 -> 1256
examples/showcase-gpui/demo.rs:224  gpui-component styled.rs:66-71 -> 68-73
examples/showcase-gpui/demo.rs:225  gpui-component styled.rs:63-65 -> 65-67
examples/showcase-gpui/demo.rs:1119  gpui-component button/button.rs:611-614 -> 615-618
examples/showcase-gpui/demo.rs:1119  gpui-component button/button.rs:713-717 -> 725-729
examples/showcase-gpui/demo.rs:2905  gpui-component button/button.rs:785-791 -> 797-803
examples/showcase-gpui/demo.rs:2906  gpui-component button/button.rs:721 -> 733
examples/showcase-gpui/demo.rs:2961  gpui-component button/button.rs:657-672 -> 669-684
examples/showcase-gpui/demo.rs:2961  gpui-component button/button.rs:721 -> 733
examples/showcase-gpui/demo.rs:3152  gpui-component input/input.rs:773 -> 785
examples/showcase-gpui/demo.rs:3152  gpui-component input/input.rs:781 -> 793
examples/showcase-gpui/demo.rs:3178  gpui-component input/input.rs:761-765 -> 773-777
examples/showcase-gpui/demo.rs:3178  gpui-component input/input.rs:781 -> 793
examples/showcase-gpui/demo.rs:3201  gpui-component input/input.rs:518 -> 529
examples/showcase-gpui/demo.rs:3202  gpui-component input/input.rs:735-742 -> 747-754
examples/showcase-gpui/demo.rs:3204  gpui-component input/input.rs:781 -> 793
examples/showcase-gpui/demo.rs:3238  gpui-pre window.rs:4505-4513 -> 4553-4561
examples/showcase-gpui/demo.rs:3277  gpui-component input/textarea.rs:207 -> 221
examples/showcase-gpui/demo.rs:3628  gpui-component radio.rs:406 -> 403
examples/showcase-gpui/demo.rs:3900  gpui-component select.rs:541 -> 542
examples/showcase-gpui/demo.rs:3914  gpui-component select.rs:534-552 -> 535-553
examples/showcase-gpui/demo.rs:3914  gpui-component combobox.rs:980-997 -> 981-998
examples/showcase-gpui/demo.rs:3915  gpui-component select.rs:794 -> 795
examples/showcase-gpui/demo.rs:5284  gpui-component button/button.rs:399 -> 403
examples/showcase-gpui/info/buttons.rs:22  gpui-component button/button.rs:966 -> 978
examples/showcase-gpui/info/buttons.rs:28  gpui-component button/button.rs:980 -> 992
examples/showcase-gpui/info/buttons.rs:36  gpui-component button/button.rs:967 -> 979
examples/showcase-gpui/info/buttons.rs:42  gpui-component button/button.rs:985 -> 997
examples/showcase-gpui/info/buttons.rs:50  gpui-component button/button.rs:968 -> 980
examples/showcase-gpui/info/buttons.rs:56  gpui-component button/button.rs:992 -> 1004
examples/showcase-gpui/info/buttons.rs:64  gpui-component button/button.rs:969 -> 981
examples/showcase-gpui/info/buttons.rs:70  gpui-component button/button.rs:1000 -> 1012
examples/showcase-gpui/info/buttons.rs:78  gpui-component button/button.rs:971 -> 983
examples/showcase-gpui/info/buttons.rs:84  gpui-component button/button.rs:1014 -> 1026
examples/showcase-gpui/info/buttons.rs:92  gpui-component button/button.rs:970 -> 982
examples/showcase-gpui/info/buttons.rs:98  gpui-component button/button.rs:1007 -> 1019
examples/showcase-gpui/info/buttons.rs:106  gpui-component button/button.rs:972 -> 984
examples/showcase-gpui/info/buttons.rs:112  gpui-component button/button.rs:1021 -> 1033
examples/showcase-gpui/info/buttons.rs:125  gpui-component button/button.rs:1024 -> 1036
examples/showcase-gpui/info/buttons.rs:131  gpui-component button/button.rs:1025 -> 1037
examples/showcase-gpui/info/buttons.rs:139  gpui-component button/button.rs:980 -> 992
examples/showcase-gpui/info/buttons.rs:147  gpui-component button/button.rs:902 -> 914
examples/showcase-gpui/info/buttons.rs:153  gpui-component button/button.rs:983 -> 995
examples/showcase-gpui/info/buttons.rs:167  gpui-component button/button.rs:1110 -> 1122
examples/showcase-gpui/info/buttons.rs:173  gpui-component button/button.rs:1194 -> 1206
examples/showcase-gpui/info/buttons.rs:181  gpui-component button/button.rs:1117 -> 1129
examples/showcase-gpui/info/buttons.rs:187  gpui-component button/button.rs:1201 -> 1213
examples/showcase-gpui/info/buttons.rs:195  gpui-component button/button.rs:1124 -> 1136
examples/showcase-gpui/info/buttons.rs:201  gpui-component button/button.rs:1208 -> 1220
examples/showcase-gpui/info/buttons.rs:209  gpui-component button/button.rs:1131 -> 1143
examples/showcase-gpui/info/buttons.rs:215  gpui-component button/button.rs:1216 -> 1228
examples/showcase-gpui/info/buttons.rs:223  gpui-component button/button.rs:1145 -> 1157
examples/showcase-gpui/info/buttons.rs:229  gpui-component button/button.rs:1230 -> 1242
examples/showcase-gpui/info/buttons.rs:237  gpui-component button/button.rs:1138 -> 1150
examples/showcase-gpui/info/buttons.rs:243  gpui-component button/button.rs:1223 -> 1235
examples/showcase-gpui/info/buttons.rs:251  gpui-component button/button.rs:1152 -> 1164
examples/showcase-gpui/info/buttons.rs:257  gpui-component button/button.rs:1237 -> 1249
examples/showcase-gpui/info/buttons.rs:279  gpui-component button/button.rs:1170 -> 1182
examples/showcase-gpui/info/buttons.rs:285  gpui-component button/button.rs:1246 -> 1258
examples/showcase-gpui/info/buttons.rs:293  gpui-component button/button.rs:1171 -> 1183
examples/showcase-gpui/info/buttons.rs:299  gpui-component button/button.rs:1247 -> 1259
examples/showcase-gpui/info/buttons.rs:307  gpui-component button/button.rs:891-895 -> 903-907
examples/showcase-gpui/info/buttons.rs:313  gpui-component button/button.rs:896-900 -> 908-912
examples/showcase-gpui/info/buttons.rs:321  gpui-component button/button.rs:905 -> 917
examples/showcase-gpui/info/buttons.rs:327  gpui-component button/button.rs:908 -> 920
examples/showcase-gpui/info/buttons.rs:334  gpui-component button/button.rs:687-692 -> 699-704
examples/showcase-gpui/info/buttons.rs:343  gpui-component button/button.rs:1032 -> 1044
examples/showcase-gpui/info/buttons.rs:349  gpui-component button/button.rs:1032 -> 1044
examples/showcase-gpui/info/buttons.rs:355  gpui-component button/button.rs:1034 -> 1046
examples/showcase-gpui/info/buttons.rs:361  gpui-component button/button.rs:1034 -> 1046
examples/showcase-gpui/info/buttons.rs:390  gpui-component button/button.rs:1307 -> 1319
examples/showcase-gpui/info/buttons.rs:399  gpui-component button/button.rs:1312 -> 1324
examples/showcase-gpui/info/buttons.rs:405  gpui-component button/button.rs:1308 -> 1320
examples/showcase-gpui/info/buttons.rs:407  gpui-component button/button.rs:1322-1326 -> 1334-1338
examples/showcase-gpui/info/buttons.rs:408  gpui-component theme/mod.rs:465-470 -> 485-490
examples/showcase-gpui/info/buttons.rs:413  gpui-component theme/mod.rs:467 -> 487
examples/showcase-gpui/info/buttons.rs:419  gpui-component theme/mod.rs:469 -> 489
examples/showcase-gpui/info/buttons.rs:426  gpui-component button/button.rs:813 -> 825
examples/showcase-gpui/info/buttons.rs:427  gpui-component button/button.rs:510-512 -> 514-516
examples/showcase-gpui/info/buttons.rs:427  gpui-component button/button.rs:700 -> 712
examples/showcase-gpui/info/buttons.rs:435  gpui-component button/button.rs:813 -> 825
examples/showcase-gpui/info/buttons.rs:438  gpui-component button/button.rs:967 -> 979
examples/showcase-gpui/info/buttons.rs:441  gpui-component button/button.rs:813 -> 825
examples/showcase-gpui/info/buttons.rs:444  gpui-component button/button.rs:985 -> 997
examples/showcase-gpui/info/buttons.rs:507  gpui-component button/button.rs:1315 -> 1327
examples/showcase-gpui/info/buttons.rs:647  gpui-component styled.rs:197 -> 190
examples/showcase-gpui/info/charts.rs:66  gpui-component styled.rs:197 -> 190
examples/showcase-gpui/info/charts.rs:99  gpui-component chart/bar_chart.rs:821 -> 842
examples/showcase-gpui/info/charts.rs:105  gpui-component chart/bar_chart.rs:791 -> 812
examples/showcase-gpui/info/charts.rs:111  gpui-component chart/bar_chart.rs:857 -> 878
examples/showcase-gpui/info/charts.rs:150  gpui-component chart/line_chart.rs:406 -> 427
examples/showcase-gpui/info/charts.rs:156  gpui-component chart/line_chart.rs:393 -> 414
examples/showcase-gpui/info/charts.rs:162  gpui-component chart/mod.rs:419 -> 493
examples/showcase-gpui/info/charts.rs:208  gpui-component chart/area_chart.rs:416 -> 437
examples/showcase-gpui/info/charts.rs:214  gpui-component chart/area_chart.rs:403 -> 424
examples/showcase-gpui/info/charts.rs:220  gpui-component chart/mod.rs:419 -> 493
examples/showcase-gpui/info/charts.rs:315  gpui-component chart/candlestick_chart.rs:227 -> 249
examples/showcase-gpui/info/charts.rs:321  gpui-component chart/candlestick_chart.rs:228 -> 250
examples/showcase-gpui/info/charts.rs:327  gpui-component chart/candlestick_chart.rs:294 -> 316
examples/showcase-gpui/info/charts.rs:333  gpui-component chart/candlestick_chart.rs:286 -> 308
examples/showcase-gpui/info/charts.rs:339  gpui-component chart/candlestick_chart.rs:304 -> 326
examples/showcase-gpui/info/chrome.rs:250  gpui-component styled.rs:197 -> 190
examples/showcase-gpui/info/chrome.rs:293  gpui-component styled.rs:197 -> 190
examples/showcase-gpui/info/chrome.rs:295  gpui-component styled.rs:26 -> 28
examples/showcase-gpui/info/chrome.rs:300  gpui-component styled.rs:35 -> 37
examples/showcase-gpui/info/chrome.rs:306  gpui-component menu/menu_item.rs:107 -> 104
examples/showcase-gpui/info/chrome.rs:324  gpui-component menu/popup_menu.rs:1253 -> 1257
examples/showcase-gpui/info/chrome.rs:714  gpui-component button/button.rs:995 -> 1007
examples/showcase-gpui/info/chrome.rs:720  gpui-component button/button.rs:1172 -> 1184
examples/showcase-gpui/info/chrome.rs:728  gpui-component button/button.rs:995 -> 1007
examples/showcase-gpui/info/chrome.rs:734  gpui-component button/button.rs:1172 -> 1184
examples/showcase-gpui/info/chrome.rs:742  gpui-component button/button.rs:995 -> 1007
examples/showcase-gpui/info/chrome.rs:748  gpui-component button/button.rs:1172 -> 1184
examples/showcase-gpui/info/chrome.rs:764  gpui-component button/button.rs:1211 -> 1223
examples/showcase-gpui/info/chrome.rs:770  gpui-component button/button.rs:1156-1162 -> 1168-1174
examples/showcase-gpui/info/chrome.rs:777  gpui-component button/button.rs:1159 -> 1171
examples/showcase-gpui/info/chrome.rs:784  gpui-component button/button.rs:1157 -> 1169
examples/showcase-gpui/info/chrome.rs:793  gpui-component theme/mod.rs:465-470 -> 485-490
examples/showcase-gpui/info/chrome.rs:800  gpui-component theme/mod.rs:467 -> 487
examples/showcase-gpui/info/chrome.rs:807  gpui-component theme/mod.rs:469 -> 489
examples/showcase-gpui/info/chrome.rs:830  gpui-component styled.rs:197 -> 190
examples/showcase-gpui/info/chrome.rs:842  gpui-component combobox.rs:999 -> 1000
examples/showcase-gpui/info/chrome.rs:1234  gpui-component command/state.rs:913 -> 932
examples/showcase-gpui/info/chrome.rs:1240  gpui-component command/state.rs:914 -> 933
examples/showcase-gpui/info/chrome.rs:1246  gpui-component command/state.rs:930 -> 949
examples/showcase-gpui/info/chrome.rs:1252  gpui-component command/state.rs:935 -> 954
examples/showcase-gpui/info/chrome.rs:1258  gpui-component command/state.rs:729 -> 748
examples/showcase-gpui/info/chrome.rs:1264  gpui-component command/state.rs:808 -> 827
examples/showcase-gpui/info/chrome.rs:1270  gpui-component command/state.rs:717 -> 736
examples/showcase-gpui/info/chrome.rs:1276  gpui-component command/state.rs:718 -> 737
examples/showcase-gpui/info/chrome.rs:1282  gpui-component command/state.rs:868 -> 887
examples/showcase-gpui/info/data.rs:299  gpui-component button/button.rs:890 -> 902
examples/showcase-gpui/info/data.rs:331  gpui-component button/button.rs:980 -> 992
examples/showcase-gpui/info/data.rs:337  gpui-component button/button.rs:1032 -> 1044
examples/showcase-gpui/info/data.rs:343  gpui-component button/button.rs:891-895 -> 903-907
examples/showcase-gpui/info/data.rs:349  gpui-component button/button.rs:896-900 -> 908-912
examples/showcase-gpui/info/data.rs:368  gpui-component button/button.rs:1315 -> 1327
examples/showcase-gpui/info/data.rs:654  gpui-component theme/mod.rs:514 -> 534
examples/showcase-gpui/info/data.rs:663  gpui-component theme/mod.rs:514 -> 534
examples/showcase-gpui/info/data.rs:670  gpui-component theme/mod.rs:514 -> 534
examples/showcase-gpui/info/data.rs:679  gpui-component theme/mod.rs:514 -> 534
examples/showcase-gpui/info/data.rs:741  gpui-component theme/mod.rs:401 -> 421
examples/showcase-gpui/info/data.rs:793  gpui-component theme/mod.rs:514 -> 534
examples/showcase-gpui/info/data.rs:800  gpui-component theme/mod.rs:514 -> 534
examples/showcase-gpui/info/data.rs:806  gpui-component theme/mod.rs:514 -> 534
examples/showcase-gpui/info/data.rs:812  gpui-component theme/mod.rs:514 -> 534
examples/showcase-gpui/info/data.rs:818  gpui-component theme/mod.rs:514 -> 534
examples/showcase-gpui/info/feedback.rs:11  gpui-component tag.rs:265 -> 269
examples/showcase-gpui/info/feedback.rs:583  gpui-pre window.rs:4505-4512 -> 4553-4560
examples/showcase-gpui/info/feedback.rs:583  gpui-pre window.rs:4649 -> 4697
examples/showcase-gpui/info/feedback.rs:584  gpui-pre window.rs:4698 -> 4746
examples/showcase-gpui/info/inputs.rs:44  gpui-component input/input.rs:773 -> 785
examples/showcase-gpui/info/inputs.rs:44  gpui-component input/input.rs:781 -> 793
examples/showcase-gpui/info/inputs.rs:51  gpui-component input/input.rs:776 -> 788
examples/showcase-gpui/info/inputs.rs:60  gpui-component input/input.rs:694 -> 706
examples/showcase-gpui/info/inputs.rs:72  gpui-component input/input.rs:547 -> 559
examples/showcase-gpui/info/inputs.rs:75  gpui-component input/input.rs:774 -> 786
examples/showcase-gpui/info/inputs.rs:75  gpui-component input/input.rs:781 -> 793
examples/showcase-gpui/info/inputs.rs:130  gpui-component input/textarea.rs:207 -> 221
examples/showcase-gpui/info/inputs.rs:131  gpui-component input/input.rs:694 -> 706
examples/showcase-gpui/info/inputs.rs:142  gpui-component input/input.rs:776 -> 788
examples/showcase-gpui/info/inputs.rs:148  gpui-component input/input.rs:739 -> 751
examples/showcase-gpui/info/inputs.rs:217  gpui-component input/input.rs:694 -> 706
examples/showcase-gpui/info/inputs.rs:251  gpui-component checkbox.rs:307 -> 304
examples/showcase-gpui/info/inputs.rs:296  gpui-component checkbox.rs:338 -> 335
examples/showcase-gpui/info/inputs.rs:303  gpui-component checkbox.rs:336 -> 333
examples/showcase-gpui/info/inputs.rs:347  gpui-component radio.rs:238 -> 235
examples/showcase-gpui/info/inputs.rs:463  gpui-component switch.rs:194-198 -> 196-200
examples/showcase-gpui/info/inputs.rs:933  gpui-component input/otp_input.rs:117 -> 109
examples/showcase-gpui/info/inputs.rs:939  gpui-component input/otp_input.rs:121 -> 113
examples/showcase-gpui/info/inputs.rs:952  gpui-component input/otp_input.rs:159 -> 153
examples/showcase-gpui/info/inputs.rs:974  gpui-component select.rs:554 -> 555
examples/showcase-gpui/info/inputs.rs:980  gpui-component select.rs:451 -> 452
examples/showcase-gpui/info/inputs.rs:1033  gpui-component combobox.rs:992 -> 993
examples/showcase-gpui/info/inputs.rs:1040  gpui-component select.rs:547 -> 548
examples/showcase-gpui/info/inputs.rs:1054  gpui-component styled.rs:197 -> 190
examples/showcase-gpui/info/inputs.rs:1060  gpui-component color_picker.rs:205 -> 212
examples/showcase-gpui/info/inputs.rs:1066  gpui-component color_picker.rs:206 -> 213
examples/showcase-gpui/info/inputs.rs:1072  gpui-component color_picker.rs:207 -> 214
examples/showcase-gpui/info/inputs.rs:1078  gpui-component color_picker.rs:208 -> 215
examples/showcase-gpui/info/inputs.rs:1084  gpui-component color_picker.rs:209 -> 216
examples/showcase-gpui/info/inputs.rs:1090  gpui-component color_picker.rs:210 -> 217
examples/showcase-gpui/info/inputs.rs:1096  gpui-component color_picker.rs:211 -> 218
examples/showcase-gpui/info/inputs.rs:1102  gpui-component color_picker.rs:212 -> 219
examples/showcase-gpui/info/inputs.rs:1108  gpui-component color_picker.rs:213 -> 220
examples/showcase-gpui/info/inputs.rs:1114  gpui-component color_picker.rs:214 -> 221
examples/showcase-gpui/info/inputs.rs:1120  gpui-component color_picker.rs:215 -> 222
examples/showcase-gpui/info/inputs.rs:1126  gpui-component color_picker.rs:216 -> 223
examples/showcase-gpui/info/inputs.rs:1146  gpui-component styled.rs:197 -> 190
examples/showcase-gpui/info/layout.rs:804  gpui-component button/button.rs:693 -> 705
examples/showcase-gpui/info/layout.rs:804  gpui-component button/button.rs:778-783 -> 790-795
examples/showcase-gpui/info/layout.rs:811  gpui-component button/button.rs:1273 -> 1285
examples/showcase-gpui/info/layout.rs:817  gpui-component button/button.rs:980 -> 992
examples/showcase-gpui/info/layout.rs:823  gpui-component button/button.rs:1032 -> 1044
examples/showcase-gpui/info/layout.rs:834  gpui-component button/button.rs:966 -> 978
examples/showcase-gpui/info/layout.rs:840  gpui-component button/button.rs:980 -> 992
examples/showcase-gpui/info/layout.rs:846  gpui-component button/button.rs:1032 -> 1044
examples/showcase-gpui/info/layout.rs:852  gpui-component button/button.rs:1110 -> 1122
examples/showcase-gpui/info/layout.rs:858  gpui-component button/button.rs:1194 -> 1206
examples/showcase-gpui/info/layout.rs:1093  gpui-component theme/mod.rs:396 -> 416
examples/showcase-gpui/info/layout.rs:1099  gpui-component theme/mod.rs:401 -> 421
examples/showcase-gpui/info/layout.rs:1105  gpui-component theme/mod.rs:412 -> 432
examples/showcase-gpui/info/leaves.rs:97  gpui-component button/button.rs:98-137 -> 102-141
examples/showcase-gpui/info/leaves.rs:97  gpui-component button/button.rs:1138-1143 -> 1150-1155
examples/showcase-gpui/info/leaves.rs:148  gpui-component button/button.rs:1283 -> 1295
examples/showcase-gpui/info/leaves.rs:183  gpui-component button/button.rs:98-137 -> 102-141
examples/showcase-gpui/info/leaves.rs:183  gpui-component button/button.rs:1138-1143 -> 1150-1155
examples/showcase-gpui/info/leaves.rs:188  gpui-component button/button.rs:98-137 -> 102-141
examples/showcase-gpui/info/leaves.rs:188  gpui-component button/button.rs:1138-1143 -> 1150-1155
examples/showcase-gpui/info/leaves.rs:193  gpui-component button/button.rs:98-137 -> 102-141
examples/showcase-gpui/info/leaves.rs:193  gpui-component button/button.rs:1138-1143 -> 1150-1155
examples/showcase-gpui/info/leaves.rs:289  gpui-component button/button.rs:1245-1249 -> 1257-1261
examples/showcase-gpui/info/leaves.rs:334  gpui-component button/button.rs:1081-1086 -> 1093-1098
examples/showcase-gpui/info/leaves.rs:369  gpui-component button/button.rs:1169-1174 -> 1181-1186
examples/showcase-gpui/info/leaves.rs:558  gpui-component combobox.rs:997 -> 998
examples/showcase-gpui/info/leaves.rs:569  gpui-component select.rs:598-599 -> 599-600
examples/showcase-gpui/info/leaves.rs:770  gpui-component switch.rs:290 -> 292
examples/showcase-gpui/info/leaves.rs:1097  gpui-base input/base/element.rs:2446 -> 2541
examples/showcase-gpui/info/leaves.rs:1102  gpui-component input/input.rs:736-739 -> 748-751
examples/showcase-gpui/info/leaves.rs:1142  gpui-component input/input.rs:547 -> 559
examples/showcase-gpui/info/leaves.rs:2082  gpui-component input/input.rs:776 -> 788
examples/showcase-gpui/info/overlays.rs:157  gpui-component button/button.rs:969 -> 981
examples/showcase-gpui/info/overlays.rs:221  gpui-component styled.rs:197 -> 190
examples/showcase-gpui/info/overlays.rs:227  gpui-component styled.rs:198 -> 191
examples/showcase-gpui/info/overlays.rs:229  gpui-component styled.rs:26 -> 28
examples/showcase-gpui/info/overlays.rs:234  gpui-component styled.rs:35 -> 37
examples/showcase-gpui/info/registry.rs:154  gpui-pre window.rs:1137-1141 -> 1146-1150
examples/showcase-gpui/info/registry.rs:156  gpui-pre elements/div.rs:3148-3157 -> 3154-3163
examples/showcase-gpui/info/registry.rs:220  gpui-pre window.rs:3538-3571 -> 3586-3619
examples/showcase-gpui/info/typography.rs:234  gpui-component input/input.rs:776 -> 788
examples/showcase-gpui/info/typography.rs:237  gpui-base input/base/element.rs:2998 -> 3093
examples/showcase-gpui/info/typography.rs:242  gpui-base input/base/element.rs:2998 -> 3093
examples/showcase-gpui/info/typography.rs:245  gpui-base input/base/element.rs:2449 -> 2544
examples/showcase-gpui/info/typography.rs:246  gpui-component input/input.rs:694 -> 706
examples/showcase-gpui/info/typography.rs:258  gpui-component input/input.rs:551 -> 563
examples/showcase-gpui/info/typography.rs:264  gpui-component input/input.rs:550 -> 562
examples/showcase-gpui/info/typography.rs:270  gpui-component input/input.rs:547 -> 559
examples/showcase-gpui/info/typography.rs:273  gpui-base input/base/element.rs:2797 -> 2892
examples/showcase-gpui/info/typography.rs:278  gpui-component input/input.rs:546 -> 558
examples/showcase-gpui/support.rs:233  gpui-pre app.rs:2786-2796 -> 2924-2934
examples/showcase-gpui/support.rs:1161  gpui-component combobox.rs:749 -> 750
examples/showcase-gpui/tests.rs:3008  gpui-component button/button.rs:1283 -> 1295
examples/showcase-gpui/tests.rs:3677  gpui-component button/button.rs:660-662 -> 672-674
examples/showcase-gpui/tests.rs:4003  gpui-component button/button.rs:1025 -> 1037
examples/showcase-gpui/tests.rs:4658  gpui-component theme/mod.rs:465-470 -> 485-490
examples/showcase-gpui/tests.rs:4724  gpui-component button/button.rs:1322-1326 -> 1334-1338
examples/showcase-gpui/tests.rs:4858  gpui-component switch.rs:194-198 -> 196-200
examples/showcase-gpui/tests.rs:4859  gpui-component switch.rs:194 -> 196
examples/showcase-gpui/tests.rs:6266  gpui-component tag.rs:265 -> 269
examples/showcase-gpui/tests.rs:6381  gpui-component tag.rs:265 -> 269
src/base_layer.rs:6  gpui-component theme/mod.rs:383-437 -> 403-457
src/base_layer.rs:8  gpui-component theme/mod.rs:435 -> 455
src/base_layer.rs:41  gpui-component theme/mod.rs:399-419 -> 419-439
src/base_layer.rs:49  gpui-component theme/mod.rs:398 -> 418
src/base_layer.rs:57  gpui-component theme/mod.rs:420-429 -> 440-449
src/base_layer.rs:117  gpui-component theme/mod.rs:431-433 -> 451-453
src/colors.rs:190  gpui-component input/input.rs:550 -> 562
src/colors.rs:273  gpui-component button/button.rs:1170 -> 1182
src/colors.rs:273  gpui-component button/button.rs:1246 -> 1258
src/colors.rs:273  gpui-component button/button.rs:1288 -> 1300
src/colors.rs:273  gpui-component button/button.rs:1164 -> 1176
src/colors.rs:273  gpui-component button/button.rs:1241 -> 1253
src/colors.rs:273  gpui-component button/button.rs:1281 -> 1293
src/colors.rs:331  gpui-component button/button.rs:960-1025 -> 972-1037
src/colors.rs:332  gpui-component button/button.rs:630-635 -> 642-647
src/colors.rs:332  gpui-component button/button.rs:924-929 -> 936-941
src/colors.rs:415  gpui-component chart/mod.rs:419 -> 493
src/colors.rs:415  gpui-component chart/bar_chart.rs:857 -> 878
src/colors.rs:416  gpui-component chart/radar_chart.rs:485 -> 507
src/contract.rs:111  gpui-component input/input.rs:550 -> 562
src/contract.rs:127  gpui-component button/button.rs:1024 -> 1036
src/contract.rs:139  gpui-component button/button.rs:1170 -> 1182
src/contract.rs:139  gpui-component button/button.rs:1246 -> 1258
src/contract.rs:139  gpui-component button/button.rs:1288 -> 1300
src/contract.rs:140  gpui-component button/button.rs:1164 -> 1176
src/contract.rs:140  gpui-component button/button.rs:1241 -> 1253
src/contract.rs:140  gpui-component button/button.rs:1281 -> 1293
src/contract.rs:916  gpui-component button/button.rs:1164 -> 1176
src/contract.rs:917  gpui-component button/button.rs:1241 -> 1253
src/contract.rs:917  gpui-component button/button.rs:1281 -> 1293
src/contract.rs:1365  gpui-component menu/popup_menu.rs:1476 -> 1480
src/contract.rs:1366  gpui-component styled.rs:193-199 -> 186-192
src/contract.rs:1373  gpui-component menu/popup_menu.rs:1476 -> 1480
src/contract.rs:1710  gpui-component button/button.rs:1164 -> 1176
src/contract.rs:1710  gpui-component button/button.rs:1241 -> 1253
src/contract.rs:1710  gpui-component button/button.rs:1281 -> 1293
src/contract.rs:1773  gpui-component input/input.rs:550 -> 562
src/geometry.rs:37  gpui-component sizing.rs:244-245 -> 256-257
src/geometry.rs:38  gpui-component sizing.rs:269-272 -> 281-284
src/geometry.rs:71  gpui-component checkbox.rs:334-339 -> 331-336
src/geometry.rs:72  gpui-component combobox.rs:997 -> 998
src/geometry.rs:158  gpui-component button/button.rs:720 -> 732
src/geometry.rs:158  gpui-component input/input.rs:761 -> 773
src/geometry.rs:158  gpui-component button/button.rs:721 -> 733
src/geometry.rs:158  gpui-component input/input.rs:781 -> 793
src/geometry.rs:159  gpui-component select.rs:541-552 -> 542-553
src/geometry.rs:160  gpui-component combobox.rs:980-997 -> 981-998
src/geometry.rs:177  gpui-component button/button.rs:657-694 -> 669-706
src/geometry.rs:177  gpui-component button/button.rs:721 -> 733
src/geometry.rs:178  gpui-component button/button.rs:729-737 -> 741-749
src/geometry.rs:190  gpui-component button/button.rs:721 -> 733
src/geometry.rs:202  gpui-component input/input.rs:766-776 -> 778-788
src/geometry.rs:202  gpui-component input/input.rs:781 -> 793
src/geometry.rs:206  gpui-component input/input.rs:768-771 -> 780-783
src/geometry.rs:209  gpui-component input/input.rs:281-285 -> 292-296
src/geometry.rs:211  gpui-component input/input.rs:770 -> 782
src/geometry.rs:211  gpui-component input/input.rs:781 -> 793
src/geometry.rs:213  gpui-component input/input.rs:762-764 -> 774-776
src/geometry.rs:218  gpui-component input/input.rs:763 -> 775
src/geometry.rs:219  gpui-component input/input.rs:781 -> 793
src/geometry.rs:222  gpui-component input/input.rs:798 -> 810
src/geometry.rs:228  gpui-component input/input.rs:762-764 -> 774-776
src/geometry.rs:242  gpui-component input/input.rs:766-776 -> 778-788
src/geometry.rs:243  gpui-component input/input.rs:781 -> 793
src/geometry.rs:254  gpui-component input/input.rs:579-593 -> 591-605
src/geometry.rs:255  gpui-component input/input.rs:762-764 -> 774-776
src/geometry.rs:256  gpui-component input/textarea.rs:161-166 -> 172-177
src/geometry.rs:256  gpui-component input/textarea.rs:191 -> 205
src/geometry.rs:257  gpui-component input/textarea.rs:64 -> 74
src/geometry.rs:293  gpui-component menu/menu_item.rs:103-105 -> 100-102
src/geometry.rs:293  gpui-component menu/menu_item.rs:111 -> 108
src/geometry.rs:492  gpui-component button/button.rs:624-626 -> 636-638
src/geometry.rs:593  gpui-component checkbox.rs:286 -> 283
src/geometry.rs:598  gpui-component checkbox.rs:334-339 -> 331-336
src/geometry.rs:608  gpui-component radio.rs:226 -> 223
src/geometry.rs:613  gpui-component radio.rs:256-257 -> 253-254
src/geometry.rs:625  gpui-component sizing.rs:244-245 -> 256-257
src/geometry.rs:625  gpui-component sizing.rs:269-272 -> 281-284
src/geometry.rs:626  gpui-component select.rs:550 -> 551
src/geometry.rs:626  gpui-component select.rs:552 -> 553
src/geometry.rs:627  gpui-component combobox.rs:995 -> 996
src/geometry.rs:627  gpui-component combobox.rs:997 -> 998
src/geometry.rs:630  gpui-component select.rs:550 -> 551
src/geometry.rs:630  gpui-component combobox.rs:995 -> 996
src/geometry.rs:631  gpui-component select.rs:552 -> 553
src/geometry.rs:631  gpui-component combobox.rs:997 -> 998
src/geometry.rs:636  gpui-component select.rs:598-599 -> 599-600
src/geometry.rs:637  gpui-component combobox.rs:654-655 -> 655-656
src/geometry.rs:637  gpui-component select.rs:563-605 -> 564-606
src/geometry.rs:637  gpui-component combobox.rs:1008-1027 -> 1009-1028
src/geometry.rs:639  gpui-component select.rs:550 -> 551
src/geometry.rs:640  gpui-component combobox.rs:995 -> 996
src/geometry.rs:657  gpui-component select.rs:541-551 -> 542-552
src/geometry.rs:657  gpui-component select.rs:552 -> 553
src/geometry.rs:662  gpui-component select.rs:550 -> 551
src/geometry.rs:662  gpui-component select.rs:552 -> 553
src/geometry.rs:666  gpui-component select.rs:598-599 -> 599-600
src/geometry.rs:667  gpui-component select.rs:563-605 -> 564-606
src/geometry.rs:672  gpui-component select.rs:545 -> 546
src/geometry.rs:676  gpui-component select.rs:483-485 -> 484-486
src/geometry.rs:682  gpui-component combobox.rs:980-996 -> 981-997
src/geometry.rs:682  gpui-component combobox.rs:997 -> 998
src/geometry.rs:688  gpui-component combobox.rs:654-655 -> 655-656
src/geometry.rs:688  gpui-component combobox.rs:1008-1027 -> 1009-1028
src/geometry.rs:691  gpui-component combobox.rs:990 -> 991
src/geometry.rs:692  gpui-component combobox.rs:997 -> 998
src/geometry.rs:693  gpui-component combobox.rs:584-590 -> 585-591
src/geometry.rs:828  gpui-component input/input.rs:781 -> 793
src/geometry.rs:841  gpui-component input/input.rs:694-705 -> 706-717
src/geometry.rs:842  gpui-component input/input.rs:773 -> 785
src/geometry.rs:842  gpui-component input/input.rs:781 -> 793
src/geometry.rs:850  gpui-component theme/mod.rs:465-471 -> 485-491
src/geometry.rs:851  gpui-component input/input.rs:701-705 -> 713-717
src/geometry.rs:895  gpui-component button/button.rs:1304-1350 -> 1316-1362
src/geometry.rs:895  gpui-component button/button.rs:1315 -> 1327
src/geometry.rs:897  gpui-component button/button.rs:785-791 -> 797-803
src/geometry.rs:898  gpui-component button/button.rs:721 -> 733
src/geometry.rs:963  gpui-component theme/mod.rs:568-570 -> 588-590
src/geometry.rs:1570  gpui-component radio.rs:226 -> 223
src/geometry.rs:1571  gpui-component select.rs:545 -> 546
src/geometry.rs:1571  gpui-component radio.rs:256-257 -> 253-254
src/geometry.rs:1571  gpui-component select.rs:552 -> 553
src/geometry.rs:1573  gpui-component select.rs:483-485 -> 484-486
src/geometry.rs:1646  gpui-component checkbox.rs:334-339 -> 331-336
src/geometry.rs:1650  gpui-component combobox.rs:990 -> 991
src/geometry.rs:1650  gpui-component combobox.rs:997 -> 998
src/geometry.rs:1650  gpui-component combobox.rs:584-590 -> 585-591
src/geometry.rs:1681  gpui-component checkbox.rs:334-339 -> 331-336
src/geometry.rs:1690  gpui-component combobox.rs:990 -> 991
src/geometry.rs:1691  gpui-component combobox.rs:997 -> 998
src/geometry.rs:1697  gpui-component checkbox.rs:334-339 -> 331-336
src/geometry.rs:1705  gpui-component combobox.rs:990 -> 991
src/geometry.rs:1705  gpui-component combobox.rs:997 -> 998
src/icons.rs:898  gpui-pre platform.rs:3034-3037 -> 3298-3301
src/icons.rs:912  gpui-pre app.rs:2845-2855 -> 2983-2993
src/icons.rs:913  gpui-pre window.rs:4989-5000 -> 5037-5048
src/icons.rs:1239  gpui-pre platform.rs:3034-3037 -> 3298-3301
src/icons.rs:1264  gpui-pre platform.rs:2923-2937 -> 3187-3201
src/icons.rs:1264  gpui-pre platform.rs:3042 -> 3306
src/icons.rs:1272  gpui-pre app.rs:2845-2855 -> 2983-2993
src/icons.rs:1273  gpui-pre window.rs:4989-5000 -> 5037-5048
src/icons.rs:1951  gpui-pre platform.rs:2932-2935 -> 3196-3199
src/lib.rs:920  gpui-component theme/mod.rs:448-449 -> 468-469
src/lib.rs:921  gpui-component theme/mod.rs:314 -> 334
src/lib.rs:922  gpui-pre app.rs:1156 -> 1237
src/lib.rs:1003  gpui-component theme/mod.rs:313 -> 333
src/lib.rs:1004  gpui-component theme/mod.rs:453-458 -> 473-478
src/lib.rs:1029  gpui-pre app.rs:1770-1772 -> 1863-1865
src/lib.rs:1030  gpui-pre app.rs:1927-1931 -> 2020-2024
src/lib.rs:1033  gpui-pre app.rs:2192-2204 -> 2330-2342
src/showcase.rs:1820  gpui-component button/button.rs:1024-1025 -> 1036-1037
src/showcase.rs:1965  gpui-component theme/mod.rs:465-470 -> 485-490
src/variants.rs:6  gpui-component button/button.rs:1156-1163 -> 1168-1175
src/widgets/checkbox.rs:601  gpui-component radio.rs:242 -> 239
src/widgets/mod.rs:171  gpui-pre window.rs:4505-4513 -> 4553-4561
src/widgets/switch.rs:17  gpui-component switch.rs:236 -> 238
src/widgets/switch.rs:158  gpui-component switch.rs:205-214 -> 207-216
src/widgets/switch.rs:161  gpui-component switch.rs:269-272 -> 271-274
src/widgets/switch.rs:337  gpui-component switch.rs:290 -> 292
tests/seams.rs:545  gpui-component input/input.rs:786 -> 798
tests/seams.rs:600  gpui-component select.rs:541 -> 542
tests/seams.rs:600  gpui-component combobox.rs:986 -> 987
tests/seams.rs:606  gpui-component input/input.rs:763 -> 775
tests/seams.rs:607  gpui-component select.rs:550 -> 551
tests/seams.rs:607  gpui-component combobox.rs:995 -> 996
tests/seams.rs:607  gpui-component input/input.rs:781 -> 793
tests/seams.rs:607  gpui-component select.rs:552 -> 553
tests/seams.rs:607  gpui-component combobox.rs:997 -> 998
tests/seams.rs:717  gpui-component select.rs:563-605 -> 564-606
tests/seams.rs:717  gpui-component combobox.rs:1008-1027 -> 1009-1028
tests/seams.rs:732  gpui-component select.rs:598-599 -> 599-600
tests/seams.rs:733  gpui-component combobox.rs:654-655 -> 655-656
tests/seams.rs:733  gpui-component select.rs:563-605 -> 564-606
tests/seams.rs:733  gpui-component combobox.rs:1008-1027 -> 1009-1028
tests/seams.rs:801  gpui-component input/textarea.rs:205-208 -> 219-222
tests/seams.rs:816  gpui-component input/input.rs:762-764 -> 774-776
tests/seams.rs:816  gpui-component input/input.rs:781 -> 793
tests/seams.rs:817  gpui-component input/input.rs:798 -> 810
tests/seams.rs:866  gpui-component input/input.rs:762-764 -> 774-776
tests/seams.rs:873  gpui-component input/input.rs:781 -> 793
```

## Appendix B -- `docs/showcase-exceptions.toml` citations

Same method (the toolkit of each citation taken from the nearest toolkit name
before it on the line): 33 moved, 3 `CHANGED` — the three `input/input.rs:531-541`
entries, which are the 0.6.6 range already stale at 0.7.0 (§9.3): re-read them,
never shift them.

```text
docs/showcase-exceptions.toml:34  gpui-component select.rs:39 -> 40
docs/showcase-exceptions.toml:58  gpui-base questionnaire/state.rs:40-45 -> 48-53
docs/showcase-exceptions.toml:58  gpui-pre app.rs:2971 -> 3109
docs/showcase-exceptions.toml:170  gpui-pre window.rs:4725-4742 -> 4773-4790
docs/showcase-exceptions.toml:170  gpui-pre app.rs:1536 -> 1617
docs/showcase-exceptions.toml:428  gpui-pre window.rs:4725-4742 -> 4773-4790
docs/showcase-exceptions.toml:591  gpui-component input/textarea.rs:141-158 -> 152-169
docs/showcase-exceptions.toml:591  gpui-component input/input.rs:531-541 -> CHANGED(range body changed (lines inserted inside; 531->542, 541->553))
docs/showcase-exceptions.toml:644  gpui-component button/button.rs:670-673 -> 682-685
docs/showcase-exceptions.toml:644  gpui-component button/button.rs:1137 -> 1149
docs/showcase-exceptions.toml:644  gpui-component button/button.rs:1001 -> 1013
docs/showcase-exceptions.toml:644  gpui-pre elements/div.rs:3448 -> 3454
docs/showcase-exceptions.toml:644  gpui-pre elements/div.rs:3467 -> 3473
docs/showcase-exceptions.toml:752  gpui-pre window.rs:4513-4521 -> 4561-4569
docs/showcase-exceptions.toml:752  gpui-pre window.rs:4586-4589 -> 4634-4637
docs/showcase-exceptions.toml:766  gpui-component radio.rs:242 -> 239
docs/showcase-exceptions.toml:775  gpui-component radio.rs:242 -> 239
docs/showcase-exceptions.toml:832  gpui-base input/base/element.rs:1822-1826 -> 1896-1900
docs/showcase-exceptions.toml:838  gpui-base input/base/element.rs:1822-1826 -> 1896-1900
docs/showcase-exceptions.toml:844  gpui-base input/base/element.rs:1822-1826 -> 1896-1900
docs/showcase-exceptions.toml:861  gpui-base input/base/element.rs:1820-1821 -> 1894-1895
docs/showcase-exceptions.toml:867  gpui-pre window.rs:5815-5816 -> 5884-5885
docs/showcase-exceptions.toml:874  gpui-component input/textarea.rs:141-158 -> 152-169
docs/showcase-exceptions.toml:874  gpui-component input/input.rs:531-541 -> CHANGED(range body changed (lines inserted inside; 531->542, 541->553))
docs/showcase-exceptions.toml:882  gpui-component input/textarea.rs:141-158 -> 152-169
docs/showcase-exceptions.toml:882  gpui-component input/input.rs:531-541 -> CHANGED(range body changed (lines inserted inside; 531->542, 541->553))
docs/showcase-exceptions.toml:893  gpui-component select.rs:568-575 -> 569-576
docs/showcase-exceptions.toml:899  gpui-component select.rs:568-575 -> 569-576
docs/showcase-exceptions.toml:905  gpui-component select.rs:568-575 -> 569-576
docs/showcase-exceptions.toml:911  gpui-component select.rs:585-590 -> 586-591
docs/showcase-exceptions.toml:911  gpui-component select.rs:58-59 -> 59-60
docs/showcase-exceptions.toml:917  gpui-component select.rs:585-590 -> 586-591
docs/showcase-exceptions.toml:917  gpui-component select.rs:58-59 -> 59-60
docs/showcase-exceptions.toml:924  gpui-component select.rs:585-590 -> 586-591
docs/showcase-exceptions.toml:924  gpui-component select.rs:58-59 -> 59-60
docs/showcase-exceptions.toml:985  gpui-pre window.rs:4725-4742 -> 4773-4790
```
