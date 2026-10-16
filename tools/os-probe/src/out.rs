//! Building `probe.toml`: value constructors shared by the platform probes.
//!
//! A query that fails or does not exist on the running OS is recorded as a
//! string starting with `unavailable:` (the OS was asked and gave no answer)
//! or `not probed:` (the probe does not ask), never by aborting the run.

use toml::{Table, Value};

/// A query the OS could not answer.
pub(crate) fn unavailable(reason: impl std::fmt::Display) -> Value {
    Value::String(format!("unavailable: {reason}"))
}

/// A query the probe deliberately does not make.
pub(crate) fn not_probed(reason: &str) -> Value {
    Value::String(format!("not probed: {reason}"))
}

pub(crate) fn text(s: impl Into<String>) -> Value {
    Value::String(s.into())
}

pub(crate) fn float(v: f64) -> Value {
    Value::Float(v)
}

#[cfg_attr(not(target_os = "windows"), allow(dead_code))]
pub(crate) fn int(v: i64) -> Value {
    Value::Integer(v)
}

/// An `isize` (NSInteger) as a TOML integer; on a target where it does not
/// fit an `i64`, its decimal text.
#[cfg_attr(not(target_os = "macos"), allow(dead_code))]
pub(crate) fn int_isize(v: isize) -> Value {
    i64::try_from(v).map_or_else(|_| Value::String(v.to_string()), Value::Integer)
}

pub(crate) fn boolean(v: bool) -> Value {
    Value::Boolean(v)
}

pub(crate) fn array(values: Vec<Value>) -> Value {
    Value::Array(values)
}

/// A float array, for sizes (`[width, height]`), rectangles
/// (`[x, y, width, height]`) and insets (`[top, left, bottom, right]`).
#[cfg_attr(not(target_os = "macos"), allow(dead_code))]
pub(crate) fn floats(values: &[f64]) -> Value {
    Value::Array(values.iter().copied().map(Value::Float).collect())
}

/// A colour channel in 0.0..=1.0 as a byte, rounded to nearest; values
/// outside the range (extended sRGB) are clamped, NaN gives 0.
#[cfg_attr(not(target_os = "macos"), allow(dead_code))]
pub(crate) fn unit_to_byte(c: f64) -> u8 {
    let scaled = (c.clamp(0.0, 1.0) * 255.0).round();
    if scaled.is_nan() {
        return 0;
    }
    // In 0.0..=255.0 after the clamp, so the cast is exact; `as` from a
    // float saturates and cannot panic.
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let byte = scaled as u8;
    byte
}

/// `#rrggbbaa` from sRGB channels in 0.0..=1.0.
#[cfg_attr(not(target_os = "macos"), allow(dead_code))]
pub(crate) fn rgba_hex(r: f64, g: f64, b: f64, a: f64) -> String {
    format!(
        "#{:02x}{:02x}{:02x}{:02x}",
        unit_to_byte(r),
        unit_to_byte(g),
        unit_to_byte(b),
        unit_to_byte(a)
    )
}

/// A length in points times a scale, rounded up to whole pixels; `None`
/// unless the result is a finite value in 1..=16384.
#[cfg_attr(not(target_os = "macos"), allow(dead_code))]
pub(crate) fn pixels(points: f64, scale: f64) -> Option<isize> {
    let px = (points * scale).ceil();
    if !px.is_finite() || !(1.0..=16384.0).contains(&px) {
        return None;
    }
    // In 1.0..=16384.0, so the cast is exact.
    #[allow(clippy::cast_possible_truncation)]
    let whole = px as isize;
    Some(whole)
}

/// Insert `key = value` into `table`.
pub(crate) fn put(table: &mut Table, key: &str, value: Value) {
    table.insert(key.to_owned(), value);
}

/// Insert a sub-table, returning nothing: callers build the sub-table first.
pub(crate) fn put_table(table: &mut Table, key: &str, sub: Table) {
    table.insert(key.to_owned(), Value::Table(sub));
}
