//! Layer 1 of the theme contracts: every palette slot this connector writes
//! and every field of a `styles::*` output, the native value each must equal,
//! and the contrast report of spec section 7.
//!
//! Test-only. The rows are asserted over the sixteen presets in both modes --
//! the four `*-live.toml` files are geometry-only merge bases and are not in
//! `Theme::list_presets()`. The contrast report prints both ratios and asserts
//! nothing: iced, not the connector, chooses the background a text input is
//! painted on. The assertion lives over `styles::*`, where we control both
//! colors.
//!
//! The coverage tripwire (section 5.2) closes the table: every field it walks
//! -- by exhaustive destructuring, so an upstream addition fails to compile
//! here first -- must appear in exactly one of the rows and `DERIVED`.

use crate::extended::contrast_ratio;
use crate::palette::to_color;
use crate::{ColorMode, ResolvedTheme};
use iced_core::Color;
use iced_core::theme::Theme;
use iced_core::theme::palette::{Danger, Extended, Pair, Primary, Secondary, Success, Warning};
use native_theme::color::Rgba;

#[cfg(feature = "widgets")]
use crate::styles;
#[cfg(feature = "widgets")]
use iced_core::{Background, Border, border::Radius};
#[cfg(feature = "widgets")]
use iced_widget::button;

/// One row of the mapping contract: a palette slot, the native field it must
/// equal, and the presets where it may legitimately differ.
///
/// Fields of a `styles::*` output are rows too, but their value depends on the
/// widget's status and is sometimes two native colors composited, so they
/// carry their own row types below.
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
/// `extended.*` slots from `extended::apply_overrides()`. The `styles::*`
/// fields are in `STYLE_ROWS` and `SCALAR_ROWS`.
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

/// Every `button::Status`, as values rather than names.
///
/// Each function declares its own such list, of its own widget's `Status`. A
/// shape-B or shape-C function, which takes no status, uses `&[()]`.
#[cfg(feature = "widgets")]
const BUTTON_STATUSES: &[button::Status] = &[
    button::Status::Active,
    button::Status::Hovered,
    button::Status::Pressed,
    button::Status::Disabled,
];

/// The fill the native fields give a button in `status`, computed here from
/// the resolved theme alone -- never from the code under test.
///
/// The match has no catch-all arm, so a status added upstream fails to compile
/// in the contract as well as in `styles`.
#[cfg(feature = "widgets")]
fn native_button_fill(r: &ResolvedTheme, status: button::Status) -> Color {
    let base = to_color(r.button.background_color);
    match status {
        button::Status::Active => base,
        // The platform layers its hover and pressed fills over the button's
        // own fill; iced replaces, so the layer is composited first (C17).
        button::Status::Hovered => over(to_color(r.button.hover_background), base),
        // `active_background` is a soft option: `None` copies the hover fill.
        button::Status::Pressed => over(
            to_color(
                r.button
                    .active_background
                    .unwrap_or(r.button.hover_background),
            ),
            base,
        ),
        // A translucent disabled fill replaces the idle one and lets the
        // window through, so it is emitted as given.
        button::Status::Disabled => to_color(
            r.button
                .disabled_background
                .unwrap_or(r.button.background_color),
        ),
    }
}

/// The label color the native fields give a button in `status`.
#[cfg(feature = "widgets")]
fn native_button_label(r: &ResolvedTheme, status: button::Status) -> Color {
    to_color(match status {
        button::Status::Active => r.button.font.color,
        button::Status::Hovered => r.button.hover_text_color,
        button::Status::Pressed => r.button.active_text_color,
        button::Status::Disabled => r.button.disabled_text_color,
    })
}

/// One row of the style contract: a field of one `styles::*` output, the
/// native value it must equal, and the statuses it holds for.
///
/// The row is generic over the widget's own `Status`, so both sides match on
/// the real enum with no catch-all: a status the table forgets cannot fall
/// through to the base state and pass while asserting nothing. `field` is the
/// path the coverage tripwire enumerates. The expected value is a `Color`, not
/// an `Rgba`, because a state layer's expected value is two native colors
/// composited (section 3.2).
#[cfg(feature = "widgets")]
struct StyleRow<S: 'static> {
    field: &'static str,
    statuses: &'static [S],
    native: fn(&ResolvedTheme, S) -> Color,
    /// `Err` when the emitted field is not a flat color: a `None` or a
    /// gradient where the contract claims a color is a failure, not an unwrap.
    get: fn(&Theme, &ResolvedTheme, S) -> Result<Color, String>,
}

/// The same, for a `Style` field that is a length rather than a color.
#[cfg(feature = "widgets")]
struct ScalarRow<S: 'static> {
    field: &'static str,
    statuses: &'static [S],
    native: fn(&ResolvedTheme, S) -> f32,
    get: fn(&Theme, &ResolvedTheme, S) -> f32,
}

/// The flat color of an emitted `Background`, or the reason it is not one.
#[cfg(feature = "widgets")]
fn flat(background: Option<Background>) -> Result<Color, String> {
    match background {
        Some(Background::Color(color)) => Ok(color),
        Some(Background::Gradient(_)) => {
            Err("a gradient where the contract claims a color".to_string())
        }
        None => Err("no background where the contract claims a color".to_string()),
    }
}

/// Every color field of `styles::button`, and the native value it carries.
///
/// One such const per function; read with `BUTTON_SCALAR_ROWS` and `DERIVED`
/// together, they name every field of the `Style` it emits.
#[cfg(feature = "widgets")]
const BUTTON_ROWS: &[StyleRow<button::Status>] = &[
    StyleRow {
        field: "styles::button.background",
        statuses: BUTTON_STATUSES,
        native: native_button_fill,
        get: |t, r, s| flat(styles::button(r)(t, s).background),
    },
    StyleRow {
        field: "styles::button.text_color",
        statuses: BUTTON_STATUSES,
        native: native_button_label,
        get: |t, r, s| Ok(styles::button(r)(t, s).text_color),
    },
    StyleRow {
        field: "styles::button.border.color",
        statuses: BUTTON_STATUSES,
        native: |r, _| to_color(r.button.border.color),
        get: |t, r, s| Ok(styles::button(r)(t, s).border.color),
    },
];

#[cfg(feature = "widgets")]
const BUTTON_SCALAR_ROWS: &[ScalarRow<button::Status>] = &[
    ScalarRow {
        field: "styles::button.border.width",
        statuses: BUTTON_STATUSES,
        native: |r, _| r.button.border.line_width,
        get: |t, r, s| styles::button(r)(t, s).border.width,
    },
    ScalarRow {
        field: "styles::button.border.radius.top_left",
        statuses: BUTTON_STATUSES,
        native: |r, _| r.button.border.corner_radius,
        get: |t, r, s| styles::button(r)(t, s).border.radius.top_left,
    },
    ScalarRow {
        field: "styles::button.border.radius.top_right",
        statuses: BUTTON_STATUSES,
        native: |r, _| r.button.border.corner_radius,
        get: |t, r, s| styles::button(r)(t, s).border.radius.top_right,
    },
    ScalarRow {
        field: "styles::button.border.radius.bottom_right",
        statuses: BUTTON_STATUSES,
        native: |r, _| r.button.border.corner_radius,
        get: |t, r, s| styles::button(r)(t, s).border.radius.bottom_right,
    },
    ScalarRow {
        field: "styles::button.border.radius.bottom_left",
        statuses: BUTTON_STATUSES,
        native: |r, _| r.button.border.corner_radius,
        get: |t, r, s| styles::button(r)(t, s).border.radius.bottom_left,
    },
];

/// The `field` of every style row, from every function's consts.
///
/// One line per function; the coverage tripwire reads it, and `rows_claiming`
/// counts in it.
#[cfg(feature = "widgets")]
fn style_row_fields() -> Vec<&'static str> {
    let mut out = Vec::new();
    out.extend(BUTTON_ROWS.iter().map(|row| row.field));
    out.extend(BUTTON_SCALAR_ROWS.iter().map(|row| row.field));
    out
}

/// Assert one function's color rows over every combination and status.
#[cfg(feature = "widgets")]
fn check_style_rows<S: Copy + std::fmt::Debug>(
    rows: &[StyleRow<S>],
    combinations: &[Combination],
    failures: &mut Vec<String>,
) -> usize {
    let mut checks = 0;
    for c in combinations {
        for row in rows {
            for &status in row.statuses {
                checks += 1;
                let expected = (row.native)(&c.resolved, status);
                match (row.get)(&c.theme, &c.resolved, status) {
                    Ok(actual) if actual == expected => {}
                    Ok(actual) => failures.push(format!(
                        "{}: {} ({status:?}) is {}, native gives {}",
                        c.label(),
                        row.field,
                        show(actual),
                        show(expected)
                    )),
                    Err(why) => failures.push(format!(
                        "{}: {} ({status:?}) is {why}",
                        c.label(),
                        row.field
                    )),
                }
            }
        }
    }
    checks
}

/// The same for one function's scalar rows.
#[cfg(feature = "widgets")]
fn check_scalar_rows<S: Copy + std::fmt::Debug>(
    rows: &[ScalarRow<S>],
    combinations: &[Combination],
    failures: &mut Vec<String>,
) -> usize {
    let mut checks = 0;
    for c in combinations {
        for row in rows {
            for &status in row.statuses {
                checks += 1;
                let expected = (row.native)(&c.resolved, status);
                let actual = (row.get)(&c.theme, &c.resolved, status);
                if actual != expected {
                    failures.push(format!(
                        "{}: {} ({status:?}) is {actual}, native gives {expected}",
                        c.label(),
                        row.field
                    ));
                }
            }
        }
    }
    checks
}

/// Every field the tripwire walks that no row claims, with the derivation that
/// fills it instead. A field is in the rows or here, never both and never
/// neither -- that is the whole of the tripwire (section 5.2).
const DERIVED: &[(&str, &str)] = &[
    (
        "extended.secondary.base.text",
        "Pair::new(input.placeholder_color, extended.background.base.text) -- \
         the label iced already chose for the window",
    ),
    (
        "extended.secondary.strong.text",
        "copy of extended.secondary.base.text",
    ),
    (
        "extended.primary.base.color",
        "iced: Extended::generate(palette).primary.base.color, from palette.primary",
    ),
    (
        "extended.success.base.color",
        "iced: Extended::generate(palette).success.base.color, from palette.success",
    ),
    (
        "extended.danger.base.color",
        "iced: Extended::generate(palette).danger.base.color, from palette.danger",
    ),
    (
        "extended.warning.base.color",
        "iced: Extended::generate(palette).warning.base.color, from palette.warning",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::button.shadow",
        "iced default: button::Style::default().shadow -- the model has a \
         shadow color but no offset or blur",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::button.snap",
        "iced default: button::Style::default().snap -- a renderer setting, \
         cfg!(feature = \"crisp\")",
    ),
];

/// Native values iced 0.14 has no receiver for, each with its evidence.
///
/// Not a way out of a row: an entry here says the toolkit cannot carry the
/// value at all, so approximating it would state something untrue. Empty until
/// `styles::scrollbar` lands, whose `scrollbar.min_thumb_length` is the one
/// entry this release knows of (section 3.3).
const UNREACHABLE: &[(&str, &str)] = &[];

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

/// The three colors a contrast measurement needs: foreground, background, and
/// the surface that background is painted on.
#[cfg(feature = "widgets")]
type Layers = (Color, Color, Color);

/// One pair of the contrast assertion: a label on the fill a `styles::*`
/// output gives it, in one status, against the same pair from the native
/// fields. Here the connector controls both colors, so section 7's
/// no-degradation rule is asserted rather than printed.
#[cfg(feature = "widgets")]
struct StylePair<S: 'static> {
    what: &'static str,
    statuses: &'static [S],
    emitted: fn(&Theme, &ResolvedTheme, S) -> Result<Layers, String>,
    /// The same three, computed from the native fields alone.
    native: fn(&ResolvedTheme, S) -> Layers,
}

/// The `styles::button` pairs the assertion covers.
#[cfg(feature = "widgets")]
const BUTTON_PAIRS: &[StylePair<button::Status>] = &[StylePair {
    what: "button label",
    statuses: BUTTON_STATUSES,
    emitted: |t, r, s| {
        let style = styles::button(r)(t, s);
        // A button is painted on the window, so that is what a translucent
        // fill shows through to.
        flat(style.background).map(|fill| {
            (
                style.text_color,
                fill,
                to_color(r.defaults.background_color),
            )
        })
    },
    native: |r, s| {
        (
            native_button_label(r, s),
            native_button_fill(r, s),
            to_color(r.defaults.background_color),
        )
    },
}];

/// WCAG 2.1 AA for normal text. Only ever used to decide what to print.
const AA: f32 = 4.5;

/// Assert one function's contrast pairs over every combination and status:
/// section 7's no-degradation rule. Pairs below AA are collected for printing,
/// never asserted.
#[cfg(feature = "widgets")]
fn check_style_pairs<S: Copy + std::fmt::Debug>(
    pairs: &[StylePair<S>],
    combinations: &[Combination],
    failures: &mut Vec<String>,
    below_aa: &mut Vec<String>,
) -> usize {
    let mut checks = 0;
    for c in combinations {
        for pair in pairs {
            for &status in pair.statuses {
                checks += 1;
                let (native_fg, native_bg, native_surface) = (pair.native)(&c.resolved, status);
                let native = pair_ratio(native_fg, native_bg, native_surface);
                match (pair.emitted)(&c.theme, &c.resolved, status) {
                    Ok((fg, bg, surface)) => {
                        let emitted = pair_ratio(fg, bg, surface);
                        if emitted < native {
                            failures.push(format!(
                                "{}: {} ({status:?}) emitted {emitted:.4}, native {native:.4}; \
                                 emitted {} on {}, native {} on {}",
                                c.label(),
                                pair.what,
                                show(fg),
                                show(bg),
                                show(native_fg),
                                show(native_bg)
                            ));
                        }
                        if emitted < AA {
                            below_aa.push(format!(
                                "{}: {} ({status:?}) {emitted:.2}",
                                c.label(),
                                pair.what
                            ));
                        }
                    }
                    Err(why) => {
                        failures.push(format!("{}: {} ({status:?}) {why}", c.label(), pair.what));
                    }
                }
            }
        }
    }
    checks
}

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

/// Composite `top` over `bottom`: straight-alpha source-over, the general
/// case, so a ratio is measured on what the screen actually shows.
///
/// `bottom` is not assumed opaque: a translucent layer over a translucent one
/// -- `link.hover_background` over `link.background_color`, which every preset
/// states as `#00000000` -- stays translucent rather than becoming opaque.
/// Written independently of `styles::composite_over`, which it must agree
/// with; the properties both are held to are asserted below.
fn over(top: Color, bottom: Color) -> Color {
    let under = bottom.a * (1.0 - top.a);
    let alpha = top.a + under;
    if alpha <= 0.0 {
        return Color::TRANSPARENT;
    }
    if under <= 0.0 {
        // Nothing of `bottom` reaches the result; un-premultiplying would
        // divide by `top.a` and move the color by a few ulps.
        return top;
    }
    let blend = |t: f32, b: f32| (t * top.a + b * under) / alpha;
    Color {
        r: blend(top.r, bottom.r),
        g: blend(top.g, bottom.g),
        b: blend(top.b, bottom.b),
        a: alpha,
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

// ── The coverage tripwire (section 5.2) ─────────────────────────────────────
//
// Each enumerator destructures its target with no `..`, so a field added
// upstream fails to compile here until someone classifies it; the test then
// requires each name it yields to sit in exactly one of the rows and
// `DERIVED`.

/// Destructure `$value` with no `..` and push one dotted path per field into
/// `$out`, from a single list of field names.
///
/// One list, two uses: the same identifiers become the pattern and the
/// strings, so silencing the compile error an upstream field causes -- by
/// adding it to the list -- cannot leave its name out of the tripwire. A field
/// listed after `@nested` is bound to a variable of its own name instead of
/// being discarded, so a second invocation can walk it under a longer prefix.
macro_rules! leaves {
    (
        $out:ident, $value:expr, $prefix:literal,
        $($ty:ident)::+ { $($leaf:ident),* $(,)? $(@nested $($nested:ident),+ $(,)?)? }
    ) => {
        let $($ty)::+ { $($leaf: _,)* $($($nested,)+)? } = $value;
        $($out.push(concat!($prefix, ".", stringify!($leaf)));)*
    };
}

/// The six `Palette` inputs.
fn palette_fields(palette: &iced_core::theme::Palette) -> Vec<&'static str> {
    let mut out = Vec::new();
    leaves!(
        out,
        palette,
        "palette",
        iced_core::theme::Palette {
            background,
            text,
            primary,
            success,
            warning,
            danger
        }
    );
    out
}

/// Both fields of every `Pair` that `extended::apply_overrides` assigns, whole
/// or in part.
fn written_extended_fields(extended: &Extended) -> Vec<&'static str> {
    let mut out = Vec::new();
    leaves!(
        out,
        extended.secondary.base,
        "extended.secondary.base",
        Pair { color, text }
    );
    leaves!(
        out,
        extended.secondary.strong,
        "extended.secondary.strong",
        Pair { color, text }
    );
    leaves!(
        out,
        extended.background.weak,
        "extended.background.weak",
        Pair { color, text }
    );
    leaves!(
        out,
        extended.primary.base,
        "extended.primary.base",
        Pair { color, text }
    );
    leaves!(
        out,
        extended.success.base,
        "extended.success.base",
        Pair { color, text }
    );
    leaves!(
        out,
        extended.danger.base,
        "extended.danger.base",
        Pair { color, text }
    );
    leaves!(
        out,
        extended.warning.base,
        "extended.warning.base",
        Pair { color, text }
    );
    out
}

/// Every color slot of an `Extended` palette, flattened. Used to decide
/// mechanically which slots the connector writes: it starts from
/// `Extended::generate`, so a slot that differs from it is one we wrote.
fn extended_slots(extended: &Extended) -> Vec<(String, Color)> {
    let Extended {
        background,
        primary,
        secondary,
        success,
        warning,
        danger,
        // Not a color slot, and the connector leaves it as iced generated it.
        is_dark: _,
    } = extended;
    let iced_core::theme::palette::Background {
        base,
        weakest,
        weaker,
        weak,
        neutral,
        strong,
        stronger,
        strongest,
    } = background;
    let Primary {
        base: primary_base,
        weak: primary_weak,
        strong: primary_strong,
    } = primary;
    let Secondary {
        base: secondary_base,
        weak: secondary_weak,
        strong: secondary_strong,
    } = secondary;
    let Success {
        base: success_base,
        weak: success_weak,
        strong: success_strong,
    } = success;
    let Warning {
        base: warning_base,
        weak: warning_weak,
        strong: warning_strong,
    } = warning;
    let Danger {
        base: danger_base,
        weak: danger_weak,
        strong: danger_strong,
    } = danger;

    let mut out = Vec::new();
    for (path, pair) in [
        ("extended.background.base", base),
        ("extended.background.weakest", weakest),
        ("extended.background.weaker", weaker),
        ("extended.background.weak", weak),
        ("extended.background.neutral", neutral),
        ("extended.background.strong", strong),
        ("extended.background.stronger", stronger),
        ("extended.background.strongest", strongest),
        ("extended.primary.base", primary_base),
        ("extended.primary.weak", primary_weak),
        ("extended.primary.strong", primary_strong),
        ("extended.secondary.base", secondary_base),
        ("extended.secondary.weak", secondary_weak),
        ("extended.secondary.strong", secondary_strong),
        ("extended.success.base", success_base),
        ("extended.success.weak", success_weak),
        ("extended.success.strong", success_strong),
        ("extended.warning.base", warning_base),
        ("extended.warning.weak", warning_weak),
        ("extended.warning.strong", warning_strong),
        ("extended.danger.base", danger_base),
        ("extended.danger.weak", danger_weak),
        ("extended.danger.strong", danger_strong),
    ] {
        let Pair { color, text } = pair;
        out.push((format!("{path}.color"), *color));
        out.push((format!("{path}.text"), *text));
    }
    out
}

/// Every field of the `button::Style` the connector emits.
#[cfg(feature = "widgets")]
fn button_style_fields(style: &button::Style) -> Vec<&'static str> {
    let mut out = Vec::new();
    leaves!(out, style, "styles::button", button::Style {
        background, text_color, shadow, snap, @nested border
    });
    leaves!(out, border, "styles::button.border", Border { color, width, @nested radius });
    leaves!(
        out,
        radius,
        "styles::button.border.radius",
        Radius {
            top_left,
            top_right,
            bottom_right,
            bottom_left
        }
    );
    out
}

/// Every field the tripwire walks: the palette inputs, the extended slots the
/// connector writes, and each `styles::*` output's fields.
#[cfg_attr(not(feature = "widgets"), allow(unused_variables))]
fn named_fields(theme: &Theme, resolved: &ResolvedTheme) -> Vec<&'static str> {
    let mut out = palette_fields(&theme.palette());
    out.extend(written_extended_fields(theme.extended_palette()));
    #[cfg(feature = "widgets")]
    out.extend(button_style_fields(&styles::button(resolved)(
        theme,
        button::Status::Active,
    )));
    out
}

/// How many rows claim `field`.
#[cfg_attr(not(feature = "widgets"), allow(unused_mut))]
fn rows_claiming(field: &str) -> usize {
    let mut count = ROWS.iter().filter(|row| row.slot == field).count();
    #[cfg(feature = "widgets")]
    {
        count += style_row_fields().iter().filter(|f| **f == field).count();
    }
    count
}

#[test]
fn the_contract_covers_sixteen_presets_in_both_modes() -> native_theme::Result<()> {
    assert_eq!(
        combinations()?.len(),
        32,
        "the contract asserts over 16 presets x 2 modes; a shrinking preset \
         list must not silently narrow it"
    );
    Ok(())
}

#[test]
fn every_named_field_has_exactly_one_declared_source() -> native_theme::Result<()> {
    let combinations = combinations()?;
    let mut failures = Vec::new();

    for c in &combinations {
        for field in named_fields(&c.theme, &c.resolved) {
            let rows = rows_claiming(field);
            let derived = DERIVED.iter().filter(|(name, _)| *name == field).count();
            match (rows, derived) {
                (1, 0) | (0, 1) => {}
                (0, 0) => failures.push(format!("{field}: in neither the rows nor DERIVED")),
                (r, d) if r > 0 && d > 0 => {
                    failures.push(format!("{field}: claimed by {r} row(s) and by {d} DERIVED"));
                }
                (r, _) if r > 1 => failures.push(format!("{field}: {r} rows claim it")),
                (_, d) => failures.push(format!("{field}: {d} DERIVED entries")),
            }
        }
    }

    failures.sort();
    failures.dedup();
    assert!(
        failures.is_empty(),
        "{} field(s) are not classified:\n{}",
        failures.len(),
        failures.join("\n")
    );

    for (field, evidence) in UNREACHABLE {
        assert_eq!(
            rows_claiming(field),
            0,
            "{field} is listed as unreachable ({evidence}) yet a row maps it"
        );
    }
    Ok(())
}

#[test]
fn no_extended_slot_is_written_without_a_declared_source() -> native_theme::Result<()> {
    let combinations = combinations()?;
    let mut written: Vec<String> = Vec::new();

    for c in &combinations {
        // The connector builds its extended palette by handing iced's own
        // generator the same base palette and then overriding; a slot that
        // differs from the generator's answer is one the connector wrote.
        let generated = Extended::generate(crate::palette::to_palette(&c.resolved));
        for ((slot, ours), (_, theirs)) in extended_slots(c.theme.extended_palette())
            .into_iter()
            .zip(extended_slots(&generated))
        {
            if ours != theirs {
                written.push(slot);
            }
        }
    }

    written.sort();
    written.dedup();
    println!(
        "extended slots the connector writes, measured over {} combinations: {}",
        combinations.len(),
        written.join(", ")
    );

    let undeclared: Vec<&String> = written
        .iter()
        .filter(|slot| {
            rows_claiming(slot) == 0 && !DERIVED.iter().any(|(name, _)| *name == slot.as_str())
        })
        .collect();
    assert!(
        undeclared.is_empty(),
        "{} extended slot(s) differ from iced's own generator with no row and \
         no DERIVED entry: {:?}",
        undeclared.len(),
        undeclared
    );
    Ok(())
}

#[cfg(feature = "widgets")]
#[test]
fn every_status_list_names_each_status_once() {
    // Generic rows close the fall-through hole a `&str` status had, but not
    // this one: a status simply left out of a list makes the rows assert less
    // without ever failing. So each list is checked against every variant of
    // its own enum here. The match is exhaustive with no catch-all, so a
    // variant added upstream fails to compile in this function first.
    let all = [
        button::Status::Active,
        button::Status::Hovered,
        button::Status::Pressed,
        button::Status::Disabled,
    ];
    for status in all {
        match status {
            button::Status::Active
            | button::Status::Hovered
            | button::Status::Pressed
            | button::Status::Disabled => {}
        }
        assert!(
            BUTTON_STATUSES.contains(&status),
            "BUTTON_STATUSES does not list {status:?}, so no button row covers it"
        );
    }
    assert_eq!(
        BUTTON_STATUSES.len(),
        all.len(),
        "BUTTON_STATUSES must name each status exactly once"
    );
}

#[cfg(feature = "widgets")]
#[test]
fn every_style_field_equals_its_native_value() -> native_theme::Result<()> {
    let combinations = combinations()?;
    let mut failures = Vec::new();

    // One line per function.
    let checks = check_style_rows(BUTTON_ROWS, &combinations, &mut failures)
        + check_scalar_rows(BUTTON_SCALAR_ROWS, &combinations, &mut failures);

    assert!(
        failures.is_empty(),
        "{} of {checks} style contract checks failed:\n{}",
        failures.len(),
        failures.join("\n")
    );
    Ok(())
}

#[cfg(feature = "widgets")]
#[test]
fn style_contrast_never_degrades_the_native_pair() -> native_theme::Result<()> {
    let combinations = combinations()?;
    let mut failures = Vec::new();
    let mut below_aa = Vec::new();

    // One line per function.
    let checks = check_style_pairs(BUTTON_PAIRS, &combinations, &mut failures, &mut below_aa);

    println!(
        "--- styles contrast: {checks} pairs, {} below AA (printed, not asserted) ---",
        below_aa.len()
    );
    for line in &below_aa {
        println!("{line}");
    }
    assert!(
        failures.is_empty(),
        "{} of {checks} style pairs are worse than the platform's own:\n{}",
        failures.len(),
        failures.join("\n")
    );
    Ok(())
}

#[cfg(feature = "widgets")]
#[test]
fn a_hovered_button_composites_the_state_layer_over_its_idle_fill() -> native_theme::Result<()> {
    // Windows 11 light states a translucent hover layer, so this preset shows
    // the difference between layering and replacing (section 3.2).
    let resolved = native_theme::theme::Theme::preset("windows-11")?
        .into_variant(ColorMode::Light)?
        .into_resolved(&native_theme::ResolutionContext::for_tests())?;
    let theme = crate::to_theme(&resolved, "windows-11");

    let raw = to_color(resolved.button.hover_background);
    let expected = over(raw, to_color(resolved.button.background_color));
    assert!(
        raw.a < 1.0 && expected != raw,
        "this preset no longer states a translucent hover layer, so it no \
         longer tests compositing: raw {}, composited {}",
        show(raw),
        show(expected)
    );
    assert_eq!(
        flat(styles::button(&resolved)(&theme, button::Status::Hovered).background),
        Ok(expected),
        "the hovered fill must be the hover layer over the idle fill"
    );
    Ok(())
}

#[cfg(feature = "widgets")]
#[test]
fn a_cleared_soft_option_copies_the_base_state_value() -> native_theme::Result<()> {
    // No bundled preset leaves these `None` after resolution, so the fallback
    // is reached by clearing them here (section 3.2).
    let mut resolved = native_theme::theme::Theme::preset("windows-11")?
        .into_variant(ColorMode::Light)?
        .into_resolved(&native_theme::ResolutionContext::for_tests())?;
    assert!(
        resolved.button.active_background.is_some()
            && resolved.button.disabled_background.is_some(),
        "this preset no longer states both soft options, so clearing them \
         proves nothing"
    );
    resolved.button.active_background = None;
    resolved.button.disabled_background = None;
    let theme = crate::to_theme(&resolved, "windows-11");
    let style = styles::button(&resolved);

    let pressed = flat(style(&theme, button::Status::Pressed).background);
    let hovered = flat(style(&theme, button::Status::Hovered).background);
    assert!(
        pressed.is_ok() && hovered.is_ok(),
        "a button fill must be a flat color: pressed {pressed:?}, hovered {hovered:?}"
    );
    assert_eq!(
        pressed, hovered,
        "a cleared active_background copies the hover fill, layer included"
    );
    assert_eq!(
        flat(style(&theme, button::Status::Disabled).background),
        Ok(to_color(resolved.button.background_color)),
        "a cleared disabled_background copies the idle fill, as given"
    );
    Ok(())
}

#[cfg(feature = "widgets")]
#[test]
fn compositing_holds_its_three_properties_on_every_native_color() -> native_theme::Result<()> {
    let combinations = combinations()?;
    let mut failures = Vec::new();
    let mut transparent_bases = 0usize;

    for c in &combinations {
        let r = &c.resolved;
        // Real layers and real fills, translucent and opaque.
        let layers = [
            ("link.hover_background", to_color(r.link.hover_background)),
            (
                "button.hover_background",
                to_color(r.button.hover_background),
            ),
            (
                "button.background_color",
                to_color(r.button.background_color),
            ),
            ("defaults.text_color", to_color(r.defaults.text_color)),
        ];
        let base = to_color(r.button.background_color);
        let link_base = to_color(r.link.background_color);

        for (name, layer) in layers {
            let label = format!("{}: {name}", c.label());

            // (a) a layer over nothing is the layer: the link case.
            if styles::composite_over(layer, Color::TRANSPARENT) != layer {
                failures.push(format!("{label}: over a transparent base is not the layer"));
            }
            if link_base.a == 0.0 {
                transparent_bases += 1;
                if styles::composite_over(layer, link_base) != layer {
                    failures.push(format!(
                        "{label}: over link.background_color ({}) is not the layer",
                        show(link_base)
                    ));
                }
            }

            // (b) over an opaque base the result is opaque, and is the
            // simple mix this formula generalises -- computed here, not
            // copied from either implementation.
            if base.a == 1.0 {
                let out = styles::composite_over(layer, base);
                let mix = |l: f32, b: f32| l * layer.a + b * (1.0 - layer.a);
                let expected = Color {
                    r: mix(layer.r, base.r),
                    g: mix(layer.g, base.g),
                    b: mix(layer.b, base.b),
                    a: 1.0,
                };
                if out != expected {
                    failures.push(format!(
                        "{label}: over an opaque base is {}, the simple mix gives {}",
                        show(out),
                        show(expected)
                    ));
                }
            }

            // (c) an opaque layer hides whatever is under it.
            if layer.a == 1.0 && styles::composite_over(layer, link_base) != layer {
                failures.push(format!(
                    "{label}: an opaque layer did not survive unchanged"
                ));
            }

            // The contract's own helper must agree with the one under test.
            if over(layer, link_base) != styles::composite_over(layer, link_base) {
                failures.push(format!(
                    "{label}: contract `over` and `composite_over` disagree"
                ));
            }
        }
    }

    assert_eq!(
        transparent_bases,
        combinations.len() * 4,
        "every preset is supposed to state link.background_color as fully \
         transparent; if that changed, property (a) is no longer covered by \
         real data"
    );
    assert!(
        failures.is_empty(),
        "{} compositing propert(ies) failed:\n{}",
        failures.len(),
        failures.join("\n")
    );
    Ok(())
}
