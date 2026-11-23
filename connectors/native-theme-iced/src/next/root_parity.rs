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

/// A listed difference: the function, the preset's key, and why.
struct Difference {
    name: &'static str,
    preset: &'static str,
    reason: &'static str,
}

const DIFFERENCES: &[Difference] = &[];

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
        if DIFFERENCES
            .iter()
            .any(|d| d.name == name && d.preset == self.preset && !d.reason.is_empty())
        {
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
            failures.extend(p.failures);
            seen.extend(p.seen);
        }
    }
    for d in DIFFERENCES {
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
