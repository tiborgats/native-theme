//! The platform functions over their vector tables (S2.2 *Checked against
//! their references*): each row's inputs in, the reference's output out, a
//! colour compared unrounded on the 0-255 scale to 1e-3.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::HashMap;

use native_theme::next::platform::adw_standalone::adw_standalone;
use native_theme::next::platform::kde_shade::{KdeShadeRole, kde_shade};
use native_theme::next::platform::kirigami_lerp::kirigami_lerp;
use native_theme::next::platform::sf_tracking::{TrackingDesign, sf_tracking};
use native_theme::next::{Color, Scheme};

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
fn vectors_adw_standalone() {
    let rows = rows("adw_standalone");
    assert_eq!(rows.len(), 628);
    for row in rows {
        let scheme = match row["scheme"].as_str() {
            "light" => Scheme::Light,
            "dark" => Scheme::Dark,
            other => panic!("no scheme `{other}`"),
        };
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
