# v0.7.2 (egui 0.37 and GPUI Kit 0.10.0): specification

Status: written 2026-11-17; reviewed once and checked once that day, revised after each, and read
by the plan's author. Approved by the maintainer on 2026-11-17; being built. It states what the two rationales decided, unit by unit; the
reasons are theirs.

The rationales are `docs/todo_v0.7.2_egui-0.37-rationale.md` (decisions E1 to E20; rows B1 to B12
and C1 to C14) and `docs/todo_v0.7.2_gpui-kit-0.10.0-rationale.md` (decisions T1 to T12; rows S1
to S12). The plan, `docs/todo_v0.7.2_egui-0.37-and-gpui-kit-0.10.0-plan.md`, orders the units and
holds their gates, reviews and sessions.

**Names.** `CE`, `CW`, `SE` are `connectors/native-theme-egui`, `connectors/native-theme-egui-widgets`
and `CE/examples/showcase-egui`; `CG`, `SG` are `connectors/native-theme-gpui` and
`CG/examples/showcase-gpui`. "The source test" is `CW/src/tests.rs`,
`the_painted_widgets_are_still_needed`. The rationales' `CN`, `SC` and "T7" are not used here: a
T-number is always the GPUI rationale's decision. `<record>` is the plan's record folder (the
plan's §1 gives its path; the brief gives it as an absolute path). `Z1` is `<record>/z1/`,
`~/.claude/projects/-home-tibi-Rust-native-theme/scratch-keep/todo_v0.7.2_egui-0.37-and-gpui-kit-0.10.0-plan/z1/`.

**Upstream sources** are named as the rationales name them: `E37`, `E36`, `P37` are the `src/`
folders of egui 0.37.0, egui 0.36.2 and epaint 0.37.0 in Cargo's registry
(`~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/`); `GC`, `GB` the `src/` folders of
gpui-component and gpui-base 0.10.0 there (0.9.1 where a line says so); any other crate by its
folder, such as `egui_extras-0.37.0` or `gpui-fast-0.1.16`. Upstream lines carry their version.
Project files are cited by path and item, never by line.

**How to read a section.** *Starts from*: the Z1 patch to apply and what replaces each interim
line it carries. *Files*: what the unit changes. *Changes*: numbered; each names the file and the
item, the result, and in brackets the decision it comes from. *Done when*: the tests and commands
that prove it. *Stops*: when the unit stops and reports. *Not here*: what another unit holds.

## Part A. What holds for every unit

- The brief is the unit's section and these rules. Spawn no sub-agents; long commands in the
  foreground, one report at the end; no search of the whole file system, nothing outside the named
  paths; no content a hook refused written another way; no `unsafe`, no panic-prone code, no code
  from LGPL sources.
- Tests: the named tests of the crate the unit changes, through `slot.sh` and `lowprio.sh` in the
  plan's work folder (`<work>/tools/`, plan §1; the brief gives their absolute paths). No whole
  suite, no gate: the gate is the orchestrator's.
- No interim code of a trial stays. `Z1/stand-in-check.sh` refuses a staged change that adds a
  line with `with_accessibility_check(false)`, `Box::leak`, `kinetic: _`, `unclamped_offset()`,
  `allow_missing_glyphs()`, `open_page_tofu`, `TrialRing`, `.widget_style::<`, or the word
  "trial" in any case; it reads source files only, not `docs/` and not Markdown. A name taken from
  a trial is renamed without that word.
- A test that fails because egui's defaults moved gets its decided form (E5, E6), never an edited
  number. The decided form: a test's expected value is read from egui or from the theme, never
  written as a number.
- A decision this section does not make is a stop: report it, do not choose.
- Citations. A new egui citation names the 0.37.0 file and a symbol, or a pinned `0.37.0` line; an
  egui-family line cited without a version before 0.7.2 is 0.36.2's and is left as it is; a
  statement whose content 0.37 changes is rewritten in the unit that changes that code [E11]. An
  upstream citation no gate reads is left as it is unless its statement changes [T11].
- In a GPUI unit, a change that moves a line of the connector's own files that a claim cites
  re-points that claim in the same commit.

## Part B. The units

### F1 — adwaita's check box

**Files:** `docs/platform-facts.md`, `native-theme/src/presets/adwaita.toml`, `docs/todo.md`.

**Changes:**
1. `docs/platform-facts.md`, the check-box table's `border.corner_radius` row, which the radio
   button shares: GNOME's cell states the check box's radius 6 (`$check_radius`, libadwaita 1.10.0
   `src/stylesheet/widgets/_checks.scss:1`, `:67-68`) and that a radio is round
   (`border-radius: 100%`, the same file, `:67-77` as `docs/todo_v0.8.0_model-completeness-gap-map-gnome.md`'s
   row GN-1.60 cites it); the other cells keep what they say about the radio. The row is edited in
   place; no line is added to the file. [E7]
2. `adwaita.toml`, `[light.checkbox.border]` and `[dark.checkbox.border]`: `corner_radius_px = 6.0`,
   with the citation comment in the form the preset gates check. [E7]
3. `docs/todo.md`, the GNOME entry: item (4) closed. [E7, E12]

**Done when:** the checkers and native-theme's tests that read the facts file and the presets
pass, and the orchestrator's gate runs every connector's tests (a core change).

### E-A — the egui move to a green tree (one commit, tasks A0, A1, A2)

Each task's sub-section stands alone with Part A. A0 and A1 are one agent, whose brief is this
whole section, since A0 has to tell the failures A1 and A2 fix from one no change names; A2's
agent gets A2's sub-section.

#### A0 — first, and reported once

**Starts from:** `Z1/E-A1.patch` and `Z1/E-A2.patch` applied; the three test fixes of
`Z1/E-A2-followup-fixes.txt` applied by hand (Q3 without its `println!` block); then
`cargo update -p egui --precise 0.37.0` and `cargo metadata --locked`. The floors and the lock stay
at 0.37.0 unless P2's record says a newer 0.37.x fixes something this move meets [decided here].
The interim lines the patches carry are replaced in A1 and A2.

**Changes:**
1. The lock: nothing moved by hand; skrifa stays 0.44.0, winit 0.30.13, muda and jiff as they
   are; no other upgrade enters (zbus 6, resvg, fontdb, windows-version, the lock-only updates).
   [E1, E2, E13]
2. `CE/Cargo.toml`: the dev-dependency `epaint_default_fonts = "0.37.0"` with its feature
   `monochrome_emoji_fonts`; the two monochrome emoji faces (`NOTO_EMOJI_REGULAR`, `EMOJI_ICON`)
   installed in `SE/tests.rs`'s harness as last fallbacks, in test setup only; then the showcase's
   tests run: what the 13 tests behind the glyph panics show is reported. [E9, E17]
3. A failing test of E6: a single-line field at rest in the «input» scope, drawn plain and drawn
   with `input_frame`, measured from `TextEdit::show` (`response.rect` against `text_clip_rect`):
   left, right, top. The bottom is asserted through the height: the larger of `interact_size.y`
   and top + the text's row + the expected bottom. Expected, with w = `input.border.line_width`:
   the accessor way, on a stated side: w + that side's padding. The plain way, on an axis with
   both sides stated: w + their mean (`to_button_padding`'s pair rule), so the two ways agree on
   every preset but windows-11, which states sides that differ and which the test names. On an
   unstated side the two ways agree, and the cell's `button_padding` component equals the base
   style's. Expected values pass through egui's own rounding
   (`Frame::apply_stroke_and_expansion_without_layout_shift`, `Frame::total_margin`), not a
   tolerance. Over the 17 presets. A0 writes it, runs it (it fails), reports what it measures,
   saves it as `<record>/e6-probe.patch` and takes it out of the tree, since E-A's gate would fail
   on it; E-B applies that patch by hand where A1 moved its context and makes it pass. [E6]
4. `CW/Cargo.toml`: the dev-dependency `egui = { version = "0.37.0", features = ["default_fonts"] }`
   at `CE`'s egui requirement, added before changes 5 and 6, so that the drop-down is laid out
   with a font face present. [E8, decided here]
5. The source test's two combo-box assertions ("let icon_size = Vec2::splat(ui.spacing().icon_width);",
   "let actual_height = galley.size().y.max(icon_size.y);") re-derived from 0.37's atom layout
   (`E37/containers/combo_box.rs:391`). [E3, C10]
6. The cause of `a_truncated_drop_down_keeps_to_its_room` (an untruncated combo stays 160 wide, 94
   tall): whether egui's own `ComboBox` wraps at that width in 0.37. [decided here, C10]
7. The MSRV checks the new lock reaches: the two egui crates at Rust 1.95.0 (the CI `msrv` job's
   form, `scripts/msrv_checks.sh`), and native-theme at 1.88.0 with smallvec 1.16.5. [egui
   rationale §1.7]

A0's agent goes on as A1; what A0 wrote stays, except change 3's test.

**Stops:** a failing test or compile error that no change of E-A's three tasks names; E6's field
that cannot be given the platform's padding by margin or frame alone; the source test's two
combo-box facts have no counterpart in 0.37's `combo_box.rs` [plan §6, E3].

#### A1 — `CE/src`, `CW/src`, the manifests, the canary

**Starts from:** A0's tree. Each interim line and what replaces it:
- `CE/src/install_tests.rs`, `fn bundled` and `t5_line_spacing_and_slider_expansion_at_factor_two`,
  `Box::leak`: change 9.
- `CE/src/style_diff.rs`, `fn scroll`, `kinetic: _` (twice): change 10.
- `CE/src/tripwires.rs`, `every_style_type_is_destructured_exhaustively`, `kinetic: _`: change 11.
- `CW/src/switch.rs`, `switch_ui`, the `.widget_style::<CheckboxStyle>` call: change 6.
- The patches' `egui::WidgetType::…` names (`DisclosureTriangle`, `CheckBox`, `Button`): `egui::Role::…`
  [E3 B2].

**Files:** `CE/Cargo.toml`, `CE/src/{atlas,fonts,icons,install_tests,plugin,roles,style_diff,tripwires}.rs`,
`CE/mapping.toml` (the unwritten list only), `CE/README.md` (the Compatibility table and the
`Id::new` example only), `CW/src/{combo_box,expander,links,parts,progress_bar,radio_button,spinner,switch,tests}.rs`
(`combo_box.rs` for its doc's `WidgetType::ComboBox` and where change 15 fixes the restated
layout; `progress_bar.rs` and `radio_button.rs` for their docs' `WidgetType::ProgressIndicator`
and `WidgetType::RadioButton`), `CW/Cargo.toml` (change 16), `.github/workflows/dependency-canary.yml`.

**Changes:**
1. `CE/Cargo.toml`: `egui = "0.37.0"`; dev-dependencies `eframe`, `egui_extras`, `egui_kittest`
   `"0.37.0"`, features unchanged; caret requirements. The `egui` dependency turns on no feature
   (no `default_fonts`, no `monochrome_emoji_fonts`). `epaint_default_fonts`' dev-dependency: A0
   added it. The comments that name 0.36.2 restated. [E1, E2, E8, E17]
2. `CE/README.md`, the Compatibility table: `egui`, `eframe`, `egui_extras`, `egui_kittest` at
   0.37.0. `epaint_default_fonts` is not listed there or in `CE/src/compat.rs`. [E12, decided here]
3. `dependency-canary.yml`: the comment on the caret requirement names `egui = "0.37.0"`. [E12]
4. `Id::new` replaced: `Id::unique` where the code needs a root `Id` (stored ids, `Context` data
   keys), `IdSalt::new` where egui asks a salt; in `CE/src/{atlas,icons,plugin,install_tests}.rs`,
   `CW/src/{links,parts,spinner,switch}.rs`, the doc comments that name `Id::new`
   (`CE/src/icons.rs`, `CE/src/plugin.rs`) and the README example
   `let id = egui::Id::new("name");`, which is a doctest. [E3 B1]
5. `WidgetType` replaced by `egui::Role` in `CE` and `CW`. `CW/src/switch.rs`: `Role::CheckBox` is
   the `WidgetInfo` role and the node's role is set to `Switch` afterwards, as today.
   `CW/src/expander.rs`: `Role::DisclosureTriangle`. [E3 B2, C4]
6. `CW/src/switch.rs`, `switch_ui`: `.frame(style.frame)`, the style call and the state lookup
   before it are removed; `Classes` comes from `egui::class` if still used. `WidgetAtom` for
   `AtomLayout`; the custom atom keyed by `IdSalt::new` from the same string; the fallback text
   colour set on the allocated atom's `container`. The published switch calls no doc-hidden egui
   function. [E15 (iv), E3 B3, B4]
7. `CE/src/fonts.rs`: bytes read with `FontData::bytes()`; a `FontBytes::Shared` face handed over
   as `FontData::from_blob(Arc::new(Arc::clone(bytes)), face.index)` (`from_blob(font: Blob,
   index: u32)`), not `from_owned(bytes.to_vec())`; the statement "epaint's only constructors
   are `FontData::from_static` … and `FontData::from_owned`"
   and "whose emoji fallbacks it keeps" corrected: the base is `FontDefinitions::default()` under
   the application's egui features, and egui 0.37 puts its emoji faces behind
   `monochrome_emoji_fonts` (`P37/text/font_data.rs:40`; `P37/text/font_definitions.rs:111-171`).
   [E3 B8, E8]
8. `CE/src/atlas.rs`: its fonts' statements corrected to the base of change 7; `install`'s doc:
   `Context::set_fonts` applies at once while no text has been laid out in the pass, and at the
   next pass after that. [E8, C7]
9. `CE/src/install_tests.rs`: `t6_fonts::bundled` and `t5_line_spacing_and_slider_expansion_at_factor_two`
   take `&'static [u8]` from `epaint_default_fonts`' constants (`epaint_default_fonts-0.37.0/src/lib.rs`):
   `HACK_REGULAR` (`:20`), `NOTO_EMOJI_REGULAR` (`:34`), `UBUNTU_LIGHT` (`:43`), `EMOJI_ICON`
   (`:57`), the two emoji constants behind `#[cfg(feature = "monochrome_emoji_fonts")]`; no copy,
   no leak, no `unwrap`. The tests keep exercising `FontBytes::Static`. [E17]
10. `CE/src/style_diff.rs`, `fn scroll`: walks `kinetic` into `spacing.scroll.kinetic.decay_time`,
    `.stop_distance` and `.rubber_band`; `CE/mapping.toml`'s unwritten list gives each the reason
    "input behaviour, no native leaf". [E18 (iii), E4 (a)]
11. `CE/src/tripwires.rs`, T1: binds `kinetic` and destructures
    `KineticScrollStyle { decay_time: _, stop_distance: _, rubber_band: _ }` below, field count 3,
    `E37/style.rs:851-875`; `ScrollStyle`'s count 17; the comments' existing citations left. [E19 (ii)]
12. `CE/src/roles.rs`: the rule "exactly one variant per container chrome in egui 0.36.2 …"
    restated for 0.37.0, with no new variant. [egui rationale §1.9]
13. `CE/src/icons.rs`, `custom_icon_to_image_loads_a_role`: the harness installs egui_extras' image
    loaders (from the patch). [E9]
14. The source test: the three slider assertions rewritten against `E37/widgets/slider.rs` and
    `E37/widgets/slider_core.rs` with the same facts (rail `inactive.bg_fill`, handle `bg_fill`,
    `slider_core.rs:382-441`; the visuals now an argument of the handle call, `slider.rs:571`); the
    two combo-box assertions re-derived from 0.37's atom layout (`E37/containers/combo_box.rs:391`),
    as A0 derived them; a new assertion on the check-box provider's
    `frame: Frame::new()` (`E37/theme/default_style.rs:182`). [E3, E15, C10, C11]
15. `a_truncated_drop_down_keeps_to_its_room`: if egui's own `ComboBox` wraps at that width, the
    widgets crate's drop-down follows egui and the test's premise is rewritten; if only the
    crate's restated layout wraps, that layout (`CW/src/combo_box.rs`) is fixed. [decided here, E3]
16. `CW/Cargo.toml`: the dev-dependency `egui = { version = "0.37.0", features = ["default_fonts"] }`
    (A0 added it), test-only; its requirement must equal `CE`'s, or a second egui copy would
    silently lose the feature; the manifest's comment that the crate names no egui version says
    so. [E8, decided here]
17. No deprecation left in `CE` or `CW`. [E3]

**Done when:** `CE`'s library tests with all and with no default features, among them
`icons::tests::custom_icon_to_image_loads_a_role`,
`t6_fonts::t6_d_the_chosen_faces_head_the_chains_at_the_asked_weight` and
`compat::the_readme_states_the_manifest_floors`; `CW`'s library tests, among them the source test
and `a_truncated_drop_down_keeps_to_its_room`; both crates' doc tests; clippy with `-D warnings`;
and, as targeted commands, the steps no trial ran: both crates' rustdoc, their feature
combinations (`scripts/check_features.sh`), the strict-panic check (`scripts/check_strict_panic.sh`).

**Stops:** the source test's two combo-box facts have no counterpart in 0.37's `combo_box.rs`
[decided here]; a twice-estimate.

#### A2 — `SE`

**Starts from:** the tree A0 and A1 left: `Z1/E-A1.patch`, `Z1/E-A2.patch` and the three test
fixes of `Z1/E-A2-followup-fixes.txt` applied, egui 0.37.0 in the lock and the manifests, `CE` and
`CW` moved. Each interim line and what replaces it:
- `SE/pages/basic.rs`, `scroll_bar`, `unclamped_offset()`: change 2.
- `SE/tests.rs`, `.with_accessibility_check(false)` in `open_at`, `the_area_stays_inside_the_page`,
  `the_side_panel_keeps_its_width_when_widget_info_scrolls`, `crossing_is_not_hovering`,
  `a_layout_dump_is_the_same_under_the_pointer`: removed; change 9.
- The patches' `egui::WidgetType::…` names (`DisclosureTriangle`, `CheckBox`, `Button`): `egui::Role::…`
  [E3 B2].

**Files:** `SE/{chrome,demo,info,tests}.rs`, `SE/pages/{basic,buttons,containers,inputs,overlays,range}.rs`
and the pages that hold an unnamed input, `docs/showcase-exceptions.toml` (one entry and the
`[egui]` comment).

**Changes:**
1. `SE/pages/basic.rs`, `button_label` and `checkbox_parts`: `Context::get_widget_style` with
   `StyleArgs` (from the patch); `Classes` from `egui::class`. [E15 (i), B3]
2. `SE/pages/basic.rs`, `scroll_bar`: `area.state.clamped_offset().y`. [E3 B5]
3. `SE/pages/basic.rs`, `list_row`: `egui::Role::Button` with the selected state. [E16 (i)]
4. `SE/info.rs`: `ui.scope_id()`. `SE/pages/basic.rs` (1) and `SE/pages/inputs.rs` (4):
   `align(Align2)` for `vertical_align`, with the same vertical centring. [E3 B6, B7]
5. `Id::new` replaced by `Id::unique` where the code needs a root `Id` (stored ids, `Context` data
   keys), by `IdSalt::new` where egui asks a salt; `WidgetType` by `egui::Role`; in `SE/{chrome,demo}.rs`,
   `SE/pages/{basic,containers,overlays}.rs`, and `SE/info.rs`'s four `WidgetType` uses (three
   `Label`, one `Button`). [E3 B1, B2]
6. `SE/pages/buttons.rs`: the "AtomLayout" section becomes `WidgetAtom` (`egui::WidgetAtom::new`),
   with its Widget Info; the module doc follows. [E10, E3 B4]
7. `SE/pages/range.rs`: a `RangeSlider` section beside the slider, with Widget Info: the rail
   (`widgets.inactive` fill and radius), the fill between the handles (`selection.bg_fill`), two
   handles (the state's `bg_fill`, `fg_stroke`, `expansion`), painted through egui's
   `slider_core::paint_handle` (`E37/widgets/slider_core.rs:382-441`), so it has the slider's
   resting-knob limitation, which it states. [E10]
8. `docs/showcase-exceptions.toml`: `Aligned` excepted: it places its contents and draws nothing of
   its own (`E37/containers/aligned.rs:40`). The comment above `[egui]` that says what the section
   lists ("The `Ui` layout methods of egui 0.36.2 no page of the egui showcase calls (the
   connector spec §10.4: "each places other widgets and paints nothing of its own")") is restated
   for egui 0.37.0, with `Aligned`, a container, in it. [E10]
9. `SE/tests.rs`: the five `Event::MouseWheel` take `source: egui::MouseWheelSource::Unknown`
   (`the_area_stays_inside_the_page`, `the_tables_keep_their_columns_at_the_largest_text_scale`
   twice, `a_narrow_colour_cell_is_cut_short_and_whole_on_hover`,
   `the_page_tabs_scroll_the_open_tab_into_view`); `every_rendered_character_is_in_the_installed_fonts`
   reads `data.bytes()`; Q1 (`the_icon_theme_drop_down_says_on_hover_why_the_system_theme_is_unavailable`:
   `harness.run_steps(1)` after the hover), Q2 (`the_innermost_hovered_target_wins`: column 0
   scrolled into view before the scene button's hover), Q3 (`every_widget_reports_itself`: a node
   of egui's own that no record contains is accepted where its centre lies in a record);
   `Role::DisclosureTriangle` added to `WIDGET_ROLES`; the harness's two monochrome emoji faces
   (`NOTO_EMOJI_REGULAR`, `EMOJI_ICON`, last fallbacks, test setup only): A0 added them; kittest's
   two checks stay on. [E3 B2, B8, B12, C4, C5, C12, C13, E9, decided here]
10. Every input the accessible-name check names gets an accessible name (seven inputs: a
    `ComboBox`, three `TextInput`s, a `MultilineTextInput`, a `Slider`, a `SpinButton`; and two
    more `ComboBox`es, pages not traced). No test adds `allow_missing_glyphs()`; one that seems to
    need it is a stop. The showcase keeps eframe's default `system_font_fallback`. [E9 (a), C5, C6]

**Done when:** `SE`'s tests with all, no and default features, among them the 13 that panicked on a
missing glyph, the 82 that failed the accessible-name check, `every_widget_reports_itself`, Q1,
Q2 and the five wheel tests; `scripts/check_widget_coverage.py` with egui 0 missing (`Aligned`,
`RangeSlider`, `WidgetAtom` met); clippy with no deprecation; the showcase's tests in the runner's
environment (`scripts/run_in_runner_env.sh`).

**Not here:** `CompletionPopup`, the submenu arrow's Widget Info and E5 to E7's Widget Info are
E-C's; the accessors' readers in `SE` are E-B's.

### E-B — the mapping

**Starts from:** E-A's commit; `<record>/e6-probe.patch`, E6's failing test, applied by hand where
E-A moved its context. The test measures a single-line field at rest in the «input» scope, drawn
plain and drawn with `input_frame`, over the 17 presets; this unit makes it pass.

**Files:** `CE/src/style/roles.rs`, `CE/src/accessors.rs`, `CE/src/convert.rs`, `CE/src/install_tests.rs`,
`CE/src/mapping.md`, `CE/mapping.toml`, `CE/README.md`, `SE/pages/inputs.rs`, `SE/pages/basic.rs`,
`SE/tests.rs`, `scripts/connector_parity_exceptions.toml`.

**Changes:**
1. `CE/src/style/roles.rs`, `fn input_role`: the «input» cell's `spacing.button_padding` is
   written as the other cells are: `to_button_padding(base.spacing.button_padding, &input.border)`.
   [E6 (a)]
2. `CE/src/accessors.rs`: `TEXT_EDIT_MARGIN` removed. `input_margin` and `text_area_margin`,
   signatures unchanged: a stated side is the side plus the line width, exactly per side, as in
   0.7.1; an unstated side is the «input» cell's component (the base style's: the button's, else
   `egui::Style::default()`'s), never a number written in the connector; an unstated `text_area`
   side takes `input_margin`'s. `input_frame` and `text_area_frame` re-derived from 0.37's
   `TextEditStyle`: the margin's stroke handling (a margin handed to `TextEdit::margin` loses the
   stroke's width, `E37/widgets/text_edit/builder.rs:693-704`), the frame's focus stroke, the min
   height `interact_size.y`. [E6, C3, decided here]
3. A `DragValue` in an «input» scope takes the field's padding from the scope: `SE/pages/basic.rs`,
   `number_input`, no longer sets `button_padding` by hand (it set the pair means of
   `input_margin`). The
   «input» scopes are searched for any other button and any collapsing header. [E6, decided here]
4. The accessors' readers, `SE/pages/inputs.rs` and `SE/pages/basic.rs` and their tests, follow
   changes 1 and 2. [E6]
5. Tests: on the 17 presets, each accessor's unstated sides equal the cell's component, and each
   stated pair's mean equals it. Two tests compute the set from the built styles and compare it
   with the names: the presets whose base `spacing.button_padding` equals
   `egui::Style::default()`'s are the 11 (the ten community presets and ios-13); those with a
   «menu», «list» or «toolbar» cell whose `interact_size.y` equals egui's are the 16 (all but
   macos-monterey). A different set is reported, not edited. [decided here, E5, E6]
6. `scripts/connector_parity_exceptions.toml`, the reason that says "a `TextEdit` paints its
   stroke inside its margin": restated with this unit's result. [E6]
7. `CE/src/mapping.md` and `CE/mapping.toml`: the rows of `input.border.padding` move from DERIVED
   to SCOPED. [E6]
8. Same files: egui 0.37's values stand where the theme states none (`button_padding` 8 × 2,
   `interact_size.y` 20, `E37/style.rs:1568`, `:1570`); every statement naming 0.36.2's value
   rewritten from 0.37.0, among them `CE/src/convert.rs`, `to_button_padding`'s doc, which names
   egui's old default ("`vec2(4.0, 1.0)`, `style.rs:1458`"): 8 × 2, `E37/style.rs:1568`. [E5 (a), C1]
9. `CE/src/mapping.md`: the check box's radius recorded as an egui limit above
   `round(0.3 × icon_width)` (`E37/theme/default_style.rs:194-197`), with the capped values: the
   ten community presets 8 → 5, ios-13 10 → 7, macos-monterey 5 → 4, macos-tahoe 6 → 5; adwaita
   (6 after F1), kde-breeze, windows-11 and material-3 under the cap. [E7 (a), C2]
10. `CE/src/mapping.md`, the `spacing.icon_width` row: the submenu arrow
    (`E37/containers/menu.rs:492-505`) recorded as a reader, so it is drawn at
    `checkbox.indicator_width`; `menu.icon_size` stays DERIVED. [E20 (b), C8]
11. `CE/README.md`, the role-styles sentence "A `TextEdit` in a `Role::Input` scope takes the": a
    plain single-line field takes one padding per axis, the mean of the two sides where a theme
    states different ones (windows-11); the accessors are the exact route; a multi-line field's
    route stays `text_area_margin` and `text_area_frame`. [E6]

**Done when:** the E6 test of *Starts from*, the tests of change 5, `CE`'s library tests, `SE`'s
tests that read the accessors.

**Stops:** a button other than a `DragValue`, or a collapsing header, in an «input» scope
[decided here; plan §6].

### E-C — the rest of the showcase and the egui documents

**Files:** `SE/pages/inputs.rs`, `SE/pages/overlays.rs`, the pages whose Widget Info E5 to E7
reach, `SE/tests.rs` (their tests), `docs/showcase-exceptions.toml`, `CE/src/lib.rs`, `CW/src/lib.rs`,
`CE/mapping.toml` (header), `CE/README.md`, `CW/README.md`.

**Changes:**
1. `SE/pages/inputs.rs`: `CompletionPopup` under a field, with Widget Info: a `Popup` with
   `Frame::popup` holding `Button::selectable` rows, the selection fill, the widget states
   (`E37/widgets/text_edit/completion.rs:169`, `:240`; `E37/containers/frame.rs:214`). [E10]
2. `SE/pages/overlays.rs`, the `SubMenuButton`'s Widget Info: one line naming the arrow's colour
   source (the button's text colour) and its size's source (`spacing.icon_width`, so
   `checkbox.indicator_width`). [E10, E20, C8]
3. Widget Info, stating what is drawn: where the theme states no padding side or row height,
   egui's 8 × 2 and 20, cited at `E37/style.rs:1568` and `:1570` [E5]; a single-line field in the input scope padded with the theme's
   field padding, one value per axis (the mean where a theme states different sides, windows-11),
   a field outside it with the button's padding or egui's 8 × 2 [E6]; the check box's
   radius capped at `round(0.3 × icon_width)` on 13 presets (the ten community presets, ios-13,
   macos-monterey, macos-tahoe) [E7].
4. `docs/showcase-exceptions.toml`: every existing parity reason that names 0.36.2's values (such as
   "egui's button_padding 4 x 1, egui-0.36.2/src/style.rs:1458") re-derived from the presets,
   not by swapping a number; material-3 states `button.border.padding` 24 and 10. [E5 (a)]
5. The sentence "an egui-family line cited without a version, written before 0.7.2, is egui
   0.36.2's" in four places: next to "Target: **egui 0.37.0**" in `CE/src/lib.rs` and
   `CW/src/lib.rs` (both "Target" lines move to 0.37.0), in `CE/mapping.toml`'s header, and in
   `docs/showcase-exceptions.toml`'s header for its egui citations, where it replaces "at the
   version Cargo.lock holds". [E11, E12]
6. `CE/README.md`: the caret paragraph for `egui = "0.37.0"`, `>=0.37.0, <0.38.0`, with the
   floor's reason for 0.37.0 (the only 0.37 release; an exact pin rejects patch fixes); the fonts'
   base chain: `FontDefinitions::default()` under the application's egui features, egui 0.37
   putting its emoji faces behind `monochrome_emoji_fonts` (an eframe application with default
   features still gets Ubuntu-Light and Hack); the submenu gap is egui's 2.0, with no theme value.
   [E1, E4, E8, E12]
7. `CW/README.md`: its egui 0.36.2 statements at 0.37.0. [E12]

**Done when:** `SE`'s tests; `CE`'s and `CW`'s library tests and doc tests;
`scripts/check_widget_coverage.py`; the documentation checkers on the changed files.

**Not here:** a parity difference that the asset run's captures find is K's; the README's
role-styles sentence on the «input» scope's padding is E-B's.

### G-A — the GPUI Kit move to a green tree

**Starts from:** `Z1/G-A.patch`; then `cargo update -p gpui-component --precise 0.10.0` and
`cargo metadata --locked`. The patch carries no interim line: its
`#[cfg(not(feature = "gpui-fast"))]` is the decided form. A newer GPUI Kit at P2 is the next
upgrade, not this one [decided here].

**Files:** `CG/Cargo.toml`, `Cargo.lock`, `CG/clippy.toml`, `CG/src/{colors,config,contract,lib}.rs`,
`CG/src/widgets/mod.rs` (the gate), `CG/README.md` (version rows; the three "139": "all 139
`ThemeColor` fields" in the opening list, "All 139 `ThemeColor` fields." under *What gets mapped*,
"139-field colour map" in the showcase's paragraph), `SG/main.rs` ("139-field"),
`SG/info/theme_map.rs`, `SG/tests.rs` (the fallback-stack test), the files that hold failing
citations.

**Changes:**
1. `CG/Cargo.toml`: gpui-component, gpui-base, gpui-kit (and the dev-dependency gpui-kit) at
   `0.10.0`; the comments that name 0.9.1, 0.1.14 and 0.1.15 re-read, the gpui-fast halves of the
   MSRV and `image` comments at 0.1.16; the sentence that the connector "follows an application
   that turns on `gpui-kit/gpui-fast`" becomes: an application on GPUI Fast turns on the
   connector's `gpui-fast` feature, which turns on the other two. The exact gpui-pre pin, the
   features' wiring, the docs.rs list and the `cold_path` reason stay. [T7, T12, §1.5 there]
2. The lock, by the two commands of *Starts from*: the packages it moves are recorded, not a stop;
   the stop is a resolution that fails or `cargo metadata --locked` failing. swash's move to
   skrifa 0.37.0 is left as cargo made it. [T12]
3. `CG/src/widgets/mod.rs`: `impl IntoElement for A11yDisabled` under
   `#[cfg(not(feature = "gpui-fast"))]` (`GB/a11y_disabled.rs:63`; `gpui-fast-0.1.16/src/element.rs:146`).
   No compile-time check of `gpui_kit::GPUI_FAST`. [T7 (a)]
4. `CG/src/colors.rs`: `list_highlight_foreground` takes the resting text colour, the value
   `foreground` takes; the tripwire counts 140. [T2 (b2)]
5. `CG/src/config.rs`: the key `list.highlight.foreground` exported, so the direct path and
   `Theme::change` give the same colour. [T2]
6. `CG/src/contract.rs`: the field's declared source is derived, the resting text colour. [T2]
7. `SG/info/theme_map.rs`: the field's token row. [T2]
8. `CG/src/lib.rs`, the crate doc: the mapping table gains the field. Every count derived from
   `ThemeColor`'s field count, found by search over `CG` and `SG`, is restated from the 0.10.0
   sources: the fields are 140 (in `CG/src/lib.rs` "all 139 colour fields", `apply`'s doc, the
   `link` row and a comment; `CG/src/colors.rs`; `CG/src/contract.rs`; the README's three and
   `SG/main.rs`'s of Files); `ThemeConfigColors` has one more than the fields (`CG/src/config.rs`,
   `theme_color_to_config_colors`'s doc: "`ThemeConfigColors` 140: the same 139 plus
   `group_box_title_foreground`" becomes 141 and 140); the exported keys gain the new one (the
   module doc's "127 of `ThemeColor`'s 139" becomes 128 of 140, the function doc's "127 are
   exported" 128); where a comment ties a count to a version of gpui-component (`CG/src/colors.rs`, the
   tripwire's comment; `CG/src/config.rs`, `theme_color_to_config_colors`'s doc), the version is
   0.10.0 and the lines it cites are re-read there. The unit re-derives each number from the
   sources and does not copy these where they differ; a difference is reported. [T2, S9, T10]
9. `CG/src/lib.rs`, `to_theme`'s doc: upstream's `Theme::sync_scrollbar_appearance` maps Windows by
   its own rule, auto-hide to `Hover`, otherwise `Always` (`GC/theme/mod.rs:145-154`, `:386-392`),
   while the connector maps `overlay_mode` to `Scrolling` on every platform. [S7, T9 (a)]
10. `CG/src/lib.rs`, `apply`'s and `forward_reduce_motion`'s docs: `ReduceMotionPreference` and
    `set_reduce_motion_preference` (`GB/reduce_motion.rs:85-127`) protect an explicit choice from
    gpui-base's readings; the connector's request still switches the flag on over an explicit
    `Allow`; an application that keeps motion sets the flag after `apply`. [S8, T8 (a)]
11. `CG/README.md`: the Compatibility table and the Quick start at 0.10.0 and GPUI Fast 0.1.16. [S11, T12]
12. The citations the two claim gates fail on, re-pointed by a script this unit writes over
    `Z1/linemap/`, run on this tree's own failing lists (v0.7.1's later units add citations; for
    an upstream file `Z1/linemap/` lacks, the map is made with `Z1/make_linemaps.py`); the changed line (`breadcrumb.rs:144`) and
    the two moved `checkbox.rs` ranges whose inside changed are read by hand. [T11 (a)]
13. `SG/tests.rs`, `the_fallback_stack_copy_is_gpuis`: cites gpui-fast 0.1.16, lines 262-273,
    unchanged in text (`Z1/other-citations.tsv`). [T11, T12]
14. `CG/clippy.toml`: the header and every reason line re-read at gpui-base 0.10.0 and gpui-fast
    0.1.16 (the new lines in `Z1/other-citations.tsv`; the `RopeExt` lines move by one). No
    `Label::new` entry. [T11, T12]

**Done when:** in the gate's three feature forms, the library's tests, among them
`showcase::every_colour_claim_is_read_at_the_line_it_cites`, `showcase::every_prose_citation_still_exists`,
`colors::tests::theme_color_field_count_tripwire`, `colors::tests::no_theme_color_field_is_left_at_default`,
`contract::every_theme_color_field_has_a_declared_source`,
`config::tests::theme_config_colors_export_every_key`,
`showcase::every_theme_color_field_has_one_theme_token`, `compat::the_readme_states_the_manifest_floors`,
`compat::the_quick_start_states_the_manifest_floors`; `SG`'s `tests::the_fallback_stack_copy_is_gpuis`;
the gate's three clippy forms for the crate (`scripts/check_release.sh`: "clippy
(native-theme-gpui, native, watch)", "clippy (native-theme-gpui, gpui-fast, native, watch)",
"clippy (native-theme-gpui, no features)"); the strict-panic check.

**Stops:** the GPUI-only lock does not resolve; a failing test that no change of this section
names; a twice-estimate [plan §6].

### G-B1 — the tab bar and the check box

**Starts from:** `Z1/G-B1.patch` (`tab_bar.rs`, the tab-bar tests in `widgets/tests.rs`). Its
interim lines: the opt-in goes (the `trial_reveal` field and the `trial_reveal_focus` builder
method, and the `reveal` parameter of the test helper `trial_tab_window`), and the focus probes
in `render` run unconditionally (change 2); the ring experiment goes
(`TrialRing` with `None`, `InsetBorder` and `InsetShadow`, the `trial_ring` field and builder
method, the ring match in `render`, the `ring` parameter of the test helper `trial_tab_window`),
change 1's inset shadow being the only ring; the tests `trial_q4a_focus_after_click_and_keys`,
`trial_q4b_focus_visible_ring`, `trial_q4c_the_focused_tab_scrolls_into_view` become change 7's
three tests, their prints replaced by assertions and their helpers renamed.

**Files:** `CG/src/widgets/tab_bar.rs`, `CG/src/widgets/checkbox.rs`, `CG/src/widgets/tests.rs`.

**Changes:**
1. `TabBar`: each tab takes `.focus_visible(…)`: an inset shadow in the theme's ring colour whose
   spread is the theme's ring width, drawn inside the tab, on keyboard focus only, not after a
   click. No border, no `FocusRing`. [T4 (b′)]
2. `TabBar`: the strip follows the focus: each tab has a one-pixel child with its own focus handle
   (not a tab stop) that answers whether the focus is within that tab; on a change the bar scrolls
   to it. It is always on; nothing is added to the public API. [T4]
3. `TabBar`'s doc: built on gpui-base's `Tabs` and `Tab`; a click activates a tab and never moves
   the focus; the tabs take the focus from the keyboard only (Tab enters on the selected tab, the
   arrows, Home and End move it); the ring as in change 1. [S6, T4]
4. `checkbox.rs`: `Checkbox::indeterminate(bool)`, on gpui-base's mixed state. The mixed mark is
   upstream's `Minus` icon, drawn as the check is drawn (upstream's `Check` icon, stroked at the
   theme's width where `CheckboxLook`'s `mark_stroke` states one), in the same colours, in the box
   as when checked. Indeterminate wins over `checked`, and activation reports `true`, as
   upstream's (`GC/checkbox.rs:256`, `GB/checkbox.rs:142`); a test asserts both. [T3 row 3]
5. `checkbox.rs`: where a doc of `Checkbox` or `Radio`, widgets on gpui-base's check box and
   radio, says a disabled control is unfocusable (found by search in `CG/src/widgets/`; today
   `Checkbox::disabled`'s "inert, unfocusable"), it says instead that a control focused when it
   becomes disabled keeps the focus so that Tab can leave it. [S12]
6. Right to left in both files: what names a side follows `LayoutDirection::global` (the tab bar's
   order and edges); per widget, one test under `LayoutDirection::Rtl.set_global` that asserts the
   mirrored bounds of the parts its left-to-right test asserts, failing before the change; what T6
   does not name (the arrow keys, the scroll's origin) as upstream's counterpart widget does at
   0.10.0: the arrow keys as gpui-base's `Tabs` (`GB/tabs.rs`, the arrow keys' step), the scroll's
   origin as gpui-component's tab bar (`GC/tab/tab_bar.rs`, the strip running from the right).
   [T6 (b), decided here]
7. `widgets/tests.rs`, the three tab-bar tests, under names without "trial", assert: [T4]
   - focus: a click on a tab activates it and leaves the keyboard focus where it was (from no
     focus, still none; from keyboard focus on tab 0, a click on tab 1 leaves the focus on tab 0);
     Tab enters the bar on the selected tab; the arrow keys, Home and End move the focus to the
     next, previous, first and last enabled tab.
   - ring: at rest no shadow on any tab; on keyboard focus exactly one inset shadow, on the
     focused tab, in the theme's ring colour with a spread equal to the theme's ring width; after
     a click none. It reads painted shadows and runs in the gate's GPUI Fast form only.
   - follow: with more tabs than fit, after the focus moves to a tab outside the strip, that tab's
     bounds lie inside the bar's; End brings the last tab in and Home the first.

**Done when:** change 7's focus and follow tests on both cores and its ring test in the gate's
GPUI Fast form, none of them passing on prints; change 6's right-to-left tests; `CG`'s widget
tests. For change 4 the unit adds, in the form of the existing check-box tests: the mixed state's
painted box and mark (the checked look, the `Minus` mark in the mark's colour, faded when
disabled) and its accessibility state as gpui-base reports it.

### G-B2 — the slider, switch, progress bar and widgets module

**Files:** `CG/src/widgets/{slider,switch,progress,mod}.rs`, `CG/src/widgets/tests.rs`.

**Changes:**
1. Right to left: the direction the slider, the progress bar and the switch run in, and whatever
   else in these files names a side, follows `LayoutDirection::global`; pixel geometry included;
   per widget, one test under `LayoutDirection::Rtl.set_global` that asserts the mirrored bounds
   of the parts its left-to-right test asserts, failing before the change; what T6 does not name
   (the slider's arrow keys among them) as upstream's counterpart widget does at 0.10.0: the
   slider as gpui-base's (`GB/slider.rs`, `offset_along` and `key_delta`: the value's start edge
   and the arrow keys), the switch and the progress bar as gpui-component's, whose mirroring is
   not in gpui-base (`GC/switch.rs`, the label's side; `GC/progress/progress.rs`, the fill from
   the start edge and the loading animation). [T6 (b), decided here]
2. `slider.rs`, `Slider`'s doc: no longer "Unlike gpui-component's, it takes keyboard focus". The
   behaviour stays v0.7.1's. [S4, T5 (a)]
3. `switch.rs`: where a doc of `Switch` (on gpui-base's switch) says a disabled control is
   unfocusable (none does today; the unit's search decides), it says instead that a control
   focused when it becomes disabled keeps the focus so that Tab can leave it. `slider.rs`'s
   "inert, unfocusable" is about the connector's own slider and stays, unless the unit's reading
   of that file shows it false, which is reported, not changed. [S12]
4. `mod.rs`, `FocusRing`'s doc: gpui-component's widgets draw a 3 px ring at half the colour's
   alpha (`focus_ring_style`), and the `Tab` now draws another, 2 px inset at the full colour, on
   keyboard focus only (`GC/tab/tab.rs:933`, `:962-973`). [S5]
5. `mod.rs`, `A11yDisabled`'s doc: gpui-base's copy now gates its `IntoElement` on the core
   (`GB/a11y_disabled.rs:63-70`). [S10, T7]
6. The capture: the builder supplies only the one-line uncommitted change, in `SG/main.rs`'s
   `fn main` (the showcase's start-up, where G-C later sets the left-to-right pin), that sets the layout
   direction to right to left, as a patch in `<record>`; the orchestrator builds the gpui
   showcase, starts it once with that change, captures it with the showcase's own capture path
   under `assets.lock` alone after the maintainer is asked for the window, and reads the image.
   [T6, decided here]

**Done when:** change 1's right-to-left tests, `CG`'s widget tests (16 physical-edge calls in
`widgets/tests.rs` may move), the change's patch in `<record>`.

### G-C — the gpui showcase and README

**Starts from:** `Z1/G-C-tooltip-test.patch` (`CG/tests/seams.rs`). Its `trial_*` functions
(`trial_q5_tooltip_two_ways` and its three helpers, `trial_tooltip_content`,
`trial_tooltip_max_w`, `trial_bubble`) and `TRIAL_MARKER` are kept under names without "trial",
as the test that holds S3; its prints become change 6's assertions.

**Files:** `SG/{demo,chrome,tests}.rs`, `SG/pages/buttons.rs`, `SG/info/{buttons,feedback,leaves}.rs`
and the info files of the widgets below, `SG/main.rs` (`fn main`, the start-up),
`CG/src/geometry.rs`, `CG/tests/seams.rs`, `CG/README.md`, the files of the `Label::new` sites.

**Changes:**
1. `SG/demo.rs`: a searchable `Select` with `highlight_matches(true)`, labelled as such; a test types
   a query and asserts the matched run's colour. Its Widget Info: the colour
   (`list_highlight_foreground`, the resting text colour), its source, and the limit: no one colour
   reads in the three row states on every preset; this one never falls below 1.5 at rest or hovered
   and does on the selected row in 7 of 34 cases, against 20 for `info`, 10 of them invisible. The
   preset Combobox and the Command palette (`SG/chrome.rs`) keep the option off; their Widget Info
   names the option and says so. [T2, T3 row 2]
2. Two mixed check boxes, each with Widget Info for the fill, the mark and its colours: upstream's
   (border `primary`, fill `tokens.primary`, `IconName::Minus` through the icon resolver in
   `primary_foreground`, `GC/checkbox.rs:230-250`, `:284-290`, `:354-370`) and the connector's
   `widgets::Checkbox` with `indeterminate(true)`. [T3 row 3]
3. Two loading buttons: one with the demo icon, one without, whose `loading_icon` is the chosen
   icon theme's loading icon; where that theme has none, as the button with an icon does today; its
   Widget Info names the icon drawn. `SG/info/buttons.rs`'s Loading arms and `SG/pages/buttons.rs`'s
   comment no longer say only a button with an icon shows the spinner (`GC/button/button.rs:777-786`).
   [T3 row 6, S2, T10]
4. One `Clipboard` with a label (`GC/clipboard.rs:52`), with a Widget Info line: text in the ghost
   button's colours. [T3 row 7]
5. Widget Info lines: upstream's `Tab` ring (`ring`, 2 px inset, a literal the theme does not
   reach); the connector's `TabBar` ring (theme ring colour and width, inside the tab; the theme's
   ring offset not reached, since the scrolling row would cut a band outside); the `Slider`'s ring
   (`ring`, `GC/slider.rs:63`, `:228`, `:233-238`); the tooltip's size change between tooltips,
   200ms, a literal (`GC/tooltip.rs:164`, `:219-243`), and whether it honours the reduce-motion
   flag, which the unit reads. [T3 rows 4, 5, 8, T4]
6. The tooltip. The two ways: `Tooltip::new(text)` with the theme's maximum width on the bubble,
   and the `tooltip_content` way. The kept test asserts that for a long one-line text without a
   bound key, on kde-breeze, windows-11 and adwaita, the bubble's bounds are equal on the two
   ways; no `println!` remains. A tooltip with a bound key: the unit measures it first, both ways,
   on the same three presets; a difference on any one is "differ". Where the two give equal
   bubbles, the test asserts that equality and `SG/demo.rs`'s `action_tooltip` takes `Tooltip::new(text)`; where they differ, the test asserts that the
   `tooltip_content` way's bubble is no wider than the theme's maximum width, `action_tooltip`
   keeps `tooltip_content` for a tooltip with a key, and the docs and the "why built" note state
   the measured difference. Either way `CG/src/geometry.rs`'s `tooltip` and `tooltip_content` docs
   say the conversion is no longer forced for a tooltip without a key
   (`GC/tooltip.rs:134-144`); `tooltip_content` stays public; `SG/info/feedback.rs`'s tooltip
   "why built" note is corrected. [S3, T10]
7. `SG/info/feedback.rs`: `TAG_HOVER_OPACITY` and the Tag's hover note go; `SG/tests.rs`,
   `a_tags_painted_fill_is_its_bg_claim` and `a_hovered_tags_info_names_its_painted_fill` lose
   their hover reasons and keep asserting the fill. [S1, T10]
8. `SG/info/leaves.rs`: gpui-component's widgets draw a 3 px ring at half the colour's alpha
   (`focus_ring_style`), and the `Tab` draws another, 2 px inset at the full colour, on keyboard
   focus only (`GC/tab/tab.rs:933`, `:962-973`). [S5]
9. Each drawing change upstream left not determined is checked against its widget's Widget Info:
   the Markdown sample under 0.10.0's white-space folding, the Select's popup while searching, the
   textarea's rounding to a device pixel, the plot labels' weight. [GPUI rationale §5]
10. `SG/main.rs`, `fn main`: the showcase sets the layout direction to left to right at start,
    with a comment that its own layout is written for that direction. [T6 (b)]
11. The 36 lines with `Label::new` in `CG/src` and `CG/examples` read; a site that builds sibling
    labels from one call gives each an id. [T12]
12. `CG/README.md`: *Backends*: an application on GPUI Fast turns on the connector's `gpui-fast`
    feature, which turns on the other two; the case that "needs nothing more" goes; GPUI Fast 0.1.16
    [T7, S11]. *Keyboard focus*: the `Tab`'s ring and the `TabBar`'s, the tabs taking the focus from
    the keyboard only [S5, S6, T4]. *Assistive technology*: what upstream's slider has from 0.10.0
    that the theme-drawn one has not: Page Up and Page Down, a numeric `SetValue`, `Release` after a
    key's `Change` [S4, T5]. The match highlight's colour, source and limit, and
    `SearchableListItem::render_highlighted` as the way to draw a match that keeps the row's text
    colour [T2]. `init`'s `scrollbar_mode` sentence: upstream's `Theme::sync_scrollbar_appearance`
    maps Windows by its own rule, auto-hide to `Hover`, otherwise `Always`
    (`GC/theme/mod.rs:145-154`, `:386-392`), while the connector maps `overlay_mode` to
    `Scrolling` on every platform [S7, T9].

**Done when:** the highlight test; `SG`'s tests; the tooltip test of change 6; the coverage gate
with gpui 0 missing.

**Stops:** none of its own beyond Part A's.

### D — the root documents

**Files:** `CHANGELOG.md`, `ROADMAP.md`, `docs/todo.md`, `docs/todo_v0.7.1_batch-speed-rationale.md`.

**Changes:**
1. `CHANGELOG.md`, the 0.7.2 entry, no migration section. *Changed*: native-theme-egui and
   native-theme-egui-widgets require egui, eframe, egui_extras and egui_kittest 0.37.0 (MSRV
   unchanged, 1.95.0), in the form of 0.7.1's GPUI Kit line, with one sentence that an application
   on egui 0.36 stays with `native-theme-egui = "=0.7.1"`; egui's check box draws its radius capped
   on 13 presets; egui's own padding (4 × 1 to 8 × 2 on 11 presets) and row floor (18 to 20 on 16)
   where a theme states none; a text field outside the input scope takes the button's padding where
   the theme states one (24 × 10 on material-3), egui's 8 × 2 elsewhere; the submenu arrow drawn at
   the check box's indicator width; the fonts' base chain. native-theme-gpui requires
   gpui-component, gpui-base and gpui-kit 0.10.0, and an application on GPUI Kit 0.9.1 stays on
   `native-theme-gpui = "=0.7.1"`; GPUI Fast needs the connector's `gpui-fast` feature, and the
   configuration v0.7.1 documented fails with E0119; the tab bar's focus ring on keyboard focus,
   tabs taking the focus from the keyboard only, the strip following the focus; the connector's
   widgets follow the layout direction. *Fixed*: a single-line text field in the input scope
   takes the theme's field padding (one value per axis; where a theme states different sides,
   as windows-11 does, their mean; the accessors give the exact sides); adwaita's check box has libadwaita's own radius, 6, in every
   connector. *Added*: the egui showcase's new sections; the mapped colour with its limit;
   `Checkbox::indeterminate` on the theme-drawn check box; the gpui showcase's new parts.
   [E5, E6, E7, E8, E10, E12, E20, T2, T3, T4, T6, T7, T12]
2. `ROADMAP.md`: "Targets egui 0.36.2" becomes 0.37.0. [E12]
3. `docs/todo.md`: the 2026-11-14 entry's egui item closed and its skrifa and muda sentence
   corrected (muda is not eframe's dependency; skrifa stays at epaint's 0.44.0) [E2, E12]. New
   entries: the rubber band and `submenu_gap` facts [E4]; macOS's check-box radius from the probe's
   renders, the check-box radius rule in `docs/inheritance-rules.toml`, KDE's 4 drawn inside the
   cell (K1.53) with the KDE items [E7]; a range counterpart in the widgets crate [E10]; the platform
   facts for a submenu arrow's size and a model leaf [E20]; the platform facts for a search match's
   emphasis and a model field [T2 (e)]; the desktops' own mixed mark [T3]; the slider's parity with
   upstream's keys and `SetValue` [T5 (b)]; the showcase's direction switch, about a hundred counted
   sites, 1.5 to 2.5 days, not measured [T6 (c)]; Windows' scroll-bar reveal from Microsoft's
   documentation [T9]; the claim gates finding a line by content [T11 (c)]. Updated: issue #4's
   fourth request (the `Tab` now rings on keyboard focus only, `focus_ring_style` does not); the
   entry on gpui-component's own icons gains `Loader` on a loading button, the mixed check box's
   `Minus`, `ChevronLeft` right to left; the public way to mark a node disabled gains T7 as a second
   reason; the GPUI Fast entries re-read at 0.1.16 [§1.5 there, T7, T12]. `[upstream]` drafts, not
   sent: egui's cap made optional [E7 (d)]; a highlight that keeps the row's text colour, or a
   colour for the selected row [T2]; a focus callback on `Tabs` [T4]; documenting
   `gpui_kit::GPUI_FAST` [T7]; a reduce-motion getter [T8]. [decided here]
4. The batch rationale: rule 19's accepted set and rule 25's example gain GPUI Kit 0.10.x beside
   egui 0.37.x. [plan, D's row]

**Done when:** the five documentation checkers (`scripts/check_release.sh`, "Data documents") pass
on the merged tree, and the CHANGELOG's first `## [` heading is 0.7.2's.

**Not here:** the GNOME entry's item (4) is F1's.

### R — the parity comparison without the desktop

**Changes:**
1. Each difference the layout half finds is a fix by the unit's author, or an exception with its
   reason and source; the new groups from egui's moved defaults [E5] and egui's capped check box on
   the 13 presets [E7] among them.
2. The checker's `--merge` keeps existing reasons; only the groups it adds carry 0.37.0 citations. [E11]

### V and K

- V: the workspace version is 0.7.2 for every crate. [E12]
- K: one asset run covers the three showcases (the lock stales every stamp); it runs
  `scripts/update_compatibility.sh run`, which writes the egui and gpui READMEs' "Verified against"
  lines; the CHANGELOG's date follows it. [E12, T12]

## Recorded, nothing to build

- E14: the trial build, done.
- E10's list, recorded and not shown: `ContainerAtom`, `CollapsingHeader::custom`,
  `MenuConfig::submenu_gap` and the rubber band, the new `Slider` and `DragValue` options,
  `accessible_name` and `Area::role`, `SvgGlyph`, `TokenType`, the `egui-icons` face.
- B11 compiles unchanged; C9: no change beyond A1's restated rule; C14: no change.
- T1: the release's scope by kind. T3 rows 10, 11, 12: recorded, not shown. T5, T8, T9: the
  behaviour stays v0.7.1's (only the statements change). T11 (b) refused.
- The GPUI rationale §1.3: `EditorState::text_gutter_margin()` is in neither version's sources.
- The egui citations in `docs/todo.md` are left to drift [E11].

## Where each decision is built

| Decision (its rows) | Units |
|---|---|
| E1 | A0, A1, E-C |
| E2 | A0, A1, D |
| E3 (B1–B10, B12; C4, C10–C13) | A0, A1, A2 |
| E4 | A1, E-C, D |
| E5 (C1) | E-B, E-C, D, R |
| E6 (C3) | A0, E-B, E-C, D |
| E7 (C2) | F1, E-B, E-C, D, R |
| E8 (C7) | A0, A1, E-C, D |
| E9 (C5, C6) | A0, A1, A2 |
| E10 | A2, E-C, D |
| E11 | Part A, E-C, R |
| E12 | F1, A1, E-C, D, V, K |
| E13 | A0 |
| E14 | recorded |
| B11, C9, C14 | recorded |
| E15 | A1, A2 |
| E16 | A2 |
| E17 | A0, A1 |
| E18 | A1 |
| E19 | A1 |
| E20 (C8) | E-B, E-C, D |
| E10's list | recorded |
| T1 | recorded |
| T2 | G-A, G-C, D |
| T3 | G-B1, G-C, D |
| T3 rows 10–12 | recorded |
| T4 (S6) | G-B1, G-C, D |
| T5 (S4) | G-B2, G-C, D |
| T6 | G-B1, G-B2, G-C, D |
| T7 (S10) | G-A, G-B2, G-C, D |
| T8 (S8) | G-A, D |
| T9 (S7) | G-A, G-C, D |
| T10 (S1, S2, S3) | G-C |
| T11 | Part A, G-A, D |
| T12 (S11) | G-A, G-C, D, K |
| S5 | G-B2, G-C |
| S9 | G-A |
| S12 | G-B1, G-B2 |
| Decided here | A0, A1, A2, E-B, G-A, G-B1, G-B2, D |
