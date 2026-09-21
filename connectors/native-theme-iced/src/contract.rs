//! Layer 1 of the theme contracts: every palette slot this connector writes,
//! the native field it must equal, and the contrast report of spec section 7.
//!
//! Test-only. The rows are asserted over the sixteen presets in both modes --
//! the four `*-live.toml` files are geometry-only merge bases and are not in
//! `Theme::list_presets()`. The contrast report prints both ratios and asserts
//! nothing: iced, not the connector, chooses the background a text input is
//! painted on. The assertion lives over `styles::*`, where we control both
//! colors.

use crate::extended::contrast_ratio;
use crate::palette::to_color;
use crate::{ColorMode, ResolvedTheme};
use iced_core::Color;
use iced_core::theme::Theme;
use native_theme::color::Rgba;

/// One row of the mapping contract: a palette slot, the native field it must
/// equal, and the presets where it may legitimately differ.
struct Row {
    slot: &'static str,
    native: fn(&ResolvedTheme) -> Rgba,
    /// The target is a palette slot, not a struct field, so the getter takes
    /// the built theme; the resolved values come along for the slots that
    /// need more than the theme to read.
    get: fn(&Theme, &ResolvedTheme) -> Color,
    /// Preset keys where this row does not hold, each with its reason.
    exceptions: &'static [(&'static str, &'static str)],
}

/// Every palette slot the connector writes, and the native field it carries.
///
/// The six base-palette slots come from `palette::to_palette()`, the
/// `extended.*` slots from `extended::apply_overrides()`.
const ROWS: &[Row] = &[
    Row {
        slot: "palette.background",
        native: |r| r.defaults.background_color,
        get: |t, _| t.palette().background,
        exceptions: &[],
    },
    Row {
        slot: "palette.text",
        native: |r| r.defaults.text_color,
        get: |t, _| t.palette().text,
        exceptions: &[],
    },
    Row {
        slot: "palette.primary",
        native: |r| r.defaults.accent_color,
        get: |t, _| t.palette().primary,
        exceptions: &[],
    },
    Row {
        slot: "palette.success",
        native: |r| r.defaults.success_color,
        get: |t, _| t.palette().success,
        exceptions: &[],
    },
    Row {
        slot: "palette.warning",
        native: |r| r.defaults.warning_color,
        get: |t, _| t.palette().warning,
        exceptions: &[],
    },
    Row {
        slot: "palette.danger",
        native: |r| r.defaults.danger_color,
        get: |t, _| t.palette().danger,
        exceptions: &[],
    },
    Row {
        slot: "extended.secondary.base.color",
        native: |r| r.input.placeholder_color,
        get: |t, _| t.extended_palette().secondary.base.color,
        exceptions: &[],
    },
    Row {
        // A hovered `button::secondary` keeps the base label and swaps only
        // the fill, so the fill is a copy of the base.
        slot: "extended.secondary.strong.color",
        native: |r| r.input.placeholder_color,
        get: |t, _| t.extended_palette().secondary.strong.color,
        exceptions: &[],
    },
    Row {
        slot: "extended.background.weak.color",
        native: |r| r.defaults.surface_color,
        get: |t, _| t.extended_palette().background.weak.color,
        exceptions: &[],
    },
    Row {
        slot: "extended.background.weak.text",
        native: |r| r.defaults.text_color,
        get: |t, _| t.extended_palette().background.weak.text,
        exceptions: &[],
    },
    Row {
        slot: "extended.primary.base.text",
        native: |r| r.defaults.accent_text_color,
        get: |t, _| t.extended_palette().primary.base.text,
        exceptions: &[],
    },
    Row {
        slot: "extended.success.base.text",
        native: |r| r.defaults.success_text_color,
        get: |t, _| t.extended_palette().success.base.text,
        exceptions: &[],
    },
    Row {
        slot: "extended.danger.base.text",
        native: |r| r.defaults.danger_text_color,
        get: |t, _| t.extended_palette().danger.base.text,
        exceptions: &[],
    },
    Row {
        slot: "extended.warning.base.text",
        native: |r| r.defaults.warning_text_color,
        get: |t, _| t.extended_palette().warning.base.text,
        exceptions: &[],
    },
];

/// One pair of the contrast report: the two colors iced ends up painting, and
/// the pair the platform itself gives for the same thing.
struct ReportPair {
    what: &'static str,
    /// Foreground, background, and the surface that background is painted on,
    /// as the connector's theme emits them.
    emitted: fn(&Theme) -> (Color, Color, Color),
    /// The same three, straight from the native fields.
    native: fn(&ResolvedTheme) -> (Rgba, Rgba, Rgba),
}

/// The palette pairs the report covers. Printed, never asserted (section 7).
const PAIRS: &[ReportPair] = &[
    ReportPair {
        what: "placeholder",
        // iced paints a text input on the window background, whatever the
        // platform's field color is.
        emitted: |t| {
            let e = t.extended_palette();
            let window = e.background.base.color;
            (e.secondary.base.color, window, window)
        },
        native: |r| {
            (
                r.input.placeholder_color,
                r.input.background_color,
                r.defaults.background_color,
            )
        },
    },
    ReportPair {
        // The label a hovered `button::secondary` keeps, on the fill it swaps
        // in (`iced_widget::button::secondary`).
        what: "secondary.hover",
        emitted: |t| {
            let e = t.extended_palette();
            (
                e.secondary.base.text,
                e.secondary.strong.color,
                e.background.base.color,
            )
        },
        native: |r| {
            (
                r.button.font.color,
                r.button.hover_background,
                r.button.background_color,
            )
        },
    },
    ReportPair {
        what: "background.weak",
        emitted: |t| {
            let e = t.extended_palette();
            (
                e.background.weak.text,
                e.background.weak.color,
                e.background.base.color,
            )
        },
        native: |r| {
            (
                r.defaults.text_color,
                r.defaults.surface_color,
                r.defaults.background_color,
            )
        },
    },
    ReportPair {
        what: "primary.base",
        emitted: |t| {
            let e = t.extended_palette();
            (
                e.primary.base.text,
                e.primary.base.color,
                e.background.base.color,
            )
        },
        native: |r| {
            (
                r.defaults.accent_text_color,
                r.defaults.accent_color,
                r.defaults.background_color,
            )
        },
    },
    ReportPair {
        what: "success.base",
        emitted: |t| {
            let e = t.extended_palette();
            (
                e.success.base.text,
                e.success.base.color,
                e.background.base.color,
            )
        },
        native: |r| {
            (
                r.defaults.success_text_color,
                r.defaults.success_color,
                r.defaults.background_color,
            )
        },
    },
    ReportPair {
        what: "danger.base",
        emitted: |t| {
            let e = t.extended_palette();
            (
                e.danger.base.text,
                e.danger.base.color,
                e.background.base.color,
            )
        },
        native: |r| {
            (
                r.defaults.danger_text_color,
                r.defaults.danger_color,
                r.defaults.background_color,
            )
        },
    },
    ReportPair {
        what: "warning.base",
        emitted: |t| {
            let e = t.extended_palette();
            (
                e.warning.base.text,
                e.warning.base.color,
                e.background.base.color,
            )
        },
        native: |r| {
            (
                r.defaults.warning_text_color,
                r.defaults.warning_color,
                r.defaults.background_color,
            )
        },
    },
];

/// WCAG 2.1 AA for normal text. Only ever used to decide what to print.
const AA: f32 = 4.5;

/// One preset in one mode: the native values and the theme built from them.
struct Combination {
    key: &'static str,
    is_dark: bool,
    resolved: ResolvedTheme,
    theme: Theme,
}

impl Combination {
    fn label(&self) -> String {
        let mode = if self.is_dark { "dark" } else { "light" };
        format!("{}/{mode}", self.key)
    }
}

/// Resolve every bundled preset in both modes and build its iced theme.
fn combinations() -> native_theme::Result<Vec<Combination>> {
    let mut out = Vec::new();
    for info in native_theme::theme::Theme::list_presets() {
        for is_dark in [false, true] {
            let mode = if is_dark {
                ColorMode::Dark
            } else {
                ColorMode::Light
            };
            let resolved = native_theme::theme::Theme::preset(info.key)?
                .into_variant(mode)?
                .into_resolved(&native_theme::ResolutionContext::for_tests())?;
            let theme = crate::to_theme(&resolved, info.key);
            out.push(Combination {
                key: info.key,
                is_dark,
                resolved,
                theme,
            });
        }
    }
    Ok(out)
}

/// Composite `top` over `bottom` when it carries alpha, so a ratio is
/// measured on what the screen actually shows.
fn over(top: Color, bottom: Color) -> Color {
    if top.a >= 1.0 {
        return top;
    }
    let blend = |t: f32, b: f32| t * top.a + b * (1.0 - top.a);
    Color {
        r: blend(top.r, bottom.r),
        g: blend(top.g, bottom.g),
        b: blend(top.b, bottom.b),
        a: 1.0,
    }
}

/// Contrast of `fg` on `bg`, with every layer composited first: `bg` over the
/// `surface` it is painted on, `fg` over the result.
fn pair_ratio(fg: Color, bg: Color, surface: Color) -> f32 {
    let bg = over(bg, surface);
    contrast_ratio(over(fg, bg), bg)
}

fn show(c: Color) -> String {
    format!("({:.4}, {:.4}, {:.4}, {:.4})", c.r, c.g, c.b, c.a)
}

#[test]
fn every_palette_slot_equals_its_native_field() -> native_theme::Result<()> {
    let combinations = combinations()?;
    let mut failures = Vec::new();

    for c in &combinations {
        for row in ROWS {
            if row.exceptions.iter().any(|(key, _)| *key == c.key) {
                continue;
            }
            let expected = to_color((row.native)(&c.resolved));
            let actual = (row.get)(&c.theme, &c.resolved);
            if actual != expected {
                failures.push(format!(
                    "{}: {} is {}, native gives {}",
                    c.label(),
                    row.slot,
                    show(actual),
                    show(expected)
                ));
            }
        }
    }

    assert!(
        failures.is_empty(),
        "{} of {} contract checks failed:\n{}",
        failures.len(),
        combinations.len() * ROWS.len(),
        failures.join("\n")
    );
    Ok(())
}

#[test]
fn palette_contrast_report() -> native_theme::Result<()> {
    let combinations = combinations()?;
    let mut below_aa = 0usize;
    let mut worse = 0usize;
    let (mut equal, mut lower, mut higher) = (0usize, 0usize, 0usize);
    let mut max_shortfall = 0.0f32;

    println!(
        "--- iced palette contrast report: {} combinations x {} pairs ---",
        combinations.len(),
        PAIRS.len()
    );

    for c in &combinations {
        for pair in PAIRS {
            let (fg, bg, surface) = (pair.emitted)(&c.theme);
            let emitted = pair_ratio(fg, bg, surface);
            let (native_fg, native_bg, native_surface) = (pair.native)(&c.resolved);
            let native = pair_ratio(
                to_color(native_fg),
                to_color(native_bg),
                to_color(native_surface),
            );

            let mut flags = String::new();
            if emitted < AA {
                below_aa += 1;
                flags.push_str("  below-AA");
            }
            if emitted < native {
                worse += 1;
                flags.push_str("  worse-than-native");
            }
            println!(
                "{:<26} {:<16} emitted {emitted:>5.2}  native {native:>5.2}{flags}",
                c.label(),
                pair.what
            );

            if pair.what == "placeholder" {
                if emitted == native {
                    equal += 1;
                } else if emitted < native {
                    lower += 1;
                    max_shortfall = max_shortfall.max(native - emitted);
                } else {
                    higher += 1;
                }
            }
        }
    }

    println!(
        "placeholder vs the platform's own pair: equal in {equal}, \
         lower in {lower} (at most {max_shortfall:.2}), higher in {higher}"
    );
    println!("pairs below AA: {below_aa}; pairs worse than the platform's own: {worse}");
    Ok(())
}
