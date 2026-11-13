//! libadwaita's snapping of the portal's accent colour to one of its nine accents.
//! Its arithmetic was made panic-free after the clean room (the identity `fmod` dropped), without reading LGPL source.

/// One of libadwaita's nine accent colours.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdwAccent {
    /// `#3584e4`.
    Blue,
    /// `#2190a4`.
    Teal,
    /// `#3a944a`.
    Green,
    /// `#c88800`.
    Yellow,
    /// `#ed5b00`.
    Orange,
    /// `#e62d42`.
    Red,
    /// `#d56199`.
    Pink,
    /// `#9141ac`.
    Purple,
    /// `#6f8396`.
    Slate,
}

fn unapply_gamma(v: f32) -> f32 {
    let v = f64::from(v);
    if v >= 0.04045 {
        let d = 1.0_f64 + 0.055;
        ((v + 0.055) / d).powf(2.4) as f32
    } else {
        (v / 12.92) as f32
    }
}

/// Snaps the sRGB colour (each channel a single in `[0, 1]`) to an accent by its OKLCH hue band,
/// as `adw_accent_color_nearest_from_rgba` does.
pub fn adw_accent_nearest(red: f32, green: f32, blue: f32) -> AdwAccent {
    let r = unapply_gamma(red);
    let g = unapply_gamma(green);
    let b = unapply_gamma(blue);

    let l = (0.4122214708_f32 * r + 0.5363325363_f32 * g) + 0.0514459929_f32 * b;
    let m = (0.2119034982_f32 * r + 0.6806995451_f32 * g) + 0.1073969566_f32 * b;
    let s = (0.0883024619_f32 * r + 0.2817188376_f32 * g) + 0.6299787005_f32 * b;
    let l = l.cbrt();
    let m = m.cbrt();
    let s = s.cbrt();

    let lab_a = (1.9779984951_f32 * l - 2.4285922050_f32 * m) + 0.4505937099_f32 * s;
    let lab_b = (0.0259040371_f32 * l + 0.7827717662_f32 * m) - 0.8086757660_f32 * s;

    let c = lab_a.hypot(lab_b);
    let t = f64::from(lab_b).atan2(f64::from(lab_a));
    // `atan2` keeps `h` within ±180, where libadwaita's `fmod(h, 360)` changes nothing.
    let mut h = ((t * 180.0) / std::f64::consts::PI) as f32;
    if h < 0.0 {
        h += 360.0_f32;
    }

    if f64::from(c) < 0.04 {
        AdwAccent::Slate
    } else if h > 345.0 {
        AdwAccent::Pink
    } else if h > 280.0 {
        AdwAccent::Purple
    } else if h > 230.0 {
        AdwAccent::Blue
    } else if h > 175.0 {
        AdwAccent::Teal
    } else if h > 115.0 {
        AdwAccent::Green
    } else if h > 75.5 {
        AdwAccent::Yellow
    } else if h > 35.0 {
        AdwAccent::Orange
    } else if h > 10.0 {
        AdwAccent::Red
    } else {
        AdwAccent::Pink
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn accent(cell: &str) -> Result<AdwAccent, String> {
        Ok(match cell {
            "blue" => AdwAccent::Blue,
            "teal" => AdwAccent::Teal,
            "green" => AdwAccent::Green,
            "yellow" => AdwAccent::Yellow,
            "orange" => AdwAccent::Orange,
            "red" => AdwAccent::Red,
            "pink" => AdwAccent::Pink,
            "purple" => AdwAccent::Purple,
            "slate" => AdwAccent::Slate,
            _ => return Err(format!("not an accent: {cell}")),
        })
    }

    fn channel(cell: &str) -> Result<f32, String> {
        let v = cell
            .parse::<f32>()
            .map_err(|_| format!("not a number: {cell}"))?;
        if v.is_finite() && (0.0..=1.0).contains(&v) {
            Ok(v)
        } else {
            Err(format!("outside the domain: {cell}"))
        }
    }

    fn expected(cells: &[&str]) -> Result<((f32, f32, f32), AdwAccent), String> {
        let [r, g, b, a] = cells else {
            return Err(format!("{} cells", cells.len()));
        };
        Ok(((channel(r)?, channel(g)?, channel(b)?), accent(a)?))
    }

    #[test]
    fn vectors_reader_adw_accent_nearest() {
        let table = include_str!("../../tests/vectors/reader.adw_accent_nearest.tsv");
        let mut lines = table.lines().filter(|l| !l.starts_with('#'));
        assert_eq!(lines.next(), Some("red\tgreen\tblue\taccent"));
        let mut rows = 0usize;
        let mut failures = Vec::new();
        for line in lines {
            rows += 1;
            let cells: Vec<&str> = line.split('\t').collect();
            match expected(&cells) {
                Ok(((r, g, b), want)) => {
                    let got = adw_accent_nearest(r, g, b);
                    if got != want {
                        failures.push(format!("{line}\n  got {got:?}, want {want:?}"));
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
        assert_eq!(rows, 134);
        println!("adw_accent_nearest: {rows} of {rows} rows pass");
    }
}
