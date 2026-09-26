//! §6.1: the five `WidgetVisuals` entries from one widget's leaves.

use egui::style::{WidgetVisuals, Widgets};

use native_theme::theme::ResolvedWidgetBorder;

use super::base::{radius, stroke};
use super::note_transparent_fill;
use crate::Note;
use crate::convert::{Rgba, composite_over, to_color32};

/// Which field a widget paints its fill from (§5.3 B1): `weak_bg_fill` for a `Button` and
/// everything built from one, `bg_fill` for a checkbox box, a slider rail, a scrollbar thumb.
#[derive(Clone, Copy, Debug)]
pub(crate) enum FillField {
    Weak,
    Bg,
}

/// How a stated hover or pressed colour reaches its entry (§6.1): a state layer composited
/// over the idle fill (C17), or a row highlight and a thumb colour written as given.
#[derive(Clone, Copy, Debug)]
pub(crate) enum Layer {
    Composite,
    AsGiven,
}

/// What `open` copies: `inactive` for every role but `Menu`, whose open item stays
/// highlighted as on hover (`egui/src/containers/menu.rs:382-384`).
#[derive(Clone, Copy, Debug)]
pub(crate) enum OpenFrom {
    Inactive,
    Hovered,
}

/// The entries a role's border reaches (§6.1, *Which entries a role writes*).
#[derive(Clone, Copy, Debug)]
pub(crate) enum BorderEntries {
    /// `inactive`, `hovered`, `active`, `open`: the base style, `Role::Menu`'s items.
    Interactive,
    /// All five: the `{5}` rows.
    All,
}

/// A fill and the model's hover and pressed leaves for it. Each colour carries its leaf
/// path for the notes. `idle: None` leaves the resting entry as it stands — `menu_style`'s
/// transparent item — and copies from it.
pub(crate) struct FillSource {
    pub field: FillField,
    pub idle: Option<(Rgba, &'static str)>,
    /// `None`: the model has no hover leaf for this field, so `hovered` copies `inactive`.
    pub hover: Option<(Rgba, &'static str)>,
    /// `None`: no pressed leaf, so `active` copies `hovered`.
    pub active: Option<(Rgba, &'static str)>,
    pub layer: Layer,
}

/// A text colour: into `inactive` (and `noninteractive` where the role's labels take it),
/// the hover and pressed colours or else the copies.
pub(crate) struct TextSource {
    pub idle: Rgba,
    pub hover: Option<Rgba>,
    pub active: Option<Rgba>,
    pub in_noninteractive: bool,
}

/// A border: colour × `defaults.border.opacity` (§6.13), width and radius, into `entries`.
/// `paths` are the colour, corner-radius and line-width leaves.
pub(crate) struct BorderSource<'a> {
    pub border: &'a ResolvedWidgetBorder,
    pub opacity: f32,
    pub paths: [&'static str; 3],
    pub entries: BorderEntries,
}

pub(crate) struct StateSource<'a> {
    pub fill: Option<FillSource>,
    pub text: Option<TextSource>,
    pub border: Option<BorderSource<'a>>,
    pub open: OpenFrom,
}

fn fill_of(entry: &WidgetVisuals, field: FillField) -> egui::Color32 {
    match field {
        FillField::Weak => entry.weak_bg_fill,
        FillField::Bg => entry.bg_fill,
    }
}

fn set_fill(entry: &mut WidgetVisuals, field: FillField, color: egui::Color32) {
    match field {
        FillField::Weak => entry.weak_bg_fill = color,
        FillField::Bg => entry.bg_fill = color,
    }
}

/// §6.4: only a `bg_fill` carries egui's "never transparent" invariant; `weak_bg_fill` may be
/// transparent (`egui/src/style.rs:1292-1300`).
fn note_transparent(
    field: FillField,
    written: egui::Color32,
    path: &'static str,
    notes: &mut Vec<Note>,
) {
    if matches!(field, FillField::Bg) {
        note_transparent_fill(written, path, notes);
    }
}

/// Apply §6.1's policy for one widget to `widgets`: each `Some` of `src` writes the entries
/// its kind names and nothing else, so a field the role states no value for keeps what the
/// entry holds. `expansion` is never written.
pub(crate) fn write_states(widgets: &mut Widgets, src: &StateSource<'_>, notes: &mut Vec<Note>) {
    if let Some(f) = &src.fill {
        let idle = match f.idle {
            Some((c, path)) => {
                let written = to_color32(c);
                note_transparent(f.field, written, path, notes);
                set_fill(&mut widgets.inactive, f.field, written);
                written
            }
            None => fill_of(&widgets.inactive, f.field),
        };
        let over = |layer: Rgba| match (f.layer, f.idle) {
            (Layer::Composite, Some((idle_rgba, _))) => composite_over(layer, idle_rgba),
            _ => to_color32(layer),
        };
        let hovered = match f.hover {
            Some((c, path)) => {
                let written = over(c);
                note_transparent(f.field, written, path, notes);
                written
            }
            None => idle,
        };
        let active = match f.active {
            Some((c, path)) => {
                let written = over(c);
                note_transparent(f.field, written, path, notes);
                written
            }
            None => hovered,
        };
        set_fill(&mut widgets.hovered, f.field, hovered);
        set_fill(&mut widgets.active, f.field, active);
        let open = match src.open {
            OpenFrom::Inactive => idle,
            OpenFrom::Hovered => hovered,
        };
        set_fill(&mut widgets.open, f.field, open);
    }

    if let Some(t) = &src.text {
        let idle = to_color32(t.idle);
        let hovered = t.hover.map_or(idle, to_color32);
        let active = t.active.map_or(hovered, to_color32);
        if t.in_noninteractive {
            widgets.noninteractive.fg_stroke.color = idle;
        }
        widgets.inactive.fg_stroke.color = idle;
        widgets.hovered.fg_stroke.color = hovered;
        widgets.active.fg_stroke.color = active;
        widgets.open.fg_stroke.color = match src.open {
            OpenFrom::Inactive => idle,
            OpenFrom::Hovered => hovered,
        };
    }

    if let Some(b) = &src.border {
        let [color_path, radius_path, width_path] = b.paths;
        let entries: Vec<&mut WidgetVisuals> = match b.entries {
            BorderEntries::Interactive => vec![
                &mut widgets.inactive,
                &mut widgets.hovered,
                &mut widgets.active,
                &mut widgets.open,
            ],
            BorderEntries::All => vec![
                &mut widgets.noninteractive,
                &mut widgets.inactive,
                &mut widgets.hovered,
                &mut widgets.active,
                &mut widgets.open,
            ],
        };
        for entry in entries {
            // one stroke and one radius in every entry the role writes, each over that entry's own
            entry.bg_stroke = stroke(
                [color_path, width_path],
                entry.bg_stroke,
                b.border.color,
                b.border.line_width,
                b.opacity,
                notes,
            );
            entry.corner_radius = radius(
                radius_path,
                entry.corner_radius,
                b.border.corner_radius,
                notes,
            );
        }
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
    use native_theme::theme::ColorMode;

    use super::*;
    use crate::convert::{
        Rgba, clamp_length, composite_over, to_color32, to_color32_with_opacity, to_corner_radius,
    };
    use crate::install_tests::resolved;
    use crate::style::BuildInput;
    use crate::style::base::base_style;
    use crate::{AccessibilityPreferences, LayoutTheme, Note, ResolvedTheme};

    fn base(scheme: egui::Theme, t: &ResolvedTheme) -> (egui::Style, Vec<Note>) {
        let prefs = AccessibilityPreferences::default();
        let layout = LayoutTheme::default();
        let input = BuildInput {
            scheme,
            theme: t,
            prefs: &prefs,
            layout: &layout,
            row_height: None,
        };
        let mut notes = Vec::new();
        (base_style(&input, &mut notes), notes)
    }

    /// C17: windows-11 states a translucent hover layer (`#0000000a`,
    /// `native-theme/src/presets/windows-11.toml:85`), which is composited over the idle fill,
    /// not written as it stands.
    #[test]
    fn a_translucent_hover_layer_is_composited_over_the_idle_fill() {
        let t = resolved("windows-11", ColorMode::Light);
        assert!(
            t.button.hover_background.a < 255,
            "the fixture must be translucent"
        );
        let (s, _) = base(egui::Theme::Light, &t);
        let w = &s.visuals.widgets;
        assert_eq!(
            w.hovered.weak_bg_fill,
            composite_over(t.button.hover_background, t.button.background_color)
        );
        assert_ne!(
            w.hovered.weak_bg_fill,
            to_color32(t.button.hover_background)
        );
        let pressed = t
            .button
            .active_background
            .unwrap_or(t.button.hover_background);
        assert_eq!(
            w.active.weak_bg_fill,
            composite_over(pressed, t.button.background_color)
        );
    }

    /// An opaque layer composites to itself (`ecolor/src/color32.rs:343-345`), so on adwaita
    /// the hover fill is the leaf's colour exactly.
    #[test]
    fn an_opaque_layer_is_written_as_itself() {
        let t = resolved("adwaita", ColorMode::Light);
        assert_eq!(t.button.hover_background.a, 255);
        let (s, _) = base(egui::Theme::Light, &t);
        assert_eq!(
            s.visuals.widgets.hovered.weak_bg_fill,
            to_color32(t.button.hover_background)
        );
    }

    /// §6.4: a `None` pressed layer means no fill change on press — `active` copies `hovered`.
    #[test]
    fn a_missing_pressed_layer_copies_the_hovered_entry() {
        let mut t = resolved("adwaita", ColorMode::Light);
        t.button.active_background = None;
        let (s, _) = base(egui::Theme::Light, &t);
        let w = &s.visuals.widgets;
        assert_eq!(w.active.weak_bg_fill, w.hovered.weak_bg_fill);
    }

    /// §6.1: a field the widget states no hover or pressed value for is `inactive`'s in every
    /// entry — egui's stock radius 3 and strokes 1.5 / 2.0 (`egui/src/style.rs:1703-1704`,
    /// `:1711`) never appear.
    #[test]
    fn hover_and_active_keep_the_resting_stroke_and_radius() {
        for (preset, mode, scheme) in [
            ("adwaita", ColorMode::Light, egui::Theme::Light),
            ("kde-breeze", ColorMode::Dark, egui::Theme::Dark),
        ] {
            let t = resolved(preset, mode);
            let (s, _) = base(scheme, &t);
            let w = &s.visuals.widgets;
            for e in [&w.hovered, &w.active, &w.open] {
                assert_eq!(e.bg_stroke, w.inactive.bg_stroke, "{preset}");
                assert_eq!(e.corner_radius, w.inactive.corner_radius, "{preset}");
                assert_eq!(e.fg_stroke.width, w.inactive.fg_stroke.width, "{preset}");
            }
            assert_eq!(
                w.inactive.bg_stroke.color,
                to_color32_with_opacity(t.button.border.color, t.defaults.border.opacity),
                "{preset}"
            );
            assert_eq!(
                w.inactive.bg_stroke.width,
                clamp_length(t.button.border.line_width),
                "{preset}"
            );
            assert_eq!(
                w.inactive.corner_radius,
                to_corner_radius(
                    scheme
                        .default_style()
                        .visuals
                        .widgets
                        .inactive
                        .corner_radius,
                    t.button.border.corner_radius
                ),
                "{preset}"
            );
        }
    }

    /// §6.1: `open` is a field-wise copy of `inactive`, but its `weak_bg_fill` is the title
    /// bar's (`egui/src/containers/window.rs:1427`).
    #[test]
    fn open_copies_inactive_except_the_title_bar_fill() {
        let t = resolved("adwaita", ColorMode::Light);
        let (s, _) = base(egui::Theme::Light, &t);
        let w = &s.visuals.widgets;
        assert_eq!(
            w.open.weak_bg_fill,
            to_color32(t.window.title_bar_background)
        );
        assert_eq!(w.open.bg_fill, w.inactive.bg_fill);
        assert_eq!(w.open.fg_stroke, w.inactive.fg_stroke);
        assert_eq!(w.open.bg_stroke, w.inactive.bg_stroke);
        assert_eq!(w.open.corner_radius, w.inactive.corner_radius);
    }

    /// §5.9: `inactive`, `open` and `hovered` `bg_fill` are the scrollbar thumb's, written as
    /// given; `active`'s is D5 (§6.9), Task 17's, and copies `hovered` here (§6.1).
    #[test]
    fn the_thumb_colours_fill_bg_fill() {
        let t = resolved("adwaita", ColorMode::Light);
        let (s, _) = base(egui::Theme::Light, &t);
        let w = &s.visuals.widgets;
        assert_eq!(w.inactive.bg_fill, to_color32(t.scrollbar.thumb_color));
        assert_eq!(w.open.bg_fill, to_color32(t.scrollbar.thumb_color));
        assert_eq!(w.hovered.bg_fill, to_color32(t.scrollbar.thumb_hover_color));
    }

    /// §6.1's table: `noninteractive` takes `defaults.text_color`, `defaults.border` and the
    /// panel fill; the interactive entries take the button's text colours.
    #[test]
    fn noninteractive_takes_the_defaults_and_the_interactive_entries_the_button() {
        let t = resolved("kde-breeze", ColorMode::Dark);
        let d = &t.defaults;
        let own = egui::Theme::Dark.default_style();
        let (s, _) = base(egui::Theme::Dark, &t);
        let w = &s.visuals.widgets;
        assert_eq!(w.noninteractive.fg_stroke.color, to_color32(d.text_color));
        assert_eq!(
            w.noninteractive.bg_stroke.color,
            to_color32_with_opacity(d.border.color, d.border.opacity)
        );
        assert_eq!(
            w.noninteractive.bg_stroke.width,
            clamp_length(d.border.line_width)
        );
        assert_eq!(
            w.noninteractive.corner_radius,
            to_corner_radius(
                own.visuals.widgets.noninteractive.corner_radius,
                d.border.corner_radius
            )
        );
        assert_eq!(w.noninteractive.bg_fill, to_color32(d.background_color));
        assert_eq!(
            w.noninteractive.weak_bg_fill,
            to_color32(d.background_color)
        );
        assert_eq!(
            w.inactive.weak_bg_fill,
            to_color32(t.button.background_color)
        );
        assert_eq!(w.inactive.fg_stroke.color, to_color32(t.button.font.color));
        assert_eq!(
            w.hovered.fg_stroke.color,
            to_color32(t.button.hover_text_color)
        );
        assert_eq!(
            w.active.fg_stroke.color,
            to_color32(t.button.active_text_color)
        );
        assert_eq!(w.open.fg_stroke.color, to_color32(t.button.font.color));
        for e in [
            &w.noninteractive,
            &w.inactive,
            &w.hovered,
            &w.active,
            &w.open,
        ] {
            assert_eq!(e.expansion, 0.0, "expansion is untouched (§6.1)");
        }
    }

    /// §6.4: a `bg_fill` written with alpha 0 is written as given and reported.
    #[test]
    fn a_transparent_bg_fill_is_written_and_reported() {
        let mut t = resolved("adwaita", ColorMode::Light);
        t.scrollbar.thumb_color = Rgba {
            a: 0,
            ..t.scrollbar.thumb_color
        };
        let (s, notes) = base(egui::Theme::Light, &t);
        assert_eq!(
            s.visuals.widgets.inactive.bg_fill,
            egui::Color32::TRANSPARENT
        );
        assert_eq!(
            notes
                .iter()
                .filter(|n| **n
                    == Note::TransparentFill {
                        path: "scrollbar.thumb_color"
                    })
                .count(),
            1
        );
    }

    /// `Role::Menu`'s rule: `open` copies `hovered` (`egui/src/containers/menu.rs:382-384`).
    #[test]
    fn open_copies_hovered_when_asked() {
        let mut widgets = egui::style::Widgets::dark();
        let idle = Rgba {
            r: 10,
            g: 20,
            b: 30,
            a: 255,
        };
        let hover = Rgba {
            r: 40,
            g: 50,
            b: 60,
            a: 255,
        };
        let mut notes = Vec::new();
        write_states(
            &mut widgets,
            &StateSource {
                fill: Some(FillSource {
                    field: FillField::Weak,
                    idle: Some((idle, "test.idle")),
                    hover: Some((hover, "test.hover")),
                    active: None,
                    layer: Layer::AsGiven,
                }),
                text: None,
                border: None,
                open: OpenFrom::Hovered,
            },
            &mut notes,
        );
        assert_eq!(widgets.open.weak_bg_fill, to_color32(hover));
        assert_eq!(widgets.active.weak_bg_fill, to_color32(hover));
        assert_eq!(widgets.inactive.weak_bg_fill, to_color32(idle));
        assert!(notes.is_empty());
    }
}
