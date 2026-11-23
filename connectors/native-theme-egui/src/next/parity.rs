//! Connector parity of the whole mapping (docs/v0.8.0-plan.md, section 1, rule 2; M4, step 1):
//! what the connector hands to egui, compiled from the 0.7.2 model by the crate root's path and
//! from the 0.8.0 model by [`super`], is equal on every preset of
//! [`super::root_parity::PRESETS`], light and dark.
//!
//! The 0.7.2 side is resolved as [`super::root_parity::resolve`] resolves it, and compiled with
//! the default preferences, whose text scale is 1, as the 0.8.0 theme is resolved for, and the
//! preset's `LayoutTheme`.
//!
//! The form of comparison per egui type the connector produces:
//!
//! - `egui::Style` derives `PartialEq` and `Debug`, and so do every type it holds that the
//!   mapping writes: `Visuals` (with `Widgets` and its five `WidgetVisuals`, `Selection`,
//!   `TextCursorStyle`, the two `Shadow`s), `Spacing` (with `ScrollStyle` and the `Margin`s),
//!   `Interaction`, `ScrollAnimation`, and the `text_styles` map of `FontId`s (egui 0.37.0
//!   `src/style.rs`). The base style and each role cell, in each of its four variants, are
//!   compared by `PartialEq`; a difference is reported by the lines in which their `Debug`
//!   forms (`{:#?}`) differ, so one listed difference names one leaf of the `Style`.
//! - `egui::Frame` derives `PartialEq` and `Debug` (egui 0.37.0 `src/containers/frame.rs`):
//!   each surface's frame is compared as the style is.
//! - The focus ring the install plugin paints is the crate's own `FocusRing` (its stroke, an
//!   `egui::Stroke`, its offset and whether it shows for the keyboard only), which derives
//!   `PartialEq` and `Debug`, compared as the style is.
//! - The fonts: `egui::FontDefinitions` are made from a `fonts::FontPlan`, which the
//!   application passes to the 0.7.2 builder; neither `to_theme` makes one, so none is
//!   compared. The text sizes the mapping writes are the `FontId`s of the styles above.
//! - The notes (`Note`) name the leaf whose value was sanitised, saturated or transparent, a
//!   0.7.2 leaf on one side and its 0.8.0 home on the other: not compared.
//!
//! Compared so far: the base style. A difference is allowed only as an entry of
//! [`DIFFERENCES`], per leaf and preset, with its cause, from which the test computes the
//! value the 0.8.0 side must hold there; an entry that differs nowhere is stale and fails.

use native_theme::next::Preset;

use super::build::base_source;
use super::root_parity::{PRESETS, resolve};
use crate::AccessibilityPreferences;
use crate::convert::to_color32;
use crate::style::base::{BaseSource, base_style_from};
use crate::style::line::Line;

/// Why a leaf differs, which gives the value the 0.8.0 side holds there.
#[derive(Clone, Copy, Debug)]
enum Cause {
    /// The 0.8.0 theme leaves the value unstated where the 0.7.2 resolver gives one, and egui's
    /// own stays (spec §10, rule 0): egui's own style's value at the leaf.
    Egui,
    /// The button's translucent border lies inside the body (`Inside` or `Center`): the 0.7.2
    /// stroke composited over the fill that entry paints (`W/M2/fill-stroke-decision.md`, the
    /// egui row; `style::line`).
    ButtonBorderOverFill,
}

/// The preset key of a difference that occurs on every preset of [`PRESETS`]; the test asks
/// it of each.
const EVERY: &str = "*";

/// A listed difference: what is compared, the preset's key (or [`EVERY`]), the leaf's path
/// in the `Debug` form, its cause, and the source of the cause.
struct Difference {
    what: &'static str,
    preset: &'static str,
    path: &'static str,
    cause: Cause,
    reason: &'static str,
}

/// Rule 0 on a state: the preset states no rule for it, egui's own entries differ there from
/// rest, so egui's own value stays.
const STATE_RULE_0: &str = "the 0.8.0 theme states no rule for the state, and egui's own \
     entries change the property between rest and the state: egui's own (spec §10, rule 0); \
     0.7.2's resolver gave the resting value or another state's";

macro_rules! differences {
    ($($what:literal $preset:literal $path:literal $cause:ident $reason:expr;)*) => {
        &[$(Difference {
            what: $what,
            preset: $preset,
            path: $path,
            cause: Cause::$cause,
            reason: $reason,
        },)*]
    };
}

/// Rule 0 on the text-stroke width: the theme states none for any state, and egui's own
/// entries widen it under the pointer and on a press.
const WIDTH_RULE_0: &str = "the 0.8.0 theme states no text-stroke width, and egui's own \
     entries change it between rest and the state (egui 0.37.0 src/style.rs, `Widgets::dark` \
     and `Widgets::light`): egui's own (spec §10, rule 0); 0.7.2 copied the resting width";

const DIFFERENCES: &[Difference] = differences! {
    "base" "*" ".visuals.widgets.hovered.fg_stroke.width" Egui WIDTH_RULE_0;
    "base" "*" ".visuals.widgets.active.fg_stroke.width" Egui WIDTH_RULE_0;
    "base" "kde-breeze" ".spacing.interact_size" Egui
        "button.min_height: 0.7.2's 32 is the height at the default font, not a minimum \
         (issue #4 Z1); button.body(NONE).min_size[1] is unstated on kde-breeze (the table of \
         homes' listed difference)";
    "base" "adwaita" ".visuals.widgets.hovered.bg_stroke.color" Egui STATE_RULE_0;
    "base" "adwaita" ".visuals.widgets.active.bg_stroke.color" Egui STATE_RULE_0;
    "base" "ios-13" ".visuals.widgets.hovered.bg_stroke.color" Egui STATE_RULE_0;
    "base" "ios-13" ".visuals.widgets.active.bg_stroke.color" Egui STATE_RULE_0;
    "base" "ios-13" ".visuals.widgets.hovered.fg_stroke.color" Egui STATE_RULE_0;
    "base" "macos-monterey" ".visuals.widgets.hovered.bg_stroke.color" Egui STATE_RULE_0;
    "base" "macos-monterey" ".visuals.widgets.active.bg_stroke.color" Egui STATE_RULE_0;
    "base" "macos-monterey" ".visuals.widgets.hovered.fg_stroke.color" Egui STATE_RULE_0;
    "base" "macos-monterey" ".visuals.widgets.inactive.bg_stroke.color" ButtonBorderOverFill
        "macOS's push-button border lies inside its body";
    "base" "macos-monterey" ".visuals.widgets.open.bg_stroke.color" ButtonBorderOverFill
        "macOS's push-button border lies inside its body";
    "base" "macos-tahoe" ".visuals.widgets.hovered.bg_stroke.color" Egui STATE_RULE_0;
    "base" "macos-tahoe" ".visuals.widgets.active.bg_stroke.color" Egui STATE_RULE_0;
    "base" "macos-tahoe" ".visuals.widgets.hovered.fg_stroke.color" Egui STATE_RULE_0;
    "base" "macos-tahoe" ".visuals.widgets.hovered.bg_fill" Egui STATE_RULE_0;
    "base" "macos-tahoe" ".visuals.widgets.active.bg_fill" Egui STATE_RULE_0;
    "base" "macos-tahoe" ".visuals.widgets.active.weak_bg_fill" Egui STATE_RULE_0;
    "base" "macos-tahoe" ".visuals.widgets.inactive.bg_stroke.color" ButtonBorderOverFill
        "macOS's push-button border lies inside its body";
    "base" "macos-tahoe" ".visuals.widgets.open.bg_stroke.color" ButtonBorderOverFill
        "macOS's push-button border lies inside its body";
    "base" "material-3" ".visuals.widgets.hovered.bg_stroke.color" Egui STATE_RULE_0;
    "base" "material-3" ".visuals.widgets.active.bg_stroke.color" Egui STATE_RULE_0;
    "base" "windows-11" ".visuals.widgets.hovered.bg_stroke.color" Egui STATE_RULE_0;
    "base" "windows-11" ".visuals.widgets.active.bg_stroke.color" Egui STATE_RULE_0;
    "base" "windows-11" ".visuals.widgets.active.bg_fill" Egui STATE_RULE_0;
};

/// Each line of a `{:#?}` form as the path of the field it holds
/// (`.visuals.widgets.hovered.bg_fill`) and the line's value, without its trailing comma.
fn leaves(debug: &str) -> Vec<(String, String)> {
    let mut stack: Vec<(usize, String)> = Vec::new();
    let mut out = Vec::new();
    for line in debug.lines() {
        let indent = line.len().saturating_sub(line.trim_start().len());
        let text = line.trim();
        while stack.last().is_some_and(|(i, _)| *i >= indent) {
            stack.pop();
        }
        let (key, value) = match text.split_once(": ") {
            Some((k, v)) if !k.contains(' ') => (k.to_owned(), v.to_owned()),
            _ => (String::new(), text.to_owned()),
        };
        let mut path: Vec<&str> = stack.iter().map(|(_, k)| k.as_str()).collect();
        path.push(&key);
        let path = path.join(".");
        let value = value.trim_end_matches(',').to_owned();
        if value.ends_with('{') || value.ends_with('[') || value.ends_with('(') {
            stack.push((indent, key.clone()));
        }
        out.push((path, value));
    }
    out
}

/// One comparison: what is compared, the preset and scheme, and egui's own style for the scheme.
struct Side<'a> {
    what: &'a str,
    preset: &'static str,
    label: &'a str,
    own: &'a egui::Style,
}

/// The leaves in which `old` and `new` differ where no entry lists the difference, or where
/// the 0.8.0 side's value is not the one the entry's cause gives (`expected`); the listed ones
/// `seen` records.
fn compare(
    side: &Side<'_>,
    old: &impl std::fmt::Debug,
    new: &impl std::fmt::Debug,
    expected: &dyn Fn(&str, Cause) -> Option<String>,
    failures: &mut Vec<String>,
    seen: &mut Vec<(&'static str, &'static str, &'static str)>,
) {
    let (old, new) = (leaves(&format!("{old:#?}")), leaves(&format!("{new:#?}")));
    let own = leaves(&format!("{:#?}", side.own));
    let label = side.label;
    let what = side.what;
    if old.len() != new.len() {
        failures.push(format!(
            "{label}: {what}: the Debug forms differ in shape: {} lines against {}",
            old.len(),
            new.len()
        ));
    }
    for ((path, o), (new_path, n)) in old.iter().zip(&new) {
        if path != new_path {
            failures.push(format!("{label}: {what}: {path} against {new_path}"));
            return;
        }
        if o == n {
            continue;
        }
        let Some(d) = DIFFERENCES.iter().find(|d| {
            d.what == what && (d.preset == side.preset || d.preset == EVERY) && d.path == path
        }) else {
            failures.push(format!("{label}: {what}: {path}: 0.7.2 {o}, 0.8.0 {n}"));
            continue;
        };
        let want = match d.cause {
            Cause::Egui => own.iter().find(|(p, _)| p == path).map(|(_, v)| v.clone()),
            cause @ Cause::ButtonBorderOverFill => expected(path, cause),
        };
        if want.as_ref() == Some(n) && !d.reason.is_empty() {
            seen.push((d.what, side.preset, d.path));
        } else {
            failures.push(format!(
                "{label}: {what}: {path}: listed ({:?}), but 0.8.0 {n}, the cause gives {want:?}",
                d.cause
            ));
        }
    }
}

/// The Body row height `Builder::build` measures for a source's Body size, on egui's
/// default faces.
fn row_height(src: &BaseSource) -> Option<f32> {
    let [_, body, ..] = src.sizes;
    body.and_then(|(size, _)| {
        crate::fonts::body_row_height(&egui::FontDefinitions::default(), size)
    })
}

/// The entry of `widgets` a `.visuals.widgets.<entry>.…` path names.
fn entry<'a>(
    widgets: &'a egui::style::Widgets,
    path: &str,
) -> Option<&'a egui::style::WidgetVisuals> {
    match path.split('.').nth(3)? {
        "noninteractive" => Some(&widgets.noninteractive),
        "inactive" => Some(&widgets.inactive),
        "hovered" => Some(&widgets.hovered),
        "active" => Some(&widgets.active),
        "open" => Some(&widgets.open),
        _ => None,
    }
}

#[test]
fn the_mapping_agrees_on_both_models() -> crate::Result<()> {
    let mut failures = Vec::new();
    let mut seen = Vec::new();
    for &(key, preset) in PRESETS {
        let layout = native_theme::theme::Theme::preset(key)?.layout;
        for (is_dark, scheme) in [(false, egui::Theme::Light), (true, egui::Theme::Dark)] {
            let (old, new) = resolve(key, preset, is_dark)?;
            let label = format!("{key}/{}", if is_dark { "dark" } else { "light" });
            let old_src = BaseSource::old(&old, &AccessibilityPreferences::default(), &layout);
            let new_src = base_source(&new);
            let old_base = base_style_from(
                scheme,
                row_height(&old_src),
                &old_src,
                false,
                &mut Vec::new(),
            );
            let new_base = base_style_from(
                scheme,
                row_height(&new_src),
                &new_src,
                true,
                &mut Vec::new(),
            );
            let own = scheme.default_style();
            // `open` paints the resting fill (`OpenFrom::Inactive`); its `weak_bg_fill` is
            // then the title bar's
            let over_fill = |path: &str, _: Cause| {
                let w = &new_base.visuals.widgets;
                let fill = if path.ends_with("open.bg_stroke.color") {
                    w.inactive.weak_bg_fill
                } else {
                    entry(w, path)?.weak_bg_fill
                };
                let color = to_color32(old.button.border.color);
                let line = Line {
                    color,
                    align: new_src.button.align,
                };
                Some(format!("{:?}", line.over(Some(fill))))
            };
            let side = Side {
                what: "base",
                preset: key,
                label: &label,
                own: &own,
            };
            compare(
                &side,
                &old_base,
                &new_base,
                &over_fill,
                &mut failures,
                &mut seen,
            );
        }
    }
    for preset in Preset::ALL {
        if !PRESETS.iter().any(|(_, p)| p == preset) {
            failures.push(format!("{preset:?}: not compared"));
        }
    }
    for d in DIFFERENCES {
        let presets: Vec<&str> = if d.preset == EVERY {
            PRESETS.iter().map(|(key, _)| *key).collect()
        } else {
            vec![d.preset]
        };
        for key in presets {
            if !seen
                .iter()
                .any(|(what, preset, path)| *what == d.what && *preset == key && *path == d.path)
            {
                failures.push(format!(
                    "{} {} on {key}: listed, but differs nowhere: remove the entry",
                    d.what, d.path
                ));
            }
        }
    }
    assert!(
        failures.is_empty(),
        "{} differences:\n{}",
        failures.len(),
        failures.join("\n")
    );
    Ok(())
}
