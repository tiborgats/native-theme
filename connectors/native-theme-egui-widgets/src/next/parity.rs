//! Connector parity of the widgets (docs/v0.8.0-plan.md, section 1, rule 2; spec §10, rule 4):
//! each widget's look and geometry read from a 0.8.0 theme (`next_of`) equal what its 0.7.2
//! reads give (`of`), field by field, on the seven platform presets ([`PRESETS`], asserted
//! against `Preset::ALL`), light and dark. A widget paints from its look and geometry alone, so
//! equal reads paint alike.
//!
//! The 0.7.2 side is resolved as the connector's root parity resolves it (the Apple presets at
//! 72 font DPI, the others at 96, `ResolutionContext::for_tests`), the 0.8.0 side for the
//! scheme's default context, whose text scale is 1. Every widget's look is given on these
//! presets, so a side's `None` (the widget drawn as egui's own) is a difference too. A
//! difference is allowed only as an entry of [`DIFFERENCES`], with its reason; an entry that no
//! longer differs fails.
#![allow(
    clippy::indexing_slicing,
    reason = "a test fails by panicking; the indexes are of fixed-size arrays"
)]

use std::fmt::Debug;

use native_theme_egui::ResolvedTheme;
use native_theme_egui::native_theme;
use native_theme_egui::native_theme::next::{Preset, ThemeContext};

use crate::combo_box::Sides;
use crate::expander::{Layout as ExpanderLayout, Look as ExpanderLook};
use crate::links::Look as LinkLook;
use crate::progress_bar::Bar;
use crate::radio_button::{Geometry as RadioGeometry, Look as RadioLook};
use crate::segmented_control::Look as SegmentedLook;
use crate::slider::{Geometry as SliderGeometry, Look as SliderLook};
use crate::spinner::Look as SpinnerLook;
use crate::switch::{Geometry as SwitchGeometry, Look as SwitchLook};

type TestResult = Result<(), Box<dyn std::error::Error>>;

/// The presets compared, each by its 0.7.2 key and its 0.8.0 preset.
pub(crate) const PRESETS: &[(&str, Preset)] = &[
    ("kde-breeze", Preset::KdeBreeze),
    ("adwaita", Preset::Adwaita),
    ("windows-11", Preset::Windows11),
    ("macos-monterey", Preset::MacosMonterey),
    ("macos-tahoe", Preset::MacosTahoe),
    ("material-3", Preset::Material3),
    ("ios-13", Preset::Ios13),
];

/// A listed difference: the leaf, the presets on which it differs, and why.
struct Difference {
    leaf: &'static str,
    presets: &'static [&'static str],
    reason: &'static str,
}

/// The listed differences.
const DIFFERENCES: &[Difference] = &[
    Difference {
        leaf: "radio_button::Geometry.width",
        presets: &[
            "kde-breeze",
            "adwaita",
            "windows-11",
            "macos-monterey",
            "macos-tahoe",
            "ios-13",
        ],
        reason: "0.7.2 leaves checkbox.radio_indicator_width unstated, so the radio keeps its \
                 Checkbox cell's icon_width (checkbox.indicator_width); the 0.8.0 theme states \
                 radio.indicator(NONE).min_size, the same width (the table of homes: if_null \
                 checkbox.indicator_width): drawn alike, \
                 next_drawn::the_radio_button_draws_alike_from_both_models",
    },
    Difference {
        leaf: "radio_button::Geometry.label_gap",
        presets: &[
            "kde-breeze",
            "adwaita",
            "windows-11",
            "macos-monterey",
            "macos-tahoe",
            "material-3",
            "ios-13",
        ],
        reason: "0.7.2 has no radio label gap, so the radio keeps its Checkbox cell's \
                 icon_spacing; the 0.8.0 theme states radio.label_gap, the same space: drawn \
                 alike, next_drawn::the_radio_button_draws_alike_from_both_models",
    },
    Difference {
        leaf: "switch::Look.disabled_text",
        presets: &["kde-breeze", "adwaita"],
        reason: "0.7.2 has no switch label and the look took defaults.disabled_text_color; the \
                 0.8.0 presets state the platform's disabled switch label: kde-breeze \
                 qqc2-desktop-style's, the Disabled [Colors:Window] text at its opacity 0.6 \
                 (#a0a1a399 / #686a6c99: Switch.qml:72, :75 at qqc2-desktop-style v6.31.0; \
                 plasmadesktoptheme.cpp:273-274 at Kirigami 8319acc7), adwaita AdwSwitchRow's \
                 title, the row's text at --disabled-opacity 50 % (#00000666 / #ffffff80: \
                 _labels.scss:8-10, _colors.scss:37, :100, :320 at libadwaita 1.10.0)",
    },
    Difference {
        leaf: "switch::Look.disabled_text",
        presets: &["material-3"],
        reason: "0.7.2 has no switch label and the look took defaults.disabled_text_color, \
                 on-surface at 0.38; the 0.8.0 material-3 states Android's MaterialSwitch \
                 text, on-background at material_emphasis_medium 0.6 when disabled \
                 (#1d1b2099 / #e6e0e999: android:Widget.CompoundButton's textColor \
                 ?attr/textColorPrimaryDisableOnly, AOSP core/res/res/values/styles.xml:386 at \
                 android-15.0.0_r1; m3_primary_text_disable_only.xml:19, dimens.xml:21, \
                 materialswitch/res/values/styles.xml:17-29 at material-components-android \
                 66c334b)",
    },
    Difference {
        leaf: "switch::Look.text",
        presets: &["adwaita"],
        reason: "0.7.2 has no switch label and the look took the window's text, #2e3436 in \
                 light (no source at libadwaita, next/presets/adwaita.rs); the 0.8.0 adwaita \
                 states AdwSwitchRow's title in the row's text, view_fg_color and \
                 card_fg_color RGB(0 0 6 / 80%) (#000006cc: _lists.scss:5-7, :521; \
                 _misc.scss:202-204; _colors.scss:37, :62 at libadwaita 1.10.0); in dark both \
                 are white",
    },
    Difference {
        leaf: "switch::Look.hover_text",
        presets: &["adwaita"],
        reason: "0.7.2 has no switch label and lettered a hovered one in its rest colour, the \
                 window's text, #2e3436 in light; the 0.8.0 adwaita states the hovered label \
                 (switch.label(HOVER)), #000005cc in light, AdwSwitchRow's title as \
                 switch::Look.text's entry gives it; in dark both are white",
    },
    Difference {
        leaf: "switch::Look.disabled_text",
        presets: &["macos-monterey", "macos-tahoe"],
        reason: "0.7.2 has no switch label and the look took defaults.disabled_text_color; the \
                 0.8.0 macOS presets state the disabled switch label (switch.label(DISABLED)) \
                 in labelColor as at rest, #000000d8 / #ffffffd8, the measured SwiftUI \
                 Toggle's (os-probe run 45210835145, switch-label-macos-{15,26}/measure.toml \
                 `*.normal.toggle_*_disabled` within 3 levels of the labelColor reference)",
    },
    Difference {
        leaf: "switch::Look.disabled_text",
        presets: &["ios-13"],
        reason: "0.7.2 has no switch label and the look took defaults.disabled_text_color; the \
                 0.8.0 ios-13 states the disabled switch label (switch.label(DISABLED)) in the \
                 label colour as at rest, #000000 / #ffffff, the list row's UILabel beside a \
                 disabled UISwitch (ios-host run 45230061151, iOS 18.6, measure_lists.toml \
                 `run1.\"<a>\".switch_row.disabled` `label_text_color`)",
    },
    Difference {
        leaf: "switch::Look.hover_text",
        presets: &["ios-13"],
        reason: "0.7.2 has no switch label and lettered a hovered one in its rest colour; the \
                 0.8.0 ios-13 states no hovered switch label (no pointer on an iPhone), so the \
                 switch letters it in egui's own hovered label colour, the scope's \
                 widgets.hovered.fg_stroke, as egui's Checkbox does (spec §10, rule 0); \
                 next_drawn::an_unstated_hovered_switch_label_is_egui_s_own",
    },
    Difference {
        leaf: "switch::Look.hover_checked_background",
        presets: &["adwaita"],
        reason: "0.7.2's adwaita states an uncited switch.hover_checked_background; 0.8.0's \
                 adwaita states libadwaita's hovered on track, currentColor (the accent's text) \
                 at 10 % over the accent, #4990e7 (`&:checked { &:hover … }`, _switch.scss:41-46, \
                 _checks.scss:58 at libadwaita 1.10.0), the whole fill the look takes as it is \
                 (spec §9, step 2)",
    },
    Difference {
        leaf: "switch::Look.hover_unchecked_background",
        presets: &["macos-monterey"],
        reason: "0.7.2's macos-monterey states an uncited hover fill (#c8c8cd / #4a4a4c); \
                 0.8.0's states the hovered off track as at rest (os-probe runs 45208260830, \
                 macOS 15, and 45217592058, macOS 26: `*.hovered.switch_off` differs from the \
                 normal capture in 0 pixels), so the hovered track paints its rest fill",
    },
    Difference {
        leaf: "segmented_control::Look.hover",
        presets: &["macos-monterey"],
        reason: "0.7.2's macos-monterey states an uncited hover fill (#e0e0e5 / #444446); \
                 0.8.0's states no hovered fill of a segment's own, the hovered segment \
                 differing from its normal capture in 0 pixels (os-probe run 45208260830, \
                 `*.hovered.segment`, macOS 15 and 26), so the hovered segment paints its rest \
                 fill",
    },
    Difference {
        leaf: "slider::Look.thumb_hover_color",
        presets: &["macos-monterey"],
        reason: "0.7.2's macos-monterey states an uncited hover fill (#e0e0e0); 0.8.0's states \
                 the hovered knob as at rest (os-probe runs 45198750893, macOS 15, and \
                 45217592058, macOS 26: `*.hovered.slider_knob` differs from the normal capture \
                 in 0 pixels), so the hovered knob paints its rest fill",
    },
    Difference {
        leaf: "slider::Look.thumb_color",
        presets: &["ios-13"],
        reason: "0.7.2's ios-13 states no slider.thumb_color, which its resolver fills in \
                 (#f2f2f7 / #1c1c1e); 0.8.0's states the \
                 measured thumb, white in both schemes (ios-host run 45230061151, iOS 18.6, \
                 measure_lists.toml `press1.\"<a>\".slider.normal` `fill`)",
    },
    Difference {
        leaf: "switch::Look.pressed_text",
        presets: &["adwaita"],
        reason: "0.7.2 has no switch label and lettered a pressed one in its rest colour, the \
                 window's text, #2e3436 in light; the 0.8.0 adwaita states the pressed label \
                 (switch.label(PRESSED)), #000005cc in light, AdwSwitchRow's title as \
                 switch::Look.text's entry gives it; in dark both are white",
    },
    Difference {
        leaf: "switch::Geometry.label_gap",
        presets: &["kde-breeze", "adwaita", "windows-11"],
        reason: "0.7.2 has no switch label gap, so the label lies at the scope's own \
                 icon_spacing; the 0.8.0 presets state the platform's: kde-breeze 4, \
                 qqc2-desktop-style's Switch spacing, Kirigami.Units.smallSpacing \
                 (Switch.qml:26, :70 at qqc2-desktop-style v6.31.0; units.cpp:37 at Kirigami \
                 8319acc7), the scope's own there, so drawn alike; adwaita 6, the least gap of \
                 AdwSwitchRow's header box (_lists.scss:132 at libadwaita 1.10.0); windows-11 \
                 12, ToggleSwitch's fixed column (ToggleSwitch_themeresources.xaml:499-503 at \
                 microsoft-ui-xaml 8463f45): next_drawn lists both",
    },
    Difference {
        leaf: "switch::Geometry.label_gap",
        presets: &["macos-monterey", "macos-tahoe"],
        reason: "0.7.2 has no switch label gap, so the label lies at the scope's own \
                 icon_spacing; the 0.8.0 macOS presets state 8, the measured SwiftUI Toggle's \
                 label 8 px before its switch (os-probe run 45210835145, macOS 15 and 26, \
                 switch-label.toml `[[item]] toggle_off` fitting width 33 + 8 + the switch): \
                 next_drawn lists the drawn switch",
    },
    Difference {
        leaf: "switch::Geometry.label_size",
        presets: &[
            "kde-breeze",
            "adwaita",
            "windows-11",
            "macos-monterey",
            "macos-tahoe",
            "material-3",
            "ios-13",
        ],
        reason: "0.7.2's SwitchTheme states no label font, so the label is lettered in the \
                 scope's own size; the 0.8.0 presets state switch.label as the platform's \
                 switch label, the body font (kde-breeze qqc2-desktop-style's control.font, \
                 adwaita AdwSwitchRow's title, windows-11 ToggleSwitch's content, macOS the \
                 body font, material-3 Android's MaterialSwitch text, ios-13 the list row's \
                 UILabel beside a UISwitch in the Body text style, 17 pt, ios-host run \
                 45230061151 measure_lists.toml `run1.\"<a>\".switch_row.normal` \
                 `label_font`), the size the scope's \
                 own is: drawn alike, next_drawn::the_switch_draws_alike_from_both_models",
    },
    Difference {
        leaf: "switch::Geometry.leading",
        presets: &[
            "adwaita",
            "macos-monterey",
            "macos-tahoe",
            "material-3",
            "ios-13",
        ],
        reason: "0.7.2's SwitchTheme states no label side, so the label lies after the track, \
                 as egui's own Checkbox lays its text; the 0.8.0 presets state \
                 switch.label_side Leading, the platform's label before its switch: adwaita \
                 AdwSwitchRow's title first, the switch a suffix at the row's end \
                 (adw-action-row.ui:40-43, :91-94, adw-switch-row.c:161 at libadwaita 1.10.0), \
                 macOS and iOS SwiftUI's ToggleStyle.switch, \"a leading label and a trailing \
                 switch\" (developer.apple.com, read 2026-11-23), material-3 Android's \
                 MaterialSwitch text at the start and the switch at the view's end \
                 (materialswitch/res/values/styles.xml:26 at material-components-android \
                 66c334b; androidx SwitchCompat.java:1217-1219, :1392-1400); next_drawn lists \
                 the drawn switch",
    },
    Difference {
        leaf: "switch::Geometry.stretches",
        presets: &["adwaita", "material-3", "ios-13"],
        reason: "0.7.2's SwitchTheme states no stretching gap, so the label keeps to the \
                 track; the 0.8.0 presets state switch.label_gap_stretches true, the label and \
                 the switch at the ends of a row wider than they are, as the sources of \
                 switch::Geometry.leading give it; a switch no wider than its content, as \
                 next_drawn draws it, lies alike",
    },
    Difference {
        leaf: "switch::Look.hover_checked_background",
        presets: &["windows-11"],
        reason: "0.7.2's switch.hover_checked_background is the window colour at alpha 0.1, \
                 a layer the look composites over the on track; 0.8.0's windows-11 states \
                 WinUI's whole hovered fill, which the look takes as it is (spec §9, step 2): \
                 AccentFillColorSecondary, the accent's fill at 0.9 opacity, #0067c0e6 / \
                 #4cc2ffe6 (ToggleSwitchFillOnPointerOver, ToggleSwitch_themeresources.xaml:144, \
                 :24; Common_themeresources_any.xaml:330, :126 at microsoft-ui-xaml 8463f45; \
                 native-theme/tests/next_homes.toml, row switch.hover_checked_background, a \
                 listed difference); next_drawn::a_hovered_switch_track_is_its_stated_whole_fill",
    },
    Difference {
        leaf: "slider::Look.thumb_hover_color",
        presets: &["adwaita"],
        reason: "0.7.2's adwaita states an uncited slider.thumb_hover_color (#ebebeb / \
                 #3a3a3e); 0.8.0's adwaita states libadwaita's $slider_hover_color, white in \
                 both schemes (_scale.scss:62-76, _colors.scss:289 at libadwaita 1.10.0; \
                 native-theme/tests/next_homes.toml, row slider.thumb_hover_color, a listed \
                 difference), so the hovered knob is white",
    },
    Difference {
        leaf: "combo_box::Sides.border",
        presets: &["macos-tahoe", "ios-13"],
        reason: "macos-tahoe: 0.7.2 states the default line (0.5); 0.8.0 states the pop-up \
                 button's outline as none, as macOS 26 draws it (os-probe run 45198750893, \
                 macOS 26, `*.normal.popup` `edge_*`: every edge pixel is the fill); ios-13: \
                 0.7.2 states the default border width (0.5, uncited); 0.8.0 states the border \
                 WebKit draws round an iOS select, 1px (html.css:1101 at WebKit 31adf9d); \
                 native-theme/tests/next_homes.toml, row combo_box.border.line_width, a listed \
                 difference; next_drawn lists the drawn field",
    },
];

/// The leaves compared and the differences found, for one preset and scheme.
struct Report {
    label: String,
    preset: &'static str,
    compared: usize,
    failures: Vec<String>,
    listed: Vec<&'static str>,
}

impl Report {
    fn leaf<T: PartialEq + Debug>(&mut self, leaf: &'static str, old: &T, new: &T) {
        self.compared += 1;
        if old == new {
            return;
        }
        if DIFFERENCES
            .iter()
            .any(|d| d.leaf == leaf && d.presets.contains(&self.preset))
        {
            self.listed.push(leaf);
            return;
        }
        self.failures.push(format!(
            "{}: {leaf}: 0.7.2 {old:?}, 0.8.0 {new:?}",
            self.label
        ));
    }
}

/// Compare two optional structs field by field: the struct is destructured whole, so a field
/// added to it and not named here fails to compile. Where one side gives none, the struct's
/// presence is the leaf.
macro_rules! compare {
    ($report:expr, $leaf:literal, $ty:ident, $old:expr, $new:expr, [$($field:ident),* $(,)?]) => {
        match ($old, $new) {
            (Some(old), Some(new)) => {
                let $ty { $($field),* } = old;
                $( $report.leaf(concat!($leaf, ".", stringify!($field)), &$field, &new.$field); )*
            }
            (old, new) => $report.leaf($leaf, &old.is_some(), &new.is_some()),
        }
    };
}

/// Both models of `key`, light or dark.
pub(crate) fn resolve(
    key: &str,
    preset: Preset,
    is_dark: bool,
) -> Result<(ResolvedTheme, native_theme::next::ResolvedTheme), Box<dyn std::error::Error>> {
    let (mode, ctx) = if is_dark {
        (native_theme::theme::ColorMode::Dark, ThemeContext::dark())
    } else {
        (native_theme::theme::ColorMode::Light, ThemeContext::light())
    };
    let apple = matches!(
        preset,
        Preset::MacosMonterey | Preset::MacosTahoe | Preset::Ios13
    );
    let context = native_theme::ResolutionContext {
        font_dpi: if apple { 72.0 } else { 96.0 },
        ..native_theme::ResolutionContext::for_tests()
    };
    let old = native_theme::theme::Theme::preset(key)?
        .into_variant(mode)?
        .into_resolved(&context)?;
    let new = native_theme::next::Theme::preset(preset).resolve(&ctx);
    Ok((old, new))
}

/// Every widget's look and geometry, of one preset and scheme.
fn compare_all(report: &mut Report, old: &ResolvedTheme, new: &native_theme::next::ResolvedTheme) {
    // The width a slider is given by its `Ui`: egui's own.
    let slider_width = native_theme_egui::egui::Style::default()
        .spacing
        .slider_width;
    compare!(
        report,
        "switch::Look",
        SwitchLook,
        Some(SwitchLook::of(old).painted()),
        SwitchLook::next_of(new).map(SwitchLook::painted),
        [
            checked_background,
            hover_checked_background,
            unchecked_background,
            hover_unchecked_background,
            disabled_checked_background,
            disabled_unchecked_background,
            thumb_background,
            unchecked_thumb_background,
            disabled_thumb_color,
            disabled_checked_thumb_color,
            unchecked_track_border_color,
            checked_track_border_color,
            disabled_unchecked_track_border_color,
            disabled_checked_track_border_color,
            thumb_border_color,
            checked_thumb_border_color,
            text,
            hover_text,
            pressed_text,
            disabled_text,
            background,
        ]
    );
    compare!(
        report,
        "switch::Geometry",
        SwitchGeometry,
        SwitchGeometry::of(&old.switch).map(SwitchGeometry::painted),
        SwitchGeometry::next_of(new).map(SwitchGeometry::painted),
        [
            track,
            radius,
            thumb,
            unchecked_thumb,
            track_border,
            thumb_border,
            checked_thumb_border,
            label_gap,
            label_size,
            leading,
            stretches,
        ]
    );
    compare!(
        report,
        "radio_button::Look",
        RadioLook,
        Some(RadioLook::of(old)),
        Some(RadioLook::next_of(new)),
        [background]
    );
    compare!(
        report,
        "radio_button::Geometry",
        RadioGeometry,
        Some(RadioGeometry::of(old)),
        Some(RadioGeometry::next_of(new)),
        [dot, width, label_gap]
    );
    compare!(
        report,
        "progress_bar::Bar",
        Bar,
        Some(Bar::of(old)),
        Some(Bar::next_of(new)),
        [min_width, radius, outline]
    );
    compare!(
        report,
        "spinner::Look",
        SpinnerLook,
        Some(SpinnerLook::of(old)),
        Some(SpinnerLook::next_of(new)),
        [fill_color, text_color, diameter, stroke_width]
    );
    compare!(
        report,
        "links::Look",
        LinkLook,
        Some(LinkLook::of(old).painted()),
        Some(LinkLook::next_of(new).painted()),
        [
            text_color,
            background_color,
            hover_background,
            hover_text_color,
            active_text_color,
            visited_text_color,
            disabled_text_color,
            underline_enabled,
        ]
    );
    compare!(
        report,
        "segmented_control::Look",
        SegmentedLook,
        Some(SegmentedLook::of(old)),
        Some(SegmentedLook::next_of(new)),
        [background, hover, padded]
    );
    compare!(
        report,
        "slider::Look",
        SliderLook,
        Some(SliderLook::of(old).painted()),
        Some(SliderLook::next_of(new).painted()),
        [
            track_color,
            fill_color,
            thumb_color,
            thumb_hover_color,
            disabled_track_color,
            disabled_fill_color,
            disabled_thumb_color,
            thumb_border_color,
            thumb_hover_border_color,
            thumb_focus_border_color,
            background,
        ]
    );
    compare!(
        report,
        "slider::Geometry",
        SliderGeometry,
        SliderGeometry::of(&old.slider, slider_width),
        SliderGeometry::next_of(new, slider_width),
        [thumb, rail, width, outline]
    );
    compare!(
        report,
        "combo_box::Sides",
        Sides,
        Sides::of(old),
        Sides::next_of(new),
        [left, right, border, column]
    );
    compare!(
        report,
        "expander::Look",
        ExpanderLook,
        Some(ExpanderLook::of(old)),
        Some(ExpanderLook::next_of(new)),
        [arrow_color, frame]
    );
    compare!(
        report,
        "expander::Layout",
        ExpanderLayout,
        Some(ExpanderLayout::of(
            old,
            &native_theme_egui::AccessibilityPreferences::default()
        )),
        Some(ExpanderLayout::next_of(new)),
        [
            trailing,
            arrow_gap,
            content_indent,
            frame_enabled,
            padding,
            arrow_size,
            line,
        ]
    );
}

#[test]
fn the_compared_presets_are_every_platform_preset() {
    let compared: Vec<Preset> = PRESETS.iter().map(|&(_, preset)| preset).collect();
    let misnamed: Vec<&str> = PRESETS
        .iter()
        .filter(|&&(key, preset)| key != preset.as_str())
        .map(|&(key, _)| key)
        .collect();
    let missing: Vec<&Preset> = Preset::ALL
        .iter()
        .filter(|p| !compared.contains(p))
        .collect();
    assert!(
        compared.len() == Preset::ALL.len() && missing.is_empty() && misnamed.is_empty(),
        "{} presets compared, {} platform presets; not compared: {missing:?}; keys that are not \
         their preset's name: {misnamed:?}",
        compared.len(),
        Preset::ALL.len()
    );
}

#[test]
fn every_widget_reads_the_same_from_both_models() -> TestResult {
    let mut failures = Vec::new();
    let mut listed = Vec::new();
    for &(key, preset) in PRESETS {
        for is_dark in [false, true] {
            let (old, new) = resolve(key, preset, is_dark)?;
            let mut report = Report {
                label: format!("{key}/{}", if is_dark { "dark" } else { "light" }),
                preset: key,
                compared: 0,
                failures: Vec::new(),
                listed: Vec::new(),
            };
            compare_all(&mut report, &old, &new);
            println!("{}: {} leaves compared", report.label, report.compared);
            failures.extend(report.failures);
            listed.extend(report.listed.into_iter().map(|leaf| (key, leaf)));
        }
    }
    for d in DIFFERENCES {
        for preset in d.presets {
            if !listed.contains(&(*preset, d.leaf)) {
                failures.push(format!(
                    "{preset}: {}: listed as a difference ({}), but equal: remove the entry",
                    d.leaf, d.reason
                ));
            }
        }
    }
    assert!(
        failures.is_empty(),
        "{} leaves differ:\n{}",
        failures.len(),
        failures.join("\n")
    );
    Ok(())
}
