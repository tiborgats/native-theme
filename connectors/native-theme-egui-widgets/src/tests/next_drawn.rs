//! The 0.8.0 path draws what the 0.7.2 path draws (docs/v0.8.0-plan.md, section 1, rule 2;
//! spec §10, rule 4): each widget on the 0.8.0 model, drawn under the atlas of each model of
//! every platform preset (`crate::next::parity::PRESETS`, asserted against `Preset::ALL`
//! there), light and dark, in each of its states, paints the same shapes, every difference
//! listed in [`DIFFERENCES`] with its cause; an entry that no longer differs fails.

use super::*;

type TestResult = Result<(), Box<dyn std::error::Error>>;

/// A case drawn: its name and what it adds.
type Case<'a> = (&'static str, &'a dyn Fn(&mut egui::Ui) -> egui::Response);

/// The cases that draw differently and are not listed, and the listed ones seen, by preset.
type Drawn = (Vec<String>, Vec<(&'static str, &'static str)>);

/// A listed difference: the widget's case, the presets on which it draws differently, and why.
struct Difference {
    case: &'static str,
    presets: &'static [&'static str],
    reason: &'static str,
}

/// The listed differences.
const DIFFERENCES: &[Difference] = &[
    Difference {
        case: "radio off",
        presets: &["windows-11", "macos-monterey", "macos-tahoe"],
        reason: RADIO_CELL,
    },
    Difference {
        case: "radio on",
        presets: &["windows-11", "macos-monterey", "macos-tahoe"],
        reason: RADIO_CELL,
    },
    Difference {
        case: "radio off disabled",
        presets: &["macos-monterey", "macos-tahoe"],
        reason: RADIO_CELL,
    },
    Difference {
        case: "radio on disabled",
        presets: &["macos-monterey", "macos-tahoe"],
        reason: RADIO_CELL,
    },
    Difference {
        case: "combo box",
        presets: &["windows-11", "macos-monterey", "macos-tahoe"],
        reason: COMBO_CELL,
    },
    Difference {
        case: "combo box disabled",
        presets: &["macos-monterey", "macos-tahoe"],
        reason: COMBO_CELL,
    },
    Difference {
        case: "segmented control",
        presets: &["windows-11", "macos-monterey", "macos-tahoe"],
        reason: SEGMENTED_CELL,
    },
    Difference {
        case: "segmented control disabled",
        presets: &["windows-11", "macos-monterey", "macos-tahoe"],
        reason: SEGMENTED_CELL,
    },
];

/// The drop-down is egui's `ComboBox`, outlined in its cell's stroke.
const COMBO_CELL: &str = "the button's outline colour alone differs: it is the connector's \
     ComboBox cell's bg_stroke, whose translucent border lies inside the part in the 0.8.0 theme \
     and is composited over its fill (native-theme-egui, next::parity, the ComboBox and \
     ComboBox/Disabled entries of its differences)";

/// The segments are framed and divided in their cell's stroke.
const SEGMENTED_CELL: &str = "the frame's and the dividers' colour alone differs: it is the \
     connector's SegmentedControl cell's bg_stroke, whose translucent border lies inside the part \
     in the 0.8.0 theme and is composited over its fill (native-theme-egui, next::parity, the \
     SegmentedControl entry of its differences); a disabled control fades that colour as one";

/// The radio's circle is egui's, outlined in its Checkbox cell's stroke.
const RADIO_CELL: &str = "the circle's outline colour alone differs: it is the connector's \
     Checkbox cell's bg_stroke, whose translucent border lies inside the part in the 0.8.0 theme \
     and is composited over its fill (native-theme-egui, next::parity, the Checkbox entry of \
     its differences)";

/// A context with the atlas of each model of `key` installed, drawn in its scheme: the 0.7.2
/// theme resolved as `crate::next::parity::resolve` resolves it, and the 0.8.0 preset as
/// `next::to_theme` takes it (its themes for an active and an inactive window). Neither atlas
/// installs a font plan, so both letter in egui's own fonts: the connector's parity compares
/// the fonts each plan installs (`native-theme-egui`, `next::root_parity`), this one the widgets.
fn both_installed(
    key: &str,
    preset: Preset,
    is_dark: bool,
) -> Result<(egui::Context, egui::Context), Box<dyn std::error::Error>> {
    let scheme = if is_dark {
        egui::Theme::Dark
    } else {
        egui::Theme::Light
    };
    let theme = NextTheme::preset(preset);
    let [light, light_inactive, dark, dark_inactive] = [
        ThemeContext::light(),
        ThemeContext::light().with_window_active(false),
        ThemeContext::dark(),
        ThemeContext::dark().with_window_active(false),
    ]
    .map(|ctx| theme.resolve(&ctx));
    let new_atlas =
        ThemeAtlas::next_builder(&light, &light_inactive, &dark, &dark_inactive).build()?;
    // The 0.7.2 atlas takes the 0.8.0 one's icon set and themes, so both spinners draw the
    // same set's indicator: the connector's parity compares the icon sets.
    let (light, _) = crate::next::parity::resolve(key, preset, false)?;
    let (dark, _) = crate::next::parity::resolve(key, preset, true)?;
    // The preset's layout, as `from_preset` hands it to the 0.7.2 builder (the connector's
    // `next::parity` builds its 0.7.2 atlas so).
    let layout = native_theme_egui::native_theme::theme::Theme::preset(key)?.layout;
    let mut old_builder = ThemeAtlas::builder(key, &light, &dark)
        .layout(&layout)
        .icon_set(new_atlas.icon_set());
    for scheme in [egui::Theme::Light, egui::Theme::Dark] {
        if let Some(name) = new_atlas.icon_theme(scheme) {
            old_builder = old_builder.icon_theme(scheme, name);
        }
    }
    let old = egui::Context::default();
    old_builder.build().install(&old);
    let new = egui::Context::default();
    new_atlas.install(&new);
    for ctx in [&old, &new] {
        ctx.set_theme(scheme);
        ctx.enable_accesskit();
    }
    Ok((old, new))
}

/// Draw each case under both atlases of every preset and scheme, the second of two passes, and
/// return the cases that draw differently and are not listed, and the listed ones seen.
fn drawn_alike(cases: &[Case<'_>]) -> Result<Drawn, Box<dyn std::error::Error>> {
    let mut failures = Vec::new();
    let mut listed = Vec::new();
    for &(key, preset) in crate::next::parity::PRESETS {
        for is_dark in [false, true] {
            let (old, new) = both_installed(key, preset, is_dark)?;
            for &(case, add) in cases {
                let _ = response(&old, at(1.0), add);
                let _ = response(&new, at(1.0), add);
                let (a, ra) = response(&old, at(1.0), add);
                let (b, rb) = response(&new, at(1.0), add);
                let (a, b) = (flat(&a.shapes), flat(&b.shapes));
                if a == b && ra.rect == rb.rect {
                    continue;
                }
                if DIFFERENCES
                    .iter()
                    .any(|d| d.case == case && d.presets.contains(&key))
                {
                    listed.push((key, case));
                    continue;
                }
                let shown = |s: &egui::Shape| match s {
                    egui::Shape::Text(t) => format!(
                        "text at {:?}, {:?}, {:?}",
                        t.pos,
                        t.galley.size(),
                        t.galley.job.sections.first().map(|s| &s.format),
                    ),
                    other => format!("{other:?}"),
                };
                let different: Vec<String> = a
                    .iter()
                    .zip(&b)
                    .filter(|(a, b)| a != b)
                    .map(|(a, b)| format!("\n    0.7.2 {}\n    0.8.0 {}", shown(a), shown(b)))
                    .collect();
                failures.push(format!(
                    "{key}/{}: {case}: rect 0.7.2 {:?}, 0.8.0 {:?}; {} and {} shapes; \
                     different:{}",
                    if is_dark { "dark" } else { "light" },
                    ra.rect,
                    rb.rect,
                    a.len(),
                    b.len(),
                    different.concat(),
                ));
            }
        }
    }
    Ok((failures, listed))
}

/// Every listed difference of `cases` that drew alike everywhere it is listed.
fn stale(cases: &[&str], listed: &[(&str, &str)]) -> Vec<String> {
    DIFFERENCES
        .iter()
        .filter(|d| cases.contains(&d.case))
        .flat_map(|d| {
            d.presets
                .iter()
                .filter(|p| !listed.contains(&(**p, d.case)))
                .map(|p| {
                    format!(
                        "{p}: {}: listed as a difference ({}), but drawn alike: remove the entry",
                        d.case, d.reason
                    )
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

/// Assert the cases draw alike from both models.
fn assert_drawn_alike(cases: &[Case<'_>]) -> TestResult {
    let (mut failures, listed) = drawn_alike(cases)?;
    let names: Vec<&str> = cases.iter().map(|&(case, _)| case).collect();
    failures.extend(stale(&names, &listed));
    assert!(
        failures.is_empty(),
        "{} cases draw differently:\n{}",
        failures.len(),
        failures.join("\n")
    );
    Ok(())
}

#[test]
fn the_switch_draws_alike_from_both_models() -> TestResult {
    let switch = |on: bool, enabled: bool| {
        move |ui: &mut egui::Ui| {
            ui.add(Switch::new(&mut on.clone()).label("Wi-Fi").enabled(enabled))
        }
    };
    let (off, on, off_disabled, on_disabled) = (
        switch(false, true),
        switch(true, true),
        switch(false, false),
        switch(true, false),
    );
    assert_drawn_alike(&[
        ("switch off", &off),
        ("switch on", &on),
        ("switch off disabled", &off_disabled),
        ("switch on disabled", &on_disabled),
    ])
}

#[test]
fn the_radio_button_draws_alike_from_both_models() -> TestResult {
    let radio = |checked: bool, enabled: bool| {
        move |ui: &mut egui::Ui| ui.add(RadioButton::new(checked, "Radio").enabled(enabled))
    };
    let (off, on, off_disabled, on_disabled) = (
        radio(false, true),
        radio(true, true),
        radio(false, false),
        radio(true, false),
    );
    assert_drawn_alike(&[
        ("radio off", &off),
        ("radio on", &on),
        ("radio off disabled", &off_disabled),
        ("radio on disabled", &on_disabled),
    ])
}

#[test]
fn the_progress_bar_and_the_spinner_draw_alike_from_both_models() -> TestResult {
    let bar = |ui: &mut egui::Ui| ui.add(ProgressBar::new(0.4));
    let spinner = |ui: &mut egui::Ui| ui.add(Spinner::new());
    assert_drawn_alike(&[("progress bar", &bar), ("spinner", &spinner)])
}

#[test]
fn the_slider_draws_alike_from_both_models() -> TestResult {
    let slider = |enabled: bool| {
        move |ui: &mut egui::Ui| {
            ui.add(
                Slider::new(&mut 40.0, 0.0..=100.0)
                    .label("Volume")
                    .enabled(enabled),
            )
        }
    };
    let (on, off) = (slider(true), slider(false));
    assert_drawn_alike(&[("slider", &on), ("slider disabled", &off)])
}

#[test]
fn the_segmented_control_draws_alike_from_both_models() -> TestResult {
    let control = |enabled: bool| {
        move |ui: &mut egui::Ui| {
            ui.add_enabled_ui(enabled, |ui| {
                ui.add(SegmentedControl::new(&mut 1, ["Day", "Week"]))
            })
            .inner
        }
    };
    let (on, off) = (control(true), control(false));
    assert_drawn_alike(&[
        ("segmented control", &on),
        ("segmented control disabled", &off),
    ])
}

#[test]
fn the_links_draw_alike_from_both_models() -> TestResult {
    let link = |enabled: bool| {
        move |ui: &mut egui::Ui| {
            ui.add_enabled_ui(enabled, |ui| {
                let w = links::link(ui, "Link");
                ui.add(w)
            })
            .inner
        }
    };
    let hyperlink = |enabled: bool| {
        move |ui: &mut egui::Ui| {
            ui.add_enabled_ui(enabled, |ui| {
                let w = links::hyperlink(ui, "Docs", "https://example.org");
                ui.add(w)
            })
            .inner
        }
    };
    let (a, b, c, d) = (link(true), link(false), hyperlink(true), hyperlink(false));
    assert_drawn_alike(&[
        ("link", &a),
        ("link disabled", &b),
        ("hyperlink", &c),
        ("hyperlink disabled", &d),
    ])
}

#[test]
fn the_combo_box_draws_alike_from_both_models() -> TestResult {
    let combo = |enabled: bool| {
        move |ui: &mut egui::Ui| {
            ComboBox::from_id_salt("fruit")
                .selected_text("Apple")
                .width(140.0)
                .enabled(enabled)
                .show_ui(ui, |ui| ui.label("Banana"))
                .response
        }
    };
    let (on, off) = (combo(true), combo(false));
    assert_drawn_alike(&[("combo box", &on), ("combo box disabled", &off)])
}

// ---- patched themes: the branches no preset reaches ---------------------------------------------

/// `preset` with `patch` laid over it, resolved light.
fn patched(preset: Preset, patch: &str) -> Result<NextResolved, Box<dyn std::error::Error>> {
    let patch = native_theme_egui::native_theme::next::ThemePatch::from_toml(patch)?;
    Ok(NextTheme::preset(preset)
        .with_patch(patch)
        .resolve(&ThemeContext::light()))
}

/// A context drawn light with the atlas of `preset` and `patch` installed, with no font plan.
fn patched_installed(
    preset: Preset,
    patch: &str,
) -> Result<egui::Context, Box<dyn std::error::Error>> {
    let patch = native_theme_egui::native_theme::next::ThemePatch::from_toml(patch)?;
    let theme = NextTheme::preset(preset).with_patch(patch);
    let [light, light_inactive, dark, dark_inactive] = [
        ThemeContext::light(),
        ThemeContext::light().with_window_active(false),
        ThemeContext::dark(),
        ThemeContext::dark().with_window_active(false),
    ]
    .map(|ctx| theme.resolve(&ctx));
    let ctx = egui::Context::default();
    ThemeAtlas::next_builder(&light, &light_inactive, &dark, &dark_inactive)
        .build()?
        .install(&ctx);
    ctx.set_theme(egui::Theme::Light);
    Ok(ctx)
}

type NextResolved = native_theme_egui::native_theme::next::ResolvedTheme;

/// The switch's parts and shapes, drawn off, enabled, in the second of two passes.
fn drawn_switch(ctx: &egui::Context) -> (Parts, Vec<egui::Shape>) {
    let add = |ui: &mut egui::Ui| ui.add(Switch::new(&mut false).label("Wi-Fi"));
    let _ = response(ctx, at(1.0), add);
    let (out, r) = response(ctx, at(1.0), add);
    (Parts::of(&r).unwrap_or_default(), flat(&out.shapes))
}

/// The rectangle painted at `rect`.
fn painted_rect(shapes: &[egui::Shape], rect: egui::Rect) -> Option<egui::epaint::RectShape> {
    shapes.iter().find_map(|s| match s {
        egui::Shape::Rect(r) if r.rect == rect => Some(r.clone()),
        _ => None,
    })
}

/// A track stroke stated as none is no outline (spec §10, rule 2), where material-3's states
/// one; a fill stated as none is transparent.
#[test]
fn a_switch_track_stated_as_none_has_no_outline_and_no_fill() -> TestResult {
    use crate::switch::Look;
    let stock = Look::next_of(&patched(Preset::Material3, "")?).ok_or("stock material-3")?;
    assert!(stock.unchecked_track_border_color.is_some(), "{stock:?}");
    let none =
        "[[widgets.switch.rules]]\nstates = []\ntrack.stroke = \"none\"\ntrack.fill = \"none\"\n";
    let look = Look::next_of(&patched(Preset::Material3, none)?).ok_or("patched material-3")?;
    assert_eq!(look.unchecked_track_border_color, None);
    assert_eq!(look.unchecked_background, Rgba::new(0, 0, 0, 0));
    let (parts, shapes) = drawn_switch(&patched_installed(Preset::Material3, none)?);
    let track = parts.get("track").ok_or("no track")?;
    let rect = painted_rect(&shapes, track).ok_or("no track painted")?;
    assert_eq!(rect.fill, egui::Color32::TRANSPARENT);
    assert_eq!(rect.stroke, egui::Stroke::NONE);
    Ok(())
}

/// `switch.label_gap`, which no preset states, puts the label that far after the track; the
/// label is lettered in `switch.label`'s colour where stated.
#[test]
fn a_switch_label_is_at_the_stated_gap_in_the_stated_colour() -> TestResult {
    let (parts, _) = drawn_switch(&patched_installed(Preset::KdeBreeze, "")?);
    let own = parts.get("label").ok_or("no label")?.left()
        - parts.get("track").ok_or("no track")?.right();
    let patch = "[widgets.switch]\nlabel_gap = 23.0\n\n[[widgets.switch.rules]]\nstates = []\nlabel.color = \"#ff00ff\"\n";
    let (parts, shapes) = drawn_switch(&patched_installed(Preset::KdeBreeze, patch)?);
    let (label, track) = (
        parts.get("label").ok_or("no label")?,
        parts.get("track").ok_or("no track")?,
    );
    assert!(
        own != 23.0,
        "egui's own gap is the stated one: the test proves nothing"
    );
    assert_eq!(label.left() - track.right(), 23.0);
    let text = shapes
        .iter()
        .find_map(|s| match s {
            egui::Shape::Text(t) => Some(t.fallback_color),
            _ => None,
        })
        .ok_or("no label drawn")?;
    assert_eq!(text, egui::Color32::from_rgb(255, 0, 255));
    Ok(())
}

/// A track with pill corners is rounded by half its shorter side.
#[test]
fn a_pill_switch_track_is_rounded_by_half_its_height() -> TestResult {
    use crate::switch::Geometry;
    let patch = "[[widgets.switch.rules]]\nstates = []\ntrack.corners = \"pill\"\ntrack.min_size = [40.0, 30.0]\n";
    let g = Geometry::next_of(&patched(Preset::Windows11, patch)?).ok_or("no geometry")?;
    assert_eq!((g.track, g.radius), (egui::vec2(40.0, 30.0), 15.0));
    Ok(())
}

/// The radio's label lies `radio.label_gap` after its circle, where the stated gap is not its
/// cell's own.
#[test]
fn a_radio_label_is_at_the_stated_gap() -> TestResult {
    let label_x = |ctx: &egui::Context| -> Option<(f32, f32)> {
        let add = |ui: &mut egui::Ui| ui.add(RadioButton::new(true, "Radio"));
        let _ = response(ctx, at(1.0), add);
        let (out, r) = response(ctx, at(1.0), add);
        let circle = Parts::of(&r)?.get("indicator")?;
        let text = flat(&out.shapes).into_iter().find_map(|s| match s {
            egui::Shape::Text(t) => Some(t.pos.x),
            _ => None,
        })?;
        Some((circle.right(), text))
    };
    let (right, own) = label_x(&patched_installed(Preset::KdeBreeze, "")?).ok_or("stock")?;
    assert!(
        own - right != 23.0,
        "the stated gap is the cell's: the test proves nothing"
    );
    let patch = "[widgets.radio]\nlabel_gap = 23.0\n";
    let (right, text) = label_x(&patched_installed(Preset::KdeBreeze, patch)?).ok_or("patched")?;
    assert_eq!(text - right, 23.0);
    Ok(())
}

/// A progress bar's track stroke stated as none is no outline, on every preset whose bar has
/// one; its pill corners are half its stated height.
#[test]
fn a_progress_bar_track_stated_as_none_has_no_outline() -> TestResult {
    use crate::progress_bar::Bar;
    let none = "[[widgets.progress_bar.rules]]\nstates = []\ntrack.stroke = \"none\"\n";
    let mut outlined = 0;
    for &(_, preset) in crate::next::parity::PRESETS {
        let stock = Bar::next_of(&patched(preset, "")?);
        if !matches!(stock.frame, Some((_, Some(_)))) {
            continue;
        }
        outlined += 1;
        let bar = Bar::next_of(&patched(preset, none)?);
        let (radius, outline) = bar.frame.ok_or("no frame")?;
        assert_eq!(outline, None, "{preset:?}");
        assert_eq!(Some(radius), stock.frame.map(|(r, _)| r), "{preset:?}");
    }
    assert!(
        outlined > 0,
        "no preset outlines its bar: the test proves nothing"
    );
    let pill = "[[widgets.progress_bar.rules]]\nstates = []\ntrack.corners = \"pill\"\n\
                track.min_size = [100.0, 6.0]\n";
    let bar = Bar::next_of(&patched(Preset::KdeBreeze, pill)?);
    assert_eq!(bar.frame.map(|(r, _)| r), Some(3.0));
    assert_eq!(bar.min_width, Some(100.0));
    Ok(())
}

/// A link's stated underline underlines it at rest, and a hover fill a rule for `HOVER`
/// states is the layer laid over its background under the pointer.
#[test]
fn a_link_takes_its_stated_underline_and_hover_layer() -> TestResult {
    use crate::links::Look;
    let underlined = |ctx: &egui::Context| -> Option<f32> {
        let add = |ui: &mut egui::Ui| {
            let w = links::link(ui, "Link");
            ui.add(w)
        };
        let _ = response(ctx, at(1.0), add);
        let (out, _) = response(ctx, at(1.0), add);
        flat(&out.shapes).into_iter().find_map(|s| match s {
            egui::Shape::Text(t) => t
                .galley
                .job
                .sections
                .first()
                .map(|s| s.format.underline.width),
            _ => None,
        })
    };
    // Breeze underlines its links at rest; the patch states the opposite.
    let magenta = Rgba::new(255, 0, 255, 255);
    let stock = Look::next_of(&patched(Preset::KdeBreeze, "")?);
    assert_eq!(stock.underline_enabled, Some(true), "{stock:?}");
    assert_ne!(stock.hover_background, Some(magenta), "{stock:?}");
    let width = underlined(&patched_installed(Preset::KdeBreeze, "")?).ok_or("no text")?;
    assert!(width > 0.0, "the stock link is not underlined: {width}");
    let patch = "[[widgets.link.rules]]\nstates = []\nlabel.decoration = { kind = \"none\" }\n\n\
                 [[widgets.link.rules]]\nstates = [\"hover\"]\ncontainer.fill = \"#ff00ff\"\n";
    let look = Look::next_of(&patched(Preset::KdeBreeze, patch)?);
    assert_eq!(look.underline_enabled, Some(false));
    assert_eq!(look.hover_background, Some(magenta));
    assert_eq!(
        underlined(&patched_installed(Preset::KdeBreeze, patch)?),
        Some(0.0)
    );
    Ok(())
}

/// A slider thumb's stroke stated as none is no outline, on every preset whose thumb has one.
#[test]
fn a_slider_thumb_stated_as_none_has_no_outline() -> TestResult {
    use crate::slider::{Geometry, Look};
    let none = "[[widgets.slider.rules]]\nstates = []\nthumb.stroke = \"none\"\n";
    let mut outlined = 0;
    for &(_, preset) in crate::next::parity::PRESETS {
        let stock = patched(preset, "")?;
        if Geometry::next_of(&stock, 1.0)
            .and_then(|g| g.outline)
            .is_none()
        {
            continue;
        }
        outlined += 1;
        let r = patched(preset, none)?;
        assert_eq!(
            Geometry::next_of(&r, 1.0).ok_or("no geometry")?.outline,
            None,
            "{preset:?}"
        );
        let look = Look::next_of(&r).ok_or("no look")?;
        assert_eq!(look.thumb_border_color, None, "{preset:?}");
    }
    assert!(
        outlined > 0,
        "no preset outlines its thumb: the test proves nothing"
    );
    Ok(())
}

/// A segmented control's stated vertical padding keeps its stated height, and a hover fill a
/// rule for `HOVER` states is its hover layer.
#[test]
fn a_segmented_control_takes_its_stated_padding_and_hover_layer() -> TestResult {
    use crate::segmented_control::Look;
    let mut unpadded = None;
    for &(_, preset) in crate::next::parity::PRESETS {
        let stock = Look::next_of(&patched(preset, "")?);
        if !stock.padded && stock.hover.is_none() {
            unpadded = Some(preset);
        }
    }
    let preset = unpadded.ok_or("every preset states the padding or a hover layer")?;
    let patch = "[[widgets.segmented_control.rules]]\nstates = []\nsegment.padding = { top = 3.0, right = 0.0, bottom = 3.0, left = 0.0 }\n\n\
                 [[widgets.segmented_control.rules]]\nstates = [\"hover\"]\nsegment.fill = \"#ff00ff\"\n";
    let look = Look::next_of(&patched(preset, patch)?);
    assert!(look.padded, "{preset:?}");
    assert_eq!(look.hover, Some(Rgba::new(255, 0, 255, 255)), "{preset:?}");
    Ok(())
}

/// A drop-down field's stroke stated as none is no border: its text is inset by the padding
/// alone.
#[test]
fn a_combo_box_field_stated_as_none_has_no_border() -> TestResult {
    use crate::combo_box::Sides;
    let none = "[[widgets.combo_box.rules]]\nstates = []\nfield.stroke = \"none\"\n";
    let mut bordered = 0;
    for &(_, preset) in crate::next::parity::PRESETS {
        let stock = Sides::next_of(&patched(preset, "")?);
        if !stock.is_some_and(|s| s.border > 0.0) {
            continue;
        }
        bordered += 1;
        let sides = Sides::next_of(&patched(preset, none)?).ok_or("no sides")?;
        assert_eq!(sides.border, 0.0, "{preset:?}");
        assert_eq!(
            (sides.left, sides.right),
            stock.map(|s| (s.left, s.right)).ok_or("no stock sides")?,
            "{preset:?}"
        );
    }
    assert!(
        bordered > 0,
        "no preset borders its field: the test proves nothing"
    );
    Ok(())
}
