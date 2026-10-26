//! Layer 1 of the theme contracts: every palette slot this connector writes
//! and every field of a `styles::*` output, the native value each must equal,
//! and the contrast report of spec section 7.
//!
//! Test-only. The rows are asserted over the seventeen presets of
//! `Theme::list_presets()` in both modes. The contrast report prints both ratios and asserts
//! nothing: iced, not the connector, chooses the background a text input is
//! painted on. The assertion lives over `styles::*`, where we control both
//! colors.
//!
//! The coverage tripwire (section 5.2) closes the table: every field it walks
//! -- by exhaustive destructuring, so an upstream addition fails to compile
//! here first -- must appear in exactly one of the rows, `DERIVED` and
//! `UNREACHABLE`. Its converse walks the model rather than the output:
//! `every_colour_leaf_reaches_a_row_or_is_named` splices every colour leaf of
//! `ResolvedTheme` and requires each to move a row's native side, or to be
//! named by `UNREAD_COLOURS` or `UNREACHABLE`. What it finds -- per leaf, the
//! rows it moves under which statuses and in which combinations -- is
//! exported as `contract-leaves.toml` at the crate root, which the iced
//! showcase reads for its Widget Info and the test holds to the walk.

use crate::extended::contrast_ratio;
use crate::palette::to_color;
use crate::{ColorMode, ResolvedTheme};
use iced_core::Color;
use iced_core::theme::Theme;
use iced_core::theme::palette::{Danger, Extended, Pair, Primary, Secondary, Success, Warning};
use native_theme::color::Rgba;
#[cfg(feature = "iced_aw")]
use std::collections::{BTreeMap, BTreeSet};

#[cfg(feature = "widgets")]
use crate::styles;
#[cfg(feature = "widgets")]
use iced_core::{Background, Border, border::Radius};
#[cfg(feature = "widgets")]
use iced_widget::overlay::menu;
#[cfg(feature = "widgets")]
use iced_widget::{
    button, checkbox, container, pane_grid, pick_list, progress_bar, radio, rule, scrollable,
    slider, text_editor, text_input, toggler,
};

// `styles::aw` and its rows are gated on `iced_aw`, which implies `widgets`.
#[cfg(feature = "iced_aw")]
use iced_aw::style::Status as AwStatus;
#[cfg(feature = "iced_aw")]
use iced_aw::style::{card, menu_bar, selection_list, sidebar, tab_bar};

// The declared data lives in three modules of its own; the row kinds, the
// walkers, the `check_*` helpers and every `#[test]` stay here, because the
// tests are the single roots of trust and read all three tables at once.
mod derived;
mod pairs;
mod rows;

use derived::{
    DERIVED, SECONDARY_LABEL_AS_STATED, SECONDARY_LABEL_SUBSTITUTED, UNREACHABLE, Unreachable,
};
#[cfg(feature = "iced_aw")]
use derived::{UNREAD_COLOURS, Unread, UnreadColour};
use pairs::*;
use rows::*;

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

/// One row of the accessor contract: a colour a crate-root function hands the
/// consumer, or one the connector's own `Spinner` draws in, and the native
/// leaf it must be.
///
/// Both sides are an `Option` for the one accessor that returns a widget,
/// `tab_indicator_line`, which draws nothing where the theme states no line:
/// its row compares the strip's colour, `None` where no strip is drawn.
struct AccessorRow {
    accessor: &'static str,
    native: fn(&ResolvedTheme) -> Option<Color>,
    get: fn(&ResolvedTheme) -> Option<Color>,
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

/// Every leaf of an iced `container::Style` under `prefix`, destructured with
/// no `..`.
///
/// One walker for every place one appears -- `styles::tooltip`,
/// `styles::container_card`, the `container` a `scrollable::Style` nests, and
/// `styles::aw::spinner` where the `iced_aw` feature is on -- so an upstream
/// addition fails to compile here once rather than once per caller. It is one
/// of the two structs with a `Default`, so without this nothing would notice
/// the addition at all (section 3.2).
#[cfg(feature = "widgets")]
fn container_fields(style: &container::Style, prefix: &str) -> Vec<String> {
    let mut out = Vec::new();
    leaves_under!(out, style, prefix, container::Style {
        text_color, background, shadow, snap, @nested border
    });
    out.extend(border_fields(border, &format!("{prefix}.border")));
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
                    Radius::new(expected.radius),
                    &format!("{}.border.radius", row.field),
                    &format!("{status:?}"),
                    &c.label(),
                    failures,
                );
            }
        }
    }
}

/// Compare the four corners of an emitted `Radius` against the corners the
/// native value gives, naming the exact corner. Returns how many comparisons
/// it made.
///
/// The model states a corner radius as a single length, so the four corners
/// are mostly one native value four times over (`Radius::new`); a selected
/// tab's is that length on its top corners only. A row that claims a `Radius`
/// claims all four, and a failure has to say which one drifted.
#[cfg(feature = "widgets")]
fn check_corners(
    actual: Radius,
    expected: Radius,
    field: &str,
    status: &str,
    label: &str,
    failures: &mut Vec<String>,
) -> usize {
    for (leaf, corner, expected) in [
        ("top_left", actual.top_left, expected.top_left),
        ("top_right", actual.top_right, expected.top_right),
        ("bottom_right", actual.bottom_right, expected.bottom_right),
        ("bottom_left", actual.bottom_left, expected.bottom_left),
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
    native: fn(&ResolvedTheme, S) -> Radius,
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

/// Declare the whole contract of one class button: its two color rows, its
/// border row, its contrast pair and its field enumerator.
///
/// `styles::button_primary`, `button_danger`, `button_success` and
/// `button_warning` are the same function four times over at rest and
/// disabled -- they differ only in the idle pair they paint -- so their
/// contract is written once here and instantiated four times. `$fill` and
/// `$label` are accessors of the idle pair, `fn(&ResolvedTheme) -> Rgba`.
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
            native: native_button_border,
            get: |t, r, s| $style(r)(t, s).border,
        }];

        /// Only `CLASS_BUTTON_NATIVE_STATUSES`: in the other two the native
        /// fields state neither the fill nor the label -- both are iced's
        /// derivation -- so there is no native pair to compare against and
        /// section 7's rule has nothing to say.
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
    /// How many of those comparisons were contrast pairs below AA. The printed
    /// lines collapse the statuses that read alike, so the count of lines is
    /// no longer the count of pairs, and the header prints both.
    below_aa: usize,
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

/// WCAG 2.1 AA for normal text. Only ever used to decide what to print.
const AA: f32 = 4.5;

/// One below-AA line before it is printed: a pair, a combination, the two
/// ratios, and every status that produced exactly those.
///
/// A widget whose status carries booleans reads the same for many of its
/// values -- a scrollable has thirty-six statuses and three distinct scrollers
/// -- so a line per status buried the list under its own repetitions. The
/// grouping is on what the reader compares, and it is a **print** only: what
/// is asserted, how many pairs ran and what `Checked` records are untouched,
/// and the count of below-AA comparisons is kept separately so the header
/// still says how many there were.
#[cfg(feature = "widgets")]
struct BelowAa {
    what: &'static str,
    label: String,
    emitted: f32,
    native: f32,
    indicator: bool,
    statuses: Vec<String>,
}

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
    let mut collapsed: Vec<BelowAa> = Vec::new();
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
                            ran.below_aa += 1;
                            // One line per pair, combination and pair of
                            // ratios: a status that reads exactly like an
                            // earlier one joins that line instead of adding
                            // another.
                            let label = c.label();
                            match collapsed.iter_mut().find(|line| {
                                line.what == pair.what
                                    && line.label == label
                                    && line.emitted == emitted
                                    && line.native == native
                            }) {
                                Some(line) => line.statuses.push(format!("{status:?}")),
                                None => collapsed.push(BelowAa {
                                    what: pair.what,
                                    label,
                                    emitted,
                                    native,
                                    indicator: pair.indicator,
                                    statuses: vec![format!("{status:?}")],
                                }),
                            }
                        }
                    }
                    Err(why) => {
                        failures.push(format!("{}: {} ({status:?}) {why}", c.label(), pair.what));
                    }
                }
            }
        }
    }

    for line in collapsed {
        // Few enough statuses to name them; otherwise their count, which is
        // what a thirty-six-value status enum makes readable.
        let which = if line.statuses.len() <= 4 {
            line.statuses.join(", ")
        } else {
            format!("{} statuses", line.statuses.len())
        };
        below_aa.push(format!(
            "{}: {} {:.2} (native {:.2}) -- {which}{}",
            line.label,
            line.what,
            line.emitted,
            line.native,
            if line.indicator { "  [indicator]" } else { "" }
        ));
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
    if top.a <= 0.0 {
        // `top` adds nothing; the composite is `bottom`, transparent or not.
        return bottom;
    }
    let under = bottom.a * (1.0 - top.a);
    let alpha = top.a + under;
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
fn every_accessor_returns_its_native_leaf() -> native_theme::Result<()> {
    let combinations = combinations()?;
    let mut failures = Vec::new();
    for c in &combinations {
        for row in ACCESSOR_ROWS {
            let (native, got) = ((row.native)(&c.resolved), (row.get)(&c.resolved));
            if native != got {
                failures.push(format!(
                    "{} on {}: {got:?}, the native leaf is {native:?}",
                    row.accessor,
                    c.label()
                ));
            }
        }
    }
    assert!(
        failures.is_empty(),
        "{} accessor check(s) failed:\n{}",
        failures.len(),
        failures.join("\n")
    );
    Ok(())
}

/// `tab_flares`' two rows draw where the theme states
/// `tab.active_outward_corner_radius`, which of the bundled combinations
/// only `windows-11`'s two do (4, G3 T15), so
/// `every_accessor_returns_its_native_leaf` holds them at `None` elsewhere
/// and at 4 there: here they hold on `windows-11` in both modes with the
/// radius set, its fill and outline drawn, the outline gone where the line
/// width is 0, and both gone where the radius is 0 or not finite.
#[cfg(feature = "widgets")]
#[test]
fn the_tab_flares_rows_hold_where_the_theme_curves_out() -> native_theme::Result<()> {
    let rows: Vec<&AccessorRow> = ACCESSOR_ROWS
        .iter()
        .filter(|row| row.accessor.starts_with("crate::tab_flares"))
        .collect();
    assert_eq!(rows.len(), 2, "the fill row and the outline row");
    let mut failures = Vec::new();
    let windows: Vec<_> = combinations()?
        .into_iter()
        .filter(|c| c.key == "windows-11")
        .collect();
    assert_eq!(windows.len(), 2, "windows-11 in its two modes");
    for c in windows {
        for (radius, width, fill, outline) in [
            (Some(4.0), None, true, true),
            (Some(4.0), Some(0.0), true, false),
            (Some(0.0), None, false, false),
            (Some(f32::NAN), None, false, false),
        ] {
            let mut r = c.resolved.clone();
            r.tab.active_outward_corner_radius = radius;
            if let Some(width) = width {
                r.tab.border.line_width = width;
            }
            for row in &rows {
                let (native, got) = ((row.native)(&r), (row.get)(&r));
                let drawn = if row.accessor.ends_with("(fill)") {
                    fill
                } else {
                    outline
                };
                if native != got || native.is_some() != drawn {
                    failures.push(format!(
                        "{} on {} at {radius:?}, width {width:?}: {got:?}, the native leaf is \
                         {native:?}",
                        row.accessor,
                        c.label()
                    ));
                }
            }
        }
    }
    assert!(failures.is_empty(), "{failures:#?}");
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
// requires each name it yields to sit in exactly one of the rows, `DERIVED`
// and `UNREACHABLE`.

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
        extended.background.base,
        "extended.background.base",
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

/// Every field of the `button::Style` `styles::button_toggle` emits.
#[cfg(feature = "widgets")]
fn button_toggle_style_fields(style: &button::Style) -> Vec<String> {
    let mut out = Vec::new();
    leaves_under!(out, style, "styles::button_toggle", button::Style {
        background, text_color, shadow, snap, @nested border
    });
    out.extend(border_fields(border, "styles::button_toggle.border"));
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

/// Every field of the `button::Style` `styles::segment` emits.
#[cfg(feature = "widgets")]
fn segment_style_fields(style: &button::Style) -> Vec<String> {
    let mut out = Vec::new();
    leaves_under!(out, style, "styles::segment", button::Style {
        background, text_color, shadow, snap, @nested border
    });
    out.extend(border_fields(border, "styles::segment.border"));
    out
}

/// Every field of the `button::Style` `styles::expander` emits.
#[cfg(feature = "widgets")]
fn expander_style_fields(style: &button::Style) -> Vec<String> {
    let mut out = Vec::new();
    leaves_under!(out, style, "styles::expander", button::Style {
        background, text_color, shadow, snap, @nested border
    });
    out.extend(border_fields(border, "styles::expander.border"));
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

/// Every leaf of one `scrollable::Rail` under `prefix`: its own fill and
/// border, and the `Scroller` that slides along it with a border of its own.
#[cfg(feature = "widgets")]
fn rail_fields(rail: &scrollable::Rail, prefix: &str) -> Vec<String> {
    let mut out = Vec::new();
    leaves_under!(out, rail, prefix, scrollable::Rail {
        background, @nested border, scroller
    });
    out.extend(border_fields(border, &format!("{prefix}.border")));
    leaves_under!(out, scroller, format!("{prefix}.scroller"), scrollable::Scroller {
        background, @nested border
    });
    out.extend(border_fields(border, &format!("{prefix}.scroller.border")));
    out
}

/// Every field of the `scrollable::Style` the connector emits, through its
/// four nested structs.
#[cfg(feature = "widgets")]
fn scrollable_style_fields(style: &scrollable::Style) -> Vec<String> {
    let mut out = Vec::new();
    leaves_under!(out, style, "styles::scrollable", scrollable::Style {
        gap, @nested container, vertical_rail, horizontal_rail, auto_scroll
    });
    out.extend(container_fields(container, "styles::scrollable.container"));
    out.extend(rail_fields(
        vertical_rail,
        "styles::scrollable.vertical_rail",
    ));
    out.extend(rail_fields(
        horizontal_rail,
        "styles::scrollable.horizontal_rail",
    ));
    leaves_under!(
        out,
        auto_scroll,
        "styles::scrollable.auto_scroll",
        scrollable::AutoScroll {
            background,
            shadow,
            icon,
            @nested border
        }
    );
    out.extend(border_fields(
        border,
        "styles::scrollable.auto_scroll.border",
    ));
    out
}

/// Every field of the `progress_bar::Style` the connector emits.
#[cfg(feature = "widgets")]
fn progress_bar_style_fields(style: &progress_bar::Style) -> Vec<String> {
    let mut out = Vec::new();
    leaves_under!(out, style, "styles::progress_bar", progress_bar::Style {
        background, bar, @nested border
    });
    out.extend(border_fields(border, "styles::progress_bar.border"));
    out
}

/// Every field of a `rule::Style` the connector emits under `prefix`
/// (`styles::rule`, `styles::toolbar_rule`). Its `radius` is a whole
/// `Radius` rather than part of a `Border`, and it is walked as the one leaf
/// it is -- as `toggler::Style.border_radius` is.
#[cfg(feature = "widgets")]
fn rule_style_fields(style: &rule::Style, prefix: &str) -> Vec<String> {
    let mut out = Vec::new();
    leaves_under!(
        out,
        style,
        prefix,
        rule::Style {
            color,
            radius,
            fill_mode,
            snap
        }
    );
    out
}

/// Every field of the `pane_grid::Style` the connector emits: two lines, a
/// colour and a width each, and the drop-target highlight, a background and an
/// iced `Border`.
#[cfg(feature = "widgets")]
fn pane_grid_style_fields(style: &pane_grid::Style) -> Vec<String> {
    let mut out = Vec::new();
    leaves_under!(out, style, "styles::pane_grid", pane_grid::Style {
        @nested hovered_region, picked_split, hovered_split
    });
    leaves_under!(
        out,
        hovered_region,
        "styles::pane_grid.hovered_region",
        pane_grid::Highlight { background, @nested border }
    );
    out.extend(border_fields(
        border,
        "styles::pane_grid.hovered_region.border",
    ));
    for (line, prefix) in [
        (picked_split, "styles::pane_grid.picked_split"),
        (hovered_split, "styles::pane_grid.hovered_split"),
    ] {
        leaves_under!(out, line, prefix, pane_grid::Line { color, width });
    }
    out
}

/// Both fields of the `theme::Style` `styles::application` emits.
#[cfg(feature = "widgets")]
fn application_style_fields(style: &iced_core::theme::Style) -> Vec<String> {
    let mut out = Vec::new();
    leaves_under!(
        out,
        style,
        "styles::application",
        iced_core::theme::Style {
            background_color,
            text_color
        }
    );
    out
}

/// Every field of the `card::Style` the connector emits. A card's border is
/// three bare fields rather than an iced `Border`, so there is nothing to walk.
#[cfg(feature = "iced_aw")]
fn aw_card_style_fields(style: &card::Style) -> Vec<String> {
    let mut out = Vec::new();
    leaves_under!(
        out,
        style,
        "styles::aw::card",
        card::Style {
            background,
            border_radius,
            border_width,
            border_color,
            head_background,
            head_text_color,
            body_background,
            body_text_color,
            foot_background,
            foot_text_color,
            close_color
        }
    );
    out
}

/// Every field of the `menu_bar::Style` the connector emits.
///
/// The struct spells its three borders as `bar_border`, `menu_border` and
/// `path_border`; their leaves are named `bar.border.*`, `menu.border.*` and
/// `path.border.*` so that one `BorderRow` claims the six of each, as it does
/// for every other function.
#[cfg(feature = "iced_aw")]
fn aw_menu_style_fields(style: &menu_bar::Style) -> Vec<String> {
    let mut out = Vec::new();
    leaves_under!(out, style, "styles::aw::menu", menu_bar::Style {
        bar_background, bar_shadow, menu_background, menu_shadow, path,
        @nested bar_border, menu_border, path_border
    });
    out.extend(border_fields(bar_border, "styles::aw::menu.bar.border"));
    out.extend(border_fields(menu_border, "styles::aw::menu.menu.border"));
    out.extend(border_fields(path_border, "styles::aw::menu.path.border"));
    out
}

/// Every field of the `tab_bar::Style` the connector emits. Its two radii are
/// whole `Radius` values rather than parts of a `Border`, and are walked as
/// the one leaf each is.
#[cfg(feature = "iced_aw")]
fn aw_tab_bar_style_fields(style: &tab_bar::Style) -> Vec<String> {
    let mut out = Vec::new();
    leaves_under!(
        out,
        style,
        "styles::aw::tab_bar",
        tab_bar::Style {
            background,
            border_color,
            border_width,
            tab_border_radius,
            tab_label_background,
            tab_label_border_color,
            tab_label_border_width,
            icon_color,
            icon_background,
            icon_border_radius,
            text_color
        }
    );
    out
}

/// Every field of the `sidebar::Style` the connector emits. It is the tab
/// bar's without the tab radius.
#[cfg(feature = "iced_aw")]
fn aw_sidebar_style_fields(style: &sidebar::Style) -> Vec<String> {
    let mut out = Vec::new();
    leaves_under!(
        out,
        style,
        "styles::aw::sidebar",
        sidebar::Style {
            background,
            border_color,
            border_width,
            tab_label_background,
            tab_label_border_color,
            tab_label_border_width,
            icon_color,
            icon_background,
            icon_border_radius,
            text_color
        }
    );
    out
}

/// Every field of the `selection_list::Style` the connector emits.
#[cfg(feature = "iced_aw")]
fn aw_selection_list_style_fields(style: &selection_list::Style) -> Vec<String> {
    let mut out = Vec::new();
    leaves_under!(
        out,
        style,
        "styles::aw::selection_list",
        selection_list::Style {
            text_color,
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
        out.extend(button_toggle_style_fields(&styles::button_toggle(
            resolved, true,
        )(
            theme, button::Status::Active
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
        out.extend(scrollable_style_fields(&styles::scrollable(resolved)(
            theme,
            scrollable_active(false, false),
        )));
        out.extend(progress_bar_style_fields(&styles::progress_bar(resolved)(
            theme,
        )));
        out.extend(rule_style_fields(
            &styles::rule(resolved)(theme),
            "styles::rule",
        ));
        out.extend(rule_style_fields(
            &styles::toolbar_rule(resolved)(theme),
            "styles::toolbar_rule",
        ));
        out.extend(container_fields(
            &styles::segment_separator(resolved, false)(theme),
            "styles::segment_separator",
        ));
        out.extend(pane_grid_style_fields(&styles::pane_grid(resolved)(theme)));
        out.extend(application_style_fields(&styles::application(resolved)(
            theme,
        )));
        // Two functions emit a `container::Style`, so the one walker runs
        // twice under two prefixes.
        out.extend(container_fields(
            &styles::tooltip(resolved)(theme),
            "styles::tooltip",
        ));
        out.extend(container_fields(
            &styles::dialog(resolved)(theme),
            "styles::dialog",
        ));
        out.extend(container_fields(
            &styles::toolbar(resolved)(theme),
            "styles::toolbar",
        ));
        out.extend(container_fields(
            &styles::container_card(resolved)(theme),
            "styles::container_card",
        ));
        out.extend(container_fields(
            &styles::segmented_control(resolved, false)(theme),
            "styles::segmented_control",
        ));
        out.extend(segment_style_fields(&styles::segment(
            resolved,
            false,
            styles::SegmentPosition::Middle,
        )(
            theme, button::Status::Active
        )));
        out.extend(expander_style_fields(&styles::expander(resolved)(
            theme,
            button::Status::Active,
        )));
        out.extend(container_fields(
            &styles::expander_frame(resolved)(theme),
            "styles::expander_frame",
        ));
    }
    #[cfg(feature = "iced_aw")]
    {
        out.extend(aw_card_style_fields(&styles::aw::card(resolved)(
            theme,
            AwStatus::Active,
        )));
        out.extend(aw_menu_style_fields(&styles::aw::menu(resolved)(
            theme,
            AwStatus::Active,
        )));
        out.extend(aw_tab_bar_style_fields(&styles::aw::tab_bar(resolved)(
            theme,
            AwStatus::Active,
        )));
        out.extend(aw_sidebar_style_fields(&styles::aw::sidebar(resolved)(
            theme,
            AwStatus::Active,
        )));
        out.extend(aw_selection_list_style_fields(&styles::aw::selection_list(
            resolved,
        )(
            theme, AwStatus::Active
        )));
        // The spinner's style is a `container::Style`, so the walker the two
        // other container functions use serves it as well.
        out.extend(container_fields(
            &styles::aw::spinner(resolved)(theme),
            "styles::aw::spinner",
        ));
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
fn the_contract_covers_seventeen_presets_in_both_modes() -> native_theme::Result<()> {
    assert_eq!(
        combinations()?.len(),
        34,
        "the contract asserts over 17 presets x 2 modes; a shrinking preset \
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
            let unreachable = UNREACHABLE.iter().filter(|u| u.field == field).count();
            match (rows, derived, unreachable) {
                (1, 0, 0) | (0, 1, 0) | (0, 0, 1) => {}
                (0, 0, 0) => failures.push(format!(
                    "{field}: in none of the rows, DERIVED and UNREACHABLE"
                )),
                (r, d, u) => failures.push(format!(
                    "{field}: claimed by {r} row(s), {d} DERIVED and {u} UNREACHABLE"
                )),
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

    // Most `UNREACHABLE` entries name a *native* field
    // (`scrollbar.min_thumb_length`), which the walk above never meets, so
    // these two cross-checks are what keeps a row or a `DERIVED` entry from
    // mapping one. An entry that names an emitted field
    // (`styles::aw::card.close_color`) is walked, and the match above already
    // lets it be claimed once; the checks hold it too. The live protection for
    // each entry's own claim is `every_unreachable_value_is_one_a_preset_loses`.
    for Unreachable {
        field, evidence, ..
    } in UNREACHABLE
    {
        assert_eq!(
            rows_claiming(field),
            0,
            "{field} is listed as unreachable ({evidence}) yet a row maps it"
        );
        assert!(
            !DERIVED.iter().any(|(name, _)| name == field),
            "{field} is listed as unreachable ({evidence}) yet DERIVED also \
             accounts for it; a value is mapped, derived or unreachable, never \
             two of the three"
        );
    }
    Ok(())
}

/// Every value `UNREACHABLE` lists is one the presets actually state and iced
/// does not draw.
///
/// `UNREACHABLE` is mostly a claim about native fields, which the coverage
/// tripwire cannot see: it walks what `styles::*` returns. What can be checked
/// is that each entry is about something real -- its `lost` reader names the
/// field, so removing it from the model breaks this file, and it answers per
/// combination what the platform states that iced does not draw. That answer
/// must be `None` exactly on the entry's declared exceptions, so a preset
/// change that moves it fails here rather than leaving the list stale, and an
/// entry every combination is an exception to would be about nothing. An
/// entry that names an emitted field must also be one the tripwire walks.
#[test]
fn every_unreachable_value_is_one_a_preset_loses() -> native_theme::Result<()> {
    let combinations = combinations()?;
    let mut failures = Vec::new();

    for entry in UNREACHABLE {
        let mut lost = Vec::new();
        for c in &combinations {
            let label = c.label();
            let excepted = entry.exceptions.iter().any(|(key, _)| *key == label);
            match ((entry.lost)(&c.theme, &c.resolved), excepted) {
                (Some(value), false) => lost.push(format!("{label}: {value}")),
                (None, true) => {}
                (Some(value), true) => failures.push(format!(
                    "{}: {label} is a declared exception yet loses {value}",
                    entry.field
                )),
                (None, false) => failures.push(format!(
                    "{}: {label} loses nothing and is not a declared exception",
                    entry.field
                )),
            }
        }
        println!(
            "{}, which iced cannot draw, lost on {} of {} combinations:\n{}",
            entry.field,
            lost.len(),
            combinations.len(),
            lost.join("\n")
        );
        if lost.is_empty() {
            failures.push(format!(
                "{}: no combination loses it, so the entry is about nothing",
                entry.field
            ));
        }
        for (key, _) in entry.exceptions {
            if !combinations.iter().any(|c| c.label() == *key) {
                failures.push(format!(
                    "{}: the exception {key} is no bundled combination",
                    entry.field
                ));
            }
        }
        if entry.field.starts_with("styles::")
            && !combinations.iter().any(|c| {
                named_fields(&c.theme, &c.resolved)
                    .iter()
                    .any(|f| f == entry.field)
            })
        {
            failures.push(format!(
                "{}: names an emitted field the tripwire never walks",
                entry.field
            ));
        }
    }

    assert!(
        failures.is_empty(),
        "{} UNREACHABLE claim(s) do not hold:\n{}",
        failures.len(),
        failures.join("\n")
    );
    Ok(())
}

/// The `extended.secondary.base.text` entry's figures, how many labels are
/// substituted and which are not, counted rather than asserted in prose.
///
/// The entry says `Pair::new` does not emit the label the connector hands it:
/// it runs `readable()` against the placeholder fill, and the platform's own
/// `defaults.text_color` survives that only where it clears iced's contrast
/// bar. How often it does is the entry's claim, and this is the measurement
/// behind it -- `SECONDARY_LABEL_SUBSTITUTED` and `SECONDARY_LABEL_AS_STATED`
/// are declared beside the sentence, so a preset added to the bundle moves
/// the count and fails here instead of leaving the prose stale.
#[test]
fn readable_substitution_of_the_secondary_label_matches_the_declared_count()
-> native_theme::Result<()> {
    let combinations = combinations()?;
    let mut substituted = Vec::new();
    let mut as_stated = Vec::new();

    for c in &combinations {
        let emitted = c.theme.extended_palette().secondary.base.text;
        let native = to_color(c.resolved.defaults.text_color);
        if emitted == native {
            as_stated.push(c.label());
        } else {
            substituted.push(format!(
                "{}: {} instead of {}",
                c.label(),
                show(emitted),
                show(native)
            ));
        }
    }

    println!(
        "readable() substitutes the secondary label on {} of {} combinations:\n{}",
        substituted.len(),
        combinations.len(),
        substituted.join("\n")
    );
    println!("it emits defaults.text_color as the platform states it on: {as_stated:?}");

    let as_stated: Vec<&str> = as_stated.iter().map(String::as_str).collect();
    assert_eq!(
        substituted.len(),
        SECONDARY_LABEL_SUBSTITUTED,
        "the DERIVED entry for extended.secondary.base.text says {} of {} \
         combinations emit a substitute; the ones that do not are {:?}",
        SECONDARY_LABEL_SUBSTITUTED,
        combinations.len(),
        as_stated
    );
    assert_eq!(
        as_stated.as_slice(),
        SECONDARY_LABEL_AS_STATED,
        "the DERIVED entry names the combinations whose label survives \
         readable() unchanged"
    );
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

    // `scrollable::Status` is the one enum whose values are a product: two
    // booleans on `Active` and four on each of the other two variants. Writing
    // thirty-six literals twice over would say less than the product does, so
    // `all` is built by counting the booleans up -- the same order
    // `SCROLLABLE_STATUSES` is written in -- and the two lists are then held
    // against each other. The exhaustive match still names every field of
    // every variant, so a fifth boolean upstream fails to compile here.
    let mut all = Vec::new();
    for horizontal_off in [false, true] {
        for vertical_off in [false, true] {
            all.push(scrollable_active(horizontal_off, vertical_off));
        }
    }
    for horizontal in [false, true] {
        for vertical in [false, true] {
            for horizontal_off in [false, true] {
                for vertical_off in [false, true] {
                    all.push(scrollable_hovered(
                        horizontal,
                        vertical,
                        horizontal_off,
                        vertical_off,
                    ));
                }
            }
        }
    }
    for horizontal in [false, true] {
        for vertical in [false, true] {
            for horizontal_off in [false, true] {
                for vertical_off in [false, true] {
                    all.push(scrollable_dragged(
                        horizontal,
                        vertical,
                        horizontal_off,
                        vertical_off,
                    ));
                }
            }
        }
    }
    assert_eq!(
        all.len(),
        SCROLLABLE_STATUS_VALUES,
        "the product above must build every value of scrollable::Status"
    );
    for status in &all {
        match status {
            scrollable::Status::Active {
                is_horizontal_scrollbar_disabled: _,
                is_vertical_scrollbar_disabled: _,
            }
            | scrollable::Status::Hovered {
                is_horizontal_scrollbar_hovered: _,
                is_vertical_scrollbar_hovered: _,
                is_horizontal_scrollbar_disabled: _,
                is_vertical_scrollbar_disabled: _,
            }
            | scrollable::Status::Dragged {
                is_horizontal_scrollbar_dragged: _,
                is_vertical_scrollbar_dragged: _,
                is_horizontal_scrollbar_disabled: _,
                is_vertical_scrollbar_disabled: _,
            } => {}
        }
        assert!(
            SCROLLABLE_STATUSES.contains(status),
            "SCROLLABLE_STATUSES does not list {status:?}, so no scrollable \
             row covers it"
        );
    }
    assert_eq!(
        SCROLLABLE_STATUSES.len(),
        all.len(),
        "SCROLLABLE_STATUSES must name each status exactly once"
    );

    // The four shape-B widgets take no status either, so their rows hold the
    // unit value once, as a menu's do.
    for (what, statuses) in [
        ("PROGRESS_BAR_STATUSES", PROGRESS_BAR_STATUSES),
        ("RULE_STATUSES", RULE_STATUSES),
        ("PANE_GRID_STATUSES", PANE_GRID_STATUSES),
        ("TOOLTIP_STATUSES", TOOLTIP_STATUSES),
        ("APPLICATION_STATUSES", APPLICATION_STATUSES),
        ("DIALOG_STATUSES", DIALOG_STATUSES),
        ("TOOLBAR_STATUSES", TOOLBAR_STATUSES),
        ("CONTAINER_CARD_STATUSES", CONTAINER_CARD_STATUSES),
        ("EXPANDER_FRAME_STATUSES", EXPANDER_FRAME_STATUSES),
    ] {
        assert_eq!(
            statuses,
            &[()],
            "{what}: the widget takes no status, so its rows hold the unit \
             value exactly once"
        );
    }
    // The segmented control's container and separators take no status, but
    // a `disabled` flag: their rows hold both values, once each.
    assert_eq!(
        SEGMENTED_CONTROL_STATUSES,
        &[false, true],
        "SEGMENTED_CONTROL_STATUSES: the styles take `disabled`, so its rows hold \
         both values exactly once"
    );

    // A segment is a button that is selected or not, and a toggle button one
    // that is checked or not: every button status, once for each.
    for (what, statuses) in [
        ("SEGMENT_STATUSES", SEGMENT_STATUSES),
        ("BUTTON_TOGGLE_STATUSES", BUTTON_TOGGLE_STATUSES),
    ] {
        for on in [false, true] {
            for status in BUTTON_STATUSES {
                assert!(
                    statuses.contains(&(on, *status)),
                    "{what} does not list {status:?} with {on}, so no row \
                     covers it"
                );
            }
        }
        assert_eq!(
            statuses.len(),
            2 * BUTTON_STATUSES.len(),
            "{what} must name each state exactly once"
        );
    }
    let all = [
        styles::SegmentPosition::First,
        styles::SegmentPosition::Middle,
        styles::SegmentPosition::Last,
        styles::SegmentPosition::Only,
    ];
    for position in all {
        match position {
            styles::SegmentPosition::First
            | styles::SegmentPosition::Middle
            | styles::SegmentPosition::Last
            | styles::SegmentPosition::Only => {}
        }
        assert!(
            SEGMENT_POSITIONS.contains(&position),
            "SEGMENT_POSITIONS does not list {position:?}, so no corner row \
             covers it"
        );
    }
    assert_eq!(
        SEGMENT_POSITIONS.len(),
        all.len(),
        "SEGMENT_POSITIONS must name each position exactly once"
    );

    // `iced_aw` states one `Status` for all of its widgets, and each widget
    // requests a subset of it. `AW_STATUSES` is the root of trust for the
    // whole enum; the three widgets whose appearance depends on the status
    // split it in two, as a class button splits iced's. `styles::aw::spinner`
    // styles a container and takes no status at all, so its list is the unit
    // value, like every other shape-B function's.
    #[cfg(feature = "iced_aw")]
    {
        assert_eq!(
            AW_SPINNER_STATUSES,
            &[()],
            "AW_SPINNER_STATUSES: the wrapped spinner takes no status, so its \
             rows hold the unit value exactly once"
        );
        let all = [
            AwStatus::Active,
            AwStatus::Hovered,
            AwStatus::Pressed,
            AwStatus::Disabled,
            AwStatus::Focused,
            AwStatus::Selected,
        ];
        for status in all {
            match status {
                AwStatus::Active
                | AwStatus::Hovered
                | AwStatus::Pressed
                | AwStatus::Disabled
                | AwStatus::Focused
                | AwStatus::Selected => {}
            }
            assert!(
                AW_STATUSES.contains(&status),
                "AW_STATUSES does not list {status:?}, so no iced_aw row covers it"
            );
        }
        assert_eq!(
            AW_STATUSES.len(),
            all.len(),
            "AW_STATUSES must name each status exactly once"
        );

        for (what, native_statuses, iced_statuses) in [
            (
                "tab bar",
                AW_TAB_BAR_NATIVE_STATUSES,
                AW_TAB_BAR_ICED_STATUSES,
            ),
            (
                "sidebar",
                AW_SIDEBAR_NATIVE_STATUSES,
                AW_SIDEBAR_ICED_STATUSES,
            ),
            (
                "selection list",
                AW_SELECTION_LIST_NATIVE_STATUSES,
                AW_SELECTION_LIST_ICED_STATUSES,
            ),
        ] {
            for status in all {
                let native = native_statuses.contains(&status);
                let from_iced = iced_statuses.contains(&status);
                assert!(
                    native != from_iced,
                    "{status:?} is in {} of the two {what} lists; each status \
                     belongs to exactly one",
                    if native { "both" } else { "neither" }
                );
            }
            assert_eq!(
                native_statuses.len() + iced_statuses.len(),
                all.len(),
                "the two {what} lists must partition the statuses, naming each \
                 exactly once between them"
            );
        }
    }
}

#[cfg(feature = "widgets")]
#[test]
fn a_class_button_takes_its_hovered_and_pressed_states_from_iced() -> native_theme::Result<()> {
    // The model states a status color and its label and no variant of
    // either for a hovered or a pressed button, so those two states are
    // iced's own class, read at run time (spec section 3.3). That is a claim
    // about the emitted value, so it is asserted rather than left to the
    // rows, which cover only `CLASS_BUTTON_NATIVE_STATUSES`. The primary
    // button is iced's derivation from its own fill instead, asserted by
    // `a_primary_button_takes_its_hovered_and_pressed_states_from_its_own_fill`.
    let combinations = combinations()?;
    let mut failures = Vec::new();
    let mut checks = 0;

    for c in &combinations {
        for (what, ours, theirs) in [
            (
                "styles::button_danger",
                &styles::button_danger(&c.resolved)
                    as &dyn Fn(&Theme, button::Status) -> button::Style,
                button::danger as fn(&Theme, button::Status) -> button::Style,
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

#[cfg(feature = "iced_aw")]
#[test]
fn an_aw_widget_takes_the_statuses_it_never_receives_from_iced_aw() -> native_theme::Result<()> {
    // A tab bar, a sidebar and a selection list each read a subset of
    // `iced_aw`'s shared `Status`, and the model describes no tab, item or row
    // in the rest. Those are `iced_aw`'s own answer, read at run time (spec
    // section 3.2). That is a claim about the emitted value, so it is asserted
    // here rather than left to the rows, which cover only the native statuses.
    let combinations = combinations()?;
    let mut failures = Vec::new();
    let mut checks = 0;

    for c in &combinations {
        let (t, r) = (&c.theme, &c.resolved);
        for &status in AW_TAB_BAR_ICED_STATUSES {
            let ours = styles::aw::tab_bar(r)(t, status);
            let theirs = iced_aw::style::tab_bar::primary(t, status);
            for (field, emitted, iced) in [
                (
                    "tab_label_background",
                    fill(ours.tab_label_background),
                    fill(theirs.tab_label_background),
                ),
                ("text_color", Ok(ours.text_color), Ok(theirs.text_color)),
                ("icon_color", Ok(ours.icon_color), Ok(theirs.icon_color)),
            ] {
                checks += 1;
                if emitted != iced {
                    failures.push(format!(
                        "{}: styles::aw::tab_bar.{field} ({status:?}) is {emitted:?}, \
                         iced_aw's own class gives {iced:?}",
                        c.label()
                    ));
                }
            }
        }
        for &status in AW_SIDEBAR_ICED_STATUSES {
            let ours = styles::aw::sidebar(r)(t, status);
            let theirs = iced_aw::style::sidebar::primary(t, status);
            for (field, emitted, iced) in [
                (
                    "tab_label_background",
                    fill(ours.tab_label_background),
                    fill(theirs.tab_label_background),
                ),
                ("text_color", Ok(ours.text_color), Ok(theirs.text_color)),
                ("icon_color", Ok(ours.icon_color), Ok(theirs.icon_color)),
            ] {
                checks += 1;
                if emitted != iced {
                    failures.push(format!(
                        "{}: styles::aw::sidebar.{field} ({status:?}) is {emitted:?}, \
                         iced_aw's own class gives {iced:?}",
                        c.label()
                    ));
                }
            }
        }
        for &status in AW_SELECTION_LIST_ICED_STATUSES {
            let ours = styles::aw::selection_list(r)(t, status);
            let theirs = iced_aw::style::selection_list::primary(t, status);
            for (field, emitted, iced) in [
                ("background", fill(ours.background), fill(theirs.background)),
                ("text_color", Ok(ours.text_color), Ok(theirs.text_color)),
            ] {
                checks += 1;
                if emitted != iced {
                    failures.push(format!(
                        "{}: styles::aw::selection_list.{field} ({status:?}) is \
                         {emitted:?}, iced_aw's own class gives {iced:?}",
                        c.label()
                    ));
                }
            }
        }
    }

    println!("iced_aw widgets vs iced_aw's own classes: {checks} field checks");
    assert!(
        failures.is_empty(),
        "{} of {checks} unrequested iced_aw states differ from iced_aw's own \
         class:\n{}",
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
    // where the two cannot state an inset. iced's toggler gives the thumb
    // what its padding leaves inside the track (`toggler.rs:444-454`), so it
    // cannot draw a thumb that overhangs its track, as kde-breeze's does
    // (docs/platform-facts.md §2.21: qqc2-desktop-style's 18 handle on a 12
    // track; `native_theme_iced::switch` draws it). This counts the
    // combinations that take the native branch and requires every other one
    // to be such an overhang, so a preset that needs the fallback for another
    // reason says so.
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
        } else if !(s.track_height > 0.0 && s.thumb_diameter > s.track_height) {
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
        "{} of {} combinations cannot state a thumb inset and do not overhang \
         their track, so styles::toggler emits iced's ratio there instead of \
         the platform's:\n{}",
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

/// The statuses in which `styles::button_primary` paints what iced's own
/// `button::primary` paints from a primary slot holding the primary pair.
/// Disabled is `class_button`'s faded pair, which the rows cover.
#[cfg(feature = "widgets")]
const PRIMARY_AS_ICED_STATUSES: &[button::Status] = &[
    button::Status::Active,
    button::Status::Hovered,
    button::Status::Pressed,
];

/// The iced theme `to_theme` builds from `r` with its accent pair replaced by
/// the primary pair, so its palette's primary slot is
/// `button.primary_background` and that slot's label `.primary_text_color`:
/// the theme under which iced's own `button::primary` is the oracle for
/// `styles::button_primary`.
#[cfg(feature = "widgets")]
fn primary_slot_theme(r: &ResolvedTheme) -> Theme {
    let mut primary = r.clone();
    primary.defaults.accent_color = r.button.primary_background;
    primary.defaults.accent_text_color = r.button.primary_text_color;
    let theme = crate::to_theme(&primary, "primary slot");
    assert_eq!(
        theme.palette().primary,
        to_color(r.button.primary_background),
        "the oracle's primary slot must hold button.primary_background"
    );
    assert_eq!(
        theme.extended_palette().primary.base.text,
        to_color(r.button.primary_text_color),
        "the oracle's primary label must be button.primary_text_color"
    );
    theme
}

/// Where `styles::button_primary(r)` under `theme` differs from iced's own
/// `button::primary` under [`primary_slot_theme`], one line per field, and
/// how many fields were compared.
#[cfg(feature = "widgets")]
fn primary_vs_iced(what: &str, theme: &Theme, r: &ResolvedTheme) -> (Vec<String>, usize) {
    let oracle = primary_slot_theme(r);
    let style = styles::button_primary(r);
    let mut failures = Vec::new();
    let mut checks = 0;
    for &status in PRIMARY_AS_ICED_STATUSES {
        let emitted = style(theme, status);
        let iced = button::primary(&oracle, status);
        checks += 1;
        if emitted.background != iced.background {
            failures.push(format!(
                "{what}: styles::button_primary.background ({status:?}) is {:?}, \
                 button::primary from button.primary_background gives {:?}",
                emitted.background, iced.background
            ));
        }
        checks += 1;
        if emitted.text_color != iced.text_color {
            failures.push(format!(
                "{what}: styles::button_primary.text_color ({status:?}) is {}, \
                 button::primary from button.primary_text_color gives {}",
                show(emitted.text_color),
                show(iced.text_color)
            ));
        }
    }
    (failures, checks)
}

#[cfg(feature = "widgets")]
#[test]
fn a_primary_button_that_is_not_the_accent_hovers_from_its_own_fill() -> native_theme::Result<()> {
    // A live Windows theme states `AccentFillColorDefault` as the primary
    // fill and the system accent as `defaults.accent_color`, so the two differ
    // there. The hovered and pressed fills are iced's derivation from the
    // primary's own fill, not from the palette slot the accent fills, and the
    // label is the primary's own, not the accent's.
    // Dark, because there the danger label (`#000000e4`) is not the accent's
    // (`#000000`); in light both are white.
    let mut resolved = native_theme::theme::Theme::preset("windows-11")?
        .into_variant(ColorMode::Dark)?
        .into_resolved(&native_theme::ResolutionContext::for_tests())?;
    resolved.button.primary_background = resolved.defaults.danger_color;
    resolved.button.primary_text_color = resolved.defaults.danger_text_color;
    assert_ne!(
        resolved.button.primary_background, resolved.defaults.accent_color,
        "the primary fill must differ from the accent, or this proves nothing"
    );
    assert_ne!(
        resolved.button.primary_text_color, resolved.defaults.accent_text_color,
        "the primary label must differ from the accent's, or this proves nothing"
    );
    let theme = crate::to_theme(&resolved, "windows-11");

    let (failures, checks) = primary_vs_iced("windows-11/dark, edited", &theme, &resolved);
    assert!(
        failures.is_empty(),
        "{} of {checks} primary-button fields differ from iced's own class \
         painted from the primary pair:\n{}",
        failures.len(),
        failures.join("\n")
    );
    assert_ne!(
        styles::button_primary(&resolved)(&theme, button::Status::Hovered).background,
        button::primary(&theme, button::Status::Hovered).background,
        "the hovered fill must not be iced's strengthened accent"
    );
    Ok(())
}

#[cfg(feature = "widgets")]
#[test]
fn a_primary_button_takes_its_hovered_and_pressed_states_from_its_own_fill()
-> native_theme::Result<()> {
    // The model states no hovered and no pressed primary fill, so those two
    // states are iced's derivation (spec section 3.3) -- applied to the
    // primary pair, `button.primary_background` and `.primary_text_color`,
    // not to `palette.primary`, which carries `defaults.accent_color`. The
    // oracle is iced's own `button::primary` under a theme whose primary slot
    // holds that pair, at rest as well. The rows cover only
    // `CLASS_BUTTON_NATIVE_STATUSES`, so the claim is asserted here.
    let combinations = combinations()?;
    let mut failures = Vec::new();
    let mut checks = 0;

    for c in &combinations {
        let (failed, compared) = primary_vs_iced(&c.label(), &c.theme, &c.resolved);
        failures.extend(failed);
        checks += compared;
    }

    println!("primary button vs iced's own class from the primary pair: {checks} field checks");
    assert!(
        failures.is_empty(),
        "{} of {checks} primary-button fields differ from iced's own class \
         painted from the primary pair:\n{}",
        failures.len(),
        failures.join("\n")
    );
    Ok(())
}

/// The `Scrollbar` the native fields ask for: two widths, and a spacing where
/// the platform's scrollbar is not an overlay.
///
/// Built here independently of `styles::scrollbar`, as a row's `native` side
/// is built independently of its `get`, and following section 3.3's rule
/// rather than copying the function's expression.
#[cfg(feature = "widgets")]
fn native_scrollbar(r: &ResolvedTheme) -> scrollable::Scrollbar {
    let s = &r.scrollbar;
    let bar = scrollable::Scrollbar::new()
        .width(s.groove_width)
        .scroller_width(s.thumb_width);
    if s.overlay_mode {
        bar
    } else {
        bar.spacing(0.0)
    }
}

/// `styles::scrollbar` is the platform's, as far as iced 0.14 lets it be read.
///
/// There is no row for this function, and not for want of trying:
/// `scrollable::Scrollbar` keeps all five of its fields private and offers no
/// getters (`scrollable.rs:322-328`), so a `StyleRow` -- whose `get` must
/// *read* the emitted value -- cannot be written. The one handle iced leaves
/// is the `PartialEq` it derives, so the expected value is built by
/// [`native_scrollbar`] from the resolved theme and the two values are
/// compared whole. `Debug` appears only in the failure message; it is not what
/// is asserted.
///
/// That comparison catches a wrong width, a wrong overlay mode, and the two
/// width setters being swapped -- but the swap only where the two lengths
/// differ, so the combinations that can see one are counted and printed, and
/// the day every preset states a single width the test says so instead of
/// quietly asserting less. The same goes for the overlay flag: how many
/// combinations take each branch is printed, and
/// `a_scrollbar_that_is_not_an_overlay_is_embedded` drives both branches from
/// one preset whatever the presets happen to state.
#[cfg(feature = "widgets")]
#[test]
fn the_scrollbar_is_the_platforms() -> native_theme::Result<()> {
    let combinations = combinations()?;
    let mut failures = Vec::new();
    let mut distinguishing = Vec::new();
    let mut embedded = Vec::new();
    let mut floating = Vec::new();

    for c in &combinations {
        let s = &c.resolved.scrollbar;
        let expected = native_scrollbar(&c.resolved);
        let actual = styles::scrollbar(&c.resolved);
        if actual != expected {
            failures.push(format!(
                "{}: {actual:?}, native gives {expected:?}",
                c.label()
            ));
        }

        let swapped = scrollable::Scrollbar::new()
            .width(s.thumb_width)
            .scroller_width(s.groove_width);
        if swapped != expected {
            distinguishing.push(format!(
                "{}: groove {} thumb {}",
                c.label(),
                s.groove_width,
                s.thumb_width
            ));
        }

        if s.overlay_mode {
            floating.push(c.label());
        } else {
            embedded.push(c.label());
        }
    }

    println!(
        "scrollbar widths: {} of {} combinations state a groove and a thumb of \
         different widths, which is what makes the two setters \
         distinguishable:\n{}",
        distinguishing.len(),
        combinations.len(),
        distinguishing.join("\n")
    );
    println!(
        "scrollbar overlay mode: {} of {} combinations float over the contents \
         and take iced's default, {} are embedded and take a spacing:\n  \
         floating: {}\n  embedded: {}",
        floating.len(),
        combinations.len(),
        embedded.len(),
        floating.join(", "),
        embedded.join(", ")
    );
    assert!(
        !distinguishing.is_empty(),
        "every combination now states one width for both, so this test can no \
         longer tell `width` from `scroller_width`"
    );
    assert!(
        failures.is_empty(),
        "{} of {} scrollbars are not the platform's:\n{}",
        failures.len(),
        combinations.len(),
        failures.join("\n")
    );
    Ok(())
}

/// Both branches of the overlay flag, whatever the presets state.
///
/// The bundled presets may all agree on `overlay_mode`, and then the loop
/// above exercises one branch and a function that ignored the flag would pass
/// it. So one resolved theme is read, its flag asserted, then flipped, and the
/// two answers are required to differ from each other and to be exactly what
/// the rule gives: iced's own `Scrollbar` while it floats, and that same
/// scrollbar with a spacing once it does not.
#[cfg(feature = "widgets")]
#[test]
fn a_scrollbar_that_is_not_an_overlay_is_embedded() -> native_theme::Result<()> {
    let mut resolved = native_theme::theme::Theme::preset("windows-11")?
        .into_variant(ColorMode::Light)?
        .into_resolved(&native_theme::ResolutionContext::for_tests())?;

    let floating = scrollable::Scrollbar::new()
        .width(resolved.scrollbar.groove_width)
        .scroller_width(resolved.scrollbar.thumb_width);
    let embedded = floating.spacing(0.0);
    assert_ne!(
        floating, embedded,
        "a spacing no longer changes a `Scrollbar`, so this test proves nothing"
    );

    resolved.scrollbar.overlay_mode = true;
    assert_eq!(
        styles::scrollbar(&resolved),
        floating,
        "an overlay scrollbar keeps iced's floating default, which is no \
         spacing at all"
    );

    resolved.scrollbar.overlay_mode = false;
    assert_eq!(
        styles::scrollbar(&resolved),
        embedded,
        "a scrollbar the platform does not overlay is embedded, which iced \
         spells as a spacing (`scrollable.rs:378-383`)"
    );
    Ok(())
}

/// M9x CI item I5 (M9-174): a floating scrollbar is hidden at rest. iced
/// 0.14 has no auto-hide and draws a floating bar in every status, its
/// style asked for `Dragged`, `Hovered` or `Active` alone (iced_widget 0.14.2
/// `src/scrollable.rs:1097-1121`, `:1176-1182`), so where the theme says the bar floats
/// (`scrollbar.overlay_mode`, macOS's and libadwaita's), both rails and their
/// scrollers are transparent while the scrollable is `Active` and shown, in
/// `scrollbar.track_color` and the scroller's colours, once the pointer is
/// over it (`Hovered`) or drags it: iced reveals on hover, not on scroll. A
/// bar that does not float is shown in every status.
#[cfg(feature = "widgets")]
#[test]
fn a_floating_scrollbar_is_hidden_at_rest() -> native_theme::Result<()> {
    let mut resolved = native_theme::theme::Theme::preset("macos-tahoe")?
        .into_variant(ColorMode::Light)?
        .into_resolved(&native_theme::ResolutionContext::for_tests())?;
    assert!(
        resolved.scrollbar.overlay_mode,
        "macos-tahoe's scrollbar no longer floats, so this test proves nothing"
    );
    let theme = crate::to_theme(&resolved, "macos-tahoe");
    let alpha = |b: Option<Background>| match b {
        Some(Background::Color(c)) => c.a,
        _ => f32::NAN,
    };
    let at_rest = styles::scrollable(&resolved)(&theme, scrollable_active(false, false));
    for (what, a) in [
        ("vertical rail", alpha(at_rest.vertical_rail.background)),
        (
            "vertical scroller",
            alpha(Some(at_rest.vertical_rail.scroller.background)),
        ),
        ("horizontal rail", alpha(at_rest.horizontal_rail.background)),
        (
            "horizontal scroller",
            alpha(Some(at_rest.horizontal_rail.scroller.background)),
        ),
    ] {
        assert_eq!(a, 0.0, "a floating bar at rest: the {what} shows");
    }
    let hovered =
        styles::scrollable(&resolved)(&theme, scrollable_hovered(false, true, false, false));
    assert_eq!(
        hovered.vertical_rail.scroller.background,
        Background::Color(to_color(resolved.scrollbar.thumb_hover_color)),
        "a floating bar under the pointer shows its scroller"
    );
    assert_eq!(
        hovered.horizontal_rail.background,
        Some(Background::Color(to_color(resolved.scrollbar.track_color))),
        "a floating bar under the pointer shows its rails"
    );
    resolved.scrollbar.overlay_mode = false;
    let embedded = styles::scrollable(&resolved)(&theme, scrollable_active(false, false));
    assert_eq!(
        embedded.vertical_rail.scroller.background,
        Background::Color(to_color(resolved.scrollbar.thumb_color)),
        "a bar that does not float shows at rest"
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
    check_style_rows(SCROLLABLE_ROWS, &combinations, &mut failures, &mut ran);
    check_style_rows(PROGRESS_BAR_ROWS, &combinations, &mut failures, &mut ran);
    check_border_rows(
        PROGRESS_BAR_BORDER_ROWS,
        &combinations,
        &mut failures,
        &mut ran,
    );
    check_style_rows(RULE_ROWS, &combinations, &mut failures, &mut ran);
    check_style_rows(PANE_GRID_ROWS, &combinations, &mut failures, &mut ran);
    check_scalar_rows(
        PANE_GRID_SCALAR_ROWS,
        &combinations,
        &mut failures,
        &mut ran,
    );
    check_style_rows(TOOLTIP_ROWS, &combinations, &mut failures, &mut ran);
    check_style_rows(APPLICATION_ROWS, &combinations, &mut failures, &mut ran);
    check_style_rows(DIALOG_ROWS, &combinations, &mut failures, &mut ran);
    check_style_rows(TOOLBAR_ROWS, &combinations, &mut failures, &mut ran);
    check_style_rows(TOOLBAR_RULE_ROWS, &combinations, &mut failures, &mut ran);
    check_style_rows(
        SEGMENT_SEPARATOR_ROWS,
        &combinations,
        &mut failures,
        &mut ran,
    );
    check_border_rows(DIALOG_BORDER_ROWS, &combinations, &mut failures, &mut ran);
    check_border_rows(TOOLTIP_BORDER_ROWS, &combinations, &mut failures, &mut ran);
    check_style_rows(CONTAINER_CARD_ROWS, &combinations, &mut failures, &mut ran);
    check_border_rows(
        CONTAINER_CARD_BORDER_ROWS,
        &combinations,
        &mut failures,
        &mut ran,
    );
    check_style_rows(
        SEGMENTED_CONTROL_ROWS,
        &combinations,
        &mut failures,
        &mut ran,
    );
    check_border_rows(
        SEGMENTED_CONTROL_BORDER_ROWS,
        &combinations,
        &mut failures,
        &mut ran,
    );
    check_style_rows(SEGMENT_ROWS, &combinations, &mut failures, &mut ran);
    check_scalar_rows(SEGMENT_SCALAR_ROWS, &combinations, &mut failures, &mut ran);
    check_style_rows(EXPANDER_ROWS, &combinations, &mut failures, &mut ran);
    check_border_rows(EXPANDER_BORDER_ROWS, &combinations, &mut failures, &mut ran);
    check_border_rows(
        EXPANDER_FRAME_BORDER_ROWS,
        &combinations,
        &mut failures,
        &mut ran,
    );
    check_style_rows(BUTTON_TOGGLE_ROWS, &combinations, &mut failures, &mut ran);
    check_border_rows(
        BUTTON_TOGGLE_BORDER_ROWS,
        &combinations,
        &mut failures,
        &mut ran,
    );
    #[cfg(feature = "iced_aw")]
    {
        check_style_rows(AW_CARD_ROWS, &combinations, &mut failures, &mut ran);
        check_scalar_rows(AW_CARD_SCALAR_ROWS, &combinations, &mut failures, &mut ran);
        check_style_rows(AW_MENU_ROWS, &combinations, &mut failures, &mut ran);
        check_border_rows(AW_MENU_BORDER_ROWS, &combinations, &mut failures, &mut ran);
        check_style_rows(AW_TAB_BAR_ROWS, &combinations, &mut failures, &mut ran);
        check_scalar_rows(
            AW_TAB_BAR_SCALAR_ROWS,
            &combinations,
            &mut failures,
            &mut ran,
        );
        check_radius_rows(
            AW_TAB_BAR_RADIUS_ROWS,
            &combinations,
            &mut failures,
            &mut ran,
        );
        check_style_rows(AW_SIDEBAR_ROWS, &combinations, &mut failures, &mut ran);
        check_scalar_rows(
            AW_SIDEBAR_SCALAR_ROWS,
            &combinations,
            &mut failures,
            &mut ran,
        );
        check_style_rows(
            AW_SELECTION_LIST_ROWS,
            &combinations,
            &mut failures,
            &mut ran,
        );
        check_scalar_rows(
            AW_SELECTION_LIST_SCALAR_ROWS,
            &combinations,
            &mut failures,
            &mut ran,
        );
        check_style_rows(AW_SPINNER_ROWS, &combinations, &mut failures, &mut ran);
    }

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
    check_style_pairs(
        SCROLLABLE_PAIRS,
        all,
        &mut failures,
        &mut below_aa,
        &mut ran,
    );
    check_style_pairs(
        PROGRESS_BAR_PAIRS,
        all,
        &mut failures,
        &mut below_aa,
        &mut ran,
    );
    check_style_pairs(TOOLTIP_PAIRS, all, &mut failures, &mut below_aa, &mut ran);
    check_style_pairs(SEGMENT_PAIRS, all, &mut failures, &mut below_aa, &mut ran);
    check_style_pairs(EXPANDER_PAIRS, all, &mut failures, &mut below_aa, &mut ran);
    check_style_pairs(
        BUTTON_TOGGLE_PAIRS,
        all,
        &mut failures,
        &mut below_aa,
        &mut ran,
    );
    #[cfg(feature = "iced_aw")]
    {
        check_style_pairs(AW_CARD_PAIRS, all, &mut failures, &mut below_aa, &mut ran);
        check_style_pairs(
            AW_TAB_BAR_PAIRS,
            all,
            &mut failures,
            &mut below_aa,
            &mut ran,
        );
        check_style_pairs(
            AW_SIDEBAR_PAIRS,
            all,
            &mut failures,
            &mut below_aa,
            &mut ran,
        );
        check_style_pairs(
            AW_SELECTION_LIST_PAIRS,
            all,
            &mut failures,
            &mut below_aa,
            &mut ran,
        );
        check_style_pairs(
            AW_SPINNER_PAIRS,
            all,
            &mut failures,
            &mut below_aa,
            &mut ran,
        );
    }

    // A pair list declared and never checked here asserts nothing, and the
    // tripwire cannot notice -- it walks `Style` fields, and whether a pair
    // exists at all is a judgment per function (section 7). So the names that
    // ran are held against the names declared.
    ran.covers(&style_pair_names(), "style_pair_names()");

    println!(
        "--- styles contrast: {} pairs, {} below AA in {} lines (printed, not \
         asserted; a line carries every status of one pair and combination \
         that reads alike, named while there are at most four of them; an \
         [indicator] line is a non-text pair, where WCAG asks 3:1) ---",
        ran.checks,
        ran.below_aa,
        below_aa.len()
    );
    for line in &below_aa {
        println!("{line}");
    }

    // The disabled checked checkbox's mark on its fill, the range docs/todo.md
    // quotes for the platforms' own disabled pair (printed, not asserted).
    let disabled = checkbox::Status::Disabled { is_checked: true };
    let mut range: Option<(f32, f32)> = None;
    let mut measured = 0usize;
    for pair in CHECKBOX_PAIRS.iter().filter(|p| p.what == "checkbox mark") {
        for c in &combinations {
            if let Ok((fg, bg, surface)) = (pair.emitted)(&c.theme, &c.resolved, disabled) {
                let ratio = pair_ratio(fg, bg, surface);
                measured += 1;
                range =
                    Some(range.map_or((ratio, ratio), |(lo, hi)| (lo.min(ratio), hi.max(ratio))));
            }
        }
    }
    if let Some((lo, hi)) = range {
        println!(
            "disabled checked checkbox mark on its fill: {lo:.2}-{hi:.2}:1 over {measured} of {} \
             combinations",
            combinations.len()
        );
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
    // A translucent hover layer shows the difference between layering and
    // replacing (section 3.2). No bundled preset states one since M2b Step 7b
    // (windows-11 states WinUI's replacing fill flattened,
    // native-theme/src/presets/windows-11.toml:127), so the fixture is
    // windows-11 light with that brush as the resource gives it,
    // `ControlFillColorSecondary` `#80F9F9F9` (docs/platform-facts.md:1192).
    let mut resolved = native_theme::theme::Theme::preset("windows-11")?
        .into_variant(ColorMode::Light)?
        .into_resolved(&native_theme::ResolutionContext::for_tests())?;
    resolved.button.hover_background = Rgba::new(0xf9, 0xf9, 0xf9, 0x80);
    let theme = crate::to_theme(&resolved, "windows-11");

    let raw = to_color(resolved.button.hover_background);
    let expected = over(raw, to_color(resolved.button.background_color));
    assert!(
        raw.a < 1.0 && expected != raw,
        "the fixture's hover layer, ControlFillColorSecondary #80F9F9F9 \
         set above, no longer differs from its composite over windows-11's \
         button fill, so it no longer tests compositing: raw {}, composited {}",
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
fn a_checked_toggle_button_paints_the_checked_pair_or_the_pressed_one() -> native_theme::Result<()>
{
    // windows-11 states WinUI's checked ToggleButton as the accent fill,
    // #0067c0 with #ffffff (native-theme/src/presets/windows-11.toml:147-148).
    let mut resolved = native_theme::theme::Theme::preset("windows-11")?
        .into_variant(ColorMode::Light)?
        .into_resolved(&native_theme::ResolutionContext::for_tests())?;
    assert_eq!(
        (
            resolved.button.checked_background,
            resolved.button.checked_text_color
        ),
        (
            Some(Rgba::new(0x00, 0x67, 0xc0, 0xff)),
            Some(Rgba::new(0xff, 0xff, 0xff, 0xff))
        ),
        "windows-11 no longer states WinUI's checked pair"
    );
    let theme = crate::to_theme(&resolved, "windows-11");
    let plain = styles::button(&resolved);
    let on = styles::button_toggle(&resolved, true);
    let off = styles::button_toggle(&resolved, false);
    for status in [
        button::Status::Active,
        button::Status::Hovered,
        button::Status::Pressed,
    ] {
        let style = on(&theme, status);
        assert_eq!(
            (flat(style.background), style.text_color),
            (
                Ok(to_color(Rgba::new(0x00, 0x67, 0xc0, 0xff))),
                to_color(Rgba::new(0xff, 0xff, 0xff, 0xff))
            ),
            "a checked toggle paints button.checked_background and \
             .checked_text_color ({status:?})"
        );
    }
    for &status in BUTTON_STATUSES {
        assert_eq!(
            off(&theme, status),
            plain(&theme, status),
            "a toggle that is off is the plain button ({status:?})"
        );
    }

    // Unstated, a checked toggle is the pressed pair
    // (docs/inheritance-rules.toml, "the pressed pair shows a checked toggle").
    resolved.button.checked_background = None;
    resolved.button.checked_text_color = None;
    let theme = crate::to_theme(&resolved, "windows-11");
    let pressed = styles::button(&resolved)(&theme, button::Status::Pressed);
    let on = styles::button_toggle(&resolved, true);
    for status in [
        button::Status::Active,
        button::Status::Hovered,
        button::Status::Pressed,
    ] {
        let style = on(&theme, status);
        assert_eq!(
            (style.background, style.text_color),
            (pressed.background, pressed.text_color),
            "an unstated checked pair falls back to the pressed one ({status:?})"
        );
    }
    Ok(())
}

#[cfg(feature = "widgets")]
#[test]
fn a_disabled_expander_header_fades_by_the_defaults_opacity() -> native_theme::Result<()> {
    // `ExpanderTheme` states no `disabled_opacity`, so a disabled header
    // fades by `defaults.disabled_opacity` (docs/inheritance-rules.toml,
    // the disabled-state note), which adwaita states as 0.5.
    let resolved = native_theme::theme::Theme::preset("adwaita")?
        .into_variant(ColorMode::Light)?
        .into_resolved(&native_theme::ResolutionContext::for_tests())?;
    let opacity = resolved.defaults.disabled_opacity;
    assert!(
        opacity < 1.0,
        "adwaita no longer fades a disabled widget, so this test proves nothing"
    );
    let theme = crate::to_theme(&resolved, "adwaita");
    let style = styles::expander(&resolved)(&theme, button::Status::Disabled);
    assert_eq!(
        style.text_color,
        dim(to_color(resolved.expander.font.color), opacity),
        "a disabled header's label is expander.font.color faded by \
         defaults.disabled_opacity"
    );
    assert_eq!(
        style.border.color,
        dim(to_color(resolved.expander.border.color), opacity),
        "a disabled header's outline is faded by defaults.disabled_opacity"
    );
    Ok(())
}

#[cfg(feature = "widgets")]
#[test]
fn an_expander_that_states_its_frame_draws_no_header_border() -> native_theme::Result<()> {
    // kde-breeze states `frame_enabled = false`: KCollapsibleGroupBox paints
    // the arrow and the title and nothing else (docs/platform-facts.md
    // §2.27), so the header has no outline.
    let resolved = native_theme::theme::Theme::preset("kde-breeze")?
        .into_variant(ColorMode::Light)?
        .into_resolved(&native_theme::ResolutionContext::for_tests())?;
    assert_eq!(
        resolved.expander.frame_enabled,
        Some(false),
        "kde-breeze no longer states an unframed expander"
    );
    assert!(
        resolved.expander.border.line_width > 0.0,
        "kde-breeze's expander border has no width, so this test proves nothing"
    );
    let theme = crate::to_theme(&resolved, "kde-breeze");
    let header = styles::expander(&resolved);
    for &status in BUTTON_STATUSES {
        assert_eq!(
            header(&theme, status).border.width,
            0.0,
            "an expander whose frame the theme states draws no header border \
             ({status:?})"
        );
    }
    Ok(())
}

/// M9x CI item I7 (M9-176): where the theme does not state whether the
/// expander is framed (macos-tahoe, whose facts give `frame_enabled` as
/// "(none)", docs/platform-facts.md §2.27), the header draws no outline, the
/// shape KDE's and egui's expanders have: an outline round the header alone
/// is no platform's.
#[cfg(feature = "widgets")]
#[test]
fn an_expander_with_no_stated_frame_draws_no_header_border() -> native_theme::Result<()> {
    let resolved = native_theme::theme::Theme::preset("macos-tahoe")?
        .into_variant(ColorMode::Light)?
        .into_resolved(&native_theme::ResolutionContext::for_tests())?;
    assert_eq!(
        resolved.expander.frame_enabled, None,
        "macos-tahoe states whether its expander is framed, so this test proves nothing"
    );
    assert!(
        resolved.expander.border.line_width > 0.0,
        "macos-tahoe's expander border has no width, so this test proves nothing"
    );
    let theme = crate::to_theme(&resolved, "macos-tahoe");
    let header = styles::expander(&resolved);
    for &status in BUTTON_STATUSES {
        assert_eq!(
            header(&theme, status).border.width,
            0.0,
            "an expander with no stated frame outlines its header ({status:?})"
        );
    }
    Ok(())
}

#[cfg(feature = "widgets")]
#[test]
fn a_hovered_split_is_the_splitters_hover_colour() -> native_theme::Result<()> {
    // adwaita states `splitter.hover_color`, a colour iced's own pane grid
    // does not paint: its hovered split is the palette's primary.
    let resolved = native_theme::theme::Theme::preset("adwaita")?
        .into_variant(ColorMode::Light)?
        .into_resolved(&native_theme::ResolutionContext::for_tests())?;
    let theme = crate::to_theme(&resolved, "adwaita");
    let hover = to_color(resolved.splitter.hover_color);
    assert_ne!(
        iced_widget::pane_grid::default(&theme).hovered_split.color,
        hover,
        "adwaita's splitter hover is iced's own hovered split, so this test \
         proves nothing"
    );
    assert_eq!(
        styles::pane_grid(&resolved)(&theme).hovered_split.color,
        hover,
        "a hovered split is splitter.hover_color"
    );
    Ok(())
}

#[cfg(feature = "widgets")]
#[test]
fn a_text_editor_is_framed_by_the_text_areas_border() -> native_theme::Result<()> {
    // Every bundled preset frames its text area as its input
    // (`text_area.border` inherits `input.border`), and every one states a
    // 1 px input border, so the text area here takes another real border of
    // another preset: material-3's check box outline, 2 px wide.
    let mut resolved = native_theme::theme::Theme::preset("adwaita")?
        .into_variant(ColorMode::Light)?
        .into_resolved(&native_theme::ResolutionContext::for_tests())?;
    let donor = native_theme::theme::Theme::preset("material-3")?
        .into_variant(ColorMode::Light)?
        .into_resolved(&native_theme::ResolutionContext::for_tests())?;
    let (own, other) = (&resolved.input.border, &donor.checkbox.border);
    assert!(
        own.color != other.color
            && own.line_width != other.line_width
            && own.corner_radius != other.corner_radius,
        "material-3's check box border no longer differs from adwaita's input \
         border in colour, width and radius, so this test proves less than it \
         says"
    );
    resolved.text_area.border.color = other.color;
    resolved.text_area.border.line_width = other.line_width;
    resolved.text_area.border.corner_radius = other.corner_radius;
    let theme = crate::to_theme(&resolved, "adwaita");

    let border = styles::text_editor(&resolved)(&theme, text_editor::Status::Active).border;
    assert_eq!(
        (border.color, border.width, border.radius),
        (
            to_color(other.color),
            other.line_width,
            Radius::new(other.corner_radius)
        ),
        "a text editor is framed by text_area.border, not input.border"
    );
    Ok(())
}

/// adwaita states no border on a text input at rest (`input.border.line_width`
/// 0, `docs/platform-facts.md` §2.4) and an inset focus ring, 2 px at offset
/// -2 (§2.1.5): a focused field still shows an edge, the ring drawn as a
/// border inside the field in `input.focus_border_color` laid over the
/// field's fill (iced's quad puts a border in place of the fill).
#[cfg(feature = "widgets")]
#[test]
fn a_focused_borderless_input_draws_the_inset_focus_ring() -> native_theme::Result<()> {
    for mode in [ColorMode::Light, ColorMode::Dark] {
        let resolved = native_theme::theme::Theme::preset("adwaita")?
            .into_variant(mode)?
            .into_resolved(&native_theme::ResolutionContext::for_tests())?;
        assert_eq!(resolved.input.border.line_width, 0.0, "adwaita {mode:?}");
        let d = &resolved.defaults;
        assert_eq!(d.focus_ring_offset, -d.focus_ring_width, "adwaita {mode:?}");
        // The translucent ring laid over the field's fill, as libadwaita
        // draws it (and egui): #75a4d2 / #5d849e, each channel within the
        // 8-bit rounding of the ring's 0x80 alpha.
        let ring = styles::composite_over(
            to_color(
                resolved
                    .input
                    .focus_border_color
                    .unwrap_or(d.focus_ring_color),
            ),
            to_color(resolved.input.background_color),
        );
        let drawn = if mode == ColorMode::Dark {
            [0x5d_u8, 0x84, 0x9e]
        } else {
            [0x75, 0xa4, 0xd2]
        };
        let rgb = [ring.r, ring.g, ring.b].map(|c| (c * 255.0).round());
        for (got, want) in rgb.iter().zip(drawn) {
            assert!(
                (got - f32::from(want)).abs() <= 1.0,
                "adwaita {mode:?}: the ring over the fill is {rgb:?}, libadwaita's {drawn:x?}"
            );
        }
        assert_eq!(ring.a, 1.0, "adwaita {mode:?}: the field's fill is opaque");
        let theme = crate::to_theme(&resolved, "adwaita");
        let focused = text_input::Status::Focused { is_hovered: false };
        let input = styles::text_input(&resolved)(&theme, focused).border;
        assert_eq!(
            (input.width, input.color),
            (d.focus_ring_width, ring),
            "adwaita {mode:?}: a focused text input has no visible edge"
        );
        let editor = styles::text_editor(&resolved)(
            &theme,
            iced_widget::text_editor::Status::Focused { is_hovered: false },
        )
        .border;
        assert_eq!(
            (editor.width, editor.color),
            (d.focus_ring_width, ring),
            "adwaita {mode:?}: a focused text area has no visible edge"
        );
    }
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
            to_color(resolved.text_area.border.color),
            "the editor reads the same soft options, over its own border \
             ({status:?})"
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
    // The twelve soft options the widget functions read: four `checkbox.*`
    // (three of which `styles::radio` reads again, through the fills it
    // shares with `styles::checkbox_over`), five `switch.*`, one
    // `combo_box.*`, one `slider.*` and one `scrollbar.*`. A bundled preset
    // rarely leaves them `None` after resolution, so the rows cannot tell a
    // right base-state field from a wrong one; the fallbacks are reached by
    // clearing them here (section 3.2).
    // windows-11 states nine of them; the switch's disabled thumb, the
    // slider's thumb hover and the scroll bar's pressed thumb have no source
    // and are already `None`.
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
            && resolved.combo_box.hover_background.is_some(),
        "this preset no longer states the nine soft options it sources, so \
         clearing them proves nothing"
    );
    // The thirteenth belongs to `styles::aw::tab_bar` and is read only when
    // that feature is on.
    #[cfg(feature = "iced_aw")]
    assert!(
        resolved.tab.hover_background.is_some(),
        "this preset no longer states tab.hover_background, so clearing it \
         proves nothing"
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
    resolved.scrollbar.thumb_active_color = None;
    #[cfg(feature = "iced_aw")]
    {
        resolved.tab.hover_background = None;
    }

    let theme = crate::to_theme(&resolved, "windows-11");
    let boxes = styles::checkbox(&resolved);
    let buttons = styles::radio(&resolved);
    let switches = styles::toggler(&resolved);
    let fields = styles::pick_list(&resolved);
    let rails = styles::slider(&resolved);
    let scrollbars = styles::scrollable(&resolved);

    // Every base-state value of section 3.2's table, read straight from the
    // model. A cleared hover layer is the fill it would cover, at rest.
    let box_fill = to_color(resolved.checkbox.background_color);
    let box_border = to_color(resolved.checkbox.border.color);
    let track_on = to_color(resolved.switch.checked_background);
    let track_off = to_color(resolved.switch.unchecked_background);
    let thumb = to_color(resolved.switch.thumb_background);
    let field_fill = to_color(resolved.combo_box.background_color);
    let handle = to_color(resolved.slider.thumb_color);
    let hovered_scroller = to_color(resolved.scrollbar.thumb_hover_color);
    #[cfg(feature = "iced_aw")]
    let tabs = styles::aw::tab_bar(&resolved);
    #[cfg(feature = "iced_aw")]
    let tab_fill = to_color(resolved.tab.background_color);

    // One entry per read site, not per model field: two functions reading the
    // same option have two read expressions.
    let read_sites = [
        (
            "checkbox.unchecked_background -> checkbox.background_color",
            fill(boxes(&theme, checkbox::Status::Active { is_checked: false }).background),
            box_fill,
        ),
        (
            "checkbox.hover_background -> the unchecked fill at rest",
            fill(boxes(&theme, checkbox::Status::Hovered { is_checked: false }).background),
            box_fill,
        ),
        // No disabled fill stated is the platform dimming by opacity alone
        // (docs/platform-facts.md §2.1.6): the box is its enabled self, the
        // checked fill here; windows-11's `disabled_opacity` is 1.0.
        (
            "checkbox.disabled_background -> checkbox.checked_background",
            fill(boxes(&theme, checkbox::Status::Disabled { is_checked: true }).background),
            to_color(resolved.checkbox.checked_background),
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
            "radio: checkbox.hover_background -> the unselected fill at rest",
            fill(buttons(&theme, radio::Status::Hovered { is_selected: false }).background),
            box_fill,
        ),
        (
            "radio: checkbox.unchecked_border_color -> checkbox.border.color",
            Ok(buttons(&theme, radio::Status::Active { is_selected: false }).border_color),
            box_border,
        ),
        (
            "switch.hover_checked_background -> switch.checked_background",
            fill(switches(&theme, toggler::Status::Hovered { is_toggled: true }).background),
            track_on,
        ),
        (
            "switch.hover_unchecked_background -> switch.unchecked_background",
            fill(switches(&theme, toggler::Status::Hovered { is_toggled: false }).background),
            track_off,
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
            field_fill,
        ),
        (
            "slider.thumb_hover_color -> slider.thumb_color",
            fill(rails(&theme, slider::Status::Hovered).handle.background),
            handle,
        ),
        (
            "scrollbar.thumb_active_color -> scrollbar.thumb_hover_color",
            fill(
                scrollbars(&theme, scrollable_dragged(false, true, false, false))
                    .vertical_rail
                    .scroller
                    .background,
            ),
            hovered_scroller,
        ),
        // A hovered tab is a row highlight, emitted as given rather than
        // composited, so a cleared option is a plain copy of the idle fill.
        #[cfg(feature = "iced_aw")]
        (
            "tab.hover_background -> tab.background_color",
            fill(tabs(&theme, AwStatus::Hovered).tab_label_background),
            tab_fill,
        ),
    ];

    let sites = read_sites.len();
    let mut failures = Vec::new();
    for (soft_option, actual, expected) in read_sites {
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
        "{} of {sites} cleared soft options do not copy their base-state \
         value:\n{}",
        failures.len(),
        failures.join("\n")
    );
    Ok(())
}

#[cfg(feature = "widgets")]
#[test]
fn an_unset_hover_soft_option_is_the_idle_fill() -> native_theme::Result<()> {
    // macos-tahoe states a translucent drop-down field and a translucent off
    // track, and no hover for either: a hovered one is the one at rest, not
    // its fill composited over itself, which would double its alpha. The
    // checkbox, the radio and the segment read the same rule, and are given
    // the field's translucent fill and no hover here, since no bundled preset
    // leaves theirs unset over a translucent fill; the unchecked box's fill is
    // set apart from the checkbox's own, so that a hover falling back onto
    // the latter is caught too.
    for mode in [ColorMode::Light, ColorMode::Dark] {
        let mut resolved = native_theme::theme::Theme::preset("macos-tahoe")?
            .into_variant(mode)?
            .into_resolved(&native_theme::ResolutionContext::for_tests())?;
        let translucent = resolved.combo_box.background_color;
        assert!(
            resolved.combo_box.hover_background.is_none()
                && translucent.a < 255
                && resolved.switch.hover_unchecked_background.is_none()
                && resolved.switch.unchecked_background.a < 255
                && resolved.checkbox.background_color != translucent,
            "macos-tahoe ({mode:?}) no longer leaves a translucent field's or \
             off track's hover unset, so this proves nothing"
        );
        resolved.checkbox.hover_background = None;
        resolved.checkbox.unchecked_background = Some(translucent);
        resolved.segmented_control.hover_background = None;
        resolved.segmented_control.background_color = translucent;

        let theme = crate::to_theme(&resolved, "macos-tahoe");
        let idle = to_color(translucent);
        assert_eq!(
            styles::pick_list(&resolved)(&theme, pick_list::Status::Hovered).background,
            Background::Color(idle),
            "macos-tahoe ({mode:?}): a hovered field with no hover stated is \
             the field at rest"
        );
        assert_eq!(
            styles::toggler(&resolved)(&theme, toggler::Status::Hovered { is_toggled: false })
                .background,
            Background::Color(to_color(resolved.switch.unchecked_background)),
            "macos-tahoe ({mode:?}): a hovered off track with no hover stated is \
             the track at rest"
        );
        assert_eq!(
            styles::checkbox(&resolved)(&theme, checkbox::Status::Hovered { is_checked: false })
                .background,
            Background::Color(idle),
            "macos-tahoe ({mode:?}): a hovered unchecked box with no hover \
             stated is the unchecked box"
        );
        assert_eq!(
            styles::radio(&resolved)(&theme, radio::Status::Hovered { is_selected: false })
                .background,
            Background::Color(idle),
            "macos-tahoe ({mode:?}): a hovered unselected radio with no hover \
             stated is the unselected radio"
        );
        let segment = styles::segment(&resolved, false, styles::SegmentPosition::Only);
        assert_eq!(
            segment(&theme, button::Status::Hovered).background,
            segment(&theme, button::Status::Active).background,
            "macos-tahoe ({mode:?}): a hovered segment with no hover stated is \
             the segment at rest"
        );
        assert_eq!(
            styles::segmented_control(&resolved, false)(&theme).background,
            Some(Background::Color(idle)),
            "macos-tahoe ({mode:?}): the group under the segments is the \
             control's fill"
        );
    }
    Ok(())
}

/// D15: a disabled windows-11 segmented control's group and edge, faded as
/// one by 0.55 over the window, are the values `docs/platform-facts.md:1646`
/// composites: the group #f0f0f0 / #1e1e1e and the edge #ebebeb / #292929.
#[cfg(feature = "widgets")]
#[test]
fn a_disabled_windows_11_segmented_control_fades_as_the_facts_compose() -> native_theme::Result<()>
{
    for (mode, group, edge) in [
        (ColorMode::Light, [0xf0, 0xf0, 0xf0], [0xeb, 0xeb, 0xeb]),
        (ColorMode::Dark, [0x1e, 0x1e, 0x1e], [0x29, 0x29, 0x29]),
    ] {
        let resolved = native_theme::theme::Theme::preset("windows-11")?
            .into_variant(mode)?
            .into_resolved(&native_theme::ResolutionContext::for_tests())?;
        let theme = crate::to_theme(&resolved, "windows-11");
        let style = styles::segmented_control(&resolved, true)(&theme);
        let rgb = |c: Color| {
            let [r, g, b, _] = c.into_rgba8();
            [r, g, b]
        };
        assert_eq!(
            flat(style.background).map(rgb),
            Ok(group),
            "{mode:?}: the group"
        );
        assert_eq!(rgb(style.border.color), edge, "{mode:?}: the edge");
    }
    Ok(())
}

#[cfg(feature = "iced_aw")]
#[test]
fn a_cleared_tab_soft_option_copies_the_idle_fill() -> native_theme::Result<()> {
    // The entry in the table above runs on windows-11, where `tab` stated the
    // same color for an idle and for a selected tab until G3 T15, so a
    // fallback onto the wrong one of the two passed it. Since G3 T15
    // windows-11 states them apart, its selected tab WinUI's
    // SolidBackgroundFillColorTertiary over the window's idle fill (#f9f9f9
    // over #f3f3f3, dark #282828 over #202020), so that entry now tells them
    // apart too, and this test could run on windows-11 again. It keeps
    // material-3, whose active tab is marked by an indicator, not a fill
    // (docs/platform-facts.md §2.11), with the selected fill set apart here,
    // to the preset's accent, and the copy is proved on that.
    let mut resolved = native_theme::theme::Theme::preset("material-3")?
        .into_variant(ColorMode::Light)?
        .into_resolved(&native_theme::ResolutionContext::for_tests())?;
    resolved.tab.active_background = resolved.defaults.accent_color;
    let idle = to_color(resolved.tab.background_color);
    let selected = to_color(resolved.tab.active_background);
    assert!(
        resolved.tab.hover_background.is_some() && idle != selected,
        "this preset no longer states a hover option, or no longer tells an \
         idle tab from a selected one, so clearing the option proves nothing: \
         idle {}, selected {}",
        show(idle),
        show(selected)
    );

    resolved.tab.hover_background = None;
    let theme = crate::to_theme(&resolved, "material-3");
    assert_eq!(
        fill(styles::aw::tab_bar(&resolved)(&theme, AwStatus::Hovered).tab_label_background),
        Ok(idle),
        "a cleared tab hover_background copies the idle tab's fill, not the \
         selected tab's"
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

/// The presets whose platform states no link hover background
/// (docs/platform-facts.md:1698, §2.28: macOS, KDE and GNOME, and Windows,
/// whose inline `Hyperlink` has no background), which they express as fully
/// transparent.
#[cfg(feature = "widgets")]
const LINK_HOVER_NONE: [&str; 5] = [
    "kde-breeze",
    "adwaita",
    "macos-monterey",
    "macos-tahoe",
    "windows-11",
];

/// The combinations whose platform states a translucent button fill:
/// `controlColor` is `#ffffff3f` in darkAqua
/// (docs/evidence/os-probe/2026-10-11-run-38129819806/os-probe-macos-14/probe.toml:408),
/// macOS 26 draws the push button in `secondarySystemFillColor`'s value, `#00000014` light and
/// `#ffffff14` dark, and WinUI 3's `ButtonBackground` is `ControlFillColorDefault`, `#B3FFFFFF`
/// light and `#0FFFFFFF` dark (docs/platform-facts.md:1177).
#[cfg(feature = "widgets")]
const TRANSLUCENT_BUTTON_FILL: [&str; 5] = [
    "macos-monterey/dark",
    "macos-tahoe/light",
    "macos-tahoe/dark",
    "windows-11/light",
    "windows-11/dark",
];

#[cfg(feature = "widgets")]
#[test]
fn compositing_holds_its_four_properties_on_every_native_color() -> native_theme::Result<()> {
    let combinations = combinations()?;
    let mut failures = Vec::new();
    let no_link_hover = combinations
        .iter()
        .filter(|c| LINK_HOVER_NONE.contains(&c.key))
        .count();
    let translucent_fill = combinations
        .iter()
        .filter(|c| TRANSLUCENT_BUTTON_FILL.contains(&c.label().as_str()))
        .count();
    for c in &combinations {
        let stated_none = LINK_HOVER_NONE.contains(&c.key);
        if stated_none != (c.resolved.link.hover_background.a == 0) {
            failures.push(format!(
                "{}: link.hover_background is {:?}, and the platform states {}",
                c.label(),
                c.resolved.link.hover_background,
                if stated_none { "none" } else { "one" }
            ));
        }
    }
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
        let declared_translucent = TRANSLUCENT_BUTTON_FILL.contains(&c.label().as_str());
        if declared_translucent != (base.a < 1.0) {
            failures.push(format!(
                "{}: button.background_color is {}, and the platform states {}",
                c.label(),
                show(base),
                if declared_translucent {
                    "a translucent one"
                } else {
                    "an opaque one"
                }
            ));
        }

        for (name, layer) in layers {
            let label = format!("{}: {name}", c.label());

            // (a) a *visible* layer over nothing is the layer: the link case.
            // A fully transparent layer is excluded, and the property is false
            // for one: a transparent layer adds nothing, so `composite_over`
            // returns the base rather than the layer's own channels, and a
            // colored layer at `a == 0.0` therefore comes back as the base's
            // (0, 0, 0, 0). Nothing is painted either way; only the discarded
            // channels differ.
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
        // Both colors are WinUI 3's, as its resources give them: windows-11
        // states `button.background_color` as `ControlFillColorDefault`
        // (`#B3FFFFFF` light, `#0FFFFFFF` dark, docs/platform-facts.md:1177),
        // and the layer is the pressed button's `ControlFillColorTertiary`
        // (`#4DF9F9F9`, `#08FFFFFF`, :1192). The preset states that one
        // flattened over the window, so it is set here, not read from it.
        let pressed_as_given = match c.label().as_str() {
            "windows-11/light" => Some(Rgba::new(0xf9, 0xf9, 0xf9, 0x4d)),
            "windows-11/dark" => Some(Rgba::new(0xff, 0xff, 0xff, 0x08)),
            _ => None,
        };
        if let Some(pressed) = pressed_as_given {
            let b = to_color(r.button.background_color);
            let l = to_color(pressed);
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
        combinations.len() * 4 - no_link_hover,
        "property (a) runs only on a layer with alpha above zero; if a preset \
         starts stating one of these four colors as fully transparent, the \
         count says so instead of the property quietly shrinking (the \
         LINK_HOVER_NONE presets state no link hover background)"
    );
    assert_eq!(
        opaque_bases,
        (combinations.len() - translucent_fill) * 4,
        "property (b) runs only over an opaque button.background_color, which \
         every combination outside TRANSLUCENT_BUTTON_FILL states; another \
         translucent one would stop the property running there"
    );
    assert!(
        opaque_layers >= combinations.len() * 2,
        "property (c) needs opaque layers: button.background_color and \
         defaults.text_color are opaque in most combinations, though not in \
         every one (macos-monterey and macos-tahoe state labelColor with \
         alpha in both modes, windows-11 light its text color, and every \
         TRANSLUCENT_BUTTON_FILL combination its button fill), so the floor \
         is an aggregate, on average two opaque layers \
         per combination, and it ran {opaque_layers} times over {} \
         combinations",
        combinations.len()
    );
    assert!(
        interior_composites >= 2,
        "property (d) needs a translucent layer over a translucent base: \
         windows-11 light and dark state button.background_color that way, \
         under WinUI's ControlFillColorTertiary, so it must run at least \
         twice, not {interior_composites} times"
    );
    assert!(
        failures.is_empty(),
        "{} compositing propert(ies) failed:\n{}",
        failures.len(),
        failures.join("\n")
    );
    Ok(())
}

#[cfg(feature = "widgets")]
#[test]
fn every_style_closure_is_clone() -> native_theme::Result<()> {
    // `iced_aw::SelectionList::new_with` takes `style: impl Fn(&Theme, Status)
    // -> Style + 'a + Clone` (`selection_list.rs:100-110`), and it is the only
    // constructor that gives the rows a class as well as the list, so a
    // consumer who wants both themed has no way round it. An `impl Trait`
    // return type leaks only auto traits and `Clone` is not one, so every
    // closure has to say `+ Clone` itself.
    //
    // Nothing is asserted at run time: the bound is the test. A closure that
    // stopped being `Clone` -- by capturing an `Arc<str>` family name, say --
    // would fail to compile on its own line here.
    fn assert_clone<T: Clone>(_: &T) {}

    let resolved = native_theme::theme::Theme::preset("adwaita")?
        .into_variant(ColorMode::Light)?
        .into_resolved(&native_theme::ResolutionContext::for_tests())?;
    let r = &resolved;

    assert_clone(&styles::button(r));
    assert_clone(&styles::button_primary(r));
    assert_clone(&styles::button_danger(r));
    assert_clone(&styles::button_success(r));
    assert_clone(&styles::button_warning(r));
    assert_clone(&styles::button_link(r));
    assert_clone(&styles::button_toggle(r, true));
    assert_clone(&styles::text_input(r));
    assert_clone(&styles::text_editor(r));
    assert_clone(&styles::checkbox(r));
    assert_clone(&styles::radio(r));
    assert_clone(&styles::toggler(r));
    assert_clone(&styles::pick_list(r));
    assert_clone(&styles::menu(r));
    assert_clone(&styles::slider(r));
    assert_clone(&styles::scrollable(r));
    assert_clone(&styles::progress_bar(r));
    assert_clone(&styles::rule(r));
    assert_clone(&styles::pane_grid(r));
    assert_clone(&styles::tooltip(r));
    assert_clone(&styles::application(r));
    assert_clone(&styles::dialog(r));
    assert_clone(&styles::toolbar(r));
    assert_clone(&styles::toolbar_rule(r));
    assert_clone(&styles::segment_separator(r, false));
    assert_clone(&styles::container_card(r));
    assert_clone(&styles::segmented_control(r, false));
    assert_clone(&styles::segment(r, true, styles::SegmentPosition::First));
    assert_clone(&styles::expander(r));
    assert_clone(&styles::expander_frame(r));

    // `styles::scrollbar` returns a value rather than a closure, and
    // `iced_widget::scrollable::Scrollbar` derives `Clone` and `Copy`
    // upstream, so it needs nothing from us.
    assert_clone(&styles::scrollbar(r));

    #[cfg(feature = "iced_aw")]
    {
        assert_clone(&styles::aw::card(r));
        assert_clone(&styles::aw::menu(r));
        assert_clone(&styles::aw::tab_bar(r));
        assert_clone(&styles::aw::sidebar(r));
        assert_clone(&styles::aw::selection_list(r));
        assert_clone(&styles::aw::spinner(r));
    }

    Ok(())
}

/// Every leaf of a serialised `ResolvedTheme`, under its dotted path, with its
/// value: the walk descends into objects and keeps everything else, a `null`
/// (an unstated soft option) included. egui's `leaf_paths`
/// (`native-theme-egui/src/mapping_tests.rs`) is the same walk.
#[cfg(feature = "iced_aw")]
fn leaf_values(value: &serde_json::Value) -> Vec<(String, serde_json::Value)> {
    fn walk(prefix: &str, value: &serde_json::Value, out: &mut Vec<(String, serde_json::Value)>) {
        match value {
            serde_json::Value::Object(map) => {
                for (key, child) in map {
                    let path = if prefix.is_empty() {
                        key.clone()
                    } else {
                        format!("{prefix}.{key}")
                    };
                    walk(&path, child, out);
                }
            }
            _ => out.push((prefix.to_owned(), value.clone())),
        }
    }
    let mut out = Vec::new();
    walk("", value, &mut out);
    out
}

/// Why an `Unread::Fallback` entry does not hold, if it does not: in the first
/// base whose `option` is stated, `option` cleared, splicing `leaf` with the
/// base's sentinels must move a row the connector fills from it -- the
/// fallback is drawn. `None` when it is.
#[cfg(feature = "iced_aw")]
fn fallback_unproved(
    leaf: &str,
    option: &str,
    combinations: &[Combination],
    jsons: &[serde_json::Value],
) -> Option<String> {
    let pointer = format!("/{}", option.replace('.', "/"));
    let Some((c, json)) = combinations
        .iter()
        .zip(jsons)
        .find(|(_, json)| json.pointer(&pointer).is_some_and(|v| !v.is_null()))
    else {
        return Some(format!(
            "UNREAD_COLOURS {leaf}: no base states its option {option}"
        ));
    };
    let mut cleared = json.clone();
    if let Some(value) = cleared.pointer_mut(&pointer) {
        *value = serde_json::Value::Null;
    }
    let Ok(base) = serde_json::from_value::<ResolvedTheme>(cleared.clone()) else {
        return Some(format!(
            "UNREAD_COLOURS {leaf}: {option} cleared does not deserialise"
        ));
    };
    let reading = Reading::of(&c.theme, &base);
    let (red, green, blue) = sentinel_rgb(&cleared);
    let reached = [
        format!("#{red:02x}{green:02x}{blue:02x}"),
        format!("#{red:02x}{green:02x}{blue:02x}80"),
    ]
    .iter()
    .filter_map(|hex| spliced_colour(&cleared, leaf, hex))
    .any(|r| {
        // A row whose native side moves, and which the connector fills
        // from the leaf: the walk's own drift check.
        let rows = reading.moved(&Reading::of(&c.theme, &r));
        let built = crate::to_theme(&r, c.key);
        !rows.is_empty() && Reading::both(&built, c.key, &r).drifted(&rows).is_empty()
    });
    (!reached).then(|| {
        format!(
            "UNREAD_COLOURS {leaf}: with {option} cleared it moves no row the \
             connector fills from it, so it is not that option's fallback"
        )
    })
}

/// `json` with the leaf at `leaf` replaced by the colour `hex`, read back as a
/// theme; `None` where the leaf already holds it or does not take a colour.
#[cfg(feature = "iced_aw")]
fn spliced_colour(json: &serde_json::Value, leaf: &str, hex: &str) -> Option<ResolvedTheme> {
    let pointer = format!("/{}", leaf.replace('.', "/"));
    let value = serde_json::Value::String(hex.to_owned());
    if json.pointer(&pointer)? == &value {
        return None;
    }
    let mut b = json.clone();
    *b.pointer_mut(&pointer)? = value;
    serde_json::from_value(b).ok()
}

/// The colour leaves of one serialised theme: a leaf whose value is a colour,
/// or a `null` a colour splices into -- an `Option<Rgba>` the theme leaves
/// unstated.
#[cfg(feature = "iced_aw")]
fn colour_leaves(json: &serde_json::Value) -> Vec<String> {
    leaf_values(json)
        .into_iter()
        .filter(|(leaf, value)| match value {
            serde_json::Value::String(s) => s.starts_with('#') && s.parse::<Rgba>().is_ok(),
            serde_json::Value::Null => spliced_colour(json, leaf, "#123456").is_some(),
            _ => false,
        })
        .map(|(leaf, _)| leaf)
        .collect()
}

/// A row of the contract under one of its statuses, or a row that has none (a
/// palette slot, an accessor, a style function that takes no status): what a
/// spliced colour moves.
#[cfg(feature = "iced_aw")]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Reached {
    row: String,
    /// The status as `{:?}` writes it; `None` for a row with no status, a
    /// style row whose one status is `()` among them.
    status: Option<String>,
}

#[cfg(feature = "iced_aw")]
impl Reached {
    fn of<S: std::fmt::Debug>(row: &str, status: S) -> Self {
        let status = format!("{status:?}");
        Self {
            row: row.to_owned(),
            status: (status != "()").then_some(status),
        }
    }

    fn row(row: &str) -> Self {
        Self {
            row: row.to_owned(),
            status: None,
        }
    }
}

#[cfg(feature = "iced_aw")]
impl std::fmt::Display for Reached {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.status {
            Some(status) => write!(f, "{} ({status})", self.row),
            None => f.write_str(&self.row),
        }
    }
}

/// Every row of the contract on one theme, as text keyed by the row's name
/// and status: each palette row, each style row under each of its statuses,
/// and each accessor row. A colour spliced into the theme reaches a row in a
/// status when that text changes.
///
/// The emitted side, what the connector builds, is read only where the theme
/// is the one `to_theme` built from `r` -- once per leaf, on the splice that
/// first moves a row -- and in the same text, so a row whose emitted text is
/// not its native text is one the connector fills from another leaf.
#[cfg(feature = "iced_aw")]
#[derive(Default)]
struct Reading {
    native: BTreeMap<Reached, String>,
    emitted: Option<BTreeMap<Reached, String>>,
}

#[cfg(feature = "iced_aw")]
impl Reading {
    fn put(&mut self, key: Reached, native: String, emitted: impl FnOnce() -> String) {
        if let Some(map) = &mut self.emitted {
            map.insert(key.clone(), emitted());
        }
        self.native.insert(key, native);
    }

    fn style<S: Copy + std::fmt::Debug>(
        &mut self,
        rows: &[StyleRow<S>],
        t: &Theme,
        r: &ResolvedTheme,
    ) {
        for row in rows {
            for &s in row.statuses {
                self.put(
                    Reached::of(row.field, s),
                    format!("{:?}", (row.native)(r, s)),
                    || match (row.get)(t, r, s) {
                        Ok(color) => format!("{color:?}"),
                        Err(why) => why,
                    },
                );
            }
        }
    }

    fn border<S: Copy + std::fmt::Debug>(
        &mut self,
        rows: &[BorderRow<S>],
        t: &Theme,
        r: &ResolvedTheme,
    ) {
        for row in rows {
            let name = format!("{}.border", row.field);
            for &s in row.statuses {
                let b = (row.native)(r, s);
                self.put(
                    Reached::of(&name, s),
                    format!("{:?} {:?} {:?}", b.color, b.width, Radius::new(b.radius)),
                    || {
                        let b = (row.get)(t, r, s);
                        format!("{:?} {:?} {:?}", b.color, b.width, b.radius)
                    },
                );
            }
        }
    }

    fn radius<S: Copy + std::fmt::Debug>(
        &mut self,
        rows: &[RadiusRow<S>],
        t: &Theme,
        r: &ResolvedTheme,
    ) {
        for row in rows {
            for &s in row.statuses {
                self.put(
                    Reached::of(row.field, s),
                    format!("{:?}", (row.native)(r, s)),
                    || match (row.get)(t, r, s) {
                        Ok(radius) => format!("{radius:?}"),
                        Err(why) => why,
                    },
                );
            }
        }
    }

    fn scalar<S: Copy + std::fmt::Debug>(
        &mut self,
        rows: &[ScalarRow<S>],
        t: &Theme,
        r: &ResolvedTheme,
    ) {
        for row in rows {
            for &s in row.statuses {
                self.put(
                    Reached::of(row.field, s),
                    format!("{:?}", (row.native)(t, r, s)),
                    || match (row.get)(t, r, s) {
                        Ok(length) => format!("{length:?}"),
                        Err(why) => why,
                    },
                );
            }
        }
    }

    /// Every row's native side on `r`. `t` is read by the one guarded scalar
    /// row alone (`ScalarRow`'s doc), for iced's own value where the guard
    /// fails, so the base combination's theme serves every splice of it: a
    /// colour moves no length.
    fn of(t: &Theme, r: &ResolvedTheme) -> Self {
        Self::read(t, r, None)
    }

    /// Both sides of every row on `r`, under `t = to_theme(r, key)`. A
    /// palette row's declared exception for `key` is a slot that may differ
    /// from its native field there, so its emitted side is taken as given.
    fn both(t: &Theme, key: &str, r: &ResolvedTheme) -> Self {
        Self::read(t, r, Some(key))
    }

    fn read(t: &Theme, r: &ResolvedTheme, key: Option<&str>) -> Self {
        let mut x = Self {
            native: BTreeMap::new(),
            emitted: key.map(|_| BTreeMap::new()),
        };
        for row in ROWS {
            let native = format!("{:?}", to_color((row.native)(r)));
            let excepted = key.is_some_and(|k| row.exceptions.iter().any(|(e, _)| *e == k));
            let emitted = if excepted {
                native.clone()
            } else {
                format!("{:?}", (row.get)(t, r))
            };
            x.put(Reached::row(row.slot), native, || emitted);
        }
        for row in ACCESSOR_ROWS {
            x.put(
                Reached::row(row.accessor),
                format!("{:?}", (row.native)(r)),
                || format!("{:?}", (row.get)(r)),
            );
        }
        x.style(BUTTON_ROWS, t, r);
        x.style(BUTTON_PRIMARY_ROWS, t, r);
        x.style(BUTTON_DANGER_ROWS, t, r);
        x.style(BUTTON_SUCCESS_ROWS, t, r);
        x.style(BUTTON_WARNING_ROWS, t, r);
        x.style(BUTTON_LINK_ROWS, t, r);
        x.style(BUTTON_TOGGLE_ROWS, t, r);
        x.style(TEXT_INPUT_ROWS, t, r);
        x.style(TEXT_EDITOR_ROWS, t, r);
        x.style(CHECKBOX_ROWS, t, r);
        x.style(RADIO_ROWS, t, r);
        x.style(TOGGLER_ROWS, t, r);
        x.style(PICK_LIST_ROWS, t, r);
        x.style(MENU_ROWS, t, r);
        x.style(SLIDER_ROWS, t, r);
        x.style(SCROLLABLE_ROWS, t, r);
        x.style(PROGRESS_BAR_ROWS, t, r);
        x.style(RULE_ROWS, t, r);
        x.style(PANE_GRID_ROWS, t, r);
        x.style(TOOLTIP_ROWS, t, r);
        x.style(APPLICATION_ROWS, t, r);
        x.style(DIALOG_ROWS, t, r);
        x.style(TOOLBAR_ROWS, t, r);
        x.style(TOOLBAR_RULE_ROWS, t, r);
        x.style(SEGMENT_SEPARATOR_ROWS, t, r);
        x.style(CONTAINER_CARD_ROWS, t, r);
        x.style(SEGMENTED_CONTROL_ROWS, t, r);
        x.style(SEGMENT_ROWS, t, r);
        x.style(EXPANDER_ROWS, t, r);
        x.border(BUTTON_BORDER_ROWS, t, r);
        x.border(BUTTON_PRIMARY_BORDER_ROWS, t, r);
        x.border(BUTTON_DANGER_BORDER_ROWS, t, r);
        x.border(BUTTON_SUCCESS_BORDER_ROWS, t, r);
        x.border(BUTTON_WARNING_BORDER_ROWS, t, r);
        x.border(BUTTON_TOGGLE_BORDER_ROWS, t, r);
        x.border(TEXT_INPUT_BORDER_ROWS, t, r);
        x.border(TEXT_EDITOR_BORDER_ROWS, t, r);
        x.border(CHECKBOX_BORDER_ROWS, t, r);
        x.border(PICK_LIST_BORDER_ROWS, t, r);
        x.border(MENU_BORDER_ROWS, t, r);
        x.border(PROGRESS_BAR_BORDER_ROWS, t, r);
        x.border(TOOLTIP_BORDER_ROWS, t, r);
        x.border(DIALOG_BORDER_ROWS, t, r);
        x.border(CONTAINER_CARD_BORDER_ROWS, t, r);
        x.border(SEGMENTED_CONTROL_BORDER_ROWS, t, r);
        x.border(EXPANDER_BORDER_ROWS, t, r);
        x.border(EXPANDER_FRAME_BORDER_ROWS, t, r);
        x.radius(TOGGLER_RADIUS_ROWS, t, r);
        x.scalar(RADIO_SCALAR_ROWS, t, r);
        x.scalar(TOGGLER_SCALAR_ROWS, t, r);
        x.scalar(SLIDER_SCALAR_ROWS, t, r);
        x.scalar(PANE_GRID_SCALAR_ROWS, t, r);
        x.scalar(SEGMENT_SCALAR_ROWS, t, r);
        x.style(AW_CARD_ROWS, t, r);
        x.style(AW_MENU_ROWS, t, r);
        x.style(AW_TAB_BAR_ROWS, t, r);
        x.style(AW_SIDEBAR_ROWS, t, r);
        x.style(AW_SELECTION_LIST_ROWS, t, r);
        x.style(AW_SPINNER_ROWS, t, r);
        x.border(AW_MENU_BORDER_ROWS, t, r);
        x.radius(AW_TAB_BAR_RADIUS_ROWS, t, r);
        x.scalar(AW_CARD_SCALAR_ROWS, t, r);
        x.scalar(AW_TAB_BAR_SCALAR_ROWS, t, r);
        x.scalar(AW_SIDEBAR_SCALAR_ROWS, t, r);
        x.scalar(AW_SELECTION_LIST_SCALAR_ROWS, t, r);
        x
    }

    /// The rows, under each status, whose native side differs between two
    /// readings.
    fn moved(&self, after: &Self) -> Vec<Reached> {
        self.native
            .iter()
            .filter(|(key, value)| after.native.get(*key) != Some(*value))
            .map(|(key, _)| key.clone())
            .collect()
    }

    /// The rows of `names` whose emitted side is not their native side, each
    /// with both texts.
    fn drifted(&self, names: &[Reached]) -> Vec<String> {
        names
            .iter()
            .filter_map(|name| {
                let native = self.native.get(name);
                let emitted = self.emitted.as_ref().and_then(|m| m.get(name));
                (native != emitted).then(|| {
                    format!(
                        "{name} emits {} where its native side gives {}",
                        emitted.map_or("nothing", String::as_str),
                        native.map_or("nothing", String::as_str)
                    )
                })
            })
            .collect()
    }
}

/// An opaque colour whose red, green and blue appear in no colour of `json`,
/// so that no row can emit it by passing another leaf through: counting up
/// from `#13579b` in the red channel.
#[cfg(feature = "iced_aw")]
fn sentinel_rgb(json: &serde_json::Value) -> (u8, u8, u8) {
    let taken: BTreeSet<(u8, u8, u8)> = leaf_values(json)
        .into_iter()
        .filter_map(|(_, value)| value.as_str().and_then(|s| s.parse::<Rgba>().ok()))
        .map(|c| (c.r, c.g, c.b))
        .collect();
    (0x13..=u8::MAX)
        .map(|red| (red, 0x57, 0x9b))
        .find(|rgb| !taken.contains(rgb))
        .unwrap_or((0x13, 0x57, 0x9b))
}

/// Each leaf the [`crate::Spinner`] draws in, with the row that reads it:
/// `spinner.fill_color`, the arc and a Material or Lucide indicator's tint,
/// and `defaults.text_color`, a freedesktop indicator's `currentColor`.
#[cfg(all(feature = "iced_aw", feature = "spinner"))]
const SPINNER_READS: &[(&str, &str)] = &[
    ("spinner.fill_color", "crate::Spinner (arc, bundled tint)"),
    (
        "defaults.text_color",
        "crate::Spinner (freedesktop currentColor)",
    ),
];

/// Whether `entry` -- a leaf, or a widget's name -- names `leaf`: the leaf
/// itself, or a leaf under it (`list.header_font` names
/// `list.header_font.color`).
#[cfg(feature = "iced_aw")]
fn names(entry: &str, leaf: &str) -> bool {
    leaf == entry
        || leaf
            .strip_prefix(entry)
            .is_some_and(|rest| rest.starts_with('.'))
}

/// The command that rewrites `contract-leaves.toml`, which
/// `every_colour_leaf_reaches_a_row_or_is_named` names when the file is stale.
#[cfg(feature = "iced_aw")]
const REGENERATE: &str =
    "cargo test -p native-theme-iced --all-features --lib -- --ignored regenerate_contract_leaves";

/// The exported leaf-to-row map as committed: the iced showcase reads it to
/// name the rows a colour leaf moves in its Widget Info.
#[cfg(feature = "iced_aw")]
const CONTRACT_LEAVES: &str = include_str!("../contract-leaves.toml");

/// What the colour-leaf walk finds over the 34 bundled combinations.
#[cfg(feature = "iced_aw")]
struct Reach {
    combinations: Vec<Combination>,
    jsons: Vec<serde_json::Value>,
    /// Every colour leaf of any combination.
    leaves: BTreeSet<String>,
    /// Per leaf, each row and status a splice of it moves, with the bases
    /// (indices into `combinations`) it moves it in.
    moves: BTreeMap<String, BTreeMap<Reached, BTreeSet<usize>>>,
    /// Per leaf, the first splice that moves a row: the base and the colour.
    first: BTreeMap<String, (usize, String)>,
}

/// Splice every colour leaf into every base with every colour, and record what
/// moves.
///
/// Each of the 34 preset/mode combinations is a base, and every colour leaf of
/// the serialised theme -- a colour, or an unstated `Option<Rgba>` a colour
/// splices into -- is spliced in turn with the leaf's value in another preset
/// of the same mode (egui's T2 method) and with an opaque and a
/// half-transparent sentinel no colour of the base shares its channels with,
/// so neither a passthrough of another leaf nor a composite under an opaque
/// layer hides the change. Every leaf is spliced in every base: a soft
/// option's fallback moves its row only where the option is unstated, and a
/// row reached in one base is not reached in every base, which the export
/// records. The bases are independent, so each is walked on a thread of its
/// own.
#[cfg(feature = "iced_aw")]
fn reach() -> native_theme::Result<Reach> {
    let combinations = combinations()?;
    let jsons: Vec<serde_json::Value> = combinations
        .iter()
        .filter_map(|c| serde_json::to_value(&c.resolved).ok())
        .collect();
    assert_eq!(
        jsons.len(),
        combinations.len(),
        "every ResolvedTheme serialises"
    );
    let mut leaves: BTreeSet<String> = BTreeSet::new();
    for json in &jsons {
        leaves.extend(colour_leaves(json));
    }

    // Per base, per leaf: the rows each colour moved, in the order tried.
    type Moved = Vec<(String, Vec<Reached>)>;
    let n = combinations.len();
    let per_base: Vec<BTreeMap<String, Moved>> = std::thread::scope(|scope| {
        let walks: Vec<_> = combinations
            .iter()
            .zip(&jsons)
            .enumerate()
            .map(|(i, (c, json))| {
                let (combinations, jsons, leaves) = (&combinations, &jsons, &leaves);
                scope.spawn(move || {
                    let base = Reading::of(&c.theme, &c.resolved);
                    let donor = (1..n)
                        .map(|k| (i + 2 * k) % n)
                        .find(|&j| combinations.get(j).is_some_and(|d| d.is_dark == c.is_dark))
                        .and_then(|j| jsons.get(j));
                    let (red, green, blue) = sentinel_rgb(json);
                    let sentinels = [
                        format!("#{red:02x}{green:02x}{blue:02x}"),
                        format!("#{red:02x}{green:02x}{blue:02x}80"),
                    ];
                    let mut found: BTreeMap<String, Moved> = BTreeMap::new();
                    for leaf in leaves {
                        let pointer = format!("/{}", leaf.replace('.', "/"));
                        let from_donor = donor
                            .and_then(|d| d.pointer(&pointer))
                            .and_then(|v| v.as_str())
                            .map(str::to_owned);
                        let mut moved = Vec::new();
                        for hex in from_donor.iter().chain(&sentinels) {
                            let Some(r) = spliced_colour(json, leaf, hex) else {
                                continue;
                            };
                            let rows = base.moved(&Reading::of(&c.theme, &r));
                            if !rows.is_empty() {
                                moved.push((hex.clone(), rows));
                            }
                        }
                        if !moved.is_empty() {
                            found.insert(leaf.clone(), moved);
                        }
                    }
                    found
                })
            })
            .collect();
        walks
            .into_iter()
            .map(|walk| {
                walk.join()
                    .unwrap_or_else(|panic| std::panic::resume_unwind(panic))
            })
            .collect()
    });

    let mut moves: BTreeMap<String, BTreeMap<Reached, BTreeSet<usize>>> = BTreeMap::new();
    let mut first: BTreeMap<String, (usize, String)> = BTreeMap::new();
    for (i, found) in per_base.into_iter().enumerate() {
        for (leaf, moved) in found {
            if let Some((hex, _)) = moved.first() {
                first.entry(leaf.clone()).or_insert((i, hex.clone()));
            }
            let rows = moves.entry(leaf).or_default();
            for (_, reached) in moved {
                for key in reached {
                    rows.entry(key).or_default().insert(i);
                }
            }
        }
    }
    Ok(Reach {
        combinations,
        jsons,
        leaves,
        moves,
        first,
    })
}

/// The feature a row exists under, where it is not there in every build that
/// has the contract: `styles::aw` needs `iced_aw`, the `Spinner` needs
/// `spinner`.
#[cfg(feature = "iced_aw")]
fn row_cfg(row: &str) -> Option<&'static str> {
    if row.starts_with("styles::aw::") {
        Some("iced_aw")
    } else if row.starts_with("crate::Spinner") {
        Some("spinner")
    } else {
        None
    }
}

/// `text` as a TOML basic string.
#[cfg(feature = "iced_aw")]
fn toml_string(text: &str) -> String {
    let mut out = String::with_capacity(text.len() + 2);
    out.push('"');
    for ch in text.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\t' => out.push_str("\\t"),
            c if c.is_control() => out.push_str(&format!("\\u{:04X}", u32::from(c))),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// A TOML array of strings, on one line.
#[cfg(feature = "iced_aw")]
fn toml_strings<'a>(items: impl IntoIterator<Item = &'a str>) -> String {
    let items: Vec<String> = items.into_iter().map(toml_string).collect();
    format!("[{}]", items.join(", "))
}

/// The table and the reason that name a colour leaf no row reads, as the
/// export records them: the table, the entry's own name (a leaf, or the
/// widget that is a prefix of it), the kind of reason and the reason.
#[cfg(feature = "iced_aw")]
fn naming(leaf: &str) -> Option<(&'static str, &'static str, String, &'static str)> {
    if let Some(entry) = UNREAD_COLOURS.iter().find(|e| names(e.leaf, leaf)) {
        let (kind, why) = match entry.why {
            Unread::NoReceiver(why) => ("no receiver".to_owned(), why),
            Unread::Inherited(why) => ("inherited".to_owned(), why),
            Unread::NoWidget(why) => ("no widget".to_owned(), why),
            Unread::Ruled(why) => ("ruled".to_owned(), why),
            Unread::Fallback { option, evidence } => (format!("fallback of {option}"), evidence),
        };
        return Some(("UNREAD_COLOURS", entry.leaf, kind, why));
    }
    UNREACHABLE
        .iter()
        .find(|u| names(u.field, leaf))
        .map(|u| ("UNREACHABLE", u.field, "no receiver".to_owned(), u.evidence))
}

/// `contract-leaves.toml` as the walk finds it: per colour leaf the rows it
/// moves, under which statuses, and the combinations in which it does not
/// move them, or the entry that names it; per row the leaves that move it.
#[cfg(feature = "iced_aw")]
fn render_contract_leaves(reach: &Reach) -> String {
    use std::fmt::Write as _;

    let labels: Vec<String> = reach.combinations.iter().map(Combination::label).collect();
    let all: BTreeSet<usize> = (0..labels.len()).collect();
    let mut out = String::new();
    let _ = write!(
        out,
        "\
# connectors/native-theme-iced/contract-leaves.toml -- generated; do not edit.
# Written by `{REGENERATE}`
# from the contract's colour-leaf walk (src/contract.rs, `reach`), and held to it by
# `every_colour_leaf_reaches_a_row_or_is_named`, which fails while the two differ. That
# test exists only with `--features iced_aw` (check_release's and CI's
# `iced_aw,watch,native` run); a default `cargo test` does not compare the file.
# The iced showcase reads it for the colour rows of its Widget Info.
#
# Every colour leaf of the serialised ResolvedTheme, spliced in each of the {} bundled
# preset/mode combinations, is a [leaves.\"<leaf>\"] table:
#   moves: each contract row the splice moves, as {{ row, statuses, except }}:
#     row       a palette slot (`palette.*`, `extended.*`), a crate-root accessor or the
#               connector's Spinner (`crate::*`), or a field of a style function's output
#               (`styles::<fn>.<field>`; `styles::<fn>.border` is the whole border);
#     statuses  the widget statuses, as `{{:?}}` writes them, it moves the row under;
#               absent for a row that takes no status;
#     except    the combinations (`<preset>/<mode>`) in which it does not move the row
#               under those statuses; absent where it moves it in all {}.
#   A leaf that moves no row in any combination has instead the entry that names it:
#     named     UNREAD_COLOURS or UNREACHABLE (src/contract/derived.rs);
#     entry     the entry's name, the leaf or a widget that is a prefix of it;
#     kind      why: no receiver, inherited, no widget, ruled, or fallback of <option>;
#     why       the entry's evidence.
# Every row a leaf moves is a [rows.\"<row>\"] table: leaves, the leaves that move it,
# and cfg, the feature it exists under where a build can lack it.
# combinations lists the {} combinations' names, in the walk's order.

combinations = {}
",
        labels.len(),
        labels.len(),
        labels.len(),
        toml_strings(labels.iter().map(String::as_str))
    );

    for leaf in &reach.leaves {
        let _ = write!(out, "\n[leaves.{}]\n", toml_string(leaf));
        match reach.moves.get(leaf) {
            Some(rows) => {
                // Per row, its statuses grouped by the bases they move in.
                type Groups<'a> = Vec<(BTreeSet<usize>, Vec<&'a str>)>;
                let mut grouped: BTreeMap<&str, Groups<'_>> = BTreeMap::new();
                for (key, bases) in rows {
                    let groups = grouped.entry(key.row.as_str()).or_default();
                    let status = key.status.as_deref();
                    match groups.iter_mut().find(|(b, _)| b == bases) {
                        Some((_, statuses)) => statuses.extend(status),
                        None => groups.push((bases.clone(), status.into_iter().collect())),
                    }
                }
                out.push_str("moves = [\n");
                for (row, groups) in &grouped {
                    for (bases, statuses) in groups {
                        let mut entry = format!("row = {}", toml_string(row));
                        if !statuses.is_empty() {
                            let _ = write!(
                                entry,
                                ", statuses = {}",
                                toml_strings(statuses.iter().copied())
                            );
                        }
                        if bases != &all {
                            let except = all
                                .difference(bases)
                                .filter_map(|&i| labels.get(i))
                                .map(String::as_str);
                            let _ = write!(entry, ", except = {}", toml_strings(except));
                        }
                        let _ = writeln!(out, "  {{ {entry} }},");
                    }
                }
                out.push_str("]\n");
            }
            None => {
                if let Some((named, entry, kind, why)) = naming(leaf) {
                    let _ = writeln!(out, "named = {}", toml_string(named));
                    let _ = writeln!(out, "entry = {}", toml_string(entry));
                    let _ = writeln!(out, "kind = {}", toml_string(&kind));
                    let _ = writeln!(out, "why = {}", toml_string(why));
                }
            }
        }
    }

    let mut converse: BTreeMap<&str, BTreeSet<&str>> = BTreeMap::new();
    for (leaf, rows) in &reach.moves {
        for key in rows.keys() {
            converse
                .entry(key.row.as_str())
                .or_default()
                .insert(leaf.as_str());
        }
    }
    for (row, leaves) in &converse {
        let _ = write!(out, "\n[rows.{}]\n", toml_string(row));
        if let Some(cfg) = row_cfg(row) {
            let _ = writeln!(out, "cfg = {}", toml_string(cfg));
        }
        let _ = writeln!(out, "leaves = {}", toml_strings(leaves.iter().copied()));
    }
    out
}

/// Rewrites `contract-leaves.toml` from the walk; run on purpose, with every
/// feature, as `REGENERATE` spells it.
#[cfg(feature = "iced_aw")]
#[test]
#[ignore = "rewrites contract-leaves.toml; run on purpose"]
fn regenerate_contract_leaves() -> native_theme::Result<()> {
    let reach = reach()?;
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/contract-leaves.toml");
    let written = std::fs::write(path, render_contract_leaves(&reach));
    assert!(written.is_ok(), "write {path}: {written:?}");
    Ok(())
}

/// M1-T7-03, iced's half: every colour leaf of `ResolvedTheme` reaches a
/// contract row, or a table says why it does not.
///
/// The tripwire (`every_named_field_has_exactly_one_declared_source`) walks
/// what the connector emits, so it cannot see a model value nothing reads.
/// This walks the model instead ([`reach`]). A leaf *reaches* a row when the
/// row's native side moves, in at least one base: a soft option's fallback
/// moves its row only where the option is unstated, which is why every base
/// is read, and one that no base leaves unstated is named by
/// `UNREAD_COLOURS` as a fallback.
///
/// A row's native closure could read a leaf the connector does not, where the
/// two are equal in every bundled combination (`text_area.border` and
/// `input.border` are), and then `every_style_field_equals_its_native_value`
/// cannot tell. So on the splice that first moves a row, the iced theme is
/// built from the spliced values and every row it moved must emit exactly
/// its new native value: a row whose native side moves and whose emitted
/// side does not fails, naming the leaf, the row and the base.
///
/// Every leaf that reaches no row must be named once, by `UNREAD_COLOURS` or
/// `UNREACHABLE`. An entry of either that names a leaf a row reads is stale,
/// and so is an `UNREAD_COLOURS` entry that names no colour leaf.
///
/// And `contract-leaves.toml`, the walk's export, must be what the walk finds:
/// a file that differs fails, naming the command that rewrites it. The test
/// exists only with `--features iced_aw` (check_release's and CI's
/// `iced_aw,watch,native` run), and compares the file only with `spinner` as
/// well, the default feature the `Spinner` rows need: a default `cargo test`
/// does not compare it.
#[cfg(feature = "iced_aw")]
#[test]
fn every_colour_leaf_reaches_a_row_or_is_named() -> native_theme::Result<()> {
    let started = std::time::Instant::now();
    let reach = reach()?;
    let Reach {
        combinations,
        jsons,
        leaves,
        moves,
        first,
    } = &reach;

    let mut failures = Vec::new();
    // The drift check, on each leaf's first moving splice.
    for (leaf, (i, hex)) in first {
        let (Some(c), Some(json)) = (combinations.get(*i), jsons.get(*i)) else {
            continue;
        };
        let Some(r) = spliced_colour(json, leaf, hex) else {
            continue;
        };
        let rows = Reading::of(&c.theme, &c.resolved).moved(&Reading::of(&c.theme, &r));
        let built = crate::to_theme(&r, c.key);
        for drift in Reading::both(&built, c.key, &r).drifted(&rows) {
            failures.push(format!(
                "{leaf} on {} moves a row the connector does not fill \
                 from it: {drift}",
                c.label()
            ));
        }
    }

    let (mut by_rows, mut by_accessors, mut by_exact, mut by_prefix, mut by_unreachable) =
        (0, 0, 0, 0, 0);
    for leaf in leaves {
        let unread: Vec<&UnreadColour> = UNREAD_COLOURS
            .iter()
            .filter(|e| names(e.leaf, leaf))
            .collect();
        let unreachable: Vec<&str> = UNREACHABLE
            .iter()
            .map(|u| u.field)
            .filter(|field| names(field, leaf))
            .collect();
        let named = unread.len() + unreachable.len();
        match moves.get(leaf) {
            Some(rows) => {
                if rows
                    .keys()
                    .all(|key| ACCESSOR_ROWS.iter().any(|a| key.row == a.accessor))
                {
                    by_accessors += 1;
                } else {
                    by_rows += 1;
                }
                if named > 0 {
                    failures.push(format!(
                        "{leaf}: reaches {} yet is named unread ({})",
                        rows.keys()
                            .map(Reached::to_string)
                            .collect::<Vec<_>>()
                            .join(", "),
                        unread
                            .iter()
                            .map(|e| e.leaf)
                            .chain(unreachable.iter().copied())
                            .collect::<Vec<_>>()
                            .join(", ")
                    ));
                }
            }
            None => match (unread.as_slice(), unreachable.as_slice()) {
                ([entry], []) if entry.leaf == leaf.as_str() => by_exact += 1,
                ([_], []) => by_prefix += 1,
                ([], [_]) => by_unreachable += 1,
                ([], []) => failures.push(format!(
                    "{leaf}: reaches no row, and neither UNREAD_COLOURS nor \
                     UNREACHABLE names it"
                )),
                _ => failures.push(format!("{leaf}: named {named} times")),
            },
        }
    }

    // The connector's own widget is a sink (the ruling in docs/archive/todo_closed.md,
    // *Contracts: the connectors' widgets*, "A design question for all connectors"): each
    // leaf the `Spinner` draws in reaches its row.
    #[cfg(feature = "spinner")]
    for (leaf, row) in SPINNER_READS {
        let rows = moves.get(*leaf);
        if !rows.is_some_and(|rows| rows.keys().any(|key| key.row == *row)) {
            failures.push(format!(
                "{leaf}: the Spinner draws in it, yet the rows it reaches ({}) \
                 leave out {row}",
                rows.map(|rows| rows
                    .keys()
                    .map(Reached::to_string)
                    .collect::<Vec<_>>()
                    .join(", "))
                    .unwrap_or_default()
            ));
        }
    }

    for entry in UNREAD_COLOURS {
        if !leaves.iter().any(|leaf| names(entry.leaf, leaf)) {
            failures.push(format!(
                "UNREAD_COLOURS names {}, which is no colour leaf and no \
                 prefix of one",
                entry.leaf
            ));
        }
        let (cited, needs) = match entry.why {
            Unread::NoReceiver(evidence) | Unread::Ruled(evidence) => {
                (evidence.contains(".rs:"), "an upstream file.rs:line")
            }
            Unread::Fallback { evidence, .. } => {
                (evidence.contains(".rs:"), "the connector's file.rs:line")
            }
            Unread::Inherited(evidence) => (
                evidence.contains("inheritance-rules.toml:"),
                "a docs/inheritance-rules.toml line",
            ),
            Unread::NoWidget(reason) => (
                !reason.is_empty() && !entry.leaf.contains('.') && !leaves.contains(entry.leaf),
                "a reason, and a widget's name rather than a leaf",
            ),
        };
        if !cited {
            failures.push(format!("UNREAD_COLOURS {}: needs {needs}", entry.leaf));
        }
        if let Unread::Fallback { option, .. } = entry.why
            && let Some(failure) = fallback_unproved(entry.leaf, option, combinations, jsons)
        {
            failures.push(failure);
        }
    }

    // The export is the walk's: every row it records exists in this build only
    // with `spinner` (the `Spinner` rows) and `iced_aw` (this test's own gate).
    #[cfg(feature = "spinner")]
    if CONTRACT_LEAVES != render_contract_leaves(&reach) {
        failures.push(format!(
            "contract-leaves.toml differs from what the colour-leaf walk finds; \
             run `{REGENERATE}`"
        ));
    }

    println!(
        "{} colour leaves: {by_rows} reach a style or palette row, {by_accessors} \
         an accessor row only, {by_exact} named by an UNREAD_COLOURS leaf, \
         {by_prefix} by an UNREAD_COLOURS widget, {by_unreachable} by \
         UNREACHABLE; {:.2?}",
        leaves.len(),
        started.elapsed()
    );
    assert!(
        failures.is_empty(),
        "{} colour-leaf classification(s) do not hold:\n{}",
        failures.len(),
        failures.join("\n")
    );
    Ok(())
}
