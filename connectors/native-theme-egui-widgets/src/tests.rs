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

use crate::combo_box::ComboBox;
use crate::progress_bar::ProgressBar;
use crate::radio_button::RadioButton;
use crate::segmented_control::SegmentedControl;
use crate::slider::Slider;
use crate::spinner::Spinner;
use crate::switch::Switch;
use crate::wrap;

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
                let w = wrap::link(ui, "Link");
                ui.add(w)
            }),
            Box::new(|ui| ui.add(egui::Link::new("Link"))),
        ),
        (
            "hyperlink",
            Box::new(|ui| {
                let w = wrap::hyperlink(ui, "Docs", "https://example.org");
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
            let link = wrap::link(ui, "Link");
            ui.add(link)
        }
        5 => {
            let hyperlink = wrap::hyperlink(ui, "Docs", "https://example.org");
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
/// docs/platform-facts.md:1475-1476), centred on the track's axis.
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
        .map(|c| 2.0 * c.radius);
    assert_eq!(thumb, t.switch.unchecked_thumb_diameter);
}

/// The radio button's circle is `checkbox.radio_indicator_width` across its outline where the
/// theme states one (Material's 20 against its check box's 18, docs/platform-facts.md:1222).
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

/// T3: a clicked hyperlink is visited on the next pass, in `link.visited_text_color`.
#[test]
fn a_clicked_hyperlink_is_visited() {
    let mut t = kde();
    let visited = egui::Color32::from_rgb(1, 2, 3);
    t.link.visited_text_color = native_theme_egui::native_theme::color::Rgba::new(1, 2, 3, 255);
    let ctx = installed(&t, &AccessibilityPreferences::default());
    let add = |ui: &mut egui::Ui| {
        let w = wrap::hyperlink(ui, "Docs", "https://example.org");
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
    let prefs = AccessibilityPreferences {
        reduce_motion: true,
        ..AccessibilityPreferences::default()
    };
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
    assert_eq!(centre.x, track.right() - 0.5 * t.switch.track_height);

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
/// disables the widget as `.enabled(false)` does, with the calling `Ui`'s own fade.
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
    let track = to_color32(t.switch.checked_background).gamma_multiply(0.5);
    let (out, r) = click(&ctx, |ui| {
        ui.add_enabled_ui(false, |ui| ui.add(Switch::new(&mut on)))
            .inner
    });
    assert!(on && !r.changed());
    assert!(
        rects(&out).iter().any(|s| s.fill == track),
        "{:#?}",
        rects(&out)
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
        // Test modules sit at the end of a file.
        let code = raw.split("#[cfg(test)]").next().unwrap_or_default();
        out.push((name, blank(code)));
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
    let text = crate::parts::Parts::of(&combo).and_then(|parts| parts.get("text"));
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
