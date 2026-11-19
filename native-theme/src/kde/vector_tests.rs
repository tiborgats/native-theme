//! The vector tables of the carried clean-room functions, read as step 1's
//! `shade` test reads its own.
//!
//! Every channel of every row passes where `|got × 255 − expected × 255| ≤
//! 1e-3`. An unparsable row is a failure of its own, reported with the rows
//! whose colour differs.

/// `kcu_tint`'s table (`tests/vectors/kcu_tint.tsv`).
const KCU_TINT: &str = include_str!("../../tests/vectors/kcu_tint.tsv");

/// The inactive selection's table (`tests/vectors/kde_selection_inactive.tsv`).
const SELECTION_INACTIVE: &str = include_str!("../../tests/vectors/kde_selection_inactive.tsv");

/// An input cell: `#rrggbb`, `#rrggbbaa` (straight alpha, each byte over
/// 255) or `srgb(r, g, b, a)`.
fn parse_colour(cell: &str) -> Option<[f32; 4]> {
    if let Some(hex) = cell.strip_prefix('#') {
        let mut out = [0.0f32, 0.0, 0.0, 1.0];
        let channels = match hex.len() {
            6 => 3,
            8 => 4,
            _ => return None,
        };
        for (i, slot) in out.iter_mut().take(channels).enumerate() {
            let part = hex.get(2 * i..2 * i + 2)?;
            let n = u8::from_str_radix(part, 16).ok()?;
            *slot = f32::from(n) / 255.0f32;
        }
        Some(out)
    } else {
        parse_srgb(cell)
    }
}

/// A `srgb(r, g, b, a)` cell as its four channels.
fn parse_srgb(cell: &str) -> Option<[f32; 4]> {
    let inner = cell.strip_prefix("srgb(")?.strip_suffix(')')?;
    let parts: Vec<f32> = inner
        .split(',')
        .map(|p| p.trim().parse::<f32>().ok())
        .collect::<Option<Vec<f32>>>()?;
    match parts.as_slice() {
        [r, g, b, a] => Some([*r, *g, *b, *a]),
        _ => None,
    }
}

/// The table's data rows: its comment lines (a `#` and no tab), blank lines
/// and its header skipped.
fn rows(table: &str) -> Vec<&str> {
    table
        .lines()
        .filter(|line| !(line.starts_with('#') && !line.contains('\t') || line.trim().is_empty()))
        .skip(1)
        .collect()
}

/// Whether `got` is `expected` within the tolerance, on every channel.
fn within(got: [f32; 4], expected: [f32; 4]) -> bool {
    got.iter()
        .zip(expected.iter())
        .all(|(g, e)| (f64::from(*g) * 255.0 - f64::from(*e) * 255.0).abs() <= 1e-3)
}

#[test]
fn kcu_tint_vectors() {
    let rows = rows(KCU_TINT);
    let mut failures: Vec<String> = Vec::new();
    for line in &rows {
        let cols: Vec<&str> = line.split('\t').collect();
        let parsed = match cols.as_slice() {
            [a, b, amount, result] => parse_colour(a).and_then(|a| {
                let b = parse_colour(b)?;
                let amount = amount.parse::<f32>().ok()?;
                let expected = parse_srgb(result)?;
                Some((a, b, amount, expected))
            }),
            _ => None,
        };
        let Some((a, b, amount, expected)) = parsed else {
            failures.push(format!("unparsable row: {line}"));
            continue;
        };
        let got = super::kcu_tint::kcu_tint(a, b, amount);
        if !within(got, expected) {
            failures.push(format!("{line}\n  got {got:?}"));
        }
    }
    assert_eq!(rows.len(), 124, "row count");
    assert!(
        failures.is_empty(),
        "{} of {} rows failed:\n{}",
        failures.len(),
        rows.len(),
        failures.join("\n")
    );
}

#[test]
fn the_readers_inactive_selection_is_the_tables() {
    let rows = rows(SELECTION_INACTIVE);
    let mut failures: Vec<String> = Vec::new();
    for line in &rows {
        let cols: Vec<&str> = line.split('\t').collect();
        let parsed = match cols.as_slice() {
            [
                selection,
                window,
                selection_background,
                role_kind,
                change,
                result,
            ] => parse_colour(selection).and_then(|selection| {
                let window = parse_colour(window)?;
                let selection_background = parse_colour(selection_background)?;
                let background = match *role_kind {
                    "background" => true,
                    "foreground" | "decoration" => false,
                    _ => return None,
                };
                let change = match *change {
                    "true" => true,
                    "false" => false,
                    _ => return None,
                };
                let expected = parse_srgb(result)?;
                Some((
                    selection,
                    window,
                    selection_background,
                    background,
                    change,
                    expected,
                ))
            }),
            _ => None,
        };
        let Some((selection, window, selection_background, background, change, expected)) = parsed
        else {
            failures.push(format!("unparsable row: {line}"));
            continue;
        };
        let got = super::colors::inactive_selection(
            Some(selection),
            Some(window),
            Some(selection_background),
            background,
            change,
        );
        match got {
            Some(got) if within(got, expected) => {}
            _ => failures.push(format!("{line}\n  got {got:?}")),
        }
    }
    assert_eq!(rows.len(), 70, "row count");
    assert!(
        failures.is_empty(),
        "{} of {} rows failed:\n{}",
        failures.len(),
        rows.len(),
        failures.join("\n")
    );
}
