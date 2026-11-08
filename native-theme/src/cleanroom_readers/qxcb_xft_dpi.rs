//! The DPI Qt's xcb platform plugin forces from the X resources' `Xft.dpi` line, as it reads the
//! root window's `RESOURCE_MANAGER` property at start (the XSETTINGS `Xft/DPI` is not modelled).

/// The bytes an `Xft.dpi` line starts with: the name, its colon and one tab.
const XFT_DPI: &[u8] = b"Xft.dpi:\t";

/// The ASCII white space Qt's byte parsers skip, vertical tab included.
fn is_ws(b: u8) -> bool {
    matches!(b, b'\t' | b'\n' | 0x0b | 0x0c | b'\r' | b' ')
}

/// `QByteArray::toInt` at base 10: `None` where Qt's `ok` is false.
fn int(bytes: &[u8]) -> Option<i32> {
    let start = bytes.iter().take_while(|b| is_ws(**b)).count();
    let rest = bytes.get(start..)?;
    let (negative, rest) = match rest.split_first() {
        Some((b'-', tail)) => (true, tail),
        Some((b'+', tail)) => (false, tail),
        _ => (false, rest),
    };
    let digits = rest.iter().take_while(|b| b.is_ascii_digit()).count();
    if digits == 0 {
        return None;
    }
    let (run, tail) = rest.split_at_checked(digits)?;
    let spaces = tail.iter().take_while(|b| is_ws(**b)).count();
    if !matches!(tail.get(spaces), None | Some(0)) {
        return None;
    }
    let mut magnitude: i64 = 0;
    for b in run {
        magnitude = magnitude
            .checked_mul(10)?
            .checked_add(i64::from(b.checked_sub(b'0')?))?;
    }
    let value = if negative {
        magnitude.checked_neg()?
    } else {
        magnitude
    };
    i32::try_from(value).ok()
}

/// Whether `text` is a whole decimal number: sign, digits with at most one `.`, at least one
/// digit, and an optional exponent of `e` or `E`, sign and one or more digits.
fn is_decimal(text: &[u8]) -> bool {
    let mut i = 0usize;
    if matches!(text.first(), Some(b'+' | b'-')) {
        i = 1;
    }
    let mut digits = 0usize;
    while text.get(i).is_some_and(u8::is_ascii_digit) {
        i = i.saturating_add(1);
        digits = digits.saturating_add(1);
    }
    if text.get(i) == Some(&b'.') {
        i = i.saturating_add(1);
        while text.get(i).is_some_and(u8::is_ascii_digit) {
            i = i.saturating_add(1);
            digits = digits.saturating_add(1);
        }
    }
    if digits == 0 {
        return false;
    }
    if matches!(text.get(i), Some(b'e' | b'E')) {
        i = i.saturating_add(1);
        if matches!(text.get(i), Some(b'+' | b'-')) {
            i = i.saturating_add(1);
        }
        let start = i;
        while text.get(i).is_some_and(u8::is_ascii_digit) {
            i = i.saturating_add(1);
        }
        if i == start {
            return false;
        }
    }
    i == text.len()
}

/// `QByteArray::toDouble`: `None` where Qt's `ok` is false, overflow and underflow included.
fn double(bytes: &[u8]) -> Option<f64> {
    let start = bytes.iter().take_while(|b| is_ws(**b)).count();
    let end = bytes
        .len()
        .saturating_sub(bytes.iter().rev().take_while(|b| is_ws(**b)).count());
    let text = bytes.get(start..end.max(start))?;
    let first = *text.first()?;
    if text.len() >= 3 && matches!(first, b'-' | b'+' | b'I' | b'i' | b'N' | b'n') {
        let signed = matches!(first, b'-' | b'+');
        let word = if signed { text.get(1..)? } else { text };
        if word.first().is_some_and(|c| *c > b'9') {
            if word.eq_ignore_ascii_case(b"inf") {
                return Some(if first == b'-' {
                    f64::NEG_INFINITY
                } else {
                    f64::INFINITY
                });
            }
            if !signed && word.eq_ignore_ascii_case(b"nan") {
                return Some(f64::NAN);
            }
            return None;
        }
    }
    if !is_decimal(text) {
        return None;
    }
    let value: f64 = std::str::from_utf8(text).ok()?.parse().ok()?;
    if value.is_infinite() {
        return None;
    }
    let mantissa = text
        .split(|b| matches!(b, b'e' | b'E'))
        .next()
        .unwrap_or_default();
    if value == 0.0 && mantissa.iter().any(|b| matches!(b, b'1'..=b'9')) {
        return None;
    }
    Some(value)
}

/// Qt's `qRound` on x86-64: `d + copysign(0.5, d)` truncated toward zero; `i32::MIN` where that
/// is NaN, infinite or outside the 32-bit range, as the x86-64 conversion gives.
#[allow(
    clippy::cast_possible_truncation,
    clippy::as_conversions,
    reason = "the cast's operand is integral and within i32's range"
)]
fn q_round(d: f64) -> i32 {
    let t = (d + 0.5f64.copysign(d)).trunc();
    if t.is_nan() || t < f64::from(i32::MIN) || t > f64::from(i32::MAX) {
        return i32::MIN;
    }
    t as i32
}

/// `parseXftDpi`: the value as an integer, else as a decimal rounded by `qRound`.
fn parse_xft_dpi(value: &[u8]) -> Option<i32> {
    int(value).or_else(|| double(value).map(q_round))
}

/// The DPI Qt's xcb plugin forces from the X resources `resources` (`xrdb -query`'s text, empty
/// where the property is absent): the last `Xft.dpi` line whose value parses decides, and its
/// value counts only where it is positive.
pub fn qxcb_xft_dpi(resources: &str) -> Option<u32> {
    let mut forced: i32 = -1;
    for line in resources.as_bytes().split(|b| *b == b'\n') {
        if let Some(value) = line.strip_prefix(XFT_DPI)
            && let Some(v) = parse_xft_dpi(value)
        {
            forced = v;
        }
    }
    if forced > 0 {
        u32::try_from(forced).ok()
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn json_string(cell: &str) -> Result<String, String> {
        let inner = cell
            .strip_prefix('"')
            .and_then(|c| c.strip_suffix('"'))
            .ok_or_else(|| format!("not a JSON string: {cell}"))?;
        let mut out = String::new();
        let mut chars = inner.chars();
        let hex4 = |chars: &mut std::str::Chars<'_>| -> Result<u32, String> {
            let h: String = chars.by_ref().take(4).collect();
            if h.len() != 4 {
                return Err(format!("short \\u escape in {cell}"));
            }
            u32::from_str_radix(&h, 16).map_err(|e| e.to_string())
        };
        while let Some(c) = chars.next() {
            if c != '\\' {
                out.push(c);
                continue;
            }
            let e = chars.next().ok_or("trailing backslash")?;
            match e {
                '"' => out.push('"'),
                '\\' => out.push('\\'),
                '/' => out.push('/'),
                'b' => out.push('\u{08}'),
                'f' => out.push('\u{0c}'),
                'n' => out.push('\n'),
                'r' => out.push('\r'),
                't' => out.push('\t'),
                'u' => {
                    let hi = hex4(&mut chars)?;
                    let code = if (0xD800..0xDC00).contains(&hi) {
                        if chars.next() != Some('\\') || chars.next() != Some('u') {
                            return Err(format!("lone surrogate in {cell}"));
                        }
                        let lo = hex4(&mut chars)?;
                        0x10000 + ((hi - 0xD800) << 10) + (lo.wrapping_sub(0xDC00) & 0x3FF)
                    } else {
                        hi
                    };
                    out.push(char::from_u32(code).ok_or("bad code point")?);
                }
                other => return Err(format!("bad escape \\{other} in {cell}")),
            }
        }
        Ok(out)
    }

    fn expected(cells: &[&str]) -> Result<(String, Option<u32>), String> {
        let [resources, dpi] = cells else {
            return Err(format!("{} cells", cells.len()));
        };
        let want = if *dpi == "none" {
            None
        } else {
            Some(
                dpi.parse::<u32>()
                    .map_err(|_| format!("not a number: {dpi}"))?,
            )
        };
        Ok((json_string(resources)?, want))
    }

    #[test]
    fn vectors_reader_qxcb_xft_dpi() {
        let table = include_str!("../../tests/vectors/reader.qxcb_xft_dpi.tsv");
        let mut lines = table.lines().filter(|l| !l.starts_with('#'));
        assert_eq!(lines.next(), Some("resources\tdpi"));
        let mut rows = 0usize;
        let mut failures = Vec::new();
        for line in lines {
            rows += 1;
            let cells: Vec<&str> = line.split('\t').collect();
            match expected(&cells) {
                Ok((input, want)) => {
                    let got = qxcb_xft_dpi(&input);
                    if got != want {
                        failures.push(format!("{line}\n  got  {got:?}\n  want {want:?}"));
                    }
                }
                Err(e) => failures.push(format!("{line}\n  unreadable row: {e}")),
            }
        }
        assert!(
            failures.is_empty(),
            "{} of {rows} rows fail:\n{}",
            failures.len(),
            failures.join("\n")
        );
        assert_eq!(rows, 97);
        println!("qxcb_xft_dpi: {rows} of {rows} rows pass");
    }

    #[test]
    fn rounding_is_one_double_addition() {
        assert_eq!(q_round(0.499_999_999_999_999_94), 1);
        assert_eq!(q_round(-0.5), -1);
        assert_eq!(q_round(-0.4), 0);
        assert_eq!(q_round(2_147_483_647.4), i32::MAX);
        assert_eq!(q_round(-2_147_483_648.4), i32::MIN);
        assert_eq!(q_round(2_147_483_647.5), i32::MIN);
        assert_eq!(q_round(f64::NAN), i32::MIN);
        assert_eq!(q_round(f64::INFINITY), i32::MIN);
    }

    #[test]
    fn doubles_in_qt_words_and_at_the_ranges() {
        assert_eq!(double(b"-inf"), Some(f64::NEG_INFINITY));
        assert_eq!(double(b" INF "), Some(f64::INFINITY));
        assert!(double(b"NaN").is_some_and(f64::is_nan));
        assert_eq!(double(b"-nan"), None);
        assert_eq!(double(b"+nan"), None);
        assert_eq!(double(b"-e5"), None);
        assert_eq!(double(b"Infinity"), None);
        assert_eq!(double(b"1e400"), None);
        assert_eq!(double(b"1e-400"), None);
        assert_eq!(double(b"0e5"), Some(0.0));
        assert_eq!(double(b"-.5"), Some(-0.5));
        assert_eq!(double(b"."), None);
        assert_eq!(double(b"-."), None);
        assert_eq!(double(b" \t"), None);
    }
}
