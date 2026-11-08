//! Plasma's dark-or-light test of the palette's window colour, from the `BackgroundNormal` entry
//! of kdeglobals' `[Colors:Window]` group as KConfig holds it.

/// The colour scheme Plasma reports for an application's palette.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scheme {
    /// The window colour's grey is 192 or more.
    Light,
    /// The window colour's grey is below 192.
    Dark,
}

/// The Window set's default `BackgroundNormal`, Breeze Light's window colour.
const DEFAULT_WINDOW: [u8; 3] = [239, 240, 241];

/// Below this grey the scheme is dark.
const DARK_BELOW: u32 = 192;

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

/// The window colour's red, green and blue as KConfig reads the entry; the default where the
/// entry is absent or KConfig refuses it. The forms KConfig reads as an invalid colour or hands to
/// Qt's colour parser (no comma, or a leading `#`) are read as unset, giving the default.
fn window_colour(background_normal: Option<&str>) -> [u8; 3] {
    let Some(value) = background_normal else {
        return DEFAULT_WINDOW;
    };
    let value = value.as_bytes();
    if value.first() == Some(&b'#') || !value.contains(&b',') {
        return DEFAULT_WINDOW;
    }
    let pieces: Vec<&[u8]> = value.split(|b| *b == b',').collect();
    if !matches!(pieces.len(), 3 | 4) {
        return DEFAULT_WINDOW;
    }
    let mut channels = [0u8; 4];
    for (slot, piece) in channels.iter_mut().zip(&pieces) {
        match int(piece).and_then(|x| u8::try_from(x).ok()) {
            Some(x) => *slot = x,
            None => return DEFAULT_WINDOW,
        }
    }
    let [r, g, b, _alpha] = channels;
    [r, g, b]
}

/// Qt's `qGray`: `(11·r + 16·g + 5·b) / 32`, truncating.
fn q_gray([r, g, b]: [u8; 3]) -> u32 {
    let sum = 11u32
        .saturating_mul(u32::from(r))
        .saturating_add(16u32.saturating_mul(u32::from(g)))
        .saturating_add(5u32.saturating_mul(u32::from(b)));
    sum / 32
}

/// The scheme Plasma reports for a palette built from a scheme file whose `[Colors:Window]`
/// `BackgroundNormal` is `background_normal` (`None` where the file has no such key).
pub fn kde_color_scheme(background_normal: Option<&str>) -> Scheme {
    if q_gray(window_colour(background_normal)) < DARK_BELOW {
        Scheme::Dark
    } else {
        Scheme::Light
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

    fn scheme(cell: &str) -> Result<Scheme, String> {
        match cell {
            "light" => Ok(Scheme::Light),
            "dark" => Ok(Scheme::Dark),
            _ => Err(format!("not a scheme: {cell}")),
        }
    }

    fn expected(cells: &[&str]) -> Result<(Option<String>, Scheme), String> {
        let [background_normal, want] = cells else {
            return Err(format!("{} cells", cells.len()));
        };
        let input = if *background_normal == "none" {
            None
        } else {
            Some(json_string(background_normal)?)
        };
        Ok((input, scheme(want)?))
    }

    #[test]
    fn vectors_reader_kde_color_scheme() {
        let table = include_str!("../../tests/vectors/reader.kde_color_scheme.tsv");
        let mut lines = table.lines().filter(|l| !l.starts_with('#'));
        assert_eq!(lines.next(), Some("background_normal\tscheme"));
        let mut rows = 0usize;
        let mut failures = Vec::new();
        for line in lines {
            rows += 1;
            let cells: Vec<&str> = line.split('\t').collect();
            match expected(&cells) {
                Ok((input, want)) => {
                    let got = kde_color_scheme(input.as_deref());
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
        assert_eq!(rows, 110);
        println!("kde_color_scheme: {rows} of {rows} rows pass");
    }

    #[test]
    fn integers_at_the_64_and_32_bit_edges() {
        assert_eq!(int(b"-2147483648"), Some(i32::MIN));
        assert_eq!(int(b"2147483648"), None);
        assert_eq!(int(b"-9223372036854775808"), None);
        assert_eq!(int(b"99999999999999999999"), None);
        assert_eq!(int(b""), None);
        assert_eq!(int(b" "), None);
        assert_eq!(int(b"+-1"), None);
        assert_eq!(int(b"41 \0junk"), Some(41));
    }

    #[test]
    fn gray_truncates_at_the_threshold() {
        assert_eq!(q_gray([200, 183, 200]), 191);
        assert_eq!(q_gray([255, 133, 242]), 191);
        assert_eq!(q_gray([255, 255, 255]), 255);
        assert_eq!(q_gray(DEFAULT_WINDOW), 239);
    }
}
