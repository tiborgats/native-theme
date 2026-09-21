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
use iced_widget::overlay::menu;
#[cfg(feature = "widgets")]
use iced_widget::{button, checkbox, pick_list, radio, slider, text_editor, text_input, toggler};

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

/// One row of the style contract for a field that is a length rather than a
/// color.
///
/// Sparse, because a `Border`'s width and its four corners are one
/// `BorderRow`: what is left for this kind is a scalar outside a border that
/// the model has a source for. `radio::Style` states its border as a bare
/// width and color rather than as an iced `Border`; `slider::Style` has
/// `rail.width`, from `slider.track_height` (section 3.3), and a handle whose
/// size is a number inside an enum. `toggler::Style`'s `padding_ratio` and
/// `scrollable::Style`'s `gap` have no native source and are derived instead.
///
/// `get` returns a `Result` for the same reason `StyleRow`'s does: the
/// slider's handle radius is a number only while the handle is a circle, and
/// a rectangular one is a failure rather than an unwrap.
///
/// `native` takes the built theme as well as the resolved one, which no other
/// row kind does. One scalar is a *guarded* native value --
/// `toggler::Style.padding_ratio` needs a track with a height and a thumb that
/// fits it -- and where the guard does not hold the field follows the
/// no-source rule and becomes iced's own, which only the theme can answer.
/// The other rows ignore the argument.
#[cfg(feature = "widgets")]
struct ScalarRow<S: 'static> {
    field: &'static str,
    statuses: &'static [S],
    native: fn(&Theme, &ResolvedTheme, S) -> f32,
    get: fn(&Theme, &ResolvedTheme, S) -> Result<f32, String>,
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

/// The same, for a `Style` that carries its `Background` directly rather than
/// as an `Option`.
#[cfg(feature = "widgets")]
fn fill(background: Background) -> Result<Color, String> {
    flat(Some(background))
}

/// The color a `Style` field states, or the reason it states none.
///
/// Several `Style`s carry an `Option<Color>` whose `None` means "inherit".
/// Where the model has the color, the connector must state it, so a `None` is
/// a failure of the row rather than something to unwrap.
#[cfg(feature = "widgets")]
fn stated(color: Option<Color>) -> Result<Color, String> {
    color.ok_or_else(|| "no color where the contract claims one".to_string())
}

/// `leaves!` with a prefix that is a value rather than a literal, pushing
/// `String`s.
///
/// The one identifier list still becomes both the no-`..` destructure and the
/// names, so an upstream field cannot be silenced out of the tripwire; what it
/// buys is a prefix a caller can pass, which is what lets one `Border` walker
/// serve every function instead of two `leaves!` invocations per function.
/// `leaves!` itself is unchanged and still serves the palette enumerators,
/// whose names are `&'static str`.
#[cfg(feature = "widgets")]
macro_rules! leaves_under {
    (
        $out:ident, $value:expr, $prefix:expr,
        $($ty:ident)::+ { $($leaf:ident),* $(,)? $(@nested $($nested:ident),+ $(,)?)? }
    ) => {
        let $($ty)::+ { $($leaf: _,)* $($($nested,)+)? } = $value;
        $($out.push(format!("{}.{}", $prefix, stringify!($leaf)));)*
    };
}

/// Every leaf of an iced `Border` under `prefix`, destructured with no `..`.
///
/// One walker for all of them: a `Border` is the same six leaves wherever it
/// appears, so an upstream addition fails to compile here once rather than
/// once per function.
#[cfg(feature = "widgets")]
fn border_fields(border: &Border, prefix: &str) -> Vec<String> {
    let mut out = Vec::new();
    leaves_under!(out, border, prefix, Border { color, width, @nested radius });
    leaves_under!(
        out,
        radius,
        format!("{prefix}.radius"),
        Radius {
            top_left,
            top_right,
            bottom_right,
            bottom_left
        }
    );
    out
}

/// How many leaves a `Border` has. `check_border_rows` compares exactly this
/// many, so the two are held together by an assertion rather than by memory.
#[cfg(feature = "widgets")]
const BORDER_LEAVES: usize = 6;

/// The native values an iced `Border` needs: a color, a width, and one corner
/// radius the four corners share.
#[cfg(feature = "widgets")]
struct NativeBorder {
    color: Color,
    width: f32,
    radius: f32,
}

/// One row for a whole `Border`: six leaves, one native source, one entry.
///
/// A `Border` is six `Style` fields with three native values behind them, so
/// nine of this file's functions would otherwise repeat the same six rows.
/// This row claims all six names for the coverage tripwire and checks all six,
/// naming the exact leaf and status when one differs. `field` is the
/// function's dotted prefix -- `"styles::button"` -- and the leaves it claims
/// are `<field>.border.color`, `.border.width` and the four
/// `.border.radius.*`.
#[cfg(feature = "widgets")]
struct BorderRow<S: 'static> {
    field: &'static str,
    statuses: &'static [S],
    native: fn(&ResolvedTheme, S) -> NativeBorder,
    get: fn(&Theme, &ResolvedTheme, S) -> Border,
}

/// The six leaf names of every border row, for the tripwire and for
/// `rows_claiming`.
#[cfg(feature = "widgets")]
fn border_row_fields<S>(rows: &[BorderRow<S>]) -> Vec<String> {
    rows.iter()
        .flat_map(|row| border_fields(&Border::default(), &format!("{}.border", row.field)))
        .collect()
}

/// Assert one function's border rows over every combination and status, leaf
/// by leaf.
#[cfg(feature = "widgets")]
fn check_border_rows<S: Copy + std::fmt::Debug>(
    rows: &[BorderRow<S>],
    combinations: &[Combination],
    failures: &mut Vec<String>,
    ran: &mut Checked,
) {
    for row in rows {
        // One border row claims six leaves, so it records all six -- the same
        // names `style_row_fields` declares for it.
        for leaf in border_fields(&Border::default(), &format!("{}.border", row.field)) {
            ran.ran(&leaf, row.statuses);
        }
    }
    for c in combinations {
        for row in rows {
            for &status in row.statuses {
                let expected = (row.native)(&c.resolved, status);
                let actual = (row.get)(&c.theme, &c.resolved, status);

                ran.checks += 1;
                if actual.color != expected.color {
                    failures.push(format!(
                        "{}: {}.border.color ({status:?}) is {}, native gives {}",
                        c.label(),
                        row.field,
                        show(actual.color),
                        show(expected.color)
                    ));
                }

                ran.checks += 1;
                if actual.width != expected.width {
                    failures.push(format!(
                        "{}: {}.border.width ({status:?}) is {}, native gives {}",
                        c.label(),
                        row.field,
                        actual.width,
                        expected.width
                    ));
                }

                ran.checks += check_corners(
                    actual.radius,
                    expected.radius,
                    &format!("{}.border.radius", row.field),
                    &format!("{status:?}"),
                    &c.label(),
                    failures,
                );
            }
        }
    }
}

/// Compare the four corners of an emitted `Radius` against the one native
/// value the model states for all of them, naming the exact corner. Returns
/// how many comparisons it made.
///
/// The model states a corner radius as a single length, so the four corners
/// are one native value four times over; a row that claims a `Radius` claims
/// all four, and a failure has to say which one drifted.
#[cfg(feature = "widgets")]
fn check_corners(
    actual: Radius,
    expected: f32,
    field: &str,
    status: &str,
    label: &str,
    failures: &mut Vec<String>,
) -> usize {
    for (leaf, corner) in [
        ("top_left", actual.top_left),
        ("top_right", actual.top_right),
        ("bottom_right", actual.bottom_right),
        ("bottom_left", actual.bottom_left),
    ] {
        if corner != expected {
            failures.push(format!(
                "{label}: {field}.{leaf} ({status}) is {corner}, native gives {expected}"
            ));
        }
    }
    4
}

/// One row of the style contract for a `Style` field that is a whole `Radius`
/// rather than part of a `Border`.
///
/// `toggler::Style.border_radius` is the one such field: an `Option<Radius>`
/// whose `None` means "perfectly round, whatever the platform states", so a
/// `None` where the model has a value is a failure of the row rather than
/// something to unwrap. The row claims the one leaf name the tripwire walks,
/// `styles::toggler.border_radius`, and checks all four corners under it.
#[cfg(feature = "widgets")]
struct RadiusRow<S: 'static> {
    field: &'static str,
    statuses: &'static [S],
    native: fn(&ResolvedTheme, S) -> f32,
    get: fn(&Theme, &ResolvedTheme, S) -> Result<Radius, String>,
}

/// Assert one function's radius rows over every combination and status.
#[cfg(feature = "widgets")]
fn check_radius_rows<S: Copy + std::fmt::Debug>(
    rows: &[RadiusRow<S>],
    combinations: &[Combination],
    failures: &mut Vec<String>,
    ran: &mut Checked,
) {
    for row in rows {
        ran.ran(row.field, row.statuses);
    }
    for c in combinations {
        for row in rows {
            for &status in row.statuses {
                let expected = (row.native)(&c.resolved, status);
                match (row.get)(&c.theme, &c.resolved, status) {
                    Ok(actual) => {
                        ran.checks += check_corners(
                            actual,
                            expected,
                            row.field,
                            &format!("{status:?}"),
                            &c.label(),
                            failures,
                        );
                    }
                    Err(why) => {
                        ran.checks += 1;
                        failures.push(format!(
                            "{}: {} ({status:?}) is {why}",
                            c.label(),
                            row.field
                        ));
                    }
                }
            }
        }
    }
}

/// The border every button function wears: the button's own, in every status.
#[cfg(feature = "widgets")]
fn native_button_border(r: &ResolvedTheme) -> NativeBorder {
    NativeBorder {
        color: to_color(r.button.border.color),
        width: r.button.border.line_width,
        radius: r.button.border.corner_radius,
    }
}

/// Every color field of `styles::button`, and the native value it carries.
///
/// One such const per function; read with `BUTTON_BORDER_ROWS` and `DERIVED`
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
];

#[cfg(feature = "widgets")]
const BUTTON_BORDER_ROWS: &[BorderRow<button::Status>] = &[BorderRow {
    field: "styles::button",
    statuses: BUTTON_STATUSES,
    native: |r, _| native_button_border(r),
    get: |t, r, s| styles::button(r)(t, s).border,
}];

/// The statuses of a class button whose fill and label the function itself
/// decides, and so the only ones its rows and its contrast pair cover.
#[cfg(feature = "widgets")]
const CLASS_BUTTON_NATIVE_STATUSES: &[button::Status] =
    &[button::Status::Active, button::Status::Disabled];

/// The statuses a class button takes whole from iced's own class, because the
/// model states no hovered and no pressed variant of an accent or a status
/// color (spec section 3.3). Asserted by
/// `a_class_button_takes_its_hovered_and_pressed_states_from_iced`.
#[cfg(feature = "widgets")]
const CLASS_BUTTON_ICED_STATUSES: &[button::Status] =
    &[button::Status::Hovered, button::Status::Pressed];

/// The fill the native fields give a class button painted `idle`.
///
/// The match has no catch-all, but only the two statuses of
/// `CLASS_BUTTON_NATIVE_STATUSES` ever reach a row: the other two are iced's
/// on both sides. Their arm still has to name a color, and the idle one is the
/// least surprising -- if a status is ever moved between the two lists, the row
/// compares it against that and fails rather than passing quietly.
#[cfg(feature = "widgets")]
fn native_class_button_fill(r: &ResolvedTheme, status: button::Status, idle: Rgba) -> Color {
    match status {
        button::Status::Active | button::Status::Hovered | button::Status::Pressed => {
            to_color(idle)
        }
        button::Status::Disabled => to_color(
            r.button
                .disabled_background
                .unwrap_or(r.button.background_color),
        ),
    }
}

/// The label the native fields give a class button whose own label is `idle`.
#[cfg(feature = "widgets")]
fn native_class_button_label(r: &ResolvedTheme, status: button::Status, idle: Rgba) -> Color {
    to_color(match status {
        button::Status::Active | button::Status::Hovered | button::Status::Pressed => idle,
        button::Status::Disabled => r.button.disabled_text_color,
    })
}

/// Declare the whole contract of one class button: its two color rows, its
/// border row, its contrast pair and its field enumerator.
///
/// `styles::button_primary`, `button_danger`, `button_success` and
/// `button_warning` are the same function four times over -- they differ only
/// in the idle pair they paint and in which of iced's own classes fills their
/// hovered and pressed states -- so their contract is written once here and
/// instantiated four times. `$fill` and `$label` are accessors of the idle
/// pair, `fn(&ResolvedTheme) -> Rgba`.
macro_rules! class_button_contract {
    (
        $rows:ident, $borders:ident, $pairs:ident, $fields:ident,
        $prefix:literal, $style:path, $what:literal, $fill:expr, $label:expr
    ) => {
        #[cfg(feature = "widgets")]
        const $rows: &[StyleRow<button::Status>] = &[
            StyleRow {
                field: concat!($prefix, ".background"),
                statuses: CLASS_BUTTON_NATIVE_STATUSES,
                native: |r, s| native_class_button_fill(r, s, ($fill)(r)),
                get: |t, r, s| flat($style(r)(t, s).background),
            },
            StyleRow {
                field: concat!($prefix, ".text_color"),
                statuses: CLASS_BUTTON_NATIVE_STATUSES,
                native: |r, s| native_class_button_label(r, s, ($label)(r)),
                get: |t, r, s| Ok($style(r)(t, s).text_color),
            },
        ];

        #[cfg(feature = "widgets")]
        const $borders: &[BorderRow<button::Status>] = &[BorderRow {
            field: $prefix,
            statuses: BUTTON_STATUSES,
            native: |r, _| native_button_border(r),
            get: |t, r, s| $style(r)(t, s).border,
        }];

        /// Only `CLASS_BUTTON_NATIVE_STATUSES`: in the other two the function
        /// decides neither the fill nor the label -- both are iced's own class
        /// -- so there is no native pair to compare against and section 7's
        /// rule has nothing to say.
        #[cfg(feature = "widgets")]
        const $pairs: &[StylePair<button::Status>] = &[StylePair {
            what: $what,
            indicator: false,
            statuses: CLASS_BUTTON_NATIVE_STATUSES,
            emitted: |t, r, s| {
                let style = $style(r)(t, s);
                flat(style.background).map(|background| {
                    (
                        style.text_color,
                        background,
                        to_color(r.defaults.background_color),
                    )
                })
            },
            native: |r, s| {
                (
                    native_class_button_label(r, s, ($label)(r)),
                    native_class_button_fill(r, s, ($fill)(r)),
                    to_color(r.defaults.background_color),
                )
            },
        }];

        #[cfg(feature = "widgets")]
        fn $fields(style: &button::Style) -> Vec<String> {
            let mut out = Vec::new();
            leaves_under!(out, style, $prefix, button::Style {
                background, text_color, shadow, snap, @nested border
            });
            out.extend(border_fields(border, concat!($prefix, ".border")));
            out
        }
    };
}

class_button_contract!(
    BUTTON_PRIMARY_ROWS,
    BUTTON_PRIMARY_BORDER_ROWS,
    BUTTON_PRIMARY_PAIRS,
    button_primary_style_fields,
    "styles::button_primary",
    styles::button_primary,
    "primary button label",
    |r: &ResolvedTheme| r.button.primary_background,
    |r: &ResolvedTheme| r.button.primary_text_color
);

class_button_contract!(
    BUTTON_DANGER_ROWS,
    BUTTON_DANGER_BORDER_ROWS,
    BUTTON_DANGER_PAIRS,
    button_danger_style_fields,
    "styles::button_danger",
    styles::button_danger,
    "danger button label",
    |r: &ResolvedTheme| r.defaults.danger_color,
    |r: &ResolvedTheme| r.defaults.danger_text_color
);

class_button_contract!(
    BUTTON_SUCCESS_ROWS,
    BUTTON_SUCCESS_BORDER_ROWS,
    BUTTON_SUCCESS_PAIRS,
    button_success_style_fields,
    "styles::button_success",
    styles::button_success,
    "success button label",
    |r: &ResolvedTheme| r.defaults.success_color,
    |r: &ResolvedTheme| r.defaults.success_text_color
);

class_button_contract!(
    BUTTON_WARNING_ROWS,
    BUTTON_WARNING_BORDER_ROWS,
    BUTTON_WARNING_PAIRS,
    button_warning_style_fields,
    "styles::button_warning",
    styles::button_warning,
    "warning button label",
    |r: &ResolvedTheme| r.defaults.warning_color,
    |r: &ResolvedTheme| r.defaults.warning_text_color
);

/// The fill the native fields give a link button in `status`.
#[cfg(feature = "widgets")]
fn native_button_link_fill(r: &ResolvedTheme, status: button::Status) -> Color {
    let base = to_color(r.link.background_color);
    match status {
        // `LinkTheme` states no pressed and no disabled fill, so both keep the
        // idle one.
        button::Status::Active | button::Status::Pressed | button::Status::Disabled => base,
        button::Status::Hovered => over(to_color(r.link.hover_background), base),
    }
}

/// The label color the native fields give a link button in `status`.
#[cfg(feature = "widgets")]
fn native_button_link_label(r: &ResolvedTheme, status: button::Status) -> Color {
    to_color(match status {
        button::Status::Active => r.link.font.color,
        button::Status::Hovered => r.link.hover_text_color,
        button::Status::Pressed => r.link.active_text_color,
        button::Status::Disabled => r.link.disabled_text_color,
    })
}

/// Every color field of `styles::button_link`.
///
/// It has no scalar rows: `LinkTheme` carries no border, so all six border
/// leaves are iced's own and sit in `DERIVED`.
#[cfg(feature = "widgets")]
const BUTTON_LINK_ROWS: &[StyleRow<button::Status>] = &[
    StyleRow {
        field: "styles::button_link.background",
        statuses: BUTTON_STATUSES,
        native: native_button_link_fill,
        get: |t, r, s| flat(styles::button_link(r)(t, s).background),
    },
    StyleRow {
        field: "styles::button_link.text_color",
        statuses: BUTTON_STATUSES,
        native: native_button_link_label,
        get: |t, r, s| Ok(styles::button_link(r)(t, s).text_color),
    },
];

/// Every value of `text_input::Status`, not every variant: `Focused` carries a
/// `bool`, so it stands for two statuses and both are listed
/// (`text_input.rs:1697-1709`).
#[cfg(feature = "widgets")]
const TEXT_INPUT_STATUSES: &[text_input::Status] = &[
    text_input::Status::Active,
    text_input::Status::Hovered,
    text_input::Status::Focused { is_hovered: false },
    text_input::Status::Focused { is_hovered: true },
    text_input::Status::Disabled,
];

/// The fill the native fields give a text input in `status`.
#[cfg(feature = "widgets")]
fn native_input_fill(r: &ResolvedTheme, disabled: bool) -> Color {
    if disabled {
        // A translucent disabled fill replaces the idle one, so it is emitted
        // as given.
        to_color(
            r.input
                .disabled_background
                .unwrap_or(r.input.background_color),
        )
    } else {
        to_color(r.input.background_color)
    }
}

/// The border color the native fields give a text input, by state.
///
/// Both soft options copy the border's own color, the base-state value of
/// spec section 3.2's table.
#[cfg(feature = "widgets")]
fn native_input_border(r: &ResolvedTheme, hovered: bool, focused: bool) -> Color {
    let i = &r.input;
    to_color(if focused {
        i.focus_border_color.unwrap_or(i.border.color)
    } else if hovered {
        i.hover_border_color.unwrap_or(i.border.color)
    } else {
        i.border.color
    })
}

/// The text color the native fields give a text input's value, by state.
#[cfg(feature = "widgets")]
fn native_input_value(r: &ResolvedTheme, disabled: bool) -> Color {
    to_color(if disabled {
        r.input.disabled_text_color
    } else {
        r.input.font.color
    })
}

/// The three `input.*` values a `text_input::Status` selects, each matched
/// exhaustively with no catch-all so an upstream variant fails to compile here.
#[cfg(feature = "widgets")]
fn native_text_input_fill(r: &ResolvedTheme, status: text_input::Status) -> Color {
    match status {
        text_input::Status::Active
        | text_input::Status::Hovered
        | text_input::Status::Focused { is_hovered: _ } => native_input_fill(r, false),
        text_input::Status::Disabled => native_input_fill(r, true),
    }
}

#[cfg(feature = "widgets")]
fn native_text_input_border(r: &ResolvedTheme, status: text_input::Status) -> Color {
    match status {
        text_input::Status::Active | text_input::Status::Disabled => {
            native_input_border(r, false, false)
        }
        text_input::Status::Hovered => native_input_border(r, true, false),
        text_input::Status::Focused { is_hovered: _ } => native_input_border(r, false, true),
    }
}

#[cfg(feature = "widgets")]
fn native_text_input_value(r: &ResolvedTheme, status: text_input::Status) -> Color {
    match status {
        text_input::Status::Active
        | text_input::Status::Hovered
        | text_input::Status::Focused { is_hovered: _ } => native_input_value(r, false),
        text_input::Status::Disabled => native_input_value(r, true),
    }
}

/// Every color field of `styles::text_input`.
#[cfg(feature = "widgets")]
const TEXT_INPUT_ROWS: &[StyleRow<text_input::Status>] = &[
    StyleRow {
        field: "styles::text_input.background",
        statuses: TEXT_INPUT_STATUSES,
        native: native_text_input_fill,
        get: |t, r, s| fill(styles::text_input(r)(t, s).background),
    },
    StyleRow {
        field: "styles::text_input.placeholder",
        statuses: TEXT_INPUT_STATUSES,
        native: |r, _| to_color(r.input.placeholder_color),
        get: |t, r, s| Ok(styles::text_input(r)(t, s).placeholder),
    },
    StyleRow {
        field: "styles::text_input.value",
        statuses: TEXT_INPUT_STATUSES,
        native: native_text_input_value,
        get: |t, r, s| Ok(styles::text_input(r)(t, s).value),
    },
    StyleRow {
        field: "styles::text_input.selection",
        statuses: TEXT_INPUT_STATUSES,
        native: |r, _| to_color(r.input.selection_background),
        get: |t, r, s| Ok(styles::text_input(r)(t, s).selection),
    },
];

/// The input's border: its width and radius are status-independent, its color
/// is not.
#[cfg(feature = "widgets")]
const TEXT_INPUT_BORDER_ROWS: &[BorderRow<text_input::Status>] = &[BorderRow {
    field: "styles::text_input",
    statuses: TEXT_INPUT_STATUSES,
    native: |r, s| NativeBorder {
        color: native_text_input_border(r, s),
        width: r.input.border.line_width,
        radius: r.input.border.corner_radius,
    },
    get: |t, r, s| styles::text_input(r)(t, s).border,
}];

/// Every value of `text_editor::Status`; `Focused` carries a `bool` there too
/// (`text_editor.rs:1409-1421`).
#[cfg(feature = "widgets")]
const TEXT_EDITOR_STATUSES: &[text_editor::Status] = &[
    text_editor::Status::Active,
    text_editor::Status::Hovered,
    text_editor::Status::Focused { is_hovered: false },
    text_editor::Status::Focused { is_hovered: true },
    text_editor::Status::Disabled,
];

/// The same three `input.*` values, selected by a `text_editor::Status`. The
/// enum is a separate type from `text_input`'s, so the match is written again
/// rather than shared -- and again with no catch-all.
#[cfg(feature = "widgets")]
fn native_text_editor_fill(r: &ResolvedTheme, status: text_editor::Status) -> Color {
    match status {
        text_editor::Status::Active
        | text_editor::Status::Hovered
        | text_editor::Status::Focused { is_hovered: _ } => native_input_fill(r, false),
        text_editor::Status::Disabled => native_input_fill(r, true),
    }
}

#[cfg(feature = "widgets")]
fn native_text_editor_border(r: &ResolvedTheme, status: text_editor::Status) -> Color {
    match status {
        text_editor::Status::Active | text_editor::Status::Disabled => {
            native_input_border(r, false, false)
        }
        text_editor::Status::Hovered => native_input_border(r, true, false),
        text_editor::Status::Focused { is_hovered: _ } => native_input_border(r, false, true),
    }
}

#[cfg(feature = "widgets")]
fn native_text_editor_value(r: &ResolvedTheme, status: text_editor::Status) -> Color {
    match status {
        text_editor::Status::Active
        | text_editor::Status::Hovered
        | text_editor::Status::Focused { is_hovered: _ } => native_input_value(r, false),
        text_editor::Status::Disabled => native_input_value(r, true),
    }
}

/// Every color field of `styles::text_editor`. There is no `DERIVED` entry for
/// this function: `text_editor::Style` has no field the model cannot fill.
#[cfg(feature = "widgets")]
const TEXT_EDITOR_ROWS: &[StyleRow<text_editor::Status>] = &[
    StyleRow {
        field: "styles::text_editor.background",
        statuses: TEXT_EDITOR_STATUSES,
        native: native_text_editor_fill,
        get: |t, r, s| fill(styles::text_editor(r)(t, s).background),
    },
    StyleRow {
        field: "styles::text_editor.placeholder",
        statuses: TEXT_EDITOR_STATUSES,
        native: |r, _| to_color(r.input.placeholder_color),
        get: |t, r, s| Ok(styles::text_editor(r)(t, s).placeholder),
    },
    StyleRow {
        field: "styles::text_editor.value",
        statuses: TEXT_EDITOR_STATUSES,
        native: native_text_editor_value,
        get: |t, r, s| Ok(styles::text_editor(r)(t, s).value),
    },
    StyleRow {
        field: "styles::text_editor.selection",
        statuses: TEXT_EDITOR_STATUSES,
        native: |r, _| to_color(r.input.selection_background),
        get: |t, r, s| Ok(styles::text_editor(r)(t, s).selection),
    },
];

/// The same border, through the editor's own `Status`.
#[cfg(feature = "widgets")]
const TEXT_EDITOR_BORDER_ROWS: &[BorderRow<text_editor::Status>] = &[BorderRow {
    field: "styles::text_editor",
    statuses: TEXT_EDITOR_STATUSES,
    native: |r, s| NativeBorder {
        color: native_text_editor_border(r, s),
        width: r.input.border.line_width,
        radius: r.input.border.corner_radius,
    },
    get: |t, r, s| styles::text_editor(r)(t, s).border,
}];

/// Every value of `checkbox::Status`, not every variant: each of the three
/// carries whether the box is checked (`checkbox.rs:506-524`), so there are
/// six.
#[cfg(feature = "widgets")]
const CHECKBOX_STATUSES: &[checkbox::Status] = &[
    checkbox::Status::Active { is_checked: false },
    checkbox::Status::Active { is_checked: true },
    checkbox::Status::Hovered { is_checked: false },
    checkbox::Status::Hovered { is_checked: true },
    checkbox::Status::Disabled { is_checked: false },
    checkbox::Status::Disabled { is_checked: true },
];

/// The three statuses in which the check mark is painted, and so the only ones
/// its contrast pair measures.
#[cfg(feature = "widgets")]
const CHECKBOX_CHECKED_STATUSES: &[checkbox::Status] = &[
    checkbox::Status::Active { is_checked: true },
    checkbox::Status::Hovered { is_checked: true },
    checkbox::Status::Disabled { is_checked: true },
];

/// The other three. An unchecked box paints no mark, so the mark's color on
/// its fill is a ratio nothing shows; this list exists so that the two can be
/// required to partition the enum rather than the pair quietly covering half
/// of it.
#[cfg(feature = "widgets")]
const CHECKBOX_UNCHECKED_STATUSES: &[checkbox::Status] = &[
    checkbox::Status::Active { is_checked: false },
    checkbox::Status::Hovered { is_checked: false },
    checkbox::Status::Disabled { is_checked: false },
];

/// The fill the native fields give a checkbox at rest, by whether it is
/// checked. `unchecked_background` is a soft option copying the checkbox's own
/// fill, and it is an idle fill, so it is emitted as given even where the
/// platform states it translucent -- windows-11 does, at `#0000000a`.
#[cfg(feature = "widgets")]
fn native_checkbox_idle(r: &ResolvedTheme, is_checked: bool) -> Color {
    let c = &r.checkbox;
    to_color(if is_checked {
        c.checked_background
    } else {
        c.unchecked_background.unwrap_or(c.background_color)
    })
}

/// The fill the native fields give a checkbox in `status`.
#[cfg(feature = "widgets")]
fn native_checkbox_fill(r: &ResolvedTheme, status: checkbox::Status) -> Color {
    let c = &r.checkbox;
    match status {
        checkbox::Status::Active { is_checked } => native_checkbox_idle(r, is_checked),
        // The hover layer goes over whichever fill the box is showing (C17).
        checkbox::Status::Hovered { is_checked } => over(
            to_color(c.hover_background.unwrap_or(c.background_color)),
            native_checkbox_idle(r, is_checked),
        ),
        // One disabled fill for both, replacing the idle one, as given.
        checkbox::Status::Disabled { is_checked: _ } => {
            to_color(c.disabled_background.unwrap_or(c.background_color))
        }
    }
}

/// The outline the native fields give a checkbox or a radio button: an
/// unchecked one may state a color of its own, a checked one wears the
/// widget's border.
#[cfg(feature = "widgets")]
fn native_checkbox_outline(r: &ResolvedTheme, is_checked: bool) -> Color {
    let c = &r.checkbox;
    to_color(if is_checked {
        c.border.color
    } else {
        c.unchecked_border_color.unwrap_or(c.border.color)
    })
}

/// The label the native fields give a checkbox in `status`: the platform dims
/// a disabled one, and states the color it dims it to.
#[cfg(feature = "widgets")]
fn native_checkbox_label(r: &ResolvedTheme, status: checkbox::Status) -> Color {
    let c = &r.checkbox;
    to_color(match status {
        checkbox::Status::Active { is_checked: _ }
        | checkbox::Status::Hovered { is_checked: _ } => c.font.color,
        checkbox::Status::Disabled { is_checked: _ } => c.disabled_text_color,
    })
}

#[cfg(feature = "widgets")]
fn native_checkbox_border(r: &ResolvedTheme, status: checkbox::Status) -> Color {
    match status {
        checkbox::Status::Active { is_checked }
        | checkbox::Status::Hovered { is_checked }
        | checkbox::Status::Disabled { is_checked } => native_checkbox_outline(r, is_checked),
    }
}

/// Every color field of `styles::checkbox`. There is no `DERIVED` entry for
/// this function: `checkbox::Style` has no field the model cannot fill.
#[cfg(feature = "widgets")]
const CHECKBOX_ROWS: &[StyleRow<checkbox::Status>] = &[
    StyleRow {
        field: "styles::checkbox.background",
        statuses: CHECKBOX_STATUSES,
        native: native_checkbox_fill,
        get: |t, r, s| fill(styles::checkbox(r)(t, s).background),
    },
    StyleRow {
        field: "styles::checkbox.icon_color",
        statuses: CHECKBOX_STATUSES,
        native: |r, _| to_color(r.checkbox.indicator_color),
        get: |t, r, s| Ok(styles::checkbox(r)(t, s).icon_color),
    },
    StyleRow {
        field: "styles::checkbox.text_color",
        statuses: CHECKBOX_STATUSES,
        native: native_checkbox_label,
        get: |t, r, s| stated(styles::checkbox(r)(t, s).text_color),
    },
];

#[cfg(feature = "widgets")]
const CHECKBOX_BORDER_ROWS: &[BorderRow<checkbox::Status>] = &[BorderRow {
    field: "styles::checkbox",
    statuses: CHECKBOX_STATUSES,
    native: |r, s| NativeBorder {
        color: native_checkbox_border(r, s),
        width: r.checkbox.border.line_width,
        radius: r.checkbox.border.corner_radius,
    },
    get: |t, r, s| styles::checkbox(r)(t, s).border,
}];

/// Every value of `radio::Status`: two variants carrying whether the button is
/// selected (`radio.rs:474-487`), so four. There is no disabled variant.
#[cfg(feature = "widgets")]
const RADIO_STATUSES: &[radio::Status] = &[
    radio::Status::Active { is_selected: false },
    radio::Status::Active { is_selected: true },
    radio::Status::Hovered { is_selected: false },
    radio::Status::Hovered { is_selected: true },
];

/// The two statuses in which the dot is painted, and so the only ones its
/// contrast pair measures.
#[cfg(feature = "widgets")]
const RADIO_SELECTED_STATUSES: &[radio::Status] = &[
    radio::Status::Active { is_selected: true },
    radio::Status::Hovered { is_selected: true },
];

/// The other two, for the same reason `CHECKBOX_UNCHECKED_STATUSES` exists.
#[cfg(feature = "widgets")]
const RADIO_UNSELECTED_STATUSES: &[radio::Status] = &[
    radio::Status::Active { is_selected: false },
    radio::Status::Hovered { is_selected: false },
];

/// The fill the native fields give a radio button in `status`. The model
/// states one `CheckboxTheme` for both controls, so these are the checkbox's
/// own fills, selected by `is_selected`.
#[cfg(feature = "widgets")]
fn native_radio_fill(r: &ResolvedTheme, status: radio::Status) -> Color {
    let c = &r.checkbox;
    match status {
        radio::Status::Active { is_selected } => native_checkbox_idle(r, is_selected),
        radio::Status::Hovered { is_selected } => over(
            to_color(c.hover_background.unwrap_or(c.background_color)),
            native_checkbox_idle(r, is_selected),
        ),
    }
}

#[cfg(feature = "widgets")]
fn native_radio_border(r: &ResolvedTheme, status: radio::Status) -> Color {
    match status {
        radio::Status::Active { is_selected } | radio::Status::Hovered { is_selected } => {
            native_checkbox_outline(r, is_selected)
        }
    }
}

/// Every color field of `styles::radio`. Nothing here is iced's either.
#[cfg(feature = "widgets")]
const RADIO_ROWS: &[StyleRow<radio::Status>] = &[
    StyleRow {
        field: "styles::radio.background",
        statuses: RADIO_STATUSES,
        native: native_radio_fill,
        get: |t, r, s| fill(styles::radio(r)(t, s).background),
    },
    StyleRow {
        field: "styles::radio.dot_color",
        statuses: RADIO_STATUSES,
        native: |r, _| to_color(r.checkbox.indicator_color),
        get: |t, r, s| Ok(styles::radio(r)(t, s).dot_color),
    },
    StyleRow {
        field: "styles::radio.border_color",
        statuses: RADIO_STATUSES,
        native: native_radio_border,
        get: |t, r, s| Ok(styles::radio(r)(t, s).border_color),
    },
    StyleRow {
        field: "styles::radio.text_color",
        statuses: RADIO_STATUSES,
        native: |r, _| to_color(r.checkbox.font.color),
        get: |t, r, s| stated(styles::radio(r)(t, s).text_color),
    },
];

/// `radio::Style` states its border width on its own rather than inside an
/// iced `Border`, so it is a scalar row rather than part of a border row.
#[cfg(feature = "widgets")]
const RADIO_SCALAR_ROWS: &[ScalarRow<radio::Status>] = &[ScalarRow {
    field: "styles::radio.border_width",
    statuses: RADIO_STATUSES,
    native: |_, r, _| r.checkbox.border.line_width,
    get: |t, r, s| Ok(styles::radio(r)(t, s).border_width),
}];

/// Every value of `toggler::Status`: three variants carrying whether the
/// switch is on (`toggler.rs:486-504`), so six.
#[cfg(feature = "widgets")]
const TOGGLER_STATUSES: &[toggler::Status] = &[
    toggler::Status::Active { is_toggled: false },
    toggler::Status::Active { is_toggled: true },
    toggler::Status::Hovered { is_toggled: false },
    toggler::Status::Hovered { is_toggled: true },
    toggler::Status::Disabled { is_toggled: false },
    toggler::Status::Disabled { is_toggled: true },
];

/// The track the native fields give a switch at rest, by whether it is on.
#[cfg(feature = "widgets")]
fn native_toggler_idle(r: &ResolvedTheme, is_toggled: bool) -> Color {
    to_color(if is_toggled {
        r.switch.checked_background
    } else {
        r.switch.unchecked_background
    })
}

/// The track the native fields give a switch in `status`.
#[cfg(feature = "widgets")]
fn native_toggler_track(r: &ResolvedTheme, status: toggler::Status) -> Color {
    let s = &r.switch;
    match status {
        toggler::Status::Active { is_toggled } => native_toggler_idle(r, is_toggled),
        // Each hover layer copies the track it covers when the platform
        // states none, and is composited over it (C17).
        toggler::Status::Hovered { is_toggled } => over(
            to_color(if is_toggled {
                s.hover_checked_background.unwrap_or(s.checked_background)
            } else {
                s.hover_unchecked_background
                    .unwrap_or(s.unchecked_background)
            }),
            native_toggler_idle(r, is_toggled),
        ),
        // A disabled track replaces the idle one, so it is as given.
        toggler::Status::Disabled { is_toggled } => to_color(if is_toggled {
            s.disabled_checked_background
                .unwrap_or(s.checked_background)
        } else {
            s.disabled_unchecked_background
                .unwrap_or(s.unchecked_background)
        }),
    }
}

/// The thumb the native fields give a switch in `status`. A thumb is painted
/// over a track the widget also paints, so it is emitted as given.
#[cfg(feature = "widgets")]
fn native_toggler_thumb(r: &ResolvedTheme, status: toggler::Status) -> Color {
    let s = &r.switch;
    to_color(match status {
        toggler::Status::Active { is_toggled: _ } | toggler::Status::Hovered { is_toggled: _ } => {
            s.thumb_background
        }
        toggler::Status::Disabled { is_toggled: _ } => {
            s.disabled_thumb_color.unwrap_or(s.thumb_background)
        }
    })
}

/// Every color field of `styles::toggler`. The other six fields of
/// `toggler::Style` are in `DERIVED`: `SwitchTheme` carries no border, no thumb
/// inset and no font.
#[cfg(feature = "widgets")]
const TOGGLER_ROWS: &[StyleRow<toggler::Status>] = &[
    StyleRow {
        field: "styles::toggler.background",
        statuses: TOGGLER_STATUSES,
        native: native_toggler_track,
        get: |t, r, s| fill(styles::toggler(r)(t, s).background),
    },
    StyleRow {
        field: "styles::toggler.foreground",
        statuses: TOGGLER_STATUSES,
        native: native_toggler_thumb,
        get: |t, r, s| fill(styles::toggler(r)(t, s).foreground),
    },
];

/// Whether the model's switch geometry can state the inset iced asks for: a
/// track with a height, and a thumb that fits inside it.
///
/// Written here as well as in `styles::toggler`, because the contract's side
/// of a guarded value has to be computed independently of the code under test.
#[cfg(feature = "widgets")]
fn switch_geometry_is_usable(r: &ResolvedTheme) -> bool {
    let s = &r.switch;
    s.track_height > 0.0 && s.thumb_diameter >= 0.0 && s.thumb_diameter <= s.track_height
}

/// The inset the native fields state between the track and the thumb, as the
/// ratio iced's receiver takes.
///
/// iced multiplies this by the track's height to get the padding on each side
/// (`toggler.rs:444`) and gives the thumb what is left (`toggler.rs:453-454`),
/// so the model's two lengths state it exactly. Where the geometry cannot
/// state it, the field follows the no-source rule and the expected value is
/// iced's own, read at run time -- never a literal.
#[cfg(feature = "widgets")]
fn native_toggler_padding_ratio(t: &Theme, r: &ResolvedTheme, status: toggler::Status) -> f32 {
    let s = &r.switch;
    if switch_geometry_is_usable(r) {
        (s.track_height - s.thumb_diameter) / (2.0 * s.track_height)
    } else {
        toggler::default(t, status).padding_ratio
    }
}

/// The switch's thumb inset, the one scalar whose native value is guarded.
#[cfg(feature = "widgets")]
const TOGGLER_SCALAR_ROWS: &[ScalarRow<toggler::Status>] = &[ScalarRow {
    field: "styles::toggler.padding_ratio",
    statuses: TOGGLER_STATUSES,
    native: native_toggler_padding_ratio,
    get: |t, r, s| Ok(styles::toggler(r)(t, s).padding_ratio),
}];

/// The switch's corner radius, `switch.track_radius`. It shapes the thumb as
/// well as the track: iced paints both quads with this one radius
/// (`toggler.rs:435`, `:461`).
#[cfg(feature = "widgets")]
const TOGGLER_RADIUS_ROWS: &[RadiusRow<toggler::Status>] = &[RadiusRow {
    field: "styles::toggler.border_radius",
    statuses: TOGGLER_STATUSES,
    native: |r, _| r.switch.track_radius,
    get: |t, r, s| {
        styles::toggler(r)(t, s).border_radius.ok_or_else(|| {
            "no radius where the contract claims one; iced's None would make \
             the switch perfectly round whatever the platform states"
                .to_string()
        })
    },
}];

/// Every value of `pick_list::Status`: `Opened` carries whether the pointer is
/// on the field (`pick_list.rs:838-849`), so there are four.
#[cfg(feature = "widgets")]
const PICK_LIST_STATUSES: &[pick_list::Status] = &[
    pick_list::Status::Active,
    pick_list::Status::Hovered,
    pick_list::Status::Opened { is_hovered: false },
    pick_list::Status::Opened { is_hovered: true },
];

/// The fill the native fields give a drop-down field in `status`. The model
/// states no open appearance, so an open field follows its own `is_hovered`.
#[cfg(feature = "widgets")]
fn native_pick_list_fill(r: &ResolvedTheme, status: pick_list::Status) -> Color {
    let c = &r.combo_box;
    let idle = to_color(c.background_color);
    let hovered = || {
        over(
            to_color(c.hover_background.unwrap_or(c.background_color)),
            idle,
        )
    };
    match status {
        pick_list::Status::Active => idle,
        pick_list::Status::Hovered => hovered(),
        pick_list::Status::Opened { is_hovered } => {
            if is_hovered {
                hovered()
            } else {
                idle
            }
        }
    }
}

/// Every color field of `styles::pick_list` but `handle_color`, which is
/// iced's and sits in `DERIVED`.
#[cfg(feature = "widgets")]
const PICK_LIST_ROWS: &[StyleRow<pick_list::Status>] = &[
    StyleRow {
        field: "styles::pick_list.background",
        statuses: PICK_LIST_STATUSES,
        native: native_pick_list_fill,
        get: |t, r, s| fill(styles::pick_list(r)(t, s).background),
    },
    StyleRow {
        field: "styles::pick_list.text_color",
        statuses: PICK_LIST_STATUSES,
        native: |r, _| to_color(r.combo_box.font.color),
        get: |t, r, s| Ok(styles::pick_list(r)(t, s).text_color),
    },
    StyleRow {
        field: "styles::pick_list.placeholder_color",
        statuses: PICK_LIST_STATUSES,
        native: |r, _| to_color(r.input.placeholder_color),
        get: |t, r, s| Ok(styles::pick_list(r)(t, s).placeholder_color),
    },
];

#[cfg(feature = "widgets")]
const PICK_LIST_BORDER_ROWS: &[BorderRow<pick_list::Status>] = &[BorderRow {
    field: "styles::pick_list",
    statuses: PICK_LIST_STATUSES,
    native: |r, _| NativeBorder {
        color: to_color(r.combo_box.border.color),
        width: r.combo_box.border.line_width,
        radius: r.combo_box.border.corner_radius,
    },
    get: |t, r, s| styles::pick_list(r)(t, s).border,
}];

/// A menu has no `Status` (shape C), so its rows hold the unit value: the row
/// types are generic over the widget's status, and `()` is what a widget
/// without one has.
#[cfg(feature = "widgets")]
const MENU_STATUSES: &[()] = &[()];

/// Every color field of `styles::menu`. Only `shadow` is iced's.
#[cfg(feature = "widgets")]
const MENU_ROWS: &[StyleRow<()>] = &[
    StyleRow {
        field: "styles::menu.background",
        statuses: MENU_STATUSES,
        native: |r, ()| to_color(r.menu.background_color),
        get: |t, r, ()| fill(styles::menu(r)(t).background),
    },
    StyleRow {
        field: "styles::menu.text_color",
        statuses: MENU_STATUSES,
        native: |r, ()| to_color(r.menu.font.color),
        get: |t, r, ()| Ok(styles::menu(r)(t).text_color),
    },
    StyleRow {
        field: "styles::menu.selected_text_color",
        statuses: MENU_STATUSES,
        native: |r, ()| to_color(r.menu.hover_text_color),
        get: |t, r, ()| Ok(styles::menu(r)(t).selected_text_color),
    },
    StyleRow {
        // A row highlight, painted over a panel the widget also paints, so it
        // is emitted as given rather than composited.
        field: "styles::menu.selected_background",
        statuses: MENU_STATUSES,
        native: |r, ()| to_color(r.menu.hover_background),
        get: |t, r, ()| fill(styles::menu(r)(t).selected_background),
    },
];

#[cfg(feature = "widgets")]
const MENU_BORDER_ROWS: &[BorderRow<()>] = &[BorderRow {
    field: "styles::menu",
    statuses: MENU_STATUSES,
    native: |r, ()| NativeBorder {
        color: to_color(r.menu.border.color),
        width: r.menu.border.line_width,
        radius: r.menu.border.corner_radius,
    },
    get: |t, r, ()| styles::menu(r)(t).border,
}];

/// Every `slider::Status` (`slider.rs:576-586`). The rail's rows hold for all
/// three; the handle's fill does not.
#[cfg(feature = "widgets")]
const SLIDER_STATUSES: &[slider::Status] = &[
    slider::Status::Active,
    slider::Status::Hovered,
    slider::Status::Dragged,
];

/// The statuses whose handle the model states a *fill* for. Only the fill: the
/// handle's shape is `slider.thumb_diameter` in every status, dragged
/// included, and so is the rail under it.
#[cfg(feature = "widgets")]
const SLIDER_NATIVE_FILL_STATUSES: &[slider::Status] =
    &[slider::Status::Active, slider::Status::Hovered];

/// The status whose handle fill is iced's, because `SliderTheme` states no
/// dragged thumb color. Asserted by
/// `a_dragged_slider_handle_takes_its_fill_from_iced`.
#[cfg(feature = "widgets")]
const SLIDER_ICED_FILL_STATUSES: &[slider::Status] = &[slider::Status::Dragged];

/// The handle fill the native fields give a slider in `status`.
///
/// The match has no catch-all, but only `SLIDER_NATIVE_FILL_STATUSES` ever
/// reaches a row; `Dragged` still has to name a color, and the thumb's own is
/// the least surprising, so moving that status between the lists fails the row
/// rather than passing quietly.
#[cfg(feature = "widgets")]
fn native_slider_handle(r: &ResolvedTheme, status: slider::Status) -> Color {
    let s = &r.slider;
    to_color(match status {
        slider::Status::Active | slider::Status::Dragged => s.thumb_color,
        // A thumb, so emitted as given rather than composited.
        slider::Status::Hovered => s.thumb_hover_color.unwrap_or(s.thumb_color),
    })
}

/// Every color field of `styles::slider`.
#[cfg(feature = "widgets")]
const SLIDER_ROWS: &[StyleRow<slider::Status>] = &[
    StyleRow {
        field: "styles::slider.rail.backgrounds.0",
        statuses: SLIDER_STATUSES,
        native: |r, _| to_color(r.slider.fill_color),
        get: |t, r, s| fill(styles::slider(r)(t, s).rail.backgrounds.0),
    },
    StyleRow {
        field: "styles::slider.rail.backgrounds.1",
        statuses: SLIDER_STATUSES,
        native: |r, _| to_color(r.slider.track_color),
        get: |t, r, s| fill(styles::slider(r)(t, s).rail.backgrounds.1),
    },
    StyleRow {
        field: "styles::slider.handle.background",
        statuses: SLIDER_NATIVE_FILL_STATUSES,
        native: native_slider_handle,
        get: |t, r, s| fill(styles::slider(r)(t, s).handle.background),
    },
];

/// The slider's two lengths.
///
/// `handle.shape` is an enum rather than a number, and this row is what
/// `ScalarRow`'s `Result` exists for: the shape carries the size the model
/// states only while it is a circle, so a rectangular handle is reported as a
/// failure of the row instead of being unwrapped. Adding a row kind for an
/// enum whose one native value is a length would have said less.
#[cfg(feature = "widgets")]
const SLIDER_SCALAR_ROWS: &[ScalarRow<slider::Status>] = &[
    ScalarRow {
        field: "styles::slider.rail.width",
        statuses: SLIDER_STATUSES,
        native: |_, r, _| r.slider.track_height,
        get: |t, r, s| Ok(styles::slider(r)(t, s).rail.width),
    },
    ScalarRow {
        field: "styles::slider.handle.shape",
        statuses: SLIDER_STATUSES,
        // iced states a circular handle by its radius, the model states the
        // thumb's diameter.
        native: |_, r, _| r.slider.thumb_diameter / 2.0,
        get: |t, r, s| match styles::slider(r)(t, s).handle.shape {
            slider::HandleShape::Circle { radius } => Ok(radius),
            slider::HandleShape::Rectangle {
                width: _,
                border_radius: _,
            } => Err("a rectangular handle where the contract claims a circle".to_string()),
        },
    },
];

/// The `field` of every style row, from every function's consts.
///
/// One line per function; the coverage tripwire reads it, and `rows_claiming`
/// counts in it.
#[cfg(feature = "widgets")]
fn style_row_fields() -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    out.extend(BUTTON_ROWS.iter().map(|row| row.field.to_string()));
    out.extend(BUTTON_PRIMARY_ROWS.iter().map(|row| row.field.to_string()));
    out.extend(BUTTON_DANGER_ROWS.iter().map(|row| row.field.to_string()));
    out.extend(BUTTON_SUCCESS_ROWS.iter().map(|row| row.field.to_string()));
    out.extend(BUTTON_WARNING_ROWS.iter().map(|row| row.field.to_string()));
    out.extend(BUTTON_LINK_ROWS.iter().map(|row| row.field.to_string()));
    out.extend(TEXT_INPUT_ROWS.iter().map(|row| row.field.to_string()));
    out.extend(TEXT_EDITOR_ROWS.iter().map(|row| row.field.to_string()));
    out.extend(CHECKBOX_ROWS.iter().map(|row| row.field.to_string()));
    out.extend(RADIO_ROWS.iter().map(|row| row.field.to_string()));
    out.extend(RADIO_SCALAR_ROWS.iter().map(|row| row.field.to_string()));
    out.extend(TOGGLER_ROWS.iter().map(|row| row.field.to_string()));
    out.extend(TOGGLER_SCALAR_ROWS.iter().map(|row| row.field.to_string()));
    out.extend(TOGGLER_RADIUS_ROWS.iter().map(|row| row.field.to_string()));
    out.extend(PICK_LIST_ROWS.iter().map(|row| row.field.to_string()));
    out.extend(MENU_ROWS.iter().map(|row| row.field.to_string()));
    out.extend(SLIDER_ROWS.iter().map(|row| row.field.to_string()));
    out.extend(SLIDER_SCALAR_ROWS.iter().map(|row| row.field.to_string()));
    // One border row claims six leaves, so its names are built rather than
    // written -- by the same walker the tripwire uses.
    out.extend(border_row_fields(BUTTON_BORDER_ROWS));
    out.extend(border_row_fields(BUTTON_PRIMARY_BORDER_ROWS));
    out.extend(border_row_fields(BUTTON_DANGER_BORDER_ROWS));
    out.extend(border_row_fields(BUTTON_SUCCESS_BORDER_ROWS));
    out.extend(border_row_fields(BUTTON_WARNING_BORDER_ROWS));
    out.extend(border_row_fields(TEXT_INPUT_BORDER_ROWS));
    out.extend(border_row_fields(TEXT_EDITOR_BORDER_ROWS));
    out.extend(border_row_fields(CHECKBOX_BORDER_ROWS));
    out.extend(border_row_fields(PICK_LIST_BORDER_ROWS));
    out.extend(border_row_fields(MENU_BORDER_ROWS));
    out
}

/// What a run of the `check_*` helpers actually compared: the name of every
/// row it ran, and how many single-value comparisons those rows made.
///
/// Registering a row in `style_row_fields` and handing it to a `check_*` call
/// are two separate hand-maintained lists, and only the first is read by the
/// coverage tripwire. A row in the first list but not the second is claimed
/// and unasserted, and nothing says so -- the printed check count is simply
/// smaller. So every helper records what it ran here, and `covers` compares
/// the two lists in both directions.
#[cfg(feature = "widgets")]
#[derive(Default)]
struct Checked {
    names: Vec<String>,
    checks: usize,
}

#[cfg(feature = "widgets")]
impl Checked {
    /// Record that a row named `name` was compared, for each of the statuses
    /// it holds for: a row with an empty status list compares nothing and is
    /// not recorded, so it is reported like an unchecked one.
    fn ran<S>(&mut self, name: &str, statuses: &[S]) {
        if !statuses.is_empty() {
            self.names.push(name.to_string());
        }
    }

    /// Require that the rows which ran are exactly the ones `declared` names.
    ///
    /// `what` names the declaration list, so a failure says which of the two
    /// hand-maintained lists to fix.
    fn covers(&self, declared: &[String], what: &str) {
        let (ran, _) = sorted(&self.names);
        let (declared, twice) = sorted(declared);

        let missing: Vec<&&str> = declared.iter().filter(|n| !ran.contains(n)).collect();
        let unexpected: Vec<&&str> = ran.iter().filter(|n| !declared.contains(n)).collect();
        assert!(
            missing.is_empty() && unexpected.is_empty() && twice.is_empty(),
            "what ran and what {what} declares differ, so a declared entry may \
             be asserting nothing:\n  declared but never checked: \
             {missing:?}\n  checked but not declared: {unexpected:?}\n  \
             declared twice: {twice:?}"
        );
    }
}

/// `names` sorted and deduplicated, with the ones that appeared more than once
/// listed separately.
#[cfg(feature = "widgets")]
fn sorted(names: &[String]) -> (Vec<&str>, Vec<&str>) {
    let mut names: Vec<&str> = names.iter().map(String::as_str).collect();
    names.sort_unstable();
    let duplicated: Vec<&str> = names
        .windows(2)
        .filter(|pair| pair[0] == pair[1])
        .map(|pair| pair[0])
        .collect();
    names.dedup();
    (names, duplicated)
}

/// Assert one function's color rows over every combination and status.
#[cfg(feature = "widgets")]
fn check_style_rows<S: Copy + std::fmt::Debug>(
    rows: &[StyleRow<S>],
    combinations: &[Combination],
    failures: &mut Vec<String>,
    ran: &mut Checked,
) {
    for row in rows {
        ran.ran(row.field, row.statuses);
    }
    for c in combinations {
        for row in rows {
            for &status in row.statuses {
                ran.checks += 1;
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
}

/// Assert one function's scalar rows over every combination and status.
#[cfg(feature = "widgets")]
fn check_scalar_rows<S: Copy + std::fmt::Debug>(
    rows: &[ScalarRow<S>],
    combinations: &[Combination],
    failures: &mut Vec<String>,
    ran: &mut Checked,
) {
    for row in rows {
        ran.ran(row.field, row.statuses);
    }
    for c in combinations {
        for row in rows {
            for &status in row.statuses {
                ran.checks += 1;
                let expected = (row.native)(&c.theme, &c.resolved, status);
                match (row.get)(&c.theme, &c.resolved, status) {
                    Ok(actual) if actual == expected => {}
                    Ok(actual) => failures.push(format!(
                        "{}: {} ({status:?}) is {actual}, native gives {expected}",
                        c.label(),
                        row.field
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
    #[cfg(feature = "widgets")]
    (
        "styles::button_primary.shadow",
        "iced default: button::primary(theme, status).shadow -- the model has \
         a shadow color but no offset or blur",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::button_primary.snap",
        "iced default: button::primary(theme, status).snap -- a renderer \
         setting, cfg!(feature = \"crisp\")",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::button_danger.shadow",
        "iced default: button::danger(theme, status).shadow -- the model has \
         a shadow color but no offset or blur",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::button_danger.snap",
        "iced default: button::danger(theme, status).snap -- a renderer \
         setting, cfg!(feature = \"crisp\")",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::button_success.shadow",
        "iced default: button::success(theme, status).shadow -- the model has \
         a shadow color but no offset or blur",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::button_success.snap",
        "iced default: button::success(theme, status).snap -- a renderer \
         setting, cfg!(feature = \"crisp\")",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::button_warning.shadow",
        "iced default: button::warning(theme, status).shadow -- the model has \
         a shadow color but no offset or blur",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::button_warning.snap",
        "iced default: button::warning(theme, status).snap -- a renderer \
         setting, cfg!(feature = \"crisp\")",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::button_link.shadow",
        "iced default: button::text(theme, status).shadow -- the model has a \
         shadow color but no offset or blur",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::button_link.snap",
        "iced default: button::text(theme, status).snap -- a renderer \
         setting, cfg!(feature = \"crisp\")",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::button_link.border.color",
        "iced default: button::text(theme, status).border.color -- LinkTheme \
         carries no border",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::button_link.border.width",
        "iced default: button::text(theme, status).border.width -- LinkTheme \
         carries no border",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::button_link.border.radius.top_left",
        "iced default: button::text(theme, status).border.radius -- LinkTheme \
         carries no border",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::button_link.border.radius.top_right",
        "iced default: button::text(theme, status).border.radius -- LinkTheme \
         carries no border",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::button_link.border.radius.bottom_right",
        "iced default: button::text(theme, status).border.radius -- LinkTheme \
         carries no border",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::button_link.border.radius.bottom_left",
        "iced default: button::text(theme, status).border.radius -- LinkTheme \
         carries no border",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::text_input.icon",
        "iced default: text_input::default(theme, status).icon -- the model \
         carries no input-icon color",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::toggler.background_border_width",
        "iced default: toggler::default(theme, status).background_border_width \
         -- SwitchTheme carries no border",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::toggler.background_border_color",
        "iced default: toggler::default(theme, status).background_border_color \
         -- SwitchTheme carries no border",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::toggler.foreground_border_width",
        "iced default: toggler::default(theme, status).foreground_border_width \
         -- SwitchTheme carries no border",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::toggler.foreground_border_color",
        "iced default: toggler::default(theme, status).foreground_border_color \
         -- SwitchTheme carries no border",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::toggler.text_color",
        "iced default: toggler::default(theme, status).text_color -- the model \
         states no font for a switch, and iced's None inherits the surrounding \
         one",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::pick_list.handle_color",
        "iced default: pick_list::default(theme, status).handle_color -- \
         ComboBoxTheme states the arrow's sizes but not its color",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::menu.shadow",
        "iced default: overlay::menu::default(theme).shadow -- the model has a \
         shadow color but no offset or blur",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::slider.rail.border.color",
        "iced default: slider::default(theme, status).rail.border.color -- \
         SliderTheme carries no border",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::slider.rail.border.width",
        "iced default: slider::default(theme, status).rail.border.width -- \
         SliderTheme carries no border",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::slider.rail.border.radius.top_left",
        "iced default: slider::default(theme, status).rail.border.radius -- \
         SliderTheme carries no border",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::slider.rail.border.radius.top_right",
        "iced default: slider::default(theme, status).rail.border.radius -- \
         SliderTheme carries no border",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::slider.rail.border.radius.bottom_right",
        "iced default: slider::default(theme, status).rail.border.radius -- \
         SliderTheme carries no border",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::slider.rail.border.radius.bottom_left",
        "iced default: slider::default(theme, status).rail.border.radius -- \
         SliderTheme carries no border",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::slider.handle.border_width",
        "iced default: slider::default(theme, status).handle.border_width -- \
         SliderTheme carries no border",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::slider.handle.border_color",
        "iced default: slider::default(theme, status).handle.border_color -- \
         SliderTheme carries no border",
    ),
];

/// Native values iced 0.14 has no receiver for, each with its evidence.
///
/// Not a way out of a row: an entry here says the toolkit cannot carry the
/// value at all, so approximating it would state something untrue. Empty until
/// `styles::scrollbar` lands, whose `scrollbar.min_thumb_length` is the one
/// entry this release knows of (section 3.3).
///
/// What does *not* belong here is native geometry whose only iced receiver is
/// a **builder method** rather than a `Style` field -- `Checkbox::spacing` for
/// `checkbox.label_gap`, `Toggler::size` for `switch.track_height`,
/// `pick_list::Handle::Arrow { size }` for `combo_box.arrow_icon_size`, a
/// menu's padding for `menu.row_height`. iced can carry those perfectly well;
/// they are simply the consumer's layout, set where the widget is built, and
/// `styles::*` returns a `Style`. They are neither unreachable nor a gap in
/// the contract, and the tripwire never sees them because it walks emitted
/// `Style` fields.
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
    /// Whether the foreground is an indicator rather than text -- a check
    /// mark, a dot, a thumb, a handle. WCAG asks 3:1 of one of those, not AA's
    /// 4.5:1, so the below-AA print tags them and the text list stays
    /// readable. A marker on a line, not a second threshold: what is
    /// *asserted* is section 7's no-degradation rule, the same for both kinds.
    indicator: bool,
}

/// The `styles::button` pairs the assertion covers.
#[cfg(feature = "widgets")]
const BUTTON_PAIRS: &[StylePair<button::Status>] = &[StylePair {
    what: "button label",
    indicator: false,
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

// The four class buttons' pairs -- `BUTTON_PRIMARY_PAIRS`,
// `BUTTON_DANGER_PAIRS`, `BUTTON_SUCCESS_PAIRS`, `BUTTON_WARNING_PAIRS` --
// are declared by `class_button_contract!` above, beside their rows.

/// The `styles::button_link` pairs the assertion covers.
#[cfg(feature = "widgets")]
const BUTTON_LINK_PAIRS: &[StylePair<button::Status>] = &[StylePair {
    what: "link button label",
    indicator: false,
    statuses: BUTTON_STATUSES,
    emitted: |t, r, s| {
        let style = styles::button_link(r)(t, s);
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
            native_button_link_label(r, s),
            native_button_link_fill(r, s),
            to_color(r.defaults.background_color),
        )
    },
}];

/// The `styles::text_input` pairs the assertion covers: both texts the
/// function paints, on the fill it paints them on.
#[cfg(feature = "widgets")]
const TEXT_INPUT_PAIRS: &[StylePair<text_input::Status>] = &[
    StylePair {
        what: "input value",
        indicator: false,
        statuses: TEXT_INPUT_STATUSES,
        emitted: |t, r, s| {
            let style = styles::text_input(r)(t, s);
            // A field is painted on the window, so that is what a translucent
            // fill shows through to.
            fill(style.background).map(|background| {
                (
                    style.value,
                    background,
                    to_color(r.defaults.background_color),
                )
            })
        },
        native: |r, s| {
            (
                native_text_input_value(r, s),
                native_text_input_fill(r, s),
                to_color(r.defaults.background_color),
            )
        },
    },
    StylePair {
        what: "input placeholder",
        indicator: false,
        statuses: TEXT_INPUT_STATUSES,
        emitted: |t, r, s| {
            let style = styles::text_input(r)(t, s);
            fill(style.background).map(|background| {
                (
                    style.placeholder,
                    background,
                    to_color(r.defaults.background_color),
                )
            })
        },
        native: |r, s| {
            (
                to_color(r.input.placeholder_color),
                native_text_input_fill(r, s),
                to_color(r.defaults.background_color),
            )
        },
    },
];

/// The `styles::text_editor` pairs the assertion covers.
#[cfg(feature = "widgets")]
const TEXT_EDITOR_PAIRS: &[StylePair<text_editor::Status>] = &[
    StylePair {
        what: "editor value",
        indicator: false,
        statuses: TEXT_EDITOR_STATUSES,
        emitted: |t, r, s| {
            let style = styles::text_editor(r)(t, s);
            fill(style.background).map(|background| {
                (
                    style.value,
                    background,
                    to_color(r.defaults.background_color),
                )
            })
        },
        native: |r, s| {
            (
                native_text_editor_value(r, s),
                native_text_editor_fill(r, s),
                to_color(r.defaults.background_color),
            )
        },
    },
    StylePair {
        what: "editor placeholder",
        indicator: false,
        statuses: TEXT_EDITOR_STATUSES,
        emitted: |t, r, s| {
            let style = styles::text_editor(r)(t, s);
            fill(style.background).map(|background| {
                (
                    style.placeholder,
                    background,
                    to_color(r.defaults.background_color),
                )
            })
        },
        native: |r, s| {
            (
                to_color(r.input.placeholder_color),
                native_text_editor_fill(r, s),
                to_color(r.defaults.background_color),
            )
        },
    },
];

/// The check mark on the box it is painted in, and the dot on its circle.
///
/// Both are indicators rather than text: WCAG asks 3:1 of one, not AA's 4.5:1,
/// so their below-AA lines are informational. What section 7 asserts of them
/// is the same as everywhere else -- that our ratio is not worse than the
/// platform's own pair.
///
/// The *label* pair is a text pair, and the only one of these functions where
/// the fill is not the function's own: a checkbox label sits beside the box,
/// on the window. Both colors are still native and both are still pinned --
/// the foreground by the row above, the window by the `palette.background`
/// row -- so section 7's rule has two native colors to compare, which is what
/// it asks for. It matters most in the disabled status, where the platform
/// dims the label on purpose.
#[cfg(feature = "widgets")]
const CHECKBOX_PAIRS: &[StylePair<checkbox::Status>] = &[
    StylePair {
        what: "checkbox mark",
        indicator: true,
        statuses: CHECKBOX_CHECKED_STATUSES,
        emitted: |t, r, s| {
            let style = styles::checkbox(r)(t, s);
            // The box is painted on the window, so that is what a translucent
            // fill -- windows-11 states one -- shows through to.
            fill(style.background).map(|background| {
                (
                    style.icon_color,
                    background,
                    to_color(r.defaults.background_color),
                )
            })
        },
        native: |r, s| {
            (
                to_color(r.checkbox.indicator_color),
                native_checkbox_fill(r, s),
                to_color(r.defaults.background_color),
            )
        },
    },
    StylePair {
        what: "checkbox label",
        indicator: false,
        statuses: CHECKBOX_STATUSES,
        // The fill is read from the *theme* the connector built, not from the
        // resolved field it was built from: otherwise both sides of the pair
        // would be the same expression and the fill half could not fail.
        // `palette.background`'s own row is what ties the two together.
        emitted: |t, r, s| {
            let window = t.extended_palette().background.base.color;
            stated(styles::checkbox(r)(t, s).text_color).map(|label| (label, window, window))
        },
        native: |r, s| {
            let window = to_color(r.defaults.background_color);
            (native_checkbox_label(r, s), window, window)
        },
    },
];

#[cfg(feature = "widgets")]
const RADIO_PAIRS: &[StylePair<radio::Status>] = &[StylePair {
    what: "radio dot",
    indicator: true,
    statuses: RADIO_SELECTED_STATUSES,
    emitted: |t, r, s| {
        let style = styles::radio(r)(t, s);
        fill(style.background).map(|background| {
            (
                style.dot_color,
                background,
                to_color(r.defaults.background_color),
            )
        })
    },
    native: |r, s| {
        (
            to_color(r.checkbox.indicator_color),
            native_radio_fill(r, s),
            to_color(r.defaults.background_color),
        )
    },
}];

/// The thumb on the track it slides along -- an indicator pair again, and the
/// only one `toggler::Style` offers: its label is painted on the window.
#[cfg(feature = "widgets")]
const TOGGLER_PAIRS: &[StylePair<toggler::Status>] = &[StylePair {
    what: "toggler thumb",
    indicator: true,
    statuses: TOGGLER_STATUSES,
    emitted: |t, r, s| {
        let style = styles::toggler(r)(t, s);
        let thumb = fill(style.foreground)?;
        fill(style.background).map(|track| (thumb, track, to_color(r.defaults.background_color)))
    },
    native: |r, s| {
        (
            native_toggler_thumb(r, s),
            native_toggler_track(r, s),
            to_color(r.defaults.background_color),
        )
    },
}];

/// Both texts a drop-down field paints, on the fill it paints them on.
#[cfg(feature = "widgets")]
const PICK_LIST_PAIRS: &[StylePair<pick_list::Status>] = &[
    StylePair {
        what: "pick list label",
        indicator: false,
        statuses: PICK_LIST_STATUSES,
        emitted: |t, r, s| {
            let style = styles::pick_list(r)(t, s);
            fill(style.background).map(|background| {
                (
                    style.text_color,
                    background,
                    to_color(r.defaults.background_color),
                )
            })
        },
        native: |r, s| {
            (
                to_color(r.combo_box.font.color),
                native_pick_list_fill(r, s),
                to_color(r.defaults.background_color),
            )
        },
    },
    StylePair {
        what: "pick list placeholder",
        indicator: false,
        statuses: PICK_LIST_STATUSES,
        emitted: |t, r, s| {
            let style = styles::pick_list(r)(t, s);
            fill(style.background).map(|background| {
                (
                    style.placeholder_color,
                    background,
                    to_color(r.defaults.background_color),
                )
            })
        },
        native: |r, s| {
            (
                to_color(r.input.placeholder_color),
                native_pick_list_fill(r, s),
                to_color(r.defaults.background_color),
            )
        },
    },
];

/// A menu's label on its panel, and a selected row's label on the highlight.
/// The highlight is painted on the panel, so that -- not the window -- is the
/// surface a translucent one shows through to.
#[cfg(feature = "widgets")]
const MENU_PAIRS: &[StylePair<()>] = &[
    StylePair {
        what: "menu label",
        indicator: false,
        statuses: MENU_STATUSES,
        emitted: |t, r, ()| {
            let style = styles::menu(r)(t);
            fill(style.background).map(|background| {
                (
                    style.text_color,
                    background,
                    to_color(r.defaults.background_color),
                )
            })
        },
        native: |r, ()| {
            (
                to_color(r.menu.font.color),
                to_color(r.menu.background_color),
                to_color(r.defaults.background_color),
            )
        },
    },
    StylePair {
        what: "menu selected label",
        indicator: false,
        statuses: MENU_STATUSES,
        emitted: |t, r, ()| {
            let style = styles::menu(r)(t);
            fill(style.selected_background).map(|highlight| {
                (
                    style.selected_text_color,
                    highlight,
                    to_color(r.menu.background_color),
                )
            })
        },
        native: |r, ()| {
            (
                to_color(r.menu.hover_text_color),
                to_color(r.menu.hover_background),
                to_color(r.menu.background_color),
            )
        },
    },
];

/// The handle on each half of the rail it slides along -- the handle sits over
/// the boundary between them, and both halves are the function's own. An
/// indicator pair, like the checkbox mark, and only for the statuses whose
/// handle *fill* the model states: a dragged handle's fill is iced's.
#[cfg(feature = "widgets")]
const SLIDER_PAIRS: &[StylePair<slider::Status>] = &[
    StylePair {
        what: "slider handle on the filled rail",
        indicator: true,
        statuses: SLIDER_NATIVE_FILL_STATUSES,
        emitted: |t, r, s| {
            let style = styles::slider(r)(t, s);
            let handle = fill(style.handle.background)?;
            fill(style.rail.backgrounds.0)
                .map(|rail| (handle, rail, to_color(r.defaults.background_color)))
        },
        native: |r, s| {
            (
                native_slider_handle(r, s),
                to_color(r.slider.fill_color),
                to_color(r.defaults.background_color),
            )
        },
    },
    StylePair {
        what: "slider handle on the remaining rail",
        indicator: true,
        statuses: SLIDER_NATIVE_FILL_STATUSES,
        emitted: |t, r, s| {
            let style = styles::slider(r)(t, s);
            let handle = fill(style.handle.background)?;
            fill(style.rail.backgrounds.1)
                .map(|rail| (handle, rail, to_color(r.defaults.background_color)))
        },
        native: |r, s| {
            (
                native_slider_handle(r, s),
                to_color(r.slider.track_color),
                to_color(r.defaults.background_color),
            )
        },
    },
];

/// The `what` of every contrast pair this file declares.
///
/// The coverage tripwire has nothing to say about pairs -- whether a
/// foreground and its fill are both ours is a judgment per function (section
/// 7) -- so there is no list that forces a pair to exist. What there is to
/// enforce is that a pair once written is actually run: one line here and one
/// `check_style_pairs` call are two hand-maintained lists, and
/// `style_contrast_never_degrades_the_native_pair` requires them to agree.
#[cfg(feature = "widgets")]
fn style_pair_names() -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    out.extend(BUTTON_PAIRS.iter().map(|pair| pair.what.to_string()));
    out.extend(BUTTON_PRIMARY_PAIRS.iter().map(|p| p.what.to_string()));
    out.extend(BUTTON_DANGER_PAIRS.iter().map(|p| p.what.to_string()));
    out.extend(BUTTON_SUCCESS_PAIRS.iter().map(|p| p.what.to_string()));
    out.extend(BUTTON_WARNING_PAIRS.iter().map(|p| p.what.to_string()));
    out.extend(BUTTON_LINK_PAIRS.iter().map(|p| p.what.to_string()));
    out.extend(TEXT_INPUT_PAIRS.iter().map(|p| p.what.to_string()));
    out.extend(TEXT_EDITOR_PAIRS.iter().map(|p| p.what.to_string()));
    out.extend(CHECKBOX_PAIRS.iter().map(|p| p.what.to_string()));
    out.extend(RADIO_PAIRS.iter().map(|p| p.what.to_string()));
    out.extend(TOGGLER_PAIRS.iter().map(|p| p.what.to_string()));
    out.extend(PICK_LIST_PAIRS.iter().map(|p| p.what.to_string()));
    out.extend(MENU_PAIRS.iter().map(|p| p.what.to_string()));
    out.extend(SLIDER_PAIRS.iter().map(|p| p.what.to_string()));
    out
}

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
    ran: &mut Checked,
) {
    for pair in pairs {
        ran.ran(pair.what, pair.statuses);
    }
    for c in combinations {
        for pair in pairs {
            for &status in pair.statuses {
                ran.checks += 1;
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
                                "{}: {} ({status:?}) {emitted:.2}{}",
                                c.label(),
                                pair.what,
                                if pair.indicator { "  [indicator]" } else { "" }
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
///
/// The four class buttons' enumerators -- `button_primary_style_fields` and
/// friends -- are declared by `class_button_contract!`, which walks the same
/// `button::Style` under its own prefix.
#[cfg(feature = "widgets")]
fn button_style_fields(style: &button::Style) -> Vec<String> {
    let mut out = Vec::new();
    leaves_under!(out, style, "styles::button", button::Style {
        background, text_color, shadow, snap, @nested border
    });
    out.extend(border_fields(border, "styles::button.border"));
    out
}

/// Every field of the `button::Style` `styles::button_link` emits.
#[cfg(feature = "widgets")]
fn button_link_style_fields(style: &button::Style) -> Vec<String> {
    let mut out = Vec::new();
    leaves_under!(out, style, "styles::button_link", button::Style {
        background, text_color, shadow, snap, @nested border
    });
    out.extend(border_fields(border, "styles::button_link.border"));
    out
}

/// Every field of the `text_input::Style` the connector emits.
#[cfg(feature = "widgets")]
fn text_input_style_fields(style: &text_input::Style) -> Vec<String> {
    let mut out = Vec::new();
    leaves_under!(out, style, "styles::text_input", text_input::Style {
        background, icon, placeholder, value, selection, @nested border
    });
    out.extend(border_fields(border, "styles::text_input.border"));
    out
}

/// Every field of the `text_editor::Style` the connector emits.
#[cfg(feature = "widgets")]
fn text_editor_style_fields(style: &text_editor::Style) -> Vec<String> {
    let mut out = Vec::new();
    leaves_under!(out, style, "styles::text_editor", text_editor::Style {
        background, placeholder, value, selection, @nested border
    });
    out.extend(border_fields(border, "styles::text_editor.border"));
    out
}

/// Every field of the `checkbox::Style` the connector emits.
#[cfg(feature = "widgets")]
fn checkbox_style_fields(style: &checkbox::Style) -> Vec<String> {
    let mut out = Vec::new();
    leaves_under!(out, style, "styles::checkbox", checkbox::Style {
        background, icon_color, text_color, @nested border
    });
    out.extend(border_fields(border, "styles::checkbox.border"));
    out
}

/// Every field of the `radio::Style` the connector emits. Its border is two
/// bare fields rather than an iced `Border`, so there is nothing to walk.
#[cfg(feature = "widgets")]
fn radio_style_fields(style: &radio::Style) -> Vec<String> {
    let mut out = Vec::new();
    leaves_under!(
        out,
        style,
        "styles::radio",
        radio::Style {
            background,
            dot_color,
            border_width,
            border_color,
            text_color
        }
    );
    out
}

/// Every field of the `toggler::Style` the connector emits.
#[cfg(feature = "widgets")]
fn toggler_style_fields(style: &toggler::Style) -> Vec<String> {
    let mut out = Vec::new();
    leaves_under!(
        out,
        style,
        "styles::toggler",
        toggler::Style {
            background,
            background_border_width,
            background_border_color,
            foreground,
            foreground_border_width,
            foreground_border_color,
            text_color,
            border_radius,
            padding_ratio
        }
    );
    out
}

/// Every field of the `pick_list::Style` the connector emits.
#[cfg(feature = "widgets")]
fn pick_list_style_fields(style: &pick_list::Style) -> Vec<String> {
    let mut out = Vec::new();
    leaves_under!(out, style, "styles::pick_list", pick_list::Style {
        text_color, placeholder_color, handle_color, background, @nested border
    });
    out.extend(border_fields(border, "styles::pick_list.border"));
    out
}

/// Every field of the `menu::Style` the connector emits.
#[cfg(feature = "widgets")]
fn menu_style_fields(style: &menu::Style) -> Vec<String> {
    let mut out = Vec::new();
    leaves_under!(out, style, "styles::menu", menu::Style {
        background, text_color, selected_text_color, selected_background, shadow, @nested border
    });
    out.extend(border_fields(border, "styles::menu.border"));
    out
}

/// Every field of the `slider::Style` the connector emits, through its two
/// nested structs.
#[cfg(feature = "widgets")]
fn slider_style_fields(style: &slider::Style) -> Vec<String> {
    let mut out = Vec::new();
    leaves_under!(out, style, "styles::slider", slider::Style { @nested rail, handle });
    leaves_under!(out, rail, "styles::slider.rail", slider::Rail {
        width, @nested backgrounds, border
    });
    // The rail's two backgrounds are a tuple rather than a struct, so they are
    // destructured here instead: a third one would fail to compile with the
    // rest of the walk rather than go unnamed.
    let (_filled, _remaining) = backgrounds;
    out.push("styles::slider.rail.backgrounds.0".to_string());
    out.push("styles::slider.rail.backgrounds.1".to_string());
    out.extend(border_fields(border, "styles::slider.rail.border"));
    leaves_under!(
        out,
        handle,
        "styles::slider.handle",
        slider::Handle {
            shape,
            background,
            border_width,
            border_color
        }
    );
    out
}

/// Every field the tripwire walks: the palette inputs, the extended slots the
/// connector writes, and each `styles::*` output's fields.
#[cfg_attr(not(feature = "widgets"), allow(unused_variables))]
fn named_fields(theme: &Theme, resolved: &ResolvedTheme) -> Vec<String> {
    // The palette enumerators name `&'static str`s built by `leaves!`; the
    // style ones build theirs under a prefix, so both arrive as `String`.
    let mut out: Vec<String> = palette_fields(&theme.palette())
        .into_iter()
        .map(String::from)
        .collect();
    out.extend(
        written_extended_fields(theme.extended_palette())
            .into_iter()
            .map(String::from),
    );
    #[cfg(feature = "widgets")]
    {
        out.extend(button_style_fields(&styles::button(resolved)(
            theme,
            button::Status::Active,
        )));
        out.extend(button_primary_style_fields(&styles::button_primary(
            resolved,
        )(
            theme,
            button::Status::Active,
        )));
        out.extend(button_danger_style_fields(
            &styles::button_danger(resolved)(theme, button::Status::Active),
        ));
        out.extend(button_success_style_fields(&styles::button_success(
            resolved,
        )(
            theme,
            button::Status::Active,
        )));
        out.extend(button_warning_style_fields(&styles::button_warning(
            resolved,
        )(
            theme,
            button::Status::Active,
        )));
        out.extend(button_link_style_fields(&styles::button_link(resolved)(
            theme,
            button::Status::Active,
        )));
        out.extend(text_input_style_fields(&styles::text_input(resolved)(
            theme,
            text_input::Status::Active,
        )));
        out.extend(text_editor_style_fields(&styles::text_editor(resolved)(
            theme,
            text_editor::Status::Active,
        )));
        out.extend(checkbox_style_fields(&styles::checkbox(resolved)(
            theme,
            checkbox::Status::Active { is_checked: true },
        )));
        out.extend(radio_style_fields(&styles::radio(resolved)(
            theme,
            radio::Status::Active { is_selected: true },
        )));
        out.extend(toggler_style_fields(&styles::toggler(resolved)(
            theme,
            toggler::Status::Active { is_toggled: true },
        )));
        out.extend(pick_list_style_fields(&styles::pick_list(resolved)(
            theme,
            pick_list::Status::Active,
        )));
        out.extend(menu_style_fields(&styles::menu(resolved)(theme)));
        out.extend(slider_style_fields(&styles::slider(resolved)(
            theme,
            slider::Status::Active,
        )));
    }
    out
}

/// How many rows claim `field`.
#[cfg_attr(not(feature = "widgets"), allow(unused_mut))]
fn rows_claiming(field: &str) -> usize {
    let mut count = ROWS.iter().filter(|row| row.slot == field).count();
    #[cfg(feature = "widgets")]
    {
        count += style_row_fields()
            .iter()
            .filter(|f| f.as_str() == field)
            .count();
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
            let rows = rows_claiming(&field);
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

/// Every status list names each of its widget's statuses exactly once.
///
/// Generic rows close the fall-through hole a `&str` status had, but not this
/// one: a status simply left out of a list makes the rows assert less without
/// ever failing. So each list is checked against every value of its own enum
/// here, and the match is exhaustive with no catch-all, so a variant added
/// upstream fails to compile in this function first.
///
/// **The hand-written `all` list is the root of trust, and it is maintained by
/// hand.** The match proves that `all` names no variant the enum does not have
/// and that a new variant is noticed; it cannot prove the converse. A variant
/// deleted from `all` *and* from the widget's list leaves both sides agreeing
/// about a smaller enum, and nothing here fails. For a data-carrying status
/// (`text_input::Status::Focused { is_hovered }`) the list holds every *value*,
/// and no match can prove that either -- `Focused { is_hovered: _ }` is one arm
/// whether the list names one of its values or both.
#[cfg(feature = "widgets")]
#[test]
fn every_status_list_names_each_status_once() {
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

    // A class button splits the same enum in two: the statuses it decides
    // itself and the two it takes whole from iced's class. Neither list alone
    // may name every status, so the pair is checked as a partition instead.
    for status in all {
        let native = CLASS_BUTTON_NATIVE_STATUSES.contains(&status);
        let from_iced = CLASS_BUTTON_ICED_STATUSES.contains(&status);
        assert!(
            native != from_iced,
            "{status:?} is in {} of the two class-button lists; each status \
             belongs to exactly one",
            if native { "both" } else { "neither" }
        );
    }
    assert_eq!(
        CLASS_BUTTON_NATIVE_STATUSES.len() + CLASS_BUTTON_ICED_STATUSES.len(),
        all.len(),
        "the two class-button lists must partition the statuses, naming each \
         exactly once between them"
    );

    let all = [
        text_input::Status::Active,
        text_input::Status::Hovered,
        text_input::Status::Focused { is_hovered: false },
        text_input::Status::Focused { is_hovered: true },
        text_input::Status::Disabled,
    ];
    for status in all {
        match status {
            text_input::Status::Active
            | text_input::Status::Hovered
            | text_input::Status::Focused { is_hovered: _ }
            | text_input::Status::Disabled => {}
        }
        assert!(
            TEXT_INPUT_STATUSES.contains(&status),
            "TEXT_INPUT_STATUSES does not list {status:?}, so no text input \
             row covers it"
        );
    }
    assert_eq!(
        TEXT_INPUT_STATUSES.len(),
        all.len(),
        "TEXT_INPUT_STATUSES must name each status exactly once"
    );

    let all = [
        text_editor::Status::Active,
        text_editor::Status::Hovered,
        text_editor::Status::Focused { is_hovered: false },
        text_editor::Status::Focused { is_hovered: true },
        text_editor::Status::Disabled,
    ];
    for status in all {
        match status {
            text_editor::Status::Active
            | text_editor::Status::Hovered
            | text_editor::Status::Focused { is_hovered: _ }
            | text_editor::Status::Disabled => {}
        }
        assert!(
            TEXT_EDITOR_STATUSES.contains(&status),
            "TEXT_EDITOR_STATUSES does not list {status:?}, so no text editor \
             row covers it"
        );
    }
    assert_eq!(
        TEXT_EDITOR_STATUSES.len(),
        all.len(),
        "TEXT_EDITOR_STATUSES must name each status exactly once"
    );

    let all = [
        checkbox::Status::Active { is_checked: false },
        checkbox::Status::Active { is_checked: true },
        checkbox::Status::Hovered { is_checked: false },
        checkbox::Status::Hovered { is_checked: true },
        checkbox::Status::Disabled { is_checked: false },
        checkbox::Status::Disabled { is_checked: true },
    ];
    for status in all {
        match status {
            checkbox::Status::Active { is_checked: _ }
            | checkbox::Status::Hovered { is_checked: _ }
            | checkbox::Status::Disabled { is_checked: _ } => {}
        }
        assert!(
            CHECKBOX_STATUSES.contains(&status),
            "CHECKBOX_STATUSES does not list {status:?}, so no checkbox row \
             covers it"
        );
    }
    assert_eq!(
        CHECKBOX_STATUSES.len(),
        all.len(),
        "CHECKBOX_STATUSES must name each status exactly once"
    );

    // The mark's contrast pair covers only the checked half, because an
    // unchecked box paints no mark. The two halves are required to partition
    // the enum, so that half cannot shrink unnoticed.
    for status in all {
        let checked = CHECKBOX_CHECKED_STATUSES.contains(&status);
        let unchecked = CHECKBOX_UNCHECKED_STATUSES.contains(&status);
        assert!(
            checked != unchecked,
            "{status:?} is in {} of the two checkbox lists; each status \
             belongs to exactly one",
            if checked { "both" } else { "neither" }
        );
    }
    assert_eq!(
        CHECKBOX_CHECKED_STATUSES.len() + CHECKBOX_UNCHECKED_STATUSES.len(),
        all.len(),
        "the two checkbox lists must partition the statuses, naming each \
         exactly once between them"
    );

    let all = [
        radio::Status::Active { is_selected: false },
        radio::Status::Active { is_selected: true },
        radio::Status::Hovered { is_selected: false },
        radio::Status::Hovered { is_selected: true },
    ];
    for status in all {
        match status {
            radio::Status::Active { is_selected: _ }
            | radio::Status::Hovered { is_selected: _ } => {}
        }
        assert!(
            RADIO_STATUSES.contains(&status),
            "RADIO_STATUSES does not list {status:?}, so no radio row covers it"
        );
    }
    assert_eq!(
        RADIO_STATUSES.len(),
        all.len(),
        "RADIO_STATUSES must name each status exactly once"
    );

    for status in all {
        let selected = RADIO_SELECTED_STATUSES.contains(&status);
        let unselected = RADIO_UNSELECTED_STATUSES.contains(&status);
        assert!(
            selected != unselected,
            "{status:?} is in {} of the two radio lists; each status belongs \
             to exactly one",
            if selected { "both" } else { "neither" }
        );
    }
    assert_eq!(
        RADIO_SELECTED_STATUSES.len() + RADIO_UNSELECTED_STATUSES.len(),
        all.len(),
        "the two radio lists must partition the statuses, naming each exactly \
         once between them"
    );

    let all = [
        toggler::Status::Active { is_toggled: false },
        toggler::Status::Active { is_toggled: true },
        toggler::Status::Hovered { is_toggled: false },
        toggler::Status::Hovered { is_toggled: true },
        toggler::Status::Disabled { is_toggled: false },
        toggler::Status::Disabled { is_toggled: true },
    ];
    for status in all {
        match status {
            toggler::Status::Active { is_toggled: _ }
            | toggler::Status::Hovered { is_toggled: _ }
            | toggler::Status::Disabled { is_toggled: _ } => {}
        }
        assert!(
            TOGGLER_STATUSES.contains(&status),
            "TOGGLER_STATUSES does not list {status:?}, so no toggler row \
             covers it"
        );
    }
    assert_eq!(
        TOGGLER_STATUSES.len(),
        all.len(),
        "TOGGLER_STATUSES must name each status exactly once"
    );

    let all = [
        pick_list::Status::Active,
        pick_list::Status::Hovered,
        pick_list::Status::Opened { is_hovered: false },
        pick_list::Status::Opened { is_hovered: true },
    ];
    for status in all {
        match status {
            pick_list::Status::Active
            | pick_list::Status::Hovered
            | pick_list::Status::Opened { is_hovered: _ } => {}
        }
        assert!(
            PICK_LIST_STATUSES.contains(&status),
            "PICK_LIST_STATUSES does not list {status:?}, so no pick list row \
             covers it"
        );
    }
    assert_eq!(
        PICK_LIST_STATUSES.len(),
        all.len(),
        "PICK_LIST_STATUSES must name each status exactly once"
    );

    // A menu has no status of its own; its rows hold the unit value once.
    assert_eq!(
        MENU_STATUSES,
        &[()],
        "a menu takes no status, so its rows hold the unit value exactly once"
    );

    let all = [
        slider::Status::Active,
        slider::Status::Hovered,
        slider::Status::Dragged,
    ];
    for status in all {
        match status {
            slider::Status::Active | slider::Status::Hovered | slider::Status::Dragged => {}
        }
        assert!(
            SLIDER_STATUSES.contains(&status),
            "SLIDER_STATUSES does not list {status:?}, so no slider row covers it"
        );
    }
    assert_eq!(
        SLIDER_STATUSES.len(),
        all.len(),
        "SLIDER_STATUSES must name each status exactly once"
    );

    // The handle's *fill* splits the same enum in two, as a class button's
    // fill does: the statuses whose thumb color the model states, and the
    // dragged one it does not. The handle's shape and the rail are native in
    // all three.
    for status in all {
        let native = SLIDER_NATIVE_FILL_STATUSES.contains(&status);
        let from_iced = SLIDER_ICED_FILL_STATUSES.contains(&status);
        assert!(
            native != from_iced,
            "{status:?} is in {} of the two slider handle-fill lists; each \
             status belongs to exactly one",
            if native { "both" } else { "neither" }
        );
    }
    assert_eq!(
        SLIDER_NATIVE_FILL_STATUSES.len() + SLIDER_ICED_FILL_STATUSES.len(),
        all.len(),
        "the two slider handle-fill lists must partition the statuses, naming \
         each exactly once between them"
    );
}

#[cfg(feature = "widgets")]
#[test]
fn a_class_button_takes_its_hovered_and_pressed_states_from_iced() -> native_theme::Result<()> {
    // The model states an accent or a status color and its label and no
    // variant of either for a hovered or a pressed button, so those two states
    // are iced's own class, read at run time (spec section 3.3). That is a
    // claim about the emitted value, so it is asserted rather than left to the
    // rows, which cover only `CLASS_BUTTON_NATIVE_STATUSES`.
    let combinations = combinations()?;
    let mut failures = Vec::new();
    let mut checks = 0;

    for c in &combinations {
        for (what, ours, theirs) in [
            (
                "styles::button_primary",
                &styles::button_primary(&c.resolved)
                    as &dyn Fn(&Theme, button::Status) -> button::Style,
                button::primary as fn(&Theme, button::Status) -> button::Style,
            ),
            (
                "styles::button_danger",
                &styles::button_danger(&c.resolved),
                button::danger,
            ),
            (
                "styles::button_success",
                &styles::button_success(&c.resolved),
                button::success,
            ),
            (
                "styles::button_warning",
                &styles::button_warning(&c.resolved),
                button::warning,
            ),
        ] {
            for &status in CLASS_BUTTON_ICED_STATUSES {
                let emitted = ours(&c.theme, status);
                let iced = theirs(&c.theme, status);
                checks += 1;
                if emitted.background != iced.background {
                    failures.push(format!(
                        "{}: {what}.background ({status:?}) is {:?}, iced's own class gives {:?}",
                        c.label(),
                        emitted.background,
                        iced.background
                    ));
                }
                checks += 1;
                if emitted.text_color != iced.text_color {
                    failures.push(format!(
                        "{}: {what}.text_color ({status:?}) is {}, iced's own class gives {}",
                        c.label(),
                        show(emitted.text_color),
                        show(iced.text_color)
                    ));
                }
            }
        }
    }

    println!("class buttons vs iced's own classes: {checks} field checks");
    assert!(
        failures.is_empty(),
        "{} of {checks} class-button states differ from iced's own class:\n{}",
        failures.len(),
        failures.join("\n")
    );
    Ok(())
}

#[cfg(feature = "widgets")]
#[test]
fn every_preset_states_a_switch_thumb_that_fits_its_track() -> native_theme::Result<()> {
    // `styles::toggler` computes `padding_ratio` from `switch.track_height`
    // and `.thumb_diameter` under a guard, and falls back to iced's own ratio
    // where the two cannot state an inset. Today the guard holds everywhere,
    // so the fallback is dead code that nothing would notice going live --
    // this counts the combinations that take the native branch and requires
    // all of them to, so a preset that starts needing the fallback says so.
    let combinations = combinations()?;
    let mut native = Vec::new();
    let mut fell_back = Vec::new();

    for c in &combinations {
        let s = &c.resolved.switch;
        if switch_geometry_is_usable(&c.resolved) {
            native.push(format!(
                "{}: track {} thumb {} -> {:.4}",
                c.label(),
                s.track_height,
                s.thumb_diameter,
                (s.track_height - s.thumb_diameter) / (2.0 * s.track_height)
            ));
        } else {
            fell_back.push(format!(
                "{}: track {} thumb {}",
                c.label(),
                s.track_height,
                s.thumb_diameter
            ));
        }
    }

    println!(
        "switch thumb insets, {} of {} from native geometry:",
        native.len(),
        combinations.len()
    );
    for line in &native {
        println!("{line}");
    }
    assert!(
        fell_back.is_empty(),
        "{} of {} combinations cannot state a thumb inset, so \
         styles::toggler emits iced's ratio there instead of the platform's:\n{}",
        fell_back.len(),
        combinations.len(),
        fell_back.join("\n")
    );
    Ok(())
}

#[cfg(feature = "widgets")]
#[test]
fn a_switch_with_no_track_height_takes_iceds_padding_ratio() -> native_theme::Result<()> {
    // The other side of that guard, which no bundled preset reaches: with no
    // track height the native expression would divide by zero, so the field
    // follows the no-source rule and is iced's own, read at run time.
    let mut resolved = native_theme::theme::Theme::preset("windows-11")?
        .into_variant(ColorMode::Light)?
        .into_resolved(&native_theme::ResolutionContext::for_tests())?;
    assert!(
        switch_geometry_is_usable(&resolved),
        "this preset no longer states usable switch geometry, so breaking it \
         proves nothing"
    );
    resolved.switch.track_height = 0.0;
    assert!(
        !switch_geometry_is_usable(&resolved),
        "a track with no height must not count as usable geometry"
    );
    let theme = crate::to_theme(&resolved, "windows-11");
    let style = styles::toggler(&resolved);

    for &status in TOGGLER_STATUSES {
        assert_eq!(
            style(&theme, status).padding_ratio,
            toggler::default(&theme, status).padding_ratio,
            "a switch whose geometry cannot state an inset takes iced's own \
             ratio ({status:?})"
        );
    }
    Ok(())
}

#[cfg(feature = "widgets")]
#[test]
fn a_dragged_slider_handle_takes_its_fill_from_iced() -> native_theme::Result<()> {
    // `SliderTheme` states a thumb color and a hovered thumb color and no
    // variant for a handle being dragged, so in that status that one *field*
    // follows spec section 3.2's no-source rule and is iced's own, read at run
    // time. Everything else about a dragged slider is still the platform's --
    // the handle's shape and both halves of the rail, which the rows cover in
    // all three statuses. The fill row covers only
    // `SLIDER_NATIVE_FILL_STATUSES`, so the claim about the third is asserted
    // here.
    let combinations = combinations()?;
    let mut failures = Vec::new();
    let mut checks = 0;

    for c in &combinations {
        for &status in SLIDER_ICED_FILL_STATUSES {
            let emitted = styles::slider(&c.resolved)(&c.theme, status);
            let iced = slider::default(&c.theme, status);
            checks += 1;
            if emitted.handle.background != iced.handle.background {
                failures.push(format!(
                    "{}: styles::slider.handle.background ({status:?}) is {:?}, \
                     iced's own default gives {:?}",
                    c.label(),
                    emitted.handle.background,
                    iced.handle.background
                ));
            }
        }
    }

    println!("dragged slider handle vs iced's own default: {checks} field checks");
    assert!(
        failures.is_empty(),
        "{} of {checks} dragged handles differ from iced's own default:\n{}",
        failures.len(),
        failures.join("\n")
    );
    Ok(())
}

#[cfg(feature = "widgets")]
#[test]
fn the_primary_pair_is_the_accent_pair() -> native_theme::Result<()> {
    // `styles::button_primary` paints `button.primary_background` at rest and
    // hands its hovered and pressed states to `button::primary`, which
    // strengthens `palette.primary` -- that is, `defaults.accent_color`. The
    // two are the same color in every bundled preset, which is what makes the
    // handover sound: iced's derivation starts from the fill the function
    // itself painted. This pins that premise, because nothing else would
    // notice if a preset broke it.
    let combinations = combinations()?;
    let mut failures = Vec::new();
    let mut checks = 0;

    for c in &combinations {
        let r = &c.resolved;
        for (what, ours, accent) in [
            (
                "button.primary_background vs defaults.accent_color",
                r.button.primary_background,
                r.defaults.accent_color,
            ),
            (
                "button.primary_text_color vs defaults.accent_text_color",
                r.button.primary_text_color,
                r.defaults.accent_text_color,
            ),
        ] {
            checks += 1;
            if ours != accent {
                failures.push(format!(
                    "{}: {what} -- {} and {}, so a hovered primary button would \
                     belong to a different color than its idle fill: iced \
                     strengthens the palette slot, which carries the accent",
                    c.label(),
                    show(to_color(ours)),
                    show(to_color(accent))
                ));
            }
        }
    }

    println!("primary pair vs accent pair: {checks} field checks");
    assert!(
        failures.is_empty(),
        "{} of {checks} primary/accent pairs differ:\n{}",
        failures.len(),
        failures.join("\n")
    );
    Ok(())
}

#[cfg(feature = "widgets")]
#[test]
fn every_style_field_equals_its_native_value() -> native_theme::Result<()> {
    let combinations = combinations()?;
    let mut failures = Vec::new();

    // One line per function, plus one per border.
    let mut ran = Checked::default();
    check_style_rows(BUTTON_ROWS, &combinations, &mut failures, &mut ran);
    check_style_rows(BUTTON_PRIMARY_ROWS, &combinations, &mut failures, &mut ran);
    check_style_rows(BUTTON_DANGER_ROWS, &combinations, &mut failures, &mut ran);
    check_style_rows(BUTTON_SUCCESS_ROWS, &combinations, &mut failures, &mut ran);
    check_style_rows(BUTTON_WARNING_ROWS, &combinations, &mut failures, &mut ran);
    check_style_rows(BUTTON_LINK_ROWS, &combinations, &mut failures, &mut ran);
    check_style_rows(TEXT_INPUT_ROWS, &combinations, &mut failures, &mut ran);
    check_style_rows(TEXT_EDITOR_ROWS, &combinations, &mut failures, &mut ran);
    check_border_rows(BUTTON_BORDER_ROWS, &combinations, &mut failures, &mut ran);
    check_border_rows(
        BUTTON_PRIMARY_BORDER_ROWS,
        &combinations,
        &mut failures,
        &mut ran,
    );
    check_border_rows(
        BUTTON_DANGER_BORDER_ROWS,
        &combinations,
        &mut failures,
        &mut ran,
    );
    check_border_rows(
        BUTTON_SUCCESS_BORDER_ROWS,
        &combinations,
        &mut failures,
        &mut ran,
    );
    check_border_rows(
        BUTTON_WARNING_BORDER_ROWS,
        &combinations,
        &mut failures,
        &mut ran,
    );
    check_border_rows(
        TEXT_INPUT_BORDER_ROWS,
        &combinations,
        &mut failures,
        &mut ran,
    );
    check_border_rows(
        TEXT_EDITOR_BORDER_ROWS,
        &combinations,
        &mut failures,
        &mut ran,
    );
    check_style_rows(CHECKBOX_ROWS, &combinations, &mut failures, &mut ran);
    check_border_rows(CHECKBOX_BORDER_ROWS, &combinations, &mut failures, &mut ran);
    check_style_rows(RADIO_ROWS, &combinations, &mut failures, &mut ran);
    check_scalar_rows(RADIO_SCALAR_ROWS, &combinations, &mut failures, &mut ran);
    check_style_rows(TOGGLER_ROWS, &combinations, &mut failures, &mut ran);
    check_scalar_rows(TOGGLER_SCALAR_ROWS, &combinations, &mut failures, &mut ran);
    check_radius_rows(TOGGLER_RADIUS_ROWS, &combinations, &mut failures, &mut ran);
    check_style_rows(PICK_LIST_ROWS, &combinations, &mut failures, &mut ran);
    check_border_rows(
        PICK_LIST_BORDER_ROWS,
        &combinations,
        &mut failures,
        &mut ran,
    );
    check_style_rows(MENU_ROWS, &combinations, &mut failures, &mut ran);
    check_border_rows(MENU_BORDER_ROWS, &combinations, &mut failures, &mut ran);
    check_style_rows(SLIDER_ROWS, &combinations, &mut failures, &mut ran);
    check_scalar_rows(SLIDER_SCALAR_ROWS, &combinations, &mut failures, &mut ran);

    // The calls above are a second hand-maintained list beside
    // `style_row_fields`, and only that one is read by the coverage tripwire:
    // a row registered there but never checked here would leave its field
    // claimed and unasserted.
    ran.covers(&style_row_fields(), "style_row_fields()");

    // `check_border_rows` compares six leaves. If upstream adds a seventh,
    // `border_fields` grows -- and claims it for the tripwire -- while the
    // check would quietly ignore it, so the two are held together here.
    assert_eq!(
        border_fields(&Border::default(), "x").len(),
        BORDER_LEAVES,
        "an iced `Border` no longer has {BORDER_LEAVES} leaves; \
         `check_border_rows` compares that many and must be extended first"
    );
    println!("style rows: {} field checks", ran.checks);

    assert!(
        failures.is_empty(),
        "{} of {} style contract checks failed:\n{}",
        failures.len(),
        ran.checks,
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
    let mut ran = Checked::default();
    let all = &combinations;
    check_style_pairs(BUTTON_PAIRS, all, &mut failures, &mut below_aa, &mut ran);
    check_style_pairs(
        BUTTON_PRIMARY_PAIRS,
        all,
        &mut failures,
        &mut below_aa,
        &mut ran,
    );
    check_style_pairs(
        BUTTON_DANGER_PAIRS,
        all,
        &mut failures,
        &mut below_aa,
        &mut ran,
    );
    check_style_pairs(
        BUTTON_SUCCESS_PAIRS,
        all,
        &mut failures,
        &mut below_aa,
        &mut ran,
    );
    check_style_pairs(
        BUTTON_WARNING_PAIRS,
        all,
        &mut failures,
        &mut below_aa,
        &mut ran,
    );
    check_style_pairs(
        BUTTON_LINK_PAIRS,
        all,
        &mut failures,
        &mut below_aa,
        &mut ran,
    );
    check_style_pairs(
        TEXT_INPUT_PAIRS,
        all,
        &mut failures,
        &mut below_aa,
        &mut ran,
    );
    check_style_pairs(
        TEXT_EDITOR_PAIRS,
        all,
        &mut failures,
        &mut below_aa,
        &mut ran,
    );
    check_style_pairs(CHECKBOX_PAIRS, all, &mut failures, &mut below_aa, &mut ran);
    check_style_pairs(RADIO_PAIRS, all, &mut failures, &mut below_aa, &mut ran);
    check_style_pairs(TOGGLER_PAIRS, all, &mut failures, &mut below_aa, &mut ran);
    check_style_pairs(PICK_LIST_PAIRS, all, &mut failures, &mut below_aa, &mut ran);
    check_style_pairs(MENU_PAIRS, all, &mut failures, &mut below_aa, &mut ran);
    check_style_pairs(SLIDER_PAIRS, all, &mut failures, &mut below_aa, &mut ran);

    // A pair list declared and never checked here asserts nothing, and the
    // tripwire cannot notice -- it walks `Style` fields, and whether a pair
    // exists at all is a judgment per function (section 7). So the names that
    // ran are held against the names declared.
    ran.covers(&style_pair_names(), "style_pair_names()");

    println!(
        "--- styles contrast: {} pairs, {} below AA (printed, not asserted; \
         an [indicator] line is a non-text pair, where WCAG asks 3:1) ---",
        ran.checks,
        below_aa.len()
    );
    for line in &below_aa {
        println!("{line}");
    }
    assert!(
        failures.is_empty(),
        "{} of {} style pairs are worse than the platform's own:\n{}",
        failures.len(),
        ran.checks,
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
fn a_cleared_input_soft_option_copies_the_base_state_value() -> native_theme::Result<()> {
    // The three `input.*` soft options, which `styles::text_input` and
    // `styles::text_editor` share. No bundled preset leaves them `None` after
    // resolution either, so they are cleared here.
    let mut resolved = native_theme::theme::Theme::preset("windows-11")?
        .into_variant(ColorMode::Light)?
        .into_resolved(&native_theme::ResolutionContext::for_tests())?;
    assert!(
        resolved.input.hover_border_color.is_some()
            && resolved.input.focus_border_color.is_some()
            && resolved.input.disabled_background.is_some(),
        "this preset no longer states all three input soft options, so \
         clearing them proves nothing"
    );
    resolved.input.hover_border_color = None;
    resolved.input.focus_border_color = None;
    resolved.input.disabled_background = None;
    let theme = crate::to_theme(&resolved, "windows-11");

    let border = to_color(resolved.input.border.color);
    let background = to_color(resolved.input.background_color);
    let input = styles::text_input(&resolved);
    let editor = styles::text_editor(&resolved);

    for status in TEXT_INPUT_STATUSES {
        assert_eq!(
            input(&theme, *status).border.color,
            border,
            "a cleared input border soft option copies the border's own color \
             ({status:?})"
        );
    }
    assert_eq!(
        fill(input(&theme, text_input::Status::Disabled).background),
        Ok(background),
        "a cleared input disabled_background copies the idle fill, as given"
    );

    for status in TEXT_EDITOR_STATUSES {
        assert_eq!(
            editor(&theme, *status).border.color,
            border,
            "the editor reads the same soft options ({status:?})"
        );
    }
    assert_eq!(
        fill(editor(&theme, text_editor::Status::Disabled).background),
        Ok(background),
        "a cleared input disabled_background copies the idle fill for the \
         editor too"
    );
    Ok(())
}

#[cfg(feature = "widgets")]
#[test]
fn a_cleared_widget_soft_option_copies_the_base_state_value() -> native_theme::Result<()> {
    // The eleven soft options batch 2's functions read: four `checkbox.*`
    // (three of which `styles::radio` reads again through its own
    // expressions), five `switch.*`, one `combo_box.*` and one `slider.*`. No
    // bundled preset leaves any of them `None` after resolution, so the rows
    // cannot tell a right base-state field from a wrong one; the fallbacks are
    // reached by clearing them here (section 3.2).
    let mut resolved = native_theme::theme::Theme::preset("windows-11")?
        .into_variant(ColorMode::Light)?
        .into_resolved(&native_theme::ResolutionContext::for_tests())?;
    assert!(
        resolved.checkbox.hover_background.is_some()
            && resolved.checkbox.disabled_background.is_some()
            && resolved.checkbox.unchecked_background.is_some()
            && resolved.checkbox.unchecked_border_color.is_some()
            && resolved.switch.hover_checked_background.is_some()
            && resolved.switch.hover_unchecked_background.is_some()
            && resolved.switch.disabled_checked_background.is_some()
            && resolved.switch.disabled_unchecked_background.is_some()
            && resolved.switch.disabled_thumb_color.is_some()
            && resolved.combo_box.hover_background.is_some()
            && resolved.slider.thumb_hover_color.is_some(),
        "this preset no longer states all eleven soft options, so clearing \
         them proves nothing"
    );
    resolved.checkbox.hover_background = None;
    resolved.checkbox.disabled_background = None;
    resolved.checkbox.unchecked_background = None;
    resolved.checkbox.unchecked_border_color = None;
    resolved.switch.hover_checked_background = None;
    resolved.switch.hover_unchecked_background = None;
    resolved.switch.disabled_checked_background = None;
    resolved.switch.disabled_unchecked_background = None;
    resolved.switch.disabled_thumb_color = None;
    resolved.combo_box.hover_background = None;
    resolved.slider.thumb_hover_color = None;

    let theme = crate::to_theme(&resolved, "windows-11");
    let boxes = styles::checkbox(&resolved);
    let buttons = styles::radio(&resolved);
    let switches = styles::toggler(&resolved);
    let fields = styles::pick_list(&resolved);
    let rails = styles::slider(&resolved);

    // Every base-state value of section 3.2's table, read straight from the
    // model. A hover layer copies the base state and is then composited over
    // the fill it covers, so the two cases are spelled out separately.
    let box_fill = to_color(resolved.checkbox.background_color);
    let box_checked = to_color(resolved.checkbox.checked_background);
    let box_border = to_color(resolved.checkbox.border.color);
    let track_on = to_color(resolved.switch.checked_background);
    let track_off = to_color(resolved.switch.unchecked_background);
    let thumb = to_color(resolved.switch.thumb_background);
    let field_fill = to_color(resolved.combo_box.background_color);
    let handle = to_color(resolved.slider.thumb_color);

    let mut failures = Vec::new();
    for (soft_option, actual, expected) in [
        (
            "checkbox.unchecked_background -> checkbox.background_color",
            fill(boxes(&theme, checkbox::Status::Active { is_checked: false }).background),
            box_fill,
        ),
        (
            "checkbox.hover_background -> checkbox.background_color",
            fill(boxes(&theme, checkbox::Status::Hovered { is_checked: true }).background),
            over(box_fill, box_checked),
        ),
        (
            "checkbox.disabled_background -> checkbox.background_color",
            fill(boxes(&theme, checkbox::Status::Disabled { is_checked: true }).background),
            box_fill,
        ),
        (
            "checkbox.unchecked_border_color -> checkbox.border.color",
            Ok(
                boxes(&theme, checkbox::Status::Active { is_checked: false })
                    .border
                    .color,
            ),
            box_border,
        ),
        (
            "radio: checkbox.unchecked_background -> checkbox.background_color",
            fill(buttons(&theme, radio::Status::Active { is_selected: false }).background),
            box_fill,
        ),
        (
            "radio: checkbox.hover_background -> checkbox.background_color",
            fill(buttons(&theme, radio::Status::Hovered { is_selected: true }).background),
            over(box_fill, box_checked),
        ),
        (
            "radio: checkbox.unchecked_border_color -> checkbox.border.color",
            Ok(buttons(&theme, radio::Status::Active { is_selected: false }).border_color),
            box_border,
        ),
        (
            "switch.hover_checked_background -> switch.checked_background",
            fill(switches(&theme, toggler::Status::Hovered { is_toggled: true }).background),
            over(track_on, track_on),
        ),
        (
            "switch.hover_unchecked_background -> switch.unchecked_background",
            fill(switches(&theme, toggler::Status::Hovered { is_toggled: false }).background),
            over(track_off, track_off),
        ),
        (
            "switch.disabled_checked_background -> switch.checked_background",
            fill(switches(&theme, toggler::Status::Disabled { is_toggled: true }).background),
            track_on,
        ),
        (
            "switch.disabled_unchecked_background -> switch.unchecked_background",
            fill(switches(&theme, toggler::Status::Disabled { is_toggled: false }).background),
            track_off,
        ),
        (
            "switch.disabled_thumb_color -> switch.thumb_background",
            fill(switches(&theme, toggler::Status::Disabled { is_toggled: true }).foreground),
            thumb,
        ),
        (
            "combo_box.hover_background -> combo_box.background_color",
            fill(fields(&theme, pick_list::Status::Hovered).background),
            over(field_fill, field_fill),
        ),
        (
            "slider.thumb_hover_color -> slider.thumb_color",
            fill(rails(&theme, slider::Status::Hovered).handle.background),
            handle,
        ),
    ] {
        match actual {
            Ok(emitted) if emitted == expected => {}
            Ok(emitted) => failures.push(format!(
                "{soft_option}: cleared, the widget emits {}, the base state \
                 gives {}",
                show(emitted),
                show(expected)
            )),
            Err(why) => failures.push(format!("{soft_option}: {why}")),
        }
    }

    assert!(
        failures.is_empty(),
        "{} of 14 cleared soft options do not copy their base-state value:\n{}",
        failures.len(),
        failures.join("\n")
    );
    Ok(())
}

/// How far apart two orders of the same composite may land in f32.
///
/// `over(over(l, b), surface)` and `over(l, over(b, surface))` are the same
/// number in exact arithmetic and differ only by rounding. One channel of one
/// composite costs at most eight roundings -- `1 - a`, the two products, their
/// sum, the alpha sum and the division -- and each order performs two
/// composites, so at most sixteen roundings of at most half an ulp each. An
/// ulp of the channel range `[0, 1]` is `f32::EPSILON` at its top end, which
/// puts the bound at `16 * f32::EPSILON`, about `1.9e-6`.
///
/// This is a floating-point bound, not a perceptual one: it says nothing about
/// how different two colors may look, only about how far apart two spellings
/// of the same arithmetic may land.
#[cfg(feature = "widgets")]
const COMPOSITE_ULPS: f32 = 16.0 * f32::EPSILON;

/// Whether two colors agree channel by channel to within `tolerance`.
#[cfg(feature = "widgets")]
fn within(a: Color, b: Color, tolerance: f32) -> bool {
    (a.r - b.r).abs() <= tolerance
        && (a.g - b.g).abs() <= tolerance
        && (a.b - b.b).abs() <= tolerance
        && (a.a - b.a).abs() <= tolerance
}

#[cfg(feature = "widgets")]
#[test]
fn compositing_holds_its_four_properties_on_every_native_color() -> native_theme::Result<()> {
    let combinations = combinations()?;
    let mut failures = Vec::new();
    let mut transparent_bases = 0usize;
    let mut visible_layers = 0usize;
    let mut opaque_bases = 0usize;
    let mut opaque_layers = 0usize;
    let mut interior_composites = 0usize;

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

            // (a) a *visible* layer over nothing is the layer: the link case.
            // A fully transparent layer is excluded, and the property is false
            // for one: a composite with no alpha has no color left to
            // un-premultiply, so `composite_over` returns `Color::TRANSPARENT`
            // rather than the layer's own channels, and a colored layer at
            // `a == 0.0` therefore comes back as (0, 0, 0, 0). Nothing is
            // painted either way; only the discarded channels differ.
            if layer.a > 0.0 {
                visible_layers += 1;
                if styles::composite_over(layer, Color::TRANSPARENT) != layer {
                    failures.push(format!("{label}: over a transparent base is not the layer"));
                }
                if link_base.a == 0.0 && styles::composite_over(layer, link_base) != layer {
                    failures.push(format!(
                        "{label}: over link.background_color ({}) is not the layer",
                        show(link_base)
                    ));
                }
            }
            if link_base.a == 0.0 {
                transparent_bases += 1;
            }

            // (b) over an opaque base the result is opaque, and is the
            // simple mix this formula generalises -- computed here, not
            // copied from either implementation.
            if base.a == 1.0 {
                opaque_bases += 1;
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
            if layer.a == 1.0 {
                opaque_layers += 1;
                if styles::composite_over(layer, link_base) != layer {
                    failures.push(format!(
                        "{label}: an opaque layer did not survive unchanged"
                    ));
                }
            }

            // The contract's own helper must agree with the one under test.
            if over(layer, link_base) != styles::composite_over(layer, link_base) {
                failures.push(format!(
                    "{label}: contract `over` and `composite_over` disagree"
                ));
            }
        }

        // (d) the interior case associates. Properties (a) to (c) pin the
        // edges -- a transparent base, an opaque base, an opaque layer -- and
        // leave the one case the formula was rewritten for, a translucent
        // layer over a translucent base, confirmed only by two helpers that
        // compute it the same way. Source-over is associative, so stacking a
        // layer on a base and then painting the pair on an opaque surface must
        // give what painting the base on the surface and then the layer gives.
        // Both colors are real: windows-11 states `checkbox.unchecked_background`
        // as a translucent fill and `link.hover_background` as a translucent
        // layer.
        if let Some(unchecked) = r.checkbox.unchecked_background {
            let b = to_color(unchecked);
            let l = to_color(r.link.hover_background);
            let surface = to_color(r.defaults.background_color);
            let translucent = |x: Color| x.a > 0.0 && x.a < 1.0;
            if translucent(b) && translucent(l) && surface.a == 1.0 {
                interior_composites += 1;
                for (what, composite) in [
                    ("contract `over`", over as fn(Color, Color) -> Color),
                    (
                        "styles::composite_over",
                        styles::composite_over as fn(Color, Color) -> Color,
                    ),
                ] {
                    let stacked = composite(composite(l, b), surface);
                    let painted = composite(l, composite(b, surface));
                    if !within(stacked, painted, COMPOSITE_ULPS) {
                        failures.push(format!(
                            "{}: {what} does not associate: (l over b) over surface is {}, \
                             l over (b over surface) is {}",
                            c.label(),
                            show(stacked),
                            show(painted)
                        ));
                    }
                }
            }
        }
    }

    println!(
        "compositing properties ran: (a) {visible_layers} / transparent bases \
         {transparent_bases}, (b) {opaque_bases}, (c) {opaque_layers}, \
         (d) {interior_composites}"
    );
    assert_eq!(
        transparent_bases,
        combinations.len() * 4,
        "every preset is supposed to state link.background_color as fully \
         transparent; if that changed, property (a) is no longer covered by \
         real data"
    );
    assert_eq!(
        visible_layers,
        combinations.len() * 4,
        "property (a) runs only on a layer with alpha above zero; if a preset \
         starts stating one of these four colors as fully transparent, the \
         count says so instead of the property quietly shrinking"
    );
    assert_eq!(
        opaque_bases,
        combinations.len() * 4,
        "property (b) runs only over an opaque button.background_color, which \
         every preset states as a six-digit hex; a translucent one would stop \
         the property running"
    );
    assert!(
        opaque_layers >= combinations.len() * 2,
        "property (c) needs an opaque layer: button.background_color and \
         defaults.text_color are opaque in every preset, so it must run at \
         least twice per combination, not {opaque_layers} times over {}",
        combinations.len()
    );
    assert!(
        interior_composites >= 2,
        "property (d) needs a translucent layer over a translucent base: \
         windows-11 light and dark state both checkbox.unchecked_background \
         and link.hover_background that way, so it must run at least twice, \
         not {interior_composites} times"
    );
    assert!(
        failures.is_empty(),
        "{} compositing propert(ies) failed:\n{}",
        failures.len(),
        failures.join("\n")
    );
    Ok(())
}
