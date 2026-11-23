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
        Some(SliderLook::of(old)),
        Some(SliderLook::next_of(new)),
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
