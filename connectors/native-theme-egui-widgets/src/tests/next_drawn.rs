//! The 0.8.0 path draws what the 0.7.2 path draws (docs/v0.8.0-plan.md, section 1, rule 2;
//! spec §10, rule 4): each widget on the 0.8.0 model, drawn under the atlas of each model of
//! every platform preset (`crate::next::parity::PRESETS`, asserted against `Preset::ALL`
//! there), light and dark, in each of its states, paints the same shapes, every difference
//! listed in [`DIFFERENCES`] with its cause and what it lets differ ([`Differs`]), and a listed
//! case still equal in every value that leaves; an entry that no longer differs fails.

use super::*;

type TestResult = Result<(), Box<dyn std::error::Error>>;

/// A case drawn: its name and what it adds.
type Case<'a> = (&'static str, &'a dyn Fn(&mut egui::Ui) -> egui::Response);

/// The cases that draw differently and are not listed, and the listed ones seen, by preset.
type Drawn = (Vec<String>, Vec<(&'static str, &'static str)>);

/// What a listed difference lets differ.
#[derive(Clone, Copy, PartialEq)]
enum Differs {
    /// The outlines' colours alone ([`outlines_masked`]).
    Outline,
    /// The label's colour alone ([`labels_masked`]).
    Label,
    /// The values the 0.8.0 theme states where 0.7.2 states others, which the reason names:
    /// any value of the case's shapes.
    Stated,
}

/// A listed difference: the widget's case, the presets on which it draws differently, what may
/// differ, and why.
struct Difference {
    case: &'static str,
    presets: &'static [&'static str],
    differs: Differs,
    reason: &'static str,
}

/// The listed differences.
const DIFFERENCES: &[Difference] = &[
    Difference {
        case: "radio off",
        presets: &["windows-11", "macos-monterey", "macos-tahoe"],
        differs: Differs::Outline,
        reason: RADIO_CELL,
    },
    Difference {
        case: "radio on",
        presets: &["windows-11", "macos-monterey", "macos-tahoe"],
        differs: Differs::Outline,
        reason: RADIO_CELL,
    },
    Difference {
        case: "radio off disabled",
        presets: &["macos-monterey", "macos-tahoe"],
        differs: Differs::Outline,
        reason: RADIO_CELL,
    },
    Difference {
        case: "radio on disabled",
        presets: &["macos-monterey", "macos-tahoe"],
        differs: Differs::Outline,
        reason: RADIO_CELL,
    },
    Difference {
        case: "radio off",
        presets: &["ios-13"],
        differs: Differs::Stated,
        reason: IOS_RADIO,
    },
    Difference {
        case: "radio on",
        presets: &["ios-13"],
        differs: Differs::Stated,
        reason: IOS_RADIO,
    },
    Difference {
        case: "radio off disabled",
        presets: &["material-3"],
        differs: Differs::Stated,
        reason: M3_RADIO_DISABLED,
    },
    Difference {
        case: "radio on disabled",
        presets: &["material-3"],
        differs: Differs::Stated,
        reason: M3_RADIO_DISABLED,
    },
    Difference {
        case: "radio off disabled",
        presets: &["ios-13"],
        differs: Differs::Stated,
        reason: IOS_RADIO,
    },
    Difference {
        case: "radio on disabled",
        presets: &["ios-13"],
        differs: Differs::Stated,
        reason: IOS_RADIO,
    },
    Difference {
        case: "combo box",
        presets: &["windows-11", "macos-monterey"],
        differs: Differs::Outline,
        reason: COMBO_CELL,
    },
    Difference {
        case: "combo box disabled",
        presets: &["macos-monterey"],
        differs: Differs::Outline,
        reason: COMBO_CELL,
    },
    Difference {
        case: "combo box",
        presets: &["macos-tahoe"],
        differs: Differs::Stated,
        reason: TAHOE_COMBO,
    },
    Difference {
        case: "combo box disabled",
        presets: &["macos-tahoe"],
        differs: Differs::Stated,
        reason: TAHOE_COMBO,
    },
    Difference {
        case: "combo box",
        presets: &["ios-13"],
        differs: Differs::Stated,
        reason: IOS_COMBO,
    },
    Difference {
        case: "combo box disabled",
        presets: &["ios-13"],
        differs: Differs::Stated,
        reason: IOS_COMBO,
    },
    Difference {
        case: "segmented control",
        presets: &["windows-11", "macos-monterey"],
        differs: Differs::Outline,
        reason: SEGMENTED_CELL,
    },
    Difference {
        case: "segmented control disabled",
        presets: &["windows-11", "macos-monterey"],
        differs: Differs::Outline,
        reason: SEGMENTED_CELL,
    },
    Difference {
        case: "segmented control",
        presets: &["macos-tahoe"],
        differs: Differs::Stated,
        reason: TAHOE_SEGMENTED,
    },
    Difference {
        case: "segmented control disabled",
        presets: &["macos-tahoe"],
        differs: Differs::Stated,
        reason: TAHOE_SEGMENTED,
    },
    Difference {
        case: "expander closed",
        presets: &["macos-tahoe"],
        differs: Differs::Stated,
        reason: TAHOE_EXPANDER,
    },
    Difference {
        case: "expander open",
        presets: &["macos-tahoe"],
        differs: Differs::Stated,
        reason: TAHOE_EXPANDER,
    },
    Difference {
        case: "expander closed disabled",
        presets: &["macos-tahoe"],
        differs: Differs::Stated,
        reason: TAHOE_EXPANDER,
    },
    Difference {
        case: "expander open disabled",
        presets: &["macos-tahoe"],
        differs: Differs::Stated,
        reason: TAHOE_EXPANDER,
    },
    Difference {
        case: "switch off disabled",
        presets: &["kde-breeze", "material-3"],
        differs: Differs::Label,
        reason: SWITCH_DISABLED_LABEL,
    },
    Difference {
        case: "switch on disabled",
        presets: &["kde-breeze", "material-3"],
        differs: Differs::Label,
        reason: SWITCH_DISABLED_LABEL,
    },
    Difference {
        case: "switch off",
        presets: &["adwaita"],
        differs: Differs::Stated,
        reason: ADWAITA_SWITCH,
    },
    Difference {
        case: "switch on",
        presets: &["adwaita"],
        differs: Differs::Stated,
        reason: ADWAITA_SWITCH,
    },
    Difference {
        case: "switch off disabled",
        presets: &["adwaita"],
        differs: Differs::Stated,
        reason: ADWAITA_SWITCH,
    },
    Difference {
        case: "switch on disabled",
        presets: &["adwaita"],
        differs: Differs::Stated,
        reason: ADWAITA_SWITCH,
    },
    Difference {
        case: "switch off",
        presets: &["windows-11"],
        differs: Differs::Stated,
        reason: WINDOWS_SWITCH_GAP,
    },
    Difference {
        case: "switch on",
        presets: &["windows-11"],
        differs: Differs::Stated,
        reason: WINDOWS_SWITCH_GAP,
    },
    Difference {
        case: "switch off disabled",
        presets: &["windows-11"],
        differs: Differs::Stated,
        reason: WINDOWS_SWITCH_GAP,
    },
    Difference {
        case: "switch on disabled",
        presets: &["windows-11"],
        differs: Differs::Stated,
        reason: WINDOWS_SWITCH_GAP,
    },
];

/// kde-breeze's and material-3's disabled switch label.
const SWITCH_DISABLED_LABEL: &str = "the label's colour alone differs: 0.7.2 has no switch \
     label and takes defaults.disabled_text_color; the 0.8.0 presets state the platform's \
     disabled switch label: kde-breeze qqc2-desktop-style's, the Disabled [Colors:Window] text \
     at its opacity 0.6 (#a0a1a399 / #686a6c99: Switch.qml:72, :75 at qqc2-desktop-style \
     v6.31.0; plasmadesktoptheme.cpp:273-274 at Kirigami 8319acc7), material-3 Android's \
     MaterialSwitch text, on-background at material_emphasis_medium 0.6 (#1d1b2099 / #e6e0e999: \
     android:Widget.CompoundButton's textColor ?attr/textColorPrimaryDisableOnly, AOSP \
     core/res/res/values/styles.xml:386 at android-15.0.0_r1; m3_primary_text_disable_only.xml:19, \
     dimens.xml:21, materialswitch/res/values/styles.xml:17-29 at material-components-android \
     66c334b); crate::next::parity, switch::Look.disabled_text";

/// adwaita's switch label.
const ADWAITA_SWITCH: &str = "the label's gap and colour: the 0.8.0 adwaita states \
     switch.label_gap 6, the least gap of AdwSwitchRow's header box (_lists.scss:132 at \
     libadwaita 1.10.0), where 0.7.2 has none and the label lies at the scope's own \
     icon_spacing, 2 pt farther; and the label in AdwSwitchRow's title colour, the row's text, \
     #000006cc in light (_lists.scss:5-7, :521; _misc.scss:202-204; _colors.scss:37, :62 at \
     libadwaita 1.10.0), at --disabled-opacity 50 % when disabled (#00000666 / #ffffff80: \
     _labels.scss:8-10, _colors.scss:37, :100, :320 at libadwaita 1.10.0), where 0.7.2 took the \
     window's text and defaults.disabled_text_color; crate::next::parity, switch::Look.text and \
     .disabled_text, switch::Geometry.label_gap";

/// windows-11's switch label.
const WINDOWS_SWITCH_GAP: &str = "the label's gap: the 0.8.0 windows-11 states \
     switch.label_gap 12, ToggleSwitch's fixed column (ToggleSwitch_themeresources.xaml:499-503 \
     at microsoft-ui-xaml 8463f45), where 0.7.2 has none and the label lies at the scope's own \
     icon_spacing, 4 pt nearer; crate::next::parity, switch::Geometry.label_gap";

/// material-3's disabled radio.
const M3_RADIO_DISABLED: &str = "the label's colour, and a checked radio's dot: 0.7.2 draws \
     both in defaults.disabled_text_color, on-surface at 0.38; the 0.8.0 material-3 states \
     Android's disabled label, the label's colour at material_emphasis_medium 0.6 (#1d1b2099 / \
     #e6e0e999: android:Widget.CompoundButton's textColor ?attr/textColorPrimaryDisableOnly, AOSP \
     core/res/res/values/styles.xml:386 at android-15.0.0_r1; radiobutton/res/values/styles.xml:19-33 \
     sets none, m3_primary_text_disable_only.xml:19, dimens.xml:21 at material-components-android \
     66c334b), and the radio states no dot for DISABLED, so the dot is the connector's \
     Checkbox/SelectedDisabled cell's fg_stroke, checkbox.checkmark(DISABLED|CHECKED)'s colour, \
     surface (selected-disabled-icon-color, _md-comp-checkbox.scss:50, v0_192 at material-web \
     cbd34a8; native-theme-egui, next::parity, the Checkbox/Disabled and \
     Checkbox/SelectedDisabled entries)";

/// ios-13's radio circle.
const IOS_RADIO: &str = "the circle's outline: 0.7.2's ios-13 outlines it in its uncited \
     default line, 0.5 px in the separator, checked or not; the 0.8.0 ios-13 states the outline \
     WebKit strokes round an empty iOS radio, checkboxRadioBorderColor (secondaryLabel), 1.5 px \
     as it shows (checkboxRadioBorderWidth × 2 clipped to the edge), and none round a checked \
     one (RenderThemeIOS.mm:1354, :1357-1368, :1483-1491, :1502 at WebKit 31adf9d; \
     native-theme/tests/next_homes.toml, rows checkbox.unchecked_border_color and \
     checkbox.border.line_width, listed differences); the circle's outer edge stays";

/// macos-tahoe's drop-down field.
const TAHOE_COMBO: &str = "the field's outline and the text's inset: 0.7.2's macos-tahoe \
     states the default line (0.5); the 0.8.0 macos-tahoe states the pop-up button's outline as \
     none, as macOS 26 draws it (os-probe run 45198750893, macOS 26, `*.normal.popup` `edge_*`: \
     every edge pixel is the fill; native-theme/tests/next_homes.toml, rows \
     combo_box.border.line_width and combo_box.border.color, listed differences), so the text \
     and the arrow lie the line's 0.5 pt nearer the edge (native-theme-egui, next::parity, the \
     ComboBox button_padding entry)";

/// ios-13's drop-down field.
const IOS_COMBO: &str = "the field's outline and the text's inset: 0.7.2's ios-13 states the \
     default border (0.5 in the separator, uncited); the 0.8.0 ios-13 states the border WebKit \
     draws round an iOS select, 1px of -webkit-control-background, systemBackground \
     (html.css:1101 at WebKit 31adf9d; RenderThemeIOS.mm:1027; \
     native-theme/tests/next_homes.toml, rows combo_box.border.line_width and \
     combo_box.border.color, listed differences), so the text and the arrow lie 0.5 pt farther \
     in";

/// macos-tahoe's segmented control.
const TAHOE_SEGMENTED: &str = "the frame: 0.7.2's macos-tahoe states the default line, which \
     the control draws as a 1 pt frame; the 0.8.0 macos-tahoe states the segmented control's \
     outline as none, as macOS 26 draws it (os-probe run 45198750893, macOS 26, \
     `*.normal.segment` `edge_top`, `edge_bottom`; native-theme/tests/next_homes.toml, rows \
     segmented_control.border.line_width and segmented_control.border.color, listed \
     differences), so the control is the frame's width smaller on each side and its end \
     segments take its corners";

/// macos-tahoe's expander header.
const TAHOE_EXPANDER: &str = "the header's side padding: the header pads a side it states none \
     of by the base style's button_padding, which macos-tahoe's push button, its bezel stroked \
     as none (M2.FS4, measured; native-theme/tests/next_homes.toml, row button.border.line_width, \
     a listed difference), now leaves at its padding alone, where 0.7.2 added its 0.5 hairline \
     (native-theme-egui, next::parity, the base button_padding entry): the arrow and the title \
     lie 0.5 pt nearer the start";

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
const RADIO_CELL: &str = "the circle's outline colour alone differs: radio.indicator's stroke, \
     laid over the connector's Checkbox cell as the cell lays its bg_stroke, a translucent border \
     inside the part in the 0.8.0 theme, composited over its fill (native-theme-egui, \
     next::parity, the Checkbox entry of its differences)";

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
                let entry = DIFFERENCES
                    .iter()
                    .find(|d| d.case == case && d.presets.contains(&key));
                let is_listed = entry.is_some();
                // A listed case differs in what its entry lets differ alone: every other value
                // of every shape is equal.
                if let Some(entry) = entry {
                    listed.push((key, case));
                    let alike = match entry.differs {
                        Differs::Outline => outlines_masked(&a) == outlines_masked(&b),
                        Differs::Label => labels_masked(&a) == labels_masked(&b),
                        Differs::Stated => true,
                    };
                    if alike && (entry.differs == Differs::Stated || ra.rect == rb.rect) {
                        continue;
                    }
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
                    "{key}/{}: {case}{}: rect 0.7.2 {:?}, 0.8.0 {:?}; {} and {} shapes; \
                     different:{}",
                    if is_dark { "dark" } else { "light" },
                    if is_listed {
                        " (listed: it differs beyond what its entry lets differ)"
                    } else {
                        ""
                    },
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

/// `shapes` with each outline's colour made transparent, and a rect filled in an outline's
/// colour (the segmented control's dividers), its width and every other value kept: what a
/// listed difference, an outline colour, leaves to compare.
fn outlines_masked(shapes: &[egui::Shape]) -> Vec<egui::Shape> {
    let none = egui::Color32::TRANSPARENT;
    let outlines: Vec<egui::Color32> = shapes
        .iter()
        .filter_map(|shape| match shape {
            egui::Shape::Rect(r) if r.stroke.width > 0.0 => Some(r.stroke.color),
            egui::Shape::Circle(c) if c.stroke.width > 0.0 => Some(c.stroke.color),
            egui::Shape::Ellipse(e) if e.stroke.width > 0.0 => Some(e.stroke.color),
            egui::Shape::LineSegment { stroke, .. } if stroke.width > 0.0 => Some(stroke.color),
            egui::Shape::Path(p) if p.stroke.width > 0.0 => match p.stroke.color {
                egui::epaint::ColorMode::Solid(c) => Some(c),
                _ => None,
            },
            _ => None,
        })
        .collect();
    shapes
        .iter()
        .cloned()
        .map(|mut shape| {
            match &mut shape {
                egui::Shape::Rect(r) => {
                    r.stroke.color = none;
                    if outlines.contains(&r.fill) {
                        r.fill = none;
                    }
                }
                egui::Shape::Circle(c) => c.stroke.color = none,
                egui::Shape::Ellipse(e) => e.stroke.color = none,
                egui::Shape::LineSegment { stroke, .. } => stroke.color = none,
                egui::Shape::Path(p) => p.stroke.color = egui::epaint::ColorMode::Solid(none),
                _ => {}
            }
            shape
        })
        .collect()
}

/// `shapes` with each text's colours, the fallback a label without one of its own is lettered
/// in and an override, made transparent, every other value kept: what a listed difference, a
/// label's colour, leaves to compare.
fn labels_masked(shapes: &[egui::Shape]) -> Vec<egui::Shape> {
    let none = egui::Color32::TRANSPARENT;
    shapes
        .iter()
        .cloned()
        .map(|mut shape| {
            if let egui::Shape::Text(t) = &mut shape {
                t.fallback_color = none;
                t.override_text_color = t.override_text_color.map(|_| none);
            }
            shape
        })
        .collect()
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

#[test]
fn the_expander_draws_alike_from_both_models() -> TestResult {
    let expander = |open: bool, enabled: bool| {
        move |ui: &mut egui::Ui| {
            Expander::new("Details")
                .default_open(open)
                .enabled(enabled)
                .show(ui, |ui| ui.label("Body"))
                .header_response
        }
    };
    let (closed, open, closed_disabled, open_disabled) = (
        expander(false, true),
        expander(true, true),
        expander(false, false),
        expander(true, false),
    );
    assert_drawn_alike(&[
        ("expander closed", &closed),
        ("expander open", &open),
        ("expander closed disabled", &closed_disabled),
        ("expander open disabled", &open_disabled),
    ])
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
    assert_eq!(look.unchecked_background, egui::Color32::TRANSPARENT);
    let (parts, shapes) = drawn_switch(&patched_installed(Preset::Material3, none)?);
    let track = parts.get("track").ok_or("no track")?;
    let rect = painted_rect(&shapes, track).ok_or("no track painted")?;
    assert_eq!(rect.fill, egui::Color32::TRANSPARENT);
    assert_eq!(rect.stroke, egui::Stroke::NONE);
    Ok(())
}

/// `switch.label_gap`, here a gap kde-breeze does not state, puts the label that far after the
/// track; the label is lettered in `switch.label`'s colour where stated.
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

/// `switch.label(NONE).size_px` letters the label at that size in egui's `Proportional` family,
/// as the connector's role fonts are, here a size kde-breeze does not state; where it is
/// unstated (ios-13 states no font of `switch.label`) the size is egui's own; a size that is not
/// positive sends the switch to egui's own counterpart.
#[test]
fn a_switch_label_takes_its_stated_size() -> TestResult {
    use crate::switch::Geometry;
    let font = |shapes: &[egui::Shape]| {
        shapes.iter().find_map(|s| match s {
            egui::Shape::Text(t) => t
                .galley
                .job
                .sections
                .first()
                .map(|section| section.format.font_id.clone()),
            _ => None,
        })
    };
    let unstated = Geometry::next_of(&patched(Preset::Ios13, "")?).ok_or("no geometry")?;
    assert_eq!(unstated.label_size, None);
    let stock = Geometry::next_of(&patched(Preset::KdeBreeze, "")?).ok_or("no geometry")?;
    assert!(
        stock.label_size.is_some_and(|size| size != 31.0),
        "{stock:?}"
    );
    let (_, shapes) = drawn_switch(&patched_installed(Preset::KdeBreeze, "")?);
    let own = font(&shapes).ok_or("no label drawn")?;
    assert!(
        own.size != 31.0,
        "egui's own size is the stated one: the test proves nothing"
    );
    let patch = "[[widgets.switch.rules]]\nstates = []\nlabel.size_px = 31.0\n";
    let g = Geometry::next_of(&patched(Preset::KdeBreeze, patch)?).ok_or("no geometry")?;
    assert_eq!(g.label_size, Some(31.0));
    let (parts, shapes) = drawn_switch(&patched_installed(Preset::KdeBreeze, patch)?);
    assert_eq!(
        font(&shapes).ok_or("no label drawn")?,
        egui::FontId::proportional(31.0)
    );
    let label = parts.get("label").ok_or("no label")?;
    assert!(label.height() >= 31.0, "{label:?}");
    let zero = "[[widgets.switch.rules]]\nstates = []\nlabel.size_px = 0.0\n";
    assert_eq!(Geometry::next_of(&patched(Preset::KdeBreeze, zero)?), None);
    Ok(())
}

/// A checked switch reads its own states: a disabled checked track and thumb at
/// `DISABLED|CHECKED`, from a rule for that state alone, and a checked thumb's outline at
/// `thumb(CHECKED)`, drawn so.
#[test]
fn a_checked_switch_reads_its_checked_disabled_and_outlined_thumb() -> TestResult {
    use crate::switch::{Geometry, Look};
    let (red, green, blue) = (
        egui::Color32::from_rgb(255, 0, 0),
        egui::Color32::from_rgb(0, 255, 0),
        egui::Color32::from_rgb(0, 0, 255),
    );
    let patch = "[[widgets.switch.rules]]\nstates = [\"disabled\", \"checked\"]\n\
                 track.fill = \"#0000ff\"\nthumb.fill = \"#ff0000\"\n\n\
                 [[widgets.switch.rules]]\nstates = [\"checked\"]\n\
                 thumb.stroke = { width = 2.0, color = \"#00ff00\" }\n";
    let r = patched(Preset::KdeBreeze, patch)?;
    let look = Look::next_of(&r).ok_or("no look")?;
    assert_eq!(look.disabled_checked_background, Some(blue));
    assert_eq!(look.disabled_checked_thumb_color, Some(red));
    assert!(look.disabled_thumb_color != Some(red), "{look:?}");
    assert_eq!(look.checked_thumb_border_color, Some(green));
    assert!(look.thumb_border_color != Some(green), "{look:?}");
    let g = Geometry::next_of(&r).ok_or("no geometry")?;
    assert_eq!(g.checked_thumb_border, Some(2.0));
    let ctx = patched_installed(Preset::KdeBreeze, patch)?;
    let add = |ui: &mut egui::Ui| ui.add(Switch::new(&mut true));
    let _ = response(&ctx, at(1.0), add);
    let (out, _) = response(&ctx, at(1.0), add);
    let thumb = flat(&out.shapes)
        .into_iter()
        .find_map(|s| match s {
            egui::Shape::Circle(c) => Some(c),
            _ => None,
        })
        .ok_or("no thumb drawn")?;
    assert_eq!(
        thumb.stroke,
        egui::Stroke::new(2.0, egui::Color32::from_rgb(0, 255, 0))
    );
    Ok(())
}

/// A hovered track's stated fill is the whole fill it shows, taken as it is (spec §9, step 2),
/// not laid over its rest fill: windows-11's hovered on track is WinUI's
/// AccentFillColorSecondary, the accent at 0.9 opacity, `#0067c0e6` in light
/// (native-theme/tests/next_homes.toml, row `switch.hover_checked_background`).
#[test]
fn a_hovered_switch_track_is_its_stated_whole_fill() -> TestResult {
    let ctx = patched_installed(Preset::Windows11, "")?;
    let add = |ui: &mut egui::Ui| ui.add(Switch::new(&mut true).label("Wi-Fi"));
    let _ = response(&ctx, at(1.0), add);
    let (out, r) = response(&ctx, at(1.0), add);
    let track = Parts::of(&r)
        .unwrap_or_default()
        .get("track")
        .ok_or("no track")?;
    let rest = painted_rect(&flat(&out.shapes), track).ok_or("no track painted")?;
    let hover = egui::RawInput {
        events: vec![egui::Event::PointerMoved(track.center())],
        ..at(1.0)
    };
    let _ = response(&ctx, hover, add);
    let (out, r) = response(&ctx, at(1.0), add);
    assert!(r.hovered(), "the switch is not hovered");
    let hovered = painted_rect(&flat(&out.shapes), track).ok_or("no hovered track painted")?;
    let stated = egui::Color32::from_rgba_unmultiplied(0x00, 0x67, 0xc0, 0xe6);
    assert_ne!(
        rest.fill, stated,
        "the rest fill is the hovered one: the test proves nothing"
    );
    assert_eq!(hovered.fill, stated);
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

/// The radio is drawn from its own homes, not from the check box's cell: a theme that styles
/// the radio unlike the check box draws the radio's circle, outline, dot and label as stated.
#[test]
fn a_radio_styled_unlike_the_check_box_is_drawn_as_stated() -> TestResult {
    let drawn = |ctx: &egui::Context| {
        let add = |ui: &mut egui::Ui| ui.add(RadioButton::new(true, "Radio"));
        let _ = response(ctx, at(1.0), add);
        let (out, _) = response(ctx, at(1.0), add);
        let shapes = flat(&out.shapes);
        let circles: Vec<egui::epaint::CircleShape> = shapes
            .iter()
            .filter_map(|s| match s {
                egui::Shape::Circle(c) => Some(*c),
                _ => None,
            })
            .collect();
        let text = shapes.iter().find_map(|s| match s {
            egui::Shape::Text(t) => Some(
                t.galley
                    .job
                    .sections
                    .first()
                    .map(|s| s.format.color)
                    .filter(|&c| c != egui::Color32::PLACEHOLDER)
                    .unwrap_or(t.fallback_color),
            ),
            _ => None,
        });
        (circles, text)
    };
    let (green, red, blue, magenta) = (
        egui::Color32::from_rgb(0, 255, 0),
        egui::Color32::from_rgb(255, 0, 0),
        egui::Color32::from_rgb(0, 0, 255),
        egui::Color32::from_rgb(255, 0, 255),
    );
    let (stock, stock_text) = drawn(&patched_installed(Preset::KdeBreeze, "")?);
    let (circle, dot) = (
        stock.first().ok_or("no circle")?,
        stock.get(1).ok_or("no dot")?,
    );
    assert!(
        circle.fill != green
            && circle.stroke != egui::Stroke::new(2.0, red)
            && dot.fill != blue
            && stock_text != Some(magenta),
        "the check box's cell is the stated radio: the test proves nothing"
    );
    let patch = "[[widgets.radio.rules]]\nstates = []\nindicator.fill = \"#00ff00\"\n\
                 indicator.stroke = { width = 2.0, color = \"#ff0000\" }\n\
                 dot.fill = \"#0000ff\"\nlabel.color = \"#ff00ff\"\n";
    let (patched, text) = drawn(&patched_installed(Preset::KdeBreeze, patch)?);
    let (circle, dot) = (
        patched.first().ok_or("no circle")?,
        patched.get(1).ok_or("no dot")?,
    );
    assert_eq!(circle.fill, green);
    assert_eq!(circle.stroke, egui::Stroke::new(2.0, red));
    assert_eq!(dot.fill, blue);
    assert_eq!(text, Some(magenta));
    Ok(())
}

/// `radio.disabled_opacity` is the fade of a disabled radio as one widget over its backdrop:
/// its circle, in the stated disabled fill, is faded over the backdrop by the stated opacity.
#[test]
fn a_disabled_radio_is_faded_by_its_stated_opacity() -> TestResult {
    let ground = egui::Color32::from_rgb(0, 0, 0);
    let green = egui::Color32::from_rgb(0, 255, 0);
    let circle = |opacity: &str| -> Result<egui::Color32, Box<dyn std::error::Error>> {
        let patch = format!(
            "[widgets.radio]\ndisabled_opacity = {opacity}\n\n\
             [[widgets.radio.rules]]\nstates = [\"disabled\"]\nindicator.fill = \"#00ff00\"\n"
        );
        let ctx = patched_installed(Preset::KdeBreeze, &patch)?;
        let add = |ui: &mut egui::Ui| {
            ui.add(
                RadioButton::new(false, "Radio")
                    .enabled(false)
                    .backdrop(ground),
            )
        };
        let _ = response(&ctx, at(1.0), add);
        let (out, _) = response(&ctx, at(1.0), add);
        Ok(flat(&out.shapes)
            .into_iter()
            .find_map(|s| match s {
                egui::Shape::Circle(c) => Some(c.fill),
                _ => None,
            })
            .ok_or("no circle drawn")?)
    };
    let (quarter, half) = (circle("0.25")?, circle("0.5")?);
    assert_eq!(quarter, crate::fade::faded(ground, green, 0.25));
    assert_eq!(half, crate::fade::faded(ground, green, 0.5));
    assert_ne!(quarter, half);
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
        if stock.outline.is_none() {
            continue;
        }
        outlined += 1;
        let bar = Bar::next_of(&patched(preset, none)?);
        assert_eq!(bar.outline, None, "{preset:?}");
        assert_eq!(bar.radius, stock.radius, "{preset:?}");
    }
    assert!(
        outlined > 0,
        "no preset outlines its bar: the test proves nothing"
    );
    let pill = "[[widgets.progress_bar.rules]]\nstates = []\ntrack.corners = \"pill\"\n\
                track.min_size = [100.0, 6.0]\n";
    let bar = Bar::next_of(&patched(Preset::KdeBreeze, pill)?);
    assert_eq!(bar.radius, Some(3.0));
    assert_eq!(bar.min_width, Some(100.0));
    Ok(())
}

/// A progress bar whose corners egui cannot draw as one radius keeps egui's own, half its
/// height, and is still outlined as stated (spec §10, rule 0, per property).
#[test]
fn a_progress_bar_outline_stays_where_its_corners_are_egui_s() -> TestResult {
    use crate::progress_bar::Bar;
    let patch = "[[widgets.progress_bar.rules]]\nstates = []\n\
                 track.corners = { radius = 8.0, curve = \"continuous\" }\n\
                 track.stroke = { width = 2.0, color = \"#ff00ff\" }\n";
    let magenta = egui::Color32::from_rgb(255, 0, 255);
    let bar = Bar::next_of(&patched(Preset::KdeBreeze, patch)?);
    assert_eq!(bar.radius, None);
    assert_eq!(bar.outline, Some((2.0, magenta)));
    let ctx = patched_installed(Preset::KdeBreeze, patch)?;
    let add = |ui: &mut egui::Ui| ui.add(ProgressBar::new(0.4).desired_width(200.0));
    let _ = response(&ctx, at(1.0), add);
    let (out, r) = response(&ctx, at(1.0), add);
    let outline = flat(&out.shapes)
        .into_iter()
        .find_map(|s| match s {
            egui::Shape::Rect(rect) if rect.stroke.color == magenta => Some(rect),
            _ => None,
        })
        .ok_or("no outline drawn")?;
    assert_eq!(outline.rect, r.rect);
    assert_eq!(outline.stroke.width, 2.0);
    assert_eq!(
        outline.corner_radius,
        egui::CornerRadius::from(0.5 * r.rect.height())
    );
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
    let magenta = egui::Color32::from_rgb(255, 0, 255);
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

/// A slider thumb's stroke stated as none is no outline (spec §10, rule 2): a width of 0, so the
/// knob is not given egui's own `fg_stroke`, on every preset whose thumb has one.
#[test]
fn a_slider_thumb_stated_as_none_has_no_outline() -> TestResult {
    use crate::slider::{Geometry, Look};
    let none = "[[widgets.slider.rules]]\nstates = []\nthumb.stroke = \"none\"\n";
    let mut outlined = 0;
    for &(_, preset) in crate::next::parity::PRESETS {
        let stock = patched(preset, "")?;
        if Geometry::next_of(&stock, 1.0)
            .and_then(|g| g.outline)
            .is_none_or(|w| w == 0.0)
        {
            continue;
        }
        outlined += 1;
        let r = patched(preset, none)?;
        assert_eq!(
            Geometry::next_of(&r, 1.0).ok_or("no geometry")?.outline,
            Some(0.0),
            "{preset:?}"
        );
        let look = Look::next_of(&r);
        assert_eq!(look.thumb_border_color, None, "{preset:?}");
        let ctx = patched_installed(preset, none)?;
        let add = |ui: &mut egui::Ui| ui.add(Slider::new(&mut 0.5, 0.0..=1.0));
        let _ = response(&ctx, at(1.0), add);
        let (out, _) = response(&ctx, at(1.0), add);
        let knob = flat(&out.shapes)
            .into_iter()
            .find_map(|s| match s {
                egui::Shape::Circle(c) => Some(c),
                _ => None,
            })
            .ok_or("no knob drawn")?;
        assert_eq!(knob.stroke, egui::Stroke::NONE, "{preset:?}");
    }
    assert!(
        outlined > 0,
        "no preset outlines its thumb: the test proves nothing"
    );
    Ok(())
}

/// A slider's rail, trailing and knob fills stated as a gradient, which egui cannot paint as one
/// colour, are each egui's own: the rail and the resting knob in the scope's `inactive.bg_fill`,
/// the trailing fill only where `slider_trailing_fill` is on; the stated sizes stay (spec §10,
/// rule 0, per property).
#[test]
fn a_slider_fill_egui_cannot_paint_is_egui_s_own() -> TestResult {
    use crate::slider::{Geometry, Look};
    let gradient = "{ kind = \"linear_gradient\", angle_deg = 0.0, stops = [{ offset = 0.0, color = \"#ff0000\" }, { offset = 1.0, color = \"#0000ff\" }] }";
    let patch = format!(
        "[[widgets.slider.rules]]\nstates = []\ntrack.fill = {gradient}\n\
         active_track.fill = {gradient}\nthumb.fill = {gradient}\n"
    );
    let r = patched(Preset::KdeBreeze, &patch)?;
    let look = Look::next_of(&r);
    assert_eq!(
        (look.track_color, look.fill_color, look.thumb_color),
        (None, None, None)
    );
    let g = Geometry::next_of(&r, 1.0).ok_or("no geometry")?;
    assert!(g.thumb.is_some() && g.rail.is_some(), "{g:?}");
    let ctx = patched_installed(Preset::KdeBreeze, &patch)?;
    let own = std::cell::Cell::new(None);
    let add = |ui: &mut egui::Ui| {
        ui.native_scope(Role::Slider, RoleVariant::Normal, |ui| {
            let v = ui.visuals();
            own.set(Some((v.widgets.inactive.bg_fill, v.slider_trailing_fill)));
        });
        ui.add(Slider::new(&mut 0.5, 0.0..=1.0))
    };
    let _ = response(&ctx, at(1.0), add);
    let (out, r) = response(&ctx, at(1.0), add);
    let (rest, trailing) = own.get().ok_or("no scope")?;
    let parts = Parts::of(&r).ok_or("no parts")?;
    let shapes = flat(&out.shapes);
    let rail = painted_rect(&shapes, parts.get("track").ok_or("no rail")?).ok_or("no rail")?;
    assert_eq!(rail.fill, rest);
    let fill = painted_rect(&shapes, parts.get("fill").ok_or("no fill")?);
    assert_eq!(fill.is_some(), trailing, "{fill:?}");
    let knob = shapes
        .iter()
        .find_map(|s| match s {
            egui::Shape::Circle(c) => Some(c.fill),
            _ => None,
        })
        .ok_or("no knob drawn")?;
    assert_eq!(knob, rest);
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
    assert_eq!(
        look.hover,
        Some(egui::Color32::from_rgb(255, 0, 255)),
        "{preset:?}"
    );
    Ok(())
}

/// A drop-down field stroked per side, which egui cannot draw as one outline, keeps egui's
/// border width and the theme's stated padding and arrow column (spec §10, rule 0, per
/// property).
#[test]
fn a_combo_box_field_stroked_per_side_keeps_its_stated_sides() -> TestResult {
    use crate::combo_box::Sides;
    let per_side = "[[widgets.combo_box.rules]]\nstates = []\nfield.stroke = { top = { width = 1.0, paint = \"#ff0000\" }, right = { width = 0.0, paint = \"none\" }, bottom = { width = 2.0, paint = \"#00ff00\" }, left = { width = 0.0, paint = \"none\" } }\n";
    let mut padded = 0;
    for &(_, preset) in crate::next::parity::PRESETS {
        let stock = Sides::next_of(&patched(preset, "")?).ok_or("no stock sides")?;
        if stock.left.is_none() && stock.right.is_none() && stock.column.is_none() {
            continue;
        }
        padded += 1;
        let sides = Sides::next_of(&patched(preset, per_side)?).ok_or("no sides")?;
        assert_eq!(sides.border, None, "{preset:?}");
        assert_eq!(
            (sides.left, sides.right, sides.column),
            (stock.left, stock.right, stock.column),
            "{preset:?}"
        );
    }
    assert!(
        padded > 0,
        "no preset states the field's sides: the test proves nothing"
    );
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
        if !stock.is_some_and(|s| s.border.is_some_and(|w| w > 0.0)) {
            continue;
        }
        bordered += 1;
        let sides = Sides::next_of(&patched(preset, none)?).ok_or("no sides")?;
        assert_eq!(sides.border, Some(0.0), "{preset:?}");
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

/// A context drawn light with the atlas of `preset` installed, its themes resolved for the
/// text scale `scale`, with no font plan.
fn scaled_installed(
    preset: Preset,
    scale: f32,
) -> Result<egui::Context, Box<dyn std::error::Error>> {
    patched_scaled_installed(preset, "", scale)
}

/// [`scaled_installed`] with `patch` applied to the preset.
fn patched_scaled_installed(
    preset: Preset,
    patch: &str,
    scale: f32,
) -> Result<egui::Context, Box<dyn std::error::Error>> {
    let patch = native_theme_egui::native_theme::next::ThemePatch::from_toml(patch)?;
    let theme = NextTheme::preset(preset).with_patch(patch);
    let [light, light_inactive, dark, dark_inactive] = [
        ThemeContext::light(),
        ThemeContext::light().with_window_active(false),
        ThemeContext::dark(),
        ThemeContext::dark().with_window_active(false),
    ]
    .map(|ctx| theme.resolve(&ctx.with_text_scale(scale)));
    let ctx = egui::Context::default();
    ThemeAtlas::next_builder(&light, &light_inactive, &dark, &dark_inactive)
        .build()?
        .install(&ctx);
    ctx.set_theme(egui::Theme::Light);
    Ok(ctx)
}

/// The expander's header response and parts, drawn open, in the second of two passes.
fn drawn_expander(ctx: &egui::Context) -> (egui::FullOutput, egui::Response, Parts) {
    let add = |ui: &mut egui::Ui| {
        Expander::new("Details")
            .default_open(true)
            .show(ui, |ui| ui.label("Body"))
            .header_response
    };
    let _ = response(ctx, at(1.0), add);
    let (out, r) = response(ctx, at(1.0), add);
    let parts = Parts::of(&r).unwrap_or_default();
    (out, r, parts)
}

/// Above a text scale of 1 the header is the row rule's height of the 0.8.0 theme, on the
/// painted header (kde-breeze, adwaita) and on egui's own (material-3): the stated
/// `expander.header(NONE)` height grown to the line box of `expander.header_label(NONE)`'s size
/// times `typography.line_height_multiplier`, rounded up, and the padding above and below it.
/// That size holds the text scale already, so a header that scaled it a second time would be
/// taller; it is not 0.7.2's scaled size: kde-breeze's is rounded after scaling (spec §6,
/// step 2), and material-3's follows Android's non-linear font scaling (`FontScaleConverter`,
/// 14 sp to 26 dp at a scale of 2.0) where 0.7.2 multiplied.
#[test]
fn the_expander_header_grows_by_its_line_box_once() -> TestResult {
    use crate::expander::Layout;
    use native_theme_egui::native_theme::next::WidgetState;
    let mut faults = Vec::new();
    let mut grown = 0;
    for preset in [Preset::KdeBreeze, Preset::Adwaita, Preset::Material3] {
        let at_one = drawn_expander(&scaled_installed(preset, 1.0)?)
            .1
            .rect
            .height();
        for scale in [1.5, 2.0] {
            let r =
                NextTheme::preset(preset).resolve(&ThemeContext::light().with_text_scale(scale));
            let layout = Layout::next_of(&r);
            let title = r.expander.header_label(WidgetState::NONE);
            let line = (title.size_px * r.typography.line_height_multiplier).ceil();
            let [_, stated] = r.expander.header(WidgetState::NONE).min_size;
            let stated = stated.ok_or("no stated header height")?;
            let ctx = scaled_installed(preset, scale)?;
            let mut own = 0.0;
            let _ = pass(&ctx, at(1.0), |ui| {
                own = ui
                    .native_scope(Role::Expander, RoleVariant::Normal, |ui| {
                        ui.spacing().button_padding.y
                    })
                    .inner;
            });
            let padding = if layout.lays_out() {
                layout.padding.top.unwrap_or(own) + layout.padding.bottom.unwrap_or(own)
            } else {
                own + own
            };
            let want = stated.max(line + padding);
            let got = drawn_expander(&ctx).1.rect.height();
            if layout.line != Some(line) || got != want {
                faults.push(format!(
                    "{preset:?} at {scale}: the header is {got}, its line box {:?}; want {want}, \
                     {line}",
                    layout.line
                ));
            }
            if got > at_one {
                grown += 1;
            }
        }
    }
    assert!(faults.is_empty(), "{faults:#?}");
    assert!(
        grown > 0,
        "no header grows above a text scale of 1: the test proves nothing"
    );
    Ok(())
}

/// A line height stated on `expander.header_label(NONE)` is the title's line box above a text
/// scale of 1, on the painted header (kde-breeze, adwaita) and on egui's own (material-3), in
/// place of its size times `typography.line_height_multiplier`: the header is the stated height
/// grown to that line box, rounded up, and the padding above and below it.
#[test]
fn the_expander_header_grows_by_its_stated_line_height() -> TestResult {
    use crate::expander::Layout;
    use native_theme_egui::native_theme::next::WidgetState;
    let patch = "[[widgets.expander.rules]]\nstates = []\nheader_label.line_height = 61.0\n";
    let scale = 2.0;
    let mut faults = Vec::new();
    for preset in [Preset::KdeBreeze, Preset::Adwaita, Preset::Material3] {
        let resolved = |patch: &str| -> Result<NextResolved, Box<dyn std::error::Error>> {
            let patch = native_theme_egui::native_theme::next::ThemePatch::from_toml(patch)?;
            Ok(NextTheme::preset(preset)
                .with_patch(patch)
                .resolve(&ThemeContext::light().with_text_scale(scale)))
        };
        let (stock, r) = (resolved("")?, resolved(patch)?);
        let title = r.expander.header_label(WidgetState::NONE);
        let line = title.line_height.ceil();
        let stock_title = stock.expander.header_label(WidgetState::NONE);
        let by_size = (stock_title.size_px * stock.typography.line_height_multiplier).ceil();
        assert_ne!(
            line, by_size,
            "{preset:?}: the stated line box is the size's: the test proves nothing"
        );
        let layout = Layout::next_of(&r);
        let [_, stated] = r.expander.header(WidgetState::NONE).min_size;
        let stated = stated.ok_or("no stated header height")?;
        let ctx = patched_scaled_installed(preset, patch, scale)?;
        let mut own = 0.0;
        let _ = pass(&ctx, at(1.0), |ui| {
            own = ui
                .native_scope(Role::Expander, RoleVariant::Normal, |ui| {
                    ui.spacing().button_padding.y
                })
                .inner;
        });
        let padding = if layout.lays_out() {
            layout.padding.top.unwrap_or(own) + layout.padding.bottom.unwrap_or(own)
        } else {
            own + own
        };
        let want = stated.max(line + padding);
        let got = drawn_expander(&ctx).1.rect.height();
        if layout.line != Some(line) || got != want {
            faults.push(format!(
                "{preset:?}: the header is {got}, its line box {:?}; want {want}, {line}",
                layout.line
            ));
        }
    }
    assert!(faults.is_empty(), "{faults:#?}");
    Ok(())
}

/// A stated arrow colour fills the arrow, on the painted header and on egui's own, and a
/// stated arrow size is the painted arrow's box.
#[test]
fn an_expander_arrow_takes_its_stated_colour_and_size() -> TestResult {
    use crate::expander::{Layout, Look};
    let magenta = egui::Color32::from_rgb(255, 0, 255);
    let patch = "[[widgets.expander.rules]]\nstates = []\narrow.color = \"#ff00ff\"\n\
                 arrow.size = 23.0\n";
    let mut painted = 0;
    for &(_, preset) in crate::next::parity::PRESETS {
        let stock = patched(preset, "")?;
        assert_ne!(
            Look::next_of(&stock).arrow_color,
            Some(magenta),
            "{preset:?}: the stated colour is the preset's: the test proves nothing"
        );
        assert_ne!(
            Layout::next_of(&stock).arrow_size,
            Some(23.0),
            "{preset:?}: the stated size is the preset's: the test proves nothing"
        );
        let (out, _, parts) = drawn_expander(&patched_installed(preset, patch)?);
        let filled = flat(&out.shapes).iter().any(|s| match s {
            egui::Shape::Path(p) => p.fill == magenta,
            _ => false,
        });
        assert!(filled, "{preset:?}: no arrow filled in the stated colour");
        if Layout::next_of(&patched(preset, patch)?).lays_out() {
            painted += 1;
            let arrow = parts.get("arrow").ok_or("no arrow")?;
            assert_eq!(arrow.size(), egui::vec2(23.0, 23.0), "{preset:?}");
        }
    }
    assert!(
        painted > 0,
        "no preset paints its header: the test proves nothing"
    );
    Ok(())
}

/// The stated frame, body indent and arrow gap lay the expander out, on every preset, those
/// whose own header is egui's included; a trailing arrow lies after the title.
#[test]
fn an_expander_takes_its_stated_frame_indent_gap_and_side() -> TestResult {
    use crate::expander::Layout;
    let leading = "[widgets.expander]\narrow_side = \"leading\"\narrow_gap = 23.0\n\
                   content_indent = 31.0\nframe_enabled = true\n";
    let trailing = "[widgets.expander]\narrow_side = \"trailing\"\n";
    let mut unframed = 0;
    for &(_, preset) in crate::next::parity::PRESETS {
        let stock = Layout::next_of(&patched(preset, "")?);
        assert!(
            stock.arrow_gap != Some(23.0) && stock.content_indent != Some(31.0),
            "{preset:?}: the stated gap or indent is the preset's: the test proves nothing"
        );
        if stock.frame_enabled != Some(true) {
            unframed += 1;
        }
        let ctx = patched_installed(preset, leading)?;
        let mut body = None;
        let mut add = |ui: &mut egui::Ui| {
            let r = Expander::new("Details")
                .default_open(true)
                .show(ui, |ui| ui.label("Body"));
            body = r.body_returned.map(|l| l.rect);
            r.header_response
        };
        let _ = response(&ctx, at(1.0), &mut add);
        let (_, header) = response(&ctx, at(1.0), &mut add);
        let body = body.ok_or("no body")?;
        let parts = Parts::of(&header).ok_or("no parts")?;
        let (arrow, title) = (
            parts.get("arrow").ok_or("no arrow")?,
            parts.get("title").ok_or("no title")?,
        );
        assert!(parts.get("frame").is_some(), "{preset:?}: no frame");
        assert_eq!(body.left() - header.rect.left(), 31.0, "{preset:?}");
        assert_eq!(title.left() - arrow.right(), 23.0, "{preset:?}");
        let (_, _, parts) = drawn_expander(&patched_installed(preset, trailing)?);
        let (arrow, title) = (
            parts.get("arrow").ok_or("no arrow")?,
            parts.get("title").ok_or("no title")?,
        );
        assert!(
            arrow.left() > title.right(),
            "{preset:?}: {arrow:?} {title:?}"
        );
    }
    assert!(
        unframed > 0,
        "every preset frames its expander: the test proves nothing"
    );
    Ok(())
}
