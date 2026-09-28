//! The 25 `Normal` role cells (§3.4, §4.4): a copy of the base style plus the role's writes.

use egui::{FontFamily, FontId, TextStyle};

use native_theme::theme::ResolvedWidgetBorder;

use super::base::{button_padding, length, opacity, radius, stroke, text_size};
use super::states::{
    BorderEntries, BorderSource, FillField, FillSource, Layer, OpenFrom, StateSource, TextSource,
    write_states,
};
use super::{BuildInput, push_note};
use crate::accessors::scaled_text_size;
use crate::convert::{Rgba, clamp_length, padding_with_border, to_button_padding, to_color32};
use crate::{Note, Role};

/// One role's `Normal` cell: `base` copied, then the role's writes (§3.4).
pub(crate) fn role_cell(
    role: Role,
    base: &egui::Style,
    input: &BuildInput<'_>,
    notes: &mut Vec<Note>,
) -> egui::Style {
    let mut s = base.clone();
    match role {
        Role::Window | Role::Card => {} // no entry, no field (§6.1): the surfaces carry them
        Role::Splitter => {}            // its three strokes are D7 (§6.9), in `derived::apply_role`
        Role::Button => button(&mut s, base, input, notes),
        Role::Input => input_role(&mut s, base, input, notes),
        Role::Checkbox => checkbox(&mut s, base, input, notes),
        Role::Menu => menu(&mut s, base, input, notes),
        Role::Tooltip => tooltip(&mut s, base, input, notes),
        Role::Scrollbar => scrollbar(&mut s, base, input, notes),
        Role::Slider => slider(&mut s, base, input, notes),
        Role::ProgressBar => progress_bar(&mut s, base, input, notes),
        Role::Tab => tab(&mut s, base, input, notes),
        Role::Sidebar => sidebar(&mut s, base, input, notes),
        Role::Toolbar => toolbar(&mut s, base, input, notes),
        Role::StatusBar => status_bar(&mut s, base, input, notes),
        Role::List => list(&mut s, base, input, notes),
        Role::Popover => popover(&mut s, base, input, notes),
        Role::Separator => separator(&mut s, base, input, notes),
        Role::Switch => switch(&mut s, base, input, notes),
        Role::Dialog => dialog(&mut s, base, input, notes),
        Role::Spinner => spinner(&mut s, base, input, notes),
        Role::ComboBox => combo_box(&mut s, base, input, notes),
        Role::SegmentedControl => segmented_control(&mut s, base, input, notes),
        Role::Expander => expander(&mut s, base, input, notes),
        Role::Link => link(&mut s, base, input, notes),
    }
    crate::style::derived::apply_role(role, &mut s, input, notes);
    s
}

/// B3 (§8.5): the role's font size on `override_font_id`, `Proportional` always; a size
/// that is not a positive normal `f32` writes no override and is reported.
fn override_font(
    s: &mut egui::Style,
    path: &'static str,
    size: f32,
    input: &BuildInput<'_>,
    notes: &mut Vec<Note>,
) {
    let scaled = scaled_text_size(size, input.prefs);
    if scaled.is_normal() && scaled > 0.0 {
        s.override_font_id = Some(FontId::new(scaled, FontFamily::Proportional));
    } else {
        push_note(notes, Note::ValueSanitised { path });
    }
}

/// A role's font size into one `text_styles` slot (§8.5's rule over the base's value there).
fn slot_size(
    s: &mut egui::Style,
    slot: TextStyle,
    path: &'static str,
    size: f32,
    own: &egui::Style,
    input: &BuildInput<'_>,
    notes: &mut Vec<Note>,
) {
    if let (Some(font), Some(own_font)) = (s.text_styles.get_mut(&slot), own.text_styles.get(&slot))
    {
        font.size = text_size(path, size, input.prefs, own_font.size, notes);
    }
}

/// An `Option<f32>` size (rationale §3.23, S2): `None` writes nothing and egui's value stands.
fn optional_length(
    path: &'static str,
    v: Option<f32>,
    own: f32,
    notes: &mut Vec<Note>,
) -> Option<f32> {
    v.map(|v| length(path, v, own, notes))
}

/// A role's text colour in all five entries: the roles §6.1 names that state a text colour
/// and no hover or pressed one.
fn text_in_all(idle: Rgba) -> Option<TextSource> {
    Some(TextSource {
        idle,
        hover: None,
        active: None,
        in_noninteractive: true,
    })
}

/// A panel role's border: the separator line `noninteractive.bg_stroke` strokes
/// (`egui/src/containers/panel.rs:911`), and no radius, which is the `Surface::Panel`
/// frame's (§5.6, §6.1).
fn panel_edge(
    s: &mut egui::Style,
    own: &egui::Style,
    paths: [&'static str; 2],
    border: &ResolvedWidgetBorder,
    notes: &mut Vec<Note>,
) {
    s.visuals.widgets.noninteractive.bg_stroke = stroke(
        paths,
        own.visuals.widgets.noninteractive.bg_stroke,
        to_color32(border.color),
        border.line_width,
        notes,
    );
}

fn button(s: &mut egui::Style, own: &egui::Style, input: &BuildInput<'_>, notes: &mut Vec<Note>) {
    let t = input.theme;
    let b = &t.button;
    write_states(
        &mut s.visuals.widgets,
        &StateSource {
            fill: Some(FillSource {
                field: FillField::Weak,
                idle: Some((b.background_color, "button.background_color")),
                hover: Some((b.hover_background, "button.hover_background")),
                active: Some((
                    b.active_background.unwrap_or(b.hover_background),
                    "button.active_background",
                )), // §6.4
                layer: Layer::Composite,
            }),
            text: Some(TextSource {
                idle: b.font.color,
                hover: Some(b.hover_text_color),
                active: Some(b.active_text_color),
                in_noninteractive: true, // `button.font.color` → {noninteractive,inactive,open} «Button» (§5.3)
            }),
            border: Some(BorderSource {
                border: &b.border,
                paths: [
                    "button.border.color",
                    "button.border.corner_radius",
                    "button.border.line_width",
                ],
                entries: BorderEntries::All, // the `{5}` rows
            }),
            open: OpenFrom::Inactive,
        },
        notes,
    );
    // the selected (primary) button: `button_style` reads `selection.*` regardless of state (§6.2)
    s.visuals.selection.bg_fill = to_color32(b.primary_background);
    s.visuals.selection.stroke.color = to_color32(b.primary_text_color);
    s.spacing.interact_size.y = length(
        "button.min_height",
        b.min_height,
        own.spacing.interact_size.y,
        notes,
    );
    s.spacing.icon_spacing = length(
        "button.icon_text_gap",
        b.icon_text_gap,
        own.spacing.icon_spacing,
        notes,
    );
    s.spacing.button_padding = button_padding(
        [
            "button.border.padding.top",
            "button.border.padding.right",
            "button.border.padding.bottom",
            "button.border.padding.left",
        ],
        &b.border,
        own.spacing.button_padding,
        notes,
    );
    s.visuals.disabled_alpha = opacity(
        "button.disabled_opacity",
        b.disabled_opacity,
        own.visuals.disabled_alpha,
        notes,
    );
    override_font(s, "button.font.size", b.font.size, input, notes);
}

fn menu(s: &mut egui::Style, own: &egui::Style, input: &BuildInput<'_>, notes: &mut Vec<Note>) {
    let t = input.theme;
    let m = &t.menu;
    // egui's own menu look first (§3.4): item padding vec2(2.0, 0.0), no item outlines, a
    // transparent resting item (`egui/src/containers/menu.rs:22-29`)
    egui::containers::menu::menu_style(s);
    write_states(
        &mut s.visuals.widgets,
        &StateSource {
            fill: Some(FillSource {
                field: FillField::Weak,
                idle: None, // menu_style's transparent item stands: no resting fill leaf (§6.1)
                hover: Some((m.hover_background, "menu.hover_background")),
                active: None,          // copies hovered
                layer: Layer::AsGiven, // a row highlight over the menu's own frame (C17)
            }),
            text: Some(TextSource {
                idle: m.font.color,
                hover: Some(m.hover_text_color),
                active: None,
                in_noninteractive: true, // `menu.font.color` → {noninteractive,inactive}
            }),
            border: Some(BorderSource {
                border: &m.border,
                paths: [
                    "menu.border.color",
                    "menu.border.corner_radius",
                    "menu.border.line_width",
                ],
                entries: BorderEntries::Interactive, // the item's outline; nothing into the frame's
            }),
            open: OpenFrom::Hovered, // the open item stays highlighted (`menu.rs:382-384`)
        },
        notes,
    );
    s.visuals.widgets.noninteractive.bg_stroke.color = to_color32(m.separator_color); // `separator_style` (§5.2)
    // the item padding over menu_style's vec2(2.0, 0.0), the unstated side's target (§5.2)
    let after_menu_style = s.spacing.button_padding;
    s.spacing.button_padding = button_padding(
        [
            "menu.border.padding.top",
            "menu.border.padding.right",
            "menu.border.padding.bottom",
            "menu.border.padding.left",
        ],
        &m.border,
        after_menu_style,
        notes,
    );
    if let Some(h) = optional_length(
        "menu.row_height",
        m.row_height,
        own.spacing.interact_size.y,
        notes,
    ) {
        s.spacing.interact_size.y = h;
    }
    s.spacing.icon_spacing = length(
        "menu.icon_text_gap",
        m.icon_text_gap,
        own.spacing.icon_spacing,
        notes,
    );
    slot_size(
        s,
        TextStyle::Body,
        "menu.font.size",
        m.font.size,
        own,
        input,
        notes,
    ); // items read Body (B3)
}

fn input_role(
    s: &mut egui::Style,
    own: &egui::Style,
    input: &BuildInput<'_>,
    notes: &mut Vec<Note>,
) {
    let t = input.theme;
    let i = &t.input;
    write_states(
        &mut s.visuals.widgets,
        &StateSource {
            fill: None,
            text: text_in_all(i.font.color),
            border: Some(BorderSource {
                border: &i.border,
                paths: [
                    "input.border.color",
                    "input.border.corner_radius",
                    "input.border.line_width",
                ],
                entries: BorderEntries::All,
            }),
            open: OpenFrom::Inactive,
        },
        notes,
    );
    // the hovered and pressed field's border (§6.4: `None` copies `border.color`)
    let hover = to_color32(i.hover_border_color.unwrap_or(i.border.color));
    s.visuals.widgets.hovered.bg_stroke.color = hover;
    s.visuals.widgets.active.bg_stroke.color = hover;
    s.visuals.weak_text_color = Some(to_color32(i.placeholder_color));
    s.visuals.selection.bg_fill = to_color32(i.selection_background);
    s.visuals.selection.stroke.color = to_color32(i.selection_text_color);
    slot_size(
        s,
        TextStyle::Body,
        "input.font.size",
        i.font.size,
        own,
        input,
        notes,
    );
    s.visuals.disabled_alpha = opacity(
        "input.disabled_opacity",
        i.disabled_opacity,
        own.visuals.disabled_alpha,
        notes,
    );
}

fn checkbox(s: &mut egui::Style, own: &egui::Style, input: &BuildInput<'_>, notes: &mut Vec<Note>) {
    let t = input.theme;
    let k = &t.checkbox;
    // §6.4: the unchecked box and its outline fall back to the widget's own colours
    let border = ResolvedWidgetBorder {
        color: k.unchecked_border_color.unwrap_or(k.border.color),
        ..k.border.clone()
    };
    write_states(
        &mut s.visuals.widgets,
        &StateSource {
            fill: Some(FillSource {
                field: FillField::Bg,
                idle: Some(match k.unchecked_background {
                    Some(c) => (c, "checkbox.unchecked_background"),
                    None => (k.background_color, "checkbox.background_color"),
                }),
                hover: k.hover_background.map(|h| (h, "checkbox.hover_background")),
                active: None,
                layer: Layer::Composite,
            }),
            text: text_in_all(k.indicator_color), // the check mark is `fg_stroke` (§5.3)
            border: Some(BorderSource {
                border: &border,
                paths: [
                    "checkbox.unchecked_border_color",
                    "checkbox.border.corner_radius",
                    "checkbox.border.line_width",
                ],
                entries: BorderEntries::All,
            }),
            open: OpenFrom::Inactive,
        },
        notes,
    );
    s.visuals.override_text_color = Some(to_color32(k.font.color)); // the label (B4)
    override_font(s, "checkbox.font.size", k.font.size, input, notes);
    s.spacing.icon_width = length(
        "checkbox.indicator_width",
        k.indicator_width,
        own.spacing.icon_width,
        notes,
    );
    // The row: `Checkbox` and `RadioButton` take their minimum size from
    // `Vec2::splat(interact_size.y)` (`widgets/checkbox.rs:85`, `widgets/radio_button.rs:54`),
    // which the base style fills from `button.min_height`. The model states no row height for
    // either, so the floor is the indicator the theme states, and the row grows round a taller
    // label as egui lays it out.
    s.spacing.interact_size.y = s.spacing.icon_width;
    s.spacing.icon_spacing = length(
        "checkbox.label_gap",
        k.label_gap,
        own.spacing.icon_spacing,
        notes,
    );
    s.visuals.disabled_alpha = opacity(
        "checkbox.disabled_opacity",
        k.disabled_opacity,
        own.visuals.disabled_alpha,
        notes,
    );
}

fn tooltip(s: &mut egui::Style, own: &egui::Style, input: &BuildInput<'_>, notes: &mut Vec<Note>) {
    let f = &input.theme.tooltip.font;
    s.visuals.widgets.noninteractive.fg_stroke.color = to_color32(f.color); // a surface role (§6.1)
    slot_size(
        s,
        TextStyle::Body,
        "tooltip.font.size",
        f.size,
        own,
        input,
        notes,
    );
}

fn popover(s: &mut egui::Style, own: &egui::Style, input: &BuildInput<'_>, notes: &mut Vec<Note>) {
    let f = &input.theme.popover.font;
    s.visuals.widgets.noninteractive.fg_stroke.color = to_color32(f.color); // a surface role (§6.1)
    slot_size(
        s,
        TextStyle::Body,
        "popover.font.size",
        f.size,
        own,
        input,
        notes,
    );
}

fn dialog(s: &mut egui::Style, own: &egui::Style, input: &BuildInput<'_>, notes: &mut Vec<Note>) {
    let d = &input.theme.dialog;
    s.visuals.widgets.noninteractive.fg_stroke.color = to_color32(d.body_font.color); // a surface role (§6.1)
    slot_size(
        s,
        TextStyle::Body,
        "dialog.body_font.size",
        d.body_font.size,
        own,
        input,
        notes,
    );
    slot_size(
        s,
        TextStyle::Heading,
        "dialog.title_font.size",
        d.title_font.size,
        own,
        input,
        notes,
    );
    s.spacing.item_spacing.x = length(
        "dialog.button_gap",
        d.button_gap,
        own.spacing.item_spacing.x,
        notes,
    );
}

fn scrollbar(
    s: &mut egui::Style,
    own: &egui::Style,
    input: &BuildInput<'_>,
    notes: &mut Vec<Note>,
) {
    let sb = &input.theme.scrollbar;
    // the base's own rows again, in the cell (§5.5, §5.9)
    s.visuals.extreme_bg_color = to_color32(sb.track_color);
    write_states(
        &mut s.visuals.widgets,
        &StateSource {
            fill: Some(FillSource {
                field: FillField::Bg,
                idle: Some((sb.thumb_color, "scrollbar.thumb_color")),
                hover: Some((sb.thumb_hover_color, "scrollbar.thumb_hover_color")),
                active: None, // the pressed thumb is D5 (§6.9), in `derived::apply_role`
                layer: Layer::AsGiven,
            }),
            text: None,
            border: None,
            open: OpenFrom::Inactive,
        },
        notes,
    );
    s.spacing.scroll.handle_min_length = length(
        "scrollbar.min_thumb_length",
        sb.min_thumb_length,
        own.spacing.scroll.handle_min_length,
        notes,
    );
}

fn slider(s: &mut egui::Style, own: &egui::Style, input: &BuildInput<'_>, notes: &mut Vec<Note>) {
    let sl = &input.theme.slider;
    s.visuals.selection.bg_fill = to_color32(sl.fill_color); // `slider_trailing_fill` is already on (§5.9)
    write_states(
        &mut s.visuals.widgets,
        &StateSource {
            fill: Some(FillSource {
                field: FillField::Bg,
                idle: Some((sl.track_color, "slider.track_color")), // I1: the rail
                hover: None, // the hovered and pressed handle: D6 (§6.9), `derived::apply_role`
                active: None,
                layer: Layer::AsGiven,
            }),
            text: None,
            border: None,
            open: OpenFrom::Inactive,
        },
        notes,
    );
    s.visuals.disabled_alpha = opacity(
        "slider.disabled_opacity",
        sl.disabled_opacity,
        own.visuals.disabled_alpha,
        notes,
    );
}

fn progress_bar(
    s: &mut egui::Style,
    own: &egui::Style,
    input: &BuildInput<'_>,
    notes: &mut Vec<Note>,
) {
    let p = &input.theme.progress_bar;
    s.visuals.selection.bg_fill = to_color32(p.fill_color);
    s.visuals.extreme_bg_color = to_color32(p.track_color);
    s.spacing.interact_size.y = length(
        "progress_bar.track_height",
        p.track_height,
        own.spacing.interact_size.y,
        notes,
    );
    if let Some(v) = p.border.padding.left {
        s.spacing.item_spacing.x = length(
            "progress_bar.border.padding.left",
            v,
            own.spacing.item_spacing.x,
            notes,
        );
    }
}

fn tab(s: &mut egui::Style, own: &egui::Style, input: &BuildInput<'_>, notes: &mut Vec<Note>) {
    let t = input.theme;
    let tb = &t.tab;
    write_states(
        &mut s.visuals.widgets,
        &StateSource {
            fill: Some(FillSource {
                field: FillField::Weak,
                idle: Some((tb.background_color, "tab.background_color")),
                hover: tb.hover_background.map(|h| (h, "tab.hover_background")),
                active: None,
                layer: Layer::Composite,
            }),
            text: Some(TextSource {
                idle: tb.font.color,
                hover: Some(tb.hover_text_color),
                active: None,
                in_noninteractive: true,
            }),
            border: Some(BorderSource {
                border: &tb.border,
                paths: [
                    "tab.border.color",
                    "tab.border.corner_radius",
                    "tab.border.line_width",
                ],
                entries: BorderEntries::All,
            }),
            open: OpenFrom::Inactive,
        },
        notes,
    );
    // the selected tab: `button_style` reads `selection.*` (§6.2)
    s.visuals.selection.bg_fill = to_color32(tb.active_background);
    s.visuals.selection.stroke.color = to_color32(tb.active_text_color);
    s.visuals.panel_fill = to_color32(tb.bar_background);
    s.spacing.interact_size.y = length(
        "tab.min_height",
        tb.min_height,
        own.spacing.interact_size.y,
        notes,
    );
    s.spacing.button_padding = button_padding(
        [
            "tab.border.padding.top",
            "tab.border.padding.right",
            "tab.border.padding.bottom",
            "tab.border.padding.left",
        ],
        &tb.border,
        own.spacing.button_padding,
        notes,
    );
    slot_size(
        s,
        TextStyle::Body,
        "tab.font.size",
        tb.font.size,
        own,
        input,
        notes,
    );
}

fn sidebar(s: &mut egui::Style, own: &egui::Style, input: &BuildInput<'_>, notes: &mut Vec<Note>) {
    let t = input.theme;
    let sb = &t.sidebar;
    s.visuals.panel_fill = to_color32(sb.background_color);
    s.visuals.selection.bg_fill = to_color32(sb.selection_background);
    s.visuals.selection.stroke.color = to_color32(sb.selection_text_color);
    write_states(
        &mut s.visuals.widgets,
        &StateSource {
            fill: Some(FillSource {
                field: FillField::Weak,
                idle: None,
                hover: Some((sb.hover_background, "sidebar.hover_background")),
                active: None,
                layer: Layer::AsGiven, // a row highlight (§6.1)
            }),
            text: text_in_all(sb.font.color),
            border: None, // a container edge (§6.1)
            open: OpenFrom::Inactive,
        },
        notes,
    );
    panel_edge(
        s,
        own,
        ["sidebar.border.color", "sidebar.border.line_width"],
        &sb.border,
        notes,
    );
    slot_size(
        s,
        TextStyle::Body,
        "sidebar.font.size",
        sb.font.size,
        own,
        input,
        notes,
    );
}

fn toolbar(s: &mut egui::Style, own: &egui::Style, input: &BuildInput<'_>, notes: &mut Vec<Note>) {
    let t = input.theme;
    let tb = &t.toolbar;
    s.visuals.panel_fill = to_color32(tb.background_color);
    if let Some(gap) = optional_length(
        "toolbar.item_gap",
        tb.item_gap,
        own.spacing.item_spacing.x,
        notes,
    ) {
        s.spacing.item_spacing.x = gap;
    }
    write_states(
        &mut s.visuals.widgets,
        &StateSource {
            fill: None,
            text: text_in_all(tb.font.color),
            border: None,
            open: OpenFrom::Inactive,
        },
        notes,
    );
    panel_edge(
        s,
        own,
        ["toolbar.border.color", "toolbar.border.line_width"],
        &tb.border,
        notes,
    );
    slot_size(
        s,
        TextStyle::Body,
        "toolbar.font.size",
        tb.font.size,
        own,
        input,
        notes,
    );
}

fn status_bar(
    s: &mut egui::Style,
    own: &egui::Style,
    input: &BuildInput<'_>,
    notes: &mut Vec<Note>,
) {
    let t = input.theme;
    let sb = &t.status_bar;
    s.visuals.panel_fill = to_color32(sb.background_color);
    write_states(
        &mut s.visuals.widgets,
        &StateSource {
            fill: None,
            text: text_in_all(sb.font.color),
            border: None,
            open: OpenFrom::Inactive,
        },
        notes,
    );
    panel_edge(
        s,
        own,
        ["status_bar.border.color", "status_bar.border.line_width"],
        &sb.border,
        notes,
    );
    slot_size(
        s,
        TextStyle::Body,
        "status_bar.font.size",
        sb.font.size,
        own,
        input,
        notes,
    );
}

fn list(s: &mut egui::Style, own: &egui::Style, input: &BuildInput<'_>, notes: &mut Vec<Note>) {
    let l = &input.theme.list;
    s.visuals.selection.bg_fill = to_color32(l.selection_background);
    s.visuals.selection.stroke.color = to_color32(l.selection_text_color);
    // the table lines: `grid_color` as given, the list's width and radius (§5.4, §6.1)
    let own_edge = &own.visuals.widgets.noninteractive;
    let edge = &mut s.visuals.widgets.noninteractive;
    edge.bg_stroke.color = to_color32(l.grid_color);
    edge.bg_stroke.width = length(
        "list.border.line_width",
        l.border.line_width,
        own_edge.bg_stroke.width,
        notes,
    );
    edge.corner_radius = radius(
        "list.border.corner_radius",
        own_edge.corner_radius,
        l.border.corner_radius,
        notes,
    );
    if let Some(h) = optional_length(
        "list.row_height",
        l.row_height,
        own.spacing.interact_size.y,
        notes,
    ) {
        s.spacing.interact_size.y = h;
    }
    write_states(
        &mut s.visuals.widgets,
        &StateSource {
            fill: Some(FillSource {
                field: FillField::Bg,
                idle: None,
                hover: Some((l.hover_background, "list.hover_background")),
                active: None,
                layer: Layer::AsGiven, // a row highlight (§6.1)
            }),
            text: None,
            border: None,
            open: OpenFrom::Inactive,
        },
        notes,
    );
    s.visuals.widgets.noninteractive.fg_stroke.color = to_color32(l.item_font.color); // the cells are `Label`s
    s.visuals.widgets.active.fg_stroke.color = to_color32(l.header_font.color); // `strong_text_color` (§5.4)
    slot_size(
        s,
        TextStyle::Body,
        "list.item_font.size",
        l.item_font.size,
        own,
        input,
        notes,
    );
}

fn separator(
    s: &mut egui::Style,
    own: &egui::Style,
    input: &BuildInput<'_>,
    notes: &mut Vec<Note>,
) {
    let sp = &input.theme.separator;
    // the line keeps its own colour: no opacity fold (§5.1)
    s.visuals.widgets.noninteractive.bg_stroke = egui::Stroke::new(
        length(
            "separator.line_width",
            sp.line_width,
            own.visuals.widgets.noninteractive.bg_stroke.width,
            notes,
        ),
        to_color32(sp.line_color),
    );
}

fn switch(s: &mut egui::Style, own: &egui::Style, input: &BuildInput<'_>, notes: &mut Vec<Note>) {
    let sw = &input.theme.switch;
    s.visuals.selection.bg_fill = to_color32(sw.checked_background); // the checked track (§6.2)
    write_states(
        &mut s.visuals.widgets,
        &StateSource {
            fill: Some(FillSource {
                field: FillField::Weak,
                idle: Some((sw.unchecked_background, "switch.unchecked_background")),
                hover: sw
                    .hover_unchecked_background
                    .map(|h| (h, "switch.hover_unchecked_background")),
                active: None,
                layer: Layer::Composite,
            }),
            text: None,
            border: None,
            open: OpenFrom::Inactive,
        },
        notes,
    );
    let w = &mut s.visuals.widgets;
    for e in [
        &mut w.noninteractive,
        &mut w.inactive,
        &mut w.hovered,
        &mut w.active,
        &mut w.open,
    ] {
        e.corner_radius = radius(
            "switch.track_radius",
            e.corner_radius,
            sw.track_radius,
            notes,
        );
    }
    s.spacing.interact_size.y = length(
        "switch.track_height",
        sw.track_height,
        own.spacing.interact_size.y,
        notes,
    );
    s.visuals.disabled_alpha = opacity(
        "switch.disabled_opacity",
        sw.disabled_opacity,
        own.visuals.disabled_alpha,
        notes,
    );
}

fn spinner(
    s: &mut egui::Style,
    _own: &egui::Style,
    input: &BuildInput<'_>,
    _notes: &mut Vec<Note>,
) {
    // the arc: `strong_text_color`, `active.fg_stroke.color` (I2); its size is D3 (§6.7), in
    // `derived::apply_role`
    s.visuals.widgets.active.fg_stroke.color = to_color32(input.theme.spinner.fill_color);
}

fn combo_box(
    s: &mut egui::Style,
    own: &egui::Style,
    input: &BuildInput<'_>,
    notes: &mut Vec<Note>,
) {
    let t = input.theme;
    let cb = &t.combo_box;
    write_states(
        &mut s.visuals.widgets,
        &StateSource {
            fill: Some(FillSource {
                field: FillField::Weak,
                idle: Some((cb.background_color, "combo_box.background_color")),
                hover: cb
                    .hover_background
                    .map(|h| (h, "combo_box.hover_background")),
                active: None,
                layer: Layer::Composite,
            }),
            text: text_in_all(cb.font.color),
            border: Some(BorderSource {
                border: &cb.border,
                paths: [
                    "combo_box.border.color",
                    "combo_box.border.corner_radius",
                    "combo_box.border.line_width",
                ],
                entries: BorderEntries::All,
            }),
            open: OpenFrom::Inactive,
        },
        notes,
    );
    s.spacing.interact_size.y = length(
        "combo_box.min_height",
        cb.min_height,
        own.spacing.interact_size.y,
        notes,
    );
    s.spacing.icon_width = length(
        "combo_box.arrow_icon_size",
        cb.arrow_icon_size,
        own.spacing.icon_width,
        notes,
    );
    slot_size(
        s,
        TextStyle::Button,
        "combo_box.font.size",
        cb.font.size,
        own,
        input,
        notes,
    );
    s.spacing.button_padding = button_padding(
        [
            "combo_box.border.padding.top",
            "combo_box.border.padding.right",
            "combo_box.border.padding.bottom",
            "combo_box.border.padding.left",
        ],
        &cb.border,
        own.spacing.button_padding,
        notes,
    );
    s.visuals.disabled_alpha = opacity(
        "combo_box.disabled_opacity",
        cb.disabled_opacity,
        own.visuals.disabled_alpha,
        notes,
    );
}

fn segmented_control(
    s: &mut egui::Style,
    own: &egui::Style,
    input: &BuildInput<'_>,
    notes: &mut Vec<Note>,
) {
    let t = input.theme;
    let sc = &t.segmented_control;
    write_states(
        &mut s.visuals.widgets,
        &StateSource {
            fill: Some(FillSource {
                field: FillField::Weak,
                idle: Some((sc.background_color, "segmented_control.background_color")),
                hover: sc
                    .hover_background
                    .map(|h| (h, "segmented_control.hover_background")),
                active: None,
                layer: Layer::Composite,
            }),
            text: text_in_all(sc.font.color),
            border: Some(BorderSource {
                border: &sc.border,
                paths: [
                    "segmented_control.border.color",
                    "segmented_control.border.corner_radius",
                    "segmented_control.border.line_width",
                ],
                entries: BorderEntries::All,
            }),
            open: OpenFrom::Inactive,
        },
        notes,
    );
    // the selected segment: `button_style` reads `selection.*` (§6.2)
    s.visuals.selection.bg_fill = to_color32(sc.active_background);
    s.visuals.selection.stroke.color = to_color32(sc.active_text_color);
    s.spacing.interact_size.y = length(
        "segmented_control.segment_height",
        sc.segment_height,
        own.spacing.interact_size.y,
        notes,
    );
    s.spacing.item_spacing.x = length(
        "segmented_control.separator_width",
        sc.separator_width,
        own.spacing.item_spacing.x,
        notes,
    );
    override_font(s, "segmented_control.font.size", sc.font.size, input, notes);
    s.spacing.button_padding = button_padding(
        [
            "segmented_control.border.padding.top",
            "segmented_control.border.padding.right",
            "segmented_control.border.padding.bottom",
            "segmented_control.border.padding.left",
        ],
        &sc.border,
        own.spacing.button_padding,
        notes,
    );
    s.visuals.disabled_alpha = opacity(
        "segmented_control.disabled_opacity",
        sc.disabled_opacity,
        own.visuals.disabled_alpha,
        notes,
    );
}

fn expander(s: &mut egui::Style, own: &egui::Style, input: &BuildInput<'_>, notes: &mut Vec<Note>) {
    let t = input.theme;
    let x = &t.expander;
    // the header's one frame layer, with no resting fill (§6.1, `collapsing_header.rs:561-568`)
    s.visuals.collapsing_header_frame = true;
    s.visuals.widgets.inactive.weak_bg_fill = egui::Color32::TRANSPARENT;
    s.visuals.widgets.open.weak_bg_fill = egui::Color32::TRANSPARENT; // `open` copies it (§6.1)
    write_states(
        &mut s.visuals.widgets,
        &StateSource {
            fill: Some(FillSource {
                field: FillField::Weak,
                idle: None,
                hover: x.hover_background.map(|h| (h, "expander.hover_background")),
                active: None,
                layer: Layer::AsGiven, // over no idle fill of its own (§6.1)
            }),
            text: text_in_all(x.font.color),
            border: Some(BorderSource {
                border: &x.border,
                paths: [
                    "expander.border.color",
                    "expander.border.corner_radius",
                    "expander.border.line_width",
                ],
                entries: BorderEntries::All,
            }),
            open: OpenFrom::Inactive,
        },
        notes,
    );
    s.spacing.interact_size.y = length(
        "expander.header_height",
        x.header_height,
        own.spacing.interact_size.y,
        notes,
    );
    slot_size(
        s,
        TextStyle::Button,
        "expander.font.size",
        x.font.size,
        own,
        input,
        notes,
    );
    // `.y` by the pair rule; `.x` is read on the trailing side only (§5.6, §7.4), so the
    // right side alone, `.left` being UNMAPPABLE with no sink to fall back in
    let p = &x.border.padding;
    for (side, path) in [
        (p.top, "expander.border.padding.top"),
        (p.right, "expander.border.padding.right"),
        (p.bottom, "expander.border.padding.bottom"),
    ] {
        if side.is_some_and(|v| !v.is_finite()) {
            push_note(notes, Note::ValueSanitised { path });
        }
    }
    let own_padding = own.spacing.button_padding;
    s.spacing.button_padding = egui::vec2(
        padding_with_border(&x.border)
            .right
            .map_or(own_padding.x, clamp_length),
        to_button_padding(own_padding, &x.border).y,
    );
}

fn link(s: &mut egui::Style, _own: &egui::Style, input: &BuildInput<'_>, notes: &mut Vec<Note>) {
    let l = &input.theme.link;
    s.visuals.hyperlink_color = to_color32(l.font.color);
    override_font(s, "link.font.size", l.font.size, input, notes);
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
        clamp_length, composite_over, to_button_padding, to_color32, to_corner_radius,
        unit_interval,
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
        let t = resolved(preset, mode);
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
            patch: None,
        };
        let mut notes = Vec::new();
        let base = base_style(&input, &mut notes);
        let c = role_cell(role, &base, &input, &mut notes);
        (c, base, t, notes)
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
    /// so the base style writes the panel's text colour there; kde-breeze light's pressed-button
    /// white `#fcfcfc` (`native-theme/src/presets/kde-breeze.toml:95`) stays in the Button cell.
    #[test]
    fn strong_text_is_the_panel_text_and_the_button_cell_keeps_its_pressed_pair() {
        let (c, base, t, _) = cell("kde-breeze", ColorMode::Light, Role::Button);
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
        ("macos-sonoma", ColorMode::Dark),
        ("material", ColorMode::Light),
        ("catppuccin-mocha", ColorMode::Dark),
    ];

    /// §5.3's Button rows, each at the entries §6.1 names, plus `selection.*` (§6.2) and B3.
    #[test]
    fn button_cell_writes_its_rows() {
        for (preset, mode) in [
            ("adwaita", ColorMode::Light),
            ("windows-11", ColorMode::Dark),
        ] {
            let (c, base, t, notes) = cell(preset, mode, Role::Button);
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
            // border in all five entries, colour as stated (§6.13 folds only `defaults.border.color`)
            for e in [
                &w.noninteractive,
                &w.inactive,
                &w.hovered,
                &w.active,
                &w.open,
            ] {
                assert_eq!(e.bg_stroke.color, to_color32(b.border.color));
                assert_eq!(e.bg_stroke.width, clamp_length(b.border.line_width));
                assert_eq!(
                    e.corner_radius,
                    to_corner_radius(
                        base.visuals.widgets.inactive.corner_radius,
                        b.border.corner_radius
                    )
                );
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
    /// hover highlight as given into {hovered,active,open}, and the frame left to the base.
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
        // the item's border, in the four interactive entries only
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
        assert_eq!(
            w.noninteractive.bg_stroke.color,
            to_color32(m.separator_color)
        );
        // the item padding over menu_style's vec2(2.0, 0.0) (`egui/src/containers/menu.rs:23`)
        let mut after_menu_style = base.clone();
        egui::containers::menu::menu_style(&mut after_menu_style);
        assert_eq!(
            c.spacing.button_padding,
            to_button_padding(after_menu_style.spacing.button_padding, &m.border)
        );
        assert_eq!(c.spacing.icon_spacing, clamp_length(m.icon_text_gap));
        match m.row_height {
            Some(h) => assert_eq!(c.spacing.interact_size.y, clamp_length(h)),
            None => assert_eq!(c.spacing.interact_size.y, base.spacing.interact_size.y),
        }
        assert_eq!(
            c.text_styles.get(&TextStyle::Body).unwrap().size,
            m.font.size
        );
        assert_eq!(c.override_font_id, None, "menu items read Body (B3)");
        // the frame is the base style's (§5.2, §5.9)
        assert_eq!(c.visuals.window_fill, base.visuals.window_fill);
        assert_eq!(c.visuals.window_stroke, base.visuals.window_stroke);
        assert_eq!(
            c.visuals.menu_corner_radius,
            base.visuals.menu_corner_radius
        );
        assert_eq!(c.visuals.popup_shadow, base.visuals.popup_shadow);
        assert_eq!(c.spacing.menu_margin, base.spacing.menu_margin);
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
            let (c, base, _, _) = cell("macos-sonoma", ColorMode::Dark, *role);
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
            c.visuals.text_edit_bg_color, base.visuals.text_edit_bg_color,
            "the base owner's; the Disabled cell writes its own"
        );
    }

    /// §5.3's Checkbox rows: the box on `bg_fill` (idle `unchecked_background`, else
    /// `background_color`, §6.4), the hover layer composited over it (C17), the mark on
    /// `fg_stroke`, the unchecked border in all five, the label on `override_text_color` (B4).
    #[test]
    fn checkbox_cell_writes_its_rows() {
        for (preset, mode) in PRESETS {
            let (c, base, t, notes) = cell(preset, mode, Role::Checkbox);
            let k = &t.checkbox;
            let w = &c.visuals.widgets;
            assert!(notes.is_empty(), "{preset}: {notes:?}");
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
            for e in all5(w) {
                assert_eq!(e.fg_stroke.color, to_color32(k.indicator_color), "{preset}");
                assert_eq!(e.bg_stroke.color, to_color32(border), "{preset}");
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

    /// §5.6's Tab rows: the resting fill with the hover layer composited over it (C17), the
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
            let hovered = match tb.hover_background {
                Some(h) => composite_over(h, tb.background_color),
                None => to_color32(tb.background_color),
            };
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
                for (e, be) in [
                    (&w.inactive, &bw.inactive),
                    (&w.hovered, &bw.hovered),
                    (&w.active, &bw.active),
                    (&w.open, &bw.open),
                ] {
                    assert_eq!(
                        e.bg_stroke, be.bg_stroke,
                        "{preset} {role:?}: no outline on the panel's buttons"
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
    /// selection, the toolbar's item gap.
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
        }
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
            match l.row_height {
                Some(h) => assert_eq!(c.spacing.interact_size.y, clamp_length(h), "{preset}"),
                None => assert_eq!(
                    c.spacing.interact_size.y, base.spacing.interact_size.y,
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
            for (e, be) in all5(w).into_iter().zip(all5(&base.visuals.widgets)) {
                assert_eq!(
                    e.corner_radius,
                    to_corner_radius(be.corner_radius, sw.track_radius),
                    "{preset}"
                );
                assert_eq!(
                    e.bg_stroke, be.bg_stroke,
                    "{preset}: the switch states no border"
                );
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
    /// hover layer composited (C17), its text in all five entries, its border in all five.
    #[test]
    fn combo_box_and_segmented_control_cells_write_their_rows() {
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
            assert_eq!(
                w.open.weak_bg_fill,
                to_color32(cb.background_color),
                "{preset}"
            );
            assert_eq!(w.hovered.weak_bg_fill, hovered, "{preset}");
            assert_eq!(w.active.weak_bg_fill, hovered, "{preset}");
            for e in all5(w) {
                assert_eq!(e.fg_stroke.color, to_color32(cb.font.color), "{preset}");
                assert_eq!(e.bg_stroke.color, to_color32(cb.border.color), "{preset}");
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
                c.text_styles.get(&TextStyle::Button).unwrap().size,
                cb.font.size,
                "{preset}"
            );
            assert_eq!(c.override_font_id, None, "{preset}");
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
                to_button_padding(base.spacing.button_padding, &x.border).y,
                "{preset}"
            );
            let right = crate::convert::padding_with_border(&x.border).right;
            assert_eq!(
                c.spacing.button_padding.x,
                right.map_or(base.spacing.button_padding.x, clamp_length),
                "{preset}"
            );
            assert_eq!(c.spacing.indent, base.spacing.indent, "{preset}");
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
