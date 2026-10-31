//! The platform functions over their vector tables (S2.2 *Checked against
//! their references*): each row's inputs in, the reference's output out, a
//! colour compared unrounded on the 0-255 scale to 1e-3.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::HashMap;

use native_theme::next::Color;
use native_theme::next::platform::kde_shade::{KdeShadeRole, kde_shade};
use native_theme::next::platform::kirigami_lerp::kirigami_lerp;

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

/// An input colour, a literal of S2.1.
fn color(cell: &str) -> Color {
    Color::parse(cell).expect(cell)
}

/// An output colour, `srgb(r, g, b, a)` with the reference's unrounded
/// channels.
fn output(cell: &str) -> [f64; 4] {
    let inner = cell
        .strip_prefix("srgb(")
        .and_then(|rest| rest.strip_suffix(')'))
        .expect(cell);
    let channels: Vec<f64> = inner
        .split(',')
        .map(|n| n.trim().parse().expect(cell))
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
