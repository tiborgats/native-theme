//! The derived formulas of spec §6.5–§6.12: the sinks no leaf reaches by a straight copy.
//!
//! Every function is total. A non-finite leaf takes egui's own value — the one the style being
//! built already carries at that sink (§6, §7.2) — and is reported once with
//! `Note::ValueSanitised`; a `defaults.*` leaf is reported by the base style (Task 13), so a
//! formula that reads one substitutes silently. Every length then passes `clamp_length`. The
//! numeric literals are §6.17's structural constants (`0.0`, `1.0`) and its cited exemptions
//! (`0.5`, `0.8`, `1.25`, `2.5`, `2.0`, `4.0 / 3.0`), each named at its use.

use native_theme::theme::ResolvedTheme;

use crate::convert::{
    clamp_length, finite_or, i8_from_f32_saturating, to_color32, to_corner_radius, to_margin,
    to_shadow, to_stroke,
};
use crate::style::{BuildInput, note_transparent_fill, push_note};
use crate::{Note, Role};

fn sanitised(path: &'static str, notes: &mut Vec<Note>) {
    push_note(notes, Note::ValueSanitised { path });
}

/// The derived writes of the base style: D1 (§6.5), D5 (§6.9) and the two shadow gates (§6.14,
/// §5.9). Called last in `base_style` (Task 13); no earlier task writes these sinks.
pub(crate) fn apply_base(style: &mut egui::Style, input: &BuildInput<'_>, notes: &mut Vec<Note>) {
    let t = input.theme;
    scrollbar_widths(style, t, notes);
    scrollbar_pressed_thumb(style, t, notes);
    // §6.14: only the colour is replaced, egui's geometry kept; `false` is exactly `Shadow::NONE`.
    // `window_shadow`'s owner is the window's own border, `popup_shadow`'s `defaults.border` (§5.9).
    let v = &mut style.visuals;
    v.window_shadow = to_shadow(
        v.window_shadow,
        t.defaults.shadow_color,
        t.window.border.shadow_enabled,
    );
    v.popup_shadow = to_shadow(
        v.popup_shadow,
        t.defaults.shadow_color,
        t.defaults.border.shadow_enabled,
    );
}

/// The derived writes of one role cell, called last in `role_cell` (Task 15); a role with none
/// returns at once.
pub(crate) fn apply_role(
    role: Role,
    style: &mut egui::Style,
    input: &BuildInput<'_>,
    notes: &mut Vec<Note>,
) {
    let t = input.theme;
    match role {
        Role::Scrollbar => {
            scrollbar_radii(style, t);
            scrollbar_pressed_thumb(style, t, notes);
        }
        Role::Slider => {
            slider_geometry(style, t, input.row_height, notes);
            slider_rail_radius(style, t);
            slider_thumb_colours(style, t, notes);
        }
        Role::Spinner => spinner_size(style, t, notes),
        Role::Splitter => splitter_strokes(style, t, notes),
        Role::Toolbar => toolbar_bar_height(style, t, notes),
        Role::ComboBox => combo_box_arrow_area(style, t, notes),
        Role::Expander => expander_arrow_size(style, t, notes),
        Role::Checkbox => checkbox_mark_inset(style, t, notes),
        _ => {}
    }
}

// ---- D1: scrollbar groove and thumb widths (§6.5) ---------------------------------------------

/// The layout reserves the platform groove while the painted bar is the thumb, centred in it:
/// `allocated_width()` is `bar_inner_margin + bar_width + bar_outer_margin`
/// (`egui/src/style.rs:656-662`). The finiteness test runs on both leaves before any arithmetic:
/// a non-finite width leaves the whole `ScrollStyle` as the starting style has it — egui's
/// `ScrollStyle::floating()`, its `Default` (`egui/src/style.rs:585-589`) — never a `0.0` bar.
fn scrollbar_widths(style: &mut egui::Style, t: &ResolvedTheme, notes: &mut Vec<Note>) {
    let sb = &t.scrollbar;
    let mut finite = true;
    if !sb.groove_width.is_finite() {
        sanitised("scrollbar.groove_width", notes);
        finite = false;
    }
    if !sb.thumb_width.is_finite() {
        sanitised("scrollbar.thumb_width", notes);
        finite = false;
    }
    if !finite {
        return;
    }
    let g = clamp_length(sb.groove_width);
    let thumb = clamp_length(sb.thumb_width).min(g); // the thumb is never wider than the groove
    let pad = (g - thumb) * 0.5; // 0.5: the midpoint (§6.17); finite and >= 0.0, as g and thumb are
    let scroll = &mut style.spacing.scroll;
    scroll.foreground_color = false; // `egui/src/style.rs:538`: the handle reads `bg_fill` (§5.5)
    if sb.overlay_mode {
        scroll.floating = true; // `:503`
        scroll.floating_allocated_width = 0.0; // `:535`: `ScrollStyle::floating()`'s own value; reserves nothing
        scroll.floating_width = thumb; // `:527`: idle thickness
        scroll.bar_width = g; // `:512`: hover thickness and hit target
    } else {
        scroll.floating = false;
        scroll.bar_width = thumb; // `:512`
        scroll.bar_inner_margin = pad; // `:518`
        scroll.bar_outer_margin = pad; // `:522`
    }
}

// ---- D5, D6, D7: the remaining indicator fallbacks (§6.9) --------------------------------------

/// The pressed thumb: `thumb_active_color`, or `thumb_hover_color` where the platform states
/// none. Written as given (C17): the thumb sits on a trough egui also paints.
fn scrollbar_pressed_thumb(style: &mut egui::Style, t: &ResolvedTheme, notes: &mut Vec<Note>) {
    let (c, path) = match t.scrollbar.thumb_active_color {
        Some(c) => (c, "scrollbar.thumb_active_color"),
        None => (t.scrollbar.thumb_hover_color, "scrollbar.thumb_hover_color"),
    };
    let c = to_color32(c);
    note_transparent_fill(c, path, notes);
    style.visuals.widgets.active.bg_fill = c;
}

/// The hovered and pressed handle: `thumb_hover_color`, or `thumb_color` where the platform
/// states none — the one route by which `thumb_color` reaches the screen (§5.5). `active :=
/// hovered` (§6.1).
fn slider_thumb_colours(style: &mut egui::Style, t: &ResolvedTheme, notes: &mut Vec<Note>) {
    let (c, path) = match t.slider.thumb_hover_color {
        Some(c) => (c, "slider.thumb_hover_color"),
        None => (t.slider.thumb_color, "slider.thumb_color"),
    };
    let c = to_color32(c);
    note_transparent_fill(c, path, notes);
    style.visuals.widgets.hovered.bg_fill = c;
    style.visuals.widgets.active.bg_fill = c;
}

/// The splitter's three lines (`egui/src/containers/panel.rs:906-911`), one width, the drag line
/// in the hover colour since the model states no drag colour. `to_stroke` applies §6's rule to
/// the width: a non-finite `divider_width` keeps each stroke's own width, the colours are still
/// written, and the leaf is reported.
fn splitter_strokes(style: &mut egui::Style, t: &ResolvedTheme, notes: &mut Vec<Note>) {
    let sp = &t.splitter;
    if !sp.divider_width.is_finite() {
        sanitised("splitter.divider_width", notes);
    }
    let w = &mut style.visuals.widgets;
    w.noninteractive.bg_stroke = to_stroke(
        w.noninteractive.bg_stroke,
        sp.divider_color,
        sp.divider_width,
    );
    w.hovered.fg_stroke = to_stroke(w.hovered.fg_stroke, sp.hover_color, sp.divider_width);
    w.active.fg_stroke = w.hovered.fg_stroke;
}

// ---- D2: slider thumb diameter (§6.6) ----------------------------------------------------------

/// egui's knob chain (`egui/src/widgets/slider.rs:957-959`, `:885`, `:815`): `thickness` is the
/// Body row height floored at `interact_size.y`, the handle radius `thickness / 2.5`, the disc
/// `handle_r + expansion` and its outline tessellated outside that. The atlas sets
/// `interact_size.y = 1.25 · d` (the reciprocal of egui's `0.8`) and one `expansion` in every
/// state so the painted knob, outline included, is `slider.thumb_diameter` across at every text
/// size; `row` is the Body row height of §6.15, `Some` only when finite and positive.
fn slider_geometry(
    style: &mut egui::Style,
    t: &ResolvedTheme,
    row: Option<f32>,
    notes: &mut Vec<Note>,
) {
    let d = t.slider.thumb_diameter;
    let base_y = style.spacing.interact_size.y; // egui's own value for this sink: what the style carries
    style.spacing.interact_size.y = if d.is_finite() {
        clamp_length(1.25 * d) // 0.8 · (1.25 d) == d; `slider.rs:885` (§6.17)
    } else {
        sanitised("slider.thumb_diameter", notes);
        base_y
    };
    let w = style.visuals.widgets.inactive.fg_stroke.width; // every state's width (§6.1): finite, >= 0.0
    let e = match row {
        Some(h) if d.is_finite() => {
            let thickness = h.max(style.spacing.interact_size.y); // `slider.rs:957-959`
            // 0.8 and 0.5: egui's chain and a midpoint (§6.17); `.min(0.0)` only ever shrinks
            ((clamp_length(d) - 0.8 * thickness) * 0.5).min(0.0) - w
        }
        _ => -w, // no row height, or no finite diameter: exact wherever the floor does not bite
    };
    let wv = &mut style.visuals.widgets;
    for entry in [
        &mut wv.noninteractive,
        &mut wv.inactive,
        &mut wv.hovered,
        &mut wv.active,
        &mut wv.open,
    ] {
        entry.expansion = e;
    }
    // A diameter presumes a round knob; egui's default `Rect { aspect_ratio: 0.75 }`
    // (`egui/src/style.rs:1553`) has no theme value behind it.
    style.visuals.handle_shape = egui::style::HandleShape::Circle; // `egui/src/style.rs:1237`
}

// ---- D3: spinner size (§6.7) -------------------------------------------------------------------

/// `Spinner` allocates `vec2(size, size)` exactly (`egui/src/widgets/spinner.rs:65-68`), so the
/// minimum collapses to a `max`. Each operand passes `finite_or` **before** the `max`, because
/// `f32::max` would silently drop a `NaN` instead of reporting it; the diameter's fallback is
/// egui's own `interact_size.y`, the minimum's the `0.0` egui has as no floor (§6.17).
fn spinner_size(style: &mut egui::Style, t: &ResolvedTheme, notes: &mut Vec<Note>) {
    let base_y = style.spacing.interact_size.y;
    if !t.spinner.diameter.is_finite() {
        sanitised("spinner.diameter", notes);
    }
    if !t.spinner.min_diameter.is_finite() {
        sanitised("spinner.min_diameter", notes);
    }
    let d = finite_or(t.spinner.diameter, base_y);
    let m = finite_or(t.spinner.min_diameter, 0.0);
    style.spacing.interact_size.y = clamp_length(d.max(m));
}

// ---- D4: corner radii with no native source (§6.8) ---------------------------------------------

/// The scrollbar's handle and trough radii, from `defaults.border.corner_radius`; no cap, since
/// the tessellator clamps a radius to half the painted rect (`epaint/src/tessellator.rs:638-642`).
/// A non-finite radius keeps each sink's own value through `to_corner_radius`; the base style
/// reports the leaf.
fn scrollbar_radii(style: &mut egui::Style, t: &ResolvedTheme) {
    let r = t.defaults.border.corner_radius;
    let w = &mut style.visuals.widgets;
    for entry in [&mut w.inactive, &mut w.hovered, &mut w.active] {
        entry.corner_radius = to_corner_radius(entry.corner_radius, r);
    }
}

/// The slider rail's radius, capped at half the rail height the scope carries
/// (`spacing.slider_rail_height`, finite by §6's rule): the rail is trailing-fill geometry too
/// (`egui/src/widgets/slider.rs:792`, `:795`). `0.5` is the definition of a fully rounded end.
fn slider_rail_radius(style: &mut egui::Style, t: &ResolvedTheme) {
    let r = t.defaults.border.corner_radius;
    let cap = style.spacing.slider_rail_height * 0.5;
    let fit = if r.is_finite() {
        r.max(0.0).min(cap)
    } else {
        r
    }; // a non-finite radius passes through
    let w = &mut style.visuals.widgets;
    w.inactive.corner_radius = to_corner_radius(w.inactive.corner_radius, fit);
}

// ---- R-BAR: toolbar bar height (§6.10) ---------------------------------------------------------

/// `Panel::show` sizes a top panel as `interact_size.y + frame.total_margin().sum().y`
/// (`egui/src/containers/panel.rs:1068-1073`), the frame being the one `resolve_frame` builds
/// from this scope (`:947-965`): egui's `side_top_panel` preset with the toolbar's padding — the
/// same frame Task 18 builds for `Surface::Panel(PanelSide::Top)` — plus the separator room it
/// reserves in the outer margin, `noninteractive.bg_stroke.width.round() as i8` (`:961-965`). So
/// the stated height, less all of that, is what the scope's `interact_size.y` must hold.
fn toolbar_bar_height(style: &mut egui::Style, t: &ResolvedTheme, notes: &mut Vec<Note>) {
    let Some(h) = t.toolbar.bar_height else {
        return; // S2: no stated height; egui's own sizing stands
    };
    if !h.is_finite() {
        sanitised("toolbar.bar_height", notes);
        return; // egui's own value: what the style carries
    }
    let frame = egui::Frame::side_top_panel(style);
    // A non-finite padding side keeps the preset's; the Panel(Top) frame reports it (Task 18).
    let inner = to_margin(frame.inner_margin, &t.toolbar.border.padding);
    let separator = f32::from(i8_from_f32_saturating(
        style.visuals.widgets.noninteractive.bg_stroke.width,
    ));
    let total_v = f32::from(inner.top)
        + f32::from(inner.bottom)
        + 2.0 * frame.stroke.width // a stroke has two sides: `Frame::total_margin`, `egui/src/containers/frame.rs:327-331` (§6.17)
        + f32::from(frame.outer_margin.top)
        + f32::from(frame.outer_margin.bottom)
        + separator;
    style.spacing.interact_size.y = clamp_length(h - total_v);
}

// ---- R-ARROW and the checkbox mark's inset (§6.11) ---------------------------------------------

/// The painted triangle is the icon rect scaled by `0.75`
/// (`egui/src/containers/collapsing_header.rs:342`), the rect being `icon_width_inner` square
/// (`egui/src/style.rs:477-478`); `4.0 / 3.0` is that scaling's reciprocal (§6.17). The
/// finiteness test comes first: `clamp_length` would turn a `NaN` into `0.0`, not egui's value.
fn expander_arrow_size(style: &mut egui::Style, t: &ResolvedTheme, notes: &mut Vec<Note>) {
    let size = t.expander.arrow_icon_size;
    if size.is_finite() {
        style.spacing.icon_width_inner = clamp_length(size * (4.0 / 3.0));
    } else {
        sanitised("expander.arrow_icon_size", notes);
    }
}

/// The mark spans a `check_size` square centred in the `icon_width` box
/// (`egui/src/widget_style.rs:180`, `egui/src/widgets/checkbox.rs:132-133`), so the padding —
/// the mark's inset — is the box less the mean of the two pairs' insets, written only where all
/// four sides are stated. `icon_width` is read back from the cell, where it is already
/// `checkbox.indicator_width` after its own rule (§5.3).
fn checkbox_mark_inset(style: &mut egui::Style, t: &ResolvedTheme, notes: &mut Vec<Note>) {
    let p = &t.checkbox.border.padding;
    let (Some(l), Some(r), Some(top), Some(b)) = (p.left, p.right, p.top, p.bottom) else {
        return; // a side unstated: egui's own value stands, and nothing is reported
    };
    let sides = [
        ("checkbox.border.padding.left", l),
        ("checkbox.border.padding.right", r),
        ("checkbox.border.padding.top", top),
        ("checkbox.border.padding.bottom", b),
    ];
    let mut finite = true;
    for (path, v) in sides {
        if !v.is_finite() {
            sanitised(path, notes);
            finite = false;
        }
    }
    if !finite {
        return;
    }
    let inset = f32::midpoint(l, r) + f32::midpoint(top, b); // §7.2's midpoint: overflow-free per pair
    style.spacing.icon_width_inner = clamp_length(style.spacing.icon_width - inset);
}

// ---- the combo-box arrow area (§6.12) ----------------------------------------------------------

/// egui reserves `icon_spacing + icon_width` for the arrow
/// (`egui/src/containers/combo_box.rs:355`, `:360`); `icon_width` is read back from the cell,
/// where it is already `combo_box.arrow_icon_size` after its rule (§5.4).
fn combo_box_arrow_area(style: &mut egui::Style, t: &ResolvedTheme, notes: &mut Vec<Note>) {
    let Some(w) = t.combo_box.arrow_area_width else {
        return; // S2: no stated width; the base style's gap stands
    };
    if !w.is_finite() {
        sanitised("combo_box.arrow_area_width", notes);
        return;
    }
    style.spacing.icon_spacing = clamp_length(w - style.spacing.icon_width);
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
    use egui::style::HandleShape;
    use native_theme::color::Rgba;
    use native_theme::theme::{ColorMode, ResolvedTheme};

    use super::*;
    use crate::convert::{
        clamp_length, composite_over, to_color32, to_color32_with_opacity, to_margin, to_shadow,
    };
    use crate::install_tests::resolved;
    use crate::style::{BuildInput, SchemeStyles, compile};
    use crate::{AccessibilityPreferences, LayoutTheme, Note, Role, RoleVariant};

    fn theme() -> ResolvedTheme {
        resolved("kde-breeze", ColorMode::Light)
    }

    fn start() -> egui::Style {
        egui::Theme::Light.default_style()
    }

    fn build(t: &ResolvedTheme) -> (SchemeStyles, Vec<Note>) {
        let prefs = AccessibilityPreferences::default();
        let layout = LayoutTheme::default();
        let input = BuildInput {
            scheme: egui::Theme::Light,
            theme: t,
            prefs: &prefs,
            layout: &layout,
            row_height: None,
            patch: None,
        };
        let mut notes = Vec::new();
        let s = compile(&input, &mut notes);
        (s, notes)
    }

    fn sanitised(notes: &[Note], path: &str) -> usize {
        notes
            .iter()
            .filter(|n| matches!(n, Note::ValueSanitised { path: p, .. } if *p == path))
            .count()
    }

    fn entries(w: &egui::style::Widgets) -> [egui::style::WidgetVisuals; 5] {
        [w.noninteractive, w.inactive, w.hovered, w.active, w.open]
    }

    // ---- D1, §6.5 -------------------------------------------------------------------------

    /// A solid scrollbar: the layout reserves the groove, the painted bar is the thumb, centred.
    #[test]
    fn d1_a_solid_groove_reserves_the_groove_and_paints_the_thumb() {
        let mut t = theme();
        t.scrollbar.overlay_mode = false;
        let (g, th) = (t.scrollbar.groove_width, t.scrollbar.thumb_width);
        assert!(g > th, "kde-breeze states 21 / 8");
        let mut s = start();
        let mut notes = Vec::new();
        scrollbar_widths(&mut s, &t, &mut notes);
        let sc = &s.spacing.scroll;
        assert!(!sc.floating);
        assert!(!sc.foreground_color);
        assert_eq!(sc.bar_width, clamp_length(th));
        assert_eq!(
            sc.bar_inner_margin,
            (clamp_length(g) - clamp_length(th)) * 0.5
        );
        assert_eq!(sc.bar_outer_margin, sc.bar_inner_margin);
        assert_eq!(sc.allocated_width(), clamp_length(g)); // `egui/src/style.rs:656-662`
        assert!(notes.is_empty());
    }

    #[test]
    fn d1_a_thumb_wider_than_the_groove_clamps_and_a_zero_groove_paints_nothing() {
        let mut t = theme();
        t.scrollbar.overlay_mode = false;
        t.scrollbar.groove_width = 4.0;
        t.scrollbar.thumb_width = 9.0;
        let mut s = start();
        scrollbar_widths(&mut s, &t, &mut Vec::new());
        assert_eq!(s.spacing.scroll.bar_width, 4.0);
        assert_eq!(s.spacing.scroll.bar_inner_margin, 0.0);
        assert_eq!(s.spacing.scroll.bar_outer_margin, 0.0);

        t.scrollbar.groove_width = 0.0;
        let mut s = start();
        scrollbar_widths(&mut s, &t, &mut Vec::new());
        assert_eq!(s.spacing.scroll.bar_width, 0.0);
        assert_eq!(s.spacing.scroll.allocated_width(), 0.0);

        t.scrollbar.groove_width = -3.0;
        let mut s = start();
        scrollbar_widths(&mut s, &t, &mut Vec::new());
        assert_eq!(s.spacing.scroll.allocated_width(), 0.0);
    }

    #[test]
    fn d1_the_overlay_branch_reserves_nothing_and_keeps_egui_s_opacities() {
        let mut t = theme();
        t.scrollbar.overlay_mode = true;
        let mut s = start();
        scrollbar_widths(&mut s, &t, &mut Vec::new());
        let sc = &s.spacing.scroll;
        let own = start().spacing.scroll;
        assert!(sc.floating);
        assert!(!sc.foreground_color);
        assert_eq!(sc.floating_allocated_width, 0.0);
        assert_eq!(sc.allocated_width(), 0.0);
        assert_eq!(
            sc.floating_width,
            clamp_length(t.scrollbar.thumb_width).min(clamp_length(t.scrollbar.groove_width))
        );
        assert_eq!(sc.bar_width, clamp_length(t.scrollbar.groove_width));
        assert_eq!(
            sc.dormant_background_opacity,
            own.dormant_background_opacity
        );
        assert_eq!(sc.dormant_handle_opacity, own.dormant_handle_opacity);
        assert_eq!(sc.active_handle_opacity, own.active_handle_opacity);
        assert_eq!(sc.interact_handle_opacity, own.interact_handle_opacity);
    }

    /// A non-finite width leaves the whole `ScrollStyle` as the starting style has it and
    /// names the non-finite leaf, and only that one.
    #[test]
    fn d1_a_non_finite_width_writes_nothing_and_names_the_leaf() {
        let mut t = theme();
        t.scrollbar.groove_width = f32::NAN;
        let mut s = start();
        let mut notes = Vec::new();
        scrollbar_widths(&mut s, &t, &mut notes);
        assert_eq!(s.spacing.scroll, start().spacing.scroll);
        assert_eq!(sanitised(&notes, "scrollbar.groove_width"), 1);
        assert_eq!(sanitised(&notes, "scrollbar.thumb_width"), 0);

        let mut t = theme();
        t.scrollbar.thumb_width = f32::INFINITY;
        let mut s = start();
        let mut notes = Vec::new();
        scrollbar_widths(&mut s, &t, &mut notes);
        assert_eq!(s.spacing.scroll, start().spacing.scroll);
        assert_eq!(sanitised(&notes, "scrollbar.thumb_width"), 1);
        assert_eq!(sanitised(&notes, "scrollbar.groove_width"), 0);
    }

    // ---- D2, §6.6 -------------------------------------------------------------------------

    /// Where the Body row does not bite, `thickness = 1.25 · d` and `e = −w`: the painted knob,
    /// outline included, is `d` across.
    #[test]
    fn d2_the_knob_is_the_diameter_where_the_row_does_not_bite() {
        let mut t = theme();
        t.slider.thumb_diameter = 20.0;
        let mut s = start();
        let w = s.visuals.widgets.inactive.fg_stroke.width;
        let mut notes = Vec::new();
        slider_geometry(&mut s, &t, Some(15.0), &mut notes);
        assert_eq!(s.spacing.interact_size.y, 25.0);
        for e in entries(&s.visuals.widgets) {
            assert_eq!(e.expansion, -w);
        }
        assert_eq!(s.visuals.handle_shape, HandleShape::Circle);
        assert!(notes.is_empty());
    }

    /// A Body row taller than `1.25 · d` — a text-scaling factor above `1.0` — shrinks the disc
    /// back so the knob stays `d`: `handle_r + e + w == d / 2`.
    #[test]
    fn d2_a_taller_body_row_shrinks_the_disc_back_to_the_diameter() {
        let mut t = theme();
        let d = 20.0_f32;
        t.slider.thumb_diameter = d;
        let mut s = start();
        let w = s.visuals.widgets.inactive.fg_stroke.width;
        let h = 30.0_f32;
        slider_geometry(&mut s, &t, Some(h), &mut Vec::new());
        let thickness = h.max(s.spacing.interact_size.y);
        let e = s.visuals.widgets.inactive.expansion;
        assert_eq!(e, ((d - 0.8 * thickness) * 0.5).min(0.0) - w);
        assert!(e < -w, "the first term is negative here");
        let handle_r = thickness / 2.5; // `egui/src/widgets/slider.rs:885`
        assert!((handle_r + e + w - d * 0.5).abs() < 1e-4);
    }

    #[test]
    fn d2_no_row_height_a_zero_diameter_and_a_non_finite_one() {
        let mut t = theme();
        t.slider.thumb_diameter = 20.0;
        let mut s = start();
        let w = s.visuals.widgets.inactive.fg_stroke.width;
        slider_geometry(&mut s, &t, None, &mut Vec::new());
        assert_eq!(
            s.visuals.widgets.hovered.expansion, -w,
            "no row height: the exact value where the floor does not bite"
        );

        t.slider.thumb_diameter = 0.0;
        let mut s = start();
        let h = 16.0_f32;
        slider_geometry(&mut s, &t, Some(h), &mut Vec::new());
        assert_eq!(s.spacing.interact_size.y, 0.0);
        assert_eq!(
            s.visuals.widgets.inactive.expansion,
            ((0.0 - 0.8 * h.max(0.0)) * 0.5).min(0.0) - w
        );

        t.slider.thumb_diameter = -7.0;
        let mut s = start();
        slider_geometry(&mut s, &t, None, &mut Vec::new());
        assert_eq!(
            s.spacing.interact_size.y, 0.0,
            "a negative diameter is floored"
        );

        t.slider.thumb_diameter = f32::NAN;
        let mut s = start();
        let mut notes = Vec::new();
        slider_geometry(&mut s, &t, Some(h), &mut notes);
        assert_eq!(
            s.spacing.interact_size.y,
            start().spacing.interact_size.y,
            "egui's own value"
        );
        assert_eq!(s.visuals.widgets.active.expansion, -w);
        assert_eq!(sanitised(&notes, "slider.thumb_diameter"), 1);
        assert_eq!(s.visuals.handle_shape, HandleShape::Circle);
    }

    // ---- D3, §6.7 -------------------------------------------------------------------------

    #[test]
    fn d3_the_spinner_takes_the_larger_of_diameter_and_minimum() {
        let mut t = theme();
        t.spinner.diameter = 16.0;
        t.spinner.min_diameter = 24.0;
        let mut s = start();
        let mut notes = Vec::new();
        spinner_size(&mut s, &t, &mut notes);
        assert_eq!(s.spacing.interact_size.y, 24.0);
        assert!(notes.is_empty());

        t.spinner.min_diameter = -1.0;
        let mut s = start();
        spinner_size(&mut s, &t, &mut Vec::new());
        assert_eq!(s.spacing.interact_size.y, 16.0);

        t.spinner.diameter = -5.0;
        let mut s = start();
        spinner_size(&mut s, &t, &mut Vec::new());
        assert_eq!(s.spacing.interact_size.y, 0.0, "a negative size is floored");
    }

    /// Each operand passes `finite_or` before the `max`: a `NaN` is reported, never silently
    /// dropped by `f32::max`.
    #[test]
    fn d3_a_non_finite_operand_takes_egui_s_value_and_is_reported() {
        let mut t = theme();
        t.spinner.diameter = f32::INFINITY;
        t.spinner.min_diameter = 0.0;
        let mut s = start();
        let mut notes = Vec::new();
        spinner_size(&mut s, &t, &mut notes);
        assert_eq!(s.spacing.interact_size.y, start().spacing.interact_size.y);
        assert_eq!(sanitised(&notes, "spinner.diameter"), 1);

        t.spinner.diameter = 16.0;
        t.spinner.min_diameter = f32::NAN;
        let mut s = start();
        let mut notes = Vec::new();
        spinner_size(&mut s, &t, &mut notes);
        assert_eq!(s.spacing.interact_size.y, 16.0, "no floor");
        assert_eq!(sanitised(&notes, "spinner.min_diameter"), 1);
    }

    // ---- D4, §6.8 -------------------------------------------------------------------------

    #[test]
    fn d4_the_scrollbar_radii_are_the_defaults_radius_and_the_rail_is_capped_at_half_its_height() {
        let mut t = theme();
        t.defaults.border.corner_radius = 3.0;
        let mut s = start();
        scrollbar_radii(&mut s, &t);
        let w = &s.visuals.widgets;
        assert_eq!(w.inactive.corner_radius, egui::CornerRadius::same(3));
        assert_eq!(w.hovered.corner_radius, egui::CornerRadius::same(3));
        assert_eq!(w.active.corner_radius, egui::CornerRadius::same(3));
        assert_eq!(
            w.noninteractive.corner_radius,
            start().visuals.widgets.noninteractive.corner_radius
        );

        let mut s = start();
        s.spacing.slider_rail_height = 8.0;
        t.defaults.border.corner_radius = 10.0;
        slider_rail_radius(&mut s, &t);
        assert_eq!(
            s.visuals.widgets.inactive.corner_radius,
            egui::CornerRadius::same(4)
        );
        t.defaults.border.corner_radius = 3.0;
        slider_rail_radius(&mut s, &t);
        assert_eq!(
            s.visuals.widgets.inactive.corner_radius,
            egui::CornerRadius::same(3)
        );
        t.defaults.border.corner_radius = -2.0;
        slider_rail_radius(&mut s, &t);
        assert_eq!(
            s.visuals.widgets.inactive.corner_radius,
            egui::CornerRadius::same(0)
        );

        // A non-finite radius keeps each sink's own value; the base style reports the leaf.
        t.defaults.border.corner_radius = f32::NAN;
        let mut s = start();
        scrollbar_radii(&mut s, &t);
        slider_rail_radius(&mut s, &t);
        assert_eq!(s.visuals.widgets, start().visuals.widgets);
    }

    // ---- D5, D6, D7, §6.9 ------------------------------------------------------------------

    #[test]
    fn d5_d6_the_pressed_thumb_and_the_hovered_handle_fall_back_by_copy() {
        let mut t = theme();
        t.scrollbar.thumb_active_color = None;
        let mut s = start();
        scrollbar_pressed_thumb(&mut s, &t, &mut Vec::new());
        assert_eq!(
            s.visuals.widgets.active.bg_fill,
            to_color32(t.scrollbar.thumb_hover_color)
        );
        let c = Rgba {
            r: 9,
            g: 8,
            b: 7,
            a: 255,
        };
        t.scrollbar.thumb_active_color = Some(c);
        scrollbar_pressed_thumb(&mut s, &t, &mut Vec::new());
        assert_eq!(s.visuals.widgets.active.bg_fill, to_color32(c));

        t.slider.thumb_hover_color = None;
        let mut s = start();
        slider_thumb_colours(&mut s, &t, &mut Vec::new());
        assert_eq!(
            s.visuals.widgets.hovered.bg_fill,
            to_color32(t.slider.thumb_color)
        );
        assert_eq!(
            s.visuals.widgets.active.bg_fill,
            to_color32(t.slider.thumb_color)
        );
        t.slider.thumb_hover_color = Some(c);
        slider_thumb_colours(&mut s, &t, &mut Vec::new());
        assert_eq!(s.visuals.widgets.hovered.bg_fill, to_color32(c));
        assert_eq!(s.visuals.widgets.active.bg_fill, to_color32(c));

        // §6.4: a colour with alpha 0 reaching a `bg_fill` is written and reported once.
        t.slider.thumb_hover_color = Some(Rgba {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        });
        let mut notes = Vec::new();
        slider_thumb_colours(&mut s, &t, &mut notes);
        assert_eq!(s.visuals.widgets.hovered.bg_fill.a(), 0);
        assert_eq!(notes.iter().filter(|n| matches!(n, Note::TransparentFill { path, .. } if *path == "slider.thumb_hover_color")).count(), 1);
    }

    #[test]
    fn d7_the_splitter_writes_three_strokes_of_one_width() {
        let mut t = theme();
        t.splitter.divider_width = 6.0;
        let mut s = start();
        let mut notes = Vec::new();
        splitter_strokes(&mut s, &t, &mut notes);
        let w = &s.visuals.widgets;
        assert_eq!(
            w.noninteractive.bg_stroke,
            egui::Stroke::new(6.0, to_color32(t.splitter.divider_color))
        );
        assert_eq!(
            w.hovered.fg_stroke,
            egui::Stroke::new(6.0, to_color32(t.splitter.hover_color))
        );
        assert_eq!(w.active.fg_stroke, w.hovered.fg_stroke);
        assert!(notes.is_empty());

        t.splitter.divider_width = 0.0;
        let mut s = start();
        splitter_strokes(&mut s, &t, &mut Vec::new());
        assert_eq!(
            s.visuals.widgets.noninteractive.bg_stroke.width, 0.0,
            "an invisible divider"
        );

        // A non-finite width keeps each stroke's own width, the colours are still written.
        t.splitter.divider_width = f32::NEG_INFINITY;
        let mut s = start();
        let own = start().visuals.widgets;
        let mut notes = Vec::new();
        splitter_strokes(&mut s, &t, &mut notes);
        let w = &s.visuals.widgets;
        assert_eq!(
            w.noninteractive.bg_stroke.width,
            own.noninteractive.bg_stroke.width
        );
        assert_eq!(
            w.noninteractive.bg_stroke.color,
            to_color32(t.splitter.divider_color)
        );
        assert_eq!(w.hovered.fg_stroke.width, own.hovered.fg_stroke.width);
        assert_eq!(
            w.hovered.fg_stroke.color,
            to_color32(t.splitter.hover_color)
        );
        assert_eq!(sanitised(&notes, "splitter.divider_width"), 1);
    }

    // ---- R-BAR, §6.10 ----------------------------------------------------------------------

    #[test]
    fn r_bar_subtracts_the_frame_and_the_separator_room_from_the_stated_height() {
        let mut t = theme();
        let mut s = start();
        s.visuals.widgets.noninteractive.bg_stroke.width = 1.0; // the Toolbar cell's `toolbar.border.line_width`
        let own = s.spacing.interact_size.y;

        t.toolbar.bar_height = None;
        toolbar_bar_height(&mut s, &t, &mut Vec::new());
        assert_eq!(
            s.spacing.interact_size.y, own,
            "no stated height: egui's own sizing stands"
        );

        t.toolbar.bar_height = Some(40.0);
        toolbar_bar_height(&mut s, &t, &mut Vec::new());
        let frame = egui::Frame::side_top_panel(&s);
        let inner = to_margin(frame.inner_margin, &t.toolbar.border.padding);
        let total_v = f32::from(inner.top)
            + f32::from(inner.bottom)
            + 2.0 * frame.stroke.width
            + f32::from(frame.outer_margin.top)
            + f32::from(frame.outer_margin.bottom)
            + 1.0;
        assert_eq!(s.spacing.interact_size.y, clamp_length(40.0 - total_v));

        t.toolbar.bar_height = Some(0.0);
        toolbar_bar_height(&mut s, &t, &mut Vec::new());
        assert_eq!(
            s.spacing.interact_size.y, 0.0,
            "a stated zero, and a height below the frame, floor at 0"
        );

        t.toolbar.bar_height = Some(-4.0);
        toolbar_bar_height(&mut s, &t, &mut Vec::new());
        assert_eq!(s.spacing.interact_size.y, 0.0);

        let mut s = start();
        t.toolbar.bar_height = Some(f32::NAN);
        let mut notes = Vec::new();
        toolbar_bar_height(&mut s, &t, &mut notes);
        assert_eq!(s.spacing.interact_size.y, own);
        assert_eq!(sanitised(&notes, "toolbar.bar_height"), 1);
    }

    // ---- R-ARROW and the checkbox inset, §6.11 ---------------------------------------------

    #[test]
    fn r_arrow_inverts_egui_s_triangle_scaling() {
        let mut t = theme();
        t.expander.arrow_icon_size = 12.0;
        let mut s = start();
        expander_arrow_size(&mut s, &t, &mut Vec::new());
        assert_eq!(s.spacing.icon_width_inner, clamp_length(12.0 * (4.0 / 3.0)));

        t.expander.arrow_icon_size = -3.0;
        expander_arrow_size(&mut s, &t, &mut Vec::new());
        assert_eq!(s.spacing.icon_width_inner, 0.0);

        t.expander.arrow_icon_size = f32::MAX;
        expander_arrow_size(&mut s, &t, &mut Vec::new());
        assert!(
            s.spacing.icon_width_inner.is_finite(),
            "the product's overflow is capped"
        );

        let mut s = start();
        t.expander.arrow_icon_size = f32::NAN;
        let mut notes = Vec::new();
        expander_arrow_size(&mut s, &t, &mut notes);
        assert_eq!(s.spacing.icon_width_inner, start().spacing.icon_width_inner);
        assert_eq!(sanitised(&notes, "expander.arrow_icon_size"), 1);
    }

    #[test]
    fn the_checkbox_inset_is_the_box_less_the_mean_of_both_pairs() {
        let mut t = theme();
        let set = |t: &mut ResolvedTheme,
                   l: Option<f32>,
                   r: Option<f32>,
                   top: Option<f32>,
                   b: Option<f32>| {
            t.checkbox.border.padding.left = l;
            t.checkbox.border.padding.right = r;
            t.checkbox.border.padding.top = top;
            t.checkbox.border.padding.bottom = b;
        };
        let mut s = start();
        s.spacing.icon_width = 20.0; // the cell's `checkbox.indicator_width` after its rule (§5.3)

        set(&mut t, Some(3.0), Some(3.0), Some(3.0), Some(3.0));
        checkbox_mark_inset(&mut s, &t, &mut Vec::new());
        assert_eq!(
            s.spacing.icon_width_inner, 14.0,
            "adwaita's 3/3/3/3 in a 20 box"
        );

        set(&mut t, Some(3.0), Some(3.0), Some(5.0), Some(5.0));
        checkbox_mark_inset(&mut s, &t, &mut Vec::new());
        assert_eq!(s.spacing.icon_width_inner, 12.0, "the mean of both pairs");

        set(&mut t, Some(30.0), Some(30.0), Some(30.0), Some(30.0));
        checkbox_mark_inset(&mut s, &t, &mut Vec::new());
        assert_eq!(
            s.spacing.icon_width_inner, 0.0,
            "an inset larger than the box is floored"
        );

        let mut s = start();
        set(&mut t, Some(3.0), None, Some(3.0), Some(3.0));
        let mut notes = Vec::new();
        checkbox_mark_inset(&mut s, &t, &mut notes);
        assert_eq!(
            s.spacing.icon_width_inner,
            start().spacing.icon_width_inner,
            "a side unstated: egui's own"
        );
        assert!(notes.is_empty());

        set(&mut t, Some(3.0), Some(f32::INFINITY), Some(3.0), Some(3.0));
        checkbox_mark_inset(&mut s, &t, &mut notes);
        assert_eq!(s.spacing.icon_width_inner, start().spacing.icon_width_inner);
        assert_eq!(sanitised(&notes, "checkbox.border.padding.right"), 1);
        assert_eq!(sanitised(&notes, "checkbox.border.padding.left"), 0);
    }

    // ---- the combo-box arrow area, §6.12 -----------------------------------------------------

    #[test]
    fn the_arrow_area_reserves_the_stated_width_beside_the_icon() {
        let mut t = theme();
        let mut s = start();
        s.spacing.icon_width = 12.0; // the cell's `combo_box.arrow_icon_size` after its rule (§5.4)
        let own = s.spacing.icon_spacing;

        t.combo_box.arrow_area_width = None;
        combo_box_arrow_area(&mut s, &t, &mut Vec::new());
        assert_eq!(s.spacing.icon_spacing, own);

        t.combo_box.arrow_area_width = Some(38.0);
        combo_box_arrow_area(&mut s, &t, &mut Vec::new());
        assert_eq!(s.spacing.icon_spacing, 26.0);
        assert_eq!(s.spacing.icon_spacing + s.spacing.icon_width, 38.0);

        t.combo_box.arrow_area_width = Some(5.0);
        combo_box_arrow_area(&mut s, &t, &mut Vec::new());
        assert_eq!(s.spacing.icon_spacing, 0.0, "never a negative gap");

        let mut s = start();
        t.combo_box.arrow_area_width = Some(f32::NAN);
        let mut notes = Vec::new();
        combo_box_arrow_area(&mut s, &t, &mut notes);
        assert_eq!(s.spacing.icon_spacing, own);
        assert_eq!(sanitised(&notes, "combo_box.arrow_area_width"), 1);
    }

    // ---- §6.4, over the cells Task 15 builds ----------------------------------------------

    /// Every `None` soft option is a copy of the leaf its row names in §6.4's table, written as
    /// given; only `button.active_background` falls back to another *layer*, composited.
    #[test]
    fn a_none_soft_option_copies_its_fallback_leaf() {
        let mut t = theme();
        let opacity = t.defaults.border.opacity;
        t.button.active_background = None;
        t.tab.hover_background = None;
        t.expander.hover_background = None;
        t.checkbox.hover_background = None;
        t.checkbox.unchecked_background = None;
        t.checkbox.unchecked_border_color = None;
        t.combo_box.hover_background = None;
        t.segmented_control.hover_background = None;
        t.switch.hover_unchecked_background = None;
        t.input.hover_border_color = None;
        let (s, _) = build(&t);
        let cell = |r: Role| s.cell(r, RoleVariant::Normal);

        let expected_active = composite_over(t.button.hover_background, t.button.background_color);
        assert_eq!(s.base.visuals.widgets.active.weak_bg_fill, expected_active);
        assert_eq!(
            cell(Role::Button).visuals.widgets.active.weak_bg_fill,
            expected_active
        );
        assert_eq!(
            cell(Role::Tab).visuals.widgets.hovered.weak_bg_fill,
            to_color32(t.tab.background_color)
        );
        assert_eq!(
            cell(Role::Expander).visuals.widgets.hovered.weak_bg_fill,
            egui::Color32::TRANSPARENT
        );
        assert_eq!(
            cell(Role::Expander).visuals.widgets.inactive.weak_bg_fill,
            egui::Color32::TRANSPARENT
        );
        let cb = cell(Role::Checkbox);
        assert_eq!(
            cb.visuals.widgets.inactive.bg_fill,
            to_color32(t.checkbox.background_color)
        );
        assert_eq!(
            cb.visuals.widgets.hovered.bg_fill,
            to_color32(t.checkbox.background_color)
        );
        assert_eq!(
            cb.visuals.widgets.inactive.bg_stroke.color,
            to_color32_with_opacity(t.checkbox.border.color, opacity)
        );
        assert_eq!(
            cell(Role::ComboBox).visuals.widgets.hovered.weak_bg_fill,
            to_color32(t.combo_box.background_color)
        );
        assert_eq!(
            cell(Role::SegmentedControl)
                .visuals
                .widgets
                .hovered
                .weak_bg_fill,
            to_color32(t.segmented_control.background_color)
        );
        assert_eq!(
            cell(Role::Switch).visuals.widgets.hovered.weak_bg_fill,
            to_color32(t.switch.unchecked_background)
        );
        assert_eq!(
            cell(Role::Input).visuals.widgets.hovered.bg_stroke.color,
            to_color32_with_opacity(t.input.border.color, opacity)
        );

        // A stated hover fill is a layer over the idle fill (§6.1); the fallback was not.
        let layer = Rgba {
            r: 0,
            g: 0,
            b: 0,
            a: 128,
        };
        t.combo_box.hover_background = Some(layer);
        let (s, _) = build(&t);
        assert_eq!(
            s.cell(Role::ComboBox, RoleVariant::Normal)
                .visuals
                .widgets
                .hovered
                .weak_bg_fill,
            composite_over(layer, t.combo_box.background_color)
        );
    }

    // ---- §6.13 and §6.14, over the base style Task 13 builds ------------------------------

    #[test]
    fn every_border_stroke_folds_the_defaults_opacity_into_its_alpha() {
        let mut t = theme();
        t.defaults.border.opacity = 0.5;
        let (s, notes) = build(&t);
        let w = &s.base.visuals.widgets;
        assert_eq!(
            w.noninteractive.bg_stroke.color,
            to_color32_with_opacity(t.defaults.border.color, 0.5)
        );
        assert_eq!(
            w.inactive.bg_stroke.color,
            to_color32_with_opacity(t.button.border.color, 0.5)
        );
        assert_eq!(
            s.base.visuals.window_stroke.color,
            to_color32_with_opacity(t.defaults.border.color, 0.5)
        );
        for e in entries(&s.cell(Role::Button, RoleVariant::Normal).visuals.widgets) {
            assert_eq!(
                e.bg_stroke.color,
                to_color32_with_opacity(t.button.border.color, 0.5)
            );
        }
        assert_eq!(sanitised(&notes, "defaults.border.opacity"), 0);

        // A non-finite opacity is `1.0` — egui folds none — reported once, by the base style.
        t.defaults.border.opacity = f32::NAN;
        let (s, notes) = build(&t);
        assert_eq!(
            s.base.visuals.widgets.noninteractive.bg_stroke.color,
            to_color32(t.defaults.border.color)
        );
        assert_eq!(sanitised(&notes, "defaults.border.opacity"), 1);
    }

    #[test]
    fn the_shadow_gate_replaces_only_the_colour() {
        let mut t = theme();
        let own = start().visuals;
        t.window.border.shadow_enabled = true;
        t.defaults.border.shadow_enabled = false;
        let (s, _) = build(&t);
        assert_eq!(
            s.base.visuals.window_shadow,
            to_shadow(own.window_shadow, t.defaults.shadow_color, true)
        );
        assert_eq!(s.base.visuals.window_shadow.blur, own.window_shadow.blur);
        assert_eq!(
            s.base.visuals.window_shadow.offset,
            own.window_shadow.offset
        );
        assert_eq!(s.base.visuals.popup_shadow, egui::Shadow::NONE);

        t.window.border.shadow_enabled = false;
        t.defaults.border.shadow_enabled = true;
        let (s, _) = build(&t);
        assert_eq!(s.base.visuals.window_shadow, egui::Shadow::NONE);
        assert_eq!(
            s.base.visuals.popup_shadow,
            to_shadow(own.popup_shadow, t.defaults.shadow_color, true)
        );
    }
}
