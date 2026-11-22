//! The kernels' vector tables read for their unit tests, ported from
//! `v0.8.0-rc1`'s `tests/platform_vectors.rs`: each row's inputs in, the
//! reference's output out, a colour compared unrounded on the 0-255 scale to
//! 1e-3.

use std::collections::HashMap;

/// A test's result.
pub(super) type TestResult = Result<(), Box<dyn std::error::Error>>;

/// One row of a table, each cell by its column name.
#[derive(Debug)]
pub(super) struct Row<'a> {
    line: &'a str,
    cells: HashMap<&'a str, &'a str>,
}

impl<'a> Row<'a> {
    /// The cell of `column`.
    pub(super) fn get(&self, column: &str) -> Result<&'a str, String> {
        self.cells
            .get(column)
            .copied()
            .ok_or_else(|| format!("no column `{column}` in `{}`", self.line))
    }
}

/// A table's rows: the `#` header lines skipped, then the column names, then
/// one row per line; every row has a cell per column, and there is a row.
pub(super) fn rows(table: &str) -> Result<Vec<Row<'_>>, String> {
    let mut lines = table.lines().skip_while(|line| line.starts_with("# "));
    let columns: Vec<&str> = lines
        .next()
        .ok_or("no column header row")?
        .split('\t')
        .collect();
    let rows = lines
        .filter(|line| !line.is_empty())
        .map(|line| {
            let cells: Vec<&str> = line.split('\t').collect();
            if cells.len() == columns.len() {
                Ok(Row {
                    line,
                    cells: columns.iter().copied().zip(cells).collect(),
                })
            } else {
                Err(format!(
                    "{} cells for {} columns: `{line}`",
                    cells.len(),
                    columns.len()
                ))
            }
        })
        .collect::<Result<Vec<_>, String>>()?;
    if rows.is_empty() {
        return Err("no row".to_owned());
    }
    Ok(rows)
}

/// An input colour as the kernels take it: `#rrggbb` or `#rrggbbaa`
/// (straight alpha, each byte over 255), or `srgb(r, g, b, a)` with the
/// reference's singles, which may carry an exponent (`-5.19812102e-06`).
pub(super) fn color(cell: &str) -> Result<[f32; 4], String> {
    match cell.strip_prefix('#') {
        Some(hex) => {
            let byte = |i: usize| {
                hex.get(i..i.saturating_add(2))
                    .and_then(|pair| u8::from_str_radix(pair, 16).ok())
                    .map(|b| f32::from(b) / 255.0_f32)
                    .ok_or_else(|| format!("no colour `{cell}`"))
            };
            let alpha = match hex.len() {
                6 => 1.0,
                8 => byte(6)?,
                _ => return Err(format!("no colour `{cell}`")),
            };
            Ok([byte(0)?, byte(2)?, byte(4)?, alpha])
        }
        None => {
            let channels = srgb(cell)?;
            let [r, g, b, a] = channels.as_slice() else {
                return Err(format!("no four channels in `{cell}`"));
            };
            Ok([*r, *g, *b, *a])
        }
    }
}

/// An output colour, `srgb(r, g, b, a)` with the reference's unrounded
/// channels.
pub(super) fn output(cell: &str) -> Result<[f64; 4], String> {
    let channels = srgb(cell)?;
    let [r, g, b, a] = channels.as_slice() else {
        return Err(format!("no four channels in `{cell}`"));
    };
    Ok([*r, *g, *b, *a])
}

/// The channels of `srgb(…)`, each parsed as `T`.
fn srgb<T: std::str::FromStr>(cell: &str) -> Result<Vec<T>, String> {
    cell.strip_prefix("srgb(")
        .and_then(|rest| rest.strip_suffix(')'))
        .ok_or_else(|| format!("no `srgb(…)` in `{cell}`"))?
        .split(',')
        .map(|n| {
            n.trim()
                .parse()
                .map_err(|_| format!("no number `{n}` in `{cell}`"))
        })
        .collect()
}

/// A number cell.
pub(super) fn number(cell: &str) -> Result<f32, String> {
    cell.parse().map_err(|_| format!("no number `{cell}`"))
}

/// A `true` or `false` cell.
pub(super) fn boolean(cell: &str) -> Result<bool, String> {
    match cell {
        "true" => Ok(true),
        "false" => Ok(false),
        other => Err(format!("no bool `{other}`")),
    }
}

/// `got` is `expected` on the 0-255 scale to 1e-3, every channel.
pub(super) fn assert_color(got: [f32; 4], expected: [f64; 4], row: &Row<'_>) {
    for (got, expected) in got.map(f64::from).iter().zip(expected) {
        assert!(
            ((got - expected) * 255.0).abs() <= 1e-3,
            "{got:?} for {expected:?}: `{}`",
            row.line
        );
    }
}
