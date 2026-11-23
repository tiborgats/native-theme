//! The base `Style` of one colour scheme (§3.4): §5.1's DIRECT rows, §5.9's base owners, the
//! five `text_styles` sizes (§8.5), `layout.widget_gap` (§6.16), the line spacing (§6.15) and
//! the tooltip delay.
//! The five `WidgetVisuals` entries are `states::write_states`'s.

use egui::TextStyle;

use native_theme::theme::ResolvedPadding;

#[cfg(test)]
use super::BuildInput;
use super::line::LineAlign;
use super::states::{
    BorderEntries, BorderSource, Edge, FillField, FillSource, Layer, OpenFrom, StateLine,
    StateSource, TextSource, Unstated, state_borders, write_states_with,
};
use super::{push_note, saturates_i8};
use crate::accessors::scaled_text_size;
use crate::convert::{
    Rgba, button_padding_outside, clamp_length, finite_or, padding_with_line, to_color32,
    to_corner_radius, to_margin, unit_interval,
};
use crate::{AccessibilityPreferences, LayoutTheme, Note, ResolvedTheme};

/// A length sink: the non-finite rule with egui's own value, reported, then `clamp_length`
/// (§7.2). A finite value merely clamped reports nothing.
pub(crate) fn length(path: &'static str, v: f32, own: f32, notes: &mut Vec<Note>) -> f32 {
    if !v.is_finite() {
        push_note(notes, Note::ValueSanitised { path });
    }
    clamp_length(finite_or(v, own))
}

/// A corner radius: `to_corner_radius` over the sink's own, reported when non-finite.
pub(crate) fn radius(
    path: &'static str,
    own: egui::CornerRadius,
    v: f32,
    notes: &mut Vec<Note>,
) -> egui::CornerRadius {
    if !v.is_finite() {
        push_note(notes, Note::ValueSanitised { path });
    }
    to_corner_radius(own, v)
}

/// A border stroke: the colour as converted, the width over the sink's own (`to_stroke`'s
/// rule, §7.2), a non-finite one reported for `width_path`. Every border colour is the
/// final line colour: `defaults.border.opacity` is already folded into it, and no stroke
/// multiplies it again (native-theme/src/model/border.rs, `DefaultsBorderSpec::opacity`).
pub(crate) fn stroke(
    width_path: &'static str,
    own: egui::Stroke,
    color: egui::Color32,
    width: f32,
    notes: &mut Vec<Note>,
) -> egui::Stroke {
    if !width.is_finite() {
        push_note(notes, Note::ValueSanitised { path: width_path });
    }
    egui::Stroke::new(clamp_length(finite_or(width, own.width)), color)
}

/// An opacity sink (`disabled_alpha`): egui's own when non-finite, reported; else clamped
/// into `0.0..=1.0` (§7.2, §7.4).
pub(crate) fn opacity(path: &'static str, v: f32, own: f32, notes: &mut Vec<Note>) -> f32 {
    if v.is_finite() {
        unit_interval(v)
    } else {
        push_note(notes, Note::ValueSanitised { path });
        own
    }
}

/// A four-sided margin: `to_margin` over the sink's own, a non-finite side reported as
/// sanitised and a saturated one as saturated (§7.2's emission rule). `paths` are the four
/// side leaves in `to_margin`'s order: top, right, bottom, left, and `sides` their values,
/// each `None` where unstated.
pub(crate) fn margin_of(
    paths: [&'static str; 4],
    own: egui::Margin,
    sides: [Option<f32>; 4],
    notes: &mut Vec<Note>,
) -> egui::Margin {
    for (side, path) in sides.into_iter().zip(paths) {
        if let Some(v) = side {
            if !v.is_finite() {
                push_note(notes, Note::ValueSanitised { path });
            } else if saturates_i8(v) {
                push_note(notes, Note::ValueSaturated { path });
            }
        }
    }
    let [top, right, bottom, left] = sides;
    to_margin(
        own,
        &ResolvedPadding {
            top,
            right,
            bottom,
            left,
        },
    )
}

/// `Spacing::button_padding` from a widget's padding (§5's pair rule, `to_button_padding`), four
/// sides given as top, right, bottom, left, each `None` where unstated, and the border's line
/// width, which a `None` leaves out of the sum; a non-finite side is reported. The line width
/// is reported where its stroke is written, not here (once per leaf).
pub(crate) fn button_padding_of(
    paths: [&'static str; 4],
    sides: [Option<f32>; 4],
    line_width: Option<f32>,
    own: egui::Vec2,
    notes: &mut Vec<Note>,
) -> egui::Vec2 {
    for (side, path) in sides.into_iter().zip(paths) {
        if side.is_some_and(|v| !v.is_finite()) {
            push_note(notes, Note::ValueSanitised { path });
        }
    }
    let [top, right, bottom, left] = sides;
    let padding = ResolvedPadding {
        top,
        right,
        bottom,
        left,
    };
    let outside = match line_width {
        Some(width) => padding_with_line(&padding, width),
        None => padding_with_line(&padding, 0.0),
    };
    button_padding_outside(own, &outside)
}

/// §6.15: the theme's line box less epaint's row, floored at egui's `0.0`
/// (`egui/src/style.rs:1465`); `0.0` too with no row or a non-finite multiplier.
pub(crate) fn line_spacing(line_height: f32, size: f32, row_height: Option<f32>) -> f32 {
    match row_height {
        Some(row) if line_height.is_finite() => clamp_length(line_height * size - row),
        _ => 0.0,
    }
}

/// A value the mapping writes, where the theme states it, with the place a note names it by:
/// the 0.7.2 leaf, or the 0.8.0 home.
pub(crate) type Read<T> = Option<(T, &'static str)>;

/// Four sides, top, right, bottom, left, each `None` where unstated, with their places.
pub(crate) struct Sides {
    pub values: [Option<f32>; 4],
    pub paths: [&'static str; 4],
}

/// The button's states the base style's four interactive entries take (§6.1).
pub(crate) struct ButtonStates {
    /// The body's fill at rest, hovered and pressed.
    pub fill: [Read<Rgba>; 3],
    /// The label's colour at rest and hovered.
    pub label: [Option<Rgba>; 2],
    pub border: Edge,
    pub border_paths: [&'static str; 2],
    pub align: LineAlign,
    /// The hovered and the pressed or focused border (egui's `active` is both).
    pub state_border: [Option<StateLine>; 2],
}

impl ButtonStates {
    /// The 0.7.2 model's button, every value stated.
    pub(crate) fn old(t: &ResolvedTheme) -> Self {
        let b = &t.button;
        Self {
            fill: [
                Some((b.background_color, "button.background_color")),
                Some((b.hover_background, "button.hover_background")),
                Some((
                    b.active_background.unwrap_or(b.hover_background),
                    "button.active_background",
                )), // §6.4
            ],
            label: [Some(b.font.color), Some(b.hover_text_color)],
            border: Edge::old(&b.border),
            border_paths: ["button.border.corner_radius", "button.border.line_width"],
            align: LineAlign::Outside,
            // a theme that states both and they differ: a pressed button takes the focus border
            state_border: [
                StateLine::colour(Some(b.hover_border_color.unwrap_or(b.border.color))),
                StateLine::colour(Some(
                    b.focus_border_color
                        .or(b.hover_border_color)
                        .unwrap_or(b.border.color),
                )),
            ],
        }
    }
}

/// What the base style and [`crate::style::derived::apply_base`] read, from either model, each
/// value `None` where the theme states none, so that egui's own stays (spec §10, rule 0).
pub(crate) struct BaseSource {
    /// The five slots' sizes, scaled by the text scale: Small, Body, Monospace, Button, Heading.
    pub sizes: [Read<f32>; 5],
    pub window_fill: Read<Rgba>,
    pub text: Option<Rgba>,
    pub muted: Option<Rgba>,
    pub shadow: Option<Rgba>,
    pub link: Option<Rgba>,
    pub selection: Option<Rgba>,
    pub selection_text: Option<Rgba>,
    pub danger: Option<Rgba>,
    pub warning: Option<Rgba>,
    /// The border of frames and separators: the window's and `noninteractive`'s stroke.
    pub border: Edge,
    pub border_paths: [&'static str; 2],
    pub radius_lg: Read<f32>,
    pub disabled_opacity: Read<f32>,
    pub menu_fill: Option<Rgba>,
    pub overlay_scrollbar: bool,
    pub track: Option<Rgba>,
    pub input_fill: Option<Rgba>,
    pub alternate_row: Option<Rgba>,
    pub caret: Option<Rgba>,
    pub blink_on_ms: Option<u32>,
    pub blink_off_ms: Option<u32>,
    pub window_radius: Read<f32>,
    pub window_padding: Sides,
    pub min_height: Read<f32>,
    pub button_padding: Sides,
    pub indicator_width: Read<f32>,
    pub label_gap: Read<f32>,
    pub rail_height: Read<f32>,
    pub combo_width: Read<f32>,
    pub tooltip_width: Read<f32>,
    pub min_thumb: Read<f32>,
    pub widget_gap: Option<f32>,
    /// The tooltip's delay, in egui's seconds.
    pub tooltip_delay: Option<f32>,
    pub button: ButtonStates,
    /// The scroll bar's thumb at rest, hovered and pressed.
    pub thumb: [Read<Rgba>; 3],
    pub title_bar_fill: Option<Rgba>,
    pub line_height: Read<f32>,
    pub groove_width: Read<f32>,
    pub thumb_width: Read<f32>,
    pub window_shadow: bool,
    pub popup_shadow: bool,
}

impl BaseSource {
    /// The 0.7.2 model's reads, every value stated, the text sizes scaled by `prefs`.
    pub(crate) fn old(
        t: &ResolvedTheme,
        prefs: &AccessibilityPreferences,
        layout: &LayoutTheme,
    ) -> Self {
        let d = &t.defaults;
        let b = &t.button;
        let sb = &t.scrollbar;
        let scaled = |size: f32, path| Some((scaled_text_size(size, prefs), path));
        let sides = |p: &ResolvedPadding, paths| Sides {
            values: [p.top, p.right, p.bottom, p.left],
            paths,
        };
        Self {
            sizes: [
                scaled(t.text_scale.caption.size, "text_scale.caption.size"),
                scaled(d.font.size, "defaults.font.size"),
                scaled(d.mono_font.size, "defaults.mono_font.size"),
                scaled(b.font.size, "button.font.size"),
                scaled(
                    t.text_scale.section_heading.size,
                    "text_scale.section_heading.size",
                ),
            ],
            window_fill: Some((d.background_color, "defaults.background_color")),
            text: Some(d.text_color),
            muted: Some(d.muted_color),
            shadow: Some(d.shadow_color),
            link: Some(d.link_color),
            selection: Some(d.selection_background),
            selection_text: Some(d.selection_text_color),
            danger: Some(d.danger_color),
            warning: Some(d.warning_color),
            border: Edge {
                color: Some(d.border.color),
                width: Some(d.border.line_width),
                radius: Some(d.border.corner_radius),
            },
            border_paths: [
                "defaults.border.corner_radius",
                "defaults.border.line_width",
            ],
            radius_lg: Some((
                d.border.corner_radius_lg,
                "defaults.border.corner_radius_lg",
            )),
            disabled_opacity: Some((d.disabled_opacity, "defaults.disabled_opacity")),
            menu_fill: Some(t.menu.background_color),
            overlay_scrollbar: sb.overlay_mode,
            track: Some(sb.track_color),
            input_fill: Some(t.input.background_color),
            alternate_row: Some(t.list.alternate_row_background),
            caret: Some(t.input.caret_color),
            blink_on_ms: t.input.caret_blink_on,
            blink_off_ms: t.input.caret_blink_off,
            window_radius: Some((t.window.border.corner_radius, "window.border.corner_radius")),
            window_padding: sides(
                &t.window.border.padding,
                [
                    "window.border.padding.top",
                    "window.border.padding.right",
                    "window.border.padding.bottom",
                    "window.border.padding.left",
                ],
            ),
            min_height: Some((b.min_height, "button.min_height")),
            button_padding: sides(
                &b.border.padding,
                [
                    "button.border.padding.top",
                    "button.border.padding.right",
                    "button.border.padding.bottom",
                    "button.border.padding.left",
                ],
            ),
            indicator_width: Some((t.checkbox.indicator_width, "checkbox.indicator_width")),
            label_gap: Some((t.checkbox.label_gap, "checkbox.label_gap")),
            rail_height: Some((t.slider.track_height, "slider.track_height")),
            combo_width: Some((t.combo_box.min_width, "combo_box.min_width")),
            tooltip_width: Some((t.tooltip.max_width, "tooltip.max_width")),
            min_thumb: Some((sb.min_thumb_length, "scrollbar.min_thumb_length")),
            widget_gap: layout.widget_gap,
            // the theme's milliseconds as egui's seconds (`egui/src/style.rs:928`)
            tooltip_delay: t.tooltip.show_delay.map(|ms| f32::from(ms) / 1000.0),
            button: ButtonStates::old(t),
            thumb: [
                Some((sb.thumb_color, "scrollbar.thumb_color")),
                Some((sb.thumb_hover_color, "scrollbar.thumb_hover_color")),
                Some(match sb.thumb_active_color {
                    Some(c) => (c, "scrollbar.thumb_active_color"),
                    None => (sb.thumb_hover_color, "scrollbar.thumb_hover_color"),
                }),
            ],
            title_bar_fill: Some(t.window.title_bar_background),
            line_height: Some((d.line_height, "defaults.line_height")),
            groove_width: Some((sb.groove_width, "scrollbar.groove_width")),
            thumb_width: Some((sb.thumb_width, "scrollbar.thumb_width")),
            window_shadow: t.window.border.shadow_enabled,
            popup_shadow: d.border.shadow_enabled,
        }
    }
}

/// A length sink written where `read` is stated.
fn set_length(sink: &mut f32, read: Read<f32>, own: f32, notes: &mut Vec<Note>) {
    if let Some((v, path)) = read {
        *sink = length(path, v, own, notes);
    }
}

/// The base style of `input.scheme` (§3.4) from the 0.7.2 model.
#[cfg(test)]
pub(crate) fn base_style(input: &BuildInput<'_>, notes: &mut Vec<Note>) -> egui::Style {
    let src = BaseSource::old(input.theme, input.prefs, input.layout);
    base_style_from(input.scheme, input.row_height, &src, false, notes)
}

/// The base style of `scheme` (§3.4): egui's own style for the scheme, with what `src` states
/// written into it. `row_height` is epaint's Body row (§6.15). With `rule_0`, a hover or press
/// `src` states nothing for takes egui's own treatment (spec §10, rule 0, the 0.8.0 model's);
/// else the 0.7.2 model's copy (§6.1).
pub(crate) fn base_style_from(
    scheme: egui::Theme,
    row_height: Option<f32>,
    src: &BaseSource,
    rule_0: bool,
    notes: &mut Vec<Note>,
) -> egui::Style {
    let own = scheme.default_style();
    let mut s = scheme.default_style();
    let unstated = if rule_0 {
        Unstated::Egui(&own.visuals.widgets)
    } else {
        Unstated::Copy
    };

    // §8.5: the five slots, sizes only — the families stay Proportional / Monospace (§8.1)
    let slots = [
        TextStyle::Small,
        TextStyle::Body,
        TextStyle::Monospace,
        TextStyle::Button,
        TextStyle::Heading,
    ];
    for (slot, read) in slots.into_iter().zip(src.sizes) {
        if let (Some(font), Some(own_font), Some((size, path))) = (
            s.text_styles.get_mut(&slot),
            own.text_styles.get(&slot),
            read,
        ) {
            font.size = sanitised_size(path, size, own_font.size, notes);
        }
    }

    // §5.1 foundation and §5.9 base owners: visuals
    let color = |sink: &mut egui::Color32, c: Option<Rgba>| {
        if let Some(c) = c {
            *sink = to_color32(c);
        }
    };
    let v = &mut s.visuals;
    color(&mut v.panel_fill, src.window_fill.map(|(c, _)| c));
    if let Some(c) = src.muted {
        v.weak_text_color = Some(to_color32(c));
    }
    color(&mut v.window_shadow.color, src.shadow); // geometry stays egui's; `apply_base` gates it (§6.14)
    color(&mut v.popup_shadow.color, src.shadow);
    color(&mut v.hyperlink_color, src.link);
    color(&mut v.selection.bg_fill, src.selection);
    color(&mut v.selection.stroke.color, src.selection_text); // width stays egui's (§5.1)
    color(&mut v.error_fg_color, src.danger);
    color(&mut v.warn_fg_color, src.warning);
    if let Some((r, path)) = src.radius_lg {
        v.menu_corner_radius = radius(path, own.visuals.menu_corner_radius, r, notes);
    }
    let [radius_path, width_path] = src.border_paths;
    if let Some(w) = src.border.width {
        v.window_stroke.width = stroke(
            width_path,
            own.visuals.window_stroke,
            own.visuals.window_stroke.color,
            w,
            notes,
        )
        .width;
    }
    color(&mut v.window_stroke.color, src.border.color);
    if let Some((o, path)) = src.disabled_opacity {
        v.disabled_alpha = opacity(path, o, own.visuals.disabled_alpha, notes);
    }
    color(&mut v.window_fill, src.menu_fill); // base owner of window_fill (§5.9)
    // egui paints the scroll-bar groove from `extreme_bg_color`, at the three
    // `*_background_opacity` when floating and at 1 when solid
    // (`egui/src/containers/scroll_area.rs:1485-1511`). An overlay bar whose stated track is fully
    // transparent has no groove: its opacities are written 0 below, and the slot serves its other
    // readers, `Frame::canvas` (`egui/src/containers/frame.rs:231`) and a role-less `ProgressBar`
    // (`egui/src/widgets/progress_bar.rs:140`), with egui's own meaning of it, "e.g. TextEdit
    // background" (`egui/src/style.rs:1513`). A solid bar keeps its stated track there.
    let no_groove = src.overlay_scrollbar && src.track.is_some_and(|c| c.a == 0);
    color(
        &mut v.extreme_bg_color,
        if no_groove { src.input_fill } else { src.track },
    );
    color(&mut v.faint_bg_color, src.alternate_row);
    if let Some(c) = src.input_fill {
        v.text_edit_bg_color = Some(to_color32(c));
    }
    color(&mut v.text_cursor.stroke.color, src.caret); // width stays egui's (§5.4)
    // the caret's blink, the theme's milliseconds as egui's seconds; a theme that states no blink
    // keeps egui's own whole, and egui has no time-out, so `input.caret_blink_timeout` has no sink
    // (the caret blink's rationale, B5)
    if let Some(off) = src.blink_off_ms {
        v.text_cursor.blink = off != 0;
        v.text_cursor.off_duration = (f64::from(off) / 1000.0) as f32;
        if let Some(on) = src.blink_on_ms {
            v.text_cursor.on_duration = (f64::from(on) / 1000.0) as f32;
        }
    }
    v.slider_trailing_fill = true; // `slider.fill_color` is invisible without it (§5.5)
    // `slider.thumb_diameter` is a diameter, so a round knob; egui's `Rect { aspect_ratio: 0.75 }`
    // (`egui/src/style.rs:1553`) has no theme value behind it (§5.9, §6.6)
    v.handle_shape = egui::style::HandleShape::Circle;
    if let Some((r, path)) = src.window_radius {
        v.window_corner_radius = radius(path, own.visuals.window_corner_radius, r, notes);
    }

    // spacing
    let sp = &mut s.spacing;
    let osp = &own.spacing;
    set_length(
        &mut sp.interact_size.y,
        src.min_height,
        osp.interact_size.y,
        notes,
    ); // .x stays inherited (§7.5)
    sp.button_padding = button_padding_of(
        src.button_padding.paths,
        src.button_padding.values,
        src.button.border.width,
        osp.button_padding,
        notes,
    );
    set_length(
        &mut sp.icon_width,
        src.indicator_width,
        osp.icon_width,
        notes,
    );
    set_length(&mut sp.icon_spacing, src.label_gap, osp.icon_spacing, notes);
    set_length(
        &mut sp.slider_rail_height,
        src.rail_height,
        osp.slider_rail_height,
        notes,
    );
    set_length(&mut sp.combo_width, src.combo_width, osp.combo_width, notes);
    set_length(
        &mut sp.tooltip_width,
        src.tooltip_width,
        osp.tooltip_width,
        notes,
    );
    set_length(
        &mut sp.scroll.handle_min_length,
        src.min_thumb,
        osp.scroll.handle_min_length,
        notes,
    );
    sp.scroll.foreground_color = false; // the handle reads `bg_fill` (§5.5)
    if no_groove {
        sp.scroll.dormant_background_opacity = 0.0;
        sp.scroll.active_background_opacity = 0.0;
        sp.scroll.interact_background_opacity = 0.0;
    }
    sp.window_margin = margin_of(
        src.window_padding.paths,
        osp.window_margin,
        src.window_padding.values,
        notes,
    );
    if let Some(gap) = src.widget_gap {
        // §6.16: one number, an asymmetric sink; a non-finite gap keeps egui's own, reported
        if gap.is_finite() {
            sp.item_spacing = egui::Vec2::splat(clamp_length(gap));
        } else {
            push_note(
                notes,
                Note::ValueSanitised {
                    path: "layout.widget_gap",
                },
            );
        }
    }

    // a theme that states no delay keeps egui's own
    if let Some(delay) = src.tooltip_delay {
        s.interaction.tooltip_delay = delay;
    }

    let b = &src.button;
    let w = &mut s.visuals.widgets;
    let ow = &own.visuals.widgets;
    // noninteractive: frames, separators, group boxes and plain labels (§5.9, §6.1)
    color(&mut w.noninteractive.fg_stroke.color, src.text);
    if let Some((c, path)) = src.window_fill {
        w.noninteractive.bg_fill = to_color32(c);
        super::note_transparent_fill(w.noninteractive.bg_fill, path, notes);
        w.noninteractive.weak_bg_fill = to_color32(c);
    }
    if let Some(width) = src.border.width {
        w.noninteractive.bg_stroke.width = stroke(
            width_path,
            ow.noninteractive.bg_stroke,
            ow.noninteractive.bg_stroke.color,
            width,
            notes,
        )
        .width;
    }
    color(&mut w.noninteractive.bg_stroke.color, src.border.color);
    if let Some(r) = src.border.radius {
        w.noninteractive.corner_radius =
            radius(radius_path, ow.noninteractive.corner_radius, r, notes);
    }
    // the four interactive entries: what every unscoped control draws with — `theme.button` (§6.1)
    let [idle, hover, active] = b.fill;
    write_states_with(
        w,
        &StateSource {
            fill: Some(FillSource {
                field: FillField::Weak,
                idle,
                hover,
                active,
                layer: Layer::Composite,
            }),
            text: Some(TextSource {
                idle: b.label[0],
                hover: b.label[1],
                // egui's strong text is `active.text_color()` (`egui/src/style.rs:1147-1149`):
                // the panel's text, not the pressed button's; the Button cell keeps that (§5.3)
                active: src.text,
                in_noninteractive: false, // the base's plain text is the window's
            }),
            border: Some(BorderSource {
                border: b.border,
                paths: b.border_paths,
                entries: BorderEntries::Interactive,
                align: b.align,
            }),
            open: OpenFrom::Inactive,
        },
        unstated,
        notes,
    );
    // the hovered button's border, which a pressed one keeps, and the focused one's; egui's
    // `active` is press and keyboard focus alike (`Response::widget_state`, egui 0.37.0
    // src/widget_style/mod.rs:212-222). Each over the fill its entry paints (`Line::over`).
    state_borders(
        w,
        b.state_border,
        b.align,
        Some(FillField::Weak),
        unstated,
        notes,
    );
    // `bg_fill`: the scrollbar thumb, which every unscoped `ScrollArea` handle paints (§5.9)
    let [thumb, thumb_hover, _] = src.thumb;
    write_states_with(
        w,
        &StateSource {
            fill: Some(FillSource {
                field: FillField::Bg,
                idle: thumb,
                hover: thumb_hover,
                active: None, // D5 (§6.9): `derived::apply_base` writes `active.bg_fill`
                layer: Layer::AsGiven,
            }),
            text: None,
            border: None,
            open: OpenFrom::Inactive,
        },
        unstated,
        notes,
    );
    // the topmost window's title bar reads `open.weak_bg_fill` (`egui/src/containers/window.rs:1427`)
    color(&mut w.open.weak_bg_fill, src.title_bar_fill);
    // §6.1: a hover or press changes only what the platform states, so on the 0.7.2 model every
    // entry keeps `inactive`'s stroke width — egui's stock 1.5 / 2.0 (`egui/src/style.rs:1703`,
    // `:1711`) never appear; §6.6's slider knob relies on one width in every state. The theme
    // states no text-stroke width, so under rule 0 a hover and a press take egui's own where
    // egui's entries change it. Every cell inherits it.
    let width = w.inactive.fg_stroke.width;
    let own_width = |e: &egui::style::WidgetVisuals| e.fg_stroke.width;
    let hovered = unstated.hovered(width, own_width);
    w.active.fg_stroke.width = unstated.active(width, hovered, own_width);
    w.hovered.fg_stroke.width = hovered;
    w.open.fg_stroke.width = width;

    // §6.15: the line spacing, from the Body size the atlas writes and the row `Builder::build` measured
    if let Some((line_height, path)) = src.line_height {
        if !line_height.is_finite() {
            push_note(notes, Note::ValueSanitised { path });
        }
        if let Some(body) = s.text_styles.get(&TextStyle::Body) {
            s.spacing.extra_text_line_spacing = line_spacing(line_height, body.size, row_height);
        }
    }
    crate::style::derived::apply_base(&mut s, src, notes);
    s
}

/// §8.5's rule on a size already scaled: the size when it is a positive normal `f32`, else
/// `own` — egui's own size for that slot — with a `Note::ValueSanitised` for `path`.
pub(crate) fn sanitised_size(path: &'static str, s: f32, own: f32, notes: &mut Vec<Note>) -> f32 {
    if s.is_normal() && s > 0.0 {
        s
    } else {
        push_note(notes, Note::ValueSanitised { path });
        own
    }
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
    use egui::{FontFamily, TextStyle};
    use native_theme::theme::ColorMode;

    use super::*;
    use crate::convert::{
        clamp_length, to_button_padding, to_color32, to_corner_radius, to_margin, unit_interval,
    };
    use crate::install_tests::resolved;
    use crate::{AccessibilityPreferences, LayoutTheme, Note, ResolvedTheme};

    fn build(
        scheme: egui::Theme,
        t: &ResolvedTheme,
        layout: &LayoutTheme,
        row_height: Option<f32>,
    ) -> (egui::Style, Vec<Note>) {
        let prefs = AccessibilityPreferences::default();
        let input = BuildInput {
            scheme,
            theme: t,
            prefs: &prefs,
            layout,
            row_height,
            defs: None,
            patch: None,
        };
        let mut notes = Vec::new();
        (base_style(&input, &mut notes), notes)
    }

    /// Every non-`Widgets` sink of §5.1 and §5.9 holds its leaf through the row's conversion,
    /// on a light and a dark theme, and every untouched field keeps egui's own value.
    #[test]
    fn the_base_style_writes_its_rows_and_nothing_else() {
        for (preset, mode, scheme) in [
            ("adwaita", ColorMode::Light, egui::Theme::Light),
            ("kde-breeze", ColorMode::Dark, egui::Theme::Dark),
        ] {
            let t = resolved(preset, mode);
            let d = &t.defaults;
            let own = scheme.default_style();
            let (s, notes) = build(scheme, &t, &LayoutTheme::default(), None);
            assert!(
                notes.is_empty(),
                "{preset}: a bundled preset needs no sanitising: {notes:?}"
            );

            // §8.5: the five slots, sizes only; families never written (§8.1)
            let size = |slot: TextStyle| s.text_styles.get(&slot).unwrap().size;
            assert_eq!(size(TextStyle::Small), t.text_scale.caption.size);
            assert_eq!(size(TextStyle::Body), d.font.size);
            assert_eq!(size(TextStyle::Monospace), d.mono_font.size);
            assert_eq!(size(TextStyle::Button), t.button.font.size);
            assert_eq!(size(TextStyle::Heading), t.text_scale.section_heading.size);
            assert_eq!(s.text_styles.len(), 5);
            for (slot, font) in &s.text_styles {
                let want = if *slot == TextStyle::Monospace {
                    FontFamily::Monospace
                } else {
                    FontFamily::Proportional
                };
                assert_eq!(font.family, want, "{slot:?}");
            }

            // §5.1 foundation, §5.9 base owners
            let v = &s.visuals;
            assert_eq!(v.panel_fill, to_color32(d.background_color));
            assert_eq!(v.weak_text_color, Some(to_color32(d.muted_color)));
            assert_eq!(v.window_shadow.color, to_color32(d.shadow_color));
            assert_eq!(v.popup_shadow.color, to_color32(d.shadow_color));
            assert_eq!(
                (
                    v.window_shadow.offset,
                    v.window_shadow.blur,
                    v.window_shadow.spread
                ),
                (
                    own.visuals.window_shadow.offset,
                    own.visuals.window_shadow.blur,
                    own.visuals.window_shadow.spread
                )
            );
            assert_eq!(v.hyperlink_color, to_color32(d.link_color));
            assert_eq!(v.selection.bg_fill, to_color32(d.selection_background));
            assert_eq!(v.selection.stroke.color, to_color32(d.selection_text_color));
            assert_eq!(v.selection.stroke.width, own.visuals.selection.stroke.width);
            assert_eq!(v.error_fg_color, to_color32(d.danger_color));
            assert_eq!(v.warn_fg_color, to_color32(d.warning_color));
            assert_eq!(
                v.menu_corner_radius,
                to_corner_radius(own.visuals.menu_corner_radius, d.border.corner_radius_lg)
            );
            assert_eq!(v.window_stroke.color, to_color32(d.border.color));
            assert_eq!(v.window_stroke.width, clamp_length(d.border.line_width));
            assert_eq!(v.disabled_alpha, unit_interval(d.disabled_opacity));
            assert_eq!(v.window_fill, to_color32(t.menu.background_color));
            assert_eq!(v.extreme_bg_color, to_color32(t.scrollbar.track_color));
            assert_eq!(
                v.faint_bg_color,
                to_color32(t.list.alternate_row_background)
            );
            assert_eq!(
                v.text_edit_bg_color,
                Some(to_color32(t.input.background_color))
            );
            assert_eq!(v.text_cursor.stroke.color, to_color32(t.input.caret_color));
            assert_eq!(
                v.text_cursor.stroke.width,
                own.visuals.text_cursor.stroke.width
            );
            let blink = if preset == "adwaita" { 0.6 } else { 0.5 };
            assert_eq!(
                (
                    v.text_cursor.blink,
                    v.text_cursor.on_duration,
                    v.text_cursor.off_duration
                ),
                (true, blink, blink),
                "{preset}: the caret's blink in seconds"
            );
            assert!(v.slider_trailing_fill);
            assert_eq!(v.handle_shape, egui::style::HandleShape::Circle);
            assert_eq!(
                v.window_corner_radius,
                to_corner_radius(
                    own.visuals.window_corner_radius,
                    t.window.border.corner_radius
                )
            );

            let sp = &s.spacing;
            assert_eq!(sp.interact_size.y, clamp_length(t.button.min_height));
            assert_eq!(
                sp.interact_size.x, own.spacing.interact_size.x,
                "§7.5: .x stays inherited"
            );
            assert_eq!(
                sp.button_padding,
                to_button_padding(own.spacing.button_padding, &t.button.border)
            );
            assert_eq!(sp.icon_width, clamp_length(t.checkbox.indicator_width));
            assert_eq!(sp.icon_spacing, clamp_length(t.checkbox.label_gap));
            assert_eq!(sp.slider_rail_height, clamp_length(t.slider.track_height));
            assert_eq!(sp.combo_width, clamp_length(t.combo_box.min_width));
            assert_eq!(sp.tooltip_width, clamp_length(t.tooltip.max_width));
            assert_eq!(
                sp.scroll.handle_min_length,
                clamp_length(t.scrollbar.min_thumb_length)
            );
            assert!(!sp.scroll.foreground_color);
            assert_eq!(
                sp.window_margin,
                to_margin(own.spacing.window_margin, &t.window.border.padding)
            );
            assert_eq!(
                sp.item_spacing, own.spacing.item_spacing,
                "no layout given: egui's gap"
            );
            assert_eq!(
                sp.extra_text_line_spacing, 0.0,
                "no row height: egui's 0.0 (§6.15)"
            );

            // untouched: egui's own (§3.4)
            assert_eq!(v.dark_mode, own.visuals.dark_mode);
            assert_eq!(v.text_options, own.visuals.text_options);
            assert_eq!(v.code_bg_color, own.visuals.code_bg_color);
            assert_eq!(sp.menu_margin, own.spacing.menu_margin);
            assert_eq!(sp.combo_height, own.spacing.combo_height);
            assert_eq!(sp.text_edit_width, own.spacing.text_edit_width);
            assert_eq!(sp.slider_width, own.spacing.slider_width);
            assert_eq!(sp.default_area_size, own.spacing.default_area_size);
            assert_eq!(sp.indent, own.spacing.indent);
            assert_eq!(
                s.interaction.tooltip_delay,
                t.tooltip
                    .show_delay
                    .map_or(own.interaction.tooltip_delay, |ms| f32::from(ms) / 1000.0),
                "{preset}: tooltip.show_delay in seconds, else egui's own"
            );
            assert_eq!(
                egui::style::Interaction {
                    tooltip_delay: own.interaction.tooltip_delay,
                    ..s.interaction.clone()
                },
                own.interaction,
                "every other Interaction field is egui's own"
            );
            assert_eq!(s.animation_time, own.animation_time);
            assert_eq!(s.override_font_id, None);
            assert_eq!(v.override_text_color, None);
        }
    }

    /// egui paints the scroll-bar groove from `extreme_bg_color`, at the three
    /// `*_background_opacity` when floating and at 1 when solid
    /// (`egui/src/containers/scroll_area.rs:1485-1511`). An overlay bar whose stated track is
    /// fully transparent has no groove: the opacities are 0 and the slot takes the input
    /// background, egui's own meaning of it (`egui/src/style.rs:1513`). A solid bar's
    /// transparent track (windows-11, kde-breeze) and an opaque one (material-3) stay in the slot.
    #[test]
    fn a_clear_overlay_track_leaves_the_slot_to_the_input_background() {
        let opacities = |s: &egui::Style| {
            let sc = &s.spacing.scroll;
            [
                sc.dormant_background_opacity,
                sc.active_background_opacity,
                sc.interact_background_opacity,
            ]
        };
        for (preset, mode, scheme) in [
            ("macos-monterey", ColorMode::Light, egui::Theme::Light),
            ("macos-monterey", ColorMode::Dark, egui::Theme::Dark),
            ("macos-tahoe", ColorMode::Light, egui::Theme::Light),
        ] {
            let t = resolved(preset, mode);
            assert!(t.scrollbar.overlay_mode, "{preset}: an overlay bar");
            assert_eq!(t.scrollbar.track_color.a, 0, "{preset}: a clear track");
            let (s, notes) = build(scheme, &t, &LayoutTheme::default(), None);
            assert!(notes.is_empty(), "{preset}: {notes:?}");
            assert_eq!(
                s.visuals.extreme_bg_color,
                to_color32(t.input.background_color),
                "{preset} {mode:?}"
            );
            assert_eq!(opacities(&s), [0.0; 3], "{preset} {mode:?}");
        }
        assert_eq!(
            resolved("material-3", ColorMode::Light)
                .scrollbar
                .track_color
                .a,
            255,
            "material-3: the opaque-track witness"
        );
        for (preset, mode, scheme) in [
            ("kde-breeze", ColorMode::Light, egui::Theme::Light),
            ("windows-11", ColorMode::Light, egui::Theme::Light),
            ("windows-11", ColorMode::Dark, egui::Theme::Dark),
            ("material-3", ColorMode::Light, egui::Theme::Light),
        ] {
            let t = resolved(preset, mode);
            let (s, _) = build(scheme, &t, &LayoutTheme::default(), None);
            assert_eq!(
                s.visuals.extreme_bg_color,
                to_color32(t.scrollbar.track_color),
                "{preset} {mode:?}"
            );
            assert_eq!(
                opacities(&s),
                opacities(&scheme.default_style()),
                "{preset} {mode:?}: egui's own"
            );
        }
        let t = resolved("windows-11", ColorMode::Light);
        assert!(
            !t.scrollbar.overlay_mode && t.scrollbar.track_color.a == 0,
            "windows-11 states its transparent track in solid mode"
        );
    }

    /// §6.16: a stated gap is `Vec2::splat`, a `None` keeps egui's asymmetric default, a
    /// non-finite gap keeps it too and is reported.
    #[test]
    fn widget_gap_is_splat_or_egui_own() {
        let t = resolved("adwaita", ColorMode::Light);
        let own = egui::Theme::Light.default_style();
        let stated = LayoutTheme {
            widget_gap: Some(11.0),
            ..LayoutTheme::default()
        };
        let (s, notes) = build(egui::Theme::Light, &t, &stated, None);
        assert_eq!(s.spacing.item_spacing, egui::Vec2::splat(11.0));
        assert!(notes.is_empty());
        let hostile = LayoutTheme {
            widget_gap: Some(f32::NAN),
            ..LayoutTheme::default()
        };
        let (s, notes) = build(egui::Theme::Light, &t, &hostile, None);
        assert_eq!(s.spacing.item_spacing, own.spacing.item_spacing);
        assert_eq!(
            notes,
            vec![Note::ValueSanitised {
                path: "layout.widget_gap"
            }]
        );
    }

    /// `tooltip.show_delay` is milliseconds, egui's `tooltip_delay` seconds
    /// (`egui/src/style.rs:928`); a theme that states none keeps egui's own.
    #[test]
    fn tooltip_delay_is_the_themes_in_seconds_or_egui_own() {
        let own = egui::Theme::Light.default_style();
        for (preset, ms) in [("kde-breeze", 700_u16), ("material-3", 0)] {
            let t = resolved(preset, ColorMode::Light);
            assert_eq!(t.tooltip.show_delay, Some(ms), "{preset}");
            let (s, notes) = build(egui::Theme::Light, &t, &LayoutTheme::default(), None);
            assert_eq!(
                s.interaction.tooltip_delay,
                f32::from(ms) / 1000.0,
                "{preset}"
            );
            assert!(notes.is_empty(), "{preset}: {notes:?}");
        }
        let mut t = resolved("adwaita", ColorMode::Light);
        t.tooltip.show_delay = Some(u16::MAX);
        let (s, _) = build(egui::Theme::Light, &t, &LayoutTheme::default(), None);
        assert_eq!(s.interaction.tooltip_delay, 65.535);
        t.tooltip.show_delay = None;
        let (s, _) = build(egui::Theme::Light, &t, &LayoutTheme::default(), None);
        assert_eq!(s.interaction.tooltip_delay, own.interaction.tooltip_delay);
    }

    /// `input.caret_blink_on` and `caret_blink_off` are milliseconds, egui's
    /// `TextCursorStyle` durations seconds; a hidden duration of 0 is no blink, and a theme that
    /// states no blink keeps egui's own three values.
    #[test]
    fn caret_blink_is_the_themes_in_seconds_or_egui_own() {
        let blink_of = |s: &egui::Style| {
            let c = &s.visuals.text_cursor;
            (c.blink, c.on_duration, c.off_duration)
        };
        let own = blink_of(&egui::Theme::Light.default_style());

        let t = resolved("ios-13", ColorMode::Light);
        let (s, notes) = build(egui::Theme::Light, &t, &LayoutTheme::default(), None);
        assert_eq!(blink_of(&s), (true, 0.65, 0.35), "ios-13");
        assert!(notes.is_empty(), "ios-13: {notes:?}");

        let t = resolved("catppuccin-mocha", ColorMode::Dark);
        let (s, notes) = build(egui::Theme::Dark, &t, &LayoutTheme::default(), None);
        assert_eq!(
            blink_of(&s),
            blink_of(&egui::Theme::Dark.default_style()),
            "catppuccin-mocha"
        );
        assert!(notes.is_empty(), "catppuccin-mocha: {notes:?}");

        let mut t = resolved("adwaita", ColorMode::Light);
        t.input.caret_blink_off = Some(0);
        let (s, notes) = build(egui::Theme::Light, &t, &LayoutTheme::default(), None);
        assert_eq!(blink_of(&s), (false, 0.6, 0.0), "adwaita, no blink");
        assert!(notes.is_empty(), "adwaita, no blink: {notes:?}");

        t.input.caret_blink_off = None;
        assert_eq!(t.input.caret_blink_on, Some(600));
        let (s, notes) = build(egui::Theme::Light, &t, &LayoutTheme::default(), None);
        assert_eq!(blink_of(&s), own, "adwaita, no hidden duration");
        assert!(notes.is_empty(), "adwaita, no hidden duration: {notes:?}");
    }

    /// §6.15: the delta between the theme's line box and the row, floored at egui's 0.0.
    #[test]
    fn line_spacing_is_the_theme_line_box_less_the_row() {
        let mut t = resolved("adwaita", ColorMode::Light);
        let size = t.defaults.font.size;
        let row = 1.0 * size; // a row exactly one em tall
        t.defaults.line_height = 1.5;
        let (s, _) = build(egui::Theme::Light, &t, &LayoutTheme::default(), Some(row));
        assert_eq!(
            s.spacing.extra_text_line_spacing,
            clamp_length(1.5 * size - row)
        );
        t.defaults.line_height = 0.8; // a line box below the row: the floor
        let (s, _) = build(egui::Theme::Light, &t, &LayoutTheme::default(), Some(row));
        assert_eq!(s.spacing.extra_text_line_spacing, 0.0);
        t.defaults.line_height = f32::INFINITY;
        let (s, notes) = build(egui::Theme::Light, &t, &LayoutTheme::default(), Some(row));
        assert_eq!(s.spacing.extra_text_line_spacing, 0.0);
        assert!(notes.contains(&Note::ValueSanitised {
            path: "defaults.line_height"
        }));
    }

    /// §8.5: a size that is not a positive normal `f32` keeps egui's own for the slot, once
    /// reported per leaf.
    #[test]
    fn a_bad_text_size_keeps_egui_own_and_is_reported() {
        let own = egui::Theme::Light.default_style();
        for bad in [f32::NAN, f32::INFINITY, 0.0, -12.0, 1e-45] {
            let mut t = resolved("adwaita", ColorMode::Light);
            t.defaults.font.size = bad;
            let (s, notes) = build(egui::Theme::Light, &t, &LayoutTheme::default(), None);
            assert_eq!(
                s.text_styles.get(&TextStyle::Body).unwrap().size,
                own.text_styles.get(&TextStyle::Body).unwrap().size,
                "{bad}"
            );
            assert_eq!(
                notes
                    .iter()
                    .filter(|n| **n
                        == Note::ValueSanitised {
                            path: "defaults.font.size"
                        })
                    .count(),
                1,
                "{bad}: once per leaf"
            );
        }
    }

    /// §7.2's emission rule: a non-finite radius, width, opacity or side keeps egui's own and
    /// is reported; a saturated margin side is reported as such.
    #[test]
    fn non_finite_and_saturated_values_are_reported() {
        let own = egui::Theme::Light.default_style();
        let mut t = resolved("adwaita", ColorMode::Light);
        t.defaults.border.corner_radius_lg = f32::NAN;
        t.defaults.border.line_width = f32::NEG_INFINITY;
        t.defaults.disabled_opacity = f32::INFINITY;
        t.window.border.padding.left = Some(1000.0);
        t.window.border.padding.top = Some(f32::NAN);
        let (s, notes) = build(egui::Theme::Light, &t, &LayoutTheme::default(), None);
        assert_eq!(s.visuals.menu_corner_radius, own.visuals.menu_corner_radius);
        assert_eq!(
            s.visuals.window_stroke.width,
            own.visuals.window_stroke.width
        );
        assert_eq!(s.visuals.disabled_alpha, own.visuals.disabled_alpha);
        assert_eq!(s.spacing.window_margin.left, i8::MAX);
        assert_eq!(s.spacing.window_margin.top, own.spacing.window_margin.top);
        for note in [
            Note::ValueSanitised {
                path: "defaults.border.corner_radius_lg",
            },
            Note::ValueSanitised {
                path: "defaults.border.line_width",
            },
            Note::ValueSanitised {
                path: "defaults.disabled_opacity",
            },
            Note::ValueSanitised {
                path: "window.border.padding.top",
            },
            Note::ValueSaturated {
                path: "window.border.padding.left",
            },
        ] {
            assert_eq!(notes.iter().filter(|n| **n == note).count(), 1, "{note:?}");
        }
    }
}
