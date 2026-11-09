//! The platform functions over their vector tables (S2.2 *Checked against
//! their references*): each row's inputs in, the reference's output out, a
//! colour compared unrounded on the 0-255 scale to 1e-3.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::HashMap;

use native_theme::platform::adw_standalone::adw_standalone;
use native_theme::platform::android_non_linear::{android_non_linear, android_non_linear_inverse};
use native_theme::platform::dynamic_type::dynamic_type;
use native_theme::platform::kcu_mix::kcu_mix;
use native_theme::platform::kcu_overlay::kcu_overlay;
use native_theme::platform::kcu_shade::kcu_shade;
use native_theme::platform::kcu_tint::kcu_tint;
use native_theme::platform::kde_effects::{KdeGroupEffects, kde_effects};
use native_theme::platform::kde_selection_inactive::kde_selection_inactive;
use native_theme::platform::kde_shade::{KdeShadeRole, kde_shade};
use native_theme::platform::kirigami_lerp::kirigami_lerp;
use native_theme::platform::kirigami_tint::kirigami_tint;
use native_theme::platform::m3_role::m3_role;
use native_theme::platform::sf_tracking::{TrackingDesign, sf_tracking};
use native_theme::platform::win_ui::win_ui;
use native_theme::{
    Color, ContentSizeCategory, KdeColorEffect, KdeContrastEffect, KdeGroup, KdeIntensityEffect,
    KdeRoleKind, MaterialVariant, Scheme, TextRole,
};

/// A table's rows, each by its column names; the header lines skipped.
fn rows(function: &str) -> Vec<HashMap<String, String>> {
    let path = format!(
        "{}/tests/vectors/{function}.tsv",
        env!("CARGO_MANIFEST_DIR")
    );
    let text = std::fs::read_to_string(&path).expect(&path);
    let mut lines = text.lines().skip_while(|line| line.starts_with('#'));
    let columns: Vec<&str> = lines
        .next()
        .expect("the column header row")
        .split('\t')
        .collect();
    let rows: Vec<HashMap<String, String>> = lines
        .map(|line| {
            let cells: Vec<&str> = line.split('\t').collect();
            assert_eq!(cells.len(), columns.len(), "{line}");
            columns
                .iter()
                .zip(cells)
                .map(|(column, cell)| ((*column).to_owned(), cell.to_owned()))
                .collect()
        })
        .collect();
    assert!(!rows.is_empty(), "{path} has no row");
    rows
}

/// An input colour, a literal of S2.1, or `srgb(r, g, b, a)` with the
/// reference's singles in the tables' number form, which allows an exponent
/// (`-5.19812102e-06`).
fn color(cell: &str) -> Color {
    Color::parse(cell).unwrap_or_else(|| {
        let channels: Vec<f32> = srgb_channels(cell)
            .map(|n| n.parse().expect(cell))
            .collect();
        from_channels(channels.try_into().expect(cell))
    })
}

/// The channels of `srgb(…)`, as written.
fn srgb_channels(cell: &str) -> impl Iterator<Item = &str> {
    cell.strip_prefix("srgb(")
        .and_then(|rest| rest.strip_suffix(')'))
        .expect(cell)
        .split(',')
        .map(str::trim)
}

/// An output colour, `srgb(r, g, b, a)` with the reference's unrounded
/// channels.
fn output(cell: &str) -> [f64; 4] {
    let channels: Vec<f64> = srgb_channels(cell)
        .map(|n| n.parse().expect(cell))
        .collect();
    channels.try_into().expect(cell)
}

/// A colour's channels, as the platform functions take them.
fn channels(c: Color) -> [f32; 4] {
    [c.r, c.g, c.b, c.a]
}

/// The colour of a platform function's channels.
fn from_channels([r, g, b, a]: [f32; 4]) -> Color {
    Color { r, g, b, a }
}

fn number(cell: &str) -> f32 {
    cell.parse().expect(cell)
}

fn boolean(cell: &str) -> bool {
    match cell {
        "true" => true,
        "false" => false,
        other => panic!("no bool `{other}`"),
    }
}

fn role_kind(cell: &str) -> KdeRoleKind {
    match cell {
        "foreground" => KdeRoleKind::Foreground,
        "decoration" => KdeRoleKind::Decoration,
        "background" => KdeRoleKind::Background,
        other => panic!("no role kind `{other}`"),
    }
}

/// `got` is `expected` on the 0-255 scale to 1e-3, every channel.
fn assert_color(got: Color, expected: [f64; 4], row: &HashMap<String, String>) {
    let got = [got.r, got.g, got.b, got.a].map(f64::from);
    for (got, expected) in got.iter().zip(expected) {
        assert!(
            ((got - expected) * 255.0).abs() <= 1e-3,
            "{got:?} for {expected:?}: {row:?}"
        );
    }
}

#[test]
fn vectors_kirigami_lerp() {
    for row in rows("kirigami_lerp") {
        let got = kirigami_lerp(
            channels(color(&row["a"])),
            channels(color(&row["b"])),
            number(&row["balance"]),
        )
        .expect("a colour in 16-bit storage");
        assert_color(from_channels(got), output(&row["result"]), &row);
    }
}

#[test]
fn vectors_kde_shade() {
    for row in rows("kde_shade") {
        let role = match row["role"].as_str() {
            "light" => KdeShadeRole::Light,
            "midlight" => KdeShadeRole::Midlight,
            "mid" => KdeShadeRole::Mid,
            "dark" => KdeShadeRole::Dark,
            "shadow" => KdeShadeRole::Shadow,
            other => panic!("no role `{other}`"),
        };
        let got = kde_shade(channels(color(&row["c"])), role, number(&row["contrast"]));
        assert_color(from_channels(got), output(&row["result"]), &row);
    }
}

#[test]
fn vectors_kcu_mix() {
    for row in rows("kcu_mix") {
        let got = kcu_mix(
            channels(color(&row["a"])),
            channels(color(&row["b"])),
            number(&row["bias"]),
        );
        assert_color(from_channels(got), output(&row["result"]), &row);
    }
}

#[test]
fn vectors_kcu_tint() {
    for row in rows("kcu_tint") {
        let got = kcu_tint(
            channels(color(&row["a"])),
            channels(color(&row["b"])),
            number(&row["amount"]),
        );
        assert_color(from_channels(got), output(&row["result"]), &row);
    }
}

#[test]
fn vectors_kcu_shade() {
    for row in rows("kcu_shade") {
        let got = kcu_shade(
            channels(color(&row["c"])),
            number(&row["luma"]),
            number(&row["chroma"]),
        );
        assert_color(from_channels(got), output(&row["result"]), &row);
    }
}

#[test]
fn vectors_kcu_overlay() {
    for row in rows("kcu_overlay") {
        let got = kcu_overlay(
            channels(color(&row["under"])),
            channels(color(&row["over"])),
        );
        assert_color(from_channels(got), output(&row["result"]), &row);
    }
}

#[test]
fn vectors_kirigami_tint() {
    for row in rows("kirigami_tint") {
        let got = kirigami_tint(
            channels(color(&row["c"])),
            channels(color(&row["tint"])),
            number(&row["alpha"]),
        );
        assert_color(from_channels(got), output(&row["result"]), &row);
    }
}

/// A row's group effects, the operands `kde_effects` reads itself from
/// `token.kde.<group>`.
fn group_effects(row: &HashMap<String, String>) -> KdeGroupEffects {
    let intensity = match row["intensity"].as_str() {
        "none" => KdeIntensityEffect::None,
        "shade" => KdeIntensityEffect::Shade,
        "darken" => KdeIntensityEffect::Darken,
        "lighten" => KdeIntensityEffect::Lighten,
        other => panic!("no intensity effect `{other}`"),
    };
    let color_effect = match row["color"].as_str() {
        "none" => KdeColorEffect::None,
        "desaturate" => KdeColorEffect::Desaturate,
        "fade" => KdeColorEffect::Fade,
        "tint" => KdeColorEffect::Tint,
        other => panic!("no colour effect `{other}`"),
    };
    let contrast = match row["contrast"].as_str() {
        "none" => KdeContrastEffect::None,
        "fade" => KdeContrastEffect::Fade,
        "tint" => KdeContrastEffect::Tint,
        other => panic!("no contrast effect `{other}`"),
    };
    KdeGroupEffects {
        intensity,
        intensity_amount: number(&row["intensity_amount"]),
        color: color_effect,
        color_amount: number(&row["color_amount"]),
        color_color: channels(color(&row["color_color"])),
        contrast,
        contrast_amount: number(&row["contrast_amount"]),
    }
}

#[test]
fn vectors_kde_effects() {
    for row in rows("kde_effects") {
        let group = match row["group"].as_str() {
            "active" => KdeGroup::Active,
            "disabled" => KdeGroup::Disabled,
            "inactive" => KdeGroup::Inactive,
            other => panic!("no group `{other}`"),
        };
        let got = kde_effects(
            channels(color(&row["c"])),
            channels(color(&row["background"])),
            group,
            role_kind(&row["role_kind"]),
            &group_effects(&row),
            boolean(&row["inactive_enabled"]),
        );
        assert_color(from_channels(got), output(&row["result"]), &row);
    }
}

#[test]
fn vectors_kde_selection_inactive() {
    for row in rows("kde_selection_inactive") {
        let got = kde_selection_inactive(
            channels(color(&row["selection"])),
            channels(color(&row["window"])),
            channels(color(&row["selection_background"])),
            role_kind(&row["role_kind"]),
            boolean(&row["inactive_change_selection_color"]),
        );
        assert_color(from_channels(got), output(&row["result"]), &row);
    }
}

fn scheme(cell: &str) -> Scheme {
    match cell {
        "light" => Scheme::Light,
        "dark" => Scheme::Dark,
        other => panic!("no scheme `{other}`"),
    }
}

#[test]
fn vectors_adw_standalone() {
    let rows = rows("adw_standalone");
    assert_eq!(rows.len(), 628);
    for row in rows {
        let scheme = scheme(&row["scheme"]);
        let got = adw_standalone(channels(color(&row["c"])), scheme);
        assert_color(from_channels(got), output(&row["result"]), &row);
    }
}

/// A published row's value, `<n>px` (S2.2: Apple's tables, every row).
fn px(cell: &str) -> f64 {
    cell.strip_suffix("px").expect(cell).parse().expect(cell)
}

#[test]
fn vectors_sf_tracking() {
    let rows = rows("sf_tracking");
    assert_eq!(rows.len(), 185);
    for row in rows {
        let design = match row["design"].as_str() {
            "sf_pro" => TrackingDesign::SfPro,
            "sf_pro_rounded" => TrackingDesign::SfProRounded,
            "new_york" => TrackingDesign::NewYork,
            other => panic!("no design `{other}`"),
        };
        let got = sf_tracking(
            number(row["font_size"].strip_suffix("px").expect("a size in px")),
            design,
            number(&row["font_dpi"]),
        );
        let expected = px(&row["tracking"]);
        assert!(
            (f64::from(got) - expected).abs() <= 1e-6 * expected.abs().max(1.0),
            "{got} for {expected}: {row:?}"
        );
    }
}

#[test]
fn vectors_m3_role() {
    let rows = rows("m3_role");
    assert_eq!(rows.len(), 2556);
    for row in rows {
        let variant = match row["variant"].as_str() {
            "tonal_spot" => MaterialVariant::TonalSpot,
            other => panic!("no variant `{other}` in the table's scheme"),
        };
        let got = m3_role(
            row["role"].strip_prefix('@').expect("a keyed role"),
            channels(color(&row["accent"])),
            variant,
            &row["spec_version"],
            number(&row["contrast_level"]),
            scheme(&row["scheme"]),
        )
        .expect("a role of the implemented scheme");
        assert_color(from_channels(got), output(&row["result"]), &row);
    }
}

/// A size cell, `<n>px`, as the f32 it reads back to.
fn size_px(cell: &str) -> f32 {
    number(cell.strip_suffix("px").expect(cell))
}

#[test]
fn vectors_android_non_linear() {
    let rows = rows("android_non_linear");
    assert_eq!(rows.len(), 1425);
    for row in rows {
        let got = android_non_linear(size_px(&row["size"]), number(&row["font_scale"]));
        assert_eq!(got, size_px(&row["result"]), "{row:?}");
    }
}

#[test]
fn vectors_android_non_linear_inverse() {
    let rows = rows("android_non_linear_inverse");
    assert_eq!(rows.len(), 1425);
    for row in rows {
        let got = android_non_linear_inverse(size_px(&row["size"]), number(&row["font_scale"]));
        assert_eq!(got, size_px(&row["result"]), "{row:?}");
    }
}

#[test]
fn vectors_win_ui() {
    let rows = rows("win_ui");
    assert_eq!(rows.len(), 1434);
    for row in rows {
        let got = win_ui(size_px(&row["size"]), number(&row["factor"]));
        assert_eq!(got, size_px(&row["result"]), "{row:?}");
    }
}

fn text_role(cell: &str) -> TextRole {
    match cell {
        "large_title" => TextRole::LargeTitle,
        "title1" => TextRole::Title1,
        "title2" => TextRole::Title2,
        "title3" => TextRole::Title3,
        "headline" => TextRole::Headline,
        "body" => TextRole::Body,
        "callout" => TextRole::Callout,
        "subheadline" => TextRole::Subheadline,
        "footnote" => TextRole::Footnote,
        "caption" => TextRole::Caption,
        "caption2" => TextRole::Caption2,
        other => panic!("no text role `{other}` in the table"),
    }
}

fn category(cell: &str) -> ContentSizeCategory {
    match cell {
        "extra_small" => ContentSizeCategory::ExtraSmall,
        "small" => ContentSizeCategory::Small,
        "medium" => ContentSizeCategory::Medium,
        "large" => ContentSizeCategory::Large,
        "extra_large" => ContentSizeCategory::ExtraLarge,
        "extra_extra_large" => ContentSizeCategory::ExtraExtraLarge,
        "extra_extra_extra_large" => ContentSizeCategory::ExtraExtraExtraLarge,
        "accessibility_medium" => ContentSizeCategory::AccessibilityMedium,
        "accessibility_large" => ContentSizeCategory::AccessibilityLarge,
        "accessibility_extra_large" => ContentSizeCategory::AccessibilityExtraLarge,
        "accessibility_extra_extra_large" => ContentSizeCategory::AccessibilityExtraExtraLarge,
        "accessibility_extra_extra_extra_large" => {
            ContentSizeCategory::AccessibilityExtraExtraExtraLarge
        }
        other => panic!("no content size category `{other}`"),
    }
}

#[test]
fn vectors_dynamic_type() {
    let rows = rows("dynamic_type");
    assert_eq!(rows.len(), 132);
    for row in rows {
        let got = dynamic_type(
            text_role(&row["role"]),
            category(&row["category"]),
            number(&row["font_dpi"]),
        )
        .expect("a role the table has a row for");
        assert_eq!(got.font_size, size_px(&row["font_size"]), "{row:?}");
        assert_eq!(got.line_height, size_px(&row["line_height"]), "{row:?}");
    }
}
