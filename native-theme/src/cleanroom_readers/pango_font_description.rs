//! Pango's font-description string parser, as GNOME's reader reads the gsettings font strings.
//! Its arithmetic was made panic-free after the clean room, without reading LGPL source.

/// A parsed Pango font description: each optional field is `None` where the parser left it unset.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PangoDescription {
    /// The family list, its entries joined by `,`; `None` when unset.
    pub family: Option<String>,
    /// Pango's style integer (Normal 0, Oblique 1, Italic 2, or a `style=N` value).
    pub style: i32,
    /// Pango's variant integer.
    pub variant: i32,
    /// Pango's weight integer.
    pub weight: i32,
    /// Pango's width integer, in permille of normal.
    pub width: i32,
    /// Pango's gravity integer; `None` when unset.
    pub gravity: Option<i32>,
    /// Pango's font-colour integer; `None` when unset.
    pub color: Option<i32>,
    /// The size in Pango units (1/1024 of a point, or of a pixel when `absolute`); `None` when unset.
    pub size: Option<i32>,
    /// Whether the size is in pixels.
    pub absolute: bool,
    /// The variations text after `@`; `None` when unset.
    pub variations: Option<String>,
    /// The features text after `#`; `None` when unset.
    pub features: Option<String>,
}

impl Default for PangoDescription {
    fn default() -> Self {
        Self {
            family: None,
            style: 0,
            variant: 0,
            weight: 400,
            width: 1000,
            gravity: None,
            color: None,
            size: None,
            absolute: false,
            variations: None,
            features: None,
        }
    }
}

fn is_space(b: u8) -> bool {
    b.is_ascii_whitespace()
}

fn byte_before(bytes: &[u8], i: usize) -> Option<u8> {
    i.checked_sub(1).and_then(|j| bytes.get(j).copied())
}

/// Returns `(start, end)` of the word ending at `end` (after skipping spaces left of `end`).
fn getword(bytes: &[u8], end: usize, comma_stops: bool) -> (usize, usize) {
    let mut end = end.min(bytes.len());
    while let Some(b) = byte_before(bytes, end) {
        if !is_space(b) {
            break;
        }
        end = end.saturating_sub(1);
    }
    let mut start = end;
    while let Some(b) = byte_before(bytes, start) {
        if is_space(b) || (comma_stops && b == b',') {
            break;
        }
        start = start.saturating_sub(1);
    }
    (start, end)
}

fn matches(name: &str, word: &str) -> bool {
    let name = name.as_bytes();
    let word = word.as_bytes();
    let mut ni = 0usize;
    let mut wi = 0usize;
    while wi < word.len() {
        let Some(&nb) = name.get(ni) else {
            break;
        };
        let Some(&wb) = word.get(wi) else {
            break;
        };
        if nb.to_ascii_lowercase() == wb.to_ascii_lowercase() {
            ni = ni.saturating_add(1);
            wi = wi.saturating_add(1);
        } else if nb == b'-' {
            ni = ni.saturating_add(1);
        } else {
            return false;
        }
    }
    wi == word.len() && ni == name.len()
}

fn parse_int(candidate: &str) -> Option<i32> {
    let rest = candidate.trim_start_matches('\u{0b}');
    let (negative, digits) = if let Some(d) = rest.strip_prefix('+') {
        (false, d)
    } else if let Some(d) = rest.strip_prefix('-') {
        (true, d)
    } else {
        (false, rest)
    };
    if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    let mut value: i64 = 0;
    for b in digits.bytes() {
        value = value
            .checked_mul(10)?
            .checked_add(i64::from(b.checked_sub(b'0')?))?;
    }
    let value = if negative {
        value.checked_neg()?
    } else {
        value
    };
    let value = i32::try_from(value).ok()?;
    (value >= 0).then_some(value)
}

#[derive(Clone, Copy)]
enum Field {
    Weight,
    Style,
    Width,
    Variant,
    Gravity,
    Color,
}

const WEIGHTS: &[(&str, i32)] = &[
    ("Thin", 100),
    ("Ultra-Light", 200),
    ("Extra-Light", 200),
    ("Light", 300),
    ("Semi-Light", 350),
    ("Demi-Light", 350),
    ("Book", 380),
    ("Regular", 400),
    ("Medium", 500),
    ("Semi-Bold", 600),
    ("Demi-Bold", 600),
    ("Bold", 700),
    ("Ultra-Bold", 800),
    ("Extra-Bold", 800),
    ("Heavy", 900),
    ("Black", 900),
    ("Ultra-Heavy", 1000),
    ("Extra-Heavy", 1000),
    ("Ultra-Black", 1000),
    ("Extra-Black", 1000),
];

const STYLES: &[(&str, i32)] = &[("Roman", 0), ("Oblique", 1), ("Italic", 2)];

const WIDTHS: &[(&str, i32)] = &[
    ("Ultra-Condensed", 500),
    ("Extra-Condensed", 625),
    ("Condensed", 750),
    ("Semi-Condensed", 875),
    ("Semi-Expanded", 1125),
    ("Expanded", 1250),
    ("Extra-Expanded", 1500),
    ("Ultra-Expanded", 2000),
];

const VARIANTS: &[(&str, i32)] = &[
    ("Small-Caps", 1),
    ("All-Small-Caps", 2),
    ("Petite-Caps", 3),
    ("All-Petite-Caps", 4),
    ("Unicase", 5),
    ("Title-Caps", 6),
];

const GRAVITIES: &[(&str, i32)] = &[
    ("Not-Rotated", 0),
    ("South", 0),
    ("Upside-Down", 2),
    ("North", 2),
    ("Rotated-Left", 1),
    ("East", 1),
    ("Rotated-Right", 3),
    ("West", 3),
];

const COLORS: &[(&str, i32)] = &[("With-Color", 1), ("Without-Color", 0)];

const FIELDS: &[(Field, &str, &[(&str, i32)])] = &[
    (Field::Weight, "weight", WEIGHTS),
    (Field::Style, "style", STYLES),
    (Field::Width, "width", WIDTHS),
    (Field::Variant, "variant", VARIANTS),
    (Field::Gravity, "gravity", GRAVITIES),
    (Field::Color, "color", COLORS),
];

fn apply(desc: &mut PangoDescription, field: Field, value: i32) {
    match field {
        Field::Weight => desc.weight = value,
        Field::Style => desc.style = value,
        Field::Width => desc.width = value,
        Field::Variant => desc.variant = value,
        Field::Gravity => desc.gravity = Some(value),
        Field::Color => desc.color = Some(value),
    }
}

/// Applies `word` to `desc` if it is a style word, and says whether it was.
fn style_word(desc: &mut PangoDescription, word: &str) -> bool {
    if matches("Normal", word) {
        return true;
    }
    for &(field, prefix, table) in FIELDS {
        let prefixed = word
            .strip_prefix(prefix)
            .and_then(|rest| rest.strip_prefix('='));
        let candidate = prefixed.unwrap_or(word);
        if let Some(&(_, value)) = table.iter().find(|(name, _)| matches(name, candidate)) {
            apply(desc, field, value);
            return true;
        }
        if prefixed.is_some() {
            if let Some(value) = parse_int(candidate) {
                apply(desc, field, value);
                return true;
            }
        }
    }
    false
}

fn digits_end(bytes: &[u8], from: usize, hex: bool) -> usize {
    let mut i = from;
    while let Some(&b) = bytes.get(i) {
        let ok = if hex {
            b.is_ascii_hexdigit()
        } else {
            b.is_ascii_digit()
        };
        if !ok {
            break;
        }
        i = i.saturating_add(1);
    }
    i
}

fn starts_with_ignore_case(bytes: &[u8], at: usize, pat: &[u8]) -> bool {
    at.checked_add(pat.len())
        .and_then(|end| bytes.get(at..end))
        .is_some_and(|s| s.eq_ignore_ascii_case(pat))
}

/// Converts a hexadecimal mantissa (with an optional point) and binary exponent to the nearest double.
fn hex_value(int_digits: &[u8], frac_digits: &[u8], exp: i64) -> f64 {
    let mut mantissa: u64 = 0;
    let mut sticky = false;
    let mut scale: i64 = exp;
    let mut seen_nonzero = false;
    for (index, &d) in int_digits.iter().chain(frac_digits.iter()).enumerate() {
        let v = char::from(d).to_digit(16).unwrap_or(0);
        let in_frac = index >= int_digits.len();
        if mantissa >> 60 == 0 {
            mantissa = (mantissa << 4) | u64::from(v);
            if in_frac {
                scale = scale.saturating_sub(4);
            }
            if v != 0 {
                seen_nonzero = true;
            }
        } else {
            if v != 0 {
                sticky = true;
            }
            if !in_frac {
                scale = scale.saturating_add(4);
            }
        }
    }
    if !seen_nonzero {
        return 0.0;
    }
    let bits = u64::BITS.saturating_sub(mantissa.leading_zeros());
    let mut kept = mantissa;
    if let Some(shift) = bits.checked_sub(53).filter(|&shift| shift > 0) {
        let dropped_mask = !(u64::MAX << shift);
        let dropped = mantissa & dropped_mask;
        let half = (1u64 << shift) >> 1;
        kept = mantissa >> shift;
        scale = scale.saturating_add(i64::from(shift));
        let round_up = dropped > half || (dropped == half && (sticky || kept & 1 == 1));
        if round_up {
            kept = kept.saturating_add(1);
        }
    }
    let mut value = kept as f64;
    let mut s = scale;
    while s > 0 {
        let step = s.min(1000);
        value *= 2f64.powi(step as i32);
        s = s.saturating_sub(step);
    }
    while s < 0 {
        let step = s.max(-1000);
        value *= 2f64.powi(step as i32);
        s = s.saturating_sub(step);
    }
    value
}

/// Parses the longest `strtod` number at the start of `bytes`: returns its value and end index.
fn strtod_prefix(bytes: &[u8]) -> Option<(f64, usize)> {
    let mut i = 0usize;
    while bytes.get(i) == Some(&0x0b) {
        i = i.saturating_add(1);
    }
    let mut negative = false;
    match bytes.get(i) {
        Some(b'+') => i = i.saturating_add(1),
        Some(b'-') => {
            negative = true;
            i = i.saturating_add(1);
        }
        _ => {}
    }
    let sign = if negative { -1.0 } else { 1.0 };
    if starts_with_ignore_case(bytes, i, b"infinity") {
        return Some((sign * f64::INFINITY, i.saturating_add(8)));
    }
    if starts_with_ignore_case(bytes, i, b"inf") {
        return Some((sign * f64::INFINITY, i.saturating_add(3)));
    }
    if starts_with_ignore_case(bytes, i, b"nan") {
        let mut end = i.saturating_add(3);
        if bytes.get(end) == Some(&b'(') {
            let mut j = end.saturating_add(1);
            while bytes
                .get(j)
                .is_some_and(|b| b.is_ascii_alphanumeric() || *b == b'_')
            {
                j = j.saturating_add(1);
            }
            if bytes.get(j) == Some(&b')') {
                end = j.saturating_add(1);
            }
        }
        return Some((f64::NAN, end));
    }
    let is_hex = bytes.get(i) == Some(&b'0')
        && matches!(bytes.get(i.saturating_add(1)), Some(b'x') | Some(b'X'));
    if is_hex {
        let int_start = i.saturating_add(2);
        let int_end = digits_end(bytes, int_start, true);
        let (frac_start, frac_end) = if bytes.get(int_end) == Some(&b'.') {
            let fs = int_end.saturating_add(1);
            (fs, digits_end(bytes, fs, true))
        } else {
            (int_end, int_end)
        };
        if int_end > int_start || frac_end > frac_start {
            let int_digits = bytes.get(int_start..int_end).unwrap_or(&[]);
            let frac_digits = bytes.get(frac_start..frac_end).unwrap_or(&[]);
            let mut end = frac_end;
            let mut exp: i64 = 0;
            if matches!(bytes.get(end), Some(b'p') | Some(b'P')) {
                let mut j = end.saturating_add(1);
                let mut exp_negative = false;
                match bytes.get(j) {
                    Some(b'+') => j = j.saturating_add(1),
                    Some(b'-') => {
                        exp_negative = true;
                        j = j.saturating_add(1);
                    }
                    _ => {}
                }
                let exp_end = digits_end(bytes, j, false);
                if exp_end > j {
                    let mut e: i64 = 0;
                    for &d in bytes.get(j..exp_end).unwrap_or(&[]) {
                        e = e
                            .saturating_mul(10)
                            .saturating_add(i64::from(char::from(d).to_digit(10)?))
                            .min(1_000_000);
                    }
                    exp = if exp_negative { e.saturating_neg() } else { e };
                    end = exp_end;
                }
            }
            return Some((sign * hex_value(int_digits, frac_digits, exp), end));
        }
        // "0x" without hex digits: the number is the "0".
        return Some((sign * 0.0, i.saturating_add(1)));
    }
    let start = i;
    let int_end = digits_end(bytes, start, false);
    let mut end = int_end;
    let mut any_digit = int_end > start;
    if bytes.get(end) == Some(&b'.') {
        let frac_start = end.saturating_add(1);
        let frac_end = digits_end(bytes, frac_start, false);
        if frac_end > frac_start {
            any_digit = true;
        }
        if any_digit {
            end = frac_end;
        }
    }
    if !any_digit {
        return None;
    }
    if matches!(bytes.get(end), Some(b'e') | Some(b'E')) {
        let mut j = end.saturating_add(1);
        if matches!(bytes.get(j), Some(b'+') | Some(b'-')) {
            j = j.saturating_add(1);
        }
        let exp_end = digits_end(bytes, j, false);
        if exp_end > j {
            end = exp_end;
        }
    }
    let text = std::str::from_utf8(bytes.get(start..end)?).ok()?;
    let value: f64 = text.parse().ok()?;
    Some((sign * value, end))
}

/// Reads a size word: `(size in Pango units, absolute)`, or `None` if `word` is no size.
fn parse_size(word: &str) -> Option<(i32, bool)> {
    let bytes = word.as_bytes();
    let (value, end) = strtod_prefix(bytes)?;
    let rest = bytes.get(end..)?;
    let absolute = match rest {
        [] => false,
        b"px" => true,
        _ => return None,
    };
    if !(0.0..=1_000_000.0).contains(&value) {
        return None;
    }
    let scaled = (value * 1024.0 + 0.5).trunc();
    Some((scaled as i32, absolute))
}

/// Parses `s` as `pango_font_description_from_string` does; every string gives a description.
pub fn pango_font_description(s: &str) -> PangoDescription {
    let bytes = s.as_bytes();
    let mut desc = PangoDescription::default();
    let mut last = bytes.len();

    let text = |start: usize, end: usize| s.get(start..end).unwrap_or("");

    let (start, end) = getword(bytes, last, false);
    if start < end {
        let word = text(start, end);
        if let Some(rest) = word.strip_prefix('#') {
            desc.features = Some(rest.to_owned());
            last = start;
        } else if let Some(rest) = word.strip_prefix('@') {
            desc.variations = Some(rest.to_owned());
            last = start;
        }
    }

    let (start, end) = getword(bytes, last, true);
    if start < end {
        let word = text(start, end);
        if desc.features.is_none() && word.starts_with('#') {
            desc.features = Some(word.get(1..).unwrap_or("").to_owned());
            last = start;
        } else if desc.variations.is_none() && word.starts_with('@') {
            desc.variations = Some(word.get(1..).unwrap_or("").to_owned());
            last = start;
        }
    }

    let (start, end) = getword(bytes, last, true);
    if start < end {
        if let Some((size, absolute)) = parse_size(text(start, end)) {
            desc.size = Some(size);
            desc.absolute = absolute;
            last = start;
        }
    }

    loop {
        let (start, end) = getword(bytes, last, true);
        if start >= end {
            break;
        }
        if style_word(&mut desc, text(start, end)) {
            last = start;
        } else {
            break;
        }
    }

    while byte_before(bytes, last).is_some_and(is_space) {
        last = last.saturating_sub(1);
    }
    if byte_before(bytes, last) == Some(b',') {
        last = last.saturating_sub(1);
    }
    while byte_before(bytes, last).is_some_and(is_space) {
        last = last.saturating_sub(1);
    }
    let mut first = 0usize;
    while first < last && bytes.get(first).copied().is_some_and(is_space) {
        first = first.saturating_add(1);
    }
    if first < last {
        let family = text(first, last)
            .split(',')
            .map(|piece| piece.trim_matches(|c: char| c.is_ascii() && is_space(c as u8)))
            .collect::<Vec<_>>()
            .join(",");
        desc.family = Some(family);
    }
    desc
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

    fn opt_string(cell: &str) -> Result<Option<String>, String> {
        if cell == "none" {
            Ok(None)
        } else {
            json_string(cell).map(Some)
        }
    }

    fn int(cell: &str) -> Result<i32, String> {
        cell.parse::<i32>().map_err(|e| format!("{cell}: {e}"))
    }

    fn opt_int(cell: &str) -> Result<Option<i32>, String> {
        if cell == "none" {
            Ok(None)
        } else {
            int(cell).map(Some)
        }
    }

    fn boolean(cell: &str) -> Result<bool, String> {
        match cell {
            "true" => Ok(true),
            "false" => Ok(false),
            _ => Err(format!("not a bool: {cell}")),
        }
    }

    fn expected(cells: &[&str]) -> Result<(String, PangoDescription), String> {
        let [
            s,
            family,
            style,
            variant,
            weight,
            width,
            gravity,
            color,
            size,
            absolute,
            variations,
            features,
        ] = cells
        else {
            return Err(format!("{} cells", cells.len()));
        };
        Ok((
            json_string(s)?,
            PangoDescription {
                family: opt_string(family)?,
                style: int(style)?,
                variant: int(variant)?,
                weight: int(weight)?,
                width: int(width)?,
                gravity: opt_int(gravity)?,
                color: opt_int(color)?,
                size: opt_int(size)?,
                absolute: boolean(absolute)?,
                variations: opt_string(variations)?,
                features: opt_string(features)?,
            },
        ))
    }

    #[test]
    fn vectors_reader_pango_font_description() {
        let table = include_str!("../../tests/vectors/reader.pango_font_description.tsv");
        let mut lines = table.lines().filter(|l| !l.starts_with('#'));
        assert_eq!(
            lines.next(),
            Some(
                "s\tfamily\tstyle\tvariant\tweight\twidth\tgravity\tcolor\tsize\tabsolute\tvariations\tfeatures"
            )
        );
        let mut rows = 0usize;
        let mut failures = Vec::new();
        for line in lines {
            rows += 1;
            let cells: Vec<&str> = line.split('\t').collect();
            match expected(&cells) {
                Ok((input, want)) => {
                    let got = pango_font_description(&input);
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
        assert_eq!(rows, 263);
        println!("pango_font_description: {rows} of {rows} rows pass");
    }

    #[test]
    fn hex_mantissa_past_53_bits_rounds_to_nearest_even() {
        // 2^53 + 1 and 2^55 + 4 are ties kept even; 2^53 + 3 and 2^55 + 12 ties rounded up;
        // 2^55 + 1 rounds down.
        assert_eq!(
            strtod_prefix(b"0x20000000000001"),
            Some((9007199254740992.0, 16))
        );
        assert_eq!(
            strtod_prefix(b"0x20000000000003"),
            Some((9007199254740996.0, 16))
        );
        assert_eq!(
            strtod_prefix(b"0x80000000000001"),
            Some((36028797018963968.0, 16))
        );
        assert_eq!(
            strtod_prefix(b"0x80000000000004"),
            Some((36028797018963968.0, 16))
        );
        assert_eq!(
            strtod_prefix(b"0x8000000000000C"),
            Some((36028797018963984.0, 16))
        );
    }

    #[test]
    fn hex_exponent_past_any_double_saturates() {
        assert_eq!(
            strtod_prefix(b"0x1p99999999999999999999"),
            Some((f64::INFINITY, 24))
        );
        assert_eq!(strtod_prefix(b"0x1p-99999999999999999999"), Some((0.0, 25)));
        let tiny = pango_font_description("Sans 0x1p-99999999999999999999");
        assert_eq!(tiny.family.as_deref(), Some("Sans"));
        assert_eq!(tiny.size, Some(0));
        let huge = pango_font_description("Sans 0x1p99999999999999999999");
        assert_eq!(
            huge.family.as_deref(),
            Some("Sans 0x1p99999999999999999999")
        );
        assert_eq!(huge.size, None);
    }
}
