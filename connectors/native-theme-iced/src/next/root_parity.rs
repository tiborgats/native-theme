//! Connector parity of the crate root's functions (docs/v0.8.0-plan.md,
//! section 1, rule 2): each function of [`super`] that has a twin at the
//! crate root, or a 0.7.2 field it reads in its place, gives on the 0.8.0
//! model what the twin gives on the 0.7.2 model, on every preset of
//! [`super::parity::PRESETS`], light and dark.
//!
//! The two sides are compared by their `Debug` forms, a value of the 0.8.0
//! side that may be unstated as `Ok` of the 0.7.2 side's value, and a 0.7.2
//! type the 0.8.0 model names anew (`DialogButtonOrder`) by its variant's
//! name. A difference is allowed only as an entry of [`DIFFERENCES`], with
//! its reason; an entry that differs nowhere is stale and fails.

use std::fmt::Debug;

use native_theme::ResolutionContext;
use native_theme::next::{Preset, ThemeContext};

use super::parity::PRESETS;
use super::{Mapped, Unmapped};

/// The two resolved themes of `preset` for `is_dark`. The 0.7.2 side is
/// resolved at the font DPI 0.7.2 resolves the platform's theme at, 72
/// "where a point is a pixel" for the Apple presets and 96 for the others
/// (`ResolutionContext::for_tests`), as native-theme's model parity resolves
/// it (native-theme/src/next/parity.rs, `MACOS_RESOLUTION_CONTEXT`,
/// `IOS_RESOLUTION_CONTEXT`): a text size is compared in the pixels both
/// models state it in.
fn resolve(
    key: &str,
    preset: Preset,
    is_dark: bool,
) -> crate::Result<(crate::ResolvedTheme, native_theme::next::ResolvedTheme)> {
    let (mode, ctx) = if is_dark {
        (crate::ColorMode::Dark, ThemeContext::dark())
    } else {
        (crate::ColorMode::Light, ThemeContext::light())
    };
    let apple = matches!(
        preset,
        Preset::MacosMonterey | Preset::MacosTahoe | Preset::Ios13
    );
    let context = ResolutionContext {
        font_dpi: if apple { 72.0 } else { 96.0 },
        ..ResolutionContext::for_tests()
    };
    let old = crate::Theme::preset(key)?
        .into_variant(mode)?
        .into_resolved(&context)?;
    let new = native_theme::next::Theme::preset(preset).resolve(&ctx);
    Ok((old, new))
}

/// A listed difference: the function, the preset's key, the 0.8.0 side's
/// value it asserts there (its `Debug` form), and why.
struct Difference {
    name: &'static str,
    preset: &'static str,
    new: &'static str,
    reason: &'static str,
}

/// The reason of Breeze's dialog size limits.
const Z2: &str = "the 0.8.0 theme states no dialog size limits where no source gives them \
     (docs/v0.8.0-spec.md §5, issue #4's leftovers, Z2: `dialog.panel.min_size` / `max_size`, \
     `None`); 0.7.2's kde-breeze preset states values no source gives";

/// The reason of Breeze's button height floor.
const Z1: &str = "the 0.8.0 theme states a button's minimum width and no minimum height \
     (docs/v0.8.0-spec.md §5, issue #4's leftovers, Z1: `button.body.min_size`, \
     `[Some(w), None]`); 0.7.2's kde-breeze preset states a height";

const DIFFERENCES: &[Difference] = &[
    Difference {
        name: "dialog_max_width",
        preset: "kde-breeze",
        new: "None",
        reason: Z2,
    },
    Difference {
        name: "dialog_max_height",
        preset: "kde-breeze",
        new: "None",
        reason: Z2,
    },
    Difference {
        name: "dialog_min_width",
        preset: "kde-breeze",
        new: "None",
        reason: Z2,
    },
    Difference {
        name: "dialog_min_height",
        preset: "kde-breeze",
        new: "None",
        reason: Z2,
    },
    Difference {
        name: "button_content_min_size",
        preset: "kde-breeze",
        new: "Ok([Some(68.0), None])",
        reason: Z1,
    },
];

/// The pairs compared on one preset and scheme.
struct Pairs {
    label: String,
    preset: &'static str,
    failures: Vec<String>,
    seen: Vec<(&'static str, &'static str)>,
}

impl Pairs {
    fn pair(&mut self, name: &'static str, old: impl Debug, new: impl Debug) {
        let (old, new) = (format!("{old:?}"), format!("{new:?}"));
        if old == new {
            return;
        }
        if DIFFERENCES.iter().any(|d| {
            d.name == name && d.preset == self.preset && d.new == new && !d.reason.is_empty()
        }) {
            self.seen.push((name, self.preset));
            return;
        }
        self.failures
            .push(format!("{}: {name}: 0.7.2 {old}, 0.8.0 {new}", self.label));
    }
}

/// `v` as the 0.8.0 side gives a value it may find unstated.
fn ok<T>(v: T) -> Mapped<T> {
    Ok::<T, Unmapped>(v)
}

/// The accessors the parity script holds iced's `next` to.
fn accessors(p: &mut Pairs, old: &crate::ResolvedTheme, new: &native_theme::next::ResolvedTheme) {
    let d = &old.defaults;
    p.pair(
        "border_line_width",
        d.border.line_width,
        super::border_line_width(new),
    );
    p.pair(
        "border_opacity",
        d.border.opacity,
        super::border_opacity(new),
    );
    p.pair(
        "border_shadow_enabled",
        d.border.shadow_enabled,
        super::border_shadow_enabled(new),
    );
    p.pair(
        "disabled_opacity",
        crate::disabled_opacity(old),
        super::disabled_opacity(new),
    );
    p.pair(
        "disabled_text_color",
        crate::palette::to_color(d.disabled_text_color),
        super::disabled_text_color(new),
    );
    p.pair(
        "focus_ring_width",
        ok(d.focus_ring_width),
        super::focus_ring_width(new),
    );
    p.pair(
        "focus_ring_offset",
        d.focus_ring_offset,
        super::focus_ring_offset(new),
    );
    let (oi, ni) = (crate::icon_sizes(old), super::icon_sizes(new));
    p.pair(
        "icon_sizes",
        [oi.toolbar, oi.small, oi.large, oi.dialog, oi.panel].map(Some),
        [ni.toolbar, ni.small, ni.large, ni.dialog, ni.panel],
    );
    let (ot, nt) = (crate::text_scale(old), super::text_scale(new));
    for (role, o, n) in [
        ("text_scale.caption", &ot.caption, nt.caption),
        (
            "text_scale.section_heading",
            &ot.section_heading,
            nt.section_heading,
        ),
        ("text_scale.dialog_title", &ot.dialog_title, nt.dialog_title),
        ("text_scale.display", &ot.display, nt.display),
    ] {
        p.pair(
            role,
            (o.size, o.weight, o.line_height),
            (n.size_px, n.weight, n.line_height),
        );
    }
    p.pair(
        "line_height",
        crate::line_height(old),
        super::line_height(new),
    );
    p.pair(
        "font_weight",
        ok(crate::font_weight(old)),
        super::font_weight(new),
    );
    p.pair(
        "mono_font_weight",
        ok(crate::mono_font_weight(old)),
        super::mono_font_weight(new),
    );
    p.pair(
        "dialog_button_order",
        ok(format!("{:?}", crate::dialog_button_order(old))),
        super::dialog_button_order(new).map(|o| format!("{o:?}")),
    );
    p.pair(
        "dialog_button_gap",
        crate::dialog_button_gap(old),
        super::dialog_button_gap(new),
    );
    p.pair(
        "scrollbar_groove_width",
        ok(crate::scrollbar_groove_width(old)),
        super::scrollbar_groove_width(new),
    );
    p.pair(
        "list_header_hover_background",
        ok(crate::list_header_hover_background(old)),
        super::list_header_hover_background(new),
    );
    p.pair(
        "list_header_active_background",
        ok(crate::list_header_active_background(old)),
        super::list_header_active_background(new),
    );
    p.pair(
        "tooltip_show_delay",
        crate::tooltip_show_delay(old),
        super::tooltip_show_delay(new),
    );
}

/// The crate root's helpers.
fn helpers(p: &mut Pairs, old: &crate::ResolvedTheme, new: &native_theme::next::ResolvedTheme) {
    p.pair(
        "border_color",
        ok(crate::border_color(old)),
        super::border_color(new),
    );
    p.pair(
        "border_corner_radius",
        crate::border_corner_radius(old),
        super::border_corner_radius(new),
    );
    p.pair(
        "border_corner_radius_lg",
        crate::border_corner_radius_lg(old),
        super::border_corner_radius_lg(new),
    );
    p.pair(
        "focus_ring_color",
        crate::focus_ring_color(old),
        super::focus_ring_color(new),
    );
    p.pair("link_color", crate::link_color(old), super::link_color(new));
    p.pair(
        "selection_background",
        crate::selection_background(old),
        super::selection_background(new),
    );
    p.pair("info_color", crate::info_color(old), super::info_color(new));
    p.pair(
        "info_text_color",
        crate::info_text_color(old),
        super::info_text_color(new),
    );
    p.pair(
        "warning_text_color",
        crate::warning_text_color(old),
        super::warning_text_color(new),
    );
    p.pair(
        "dialog_title_font_color",
        ok(crate::dialog_title_font_color(old)),
        super::dialog_title_font_color(new),
    );
    p.pair(
        "expander_arrow_color",
        ok(crate::expander_arrow_color(old)),
        super::expander_arrow_color(new),
    );
    p.pair(
        "font_family",
        crate::font_family(old),
        super::font_family(new),
    );
    p.pair(
        "mono_font_family",
        crate::mono_font_family(old),
        super::mono_font_family(new),
    );
    let prefs = crate::AccessibilityPreferences::default();
    p.pair(
        "font_size",
        crate::font_size(old, &prefs),
        super::font_size(new),
    );
    p.pair(
        "mono_font_size",
        crate::mono_font_size(old, &prefs),
        super::mono_font_size(new),
    );
    p.pair(
        "dialog_max_width",
        Some(crate::dialog_max_width(old)),
        super::dialog_max_width(new),
    );
    p.pair(
        "dialog_max_height",
        Some(crate::dialog_max_height(old)),
        super::dialog_max_height(new),
    );
    p.pair(
        "dialog_min_width",
        Some(crate::dialog_min_width(old)),
        super::dialog_min_width(new),
    );
    p.pair(
        "dialog_min_height",
        Some(crate::dialog_min_height(old)),
        super::dialog_min_height(new),
    );
    p.pair(
        "list_header_height",
        crate::list_header_height(old),
        super::list_header_height(new),
    );
    p.pair(
        "toolbar_bar_height",
        crate::toolbar_bar_height(old),
        super::toolbar_bar_height(new),
    );
    p.pair(
        "toolbar_item_gap",
        crate::toolbar_item_gap(old),
        super::toolbar_item_gap(new),
    );
    p.pair(
        "toolbar_border_line_width",
        ok(crate::toolbar_border_line_width(old)),
        super::toolbar_border_line_width(new),
    );
    let own = iced_core::Padding::new(1.0);
    p.pair(
        "dialog_padding",
        ok(crate::dialog_padding(old, own)),
        super::dialog_padding(new, own),
    );
    p.pair(
        "toolbar_padding",
        crate::toolbar_padding(old, own),
        super::toolbar_padding(new, own),
    );
    #[cfg(feature = "widgets")]
    {
        p.pair(
            "button_padding",
            ok(crate::button_padding(old)),
            super::button_padding(new),
        );
        p.pair(
            "input_padding",
            ok(crate::input_padding(old)),
            super::input_padding(new),
        );
        p.pair(
            "text_area_padding",
            ok(crate::text_area_padding(old)),
            super::text_area_padding(new),
        );
        p.pair(
            "combo_box_padding",
            ok(crate::combo_box_padding(old)),
            super::combo_box_padding(new),
        );
        let s = crate::button_content_min_size(old);
        p.pair(
            "button_content_min_size",
            ok([Some(s.width), Some(s.height)]),
            super::button_content_min_size(new),
        );
    }
}

#[test]
fn the_crate_roots_functions_agree_on_both_models() -> crate::Result<()> {
    let mut failures = Vec::new();
    let mut seen = Vec::new();
    for &(key, preset) in PRESETS {
        for is_dark in [false, true] {
            let (old, new) = resolve(key, preset, is_dark)?;
            let mut p = Pairs {
                label: format!("{key} {}", if is_dark { "dark" } else { "light" }),
                preset: key,
                failures: Vec::new(),
                seen: Vec::new(),
            };
            accessors(&mut p, &old, &new);
            helpers(&mut p, &old, &new);
            failures.extend(p.failures);
            seen.extend(p.seen);
        }
    }
    for d in DIFFERENCES {
        // The button's helpers are compared with the `widgets` feature only.
        if d.name == "button_content_min_size" && !cfg!(feature = "widgets") {
            continue;
        }
        if !seen.contains(&(d.name, d.preset)) {
            failures.push(format!(
                "{} on {}: listed, but differs nowhere",
                d.name, d.preset
            ));
        }
    }
    assert!(
        failures.is_empty(),
        "{} values differ:\n{}",
        failures.len(),
        failures.join("\n")
    );
    Ok(())
}
