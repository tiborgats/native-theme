//! Headless tests (§6 of the crate's specification, T1-T8): a bare `egui::Context` driven with
//! `RawInput`, AccessKit enabled where a test reads nodes.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    reason = "a test fails by panicking"
)]

use std::path::{Path, PathBuf};

use native_theme_egui::convert::{composite_over, to_color32};
use native_theme_egui::egui::{self, accesskit};
use native_theme_egui::native_theme::theme::IconSet;
use native_theme_egui::{
    AccessibilityPreferences, ResolvedTheme, ThemeAtlas, focus_ring_color, focus_ring_offset,
    from_preset,
};
use native_theme_test_fixtures::sources::{every_section_is_named, rendered_lines};

use crate::combo_box::ComboBox;
use crate::expander::Expander;
use crate::links;
use crate::parts::Parts;
use crate::progress_bar::ProgressBar;
use crate::radio_button::RadioButton;
use crate::segmented_control::SegmentedControl;
use crate::slider::Slider;
use crate::spinner::Spinner;
use crate::switch::Switch;

// ---- harness ----------------------------------------------------------------------------------

/// A `kde-breeze` light theme, resolved as the connector resolves it.
fn kde() -> ResolvedTheme {
    from_preset("kde-breeze", false, &AccessibilityPreferences::default())
        .unwrap()
        .1
}

/// A context with `t` installed in both schemes, drawn light, its icon set one with no
/// animated indicator so the spinner paints its arc.
fn installed(t: &ResolvedTheme, prefs: &AccessibilityPreferences) -> egui::Context {
    let ctx = egui::Context::default();
    ThemeAtlas::builder("test", t, t)
        .accessibility(prefs)
        .icon_set(IconSet::SfSymbols)
        .build()
        .install(&ctx);
    ctx.set_theme(egui::Theme::Light);
    ctx.enable_accesskit();
    ctx
}

fn bare() -> egui::Context {
    let ctx = egui::Context::default();
    ctx.set_theme(egui::Theme::Light);
    ctx.enable_accesskit();
    ctx
}

/// One pass; the texture delta is dropped unapplied, as egui's own tests do.
fn pass(
    ctx: &egui::Context,
    input: egui::RawInput,
    f: impl FnMut(&mut egui::Ui),
) -> egui::FullOutput {
    let mut out = ctx.run_ui(input, f);
    out.textures_delta.clear();
    out
}

fn at(time: f64) -> egui::RawInput {
    egui::RawInput {
        time: Some(time),
        focused: true,
        ..Default::default()
    }
}

/// A pass with the window focused and egui's own clock.
fn idle() -> egui::RawInput {
    egui::RawInput {
        focused: true,
        ..Default::default()
    }
}

fn pointer(pos: egui::Pos2, pressed: bool) -> egui::RawInput {
    egui::RawInput {
        events: vec![
            egui::Event::PointerMoved(pos),
            egui::Event::PointerButton {
                pos,
                button: egui::PointerButton::Primary,
                pressed,
                modifiers: egui::Modifiers::NONE,
            },
        ],
        focused: true,
        ..Default::default()
    }
}

fn tab() -> egui::RawInput {
    egui::RawInput {
        events: vec![egui::Event::Key {
            key: egui::Key::Tab,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }],
        focused: true,
        ..Default::default()
    }
}

fn flat(shapes: &[egui::epaint::ClippedShape]) -> Vec<egui::Shape> {
    fn walk(shape: &egui::Shape, out: &mut Vec<egui::Shape>) {
        match shape {
            egui::Shape::Vec(inner) => inner.iter().for_each(|s| walk(s, out)),
            other => out.push(other.clone()),
        }
    }
    let mut out = Vec::new();
    shapes.iter().for_each(|s| walk(&s.shape, &mut out));
    out
}

fn rects(out: &egui::FullOutput) -> Vec<egui::epaint::RectShape> {
    flat(&out.shapes)
        .into_iter()
        .filter_map(|s| match s {
            egui::Shape::Rect(r) => Some(r),
            _ => None,
        })
        .collect()
}

fn circles(out: &egui::FullOutput) -> Vec<egui::epaint::CircleShape> {
    flat(&out.shapes)
        .into_iter()
        .filter_map(|s| match s {
            egui::Shape::Circle(c) => Some(c),
            _ => None,
        })
        .collect()
}

fn node(out: &egui::FullOutput, id: egui::Id) -> accesskit::Node {
    let update = out.platform_output.accesskit_update.as_ref().unwrap();
    update
        .nodes
        .iter()
        .find(|(n, _)| *n == id.accesskit_id())
        .map(|(_, node)| node.clone())
        .unwrap()
}

/// The response `add` returns from one pass on `ctx`.
fn response(
    ctx: &egui::Context,
    input: egui::RawInput,
    mut add: impl FnMut(&mut egui::Ui) -> egui::Response,
) -> (egui::FullOutput, egui::Response) {
    let mut got = None;
    let out = pass(ctx, input, |ui| got = Some(add(ui)));
    (out, got.unwrap())
}

/// Click the centre of what `add` draws: a layout pass, a press, a release.
fn click(
    ctx: &egui::Context,
    mut add: impl FnMut(&mut egui::Ui) -> egui::Response,
) -> (egui::FullOutput, egui::Response) {
    let (_, r) = response(ctx, idle(), &mut add);
    let centre = r.rect.center();
    let _ = response(ctx, pointer(centre, true), &mut add);
    response(ctx, pointer(centre, false), &mut add)
}

// ---- T1: no atlas ------------------------------------------------------------------------------

/// T1: with no atlas, each widget paints exactly what its egui counterpart paints.
#[test]
fn with_no_atlas_every_widget_is_its_egui_counterpart() {
    type Add = Box<dyn Fn(&mut egui::Ui) -> egui::Response>;
    let cases: Vec<(&str, Add, Add)> = vec![
        (
            "switch",
            Box::new(|ui| ui.add(Switch::new(&mut true).label("Wi-Fi"))),
            Box::new(|ui| ui.add(egui::Checkbox::new(&mut true, "Wi-Fi"))),
        ),
        (
            "disabled switch",
            Box::new(|ui| ui.add(Switch::new(&mut false).enabled(false))),
            Box::new(|ui| ui.add_enabled(false, egui::Checkbox::without_text(&mut false))),
        ),
        (
            "slider",
            Box::new(|ui| ui.add(Slider::new(&mut 40.0, 0.0..=100.0).label("Volume"))),
            Box::new(|ui| {
                ui.add(
                    egui::Slider::new(&mut 40.0, 0.0..=100.0)
                        .show_value(false)
                        .text("Volume"),
                )
            }),
        ),
        (
            "spinner",
            Box::new(|ui| ui.add(Spinner::new())),
            Box::new(|ui| ui.add(egui::Spinner::new())),
        ),
        (
            "radio button",
            Box::new(|ui| ui.add(RadioButton::new(true, "Radio"))),
            Box::new(|ui| ui.add(egui::RadioButton::new(true, "Radio"))),
        ),
        (
            "disabled radio button",
            Box::new(|ui| ui.add(RadioButton::new(false, "Radio").enabled(false))),
            Box::new(|ui| ui.add_enabled(false, egui::RadioButton::new(false, "Radio"))),
        ),
        (
            "segmented control",
            Box::new(|ui| ui.add(SegmentedControl::new(&mut 1, ["Day", "Week"]))),
            Box::new(|ui| {
                ui.horizontal(|ui| {
                    ui.add(egui::Button::new("Day").selected(false));
                    ui.add(egui::Button::new("Week").selected(true));
                })
                .response
            }),
        ),
        (
            "link",
            Box::new(|ui| {
                let w = links::link(ui, "Link");
                ui.add(w)
            }),
            Box::new(|ui| ui.add(egui::Link::new("Link"))),
        ),
        (
            "hyperlink",
            Box::new(|ui| {
                let w = links::hyperlink(ui, "Docs", "https://example.org");
                ui.add(w)
            }),
            Box::new(|ui| {
                ui.add(egui::Hyperlink::from_label_and_url(
                    "Docs",
                    "https://example.org",
                ))
            }),
        ),
    ];
    let mut cases = cases;
    cases.push((
        "combo box",
        Box::new(|ui| {
            ComboBox::from_id_salt("fruit")
                .selected_text("Apple")
                .width(140.0)
                .show_ui(ui, |ui| ui.label("Banana"))
                .response
        }),
        Box::new(|ui| {
            egui::ComboBox::from_id_salt("fruit")
                .selected_text("Apple")
                .width(140.0)
                .show_ui(ui, |ui| ui.label("Banana"))
                .response
        }),
    ));
    for (name, ours, egui_s) in cases {
        let (a, ra) = response(&bare(), at(1.0), |ui| ours(ui));
        let (b, rb) = response(&bare(), at(1.0), |ui| egui_s(ui));
        assert!(ra.rect.is_finite(), "{name}");
        assert_eq!(ra.rect, rb.rect, "{name}: a different size from egui's");
        assert_eq!(
            flat(&a.shapes),
            flat(&b.shapes),
            "{name}: not egui's pixels"
        );
    }
}

// ---- T2: a hostile theme -----------------------------------------------------------------------

fn hostile(v: f32) -> ResolvedTheme {
    let mut t = kde();
    t.switch.track_width = v;
    t.switch.track_height = v;
    t.switch.thumb_diameter = v;
    t.switch.track_radius = v;
    t.slider.thumb_diameter = v;
    t.slider.track_height = v;
    t.spinner.diameter = v;
    t.spinner.stroke_width = v;
    t.spinner.min_diameter = v;
    t.segmented_control.segment_height = v;
    t.segmented_control.separator_width = v;
    t.checkbox.radio_dot_diameter = Some(v);
    t
}

/// How many widgets `one_widget` adds.
const WIDGETS: usize = 8;

/// Widget `i` of every widget of the crate: switch, slider, spinner, segmented control, link,
/// hyperlink, combo box, radio button.
fn one_widget(ui: &mut egui::Ui, i: usize) -> egui::Response {
    match i {
        0 => ui.add(Switch::new(&mut true).label("Wi-Fi")),
        1 => ui.add(Slider::new(&mut 40.0, 0.0..=100.0)),
        2 => ui.add(Spinner::new()),
        3 => ui.add(SegmentedControl::new(&mut 0, ["Day", "Week"])),
        4 => {
            let link = links::link(ui, "Link");
            ui.add(link)
        }
        5 => {
            let hyperlink = links::hyperlink(ui, "Docs", "https://example.org");
            ui.add(hyperlink)
        }
        6 => {
            ComboBox::from_id_salt("fruit")
                .selected_text("Apple")
                .show_ui(ui, |ui| ui.label("Banana"))
                .response
        }
        _ => ui.add(RadioButton::new(true, "Radio")),
    }
}

fn all_widgets(ui: &mut egui::Ui) -> Vec<egui::Response> {
    (0..WIDGETS).map(|i| one_widget(ui, i)).collect()
}

/// T2: `NaN`, `±∞`, `-0.0` and huge values in every length a widget reads: finite space, and a
/// painted widget whose geometry is not finite is its egui counterpart.
#[test]
fn a_hostile_theme_allocates_finite_space() {
    for v in [
        f32::NAN,
        f32::INFINITY,
        f32::NEG_INFINITY,
        -0.0,
        f32::MAX,
        1.0e30,
    ] {
        // One widget per context: two stacked lengths near the ceiling overflow egui's layout
        // whoever draws them (the connector's `clamp_length`).
        for i in 0..WIDGETS {
            let ctx = installed(&hostile(v), &AccessibilityPreferences::default());
            for time in [0.0, 0.5] {
                let (out, r) = response(&ctx, at(time), |ui| one_widget(ui, i));
                assert!(r.rect.is_finite(), "{v}, widget {i}: {:?}", r.rect);
                if i == 0 && !v.is_finite() {
                    assert_eq!(node(&out, r.id).role(), accesskit::Role::CheckBox, "{v}");
                }
            }
        }
    }
}

// ---- T3: accessibility and focus ---------------------------------------------------------------

/// T3: every widget's node carries the role and states of §2.7.
#[test]
fn every_widget_reports_its_role() {
    let ctx = installed(&kde(), &AccessibilityPreferences::default());
    let mut ids = Vec::new();
    let out = pass(&ctx, at(0.0), |ui| {
        ids = all_widgets(ui).into_iter().map(|r| r.id).collect();
    });
    let switch = node(&out, ids[0]);
    assert_eq!(switch.role(), accesskit::Role::Switch);
    assert_eq!(switch.toggled(), Some(accesskit::Toggled::True));
    let slider = node(&out, ids[1]);
    assert_eq!(slider.role(), accesskit::Role::Slider);
    assert_eq!(slider.numeric_value(), Some(40.0));
    assert_eq!(slider.min_numeric_value(), Some(0.0));
    assert_eq!(slider.max_numeric_value(), Some(100.0));
    assert_eq!(
        node(&out, ids[2]).role(),
        accesskit::Role::ProgressIndicator
    );
    let update = out.platform_output.accesskit_update.as_ref().unwrap();
    let by_id = |id: &accesskit::NodeId| {
        update
            .nodes
            .iter()
            .find(|(n, _)| n == id)
            .map(|(_, node)| node.clone())
            .unwrap()
    };
    // The control is the outline; the row inside it, its child, is the radio group.
    let groups: Vec<accesskit::Node> = node(&out, ids[3])
        .children()
        .iter()
        .map(by_id)
        .filter(|n| n.role() == accesskit::Role::RadioGroup)
        .collect();
    assert_eq!(groups.len(), 1);
    let buttons: Vec<accesskit::Node> = groups[0].children().iter().map(by_id).collect();
    assert_eq!(buttons.len(), 2);
    assert!(
        buttons
            .iter()
            .all(|b| b.role() == accesskit::Role::RadioButton)
    );
    assert_eq!(buttons[0].toggled(), Some(accesskit::Toggled::True));
    assert_eq!(buttons[1].toggled(), Some(accesskit::Toggled::False));
    // A wrapped link reports what egui's own `Link` reports: with `selectable_labels` on, egui
    // 0.36.2's label text selection makes its node a `Label`.
    let (egui_out, egui_link) = response(&bare(), at(0.0), |ui| ui.add(egui::Link::new("Link")));
    let egui_role = node(&egui_out, egui_link.id).role();
    assert_eq!(node(&out, ids[4]).role(), egui_role);
    assert_eq!(node(&out, ids[5]).role(), egui_role);
    assert!(!node(&out, ids[5]).is_visited());
    // The drop-down is egui's own, and reports what egui's reports.
    assert_eq!(node(&out, ids[6]).role(), accesskit::Role::ComboBox);
    // So is the radio button.
    assert_eq!(node(&out, ids[7]).role(), accesskit::Role::RadioButton);
}

/// An off switch's thumb is `switch.unchecked_thumb_diameter` across in
/// `unchecked_thumb_background` where the theme states them (Material's 16 in `outline`,
/// docs/platform-facts.md:1525-1526), centred on the track's axis.
#[test]
fn the_off_thumb_is_the_stated_off_thumb() {
    let mut t = kde();
    assert_eq!(t.switch.unchecked_thumb_diameter, None);
    assert_eq!(t.switch.unchecked_thumb_background, None);
    t.switch.unchecked_thumb_diameter = Some(t.switch.thumb_diameter - 6.0);
    t.switch.unchecked_thumb_background = Some(t.defaults.danger_color);
    let ctx = installed(&t, &AccessibilityPreferences::default());
    let mut on = false;
    let (out, _) = response(&ctx, at(0.0), |ui| ui.add(Switch::new(&mut on)));
    let thumb = circles(&out)
        .into_iter()
        .find(|c| c.fill == to_color32(t.defaults.danger_color))
        .map(|c| 2.0 * (c.radius + c.stroke.width));
    assert_eq!(thumb, t.switch.unchecked_thumb_diameter);
}

/// docs/platform-facts.md §2.21: qqc2-desktop-style's handle is the 18 of the indicator on a
/// 12-tall track, so it overhangs the track by 3 above and below and sits flush with the
/// track's ends (`SwitchIndicator.qml:41-48`, `:65-76`, `:68`).
#[test]
fn a_thumb_taller_than_its_track_sits_flush_with_its_ends() {
    let t = kde();
    let s = &t.switch;
    assert!(
        s.thumb_diameter > s.track_height,
        "kde-breeze's thumb no longer overhangs its track"
    );
    for checked in [false, true] {
        let ctx = installed(&t, &AccessibilityPreferences::default());
        let mut on = checked;
        let (out, _) = response(&ctx, at(0.0), |ui| ui.add(Switch::new(&mut on)));
        let size = egui::vec2(s.track_width, s.track_height);
        let track = rects(&out)
            .into_iter()
            .rev()
            .find(|r| r.rect.size() == size)
            .unwrap()
            .rect;
        let thumb = circles(&out)
            .into_iter()
            .find(|c| 2.0 * (c.radius + c.stroke.width) == s.thumb_diameter)
            .unwrap();
        let half = 0.5 * s.thumb_diameter;
        assert_eq!(
            thumb.center.y,
            track.center().y,
            "checked {checked}: on the axis"
        );
        if checked {
            assert_eq!(
                thumb.center.x + half,
                track.right(),
                "on: flush with the right end"
            );
        } else {
            assert_eq!(
                thumb.center.x - half,
                track.left(),
                "off: flush with the left end"
            );
        }
    }
}

/// The track is outlined inside its edge in the colour the theme states for its state,
/// `switch.track_border_line_width` wide, and the thumb inside its edge in
/// `thumb_border_color`, `thumb_border_line_width` wide (epaint strokes a circle outside its
/// radius, `epaint/src/tessellator.rs:1531`); a part whose colour or width is unset has no
/// outline (docs/platform-facts.md §2.21). Under kde-breeze (both tracks and the thumb
/// outlined), windows-11 and material-3 (the off track only), on and off, enabled and disabled.
#[test]
fn the_switch_draws_the_stated_outlines() {
    for preset in ["kde-breeze", "windows-11", "material-3"] {
        for dark in [false, true] {
            let t = from_preset(preset, dark, &AccessibilityPreferences::default())
                .unwrap()
                .1;
            let s = &t.switch;
            for (checked, enabled) in [(false, true), (true, true), (false, false), (true, false)] {
                let state = format!("{preset} dark {dark} checked {checked} enabled {enabled}");
                assert_eq!(s.disabled_opacity, 1.0, "{state}: drawn unfaded");
                assert!(
                    s.unchecked_track_border_color.is_some() && s.track_border_line_width.is_some(),
                    "{state}: {preset} no longer states the off track's outline \
                     (docs/platform-facts.md §2.21)"
                );
                let ctx = installed(&t, &AccessibilityPreferences::default());
                let mut on = checked;
                let (out, _) = response(&ctx, at(0.0), |ui| {
                    ui.add(Switch::new(&mut on).enabled(enabled))
                });
                let colour = match (checked, enabled) {
                    (true, true) => s.checked_track_border_color,
                    (false, true) => s.unchecked_track_border_color,
                    (true, false) => s
                        .disabled_checked_track_border_color
                        .or(s.checked_track_border_color),
                    (false, false) => s
                        .disabled_unchecked_track_border_color
                        .or(s.unchecked_track_border_color),
                };
                let stated = |colour: Option<native_theme_egui::native_theme::color::Rgba>,
                              width: Option<f32>| {
                    colour
                        .zip(width)
                        .map(|(colour, width)| (width, to_color32(colour)))
                };
                let size = egui::vec2(s.track_width, s.track_height);
                // The track, painted after the layout's frame of the same size.
                let track = rects(&out)
                    .into_iter()
                    .rev()
                    .find(|r| r.rect.size() == size)
                    .unwrap();
                let edge = (track.stroke.width > 0.0).then(|| {
                    assert_eq!(track.stroke_kind, egui::StrokeKind::Inside, "{state}");
                    (track.stroke.width, track.stroke.color)
                });
                assert_eq!(
                    edge,
                    stated(colour, s.track_border_line_width),
                    "{state}: the track"
                );
                let diameter = if checked {
                    s.thumb_diameter
                } else {
                    s.unchecked_thumb_diameter.unwrap_or(s.thumb_diameter)
                };
                let thumb = circles(&out)
                    .into_iter()
                    .find(|c| 2.0 * (c.radius + c.stroke.width) == diameter)
                    .unwrap();
                let edge =
                    (thumb.stroke.width > 0.0).then_some((thumb.stroke.width, thumb.stroke.color));
                assert_eq!(
                    edge,
                    stated(s.thumb_border_color, s.thumb_border_line_width),
                    "{state}: the thumb"
                );
            }
        }
    }
}

/// The radio button's circle is `checkbox.radio_indicator_width` across its outline where the
/// theme states one (Material's 20 against its check box's 18, docs/platform-facts.md:1243).
#[test]
fn the_radio_circle_is_the_stated_radio_width() {
    let mut t = kde();
    assert_eq!(t.checkbox.radio_indicator_width, None);
    t.checkbox.radio_indicator_width = Some(t.checkbox.indicator_width + 6.0);
    let ctx = installed(&t, &AccessibilityPreferences::default());
    let (out, _) = response(&ctx, at(0.0), |ui| ui.add(RadioButton::new(true, "Radio")));
    let across = circles(&out)
        .iter()
        .find(|c| c.fill == to_color32(t.checkbox.checked_background))
        .map(|c| 2.0 * (c.radius + c.stroke.width));
    assert_eq!(across, t.checkbox.radio_indicator_width);
}

/// The radio button's dot is `checkbox.radio_dot_diameter` across, in `indicator_color`, on
/// the checked circle `indicator_width` across its outline in `checked_background` (epaint
/// strokes a circle outside its radius, `epaint/src/tessellator.rs:1531`, as a check box's
/// square is its outline's outer edge); where the theme states no dot size it is egui's own.
#[test]
fn the_radio_dot_is_the_themes() {
    let t = kde();
    assert_eq!(t.checkbox.radio_dot_diameter, Some(6.0));
    let ctx = installed(&t, &AccessibilityPreferences::default());
    let (out, _) = response(&ctx, at(0.0), |ui| ui.add(RadioButton::new(true, "Radio")));
    let shapes = circles(&out);
    let circle = shapes
        .iter()
        .find(|c| c.fill == to_color32(t.checkbox.checked_background))
        .unwrap();
    assert_eq!(
        2.0 * (circle.radius + circle.stroke.width),
        t.checkbox.indicator_width
    );
    let dot = shapes
        .iter()
        .find(|c| c.fill == to_color32(t.checkbox.indicator_color))
        .unwrap();
    assert_eq!(2.0 * dot.radius, 6.0);
    assert_eq!(dot.center, circle.center);

    let mut unstated = t.clone();
    unstated.checkbox.radio_dot_diameter = None;
    let ctx = installed(&unstated, &AccessibilityPreferences::default());
    let (out, _) = response(&ctx, at(0.0), |ui| ui.add(RadioButton::new(true, "Radio")));
    let (egui_out, _) = response(&ctx, at(0.0), |ui| {
        native_theme_egui::NativeThemeUiExt::native_scope(
            ui,
            native_theme_egui::Role::Checkbox,
            native_theme_egui::RoleVariant::Selected,
            |ui| ui.add(egui::RadioButton::new(true, "Radio")),
        )
        .inner
    });
    // egui's own dot, on the circle drawn inside the indicator's width.
    let dots = |out: &egui::FullOutput| {
        circles(out)
            .into_iter()
            .filter(|c| c.fill == to_color32(unstated.checkbox.indicator_color))
            .collect::<Vec<_>>()
    };
    assert_eq!(dots(&out), dots(&egui_out), "egui's own dot");
    assert!(!dots(&out).is_empty(), "the checked radio draws a dot");
}

/// The expander lays out what `expander.*` states. KDE: the arrow before the title,
/// `arrow_gap` between its box and the title, the body `content_indent` in, and no frame.
/// GNOME: the arrow at the row's end and one frame round header and body.
#[test]
fn the_expander_is_laid_out_as_stated() {
    use crate::expander::Expander;
    use native_theme_egui::NativeThemeUiExt as _;

    let t = kde();
    let e = &t.expander;
    assert!(
        e.arrow_gap.is_some() && e.content_indent.is_some(),
        "kde-breeze states the expander's gap and indent"
    );
    let (gap, indent) = (
        e.arrow_gap.unwrap_or_default(),
        e.content_indent.unwrap_or_default(),
    );
    assert_eq!(e.frame_enabled, Some(false));
    let ctx = installed(&t, &AccessibilityPreferences::default());
    let mut shown = None;
    let mut pad_left = 0.0;
    let show = |ui: &mut egui::Ui| {
        Expander::new("Details")
            .default_open(true)
            .show(ui, |ui| ui.label("Body"))
    };
    let _ = pass(&ctx, at(0.0), |ui| {
        let _ = show(ui);
    });
    let out = pass(&ctx, at(0.0), |ui| {
        pad_left = ui
            .native_scope(
                native_theme_egui::Role::Expander,
                native_theme_egui::RoleVariant::Normal,
                |ui| ui.spacing().button_padding.x,
            )
            .inner;
        let r = show(ui);
        shown = Some((r.header_response.rect, r.body_returned.map(|l| l.rect)));
    });
    assert!(
        matches!(shown, Some((_, Some(_)))),
        "the expander shows its open body"
    );
    let (header, body) = match shown {
        Some((header, Some(body))) => (header, body),
        _ => (egui::Rect::NOTHING, egui::Rect::NOTHING),
    };
    assert_eq!(body.left() - header.left(), indent, "content_indent");
    let title = flat(&out.shapes).into_iter().find_map(|s| match s {
        egui::Shape::Text(text) if text.galley.text() == "Details" => Some(text.pos.x),
        _ => None,
    });
    assert_eq!(
        title,
        Some(header.left() + pad_left + e.arrow_icon_size + gap),
        "the title arrow_gap after the arrow's box"
    );
    assert!(
        !rects(&out).iter().any(|r| r.stroke.width > 0.0),
        "an unframed expander strokes nothing"
    );

    let adw = from_preset("adwaita", false, &AccessibilityPreferences::default())
        .unwrap()
        .1;
    assert_eq!(adw.expander.frame_enabled, Some(true));
    let ctx = installed(&adw, &AccessibilityPreferences::default());
    let _ = pass(&ctx, at(0.0), |ui| {
        let _ = show(ui);
    });
    let mut header = egui::Rect::NOTHING;
    let out = pass(&ctx, at(0.0), |ui| header = show(ui).header_response.rect);
    let title = flat(&out.shapes).into_iter().find_map(|s| match s {
        egui::Shape::Text(text) if text.galley.text() == "Details" => Some(text.pos.x),
        _ => None,
    });
    assert!(
        title.is_some_and(|x| x < header.center().x),
        "a trailing arrow leaves the title at the row's start"
    );
    assert!(
        rects(&out)
            .iter()
            .any(|r| r.stroke.width > 0.0 && r.rect.contains_rect(header)),
        "one frame round the header and the body"
    );
}

/// `scale` as the OS's text-scaling factor.
fn scaled_by(scale: f32) -> AccessibilityPreferences {
    let mut prefs = AccessibilityPreferences::default();
    prefs.text_scaling_factor = scale;
    prefs
}

/// The header's height by the rule every stated row height takes (M9-150), as the showcases'
/// `rows::scaled_height` computes it: at a text scale of 1 or less `stated`; above 1 at least
/// the line box of the title's font at its scaled size and `defaults.line_height`, rounded up,
/// plus `padding`, the header's own above and below.
fn header_rule(t: &ResolvedTheme, prefs: &AccessibilityPreferences, padding: f32) -> f32 {
    let stated = t.expander.header_height;
    if prefs.text_scale_factor() <= 1.0 {
        return stated;
    }
    let line = (prefs.scaled(t.expander.font.size) * t.defaults.line_height).ceil();
    stated.max(line + padding)
}

/// The padding above and below the title: on the painted path each stated side of
/// `expander.border.padding`, the scope's `button_padding.y` where unstated; on egui's own, the
/// scope's `button_padding.y` twice (`egui/src/containers/collapsing_header.rs:531`).
fn header_padding(t: &ResolvedTheme, ctx: &egui::Context, painted: bool) -> f32 {
    use native_theme_egui::NativeThemeUiExt as _;
    let mut own = 0.0;
    let _ = pass(ctx, at(0.0), |ui| {
        own = ui
            .native_scope(
                native_theme_egui::Role::Expander,
                native_theme_egui::RoleVariant::Normal,
                |ui| ui.spacing().button_padding.y,
            )
            .inner;
    });
    let pad = &t.expander.border.padding;
    if painted {
        pad.top.unwrap_or(own) + pad.bottom.unwrap_or(own)
    } else {
        2.0 * own
    }
}

/// The header is `expander.header_height` tall at a text scale of 1 or less, and above 1 the
/// rule's height, the stated height grown to hold its scaled line box and padding: on the
/// painted path (kde-breeze, adwaita) and on egui's own `CollapsingHeader` (material-3), as
/// the gpui showcase's header is the stated height at 1 (`geometry::accordion_title`). At 1.5
/// kde-breeze's 13.33 px title's line box is 27.2 at its 1.36 line height, 28 rounded up, so the
/// header is 28 + 7 + 7 = 42, past the stated 40, where it was its title's 27 and its padding.
#[test]
fn the_header_is_its_stated_height_by_the_row_rule() {
    let mut faults = Vec::new();
    for (preset, painted) in [
        ("kde-breeze", true),
        ("adwaita", true),
        ("material-3", false),
    ] {
        for scale in [1.0, 1.5] {
            let prefs = scaled_by(scale);
            let t = from_preset(preset, false, &prefs).unwrap().1;
            let ctx = installed(&t, &prefs);
            let show = |ui: &mut egui::Ui| {
                Expander::new("Details")
                    .show(ui, |ui| ui.label("Body"))
                    .header_response
            };
            let _ = response(&ctx, at(0.0), show);
            let (_, header) = response(&ctx, at(0.0), show);
            let want = header_rule(&t, &prefs, header_padding(&t, &ctx, painted));
            if (header.rect.height() - want).abs() > 0.01 {
                faults.push(format!(
                    "{preset} at {scale}: the header is {}, want {want}",
                    header.rect.height()
                ));
            }
        }
    }
    assert!(faults.is_empty(), "{faults:#?}");
}

/// At a text scale of 1 or less the painted header is its stated height and its title one
/// line, cut short with egui's ellipsis where it is too long for its column
/// (`TextWrapMode::Truncate`, `epaint/src/text/text_layout_types.rs:707-714`; the `…`,
/// `text_layout.rs:146-149`), inside the header and clear of a trailing arrow, as egui's own
/// single-line labels truncate; at least as wide as what is left of it where no width is left
/// for the title at all. Above 1 the title
/// wraps inside the header, which grows to hold it.
#[test]
fn a_long_title_is_one_line_inside_the_stated_header() {
    let title = "A title far longer than the narrow column it is shown in";
    for scale in [1.0, 1.5] {
        let prefs = scaled_by(scale);
        let t = from_preset("adwaita", false, &prefs).unwrap().1;
        assert_eq!(
            t.expander.arrow_side,
            Some(native_theme_egui::native_theme::theme::ArrowSide::Trailing)
        );
        let ctx = installed(&t, &prefs);
        // 120 points is narrower than the title; 1 point leaves no width for it at all.
        for width in [120.0, 1.0] {
            let show = |ui: &mut egui::Ui| {
                ui.allocate_ui(egui::vec2(width, f32::INFINITY), |ui| {
                    Expander::new(title)
                        .show(ui, |ui| ui.label("Body"))
                        .header_response
                })
                .inner
            };
            let _ = response(&ctx, at(0.0), show);
            let (out, header) = response(&ctx, at(0.0), show);
            let text = flat(&out.shapes)
                .into_iter()
                .find_map(|s| match s {
                    egui::Shape::Text(text) if text.galley.text() == title => Some(text),
                    _ => None,
                })
                .unwrap();
            let painted = egui::Rect::from_min_size(text.pos, text.galley.size());
            if scale <= 1.0 {
                assert_eq!(text.galley.rows.len(), 1, "{width}: one line at 1");
                assert!(text.galley.elided, "{width}: cut short at 1");
                assert_eq!(
                    header.rect.height(),
                    t.expander.header_height,
                    "{width}: the stated height at 1"
                );
                assert!(
                    header.rect.x_range().contains(painted.left())
                        && header.rect.x_range().contains(painted.right()),
                    "{width}: {painted:?} across inside {:?}",
                    header.rect
                );
            } else {
                assert!(
                    text.galley.rows.len() > 1,
                    "{width}: the title wraps above 1"
                );
                assert!(
                    header.rect.contains_rect(painted),
                    "{width}: {painted:?} inside {:?}",
                    header.rect
                );
            }
            let arrow = Parts::of(&header).unwrap().get("arrow").unwrap();
            assert!(
                painted.right() <= arrow.left(),
                "{scale}, {width}: {painted:?} clear of the arrow {arrow:?}"
            );
        }
    }
}

/// A title the stated header cuts short shows its whole text under the pointer, as egui's own
/// truncated `Label` does (`egui/src/widgets/label.rs:285-294`, `show_tooltip_when_elided`,
/// true by default): adwaita in a 120-point column, the pointer resting on the header past the
/// tooltip delay, paints the full title once, unelided. A title that fits shows none.
#[test]
fn a_cut_title_shows_its_whole_text_on_hover() {
    let long = "A title far longer than the narrow column it is shown in";
    let prefs = AccessibilityPreferences::default();
    let t = from_preset("adwaita", false, &prefs).unwrap().1;
    for (title, tip) in [(long, true), ("Details", false)] {
        let ctx = installed(&t, &prefs);
        let show = |ui: &mut egui::Ui| {
            ui.allocate_ui(egui::vec2(120.0, f32::INFINITY), |ui| {
                Expander::new(title)
                    .show(ui, |ui| ui.label("Body"))
                    .header_response
            })
            .inner
        };
        let (_, header) = response(&ctx, at(0.0), show);
        let centre = header.rect.center();
        let mut last = None;
        // the pointer moves onto the header, then rests there past egui's tooltip delay
        for (i, time) in [0.1, 0.2, 5.0, 10.0].into_iter().enumerate() {
            let mut input = at(time);
            if i == 0 {
                input.events = vec![egui::Event::PointerMoved(centre)];
            }
            last = Some(response(&ctx, input, show).0);
        }
        let out = last.unwrap();
        let painted: Vec<bool> = flat(&out.shapes)
            .into_iter()
            .filter_map(|s| match s {
                egui::Shape::Text(text) if text.galley.text() == title => Some(text.galley.elided),
                _ => None,
            })
            .collect();
        // the header's own title, elided where it is cut, and the tooltip's, never elided
        let want = if tip { vec![true, false] } else { vec![false] };
        assert_eq!(painted, want, "{title}: the title's shapes, elided or not");
    }
}

/// `open` opens or closes the body whatever the pointer does, and `enabled(false)` leaves the
/// header disabled and unclickable: on the painted path, on egui's own and with no atlas.
#[test]
fn the_expander_opens_and_disables_as_told() {
    let own = from_preset("material-3", false, &AccessibilityPreferences::default())
        .unwrap()
        .1;
    let e = &own.expander;
    assert!(
        e.arrow_side.is_none()
            && e.arrow_gap.is_none()
            && e.content_indent.is_none()
            && e.frame_enabled.is_none(),
        "material-3 states none of the painted path's four"
    );
    let contexts = [
        (
            "painted",
            installed(&kde(), &AccessibilityPreferences::default()),
        ),
        ("own", installed(&own, &AccessibilityPreferences::default())),
        ("bare", bare()),
    ];
    for (path, ctx) in contexts {
        let mut time = 0.0;
        let mut show = |open: Option<bool>, enabled: bool, input: Option<(egui::Pos2, bool)>| {
            // Far apart, so each pass finds the last one's animation done; a press and its
            // release close, so egui takes them for a click (`InputOptions::max_click_duration`).
            time += if input.is_some() { 0.1 } else { 10.0 };
            let mut raw = at(time);
            if let Some((pos, pressed)) = input {
                raw.events = pointer(pos, pressed).events;
            }
            let mut got = None;
            let _ = pass(&ctx, raw, |ui| {
                let r = Expander::new("Details")
                    .open(open)
                    .enabled(enabled)
                    .show(ui, |ui| ui.label("Body"));
                got = Some((r.header_response, r.body_returned.is_some()));
            });
            got.unwrap()
        };
        let (header, shown) = show(None, true, None);
        assert!(!shown, "{path}: closed at first");
        let centre = header.rect.center();
        let (header, _) = show(Some(true), true, None);
        assert!(header.changed(), "{path}: open(Some(true)) opens it");
        assert!(show(None, true, None).1, "{path}: and it stays open");
        let _ = show(Some(true), true, Some((centre, true)));
        let (header, _) = show(Some(true), true, Some((centre, false)));
        assert!(!header.changed(), "{path}: a click leaves it open");
        assert!(show(None, true, None).1, "{path}");
        let (header, _) = show(Some(false), true, None);
        assert!(header.changed(), "{path}: open(Some(false)) closes it");
        assert!(!show(None, true, None).1, "{path}: and it stays closed");
        let (header, _) = show(None, false, None);
        assert!(
            !header.enabled(),
            "{path}: enabled(false) disables the header"
        );
        let _ = show(None, false, Some((centre, true)));
        let (header, _) = show(None, false, Some((centre, false)));
        assert!(!header.clicked() && !header.changed(), "{path}");
        assert!(
            !show(None, false, None).1,
            "{path}: a disabled header is not clicked open"
        );
        // egui senses a press on the widgets the pass before laid out, so one enabled first.
        let _ = show(None, true, None);
        let _ = show(None, true, Some((centre, true)));
        let (header, _) = show(None, true, Some((centre, false)));
        assert!(header.changed(), "{path}: an enabled one is");
    }
}

/// T3: a clicked hyperlink is visited on the next pass, in `link.visited_text_color`.
#[test]
fn a_clicked_hyperlink_is_visited() {
    let mut t = kde();
    let visited = egui::Color32::from_rgb(1, 2, 3);
    t.link.visited_text_color = native_theme_egui::native_theme::color::Rgba::new(1, 2, 3, 255);
    let ctx = installed(&t, &AccessibilityPreferences::default());
    let add = |ui: &mut egui::Ui| {
        let w = links::hyperlink(ui, "Docs", "https://example.org");
        ui.add(w)
    };
    let (_, r) = click(&ctx, add);
    assert!(r.clicked());
    // Away from the link, so it is at rest.
    let away = egui::RawInput {
        events: vec![egui::Event::PointerGone],
        focused: true,
        ..Default::default()
    };
    let _ = response(&ctx, away.clone(), add);
    let (out, r) = response(&ctx, away, add);
    assert!(node(&out, r.id).is_visited());
    let colours: Vec<egui::Color32> = flat(&out.shapes)
        .into_iter()
        .filter_map(|s| match s {
            egui::Shape::Text(text) => Some(text.galley.job.sections[0].format.color),
            _ => None,
        })
        .collect();
    assert_eq!(colours, vec![visited]);
}

/// T3: a switch focused with Tab has its track ringed, and no widget paints a ring of its own.
#[test]
fn the_switch_rings_its_track() {
    let t = kde();
    let ctx = installed(&t, &AccessibilityPreferences::default());
    let add = |ui: &mut egui::Ui| ui.add(Switch::new(&mut false).label("A long label"));
    let _ = response(&ctx, at(0.0), add);
    let _ = response(&ctx, tab(), add);
    let (out, r) = response(&ctx, at(0.0), add);
    assert!(r.has_focus());
    let track_fill = to_color32(t.switch.unchecked_background);
    let track = rects(&out)
        .into_iter()
        .find(|s| s.fill == track_fill)
        .unwrap()
        .rect;
    assert!(
        track.width() < r.rect.width(),
        "the track is not the whole switch"
    );
    let ring = focus_ring_color(&t);
    let rings: Vec<egui::Rect> = rects(&out)
        .into_iter()
        .filter(|s| s.stroke.color == ring && s.stroke.width > 0.0)
        .map(|s| s.rect)
        .collect();
    assert_eq!(rings, vec![track.expand(focus_ring_offset(&t))]);
}

// ---- T4: reduced motion ------------------------------------------------------------------------

/// T4: under reduced motion the thumb reaches its end in the pass of the click, and the painted
/// spinner requests no repaint and paints the same on two passes.
#[test]
fn reduced_motion_holds_still() {
    let t = kde();
    let mut prefs = AccessibilityPreferences::default();
    prefs.reduce_motion = true;
    let ctx = installed(&t, &prefs);
    let mut on = false;
    let (out, r) = click(&ctx, |ui| ui.add(Switch::new(&mut on)));
    assert!(r.changed());
    let thumb = to_color32(t.switch.thumb_background);
    let centre = circles(&out)
        .into_iter()
        .find(|c| c.fill == thumb)
        .unwrap()
        .center;
    // Found by its size: the pointer is still on it, so it is hovered.
    let size = egui::vec2(t.switch.track_width, t.switch.track_height);
    let track = rects(&out)
        .into_iter()
        .find(|s| s.rect.size() == size)
        .unwrap()
        .rect;
    // At the right end: half the track's height in, or flush where the thumb is the taller.
    let half = 0.5 * t.switch.track_height.max(t.switch.thumb_diameter);
    assert_eq!(centre.x, track.right() - half);

    let ctx = installed(&t, &prefs);
    // egui repaints its first passes itself (the fonts, the first layout).
    for time in [0.0, 0.1, 0.2] {
        let _ = pass(&ctx, at(time), |ui| {
            ui.add(Spinner::new());
        });
    }
    let a = pass(&ctx, at(0.3), |ui| {
        ui.add(Spinner::new());
    });
    let b = pass(&ctx, at(0.9), |ui| {
        ui.add(Spinner::new());
    });
    assert_eq!(flat(&a.shapes), flat(&b.shapes));
    let delay = b.viewport_output[&egui::ViewportId::ROOT].repaint_delay;
    assert_eq!(delay, std::time::Duration::MAX);
}

/// The painted arc: `spinner.diameter` across its outer edge at `spinner.stroke_width`.
#[test]
fn the_painted_arc_is_the_themes() {
    let t = kde();
    let ctx = installed(&t, &AccessibilityPreferences::default());
    let (out, r) = response(&ctx, at(1.0), |ui| ui.add(Spinner::new()));
    assert_eq!(r.rect.size(), egui::Vec2::splat(t.spinner.diameter));
    let lines: Vec<egui::epaint::PathShape> = flat(&out.shapes)
        .into_iter()
        .filter_map(|s| match s {
            egui::Shape::Path(p) => Some(p),
            _ => None,
        })
        .collect();
    assert_eq!(lines.len(), 1);
    let line = &lines[0];
    assert_eq!(line.stroke.width, t.spinner.stroke_width);
    let radius = 0.5 * (t.spinner.diameter - t.spinner.stroke_width);
    for p in &line.points {
        let d = (*p - r.rect.center()).length();
        assert!((d - radius).abs() < 1.0e-3, "{d} != {radius}");
    }
}

/// The painted arc turns but keeps egui's widest sweep, 240°, at every frame: egui's own
/// `240° · sin(time)` is nothing at time 0, which a still frame catches as a dot.
#[test]
fn the_painted_arc_keeps_its_sweep() {
    let t = kde();
    let ctx = installed(&t, &AccessibilityPreferences::default());
    let span = |time: f64| {
        let (out, r) = response(&ctx, at(time), |ui| ui.add(Spinner::new()));
        let line = flat(&out.shapes)
            .into_iter()
            .find_map(|s| match s {
                egui::Shape::Path(p) => Some(p),
                _ => None,
            })
            .unwrap();
        let angle = |p: egui::Pos2| {
            let v = p - r.rect.center();
            v.y.atan2(v.x)
        };
        let n = line.points.len();
        let step = angle(line.points[1]) - angle(line.points[0]);
        (n, step.rem_euclid(std::f32::consts::TAU))
    };
    let (n, step) = span(0.0);
    let swept = step * n as f32;
    assert!(
        (swept.to_degrees() - 240.0).abs() < 0.1,
        "{}",
        swept.to_degrees()
    );
    for time in [0.25, 0.5, 1.0] {
        let (m, other) = span(time);
        assert_eq!(m, n);
        assert!((other - step).abs() < 1.0e-4, "{time}: {other} != {step}");
    }
}

// ---- T5: disabled ------------------------------------------------------------------------------

/// T5: `.enabled(false)` paints the disabled leaves and takes no click; on kde-breeze, which
/// dims by colour alone, `disabled_opacity` is 1.0 and they are painted as stated.
#[test]
fn a_disabled_widget_paints_its_disabled_leaves() {
    let t = kde();
    let ctx = installed(&t, &AccessibilityPreferences::default());
    let mut on = true;
    let (out, r) = click(&ctx, |ui| ui.add(Switch::new(&mut on).enabled(false)));
    assert!(on && !r.changed());
    let want = to_color32(
        t.switch
            .disabled_checked_background
            .unwrap_or(t.switch.checked_background),
    );
    assert!(rects(&out).iter().any(|s| s.fill == want));
    let thumb = to_color32(
        t.switch
            .disabled_thumb_color
            .unwrap_or(t.switch.thumb_background),
    );
    assert!(circles(&out).iter().any(|c| c.fill == thumb));

    let mut value = 40.0;
    let (out, _) = click(&ctx, |ui| {
        ui.add(Slider::new(&mut value, 0.0..=100.0).enabled(false))
    });
    assert_eq!(value, 40.0);
    let rail = to_color32(
        t.slider
            .disabled_track_color
            .unwrap_or(t.slider.track_color),
    );
    let fill = to_color32(t.slider.disabled_fill_color.unwrap_or(t.slider.fill_color));
    let knob = to_color32(
        t.slider
            .disabled_thumb_color
            .unwrap_or(t.slider.thumb_color),
    );
    assert!(rects(&out).iter().any(|s| s.fill == rail));
    assert!(rects(&out).iter().any(|s| s.fill == fill));
    assert!(circles(&out).iter().any(|c| c.fill == knob));
}

/// The disabled rule (docs/platform-facts.md §2.1.6): adwaita dims by opacity alone, stating
/// no disabled switch colours and `disabled_opacity` 0.5, so a disabled switch is its enabled
/// self faded as one widget over the window, as libadwaita's `filter: Opacity(..)` fades it:
/// the white thumb stays white on the track and the pair fades over the window (light
/// #fcfcfd, where fading each shape on its own gave #cbdff7). A disabled calling `Ui`
/// disables the widget as `.enabled(false)` does, and fades it as one by its own
/// `disabled_alpha`.
#[test]
fn a_disabled_widget_fades_by_its_disabled_opacity() {
    let t = from_preset("adwaita", false, &AccessibilityPreferences::default())
        .unwrap()
        .1;
    assert_eq!(t.switch.disabled_checked_background, None);
    assert_eq!(t.switch.disabled_opacity, 0.5);
    let ctx = installed(&t, &AccessibilityPreferences::default());
    let ground = to_color32(t.defaults.background_color);
    let on_ground = ground.blend(to_color32(t.switch.checked_background));
    let mut on = true;
    let (out, _) = click(&ctx, |ui| ui.add(Switch::new(&mut on).enabled(false)));
    let as_one = ground.lerp_to_gamma(on_ground, 0.5);
    assert!(
        rects(&out).iter().any(|s| s.fill == as_one),
        "{:#?}",
        rects(&out)
    );
    let thumb = ground.lerp_to_gamma(on_ground.blend(to_color32(t.switch.thumb_background)), 0.5);
    assert!(circles(&out).iter().any(|c| c.fill == thumb));
    assert!(
        [thumb.r(), thumb.g(), thumb.b()]
            .iter()
            .zip([0xfc, 0xfc, 0xfd])
            .all(|(got, want)| got.abs_diff(want) <= 1),
        "{thumb:?}"
    );
    let (out, r) = click(&ctx, |ui| {
        ui.add_enabled_ui(false, |ui| ui.add(Switch::new(&mut on)))
            .inner
    });
    assert!(on && !r.changed());
    assert!(
        rects(&out).iter().any(|s| s.fill == as_one),
        "{:#?}",
        rects(&out)
    );
    assert!(circles(&out).iter().any(|c| c.fill == thumb));
}

/// A disabled switch faded as one fades its track's outline over the track it is drawn inside,
/// not over the window: a translucent outline (windows-11's `#00000037`, material-3's
/// `#1d1b201f`) shows the track through it. On adwaita, which fades by 0.5, with an outline no
/// bundled preset states there.
#[test]
fn a_faded_switch_fades_its_outline_over_its_track() {
    let mut t = from_preset("adwaita", false, &AccessibilityPreferences::default())
        .unwrap()
        .1;
    let edge = native_theme_egui::native_theme::color::Rgba::new(0, 0, 0, 0x37);
    t.switch.unchecked_track_border_color = Some(edge);
    t.switch.track_border_line_width = Some(1.0);
    assert_eq!(t.switch.disabled_opacity, 0.5);
    let ctx = installed(&t, &AccessibilityPreferences::default());
    let ground = to_color32(t.defaults.background_color);
    let track = to_color32(
        t.switch
            .disabled_unchecked_background
            .unwrap_or(t.switch.unchecked_background),
    );
    let on_track = ground.blend(track).blend(to_color32(edge));
    let want = ground.lerp_to_gamma(on_track, 0.5);
    let mut on = false;
    let (out, _) = response(&ctx, at(0.0), |ui| {
        ui.add(Switch::new(&mut on).enabled(false))
    });
    let size = egui::vec2(t.switch.track_width, t.switch.track_height);
    let stroke = rects(&out)
        .into_iter()
        .rev()
        .find(|r| r.rect.size() == size)
        .map(|r| r.stroke);
    assert_eq!(stroke, Some(egui::Stroke::new(1.0, want)));
}

/// A thumb outline as wide as the thumb's radius fills the thumb: the thumb is still drawn, a
/// disc of its diameter in the outline's colour (epaint draws no circle of radius 0).
#[test]
fn a_thumb_outline_as_wide_as_its_radius_fills_the_thumb() {
    let mut t = kde();
    let edge = t.defaults.danger_color;
    t.switch.thumb_border_color = Some(edge);
    t.switch.thumb_border_line_width = Some(0.5 * t.switch.thumb_diameter);
    let ctx = installed(&t, &AccessibilityPreferences::default());
    let mut on = true;
    let (out, _) = response(&ctx, at(0.0), |ui| ui.add(Switch::new(&mut on)));
    let thumb = circles(&out)
        .into_iter()
        .find(|c| c.fill == to_color32(edge))
        .map(|c| 2.0 * c.radius);
    assert_eq!(thumb, Some(t.switch.thumb_diameter));
}

/// adwaita light, the opacity-only platform (`disabled_opacity` 0.5, no disabled colour).
fn adwaita() -> ResolvedTheme {
    let t = from_preset("adwaita", false, &AccessibilityPreferences::default())
        .unwrap()
        .1;
    assert_eq!(t.defaults.disabled_opacity, 0.5);
    t
}

/// The disabled rule on a checked radio button: adwaita states no `checkbox.disabled_background`,
/// so a disabled checked radio is its checked self, the checked fill with its
/// `indicator_color` dot, faded as one over the window, not the unchecked circle.
#[test]
fn a_disabled_checked_radio_is_its_checked_self_faded() {
    let t = adwaita();
    assert_eq!(t.checkbox.disabled_background, None);
    assert_eq!(t.checkbox.disabled_opacity, 0.5);
    let ctx = installed(&t, &AccessibilityPreferences::default());
    let ground = to_color32(t.defaults.background_color);
    let fill = ground.blend(to_color32(t.checkbox.checked_background));
    let circle = ground.lerp_to_gamma(fill, 0.5);
    let dot = ground.lerp_to_gamma(fill.blend(to_color32(t.checkbox.indicator_color)), 0.5);
    // Disabled by its own `.enabled(false)`, and under a disabled `Ui`, whose `disabled_alpha`
    // (`defaults.disabled_opacity`, 0.5) it is faded by.
    type Add = Box<dyn Fn(&mut egui::Ui) -> egui::Response>;
    let ways: [Add; 2] = [
        Box::new(|ui| ui.add(RadioButton::new(true, "Radio").enabled(false))),
        Box::new(|ui| ui.add_enabled(false, RadioButton::new(true, "Radio"))),
    ];
    for add in &ways {
        let (out, _) = click(&ctx, add);
        let shapes = circles(&out);
        assert!(shapes.iter().any(|c| c.fill == circle), "{shapes:#?}");
        assert!(shapes.iter().any(|c| c.fill == dot), "{shapes:#?}");
    }

    // kde-breeze dims by colour (`disabled_opacity` 1) and states a disabled fill: the circle
    // takes it and the dot the disabled text colour, painted as stated.
    let t = kde();
    assert_eq!(t.checkbox.disabled_opacity, 1.0);
    let stated = t
        .checkbox
        .disabled_background
        .expect("kde-breeze states it");
    let ctx = installed(&t, &AccessibilityPreferences::default());
    for add in &ways {
        let (out, _) = click(&ctx, add);
        let shapes = circles(&out);
        assert!(
            shapes.iter().any(|c| c.fill == to_color32(stated)),
            "{shapes:#?}"
        );
        assert!(
            shapes
                .iter()
                .any(|c| c.fill == to_color32(t.checkbox.disabled_text_color)),
            "{shapes:#?}"
        );
    }
}

/// A disabled slider is faded as one over its backdrop, as libadwaita fades the whole widget:
/// the rail over the window, the trailing fill over the rail, the knob over the fill, each
/// faded by `slider.disabled_opacity` over the window and painted at opacity 1, so no shape
/// shows another through it.
#[test]
fn a_disabled_slider_fades_as_one() {
    let t = adwaita();
    let sl = &t.slider;
    assert_eq!(sl.disabled_opacity, 0.5);
    assert_eq!(sl.disabled_fill_color, None);
    let ctx = installed(&t, &AccessibilityPreferences::default());
    let ground = to_color32(t.defaults.background_color);
    let rail = ground.blend(to_color32(sl.track_color));
    let fill = rail.blend(to_color32(sl.fill_color));
    let knob = ground.lerp_to_gamma(ground.blend(to_color32(sl.thumb_color)), 0.5);
    // The knob's outline: the slider's `Disabled` cell's `inactive` stroke, over the fill.
    let mut outline = egui::Stroke::NONE;
    let _ = response(&ctx, at(0.0), |ui| {
        use native_theme_egui::NativeThemeUiExt as _;
        ui.native_scope(
            native_theme_egui::Role::Slider,
            native_theme_egui::RoleVariant::Disabled,
            |ui| outline = ui.visuals().widgets.inactive.fg_stroke,
        )
        .response
    });
    assert!(outline.width > 0.0);
    let ring = ground.lerp_to_gamma(ground.blend(outline.color), 0.5);
    // Disabled by its own `.enabled(false)`, and under a disabled `Ui`, whose `disabled_alpha`
    // (`defaults.disabled_opacity`, 0.5) it is faded by.
    type Add = Box<dyn Fn(&mut egui::Ui) -> egui::Response>;
    let ways: [Add; 2] = [
        Box::new(|ui| ui.add(Slider::new(&mut 40.0, 0.0..=100.0).enabled(false))),
        Box::new(|ui| ui.add_enabled(false, Slider::new(&mut 40.0, 0.0..=100.0))),
    ];
    for add in &ways {
        let (out, _) = click(&ctx, add);
        let all = rects(&out);
        for want in [
            ground.lerp_to_gamma(rail, 0.5),
            ground.lerp_to_gamma(fill, 0.5),
        ] {
            assert!(all.iter().any(|s| s.fill == want), "{want:?} in {all:#?}");
        }
        let disc = circles(&out)
            .into_iter()
            .find(|c| c.fill == knob)
            .unwrap_or_else(|| panic!("{:#?}", circles(&out)));
        assert_eq!(disc.stroke, egui::Stroke::new(outline.width, ring));
    }

    // A backdrop of the caller's is the ground instead.
    let card = to_color32(t.card.background_color);
    assert_ne!(card, ground);
    let rail = card.blend(to_color32(sl.track_color));
    let (out, _) = click(&ctx, |ui| {
        ui.add(
            Slider::new(&mut 40.0, 0.0..=100.0)
                .enabled(false)
                .backdrop(card),
        )
    });
    assert!(
        rects(&out)
            .iter()
            .any(|s| s.fill == card.lerp_to_gamma(rail, 0.5))
    );
}

/// A translucent knob is taken over the backdrop it is faded against, where most of it lies (it
/// is taller than the rail), not over the trailing fill: macos-tahoe's dark `thumb_color`
/// `#ffffffd9`, with the slider's `disabled_opacity` lowered to 0.5 here, since the preset dims
/// a slider by colour alone.
#[test]
fn a_disabled_translucent_knob_fades_over_the_backdrop() {
    let mut t = from_preset("macos-tahoe", true, &AccessibilityPreferences::default())
        .unwrap()
        .1;
    let sl = &t.slider;
    assert_eq!(sl.disabled_thumb_color, None);
    assert!(
        sl.thumb_color.a < 255,
        "macos-tahoe's dark knob is translucent"
    );
    assert!(sl.thumb_diameter > sl.track_height);
    t.slider.disabled_opacity = 0.5;
    let ctx = installed(&t, &AccessibilityPreferences::default());
    let ground = to_color32(t.defaults.background_color);
    let knob = ground.lerp_to_gamma(ground.blend(to_color32(t.slider.thumb_color)), 0.5);
    let (out, _) = click(&ctx, |ui| {
        ui.add(Slider::new(&mut 40.0, 0.0..=100.0).enabled(false))
    });
    assert!(
        circles(&out).iter().any(|c| c.fill == knob),
        "{knob:?} in {:#?}",
        circles(&out)
    );
}

/// A segmented control under a disabled `Ui` is faded as one over its backdrop: each segment's
/// fill over the outline's, faded over the window, painted at opacity 1, so the outline's
/// colour does not show through a segment.
#[test]
fn a_disabled_segmented_control_fades_as_one() {
    let t = adwaita();
    let ctx = installed(&t, &AccessibilityPreferences::default());
    let ground = to_color32(t.defaults.background_color);
    let mut cell = None;
    let (out, control) = response(&ctx, at(0.0), |ui| {
        use native_theme_egui::NativeThemeUiExt as _;
        cell = Some(
            ui.native_scope(
                native_theme_egui::Role::SegmentedControl,
                native_theme_egui::RoleVariant::Normal,
                |ui| ui.visuals().clone(),
            )
            .inner,
        );
        ui.add_enabled(false, SegmentedControl::new(&mut 0, ["Day", "Week"]))
    });
    let v = cell.unwrap();
    let alpha = v.disabled_alpha();
    assert!(alpha < 1.0);
    let outline = ground.blend(v.widgets.inactive.bg_stroke.color);
    let segment = v.widgets.inactive.weak_bg_fill;
    assert_eq!(segment.a(), 255, "the segment fill is opaque on adwaita");
    let fill_of = |out: &egui::FullOutput, control: &egui::Response, name: &str| {
        let rect = match name {
            "outline" => control.rect,
            segment => Parts::of(control).unwrap().get(segment).unwrap(),
        };
        rects(out)
            .into_iter()
            .find(|s| s.rect == rect)
            .map(|s| s.fill)
            .unwrap()
    };
    assert_eq!(
        fill_of(&out, &control, "segment_1"),
        ground.lerp_to_gamma(segment, alpha)
    );
    assert_eq!(
        fill_of(&out, &control, "segment_0"),
        ground.lerp_to_gamma(outline.blend(v.selection.bg_fill), alpha)
    );
    assert_eq!(
        fill_of(&out, &control, "outline"),
        ground.lerp_to_gamma(outline, alpha)
    );
    // The labels, each over its segment: the unselected one in the scope's text colour, the
    // selected one in the selection's.
    let label = |text: &str| {
        flat(&out.shapes)
            .into_iter()
            .find_map(|s| match s {
                egui::Shape::Text(t) if t.galley.text() == text => Some(t.fallback_color),
                _ => None,
            })
            .unwrap()
    };
    let text = v
        .override_text_color
        .unwrap_or(v.widgets.inactive.fg_stroke.color);
    assert_eq!(
        label("Week"),
        ground.lerp_to_gamma(outline.blend(segment).blend(text), alpha)
    );
    let selected = outline.blend(v.selection.bg_fill);
    assert_eq!(
        label("Day"),
        ground.lerp_to_gamma(selected.blend(v.selection.stroke.color), alpha)
    );

    // A backdrop of the caller's is the ground instead.
    let card = to_color32(t.card.background_color);
    assert_ne!(card, ground);
    let (out, control) = response(&ctx, at(0.0), |ui| {
        ui.add_enabled(
            false,
            SegmentedControl::new(&mut 0, ["Day", "Week"]).backdrop(card),
        )
    });
    let outline = card.blend(v.widgets.inactive.bg_stroke.color);
    assert_eq!(
        fill_of(&out, &control, "outline"),
        card.lerp_to_gamma(outline, alpha)
    );
    assert_eq!(
        fill_of(&out, &control, "segment_1"),
        card.lerp_to_gamma(segment, alpha)
    );
}

/// The progress bar is outlined as `progress_bar.border` states: kde-breeze's 1px in the
/// window text at 20 % inside the bar, adwaita's width 0 not at all.
#[test]
fn the_progress_bar_is_outlined_as_stated() {
    let t = kde();
    assert!(t.progress_bar.border.line_width > 0.0);
    let ctx = installed(&t, &AccessibilityPreferences::default());
    let add = |ui: &mut egui::Ui| ui.add(ProgressBar::new(0.4).desired_width(200.0));
    let (out, bar) = response(&ctx, at(0.0), add);
    assert_eq!(bar.rect.height(), t.progress_bar.track_height);
    let outline: Vec<_> = rects(&out)
        .into_iter()
        .filter(|r| r.stroke.width > 0.0)
        .collect();
    assert_eq!(outline.len(), 1, "{outline:#?}");
    assert_eq!(outline[0].rect, bar.rect);
    assert_eq!(
        outline[0].stroke,
        egui::Stroke::new(
            t.progress_bar.border.line_width,
            to_color32(t.progress_bar.border.color)
        )
    );
    assert!(
        rects(&out)
            .iter()
            .any(|r| r.fill == to_color32(t.progress_bar.fill_color))
    );

    let t = from_preset("adwaita", false, &AccessibilityPreferences::default())
        .unwrap()
        .1;
    assert_eq!(t.progress_bar.border.line_width, 0.0);
    let ctx = installed(&t, &AccessibilityPreferences::default());
    let (out, _) = response(&ctx, at(0.0), add);
    assert!(rects(&out).iter().all(|r| r.stroke.width == 0.0));
}

/// With no `desired_width`, the bar is the `Ui`'s available width, at least
/// `progress_bar.min_width`, where egui's own floor is 96.
#[test]
fn the_progress_bar_is_at_least_its_min_width() {
    for (preset, want) in [("macos-monterey", 100.0), ("kde-breeze", 50.0)] {
        let t = from_preset(preset, false, &AccessibilityPreferences::default())
            .unwrap()
            .1;
        let ctx = installed(&t, &AccessibilityPreferences::default());
        let column = 50.0;
        assert_eq!(want, t.progress_bar.min_width.max(column), "{preset}");
        let (_, bar) = response(&ctx, at(0.0), |ui| {
            ui.allocate_ui(egui::vec2(column, f32::INFINITY), |ui| {
                ui.add(ProgressBar::new(0.4))
            })
            .inner
        });
        assert_eq!(bar.rect.width(), want, "{preset}");
    }
}

/// `animate(true)` is egui's own busy bar: under the stated radius, its fill pulses, as egui
/// draws it, and it repaints; under reduced motion it holds still.
#[test]
fn the_animated_progress_bar_pulses_unless_motion_is_reduced() {
    let t = kde();
    let fill = to_color32(t.progress_bar.fill_color);
    let mut still = AccessibilityPreferences::default();
    still.reduce_motion = true;
    for (prefs, animate, pulses) in [
        (AccessibilityPreferences::default(), true, true),
        (AccessibilityPreferences::default(), false, false),
        (still, true, false),
    ] {
        let ctx = installed(&t, &prefs);
        let add =
            |ui: &mut egui::Ui| ui.add(ProgressBar::new(0.4).desired_width(200.0).animate(animate));
        // egui repaints its first passes itself (the fonts, the first layout).
        for time in [0.0, 0.1, 0.2] {
            let _ = response(&ctx, at(time), add);
        }
        // A quarter turn of egui's clock, where its pulse is darkest (`time.cos().abs()`).
        let (out, bar) = response(&ctx, at(std::f64::consts::FRAC_PI_2), add);
        let filled = Parts::of(&bar).unwrap().get("fill").unwrap();
        let painted = rects(&out)
            .into_iter()
            .find(|r| r.rect == filled)
            .unwrap()
            .fill;
        assert_eq!(painted != fill, pulses, "{animate} {prefs:?}");
        let delay = out.viewport_output[&egui::ViewportId::ROOT].repaint_delay;
        assert_eq!(
            delay == std::time::Duration::ZERO,
            pulses,
            "{animate} {prefs:?}"
        );
    }
}

/// The slider's rail, fill and knob are the theme's, the knob's hover a layer over it.
#[test]
fn the_slider_paints_the_themes_knob() {
    let t = kde();
    let ctx = installed(&t, &AccessibilityPreferences::default());
    let mut value = 50.0;
    let (out, r) = response(&ctx, at(0.0), |ui| {
        ui.add(Slider::new(&mut value, 0.0..=100.0))
    });
    assert_eq!(
        r.rect.height(),
        t.slider.thumb_diameter.max(t.slider.track_height)
    );
    let rail = rects(&out)
        .into_iter()
        .find(|s| s.fill == to_color32(t.slider.track_color))
        .unwrap();
    assert_eq!(rail.rect.height(), t.slider.track_height);
    assert_ne!(t.slider.thumb_color, t.slider.track_color);
    let knob = circles(&out)
        .into_iter()
        .find(|c| c.fill == to_color32(t.slider.thumb_color))
        .unwrap();
    assert_eq!(knob.center.x, r.rect.center().x);
    assert_eq!(
        2.0 * (knob.radius + knob.stroke.width),
        t.slider.thumb_diameter
    );
    // Hovered: the hover colour over the knob's.
    let hover = egui::RawInput {
        events: vec![egui::Event::PointerMoved(knob.center)],
        focused: true,
        ..Default::default()
    };
    let (out, _) = response(&ctx, hover, |ui| {
        ui.add(Slider::new(&mut value, 0.0..=100.0))
    });
    let want = match t.slider.thumb_hover_color {
        Some(layer) => composite_over(layer, t.slider.thumb_color),
        None => to_color32(t.slider.thumb_color),
    };
    assert!(circles(&out).iter().any(|c| c.fill == want));
    // A drag to the rail's end sets the range's end.
    let end = egui::pos2(r.rect.right(), r.rect.center().y);
    let _ = response(&ctx, pointer(r.rect.center(), true), |ui| {
        ui.add(Slider::new(&mut value, 0.0..=100.0))
    });
    let moved = egui::RawInput {
        events: vec![egui::Event::PointerMoved(end)],
        focused: true,
        ..Default::default()
    };
    let _ = response(&ctx, moved, |ui| {
        ui.add(Slider::new(&mut value, 0.0..=100.0))
    });
    assert_eq!(value, 100.0);
}

// ---- T6: no hardcoded values -------------------------------------------------------------------

/// The crate's sources outside its tests, comments and string literals blanked.
fn painting_sources() -> Vec<(String, String)> {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut out = Vec::new();
    for entry in std::fs::read_dir(&dir).unwrap() {
        let path = entry.unwrap().path();
        let name = path.file_name().unwrap().to_string_lossy().to_string();
        if name == "tests.rs" {
            continue;
        }
        let raw = std::fs::read_to_string(&path).unwrap();
        let code: Vec<&str> = rendered_lines(&raw).into_iter().map(|(_, l)| l).collect();
        out.push((name, blank(&code.join("\n"))));
    }
    out
}

/// `code` with every `//` comment and every string literal replaced by spaces.
fn blank(code: &str) -> String {
    let mut out = String::new();
    for line in code.lines() {
        let mut in_string = false;
        let mut escaped = false;
        let mut chars = line.chars().peekable();
        while let Some(c) = chars.next() {
            if in_string {
                if escaped {
                    escaped = false;
                } else if c == '\\' {
                    escaped = true;
                } else if c == '"' {
                    in_string = false;
                }
                out.push(' ');
            } else if c == '"' {
                in_string = true;
                out.push(' ');
            } else if c == '/' && chars.peek() == Some(&'/') {
                break;
            } else {
                out.push(c);
            }
        }
        out.push('\n');
    }
    out
}

/// Every numeric literal in `code`: a digit that does not continue an identifier, with the
/// digits, `.` and `_` after it.
fn literals(code: &str) -> Vec<String> {
    let chars: Vec<char> = code.chars().collect();
    let mut out = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        let starts = c.is_ascii_digit()
            && (i == 0 || !(chars[i - 1].is_alphanumeric() || chars[i - 1] == '_'));
        if starts {
            let mut j = i;
            while j < chars.len()
                && (chars[j].is_ascii_digit()
                    || chars[j] == '_'
                    || (chars[j] == '.' && chars.get(j + 1).is_some_and(char::is_ascii_digit)))
            {
                j += 1;
            }
            out.push(chars[i..j].iter().collect());
            i = j;
        } else if c.is_alphanumeric() || c == '_' {
            while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
        } else {
            i += 1;
        }
    }
    out
}

/// T6: the only numeric literals are §2.3's: `0.5`, the identities `0` `0.0` `1.0`, egui's
/// spinner constants `240` `8` `128`, egui's slider key step `1.0`, egui's combo-box arrow
/// proportions `0.7` `0.45`, egui's radio dot divisor `3.0` and egui's collapsing-header
/// arrow share `0.75`.
#[test]
fn the_crate_hardcodes_no_values() {
    let allowed = [
        "0", "0.0", "0.5", "1.0", "240.0", "8", "128", "0.7", "0.45", "3.0", "0.75",
    ];
    let mut found = Vec::new();
    for (file, code) in painting_sources() {
        for literal in literals(&code) {
            if !allowed.contains(&literal.as_str()) {
                found.push(format!("{file}: {literal}"));
            }
        }
    }
    assert!(found.is_empty(), "{found:#?}");
    assert_eq!(literals("x2 1.5 f32 a_1 0..8"), vec!["1.5", "0", "8"]);
}

// ---- T7: the Tier P promotions are still justified --------------------------------------------

/// egui's `src/` directory, from `cargo metadata` (never a registry path or a version literal).
fn egui_src() -> PathBuf {
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let out = std::process::Command::new(cargo)
        .args(["metadata", "--format-version", "1"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .unwrap();
    assert!(out.status.success());
    let meta: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    let manifest = meta["packages"]
        .as_array()
        .unwrap()
        .iter()
        .find(|p| p["name"] == "egui")
        .unwrap()["manifest_path"]
        .as_str()
        .unwrap()
        .to_string();
    Path::new(&manifest).parent().unwrap().join("src")
}

/// T7: each Tier P citation still describes a hardcoded upstream fact; a failure demotes the
/// widget (§1.5).
#[test]
fn the_painted_widgets_are_still_needed() {
    let widgets = egui_src().join("widgets");
    let names: Vec<String> = std::fs::read_dir(&widgets)
        .unwrap()
        .map(|e| e.unwrap().file_name().to_string_lossy().to_lowercase())
        .collect();
    assert!(names.iter().any(|n| n == "checkbox.rs"), "{names:?}");
    assert!(
        !names
            .iter()
            .any(|n| n.contains("switch") || n.contains("toggle")),
        "egui has a switch now: {names:?}"
    );
    let spinner = std::fs::read_to_string(widgets.join("spinner.rs")).unwrap();
    assert!(spinner.contains("Stroke::new(3.0, color)"));
    assert!(spinner.contains("- 2.0;"));
    let slider = std::fs::read_to_string(widgets.join("slider.rs")).unwrap();
    assert!(
        slider.contains("rect_filled(rail_rect, corner_radius, widget_visuals.inactive.bg_fill)")
    );
    assert!(slider.contains("let visuals = ui.style().interact(response);"));
    assert!(slider.contains("fill: visuals.bg_fill,"));
    // The drop-down's per-instance change: egui's arrow box is square and raises the content
    // to its height; its arrow is the triangle `ComboBox` repaints in the stated box.
    let combo =
        std::fs::read_to_string(egui_src().join("containers").join("combo_box.rs")).unwrap();
    assert!(combo.contains("let icon_size = Vec2::splat(ui.spacing().icon_width);"));
    assert!(combo.contains("let actual_height = galley.size().y.max(icon_size.y);"));
    assert!(combo.contains("vec2(rect.width() * 0.7, rect.height() * 0.45),"));
    // The radio button's per-instance change: egui's dot is a third of `icon_width_inner`.
    let radio = std::fs::read_to_string(widgets.join("radio_button.rs")).unwrap();
    assert!(radio.contains("radius: small_icon_rect.width() / 3.0,"));
    // The expander: egui's header puts its title and its arrow by one `indent`, the arrow
    // always first and three quarters of its box; its body is `indent` in.
    let collapsing =
        std::fs::read_to_string(egui_src().join("containers").join("collapsing_header.rs"))
            .unwrap_or_default();
    assert!(collapsing.contains("let text_pos = available.min + vec2(ui.spacing().indent, 0.0);"));
    assert!(collapsing.contains("header_response.rect.left() + ui.spacing().indent / 2.0,"));
    assert!(collapsing.contains(&format!(
        "vec2(rect.width(), rect.height()) * {});",
        crate::expander::EGUI_ARROW_SHARE
    )));
    assert!(collapsing.contains("ui.indent(id, |ui| {"));
    // The progress bar's outline: egui's paints only filled rectangles, no stroke round the bar.
    let progress = std::fs::read_to_string(widgets.join("progress_bar.rs")).unwrap();
    assert!(!progress.contains("rect_stroke"));
    assert!(
        progress.contains(".rect_filled(outer_rect, corner_radius, visuals.extreme_bg_color);")
    );
}

// ---- T8: Tier C stays cheap --------------------------------------------------------------------

/// T8: the segmented control allocates and senses nothing itself; every segment is a `Button`.
#[test]
fn the_segmented_control_is_composed() {
    let raw = std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("src/segmented_control.rs"),
    )
    .unwrap();
    let code = blank(&raw);
    for banned in [
        "allocate_response",
        "allocate_exact_size",
        "Sense::",
        "painter(",
    ] {
        assert!(!code.contains(banned), "{banned}");
    }
    assert!(code.contains("egui::Button::new(label).selected("));
}

/// The segments are `separator_width` apart, each `segment_height` tall.
#[test]
fn the_segments_are_separator_width_apart() {
    let t = kde();
    let ctx = installed(&t, &AccessibilityPreferences::default());
    let mut buttons = Vec::new();
    let _ = pass(&ctx, at(0.0), |ui| {
        let _ = ui.add(SegmentedControl::new(&mut 0, ["Day", "Week", "Month"]));
    });
    let out = pass(&ctx, at(0.0), |ui| {
        let _ = ui.add(SegmentedControl::new(&mut 0, ["Day", "Week", "Month"]));
    });
    let update = out.platform_output.accesskit_update.as_ref().unwrap();
    for (_, node) in &update.nodes {
        if node.role() == accesskit::Role::RadioButton {
            buttons.push(node.bounds().unwrap());
        }
    }
    buttons.sort_by(|a, b| a.x0.total_cmp(&b.x0));
    assert_eq!(buttons.len(), 3);
    for pair in buttons.windows(2) {
        assert_eq!(
            pair[1].x0 - pair[0].x1,
            f64::from(t.segmented_control.separator_width)
        );
    }
    // kde-breeze states no segment padding: the scope's, the button's, is narrowed until the
    // segment is `segment_height` tall, within egui's layout rounding.
    assert!(t.segmented_control.border.padding.top.is_none());
    for b in &buttons {
        let height = b.y1 - b.y0;
        let want = f64::from(t.segmented_control.segment_height);
        assert!((height - want).abs() < 0.5, "{height} != {want}");
    }
}

/// The segments are joined: one outline in the scope's border colour, as wide as the border
/// all round, its fill showing between the segments; no segment strokes itself, and only the
/// outer corners are rounded, to the outline's inner radius.
#[test]
fn the_segments_are_joined() {
    let t = kde();
    let ctx = installed(&t, &AccessibilityPreferences::default());
    let add = |ui: &mut egui::Ui| ui.add(SegmentedControl::new(&mut 1, ["Day", "Week", "Month"]));
    let _ = response(&ctx, at(0.0), add);
    let mut border = egui::Stroke::NONE;
    let mut radius = egui::CornerRadius::ZERO;
    let (out, control) = response(&ctx, at(0.0), |ui| {
        use native_theme_egui::NativeThemeUiExt as _;
        let idle = ui
            .native_scope(
                native_theme_egui::Role::SegmentedControl,
                native_theme_egui::RoleVariant::Normal,
                |ui| ui.visuals().widgets.inactive,
            )
            .inner;
        border = idle.bg_stroke;
        radius = idle.corner_radius;
        add(ui)
    });
    assert!(border.width > 0.0);
    let all = rects(&out);
    let outline: Vec<_> = all.iter().filter(|r| r.fill == border.color).collect();
    assert_eq!(outline.len(), 1, "{all:#?}");
    assert_eq!(outline[0].rect, control.rect);
    assert_eq!(outline[0].corner_radius, radius);
    let segments: Vec<_> = all.iter().filter(|r| r.fill != border.color).collect();
    assert_eq!(segments.len(), 3, "{all:#?}");
    let inner = control.rect.shrink(border.width);
    let inset = native_theme_egui::convert::u8_from_f32_saturating(border.width);
    for (i, s) in segments.iter().enumerate() {
        assert_eq!(s.stroke.width, 0.0, "segment {i}");
        assert_eq!(s.rect.top(), inner.top(), "segment {i}");
        assert_eq!(s.rect.bottom(), inner.bottom(), "segment {i}");
        let c = s.corner_radius;
        let (left, right) = (i == 0, i == 2);
        assert_eq!(
            c.nw,
            if left { radius.nw - inset } else { 0 },
            "segment {i}"
        );
        assert_eq!(
            c.sw,
            if left { radius.sw - inset } else { 0 },
            "segment {i}"
        );
        assert_eq!(
            c.ne,
            if right { radius.ne - inset } else { 0 },
            "segment {i}"
        );
        assert_eq!(
            c.se,
            if right { radius.se - inset } else { 0 },
            "segment {i}"
        );
    }
    assert_eq!(segments[0].rect.left(), inner.left());
    assert_eq!(segments[2].rect.right(), inner.right());
    for pair in segments.windows(2) {
        assert_eq!(
            pair[1].rect.left() - pair[0].rect.right(),
            t.segmented_control.separator_width
        );
    }
}

/// Two segments of different heights in one centred row are one row: a divider between them.
#[test]
fn segments_of_two_heights_share_a_row() {
    let t = kde();
    let ctx = installed(&t, &AccessibilityPreferences::default());
    // A label twice the segment's height, so its segment is taller than the other.
    let tall = egui::RichText::new("Week").size(2.0 * t.segmented_control.segment_height);
    let labels = [egui::WidgetText::from("Day"), tall.into()];
    for wrap in [false, true] {
        let (_, control) = response(&ctx, at(0.0), |ui| {
            let mut selected = 0;
            let control = SegmentedControl::new(&mut selected, labels.clone());
            ui.add(if wrap { control.wrap() } else { control })
        });
        let parts = Parts::of(&control).unwrap();
        let (day, week) = (
            parts.get("segment_0").unwrap(),
            parts.get("segment_1").unwrap(),
        );
        assert!(day.height() < week.height(), "{wrap}: {day:?} {week:?}");
        assert!(parts.get("divider_0").is_some(), "{wrap}: {parts:?}");
    }
}

/// A wrapped control in a `Ui` that is not shown asks egui for no discarded pass, as egui's
/// `Grid` asks for none there; and where egui declines the discard (`max_passes` 1), it asks
/// for a repaint, so the next pass corrects the corners it guessed.
#[test]
fn a_wrapped_control_corrects_its_guess_once_shown() {
    let t = from_preset("adwaita", false, &AccessibilityPreferences::default())
        .unwrap()
        .1;
    let labels = ["Monday", "Tuesday", "Wed", "Thu"];
    let narrow = |ui: &mut egui::Ui, width: f32| {
        ui.allocate_ui(egui::vec2(width, f32::INFINITY), |ui| {
            ui.add(SegmentedControl::new(&mut 0, labels).wrap())
        })
        .inner
    };
    let ctx = installed(&t, &AccessibilityPreferences::default());
    let (_, unwrapped) = response(&ctx, at(0.0), |ui| {
        ui.add(SegmentedControl::new(&mut 0, labels))
    });
    let parts = Parts::of(&unwrapped).unwrap();
    let (first, second) = (
        parts.get("segment_1").unwrap(),
        parts.get("segment_2").unwrap(),
    );
    let width = first.right() - unwrapped.rect.left() + 0.5 * second.width();

    let ctx = installed(&t, &AccessibilityPreferences::default());
    let (out, _) = response(&ctx, at(0.0), |ui| {
        ui.scope_builder(egui::UiBuilder::new().invisible(), |ui| narrow(ui, width))
            .inner
    });
    assert_eq!(out.platform_output.num_completed_passes, 1, "not shown");

    let ctx = installed(&t, &AccessibilityPreferences::default());
    ctx.options_mut(|o| o.max_passes = std::num::NonZeroUsize::MIN);
    // A wide `Ui` first, until egui's own first-pass repaints are over: one row.
    for time in [0.0, 0.1, 0.2, 0.3] {
        let _ = response(&ctx, at(time), |ui| narrow(ui, 2.0 * width));
    }
    let (out, _) = response(&ctx, at(0.4), |ui| narrow(ui, 2.0 * width));
    let delay = |out: &egui::FullOutput| out.viewport_output[&egui::ViewportId::ROOT].repaint_delay;
    assert_eq!(delay(&out), std::time::Duration::MAX, "one row, settled");
    // Narrowed, it wraps: the guess (one row) is wrong, the discard declined, a repaint asked.
    let (out, _) = response(&ctx, at(0.5), |ui| narrow(ui, width));
    assert_eq!(out.platform_output.num_completed_passes, 1);
    assert_eq!(
        delay(&out),
        std::time::Duration::ZERO,
        "a repaint corrects it"
    );
    let (out, control) = response(&ctx, at(0.6), |ui| narrow(ui, width));
    let border = to_color32(t.segmented_control.border.color);
    let corrected = rects(&out)
        .iter()
        .filter(|r| r.fill != border)
        .any(|r| r.corner_radius.ne > 0 && r.rect.top() < control.rect.center().y);
    assert!(
        corrected,
        "the first row's last segment rounds its top-right corner"
    );
}

/// A wrapped control rounds the corners its segments share with the outline, row by row, from
/// its first pass on (one discarded pass to learn the rows), and records a divider only
/// between two segments of one row.
#[test]
fn a_wrapped_control_rounds_its_corners_per_row() {
    use native_theme_egui::NativeThemeUiExt as _;

    let t = from_preset("adwaita", false, &AccessibilityPreferences::default())
        .unwrap()
        .1;
    let labels = ["Monday", "Tuesday", "Wed", "Thu"];
    // A column as wide as the first two segments and the outline: the last two wrap.
    let ctx = installed(&t, &AccessibilityPreferences::default());
    let (_, unwrapped) = response(&ctx, at(0.0), |ui| {
        ui.add(SegmentedControl::new(&mut 0, labels))
    });
    let parts = Parts::of(&unwrapped).unwrap();
    let (first, second, third) = (
        parts.get("segment_1").unwrap(),
        parts.get("segment_2").unwrap(),
        parts.get("segment_3").unwrap(),
    );
    let margin = unwrapped.rect.right() - third.right();
    let width = first.right() - unwrapped.rect.left() + margin + 0.5 * second.width();
    let add = |ui: &mut egui::Ui| {
        ui.allocate_ui(egui::vec2(width, f32::INFINITY), |ui| {
            ui.add(SegmentedControl::new(&mut 0, labels).wrap())
        })
        .inner
    };

    let ctx = installed(&t, &AccessibilityPreferences::default());
    let mut border = egui::Stroke::NONE;
    let mut radius = egui::CornerRadius::ZERO;
    let mut add_reading = |ui: &mut egui::Ui| {
        let idle = ui
            .native_scope(
                native_theme_egui::Role::SegmentedControl,
                native_theme_egui::RoleVariant::Normal,
                |ui| ui.visuals().widgets.inactive,
            )
            .inner;
        border = idle.bg_stroke;
        radius = idle.corner_radius;
        add(ui)
    };
    let first_pass = response(&ctx, at(0.0), &mut add_reading);
    let later_pass = response(&ctx, at(0.0), &mut add_reading);
    assert!(border.width > 0.0 && radius.nw > 0);
    let inset = native_theme_egui::convert::u8_from_f32_saturating(border.width);
    let inner = |r: u8| r.saturating_sub(inset);
    for (pass_name, (out, control)) in [("first", first_pass), ("later", later_pass)] {
        let all = rects(&out);
        let segments: Vec<_> = all.iter().filter(|r| r.fill != border.color).collect();
        assert_eq!(segments.len(), 4, "{pass_name}: {all:#?}");
        let s: Vec<egui::Rect> = segments.iter().map(|r| r.rect).collect();
        assert_eq!(s[0].top(), s[1].top(), "{pass_name}: one row");
        assert_eq!(s[2].top(), s[3].top(), "{pass_name}: one row");
        assert!(s[2].top() > s[0].bottom(), "{pass_name}: the last two wrap");
        let outline = control.rect.shrink(border.width);
        assert_eq!(s[1].right(), outline.right(), "{pass_name}");
        assert!(s[3].right() < outline.right() - 1.0, "{pass_name}");
        let want = [
            egui::CornerRadius {
                nw: inner(radius.nw),
                ..egui::CornerRadius::ZERO
            },
            egui::CornerRadius {
                ne: inner(radius.ne),
                ..egui::CornerRadius::ZERO
            },
            egui::CornerRadius {
                sw: inner(radius.sw),
                ..egui::CornerRadius::ZERO
            },
            egui::CornerRadius::ZERO,
        ];
        for (i, (segment, want)) in segments.iter().zip(want).enumerate() {
            assert_eq!(segment.corner_radius, want, "{pass_name}: segment {i}");
        }
        let parts = Parts::of(&control).unwrap();
        for (name, rect) in parts.iter() {
            assert!(
                !name.starts_with("divider_") || rect.min.x <= rect.max.x,
                "{pass_name}: {name} {rect:?}"
            );
        }
        assert!(parts.get("divider_0").is_some(), "{pass_name}");
        assert_eq!(
            parts.get("divider_1"),
            None,
            "{pass_name}: no divider at the wrap"
        );
        assert!(parts.get("divider_2").is_some(), "{pass_name}");
        // The first pass guesses one row and is discarded; once the rows are known, none is.
        let passes = if pass_name == "first" { 2 } else { 1 };
        assert_eq!(
            out.platform_output.num_completed_passes, passes,
            "{pass_name}"
        );
    }
}

/// The drop-down is as tall as its text and padding, at least `combo_box.min_height`, where
/// egui's square arrow box would make it taller; the arrow keeps `arrow_icon_size`'s box, its
/// column the stated width. kde-breeze's 10px arrow is shorter than its line, so the case is
/// an arrow box taller than the text: 20px over the 18px line.
#[test]
fn the_drop_down_is_as_tall_as_its_text() {
    let mut t = kde();
    t.combo_box.arrow_icon_size = 20.0;
    let ctx = installed(&t, &AccessibilityPreferences::default());
    let add = |ui: &mut egui::Ui| {
        ComboBox::from_id_salt("fruit")
            .selected_text("Apple")
            .width(140.0)
            .show_ui(ui, |ui| ui.label("Banana"))
            .response
    };
    let _ = response(&ctx, at(0.0), add);
    let mut line = 0.0;
    let mut padding = egui::Vec2::ZERO;
    let (out, combo) = response(&ctx, at(0.0), |ui| {
        use native_theme_egui::NativeThemeUiExt as _;
        (line, padding) = ui
            .native_scope(
                native_theme_egui::Role::ComboBox,
                native_theme_egui::RoleVariant::Normal,
                |ui| {
                    let galley = egui::WidgetText::from("Apple").into_galley(
                        ui,
                        Some(egui::TextWrapMode::Extend),
                        f32::INFINITY,
                        egui::TextStyle::Button,
                    );
                    (galley.size().y, ui.spacing().button_padding)
                },
            )
            .inner;
        add(ui)
    });
    let arrow = t.combo_box.arrow_icon_size;
    assert!(line < arrow, "the case this covers: {line} < {arrow}");
    let want = (line + 2.0 * padding.y).max(t.combo_box.min_height);
    let height = combo.rect.height();
    assert!((height - want).abs() < 0.5, "{height} != {want}");
    assert!((height - t.combo_box.min_height).abs() < 0.5, "{height}");
    let a = chevron(&out);
    assert!((a.width() - 0.7 * arrow).abs() < 1e-3, "{a:?}");
    assert!((a.height() - 0.45 * arrow).abs() < 1e-3, "{a:?}");
    // The arrow is centred in its `arrow_area_width` column, which ends the border's width and
    // `border.padding.right` in from the frame's right edge (Breeze's
    // `MenuButton_IndicatorWidth` column, docs/platform-facts.md §2.24); the text starts
    // `border.padding.left` inside the border.
    let c = &t.combo_box;
    let (right, column, left) = (
        c.border.padding.right,
        c.arrow_area_width,
        c.border.padding.left,
    );
    assert!(
        right.is_some() && column.is_some() && left.is_some(),
        "kde states the drop-down's side paddings and arrow column"
    );
    if let (Some(right), Some(column)) = (right, column) {
        let centre = combo.rect.right() - c.border.line_width - right - 0.5 * column;
        assert!((a.center().x - centre).abs() < 1e-3, "{a:?} {centre}");
    }
    assert!((a.center().y - combo.rect.center().y).abs() < 0.5, "{a:?}");
    let text = Parts::of(&combo).and_then(|parts| parts.get("text"));
    assert!(
        text.zip(left).is_some_and(|(text, left)| {
            (text.left() - (combo.rect.left() + c.border.line_width + left)).abs() < 1e-3
        }),
        "{text:?}"
    );
}

/// The one open chevron a drop-down paints (`docs/platform-facts.md` §2.24): a three-point
/// path, not closed and not filled, stroked; its bounding rectangle.
fn chevron(out: &egui::FullOutput) -> egui::Rect {
    let chevrons: Vec<egui::Rect> = flat(&out.shapes)
        .into_iter()
        .filter_map(|s| match s {
            egui::Shape::Path(p) if p.points.len() == 3 => {
                assert!(!p.closed, "a chevron, not a triangle: {p:?}");
                assert_eq!(p.fill, egui::Color32::TRANSPARENT, "{p:?}");
                assert!(p.stroke.width > 0.0, "{p:?}");
                Some(egui::Rect::from_points(&p.points))
            }
            _ => None,
        })
        .collect();
    assert_eq!(chevrons.len(), 1, "{chevrons:?}");
    chevrons[0]
}

/// kde-breeze's drop-down as the theme states it: its 10px chevron (Breeze's `ArrowSize`) is
/// shorter than the text, so the drop-down is `combo_box.min_height`, 32, tall, and the
/// chevron spans egui's arrow rectangle of that box.
#[test]
fn the_kde_drop_down_is_its_min_height_with_a_chevron() {
    let t = kde();
    assert_eq!(t.combo_box.arrow_icon_size, 10.0);
    let ctx = installed(&t, &AccessibilityPreferences::default());
    let add = |ui: &mut egui::Ui| {
        ComboBox::from_id_salt("fruit")
            .selected_text("Apple")
            .width(140.0)
            .show_ui(ui, |ui| ui.label("Banana"))
            .response
    };
    let _ = response(&ctx, at(0.0), add);
    let (out, combo) = response(&ctx, at(0.0), add);
    assert!(
        (combo.rect.height() - t.combo_box.min_height).abs() < 0.5,
        "{}",
        combo.rect.height()
    );
    let a = chevron(&out);
    let arrow = t.combo_box.arrow_icon_size;
    assert!((a.width() - 0.7 * arrow).abs() < 1e-3, "{a:?}");
    assert!((a.height() - 0.45 * arrow).abs() < 1e-3, "{a:?}");
}

// ---- M7-22: parts of widgets no longer drawn -----------------------------------------------------

/// The widgets of `one_widget` and a progress bar under ids salted with `pass_nr`, and an
/// expander under one id: egui's `CollapsingHeader` keeps a state per header id itself
/// (`egui/src/containers/collapsing_header.rs:39`, `:187`).
fn churning_pass(ctx: &egui::Context, pass_nr: usize) {
    let _ = pass(ctx, idle(), |ui| {
        ui.push_id(pass_nr, |ui| {
            for i in 0..WIDGETS {
                let _ = one_widget(ui, i);
            }
            ui.add(ProgressBar::new(0.4));
        });
        let _ = Expander::new("Details").show(ui, |_| ());
    });
}

/// M7-22: a widget whose id changes every pass leaves no `Parts` behind: the `Context`'s data
/// holds as many entries after 200 such passes as after the first two, and the parts kept are
/// those of the last two passes' widgets.
#[test]
fn parts_of_widgets_no_longer_drawn_are_not_kept() {
    let ctx = installed(&kde(), &AccessibilityPreferences::default());
    churning_pass(&ctx, 0);
    let per_pass = crate::parts::stored(&ctx);
    churning_pass(&ctx, 1);
    let early = ctx.data(|d| d.len());
    for pass_nr in 2..200 {
        churning_pass(&ctx, pass_nr);
        assert_eq!(crate::parts::stored(&ctx), 2 * per_pass, "pass {pass_nr}");
    }
    assert_eq!(ctx.data(|d| d.len()), early);
}

/// M7-22: a widget's parts answer through the pass after the one that drew it, and not once
/// two passes that drew other widgets of the crate have gone by without it.
#[test]
fn parts_answer_until_two_passes_go_by_without_the_widget() {
    let ctx = installed(&kde(), &AccessibilityPreferences::default());
    let (_, drawn) = response(&ctx, idle(), |ui| {
        ui.push_id("gone", |ui| ui.add(Switch::new(&mut true)))
            .inner
    });
    let parts = Parts::of(&drawn).expect("parts after the pass that drew it");
    assert!(parts.get("thumb").is_some());
    let other = |ui: &mut egui::Ui| {
        ui.push_id("kept", |ui| ui.add(Switch::new(&mut true)))
            .inner
    };
    let mut during = None;
    let _ = pass(&ctx, idle(), |ui| {
        let _ = other(ui);
        during = Some(Parts::of(&drawn));
    });
    assert_eq!(during, Some(Some(parts.clone())), "in the next pass");
    assert_eq!(Parts::of(&drawn), Some(parts), "after the next pass");
    let (_, kept) = response(&ctx, idle(), other);
    assert_eq!(Parts::of(&drawn), None, "two passes without it");
    assert!(Parts::of(&kept).is_some());
}

/// M7-22: a widget drawn in a child viewport answers when read outside it, and the parts of a
/// viewport egui has closed are let go with it.
#[test]
fn parts_of_a_closed_viewport_are_let_go() {
    let ctx = installed(&kde(), &AccessibilityPreferences::default());
    ctx.set_embed_viewports(false);
    let child = egui::ViewportId::from_hash_of("child");
    let root = |ctx: &egui::Context, open: bool| {
        let _ = pass(ctx, idle(), |ui| {
            ui.add(Switch::new(&mut true));
            if open {
                ui.ctx()
                    .show_viewport_deferred(child, egui::ViewportBuilder::default(), |_, _| ());
            }
        });
    };
    root(&ctx, false);
    root(&ctx, true);
    let in_child = egui::RawInput {
        viewport_id: child,
        viewports: std::iter::once((child, egui::ViewportInfo::default())).collect(),
        ..idle()
    };
    let (_, drawn) = response(&ctx, in_child, |ui| {
        ui.push_id("child", |ui| ui.add(Switch::new(&mut true)))
            .inner
    });
    assert!(Parts::of(&drawn).is_some(), "read outside the child");
    assert_eq!(
        crate::parts::stored_viewports(&ctx).len(),
        2,
        "the root's and the child's"
    );
    // egui drops the child at the end of the first root pass that does not show it.
    root(&ctx, false);
    root(&ctx, false);
    assert_eq!(
        ctx.cumulative_pass_nr_for(child),
        0,
        "egui closed the child"
    );
    assert_eq!(
        crate::parts::stored_viewports(&ctx),
        vec![egui::ViewportId::ROOT]
    );
}

// ---- M7-25: the spinner's frame sources ------------------------------------------------------------

/// A context with `t` installed with Material's indicator, drawn light, under reduced motion so
/// the spinner draws the indicator's first frame.
#[cfg(feature = "material-icons")]
fn material_spinner(t: &ResolvedTheme) -> (egui::Context, ThemeAtlas) {
    let mut prefs = AccessibilityPreferences::default();
    prefs.reduce_motion = true;
    let ctx = egui::Context::default();
    let atlas = ThemeAtlas::builder("test", t, t)
        .accessibility(&prefs)
        .icon_set(IconSet::Material)
        .build();
    atlas.install(&ctx);
    ctx.set_theme(egui::Theme::Light);
    (ctx, atlas)
}

/// M7-25: the source a spinner keeps for a frame is registered again after the install of
/// another atlas forgot it, so the next `forget_icons` (here `ThemeAtlas::clear`) forgets its
/// bytes too.
#[cfg(feature = "material-icons")]
#[test]
fn a_kept_spinner_frame_is_forgotten_with_the_icons() {
    use native_theme_egui::icons::{self, IconKey};
    use native_theme_egui::native_theme::icons::load_icon_indicator;

    let t = kde();
    let (ctx, _) = material_spinner(&t);
    // The frame's URI, made in a `Context` of its own so that it registers nothing here.
    let key = IconKey::name("indicator", IconSet::Material)
        .size(t.spinner.diameter)
        .tint(to_color32(t.spinner.fill_color));
    let anim = load_icon_indicator(IconSet::Material, None).unwrap();
    let uri = match icons::to_image_source(&egui::Context::default(), &key, anim.first_frame()) {
        Some(egui::ImageSource::Bytes { uri, .. }) => uri.into_owned(),
        other => panic!("an SVG frame, not {other:?}"),
    };
    let draw = |ctx: &egui::Context| {
        let _ = pass(ctx, idle(), |ui| {
            ui.add(Spinner::new());
        });
    };
    draw(&ctx);
    assert!(ctx.try_load_bytes(&uri).is_ok(), "drawn");
    let (_, again) = material_spinner(&t);
    again.install(&ctx);
    assert!(
        ctx.try_load_bytes(&uri).is_err(),
        "forgotten by the install"
    );
    draw(&ctx);
    assert!(ctx.try_load_bytes(&uri).is_ok(), "drawn again");
    ThemeAtlas::clear(&ctx);
    assert!(ctx.try_load_bytes(&uri).is_err(), "forgotten by clear");
}

/// M7-25: a spinner draws its frame from the source it kept in the pass before, not from a new
/// copy of the frame's SVG.
#[cfg(feature = "material-icons")]
#[test]
fn a_kept_spinner_frame_is_drawn_again() {
    let (ctx, _) = material_spinner(&kde());
    let draw = || {
        let _ = pass(&ctx, idle(), |ui| {
            ui.add(Spinner::new());
        });
    };
    draw();
    let first = crate::spinner::kept_frames(&ctx, IconSet::Material);
    assert_eq!(first.len(), 1, "the first frame, under reduced motion");
    draw();
    draw();
    let later = crate::spinner::kept_frames(&ctx, IconSet::Material);
    assert_eq!(later.len(), 1);
    assert_eq!(first[0].as_ptr(), later[0].as_ptr(), "the same bytes");
}

// ---- M7-06: a `§` citation names its document ---------------------------------------------------

/// What names a `§` citation's document, right before the `§`: `spec` (`Spec` at a sentence's
/// start) for the crate's own specification, which the crate docs name, or a document by name
/// (`docs/platform-facts.md`), the names `sources::every_section_is_named` takes.
const CITATION_NAMES: &[&str] = &["spec", "Spec", ".md"];

/// Every `///` or `//!` line of the crate's sources outside its tests that holds a `§` names
/// the document each `§` cites (M6-38's ruling, `sources::every_section_is_named`), so a docs.rs
/// reader can look it up. As in [`painting_sources`], a file's inline test module is left out
/// (`sources::rendered_lines`).
#[test]
fn every_section_citation_names_its_document() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut scanned = 0;
    let mut bare = Vec::new();
    for entry in std::fs::read_dir(&dir).unwrap() {
        let path = entry.unwrap().path();
        let name = path.file_name().unwrap().to_string_lossy().to_string();
        if name == "tests.rs" || !name.ends_with(".rs") {
            continue;
        }
        let raw = std::fs::read_to_string(&path).unwrap();
        for (number, line) in rendered_lines(&raw) {
            let doc = line.trim_start();
            let is_doc = doc.starts_with("///") || doc.starts_with("//!");
            if is_doc && !every_section_is_named(doc, CITATION_NAMES) {
                bare.push(format!("src/{name}:{number}: {doc}"));
            }
        }
        scanned += 1;
    }
    assert!(scanned > 10, "only {scanned} sources read");
    assert!(
        bare.is_empty(),
        "a `§` that names no document (write `spec §N`):\n{}",
        bare.join("\n")
    );
}

// ---- the widgets' colour reads (docs/archive/todo_closed.md, "Contracts: the connectors' widgets") ----

/// Each widget's colour reads as the coverage test sees them: its `Look` of `t`, as text.
fn looks(t: &ResolvedTheme) -> Vec<(&'static str, String)> {
    vec![
        ("Spinner", format!("{:?}", crate::spinner::Look::of(t))),
        ("Switch", format!("{:?}", crate::switch::Look::of(t))),
        ("Slider", format!("{:?}", crate::slider::Look::of(t))),
        ("links", format!("{:?}", crate::links::Look::of(t))),
        (
            "ProgressBar",
            format!("{:?}", crate::progress_bar::Look::of(t)),
        ),
        (
            "RadioButton",
            format!("{:?}", crate::radio_button::Look::of(t)),
        ),
        (
            "SegmentedControl",
            format!("{:?}", crate::segmented_control::Look::of(t)),
        ),
        ("Expander", format!("{:?}", crate::expander::Look::of(t))),
    ]
}

/// The colour leaves each widget draws in, beyond the atlas's cells, which the connector's
/// `mapping.toml` rows. `ComboBox` reads no colour of the theme's: its colours are its cell's.
const WIDGET_READS: &[(&str, &[&str])] = &[
    ("Spinner", &["spinner.fill_color", "defaults.text_color"]),
    (
        "Switch",
        &[
            "switch.checked_background",
            "switch.hover_checked_background",
            "switch.unchecked_background",
            "switch.hover_unchecked_background",
            "switch.disabled_checked_background",
            "switch.disabled_unchecked_background",
            "switch.thumb_background",
            "switch.unchecked_thumb_background",
            "switch.disabled_thumb_color",
            "switch.unchecked_track_border_color",
            "switch.checked_track_border_color",
            "switch.disabled_unchecked_track_border_color",
            "switch.disabled_checked_track_border_color",
            "switch.thumb_border_color",
            "defaults.font.color",
            "defaults.disabled_text_color",
            "defaults.background_color",
        ],
    ),
    (
        "Slider",
        &[
            "slider.track_color",
            "slider.fill_color",
            "slider.thumb_color",
            "slider.thumb_hover_color",
            "slider.disabled_track_color",
            "slider.disabled_fill_color",
            "slider.disabled_thumb_color",
            "defaults.background_color",
        ],
    ),
    (
        "links",
        &[
            "link.font.color",
            "link.background_color",
            "link.hover_background",
            "link.hover_text_color",
            "link.active_text_color",
            "link.visited_text_color",
            "link.disabled_text_color",
        ],
    ),
    ("ProgressBar", &["progress_bar.border.color"]),
    ("RadioButton", &["defaults.background_color"]),
    ("SegmentedControl", &["defaults.background_color"]),
    ("Expander", &["expander.arrow_color"]),
];

/// Every widget's `Look` is spliced: the crate's sources define as many `pub(crate) struct Look`
/// as `looks` reads, and `WIDGET_READS` has a row for each widget `looks` names. It counts the
/// literal `pub(crate) struct Look ` (the shape all eight take): a `Look` declared another way, or
/// a look struct under another name, is not counted.
#[test]
fn every_look_is_read_by_the_coverage_test() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut defined = Vec::new();
    for entry in std::fs::read_dir(&dir).unwrap() {
        let path = entry.unwrap().path();
        let name = path.file_name().unwrap().to_string_lossy().to_string();
        if name == "tests.rs" || !name.ends_with(".rs") {
            continue;
        }
        let code = std::fs::read_to_string(&path).unwrap();
        for _ in code.matches("pub(crate) struct Look ") {
            defined.push(name.clone());
        }
    }
    defined.sort();
    let read = looks(&kde());
    assert_eq!(
        defined.len(),
        read.len(),
        "the `Look`s defined ({defined:?}) are not the ones `looks` reads ({:?})",
        read.iter().map(|(w, _)| *w).collect::<Vec<_>>()
    );
    let mut named: Vec<&str> = read.iter().map(|(w, _)| *w).collect();
    let mut rowed: Vec<&str> = WIDGET_READS.iter().map(|(w, _)| *w).collect();
    named.sort_unstable();
    rowed.sort_unstable();
    assert_eq!(
        named, rowed,
        "a widget `looks` reads without its WIDGET_READS row, or the reverse"
    );
}

/// The leaves of a serialised theme, each with its value.
fn leaf_values(value: &serde_json::Value) -> Vec<(String, serde_json::Value)> {
    fn walk(prefix: &str, value: &serde_json::Value, out: &mut Vec<(String, serde_json::Value)>) {
        match value {
            serde_json::Value::Object(map) => {
                for (key, child) in map {
                    let path = if prefix.is_empty() {
                        key.clone()
                    } else {
                        format!("{prefix}.{key}")
                    };
                    walk(&path, child, out);
                }
            }
            _ => out.push((prefix.to_owned(), value.clone())),
        }
    }
    let mut out = Vec::new();
    walk("", value, &mut out);
    out
}

/// `json` with the leaf at `leaf` replaced by the colour `hex`, read back as a theme; `None`
/// where the leaf already holds it or does not take a colour.
fn spliced_colour(json: &serde_json::Value, leaf: &str, hex: &str) -> Option<ResolvedTheme> {
    let pointer = format!("/{}", leaf.replace('.', "/"));
    let value = serde_json::Value::String(hex.to_owned());
    if json.pointer(&pointer)? == &value {
        return None;
    }
    let mut spliced = json.clone();
    *spliced.pointer_mut(&pointer)? = value;
    serde_json::from_value(spliced).ok()
}

/// The widgets' own colour reads are sinks of a coverage test, as the iced `Spinner`'s and gpui's
/// Looks are of theirs: every colour leaf of `ResolvedTheme` -- a colour, or an unstated
/// `Option<Rgba>` a colour splices into -- is spliced in each bundled preset in both modes with
/// two colours no leaf of the base holds, and every widget whose `Look` moves must list the leaf
/// in `WIDGET_READS`; every leaf a widget lists must move its `Look` in at least one base.
#[test]
fn each_widget_draws_in_the_colour_leaves_it_lists() {
    use native_theme_egui::native_theme::color::Rgba;
    use native_theme_egui::native_theme::theme::Theme;
    use std::collections::BTreeSet;

    let mut moved: BTreeSet<(&str, String)> = BTreeSet::new();
    let mut failures = Vec::new();
    let mut bases = 0;
    for info in Theme::list_presets() {
        for is_dark in [false, true] {
            let t = from_preset(info.key, is_dark, &AccessibilityPreferences::default())
                .unwrap()
                .1;
            let json = serde_json::to_value(&t).unwrap();
            let base = looks(&t);
            let taken: BTreeSet<String> = leaf_values(&json)
                .into_iter()
                .filter_map(|(_, v)| v.as_str().map(str::to_ascii_lowercase))
                .collect();
            let sentinels: Vec<String> = (0x13..=u8::MAX)
                .map(|red| format!("#{red:02x}579b"))
                .filter(|hex| !taken.contains(hex))
                .take(2)
                .collect();
            bases += 1;
            for (leaf, value) in leaf_values(&json) {
                let colour = match &value {
                    serde_json::Value::String(s) => s.starts_with('#') && s.parse::<Rgba>().is_ok(),
                    serde_json::Value::Null => true,
                    _ => false,
                };
                if !colour {
                    continue;
                }
                for hex in &sentinels {
                    let Some(spliced) = spliced_colour(&json, &leaf, hex) else {
                        continue;
                    };
                    for ((widget, before), (_, after)) in base.iter().zip(looks(&spliced)) {
                        if *before == after {
                            continue;
                        }
                        let listed = WIDGET_READS
                            .iter()
                            .any(|(w, leaves)| w == widget && leaves.contains(&leaf.as_str()));
                        if !listed && moved.insert((widget, leaf.clone())) {
                            failures.push(format!(
                                "{widget} draws in {leaf} (on {} {}), which WIDGET_READS does \
                                 not list",
                                info.key,
                                if is_dark { "dark" } else { "light" }
                            ));
                        }
                        moved.insert((widget, leaf.clone()));
                    }
                }
            }
        }
    }
    assert!(bases > 20, "only {bases} preset/mode combinations read");
    for (widget, leaves) in WIDGET_READS {
        for leaf in *leaves {
            if !moved.contains(&(*widget, (*leaf).to_owned())) {
                failures.push(format!(
                    "{widget} lists {leaf}, which moves its Look in no combination"
                ));
            }
        }
    }
    assert!(
        failures.is_empty(),
        "{} widget colour read(s) do not hold:\n{}",
        failures.len(),
        failures.join("\n")
    );
}

/// A freedesktop indicator whose `currentColor` its document leaves unset, as GNOME's
/// `process-working-symbolic` does, is drawn in `defaults.text_color`, the colour the
/// spinner's `Look` carries to it as the icon key's tint. The theme is a fixture in a child
/// process (`native_theme_test_fixtures`).
#[cfg(all(target_os = "linux", feature = "system-icons"))]
#[test]
fn a_symbolic_freedesktop_indicator_is_drawn_in_the_text_colour()
-> Result<(), Box<dyn std::error::Error>> {
    use native_theme_test_fixtures::{Child, in_child, run, write_scalable_theme};
    const NAME: &str = "tests::a_symbolic_freedesktop_indicator_is_drawn_in_the_text_colour";
    if in_child() {
        let t = from_preset("adwaita", true, &AccessibilityPreferences::default())?.1;
        let mut prefs = AccessibilityPreferences::default();
        prefs.reduce_motion = true;
        let ctx = egui::Context::default();
        ThemeAtlas::builder("test", &t, &t)
            .accessibility(&prefs)
            .icon_set(IconSet::Freedesktop)
            .build()
            .install(&ctx);
        ctx.set_theme(egui::Theme::Dark);
        let _ = pass(&ctx, idle(), |ui| {
            ui.add(Spinner::new());
        });
        let frames = crate::spinner::kept_frames(&ctx, IconSet::Freedesktop);
        assert_eq!(frames.len(), 1, "the indicator's one frame, drawn still");
        let svg = String::from_utf8(frames[0].to_vec())?;
        let c = t.defaults.text_color;
        let hex = format!("#{:02x}{:02x}{:02x}", c.r, c.g, c.b);
        assert!(
            !svg.contains("currentColor") && svg.contains(&hex),
            "{hex}: {svg}"
        );
        return Ok(());
    }
    // The system's icon theme is the fixture's, the only one holding the indicator.
    run(NAME, Child::kde("symbolic-only"), |icons| {
        write_scalable_theme(
            icons,
            "symbolic-only",
            &[(
                "process-working-symbolic",
                r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16"><path fill="currentColor" d="M0 0h16v16H0z"/></svg>"##,
            )],
        )
    })?;
    Ok(())
}

/// The key's tint carries `defaults.text_color` to a freedesktop indicator's `currentColor`
/// unchanged in every bundled preset and mode: `fill_current_color` writes the colour's
/// `#rrggbb`, and the tint's `Color32` gives the same red, green and blue back.
#[test]
fn the_text_colour_survives_the_tint() {
    use native_theme_egui::native_theme::theme::Theme;
    for info in Theme::list_presets() {
        for is_dark in [false, true] {
            let t = from_preset(info.key, is_dark, &AccessibilityPreferences::default())
                .unwrap()
                .1;
            let c = t.defaults.text_color;
            let [r, g, b, _] = to_color32(c).to_srgba_unmultiplied();
            assert_eq!(
                (r, g, b),
                (c.r, c.g, c.b),
                "{} dark={is_dark}: {c:?}",
                info.key
            );
        }
    }
}
