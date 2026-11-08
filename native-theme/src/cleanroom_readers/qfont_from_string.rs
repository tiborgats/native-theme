//! Qt's `QFont::fromString`, as KDE's reader reads kdeglobals font entries.
//! Its arithmetic was made panic-free after the clean room, without reading LGPL source.

use std::collections::BTreeMap;

/// The fields `QFont::fromString` sets: each `Some` where the call set it, `None` where it left it.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct QtFontFields {
    /// The family, the first field as it is.
    pub family: Option<String>,
    /// The point size; `None` also where the size is a pixel size.
    pub point_size: Option<f64>,
    /// The pixel size; `None` also where the size is a point size.
    pub pixel_size: Option<i32>,
    /// `QFont::StyleHint`, stored in 8 bits.
    pub style_hint: Option<u8>,
    /// The OpenType weight, 1..=1000.
    pub weight: Option<i32>,
    /// `QFont::Style`, stored in 2 bits.
    pub style: Option<u8>,
    /// Whether the font is underlined.
    pub underline: Option<bool>,
    /// Whether the font is struck out.
    pub strike_out: Option<bool>,
    /// Whether the font is fixed pitch.
    pub fixed_pitch: Option<bool>,
    /// `QFont::Capitalization`, stored in 3 bits.
    pub capitalization: Option<u8>,
    /// `QFont::SpacingType`: 1 absolute, 0 percentage.
    pub letter_spacing_type: Option<u8>,
    /// The letter spacing, in Qt's 26.6 fixed point.
    pub letter_spacing: Option<f64>,
    /// The word spacing, in Qt's 26.6 fixed point.
    pub word_spacing: Option<f64>,
    /// The stretch in percent, 0..=4000.
    pub stretch: Option<i32>,
    /// `QFont::StyleStrategy`, stored in 16 bits.
    pub style_strategy: Option<u16>,
    /// The style name.
    pub style_name: Option<String>,
    /// The OpenType features, by tag (the tag's four bytes).
    pub features: Option<BTreeMap<[u8; 4], u32>>,
    /// The variable axes, by tag (the tag's four bytes).
    pub axes: Option<BTreeMap<[u8; 4], f32>>,
}

/// Qt's `QChar::isSpace`.
fn is_qt_space(c: char) -> bool {
    c.is_whitespace()
}

/// The ASCII white space `toInt` skips, vertical tab included.
fn is_ascii_space(c: char) -> bool {
    matches!(c, ' ' | '\t' | '\n' | '\u{0b}' | '\u{0c}' | '\r')
}

/// A `toDouble` result: its value and whether it parsed without overflow.
struct Real {
    value: f64,
    ok: bool,
}

fn is_decimal(text: &str) -> bool {
    let bytes = text.as_bytes();
    let mut i = 0usize;
    if matches!(bytes.first(), Some(b'+') | Some(b'-')) {
        i = 1;
    }
    let mut digits = 0usize;
    while bytes.get(i).is_some_and(u8::is_ascii_digit) {
        i = i.saturating_add(1);
        digits = digits.saturating_add(1);
    }
    if bytes.get(i) == Some(&b'.') {
        i = i.saturating_add(1);
        while bytes.get(i).is_some_and(u8::is_ascii_digit) {
            i = i.saturating_add(1);
            digits = digits.saturating_add(1);
        }
    }
    if digits == 0 {
        return false;
    }
    if matches!(bytes.get(i), Some(b'e') | Some(b'E')) {
        i = i.saturating_add(1);
        if matches!(bytes.get(i), Some(b'+') | Some(b'-')) {
            i = i.saturating_add(1);
        }
        let start = i;
        while bytes.get(i).is_some_and(u8::is_ascii_digit) {
            i = i.saturating_add(1);
        }
        if i == start {
            return false;
        }
    }
    i == bytes.len()
}

fn real(field: &str) -> Real {
    let failed = Real {
        value: 0.0,
        ok: false,
    };
    let text = field.trim_matches(is_qt_space);
    if !text.is_ascii() {
        return failed;
    }
    let (sign, unsigned) = if let Some(rest) = text.strip_prefix('-') {
        (-1.0, rest)
    } else if let Some(rest) = text.strip_prefix('+') {
        (1.0, rest)
    } else {
        (1.0, text)
    };
    if unsigned.eq_ignore_ascii_case("inf") {
        return Real {
            value: sign * f64::INFINITY,
            ok: true,
        };
    }
    if text.eq_ignore_ascii_case("nan") {
        return Real {
            value: f64::NAN,
            ok: true,
        };
    }
    if !is_decimal(text) {
        return failed;
    }
    match text.parse::<f64>() {
        Ok(value) => Real {
            value,
            ok: value.is_finite(),
        },
        Err(_) => failed,
    }
}

/// Reads optional sign and digits surrounded by ASCII white space: `(negative, magnitude)`.
fn integer_parts(field: &str) -> Option<(bool, u64)> {
    let text = field.trim_start_matches(is_ascii_space);
    let (negative, rest) = if let Some(rest) = text.strip_prefix('-') {
        (true, rest)
    } else if let Some(rest) = text.strip_prefix('+') {
        (false, rest)
    } else {
        (false, text)
    };
    let digits_len = rest.bytes().take_while(u8::is_ascii_digit).count();
    if digits_len == 0 {
        return None;
    }
    let digits = rest.get(..digits_len)?;
    let tail = rest.get(digits_len..)?;
    if !tail.chars().all(is_ascii_space) {
        return None;
    }
    let mut magnitude: u64 = 0;
    for b in digits.bytes() {
        magnitude = magnitude
            .checked_mul(10)?
            .checked_add(u64::from(b.checked_sub(b'0')?))?;
    }
    Some((negative, magnitude))
}

fn checked_int(field: &str) -> Option<i32> {
    let (negative, magnitude) = integer_parts(field)?;
    let magnitude = i64::try_from(magnitude).ok()?;
    let value = if negative {
        magnitude.checked_neg()?
    } else {
        magnitude
    };
    i32::try_from(value).ok()
}

/// `toInt`: the value, or 0 where the field is no 32-bit integer.
fn int(field: &str) -> i32 {
    checked_int(field).unwrap_or_default()
}

fn uint(field: &str) -> Option<u32> {
    let (negative, magnitude) = integer_parts(field)?;
    if negative {
        return None;
    }
    u32::try_from(magnitude).ok()
}

fn float(field: &str) -> Option<f32> {
    let r = real(field);
    if !r.ok {
        return None;
    }
    let single = r.value as f32;
    if r.value.is_finite() && single.is_infinite() {
        return None;
    }
    if r.value != 0.0 && single == 0.0 {
        return None;
    }
    Some(single)
}

/// Qt 5's weight to OpenType's (`convertWeights`).
fn legacy(w: i32) -> i32 {
    const PAIRS: [(i64, i32); 9] = [
        (0, 100),
        (12, 200),
        (25, 300),
        (50, 400),
        (57, 500),
        (63, 600),
        (75, 700),
        (81, 800),
        (87, 900),
    ];
    let w = i64::from(w);
    let [(first_old, first_new), rest @ ..] = PAIRS;
    let mut best_distance = first_old.abs_diff(w);
    let mut result = first_new;
    for (old, new) in rest {
        let d = old.abs_diff(w);
        if d >= best_distance {
            break;
        }
        best_distance = d;
        result = new;
    }
    result
}

fn fixed_26_6(value: f64) -> f64 {
    let units = (value * 64.0).trunc();
    let units = if units == 0.0 { 0.0 } else { units };
    units / 64.0
}

/// Splits an entry at its first `=`: the tag's four bytes and the value text.
fn entry(text: &str) -> Option<([u8; 4], &str)> {
    let (tag, value) = text.split_once('=')?;
    let mut bytes = [0u8; 4];
    let mut count = 0usize;
    for c in tag.encode_utf16() {
        let slot = bytes.get_mut(count)?;
        *slot = u8::try_from(c).unwrap_or_default();
        count = count.saturating_add(1);
    }
    if count != 4 || bytes == [0; 4] {
        return None;
    }
    Some((bytes, value))
}

/// Parses `s` as `QFont::fromString` does on a fresh font; `None` where Qt refuses the string.
pub fn qfont_from_string(s: &str) -> Option<QtFontFields> {
    let l: Vec<&str> = s.trim_matches(is_qt_space).split(',').collect();
    let count = l.len();
    let first = *l.first()?;
    if (3..=9).contains(&count) || first.is_empty() {
        return None;
    }
    let field = |i: usize| l.get(i).copied().unwrap_or("");
    let mut font = QtFontFields {
        family: Some(first.to_owned()),
        ..QtFontFields::default()
    };

    if count > 1 {
        let size = real(field(1)).value;
        if size > 0.0 {
            font.point_size = Some(size);
            font.pixel_size = None;
        }
    }
    if count < 10 {
        return Some(font);
    }

    let pixel = int(field(2));
    if pixel > 0 {
        font.pixel_size = Some(pixel);
        font.point_size = None;
    }
    font.style_hint = Some(int(field(3)) as u8);
    font.style_strategy = Some(1);
    let raw_weight = int(field(4));
    let weight = if count >= 16 {
        raw_weight
    } else {
        legacy(raw_weight)
    };
    font.weight = Some(weight.clamp(1, 1000));
    font.style = Some((int(field(5)) & 3) as u8);
    font.underline = Some(int(field(6)) != 0);
    font.strike_out = Some(int(field(7)) != 0);
    font.fixed_pitch = Some(int(field(8)) != 0);

    if count >= 16 {
        font.capitalization = Some((int(field(10)) & 7) as u8);
        font.letter_spacing_type = Some(u8::from(int(field(11)) == 1));
        font.letter_spacing = Some(fixed_26_6(real(field(12)).value));
        font.word_spacing = Some(fixed_26_6(real(field(13)).value));
        let stretch = int(field(14));
        if (0..=4000).contains(&stretch) {
            font.stretch = Some(stretch);
        }
        font.style_strategy = Some(int(field(15)) as u16);
    }

    font.style_name = Some(if count == 11 {
        field(10).to_owned()
    } else if count >= 17 {
        field(16).to_owned()
    } else {
        String::new()
    });

    let (features, axes) = entries(&l);
    font.features = Some(features);
    font.axes = Some(axes);
    Some(font)
}

type Features = BTreeMap<[u8; 4], u32>;
type Axes = BTreeMap<[u8; 4], f32>;

/// Reads the feature and axis lists that follow the 17th field.
fn entries(l: &[&str]) -> (Features, Axes) {
    let mut features = Features::new();
    let mut axes = Axes::new();
    let count = l.len();
    let mut pos = 17usize;

    let Some(n) = l.get(pos).map(|f| int(f)) else {
        return (features, axes);
    };
    pos = pos.saturating_add(1);
    let n = usize::try_from(n).unwrap_or_default();
    let Some(end) = pos.checked_add(n).filter(|end| *end <= count) else {
        return (features, axes);
    };
    for text in l.get(pos..end).unwrap_or_default() {
        if let Some((tag, value)) = entry(text) {
            if let Some(value) = uint(value) {
                features.insert(tag, value);
            }
        }
    }
    pos = end;

    let Some(m) = l.get(pos).map(|f| int(f)) else {
        return (features, axes);
    };
    pos = pos.saturating_add(1);
    let m = usize::try_from(m).unwrap_or_default();
    let Some(end) = pos.checked_add(m).filter(|end| *end <= count) else {
        return (features, axes);
    };
    for text in l.get(pos..end).unwrap_or_default() {
        if let Some((tag, value)) = entry(text) {
            if let Some(value) = float(value) {
                axes.insert(tag, value);
            }
        }
    }
    (features, axes)
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

    fn opt<T>(cell: &str, read: impl Fn(&str) -> Result<T, String>) -> Result<Option<T>, String> {
        if cell == "none" {
            Ok(None)
        } else {
            read(cell).map(Some)
        }
    }

    fn number<T: std::str::FromStr>(cell: &str) -> Result<T, String> {
        cell.parse::<T>()
            .map_err(|_| format!("not a number: {cell}"))
    }

    fn boolean(cell: &str) -> Result<bool, String> {
        match cell {
            "true" => Ok(true),
            "false" => Ok(false),
            _ => Err(format!("not a bool: {cell}")),
        }
    }

    fn list<T>(
        cell: &str,
        read: impl Fn(&str) -> Result<T, String>,
    ) -> Result<BTreeMap<[u8; 4], T>, String> {
        let text = json_string(cell)?;
        let mut map = BTreeMap::new();
        if text.is_empty() {
            return Ok(map);
        }
        for item in text.split(',') {
            let (tag, value) = entry(item).ok_or_else(|| format!("bad entry {item}"))?;
            map.insert(tag, read(value)?);
        }
        Ok(map)
    }

    /// Floats compared by bits (NaN by kind), so `-0.0` and `0.0` differ.
    fn same_f64(a: Option<f64>, b: Option<f64>) -> bool {
        match (a, b) {
            (Some(x), Some(y)) => x.to_bits() == y.to_bits() || (x.is_nan() && y.is_nan()),
            (None, None) => true,
            _ => false,
        }
    }

    fn same(got: &Option<QtFontFields>, want: &Option<QtFontFields>) -> bool {
        match (got, want) {
            (None, None) => true,
            (Some(g), Some(w)) => {
                let axes_same = match (&g.axes, &w.axes) {
                    (Some(a), Some(b)) => {
                        a.len() == b.len()
                            && a.iter().zip(b.iter()).all(|((ta, va), (tb, vb))| {
                                ta == tb && va.to_bits() == vb.to_bits()
                            })
                    }
                    (None, None) => true,
                    _ => false,
                };
                g.family == w.family
                    && same_f64(g.point_size, w.point_size)
                    && g.pixel_size == w.pixel_size
                    && g.style_hint == w.style_hint
                    && g.weight == w.weight
                    && g.style == w.style
                    && g.underline == w.underline
                    && g.strike_out == w.strike_out
                    && g.fixed_pitch == w.fixed_pitch
                    && g.capitalization == w.capitalization
                    && g.letter_spacing_type == w.letter_spacing_type
                    && same_f64(g.letter_spacing, w.letter_spacing)
                    && same_f64(g.word_spacing, w.word_spacing)
                    && g.stretch == w.stretch
                    && g.style_strategy == w.style_strategy
                    && g.style_name == w.style_name
                    && g.features == w.features
                    && axes_same
            }
            _ => false,
        }
    }

    fn expected(cells: &[&str]) -> Result<(String, Option<QtFontFields>), String> {
        let [
            s,
            ok,
            family,
            point_size,
            pixel_size,
            style_hint,
            weight,
            style,
            underline,
            strike_out,
            fixed_pitch,
            capitalization,
            letter_spacing_type,
            letter_spacing,
            word_spacing,
            stretch,
            style_strategy,
            style_name,
            features,
            axes,
        ] = cells
        else {
            return Err(format!("{} cells", cells.len()));
        };
        let input = json_string(s)?;
        if !boolean(ok)? {
            if cells.iter().skip(2).any(|c| *c != "none") {
                return Err("ok false with a set field".to_owned());
            }
            return Ok((input, None));
        }
        let fields = QtFontFields {
            family: opt(family, json_string)?,
            point_size: opt(point_size, number::<f64>)?,
            pixel_size: opt(pixel_size, number::<i32>)?,
            style_hint: opt(style_hint, number::<u8>)?,
            weight: opt(weight, number::<i32>)?,
            style: opt(style, number::<u8>)?,
            underline: opt(underline, boolean)?,
            strike_out: opt(strike_out, boolean)?,
            fixed_pitch: opt(fixed_pitch, boolean)?,
            capitalization: opt(capitalization, number::<u8>)?,
            letter_spacing_type: opt(letter_spacing_type, number::<u8>)?,
            letter_spacing: opt(letter_spacing, number::<f64>)?,
            word_spacing: opt(word_spacing, number::<f64>)?,
            stretch: opt(stretch, number::<i32>)?,
            style_strategy: opt(style_strategy, number::<u16>)?,
            style_name: opt(style_name, json_string)?,
            features: opt(features, |c| list(c, number::<u32>))?,
            axes: opt(axes, |c| list(c, number::<f32>))?,
        };
        Ok((input, Some(fields)))
    }

    #[test]
    fn vectors_reader_qfont_from_string() {
        let table = include_str!("../../tests/vectors/reader.qfont_from_string.tsv");
        let mut lines = table.lines().filter(|l| !l.starts_with('#'));
        assert_eq!(
            lines.next(),
            Some(
                "s\tok\tfamily\tpoint_size\tpixel_size\tstyle_hint\tweight\tstyle\tunderline\tstrike_out\t\
                 fixed_pitch\tcapitalization\tletter_spacing_type\tletter_spacing\tword_spacing\tstretch\t\
                 style_strategy\tstyle_name\tfeatures\taxes"
            )
        );
        let mut rows = 0usize;
        let mut failures = Vec::new();
        for line in lines {
            rows += 1;
            let cells: Vec<&str> = line.split('\t').collect();
            match expected(&cells) {
                Ok((input, want)) => {
                    let got = qfont_from_string(&input);
                    if !same(&got, &want) {
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
        assert_eq!(rows, 284);
        println!("qfont_from_string: {rows} of {rows} rows pass");
    }

    #[test]
    fn integers_at_the_64_and_32_bit_edges() {
        assert_eq!(checked_int("-2147483648"), Some(i32::MIN));
        assert_eq!(checked_int("-2147483649"), None);
        assert_eq!(int("-9223372036854775808"), 0);
        assert_eq!(int("-18446744073709551615"), 0);
        assert_eq!(int("18446744073709551616"), 0);
    }

    #[test]
    fn legacy_weights_at_the_32_bit_edges() {
        assert_eq!(legacy(i32::MIN), 100);
        assert_eq!(legacy(i32::MAX), 900);
    }
}
