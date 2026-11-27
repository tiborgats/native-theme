//! The 0.8.0 model's reads for the mapping's shared builders (`crate::style`): each source
//! the 0.7.2 path fills from its fields, filled here at the homes the table of homes
//! (`native-theme/tests/next_homes.toml`) names, by the query its row names. A value the
//! theme states nothing for is `None`, and the builder leaves egui's own there (spec §10,
//! rule 0); a place a note names is the home's.

use native_theme::next::{
    ButtonQuery, CheckboxWidget, Color, CornerRadius, FocusIndicator, FocusKind, FocusTrigger,
    IconStated, IconStyle, ListQuery, Orientation, PartStated, PartStyle, Preset, ResolvedTheme,
    Stroke, StyleQuery, TextStated, TextStyle, WidgetState, WindowState,
};

use super::{Mapped, Unmapped, padding_of, radius_of, reads, stroke_width_of};
use crate::Note;
use crate::convert::Rgba;
use crate::plugin::FocusRingSource;
use crate::style::base::{BaseSource, ButtonStates, DropShadow, Read, Sides};
use crate::style::cells::{
    ButtonCell, Capsule, CellSource, CheckboxCell, ComboBoxCell, DialogCell, ExpanderCell,
    InputCell, ListCell, MenuCell, Outline, PanelCell, PanelEdge, ProgressBarCell, ScrollbarCell,
    SegmentedControlCell, SeparatorCell, SidebarCell, SliderCell, SpinnerCell, SplitterCell,
    SwitchCell, TabCell, Text, ToolbarCell,
};
use crate::style::frames::{FrameSource, SurfaceSource};
use crate::style::line::LineAlign;
use crate::style::states::{Edge, StateLine};
use crate::style::variants::{
    CheckedBox, Disabled, FlatButton, MarkedBox, MixedBox, PrimaryDisabled, VariantSource,
};
use crate::style::{Sources, push_note};

/// A colour of the 0.8.0 model as the 0.7.2 model's: its 8-bit sRGB channels with straight
/// alpha, which the builders convert as they convert a 0.7.2 colour.
pub(crate) fn rgba(c: Color) -> Rgba {
    let [r, g, b, a] = c.to_srgb8();
    Rgba { r, g, b, a }
}

/// A part's fill, where stated ([`reads::fill`]); `None` and a [`Note::Unmapped`] at `home`
/// where it is stated as layers that are all gradients, which have no single colour for egui
/// to paint (spec §10, rule 4): the builder leaves egui's own there.
pub(crate) fn fill(part: &PartStyle, home: &'static str, notes: &mut Vec<Note>) -> Option<Rgba> {
    let fill = reads::fill(part);
    if fill.is_none() && part.stated.contains(PartStated::FILL) {
        push_note(notes, Note::Unmapped { path: home });
    }
    fill.map(rgba)
}

/// [`fill`] where a rule states it for the queried state (its `from_state` bit; the rule is for
/// some flag of the query, not a given one: spec §2, `from_state`).
pub(crate) fn state_fill(
    part: &PartStyle,
    home: &'static str,
    notes: &mut Vec<Note>,
) -> Option<Rgba> {
    part.from_state
        .contains(PartStated::FILL)
        .then(|| fill(part, home, notes))
        .flatten()
}

/// Whether the theme states the property `bit` for a flag of a part drawn at a state holding it
/// with other flags (spec §2, `from_state`, the two queries): `alone`, the part queried with the
/// flag as its only state flag, or `own`, the part's own query asking the flag, has the bit.
fn stated_for(alone: &PartStyle, own: &PartStyle, bit: PartStated) -> bool {
    alone.from_state.contains(bit) || own.from_state.contains(bit)
}

/// [`stated_for`] of an icon.
fn icon_stated_for(alone: &IconStyle, own: &IconStyle, bit: IconStated) -> bool {
    alone.from_state.contains(bit) || own.from_state.contains(bit)
}

/// [`stated_for`] of a text.
fn text_stated_for(alone: &TextStyle, own: &TextStyle, bit: TextStated) -> bool {
    alone.from_state.contains(bit) || own.from_state.contains(bit)
}

/// [`fill`] with the home a note names it by.
fn fill_at(part: &PartStyle, home: &'static str, notes: &mut Vec<Note>) -> Read<Rgba> {
    fill(part, home, notes).map(|c| (c, home))
}

/// [`state_fill`] with the home a note names it by.
fn state_fill_at(part: &PartStyle, home: &'static str, notes: &mut Vec<Note>) -> Read<Rgba> {
    state_fill(part, home, notes).map(|c| (c, home))
}

/// A [`Note::Unmapped`] at `home` where `part` states corners egui cannot draw as the one
/// radius it rounds a widget by: radii that differ corner by corner, or concentric ones (spec
/// §10, rule 2). egui's own radius stays, as [`radius_of`] gives none.
pub(crate) fn meet_corners(part: &PartStyle, home: &'static str, notes: &mut Vec<Note>) {
    if !part.stated.contains(PartStated::CORNERS) {
        return;
    }
    let unmapped = match part.corners.radius {
        CornerRadius::PerCorner {
            top_left,
            top_right,
            bottom_right,
            bottom_left,
        } => [top_right, bottom_right, bottom_left]
            .iter()
            .any(|r| *r != top_left),
        CornerRadius::Concentric { .. } => true,
        _ => false,
    };
    if unmapped {
        push_note(notes, Note::Unmapped { path: home });
    }
}

/// A [`Note::Unmapped`] at `home` where `part` states a stroke per side, which egui cannot
/// draw as the one `egui::Stroke` it outlines a widget by (spec §10, rule 2). egui's own
/// outline stays, as [`line()`] and [`stroke_width_of`] give none.
fn meet_stroke(part: &PartStyle, home: &'static str, notes: &mut Vec<Note>) {
    if part.stated.contains(PartStated::STROKE) && matches!(part.stroke, Stroke::Complex(_)) {
        push_note(notes, Note::Unmapped { path: home });
    }
}

/// [`radius_of`] with its home, `home` noted where egui cannot draw the corners
/// ([`meet_corners`]).
fn radius_at(part: &PartStyle, home: &'static str, notes: &mut Vec<Note>) -> Read<f32> {
    meet_corners(part, home, notes);
    radius_of(part).map(|v| (v, home))
}

/// A part's stroke colour, where stated as one stroke of one colour in an alignment the
/// connector knows.
pub(crate) fn line(part: &PartStyle) -> Option<Rgba> {
    match &part.stroke {
        Stroke::Uniform { align, .. } if LineAlign::of(*align).is_some() => {
            reads::stroke(part).map(rgba)
        }
        _ => None,
    }
}

/// The border a rule states for the queried state (its `from_state` bit), as [`line_of`] reads
/// it; `None` where no rule states the stroke.
pub(crate) fn state_line(
    part: &PartStyle,
    home: &'static str,
    notes: &mut Vec<Note>,
) -> Option<StateLine> {
    part.from_state
        .contains(PartStated::STROKE)
        .then(|| line_of(part, home, notes))
}

/// A part's border as a state's: its colour as [`line()`] reads it, transparent for a stroke
/// stated as none (spec §10, rule 2), and its width, with `home`, the width's, noted where
/// stated per side ([`meet_stroke`]).
fn line_of(part: &PartStyle, home: &'static str, notes: &mut Vec<Note>) -> StateLine {
    meet_stroke(part, home, notes);
    let color = match part.stroke {
        Stroke::None if part.stated.contains(PartStated::STROKE) => Some(rgba(Color::TRANSPARENT)),
        _ => line(part),
    };
    StateLine {
        color,
        width: stroke_width_of(part).map(|w| (w, home)),
    }
}

/// [`line()`] where a rule states it for the queried state (its `from_state` bit), `home`
/// noted where stated per side ([`meet_stroke`]).
fn state_line_color(part: &PartStyle, home: &'static str, notes: &mut Vec<Note>) -> Option<Rgba> {
    part.from_state
        .contains(PartStated::STROKE)
        .then(|| {
            meet_stroke(part, home, notes);
            line(part)
        })
        .flatten()
}

/// A part's outline as the builders take it, and where its stroke lies (`Outside` for one
/// the theme states none of, or states as none, which composites nothing).
pub(crate) fn edge(part: &PartStyle) -> (Edge, LineAlign) {
    let align = match &part.stroke {
        Stroke::Uniform { align, .. } => LineAlign::of(*align),
        _ => None,
    };
    (
        Edge {
            color: line(part),
            width: stroke_width_of(part),
            radius: radius_of(part),
        },
        align.unwrap_or(LineAlign::Outside),
    )
}

/// [`edge()`], its corners' and its stroke width's homes `paths` noted where egui cannot draw
/// them ([`meet_corners`], [`meet_stroke`]).
fn edge_at(part: &PartStyle, paths: [&'static str; 2], notes: &mut Vec<Note>) -> (Edge, LineAlign) {
    let [corners, width] = paths;
    meet_corners(part, corners, notes);
    meet_stroke(part, width, notes);
    edge(part)
}

/// A text style's colour, where stated.
pub(crate) fn text_color(text: &TextStyle) -> Option<Rgba> {
    reads::text(text).map(rgba)
}

/// [`text_color`] where a rule states it for the queried state ([`reads::state_text`]).
pub(crate) fn state_text_color(text: &TextStyle) -> Option<Rgba> {
    reads::state_text(text).map(rgba)
}

/// A text style's size in logical pixels, where stated, with its home.
fn size(text: &TextStyle, home: &'static str) -> Read<f32> {
    text.stated
        .contains(TextStated::SIZE_PX)
        .then_some((text.size_px, home))
}

/// A part's padding with the homes of its four sides, `<part>.padding.<side>`.
fn sides(part: &PartStyle, paths: [&'static str; 4]) -> Sides {
    Sides {
        values: padding_of(part),
        paths,
    }
}

/// Whether a part is drawn with a shadow: unless the theme states it has none (spec §5,
/// `0.7.2`'s `border.shadow_enabled`: `false` is `SHADOWS` stated with no layer), or none egui
/// draws (only inset layers).
fn shadowed(part: &PartStyle) -> bool {
    !part.stated.contains(PartStated::SHADOWS) || drop_shadow(part, "").is_some()
}

/// A part's primary drop shadow layer, its first outset one (spec §10, rule 2), where the theme
/// states its shadows, with `home`, the shadows'.
fn drop_shadow(part: &PartStyle, home: &'static str) -> Option<DropShadow> {
    if !part.stated.contains(PartStated::SHADOWS) {
        return None;
    }
    let layer = part.shadows.as_deref()?.iter().find(|layer| !layer.inset)?;
    Some(DropShadow {
        offset: [layer.offset_x, layer.offset_y],
        blur: layer.blur,
        spread: layer.spread,
        color: rgba(layer.color),
        home,
    })
}

/// [`BaseSource`] of the 0.8.0 model. The scroll bar's thumb at rest is the handle of a view
/// without the focus, where 0.7.2's `scrollbar.thumb_color` lives on kde-breeze; a theme with
/// no rule for that state returns its plain thumb there. A caption or headline style is laid
/// over the body (`Typography::over_body`); one the typography does not define is the body's
/// (the table's `home_fallback`).
pub(crate) fn base_source(r: &ResolvedTheme, notes: &mut Vec<Note>) -> BaseSource {
    let c = &r.colors;
    let t = &r.typography;
    let b = &r.button;
    let rest = b.body(WidgetState::NONE);
    let sb = &r.scrollbar;
    let vertical = StyleQuery::new(WidgetState::NONE).orientation(Orientation::Vertical);
    let horizontal = StyleQuery::new(WidgetState::NONE).orientation(Orientation::Horizontal);
    let frame = r.window.frame(WindowState::default());
    let menu_panel = r.menu.panel(WidgetState::NONE);
    BaseSource {
        sizes: [
            size(
                &t.over_body(t.caption.as_ref()),
                "typography.caption.size_px",
            ),
            size(&t.body, "typography.body.size_px"),
            size(&t.monospace, "typography.monospace.size_px"),
            size(&b.label(WidgetState::NONE), "button.label(NONE).size_px"),
            size(
                &t.over_body(t.headline.as_ref()),
                "typography.headline.size_px",
            ),
        ],
        window_fill: c
            .window
            .background
            .map(|v| (rgba(v), "colors.window.background")),
        text: c.window.foreground.map(rgba),
        muted: c.window.foreground_muted.map(rgba),
        shadow: Some(rgba(c.shadow)),
        link: Some(rgba(c.link)),
        selection: Some(rgba(c.selection.background)),
        selection_text: Some(rgba(c.selection.foreground)),
        danger: Some(rgba(c.error.fill)),
        warning: Some(rgba(c.warning.fill)),
        border: Edge {
            color: c.window.border.map(rgba),
            width: Some(r.layout.border_width),
            radius: Some(r.layout.corner_radius),
        },
        border_paths: ["layout.corner_radius", "layout.border_width"],
        radius_lg: Some((r.layout.corner_radius_lg, "layout.corner_radius_lg")),
        disabled_opacity: Some((c.disabled_opacity, "colors.disabled_opacity")),
        menu_fill: fill(&menu_panel, "menu.panel(NONE).fill", notes),
        overlay_scrollbar: sb.overlay_mode,
        track: fill(
            &sb.track(WidgetState::NONE),
            "scrollbar.track(NONE).fill",
            notes,
        ),
        input_fill: fill(
            &r.input.field(WidgetState::NONE),
            "input.field(NONE).fill",
            notes,
        ),
        alternate_row: fill(
            &r.list
                .row(ListQuery::new(WidgetState::NONE).alternate(true)),
            "list.row(NONE; alternate=true).fill",
            notes,
        ),
        caret: Some(rgba(c.caret)),
        blink_on_ms: r.caret.blink_on_ms,
        blink_off_ms: r.caret.blink_off_ms,
        window_radius: radius_at(&frame, "window.frame(normal).corners", notes),
        window_padding: sides(
            &frame,
            [
                "window.frame(normal).padding.top",
                "window.frame(normal).padding.right",
                "window.frame(normal).padding.bottom",
                "window.frame(normal).padding.left",
            ],
        ),
        min_height: rest.min_size[1].map(|v| (v, "button.body(NONE).min_size[1]")),
        button_padding: sides(
            &rest,
            [
                "button.body(NONE).padding.top",
                "button.body(NONE).padding.right",
                "button.body(NONE).padding.bottom",
                "button.body(NONE).padding.left",
            ],
        ),
        indicator_width: r.checkbox.indicator(WidgetState::NONE).min_size[0]
            .map(|v| (v, "checkbox.indicator(NONE).min_size[0]")),
        label_gap: Some((r.checkbox.label_gap, "checkbox.label_gap")),
        rail_height: r.slider.track(horizontal).min_size[1]
            .map(|v| (v, "slider.track(NONE; orientation=Horizontal).min_size[1]")),
        combo_width: r.combo_box.field(WidgetState::NONE).min_size[0]
            .map(|v| (v, "combo_box.field(NONE).min_size[0]")),
        tooltip_width: r.tooltip.panel.max_size[0].map(|v| (v, "tooltip.panel.max_size[0]")),
        min_thumb: Some((sb.min_thumb_length, "scrollbar.min_thumb_length")),
        widget_gap: r.layout.widget_gap,
        // the theme's milliseconds as egui's seconds (`egui/src/style.rs:928`)
        tooltip_delay: r
            .timings
            .tooltip_show_ms
            .map(|ms| (f64::from(ms) / 1000.0) as f32),
        button: button_states(r, notes),
        thumb: thumb(r, notes),
        title_bar_fill: fill(
            &r.window.title_bar(WindowState::default()),
            "window.title_bar(normal).fill",
            notes,
        ),
        line_height: Some((
            t.line_height_multiplier,
            "typography.line_height_multiplier",
        )),
        groove_width: sb.track(vertical).min_size[0]
            .map(|v| (v, "scrollbar.track(NONE; orientation=Vertical).min_size[0]")),
        thumb_width: sb.thumb(vertical).min_size[0]
            .map(|v| (v, "scrollbar.thumb(NONE; orientation=Vertical).min_size[0]")),
        window_shadow: shadowed(&frame),
        popup_shadow: r.layout.shadow_enabled,
        window_drop: drop_shadow(&frame, "window.frame(normal).shadows"),
        // egui's menus and popups: the menu's panel where it states its shadows (spec §5, notes
        // to the table, the menu's popup frame), else the popover's
        popup_drop: if menu_panel.stated.contains(PartStated::SHADOWS) {
            drop_shadow(&menu_panel, "menu.panel(NONE).shadows")
        } else {
            drop_shadow(&r.popover.panel, "popover.panel.shadows")
        },
    }
}

/// The button's states the base style and the Button cell take.
fn button_states(r: &ResolvedTheme, notes: &mut Vec<Note>) -> ButtonStates {
    let b = &r.button;
    let rest = b.body(WidgetState::NONE);
    let hover = b.body(WidgetState::HOVER);
    let pressed = b.body(WidgetState::PRESSED);
    let border_paths = [
        "button.body(NONE).corners",
        "button.body(NONE).stroke.width",
    ];
    let (border, align) = edge_at(&rest, border_paths, notes);
    ButtonStates {
        fill: [
            fill_at(&rest, "button.body(NONE).fill", notes),
            state_fill_at(&hover, "button.body(HOVER).fill", notes),
            state_fill_at(&pressed, "button.body(PRESSED).fill", notes),
        ],
        label: [
            text_color(&b.label(WidgetState::NONE)),
            state_text_color(&b.label(WidgetState::HOVER)),
        ],
        border,
        border_paths,
        align,
        // egui's `active` is press and keyboard focus alike: the focus ring's border where
        // the theme states one, else the pressed one
        state_border: [
            state_line(&hover, "button.body(HOVER).stroke.width", notes),
            state_line(
                &b.body(WidgetState::FOCUS_VISIBLE),
                "button.body(FOCUS_VISIBLE).stroke.width",
                notes,
            )
            .or_else(|| state_line(&pressed, "button.body(PRESSED).stroke.width", notes)),
        ],
    }
}

/// The flat button's states, `button.body(<state>; variant=Flat)` and its label, each where a
/// rule states it for the state (a rule that names no variant is the flat button's too, spec
/// §2): the fill and the label of `HOVER` and `PRESSED`, the hovered border, and, as egui's
/// `active` is press and keyboard focus alike, the `FOCUS_VISIBLE` border, else the `PRESSED`
/// one, as [`button_states`] reads the push button's.
pub(crate) fn flat_button(r: &ResolvedTheme, notes: &mut Vec<Note>) -> FlatButton {
    let b = &r.button;
    let flat = ButtonQuery::flat();
    let at = |state: WidgetState| flat.with_state(state);
    let (hover, pressed) = (
        b.body(at(WidgetState::HOVER)),
        b.body(at(WidgetState::PRESSED)),
    );
    FlatButton {
        fill: [
            state_fill_at(&hover, "button.body(HOVER; variant=Flat).fill", notes),
            state_fill_at(&pressed, "button.body(PRESSED; variant=Flat).fill", notes),
        ],
        label: [
            state_text_color(&b.label(at(WidgetState::HOVER))),
            state_text_color(&b.label(at(WidgetState::PRESSED))),
        ],
        state_border: [
            state_line(
                &hover,
                "button.body(HOVER; variant=Flat).stroke.width",
                notes,
            ),
            state_line(
                &b.body(at(WidgetState::FOCUS_VISIBLE)),
                "button.body(FOCUS_VISIBLE; variant=Flat).stroke.width",
                notes,
            )
            .or_else(|| {
                state_line(
                    &pressed,
                    "button.body(PRESSED; variant=Flat).stroke.width",
                    notes,
                )
            }),
        ],
        align: edge(&b.body(flat)).1,
    }
}

/// The scroll bar's thumb at rest (see [`base_source`]), hovered and pressed.
fn thumb(r: &ResolvedTheme, notes: &mut Vec<Note>) -> [Read<Rgba>; 3] {
    let sb = &r.scrollbar;
    [
        fill_at(
            &sb.thumb(WidgetState::UNFOCUSED_VIEW),
            "scrollbar.thumb(UNFOCUSED_VIEW).fill",
            notes,
        ),
        state_fill_at(
            &sb.thumb(WidgetState::HOVER),
            "scrollbar.thumb(HOVER).fill",
            notes,
        ),
        state_fill_at(
            &sb.thumb(WidgetState::PRESSED),
            "scrollbar.thumb(PRESSED).fill",
            notes,
        ),
    ]
}

/// The scroll bar thumb's corners (`scrollbar.thumb(NONE).corners`) as the one radius egui
/// rounds the thumb and its trough by ([`radius_of`]; a pill the largest radius egui's
/// `CornerRadius` holds, which epaint clamps to half the shape's shorter side, `epaint-0.37.0`
/// `src/rounded_rect.rs`, `RoundedRect::new`), noted where egui cannot draw them
/// ([`meet_corners`]).
fn thumb_radius(thumb: &PartStyle, notes: &mut Vec<Note>) -> Option<f32> {
    meet_corners(thumb, "scrollbar.thumb(NONE).corners", notes);
    radius_of(thumb)
}

/// A part's outline with the homes of its corners and stroke width ([`edge_at`]).
fn outline(part: &PartStyle, paths: [&'static str; 2], notes: &mut Vec<Note>) -> Outline {
    let (edge, align) = edge_at(part, paths, notes);
    Outline { edge, paths, align }
}

/// The menu item's outline: its corners where stated, else, on the preset the table of homes
/// names `menu.panel(NONE).corners` the home of `menu.border.corner_radius` on (material-3,
/// whose menu rounds the panel alone), the panel's.
fn menu_outline(r: &ResolvedTheme, notes: &mut Vec<Note>) -> Outline {
    let m = &r.menu;
    let mut outline = outline(
        &m.item(WidgetState::NONE),
        ["menu.item(NONE).corners", "menu.item(NONE).stroke.width"],
        notes,
    );
    if outline.edge.radius.is_none() && r.preset == Preset::Material3 {
        let home = "menu.panel(NONE).corners";
        outline.edge.radius = radius_at(&m.panel(WidgetState::NONE), home, notes).map(|(v, _)| v);
        outline.paths[0] = home;
    }
    outline
}

/// A panel part's separator line, with the home of its width, noted where stated per side
/// ([`meet_stroke`]).
fn panel_edge(part: &PartStyle, width_path: &'static str, notes: &mut Vec<Note>) -> PanelEdge {
    meet_stroke(part, width_path, notes);
    PanelEdge {
        color: line(part),
        width: stroke_width_of(part).map(|w| (w, width_path)),
    }
}

/// A text style's colour and size, with the home of the size.
fn text(t: &TextStyle, home: &'static str) -> Text {
    Text {
        color: text_color(t),
        size: size(t, home),
    }
}

/// An icon's colour, where stated.
fn icon_color(icon: &IconStyle) -> Option<Rgba> {
    reads::icon(icon).map(rgba)
}

/// [`icon_color`] where a rule states it for the queried state ([`reads::state_icon`]).
fn state_icon_color(icon: &IconStyle) -> Option<Rgba> {
    reads::state_icon(icon).map(rgba)
}

/// An icon's stroke width where a rule states it for the queried state, read at `home`.
fn state_mark_width(icon: &IconStyle, home: &'static str) -> Read<f32> {
    icon.from_state
        .contains(IconStated::STROKE_WIDTH)
        .then_some(icon.stroke_width)
        .flatten()
        .map(|v| (v, home))
}

/// A text style's hovered and pressed colours where a rule states them for the state, the
/// text style `at` gives for a state.
fn hover_press_text(at: impl Fn(WidgetState) -> TextStyle) -> [Option<Rgba>; 2] {
    [
        state_text_color(&at(WidgetState::HOVER)),
        state_text_color(&at(WidgetState::PRESSED)),
    ]
}

/// An icon's size, where stated, with its home.
fn icon_size(icon: &IconStyle, home: &'static str) -> Read<f32> {
    icon.stated
        .contains(IconStated::SIZE)
        .then_some((icon.size, home))
}

/// A widget's disabled opacity, where stated, with its home.
fn opacity_at(v: Option<f32>, home: &'static str) -> Read<f32> {
    v.map(|v| (v, home))
}

/// [`CellSource`] of the 0.8.0 model. A checked, selected or open state's value is the one a
/// rule states for that state; a hover's or a press's as [`state_fill`] reads it.
pub(crate) fn cell_source(r: &ResolvedTheme, notes: &mut Vec<Note>) -> CellSource {
    use WidgetState as S;
    let b = &r.button;
    let rest = b.body(S::NONE);
    let primary = ButtonQuery::primary();
    let m = &r.menu;
    let item = m.item(S::NONE);
    let [bar_top, _, _, bar_left] = padding_of(&r.menu_bar.item(S::NONE));
    let i = &r.input;
    let field = i.field(S::NONE);
    let k = &r.checkbox;
    let indicator = k.indicator(S::NONE);
    let sb = &r.scrollbar;
    let sl = &r.slider;
    let p = &r.progress_bar;
    let track = p.track(S::NONE);
    let [_, _, _, track_left] = padding_of(&track);
    let tb = &r.tab;
    let tab = tb.tab(S::NONE);
    let side = &r.sidebar;
    let side_box = side.container(S::NONE);
    let tool = &r.toolbar;
    let bar = tool.bar(S::NONE);
    let l = &r.list;
    let list_box = l.container(S::NONE);
    let sw = &r.switch;
    let sw_track = sw.track(S::NONE);
    let cb = &r.combo_box;
    let cb_field = cb.field(S::NONE);
    let sc = &r.segmented_control;
    let segment = sc.segment(S::NONE);
    let sc_track = sc.track(S::NONE);
    let x = &r.expander;
    let header = x.header(S::NONE);
    CellSource {
        button: ButtonCell {
            states: button_states(r, notes),
            pressed_label: state_text_color(&b.label(S::PRESSED)),
            primary_fill: fill(
                &b.body(primary),
                "button.body(NONE; variant=Primary).fill",
                notes,
            ),
            primary_text: text_color(&b.label(primary)),
            min_height: rest.min_size[1].map(|v| (v, "button.body(NONE).min_size[1]")),
            icon_gap: Some((b.icon_text_gap, "button.icon_text_gap")),
            padding: sides(
                &rest,
                [
                    "button.body(NONE).padding.top",
                    "button.body(NONE).padding.right",
                    "button.body(NONE).padding.bottom",
                    "button.body(NONE).padding.left",
                ],
            ),
            disabled_opacity: opacity_at(b.disabled_opacity, "button.disabled_opacity"),
            size: size(&b.label(S::NONE), "button.label(NONE).size_px"),
        },
        menu: MenuCell {
            hover_fill: state_fill_at(&m.item(S::HOVER), "menu.item(HOVER).fill", notes),
            text: [
                text_color(&m.item_label(S::NONE)),
                state_text_color(&m.item_label(S::HOVER)),
            ],
            outline: menu_outline(r, notes),
            hover_border: state_line(&m.item(S::HOVER), "menu.item(HOVER).stroke.width", notes),
            bar_padding: [
                bar_left.map(|v| (v, "menu_bar.item(NONE).padding.left")),
                bar_top.map(|v| (v, "menu_bar.item(NONE).padding.top")),
            ],
            bar_fill: [
                state_fill(
                    &r.menu_bar.item(S::HOVER),
                    "menu_bar.item(HOVER).fill",
                    notes,
                ),
                state_fill(&r.menu_bar.item(S::OPEN), "menu_bar.item(OPEN).fill", notes),
            ],
            bar_text: [
                state_text_color(&r.menu_bar.item_label(S::HOVER)),
                state_text_color(&r.menu_bar.item_label(S::OPEN)),
            ],
            separator: fill(&m.separator(S::NONE), "menu.separator(NONE).fill", notes),
            padding: sides(
                &item,
                [
                    "menu.item(NONE).padding.top",
                    "menu.item(NONE).padding.right",
                    "menu.item(NONE).padding.bottom",
                    "menu.item(NONE).padding.left",
                ],
            ),
            // a height the item is held to is a row height; a least height alone, a floor
            row_height: item.max_size[1].map(|v| (v, "menu.item(NONE).max_size[1]")),
            min_row_height: item.min_size[1].map(|v| (v, "menu.item(NONE).min_size[1]")),
            panel_padding: sides(
                &m.panel(S::NONE),
                [
                    "menu.panel(NONE).padding.top",
                    "menu.panel(NONE).padding.right",
                    "menu.panel(NONE).padding.bottom",
                    "menu.panel(NONE).padding.left",
                ],
            ),
            icon_gap: Some((m.icon_text_gap, "menu.icon_text_gap")),
            size: size(&m.item_label(S::NONE), "menu.item_label(NONE).size_px"),
        },
        input: InputCell {
            text: {
                let [hover, press] = hover_press_text(|s| i.text(s));
                [text_color(&i.text(S::NONE)), hover, press]
            },
            outline: outline(
                &field,
                [
                    "input.field(NONE).corners",
                    "input.field(NONE).stroke.width",
                ],
                notes,
            ),
            state_border: [
                state_line(&i.field(S::HOVER), "input.field(HOVER).stroke.width", notes),
                state_line(
                    &i.field(S::PRESSED),
                    "input.field(PRESSED).stroke.width",
                    notes,
                ),
            ],
            placeholder: text_color(&i.placeholder(S::NONE)),
            selection: Some(rgba(r.colors.text_selection.background)),
            selection_text: Some(rgba(r.colors.text_selection.foreground)),
            padding: sides(
                &field,
                [
                    "input.field(NONE).padding.top",
                    "input.field(NONE).padding.right",
                    "input.field(NONE).padding.bottom",
                    "input.field(NONE).padding.left",
                ],
            ),
            size: size(&i.text(S::NONE), "input.text(NONE).size_px"),
            disabled_opacity: opacity_at(i.disabled_opacity, "input.disabled_opacity"),
        },
        checkbox: CheckboxCell {
            fill: [
                fill_at(&indicator, "checkbox.indicator(NONE).fill", notes),
                state_fill_at(
                    &k.indicator(S::HOVER),
                    "checkbox.indicator(HOVER).fill",
                    notes,
                ),
                state_fill_at(
                    &k.indicator(S::PRESSED),
                    "checkbox.indicator(PRESSED).fill",
                    notes,
                ),
            ],
            mark: [
                icon_color(&k.checkmark(S::CHECKED)),
                state_icon_color(&k.checkmark(S::HOVER)),
                state_icon_color(&k.checkmark(S::PRESSED)),
            ],
            outline: outline(
                &indicator,
                [
                    "checkbox.indicator(NONE).corners",
                    "checkbox.indicator(NONE).stroke.width",
                ],
                notes,
            ),
            state_border: [
                state_line(
                    &k.indicator(S::HOVER),
                    "checkbox.indicator(HOVER).stroke.width",
                    notes,
                ),
                state_line(
                    &k.indicator(S::PRESSED),
                    "checkbox.indicator(PRESSED).stroke.width",
                    notes,
                ),
            ],
            mark_width: [
                k.checkmark(S::NONE)
                    .stroke_width
                    .map(|v| (v, "checkbox.checkmark(NONE).stroke_width")),
                state_mark_width(
                    &k.checkmark(S::HOVER),
                    "checkbox.checkmark(HOVER).stroke_width",
                ),
                state_mark_width(
                    &k.checkmark(S::PRESSED),
                    "checkbox.checkmark(PRESSED).stroke_width",
                ),
            ],
            label: text(&k.label(S::NONE), "checkbox.label(NONE).size_px"),
            indicator: indicator.min_size[0].map(|v| (v, "checkbox.indicator(NONE).min_size[0]")),
            label_gap: Some((k.label_gap, "checkbox.label_gap")),
            disabled_opacity: opacity_at(k.disabled_opacity, "checkbox.disabled_opacity"),
            padding: sides(
                &indicator,
                [
                    "checkbox.indicator(NONE).padding.top",
                    "checkbox.indicator(NONE).padding.right",
                    "checkbox.indicator(NONE).padding.bottom",
                    "checkbox.indicator(NONE).padding.left",
                ],
            ),
        },
        tooltip: text(&r.tooltip.text, "tooltip.text.size_px"),
        popover: text(&r.popover.text, "popover.text.size_px"),
        dialog: DialogCell {
            body: text(&r.dialog.body, "dialog.body.size_px"),
            title_size: size(&r.dialog.title, "dialog.title.size_px"),
            button_gap: Some((r.dialog.button_gap, "dialog.button_gap")),
        },
        scrollbar: ScrollbarCell {
            track: fill(&sb.track(S::NONE), "scrollbar.track(NONE).fill", notes),
            thumb: thumb(r, notes),
            min_thumb: Some((sb.min_thumb_length, "scrollbar.min_thumb_length")),
            radius: thumb_radius(&sb.thumb(S::NONE), notes),
        },
        slider: SliderCell {
            fill: fill(
                &sl.active_track(S::NONE),
                "slider.active_track(NONE).fill",
                notes,
            ),
            track: fill_at(&sl.track(S::NONE), "slider.track(NONE).fill", notes),
            disabled_opacity: opacity_at(sl.disabled_opacity, "slider.disabled_opacity"),
            thumb_diameter: sl.thumb(S::NONE).min_size[0]
                .map(|v| (v, "slider.thumb(NONE).min_size[0]")),
            thumb: [
                fill_at(&sl.thumb(S::NONE), "slider.thumb(NONE).fill", notes),
                state_fill_at(&sl.thumb(S::HOVER), "slider.thumb(HOVER).fill", notes),
                state_fill_at(&sl.thumb(S::PRESSED), "slider.thumb(PRESSED).fill", notes),
            ],
            radius: Some(r.layout.corner_radius),
        },
        progress_bar: ProgressBarCell {
            fill: fill(&p.fill(S::NONE), "progress_bar.fill(NONE).fill", notes),
            track: fill(&track, "progress_bar.track(NONE).fill", notes),
            height: track.min_size[1].map(|v| (v, "progress_bar.track(NONE).min_size[1]")),
            gap: track_left.map(|v| (v, "progress_bar.track(NONE).padding.left")),
        },
        tab: TabCell {
            fill: [
                fill_at(&tab, "tab.tab(NONE).fill", notes),
                state_fill_at(&tb.tab(S::HOVER), "tab.tab(HOVER).fill", notes),
                state_fill_at(&tb.tab(S::PRESSED), "tab.tab(PRESSED).fill", notes),
            ],
            text: [
                text_color(&tb.label(S::NONE)),
                state_text_color(&tb.label(S::HOVER)),
                state_text_color(&tb.label(S::PRESSED)),
            ],
            // the selected tab's outline is the tab's (the table of homes' `tab.border` rows)
            outline: outline(
                &tb.tab(S::SELECTED),
                [
                    "tab.tab(SELECTED).corners",
                    "tab.tab(SELECTED).stroke.width",
                ],
                notes,
            ),
            state_border: [
                state_line(&tb.tab(S::HOVER), "tab.tab(HOVER).stroke.width", notes),
                state_line(&tb.tab(S::PRESSED), "tab.tab(PRESSED).stroke.width", notes),
            ],
            selected_fill: state_fill(&tb.tab(S::SELECTED), "tab.tab(SELECTED).fill", notes),
            selected_text: state_text_color(&tb.label(S::SELECTED)),
            bar_fill: fill(&tb.bar(S::NONE), "tab.bar(NONE).fill", notes),
            min_height: tab.min_size[1].map(|v| (v, "tab.tab(NONE).min_size[1]")),
            item_gap: tb.item_gap.map(|v| (v, "tab.item_gap")),
            padding: sides(
                &tab,
                [
                    "tab.tab(NONE).padding.top",
                    "tab.tab(NONE).padding.right",
                    "tab.tab(NONE).padding.bottom",
                    "tab.tab(NONE).padding.left",
                ],
            ),
            size: size(&tb.label(S::NONE), "tab.label(NONE).size_px"),
        },
        sidebar: SidebarCell {
            panel: PanelCell {
                fill: fill(&side_box, "sidebar.container(NONE).fill", notes),
                text: text(
                    &side.item_label(S::NONE),
                    "sidebar.item_label(NONE).size_px",
                ),
                edge: panel_edge(&side_box, "sidebar.container(NONE).stroke.width", notes),
            },
            selection: state_fill(
                &side.item(S::SELECTED),
                "sidebar.item(SELECTED).fill",
                notes,
            ),
            selection_text: state_text_color(&side.item_label(S::SELECTED)),
            item_fill: [
                state_fill_at(&side.item(S::HOVER), "sidebar.item(HOVER).fill", notes),
                state_fill_at(&side.item(S::PRESSED), "sidebar.item(PRESSED).fill", notes),
            ],
            item_text: hover_press_text(|s| side.item_label(s)),
        },
        toolbar: ToolbarCell {
            panel: PanelCell {
                fill: fill(&bar, "toolbar.bar(NONE).fill", notes),
                text: text(
                    &tool.button_label(S::NONE),
                    "toolbar.button_label(NONE).size_px",
                ),
                edge: panel_edge(&bar, "toolbar.bar(NONE).stroke.width", notes),
            },
            bar_height: bar.min_size[1].map(|v| (v, "toolbar.bar(NONE).min_size[1]")),
            item_gap: tool.item_gap.map(|v| (v, "toolbar.item_gap")),
            rest: Capsule {
                fill: fill_at(&tool.button(S::NONE), "toolbar.button(NONE).fill", notes),
                radius: radius_at(&tool.button(S::NONE), "toolbar.button(NONE).corners", notes),
            },
            button_fill: [
                state_fill(&tool.button(S::HOVER), "toolbar.button(HOVER).fill", notes),
                state_fill(
                    &tool.button(S::PRESSED),
                    "toolbar.button(PRESSED).fill",
                    notes,
                ),
            ],
            button_border: [
                state_line(
                    &tool.button(S::HOVER),
                    "toolbar.button(HOVER).stroke.width",
                    notes,
                ),
                state_line(
                    &tool.button(S::PRESSED),
                    "toolbar.button(PRESSED).stroke.width",
                    notes,
                ),
            ],
            button_text: hover_press_text(|s| tool.button_label(s)),
            padding: sides(
                &bar,
                [
                    "toolbar.bar(NONE).padding.top",
                    "toolbar.bar(NONE).padding.right",
                    "toolbar.bar(NONE).padding.bottom",
                    "toolbar.bar(NONE).padding.left",
                ],
            ),
        },
        status_bar: PanelCell {
            fill: fill(&r.status_bar.bar, "status_bar.bar.fill", notes),
            text: text(&r.status_bar.label, "status_bar.label.size_px"),
            edge: panel_edge(&r.status_bar.bar, "status_bar.bar.stroke.width", notes),
        },
        list: ListCell {
            selection: state_fill(
                &l.highlight(S::SELECTED),
                "list.highlight(SELECTED).fill",
                notes,
            ),
            selection_text: state_text_color(&l.item_label(S::SELECTED)),
            grid: Some(rgba(l.grid_color)),
            width: {
                let home = "list.container(NONE).stroke.width";
                meet_stroke(&list_box, home, notes);
                stroke_width_of(&list_box).map(|v| (v, home))
            },
            radius: radius_at(&list_box, "list.container(NONE).corners", notes),
            row_height: l.row(S::NONE).min_size[1].map(|v| (v, "list.row(NONE).min_size[1]")),
            highlight: [
                state_fill_at(&l.highlight(S::HOVER), "list.highlight(HOVER).fill", notes),
                state_fill_at(
                    &l.highlight(S::PRESSED),
                    "list.highlight(PRESSED).fill",
                    notes,
                ),
            ],
            item: text(&l.item_label(S::NONE), "list.item_label(NONE).size_px"),
            header_text: text_color(&l.header_label(S::NONE)),
        },
        separator: SeparatorCell {
            color: Some(rgba(r.separator.color)),
            width: Some((r.separator.thickness, "separator.thickness")),
        },
        switch: SwitchCell {
            checked: state_fill(&sw.track(S::CHECKED), "switch.track(CHECKED).fill", notes),
            fill: [
                fill_at(&sw_track, "switch.track(NONE).fill", notes),
                state_fill_at(&sw.track(S::HOVER), "switch.track(HOVER).fill", notes),
                state_fill_at(&sw.track(S::PRESSED), "switch.track(PRESSED).fill", notes),
            ],
            radius: radius_at(&sw_track, "switch.track(NONE).corners", notes),
            height: sw_track.min_size[1].map(|v| (v, "switch.track(NONE).min_size[1]")),
            disabled_opacity: opacity_at(sw.disabled_opacity, "switch.disabled_opacity"),
        },
        spinner: SpinnerCell {
            color: Some(rgba(r.spinner.color)),
            diameter: Some((r.spinner.diameter, "spinner.diameter")),
            min_diameter: Some((r.spinner.min_diameter, "spinner.min_diameter")),
        },
        combo_box: ComboBoxCell {
            fill: [
                fill_at(&cb_field, "combo_box.field(NONE).fill", notes),
                state_fill_at(&cb.field(S::HOVER), "combo_box.field(HOVER).fill", notes),
                state_fill_at(
                    &cb.field(S::PRESSED),
                    "combo_box.field(PRESSED).fill",
                    notes,
                ),
            ],
            text: text(&cb.label(S::NONE), "combo_box.label(NONE).size_px"),
            text_states: hover_press_text(|s| cb.label(s)),
            outline: outline(
                &cb_field,
                [
                    "combo_box.field(NONE).corners",
                    "combo_box.field(NONE).stroke.width",
                ],
                notes,
            ),
            state_border: [
                state_line(
                    &cb.field(S::HOVER),
                    "combo_box.field(HOVER).stroke.width",
                    notes,
                ),
                state_line(
                    &cb.field(S::PRESSED),
                    "combo_box.field(PRESSED).stroke.width",
                    notes,
                ),
            ],
            open_fill: state_fill(&cb.field(S::OPEN), "combo_box.field(OPEN).fill", notes),
            open_border: state_line_color(
                &cb.field(S::OPEN),
                "combo_box.field(OPEN).stroke.color",
                notes,
            ),
            min_height: cb_field.min_size[1].map(|v| (v, "combo_box.field(NONE).min_size[1]")),
            arrow: icon_size(&cb.chevron(S::NONE), "combo_box.chevron(NONE).size"),
            arrow_area: cb
                .arrow_area_width
                .map(|v| (v, "combo_box.arrow_area_width")),
            padding: sides(
                &cb_field,
                [
                    "combo_box.field(NONE).padding.top",
                    "combo_box.field(NONE).padding.right",
                    "combo_box.field(NONE).padding.bottom",
                    "combo_box.field(NONE).padding.left",
                ],
            ),
            disabled_opacity: opacity_at(cb.disabled_opacity, "combo_box.disabled_opacity"),
        },
        segmented_control: SegmentedControlCell {
            fill: [
                fill_at(&sc_track, "segmented_control.track(NONE).fill", notes),
                state_fill_at(
                    &sc.segment(S::HOVER),
                    "segmented_control.segment(HOVER).fill",
                    notes,
                ),
                state_fill_at(
                    &sc.segment(S::PRESSED),
                    "segmented_control.segment(PRESSED).fill",
                    notes,
                ),
            ],
            text: text(&sc.label(S::NONE), "segmented_control.label(NONE).size_px"),
            text_states: hover_press_text(|s| sc.label(s)),
            outline: outline(
                &sc_track,
                [
                    "segmented_control.track(NONE).corners",
                    "segmented_control.track(NONE).stroke.width",
                ],
                notes,
            ),
            state_border: [
                state_line(
                    &sc.segment(S::HOVER),
                    "segmented_control.segment(HOVER).stroke.width",
                    notes,
                ),
                state_line(
                    &sc.segment(S::PRESSED),
                    "segmented_control.segment(PRESSED).stroke.width",
                    notes,
                ),
            ],
            selected_fill: state_fill(
                &sc.segment(S::SELECTED),
                "segmented_control.segment(SELECTED).fill",
                notes,
            ),
            selected_text: state_text_color(&sc.label(S::SELECTED)),
            height: segment.min_size[1].map(|v| (v, "segmented_control.segment(NONE).min_size[1]")),
            separator: Some((sc.separator_width, "segmented_control.separator_width")),
            padding: sides(
                &segment,
                [
                    "segmented_control.segment(NONE).padding.top",
                    "segmented_control.segment(NONE).padding.right",
                    "segmented_control.segment(NONE).padding.bottom",
                    "segmented_control.segment(NONE).padding.left",
                ],
            ),
            disabled_opacity: opacity_at(sc.disabled_opacity, "segmented_control.disabled_opacity"),
        },
        expander: ExpanderCell {
            fill: [
                state_fill_at(&x.header(S::HOVER), "expander.header(HOVER).fill", notes),
                state_fill_at(
                    &x.header(S::PRESSED),
                    "expander.header(PRESSED).fill",
                    notes,
                ),
            ],
            text: text(
                &x.header_label(S::NONE),
                "expander.header_label(NONE).size_px",
            ),
            text_states: hover_press_text(|s| x.header_label(s)),
            outline: outline(
                &x.container(S::NONE),
                [
                    "expander.container(NONE).corners",
                    "expander.container(NONE).stroke.width",
                ],
                notes,
            ),
            header_height: header.min_size[1].map(|v| (v, "expander.header(NONE).min_size[1]")),
            padding: sides(
                &header,
                [
                    "expander.header(NONE).padding.top",
                    "expander.header(NONE).padding.right",
                    "expander.header(NONE).padding.bottom",
                    "expander.header(NONE).padding.left",
                ],
            ),
            framed: x.frame_enabled == Some(true),
            arrow: icon_size(&x.arrow(S::NONE), "expander.arrow(NONE).size"),
        },
        link: text(&r.link.label(S::NONE), "link.label(NONE).size_px"),
        splitter: SplitterCell {
            color: fill(
                &r.splitter.handle(S::NONE),
                "splitter.handle(NONE).fill",
                notes,
            ),
            hover: state_fill(
                &r.splitter.handle(S::HOVER),
                "splitter.handle(HOVER).fill",
                notes,
            ),
            pressed: state_fill(
                &r.splitter.handle(S::PRESSED),
                "splitter.handle(PRESSED).fill",
                notes,
            ),
            width: Some((r.splitter.divider_width, "splitter.divider_width")),
        },
        scaled_up: r.context.text_scale > 1.0,
    }
}

/// A role's disabled look: the label's and the fill's values a rule states for the disabled
/// state, and the role's resting outline that fill is drawn under.
fn disabled(
    label: &TextStyle,
    part: &PartStyle,
    home: &'static str,
    rest: &PartStyle,
    notes: &mut Vec<Note>,
) -> Disabled {
    let (outline, align) = edge(rest);
    Disabled {
        text: state_text_color(label),
        fill: state_fill_at(part, home, notes),
        outline: outline.color,
        align,
        border: None,
    }
}

/// The border a rule states for the queried state ([`state_line`]), and where it lies.
fn state_edge(
    part: &PartStyle,
    home: &'static str,
    notes: &mut Vec<Note>,
) -> Option<(StateLine, LineAlign)> {
    state_line(part, home, notes).map(|line| (line, edge(part).1))
}

/// The homes a note names a [`MarkedBox`]'s values by: its fill and outline width at rest, the
/// hovered and the pressed ones, and its mark's width at rest, hovered and pressed.
struct MarkedHomes {
    fill: &'static str,
    width: &'static str,
    state_fill: [&'static str; 2],
    state_width: [&'static str; 2],
    mark_width: [&'static str; 3],
}

/// The box `flag` marks (`CHECKED`, `INDETERMINATE`): its fill and its outline's width where a
/// rule states them for the flag; its outline colour, the flag's query's, transparent for one
/// stated as none (spec §10, rule 2), so the box does not keep the unchecked box's over its own
/// fill; its mark's width where a rule states it for the flag; and under the pointer and
/// pressed, each of fill, outline and mark width the combined state's where the theme states it
/// for `HOVER` or `PRESSED`, else `None`, and the box keeps egui's own treatment of the hover or
/// the press (spec §2, `from_state`: a connector that keeps its own treatment of one flag asks
/// the two queries, the flag alone and the combined state asking it; [`stated_for`]).
fn marked_box(
    k: &CheckboxWidget,
    flag: WidgetState,
    homes: MarkedHomes,
    notes: &mut Vec<Note>,
) -> MarkedBox {
    use WidgetState as S;
    let part = k.indicator(flag);
    // the box at `state | flag`, asking `state`
    let own = |state: WidgetState| k.indicator(StyleQuery::new(state | flag).asking(state));
    let stated_fill = |state: WidgetState, home: &'static str, notes: &mut Vec<Note>| {
        let own = own(state);
        stated_for(&k.indicator(state), &own, PartStated::FILL)
            .then(|| fill_at(&own, home, notes))
            .flatten()
    };
    let stated_line = |state: WidgetState, home: &'static str, notes: &mut Vec<Note>| {
        let own = own(state);
        stated_for(&k.indicator(state), &own, PartStated::STROKE)
            .then(|| line_of(&own, home, notes))
    };
    // the mark's width where a rule states it for `asked`, the query `at`'s asking it
    let mark_width = |asked: WidgetState, at: WidgetState, home: &'static str| {
        let own = k.checkmark(StyleQuery::new(at).asking(asked));
        icon_stated_for(&k.checkmark(asked), &own, IconStated::STROKE_WIDTH)
            .then_some(own.stroke_width)
            .flatten()
            .map(|v| (v, home))
    };
    let [hover_fill, press_fill] = homes.state_fill;
    let [hover_width, press_width] = homes.state_width;
    let [mark_rest, mark_hover, mark_press] = homes.mark_width;
    MarkedBox {
        fill: state_fill_at(&part, homes.fill, notes),
        width: state_line(&part, homes.width, notes).and_then(|line| line.width),
        outline: line_of(&part, homes.width, notes).color,
        align: edge(&part).1,
        state_fill: [
            stated_fill(S::HOVER, hover_fill, notes),
            stated_fill(S::PRESSED, press_fill, notes),
        ],
        state_border: [
            stated_line(S::HOVER, hover_width, notes),
            stated_line(S::PRESSED, press_width, notes),
        ],
        mark_width: [
            mark_width(flag, flag, mark_rest),
            mark_width(S::HOVER, S::HOVER | flag, mark_hover),
            mark_width(S::PRESSED, S::PRESSED | flag, mark_press),
        ],
    }
}

/// The mixed check box of the 0.8.0 theme `r`, as a rule states it for `INDETERMINATE`: the
/// box ([`marked_box`]) and its mark's colour at rest and, where the theme states one for
/// `HOVER` or `PRESSED` by the two queries ([`stated_for`]), the combined state's.
pub(crate) fn mixed_box(r: &ResolvedTheme, notes: &mut Vec<Note>) -> MixedBox {
    use WidgetState as S;
    let k = &r.checkbox;
    let flag = S::INDETERMINATE;
    let state_mark = |state: WidgetState| {
        let own = k.checkmark(StyleQuery::new(state | flag).asking(state));
        icon_stated_for(&k.checkmark(state), &own, IconStated::COLOR)
            .then(|| icon_color(&own))
            .flatten()
    };
    MixedBox {
        look: marked_box(
            k,
            flag,
            MarkedHomes {
                fill: "checkbox.indicator(INDETERMINATE).fill",
                width: "checkbox.indicator(INDETERMINATE).stroke.width",
                state_fill: [
                    "checkbox.indicator(HOVER|INDETERMINATE).fill",
                    "checkbox.indicator(PRESSED|INDETERMINATE).fill",
                ],
                state_width: [
                    "checkbox.indicator(HOVER|INDETERMINATE).stroke.width",
                    "checkbox.indicator(PRESSED|INDETERMINATE).stroke.width",
                ],
                mark_width: [
                    "checkbox.checkmark(INDETERMINATE).stroke_width",
                    "checkbox.checkmark(HOVER|INDETERMINATE).stroke_width",
                    "checkbox.checkmark(PRESSED|INDETERMINATE).stroke_width",
                ],
            },
            notes,
        ),
        mark: [
            state_icon_color(&k.checkmark(flag)),
            state_mark(S::HOVER),
            state_mark(S::PRESSED),
        ],
    }
}

/// [`VariantSource`] of the 0.8.0 model. A disabled or checked value is the one a rule states
/// for that state. The checked box's fill and outline under the pointer or pressed keep egui's
/// own treatment of the hover or the press, each property apart, unless the theme states one
/// for it (spec §2, `from_state`: the state alone, or the combined state asking it;
/// [`stated_for`]), and are then the combined state's value; a checked box's disabled fill,
/// mark, outline and label likewise (the label else the unchecked disabled box's), and the
/// checked switch's disabled track. The disabled box takes
/// the outline a rule states for the disabled state, else keeps its resting one.
pub(crate) fn variant_source(r: &ResolvedTheme, notes: &mut Vec<Note>) -> VariantSource {
    use WidgetState as S;
    let b = &r.button;
    let cb = &r.combo_box;
    let k = &r.checkbox;
    let i = &r.input;
    let primary_disabled = ButtonQuery::primary().with_state(S::DISABLED);
    let off_on = StyleQuery::new(S::DISABLED | S::CHECKED).asking(S::DISABLED);
    let (box_off, box_off_on) = (k.indicator(S::DISABLED), k.indicator(off_on));
    let mark_off_on = k.checkmark(off_on);
    let (label_off, label_off_on) = (k.label(S::DISABLED), k.label(off_on));
    let track_off_on = r.switch.track(off_on);
    VariantSource {
        button: disabled(
            &b.label(S::DISABLED),
            &b.body(S::DISABLED),
            "button.body(DISABLED).fill",
            &b.body(S::NONE),
            notes,
        ),
        primary: PrimaryDisabled {
            fill: state_fill(
                &b.body(primary_disabled),
                "button.body(DISABLED; variant=Primary).fill",
                notes,
            ),
            text: state_text_color(&b.label(primary_disabled)),
        },
        combo_box: disabled(
            &cb.label(S::DISABLED),
            &cb.field(S::DISABLED),
            "combo_box.field(DISABLED).fill",
            &cb.field(S::NONE),
            notes,
        ),
        checkbox: Disabled {
            border: state_edge(
                &k.indicator(S::DISABLED),
                "checkbox.indicator(DISABLED).stroke.width",
                notes,
            ),
            ..disabled(
                &k.label(S::DISABLED),
                &k.indicator(S::DISABLED),
                "checkbox.indicator(DISABLED).fill",
                &k.indicator(S::NONE),
                notes,
            )
        },
        checked: CheckedBox {
            look: marked_box(
                k,
                S::CHECKED,
                MarkedHomes {
                    fill: "checkbox.indicator(CHECKED).fill",
                    width: "checkbox.indicator(CHECKED).stroke.width",
                    state_fill: [
                        "checkbox.indicator(HOVER|CHECKED).fill",
                        "checkbox.indicator(PRESSED|CHECKED).fill",
                    ],
                    state_width: [
                        "checkbox.indicator(HOVER|CHECKED).stroke.width",
                        "checkbox.indicator(PRESSED|CHECKED).stroke.width",
                    ],
                    mark_width: [
                        "checkbox.checkmark(CHECKED).stroke_width",
                        "checkbox.checkmark(HOVER|CHECKED).stroke_width",
                        "checkbox.checkmark(PRESSED|CHECKED).stroke_width",
                    ],
                },
                notes,
            ),
            disabled_fill: stated_for(&box_off, &box_off_on, PartStated::FILL)
                .then(|| {
                    fill_at(
                        &box_off_on,
                        "checkbox.indicator(DISABLED|CHECKED).fill",
                        notes,
                    )
                })
                .flatten(),
            disabled_mark: icon_stated_for(
                &k.checkmark(S::DISABLED),
                &mark_off_on,
                IconStated::COLOR,
            )
            .then(|| icon_color(&mark_off_on))
            .flatten(),
            disabled_border: stated_for(&box_off, &box_off_on, PartStated::STROKE).then(|| {
                (
                    line_of(
                        &box_off_on,
                        "checkbox.indicator(DISABLED|CHECKED).stroke.width",
                        notes,
                    ),
                    edge(&box_off_on).1,
                )
            }),
            disabled_label: if text_stated_for(&label_off, &label_off_on, TextStated::COLOR) {
                text_color(&label_off_on)
            } else {
                state_text_color(&label_off)
            },
        },
        input: Disabled {
            text: state_text_color(&i.text(S::DISABLED)),
            fill: state_fill_at(&i.field(S::DISABLED), "input.field(DISABLED).fill", notes),
            outline: None,
            align: LineAlign::Outside,
            border: None,
        },
        slider_track: state_fill_at(
            &r.slider.track(S::DISABLED),
            "slider.track(DISABLED).fill",
            notes,
        ),
        slider_fill: state_fill(
            &r.slider.active_track(S::DISABLED),
            "slider.active_track(DISABLED).fill",
            notes,
        ),
        switch: [
            state_fill(
                &r.switch.track(S::DISABLED),
                "switch.track(DISABLED).fill",
                notes,
            ),
            stated_for(
                &r.switch.track(S::DISABLED),
                &track_off_on,
                PartStated::FILL,
            )
            .then(|| fill(&track_off_on, "switch.track(DISABLED|CHECKED).fill", notes))
            .flatten(),
        ],
        menu: state_text_color(&r.menu.item_label(S::DISABLED)),
        list: state_text_color(&r.list.item_label(S::DISABLED)),
        link: state_text_color(&r.link.label(S::DISABLED)),
        toolbar: {
            let part = r.toolbar.button(S::DISABLED);
            Capsule {
                fill: state_fill_at(&part, "toolbar.button(DISABLED).fill", notes),
                radius: if part.from_state.contains(PartStated::CORNERS) {
                    radius_at(&part, "toolbar.button(DISABLED).corners", notes)
                } else {
                    None
                },
            }
        },
        toolbar_label: state_text_color(&r.toolbar.button_label(S::DISABLED)),
    }
}

/// A surface's frame reads from its part, each with its home: `<home>.fill`,
/// `<home>.stroke.width`, `<home>.corners`, `<home>.padding.<side>` and `<home>.shadows`.
fn surface(part: &PartStyle, homes: [&'static str; 8], notes: &mut Vec<Note>) -> SurfaceSource {
    let [fill_home, width, corners, top, right, bottom, left, shadows] = homes;
    let (edge, align) = edge_at(part, [corners, width], notes);
    SurfaceSource {
        fill: fill(part, fill_home, notes),
        stroke: edge.color,
        width: edge.width.map(|v| (v, width)),
        align,
        radius: edge.radius.map(|v| (v, corners)),
        padding: sides(part, [top, right, bottom, left]),
        shadow: shadowed(part),
        drop: drop_shadow(part, shadows),
    }
}

/// The eight homes [`surface`] names, of the part whose path is `$p`.
macro_rules! homes {
    ($p:literal) => {
        [
            concat!($p, ".fill"),
            concat!($p, ".stroke.width"),
            concat!($p, ".corners"),
            concat!($p, ".padding.top"),
            concat!($p, ".padding.right"),
            concat!($p, ".padding.bottom"),
            concat!($p, ".padding.left"),
            concat!($p, ".shadows"),
        ]
    };
}

/// [`FrameSource`] of the 0.8.0 model; `inactive` is the same theme resolved for a window that
/// is not active (`ThemeContext::window_active` false), whose title bar the
/// `WindowTitleBar` frame fills from (the table of homes' `window.inactive_title_bar_background`
/// row).
pub(crate) fn frame_source(
    r: &ResolvedTheme,
    inactive: &ResolvedTheme,
    notes: &mut Vec<Note>,
) -> FrameSource {
    use WidgetState as S;
    let window = WindowState::default();
    FrameSource {
        window: surface(
            &r.window.frame(window),
            homes!("window.frame(normal)"),
            notes,
        ),
        title_bar: fill(
            &inactive.window.title_bar(window),
            "window.title_bar(normal).fill",
            notes,
        ),
        dialog: surface(&r.dialog.panel, homes!("dialog.panel"), notes),
        popover: surface(&r.popover.panel, homes!("popover.panel"), notes),
        tooltip: surface(&r.tooltip.panel, homes!("tooltip.panel"), notes),
        card: surface(
            &r.card.container(S::NONE),
            homes!("card.container(NONE)"),
            notes,
        ),
        sidebar: surface(
            &r.sidebar.container(S::NONE),
            homes!("sidebar.container(NONE)"),
            notes,
        ),
        toolbar: surface(&r.toolbar.bar(S::NONE), homes!("toolbar.bar(NONE)"), notes),
        status_bar: surface(&r.status_bar.bar, homes!("status_bar.bar"), notes),
        shadow_color: rgba(r.colors.shadow),
        window_margin: r.layout.window_margin,
    }
}

/// Every source of one scheme's styles from the 0.8.0 model, an unstated hover or press taking
/// egui's own treatment (spec §10, rule 0); `inactive` as [`frame_source`] takes it.
///
/// # Errors
///
/// [`focus_ring_source`]'s.
pub(crate) fn sources(
    r: &ResolvedTheme,
    inactive: &ResolvedTheme,
    notes: &mut Vec<Note>,
) -> Mapped<Sources> {
    Ok(Sources {
        base: base_source(r, notes),
        cells: cell_source(r, notes),
        variants: variant_source(r, notes),
        frames: frame_source(r, inactive, notes),
        focus_ring: focus_ring_source(r)?,
        part_rings: Some(part_ring_sources(r, notes)),
        rule_0: true,
    })
}

/// The homes of a focus indicator's fields, which a note names where egui cannot draw them:
/// the global ring's (`focus_ring`), or a part's own (`PartStyle.focus`) at the state the part
/// is asked at.
pub(crate) struct RingHomes {
    kind: &'static str,
    stroke_width: &'static str,
    offset: &'static str,
    inner_stroke: &'static str,
    inner_width: &'static str,
    corners: &'static str,
}

/// The [`RingHomes`] of the indicator at `$p` followed by `$at`.
macro_rules! ring_homes {
    ($p:literal, $at:literal) => {
        RingHomes {
            kind: concat!($p, $at, ".kind"),
            stroke_width: concat!($p, $at, ".stroke.width"),
            offset: concat!($p, $at, ".offset"),
            inner_stroke: concat!($p, $at, ".inner_stroke"),
            inner_width: concat!($p, $at, ".inner_stroke.width"),
            corners: concat!($p, $at, ".corners"),
        }
    };
}

/// The global ring's homes.
const GLOBAL_RING: RingHomes = ring_homes!("focus_ring", "");

/// The [`RingHomes`] of the part `$p`'s own ring asked at `FOCUSED` and at `FOCUS_VISIBLE`.
macro_rules! part_ring_homes {
    ($p:literal) => {
        [
            ring_homes!($p, "(FOCUSED).focus"),
            ring_homes!($p, "(FOCUS_VISIBLE).focus"),
        ]
    };
}

/// The part whose fill and stroke a role's cell reads, asked at `state`, with the homes of its
/// own ring's fields at `FOCUSED` and at `FOCUS_VISIBLE` (README, the focus ring's table); `None`
/// for a role that draws no focusable part, whose widgets take the global ring.
fn focus_part(
    r: &ResolvedTheme,
    role: crate::Role,
    state: WidgetState,
) -> Option<(PartStyle, [RingHomes; 2])> {
    use crate::Role as R;
    Some(match role {
        R::Button => (r.button.body(state), part_ring_homes!("button.body")),
        R::Input => (r.input.field(state), part_ring_homes!("input.field")),
        R::Checkbox => (
            r.checkbox.indicator(state),
            part_ring_homes!("checkbox.indicator"),
        ),
        R::Slider => (r.slider.thumb(state), part_ring_homes!("slider.thumb")),
        R::Tab => (r.tab.tab(state), part_ring_homes!("tab.tab")),
        R::Switch => (r.switch.track(state), part_ring_homes!("switch.track")),
        R::ComboBox => (
            r.combo_box.field(state),
            part_ring_homes!("combo_box.field"),
        ),
        R::SegmentedControl => (
            r.segmented_control.segment(state),
            part_ring_homes!("segmented_control.segment"),
        ),
        R::Expander => (
            r.expander.header(state),
            part_ring_homes!("expander.header"),
        ),
        R::Menu => (r.menu.item(state), part_ring_homes!("menu.item")),
        R::List => (r.list.row(state), part_ring_homes!("list.row")),
        R::Toolbar => (r.toolbar.button(state), part_ring_homes!("toolbar.button")),
        R::Sidebar => (r.sidebar.item(state), part_ring_homes!("sidebar.item")),
        _ => return None,
    })
}

/// What each role's part ([`focus_part`]) states of its own ring in each variant, asked at
/// `FOCUSED` for a focus that followed no key press and at `FOCUS_VISIBLE` for one that did (spec
/// §2, `FocusIndicator`), a checked check box's (`Selected`, `SelectedDisabled`) with `CHECKED`:
/// none where the part states none, the ring where it states one egui can draw, and where it
/// states one egui cannot draw, none and a [`Note::Unmapped`] at the field's home.
fn part_ring_sources(r: &ResolvedTheme, notes: &mut Vec<Note>) -> crate::style::PartRingSources {
    use crate::plugin::PartRingSource;
    use crate::{Role, RoleVariant};

    let mut rings = crate::style::PartRingSources::default();
    for (row, role) in rings.iter_mut().zip(Role::all()) {
        for (slot, variant) in row.iter_mut().zip(crate::roles::VARIANTS) {
            let checked = *role == Role::Checkbox
                && matches!(
                    variant,
                    RoleVariant::Selected | RoleVariant::SelectedDisabled
                );
            let extra = if checked {
                WidgetState::CHECKED
            } else {
                WidgetState::NONE
            };
            let at = [WidgetState::FOCUSED, WidgetState::FOCUS_VISIBLE];
            for (ring, focus) in slot.iter_mut().zip(at) {
                let Some((part, homes)) = focus_part(r, *role, focus | extra) else {
                    continue;
                };
                if !part.stated.contains(PartStated::FOCUS) {
                    continue;
                }
                let [pointer, keyboard] = &homes;
                let homes = if focus == WidgetState::FOCUSED {
                    pointer
                } else {
                    keyboard
                };
                *ring = match part.focus.as_deref() {
                    None => PartRingSource::None,
                    Some(f) if matches!(f.stroke, Stroke::Complex(_)) => {
                        push_note(
                            notes,
                            Note::Unmapped {
                                path: homes.stroke_width,
                            },
                        );
                        PartRingSource::None
                    }
                    Some(f) => match indicator_source(f, homes) {
                        Ok(src) => PartRingSource::Ring(src),
                        Err(Unmapped(path)) => {
                            push_note(notes, Note::Unmapped { path });
                            PartRingSource::None
                        }
                    },
                };
            }
        }
    }
    // egui has no radio role: a `RadioButton` takes its scope's ring (README, the focus ring),
    // so a ring the radio's indicator states is not drawn
    for (focus, path) in [
        (WidgetState::FOCUSED, "radio.indicator(FOCUSED).focus"),
        (
            WidgetState::FOCUS_VISIBLE,
            "radio.indicator(FOCUS_VISIBLE).focus",
        ),
    ] {
        if r.radio.indicator(focus).stated.contains(PartStated::FOCUS) {
            push_note(notes, Note::Unmapped { path });
        }
    }
    rings
}

/// [`FocusRingSource`] of the 0.8.0 model: the ring's stroke where it is one stroke of one
/// colour (a stroke stated as none is a zero width, no ring; a per-side one gives no ring, as
/// [`super::focus_ring_width`] cannot map it), its offset, its trigger, the second ring inside
/// it (`inner_stroke`) and the corners of the line the offset places (`corners`) where the
/// theme states them. The plugin strokes the ring outward from the widget's edge grown by the
/// offset and the second ring inward from it (spec §2, `FocusIndicator`); a continuous curve
/// is drawn as the circular corner of its radius (spec §10, rule 2).
///
/// # Errors
///
/// [`Unmapped`] `focus_ring.kind` where the indicator is not a ring (a fill, an underline),
/// which the plugin's ring cannot draw, `focus_ring.inner_stroke` where the second ring is
/// stated per side, and `focus_ring.corners` where the corners are concentric, which needs
/// the container's radius.
pub(crate) fn focus_ring_source(r: &ResolvedTheme) -> Mapped<FocusRingSource> {
    indicator_source(&r.focus_ring, &GLOBAL_RING)
}

/// [`focus_ring_source`] of the indicator `ring`, the homes of its fields `homes`: the global
/// ring's, or a part's own.
///
/// # Errors
///
/// As [`focus_ring_source`], at `homes`.
fn indicator_source(ring: &FocusIndicator, homes: &RingHomes) -> Mapped<FocusRingSource> {
    use crate::plugin::RingCorners;

    if ring.kind != FocusKind::Ring {
        return Err(Unmapped(homes.kind));
    }
    let (color, width) = match &ring.stroke {
        Stroke::None => (None, Some(0.0)),
        Stroke::Uniform { color, width, .. } => (Some(rgba(*color)), Some(*width)),
        _ => (None, None),
    };
    let inner = match &ring.inner_stroke {
        Stroke::None => None,
        Stroke::Uniform { color, width, .. } => Some((rgba(*color), (*width, homes.inner_width))),
        _ => return Err(Unmapped(homes.inner_stroke)),
    };
    let corners = match ring.corners.map(|c| c.radius) {
        None => None,
        Some(CornerRadius::None) => Some(RingCorners::Radii([0.0; 4])),
        Some(CornerRadius::Uniform(v)) => Some(RingCorners::Radii([v; 4])),
        Some(CornerRadius::PerCorner {
            top_left,
            top_right,
            bottom_right,
            bottom_left,
        }) => Some(RingCorners::Radii([
            top_left,
            top_right,
            bottom_right,
            bottom_left,
        ])),
        Some(CornerRadius::Pill) => Some(RingCorners::Pill),
        Some(_) => return Err(Unmapped(homes.corners)),
    };
    Ok(FocusRingSource {
        color,
        width: width.map(|v| (v, homes.stroke_width)),
        offset: Some((ring.offset, homes.offset)),
        keyboard_only: ring.trigger == Some(FocusTrigger::KeyboardOnly),
        inner,
        corners,
        corners_path: homes.corners,
    })
}
