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
use iced_widget::{button, text_editor, text_input};

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

// A row kind for a `Style` field that is a length rather than a color lived
// here until `BorderRow` arrived: its only five users were a border's width
// and its four radius corners, which one border row now covers. A function
// with a scalar field outside a border -- `toggler::Style.padding_ratio`,
// `scrollable::Style.gap` -- brings it back; `git show e2e2eaf` has it.

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
) -> usize {
    let mut checks = 0;
    for c in combinations {
        for row in rows {
            for &status in row.statuses {
                let expected = (row.native)(&c.resolved, status);
                let actual = (row.get)(&c.theme, &c.resolved, status);

                checks += 1;
                if actual.color != expected.color {
                    failures.push(format!(
                        "{}: {}.border.color ({status:?}) is {}, native gives {}",
                        c.label(),
                        row.field,
                        show(actual.color),
                        show(expected.color)
                    ));
                }

                checks += 1;
                if actual.width != expected.width {
                    failures.push(format!(
                        "{}: {}.border.width ({status:?}) is {}, native gives {}",
                        c.label(),
                        row.field,
                        actual.width,
                        expected.width
                    ));
                }

                for (leaf, corner) in [
                    ("top_left", actual.radius.top_left),
                    ("top_right", actual.radius.top_right),
                    ("bottom_right", actual.radius.bottom_right),
                    ("bottom_left", actual.radius.bottom_left),
                ] {
                    checks += 1;
                    if corner != expected.radius {
                        failures.push(format!(
                            "{}: {}.border.radius.{leaf} ({status:?}) is {corner}, \
                             native gives {}",
                            c.label(),
                            row.field,
                            expected.radius
                        ));
                    }
                }
            }
        }
    }
    checks
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
    // One border row claims six leaves, so its names are built rather than
    // written -- by the same walker the tripwire uses.
    out.extend(border_row_fields(BUTTON_BORDER_ROWS));
    out.extend(border_row_fields(BUTTON_PRIMARY_BORDER_ROWS));
    out.extend(border_row_fields(BUTTON_DANGER_BORDER_ROWS));
    out.extend(border_row_fields(BUTTON_SUCCESS_BORDER_ROWS));
    out.extend(border_row_fields(BUTTON_WARNING_BORDER_ROWS));
    out.extend(border_row_fields(TEXT_INPUT_BORDER_ROWS));
    out.extend(border_row_fields(TEXT_EDITOR_BORDER_ROWS));
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

// `check_scalar_rows` went with `ScalarRow`; `check_border_rows` above is what
// covers the five fields it used to.

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

// The four class buttons' pairs -- `BUTTON_PRIMARY_PAIRS`,
// `BUTTON_DANGER_PAIRS`, `BUTTON_SUCCESS_PAIRS`, `BUTTON_WARNING_PAIRS` --
// are declared by `class_button_contract!` above, beside their rows.

/// The `styles::button_link` pairs the assertion covers.
#[cfg(feature = "widgets")]
const BUTTON_LINK_PAIRS: &[StylePair<button::Status>] = &[StylePair {
    what: "link button label",
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
    let checks = check_style_rows(BUTTON_ROWS, &combinations, &mut failures)
        + check_style_rows(BUTTON_PRIMARY_ROWS, &combinations, &mut failures)
        + check_style_rows(BUTTON_DANGER_ROWS, &combinations, &mut failures)
        + check_style_rows(BUTTON_SUCCESS_ROWS, &combinations, &mut failures)
        + check_style_rows(BUTTON_WARNING_ROWS, &combinations, &mut failures)
        + check_style_rows(BUTTON_LINK_ROWS, &combinations, &mut failures)
        + check_style_rows(TEXT_INPUT_ROWS, &combinations, &mut failures)
        + check_style_rows(TEXT_EDITOR_ROWS, &combinations, &mut failures)
        + check_border_rows(BUTTON_BORDER_ROWS, &combinations, &mut failures)
        + check_border_rows(BUTTON_PRIMARY_BORDER_ROWS, &combinations, &mut failures)
        + check_border_rows(BUTTON_DANGER_BORDER_ROWS, &combinations, &mut failures)
        + check_border_rows(BUTTON_SUCCESS_BORDER_ROWS, &combinations, &mut failures)
        + check_border_rows(BUTTON_WARNING_BORDER_ROWS, &combinations, &mut failures)
        + check_border_rows(TEXT_INPUT_BORDER_ROWS, &combinations, &mut failures)
        + check_border_rows(TEXT_EDITOR_BORDER_ROWS, &combinations, &mut failures);

    // `check_border_rows` compares six leaves. If upstream adds a seventh,
    // `border_fields` grows -- and claims it for the tripwire -- while the
    // check would quietly ignore it, so the two are held together here.
    assert_eq!(
        border_fields(&Border::default(), "x").len(),
        BORDER_LEAVES,
        "an iced `Border` no longer has {BORDER_LEAVES} leaves; \
         `check_border_rows` compares that many and must be extended first"
    );
    println!("style rows: {checks} field checks");

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
    let checks = check_style_pairs(BUTTON_PAIRS, &combinations, &mut failures, &mut below_aa)
        + check_style_pairs(
            BUTTON_PRIMARY_PAIRS,
            &combinations,
            &mut failures,
            &mut below_aa,
        )
        + check_style_pairs(
            BUTTON_DANGER_PAIRS,
            &combinations,
            &mut failures,
            &mut below_aa,
        )
        + check_style_pairs(
            BUTTON_SUCCESS_PAIRS,
            &combinations,
            &mut failures,
            &mut below_aa,
        )
        + check_style_pairs(
            BUTTON_WARNING_PAIRS,
            &combinations,
            &mut failures,
            &mut below_aa,
        )
        + check_style_pairs(
            BUTTON_LINK_PAIRS,
            &combinations,
            &mut failures,
            &mut below_aa,
        )
        + check_style_pairs(
            TEXT_INPUT_PAIRS,
            &combinations,
            &mut failures,
            &mut below_aa,
        )
        + check_style_pairs(
            TEXT_EDITOR_PAIRS,
            &combinations,
            &mut failures,
            &mut below_aa,
        );

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
