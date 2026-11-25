//! `RoleVariant::Selected`, `RoleVariant::Disabled` and `RoleVariant::SelectedDisabled` cells
//! (spec §6.2, §6.3).
//!
//! Each cell starts from its role's `Normal` cell, a `SelectedDisabled` one from its `Selected`
//! cell, and writes only the variant's leaves (§3.4). A role with no variant data returns
//! `None`, and `compile` then shares the `Normal` `Arc`, for `SelectedDisabled` the `Disabled`
//! one.
//!
//! The cells read a [`VariantSource`]: [`VariantSource::old`] fills it from the 0.7.2 model,
//! `next::build::variant_source` from the 0.8.0 model's homes, each value `None` where the
//! theme states none for the state, and the cell then keeps its `Normal` cell's value there
//! (spec §10, rule 0): egui's own checked box is its unchecked one with the mark
//! (`egui/src/widget_style.rs:182-188`), and egui dims a disabled widget by `disabled_alpha`
//! alone (egui 0.37.0 `src/ui.rs:523-529`, `Ui::disable`).

use egui::style::WidgetVisuals;

use super::base::{Read, length, stroke};
use super::line::{Line, LineAlign};
use super::roles::CellInput;
use super::states::{
    FillField, FillSource, Layer, OpenFrom, StateLine, StateSource, TextSource, Unstated,
    state_borders, write_states_with,
};
use crate::convert::{Rgba, to_color32};
use crate::style::note_transparent_fill;
use crate::{Note, ResolvedTheme, Role};

/// A role's disabled look: its label's or text's colour, its fill with the place a note names
/// it by, and the role's resting outline that fill is drawn under, with where it lies.
pub(crate) struct Disabled {
    pub text: Option<Rgba>,
    pub fill: Read<Rgba>,
    pub outline: Option<Rgba>,
    pub align: LineAlign,
    /// The outline a rule states for the disabled state, and where it lies: the check box's;
    /// `None` where none does, and the resting outline stays.
    pub border: Option<(StateLine, LineAlign)>,
}

/// A box a state flag marks, the checked one or the mixed one: its fill, outline width and
/// outline colour at rest, where its stroke lies, its fill and outline under the pointer and
/// pressed, and its mark's width at rest, under the pointer and pressed.
pub(crate) struct MarkedBox {
    pub fill: Read<Rgba>,
    pub width: Read<f32>,
    pub outline: Option<Rgba>,
    pub align: LineAlign,
    /// The hovered and the pressed box's fill.
    pub state_fill: [Read<Rgba>; 2],
    /// The hovered and the pressed box's outline.
    pub state_border: [Option<StateLine>; 2],
    /// The resting, the hovered and the pressed box's mark width.
    pub mark_width: [Read<f32>; 3],
}

/// The checked box.
pub(crate) struct CheckedBox {
    pub look: MarkedBox,
    /// The checked box's fill where the theme states a disabled fill.
    pub disabled_fill: Read<Rgba>,
    /// The checked box's mark where the theme states a disabled mark.
    pub disabled_mark: Option<Rgba>,
    /// The checked box's outline, and where it lies, where the theme states a disabled outline.
    pub disabled_border: Option<(StateLine, LineAlign)>,
}

/// The primary button's disabled look: its fill where the theme states one, and the label
/// that fill is lettered in.
pub(crate) struct PrimaryDisabled {
    pub fill: Option<Rgba>,
    pub text: Option<Rgba>,
}

/// Every variant cell's reads.
pub(crate) struct VariantSource {
    pub button: Disabled,
    pub primary: PrimaryDisabled,
    pub combo_box: Disabled,
    pub checkbox: Disabled,
    pub checked: CheckedBox,
    pub input: Disabled,
    pub slider_track: Read<Rgba>,
    pub slider_fill: Option<Rgba>,
    /// The unchecked and the checked switch's track.
    pub switch: [Option<Rgba>; 2],
    pub menu: Option<Rgba>,
    pub list: Option<Rgba>,
    pub link: Option<Rgba>,
}

impl VariantSource {
    /// The 0.7.2 model's reads: a disabled fill the model leaves `None` is the role's own
    /// (§6.3), and every outline lies `Outside`, drawn as given.
    pub(crate) fn old(t: &ResolvedTheme) -> Self {
        let disabled = |text, fill| Disabled {
            text: Some(text),
            fill: Some(fill),
            outline: None,
            align: LineAlign::Outside,
            border: None,
        };
        let k = &t.checkbox;
        Self {
            button: disabled(
                t.button.disabled_text_color,
                (
                    t.button
                        .disabled_background
                        .unwrap_or(t.button.background_color),
                    "button.disabled_background",
                ),
            ),
            primary: PrimaryDisabled {
                fill: t.button.disabled_background,
                text: Some(t.button.disabled_text_color),
            },
            combo_box: disabled(
                t.combo_box.disabled_text_color,
                (
                    t.combo_box
                        .disabled_background
                        .unwrap_or(t.combo_box.background_color),
                    "combo_box.disabled_background",
                ),
            ),
            checkbox: disabled(
                k.disabled_text_color,
                match (k.disabled_background, k.unchecked_background) {
                    (Some(c), _) => (c, "checkbox.disabled_background"),
                    (None, Some(c)) => (c, "checkbox.unchecked_background"),
                    (None, None) => (k.background_color, "checkbox.background_color"),
                },
            ),
            checked: CheckedBox {
                look: MarkedBox {
                    fill: Some((k.checked_background, "checkbox.checked_background")),
                    width: None,
                    outline: Some(k.border.color),
                    align: LineAlign::Outside,
                    state_fill: [None; 2],
                    state_border: [StateLine::colour(k.hover_border_color); 2],
                    mark_width: [None; 3],
                },
                disabled_fill: k
                    .disabled_background
                    .map(|c| (c, "checkbox.disabled_background")),
                disabled_mark: None,
                disabled_border: None,
            },
            input: disabled(
                t.input.disabled_text_color,
                (
                    t.input
                        .disabled_background
                        .unwrap_or(t.input.background_color),
                    "input.disabled_background",
                ),
            ),
            slider_track: Some(match t.slider.disabled_track_color {
                Some(c) => (c, "slider.disabled_track_color"),
                None => (t.slider.track_color, "slider.track_color"),
            }),
            slider_fill: Some(t.slider.disabled_fill_color.unwrap_or(t.slider.fill_color)),
            switch: [
                Some(
                    t.switch
                        .disabled_unchecked_background
                        .unwrap_or(t.switch.unchecked_background),
                ),
                Some(
                    t.switch
                        .disabled_checked_background
                        .unwrap_or(t.switch.checked_background),
                ),
            ],
            menu: Some(t.menu.disabled_text_color),
            list: Some(t.list.disabled_text_color),
            link: Some(t.link.disabled_text_color),
        }
    }
}

/// `entry`'s outline, `outline` where it lies over the fill the entry paints (`painted`), drawn
/// over that fill ([`Line::over`]); an outline that lies `Outside` or none stays as written.
fn outline_over(
    entry: &mut WidgetVisuals,
    outline: Option<Rgba>,
    align: LineAlign,
    painted: FillField,
) {
    if let Some(color) = outline {
        let fill = match painted {
            FillField::Bg => entry.bg_fill,
            FillField::Weak => entry.weak_bg_fill,
        };
        entry.bg_stroke.color = Line {
            color: to_color32(color),
            align,
        }
        .over(Some(fill));
    }
}

/// `entry`'s outline where a rule states one for the disabled state (`line`, lying at `align`):
/// its width over the entry's ([`stroke`]) and its colour drawn over the fill the entry paints
/// (`bg_fill`, [`Line::over`]), each where stated.
fn stated_outline(
    entry: &mut WidgetVisuals,
    (line, align): (StateLine, LineAlign),
    notes: &mut Vec<Note>,
) {
    if let Some((width, path)) = line.width {
        entry.bg_stroke = stroke(path, entry.bg_stroke, entry.bg_stroke.color, width, notes);
    }
    if let Some(color) = line.color {
        entry.bg_stroke.color = Line {
            color: to_color32(color),
            align,
        }
        .over(Some(entry.bg_fill));
    }
}

/// A fill read as egui's colour, reported where its alpha is `0` (§6.4).
fn noted(read: Read<Rgba>, notes: &mut Vec<Note>) -> Option<egui::Color32> {
    read.map(|(c, path)| {
        let c = to_color32(c);
        note_transparent_fill(c, path, notes);
        c
    })
}

/// A disabled label's colour, where stated, in the entries a disabled widget's and a plain
/// label's text are painted from: `inactive` and `noninteractive`.
fn labels(w: &mut egui::style::Widgets, text: Option<Rgba>) {
    if let Some(text) = text.map(to_color32) {
        w.inactive.fg_stroke.color = text;
        w.noninteractive.fg_stroke.color = text;
    }
}

/// The `Selected` cell of `role`, or `None` where the role's selected look lives in its
/// `Normal` cell (`Button`, `Tab`, `SegmentedControl`, `Switch`, §6.2) or the role has none.
pub(crate) fn selected_cell(
    role: Role,
    normal: &egui::Style,
    src: &VariantSource,
    input: &CellInput<'_>,
    notes: &mut Vec<Note>,
) -> Option<egui::Style> {
    if role != Role::Checkbox {
        return None;
    }
    let mut style = normal.clone();
    // The checked box: `checkbox_style` fills from the state's `bg_fill`
    // (`egui/src/widget_style.rs:182`) and outlines from `bg_stroke` (`:184`), and its hover
    // outline, where stated, under the pointer; the mark keeps `indicator_color` in `fg_stroke`
    // (§5.3). `checkbox.hover_background` is the unchecked box's hover fill, so the hovered
    // entry holds the checked fill too (§6.4).
    let own = input.scheme.default_style();
    let unstated = if input.rule_0 {
        Unstated::Egui(&own.visuals.widgets)
    } else {
        Unstated::Copy
    };
    marked_box(
        &mut style.visuals.widgets,
        &src.checked.look,
        unstated,
        notes,
    );
    Some(style)
}

/// The box a state flag marks, `k`, laid over `w`, the unchecked box's entries: its outline's
/// width at rest in every entry ([`stroke`]); its fill in every entry; under the pointer and
/// pressed the fill the theme states for that state, else what `unstated` gives for the fill at
/// rest; its outline colour drawn over the fill of each entry ([`Line::over`]), and under the
/// pointer and pressed the outline the theme states for that state, else what `unstated` gives
/// ([`state_borders`]); its mark's width at rest, and under the pointer and pressed the width
/// the theme states for that state, else what `unstated` gives for the width at rest (§6.1).
pub(crate) fn marked_box(
    w: &mut egui::style::Widgets,
    k: &MarkedBox,
    unstated: Unstated<'_>,
    notes: &mut Vec<Note>,
) {
    if let Some((width, path)) = k.width {
        for e in every_entry(w) {
            e.bg_stroke = stroke(path, e.bg_stroke, e.bg_stroke.color, width, notes);
        }
    }
    if let Some(fill) = noted(k.fill, notes) {
        for entry in [
            &mut w.noninteractive,
            &mut w.inactive,
            &mut w.hovered,
            &mut w.active,
            &mut w.open,
        ] {
            entry.bg_fill = fill;
        }
        // Under the pointer and on a press, the fill the theme states for the state, else what
        // `unstated` gives for the fill at rest.
        let own_fill = |e: &WidgetVisuals| e.bg_fill;
        let [hover, press] = k.state_fill;
        let hovered = noted(hover, notes).unwrap_or_else(|| unstated.hovered(fill, own_fill));
        w.active.bg_fill =
            noted(press, notes).unwrap_or_else(|| unstated.active(fill, hovered, own_fill));
        w.hovered.bg_fill = hovered;
    }
    for entry in [
        &mut w.noninteractive,
        &mut w.inactive,
        &mut w.hovered,
        &mut w.active,
        &mut w.open,
    ] {
        outline_over(entry, k.outline, k.align, FillField::Bg);
    }
    state_borders(
        w,
        k.state_border,
        k.align,
        Some(FillField::Bg),
        unstated,
        notes,
    );
    // The mark is a line in the state's `fg_stroke` (`egui/src/widgets/checkbox.rs:151-158`).
    let [rest, hover, press] = k.mark_width;
    let rest = rest.map(|(width, path)| length(path, width, w.inactive.fg_stroke.width, notes));
    mark_widths(w, rest, [hover, press], unstated, notes);
}

/// A check mark's width laid over `w`, the mark a line in the state's `fg_stroke`: `rest` at
/// rest; under the pointer and pressed the width the theme states for the state, else what
/// `unstated` gives for the width at rest; with no width at rest, only the stated ones.
pub(crate) fn mark_widths(
    w: &mut egui::style::Widgets,
    rest: Option<f32>,
    [hover, press]: [Read<f32>; 2],
    unstated: Unstated<'_>,
    notes: &mut Vec<Note>,
) {
    let own_width = |e: &WidgetVisuals| e.fg_stroke.width;
    let hovered = match hover {
        Some((width, path)) => Some(length(path, width, w.hovered.fg_stroke.width, notes)),
        None => rest.map(|rest| unstated.hovered(rest, own_width)),
    };
    let active = match press {
        Some((width, path)) => Some(length(path, width, w.active.fg_stroke.width, notes)),
        None => rest.map(|rest| unstated.active(rest, hovered.unwrap_or(rest), own_width)),
    };
    if let Some(rest) = rest {
        for e in [&mut w.noninteractive, &mut w.inactive, &mut w.open] {
            e.fg_stroke.width = rest;
        }
    }
    if let Some(hovered) = hovered {
        w.hovered.fg_stroke.width = hovered;
    }
    if let Some(active) = active {
        w.active.fg_stroke.width = active;
    }
}

/// The mixed check box: the [`MarkedBox`] of the mixed state and its mark's colour at rest,
/// under the pointer and pressed, each where a rule states it.
pub(crate) struct MixedBox {
    pub look: MarkedBox,
    pub mark: [Option<Rgba>; 3],
}

/// The mixed box `m` laid over `v`, the visuals of the unchecked box's scope: egui draws a mixed
/// box as its unchecked one with a dash in the mark's stroke (egui 0.37.0
/// `src/widgets/checkbox.rs:135-141`). [`marked_box`] lays the box and its mark's width over the
/// entries, as the checked box's cell does; the mark's colours go into `fg_stroke` as the
/// unchecked box's cell writes its check mark ([`write_states_with`]), an unstated hovered or
/// pressed one what `unstated` gives. Where it states a mark, the label is first held to the
/// colour it has at rest: egui letters a check box's label in the mark's `fg_stroke` where
/// `override_text_color` is unset (egui 0.37.0 `src/theme/default_style.rs:182-186`,
/// `src/widget_text.rs:831-835`).
pub(crate) fn mixed_cell(
    v: &mut egui::Visuals,
    m: &MixedBox,
    unstated: Unstated<'_>,
    notes: &mut Vec<Note>,
) {
    marked_box(&mut v.widgets, &m.look, unstated, notes);
    let [idle, hover, active] = m.mark;
    if idle.is_some() || hover.is_some() || active.is_some() {
        if v.override_text_color.is_none() {
            v.override_text_color = Some(v.widgets.inactive.fg_stroke.color);
        }
        write_states_with(
            &mut v.widgets,
            &StateSource {
                fill: None,
                text: Some(TextSource {
                    idle,
                    hover,
                    active,
                    in_noninteractive: true,
                }),
                border: None,
                open: OpenFrom::Inactive,
            },
            unstated,
            notes,
        );
    }
}

/// The flat button under the pointer and pressed or focused (egui's `active` entry is both):
/// the body's fill and the label's colour of each state, and the hovered and the pressed or
/// focused border, each where a rule states it for the state.
pub(crate) struct FlatButton {
    pub fill: [Read<Rgba>; 2],
    pub label: [Option<Rgba>; 2],
    pub state_border: [Option<StateLine>; 2],
    pub align: LineAlign,
}

/// The flat button `f` laid over `w`, the entries of the scope a flat button is drawn in: its
/// hovered and `active` fill, label colour and border, each unstated one what `unstated`
/// gives. The resting entry is left as it stands: egui draws a
/// `Button::frame_when_inactive(false)` unframed at rest (egui 0.37.0
/// `src/theme/default_style.rs:77-87`). egui lays a button out at the inner margin
/// `button_padding + expansion − bg_stroke.width` of its state (`src/theme/default_style.rs:54-71`,
/// `src/containers/frame.rs:313-323`), so a state's outline narrower than the resting one grows
/// the button by the difference. A hovered or pressed outline of no width is therefore a
/// transparent one of the resting entry's width, which keeps the box, and either pen paints
/// nothing. One of a width between none and the resting one is drawn at its stated width and
/// the button grows: egui has no per-state room to keep the box by (one `button_padding` for
/// every state, and `expansion` grows the painted rect too), an egui limit the README records
/// (spec §10, rule 4); no bundled preset states such a width.
pub(crate) fn flat_cell(
    w: &mut egui::style::Widgets,
    f: &FlatButton,
    unstated: Unstated<'_>,
    notes: &mut Vec<Note>,
) {
    let [hover, active] = f.fill;
    let [hover_text, active_text] = f.label;
    write_states_with(
        w,
        &StateSource {
            fill: Some(FillSource {
                field: FillField::Weak,
                idle: None,
                hover,
                active,
                layer: Layer::AsGiven,
            }),
            text: Some(TextSource {
                idle: None,
                hover: hover_text,
                active: active_text,
                in_noninteractive: false,
            }),
            border: None,
            open: OpenFrom::Inactive,
        },
        unstated,
        notes,
    );
    state_borders(
        w,
        f.state_border,
        f.align,
        Some(FillField::Weak),
        unstated,
        notes,
    );
    let rest = w.inactive.bg_stroke.width;
    for entry in [&mut w.hovered, &mut w.active] {
        if entry.bg_stroke.width <= 0.0 && rest > 0.0 {
            entry.bg_stroke = egui::Stroke::new(rest, egui::Color32::TRANSPARENT);
        }
    }
}

/// The five entries of `w`.
fn every_entry(w: &mut egui::style::Widgets) -> [&mut WidgetVisuals; 5] {
    [
        &mut w.noninteractive,
        &mut w.inactive,
        &mut w.hovered,
        &mut w.active,
        &mut w.open,
    ]
}

/// The `SelectedDisabled` cell of `role`, from its `Selected` cell `selected`, or `None` where
/// the role's checked look is not a cell of its own: every role but `Checkbox`, whose
/// `SelectedDisabled` cell is its `Disabled` one.
pub(crate) fn selected_disabled_cell(
    role: Role,
    selected: &egui::Style,
    src: &VariantSource,
    notes: &mut Vec<Note>,
) -> Option<egui::Style> {
    if role != Role::Checkbox {
        return None;
    }
    let k = &src.checked;
    let mut style = selected.clone();
    let v = &mut style.visuals;
    // The label in the disabled text colour, as the `Disabled` cell's (§6.3), checked or not.
    let text = src.checkbox.text.map(to_color32);
    if text.is_some() {
        v.override_text_color = text;
    }
    // The disabled rule (docs/platform-facts.md §2.1.6): where the theme states a disabled
    // fill the box takes it, and the mark the disabled text colour, as the `Disabled` cell's
    // box does; where it states none the platform dims by opacity alone, and the box is its
    // checked self, `checked_background` and the `indicator_color` mark, which
    // `disabled_alpha` fades. The outline stays the checked box's, `checkbox.border.color`,
    // drawn over the disabled fill where it lies over it, unless the theme states an outline
    // for the disabled state, which the box then takes; with none, egui dims the checked
    // outline by `disabled_alpha` alone (egui 0.37.0 `src/ui.rs:523-529`, `Ui::disable`).
    if let Some((fill, path)) = k.disabled_fill {
        let fill = to_color32(fill);
        note_transparent_fill(fill, path, notes);
        v.widgets.inactive.bg_fill = fill;
        if let Some(text) = text {
            v.widgets.inactive.fg_stroke.color = text;
        }
        outline_over(
            &mut v.widgets.inactive,
            k.look.outline,
            k.look.align,
            FillField::Bg,
        );
    }
    if let Some(border) = k.disabled_border {
        stated_outline(&mut v.widgets.inactive, border, notes);
    }
    // A disabled mark the theme states is drawn in its own colour. egui 0.37.0 letters the
    // label in the mark's `fg_stroke` where `override_text_color` is unset
    // (`src/theme/default_style.rs:183-201`, `src/widget_text.rs:831-835`), so the label is
    // first held to the colour it had.
    if let Some(mark) = k.disabled_mark {
        let w = &mut v.widgets.inactive;
        if v.override_text_color.is_none() {
            v.override_text_color = Some(w.fg_stroke.color);
        }
        w.fg_stroke.color = to_color32(mark);
    }
    Some(style)
}

/// The `Disabled` cell of `role`, or `None` where the role writes none of §6.3's fifteen leaves.
pub(crate) fn disabled_cell(
    role: Role,
    normal: &egui::Style,
    src: &VariantSource,
    notes: &mut Vec<Note>,
) -> Option<egui::Style> {
    let mut style = normal.clone();
    let v = &mut style.visuals;
    match role {
        // `button_style` fills a button from `weak_bg_fill` (`egui/src/widget_style.rs:159`) and
        // a selected one from `selection.*` regardless of state (`:150-155`), so the primary
        // button in a disabled scope takes the disabled pair too (§6.2) — where the platform
        // states a disabled fill. With none, it dims by opacity alone (docs/platform-facts.md
        // §2.1.6), and the primary button keeps its own pair.
        Role::Button => {
            let d = &src.button;
            if let Some((fill, _)) = d.fill {
                v.widgets.inactive.weak_bg_fill = to_color32(fill);
                outline_over(&mut v.widgets.inactive, d.outline, d.align, FillField::Weak);
            }
            labels(&mut v.widgets, d.text);
            if let Some(fill) = src.primary.fill {
                v.selection.bg_fill = to_color32(fill);
                if let Some(text) = src.primary.text {
                    v.selection.stroke.color = to_color32(text);
                }
            }
        }
        // `button_frame` fills the trigger from `weak_bg_fill` (`egui/src/containers/combo_box.rs:460`).
        Role::ComboBox => {
            let d = &src.combo_box;
            if let Some((fill, _)) = d.fill {
                v.widgets.inactive.weak_bg_fill = to_color32(fill);
                outline_over(&mut v.widgets.inactive, d.outline, d.align, FillField::Weak);
            }
            labels(&mut v.widgets, d.text);
        }
        // The box fills from `bg_fill` (`egui/src/widget_style.rs:182`); the label reads
        // `override_text_color` (`:133-136`) and the mark strokes `inactive.fg_stroke` (`:188`).
        // The box outlines from `bg_stroke` (egui 0.37.0 `src/theme/default_style.rs:198`): the
        // outline a rule states for the disabled state where one does, else the resting one,
        // which egui dims by `disabled_alpha` alone (`src/ui.rs:523-529`, `Ui::disable`).
        Role::Checkbox => {
            let d = &src.checkbox;
            if let Some((fill, path)) = d.fill {
                let fill = to_color32(fill);
                note_transparent_fill(fill, path, notes);
                v.widgets.inactive.bg_fill = fill;
                outline_over(&mut v.widgets.inactive, d.outline, d.align, FillField::Bg);
            }
            if let Some(border) = d.border {
                stated_outline(&mut v.widgets.inactive, border, notes);
            }
            if let Some(text) = d.text.map(to_color32) {
                v.override_text_color = Some(text);
                v.widgets.inactive.fg_stroke.color = text;
            }
        }
        // A `TextEdit` fills from `Visuals::text_edit_bg_color()`
        // (`egui/src/widgets/text_edit/builder.rs:739`) and takes its text colour from
        // `inactive` in every state (`:480-483`).
        Role::Input => {
            let d = &src.input;
            if let Some((fill, _)) = d.fill {
                v.text_edit_bg_color = Some(to_color32(fill));
            }
            labels(&mut v.widgets, d.text);
        }
        // The rail is always `inactive.bg_fill` (`egui/src/widgets/slider.rs:775`) and the
        // trailing fill `selection.bg_fill` (`:802`).
        Role::Slider => {
            if let Some((track, path)) = src.slider_track {
                let track = to_color32(track);
                note_transparent_fill(track, path, notes);
                v.widgets.inactive.bg_fill = track;
            }
            if let Some(fill) = src.slider_fill {
                v.selection.bg_fill = to_color32(fill);
            }
        }
        // The substitute `Button::new(..).selected(checked)`: unchecked it fills from the state's
        // `weak_bg_fill`, checked from `selection.bg_fill` in every state (§6.2, §6.3).
        Role::Switch => {
            let [unchecked, checked] = src.switch;
            if let Some(fill) = unchecked {
                v.widgets.inactive.weak_bg_fill = to_color32(fill);
            }
            if let Some(fill) = checked {
                v.selection.bg_fill = to_color32(fill);
            }
        }
        // A disabled item is a `Button` in `WidgetState::Inactive`; a plain label in the menu
        // reads `noninteractive` (§5.2).
        Role::Menu => labels(&mut v.widgets, src.menu),
        // Cells are `Label`s, painted from `Visuals::text_color()` (§5.4).
        Role::List => {
            if let Some(text) = src.list {
                v.widgets.noninteractive.fg_stroke.color = to_color32(text);
            }
        }
        // `Link::ui` paints with `hyperlink_color` alone (`egui/src/widgets/hyperlink.rs:47`).
        Role::Link => {
            if let Some(text) = src.link {
                v.hyperlink_color = to_color32(text);
            }
        }
        _ => return None,
    }
    // `disabled_alpha` stays the role's own, its `disabled_opacity` (or, for a role with none,
    // `defaults.disabled_opacity`, the base style's): `Ui::disable` multiplies painter opacity
    // by it (`egui/src/ui.rs:497-502`) on top of these colours. A platform dims by one of the
    // two, and the data makes the other an identity (docs/platform-facts.md §2.1.6).
    Some(style)
}

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    reason = "a test fails by panicking"
)]
mod tests {
    use std::sync::Arc;

    use egui::Theme;
    use native_theme::color::Rgba;
    use native_theme::theme::{ColorMode, ResolvedTheme};

    use crate::convert::to_color32;
    use crate::install_tests::resolved;
    use crate::style::{BuildInput, SchemeStyles, compile};
    use crate::{AccessibilityPreferences, LayoutTheme, Note, Role, RoleVariant};

    fn build(t: &ResolvedTheme) -> (SchemeStyles, Vec<Note>) {
        let prefs = AccessibilityPreferences::default();
        let layout = LayoutTheme::default();
        let input = BuildInput {
            scheme: Theme::Light,
            theme: t,
            prefs: &prefs,
            layout: &layout,
            row_height: None,
            defs: None,
            patch: None,
        };
        let mut notes = Vec::new();
        let styles = compile(&input, &mut notes);
        (styles, notes)
    }

    /// §6.2, §6.3: only Checkbox has a `Selected` cell of its own; only the nine roles §6.3
    /// lists have a `Disabled` cell of their own. Every other slot is the `Normal` `Arc`.
    #[test]
    fn cells_without_variant_data_share_the_normal_arc() {
        let t = resolved("kde-breeze", ColorMode::Light);
        let (s, _) = build(&t);
        const DISABLED: [Role; 9] = [
            Role::Button,
            Role::ComboBox,
            Role::Checkbox,
            Role::Input,
            Role::Slider,
            Role::Switch,
            Role::Menu,
            Role::List,
            Role::Link,
        ];
        for role in Role::all() {
            let normal = s.cell(*role, RoleVariant::Normal);
            let selected = s.cell(*role, RoleVariant::Selected);
            let disabled = s.cell(*role, RoleVariant::Disabled);
            assert_eq!(
                Arc::ptr_eq(normal, selected),
                *role != Role::Checkbox,
                "{role:?} Selected"
            );
            assert_eq!(
                Arc::ptr_eq(normal, disabled),
                !DISABLED.contains(role),
                "{role:?} Disabled"
            );
        }
    }

    /// The `SelectedDisabled` cell, per role: every role but `Checkbox` shares its `Disabled`
    /// `Arc`. The checkbox's is its `Selected` cell with the label in `disabled_text_color`,
    /// and, where the theme states a disabled fill (kde-breeze), that fill and the mark in
    /// `disabled_text_color` too; where it states none (adwaita) the box keeps its checked
    /// fill and its `indicator_color` mark, which `disabled_alpha` fades (the disabled rule,
    /// docs/platform-facts.md §2.1.6).
    #[test]
    fn the_selected_disabled_cell_is_the_checked_box_disabled() {
        for (preset, stated) in [("kde-breeze", true), ("adwaita", false)] {
            let t = resolved(preset, ColorMode::Light);
            assert_eq!(t.checkbox.disabled_background.is_some(), stated, "{preset}");
            let (s, _) = build(&t);
            for role in Role::all() {
                let both = s.cell(*role, RoleVariant::SelectedDisabled);
                let disabled = s.cell(*role, RoleVariant::Disabled);
                assert_eq!(
                    Arc::ptr_eq(both, disabled),
                    *role != Role::Checkbox,
                    "{preset} {role:?}"
                );
            }
            let sel = s.cell(Role::Checkbox, RoleVariant::Selected);
            let both = s.cell(Role::Checkbox, RoleVariant::SelectedDisabled);
            let text = to_color32(t.checkbox.disabled_text_color);
            assert_eq!(both.visuals.override_text_color, Some(text), "{preset}");
            let (fill, mark) = match t.checkbox.disabled_background {
                Some(fill) => (to_color32(fill), text),
                None => (
                    to_color32(t.checkbox.checked_background),
                    to_color32(t.checkbox.indicator_color),
                ),
            };
            let (w, sw) = (&both.visuals.widgets, &sel.visuals.widgets);
            assert_eq!(w.inactive.bg_fill, fill, "{preset}");
            assert_eq!(w.inactive.fg_stroke.color, mark, "{preset}");
            assert_eq!(
                w.inactive.bg_stroke.color,
                to_color32(t.checkbox.border.color),
                "{preset}: the checked outline"
            );
            assert_eq!(
                both.visuals.disabled_alpha,
                crate::convert::unit_interval(t.checkbox.disabled_opacity),
                "{preset}"
            );
            for (e, se) in [
                (w.noninteractive, sw.noninteractive),
                (w.hovered, sw.hovered),
                (w.active, sw.active),
                (w.open, sw.open),
            ] {
                assert_eq!(e, se, "{preset}: only `inactive` moves");
            }
            assert_eq!(both.spacing, sel.spacing);
            assert_eq!(both.visuals.selection, sel.visuals.selection);
        }
    }

    /// §6.2: the checked box fills from `checked_background` in all five entries, its outline is
    /// `checkbox.border.color` in `noninteractive`, `inactive` and `open` and
    /// `checkbox.hover_border_color` in `hovered` and `active` (a sentinel no other `checkbox.*`
    /// colour holds, since kde-breeze's border and hover outline are one colour), the mark
    /// keeps `indicator_color`, and nothing else moves.
    #[test]
    fn the_checkbox_selected_cell_fills_the_box_and_outlines_it() {
        let mut t = resolved("kde-breeze", ColorMode::Light);
        let sentinel = Rgba {
            r: 0x12,
            g: 0x34,
            b: 0x56,
            a: 0xff,
        };
        let k = &t.checkbox;
        for other in [
            Some(k.background_color),
            Some(k.checked_background),
            Some(k.indicator_color),
            Some(k.disabled_text_color),
            Some(k.font.color),
            Some(k.border.color),
            k.hover_background,
            k.disabled_background,
            k.unchecked_background,
            k.unchecked_border_color,
        ] {
            assert_ne!(other, Some(sentinel));
        }
        t.checkbox.hover_border_color = Some(sentinel);
        let (s, _) = build(&t);
        let normal = s.cell(Role::Checkbox, RoleVariant::Normal);
        let sel = s.cell(Role::Checkbox, RoleVariant::Selected);
        let fill = to_color32(t.checkbox.checked_background);
        let outline = to_color32(t.checkbox.border.color);
        let hover = to_color32(sentinel);
        let entries =
            |w: &egui::style::Widgets| [w.noninteractive, w.inactive, w.hovered, w.active, w.open];
        for ((n, e), want) in entries(&normal.visuals.widgets)
            .iter()
            .zip(entries(&sel.visuals.widgets))
            .zip([outline, outline, hover, hover, outline])
        {
            assert_eq!(e.bg_fill, fill);
            assert_eq!(e.bg_stroke.color, want);
            assert_eq!(e.bg_stroke.width, n.bg_stroke.width);
            assert_eq!(e.fg_stroke, n.fg_stroke, "the mark keeps indicator_color");
            assert_eq!(e.weak_bg_fill, n.weak_bg_fill);
            assert_eq!(e.corner_radius, n.corner_radius);
        }
        assert_eq!(sel.spacing, normal.spacing);
        assert_eq!(sel.text_styles, normal.text_styles);
        assert_eq!(sel.visuals.selection, normal.visuals.selection);
        assert_eq!(
            sel.visuals.override_text_color,
            normal.visuals.override_text_color
        );
        assert_eq!(sel.visuals.disabled_alpha, normal.visuals.disabled_alpha);
    }

    /// §6.3, §6.4: windows-11 states a fully transparent disabled checkbox fill, `#f9f9f900`
    /// (`native-theme/src/presets/windows-11.toml:210`; dark `#33333300`, `:810`); it is written
    /// as given into the `Disabled` cell's `inactive.bg_fill` and reported once, for the leaf.
    #[test]
    fn a_transparent_disabled_fill_is_written_and_noted_once() {
        let t = resolved("windows-11", ColorMode::Light);
        let stated = t
            .checkbox
            .disabled_background
            .expect("windows-11 states it");
        assert_eq!(stated.a, 0, "the preset states alpha 0");
        let (s, notes) = build(&t);
        let dis = s.cell(Role::Checkbox, RoleVariant::Disabled);
        assert_eq!(dis.visuals.widgets.inactive.bg_fill.a(), 0);
        let hits = notes
            .iter()
            .filter(|n| matches!(n, Note::TransparentFill { path, .. } if *path == "checkbox.disabled_background"))
            .count();
        assert_eq!(hits, 1);
        // The `Selected` cell's checked box is opaque on this preset and reports nothing.
        let sel = s.cell(Role::Checkbox, RoleVariant::Selected);
        assert_ne!(sel.visuals.widgets.inactive.bg_fill.a(), 0);
        assert!(!notes.iter().any(|n| matches!(n, Note::TransparentFill { path, .. } if *path == "checkbox.checked_background")));
    }

    /// §6.3: the Button `Disabled` cell — stated fills and text colour, the selection pair for a
    /// primary button, and the button's own `disabled_opacity` for egui to fade it by on top
    /// (docs/platform-facts.md §2.1.6); the `hovered` entry is untouched.
    #[test]
    fn the_button_disabled_cell_writes_what_a_disabled_button_reads() {
        let mut t = resolved("kde-breeze", ColorMode::Light);
        let fill = Rgba {
            r: 1,
            g: 2,
            b: 3,
            a: 255,
        };
        let text = Rgba {
            r: 10,
            g: 20,
            b: 30,
            a: 255,
        };
        t.button.disabled_background = Some(fill);
        t.button.disabled_text_color = text;
        let (s, _) = build(&t);
        let normal = s.cell(Role::Button, RoleVariant::Normal);
        let dis = s.cell(Role::Button, RoleVariant::Disabled);
        let w = &dis.visuals.widgets;
        assert_eq!(w.inactive.weak_bg_fill, to_color32(fill));
        assert_eq!(dis.visuals.selection.bg_fill, to_color32(fill));
        assert_eq!(w.inactive.fg_stroke.color, to_color32(text));
        assert_eq!(w.noninteractive.fg_stroke.color, to_color32(text));
        assert_eq!(dis.visuals.selection.stroke.color, to_color32(text));
        assert_eq!(dis.visuals.disabled_alpha, normal.visuals.disabled_alpha);
        assert_eq!(
            dis.visuals.disabled_alpha,
            crate::convert::unit_interval(t.button.disabled_opacity)
        );
        assert_eq!(w.hovered, normal.visuals.widgets.hovered);
        assert_eq!(
            w.inactive.bg_stroke,
            normal.visuals.widgets.inactive.bg_stroke
        );

        // A `None` disabled fill is the platform stating no distinct fill: the idle fill is
        // copied as given (§6.4, C16), never composited, and a primary button keeps its own
        // pair, which `disabled_opacity` alone dims.
        t.button.disabled_background = None;
        let (s, _) = build(&t);
        let normal = s.cell(Role::Button, RoleVariant::Normal);
        let dis = s.cell(Role::Button, RoleVariant::Disabled);
        assert_eq!(
            dis.visuals.widgets.inactive.weak_bg_fill,
            to_color32(t.button.background_color)
        );
        assert_eq!(dis.visuals.selection, normal.visuals.selection);
    }

    /// §6.3: each of the other eight `Disabled` cells writes the field its widget reads.
    #[test]
    fn each_disabled_cell_writes_the_fields_its_widget_reads() {
        let mut t = resolved("kde-breeze", ColorMode::Light);
        let c = |r: u8| Rgba {
            r,
            g: r,
            b: r,
            a: 255,
        };
        t.combo_box.disabled_background = Some(c(11));
        t.combo_box.disabled_text_color = c(12);
        t.checkbox.disabled_background = Some(c(21));
        t.checkbox.disabled_text_color = c(22);
        t.input.disabled_background = Some(c(31));
        t.input.disabled_text_color = c(32);
        t.slider.disabled_track_color = Some(c(41));
        t.slider.disabled_fill_color = Some(c(42));
        t.switch.disabled_unchecked_background = Some(c(51));
        t.switch.disabled_checked_background = Some(c(52));
        t.menu.disabled_text_color = c(61);
        t.list.disabled_text_color = c(71);
        t.link.disabled_text_color = c(81);
        let (s, _) = build(&t);
        let cell = |r: Role| s.cell(r, RoleVariant::Disabled);

        let d = cell(Role::ComboBox);
        assert_eq!(d.visuals.widgets.inactive.weak_bg_fill, to_color32(c(11)));
        assert_eq!(
            d.visuals.widgets.inactive.fg_stroke.color,
            to_color32(c(12))
        );
        assert_eq!(
            d.visuals.widgets.noninteractive.fg_stroke.color,
            to_color32(c(12))
        );

        let d = cell(Role::Checkbox);
        assert_eq!(d.visuals.widgets.inactive.bg_fill, to_color32(c(21)));
        assert_eq!(d.visuals.override_text_color, Some(to_color32(c(22))));
        assert_eq!(
            d.visuals.widgets.inactive.fg_stroke.color,
            to_color32(c(22))
        );

        let d = cell(Role::Input);
        assert_eq!(d.visuals.text_edit_bg_color, Some(to_color32(c(31))));
        assert_eq!(
            d.visuals.widgets.inactive.fg_stroke.color,
            to_color32(c(32))
        );
        assert_eq!(
            d.visuals.widgets.noninteractive.fg_stroke.color,
            to_color32(c(32))
        );

        let d = cell(Role::Slider);
        assert_eq!(d.visuals.widgets.inactive.bg_fill, to_color32(c(41)));
        assert_eq!(d.visuals.selection.bg_fill, to_color32(c(42)));

        let d = cell(Role::Switch);
        assert_eq!(d.visuals.widgets.inactive.weak_bg_fill, to_color32(c(51)));
        assert_eq!(d.visuals.selection.bg_fill, to_color32(c(52)));

        let d = cell(Role::Menu);
        assert_eq!(
            d.visuals.widgets.inactive.fg_stroke.color,
            to_color32(c(61))
        );
        assert_eq!(
            d.visuals.widgets.noninteractive.fg_stroke.color,
            to_color32(c(61))
        );

        let d = cell(Role::List);
        assert_eq!(
            d.visuals.widgets.noninteractive.fg_stroke.color,
            to_color32(c(71))
        );

        let d = cell(Role::Link);
        assert_eq!(d.visuals.hyperlink_color, to_color32(c(81)));

        // Each keeps its role's `disabled_opacity`, and a role with none the defaults' one, for
        // egui to fade it by on top of those colours (docs/platform-facts.md §2.1.6).
        let u = crate::convert::unit_interval;
        for (role, opacity) in [
            (Role::ComboBox, t.combo_box.disabled_opacity),
            (Role::Checkbox, t.checkbox.disabled_opacity),
            (Role::Input, t.input.disabled_opacity),
            (Role::Slider, t.slider.disabled_opacity),
            (Role::Switch, t.switch.disabled_opacity),
            (Role::Menu, t.defaults.disabled_opacity),
            (Role::List, t.defaults.disabled_opacity),
            (Role::Link, t.defaults.disabled_opacity),
        ] {
            assert_eq!(cell(role).visuals.disabled_alpha, u(opacity), "{role:?}");
        }
    }

    /// The disabled rule on the platforms (docs/platform-facts.md §2.1.6): kde-breeze dims by
    /// its disabled colours with an opacity of 1.0, adwaita by its opacity of 0.5 with no
    /// disabled fill of its own, so each `Disabled` cell carries exactly one of the two.
    #[test]
    fn each_platform_dims_by_one_mechanism() {
        let kde = resolved("kde-breeze", ColorMode::Light);
        let (s, _) = build(&kde);
        let dis = s.cell(Role::Button, RoleVariant::Disabled);
        assert_eq!(dis.visuals.disabled_alpha, 1.0);
        assert_eq!(
            dis.visuals.widgets.inactive.weak_bg_fill,
            to_color32(
                kde.button
                    .disabled_background
                    .expect("kde-breeze states it")
            )
        );

        let adw = resolved("adwaita", ColorMode::Light);
        let (s, _) = build(&adw);
        let normal = s.cell(Role::Button, RoleVariant::Normal);
        let dis = s.cell(Role::Button, RoleVariant::Disabled);
        assert_eq!(dis.visuals.disabled_alpha, 0.5);
        assert_eq!(adw.button.disabled_background, None);
        assert_eq!(
            dis.visuals.widgets.inactive.weak_bg_fill,
            normal.visuals.widgets.inactive.weak_bg_fill
        );
        assert_eq!(
            dis.visuals.widgets.inactive.fg_stroke.color,
            normal.visuals.widgets.inactive.fg_stroke.color
        );
    }

    /// §6.3: the eight disabled fills fall back to the role's own fill for that part.
    #[test]
    fn a_none_disabled_fill_copies_the_roles_own_fill() {
        let mut t = resolved("kde-breeze", ColorMode::Light);
        t.combo_box.disabled_background = None;
        t.checkbox.disabled_background = None;
        t.checkbox.unchecked_background = None;
        t.input.disabled_background = None;
        t.slider.disabled_track_color = None;
        t.slider.disabled_fill_color = None;
        t.switch.disabled_unchecked_background = None;
        t.switch.disabled_checked_background = None;
        let (s, _) = build(&t);
        let cell = |r: Role| s.cell(r, RoleVariant::Disabled);
        assert_eq!(
            cell(Role::ComboBox).visuals.widgets.inactive.weak_bg_fill,
            to_color32(t.combo_box.background_color)
        );
        assert_eq!(
            cell(Role::Checkbox).visuals.widgets.inactive.bg_fill,
            to_color32(t.checkbox.background_color)
        );
        assert_eq!(
            cell(Role::Input).visuals.text_edit_bg_color,
            Some(to_color32(t.input.background_color))
        );
        assert_eq!(
            cell(Role::Slider).visuals.widgets.inactive.bg_fill,
            to_color32(t.slider.track_color)
        );
        assert_eq!(
            cell(Role::Slider).visuals.selection.bg_fill,
            to_color32(t.slider.fill_color)
        );
        assert_eq!(
            cell(Role::Switch).visuals.widgets.inactive.weak_bg_fill,
            to_color32(t.switch.unchecked_background)
        );
        assert_eq!(
            cell(Role::Switch).visuals.selection.bg_fill,
            to_color32(t.switch.checked_background)
        );
    }

    /// §6.3: a role that writes none of the disabled leaves keeps the theme's own alpha.
    #[test]
    fn every_other_role_keeps_the_themes_disabled_alpha() {
        let t = resolved("adwaita", ColorMode::Dark);
        let (s, _) = build(&t);
        for role in [
            Role::Tab,
            Role::Sidebar,
            Role::Toolbar,
            Role::Expander,
            Role::Card,
        ] {
            let d = s.cell(role, RoleVariant::Disabled);
            assert_eq!(
                d.visuals.disabled_alpha, s.base.visuals.disabled_alpha,
                "{role:?}"
            );
        }
    }

    /// A flat button's hovered outline stated narrower than the resting one, but wider than
    /// none, is drawn at its stated width, the button then growing by the difference (the
    /// README's limits); a pressed outline stated as no width is a transparent one of the
    /// resting width.
    #[test]
    fn a_flat_buttons_narrower_state_outline_is_drawn_at_its_width() {
        use super::{FlatButton, flat_cell};
        use crate::style::line::LineAlign;
        use crate::style::states::{StateLine, Unstated};

        let mut w = Theme::Light.default_style().visuals.widgets;
        w.inactive.bg_stroke = egui::Stroke::new(1.0, egui::Color32::BLACK);
        let own = w.clone();
        let colour = Rgba {
            r: 255,
            g: 0,
            b: 255,
            a: 255,
        };
        let line = |width: f32, path| {
            Some(StateLine {
                color: Some(colour),
                width: Some((width, path)),
            })
        };
        let mut notes = Vec::new();
        flat_cell(
            &mut w,
            &FlatButton {
                fill: [None; 2],
                label: [None; 2],
                state_border: [line(0.5, "hovered"), line(0.0, "pressed")],
                align: LineAlign::Outside,
            },
            Unstated::Egui(&own),
            &mut notes,
        );
        assert!(notes.is_empty(), "{notes:?}");
        assert_eq!(
            w.hovered.bg_stroke,
            egui::Stroke::new(0.5, to_color32(colour))
        );
        assert_eq!(
            w.active.bg_stroke,
            egui::Stroke::new(1.0, egui::Color32::TRANSPARENT)
        );
    }
}
