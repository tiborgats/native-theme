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
use iced_widget::{
    button, checkbox, container, pick_list, progress_bar, radio, rule, scrollable, slider,
    text_editor, text_input, toggler,
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

use derived::{DERIVED, SECONDARY_LABEL_AS_STATED, SECONDARY_LABEL_SUBSTITUTED, UNREACHABLE};
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

/// Every field of the `rule::Style` the connector emits. Its `radius` is a
/// whole `Radius` rather than part of a `Border`, and it is walked as the one
/// leaf it is -- as `toggler::Style.border_radius` is.
#[cfg(feature = "widgets")]
fn rule_style_fields(style: &rule::Style) -> Vec<String> {
    let mut out = Vec::new();
    leaves_under!(
        out,
        style,
        "styles::rule",
        rule::Style {
            color,
            radius,
            fill_mode,
            snap
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
        out.extend(rule_style_fields(&styles::rule(resolved)(theme)));
        // Two functions emit a `container::Style`, so the one walker runs
        // twice under two prefixes.
        out.extend(container_fields(
            &styles::tooltip(resolved)(theme),
            "styles::tooltip",
        ));
        out.extend(container_fields(
            &styles::container_card(resolved)(theme),
            "styles::container_card",
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

    // These two cross-checks cannot fire while the naming convention holds: an
    // `UNREACHABLE` entry names a *native* field (`scrollbar.min_thumb_length`)
    // and every row and `DERIVED` name is an emitted one -- `palette.*`,
    // `extended.*` or `styles::*` -- so the two namespaces never meet. What
    // they guard is the convention itself -- a row or a `DERIVED` entry
    // written under a native name would be caught here. The live protection
    // for the entry's own claim is
    // `the_unreachable_native_value_is_stated_by_every_preset`.
    for (field, evidence) in UNREACHABLE {
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

/// The one native value iced 0.14 has no receiver for is a value the presets
/// actually state.
///
/// `UNREACHABLE` is a claim about a native field, not about an emitted one, so
/// the coverage tripwire cannot see it: it walks what `styles::*` returns. What
/// can be checked is that the entry is about something real -- the field is
/// read here, so removing it from the model breaks this file, and its value is
/// printed for every combination so the record says what was dropped rather
/// than only that something was.
#[cfg(feature = "widgets")]
#[test]
fn the_unreachable_native_value_is_stated_by_every_preset() -> native_theme::Result<()> {
    let combinations = combinations()?;
    let mut missing = Vec::new();
    let mut stated = Vec::new();

    for c in &combinations {
        let length = c.resolved.scrollbar.min_thumb_length;
        if length > 0.0 {
            stated.push(format!("{}: {length}", c.label()));
        } else {
            missing.push(format!("{}: {length}", c.label()));
        }
    }

    println!(
        "scrollbar.min_thumb_length, which iced 0.14 cannot take, stated by \
         {} of {} combinations:\n{}",
        stated.len(),
        combinations.len(),
        stated.join("\n")
    );
    assert!(
        missing.is_empty(),
        "{} combination(s) state no minimum thumb length, so the UNREACHABLE \
         entry no longer describes a value the platform gives:\n{}",
        missing.len(),
        missing.join("\n")
    );
    Ok(())
}

/// The `extended.secondary.base.text` entry's "31 of 32", counted rather than
/// asserted in prose.
///
/// The entry says `Pair::new` does not emit the label the connector hands it:
/// it runs `readable()` against the placeholder fill, and the platform's own
/// `defaults.text_color` survives that only where it clears iced's contrast
/// bar. How often it does is the entry's claim, and this is the measurement
/// behind it -- `SECONDARY_LABEL_SUBSTITUTED` and `SECONDARY_LABEL_AS_STATED`
/// are declared beside the sentence, so a preset added to the bundle moves
/// the count and fails here instead of leaving the prose stale.
#[test]
fn readable_substitutes_the_secondary_label_on_every_combination_but_one()
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
        ("TOOLTIP_STATUSES", TOOLTIP_STATUSES),
        ("CONTAINER_CARD_STATUSES", CONTAINER_CARD_STATUSES),
    ] {
        assert_eq!(
            statuses,
            &[()],
            "{what}: the widget takes no status, so its rows hold the unit \
             value exactly once"
        );
    }

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
    check_style_rows(TOOLTIP_ROWS, &combinations, &mut failures, &mut ran);
    check_border_rows(TOOLTIP_BORDER_ROWS, &combinations, &mut failures, &mut ran);
    check_style_rows(CONTAINER_CARD_ROWS, &combinations, &mut failures, &mut ran);
    check_border_rows(
        CONTAINER_CARD_BORDER_ROWS,
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
    // The twelve soft options the widget functions read: four `checkbox.*`
    // (three of which `styles::radio` reads again through its own
    // expressions), five `switch.*`, one `combo_box.*`, one `slider.*` and one
    // `scrollbar.*`. No bundled preset leaves any of them `None` after
    // resolution, so the rows cannot tell a right base-state field from a
    // wrong one; the fallbacks are reached by clearing them here (section 3.2).
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
            && resolved.slider.thumb_hover_color.is_some()
            && resolved.scrollbar.thumb_active_color.is_some(),
        "this preset no longer states all twelve soft options, so clearing \
         them proves nothing"
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
    let hovered_scroller = to_color(resolved.scrollbar.thumb_hover_color);
    #[cfg(feature = "iced_aw")]
    let tabs = styles::aw::tab_bar(&resolved);
    #[cfg(feature = "iced_aw")]
    let tab_fill = to_color(resolved.tab.background_color);

    // One entry per read site, not per model field: two functions reading the
    // same option have two `unwrap_or` expressions.
    let read_sites = [
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

#[cfg(feature = "iced_aw")]
#[test]
fn a_cleared_tab_soft_option_copies_the_idle_fill() -> native_theme::Result<()> {
    // The entry in the table above runs on windows-11, where `tab` states the
    // same color for an idle and for a selected tab, so a fallback onto the
    // wrong one of the two would pass it. `material` is the one bundled preset
    // that states them differently (measured 2026-09-21 over all 32
    // combinations), so the copy is proved here instead.
    let mut resolved = native_theme::theme::Theme::preset("material")?
        .into_variant(ColorMode::Light)?
        .into_resolved(&native_theme::ResolutionContext::for_tests())?;
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
    let theme = crate::to_theme(&resolved, "material");
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
    assert_clone(&styles::tooltip(r));
    assert_clone(&styles::container_card(r));

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
