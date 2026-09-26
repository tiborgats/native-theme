//! The eleven `Surface` frames (spec §3.4, §4.4, §5.2, §5.6).
//!
//! Each frame starts from the frame egui builds for that container over the scheme's base
//! style and sets only the fields its surface's rows state; every other field keeps the
//! preset's value, and through it whatever the base style holds (§3.4). A stroke colour folds
//! `defaults.border.opacity` (§6.13); the shadow gate keeps egui's geometry (§6.14); a padding
//! side the theme leaves unstated keeps the preset's, and a stated one is rounded to a whole
//! point, saturating (`convert::to_margin`, §7.2).

use native_theme::color::Rgba;
use native_theme::theme::{ResolvedTheme, ResolvedWidgetBorder};

use super::base::{margin, radius, stroke};
use crate::convert::{i8_from_f32_saturating, to_color32, to_shadow};
use crate::style::{BuildInput, push_note, saturates_i8};
use crate::{Note, PanelSide, Surface};

/// The leaf paths of one widget's border, for Task 13's reporting helpers (§7.2's emission rule).
struct BorderPaths {
    color: &'static str,
    line_width: &'static str,
    corner_radius: &'static str,
    /// top, right, bottom, left — `to_margin`'s order
    padding: [&'static str; 4],
}

/// The paths of `<widget>.border.*`, spelled once.
macro_rules! border_paths {
    ($w:literal) => {
        BorderPaths {
            color: concat!($w, ".border.color"),
            line_width: concat!($w, ".border.line_width"),
            corner_radius: concat!($w, ".border.corner_radius"),
            padding: [
                concat!($w, ".border.padding.top"),
                concat!($w, ".border.padding.right"),
                concat!($w, ".border.padding.bottom"),
                concat!($w, ".border.padding.left"),
            ],
        }
    };
}

const WINDOW: BorderPaths = border_paths!("window");
const DIALOG: BorderPaths = border_paths!("dialog");
const POPOVER: BorderPaths = border_paths!("popover");
const TOOLTIP: BorderPaths = border_paths!("tooltip");
const CARD: BorderPaths = border_paths!("card");
const SIDEBAR: BorderPaths = border_paths!("sidebar");
const TOOLBAR: BorderPaths = border_paths!("toolbar");
const STATUS_BAR: BorderPaths = border_paths!("status_bar");

/// A widget border's stroke over the preset frame's, through Task 13's `stroke`: the width by §6's
/// rule (a non-finite one keeps the preset's, reported), the colour folded with
/// `defaults.border.opacity` (§6.13).
fn border_stroke(
    own: egui::Stroke,
    border: &ResolvedWidgetBorder,
    paths: &BorderPaths,
    t: &ResolvedTheme,
    notes: &mut Vec<Note>,
) -> egui::Stroke {
    stroke(
        [paths.color, paths.line_width],
        own,
        border.color,
        border.line_width,
        t.defaults.border.opacity,
        notes,
    )
}

/// A popup-preset surface — `Dialog`, `Popover`, `Tooltip` (`egui/src/containers/modal.rs:100`,
/// `egui/src/containers/popup.rs:603`): fill, stroke, radius, the shadow gate and the padding
/// over `Frame::popup` (`egui/src/containers/frame.rs:214-221`).
fn popup_like(
    base: &egui::Style,
    fill: Rgba,
    border: &ResolvedWidgetBorder,
    paths: &BorderPaths,
    input: &BuildInput<'_>,
    notes: &mut Vec<Note>,
) -> egui::Frame {
    let t = input.theme;
    let mut f = egui::Frame::popup(base);
    f.fill = to_color32(fill);
    f.stroke = border_stroke(f.stroke, border, paths, t, notes);
    f.corner_radius = radius(
        paths.corner_radius,
        f.corner_radius,
        border.corner_radius,
        notes,
    );
    // the surface's own gate over egui's own geometry (§6.14), not over the base style's
    // `popup_shadow`, which `defaults.border.shadow_enabled` gates
    let own = input.scheme.default_style().visuals.popup_shadow;
    f.shadow = to_shadow(own, t.defaults.shadow_color, border.shadow_enabled);
    f.inner_margin = margin(paths.padding, f.inner_margin, &border.padding, notes);
    f
}

/// A panel surface over `Frame::side_top_panel` (`egui/src/containers/frame.rs:185-189`,
/// `egui/src/containers/panel.rs:948-950`): fill, radius and padding; no stroke, because a
/// panel's separator line is painted from the parent `Ui`'s style (`panel.rs:906-911`, §4.4).
fn panel_like(
    base: &egui::Style,
    fill: Rgba,
    border: &ResolvedWidgetBorder,
    paths: &BorderPaths,
    notes: &mut Vec<Note>,
) -> egui::Frame {
    let mut f = egui::Frame::side_top_panel(base);
    f.fill = to_color32(fill);
    f.corner_radius = radius(
        paths.corner_radius,
        f.corner_radius,
        border.corner_radius,
        notes,
    );
    f.inner_margin = margin(paths.padding, f.inner_margin, &border.padding, notes);
    f
}

/// The frame of `surface` for one colour scheme, over that scheme's base style (§3.4).
pub(crate) fn surface_frame(
    surface: Surface,
    base: &egui::Style,
    input: &BuildInput<'_>,
    notes: &mut Vec<Note>,
) -> egui::Frame {
    let t = input.theme;
    match surface {
        // `Window::frame` (`egui/src/containers/window.rs:265`): fill and stroke; the radius,
        // margin and shadow are `Frame::window`'s (`frame.rs:196-203`), which reads the base
        // style's `window_corner_radius`, `window_margin` and `window_shadow` — the window's own
        // DIRECT rows (§5.2).
        Surface::Window => {
            let mut f = egui::Frame::window(base);
            f.fill = to_color32(t.window.background_color);
            f.stroke = border_stroke(f.stroke, &t.window.border, &WINDOW, t, notes);
            f
        }
        // `Window::title_frame` (`window.rs:272`): the window frame with the inactive title-bar
        // fill; egui paints the active title bar's fill from `widgets.open.weak_bg_fill` instead
        // (`window.rs:1426-1428`, §5.2), so this fill shows on a window that is not topmost.
        Surface::WindowTitleBar => {
            let mut f = egui::Frame::window(base);
            f.fill = to_color32(t.window.inactive_title_bar_background);
            f
        }
        Surface::Dialog => popup_like(
            base,
            t.dialog.background_color,
            &t.dialog.border,
            &DIALOG,
            input,
            notes,
        ),
        Surface::Popover => popup_like(
            base,
            t.popover.background_color,
            &t.popover.border,
            &POPOVER,
            input,
            notes,
        ),
        Surface::Tooltip => popup_like(
            base,
            t.tooltip.background_color,
            &t.tooltip.border,
            &TOOLTIP,
            input,
            notes,
        ),
        // `Frame::show` (`frame.rs:404`) over `Frame::group` (`:178-183`), which has no fill and
        // no shadow; `card.border.shadow_enabled` is UNMAPPABLE (§5.2), so the shadow stays `NONE`.
        Surface::Card => {
            let mut f = egui::Frame::group(base);
            f.fill = to_color32(t.card.background_color);
            f.stroke = border_stroke(f.stroke, &t.card.border, &CARD, t, notes);
            f.corner_radius = radius(
                CARD.corner_radius,
                f.corner_radius,
                t.card.border.corner_radius,
                notes,
            );
            f.inner_margin = margin(CARD.padding, f.inner_margin, &t.card.border.padding, notes);
            f
        }
        Surface::Panel(PanelSide::Left | PanelSide::Right) => panel_like(
            base,
            t.sidebar.background_color,
            &t.sidebar.border,
            &SIDEBAR,
            notes,
        ),
        Surface::Panel(PanelSide::Top) => panel_like(
            base,
            t.toolbar.background_color,
            &t.toolbar.border,
            &TOOLBAR,
            notes,
        ),
        Surface::Panel(PanelSide::Bottom) => panel_like(
            base,
            t.status_bar.background_color,
            &t.status_bar.border,
            &STATUS_BAR,
            notes,
        ),
        // `CentralPanel::frame` (`egui/src/containers/panel.rs:1206`) over `Frame::central_panel`
        // (`frame.rs:191-193`), whose fill carries the base style's `panel_fill`; the inner margin
        // is `layout.window_margin` on all four sides, egui's own where the layout states none (§5.1).
        Surface::CentralPanel => {
            let mut f = egui::Frame::central_panel(base);
            match input.layout.window_margin {
                Some(m) if m.is_finite() => {
                    if saturates_i8(m) {
                        push_note(
                            notes,
                            Note::ValueSaturated {
                                path: "layout.window_margin",
                            },
                        );
                    }
                    f.inner_margin = egui::Margin::same(i8_from_f32_saturating(m));
                }
                Some(_) => push_note(
                    notes,
                    Note::ValueSanitised {
                        path: "layout.window_margin",
                    },
                ),
                None => {}
            }
            f
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
    use native_theme::theme::{ColorMode, ResolvedPadding, ResolvedTheme};

    use super::*;
    use crate::convert::{clamp_length, to_color32, to_color32_with_opacity, to_shadow};
    use crate::install_tests::resolved;
    use crate::{AccessibilityPreferences, LayoutTheme, Note, PanelSide, Surface};

    fn theme() -> ResolvedTheme {
        resolved("kde-breeze", ColorMode::Light)
    }

    fn base() -> egui::Style {
        egui::Theme::Light.default_style()
    }

    fn frame(
        surface: Surface,
        t: &ResolvedTheme,
        layout: &LayoutTheme,
    ) -> (egui::Frame, Vec<Note>) {
        let prefs = AccessibilityPreferences::default();
        let input = BuildInput {
            scheme: egui::Theme::Light,
            theme: t,
            prefs: &prefs,
            layout,
            row_height: None,
        };
        let mut notes = Vec::new();
        let f = surface_frame(surface, &base(), &input, &mut notes);
        (f, notes)
    }

    fn unstated() -> ResolvedPadding {
        ResolvedPadding {
            top: None,
            right: None,
            bottom: None,
            left: None,
        }
    }

    fn sanitised(notes: &[Note], path: &str) -> usize {
        notes
            .iter()
            .filter(|n| matches!(n, Note::ValueSanitised { path: p, .. } if *p == path))
            .count()
    }

    fn saturated(notes: &[Note], path: &str) -> usize {
        notes
            .iter()
            .filter(|n| matches!(n, Note::ValueSaturated { path: p, .. } if *p == path))
            .count()
    }

    /// §5.2: `Surface::Window` writes fill and stroke; radius, margin and shadow are the
    /// preset's, which reads the base style's `window_*` fields. The title-bar frame differs
    /// from it in its fill alone.
    #[test]
    fn the_window_frames_write_fill_and_stroke_over_egui_s_window_preset() {
        let t = theme();
        let own = egui::Frame::window(&base());
        let (f, notes) = frame(Surface::Window, &t, &LayoutTheme::default());
        assert_eq!(f.fill, to_color32(t.window.background_color));
        assert_eq!(
            f.stroke.color,
            to_color32_with_opacity(t.window.border.color, t.defaults.border.opacity)
        );
        assert_eq!(f.stroke.width, clamp_length(t.window.border.line_width));
        assert_eq!(f.corner_radius, own.corner_radius);
        assert_eq!(f.inner_margin, own.inner_margin);
        assert_eq!(f.outer_margin, own.outer_margin);
        assert_eq!(f.shadow, own.shadow);
        assert!(notes.is_empty());

        let (tb, _) = frame(Surface::WindowTitleBar, &t, &LayoutTheme::default());
        assert_eq!(tb.fill, to_color32(t.window.inactive_title_bar_background));
        assert_eq!(
            egui::Frame {
                fill: own.fill,
                ..tb
            },
            own
        );
    }

    /// §5.2: the three popup-preset surfaces write fill, stroke, radius, the shadow gate and
    /// the padding; an unstated side keeps the preset's `menu_margin` side.
    #[test]
    fn dialog_popover_and_tooltip_write_their_rows_over_egui_s_popup_preset() {
        let mut t = theme();
        let own = egui::Frame::popup(&base());
        let opacity = t.defaults.border.opacity;
        t.dialog.border.padding = unstated();
        t.dialog.border.shadow_enabled = true;
        t.popover.border.shadow_enabled = false;
        let (d, notes) = frame(Surface::Dialog, &t, &LayoutTheme::default());
        assert_eq!(d.fill, to_color32(t.dialog.background_color));
        assert_eq!(
            d.stroke.color,
            to_color32_with_opacity(t.dialog.border.color, opacity)
        );
        assert_eq!(d.stroke.width, clamp_length(t.dialog.border.line_width));
        assert_eq!(
            d.corner_radius,
            egui::CornerRadius::same(crate::convert::u8_from_f32_saturating(
                t.dialog.border.corner_radius
            ))
        );
        assert_eq!(
            d.shadow,
            to_shadow(own.shadow, t.defaults.shadow_color, true)
        );
        assert_eq!(
            d.shadow.blur, own.shadow.blur,
            "only the colour is replaced"
        );
        assert_eq!(
            d.inner_margin, own.inner_margin,
            "every side unstated keeps the preset's"
        );
        assert_eq!(d.outer_margin, own.outer_margin);
        assert!(notes.is_empty());

        let (p, _) = frame(Surface::Popover, &t, &LayoutTheme::default());
        assert_eq!(p.fill, to_color32(t.popover.background_color));
        assert_eq!(
            p.shadow,
            egui::Shadow::NONE,
            "the gate off is exactly no shadow"
        );

        let (tt, _) = frame(Surface::Tooltip, &t, &LayoutTheme::default());
        assert_eq!(tt.fill, to_color32(t.tooltip.background_color));
        assert_eq!(
            tt.stroke.color,
            to_color32_with_opacity(t.tooltip.border.color, opacity)
        );
    }

    /// §5.2: a stated side is rounded to a whole point and lands on its own side; the others
    /// keep the preset's.
    #[test]
    fn a_stated_padding_side_lands_on_its_side_alone() {
        let mut t = theme();
        let own = egui::Frame::popup(&base());
        t.popover.border.padding = ResolvedPadding {
            top: Some(9.0),
            right: None,
            bottom: Some(2.4),
            left: None,
        };
        let (p, notes) = frame(Surface::Popover, &t, &LayoutTheme::default());
        assert_eq!(p.inner_margin.top, 9);
        assert_eq!(p.inner_margin.bottom, 2);
        assert_eq!(p.inner_margin.right, own.inner_margin.right);
        assert_eq!(p.inner_margin.left, own.inner_margin.left);
        assert!(notes.is_empty());
    }

    /// §5.2: `Surface::Card` starts from `Frame::group`, which has no fill and no shadow; the
    /// theme's fill, stroke, radius and padding are set, the shadow stays `NONE` (its row is
    /// UNMAPPABLE), and an unstated side keeps the preset's `6`.
    #[test]
    fn the_card_writes_over_egui_s_group_preset_and_casts_no_shadow() {
        let mut t = theme();
        let own = egui::Frame::group(&base());
        t.card.border.padding = unstated();
        t.card.border.shadow_enabled = true;
        let (c, _) = frame(Surface::Card, &t, &LayoutTheme::default());
        assert_eq!(c.fill, to_color32(t.card.background_color));
        assert_eq!(
            c.stroke.color,
            to_color32_with_opacity(t.card.border.color, t.defaults.border.opacity)
        );
        assert_eq!(c.stroke.width, clamp_length(t.card.border.line_width));
        assert_eq!(c.inner_margin, own.inner_margin);
        assert_eq!(c.shadow, egui::Shadow::NONE);
        assert_eq!(c.outer_margin, own.outer_margin);
    }

    /// §4.4, §5.6: the four panel sides are fed from sidebar, toolbar and status bar; a panel
    /// frame writes fill, radius and padding and no stroke — the separator line is the parent
    /// `Ui`'s style's, not the frame's.
    #[test]
    fn the_panels_take_their_side_s_native_struct() {
        let mut t = theme();
        let own = egui::Frame::side_top_panel(&base());
        t.sidebar.border.padding = ResolvedPadding {
            top: Some(4.0),
            right: Some(6.0),
            bottom: Some(4.0),
            left: Some(6.0),
        };
        t.toolbar.border.padding = unstated();
        t.status_bar.border.padding = unstated();
        for side in [PanelSide::Left, PanelSide::Right] {
            let (f, notes) = frame(Surface::Panel(side), &t, &LayoutTheme::default());
            assert_eq!(f.fill, to_color32(t.sidebar.background_color), "{side:?}");
            assert_eq!(
                f.inner_margin,
                egui::Margin {
                    left: 6,
                    right: 6,
                    top: 4,
                    bottom: 4
                }
            );
            assert_eq!(
                f.corner_radius,
                egui::CornerRadius::same(crate::convert::u8_from_f32_saturating(
                    t.sidebar.border.corner_radius
                ))
            );
            assert_eq!(f.stroke, own.stroke);
            assert_eq!(f.shadow, own.shadow);
            assert!(notes.is_empty());
        }
        let (top, _) = frame(Surface::Panel(PanelSide::Top), &t, &LayoutTheme::default());
        assert_eq!(top.fill, to_color32(t.toolbar.background_color));
        assert_eq!(top.inner_margin, own.inner_margin);
        let (bottom, _) = frame(
            Surface::Panel(PanelSide::Bottom),
            &t,
            &LayoutTheme::default(),
        );
        assert_eq!(bottom.fill, to_color32(t.status_bar.background_color));
        assert_eq!(bottom.inner_margin, own.inner_margin);
    }

    /// §5.1: the central panel's inner margin is `layout.window_margin` on all four sides, and
    /// egui's own `central_panel` margin where the layout states none; its fill is the preset's,
    /// which carries the base style's `panel_fill`.
    #[test]
    fn the_central_panel_takes_the_layout_s_window_margin() {
        let t = theme();
        let own = egui::Frame::central_panel(&base());
        let (f, notes) = frame(Surface::CentralPanel, &t, &LayoutTheme::default());
        assert_eq!(f, own, "nothing stated: egui's own frame");
        assert!(notes.is_empty());

        let layout = LayoutTheme {
            window_margin: Some(12.0),
            ..LayoutTheme::default()
        };
        let (f, _) = frame(Surface::CentralPanel, &t, &layout);
        assert_eq!(f.inner_margin, egui::Margin::same(12));
        assert_eq!(f.fill, own.fill);

        let layout = LayoutTheme {
            window_margin: Some(f32::NAN),
            ..LayoutTheme::default()
        };
        let (f, notes) = frame(Surface::CentralPanel, &t, &layout);
        assert_eq!(f.inner_margin, own.inner_margin);
        assert_eq!(sanitised(&notes, "layout.window_margin"), 1);

        let layout = LayoutTheme {
            window_margin: Some(300.0),
            ..LayoutTheme::default()
        };
        let (f, notes) = frame(Surface::CentralPanel, &t, &layout);
        assert_eq!(f.inner_margin, egui::Margin::same(127));
        assert_eq!(saturated(&notes, "layout.window_margin"), 1);
    }

    /// §6, §7.2: a non-finite width, radius or side keeps the preset's value and is reported for
    /// its leaf; a saturated side is reported as data loss.
    #[test]
    fn hostile_border_values_keep_the_preset_s_and_are_reported() {
        let mut t = theme();
        let own = egui::Frame::popup(&base());
        t.dialog.border.line_width = f32::NAN;
        t.dialog.border.corner_radius = f32::INFINITY;
        t.dialog.border.padding = ResolvedPadding {
            top: Some(f32::NEG_INFINITY),
            right: Some(300.0),
            bottom: Some(-300.0),
            left: Some(3.0),
        };
        let (d, notes) = frame(Surface::Dialog, &t, &LayoutTheme::default());
        assert_eq!(d.stroke.width, own.stroke.width);
        assert_eq!(
            d.stroke.color,
            to_color32_with_opacity(t.dialog.border.color, t.defaults.border.opacity),
            "the colour is still written"
        );
        assert_eq!(d.corner_radius, own.corner_radius);
        assert_eq!(d.inner_margin.top, own.inner_margin.top);
        assert_eq!(d.inner_margin.right, 127);
        assert_eq!(d.inner_margin.bottom, -128);
        assert_eq!(d.inner_margin.left, 3);
        assert_eq!(sanitised(&notes, "dialog.border.line_width"), 1);
        assert_eq!(sanitised(&notes, "dialog.border.corner_radius"), 1);
        assert_eq!(sanitised(&notes, "dialog.border.padding.top"), 1);
        assert_eq!(saturated(&notes, "dialog.border.padding.right"), 1);
        assert_eq!(saturated(&notes, "dialog.border.padding.bottom"), 1);
        assert_eq!(saturated(&notes, "dialog.border.padding.left"), 0);
        assert_eq!(notes.len(), 5);
    }
}
