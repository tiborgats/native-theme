use super::*;
use serde_json::json;
use std::sync::Arc;

use crate::next::geometry::{Corners, Edges, ShadowLayer, StrokeSide};
use crate::next::paint::{BlendMode, GradientStop, PaintLayer};
use crate::next::preset::nothing_stated;
use crate::next::rules::{
    ButtonMatcher, IconProp, ListMatcher, PartProp, RuleBuilder, RuleSelector,
};
use crate::next::widgets::{
    ButtonWidget, CheckboxWidget, ListWidget, RadioWidget, ScrollbarWidget, WidgetOrigin,
};
use crate::next::widgets::{InputWidget, LinkWidget};
use crate::next::window::{CaptionLayout, EdgeFlags, WindowChrome};
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
        empty_part(),
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

const NULL_AT_A_STATE: &str = r#"
[[row]]
key = "n.hover_stroke"
homes = ["button.body(HOVER).stroke.color"]
[[row]]
key = "n.hover_fill"
homes = ["button.body(HOVER).fill"]
[[row]]
key = "n.rest_fill"
homes = ["button.body(NONE).fill"]
[[row]]
key = "n.checked_hover"
homes = ["checkbox.indicator(HOVER).stroke.color", "checkbox.indicator(HOVER|CHECKED).stroke.color"]
null_reference = "CHECKED"
[[row]]
key = "n.checked_hover_against_rest"
homes = ["checkbox.indicator(HOVER|CHECKED).stroke.color"]
[[row]]
key = "n.hover_opacity"
homes = ["checkbox.indicator(HOVER).opacity"]
"#;

#[test]
fn an_old_null_under_a_state_is_the_property_at_the_reference_state() -> TestResult {
    let old = json!({ "n": {
        "hover_stroke": null,
        "hover_fill": null,
        "rest_fill": null,
        "checked_hover": null,
        "checked_hover_against_rest": null,
        "hover_opacity": null,
    } });
    let mut t = theme();
    let indicator = RuleBuilder::new(PartStyle::default())
        .base([PartProp::Stroke(Stroke::uniform(1.0, srgb(STROKE)))])
        .rule(
            RuleSelector::on(WidgetState::CHECKED),
            [PartProp::Stroke(Stroke::uniform(1.0, srgb(COLOR)))],
        )
        // The default opacity, stated: the value is the rest state's, its bit is not.
        .rule(
            RuleSelector::on(WidgetState::HOVER),
            [PartProp::Opacity(1.0)],
        )
        .build();
    t.checkbox = CheckboxWidget::new(
        WidgetOrigin::Native,
        indicator,
        RuleBuilder::new(IconStyle::default()).build(),
        RuleBuilder::new(TextStyle::default()).build(),
        6.0,
        None,
    );
    let report = compare_rows(&rows(NULL_AT_A_STATE)?, &old, &t, None, Preset::KdeBreeze)?;
    // No rule for the state: equal to the rest state (the button's stroke) or to the named reference
    // (`CHECKED` for the `HOVER|CHECKED` home, `NONE` for the `HOVER` home of the same row). A rule that
    // changes the property's value or its `stated` bit at the state, or a home at the rest state that
    // states the property, mismatches.
    assert_eq!(
        report.mismatched_keys(),
        vec![
            "n.hover_fill",
            "n.rest_fill",
            "n.checked_hover_against_rest",
            "n.hover_opacity"
        ],
        "{report}"
    );
    let Some(Outcome::Mismatch(homes)) = report.rows.get(4).map(|r| &r.outcome) else {
        return Err(format!("the fifth row does not mismatch: {report}").into());
    };
    assert_eq!(
        homes.iter().map(|m| m.home.as_str()).collect::<Vec<_>>(),
        vec![
            "checkbox.indicator(HOVER|CHECKED).stroke.color against \
             checkbox.indicator(NONE).stroke.color"
        ]
    );
    Ok(())
}

const STATED_FOR_A_STATE: &str = r#"
[[row]]
key = "s.hover_fill"
homes = ["button.body(HOVER).fill"]
[[row]]
key = "s.pressed_fill"
homes = ["button.body(PRESSED).fill"]
[[row]]
key = "s.checked_hover"
homes = ["checkbox.indicator(HOVER|CHECKED).stroke.color"]
null_reference = "CHECKED"
"#;

/// [`theme`] with a button body that restates its rest fill for `HOVER`, and a check box indicator with a
/// stroke for `CHECKED`.
fn stated_for_a_state() -> ResolvedTheme {
    let mut t = theme();
    t.button = ButtonWidget::new(
        WidgetOrigin::Native,
        RuleBuilder::new(PartStyle::default())
            .base([PartProp::Fill(Paint::Solid(srgb(FACE)))])
            .rule(
                RuleSelector::on(WidgetState::HOVER),
                [PartProp::Fill(Paint::Solid(srgb(FACE)))],
            )
            .build(),
        RuleBuilder::new(TextStyle::default()).build(),
        RuleBuilder::new(IconStyle::default()).build(),
        4.0,
        None,
    );
    t.checkbox = CheckboxWidget::new(
        WidgetOrigin::Native,
        RuleBuilder::new(PartStyle::default())
            .base([PartProp::Stroke(Stroke::uniform(1.0, srgb(STROKE)))])
            .rule(
                RuleSelector::on(WidgetState::CHECKED),
                [PartProp::Stroke(Stroke::uniform(1.0, srgb(COLOR)))],
            )
            .build(),
        RuleBuilder::new(IconStyle::default()).build(),
        RuleBuilder::new(TextStyle::default()).build(),
        6.0,
        None,
    );
    t
}

#[test]
fn an_old_null_under_a_state_needs_its_from_state_bit_as_at_the_reference() -> TestResult {
    let old = json!({ "s": { "hover_fill": null, "pressed_fill": null, "checked_hover": null } });
    let report = compare_rows(
        &rows(STATED_FOR_A_STATE)?,
        &old,
        &stated_for_a_state(),
        None,
        Preset::KdeBreeze,
    )?;
    // The hover fill equals the rest fill, but a rule states it for `HOVER`: 0.7.2's null says there is
    // none. No rule for `PRESSED`, and the `CHECKED` rule sets the bit at the reference too: equal.
    assert_eq!(report.mismatched_keys(), vec!["s.hover_fill"], "{report}");
    let Some(Outcome::Mismatch(homes)) = report.rows.first().map(|r| &r.outcome) else {
        return Err(format!("the first row does not mismatch: {report}").into());
    };
    assert_eq!(
        homes
            .iter()
            .map(|m| (m.home.as_str(), m.new.ends_with("`from_state` set")))
            .collect::<Vec<_>>(),
        vec![(
            "button.body(HOVER).fill against button.body(NONE).fill",
            true
        )]
    );
    Ok(())
}

#[test]
fn an_old_value_under_a_state_is_compared_whatever_its_from_state_bit() -> TestResult {
    // FACE, the value stated for `HOVER`.
    let report = one_row(
        "button.body(HOVER).fill",
        json!("#fcfcfc"),
        &stated_for_a_state(),
    )?;
    assert!(report.mismatched_keys().is_empty(), "{report}");
    let other = one_row(
        "button.body(HOVER).fill",
        json!("#000000"),
        &stated_for_a_state(),
    )?;
    assert_eq!(other.mismatched_keys(), vec!["x.v"], "{other}");
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
        ("switch.track(NONE).fill", "e.v"),
        ("input.field(NONE; shape=Pill).fill", "e.v"),
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
    "window.border.shadow_enabled",
];

#[test]
fn kde_breeze_against_a_theme_with_nothing_stated() -> TestResult {
    let theme = crate::theme::Theme::preset("kde-breeze")?;
    let resolved = theme.resolve_with(ColorMode::Light, &ResolutionContext::for_tests())?;
    let old = serde_json::to_value(&resolved.variant)?;
    let unresolved = serde_json::to_value(theme.pick_variant(ColorMode::Light)?)?;
    let new = nothing_stated(Preset::KdeBreeze, ThemeContext::light());
    let with_context = |context: RowContext| {
        nothing_stated(
            Preset::KdeBreeze,
            context.applied_to(&ThemeContext::light()),
        )
    };
    let extra = Extra {
        unresolved: Some(&unresolved),
        with_context: Some(&with_context),
    };
    let report = compare(&old, &new, None, Preset::KdeBreeze, &extra)?;
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

/// The report of one row: `home` against the old value `value`.
fn one_row(
    home: &str,
    value: Value,
    theme: &ResolvedTheme,
) -> Result<Report, Box<dyn std::error::Error>> {
    let table = format!("[[row]]\nkey = \"x.v\"\nhomes = [\"{home}\"]\n");
    let old = json!({ "x": { "v": value } });
    Ok(compare_rows(
        &rows(&table)?,
        &old,
        theme,
        None,
        Preset::KdeBreeze,
    )?)
}

/// [`theme`] with a button whose body's base states `props`.
fn with_body(props: Vec<PartProp>) -> ResolvedTheme {
    let mut t = theme();
    t.button = ButtonWidget::new(
        WidgetOrigin::Native,
        RuleBuilder::new(PartStyle::default()).base(props).build(),
        RuleBuilder::new(TextStyle::default()).build(),
        RuleBuilder::new(IconStyle::default()).build(),
        4.0,
        None,
    );
    t
}

#[test]
fn a_chosen_home_list_that_is_empty_is_an_error() -> TestResult {
    let table = "[[row]]\nkey = \"x.v\"\nhomes = [\"colors.link\"]\n\
                 homes_by_preset = { \"kde-breeze\" = [] }\n";
    let old = json!({ "x": { "v": "#3daee9" } });
    let theme = theme();
    match compare_rows(&rows(table)?, &old, &theme, None, Preset::KdeBreeze) {
        Err(e) => assert!(e.starts_with("row `x.v`"), "{e}"),
        Ok(report) => return Err(format!("no error: {report}").into()),
    }
    // On another preset the row's own homes are read.
    let report = compare_rows(&rows(table)?, &old, &theme, None, Preset::Adwaita)?;
    assert!(report.mismatched_keys().is_empty(), "{report}");
    Ok(())
}

#[test]
fn a_colour_out_of_range_or_not_finite_mismatches() -> TestResult {
    let mut t = theme();
    for (color, equal) in [
        (Color::new(1.0, 0.0, 0.0, 1.0), true),
        (Color::new(1.3, 0.0, 0.0, 1.0), false),
        (Color::new(1.0, f32::NAN, 0.0, 1.0), false),
        (Color::new(1.0, -0.1, 0.0, 1.0), false),
        (Color::new(1.0, 0.0, 0.0, 1.3), false),
        (Color::new(1.0, 0.0, f32::INFINITY, 1.0), false),
    ] {
        t.colors.link = color;
        let report = one_row("colors.link", json!("#ff0000"), &t)?;
        assert_eq!(
            report.mismatched_keys().is_empty(),
            equal,
            "{color:?}: {report}"
        );
    }
    Ok(())
}

#[test]
fn a_part_property_of_a_form_0_7_2_does_not_hold_mismatches() -> TestResult {
    let c = srgb(STROKE);
    let side = || StrokeSide {
        width: 1.0,
        paint: Paint::Solid(c),
    };
    let sides = || Stroke::sides(side(), side(), side(), side(), StrokeAlign::Inside);
    let gradient = Paint::layers([PaintLayer::LinearGradient {
        angle_deg: 0.0,
        stops: Arc::from([
            GradientStop {
                offset: 0.0,
                color: c,
            },
            GradientStop {
                offset: 1.0,
                color: c,
            },
        ]),
        blend: BlendMode::Normal,
    }]);
    let layered = Paint::layers([PaintLayer::Solid {
        color: c,
        blend: BlendMode::Normal,
    }]);
    let per_corner = Corners {
        radius: CornerRadius::PerCorner {
            top_left: 5.0,
            top_right: 5.0,
            bottom_right: 5.0,
            bottom_left: 5.0,
        },
        curve: CornerCurve::Circular,
    };
    let shadow = || PartProp::Shadows(Some(Arc::from([ShadowLayer::default()])));
    let color = json!("#bcbec0");
    let cases: Vec<(PartProp, &str, Value, bool)> = vec![
        // The forms 0.7.2 holds.
        (PartProp::Fill(Paint::Solid(c)), "fill", color.clone(), true),
        (
            PartProp::Corners(Corners::uniform(5.0)),
            "corners",
            json!(5.0),
            true,
        ),
        (
            PartProp::Corners(Corners::default()),
            "corners",
            json!(0.0),
            true,
        ),
        (
            PartProp::Stroke(Stroke::uniform(1.0, c)),
            "stroke.width",
            json!(1.0),
            true,
        ),
        (
            PartProp::Stroke(Stroke::uniform(1.0, c)),
            "stroke.color",
            color.clone(),
            true,
        ),
        (
            PartProp::Stroke(Stroke::None),
            "stroke.width",
            json!(0.0),
            true,
        ),
        (
            PartProp::Stroke(Stroke::uniform(0.0, c)),
            "stroke.width",
            json!(0.0),
            true,
        ),
        (
            PartProp::Stroke(Stroke::uniform(0.0, c)),
            "stroke.color",
            color.clone(),
            true,
        ),
        // The forms it does not.
        (PartProp::Fill(gradient), "fill", color.clone(), false),
        (PartProp::Fill(layered), "fill", color.clone(), false),
        (PartProp::Fill(Paint::None), "fill", color.clone(), false),
        (
            PartProp::Corners(Corners::continuous(5.0)),
            "corners",
            json!(5.0),
            false,
        ),
        (
            PartProp::Corners(Corners::pill()),
            "corners",
            json!(5.0),
            false,
        ),
        (PartProp::Corners(per_corner), "corners", json!(5.0), false),
        (
            PartProp::Stroke(Stroke::None),
            "stroke.color",
            color.clone(),
            false,
        ),
        (
            PartProp::Stroke(Stroke::uniform_aligned(1.0, c, StrokeAlign::Center)),
            "stroke.width",
            json!(1.0),
            false,
        ),
        (
            PartProp::Stroke(Stroke::uniform_aligned(1.0, c, StrokeAlign::Outside)),
            "stroke.color",
            color.clone(),
            false,
        ),
        (PartProp::Stroke(sides()), "stroke.width", json!(1.0), false),
        (
            PartProp::Stroke(sides()),
            "stroke.color",
            color.clone(),
            false,
        ),
        (shadow(), "shadows", json!(true), false),
        (shadow(), "shadows", json!(false), false),
    ];
    for (prop, property, value, equal) in cases {
        let shown = format!("{prop:?} at {property} against {value}");
        let report = one_row(
            &format!("button.body(NONE).{property}"),
            value,
            &with_body(vec![prop]),
        )?;
        assert_eq!(
            report.mismatched_keys().is_empty(),
            equal,
            "{shown}: {report}"
        );
    }
    Ok(())
}

#[test]
fn an_icon_part_reads_its_colour_and_stroke_width() -> TestResult {
    let checkbox = |checkmark| {
        CheckboxWidget::new(
            WidgetOrigin::Native,
            RuleBuilder::new(PartStyle::default()).build(),
            checkmark,
            RuleBuilder::new(TextStyle::default()).build(),
            4.0,
            None,
        )
    };
    let mut t = theme();
    t.checkbox = checkbox(
        RuleBuilder::new(IconStyle::default())
            .base([IconProp::StrokeWidth(Some(2.0))])
            .rule(
                RuleSelector::on(WidgetState::CHECKED),
                [IconProp::Color(srgb(STROKE))],
            )
            .build(),
    );
    for (home, value, equal) in [
        ("checkbox.checkmark(CHECKED).color", json!("#bcbec0"), true),
        ("checkbox.checkmark(NONE).color", json!("#bcbec0"), false),
        ("checkbox.checkmark(NONE).stroke_width", json!(2.0), true),
        (
            "checkbox.checkmark(NONE).stroke_width",
            json!(next_up(2.0)),
            false,
        ),
    ] {
        let report = one_row(home, value, &t)?;
        assert_eq!(
            report.mismatched_keys().is_empty(),
            equal,
            "{home}: {report}"
        );
    }
    // A stroke width stated as `None` is no 0.7.2 value.
    t.checkbox = checkbox(
        RuleBuilder::new(IconStyle::default())
            .base([IconProp::StrokeWidth(None)])
            .build(),
    );
    let report = one_row("checkbox.checkmark(NONE).stroke_width", json!(2.0), &t)?;
    assert_eq!(report.mismatched_keys(), vec!["x.v"], "{report}");
    Ok(())
}

#[test]
fn the_variant_and_orientation_axes_reach_the_query() -> TestResult {
    let mut t = theme();
    let primary = RuleSelector::on(WidgetState::NONE).matcher(ButtonMatcher {
        variant: Some(ButtonVariant::Primary),
        shape: None,
    });
    t.button = ButtonWidget::new(
        WidgetOrigin::Native,
        RuleBuilder::new(PartStyle::default())
            .rule(primary, [PartProp::Fill(Paint::Solid(srgb(COLOR)))])
            .build(),
        RuleBuilder::new(TextStyle::default()).build(),
        RuleBuilder::new(IconStyle::default()).build(),
        4.0,
        None,
    );
    let vertical = RuleSelector::on(WidgetState::NONE).orientation(Orientation::Vertical);
    t.scrollbar = ScrollbarWidget::new(
        WidgetOrigin::Native,
        RuleBuilder::new(PartStyle::default())
            .rule(vertical, [PartProp::MinSize([Some(21.0), None])])
            .build(),
        RuleBuilder::new(PartStyle::default()).build(),
        20.0,
        false,
    );
    for (home, value, equal) in [
        (
            "button.body(NONE; variant=Primary).fill",
            json!("#3daee9"),
            true,
        ),
        ("button.body(NONE).fill", json!("#3daee9"), false),
        (
            "button.body(NONE; variant=Flat).fill",
            json!("#3daee9"),
            false,
        ),
        (
            "scrollbar.track(NONE; orientation=Vertical).min_size[0]",
            json!(21.0),
            true,
        ),
        (
            "scrollbar.track(NONE; orientation=Horizontal).min_size[0]",
            json!(21.0),
            false,
        ),
    ] {
        let report = one_row(home, value, &t)?;
        assert_eq!(
            report.mismatched_keys().is_empty(),
            equal,
            "{home}: {report}"
        );
    }
    Ok(())
}

#[test]
fn every_size_and_side_is_read_from_its_own_field() -> TestResult {
    let mut t = theme();
    t.checkbox = CheckboxWidget::new(
        WidgetOrigin::Native,
        RuleBuilder::new(PartStyle::default())
            .base([
                PartProp::MinSize([Some(10.0), Some(20.0)]),
                PartProp::MaxSize([Some(30.0), Some(40.0)]),
                PartProp::Padding(Edges::new(1.0, 2.0, 3.0, 4.0)),
                PartProp::Margin(Edges::new(5.0, 6.0, 7.0, 8.0)),
            ])
            .build(),
        RuleBuilder::new(IconStyle::default()).build(),
        RuleBuilder::new(TextStyle::default()).build(),
        4.0,
        None,
    );
    let fields: [(&str, f32); 12] = [
        ("min_size[0]", 10.0),
        ("min_size[1]", 20.0),
        ("max_size[0]", 30.0),
        ("max_size[1]", 40.0),
        ("padding.top", 1.0),
        ("padding.right", 2.0),
        ("padding.bottom", 3.0),
        ("padding.left", 4.0),
        ("margin.top", 5.0),
        ("margin.right", 6.0),
        ("margin.bottom", 7.0),
        ("margin.left", 8.0),
    ];
    // Each field holds its own value and no other field's.
    for (field, own) in fields {
        for (_, value) in fields {
            let home = format!("checkbox.indicator(NONE).{field}");
            let report = one_row(&home, json!(value), &t)?;
            assert_eq!(
                report.mismatched_keys().is_empty(),
                value.to_bits() == own.to_bits(),
                "{home} against {value}: {report}"
            );
        }
    }
    Ok(())
}

#[test]
fn a_fallback_key_missing_from_old_is_an_error_and_an_absent_slot_is_unstated() -> TestResult {
    let theme = theme();
    let old = json!({ "x": { "v": null } });
    for field in ["if_null", "only_if_null"] {
        let table =
            format!("[[row]]\nkey = \"x.v\"\nhomes = [\"colors.link\"]\n{field} = \"x.gone\"\n");
        match compare_rows(&rows(&table)?, &old, &theme, None, Preset::KdeBreeze) {
            Err(e) => assert!(e.contains("`x.gone` is not in old"), "{field}: {e}"),
            Ok(report) => return Err(format!("{field}: no error, {report}").into()),
        }
    }
    // An absent `Typography` slot without `home_fallback`: unstated, so a value mismatches and a null is
    // equal.
    assert_eq!(theme.typography.caption, None);
    let report = one_row("typography.caption.size_px", json!(10.0), &theme)?;
    assert_eq!(report.mismatched_keys(), vec!["x.v"], "{report}");
    let report = one_row("typography.caption.size_px", Value::Null, &theme)?;
    assert!(report.mismatched_keys().is_empty(), "{report}");
    Ok(())
}

const IN_A_CONTEXT: &str = r#"
[[row]]
key = "x.inactive"
homes = ["colors.link"]
context = { window_active = false }
[[row]]
key = "x.active"
homes = ["colors.link"]
"#;

#[test]
fn a_row_with_a_context_reads_its_homes_from_the_theme_resolved_with_it() -> TestResult {
    let theme = theme();
    let asked = std::cell::Cell::new(None);
    let with_context = |context: RowContext| {
        asked.set(Some(context));
        let mut t = nothing_stated(
            Preset::KdeBreeze,
            context.applied_to(&ThemeContext::light()),
        );
        t.colors.link = srgb(HOVER_FILL);
        t
    };
    let extra = Extra {
        unresolved: None,
        with_context: Some(&with_context),
    };
    let rows = rows(IN_A_CONTEXT)?;
    let old = json!({ "x": { "inactive": "#d6ecf8", "active": "#3daee9" } });
    let report = compare_rows_with(&rows, &old, &theme, None, Preset::KdeBreeze, &extra)?;
    assert!(report.mismatched_keys().is_empty(), "{report}");
    assert_eq!(
        asked.get(),
        Some(RowContext {
            window_active: false
        })
    );
    let swapped = json!({ "x": { "inactive": "#3daee9", "active": "#3daee9" } });
    let report = compare_rows_with(&rows, &swapped, &theme, None, Preset::KdeBreeze, &extra)?;
    assert_eq!(report.mismatched_keys(), vec!["x.inactive"], "{report}");
    // Without the theme resolved with the context: an error naming the row.
    match compare_rows(&rows, &old, &theme, None, Preset::KdeBreeze) {
        Err(e) => assert!(e.starts_with("row `x.inactive`"), "{e}"),
        Ok(report) => return Err(format!("no error: {report}").into()),
    }
    Ok(())
}

const UNSTATED_IF_UNSET: &str = r#"
[[row]]
key = "u.hover_border"
homes = ["button.body(HOVER).stroke.color"]
unstated_if_unset = true
[[row]]
key = "u.border.corner_radius"
homes = ["button.body(NONE).corners"]
unstated_if_unset = true
"#;

#[test]
fn unstated_if_unset_compares_a_field_the_unresolved_theme_leaves_unset_as_null() -> TestResult {
    let theme = theme();
    let rows = rows(UNSTATED_IF_UNSET)?;
    // 0.7.2's resolver made up the hover border; the theme states a radius of 5.
    let old = json!({ "u": { "hover_border": "#ffffff", "border": { "corner_radius": 5.0 } } });
    let report_on = |unresolved: &Value| {
        let extra = Extra {
            unresolved: Some(unresolved),
            with_context: None,
        };
        compare_rows_with(&rows, &old, &theme, None, Preset::KdeBreeze, &extra)
    };
    // The hover border unset: compared as null, so as the rest state's stroke. The radius stated, by its
    // serde name: compared as any other row.
    let unset = json!({ "u": { "border": { "corner_radius_px": 5.0 } } });
    let report = report_on(&unset)?;
    assert!(report.mismatched_keys().is_empty(), "{report}");
    assert_eq!(
        report.rows.first().and_then(|r| r.value.clone()),
        Some(Value::Null)
    );
    // Stated, the hover border is compared as any other row.
    let stated =
        json!({ "u": { "hover_border": "#ffffff", "border": { "corner_radius_px": 5.0 } } });
    let report = report_on(&stated)?;
    assert_eq!(report.mismatched_keys(), vec!["u.hover_border"], "{report}");
    // A border's leaf is read by its serde name only: by its field's name it is unset, so the stated
    // radius at the rest state mismatches the null.
    let by_field_name = json!({ "u": { "border": { "corner_radius": 5.0 } } });
    let report = report_on(&by_field_name)?;
    assert_eq!(
        report.mismatched_keys(),
        vec!["u.border.corner_radius"],
        "{report}"
    );
    // Without the unresolved theme: an error naming the row.
    match compare_rows(&rows, &old, &theme, None, Preset::KdeBreeze) {
        Err(e) => assert!(e.starts_with("row `u.hover_border`"), "{e}"),
        Ok(report) => return Err(format!("no error: {report}").into()),
    }
    Ok(())
}

const NO_OUTLINE: &str = r#"
[[row]]
key = "m.stroke"
homes = ["button.body(NONE).stroke.color", "button.body(HOVER).stroke.color"]
null_means = "none"
"#;

#[test]
fn null_means_none_requires_a_stated_stroke_of_none() -> TestResult {
    let rows = rows(NO_OUTLINE)?;
    let null = json!({ "m": { "stroke": null } });
    for (theme, equal) in [
        (with_body(vec![PartProp::Stroke(Stroke::None)]), true),
        // Unstated at every state: equal to the reference, but no outline is not stated.
        (with_body(vec![]), false),
        (theme(), false),
    ] {
        let report = compare_rows(&rows, &null, &theme, None, Preset::KdeBreeze)?;
        assert_eq!(report.mismatched_keys().is_empty(), equal, "{report}");
    }
    // A stated key is compared as any other: the stroke's colour at both homes.
    let stated = json!({ "m": { "stroke": "#bcbec0" } });
    let report = compare_rows(&rows, &stated, &theme(), None, Preset::KdeBreeze)?;
    assert!(report.mismatched_keys().is_empty(), "{report}");
    Ok(())
}

const IF_NULL_CHAIN: &str = r#"
[[row]]
key = "i.a"
homes = ["layout.border_width"]
if_null = "i.b"
[[row]]
key = "i.b"
homes = ["layout.border_width"]
if_null = "i.c"
[[row]]
key = "i.c"
homes = ["layout.border_width"]
"#;

#[test]
fn if_null_follows_the_chain_of_null_keys() -> TestResult {
    let theme = theme();
    let rows = rows(IF_NULL_CHAIN)?;
    let old = json!({ "i": { "a": null, "b": null, "c": NUMBER } });
    let report = compare_rows(&rows, &old, &theme, None, Preset::KdeBreeze)?;
    assert!(report.mismatched_keys().is_empty(), "{report}");
    assert_eq!(
        report.rows.first().and_then(|r| r.value.clone()),
        Some(json!(NUMBER))
    );
    // The chain ends in a null key: the row is compared as a null key.
    let all_null = json!({ "i": { "a": null, "b": null, "c": null } });
    let report = compare_rows(&rows, &all_null, &theme, None, Preset::KdeBreeze)?;
    assert_eq!(
        report.rows.first().and_then(|r| r.value.clone()),
        Some(Value::Null)
    );
    // A chain that returns to a key is an error naming the row.
    let looped = IF_NULL_CHAIN.replacen("if_null = \"i.c\"", "if_null = \"i.a\"", 1);
    match compare_rows(
        &self::rows(&looped)?,
        &all_null,
        &theme,
        None,
        Preset::KdeBreeze,
    ) {
        Err(e) => assert!(e.contains("returns to `i.a`"), "{e}"),
        Ok(report) => return Err(format!("no error: {report}").into()),
    }
    Ok(())
}

#[test]
fn a_decoration_reads_as_underline_enabled_and_an_input_home_takes_its_variant() -> TestResult {
    let mut theme = theme();
    let underline = TextDecoration {
        kind: DecorationKind::Underline,
        color: None,
        thickness: None,
    };
    let label = RuleBuilder::new(TextStyle::default())
        .base([crate::next::rules::TextProp::Decoration(underline)])
        .rule(
            RuleSelector::on(WidgetState::HOVER),
            [crate::next::rules::TextProp::Decoration(TextDecoration {
                kind: DecorationKind::None,
                ..underline
            })],
        )
        .rule(
            RuleSelector::on(WidgetState::PRESSED),
            [crate::next::rules::TextProp::Decoration(TextDecoration {
                color: Some(srgb(COLOR)),
                ..underline
            })],
        )
        .build();
    theme.link = LinkWidget::new(
        WidgetOrigin::Native,
        RuleBuilder::new(PartStyle::default()).build(),
        label,
    );
    let search = RuleBuilder::new(PartStyle::default())
        .rule(
            RuleSelector::on(WidgetState::NONE).matcher(Some(InputVariant::Search)),
            [PartProp::MinSize([None, Some(30.0)])],
        )
        .build();
    let empty_text = || RuleBuilder::new(TextStyle::default()).build();
    theme.input = InputWidget::new(
        WidgetOrigin::Native,
        search,
        empty_text(),
        empty_text(),
        empty_text(),
        Some(0.5),
    );
    for (home, value, equal) in [
        ("link.label(NONE).decoration", json!(true), true),
        ("link.label(NONE).decoration", json!(false), false),
        ("link.label(HOVER).decoration", json!(false), true),
        ("link.label(PRESSED).decoration", json!(true), false),
        ("link.label(PRESSED).decoration", json!(false), false),
        (
            "input.field(NONE; variant=Search).min_size[1]",
            json!(30.0),
            true,
        ),
        ("input.field(NONE).min_size[1]", Value::Null, true),
        ("input.disabled_opacity", json!(0.5), true),
        ("text_area.disabled_opacity", Value::Null, true),
    ] {
        let report = one_row(home, value.clone(), &theme)?;
        let outcome = report.rows.first().map(|r| &r.outcome);
        assert_eq!(
            matches!(outcome, Some(Outcome::Equal)),
            equal,
            "{home} {value}: {report}"
        );
    }
    Ok(())
}

#[test]
fn a_window_home_reads_its_getter_at_the_normal_state() -> TestResult {
    let mut theme = theme();
    let stated_fill = |hex: u32| PartStyle {
        fill: Paint::Solid(srgb(hex)),
        stated: PartStated::FILL,
        ..PartStyle::default()
    };
    theme.window = WindowChrome::new(
        WidgetOrigin::Native,
        Some(CaptionLayout::new(Arc::from([]), Arc::from([]))),
        PartStyle {
            corners: Corners::uniform(4.0),
            stated: PartStated::CORNERS,
            ..PartStyle::default()
        },
        PartStyle::default(),
        stated_fill(0xffffff),
        stated_fill(0x000000),
        TextStyle::default(),
        RuleBuilder::new(PartStyle::default()).build(),
        RuleBuilder::new(IconStyle::default()).build(),
    );
    for (home, value, equal) in [
        ("window.title_bar(normal).fill", json!("#ffffff"), true),
        ("window.title_bar(normal).fill", Value::Null, false),
        ("window.frame(normal).corners", json!(4.0), true),
        ("window.frame(normal).fill", Value::Null, true),
        ("window.title(normal).color", Value::Null, true),
        ("window.title(normal).color", json!("#ffffff"), false),
    ] {
        let report = one_row(home, value.clone(), &theme)?;
        let outcome = report.rows.first().map(|r| &r.outcome);
        assert_eq!(
            matches!(outcome, Some(Outcome::Equal)),
            equal,
            "{home} {value}: {report}"
        );
    }
    Ok(())
}
