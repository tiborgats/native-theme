//! The base `Style` of one colour scheme (§3.4): §5.1's DIRECT rows, §5.9's base owners, the
//! five `text_styles` sizes (§8.5), `layout.widget_gap` (§6.16) and the line spacing (§6.15).
//! The five `WidgetVisuals` entries are `states::write_states`'s.

use egui::TextStyle;

use native_theme::theme::{ResolvedPadding, ResolvedWidgetBorder};

use super::{BuildInput, push_note, saturates_i8};
use crate::accessors::scaled_text_size;
use crate::convert::{
    Rgba, clamp_length, finite_or, to_button_padding, to_color32, to_color32_with_opacity,
    to_corner_radius, to_margin, unit_interval,
};
use crate::{AccessibilityPreferences, Note};

/// §8.5's rule: the scaled size when it is a positive normal `f32`, else `own` — egui's own
/// size for that slot — with a `Note::ValueSanitised` for `path`.
pub(crate) fn text_size(
    path: &'static str,
    size: f32,
    prefs: &AccessibilityPreferences,
    own: f32,
    notes: &mut Vec<Note>,
) -> f32 {
    let s = scaled_text_size(size, prefs);
    if s.is_normal() && s > 0.0 {
        s
    } else {
        push_note(notes, Note::ValueSanitised { path });
        own
    }
}

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

/// A border stroke: the colour with `defaults.border.opacity` folded into its alpha (§6.13),
/// the width over the sink's own (`to_stroke`'s rule, §7.2). `paths` are the colour's and the
/// width's leaves; the opacity's own note is emitted once per style by `base_style`.
pub(crate) fn stroke(
    paths: [&'static str; 2],
    own: egui::Stroke,
    color: Rgba,
    width: f32,
    opacity: f32,
    notes: &mut Vec<Note>,
) -> egui::Stroke {
    let [_color_path, width_path] = paths;
    if !width.is_finite() {
        push_note(notes, Note::ValueSanitised { path: width_path });
    }
    egui::Stroke::new(
        clamp_length(finite_or(width, own.width)),
        to_color32_with_opacity(color, opacity),
    )
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
/// side leaves in `to_margin`'s order: top, right, bottom, left.
pub(crate) fn margin(
    paths: [&'static str; 4],
    own: egui::Margin,
    padding: &ResolvedPadding,
    notes: &mut Vec<Note>,
) -> egui::Margin {
    let sides = [padding.top, padding.right, padding.bottom, padding.left];
    for (side, path) in sides.into_iter().zip(paths) {
        if let Some(v) = side {
            if !v.is_finite() {
                push_note(notes, Note::ValueSanitised { path });
            } else if saturates_i8(v) {
                push_note(notes, Note::ValueSaturated { path });
            }
        }
    }
    to_margin(own, padding)
}

/// `Spacing::button_padding` from a widget's border (§5's pair rule, `to_button_padding`);
/// a non-finite side is reported. The border's `line_width` is reported where its stroke is
/// written, not here (once per leaf).
pub(crate) fn button_padding(
    paths: [&'static str; 4],
    border: &ResolvedWidgetBorder,
    own: egui::Vec2,
    notes: &mut Vec<Note>,
) -> egui::Vec2 {
    let p = &border.padding;
    for (side, path) in [p.top, p.right, p.bottom, p.left].into_iter().zip(paths) {
        if side.is_some_and(|v| !v.is_finite()) {
            push_note(notes, Note::ValueSanitised { path });
        }
    }
    to_button_padding(own, border)
}

/// §6.15: the theme's line box less epaint's row, floored at egui's `0.0`
/// (`egui/src/style.rs:1465`); `0.0` too with no row or a non-finite multiplier.
pub(crate) fn line_spacing(line_height: f32, size: f32, row_height: Option<f32>) -> f32 {
    match row_height {
        Some(row) if line_height.is_finite() => clamp_length(line_height * size - row),
        _ => 0.0,
    }
}

/// The base style of `input.scheme` (§3.4): egui's own style for the scheme, assigned.
pub(crate) fn base_style(input: &BuildInput<'_>, notes: &mut Vec<Note>) -> egui::Style {
    let t = input.theme;
    let d = &t.defaults;
    let prefs = input.prefs;
    let own = input.scheme.default_style();
    let mut s = input.scheme.default_style();

    // §6.13: the one border opacity, reported once per style
    let opacity_fold = if d.border.opacity.is_finite() {
        d.border.opacity
    } else {
        push_note(
            notes,
            Note::ValueSanitised {
                path: "defaults.border.opacity",
            },
        );
        1.0
    };

    // §8.5: the five slots, sizes only — the families stay Proportional / Monospace (§8.1)
    let slots = [
        (
            TextStyle::Small,
            "text_scale.caption.size",
            t.text_scale.caption.size,
        ),
        (TextStyle::Body, "defaults.font.size", d.font.size),
        (
            TextStyle::Monospace,
            "defaults.mono_font.size",
            d.mono_font.size,
        ),
        (TextStyle::Button, "button.font.size", t.button.font.size),
        (
            TextStyle::Heading,
            "text_scale.section_heading.size",
            t.text_scale.section_heading.size,
        ),
    ];
    for (slot, path, size) in slots {
        if let (Some(font), Some(own_font)) =
            (s.text_styles.get_mut(&slot), own.text_styles.get(&slot))
        {
            font.size = text_size(path, size, prefs, own_font.size, notes);
        }
    }

    // §5.1 foundation and §5.9 base owners: visuals
    let v = &mut s.visuals;
    v.panel_fill = to_color32(d.background_color);
    v.weak_text_color = Some(to_color32(d.muted_color));
    v.window_shadow.color = to_color32(d.shadow_color); // geometry stays egui's (§6.14 gates it, Task 17)
    v.popup_shadow.color = to_color32(d.shadow_color);
    v.hyperlink_color = to_color32(d.link_color);
    v.selection.bg_fill = to_color32(d.selection_background);
    v.selection.stroke.color = to_color32(d.selection_text_color); // width stays egui's (§5.1)
    v.error_fg_color = to_color32(d.danger_color);
    v.warn_fg_color = to_color32(d.warning_color);
    v.menu_corner_radius = radius(
        "defaults.border.corner_radius_lg",
        own.visuals.menu_corner_radius,
        d.border.corner_radius_lg,
        notes,
    );
    v.window_stroke = stroke(
        ["defaults.border.color", "defaults.border.line_width"],
        own.visuals.window_stroke,
        d.border.color,
        d.border.line_width,
        opacity_fold,
        notes,
    );
    v.disabled_alpha = opacity(
        "defaults.disabled_opacity",
        d.disabled_opacity,
        own.visuals.disabled_alpha,
        notes,
    );
    v.window_fill = to_color32(t.menu.background_color); // base owner of window_fill (§5.9)
    v.extreme_bg_color = to_color32(t.scrollbar.track_color);
    v.faint_bg_color = to_color32(t.list.alternate_row_background);
    v.text_edit_bg_color = Some(to_color32(t.input.background_color));
    v.text_cursor.stroke.color = to_color32(t.input.caret_color); // width stays egui's (§5.4)
    v.slider_trailing_fill = true; // `slider.fill_color` is invisible without it (§5.5)
    v.handle_shape = egui::style::HandleShape::Circle; // `slider.thumb_diameter` is a diameter (§5.9)
    v.window_corner_radius = radius(
        "window.border.corner_radius",
        own.visuals.window_corner_radius,
        t.window.border.corner_radius,
        notes,
    );

    // spacing
    let sp = &mut s.spacing;
    sp.interact_size.y = length(
        "button.min_height",
        t.button.min_height,
        own.spacing.interact_size.y,
        notes,
    ); // .x stays inherited (§7.5)
    sp.button_padding = button_padding(
        [
            "button.border.padding.top",
            "button.border.padding.right",
            "button.border.padding.bottom",
            "button.border.padding.left",
        ],
        &t.button.border,
        own.spacing.button_padding,
        notes,
    );
    sp.icon_width = length(
        "checkbox.indicator_width",
        t.checkbox.indicator_width,
        own.spacing.icon_width,
        notes,
    );
    sp.icon_spacing = length(
        "checkbox.label_gap",
        t.checkbox.label_gap,
        own.spacing.icon_spacing,
        notes,
    );
    sp.slider_rail_height = length(
        "slider.track_height",
        t.slider.track_height,
        own.spacing.slider_rail_height,
        notes,
    );
    sp.combo_width = length(
        "combo_box.min_width",
        t.combo_box.min_width,
        own.spacing.combo_width,
        notes,
    );
    sp.tooltip_width = length(
        "tooltip.max_width",
        t.tooltip.max_width,
        own.spacing.tooltip_width,
        notes,
    );
    sp.scroll.handle_min_length = length(
        "scrollbar.min_thumb_length",
        t.scrollbar.min_thumb_length,
        own.spacing.scroll.handle_min_length,
        notes,
    );
    sp.scroll.foreground_color = false; // the handle reads `bg_fill` (§5.5)
    sp.window_margin = margin(
        [
            "window.border.padding.top",
            "window.border.padding.right",
            "window.border.padding.bottom",
            "window.border.padding.left",
        ],
        own.spacing.window_margin,
        &t.window.border.padding,
        notes,
    );
    if let Some(gap) = input.layout.widget_gap {
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

    // §6.15: the line spacing, from the Body size the atlas writes and the row `Builder::build` measured
    if !d.line_height.is_finite() {
        push_note(
            notes,
            Note::ValueSanitised {
                path: "defaults.line_height",
            },
        );
    }
    if let Some(body) = s.text_styles.get(&TextStyle::Body) {
        s.spacing.extra_text_line_spacing =
            line_spacing(d.line_height, body.size, input.row_height);
    }
    s
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
        clamp_length, to_button_padding, to_color32, to_color32_with_opacity, to_corner_radius,
        to_margin, unit_interval,
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
            assert_eq!(
                v.window_stroke.color,
                to_color32_with_opacity(d.border.color, d.border.opacity)
            );
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
            assert_eq!(
                v.widgets, own.visuals.widgets,
                "the five entries are Task 14's"
            );
            assert_eq!(sp.menu_margin, own.spacing.menu_margin);
            assert_eq!(sp.combo_height, own.spacing.combo_height);
            assert_eq!(sp.text_edit_width, own.spacing.text_edit_width);
            assert_eq!(sp.slider_width, own.spacing.slider_width);
            assert_eq!(sp.default_area_size, own.spacing.default_area_size);
            assert_eq!(sp.indent, own.spacing.indent);
            assert_eq!(s.interaction, own.interaction);
            assert_eq!(s.animation_time, own.animation_time);
            assert_eq!(s.override_font_id, None);
            assert_eq!(v.override_text_color, None);
        }
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
