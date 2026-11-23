//! Connector parity of the 0.8.0 work (docs/v0.8.0-plan.md, section 1, rule
//! 2; M4, step 1): what the connector hands to iced, computed from the 0.7.2
//! model by the connector's public mapping and from the 0.8.0 model by
//! [`super`], is equal: the iced theme and the style each ported style
//! function returns, in every status of its widget.
//!
//! Presets covered: kde-breeze, adwaita, windows-11, macos-monterey,
//! macos-tahoe and material-3, light and dark ([`PRESETS`]). The 0.7.2 side
//! is resolved as the connector's tests resolve it
//! (`ResolutionContext::for_tests`), with no accessibility scaling; the
//! 0.8.0 side for the scheme's default context, whose text scale is 1.
//!
//! The form of comparison per iced type the connector produces:
//!
//! - `iced_core::Theme` (with its `Palette` and `palette::Extended`) derives
//!   `PartialEq`: the palette, the extended palette and the name are each
//!   compared by it, the name being the theme's own on either side.
//! - Each widget's `Style` that a style function returns —
//!   `button::Style`, `text_input::Style`, `text_editor::Style`,
//!   `checkbox::Style`, `radio::Style`, `toggler::Style`,
//!   `pick_list::Style`, `overlay::menu::Style`, `slider::Style`,
//!   `scrollable::Style`, `progress_bar::Style`, `rule::Style`,
//!   `pane_grid::Style`, `container::Style` — derives `PartialEq` and is
//!   compared by it, the function called with its side's own iced theme in
//!   every `Status` of the widget (the style functions read iced's own
//!   fields from the theme in effect).
//! - `scrollable::Scrollbar`, which [`crate::styles::scrollbar`] returns,
//!   has `Debug` but no `PartialEq`, and is compared in its `Debug` form.
//! - The `iced_aw` styles (feature `iced_aw`) derive `PartialEq` and are
//!   compared by it, as the widget styles are.
//! - The scalar helpers of the crate root (`f32`, `Option<f32>`,
//!   `iced_core::Padding`, `iced_core::Size`, `iced_core::Color`, `u16`,
//!   `&str`, `DialogButtonOrder`) by their `PartialEq`.
//!
//! A difference is allowed only as an entry of [`DIFFERENCES`], with its
//! reason; an entry that no longer differs fails.

use std::fmt::Debug;

use iced_core::theme::Theme;
use native_theme::next::{Preset, ThemeContext};

use crate::{ColorMode, ResolvedTheme};

/// The presets the test covers: the 0.7.2 key and the 0.8.0 preset.
const PRESETS: &[(&str, Preset)] = &[
    ("kde-breeze", Preset::KdeBreeze),
    ("adwaita", Preset::Adwaita),
    ("windows-11", Preset::Windows11),
    ("macos-monterey", Preset::MacosMonterey),
    ("macos-tahoe", Preset::MacosTahoe),
    ("material-3", Preset::Material3),
];

/// A listed difference (plan, section 1, rule 2): the leaf, the presets on
/// which it differs, and why.
struct Difference {
    /// The style functions or theme leaves that differ.
    leaves: &'static [&'static str],
    presets: &'static [&'static str],
    /// The one part of the output that differs: a prefix of the path
    /// `debug_diff` gives each differing field; any other field differing
    /// fails.
    field: &'static str,
    reason: &'static str,
}

/// The listed differences.
const DIFFERENCES: &[Difference] = &[Difference {
    leaves: &[
        "button",
        "button_toggle",
        "button_primary",
        "button_default",
        "button_danger",
        "button_success",
        "button_warning",
    ],
    presets: &["macos-monterey", "macos-tahoe"],
    field: "Style.border: Border.color",
    reason: "the button's border is a translucent stroke the theme lays inside the box, over the \
             fill (button.body(NONE).stroke, StrokeAlign::Inside); iced on wgpu ends the fill at \
             the border's inner edge, so the connector hands iced the stroke composited over the \
             fill, where 0.7.2 handed it the stated colour, which showed the window through the \
             band (docs/v0.8.0-spec.md §2, StrokeAlign; the M2 fill/stroke decision, \
             connector table, iced row)",
}];

/// The leaves compared and the differences found, for one preset and scheme.
struct Report {
    label: String,
    preset: &'static str,
    compared: usize,
    /// The listed differences found differing.
    /// The listed differences found differing, by index, with the preset.
    listed_seen: Vec<(usize, &'static str)>,
    failures: Vec<String>,
}

impl Report {
    fn new(preset: &'static str, is_dark: bool) -> Self {
        let scheme = if is_dark { "dark" } else { "light" };
        Self {
            label: format!("{preset}/{scheme}"),
            preset,
            compared: 0,
            listed_seen: Vec::new(),
            failures: Vec::new(),
        }
    }

    /// Compare one leaf; one that differs fails unless listed.
    fn leaf<T: PartialEq + Debug>(&mut self, leaf: &str, old: T, new: T) {
        self.compared += 1;
        if old == new {
            return;
        }
        let diff = debug_diff(&format!("{old:?}"), &format!("{new:?}"));
        let function = leaf.split('(').next().unwrap_or(leaf);
        let listed = DIFFERENCES.iter().position(|d| {
            d.leaves.contains(&function)
                && d.presets.contains(&self.preset)
                && diff.split("; ").all(|part| part.starts_with(d.field))
        });
        match listed {
            Some(i) => self.listed_seen.push((i, self.preset)),
            None => self
                .failures
                .push(format!("{}: {leaf}: {diff}", self.label)),
        }
    }

    /// The 0.8.0 side's value, or a failure where it is unmapped.
    fn mapped<T>(&mut self, leaf: &str, new: super::Mapped<T>) -> Option<T> {
        match new {
            Ok(v) => Some(v),
            Err(super::Unmapped(home)) => {
                self.failures
                    .push(format!("{}: {leaf}: 0.8.0 lacks {home}", self.label));
                None
            }
        }
    }
}

/// Where two `Debug` forms differ: each differing field, as the path of
/// the struct names and field names that lead to it, 0.7.2's value and
/// 0.8.0's.
fn debug_diff(old: &str, new: &str) -> String {
    let split = |s: &str| -> Vec<String> {
        let mut path: Vec<String> = Vec::new();
        let mut out = Vec::new();
        let mut word = String::new();
        for ch in s.chars() {
            match ch {
                '{' | '(' => {
                    path.push(word.trim().to_owned());
                    word.clear();
                }
                '}' | ')' | ',' => {
                    if !word.trim().is_empty() {
                        out.push(format!("{}.{}", path.join("."), word.trim()));
                    }
                    word.clear();
                    if ch != ',' {
                        path.pop();
                    }
                }
                _ => word.push(ch),
            }
        }
        if !word.trim().is_empty() {
            out.push(word.trim().to_owned());
        }
        out
    };
    let (o, n) = (split(old), split(new));
    if o.len() != n.len() {
        return format!("0.7.2 {old}, 0.8.0 {new}");
    }
    o.iter()
        .zip(&n)
        .filter(|(a, b)| a != b)
        .map(|(a, b)| format!("{a} -> {}", b.rsplit(':').next().unwrap_or(b).trim()))
        .collect::<Vec<_>>()
        .join("; ")
}

/// The two resolved themes of `preset` for `is_dark`, and 0.7.2's name and
/// mode, which its theme holds beside the variants.
fn resolve(
    key: &str,
    preset: Preset,
    is_dark: bool,
) -> crate::Result<(
    ResolvedTheme,
    String,
    ColorMode,
    native_theme::next::ResolvedTheme,
)> {
    let (mode, ctx) = if is_dark {
        (ColorMode::Dark, ThemeContext::dark())
    } else {
        (ColorMode::Light, ThemeContext::light())
    };
    let theme = crate::Theme::preset(key)?;
    let name = theme.name.to_string();
    let old = theme
        .into_variant(mode)?
        .into_resolved(&native_theme::ResolutionContext::for_tests())?;
    let new = native_theme::next::Theme::preset(preset).resolve(&ctx);
    Ok((old, name, mode, new))
}

/// The iced theme's leaves.
fn compare_theme(report: &mut Report, old: &Theme, new: &super::Mapped<Theme>) {
    let Some(new) = report.mapped("to_theme", new.clone()) else {
        return;
    };
    report.leaf("to_theme.palette", old.palette(), new.palette());
    report.leaf(
        "to_theme.extended_palette",
        *old.extended_palette(),
        *new.extended_palette(),
    );
    report.leaf("to_theme.name", old.to_string(), new.to_string());
}

/// Compare a style function's two outputs in each of `$statuses`, each side
/// called with its own iced theme.
macro_rules! style {
    ($report:expr, $themes:expr, $name:literal, $old:expr, $new:expr, $statuses:expr) => {{
        let (old_theme, new_theme): (&Theme, &Theme) = $themes;
        let old = $old;
        if let Some(new) = $report.mapped($name, $new) {
            for status in $statuses {
                $report.leaf(
                    &format!("{}({status:?})", $name),
                    old(old_theme, status),
                    new(new_theme, status),
                );
            }
        }
    }};
}

/// The style functions ported so far.
#[cfg(feature = "widgets")]
fn compare_styles(
    report: &mut Report,
    themes: (&Theme, &Theme),
    old: &ResolvedTheme,
    new: &native_theme::next::ResolvedTheme,
) {
    use crate::styles as o;
    use iced_widget::button::Status::{Active, Disabled, Hovered, Pressed};

    use super::styles as n;

    let button = [Active, Hovered, Pressed, Disabled];
    style!(
        report,
        themes,
        "button",
        o::button(old),
        n::button(new),
        button
    );
    for checked in [false, true] {
        style!(
            report,
            themes,
            "button_toggle",
            o::button_toggle(old, checked),
            n::button_toggle(new, checked),
            button
        );
        style!(
            report,
            themes,
            "toolbar_button",
            o::toolbar_button(old, checked),
            n::toolbar_button(new, checked),
            button
        );
    }
    style!(
        report,
        themes,
        "button_primary",
        o::button_primary(old),
        n::button_primary(new),
        button
    );
    style!(
        report,
        themes,
        "button_default",
        o::button_default(old),
        n::button_default(new),
        button
    );
    style!(
        report,
        themes,
        "button_danger",
        o::button_danger(old),
        n::button_danger(new),
        button
    );
    style!(
        report,
        themes,
        "button_success",
        o::button_success(old),
        n::button_success(new),
        button
    );
    style!(
        report,
        themes,
        "button_warning",
        o::button_warning(old),
        n::button_warning(new),
        button
    );
    style!(
        report,
        themes,
        "button_link",
        o::button_link(old),
        n::button_link(new),
        button
    );
}

/// Every leaf of `preset` for `is_dark`, compared.
fn compare_all(key: &'static str, preset: Preset, is_dark: bool) -> crate::Result<Report> {
    let (old, name, mode, new) = resolve(key, preset, is_dark)?;
    let mut report = Report::new(key, is_dark);
    let old_theme = crate::to_theme_in(&old, &name, mode);
    let new_theme = super::to_theme(&new);
    compare_theme(&mut report, &old_theme, &new_theme);
    #[cfg(feature = "widgets")]
    if let Ok(new_theme) = &new_theme {
        compare_styles(&mut report, (&old_theme, new_theme), &old, &new);
    }
    Ok(report)
}

#[test]
fn the_connector_hands_iced_the_same_from_both_models() -> crate::Result<()> {
    let mut failures = Vec::new();
    let mut listed_seen = Vec::new();
    for &(key, preset) in PRESETS {
        for is_dark in [false, true] {
            let report = compare_all(key, preset, is_dark)?;
            println!("{}: {} leaves compared", report.label, report.compared);
            failures.extend(report.failures);
            listed_seen.extend(report.listed_seen);
        }
    }
    for (i, d) in DIFFERENCES.iter().enumerate() {
        for &p in d.presets {
            if !listed_seen.contains(&(i, p)) {
                failures.push(format!(
                    "{:?} on {p}: listed, but differs nowhere: remove the preset from the entry",
                    d.leaves
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

#[test]
fn every_listed_difference_names_a_covered_preset() {
    for d in DIFFERENCES {
        assert!(!d.reason.is_empty(), "{:?}: no reason", d.leaves);
        for p in d.presets {
            assert!(
                PRESETS.iter().any(|(key, _)| key == p),
                "{:?}: {p} is not covered",
                d.leaves
            );
        }
    }
}
