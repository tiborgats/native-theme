//! The 25 `Normal` role cells (§3.4, §4.4): a copy of the base style plus the role's writes.

use egui::style::Widgets;
use egui::{FontFamily, FontId, TextStyle};

use native_theme::theme::ResolvedPadding;

#[cfg(test)]
use super::BuildInput;
use super::base::{
    Read, Sides, button_padding_of, length, margin_of, opacity, radius, sanitised_size, stroke,
};
use super::cells::{CellSource, Outline, PanelEdge, Text};
use super::push_note;
use super::states::{
    BorderEntries, BorderSource, FillField, FillSource, Layer, OpenFrom, StateLine, StateSource,
    TextSource, Unstated, state_borders, write_states_with,
};
use crate::convert::{Rgba, button_padding_outside, clamp_length, padding_with_line, to_color32};
use crate::{Note, Role};

/// What a role cell is built with besides its source: the scheme, epaint's Body row (§6.15),
/// the font definitions a stated row height is fitted on ([`fit_row_padding`]), and whether a
/// hover or press the source leaves `None` takes egui's own treatment (spec §10, rule 0, the
/// 0.8.0 model's) or the 0.7.2 model's copy (§6.1).
pub(crate) struct CellInput<'a> {
    pub scheme: egui::Theme,
    pub row_height: Option<f32>,
    pub defs: Option<&'a egui::FontDefinitions>,
    pub rule_0: bool,
}

impl<'a> CellInput<'a> {
    /// The 0.7.2 model's: an unstated hover or press copies.
    #[cfg(test)]
    pub(crate) fn old(input: &BuildInput<'a>) -> Self {
        Self {
            scheme: input.scheme,
            row_height: input.row_height,
            defs: input.defs,
            rule_0: false,
        }
    }
}

/// A [`CellInput`] with the treatment of an unstated state resolved.
struct Cx<'a> {
    scheme: egui::Theme,
    row_height: Option<f32>,
    defs: Option<&'a egui::FontDefinitions>,
    scaled_up: bool,
    unstated: Unstated<'a>,
    /// How a control's hovered, pressed and open fills reach their entries.
    layer: Layer,
}

/// One role's `Normal` cell from the 0.7.2 model.
#[cfg(test)]
pub(crate) fn role_cell(
    role: Role,
    base: &egui::Style,
    input: &BuildInput<'_>,
    notes: &mut Vec<Note>,
) -> egui::Style {
    let src = CellSource::old(input.theme, input.prefs);
    role_cell_from(role, base, &src, &CellInput::old(input), notes)
}

/// One role's `Normal` cell: `base` copied, then the role's writes from `src` (§3.4).
pub(crate) fn role_cell_from(
    role: Role,
    base: &egui::Style,
    src: &CellSource,
    input: &CellInput<'_>,
    notes: &mut Vec<Note>,
) -> egui::Style {
    let own = input.scheme.default_style();
    let cx = Cx {
        scheme: input.scheme,
        row_height: input.row_height,
        defs: input.defs,
        scaled_up: src.scaled_up,
        unstated: if input.rule_0 {
            Unstated::Egui(&own.visuals.widgets)
        } else {
            Unstated::Copy
        },
        layer: Layer::of_states(input.rule_0),
    };
    let mut s = base.clone();
    match role {
        Role::Window | Role::Card => {} // no entry, no field (§6.1): the surfaces carry them
        Role::Splitter => {}            // its three strokes are D7 (§6.9), in `derived::apply_role`
        Role::Button => button(&mut s, base, src, &cx, notes),
        Role::Input => input_role(&mut s, base, src, &cx, notes),
        Role::Checkbox => checkbox(&mut s, base, src, &cx, notes),
        Role::Menu => menu(&mut s, base, src, &cx, notes),
        Role::Tooltip => surface_text(&mut s, base, &src.tooltip, notes),
        Role::Scrollbar => scrollbar(&mut s, base, src, &cx, notes),
        Role::Slider => slider(&mut s, base, src, &cx, notes),
        Role::ProgressBar => progress_bar(&mut s, base, src, notes),
        Role::Tab => tab(&mut s, base, src, &cx, notes),
        Role::Sidebar => sidebar(&mut s, base, src, &cx, notes),
        Role::Toolbar => toolbar(&mut s, base, src, &cx, notes),
        Role::StatusBar => status_bar(&mut s, base, src, &cx, notes),
        Role::List => list(&mut s, base, src, &cx, notes),
        Role::Popover => surface_text(&mut s, base, &src.popover, notes),
        Role::Separator => separator(&mut s, base, src, notes),
        Role::Switch => switch(&mut s, base, src, &cx, notes),
        Role::Dialog => dialog(&mut s, base, src, notes),
        Role::Spinner => spinner(&mut s, src),
        Role::ComboBox => combo_box(&mut s, base, src, &cx, notes),
        Role::SegmentedControl => segmented_control(&mut s, base, src, &cx, notes),
        Role::Expander => expander(&mut s, base, src, &cx, notes),
        Role::Link => link(&mut s, src, notes),
    }
    if keeps_resting_stroke(role, src) {
        let w = &mut s.visuals.widgets;
        w.hovered.bg_stroke.color = w.inactive.bg_stroke.color;
        w.active.bg_stroke.color = w.inactive.bg_stroke.color;
    }
    crate::style::derived::apply_role(role, &mut s, src, cx.row_height, cx.unstated, notes);
    s
}

/// The roles whose cells keep their resting stroke when hovered or pressed: the button's
/// hover and focus outline is not theirs, a toolbar's or a list's buttons and the controls
/// inside a slider's or a switch's scope among them (D5; the backlog decides the others).
/// Each passes `border: None` to `write_states`. A toolbar whose theme states its
/// buttons' outline (`toolbar.button_hover_border_color`) keeps the one `toolbar`
/// writes (D7).
fn keeps_resting_stroke(role: Role, src: &CellSource) -> bool {
    match role {
        Role::Toolbar => src.toolbar.button_border.iter().all(Option::is_none),
        Role::Scrollbar
        | Role::Slider
        | Role::Sidebar
        | Role::StatusBar
        | Role::List
        | Role::Switch => true,
        _ => false,
    }
}

/// B3 (§8.5): the role's font size on `override_font_id`, `Proportional` always; a size
/// that is not a positive normal `f32` writes no override and is reported.
/// The size is already scaled by the text scale; an unstated one writes nothing.
fn override_font(s: &mut egui::Style, size: Read<f32>, notes: &mut Vec<Note>) {
    let Some((scaled, path)) = size else {
        return;
    };
    if scaled.is_normal() && scaled > 0.0 {
        s.override_font_id = Some(FontId::new(scaled, FontFamily::Proportional));
    } else {
        push_note(notes, Note::ValueSanitised { path });
    }
}

/// A role's font size, already scaled, into one `text_styles` slot (§8.5's rule over the
/// base's value there); an unstated one writes nothing.
fn slot_size(
    s: &mut egui::Style,
    slot: TextStyle,
    size: Read<f32>,
    own: &egui::Style,
    notes: &mut Vec<Note>,
) {
    if let (Some(font), Some(own_font), Some((size, path))) = (
        s.text_styles.get_mut(&slot),
        own.text_styles.get(&slot),
        size,
    ) {
        font.size = sanitised_size(path, size, own_font.size, notes);
    }
}

/// A length sink written where `read` is stated, over `own`.
fn set_length(sink: &mut f32, read: Read<f32>, own: f32, notes: &mut Vec<Note>) {
    if let Some((v, path)) = read {
        *sink = length(path, v, own, notes);
    }
}

/// `disabled_alpha` where `read` is stated.
fn set_opacity(s: &mut egui::Style, read: Read<f32>, own: &egui::Style, notes: &mut Vec<Note>) {
    if let Some((v, path)) = read {
        s.visuals.disabled_alpha = opacity(path, v, own.visuals.disabled_alpha, notes);
    }
}

/// A colour sink written where `c` is stated.
fn set_color(sink: &mut egui::Color32, c: Option<Rgba>) {
    if let Some(c) = c {
        *sink = to_color32(c);
    }
}

/// The selected look `button_style` reads (§6.2): `selection.bg_fill` and its text colour.
fn set_selection(s: &mut egui::Style, fill: Option<Rgba>, text: Option<Rgba>) {
    set_color(&mut s.visuals.selection.bg_fill, fill);
    set_color(&mut s.visuals.selection.stroke.color, text);
}

/// `Spacing::button_padding` from a widget's padding and its outline's width, over the base's.
fn padding_of(
    sides: &Sides,
    outline: &Outline,
    own: &egui::Style,
    notes: &mut Vec<Note>,
) -> egui::Vec2 {
    button_padding_of(
        sides.paths,
        sides.values,
        outline.edge.width,
        own.spacing.button_padding,
        notes,
    )
}

/// A border into every entry.
fn border_all(outline: &Outline) -> Option<BorderSource> {
    Some(BorderSource {
        border: outline.edge,
        paths: outline.paths,
        entries: BorderEntries::All,
        align: outline.align,
    })
}

/// A row height the theme may leave unstated — `menu.row_height`, `list.row_height`,
/// `toolbar.bar_height`, the three `soft_option` claimants of `interact_size.y` (§1.3, §5.11) —
/// into the cell's `interact_size.y`: stated, its length; unstated, egui's own value for the
/// scheme (`Theme::default_style`, which the base style starts from), so egui's own value
/// stands as §2 *Unstated sizes* promises. A cell is a copy of the base style, whose
/// `interact_size.y` is `button.min_height` (§5.9), and leaving that there would make a row
/// the platform sizes to its content (`docs/platform-facts.md` §2.6, §2.13, §2.15) at least a
/// push button tall. The toolbar's stated height is §6.10's, written after this. A stated menu
/// or list row height is a row's least height to egui, so the cell's vertical padding is fitted
/// to it once the cell's font is written ([`fit_row_padding`]). The menu's `min_row_height`, a
/// least height the row grows past, goes into `interact_size.y` in its place where
/// `menu.row_height` is unstated (`menu`), and fits no padding.
fn row_height(s: &mut egui::Style, v: Read<f32>, cx: &Cx<'_>, notes: &mut Vec<Note>) {
    let own = cx.scheme.default_style().spacing.interact_size.y;
    s.spacing.interact_size.y = v.map_or(own, |(v, path)| length(path, v, own, notes));
}

/// A stated row height (`menu.row_height`, `list.row_height`, `expander.header_height`) as the
/// drawn height of a one-line row at a text scale of 1 or less (not the menu's
/// `min_row_height`, a least height, which egui's `Button` takes as it is), as gpui
/// (`geometry::with_height_rule`'s fixed `h`, `geometry::accordion_title`'s) and iced
/// (`Length::Fixed`) draw it. egui's `Button` takes `interact_size.y`, where [`row_height`]
/// writes the stated height, as its least height and grows to its label and vertical padding
/// (`egui/src/widgets/button.rs:306-309`, `src/atomics/atom_layout.rs:409`), and its
/// `CollapsingHeader` its header the same way (`egui/src/containers/collapsing_header.rs:531-532`),
/// so the cell's vertical `button_padding` is narrowed until a one-line label in the cell's
/// `slot` font (Body for a menu or list row, Button for the header's title) — epaint's row at
/// that size (`fonts::body_row_height`) and `extra_text_line_spacing`
/// (`egui/src/widget_text.rs:775-776`) — fits the stated height with it: floored to whole
/// points, as egui rounds the margin (`epaint/src/margin.rs:115-119`), and never widened. A
/// stated height that is not finite fits nothing, as `interact_size.y` keeps egui's own there.
/// A row that already fits is unchanged: egui centres the label in the room the height leaves
/// (`atom_layout.rs:599`, `:619`; `collapsing_header.rs:536-539`). Where the label alone is
/// taller than the stated height the padding goes to 0 and the row is the label's height. Above
/// a text scale of 1 the theme's padding stands: the row grows past the stated height to hold
/// its scaled text, as gpui's menu and list rows do (`min_h`), and keeps its padding round it.
/// The same fit as the widgets crate's segmented control (`native-theme-egui-widgets`
/// `segmented_control.rs`, `fit_height`).
fn fit_row_padding(s: &mut egui::Style, stated: Read<f32>, slot: TextStyle, cx: &Cx<'_>) {
    let (Some(_), Some(defs)) = (stated.filter(|(h, _)| h.is_finite()), cx.defs) else {
        return;
    };
    if cx.scaled_up {
        return;
    }
    let Some(size) = s.text_styles.get(&slot).map(|font| font.size) else {
        return;
    };
    let Some(row) = crate::fonts::body_row_height(defs, size) else {
        return;
    };
    // `interact_size.y` holds the stated height as `row_height` wrote it, sanitised
    let label = row + s.spacing.extra_text_line_spacing;
    let fit = ((s.spacing.interact_size.y - label) / 2.0).floor().max(0.0);
    if fit.is_finite() && fit < s.spacing.button_padding.y {
        s.spacing.button_padding.y = fit;
    }
}

/// A role's text colour in all five entries (the roles §6.1 names that state a text colour)
/// and the hovered and pressed colours the theme states, an unstated one what [`Unstated`]
/// gives. With none stated, the entries keep theirs.
fn text_states(idle: Option<Rgba>, [hover, active]: [Option<Rgba>; 2]) -> Option<TextSource> {
    (idle.is_some() || hover.is_some() || active.is_some()).then_some(TextSource {
        idle,
        hover,
        active,
        in_noninteractive: true,
    })
}

/// A panel role's border: the separator line `noninteractive.bg_stroke` strokes
/// (`egui/src/containers/panel.rs:911`), and no radius, which is the `Surface::Panel`
/// frame's (§5.6, §6.1); each of colour and width where stated.
fn panel_edge(s: &mut egui::Style, own: &egui::Style, edge: &PanelEdge, notes: &mut Vec<Note>) {
    let own_line = own.visuals.widgets.noninteractive.bg_stroke;
    let line = &mut s.visuals.widgets.noninteractive.bg_stroke;
    if let Some((width, path)) = edge.width {
        line.width = stroke(path, own_line, own_line.color, width, notes).width;
    }
    set_color(&mut line.color, edge.color);
}

fn button(
    s: &mut egui::Style,
    own: &egui::Style,
    src: &CellSource,
    cx: &Cx<'_>,
    notes: &mut Vec<Note>,
) {
    let b = &src.button;
    let st = &b.states;
    let [idle, hover, active] = st.fill;
    write_states_with(
        &mut s.visuals.widgets,
        &StateSource {
            fill: Some(FillSource {
                field: FillField::Weak,
                idle,
                hover,
                active,
                layer: cx.layer,
            }),
            text: Some(TextSource {
                idle: st.label[0],
                hover: st.label[1],
                active: b.pressed_label,
                in_noninteractive: true, // `button.font.color` → {noninteractive,inactive,open} «Button» (§5.3)
            }),
            border: Some(BorderSource {
                border: st.border,
                paths: st.border_paths,
                align: st.align,
                entries: BorderEntries::All, // the `{5}` rows
            }),
            open: OpenFrom::Inactive,
        },
        cx.unstated,
        notes,
    );
    // the hovered and the focused button's border, as the base writes them (`base_style`)
    state_borders(
        &mut s.visuals.widgets,
        st.state_border,
        st.align,
        Some(FillField::Weak),
        cx.unstated,
        notes,
    );
    // the selected (primary) button: `button_style` reads `selection.*` regardless of state (§6.2)
    set_selection(s, b.primary_fill, b.primary_text);
    set_length(
        &mut s.spacing.interact_size.y,
        b.min_height,
        own.spacing.interact_size.y,
        notes,
    );
    set_length(
        &mut s.spacing.icon_spacing,
        b.icon_gap,
        own.spacing.icon_spacing,
        notes,
    );
    s.spacing.button_padding = button_padding_of(
        b.padding.paths,
        b.padding.values,
        st.border.width,
        own.spacing.button_padding,
        notes,
    );
    set_opacity(s, b.disabled_opacity, own, notes);
    override_font(s, b.size, notes);
}

fn menu(
    s: &mut egui::Style,
    own: &egui::Style,
    src: &CellSource,
    cx: &Cx<'_>,
    notes: &mut Vec<Note>,
) {
    let m = &src.menu;
    // egui's own menu look first (§3.4): item padding vec2(2.0, 0.0), no item outlines, a
    // transparent resting item (`egui/src/containers/menu.rs:22-29`)
    egui::containers::menu::menu_style(s);
    write_states_with(
        &mut s.visuals.widgets,
        &StateSource {
            fill: Some(FillSource {
                field: FillField::Weak,
                idle: None, // menu_style's transparent item stands: no resting fill leaf (§6.1)
                hover: m.hover_fill,
                active: None,          // copies hovered
                layer: Layer::AsGiven, // a row highlight over the menu's own frame (C17)
            }),
            text: Some(TextSource {
                idle: m.text[0],
                hover: m.text[1],
                active: None,
                in_noninteractive: true, // `menu.font.color` → {noninteractive,inactive}
            }),
            border: Some(BorderSource {
                border: m.outline.edge,
                paths: m.outline.paths,
                align: m.outline.align,
                entries: BorderEntries::Interactive, // the item's outline; nothing into the frame's
            }),
            // `active` copies `hovered`, so `menu.active_background` and `active_text_color`
            // are not drawn: egui draws a focused widget in `active`
            // (egui-0.37.0/src/widget_style/mod.rs:212-222) and moves the focus along a menu's
            // items with the arrow keys (src/containers/menu.rs:233-288), where Breeze's pressed
            // look would stay on every keyboard-selected item (S6)
            open: OpenFrom::Hovered, // the open item stays highlighted (`menu.rs:382-384`)
        },
        cx.unstated,
        notes,
    );
    // S6 whatever the theme states of the pressed item: `active` is the hover look, not
    // egui's own pressed one, which a keyboard-selected item would show
    let w = &mut s.visuals.widgets;
    w.active.weak_bg_fill = w.hovered.weak_bg_fill;
    w.active.fg_stroke.color = w.hovered.fg_stroke.color;
    // E1: the base's `window_stroke.width` is `defaults.border.line_width`, sanitised; egui lays
    // the stroke inside the padding (egui-0.37.0/src/theme/default_style.rs:68-71): nothing moves.
    // Unstated, the item keeps its resting outline in every state: egui's own menu entries
    // (`menu_style`) stroke no item in any state, so rule 0 keeps the rest (spec §10); a
    // stated width is laid over, an unstated one the window stroke's
    if let Some(StateLine {
        color: Some(c),
        width,
    }) = m.hover_border
    {
        let frame = own.visuals.window_stroke;
        let line = match width {
            Some((w, path)) => stroke(path, frame, to_color32(c), w, notes),
            None => egui::Stroke::new(frame.width, to_color32(c)),
        };
        let w = &mut s.visuals.widgets;
        (w.hovered.bg_stroke, w.open.bg_stroke) = (line, line); // open: a submenu shown, S4
        w.active.bg_stroke = line; // `active` stays the hover look (S6)
    }
    // E3: `menu_bar_titles` keeps the cell's padding on a non-finite bar side; reported here,
    // where the atlas is built
    for (side, path) in m.bar_padding.into_iter().flatten() {
        if !side.is_finite() {
            push_note(notes, Note::ValueSanitised { path });
        }
    }
    set_color(
        &mut s.visuals.widgets.noninteractive.bg_stroke.color,
        m.separator,
    ); // `separator_style` (§5.2)
    // the item padding over menu_style's vec2(2.0, 0.0), the unstated side's target (§5.2)
    let after_menu_style = s.spacing.button_padding;
    s.spacing.button_padding = button_padding_of(
        m.padding.paths,
        m.padding.values,
        m.outline.edge.width,
        after_menu_style,
        notes,
    );
    // E4: egui has one `button_padding.x` for both sides of an item: `menu.border.padding.left`
    // where stated, the shortcut and a submenu's arrow then that far from the right edge too
    // (README, *Limits*); else the pair's mean
    let [top, right, bottom, left] = m.padding.values;
    let items = ResolvedPadding {
        top,
        right: left.filter(|v| v.is_finite()).or(right),
        bottom,
        left,
    };
    let items = match m.outline.edge.width {
        Some(width) => padding_with_line(&items, width),
        None => padding_with_line(&items, 0.0),
    };
    s.spacing.button_padding.x = button_padding_outside(after_menu_style, &items).x;
    // E4: a stated row height is drawn exactly and wins (S2); else a least height, which egui's
    // Button grows past (egui-0.37.0/src/theme/default_style.rs:97-101,
    // src/widgets/button.rs:351); else egui's own
    match (m.row_height, m.min_row_height) {
        (None, Some((least, path))) => {
            s.spacing.interact_size.y = length(
                path,
                least,
                cx.scheme.default_style().spacing.interact_size.y,
                notes,
            );
        }
        (_, least) => {
            // a non-finite least height is reported though the row height wins over it
            if let Some((v, path)) = least
                && !v.is_finite()
            {
                push_note(notes, Note::ValueSanitised { path });
            }
            row_height(s, m.row_height, cx, notes);
        }
    }
    // E4: the panel's inset of its items, where egui frames a menu's popup: `MenuConfig::style`
    // styles a menu's and a submenu's popup (egui-0.37.0/src/containers/menu.rs:457, :714), and
    // `Popup::show` frames it with `Frame::popup`, whose inner margin is `spacing.menu_margin`
    // (src/containers/popup.rs:650-651, src/containers/frame.rs:214-220). Physical sides, as
    // every egui frame pads, in a right-to-left layout too.
    s.spacing.menu_margin = margin_of(
        m.panel_padding.paths,
        own.spacing.menu_margin,
        m.panel_padding.values,
        notes,
    );
    set_length(
        &mut s.spacing.icon_spacing,
        m.icon_gap,
        own.spacing.icon_spacing,
        notes,
    );
    slot_size(s, TextStyle::Body, m.size, own, notes); // items read Body (B3)
    fit_row_padding(s, m.row_height, TextStyle::Body, cx);
}

/// E3: a menu bar's titles over the `Role::Menu` cell ([`ThemeAtlas::menu_bar_modifier`]).
/// A title draws no line under the pointer or open (Breeze draws none; E1's item outline would
/// show), so `hovered`, `active` and `open` take `inactive`'s stroke. `hovered` and `active`
/// (also egui's keyboard-focused state, which stays on a closed title, S6) take
/// `menu.bar_hover_background`, `open` `menu.bar_open_background`, else the hover fill;
/// `menu.bar_hover_text_color` letters the three; the title's padding is
/// `menu.bar_padding_horizontal` and `menu.bar_padding_vertical`, a finite side through
/// [`clamp_length`] (a title has no line); a non-finite side keeps the cell's padding and is
/// reported when the atlas is built (`menu`). Each where stated: unstated, the cell's own look.
/// The 0.8.0 model's homes are `menu_bar.item(HOVER)`'s and `(OPEN)`'s fill,
/// `menu_bar.item_label(HOVER)`'s and `(OPEN)`'s colour (the hovered and pressed titles the
/// former, the open one the latter), and `menu_bar.item(NONE)`'s left and top padding.
///
/// [`ThemeAtlas::menu_bar_modifier`]: crate::ThemeAtlas::menu_bar_modifier
pub(crate) fn menu_bar_titles(m: &MenuBarTitles, s: &mut egui::Style) {
    let w = &mut s.visuals.widgets;
    let rest = w.inactive.bg_stroke;
    for e in [&mut w.hovered, &mut w.active, &mut w.open] {
        e.bg_stroke = rest;
    }
    let [hover_fill, open_fill] = m.fill.map(|c| c.map(to_color32));
    let [hover_text, open_text] = m.text.map(|c| c.map(to_color32));
    for (e, fill, text) in [
        (&mut w.hovered, hover_fill, hover_text),
        (&mut w.active, hover_fill, hover_text),
        (&mut w.open, open_fill, open_text),
    ] {
        if let Some(fill) = fill {
            e.weak_bg_fill = fill;
        }
        if let Some(text) = text {
            e.fg_stroke.color = text;
        }
    }
    let [x, y] = m.padding;
    let padding = &mut s.spacing.button_padding;
    if let Some(x) = x.filter(|x| x.is_finite()) {
        padding.x = clamp_length(x);
    }
    if let Some(y) = y.filter(|y| y.is_finite()) {
        padding.y = clamp_length(y);
    }
}

/// What [`menu_bar_titles`] writes, kept with a scheme's compiled styles: the titles' fill and
/// text under the pointer and open, and their horizontal and vertical padding.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct MenuBarTitles {
    pub fill: [Option<Rgba>; 2],
    pub text: [Option<Rgba>; 2],
    pub padding: [Option<f32>; 2],
}

impl MenuBarTitles {
    /// The titles of a menu cell's reads.
    pub(crate) fn of(m: &super::cells::MenuCell) -> Self {
        Self {
            fill: m.bar_fill,
            text: m.bar_text,
            padding: m.bar_padding.map(|side| side.map(|(v, _)| v)),
        }
    }
}

fn input_role(
    s: &mut egui::Style,
    own: &egui::Style,
    src: &CellSource,
    cx: &Cx<'_>,
    notes: &mut Vec<Note>,
) {
    let i = &src.input;
    write_states_with(
        &mut s.visuals.widgets,
        &StateSource {
            fill: None,
            text: text_states(i.text[0], [i.text[1], i.text[2]]),
            border: border_all(&i.outline),
            open: OpenFrom::Inactive,
        },
        cx.unstated,
        notes,
    );
    // the hovered and pressed field's border (§6.4: `None` copies `border.color`)
    state_borders(
        &mut s.visuals.widgets,
        i.state_border,
        i.outline.align,
        None,
        cx.unstated,
        notes,
    );
    if let Some(c) = i.placeholder {
        s.visuals.weak_text_color = Some(to_color32(c));
    }
    set_selection(s, i.selection, i.selection_text);
    s.spacing.button_padding = padding_of(&i.padding, &i.outline, own, notes);
    slot_size(s, TextStyle::Body, i.size, own, notes);
    set_opacity(s, i.disabled_opacity, own, notes);
}

fn checkbox(
    s: &mut egui::Style,
    own: &egui::Style,
    src: &CellSource,
    cx: &Cx<'_>,
    notes: &mut Vec<Note>,
) {
    let k = &src.checkbox;
    write_states_with(
        &mut s.visuals.widgets,
        &StateSource {
            fill: Some(FillSource {
                field: FillField::Bg,
                idle: k.fill[0],
                hover: k.fill[1],
                active: k.fill[2],
                layer: cx.layer,
            }),
            text: text_states(k.mark[0], [k.mark[1], k.mark[2]]), // the check mark is `fg_stroke` (§5.3)
            border: border_all(&k.outline),
            open: OpenFrom::Inactive,
        },
        cx.unstated,
        notes,
    );
    // the hovered and pressed box's outline where stated (§6.4: `None` keeps the unchecked
    // border `write_states` copied); `active` is also egui's keyboard-focused state, so a box
    // reached by Tab shows the hover outline, which Breeze draws under the pointer only
    state_borders(
        &mut s.visuals.widgets,
        k.state_border,
        k.outline.align,
        Some(FillField::Bg),
        cx.unstated,
        notes,
    );
    // The check mark is a line in the state's `fg_stroke` (`egui/src/widgets/checkbox.rs:151-158`,
    // `check_stroke` from `widget_style.rs:131`, `:188`), so its width is the stroke width, one in
    // every state (§6.1); the radio's dot takes only the stroke's colour (`radio_button.rs:97-103`).
    if let Some(width) = k
        .mark_width
        .map(|(v, path)| length(path, v, own.visuals.widgets.inactive.fg_stroke.width, notes))
    {
        let w = &mut s.visuals.widgets;
        for e in [
            &mut w.noninteractive,
            &mut w.inactive,
            &mut w.hovered,
            &mut w.active,
            &mut w.open,
        ] {
            e.fg_stroke.width = width;
        }
    }
    if let Some(c) = k.label.color {
        s.visuals.override_text_color = Some(to_color32(c)); // the label (B4)
    }
    override_font(s, k.label.size, notes);
    if let Some((v, path)) = k.indicator {
        s.spacing.icon_width = length(path, v, own.spacing.icon_width, notes);
        // The row: `Checkbox` and `RadioButton` take their minimum size from
        // `Vec2::splat(interact_size.y)` (`widgets/checkbox.rs:85`, `widgets/radio_button.rs:54`),
        // which the base style fills from `button.min_height`. The model states no row height
        // for either, so the floor is the indicator the theme states, and the row grows round a
        // taller label as egui lays it out.
        s.spacing.interact_size.y = s.spacing.icon_width;
    }
    set_length(
        &mut s.spacing.icon_spacing,
        k.label_gap,
        own.spacing.icon_spacing,
        notes,
    );
    set_opacity(s, k.disabled_opacity, own, notes);
}

/// A surface role's text (§6.1): `noninteractive`'s colour and the Body size.
fn surface_text(s: &mut egui::Style, own: &egui::Style, text: &Text, notes: &mut Vec<Note>) {
    set_color(
        &mut s.visuals.widgets.noninteractive.fg_stroke.color,
        text.color,
    );
    slot_size(s, TextStyle::Body, text.size, own, notes);
}

fn dialog(s: &mut egui::Style, own: &egui::Style, src: &CellSource, notes: &mut Vec<Note>) {
    let d = &src.dialog;
    surface_text(s, own, &d.body, notes);
    slot_size(s, TextStyle::Heading, d.title_size, own, notes);
    set_length(
        &mut s.spacing.item_spacing.x,
        d.button_gap,
        own.spacing.item_spacing.x,
        notes,
    );
}

fn scrollbar(
    s: &mut egui::Style,
    own: &egui::Style,
    src: &CellSource,
    cx: &Cx<'_>,
    notes: &mut Vec<Note>,
) {
    let sb = &src.scrollbar;
    // the base's own rows again, in the cell (§5.5, §5.9)
    set_color(&mut s.visuals.extreme_bg_color, sb.track);
    let [idle, hover, _] = sb.thumb;
    write_states_with(
        &mut s.visuals.widgets,
        &StateSource {
            fill: Some(FillSource {
                field: FillField::Bg,
                idle,
                hover,
                active: None, // the pressed thumb is D5 (§6.9), in `derived::apply_role`
                layer: Layer::AsGiven,
            }),
            text: None,
            border: None,
            open: OpenFrom::Inactive,
        },
        cx.unstated,
        notes,
    );
    set_length(
        &mut s.spacing.scroll.handle_min_length,
        sb.min_thumb,
        own.spacing.scroll.handle_min_length,
        notes,
    );
}

fn slider(
    s: &mut egui::Style,
    own: &egui::Style,
    src: &CellSource,
    cx: &Cx<'_>,
    notes: &mut Vec<Note>,
) {
    let sl = &src.slider;
    set_color(&mut s.visuals.selection.bg_fill, sl.fill); // `slider_trailing_fill` is already on (§5.9)
    write_states_with(
        &mut s.visuals.widgets,
        &StateSource {
            fill: Some(FillSource {
                field: FillField::Bg,
                idle: sl.track, // I1: the rail
                hover: None,    // the hovered and pressed handle: D6 (§6.9), `derived::apply_role`
                active: None,
                layer: Layer::AsGiven,
            }),
            text: None,
            border: None,
            open: OpenFrom::Inactive,
        },
        cx.unstated,
        notes,
    );
    set_opacity(s, sl.disabled_opacity, own, notes);
}

fn progress_bar(s: &mut egui::Style, own: &egui::Style, src: &CellSource, notes: &mut Vec<Note>) {
    let p = &src.progress_bar;
    set_color(&mut s.visuals.selection.bg_fill, p.fill);
    set_color(&mut s.visuals.extreme_bg_color, p.track);
    set_length(
        &mut s.spacing.interact_size.y,
        p.height,
        own.spacing.interact_size.y,
        notes,
    );
    set_length(
        &mut s.spacing.item_spacing.x,
        p.gap,
        own.spacing.item_spacing.x,
        notes,
    );
}

/// A role's hovered and pressed outline over its fill (`weak_bg_fill`), each property apart, as
/// [`state_borders`] writes it, except that a property the role states at none of rest, hover
/// and press keeps the base's in both entries (M4.E2c); one it states at any of them is
/// [`state_borders`]' in both, an unstated state's by rule 0.
fn role_state_borders(
    widgets: &mut Widgets,
    lines: [Option<StateLine>; 2],
    outline: &Outline,
    unstated: Unstated<'_>,
    notes: &mut Vec<Note>,
) {
    let base = [widgets.hovered.bg_stroke, widgets.active.bg_stroke];
    state_borders(
        widgets,
        lines,
        outline.align,
        Some(FillField::Weak),
        unstated,
        notes,
    );
    let lines = lines.map(Option::unwrap_or_default);
    let no_color = outline.edge.color.is_none() && lines.iter().all(|l| l.color.is_none());
    let no_width = outline.edge.width.is_none() && lines.iter().all(|l| l.width.is_none());
    let entries = [&mut widgets.hovered, &mut widgets.active];
    for (entry, base) in entries.into_iter().zip(base) {
        if no_color {
            entry.bg_stroke.color = base.color;
        }
        if no_width {
            entry.bg_stroke.width = base.width;
        }
    }
}

fn tab(
    s: &mut egui::Style,
    own: &egui::Style,
    src: &CellSource,
    cx: &Cx<'_>,
    notes: &mut Vec<Note>,
) {
    let tb = &src.tab;
    write_states_with(
        &mut s.visuals.widgets,
        &StateSource {
            fill: Some(FillSource {
                field: FillField::Weak,
                idle: tb.fill[0],
                hover: tb.fill[1],
                active: tb.fill[2],
                // a row highlight (C17): it replaces the idle tab's fill and lies over the tab
                // bar, as Breeze paints it (docs/platform-facts.md §2.11, `background_color`)
                layer: Layer::AsGiven,
            }),
            text: Some(TextSource {
                idle: tb.text[0],
                hover: tb.text[1],
                active: tb.text[2],
                in_noninteractive: true,
            }),
            border: border_all(&tb.outline),
            open: OpenFrom::Inactive,
        },
        cx.unstated,
        notes,
    );
    // a tab is drawn as a button (§6.2): its hovered and pressed outline as the button's
    role_state_borders(
        &mut s.visuals.widgets,
        tb.state_border,
        &tb.outline,
        cx.unstated,
        notes,
    );
    // the selected tab: `button_style` reads `selection.*` (§6.2)
    set_selection(s, tb.selected_fill, tb.selected_text);
    set_color(&mut s.visuals.panel_fill, tb.bar_fill);
    set_length(
        &mut s.spacing.interact_size.y,
        tb.min_height,
        own.spacing.interact_size.y,
        notes,
    );
    // the space between neighbouring tabs, which a tab row lays out in the scope
    set_length(
        &mut s.spacing.item_spacing.x,
        tb.item_gap,
        own.spacing.item_spacing.x,
        notes,
    );
    s.spacing.button_padding = padding_of(&tb.padding, &tb.outline, own, notes);
    slot_size(s, TextStyle::Body, tb.size, own, notes);
}

/// A panel role's fill, text and separator line (§6.1), after `fill` writes its entries' fills.
fn panel(
    s: &mut egui::Style,
    own: &egui::Style,
    p: &super::cells::PanelCell,
    fill: Option<FillSource>,
    text: [Option<Rgba>; 2],
    cx: &Cx<'_>,
    notes: &mut Vec<Note>,
) {
    set_color(&mut s.visuals.panel_fill, p.fill);
    write_states_with(
        &mut s.visuals.widgets,
        &StateSource {
            fill,
            text: text_states(p.text.color, text),
            border: None, // a container edge (§6.1)
            open: OpenFrom::Inactive,
        },
        cx.unstated,
        notes,
    );
    panel_edge(s, own, &p.edge, notes);
    slot_size(s, TextStyle::Body, p.text.size, own, notes);
}

fn sidebar(
    s: &mut egui::Style,
    own: &egui::Style,
    src: &CellSource,
    cx: &Cx<'_>,
    notes: &mut Vec<Note>,
) {
    let sb = &src.sidebar;
    set_selection(s, sb.selection, sb.selection_text);
    let fill = FillSource {
        field: FillField::Weak,
        idle: None,
        hover: sb.item_fill[0],
        active: sb.item_fill[1],
        layer: Layer::AsGiven, // a row highlight (§6.1)
    };
    panel(s, own, &sb.panel, Some(fill), sb.item_text, cx, notes);
}

fn toolbar(
    s: &mut egui::Style,
    own: &egui::Style,
    src: &CellSource,
    cx: &Cx<'_>,
    notes: &mut Vec<Note>,
) {
    let tb = &src.toolbar;
    if !tb.bar_height.is_some_and(|(h, _)| h.is_finite()) {
        // A stated height is R-BAR's (§6.10, `derived::apply_role`), which also reports a
        // non-finite one.
        row_height(s, None, cx, notes);
    }
    set_length(
        &mut s.spacing.item_spacing.x,
        tb.item_gap,
        own.spacing.item_spacing.x,
        notes,
    );
    panel(s, own, &tb.panel, None, tb.button_text, cx, notes);
    // D7: a flat tool button's hovered and pressed states, each field where stated and as
    // stated, not composited; the rest is the button's own
    let w = &mut s.visuals.widgets;
    let [hover, active] = tb.button_fill;
    set_color(&mut w.hovered.weak_bg_fill, hover);
    set_color(&mut w.active.weak_bg_fill, active);
    // the colour and the width each where stated and as stated, an unstated one what `unstated`
    // gives; `active` is also egui's keyboard-focused state, so a tool button reached by Tab
    // shows the outline, which a Qt toolbar button, taking no focus, never shows
    if tb.button_border.iter().any(Option::is_some) {
        let [hover, active] = tb.button_border.map(Option::unwrap_or_default);
        let own = |e: &egui::style::WidgetVisuals| e.bg_stroke.color;
        let rest = w.inactive.bg_stroke.color;
        let hovered = hover
            .color
            .map_or_else(|| cx.unstated.hovered(rest, own), to_color32);
        let pressed = active
            .color
            .map_or_else(|| cx.unstated.active(rest, hovered, own), to_color32);
        let own_width = |e: &egui::style::WidgetVisuals| e.bg_stroke.width;
        let mut width = |state: Read<f32>, entry: &egui::style::WidgetVisuals| {
            state.map(|(v, path)| {
                stroke(path, entry.bg_stroke, entry.bg_stroke.color, v, notes).width
            })
        };
        let hovered_width = width(hover.width, &w.hovered)
            .unwrap_or_else(|| cx.unstated.hovered(w.hovered.bg_stroke.width, own_width));
        // the 0.7.2 model states no width: its entries keep their own, as before
        let resting = w.active.bg_stroke.width;
        let pressed_width = width(active.width, &w.active)
            .unwrap_or_else(|| cx.unstated.active(resting, resting, own_width));
        w.hovered.bg_stroke = egui::Stroke::new(hovered_width, hovered);
        w.active.bg_stroke = egui::Stroke::new(pressed_width, pressed);
    }
}

fn status_bar(
    s: &mut egui::Style,
    own: &egui::Style,
    src: &CellSource,
    cx: &Cx<'_>,
    notes: &mut Vec<Note>,
) {
    panel(s, own, &src.status_bar, None, [None; 2], cx, notes);
}

fn list(
    s: &mut egui::Style,
    own: &egui::Style,
    src: &CellSource,
    cx: &Cx<'_>,
    notes: &mut Vec<Note>,
) {
    let l = &src.list;
    set_selection(s, l.selection, l.selection_text);
    // the table lines: `grid_color` as given, the list's width and radius (§5.4, §6.1)
    let own_edge = &own.visuals.widgets.noninteractive;
    let edge = &mut s.visuals.widgets.noninteractive;
    set_color(&mut edge.bg_stroke.color, l.grid);
    set_length(
        &mut edge.bg_stroke.width,
        l.width,
        own_edge.bg_stroke.width,
        notes,
    );
    if let Some((r, path)) = l.radius {
        edge.corner_radius = radius(path, own_edge.corner_radius, r, notes);
    }
    row_height(s, l.row_height, cx, notes);
    write_states_with(
        &mut s.visuals.widgets,
        &StateSource {
            fill: Some(FillSource {
                field: FillField::Bg,
                idle: None,
                hover: l.highlight[0],
                active: l.highlight[1],
                layer: Layer::AsGiven, // a row highlight (§6.1)
            }),
            text: None,
            border: None,
            open: OpenFrom::Inactive,
        },
        cx.unstated,
        notes,
    );
    set_color(
        &mut s.visuals.widgets.noninteractive.fg_stroke.color,
        l.item.color,
    ); // the cells are `Label`s
    set_color(&mut s.visuals.widgets.active.fg_stroke.color, l.header_text); // `strong_text_color` (§5.4)
    slot_size(s, TextStyle::Body, l.item.size, own, notes);
    fit_row_padding(s, l.row_height, TextStyle::Body, cx);
}

fn separator(s: &mut egui::Style, own: &egui::Style, src: &CellSource, notes: &mut Vec<Note>) {
    let sp = &src.separator;
    // the line keeps its own colour: no opacity fold (§5.1)
    let line = &mut s.visuals.widgets.noninteractive.bg_stroke;
    set_length(
        &mut line.width,
        sp.width,
        own.visuals.widgets.noninteractive.bg_stroke.width,
        notes,
    );
    set_color(&mut line.color, sp.color);
}

fn switch(
    s: &mut egui::Style,
    own: &egui::Style,
    src: &CellSource,
    cx: &Cx<'_>,
    notes: &mut Vec<Note>,
) {
    let sw = &src.switch;
    set_color(&mut s.visuals.selection.bg_fill, sw.checked); // the checked track (§6.2)
    write_states_with(
        &mut s.visuals.widgets,
        &StateSource {
            fill: Some(FillSource {
                field: FillField::Weak,
                idle: sw.fill[0],
                hover: sw.fill[1],
                active: sw.fill[2],
                layer: cx.layer,
            }),
            text: None,
            border: None,
            open: OpenFrom::Inactive,
        },
        cx.unstated,
        notes,
    );
    if let Some((r, path)) = sw.radius {
        let w = &mut s.visuals.widgets;
        for e in [
            &mut w.noninteractive,
            &mut w.inactive,
            &mut w.hovered,
            &mut w.active,
            &mut w.open,
        ] {
            e.corner_radius = radius(path, e.corner_radius, r, notes);
        }
    }
    set_length(
        &mut s.spacing.interact_size.y,
        sw.height,
        own.spacing.interact_size.y,
        notes,
    );
    set_opacity(s, sw.disabled_opacity, own, notes);
}

fn spinner(s: &mut egui::Style, src: &CellSource) {
    // the arc: `strong_text_color`, `active.fg_stroke.color` (I2); its size is D3 (§6.7), in
    // `derived::apply_role`
    set_color(
        &mut s.visuals.widgets.active.fg_stroke.color,
        src.spinner.color,
    );
}

fn combo_box(
    s: &mut egui::Style,
    own: &egui::Style,
    src: &CellSource,
    cx: &Cx<'_>,
    notes: &mut Vec<Note>,
) {
    let cb = &src.combo_box;
    write_states_with(
        &mut s.visuals.widgets,
        &StateSource {
            fill: Some(FillSource {
                field: FillField::Weak,
                idle: cb.fill[0],
                hover: cb.fill[1],
                active: cb.fill[2],
                layer: cx.layer,
            }),
            text: text_states(cb.text.color, cb.text_states),
            border: border_all(&cb.outline),
            open: OpenFrom::Inactive,
        },
        cx.unstated,
        notes,
    );
    // the hovered and pressed trigger's outline where stated (§6.4: `None` keeps the border
    // `write_states` copied); an open combo box is egui's `open` state, written below
    state_borders(
        &mut s.visuals.widgets,
        cb.state_border,
        cb.outline.align,
        Some(FillField::Weak),
        cx.unstated,
        notes,
    );
    // E10: an open combo box, drawn from `open` under `ComboBox::CLASS_OPEN` whatever the
    // pointer does (egui-0.37.0/src/theme/default_style.rs:41-45), as Breeze draws it: the fill
    // as the hover's reaches its entry (a 0.7.2 state layer over the idle fill, opaque is exact),
    // the border at the width `write_states` put there. Unstated, `open` keeps the resting look.
    if let Some(fill) = cb.open_fill {
        s.visuals.widgets.open.weak_bg_fill = cx.layer.paint(fill, cb.fill[0].map(|(c, _)| c));
    }
    set_color(&mut s.visuals.widgets.open.bg_stroke.color, cb.open_border);
    set_length(
        &mut s.spacing.interact_size.y,
        cb.min_height,
        own.spacing.interact_size.y,
        notes,
    );
    set_length(
        &mut s.spacing.icon_width,
        cb.arrow,
        own.spacing.icon_width,
        notes,
    );
    s.spacing.button_padding = padding_of(&cb.padding, &cb.outline, own, notes);
    set_opacity(s, cb.disabled_opacity, own, notes);
    // B3: egui 0.37 draws the selected text, and the popup's items, with `ButtonStyle`'s font,
    // `override_font_id` before Body (`egui-0.37.0/src/theme/default_style.rs`,
    // `StyleProvider<ButtonStyle>`; `AtomLayoutStyle::apply`,
    // `egui-0.37.0/src/widget_style/mod.rs`, replaces the `TextStyle::Button` fallback
    // `combo_box_dyn` sets)
    override_font(s, cb.text.size, notes);
}

fn segmented_control(
    s: &mut egui::Style,
    own: &egui::Style,
    src: &CellSource,
    cx: &Cx<'_>,
    notes: &mut Vec<Note>,
) {
    let sc = &src.segmented_control;
    write_states_with(
        &mut s.visuals.widgets,
        &StateSource {
            fill: Some(FillSource {
                field: FillField::Weak,
                idle: sc.fill[0],
                hover: sc.fill[1],
                active: sc.fill[2],
                layer: cx.layer,
            }),
            text: text_states(sc.text.color, sc.text_states),
            border: border_all(&sc.outline),
            open: OpenFrom::Inactive,
        },
        cx.unstated,
        notes,
    );
    role_state_borders(
        &mut s.visuals.widgets,
        sc.state_border,
        &sc.outline,
        cx.unstated,
        notes,
    );
    // the selected segment: `button_style` reads `selection.*` (§6.2)
    set_selection(s, sc.selected_fill, sc.selected_text);
    set_length(
        &mut s.spacing.interact_size.y,
        sc.height,
        own.spacing.interact_size.y,
        notes,
    );
    set_length(
        &mut s.spacing.item_spacing.x,
        sc.separator,
        own.spacing.item_spacing.x,
        notes,
    );
    override_font(s, sc.text.size, notes);
    s.spacing.button_padding = padding_of(&sc.padding, &sc.outline, own, notes);
    set_opacity(s, sc.disabled_opacity, own, notes);
}

fn expander(
    s: &mut egui::Style,
    own: &egui::Style,
    src: &CellSource,
    cx: &Cx<'_>,
    notes: &mut Vec<Note>,
) {
    let x = &src.expander;
    // the header's one frame layer, with no resting fill (§6.1, `collapsing_header.rs:561-568`)
    s.visuals.collapsing_header_frame = true;
    s.visuals.widgets.inactive.weak_bg_fill = egui::Color32::TRANSPARENT;
    s.visuals.widgets.open.weak_bg_fill = egui::Color32::TRANSPARENT; // `open` copies it (§6.1)
    write_states_with(
        &mut s.visuals.widgets,
        &StateSource {
            fill: Some(FillSource {
                field: FillField::Weak,
                idle: None,
                hover: x.fill[0],
                active: x.fill[1],
                layer: Layer::AsGiven, // over no idle fill of its own (§6.1)
            }),
            text: text_states(x.text.color, x.text_states),
            border: border_all(&x.outline),
            open: OpenFrom::Inactive,
        },
        cx.unstated,
        notes,
    );
    // The header draws no outline, whatever the theme states of `expander.frame_enabled`:
    // egui's frame layer strokes the header alone (`collapsing_header.rs:561-568`), a shape no
    // platform draws; a stated frame holds header and body together, the companion crate's
    // `Expander` drawing it in the `noninteractive` stroke, which keeps the border's width.
    let w = &mut s.visuals.widgets;
    for e in [&mut w.inactive, &mut w.hovered, &mut w.active, &mut w.open] {
        e.bg_stroke.width = 0.0;
    }
    set_length(
        &mut s.spacing.interact_size.y,
        x.header_height,
        own.spacing.interact_size.y,
        notes,
    );
    slot_size(s, TextStyle::Button, x.text.size, own, notes);
    // `.y` by the pair rule; `.x` is read on the trailing side only (§5.6, §7.4), so the
    // right side alone; `.left` is `spacing.indent`, in `derived::apply_role`
    let [top, right, bottom, _] = x.padding.values;
    let [top_path, right_path, bottom_path, _] = x.padding.paths;
    for (side, path) in [(top, top_path), (right, right_path), (bottom, bottom_path)] {
        if side.is_some_and(|v| !v.is_finite()) {
            push_note(notes, Note::ValueSanitised { path });
        }
    }
    // a stated side lies inside the border only where the frame is drawn (M9-178)
    let own_padding = own.spacing.button_padding;
    let padding = x.outside_padding();
    s.spacing.button_padding = egui::vec2(
        padding.right.map_or(own_padding.x, clamp_length),
        button_padding_outside(own_padding, &padding).y,
    );
    // the header's title is laid out in the Button font
    // (`egui/src/containers/collapsing_header.rs:518-523`)
    fit_row_padding(s, x.header_height, TextStyle::Button, cx);
}

fn link(s: &mut egui::Style, src: &CellSource, notes: &mut Vec<Note>) {
    set_color(&mut s.visuals.hyperlink_color, src.link.color);
    override_font(s, src.link.size, notes);
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
    use egui::{FontFamily, FontId, TextStyle};
    use native_theme::theme::ColorMode;

    use super::*;
    use crate::convert::{
        clamp_length, composite_over, expander_padding_border, to_button_padding, to_color32,
        to_corner_radius, unit_interval,
    };
    use crate::install_tests::resolved;
    use crate::style::BuildInput;
    use crate::style::base::base_style;
    use crate::{AccessibilityPreferences, LayoutTheme, Note, ResolvedTheme, Role};

    /// (cell, base, notes) of one role on one preset and mode.
    fn cell(
        preset: &str,
        mode: ColorMode,
        role: Role,
    ) -> (egui::Style, egui::Style, ResolvedTheme, Vec<Note>) {
        cell_of(resolved(preset, mode), mode, role)
    }

    /// [`cell`] of a resolved theme, which a test may have edited.
    fn cell_of(
        t: ResolvedTheme,
        mode: ColorMode,
        role: Role,
    ) -> (egui::Style, egui::Style, ResolvedTheme, Vec<Note>) {
        let scheme = if mode == ColorMode::Dark {
            egui::Theme::Dark
        } else {
            egui::Theme::Light
        };
        let prefs = AccessibilityPreferences::default();
        let layout = LayoutTheme::default();
        let input = BuildInput {
            scheme,
            theme: &t,
            prefs: &prefs,
            layout: &layout,
            row_height: None,
            defs: None,
            patch: None,
        };
        let mut notes = Vec::new();
        let base = base_style(&input, &mut notes);
        let c = role_cell(role, &base, &input, &mut notes);
        (c, base, t, notes)
    }

    /// The expander cell's padding counts the border's width inside a stated side only where
    /// `expander.frame_enabled` is true, the frame the companion crate draws round header and
    /// body; elsewhere the header draws no outline (M9-178), and the side is the padding alone,
    /// as the iced connector's since M9-176 (round 4 review, item 17 minor 7): windows-11
    /// states the left side 16 and the right 0, its border 1 wide.
    #[test]
    fn the_expander_padding_counts_the_border_only_where_the_frame_is_drawn() {
        let t = resolved("windows-11", ColorMode::Light);
        let width = t.expander.border.line_width;
        assert!(width > 0.0, "windows-11's expander border has a width");
        let arrow = t.expander.arrow_icon_size;
        for (frame, inside) in [(Some(true), width), (Some(false), 0.0), (None, 0.0)] {
            let mut t = t.clone();
            t.expander.frame_enabled = frame;
            let prefs = AccessibilityPreferences::default();
            let layout = LayoutTheme::default();
            let input = BuildInput {
                scheme: egui::Theme::Light,
                theme: &t,
                prefs: &prefs,
                layout: &layout,
                row_height: None,
                defs: None,
                patch: None,
            };
            let mut notes = Vec::new();
            let base = base_style(&input, &mut notes);
            let c = role_cell(Role::Expander, &base, &input, &mut notes);
            let p = &t.expander.border.padding;
            let (Some(left), Some(right)) = (p.left, p.right) else {
                panic!("windows-11 states the expander's left and right padding");
            };
            assert_eq!(
                c.spacing.indent,
                clamp_length(2.0 * (left + inside) + arrow),
                "frame_enabled {frame:?}: the leading padding"
            );
            assert_eq!(
                c.spacing.button_padding.x,
                clamp_length(right + inside),
                "frame_enabled {frame:?}: the trailing padding"
            );
        }
    }

    /// The five entries, in `Widgets`' field order.
    fn all5(w: &egui::style::Widgets) -> [&egui::style::WidgetVisuals; 5] {
        [
            &w.noninteractive,
            &w.inactive,
            &w.hovered,
            &w.active,
            &w.open,
        ]
    }

    /// §5.3, §6.1: egui's strong text is `widgets.active.text_color()` (`egui/src/style.rs:1147-1149`),
    /// so the base style writes the panel's text colour there; windows-11 light's pressed-button
    /// text `#0000009e` (`native-theme/src/presets/windows-11.toml:132`), against its window text
    /// `#000000e4`, stays in the Button cell.
    #[test]
    fn strong_text_is_the_panel_text_and_the_button_cell_keeps_its_pressed_pair() {
        let (c, base, t, _) = cell("windows-11", ColorMode::Light, Role::Button);
        assert_ne!(t.defaults.text_color, t.button.active_text_color);
        assert_eq!(
            base.visuals.strong_text_color(),
            to_color32(t.defaults.text_color)
        );
        assert_eq!(
            base.visuals.widgets.active.fg_stroke.color,
            to_color32(t.defaults.text_color)
        );
        assert_eq!(
            c.visuals.widgets.active.fg_stroke.color,
            to_color32(t.button.active_text_color)
        );
    }

    /// §6.4: a transparent box fill is reported under the leaf whose value it is — the
    /// widget's own `background_color` when `unchecked_background` is `None`.
    #[test]
    fn a_transparent_checkbox_fill_names_the_leaf_it_came_from() {
        let mut t = resolved("kde-breeze", ColorMode::Light);
        t.checkbox.unchecked_background = None;
        t.checkbox.background_color = Rgba {
            a: 0,
            ..t.checkbox.background_color
        };
        let prefs = AccessibilityPreferences::default();
        let layout = LayoutTheme::default();
        let input = BuildInput {
            scheme: egui::Theme::Light,
            theme: &t,
            prefs: &prefs,
            layout: &layout,
            row_height: None,
            defs: None,
            patch: None,
        };
        let mut notes = Vec::new();
        let base = base_style(&input, &mut notes);
        let mut notes = Vec::new();
        let _ = role_cell(Role::Checkbox, &base, &input, &mut notes);
        assert!(
            notes.contains(&Note::TransparentFill {
                path: "checkbox.background_color"
            }),
            "{notes:?}"
        );
        assert!(
            !notes.contains(&Note::TransparentFill {
                path: "checkbox.unchecked_background"
            }),
            "{notes:?}"
        );
    }

    /// The presets every per-role test reads: a light and a dark one from each platform
    /// family, so a soft option is met both stated and `None`.
    const PRESETS: [(&str, ColorMode); 6] = [
        ("adwaita", ColorMode::Light),
        ("kde-breeze", ColorMode::Dark),
        ("windows-11", ColorMode::Light),
        ("macos-monterey", ColorMode::Dark),
        ("material-3", ColorMode::Light),
        ("catppuccin-mocha", ColorMode::Dark),
    ];

    /// §5.3's Button rows, each at the entries §6.1 names, plus `selection.*` (§6.2) and B3.
    #[test]
    fn button_cell_writes_its_rows() {
        // a theme that states the hover border alone, which no preset does: `active` takes it
        let mut hover_only = resolved("adwaita", ColorMode::Light);
        hover_only.button.hover_border_color = Some(Rgba::rgb(255, 0, 0));
        hover_only.button.focus_border_color = None;
        for (preset, mode, t) in [
            (
                "adwaita",
                ColorMode::Light,
                resolved("adwaita", ColorMode::Light),
            ),
            (
                "windows-11",
                ColorMode::Dark,
                resolved("windows-11", ColorMode::Dark),
            ),
            (
                "kde-breeze",
                ColorMode::Dark,
                resolved("kde-breeze", ColorMode::Dark),
            ),
            (
                "adwaita, the hover border alone",
                ColorMode::Light,
                hover_only,
            ),
        ] {
            let (c, base, t, notes) = cell_of(t, mode, Role::Button);
            let b = &t.button;
            let w = &c.visuals.widgets;
            assert!(notes.is_empty(), "{preset}: {notes:?}");
            // fills: idle into {inactive,open}, layers composited into hovered/active (C17)
            assert_eq!(w.inactive.weak_bg_fill, to_color32(b.background_color));
            assert_eq!(
                w.open.weak_bg_fill,
                to_color32(b.background_color),
                "open copies inactive here, not the title bar"
            );
            assert_eq!(
                w.hovered.weak_bg_fill,
                composite_over(b.hover_background, b.background_color)
            );
            assert_eq!(
                w.active.weak_bg_fill,
                composite_over(
                    b.active_background.unwrap_or(b.hover_background),
                    b.background_color
                )
            );
            // text: {noninteractive,inactive,open} ← font.color; hovered/active ← their leaves
            for e in [&w.noninteractive, &w.inactive, &w.open] {
                assert_eq!(e.fg_stroke.color, to_color32(b.font.color));
            }
            assert_eq!(w.hovered.fg_stroke.color, to_color32(b.hover_text_color));
            assert_eq!(w.active.fg_stroke.color, to_color32(b.active_text_color));
            // border in all five entries, colour as stated (§6.13 folds only `defaults.border.color`);
            // hovered the hover border, active the focus border, else the hover border
            let hover = b.hover_border_color.unwrap_or(b.border.color);
            let focus = b
                .focus_border_color
                .or(b.hover_border_color)
                .unwrap_or(b.border.color);
            for (e, colour) in [
                (&w.noninteractive, b.border.color),
                (&w.inactive, b.border.color),
                (&w.hovered, hover),
                (&w.active, focus),
                (&w.open, b.border.color),
            ] {
                assert_eq!(e.bg_stroke.color, to_color32(colour), "{preset}");
                assert_eq!(e.bg_stroke.width, clamp_length(b.border.line_width));
                assert_eq!(
                    e.corner_radius,
                    to_corner_radius(
                        base.visuals.widgets.inactive.corner_radius,
                        b.border.corner_radius
                    )
                );
            }
            if preset == "kde-breeze" {
                // Breeze's highlight (docs/platform-facts.md §2.3, `hover_border_color`)
                let highlight = egui::Color32::from_rgb(0x3d, 0xae, 0xe9);
                assert_eq!(w.hovered.bg_stroke.color, highlight);
                assert_eq!(w.active.bg_stroke.color, highlight);
            }
            // the selected (primary) button (§6.2)
            assert_eq!(
                c.visuals.selection.bg_fill,
                to_color32(b.primary_background)
            );
            assert_eq!(
                c.visuals.selection.stroke.color,
                to_color32(b.primary_text_color)
            );
            assert_eq!(
                c.visuals.selection.stroke.width,
                base.visuals.selection.stroke.width
            );
            // sizes and gaps
            assert_eq!(c.spacing.interact_size.y, clamp_length(b.min_height));
            assert_eq!(c.spacing.icon_spacing, clamp_length(b.icon_text_gap));
            assert_eq!(
                c.spacing.button_padding,
                to_button_padding(base.spacing.button_padding, &b.border)
            );
            assert_eq!(c.visuals.disabled_alpha, unit_interval(b.disabled_opacity));
            // B3: the font travels on override_font_id, never on text_styles[Button]
            assert_eq!(
                c.override_font_id,
                Some(FontId::new(b.font.size, FontFamily::Proportional))
            );
            assert_eq!(c.text_styles, base.text_styles);
            // inherited from the base, untouched (§3.4)
            assert_eq!(c.visuals.panel_fill, base.visuals.panel_fill);
            assert_eq!(c.visuals.window_fill, base.visuals.window_fill);
            assert_eq!(c.visuals.hyperlink_color, base.visuals.hyperlink_color);
            assert_eq!(
                c.spacing.extra_text_line_spacing,
                base.spacing.extra_text_line_spacing
            );
            assert_eq!(
                w.inactive.bg_fill, base.visuals.widgets.inactive.bg_fill,
                "the thumb colour stays"
            );
        }
    }

    /// §5.2's Menu rows over `menu_style` (§3.4): the item's border, radius and padding, the
    /// hover highlight as given into {hovered,active,open}, its outline there where the theme
    /// states one (E1), and the frame left to the base.
    #[test]
    fn menu_cell_applies_menu_style_then_its_rows() {
        let (c, base, t, notes) = cell("kde-breeze", ColorMode::Light, Role::Menu);
        let m = &t.menu;
        let w = &c.visuals.widgets;
        assert!(notes.is_empty(), "{notes:?}");
        // menu_style's resting item stands: transparent, no outline unless the theme states one
        assert_eq!(w.inactive.weak_bg_fill, egui::Color32::TRANSPARENT);
        // the row highlight, as given (C17), in the three entries; open copies hovered
        for e in [&w.hovered, &w.active, &w.open] {
            assert_eq!(e.weak_bg_fill, to_color32(m.hover_background));
            assert_eq!(e.fg_stroke.color, to_color32(m.hover_text_color));
        }
        for e in [&w.noninteractive, &w.inactive] {
            assert_eq!(e.fg_stroke.color, to_color32(m.font.color));
        }
        // E1: the hovered, open and pressed item outlined in Breeze's `focusOutlineColor()`
        // at `defaults.border.line_width` (docs/platform-facts.md §2.6), the resting item's
        // border unchanged; the radius in the four interactive entries, 4.5 drawn 5
        let outline = egui::Stroke::new(1.0, egui::Color32::from_rgb(0x39, 0x9a, 0xcc));
        for e in [&w.hovered, &w.open, &w.active] {
            assert_eq!(e.bg_stroke, outline);
        }
        assert_eq!(w.inactive.bg_stroke.color, to_color32(m.border.color));
        assert_eq!(
            w.inactive.bg_stroke.width,
            clamp_length(m.border.line_width)
        );
        for e in [&w.inactive, &w.hovered, &w.active, &w.open] {
            assert_eq!(e.corner_radius, egui::CornerRadius::same(5));
        }
        assert_eq!(
            w.noninteractive.bg_stroke.color,
            to_color32(m.separator_color)
        );
        // E2 is not drawn (S6): the pressed item is the hovered one, outline included, though
        // the theme states Breeze's pressed look
        assert!(m.active_background.is_some() && m.active_text_color.is_some());
        assert_eq!(w.active, w.hovered);
        assert_eq!(c.spacing.icon_spacing, clamp_length(m.icon_text_gap));
        assert_eq!(
            c.text_styles.get(&TextStyle::Body).unwrap().size,
            m.font.size
        );
        assert_eq!(c.override_font_id, None, "menu items read Body (B3)");
        // the frame is the base style's (§5.2, §5.9), its items inset by the panel padding (E4)
        assert_eq!(c.visuals.window_fill, base.visuals.window_fill);
        assert_eq!(c.visuals.window_stroke, base.visuals.window_stroke);
        assert_eq!(
            c.visuals.menu_corner_radius,
            base.visuals.menu_corner_radius
        );
        assert_eq!(c.visuals.popup_shadow, base.visuals.popup_shadow);
        assert_eq!(c.spacing.menu_margin, egui::Margin::same(3));
        assert_eq!(c.spacing.button_padding, egui::vec2(14.0, 4.0));
        assert_eq!(c.spacing.interact_size.y, 29.0);

        // a theme that states no outline: the item's border, in the four interactive entries only
        let (c, base, t, notes) = cell("adwaita", ColorMode::Light, Role::Menu);
        let m = &t.menu;
        assert!(notes.is_empty(), "{notes:?}");
        assert_eq!(m.hover_border_color, None);
        let w = &c.visuals.widgets;
        for e in [&w.inactive, &w.hovered, &w.active, &w.open] {
            assert_eq!(e.bg_stroke.color, to_color32(m.border.color));
            assert_eq!(e.bg_stroke.width, clamp_length(m.border.line_width));
            assert_eq!(
                e.corner_radius,
                to_corner_radius(
                    base.visuals.widgets.inactive.corner_radius,
                    m.border.corner_radius
                )
            );
        }
        assert_eq!(c.spacing.menu_margin, base.spacing.menu_margin);

        // over both: the item padding over menu_style's vec2(2.0, 0.0)
        // (`egui/src/containers/menu.rs:23`), its `x` the stated left side on both sides (E4),
        // else the pair's mean; the row `menu.row_height`, else the least height, else egui's
        // own (§2 *Unstated sizes*), not the base's `button.min_height`
        for preset in ["kde-breeze", "adwaita"] {
            let (c, base, t, _) = cell(preset, ColorMode::Light, Role::Menu);
            let m = &t.menu;
            let mut after_menu_style = base.clone();
            egui::containers::menu::menu_style(&mut after_menu_style);
            let pair = to_button_padding(after_menu_style.spacing.button_padding, &m.border);
            let x = match m.border.padding.left {
                Some(left) => left + m.border.line_width,
                None => pair.x,
            };
            assert_eq!(c.spacing.button_padding.x, x, "{preset}");
            if m.row_height.is_none() {
                assert_eq!(c.spacing.button_padding.y, pair.y, "{preset}");
            }
            let own = egui::Theme::Light.default_style().spacing.interact_size.y;
            let row = m.row_height.or(m.min_row_height).map_or(own, clamp_length);
            assert_eq!(c.spacing.interact_size.y, row, "{preset}");
        }
    }

    /// E4: `menu.min_row_height` is a least height in the menu cell's `interact_size.y`, which
    /// egui's `Button` grows past, and fits no padding; a stated `menu.row_height` wins over it
    /// (S2) and is fitted as today; with neither, egui's own.
    #[test]
    fn the_menu_row_is_at_least_min_row_height_unless_a_row_height_is_stated() {
        let built = |t: &ResolvedTheme| {
            let atlas = crate::ThemeAtlas::builder("t", t, t).build();
            let mut s = egui::Style::default();
            atlas
                .role_modifier(egui::Theme::Light, Role::Menu, crate::RoleVariant::Normal)
                .apply(&mut s);
            s
        };
        let mut t = crate::install_tests::resolved("kde-breeze", ColorMode::Light);
        assert_eq!(
            (t.menu.row_height, t.menu.min_row_height),
            (None, Some(29.0))
        );
        let s = built(&t);
        assert_eq!(s.spacing.interact_size.y, 29.0);
        assert_eq!(s.spacing.button_padding.y, 4.0);

        t.menu.row_height = Some(24.0);
        let s = built(&t);
        assert_eq!(s.spacing.interact_size.y, 24.0, "row_height wins");
        let defs = egui::FontDefinitions::default();
        let size = s.text_styles.get(&TextStyle::Body).unwrap().size;
        let label =
            crate::fonts::body_row_height(&defs, size).unwrap() + s.spacing.extra_text_line_spacing;
        let fitted = ((24.0 - label) / 2.0).floor().max(0.0);
        assert!(fitted < 4.0, "the 24 px row narrows the padding: {label}");
        assert_eq!(s.spacing.button_padding.y, fitted);

        t.menu.row_height = None;
        t.menu.min_row_height = None;
        let s = built(&t);
        assert_eq!(
            s.spacing.interact_size.y,
            egui::Theme::Light.default_style().spacing.interact_size.y
        );
        assert_eq!(s.spacing.button_padding.y, 4.0);
    }

    /// E4: a non-finite `menu.border.padding.left` is no stated left side, so the item's one
    /// horizontal padding keeps the finite right side, as an unstated left does.
    #[test]
    fn a_non_finite_left_menu_padding_keeps_the_right_side() {
        let x = |t: &ResolvedTheme| {
            let atlas = crate::ThemeAtlas::builder("t", t, t).build();
            let mut s = egui::Style::default();
            atlas
                .role_modifier(egui::Theme::Light, Role::Menu, crate::RoleVariant::Normal)
                .apply(&mut s);
            s.spacing.button_padding.x
        };
        let mut t = crate::install_tests::resolved("kde-breeze", ColorMode::Light);
        assert!(t.menu.border.padding.right.is_some_and(f32::is_finite));
        t.menu.border.padding.left = None;
        let unstated = x(&t);
        t.menu.border.padding.right = None;
        let neither = x(&t);
        assert_ne!(unstated, neither, "the right side counts");
        t = crate::install_tests::resolved("kde-breeze", ColorMode::Light);
        for left in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
            t.menu.border.padding.left = Some(left);
            assert_eq!(x(&t), unstated, "{left}");
        }
    }

    /// §6.1: `Window` and `Card` write no entry and no field; their cell is the base style.
    #[test]
    fn window_and_card_cells_equal_the_base() {
        for role in [Role::Window, Role::Card] {
            let (c, base, _, _) = cell("adwaita", ColorMode::Light, role);
            assert_eq!(c, base, "{role:?}");
        }
    }

    /// §8.1, §7.5: no cell adds or removes a `text_styles` key, and none names a family.
    #[test]
    fn no_cell_changes_the_text_style_keys() {
        for role in Role::all() {
            let (c, base, _, _) = cell("macos-monterey", ColorMode::Dark, *role);
            assert_eq!(c.text_styles.len(), base.text_styles.len(), "{role:?}");
            for (slot, font) in &c.text_styles {
                assert_eq!(
                    font.family,
                    base.text_styles.get(slot).unwrap().family,
                    "{role:?} {slot:?}"
                );
            }
            if let Some(f) = &c.override_font_id {
                assert_eq!(f.family, FontFamily::Proportional, "{role:?}");
            }
        }
    }

    /// §5.4's Input rows: the text colour in all five entries, the border with the hover
    /// border colour in {hovered,active} (§6.4), the placeholder, the text selection.
    #[test]
    fn input_cell_writes_its_rows() {
        let (c, base, t, notes) = cell("windows-11", ColorMode::Light, Role::Input);
        let i = &t.input;
        let w = &c.visuals.widgets;
        assert!(notes.is_empty(), "{notes:?}");
        for e in [
            &w.noninteractive,
            &w.inactive,
            &w.hovered,
            &w.active,
            &w.open,
        ] {
            assert_eq!(e.fg_stroke.color, to_color32(i.font.color));
            assert_eq!(e.bg_stroke.width, clamp_length(i.border.line_width));
            assert_eq!(
                e.corner_radius,
                to_corner_radius(
                    base.visuals.widgets.inactive.corner_radius,
                    i.border.corner_radius
                )
            );
        }
        for e in [&w.noninteractive, &w.inactive, &w.open] {
            assert_eq!(e.bg_stroke.color, to_color32(i.border.color));
        }
        let hover = i.hover_border_color.unwrap_or(i.border.color);
        for e in [&w.hovered, &w.active] {
            assert_eq!(e.bg_stroke.color, to_color32(hover));
        }
        assert_eq!(
            c.visuals.weak_text_color,
            Some(to_color32(i.placeholder_color))
        );
        assert_eq!(
            c.visuals.selection.bg_fill,
            to_color32(i.selection_background)
        );
        assert_eq!(
            c.visuals.selection.stroke.color,
            to_color32(i.selection_text_color)
        );
        assert_eq!(
            c.text_styles.get(&TextStyle::Body).unwrap().size,
            i.font.size
        );
        assert_eq!(c.visuals.disabled_alpha, unit_interval(i.disabled_opacity));
        assert_eq!(
            c.spacing.button_padding,
            to_button_padding(base.spacing.button_padding, &i.border)
        );
        assert_eq!(
            c.visuals.text_edit_bg_color, base.visuals.text_edit_bg_color,
            "the base owner's; the Disabled cell writes its own"
        );
    }

    /// §5.3's Checkbox rows: the box on `bg_fill` (idle `unchecked_background`, else
    /// `background_color`, §6.4), the hover layer composited over it (C17), the mark on
    /// `fg_stroke`, the unchecked border in all five but `hovered` and `active`, which take
    /// `hover_border_color` where stated, the label on `override_text_color` (B4).
    #[test]
    fn checkbox_cell_writes_its_rows() {
        for (preset, mode) in PRESETS {
            let (c, base, t, notes) = cell(preset, mode, Role::Checkbox);
            let k = &t.checkbox;
            let w = &c.visuals.widgets;
            // The one note a preset may cause here: material-3's unchecked box has no
            // container (docs/platform-facts.md §2.5), a transparent fill written as given.
            let transparent_box = k.unchecked_background.is_some_and(|c| c.a == 0);
            assert!(
                notes.iter().all(|n| transparent_box
                    && *n
                        == Note::TransparentFill {
                            path: "checkbox.unchecked_background"
                        }),
                "{preset}: {notes:?}"
            );
            let idle = k.unchecked_background.unwrap_or(k.background_color);
            assert_eq!(w.inactive.bg_fill, to_color32(idle), "{preset}");
            assert_eq!(w.open.bg_fill, to_color32(idle), "{preset}");
            let hovered = match k.hover_background {
                Some(h) => composite_over(h, idle),
                None => to_color32(idle),
            };
            assert_eq!(w.hovered.bg_fill, hovered, "{preset}");
            assert_eq!(w.active.bg_fill, hovered, "{preset}");
            let border = k.unchecked_border_color.unwrap_or(k.border.color);
            let hover_border = k.hover_border_color.unwrap_or(border);
            let mark = k
                .check_mark_stroke_width
                .map_or(base.visuals.widgets.inactive.fg_stroke.width, clamp_length);
            for (e, outline) in
                all5(w)
                    .into_iter()
                    .zip([border, border, hover_border, hover_border, border])
            {
                assert_eq!(e.fg_stroke.color, to_color32(k.indicator_color), "{preset}");
                assert_eq!(e.fg_stroke.width, mark, "{preset}: the check mark's stroke");
                assert_eq!(e.bg_stroke.color, to_color32(outline), "{preset}");
                assert_eq!(
                    e.bg_stroke.width,
                    clamp_length(k.border.line_width),
                    "{preset}"
                );
                assert_eq!(
                    e.corner_radius,
                    to_corner_radius(
                        base.visuals.widgets.inactive.corner_radius,
                        k.border.corner_radius
                    ),
                    "{preset}"
                );
            }
            assert_eq!(
                c.visuals.override_text_color,
                Some(to_color32(k.font.color)),
                "{preset}"
            );
            assert_eq!(
                c.override_font_id,
                Some(FontId::new(k.font.size, FontFamily::Proportional)),
                "{preset}"
            );
            assert_eq!(
                c.spacing.icon_width,
                clamp_length(k.indicator_width),
                "{preset}"
            );
            // the row's floor is the indicator, not the base's `button.min_height`
            assert_eq!(
                c.spacing.interact_size.y,
                clamp_length(k.indicator_width),
                "{preset}"
            );
            assert_eq!(
                c.spacing.interact_size.x, base.spacing.interact_size.x,
                "{preset}"
            );
            assert_eq!(
                c.spacing.icon_spacing,
                clamp_length(k.label_gap),
                "{preset}"
            );
            assert_eq!(
                c.visuals.disabled_alpha,
                unit_interval(k.disabled_opacity),
                "{preset}"
            );
            assert_eq!(c.text_styles, base.text_styles, "{preset}");
        }
    }

    /// §5.2 and §6.1's surface roles: `Tooltip` and `Popover` write their text colour into
    /// `noninteractive` alone and their size into Body; the interactive entries stay the base's.
    #[test]
    fn tooltip_and_popover_cells_write_their_rows() {
        for (preset, mode) in PRESETS {
            for role in [Role::Tooltip, Role::Popover] {
                let (c, base, t, notes) = cell(preset, mode, role);
                let font = if role == Role::Tooltip {
                    &t.tooltip.font
                } else {
                    &t.popover.font
                };
                assert!(notes.is_empty(), "{preset} {role:?}: {notes:?}");
                let w = &c.visuals.widgets;
                let bw = &base.visuals.widgets;
                assert_eq!(
                    w.noninteractive.fg_stroke.color,
                    to_color32(font.color),
                    "{preset} {role:?}"
                );
                assert_eq!(
                    (&w.inactive, &w.hovered, &w.active, &w.open),
                    (&bw.inactive, &bw.hovered, &bw.active, &bw.open),
                    "{preset} {role:?}"
                );
                assert_eq!(
                    c.text_styles.get(&TextStyle::Body).unwrap().size,
                    font.size,
                    "{preset} {role:?}"
                );
                assert_eq!(c.override_font_id, None, "{preset} {role:?}");
            }
        }
    }

    /// §5.2's Dialog rows: body text into `noninteractive`, body and title sizes into Body and
    /// Heading, the button gap into `item_spacing.x`.
    #[test]
    fn dialog_cell_writes_its_rows() {
        for (preset, mode) in PRESETS {
            let (c, base, t, notes) = cell(preset, mode, Role::Dialog);
            let d = &t.dialog;
            assert!(notes.is_empty(), "{preset}: {notes:?}");
            assert_eq!(
                c.visuals.widgets.noninteractive.fg_stroke.color,
                to_color32(d.body_font.color),
                "{preset}"
            );
            assert_eq!(
                c.visuals.widgets.inactive, base.visuals.widgets.inactive,
                "{preset}"
            );
            assert_eq!(
                c.text_styles.get(&TextStyle::Body).unwrap().size,
                d.body_font.size,
                "{preset}"
            );
            assert_eq!(
                c.text_styles.get(&TextStyle::Heading).unwrap().size,
                d.title_font.size,
                "{preset}"
            );
            assert_eq!(
                c.spacing.item_spacing.x,
                clamp_length(d.button_gap),
                "{preset}"
            );
            assert_eq!(
                c.spacing.item_spacing.y, base.spacing.item_spacing.y,
                "{preset}"
            );
        }
    }

    /// §5.5's Scrollbar rows: the base's own thumb and track values again; the pressed thumb
    /// (D5) and the radius (§6.8) are `derived`'s, tested there.
    #[test]
    fn scrollbar_cell_writes_its_rows() {
        for (preset, mode) in PRESETS {
            let (c, _, t, notes) = cell(preset, mode, Role::Scrollbar);
            let sb = &t.scrollbar;
            let w = &c.visuals.widgets;
            assert!(notes.is_empty(), "{preset}: {notes:?}");
            assert_eq!(
                c.visuals.extreme_bg_color,
                to_color32(sb.track_color),
                "{preset}"
            );
            assert_eq!(w.inactive.bg_fill, to_color32(sb.thumb_color), "{preset}");
            assert_eq!(w.open.bg_fill, to_color32(sb.thumb_color), "{preset}");
            assert_eq!(
                w.hovered.bg_fill,
                to_color32(sb.thumb_hover_color),
                "{preset}"
            );
            assert_eq!(
                c.spacing.scroll.handle_min_length,
                clamp_length(sb.min_thumb_length),
                "{preset}"
            );
            assert!(!c.spacing.scroll.foreground_color, "{preset}");
        }
    }

    /// §5.5's Slider rows the role function writes: the fill colour on `selection.bg_fill`, the
    /// rail on {inactive,open} `bg_fill`; the handle's hover and press (D6) are `derived`'s.
    #[test]
    fn slider_cell_writes_its_rows() {
        for (preset, mode) in PRESETS {
            let (c, _, t, notes) = cell(preset, mode, Role::Slider);
            let sl = &t.slider;
            let w = &c.visuals.widgets;
            assert!(notes.is_empty(), "{preset}: {notes:?}");
            assert_eq!(
                c.visuals.selection.bg_fill,
                to_color32(sl.fill_color),
                "{preset}"
            );
            assert!(c.visuals.slider_trailing_fill, "{preset}");
            assert_eq!(
                c.visuals.handle_shape,
                egui::style::HandleShape::Circle,
                "{preset}"
            );
            assert_eq!(w.inactive.bg_fill, to_color32(sl.track_color), "{preset}");
            assert_eq!(w.open.bg_fill, to_color32(sl.track_color), "{preset}");
            assert_eq!(
                c.visuals.disabled_alpha,
                unit_interval(sl.disabled_opacity),
                "{preset}"
            );
        }
    }

    /// §5.5's ProgressBar rows: fill, track, height, and the label gap from the left padding.
    #[test]
    fn progress_bar_cell_writes_its_rows() {
        for (preset, mode) in PRESETS {
            let (c, base, t, notes) = cell(preset, mode, Role::ProgressBar);
            let p = &t.progress_bar;
            assert!(notes.is_empty(), "{preset}: {notes:?}");
            assert_eq!(
                c.visuals.selection.bg_fill,
                to_color32(p.fill_color),
                "{preset}"
            );
            assert_eq!(
                c.visuals.extreme_bg_color,
                to_color32(p.track_color),
                "{preset}"
            );
            assert_eq!(
                c.spacing.interact_size.y,
                clamp_length(p.track_height),
                "{preset}"
            );
            match p.border.padding.left {
                Some(v) => assert_eq!(c.spacing.item_spacing.x, clamp_length(v), "{preset}"),
                None => assert_eq!(
                    c.spacing.item_spacing.x, base.spacing.item_spacing.x,
                    "{preset}"
                ),
            }
        }
    }

    /// §5.6's Tab rows: the resting fill, the hover as given in its place (C17's row highlight), the
    /// hover text colour into {hovered,active}, the border in all five, the selected tab on
    /// `selection.*` (§6.2), the bar on `panel_fill`.
    #[test]
    fn tab_cell_writes_its_rows() {
        for (preset, mode) in PRESETS {
            let (c, base, t, notes) = cell(preset, mode, Role::Tab);
            let tb = &t.tab;
            let w = &c.visuals.widgets;
            assert!(notes.is_empty(), "{preset}: {notes:?}");
            assert_eq!(
                w.inactive.weak_bg_fill,
                to_color32(tb.background_color),
                "{preset}"
            );
            assert_eq!(
                w.open.weak_bg_fill,
                to_color32(tb.background_color),
                "{preset}"
            );
            let hovered = to_color32(tb.hover_background.unwrap_or(tb.background_color));
            assert_eq!(w.hovered.weak_bg_fill, hovered, "{preset}");
            assert_eq!(w.active.weak_bg_fill, hovered, "{preset}");
            for e in [&w.noninteractive, &w.inactive, &w.open] {
                assert_eq!(e.fg_stroke.color, to_color32(tb.font.color), "{preset}");
            }
            for e in [&w.hovered, &w.active] {
                assert_eq!(
                    e.fg_stroke.color,
                    to_color32(tb.hover_text_color),
                    "{preset}"
                );
            }
            for e in all5(w) {
                assert_eq!(e.bg_stroke.color, to_color32(tb.border.color), "{preset}");
                assert_eq!(
                    e.bg_stroke.width,
                    clamp_length(tb.border.line_width),
                    "{preset}"
                );
                assert_eq!(
                    e.corner_radius,
                    to_corner_radius(
                        base.visuals.widgets.inactive.corner_radius,
                        tb.border.corner_radius
                    ),
                    "{preset}"
                );
            }
            assert_eq!(
                c.visuals.selection.bg_fill,
                to_color32(tb.active_background),
                "{preset}"
            );
            assert_eq!(
                c.visuals.selection.stroke.color,
                to_color32(tb.active_text_color),
                "{preset}"
            );
            assert_eq!(
                c.visuals.panel_fill,
                to_color32(tb.bar_background),
                "{preset}"
            );
            assert_eq!(
                c.spacing.interact_size.y,
                clamp_length(tb.min_height),
                "{preset}"
            );
            assert_eq!(
                c.spacing.button_padding,
                to_button_padding(base.spacing.button_padding, &tb.border),
                "{preset}"
            );
            assert_eq!(
                c.spacing.item_spacing.x,
                tb.item_gap
                    .map_or(base.spacing.item_spacing.x, clamp_length),
                "{preset}: tab.item_gap, egui's own where unstated"
            );
            assert_eq!(
                c.text_styles.get(&TextStyle::Body).unwrap().size,
                tb.font.size,
                "{preset}"
            );
        }
    }

    /// §5.6's three panel roles and §6.1's container-edge rule: the panel fill, the text colour
    /// in all five entries, the border stroke into `noninteractive` alone, no radius (§5.6).
    #[test]
    fn panel_role_cells_write_their_rows() {
        for (preset, mode) in PRESETS {
            for role in [Role::Sidebar, Role::Toolbar, Role::StatusBar] {
                let (c, base, t, notes) = cell(preset, mode, role);
                let (bg, font, border) = match role {
                    Role::Sidebar => (
                        t.sidebar.background_color,
                        &t.sidebar.font,
                        &t.sidebar.border,
                    ),
                    Role::Toolbar => (
                        t.toolbar.background_color,
                        &t.toolbar.font,
                        &t.toolbar.border,
                    ),
                    _ => (
                        t.status_bar.background_color,
                        &t.status_bar.font,
                        &t.status_bar.border,
                    ),
                };
                let w = &c.visuals.widgets;
                let bw = &base.visuals.widgets;
                assert!(notes.is_empty(), "{preset} {role:?}: {notes:?}");
                assert_eq!(c.visuals.panel_fill, to_color32(bg), "{preset} {role:?}");
                for e in all5(w) {
                    assert_eq!(
                        e.fg_stroke.color,
                        to_color32(font.color),
                        "{preset} {role:?}"
                    );
                }
                assert_eq!(
                    w.noninteractive.bg_stroke.color,
                    to_color32(border.color),
                    "{preset} {role:?}"
                );
                assert_eq!(
                    w.noninteractive.bg_stroke.width,
                    clamp_length(border.line_width),
                    "{preset} {role:?}"
                );
                assert_eq!(
                    w.noninteractive.corner_radius, bw.noninteractive.corner_radius,
                    "{preset} {role:?}: the radius is the Panel frame's"
                );
                // hovered and active keep the base's resting stroke, not the button's hover
                // and focus border; a toolbar's take its buttons' outline where stated (D7)
                let edge = match role {
                    Role::Toolbar => t.toolbar.button_hover_border_color,
                    _ => None,
                };
                for (e, be, pointer) in [
                    (&w.inactive, &bw.inactive, false),
                    (&w.hovered, &bw.inactive, true),
                    (&w.active, &bw.inactive, true),
                    (&w.open, &bw.open, false),
                ] {
                    let want = match edge {
                        Some(edge) if pointer => {
                            egui::Stroke::new(be.bg_stroke.width, to_color32(edge))
                        }
                        _ => be.bg_stroke,
                    };
                    assert_eq!(
                        e.bg_stroke, want,
                        "{preset} {role:?}: no outline on the panel's buttons but the toolbar's"
                    );
                    assert_eq!(e.corner_radius, be.corner_radius, "{preset} {role:?}");
                }
                assert_eq!(
                    c.text_styles.get(&TextStyle::Body).unwrap().size,
                    font.size,
                    "{preset} {role:?}"
                );
            }
        }
    }

    /// The panel roles' own rows beyond the shared ones: the sidebar's hover highlight and
    /// selection, the toolbar's item gap and its buttons' hover and pressed fills and hover
    /// outline (`toolbar.button_hover_background`, `.button_active_background`,
    /// `.button_hover_border_color`), with `inactive` left the base's.
    #[test]
    fn sidebar_and_toolbar_write_their_own_rows() {
        for (preset, mode) in PRESETS {
            let (c, base, t, _) = cell(preset, mode, Role::Sidebar);
            let sb = &t.sidebar;
            let w = &c.visuals.widgets;
            assert_eq!(
                w.hovered.weak_bg_fill,
                to_color32(sb.hover_background),
                "{preset}"
            );
            assert_eq!(
                w.active.weak_bg_fill,
                to_color32(sb.hover_background),
                "{preset}"
            );
            assert_eq!(
                w.inactive.weak_bg_fill, base.visuals.widgets.inactive.weak_bg_fill,
                "{preset}"
            );
            assert_eq!(
                c.visuals.selection.bg_fill,
                to_color32(sb.selection_background),
                "{preset}"
            );
            assert_eq!(
                c.visuals.selection.stroke.color,
                to_color32(sb.selection_text_color),
                "{preset}"
            );
            let (c, base, t, _) = cell(preset, mode, Role::Toolbar);
            match t.toolbar.item_gap {
                Some(g) => assert_eq!(c.spacing.item_spacing.x, clamp_length(g), "{preset}"),
                None => assert_eq!(
                    c.spacing.item_spacing.x, base.spacing.item_spacing.x,
                    "{preset}"
                ),
            }
            // D7: a flat tool button's hover and pressed fills and its outline as given where
            // stated; the button at rest is the caller's, so `inactive` is the base's
            let tb = &t.toolbar;
            let w = &c.visuals.widgets;
            let bw = &base.visuals.widgets;
            assert_eq!(
                w.hovered.weak_bg_fill,
                tb.button_hover_background
                    .map_or(bw.hovered.weak_bg_fill, to_color32),
                "{preset}"
            );
            assert_eq!(
                w.active.weak_bg_fill,
                tb.button_active_background
                    .map_or(bw.active.weak_bg_fill, to_color32),
                "{preset}"
            );
            let edge = tb
                .button_hover_border_color
                .map_or(w.inactive.bg_stroke.color, to_color32);
            assert_eq!(w.hovered.bg_stroke.color, edge, "{preset}");
            assert_eq!(w.active.bg_stroke.color, edge, "{preset}");
            assert_eq!(
                w.inactive.weak_bg_fill, bw.inactive.weak_bg_fill,
                "{preset}"
            );
            assert_eq!(w.inactive.bg_fill, bw.inactive.bg_fill, "{preset}");
            assert_eq!(w.inactive.bg_stroke, bw.inactive.bg_stroke, "{preset}");
            assert_eq!(
                w.inactive.fg_stroke.color,
                to_color32(tb.font.color),
                "{preset}"
            );
        }
        let breeze = resolved("kde-breeze", ColorMode::Dark).toolbar;
        assert!(
            breeze.button_hover_background.is_some()
                && breeze.button_hover_border_color.is_some()
                && breeze.button_active_background.is_some(),
            "kde-breeze, one of PRESETS, states the three, so the stated paths are met"
        );
    }

    /// §5.4's List rows: the grid line on `noninteractive.bg_stroke` as given, the row height,
    /// the hover highlight on {hovered,active} `bg_fill`, the cell and header text colours.
    #[test]
    fn list_cell_writes_its_rows() {
        for (preset, mode) in PRESETS {
            let (c, base, t, notes) = cell(preset, mode, Role::List);
            let l = &t.list;
            let w = &c.visuals.widgets;
            assert!(notes.is_empty(), "{preset}: {notes:?}");
            assert_eq!(
                c.visuals.selection.bg_fill,
                to_color32(l.selection_background),
                "{preset}"
            );
            assert_eq!(
                c.visuals.selection.stroke.color,
                to_color32(l.selection_text_color),
                "{preset}"
            );
            assert_eq!(
                w.noninteractive.bg_stroke.color,
                to_color32(l.grid_color),
                "{preset}"
            );
            assert_eq!(
                w.noninteractive.bg_stroke.width,
                clamp_length(l.border.line_width),
                "{preset}"
            );
            assert_eq!(
                w.noninteractive.corner_radius,
                to_corner_radius(
                    base.visuals.widgets.noninteractive.corner_radius,
                    l.border.corner_radius
                ),
                "{preset}"
            );
            // §2 *Unstated sizes*: egui's own row where the theme states none
            match l.row_height {
                Some(h) => assert_eq!(c.spacing.interact_size.y, clamp_length(h), "{preset}"),
                None => assert_eq!(
                    c.spacing.interact_size.y,
                    egui::Theme::Light.default_style().spacing.interact_size.y,
                    "{preset}"
                ),
            }
            assert_eq!(
                w.hovered.bg_fill,
                to_color32(l.hover_background),
                "{preset}"
            );
            assert_eq!(w.active.bg_fill, to_color32(l.hover_background), "{preset}");
            assert_eq!(
                w.inactive.bg_fill, base.visuals.widgets.inactive.bg_fill,
                "{preset}"
            );
            assert_eq!(
                w.noninteractive.fg_stroke.color,
                to_color32(l.item_font.color),
                "{preset}"
            );
            assert_eq!(
                w.active.fg_stroke.color,
                to_color32(l.header_font.color),
                "{preset}"
            );
            assert_eq!(
                c.text_styles.get(&TextStyle::Body).unwrap().size,
                l.item_font.size,
                "{preset}"
            );
        }
    }

    /// §5.5's Separator rows: the line's own colour, no opacity fold (§5.1), and its width.
    #[test]
    fn separator_cell_writes_its_rows() {
        for (preset, mode) in PRESETS {
            let (c, base, t, notes) = cell(preset, mode, Role::Separator);
            assert!(notes.is_empty(), "{preset}: {notes:?}");
            let e = &c.visuals.widgets.noninteractive;
            assert_eq!(
                e.bg_stroke,
                egui::Stroke::new(
                    clamp_length(t.separator.line_width),
                    to_color32(t.separator.line_color)
                ),
                "{preset}"
            );
            assert_eq!(
                c.visuals.widgets.inactive, base.visuals.widgets.inactive,
                "{preset}"
            );
        }
    }

    /// §5.3's Switch rows: the checked track on `selection.bg_fill` (§6.2), the unchecked
    /// track with its hover layer composited (C17), the track radius in all five entries.
    #[test]
    fn switch_cell_writes_its_rows() {
        for (preset, mode) in PRESETS {
            let (c, base, t, notes) = cell(preset, mode, Role::Switch);
            let sw = &t.switch;
            let w = &c.visuals.widgets;
            assert!(notes.is_empty(), "{preset}: {notes:?}");
            assert_eq!(
                c.visuals.selection.bg_fill,
                to_color32(sw.checked_background),
                "{preset}"
            );
            assert_eq!(
                c.visuals.selection.stroke, base.visuals.selection.stroke,
                "{preset}"
            );
            assert_eq!(
                w.inactive.weak_bg_fill,
                to_color32(sw.unchecked_background),
                "{preset}"
            );
            assert_eq!(
                w.open.weak_bg_fill,
                to_color32(sw.unchecked_background),
                "{preset}"
            );
            let hovered = match sw.hover_unchecked_background {
                Some(h) => composite_over(h, sw.unchecked_background),
                None => to_color32(sw.unchecked_background),
            };
            assert_eq!(w.hovered.weak_bg_fill, hovered, "{preset}");
            assert_eq!(w.active.weak_bg_fill, hovered, "{preset}");
            let bw = &base.visuals.widgets;
            // hovered and active keep the base's resting stroke, not the button's hover and
            // focus border
            let strokes = [
                bw.noninteractive.bg_stroke,
                bw.inactive.bg_stroke,
                bw.inactive.bg_stroke,
                bw.inactive.bg_stroke,
                bw.open.bg_stroke,
            ];
            for ((e, be), stroke) in all5(w).into_iter().zip(all5(bw)).zip(strokes) {
                assert_eq!(
                    e.corner_radius,
                    to_corner_radius(be.corner_radius, sw.track_radius),
                    "{preset}"
                );
                assert_eq!(e.bg_stroke, stroke, "{preset}: the switch states no border");
            }
            assert_eq!(
                c.spacing.interact_size.y,
                clamp_length(sw.track_height),
                "{preset}"
            );
            assert_eq!(
                c.visuals.disabled_alpha,
                unit_interval(sw.disabled_opacity),
                "{preset}"
            );
        }
    }

    /// §5.5's Spinner row the role function writes: the arc on `active.fg_stroke.color` (I2); the
    /// size (D3) is `derived`'s.
    #[test]
    fn spinner_cell_writes_its_rows() {
        for (preset, mode) in PRESETS {
            let (c, base, t, notes) = cell(preset, mode, Role::Spinner);
            assert!(notes.is_empty(), "{preset}: {notes:?}");
            assert_eq!(
                c.visuals.widgets.active.fg_stroke.color,
                to_color32(t.spinner.fill_color),
                "{preset}"
            );
            assert_eq!(
                c.visuals.widgets.inactive, base.visuals.widgets.inactive,
                "{preset}"
            );
        }
    }

    /// §5.4's ComboBox and §5.3's SegmentedControl rows: a framed control's fill with its
    /// hover layer composited (C17), its text in all five entries, its border in all five, a
    /// combo box's `hovered` and `active` taking `combo_box.hover_border_color` where stated.
    #[test]
    fn combo_box_and_segmented_control_cells_write_their_rows() {
        // E10: kde-breeze light draws an open combo box as Breeze's pressed button, `Button`
        // mixed 0.3 towards `Highlight`, framed in `Highlight` (docs/platform-facts.md §2.24);
        // adwaita states neither, and its open box is its resting one
        let (c, _, _, _) = cell("kde-breeze", ColorMode::Light, Role::ComboBox);
        let open = &c.visuals.widgets.open;
        assert_eq!(open.weak_bg_fill, egui::Color32::from_rgb(0xc3, 0xe5, 0xf6));
        assert_eq!(
            open.bg_stroke.color,
            egui::Color32::from_rgb(0x3d, 0xae, 0xe9)
        );
        let (c, _, _, _) = cell("adwaita", ColorMode::Light, Role::ComboBox);
        assert_eq!(c.visuals.widgets.open, c.visuals.widgets.inactive);
        for (preset, mode) in PRESETS {
            let (c, base, t, notes) = cell(preset, mode, Role::ComboBox);
            let cb = &t.combo_box;
            let w = &c.visuals.widgets;
            assert!(notes.is_empty(), "{preset}: {notes:?}");
            let hovered = match cb.hover_background {
                Some(h) => composite_over(h, cb.background_color),
                None => to_color32(cb.background_color),
            };
            assert_eq!(
                w.inactive.weak_bg_fill,
                to_color32(cb.background_color),
                "{preset}"
            );
            let open = match cb.open_background {
                Some(o) => composite_over(o, cb.background_color),
                None => to_color32(cb.background_color),
            };
            assert_eq!(w.open.weak_bg_fill, open, "{preset}");
            assert_eq!(w.hovered.weak_bg_fill, hovered, "{preset}");
            assert_eq!(w.active.weak_bg_fill, hovered, "{preset}");
            let border = cb.border.color;
            let hover_border = cb.hover_border_color.unwrap_or(border);
            let open_border = cb.open_border_color.unwrap_or(border);
            for (e, outline) in
                all5(w)
                    .into_iter()
                    .zip([border, border, hover_border, hover_border, open_border])
            {
                assert_eq!(e.fg_stroke.color, to_color32(cb.font.color), "{preset}");
                assert_eq!(e.bg_stroke.color, to_color32(outline), "{preset}");
                assert_eq!(
                    e.bg_stroke.width,
                    clamp_length(cb.border.line_width),
                    "{preset}"
                );
                assert_eq!(
                    e.corner_radius,
                    to_corner_radius(
                        base.visuals.widgets.inactive.corner_radius,
                        cb.border.corner_radius
                    ),
                    "{preset}"
                );
            }
            assert_eq!(
                c.spacing.interact_size.y,
                clamp_length(cb.min_height),
                "{preset}"
            );
            assert_eq!(
                c.spacing.icon_width,
                clamp_length(cb.arrow_icon_size),
                "{preset}"
            );
            assert_eq!(
                c.override_font_id,
                Some(FontId::new(cb.font.size, FontFamily::Proportional)),
                "{preset}"
            );
            assert_eq!(c.text_styles, base.text_styles, "{preset}");
            assert_eq!(
                c.spacing.button_padding,
                to_button_padding(base.spacing.button_padding, &cb.border),
                "{preset}"
            );
            assert_eq!(
                c.visuals.disabled_alpha,
                unit_interval(cb.disabled_opacity),
                "{preset}"
            );

            let (c, base, t, notes) = cell(preset, mode, Role::SegmentedControl);
            let sc = &t.segmented_control;
            let w = &c.visuals.widgets;
            assert!(notes.is_empty(), "{preset}: {notes:?}");
            let hovered = match sc.hover_background {
                Some(h) => composite_over(h, sc.background_color),
                None => to_color32(sc.background_color),
            };
            assert_eq!(
                w.inactive.weak_bg_fill,
                to_color32(sc.background_color),
                "{preset}"
            );
            assert_eq!(w.hovered.weak_bg_fill, hovered, "{preset}");
            assert_eq!(w.active.weak_bg_fill, hovered, "{preset}");
            for e in all5(w) {
                assert_eq!(e.fg_stroke.color, to_color32(sc.font.color), "{preset}");
                assert_eq!(e.bg_stroke.color, to_color32(sc.border.color), "{preset}");
                assert_eq!(
                    e.corner_radius,
                    to_corner_radius(
                        base.visuals.widgets.inactive.corner_radius,
                        sc.border.corner_radius
                    ),
                    "{preset}"
                );
            }
            assert_eq!(
                c.visuals.selection.bg_fill,
                to_color32(sc.active_background),
                "{preset}"
            );
            assert_eq!(
                c.visuals.selection.stroke.color,
                to_color32(sc.active_text_color),
                "{preset}"
            );
            assert_eq!(
                c.spacing.interact_size.y,
                clamp_length(sc.segment_height),
                "{preset}"
            );
            assert_eq!(
                c.spacing.item_spacing.x,
                clamp_length(sc.separator_width),
                "{preset}"
            );
            assert_eq!(
                c.override_font_id,
                Some(FontId::new(sc.font.size, FontFamily::Proportional)),
                "{preset}"
            );
            assert_eq!(
                c.spacing.button_padding,
                to_button_padding(base.spacing.button_padding, &sc.border),
                "{preset}"
            );
            assert_eq!(
                c.visuals.disabled_alpha,
                unit_interval(sc.disabled_opacity),
                "{preset}"
            );
        }
    }

    /// §5.6's Expander rows: the header frame on, no resting fill, the hover fill as given,
    /// the text and border in all five entries, the header height, the trailing padding.
    #[test]
    fn expander_cell_writes_its_rows() {
        for (preset, mode) in PRESETS {
            let (c, base, t, notes) = cell(preset, mode, Role::Expander);
            let x = &t.expander;
            let w = &c.visuals.widgets;
            assert!(notes.is_empty(), "{preset}: {notes:?}");
            assert!(c.visuals.collapsing_header_frame, "{preset}");
            assert_eq!(
                w.inactive.weak_bg_fill,
                egui::Color32::TRANSPARENT,
                "{preset}"
            );
            let hovered = x
                .hover_background
                .map_or(egui::Color32::TRANSPARENT, to_color32);
            assert_eq!(w.hovered.weak_bg_fill, hovered, "{preset}");
            assert_eq!(w.active.weak_bg_fill, hovered, "{preset}");
            for e in all5(w) {
                assert_eq!(e.fg_stroke.color, to_color32(x.font.color), "{preset}");
                assert_eq!(e.bg_stroke.color, to_color32(x.border.color), "{preset}");
                assert_eq!(
                    e.corner_radius,
                    to_corner_radius(
                        base.visuals.widgets.inactive.corner_radius,
                        x.border.corner_radius
                    ),
                    "{preset}"
                );
            }
            // the header draws no outline; the companion crate's frame keeps the width
            for e in [&w.inactive, &w.hovered, &w.active, &w.open] {
                assert_eq!(e.bg_stroke.width, 0.0, "{preset}");
            }
            assert_eq!(
                w.noninteractive.bg_stroke.width,
                clamp_length(x.border.line_width),
                "{preset}"
            );
            assert_eq!(
                c.spacing.interact_size.y,
                clamp_length(x.header_height),
                "{preset}"
            );
            assert_eq!(
                c.text_styles.get(&TextStyle::Button).unwrap().size,
                x.font.size,
                "{preset}"
            );
            assert_eq!(
                c.spacing.button_padding.y,
                to_button_padding(base.spacing.button_padding, &expander_padding_border(x)).y,
                "{preset}"
            );
            let right = crate::convert::padding_with_border(&expander_padding_border(x)).right;
            assert_eq!(
                c.spacing.button_padding.x,
                right.map_or(base.spacing.button_padding.x, clamp_length),
                "{preset}"
            );
            // the leading padding: the arrow, centred on `indent / 2`, starts it plus the
            // border's width in; egui's own indent where the side is unstated
            let left = crate::convert::padding_with_border(&expander_padding_border(x)).left;
            assert_eq!(
                c.spacing.indent,
                left.map_or(base.spacing.indent, |l| clamp_length(
                    2.0 * l + x.arrow_icon_size
                )),
                "{preset}"
            );
        }
    }

    /// §5.3's Link rows: the colour on `hyperlink_color`, the size on the override (B3).
    #[test]
    fn link_cell_writes_its_rows() {
        for (preset, mode) in PRESETS {
            let (c, base, t, notes) = cell(preset, mode, Role::Link);
            assert!(notes.is_empty(), "{preset}: {notes:?}");
            assert_eq!(
                c.visuals.hyperlink_color,
                to_color32(t.link.font.color),
                "{preset}"
            );
            assert_eq!(
                c.override_font_id,
                Some(FontId::new(t.link.font.size, FontFamily::Proportional)),
                "{preset}"
            );
            assert_eq!(c.visuals.widgets, base.visuals.widgets, "{preset}");
        }
    }
}
