//! The value half of model parity (plan, section 1, rule 2): [`compare`] reads, for each row of the table of
//! homes, the value of its key in the JSON serialisation of a 0.7.2 resolved theme and the value at each of
//! its homes in a 0.8.0 [`ResolvedTheme`], and compares the two exactly, by the kind of the home.
//!
//! The kinds: a colour (the new colour's 8-bit channels and alpha against 0.7.2's), an `f32` (bit-equal, no
//! tolerance), an integer, a bool, a string, an enum (one mapping table each, below, written from the two
//! declarations), a font's `defined_size` against [`FontSize`], and an old `null` against "unstated" at the
//! home: the `stated` bit clear for a part's property (per side for padding and margin), `None` for an
//! `Option` slot, metric or global. An old value against an unstated home is a mismatch, and so is an old
//! `null` against a stated one. An old `null` at a ruled part's property under a state is no rule for that
//! state (spec §5, notes to the table): the property there, value and `stated` bit, must equal the same
//! property at the reference state, `NONE` or the row's `null_reference` where the home's state holds it,
//! and so must its `from_state` bit, which is clear at `NONE` (spec §2: a value stated for the state, even
//! one equal to the reference's, is a rule for it; a `null_reference` state's own rule sets the bit at both).
//! An old value at such a home is compared as any other, whatever its `from_state` bit. Spec §5's notes add the special homes: a part's `shadows` against 0.7.2's
//! `border.shadow_enabled` (`false` is `SHADOWS` stated with `shadows = None`, `true` is `SHADOWS`
//! unstated), a part's stroke colour and width read out of `Stroke::Uniform` inside the edge (`Stroke::None`
//! is the width 0.0 and no colour) and a corner radius out of `CornerRadius::Uniform` with circular corners
//! (`CornerRadius::None` is 0.0), and a text's `decoration` against 0.7.2's `underline_enabled` (a plain
//! underline is `true`, no line `false`); any other form there (a gradient fill, a per-side or centred
//! stroke, a pill, continuous corners, a coloured underline) is a value no 0.7.2 field holds, so a mismatch. A colour is compared by its 8-bit
//! channels only where every channel is finite and in 0..=1; any other colour is a mismatch.
//!
//! The table's keys that change how a row is read: `if_null` is followed along the chain of the rows of
//! the keys it names while their values are null; `unstated_if_unset` compares the row as if its key were
//! null where the unresolved 0.7.2 theme leaves the field unset ([`Extra::unresolved`]); `null_means =
//! "none"` requires, where the key is null, `Stroke::None` with the `STROKE` bit set at every home, whatever
//! the reference state holds; `context` reads the homes from the theme resolved with it
//! ([`Extra::with_context`]). A home in `WindowChrome` (`window.<getter>(normal).<property>`) reads the
//! getter at `WindowState::default()`.
//!
//! A row written by hand names a field that does not serialise; it is read from `old` at its key where the
//! caller has added it there, in the form `serde_json` gives the field's type (`Resolved.mode`, whose
//! `ColorMode` has no serialisation, as the variant's name: `"Light"`, `"Dark"`), and skipped where the key
//! is absent. A home this module cannot read, or an old value not of its home's kind, is an error naming the
//! row, never a skip.

use std::collections::BTreeMap;
use std::fmt;
use std::str::FromStr;

use serde_json::Value;

use super::{HOMES, NullMeans, Row, RowContext, Table};
use crate::IconSet;
use crate::next::color::Color;
use crate::next::geometry::{CornerCurve, CornerRadius, Stroke};
use crate::next::layout::{ArrowSide, DialogButtonOrder, TabIndicatorSide};
use crate::next::motion::DialogEntranceStyle;
use crate::next::paint::Paint;
use crate::next::preset::Preset;
use crate::next::query::{
    ButtonKind, ButtonQuery, ButtonShape, ButtonVariant, Contrast, ControlSize, GroupPosition,
    InputQuery, InputVariant, ListKind, ListQuery, ListVariant, MotionPreference, Orientation,
    Scheme, StyleQuery, Surface, TabQuery, TabVariant, ThemeContext, WidgetState,
};
use crate::next::roles::SurfaceColors;
use crate::next::style::{
    DecorationKind, FocusTrigger, FontSize, FontStyle, IconStated, IconStyle, PartStated,
    PartStyle, TextDecoration, TextStated, TextStyle,
};
use crate::next::widgets::ResolvedTheme;
use crate::next::window::WindowState;
use crate::theme::ColorMode;

// ---------------------------------------------------------------------------------------------------------
// The mapping tables of the enums, each written from 0.7.2's declaration and the 0.8.0 one.
// ---------------------------------------------------------------------------------------------------------

/// 0.7.2's `FontStyle` (`src/model/font.rs`, serialised in lowercase) to 0.8.0's (spec §2).
const FONT_STYLES: [(crate::theme::FontStyle, FontStyle); 3] = [
    (crate::theme::FontStyle::Normal, FontStyle::Normal),
    (crate::theme::FontStyle::Italic, FontStyle::Italic),
    (crate::theme::FontStyle::Oblique, FontStyle::Oblique),
];

/// 0.7.2's `ColorMode` (`src/model/mod.rs`) to 0.8.0's `Scheme` (spec §3), the table's note on
/// `Resolved.mode`.
const SCHEMES: [(ColorMode, Scheme); 2] = [
    (ColorMode::Light, Scheme::Light),
    (ColorMode::Dark, Scheme::Dark),
];

/// 0.7.2's `focus_ring_keyboard_only: Option<bool>` to 0.8.0's `FocusIndicator::trigger` (spec §2), the
/// table's note on `defaults.focus_ring_keyboard_only`; `None` is `None`.
const FOCUS_TRIGGERS: [(bool, FocusTrigger); 2] = [
    (true, FocusTrigger::KeyboardOnly),
    (false, FocusTrigger::Always),
];

/// 0.7.2's `TabIndicatorSide` (`src/model/tab_indicator_side.rs`, serialised in lowercase) to 0.8.0's
/// (`layout.rs`).
const TAB_INDICATOR_SIDES: [(crate::theme::TabIndicatorSide, TabIndicatorSide); 2] = [
    (crate::theme::TabIndicatorSide::Top, TabIndicatorSide::Top),
    (
        crate::theme::TabIndicatorSide::Bottom,
        TabIndicatorSide::Bottom,
    ),
];

/// 0.7.2's `DialogButtonOrder` (`src/model/dialog_order.rs`, serialised in snake case) to 0.8.0's
/// (`layout.rs`).
const DIALOG_BUTTON_ORDERS: [(crate::theme::DialogButtonOrder, DialogButtonOrder); 2] = [
    (
        crate::theme::DialogButtonOrder::PrimaryRight,
        DialogButtonOrder::PrimaryRight,
    ),
    (
        crate::theme::DialogButtonOrder::PrimaryLeft,
        DialogButtonOrder::PrimaryLeft,
    ),
];

/// 0.7.2's `DialogEntranceStyle` (`src/model/dialog_entrance_style.rs`, serialised in snake case) to
/// 0.8.0's (`motion.rs`), which adds `Fade` and `None`.
const DIALOG_ENTRANCES: [(crate::theme::DialogEntranceStyle, DialogEntranceStyle); 2] = [
    (
        crate::theme::DialogEntranceStyle::ScaleFade,
        DialogEntranceStyle::ScaleFade,
    ),
    (
        crate::theme::DialogEntranceStyle::FadeRise,
        DialogEntranceStyle::FadeRise,
    ),
];

/// 0.7.2's `ArrowSide` (`src/model/arrow_side.rs`, serialised in lowercase) to 0.8.0's (`layout.rs`).
const ARROW_SIDES: [(crate::theme::ArrowSide, ArrowSide); 2] = [
    (crate::theme::ArrowSide::Leading, ArrowSide::Leading),
    (crate::theme::ArrowSide::Trailing, ArrowSide::Trailing),
];

/// Whether `old`, the serialisation of a 0.7.2 enum value, maps to `new` in `table`; `None` where no entry
/// of `table` serialises to `old`.
fn same_variant<O: serde::Serialize, N: PartialEq>(
    old: &Value,
    table: &[(O, N)],
    new: &N,
) -> Result<Option<bool>, String> {
    for (old_variant, new_variant) in table {
        if serde_json::to_value(old_variant).map_err(|e| e.to_string())? == *old {
            return Ok(Some(new_variant == new));
        }
    }
    Ok(None)
}

/// The variant of 0.7.2's `FontSize` (`src/model/font.rs`, serialised externally tagged, `{"Pt": 10.0}`)
/// that 0.8.0's `FontSize` (spec §2) of the same name is, and its value.
fn font_size_parts(size: FontSize) -> (&'static str, f32) {
    match size {
        FontSize::Pt(v) => ("Pt", v),
        FontSize::Px(v) => ("Px", v),
    }
}

// ---------------------------------------------------------------------------------------------------------
// The report.
// ---------------------------------------------------------------------------------------------------------

/// Why a row was not compared.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Skip {
    /// A home is the reader's context, and no snapshot was given.
    NoSnapshot,
    /// The row is written by hand, and the caller did not add its key to `old`.
    HandKeyAbsent,
    /// The row is compared only where its `only_if_null` key is null, and that key is stated.
    OtherKeyStated,
}

impl fmt::Display for Skip {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::NoSnapshot => "no snapshot",
            Self::HandKeyAbsent => "written by hand, not in old",
            Self::OtherKeyStated => "its only_if_null key is stated",
        })
    }
}

/// A home whose value differs from the old one.
#[derive(Clone, Debug, PartialEq)]
pub(super) struct Mismatch {
    /// The row's key.
    pub(super) key: String,
    /// The home, followed by the fallback home read where the first is an absent slot.
    pub(super) home: String,
    /// The value compared, as `old` holds it (numbers also as the `f32` they read as, with its bits).
    pub(super) old: String,
    /// The value at the home, in full precision.
    pub(super) new: String,
}

/// What became of a row.
#[derive(Clone, Debug, PartialEq)]
pub(super) enum Outcome {
    /// Every home holds the value.
    Equal,
    /// The homes that do not.
    Mismatch(Vec<Mismatch>),
    /// Not compared.
    Skipped(Skip),
}

/// One row's result.
#[derive(Clone, Debug, PartialEq)]
pub(super) struct RowReport {
    /// The row's key.
    pub(super) key: String,
    /// The value compared with the homes (after `if_null` and a listed difference), where compared.
    pub(super) value: Option<Value>,
    /// The outcome.
    pub(super) outcome: Outcome,
}

/// The totals of a report.
#[derive(Clone, Debug, Default, PartialEq)]
pub(super) struct Totals {
    /// Rows in the table.
    pub(super) rows: usize,
    /// Rows compared: equal or mismatching.
    pub(super) compared: usize,
    /// Rows whose every home is equal.
    pub(super) equal: usize,
    /// Rows with at least one mismatching home.
    pub(super) mismatches: usize,
    /// Rows skipped, by reason.
    pub(super) skipped: BTreeMap<Skip, usize>,
}

/// The result of a comparison: each row's outcome, in the table's order, and the totals.
#[derive(Clone, Debug, Default, PartialEq)]
pub(super) struct Report {
    pub(super) rows: Vec<RowReport>,
    pub(super) totals: Totals,
}

impl Report {
    fn push(&mut self, row: RowReport) {
        let t = &mut self.totals;
        t.rows = t.rows.saturating_add(1);
        match &row.outcome {
            Outcome::Equal => {
                t.compared = t.compared.saturating_add(1);
                t.equal = t.equal.saturating_add(1);
            }
            Outcome::Mismatch(_) => {
                t.compared = t.compared.saturating_add(1);
                t.mismatches = t.mismatches.saturating_add(1);
            }
            Outcome::Skipped(skip) => {
                let n = t.skipped.entry(*skip).or_default();
                *n = n.saturating_add(1);
            }
        }
        self.rows.push(row);
    }

    /// The keys of the rows that mismatch, in the table's order.
    pub(super) fn mismatched_keys(&self) -> Vec<&str> {
        self.rows
            .iter()
            .filter(|r| matches!(r.outcome, Outcome::Mismatch(_)))
            .map(|r| r.key.as_str())
            .collect()
    }

    /// The number of rows skipped, for every reason.
    pub(super) fn skipped(&self) -> usize {
        self.totals.skipped.values().sum()
    }
}

impl fmt::Display for Report {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let t = &self.totals;
        writeln!(
            f,
            "{} rows: {} compared ({} equal, {} mismatching), skipped {:?}",
            t.rows, t.compared, t.equal, t.mismatches, t.skipped
        )?;
        for row in &self.rows {
            match &row.outcome {
                Outcome::Equal => {}
                Outcome::Mismatch(homes) => {
                    for m in homes {
                        writeln!(
                            f,
                            "mismatch `{}` at `{}`: old {}, new {}",
                            m.key, m.home, m.old, m.new
                        )?;
                    }
                }
                Outcome::Skipped(skip) => writeln!(f, "skipped `{}`: {skip}", row.key)?,
            }
        }
        Ok(())
    }
}

// ---------------------------------------------------------------------------------------------------------
// The comparison.
// ---------------------------------------------------------------------------------------------------------

/// What some rows read beside the two resolved themes.
#[derive(Default)]
pub(super) struct Extra<'a> {
    /// The JSON serialisation of the unresolved 0.7.2 theme that `old` was resolved from (the variant of
    /// its scheme), which the `unstated_if_unset` rows read; a field it leaves `None` is absent from it.
    pub(super) unresolved: Option<&'a Value>,
    /// The 0.8.0 theme resolved with a row's `context` in place of the defaults, which the `context` rows
    /// read their homes from.
    pub(super) with_context: Option<&'a dyn Fn(RowContext) -> ResolvedTheme>,
}

/// The report of the table of homes on `preset`: `old` is the JSON serialisation of 0.7.2's resolved theme
/// (with any row written by hand added at its key), `new` the 0.8.0 resolved theme, `snapshot_context` the
/// reader's context where a snapshot is compared, `extra` what the rows of the keys `unstated_if_unset` and
/// `context` read. An error names the row whose home or old value cannot be read.
pub(super) fn compare(
    old: &Value,
    new: &ResolvedTheme,
    snapshot_context: Option<&ThemeContext>,
    preset: Preset,
    extra: &Extra<'_>,
) -> Result<Report, String> {
    let table: Table = toml::from_str(HOMES).map_err(|e| e.to_string())?;
    compare_rows_with(&table.row, old, new, snapshot_context, preset, extra)
}

/// [`compare`] on `rows`, without [`Extra`].
pub(super) fn compare_rows(
    rows: &[Row],
    old: &Value,
    new: &ResolvedTheme,
    snapshot_context: Option<&ThemeContext>,
    preset: Preset,
) -> Result<Report, String> {
    compare_rows_with(rows, old, new, snapshot_context, preset, &Extra::default())
}

/// [`compare`] on `rows`.
pub(super) fn compare_rows_with(
    rows: &[Row],
    old: &Value,
    new: &ResolvedTheme,
    snapshot_context: Option<&ThemeContext>,
    preset: Preset,
    extra: &Extra<'_>,
) -> Result<Report, String> {
    let mut report = Report::default();
    for row in rows {
        let result = compare_row(row, rows, old, new, snapshot_context, preset, extra)
            .map_err(|e| format!("row `{}`: {e}", row.key))?;
        report.push(result);
    }
    Ok(report)
}

const SNAPSHOT_PREFIX: &str = "snapshot.context.";

/// The prefix of a home in `WindowChrome` (spec §4).
const WINDOW_PREFIX: &str = "window.";

#[allow(
    clippy::too_many_arguments,
    reason = "the row, its table and the inputs of `compare_rows_with`"
)]
fn compare_row(
    row: &Row,
    rows: &[Row],
    old: &Value,
    new: &ResolvedTheme,
    snapshot_context: Option<&ThemeContext>,
    preset: Preset,
    extra: &Extra<'_>,
) -> Result<RowReport, String> {
    let skipped = |skip| RowReport {
        key: row.key.clone(),
        value: None,
        outcome: Outcome::Skipped(skip),
    };
    let homes = row
        .homes_by_preset
        .get(preset.as_str())
        .unwrap_or(&row.homes);
    if homes.is_empty() {
        return Err(format!("no home on `{}`", preset.as_str()));
    }
    if snapshot_context.is_none() && homes.iter().any(|h| h.starts_with(SNAPSHOT_PREFIX)) {
        return Ok(skipped(Skip::NoSnapshot));
    }
    if let Some(other) = &row.only_if_null {
        match lookup(old, other) {
            Some(Value::Null) => {}
            Some(_) => return Ok(skipped(Skip::OtherKeyStated)),
            None => return Err(format!("its only_if_null key `{other}` is not in old")),
        }
    }
    let value = match lookup(old, &row.key) {
        Some(value) => value,
        None if row.hand => return Ok(skipped(Skip::HandKeyAbsent)),
        None => return Err("its key is not in old".to_owned()),
    };
    let value = if row.unstated_if_unset {
        let unresolved = extra
            .unresolved
            .ok_or("unstated_if_unset needs the unresolved 0.7.2 theme")?;
        if unresolved_states(unresolved, &row.key) {
            value
        } else {
            &Value::Null
        }
    } else {
        value
    };
    let value = follow_if_null(row, rows, value, old)?;
    let (value, old_null) = match listed_difference(row, preset)? {
        Some(listed) => (listed, false),
        None => (value.clone(), value.is_null()),
    };
    let with_context;
    let new = match row.context {
        Some(context) => {
            let resolve = extra
                .with_context
                .ok_or("its context needs the 0.8.0 theme resolved with it")?;
            with_context = resolve(context);
            &with_context
        }
        None => new,
    };
    let mut mismatches = Vec::new();
    for home in homes {
        if old_null && row.null_means == Some(NullMeans::NoOutline) {
            let style = part_style_at(home, new)?
                .ok_or_else(|| format!("null_means: `{home}` is no part's stroke"))?;
            let stated = style.stated.contains(PartStated::STROKE);
            if !stated || style.stroke != Stroke::None {
                let bit = if stated { "stated" } else { "unstated" };
                mismatches.push(Mismatch {
                    key: row.key.clone(),
                    home: home.clone(),
                    old: "null: no outline, `Stroke::None` with STROKE stated".to_owned(),
                    new: format!("{:?}, STROKE {bit}", style.stroke),
                });
            }
            continue;
        }
        if old_null && let Some(reference) = reference_home(home, row.null_reference.as_deref())? {
            let got = read(home, new, snapshot_context)?;
            let at_reference = read(&reference, new, snapshot_context)?;
            if got != at_reference {
                mismatches.push(Mismatch {
                    key: row.key.clone(),
                    home: format!("{home} against {reference}"),
                    old: format!("null: no rule, as at the reference ({at_reference})"),
                    new: got.to_string(),
                });
            } else {
                let (here, there) = (from_state_at(home, new)?, from_state_at(&reference, new)?);
                if here != there {
                    let shown = |bit: Option<bool>| match bit {
                        Some(true) => "set",
                        Some(false) | None => "clear",
                    };
                    mismatches.push(Mismatch {
                        key: row.key.clone(),
                        home: format!("{home} against {reference}"),
                        old: format!(
                            "null: no rule, `from_state` as at the reference ({})",
                            shown(there)
                        ),
                        new: format!("{got}, `from_state` {}", shown(here)),
                    });
                }
            }
            continue;
        }
        let (shown, got) = match (read(home, new, snapshot_context)?, &row.home_fallback) {
            (Got::NoSlot, Some(fallback)) => (
                format!("{home} -> {fallback}"),
                read(fallback, new, snapshot_context)?,
            ),
            (got, _) => (home.clone(), got),
        };
        if !equal(&value, &got).map_err(|e| format!("at `{home}`: {e}"))? {
            mismatches.push(Mismatch {
                key: row.key.clone(),
                home: shown,
                old: show_old(&value),
                new: got.to_string(),
            });
        }
    }
    let outcome = if mismatches.is_empty() {
        Outcome::Equal
    } else {
        Outcome::Mismatch(mismatches)
    };
    Ok(RowReport {
        key: row.key.clone(),
        value: Some(value),
        outcome,
    })
}

/// Where an old `null` compares `home` when the home is a ruled part's property at a state: the same
/// property at the reference state, which is `reference` (a row's `null_reference`) where the home's state
/// holds all its flags and more, else `NONE`; the axes are kept. `None` for any other home (no query, or the
/// state `NONE`), where an old `null` is "unstated".
fn reference_home(home: &str, reference: Option<&str>) -> Result<Option<String>, String> {
    if home.starts_with(WINDOW_PREFIX) {
        return Ok(None);
    }
    let Some((head, after)) = home.split_once('(') else {
        return Ok(None);
    };
    let Some((query, property)) = after.split_once(").") else {
        return Ok(None);
    };
    let (states, axes) = match query.split_once("; ") {
        Some((states, axes)) => (states, Some(axes)),
        None => (query, None),
    };
    let state = Query::parse(states)?.state;
    if state.is_empty() {
        return Ok(None);
    }
    let reference = match reference {
        Some(text) => {
            let named = Query::parse(text)?.state;
            if state.contains(named) && state != named {
                text
            } else {
                "NONE"
            }
        }
        None => "NONE",
    };
    let axes = axes.map(|a| format!("; {a}")).unwrap_or_default();
    Ok(Some(format!("{head}({reference}{axes}).{property}")))
}

/// The value at the dotted `key` of `old`.
fn lookup<'a>(old: &'a Value, key: &str) -> Option<&'a Value> {
    key.split('.')
        .try_fold(old, |value, segment| value.get(segment))
}

/// The leaves of a 0.7.2 border whose serde name is not their field's name (`model/border.rs`).
const BORDER_SERDE_NAMES: [(&str, &str); 3] = [
    ("corner_radius", "corner_radius_px"),
    ("corner_radius_lg", "corner_radius_lg_px"),
    ("line_width", "line_width_px"),
];

/// Whether the unresolved 0.7.2 theme `unresolved` states the field of `key`: the field's path there is
/// the key's, a border's leaf by its serde name; a field the theme leaves `None` is absent.
fn unresolved_states(unresolved: &Value, key: &str) -> bool {
    let path = match key.rsplit_once('.') {
        Some((parent, leaf)) if parent == "border" || parent.ends_with(".border") => {
            match BORDER_SERDE_NAMES.iter().find(|(name, _)| *name == leaf) {
                Some((_, serde_name)) => format!("{parent}.{serde_name}"),
                None => key.to_owned(),
            }
        }
        _ => key.to_owned(),
    };
    lookup(unresolved, &path).is_some_and(|v| !v.is_null())
}

/// `value`, the old value of `row`'s key, or where it is null the value its `if_null` chain ends in: the
/// `if_null` key's value, and where that is null too, the value its own row's `if_null` names, and so on;
/// where the chain ends in a null key, null.
fn follow_if_null<'a>(
    row: &Row,
    rows: &[Row],
    value: &'a Value,
    old: &'a Value,
) -> Result<&'a Value, String> {
    let mut value = value;
    let mut next = row.if_null.as_deref();
    let mut seen = vec![row.key.as_str()];
    while value.is_null()
        && let Some(other) = next
    {
        if seen.contains(&other) {
            return Err(format!("its if_null chain returns to `{other}`"));
        }
        seen.push(other);
        value =
            lookup(old, other).ok_or_else(|| format!("its if_null key `{other}` is not in old"))?;
        next = rows
            .iter()
            .find(|r| r.key == other)
            .and_then(|r| r.if_null.as_deref());
    }
    Ok(value)
}

/// The value 0.8.0 has at every home of `row` on `preset` where the row is a listed difference there:
/// "unstated" as `null`, any other value as JSON.
fn listed_difference(row: &Row, preset: Preset) -> Result<Option<Value>, String> {
    let listed = row
        .difference_presets
        .iter()
        .flatten()
        .any(|p| p == preset.as_str());
    if !listed {
        return Ok(None);
    }
    let value = row
        .difference_values
        .as_ref()
        .and_then(|values| values.get(preset.as_str()))
        .ok_or_else(|| format!("no difference_values for `{}`", preset.as_str()))?;
    match value {
        toml::Value::String(s) if s == "unstated" => Ok(Some(Value::Null)),
        toml::Value::String(s) => Ok(Some(Value::String(s.clone()))),
        toml::Value::Integer(i) => Ok(Some(Value::from(*i))),
        toml::Value::Float(x) => Ok(Some(Value::from(*x))),
        toml::Value::Boolean(b) => Ok(Some(Value::Bool(*b))),
        other => Err(format!("difference value `{other}` is not a single value")),
    }
}

/// `old` for the report: as JSON, a number also as the `f32` it reads as, with its bits.
fn show_old(old: &Value) -> String {
    match old.as_f64() {
        Some(x) => format!("{old} (f32 {})", show_f32(narrow(x))),
        None => old.to_string(),
    }
}

/// `v` in full precision: the shortest form that reads back as `v`, and its bits.
fn show_f32(v: f32) -> String {
    format!("{v:?} [{:#010x}]", v.to_bits())
}

/// An `f64` that `serde_json` holds for an `f32` of 0.7.2 (or a table's TOML float) as that `f32`: the
/// widening was exact, so the narrowing is.
fn narrow(x: f64) -> f32 {
    x as f32
}

// ---------------------------------------------------------------------------------------------------------
// The value at a home.
// ---------------------------------------------------------------------------------------------------------

/// A value read at a home, by kind.
#[derive(Clone, Debug, PartialEq)]
enum Got {
    /// The `stated` bit clear, or an `Option` slot `None`.
    Unstated,
    /// An `Option<TextStyle>` slot of `Typography` that is `None` (the row's `home_fallback` applies).
    NoSlot,
    Color(Color),
    F32(f32),
    Int(u64),
    Bool(bool),
    Str(String),
    FontSize(FontSize),
    FontStyle(FontStyle),
    Scheme(Scheme),
    IconSet(IconSet),
    FocusTrigger(FocusTrigger),
    Motion(MotionPreference),
    Contrast(Contrast),
    TabIndicatorSide(TabIndicatorSide),
    DialogButtonOrder(DialogButtonOrder),
    DialogEntrance(DialogEntranceStyle),
    ArrowSide(ArrowSide),
    /// A part's `shadows` with the `SHADOWS` bit clear.
    ShadowsUnstated,
    /// A part's `shadows` stated as `None`.
    ShadowsNone,
    /// A value of a form no 0.7.2 field holds, as its `Debug` text.
    Other(String),
}

impl fmt::Display for Got {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unstated => f.write_str("unstated"),
            Self::NoSlot => f.write_str("no slot"),
            Self::Color(c) => {
                let [r, g, b, a] = c.to_srgb8();
                write!(f, "{c:?} = 8-bit [{r}, {g}, {b}, {a}]")
            }
            Self::F32(v) => f.write_str(&show_f32(*v)),
            Self::FontSize(size) => {
                let (unit, v) = font_size_parts(*size);
                write!(f, "{unit}({})", show_f32(v))
            }
            Self::ShadowsUnstated => f.write_str("SHADOWS unstated"),
            Self::ShadowsNone => f.write_str("SHADOWS stated, no shadow"),
            Self::Other(text) => write!(f, "{text} (no 0.7.2 value)"),
            other => write!(f, "{other:?}"),
        }
    }
}

/// Whether `old` and `got` are the same value, or why they cannot be compared.
fn equal(old: &Value, got: &Got) -> Result<bool, String> {
    let kind_error = || format!("the old value {old} is not of the home's kind ({got})");
    match (got, old) {
        (Got::ShadowsUnstated, Value::Null) => Ok(true),
        (Got::ShadowsUnstated, Value::Bool(enabled)) => Ok(*enabled),
        (Got::ShadowsNone, Value::Null) => Ok(false),
        (Got::ShadowsNone, Value::Bool(enabled)) => Ok(!*enabled),
        (Got::ShadowsUnstated | Got::ShadowsNone, _) => Err(kind_error()),
        (Got::Unstated | Got::NoSlot, old) => Ok(old.is_null()),
        (_, Value::Null) | (Got::Other(_), _) => Ok(false),
        (Got::Color(c), Value::String(s)) => {
            let rgba = crate::Rgba::from_str(s).map_err(|_| kind_error())?;
            // `to_srgb8` clamps and maps a non-finite channel to 0: only a colour every channel of which is
            // finite and in 0..=1 is compared by its 8-bit values.
            Ok(c.clamped() == *c && c.to_srgb8() == [rgba.r, rgba.g, rgba.b, rgba.a])
        }
        (Got::F32(v), Value::Number(n)) => {
            let x = n.as_f64().ok_or_else(kind_error)?;
            Ok(narrow(x).to_bits() == v.to_bits())
        }
        (Got::Int(v), Value::Number(n)) => Ok(n.as_u64().ok_or_else(kind_error)? == *v),
        (Got::Bool(v), Value::Bool(b)) => Ok(v == b),
        (Got::Str(v), Value::String(s)) => Ok(v == s),
        (Got::FontSize(size), Value::Object(map)) => {
            let mut entries = map.iter();
            let (Some((unit, n)), None) = (entries.next(), entries.next()) else {
                return Err(kind_error());
            };
            let x = n.as_f64().ok_or_else(kind_error)?;
            let (new_unit, v) = font_size_parts(*size);
            Ok(unit == new_unit && narrow(x).to_bits() == v.to_bits())
        }
        (Got::FontStyle(style), _) => {
            for (old_style, new_style) in FONT_STYLES {
                if serde_json::to_value(old_style).map_err(|e| e.to_string())? == *old {
                    return Ok(new_style == *style);
                }
            }
            Err(kind_error())
        }
        (Got::Scheme(scheme), Value::String(s)) => SCHEMES
            .iter()
            .find(|(mode, _)| format!("{mode:?}") == *s)
            .map(|(_, mapped)| mapped == scheme)
            .ok_or_else(kind_error),
        (Got::IconSet(set), _) => Ok(serde_json::to_value(set).map_err(|e| e.to_string())? == *old),
        (Got::FocusTrigger(trigger), Value::Bool(b)) => FOCUS_TRIGGERS
            .iter()
            .find(|(keyboard_only, _)| keyboard_only == b)
            .map(|(_, mapped)| mapped == trigger)
            .ok_or_else(kind_error),
        // The table's notes: `reduce_motion` is `motion != Full`, `high_contrast` is `contrast != Normal`.
        (Got::Motion(motion), Value::Bool(b)) => Ok((*motion != MotionPreference::Full) == *b),
        (Got::Contrast(contrast), Value::Bool(b)) => Ok((*contrast != Contrast::Normal) == *b),
        (Got::TabIndicatorSide(side), _) => {
            for (old_side, new_side) in TAB_INDICATOR_SIDES {
                if serde_json::to_value(old_side).map_err(|e| e.to_string())? == *old {
                    return Ok(new_side == *side);
                }
            }
            Err(kind_error())
        }
        (Got::DialogButtonOrder(order), _) => {
            same_variant(old, &DIALOG_BUTTON_ORDERS, order)?.ok_or_else(kind_error)
        }
        (Got::DialogEntrance(entrance), _) => {
            same_variant(old, &DIALOG_ENTRANCES, entrance)?.ok_or_else(kind_error)
        }
        (Got::ArrowSide(side), _) => same_variant(old, &ARROW_SIDES, side)?.ok_or_else(kind_error),
        _ => Err(kind_error()),
    }
}

/// The value at `home` in `theme`, or in `snapshot_context` for a `snapshot.context.` home.
fn read(
    home: &str,
    theme: &ResolvedTheme,
    snapshot_context: Option<&ThemeContext>,
) -> Result<Got, String> {
    let unreadable = || format!("the home `{home}` is no form the comparator reads");
    if let Some(field) = home.strip_prefix(SNAPSHOT_PREFIX) {
        let context = snapshot_context.ok_or_else(|| format!("`{home}` needs a snapshot"))?;
        return context_field(context, field).ok_or_else(unreadable);
    }
    if home == "name" {
        return Ok(Got::Str(theme.name.to_string()));
    }
    if let Some(rest) = home.strip_prefix(WINDOW_PREFIX) {
        return window(theme, rest).ok_or_else(unreadable);
    }
    let (head, rest) = home.split_once('.').ok_or_else(unreadable)?;
    let got = match head {
        "context" => context_field(&theme.context, rest),
        "colors" => colors(theme, rest),
        "typography" => typography(theme, rest),
        "icons" => icons(theme, rest),
        "layout" => layout(theme, rest),
        "timings" => timings(theme, rest),
        "focus_ring" => focus_ring(theme, rest),
        "caret" => caret(theme, rest),
        "tooltip" => tooltip(theme, rest),
        "status_bar" => status_bar(theme, rest),
        "popover" => popover(theme, rest),
        "dialog" => dialog(theme, rest),
        widget => match metric(theme, widget, rest) {
            Some(got) => Some(got),
            None => ruled_part(theme, widget, rest).map_err(|e| format!("`{home}`: {e}"))?,
        },
    };
    got.ok_or_else(unreadable)
}

fn opt_color(v: Option<Color>) -> Got {
    v.map_or(Got::Unstated, Got::Color)
}

fn opt_f32(v: Option<f32>) -> Got {
    v.map_or(Got::Unstated, Got::F32)
}

fn opt_u32(v: Option<u32>) -> Got {
    v.map_or(Got::Unstated, |v| Got::Int(u64::from(v)))
}

fn opt_str(v: Option<&str>) -> Got {
    v.map_or(Got::Unstated, |v| Got::Str(v.to_owned()))
}

fn context_field(context: &ThemeContext, field: &str) -> Option<Got> {
    Some(match field {
        "scheme" => Got::Scheme(context.scheme),
        "contrast" => Got::Contrast(context.contrast),
        "motion" => Got::Motion(context.motion),
        "reduced_transparency" => Got::Bool(context.reduced_transparency),
        "bold_text" => Got::Bool(context.bold_text),
        "button_shapes" => Got::Bool(context.button_shapes),
        "differentiate_without_color" => Got::Bool(context.differentiate_without_color),
        "window_active" => Got::Bool(context.window_active),
        "text_scale" => Got::F32(context.text_scale),
        _ => return None,
    })
}

fn surface(colors: &SurfaceColors, field: &str) -> Option<Got> {
    Some(opt_color(match field {
        "background" => colors.background,
        "foreground" => colors.foreground,
        "foreground_muted" => colors.foreground_muted,
        "border" => colors.border,
        "divider" => colors.divider,
        _ => return None,
    }))
}

fn colors(theme: &ResolvedTheme, rest: &str) -> Option<Got> {
    let c = &theme.colors;
    let single = match rest {
        "find_highlight" => Some(opt_color(c.find_highlight)),
        "scrim" => Some(opt_color(c.scrim)),
        "link_visited" => Some(opt_color(c.link_visited)),
        "focus_ring" => Some(Got::Color(c.focus_ring)),
        "caret" => Some(Got::Color(c.caret)),
        "link" => Some(Got::Color(c.link)),
        "disabled_text" => Some(Got::Color(c.disabled_text)),
        "shadow" => Some(Got::Color(c.shadow)),
        "disabled_opacity" => Some(Got::F32(c.disabled_opacity)),
        _ => None,
    };
    if single.is_some() {
        return single;
    }
    let (group, field) = rest.split_once('.')?;
    let surface_colors = match group {
        "window" => Some(&c.window),
        "view" => Some(&c.view),
        "header" => Some(&c.header),
        "sidebar" => Some(&c.sidebar),
        "card" => Some(&c.card),
        "dialog" => Some(&c.dialog),
        "popover" => Some(&c.popover),
        "tooltip" => Some(&c.tooltip),
        "osd" => Some(&c.osd),
        _ => None,
    };
    if let Some(s) = surface_colors {
        return surface(s, field);
    }
    let selection = match group {
        "selection" => Some(&c.selection),
        "text_selection" => Some(&c.text_selection),
        _ => None,
    };
    if let Some(s) = selection {
        return Some(match field {
            "background" => Got::Color(s.background),
            "foreground" => Got::Color(s.foreground),
            "unfocused_background" => opt_color(s.unfocused_background),
            "unfocused_foreground" => opt_color(s.unfocused_foreground),
            _ => return None,
        });
    }
    let (fill, foreground, standalone) = match group {
        "accent" => (c.accent.fill, c.accent.foreground, c.accent.standalone),
        "error" => (c.error.fill, c.error.foreground, c.error.standalone),
        "warning" => (c.warning.fill, c.warning.foreground, c.warning.standalone),
        "success" => (c.success.fill, c.success.foreground, c.success.standalone),
        "info" => (c.info.fill, c.info.foreground, c.info.standalone),
        _ => return None,
    };
    Some(match field {
        "fill" => Got::Color(fill),
        "foreground" => Got::Color(foreground),
        "standalone" => opt_color(standalone),
        _ => return None,
    })
}

fn typography(theme: &ResolvedTheme, rest: &str) -> Option<Got> {
    let t = &theme.typography;
    let single = match rest {
        "line_height_multiplier" => Some(Got::F32(t.line_height_multiplier)),
        "font_dpi" => Some(Got::F32(t.font_dpi)),
        "font_whole_pixels" => Some(t.font_whole_pixels.map_or(Got::Unstated, Got::Bool)),
        "emoji_family" => Some(opt_str(t.emoji_family.as_deref())),
        _ => None,
    };
    if single.is_some() {
        return single;
    }
    let (slot, property) = rest.split_once('.')?;
    let style = match slot {
        "body" => Some(&t.body),
        "monospace" => Some(&t.monospace),
        "display" => t.display.as_ref(),
        "title1" => t.title1.as_ref(),
        "title2" => t.title2.as_ref(),
        "title3" => t.title3.as_ref(),
        "title4" => t.title4.as_ref(),
        "headline" => t.headline.as_ref(),
        "callout" => t.callout.as_ref(),
        "subheadline" => t.subheadline.as_ref(),
        "caption" => t.caption.as_ref(),
        _ => return None,
    };
    match style {
        Some(style) => text_property(style, property),
        None => text_property(&TextStyle::default(), property).map(|_| Got::NoSlot),
    }
}

fn icons(theme: &ResolvedTheme, rest: &str) -> Option<Got> {
    let i = &theme.icons;
    Some(match rest {
        "set" => Got::IconSet(i.set),
        "theme" => opt_str(i.theme.as_deref()),
        "theme_explicit" => Got::Bool(i.theme_explicit),
        "sizes.toolbar" => opt_f32(i.sizes.toolbar),
        "sizes.small" => opt_f32(i.sizes.small),
        "sizes.large" => opt_f32(i.sizes.large),
        "sizes.dialog" => opt_f32(i.sizes.dialog),
        "sizes.panel" => opt_f32(i.sizes.panel),
        _ => return None,
    })
}

fn layout(theme: &ResolvedTheme, rest: &str) -> Option<Got> {
    let l = &theme.layout;
    Some(match rest {
        "widget_gap" => opt_f32(l.widget_gap),
        "container_margin" => opt_f32(l.container_margin),
        "window_margin" => opt_f32(l.window_margin),
        "section_gap" => opt_f32(l.section_gap),
        "corner_radius" => Got::F32(l.corner_radius),
        "corner_radius_lg" => Got::F32(l.corner_radius_lg),
        "border_width" => Got::F32(l.border_width),
        "border_opacity" => Got::F32(l.border_opacity),
        "shadow_enabled" => Got::Bool(l.shadow_enabled),
        "cursor_size_px" => opt_f32(l.cursor_size_px),
        "cursor_theme" => opt_str(l.cursor_theme.as_deref()),
        _ => return None,
    })
}

fn timings(theme: &ResolvedTheme, rest: &str) -> Option<Got> {
    let t = &theme.timings;
    Some(match rest {
        "tooltip_show_ms" => opt_u32(t.tooltip_show_ms),
        "tooltip_hide_ms" => opt_u32(t.tooltip_hide_ms),
        "tooltip_reshow_ms" => opt_u32(t.tooltip_reshow_ms),
        "double_click_ms" => opt_u32(t.double_click_ms),
        "drag_threshold_px" => opt_f32(t.drag_threshold_px),
        "scrollbar_auto_hide_ms" => opt_u32(t.scrollbar_auto_hide_ms),
        "menu_hover_open_ms" => opt_u32(t.menu_hover_open_ms),
        "key_repeat_delay_ms" => opt_u32(t.key_repeat_delay_ms),
        "key_repeat_interval_ms" => opt_u32(t.key_repeat_interval_ms),
        _ => return None,
    })
}

fn focus_ring(theme: &ResolvedTheme, rest: &str) -> Option<Got> {
    let ring = &theme.focus_ring;
    match rest {
        "offset" => Some(Got::F32(ring.offset)),
        "trigger" => Some(ring.trigger.map_or(Got::Unstated, Got::FocusTrigger)),
        _ => match rest.split_once('.')? {
            ("stroke", field) => stroke_field(&ring.stroke, field),
            ("inner_stroke", field) => stroke_field(&ring.inner_stroke, field),
            _ => None,
        },
    }
}

fn caret(theme: &ResolvedTheme, rest: &str) -> Option<Got> {
    let c = &theme.caret;
    Some(match rest {
        "width" => opt_f32(c.width),
        "blink_on_ms" => opt_u32(c.blink_on_ms),
        "blink_off_ms" => opt_u32(c.blink_off_ms),
        "blink_timeout_ms" => opt_u32(c.blink_timeout_ms),
        _ => return None,
    })
}

/// `<getter>(normal).<property>` of `WindowChrome`, at `WindowState::default()`.
fn window(theme: &ResolvedTheme, rest: &str) -> Option<Got> {
    let (getter, property) = rest.split_once("(normal).")?;
    let normal = WindowState::default();
    match getter {
        "frame" => part_property(&theme.window.frame(normal), property),
        "title_bar" => part_property(&theme.window.title_bar(normal), property),
        "title" => text_property(&theme.window.title(normal), property),
        _ => None,
    }
}

fn tooltip(theme: &ResolvedTheme, rest: &str) -> Option<Got> {
    match rest.split_once('.')? {
        ("panel", property) => part_property(&theme.tooltip.panel, property),
        ("text", property) => text_property(&theme.tooltip.text, property),
        _ => None,
    }
}

fn status_bar(theme: &ResolvedTheme, rest: &str) -> Option<Got> {
    match rest.split_once('.')? {
        ("bar", property) => part_property(&theme.status_bar.bar, property),
        ("label", property) => text_property(&theme.status_bar.label, property),
        ("separator", property) => part_property(&theme.status_bar.separator, property),
        _ => None,
    }
}

fn popover(theme: &ResolvedTheme, rest: &str) -> Option<Got> {
    match rest.split_once('.')? {
        ("panel", property) => part_property(&theme.popover.panel, property),
        ("arrow", property) => part_property(&theme.popover.arrow, property),
        ("text", property) => text_property(&theme.popover.text, property),
        _ => None,
    }
}

fn dialog(theme: &ResolvedTheme, rest: &str) -> Option<Got> {
    let d = &theme.dialog;
    match rest {
        "button_gap" => return Some(Got::F32(d.button_gap)),
        "button_order" => {
            return Some(d.button_order.map_or(Got::Unstated, Got::DialogButtonOrder));
        }
        "entrance" => return Some(d.entrance.map_or(Got::Unstated, Got::DialogEntrance)),
        _ => {}
    }
    match rest.split_once('.')? {
        ("panel", property) => part_property(&d.panel, property),
        ("title", property) => text_property(&d.title, property),
        ("body", property) => text_property(&d.body, property),
        ("icon", property) => icon_property(&d.icon, property),
        _ => None,
    }
}

/// A direct metric of a widget.
fn metric(theme: &ResolvedTheme, widget: &str, name: &str) -> Option<Got> {
    Some(match (widget, name) {
        ("button", "icon_text_gap") => Got::F32(theme.button.icon_text_gap),
        ("button", "disabled_opacity") => opt_f32(theme.button.disabled_opacity),
        ("checkbox", "label_gap") => Got::F32(theme.checkbox.label_gap),
        ("checkbox", "disabled_opacity") => opt_f32(theme.checkbox.disabled_opacity),
        ("radio", "label_gap") => Got::F32(theme.radio.label_gap),
        ("radio", "disabled_opacity") => opt_f32(theme.radio.disabled_opacity),
        ("scrollbar", "min_thumb_length") => Got::F32(theme.scrollbar.min_thumb_length),
        ("scrollbar", "overlay_mode") => Got::Bool(theme.scrollbar.overlay_mode),
        ("list", "grid_color") => Got::Color(theme.list.grid_color),
        ("input", "disabled_opacity") => opt_f32(theme.input.disabled_opacity),
        ("text_area", "disabled_opacity") => opt_f32(theme.text_area.disabled_opacity),
        ("slider", "tick_length") => Got::F32(theme.slider.tick_length),
        ("slider", "disabled_opacity") => opt_f32(theme.slider.disabled_opacity),
        ("progress_bar", "indeterminate_period_ms") => {
            opt_u32(theme.progress_bar.indeterminate_period_ms)
        }
        ("splitter", "divider_width") => Got::F32(theme.splitter.divider_width),
        ("separator", "color") => Got::Color(theme.separator.color),
        ("separator", "thickness") => Got::F32(theme.separator.thickness),
        ("separator", "inset") => opt_f32(theme.separator.inset),
        ("separator", "margin") => opt_f32(theme.separator.margin),
        ("spinner", "color") => Got::Color(theme.spinner.color),
        ("spinner", "diameter") => Got::F32(theme.spinner.diameter),
        ("spinner", "min_diameter") => Got::F32(theme.spinner.min_diameter),
        ("spinner", "stroke_width") => Got::F32(theme.spinner.stroke_width),
        ("spinner", "period_ms") => opt_u32(theme.spinner.period_ms),
        ("tab", "item_gap") => opt_f32(theme.tab.item_gap),
        ("tab", "indicator_side") => theme
            .tab
            .indicator_side
            .map_or(Got::Unstated, Got::TabIndicatorSide),
        ("tab", "active_outward_corner_radius") => opt_f32(theme.tab.active_outward_corner_radius),
        ("menu", "icon_text_gap") => Got::F32(theme.menu.icon_text_gap),
        ("menu_bar", "item_gap") => opt_f32(theme.menu_bar.item_gap),
        ("sidebar", "item_gap") => opt_f32(theme.sidebar.item_gap),
        ("toolbar", "item_gap") => opt_f32(theme.toolbar.item_gap),
        ("switch", "label_gap") => opt_f32(theme.switch.label_gap),
        ("switch", "show_state_glyphs") => theme
            .switch
            .show_state_glyphs
            .map_or(Got::Unstated, Got::Bool),
        ("switch", "disabled_opacity") => opt_f32(theme.switch.disabled_opacity),
        ("combo_box", "arrow_area_width") => opt_f32(theme.combo_box.arrow_area_width),
        ("combo_box", "disabled_opacity") => opt_f32(theme.combo_box.disabled_opacity),
        ("segmented_control", "separator_width") => {
            Got::F32(theme.segmented_control.separator_width)
        }
        ("segmented_control", "disabled_opacity") => {
            opt_f32(theme.segmented_control.disabled_opacity)
        }
        ("expander", "arrow_side") => theme
            .expander
            .arrow_side
            .map_or(Got::Unstated, Got::ArrowSide),
        ("expander", "arrow_gap") => opt_f32(theme.expander.arrow_gap),
        ("expander", "content_indent") => opt_f32(theme.expander.content_indent),
        ("expander", "frame_enabled") => theme
            .expander
            .frame_enabled
            .map_or(Got::Unstated, Got::Bool),
        _ => return None,
    })
}

/// A ruled part's style for a home's query.
enum Ruled {
    Part(PartStyle),
    Text(TextStyle),
    Icon(IconStyle),
}

/// `<part>(<query>).<property>` of a ruled widget: the style of the part for the query and the property,
/// `None` where the widget or part is none the comparator reads, an error where the query does not parse.
fn ruled_style<'a>(
    theme: &ResolvedTheme,
    widget: &str,
    rest: &'a str,
) -> Result<Option<(Ruled, &'a str)>, String> {
    let Some((part, after)) = rest.split_once('(') else {
        return Ok(None);
    };
    let Some((query, property)) = after.split_once(").") else {
        return Ok(None);
    };
    let q = Query::parse(query)?;
    let style = match (widget, part) {
        ("button", "body") => Ruled::Part(theme.button.body(q.button()?)),
        ("button", "label") => Ruled::Text(theme.button.label(q.button()?)),
        ("button", "icon") => Ruled::Icon(theme.button.icon(q.button()?)),
        ("checkbox", "indicator") => Ruled::Part(theme.checkbox.indicator(q.style()?)),
        ("checkbox", "checkmark") => Ruled::Icon(theme.checkbox.checkmark(q.style()?)),
        ("checkbox", "label") => Ruled::Text(theme.checkbox.label(q.style()?)),
        ("radio", "indicator") => Ruled::Part(theme.radio.indicator(q.style()?)),
        ("radio", "dot") => Ruled::Part(theme.radio.dot(q.style()?)),
        ("radio", "label") => Ruled::Text(theme.radio.label(q.style()?)),
        ("scrollbar", "track") => Ruled::Part(theme.scrollbar.track(q.style()?)),
        ("scrollbar", "thumb") => Ruled::Part(theme.scrollbar.thumb(q.style()?)),
        ("list", "container") => Ruled::Part(theme.list.container(q.list()?)),
        ("list", "row") => Ruled::Part(theme.list.row(q.list()?)),
        ("list", "highlight") => Ruled::Part(theme.list.highlight(q.list()?)),
        ("list", "item_label") => Ruled::Text(theme.list.item_label(q.list()?)),
        ("list", "header") => Ruled::Part(theme.list.header(q.list()?)),
        ("list", "header_label") => Ruled::Text(theme.list.header_label(q.list()?)),
        ("list", "header_separator") => Ruled::Part(theme.list.header_separator(q.list()?)),
        ("input", "field") => Ruled::Part(theme.input.field(q.input()?)),
        ("input", "text") => Ruled::Text(theme.input.text(q.input()?)),
        ("input", "placeholder") => Ruled::Text(theme.input.placeholder(q.input()?)),
        ("input", "helper_text") => Ruled::Text(theme.input.helper_text(q.input()?)),
        ("text_area", "field") => Ruled::Part(theme.text_area.field(q.input()?)),
        ("text_area", "text") => Ruled::Text(theme.text_area.text(q.input()?)),
        ("text_area", "placeholder") => Ruled::Text(theme.text_area.placeholder(q.input()?)),
        ("text_area", "helper_text") => Ruled::Text(theme.text_area.helper_text(q.input()?)),
        ("link", "container") => Ruled::Part(theme.link.container(q.style()?)),
        ("link", "label") => Ruled::Text(theme.link.label(q.style()?)),
        ("slider", "track") => Ruled::Part(theme.slider.track(q.style()?)),
        ("slider", "active_track") => Ruled::Part(theme.slider.active_track(q.style()?)),
        ("slider", "thumb") => Ruled::Part(theme.slider.thumb(q.style()?)),
        ("slider", "tick") => Ruled::Part(theme.slider.tick(q.style()?)),
        ("progress_bar", "track") => Ruled::Part(theme.progress_bar.track(q.style()?)),
        ("progress_bar", "fill") => Ruled::Part(theme.progress_bar.fill(q.style()?)),
        ("splitter", "handle") => Ruled::Part(theme.splitter.handle(q.style()?)),
        ("tab", "bar") => Ruled::Part(theme.tab.bar(q.tab()?)),
        ("tab", "tab") => Ruled::Part(theme.tab.tab(q.tab()?)),
        ("tab", "label") => Ruled::Text(theme.tab.label(q.tab()?)),
        ("tab", "indicator") => Ruled::Part(theme.tab.indicator(q.tab()?)),
        ("tab", "close_button") => Ruled::Part(theme.tab.close_button(q.tab()?)),
        ("tab", "close_icon") => Ruled::Icon(theme.tab.close_icon(q.tab()?)),
        ("menu", "panel") => Ruled::Part(theme.menu.panel(q.style()?)),
        ("menu", "item") => Ruled::Part(theme.menu.item(q.style()?)),
        ("menu", "item_label") => Ruled::Text(theme.menu.item_label(q.style()?)),
        ("menu", "shortcut") => Ruled::Text(theme.menu.shortcut(q.style()?)),
        ("menu", "icon") => Ruled::Icon(theme.menu.icon(q.style()?)),
        ("menu", "submenu_arrow") => Ruled::Icon(theme.menu.submenu_arrow(q.style()?)),
        ("menu", "separator") => Ruled::Part(theme.menu.separator(q.style()?)),
        ("menu_bar", "bar") => Ruled::Part(theme.menu_bar.bar(q.style()?)),
        ("menu_bar", "item") => Ruled::Part(theme.menu_bar.item(q.style()?)),
        ("menu_bar", "item_label") => Ruled::Text(theme.menu_bar.item_label(q.style()?)),
        ("sidebar", "container") => Ruled::Part(theme.sidebar.container(q.style()?)),
        ("sidebar", "item") => Ruled::Part(theme.sidebar.item(q.style()?)),
        ("sidebar", "item_label") => Ruled::Text(theme.sidebar.item_label(q.style()?)),
        ("sidebar", "item_icon") => Ruled::Icon(theme.sidebar.item_icon(q.style()?)),
        ("sidebar", "section_header") => Ruled::Text(theme.sidebar.section_header(q.style()?)),
        ("toolbar", "bar") => Ruled::Part(theme.toolbar.bar(q.style()?)),
        ("toolbar", "button") => Ruled::Part(theme.toolbar.button(q.style()?)),
        ("toolbar", "button_label") => Ruled::Text(theme.toolbar.button_label(q.style()?)),
        ("toolbar", "button_icon") => Ruled::Icon(theme.toolbar.button_icon(q.style()?)),
        ("toolbar", "separator") => Ruled::Part(theme.toolbar.separator(q.style()?)),
        ("switch", "track") => Ruled::Part(theme.switch.track(q.style()?)),
        ("switch", "thumb") => Ruled::Part(theme.switch.thumb(q.style()?)),
        ("switch", "label") => Ruled::Text(theme.switch.label(q.style()?)),
        ("combo_box", "field") => Ruled::Part(theme.combo_box.field(q.style()?)),
        ("combo_box", "label") => Ruled::Text(theme.combo_box.label(q.style()?)),
        ("combo_box", "chevron") => Ruled::Icon(theme.combo_box.chevron(q.style()?)),
        ("segmented_control", "track") => Ruled::Part(theme.segmented_control.track(q.style()?)),
        ("segmented_control", "segment") => {
            Ruled::Part(theme.segmented_control.segment(q.style()?))
        }
        ("segmented_control", "label") => Ruled::Text(theme.segmented_control.label(q.style()?)),
        ("segmented_control", "separator") => {
            Ruled::Part(theme.segmented_control.separator(q.style()?))
        }
        ("card", "container") => Ruled::Part(theme.card.container(q.style()?)),
        ("expander", "container") => Ruled::Part(theme.expander.container(q.style()?)),
        ("expander", "header") => Ruled::Part(theme.expander.header(q.style()?)),
        ("expander", "header_label") => Ruled::Text(theme.expander.header_label(q.style()?)),
        ("expander", "arrow") => Ruled::Icon(theme.expander.arrow(q.style()?)),
        _ => return Ok(None),
    };
    Ok(Some((style, property)))
}

/// `<part>(<query>).<property>` of a ruled widget, `None` where the widget or part is none the comparator
/// reads, an error where the query does not parse.
fn ruled_part(theme: &ResolvedTheme, widget: &str, rest: &str) -> Result<Option<Got>, String> {
    Ok(match ruled_style(theme, widget, rest)? {
        Some((Ruled::Part(style), property)) => part_property(&style, property),
        Some((Ruled::Text(style), property)) => text_property(&style, property),
        Some((Ruled::Icon(style), property)) => icon_property(&style, property),
        None => None,
    })
}

/// The part style at `home`, a part's property (`tooltip.panel.<property>` or a ruled part's), `None`
/// where the home is no part the comparator reads or a text or icon part.
fn part_style_at(home: &str, theme: &ResolvedTheme) -> Result<Option<PartStyle>, String> {
    let Some((widget, rest)) = home.split_once('.') else {
        return Ok(None);
    };
    if widget == "tooltip" {
        return Ok(rest
            .starts_with("panel.")
            .then(|| theme.tooltip.panel.clone()));
    }
    if widget == "window" {
        let normal = WindowState::default();
        return Ok(match rest.split_once("(normal).") {
            Some(("frame", _)) => Some(theme.window.frame(normal)),
            Some(("title_bar", _)) => Some(theme.window.title_bar(normal)),
            _ => None,
        });
    }
    Ok(match ruled_style(theme, widget, rest)? {
        Some((Ruled::Part(style), _)) => Some(style),
        Some((Ruled::Text(_) | Ruled::Icon(_), _)) | None => None,
    })
}

/// Whether the property at the ruled part's `home` got its value from a rule that requires a state (its
/// `from_state` bit); `None` for a property without a bit (`defined_size`), an error for a home
/// that is no ruled part's property.
fn from_state_at(home: &str, theme: &ResolvedTheme) -> Result<Option<bool>, String> {
    let unreadable = || format!("the home `{home}` is no ruled part's property");
    let (widget, rest) = home.split_once('.').ok_or_else(unreadable)?;
    let (style, property) = ruled_style(theme, widget, rest)?.ok_or_else(unreadable)?;
    Ok(match style {
        Ruled::Part(style) => part_bit(property).map(|bit| style.from_state.contains(bit)),
        Ruled::Text(style) => text_bit(property).map(|bit| style.from_state.contains(bit)),
        Ruled::Icon(style) => icon_bit(property).map(|bit| style.from_state.contains(bit)),
    })
}

/// The `from_state` bit of a part's property the comparator reads.
fn part_bit(property: &str) -> Option<PartStated> {
    Some(match property {
        "min_size[0]" | "min_size[1]" => PartStated::MIN_SIZE,
        "max_size[0]" | "max_size[1]" => PartStated::MAX_SIZE,
        "fill" => PartStated::FILL,
        "stroke.color" | "stroke.width" => PartStated::STROKE,
        "corners" => PartStated::CORNERS,
        "shadows" => PartStated::SHADOWS,
        "opacity" => PartStated::OPACITY,
        "padding.top" => PartStated::PADDING_TOP,
        "padding.right" => PartStated::PADDING_RIGHT,
        "padding.bottom" => PartStated::PADDING_BOTTOM,
        "padding.left" => PartStated::PADDING_LEFT,
        "margin.top" => PartStated::MARGIN_TOP,
        "margin.right" => PartStated::MARGIN_RIGHT,
        "margin.bottom" => PartStated::MARGIN_BOTTOM,
        "margin.left" => PartStated::MARGIN_LEFT,
        _ => return None,
    })
}

/// The bit of a text property the comparator reads, `None` for `defined_size` (it has none).
fn text_bit(property: &str) -> Option<TextStated> {
    Some(match property {
        "family" => TextStated::FAMILY,
        "color" => TextStated::COLOR,
        "size_px" => TextStated::SIZE_PX,
        "weight" => TextStated::WEIGHT,
        "style" => TextStated::STYLE,
        "line_height" => TextStated::LINE_HEIGHT,
        "letter_spacing" => TextStated::LETTER_SPACING,
        "decoration" => TextStated::DECORATION,
        _ => return None,
    })
}

/// The bit of an icon property the comparator reads.
fn icon_bit(property: &str) -> Option<IconStated> {
    Some(match property {
        "size" => IconStated::SIZE,
        "color" => IconStated::COLOR,
        "opacity" => IconStated::OPACITY,
        "weight" => IconStated::WEIGHT,
        "stroke_width" => IconStated::STROKE_WIDTH,
        _ => return None,
    })
}

fn stroke_field(stroke: &Stroke, field: &str) -> Option<Got> {
    Some(match (stroke, field) {
        (Stroke::Uniform { color, .. }, "color") => Got::Color(*color),
        (Stroke::Uniform { width, .. }, "width") => Got::F32(*width),
        (other, "color" | "width") => Got::Other(format!("{other:?}")),
        _ => return None,
    })
}

/// A part's stroke field: as [`stroke_field`] in every alignment, as the stroke is a band of its width
/// inside the part's box whatever its `align`, which says only how far the fill runs under it (spec §2,
/// "Fill and stroke"); and no stroke is the width 0.0 (and no colour).
fn part_stroke(stroke: &Stroke, field: &str) -> Option<Got> {
    Some(match (stroke, field) {
        (Stroke::None, "width") => Got::F32(0.0),
        _ => return stroke_field(stroke, field),
    })
}

fn part_property(style: &PartStyle, property: &str) -> Option<Got> {
    let stated = |bit: PartStated, got: Got| {
        if style.stated.contains(bit) {
            got
        } else {
            Got::Unstated
        }
    };
    let [min_width, min_height] = style.min_size;
    let [max_width, max_height] = style.max_size;
    Some(match property {
        "fill" => stated(
            PartStated::FILL,
            match &style.fill {
                Paint::Solid(c) => Got::Color(*c),
                other => Got::Other(format!("{other:?}")),
            },
        ),
        "stroke.color" => stated(PartStated::STROKE, part_stroke(&style.stroke, "color")?),
        "stroke.width" => stated(PartStated::STROKE, part_stroke(&style.stroke, "width")?),
        "corners" => stated(
            PartStated::CORNERS,
            match (style.corners.radius, style.corners.curve) {
                (CornerRadius::None, _) => Got::F32(0.0),
                (CornerRadius::Uniform(r), CornerCurve::Circular) => Got::F32(r),
                _ => Got::Other(format!("{:?}", style.corners)),
            },
        ),
        "shadows" => match (style.stated.contains(PartStated::SHADOWS), &style.shadows) {
            (false, _) => Got::ShadowsUnstated,
            (true, None) => Got::ShadowsNone,
            (true, Some(layers)) => Got::Other(format!("{layers:?}")),
        },
        "opacity" => stated(PartStated::OPACITY, Got::F32(style.opacity)),
        "padding.top" => stated(PartStated::PADDING_TOP, Got::F32(style.padding.top)),
        "padding.right" => stated(PartStated::PADDING_RIGHT, Got::F32(style.padding.right)),
        "padding.bottom" => stated(PartStated::PADDING_BOTTOM, Got::F32(style.padding.bottom)),
        "padding.left" => stated(PartStated::PADDING_LEFT, Got::F32(style.padding.left)),
        "margin.top" => stated(PartStated::MARGIN_TOP, Got::F32(style.margin.top)),
        "margin.right" => stated(PartStated::MARGIN_RIGHT, Got::F32(style.margin.right)),
        "margin.bottom" => stated(PartStated::MARGIN_BOTTOM, Got::F32(style.margin.bottom)),
        "margin.left" => stated(PartStated::MARGIN_LEFT, Got::F32(style.margin.left)),
        "min_size[0]" => opt_f32(min_width),
        "min_size[1]" => opt_f32(min_height),
        "max_size[0]" => opt_f32(max_width),
        "max_size[1]" => opt_f32(max_height),
        _ => return None,
    })
}

fn text_property(style: &TextStyle, property: &str) -> Option<Got> {
    let stated = |bit: TextStated, got: Got| {
        if style.stated.contains(bit) {
            got
        } else {
            Got::Unstated
        }
    };
    Some(match property {
        "family" => stated(TextStated::FAMILY, Got::Str(style.family.to_string())),
        "color" => stated(TextStated::COLOR, Got::Color(style.color)),
        "size_px" => stated(TextStated::SIZE_PX, Got::F32(style.size_px)),
        "defined_size" => style.defined_size.map_or(Got::Unstated, Got::FontSize),
        "weight" => stated(TextStated::WEIGHT, Got::Int(u64::from(style.weight))),
        "style" => stated(TextStated::STYLE, Got::FontStyle(style.style)),
        "line_height" => stated(TextStated::LINE_HEIGHT, Got::F32(style.line_height)),
        "letter_spacing" => stated(TextStated::LETTER_SPACING, Got::F32(style.letter_spacing)),
        "decoration" => stated(TextStated::DECORATION, decoration(&style.decoration)),
        _ => return None,
    })
}

/// A text's decoration as 0.7.2's `underline_enabled` holds it: a plain underline is `true`, no line
/// `false`; a decoration with its own colour or thickness, or another line, is a form no 0.7.2 field holds.
fn decoration(decoration: &TextDecoration) -> Got {
    match (decoration.kind, decoration.color, decoration.thickness) {
        (DecorationKind::Underline, None, None) => Got::Bool(true),
        (DecorationKind::None, None, None) => Got::Bool(false),
        _ => Got::Other(format!("{decoration:?}")),
    }
}

fn icon_property(style: &IconStyle, property: &str) -> Option<Got> {
    let stated = |bit: IconStated, got: Got| {
        if style.stated.contains(bit) {
            got
        } else {
            Got::Unstated
        }
    };
    Some(match property {
        "size" => stated(IconStated::SIZE, Got::F32(style.size)),
        "color" => stated(IconStated::COLOR, Got::Color(style.color)),
        "opacity" => stated(IconStated::OPACITY, Got::F32(style.opacity)),
        "weight" => stated(IconStated::WEIGHT, Got::Int(u64::from(style.weight))),
        "stroke_width" => stated(
            IconStated::STROKE_WIDTH,
            style
                .stroke_width
                .map_or_else(|| Got::Other("None, stated".to_owned()), Got::F32),
        ),
        _ => return None,
    })
}

// ---------------------------------------------------------------------------------------------------------
// The query of a home.
// ---------------------------------------------------------------------------------------------------------

/// `WidgetState`'s bits by name (spec §3).
const STATE_NAMES: [(&str, WidgetState); 19] = [
    ("HOVER", WidgetState::HOVER),
    ("PRESSED", WidgetState::PRESSED),
    ("FOCUSED", WidgetState::FOCUSED),
    ("FOCUS_WITHIN", WidgetState::FOCUS_WITHIN),
    ("FOCUS_VISIBLE", WidgetState::FOCUS_VISIBLE),
    ("UNFOCUSED_VIEW", WidgetState::UNFOCUSED_VIEW),
    ("DISABLED", WidgetState::DISABLED),
    ("CHECKED", WidgetState::CHECKED),
    ("SELECTED", WidgetState::SELECTED),
    ("INDETERMINATE", WidgetState::INDETERMINATE),
    ("OPEN", WidgetState::OPEN),
    ("DEFAULT", WidgetState::DEFAULT),
    ("INVALID", WidgetState::INVALID),
    ("READ_ONLY", WidgetState::READ_ONLY),
    ("VISITED", WidgetState::VISITED),
    ("DRAGGED", WidgetState::DRAGGED),
    ("DROP_TARGET", WidgetState::DROP_TARGET),
    ("SCROLLED", WidgetState::SCROLLED),
    ("HOVER_WITHIN", WidgetState::HOVER_WITHIN),
];

/// The value of `name` in `values`, or an error naming the axis.
fn named<T: Copy>(axis: &str, name: &str, values: &[(&str, T)]) -> Result<T, String> {
    values
        .iter()
        .find(|(n, _)| *n == name)
        .map(|(_, v)| *v)
        .ok_or_else(|| format!("`{name}` is no value of `{axis}`"))
}

/// A home's query: `<states>[; <axis>=<value>, …]`, its variant axes kept as written until the widget's
/// query type is known.
#[derive(Default)]
struct Query<'a> {
    state: WidgetState,
    size: ControlSize,
    surface: Surface,
    position: GroupPosition,
    orientation: Orientation,
    variant: Option<&'a str>,
    shape: Option<&'a str>,
    alternate: Option<&'a str>,
}

impl<'a> Query<'a> {
    fn parse(text: &'a str) -> Result<Self, String> {
        let (states, axes) = match text.split_once("; ") {
            Some((states, axes)) => (states, Some(axes)),
            None => (text, None),
        };
        let mut q = Query::default();
        if states != "NONE" {
            for name in states.split('|') {
                q.state |= named("state", name, &STATE_NAMES)?;
            }
        }
        for axis in axes.into_iter().flat_map(|a| a.split(", ")) {
            let (name, value) = axis
                .split_once('=')
                .ok_or_else(|| format!("an axis is `<name>=<value>`, not `{axis}`"))?;
            match name {
                "size" => {
                    q.size = named(
                        name,
                        value,
                        &[
                            ("Mini", ControlSize::Mini),
                            ("Small", ControlSize::Small),
                            ("Regular", ControlSize::Regular),
                            ("Large", ControlSize::Large),
                            ("ExtraLarge", ControlSize::ExtraLarge),
                        ],
                    )?;
                }
                "surface" => {
                    q.surface = named(
                        name,
                        value,
                        &[
                            ("Window", Surface::Window),
                            ("View", Surface::View),
                            ("HeaderBar", Surface::HeaderBar),
                            ("Sidebar", Surface::Sidebar),
                            ("Card", Surface::Card),
                            ("Dialog", Surface::Dialog),
                            ("Popover", Surface::Popover),
                            ("Tooltip", Surface::Tooltip),
                            ("Osd", Surface::Osd),
                        ],
                    )?;
                }
                "position" => {
                    q.position = named(
                        name,
                        value,
                        &[
                            ("Single", GroupPosition::Single),
                            ("First", GroupPosition::First),
                            ("Middle", GroupPosition::Middle),
                            ("Last", GroupPosition::Last),
                        ],
                    )?;
                }
                "orientation" => {
                    q.orientation = named(
                        name,
                        value,
                        &[
                            ("Horizontal", Orientation::Horizontal),
                            ("Vertical", Orientation::Vertical),
                        ],
                    )?;
                }
                "variant" => q.variant = Some(value),
                "shape" => q.shape = Some(value),
                "alternate" => q.alternate = Some(value),
                other => return Err(format!("no axis `{other}`")),
            }
        }
        Ok(q)
    }

    fn base<V: Default>(&self) -> StyleQuery<V> {
        StyleQuery::new(self.state)
            .size(self.size)
            .surface(self.surface)
            .position(self.position)
            .orientation(self.orientation)
    }

    /// The query of a widget without variant axes.
    fn style(&self) -> Result<StyleQuery, String> {
        if self.variant.is_some() || self.shape.is_some() || self.alternate.is_some() {
            return Err("the widget's query has no variant axis".to_owned());
        }
        Ok(self.base())
    }

    fn button(&self) -> Result<ButtonQuery, String> {
        if self.alternate.is_some() {
            return Err("a button's query has no axis `alternate`".to_owned());
        }
        let mut kind = ButtonKind::default();
        if let Some(v) = self.variant {
            kind.variant = named(
                "variant",
                v,
                &[
                    ("Standard", ButtonVariant::Standard),
                    ("Primary", ButtonVariant::Primary),
                    ("Secondary", ButtonVariant::Secondary),
                    ("Destructive", ButtonVariant::Destructive),
                    ("Outlined", ButtonVariant::Outlined),
                    ("Flat", ButtonVariant::Flat),
                    ("Glass", ButtonVariant::Glass),
                    ("Link", ButtonVariant::Link),
                ],
            )?;
        }
        if let Some(s) = self.shape {
            kind.shape = named(
                "shape",
                s,
                &[
                    ("Default", ButtonShape::Default),
                    ("Pill", ButtonShape::Pill),
                    ("Circle", ButtonShape::Circle),
                    ("Square", ButtonShape::Square),
                ],
            )?;
        }
        Ok(self.base().variant(kind))
    }

    fn input(&self) -> Result<InputQuery, String> {
        if self.shape.is_some() || self.alternate.is_some() {
            return Err("an input's query has no axis but `variant`".to_owned());
        }
        let mut variant = InputVariant::default();
        if let Some(v) = self.variant {
            variant = named(
                "variant",
                v,
                &[
                    ("Standard", InputVariant::Standard),
                    ("Outlined", InputVariant::Outlined),
                    ("Filled", InputVariant::Filled),
                    ("Search", InputVariant::Search),
                    ("Borderless", InputVariant::Borderless),
                ],
            )?;
        }
        Ok(self.base().variant(variant))
    }

    fn list(&self) -> Result<ListQuery, String> {
        if self.shape.is_some() {
            return Err("a list's query has no axis `shape`".to_owned());
        }
        let mut kind = ListKind::default();
        if let Some(v) = self.variant {
            kind.variant = named(
                "variant",
                v,
                &[
                    ("Plain", ListVariant::Plain),
                    ("InsetGrouped", ListVariant::InsetGrouped),
                    ("Sidebar", ListVariant::Sidebar),
                    ("Table", ListVariant::Table),
                ],
            )?;
        }
        if let Some(a) = self.alternate {
            kind.alternate = named("alternate", a, &[("true", true), ("false", false)])?;
        }
        Ok(self.base().variant(kind))
    }

    fn tab(&self) -> Result<TabQuery, String> {
        if self.shape.is_some() || self.alternate.is_some() {
            return Err("a tab's query has no axis but `variant`".to_owned());
        }
        let mut variant = TabVariant::default();
        if let Some(v) = self.variant {
            variant = named(
                "variant",
                v,
                &[
                    ("Standard", TabVariant::Standard),
                    ("Document", TabVariant::Document),
                    ("Pill", TabVariant::Pill),
                    ("Segmented", TabVariant::Segmented),
                ],
            )?;
        }
        Ok(self.base().variant(variant))
    }
}

#[cfg(test)]
mod tests;
