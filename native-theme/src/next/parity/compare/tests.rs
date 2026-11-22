use super::*;
use serde_json::json;
use std::sync::Arc;

use crate::next::geometry::{Corners, Edges};
use crate::next::preset::nothing_stated;
use crate::next::rules::{ListMatcher, PartProp, RuleBuilder, RuleSelector};
use crate::next::widgets::{ButtonWidget, CheckboxWidget, ListWidget, RadioWidget, WidgetOrigin};
use crate::next::window::EdgeFlags;
use crate::resolve::ResolutionContext;

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn srgb(hex: u32) -> Color {
    let [_, r, g, b] = hex.to_be_bytes();
    Color::from_srgb8(r, g, b, u8::MAX)
}

/// The `f32` one ULP above `v` (for a positive `v`).
fn next_up(v: f32) -> f32 {
    f32::from_bits(v.to_bits().saturating_add(1))
}

const NUMBER: f32 = 0.3;

/// One row per kind and per special home, every row of the same form as the table's.
const ROWS: &str = r#"
[[row]]
key = "c.color"
homes = ["colors.link"]
[[row]]
key = "c.number"
homes = ["layout.border_width"]
[[row]]
key = "c.int"
homes = ["timings.tooltip_show_ms"]
[[row]]
key = "c.bool"
homes = ["scrollbar.overlay_mode"]
[[row]]
key = "c.string"
homes = ["button.label(NONE).family"]
[[row]]
key = "c.style"
homes = ["button.label(NONE).style"]
[[row]]
key = "c.scheme"
homes = ["context.scheme"]
[[row]]
key = "c.icon_set"
homes = ["icons.set"]
[[row]]
key = "c.trigger"
homes = ["focus_ring.trigger"]
[[row]]
key = "c.defined_size"
homes = ["button.label(NONE).defined_size"]
[[row]]
key = "c.fill"
homes = ["button.body(HOVER).fill"]
[[row]]
key = "c.stroke_color"
homes = ["button.body(NONE).stroke.color"]
[[row]]
key = "c.stroke_width"
homes = ["button.body(NONE).stroke.width"]
[[row]]
key = "c.radius"
homes = ["button.body(NONE).corners"]
[[row]]
key = "c.padding_top"
homes = ["button.body(NONE).padding.top"]
[[row]]
key = "c.padding_left"
homes = ["button.body(NONE).padding.left"]
[[row]]
key = "c.shadow_off"
homes = ["button.body(NONE).shadows"]
[[row]]
key = "c.shadow_on"
homes = ["tooltip.panel.shadows"]
[[row]]
key = "c.exact"
homes = ["checkbox.indicator(NONE).min_size[0]", "checkbox.indicator(NONE).max_size[0]"]
[[row]]
key = "c.floor"
homes = ["button.body(NONE).min_size[0]"]
[[row]]
key = "c.unset_metric"
homes = ["button.disabled_opacity"]
[[row]]
key = "c.unset_part"
homes = ["tooltip.panel.fill"]
[[row]]
key = "c.alternate"
homes = ["list.row(NONE; alternate=true).fill"]
[[row]]
key = "c.motion"
homes = ["snapshot.context.motion"]
[[row]]
key = "c.contrast"
homes = ["snapshot.context.contrast"]
"#;

const COLOR: u32 = 0x003d_aee9;
const HOVER_FILL: u32 = 0x00d6_ecf8;
const STROKE: u32 = 0x00bc_bec0;
const ALTERNATE: u32 = 0x00f7_f7f7;
const FACE: u32 = 0x00fc_fcfc;

/// The old values of [`ROWS`], each equal to [`theme`]'s value at its home.
fn old() -> Value {
    json!({
        "c": {
            "color": "#3daee9",
            "number": NUMBER,
            "int": 700,
            "bool": true,
            "string": "Noto Sans",
            "style": "italic",
            "scheme": "Light",
            "icon_set": "freedesktop",
            "trigger": true,
            "defined_size": { "Pt": 10.0 },
            "fill": "#d6ecf8",
            "stroke_color": "#bcbec0",
            "stroke_width": 1.0,
            "radius": 5.0,
            "padding_top": 6.0,
            "padding_left": null,
            "shadow_off": false,
            "shadow_on": true,
            "exact": 16.0,
            "floor": 80.0,
            "unset_metric": null,
            "unset_part": null,
            "alternate": "#f7f7f7",
            "motion": true,
            "contrast": true,
        }
    })
}

/// A button label of these properties: the family and style stated, the defined size 10 pt.
fn label(stated: TextStated, style: FontStyle) -> TextStyle {
    TextStyle {
        family: Arc::from("Noto Sans"),
        style,
        defined_size: Some(FontSize::Pt(10.0)),
        stated,
        ..TextStyle::default()
    }
}

fn button(label: TextStyle) -> ButtonWidget {
    let body = RuleBuilder::new(PartStyle::default())
        .base([
            PartProp::Fill(Paint::Solid(srgb(FACE))),
            PartProp::Stroke(Stroke::uniform(1.0, srgb(STROKE))),
            PartProp::Corners(Corners::uniform(5.0)),
            PartProp::PaddingSides(Edges::all(6.0), EdgeFlags::TOP),
            PartProp::Shadows(None),
            PartProp::MinSize([Some(80.0), None]),
        ])
        .rule(
            RuleSelector::on(WidgetState::HOVER),
            [PartProp::Fill(Paint::Solid(srgb(HOVER_FILL)))],
        )
        .build();
    ButtonWidget::new(
        WidgetOrigin::Native,
        body,
        RuleBuilder::new(label).build(),
        RuleBuilder::new(IconStyle::default()).build(),
        4.0,
        None,
    )
}

/// A theme holding the values of [`old`] at the homes of [`ROWS`], and nothing else stated.
fn theme() -> ResolvedTheme {
    let mut t = nothing_stated(Preset::KdeBreeze, ThemeContext::light());
    t.colors.link = srgb(COLOR);
    t.layout.border_width = NUMBER;
    t.timings.tooltip_show_ms = Some(700);
    t.focus_ring.trigger = Some(FocusTrigger::KeyboardOnly);
    t.typography.body = TextStyle {
        size_px: 10.0,
        stated: TextStated::SIZE_PX,
        ..TextStyle::default()
    };
    t.button = button(label(
        TextStated::FAMILY | TextStated::STYLE,
        FontStyle::Italic,
    ));
    let square = |side: f32| {
        RuleBuilder::new(PartStyle::default())
            .base([
                PartProp::MinSize([Some(side), Some(side)]),
                PartProp::MaxSize([Some(side), Some(side)]),
            ])
            .build()
    };
    t.checkbox = CheckboxWidget::new(
        WidgetOrigin::Native,
        square(16.0),
        RuleBuilder::new(IconStyle::default()).build(),
        RuleBuilder::new(TextStyle::default()).build(),
        6.0,
        None,
    );
    t.radio = RadioWidget::new(
        WidgetOrigin::Native,
        square(16.0),
        RuleBuilder::new(PartStyle::default()).build(),
        RuleBuilder::new(TextStyle::default()).build(),
        6.0,
        None,
    );
    let row = RuleBuilder::new(PartStyle::default())
        .base([
            PartProp::PaddingSides(Edges::all(2.0), EdgeFlags::TOP),
            PartProp::MarginSides(Edges::all(3.0), EdgeFlags::TOP),
        ])
        .rule(
            RuleSelector::on(WidgetState::NONE).matcher(ListMatcher {
                variant: None,
                alternate: Some(true),
            }),
            [PartProp::Fill(Paint::Solid(srgb(ALTERNATE)))],
        )
        .build();
    let empty_part = || RuleBuilder::new(PartStyle::default()).build();
    let empty_text = || RuleBuilder::new(TextStyle::default()).build();
    t.list = ListWidget::new(
        WidgetOrigin::Native,
        empty_part(),
        row,
        empty_text(),
        empty_part(),
        empty_text(),
        empty_part(),
        Color::default(),
    );
    t.scrollbar.overlay_mode = true;
    t
}

/// The reader's context of the snapshot rows: reduced motion, high contrast.
fn snapshot() -> ThemeContext {
    ThemeContext::light()
        .with_motion(MotionPreference::Reduced)
        .with_contrast(Contrast::High)
}

fn rows(toml: &str) -> Result<Vec<Row>, toml::de::Error> {
    Ok(toml::from_str::<Table>(toml)?.row)
}

/// The report of [`ROWS`] on `kde-breeze`.
fn report_of(old: &Value, theme: &ResolvedTheme) -> Result<Report, Box<dyn std::error::Error>> {
    Ok(compare_rows(
        &rows(ROWS)?,
        old,
        theme,
        Some(&snapshot()),
        Preset::KdeBreeze,
    )?)
}

/// `old()` with `key` (`c.<name>`) set to `value`.
fn old_with(key: &str, value: Value) -> Result<Value, String> {
    let mut old = old();
    let slot = key
        .split('.')
        .try_fold(&mut old, |v, segment| v.get_mut(segment))
        .ok_or_else(|| format!("no key `{key}`"))?;
    *slot = value;
    Ok(old)
}

#[test]
fn every_kind_equal_gives_no_mismatch() -> TestResult {
    let report = report_of(&old(), &theme())?;
    let n = rows(ROWS)?.len();
    assert_eq!(
        report.totals,
        Totals {
            rows: n,
            compared: n,
            equal: n,
            mismatches: 0,
            skipped: BTreeMap::new(),
        },
        "{report}"
    );
    Ok(())
}

#[test]
fn each_kind_off_by_the_smallest_step_mismatches_that_row_alone() -> TestResult {
    let ulp = |v: f32| json!(next_up(v));
    let cases = [
        ("c.color", json!("#3daeea")),     // one 8-bit step of blue
        ("c.color", json!("#3daee9fe")),   // one 8-bit step of alpha
        ("c.number", ulp(NUMBER)),         // one f32 ULP
        ("c.int", json!(701)),             // the next integer
        ("c.bool", json!(false)),          // the other bool
        ("c.string", json!("Noto Sanz")),  // another string
        ("c.style", json!("oblique")),     // another FontStyle
        ("c.scheme", json!("Dark")),       // another scheme
        ("c.icon_set", json!("material")), // another icon set
        ("c.trigger", json!(false)),       // another trigger
        ("c.trigger", Value::Null),        // an unstated trigger against a stated one
        ("c.defined_size", json!({ "Pt": next_up(10.0) })), // one ULP of the size
        ("c.defined_size", json!({ "Px": 10.0 })), // another unit
        ("c.fill", json!("#d6ecf9")),      // a part's colour under a state
        ("c.stroke_color", json!("#bcbec1")),
        ("c.stroke_width", ulp(1.0)),
        ("c.radius", ulp(5.0)),
        ("c.padding_top", ulp(6.0)),
        ("c.padding_top", Value::Null), // null against a stated side
        ("c.padding_left", json!(0.0)), // a value against an unstated side
        ("c.shadow_off", json!(true)),  // shadow on against SHADOWS stated as none
        ("c.shadow_on", json!(false)),  // shadow off against SHADOWS unstated
        ("c.exact", ulp(16.0)),         // both homes of an exact size
        ("c.floor", ulp(80.0)),
        ("c.unset_metric", json!(1.0)), // a value against a `None` metric
        ("c.unset_part", json!("#ffffff")), // a value against a clear bit
        ("c.alternate", json!("#f7f7f8")),
        ("c.motion", json!(false)),   // motion is reduced
        ("c.contrast", json!(false)), // contrast is high
    ];
    let theme = theme();
    for (key, value) in cases {
        let shown = value.to_string();
        let report = report_of(&old_with(key, value)?, &theme)?;
        assert_eq!(
            report.mismatched_keys(),
            vec![key],
            "{key} = {shown}: {report}"
        );
    }
    Ok(())
}

#[test]
fn the_theme_off_by_a_stated_bit_or_a_variant_mismatches_that_row_alone() -> TestResult {
    let mut cases: Vec<(&str, ResolvedTheme)> = Vec::new();
    // The family's bit cleared.
    let mut t = theme();
    t.button = button(label(TextStated::STYLE, FontStyle::Italic));
    cases.push(("c.string", t));
    // Another FontStyle.
    let mut t = theme();
    t.button = button(label(
        TextStated::FAMILY | TextStated::STYLE,
        FontStyle::Oblique,
    ));
    cases.push(("c.style", t));
    // A stated `Option` metric against an old null.
    let mut t = theme();
    t.button.disabled_opacity = Some(1.0);
    cases.push(("c.unset_metric", t));
    // Another scheme, another trigger, an absent trigger.
    let mut t = theme();
    t.context = ThemeContext::dark();
    cases.push(("c.scheme", t));
    let mut t = theme();
    t.focus_ring.trigger = Some(FocusTrigger::Always);
    cases.push(("c.trigger", t));
    let mut t = theme();
    t.focus_ring.trigger = None;
    cases.push(("c.trigger", t));
    for (key, t) in &cases {
        let report = report_of(&old(), t)?;
        assert_eq!(report.mismatched_keys(), vec![*key], "{key}: {report}");
    }
    Ok(())
}

#[test]
fn a_mismatch_shows_both_values_in_full_precision() -> TestResult {
    let report = report_of(&old_with("c.number", json!(next_up(NUMBER)))?, &theme())?;
    let Some(Outcome::Mismatch(homes)) = report.rows.get(1).map(|r| &r.outcome) else {
        return Err(format!("the number's row does not mismatch: {report}").into());
    };
    let expected = vec![Mismatch {
        key: "c.number".to_owned(),
        home: "layout.border_width".to_owned(),
        old: format!(
            "{} (f32 0.30000004 [{:#010x}])",
            json!(next_up(NUMBER)),
            next_up(NUMBER).to_bits()
        ),
        new: format!("0.3 [{:#010x}]", NUMBER.to_bits()),
    }];
    assert_eq!(homes, &expected);
    Ok(())
}

const DIFFERENCE: &str = r#"
[[row]]
key = "d.min_height"
homes = ["button.body(NONE).min_size[1]"]
difference_presets = ["kde-breeze"]
difference_reason = "a reason"
difference_citation = "a citation"
difference_values = { "kde-breeze" = "unstated" }
[[row]]
key = "d.min_width"
homes = ["button.body(NONE).min_size[0]"]
difference_presets = ["kde-breeze"]
difference_reason = "a reason"
difference_citation = "a citation"
difference_values = { "kde-breeze" = 80.0 }
"#;

#[test]
fn a_listed_difference_compares_the_value_stated_for_its_preset() -> TestResult {
    let old = json!({ "d": { "min_height": 32.0, "min_width": 82.0 } });
    let rows = rows(DIFFERENCE)?;
    let theme = theme();
    let breeze = compare_rows(&rows, &old, &theme, None, Preset::KdeBreeze)?;
    assert!(breeze.mismatched_keys().is_empty(), "{breeze}");
    let values: Vec<_> = breeze.rows.iter().map(|r| r.value.clone()).collect();
    assert_eq!(values, vec![Some(Value::Null), Some(json!(80.0))]);
    // On another preset the old values are compared.
    let other = compare_rows(&rows, &old, &theme, None, Preset::Adwaita)?;
    assert_eq!(
        other.mismatched_keys(),
        vec!["d.min_height", "d.min_width"],
        "{other}"
    );
    Ok(())
}

const FALLBACKS: &str = r#"
[[row]]
key = "f.radio_width"
homes = ["radio.indicator(NONE).min_size[0]"]
if_null = "f.width"
[[row]]
key = "f.background"
homes = ["checkbox.indicator(NONE).fill"]
only_if_null = "f.unchecked"
[[row]]
key = "f.caption"
homes = ["typography.caption.size_px"]
home_fallback = "typography.body.size_px"
[[row]]
key = "f.padding"
homes = ["list.row(NONE).padding.top"]
homes_by_preset = { "kde-breeze" = ["list.row(NONE).margin.top"] }
"#;

fn fallback_report(old: &Value, theme: &ResolvedTheme, preset: Preset) -> Result<Report, String> {
    let rows = rows(FALLBACKS).map_err(|e| e.to_string())?;
    compare_rows(&rows, old, theme, None, preset)
}

#[test]
fn if_null_reads_the_other_key_where_the_key_is_null() -> TestResult {
    let theme = theme();
    let unset = json!({ "f": { "radio_width": null, "width": 16.0, "background": null,
        "unchecked": null, "caption": 10.0, "padding": 3.0 } });
    let report = fallback_report(&unset, &theme, Preset::KdeBreeze)?;
    assert!(report.mismatched_keys().is_empty(), "{report}");
    assert_eq!(
        report.rows.first().and_then(|r| r.value.clone()),
        Some(json!(16.0))
    );
    // A stated key is compared itself.
    let stated = old_with_fallbacks("f.radio_width", json!(18.0))?;
    let report = fallback_report(&stated, &theme, Preset::KdeBreeze)?;
    assert_eq!(report.mismatched_keys(), vec!["f.radio_width"], "{report}");
    Ok(())
}

/// The old values of [`FALLBACKS`] that equal [`theme`], with `key` set to `value`.
fn old_with_fallbacks(key: &str, value: Value) -> Result<Value, String> {
    let mut old = json!({ "f": { "radio_width": 16.0, "width": 12.0, "background": null,
        "unchecked": null, "caption": 10.0, "padding": 3.0 } });
    let slot = key
        .split('.')
        .try_fold(&mut old, |v, segment| v.get_mut(segment))
        .ok_or_else(|| format!("no key `{key}`"))?;
    *slot = value;
    Ok(old)
}

#[test]
fn only_if_null_skips_the_row_where_the_other_key_is_stated() -> TestResult {
    let theme = theme();
    let stated = old_with_fallbacks("f.unchecked", json!("#ffffff"))?;
    let report = fallback_report(&stated, &theme, Preset::KdeBreeze)?;
    let outcome = report.rows.get(1).map(|r| r.outcome.clone());
    assert_eq!(outcome, Some(Outcome::Skipped(Skip::OtherKeyStated)));
    assert_eq!(report.totals.skipped.get(&Skip::OtherKeyStated), Some(&1));
    // Where the other key is null the row is compared: a stated background against an unstated fill.
    let compared = old_with_fallbacks("f.background", json!("#ffffff"))?;
    let report = fallback_report(&compared, &theme, Preset::KdeBreeze)?;
    assert_eq!(report.mismatched_keys(), vec!["f.background"], "{report}");
    Ok(())
}

#[test]
fn home_fallback_is_read_where_the_slot_is_absent() -> TestResult {
    // No caption slot: the body's size is compared.
    let mut theme = theme();
    let report = fallback_report(
        &old_with_fallbacks("f.caption", json!(10.0))?,
        &theme,
        Preset::KdeBreeze,
    )?;
    assert!(report.mismatched_keys().is_empty(), "{report}");
    let report = fallback_report(
        &old_with_fallbacks("f.caption", json!(next_up(10.0)))?,
        &theme,
        Preset::KdeBreeze,
    )?;
    let homes = match report.rows.get(2).map(|r| &r.outcome) {
        Some(Outcome::Mismatch(homes)) => homes.clone(),
        other => return Err(format!("the caption's row: {other:?}").into()),
    };
    let shown: Vec<&str> = homes.iter().map(|m| m.home.as_str()).collect();
    assert_eq!(
        shown,
        vec!["typography.caption.size_px -> typography.body.size_px"]
    );
    // A caption slot: its own size is compared, not the body's.
    theme.typography.caption = Some(TextStyle {
        size_px: 8.0,
        stated: TextStated::SIZE_PX,
        ..TextStyle::default()
    });
    let report = fallback_report(
        &old_with_fallbacks("f.caption", json!(8.0))?,
        &theme,
        Preset::KdeBreeze,
    )?;
    assert!(report.mismatched_keys().is_empty(), "{report}");
    let report = fallback_report(
        &old_with_fallbacks("f.caption", json!(10.0))?,
        &theme,
        Preset::KdeBreeze,
    )?;
    assert_eq!(report.mismatched_keys(), vec!["f.caption"], "{report}");
    Ok(())
}

#[test]
fn homes_by_preset_replace_the_homes_on_their_preset() -> TestResult {
    let theme = theme();
    // kde-breeze reads the margin (3), another preset the padding (2).
    let margin = old_with_fallbacks("f.padding", json!(3.0))?;
    let padding = old_with_fallbacks("f.padding", json!(2.0))?;
    let report = fallback_report(&margin, &theme, Preset::KdeBreeze)?;
    assert!(report.mismatched_keys().is_empty(), "{report}");
    let report = fallback_report(&padding, &theme, Preset::KdeBreeze)?;
    assert_eq!(report.mismatched_keys(), vec!["f.padding"], "{report}");
    let report = fallback_report(&padding, &theme, Preset::Adwaita)?;
    assert!(report.mismatched_keys().is_empty(), "{report}");
    Ok(())
}

#[test]
fn rows_without_a_snapshot_or_a_hand_value_are_skipped_with_the_reason() -> TestResult {
    let table = r#"
[[row]]
key = "s.motion"
homes = ["snapshot.context.motion"]
[[row]]
key = "Resolved.mode"
homes = ["context.scheme"]
hand = true
[[row]]
key = "s.color"
homes = ["colors.link"]
"#;
    let rows = rows(table)?;
    let old = json!({ "s": { "motion": true, "color": "#3daee9" } });
    let report = compare_rows(&rows, &old, &theme(), None, Preset::KdeBreeze)?;
    let outcomes: Vec<_> = report.rows.iter().map(|r| r.outcome.clone()).collect();
    assert_eq!(
        outcomes,
        vec![
            Outcome::Skipped(Skip::NoSnapshot),
            Outcome::Skipped(Skip::HandKeyAbsent),
            Outcome::Equal,
        ]
    );
    assert_eq!(report.totals.compared, 1);
    assert_eq!(report.skipped(), 2);
    // A hand row whose key the caller added is compared.
    let old =
        json!({ "s": { "motion": true, "color": "#3daee9" }, "Resolved": { "mode": "Light" } });
    let report = compare_rows(&rows, &old, &theme(), Some(&snapshot()), Preset::KdeBreeze)?;
    assert_eq!(report.totals.equal, 3, "{report}");
    Ok(())
}

#[test]
fn an_unreadable_home_or_old_value_is_an_error_naming_the_row() -> TestResult {
    let theme = theme();
    let old = json!({ "e": { "v": "#3daee9", "n": 1.0 } });
    for (home, key) in [
        ("button.body(NONE).transform", "e.v"),
        ("button.body(NONE).focus", "e.v"),
        ("input.field(NONE).fill", "e.v"),
        ("colors.error.origin", "e.v"),
        ("typography.rendering.hinting", "e.v"),
        ("button.body(HOVERED).fill", "e.v"),
        ("button.body(NONE; alternate=true).fill", "e.v"),
        ("checkbox.indicator(NONE; variant=Primary).fill", "e.v"),
        ("tooltip.panel(NONE).fill", "e.v"),
        ("nowhere", "e.v"),
        // An old value not of the home's kind.
        ("colors.link", "e.n"),
        ("layout.border_width", "e.v"),
        ("button.body(NONE).shadows", "e.n"),
    ] {
        let table = format!("[[row]]\nkey = \"{key}\"\nhomes = [\"{home}\"]\n");
        let result = compare_rows(&rows(&table)?, &old, &theme, None, Preset::KdeBreeze);
        match result {
            Err(e) => assert!(e.starts_with(&format!("row `{key}`")), "{home}: {e}"),
            Ok(report) => return Err(format!("{home}: no error, {report}").into()),
        }
    }
    // A key that is not in old, on a row not written by hand.
    let table = "[[row]]\nkey = \"e.gone\"\nhomes = [\"colors.link\"]\n";
    assert!(compare_rows(&rows(table)?, &old, &theme, None, Preset::KdeBreeze).is_err());
    Ok(())
}

/// The rows of 0.7.2's `kde-breeze` (light) whose value is stated and equals the theme with nothing stated:
/// a part's `border.shadow_enabled = true`, which is `SHADOWS` unstated (spec §5, notes to the table), and
/// `scrollbar.overlay_mode = false`, a `bool` without `Option` holding its type's default.
const EQUAL_THOUGH_STATED: &[&str] = &[
    "button.border.shadow_enabled",
    "list.border.shadow_enabled",
    "scrollbar.overlay_mode",
    "tooltip.border.shadow_enabled",
];

#[test]
fn kde_breeze_against_a_theme_with_nothing_stated() -> TestResult {
    let resolved = crate::theme::Theme::preset("kde-breeze")?
        .resolve_with(ColorMode::Light, &ResolutionContext::for_tests())?;
    let old = serde_json::to_value(&resolved.variant)?;
    let new = nothing_stated(Preset::KdeBreeze, ThemeContext::light());
    let report = compare(&old, &new, None, Preset::KdeBreeze)?;
    let table: Table = toml::from_str(HOMES)?;
    assert_eq!(report.totals.rows, table.row.len());
    assert_eq!(
        report.totals.compared.saturating_add(report.skipped()),
        table.row.len()
    );
    let mut equal_though_stated = Vec::new();
    for row in &report.rows {
        let stated = row.value.as_ref().is_some_and(|v| !v.is_null());
        match (&row.outcome, stated) {
            (Outcome::Mismatch(_), true) | (Outcome::Equal, false) => {}
            (Outcome::Equal, true) => equal_though_stated.push(row.key.as_str()),
            (Outcome::Mismatch(m), false) => {
                return Err(format!("`{}`: null, but mismatches: {m:?}", row.key).into());
            }
            (Outcome::Skipped(skip), _) => {
                let expected = if row.key.starts_with("accessibility.") {
                    Skip::NoSnapshot
                } else if row.key.starts_with("Resolved.") || row.key.starts_with("layout.") {
                    Skip::HandKeyAbsent
                } else {
                    Skip::OtherKeyStated
                };
                assert_eq!(*skip, expected, "{}", row.key);
            }
        }
    }
    assert_eq!(equal_though_stated, EQUAL_THOUGH_STATED, "{report}");
    assert!(report.totals.mismatches > 0, "{report}");
    Ok(())
}
