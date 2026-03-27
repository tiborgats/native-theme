// Rgba color type with custom hex serde

use serde::de;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::str::FromStr;

/// An sRGB color with alpha, stored as four u8 components.
///
/// All values are in the sRGB color space. When parsing hex strings,
/// alpha defaults to 255 (fully opaque) if omitted.
///
/// # Hex Format
///
/// Supports parsing from and displaying as hex strings:
/// - `#RGB` / `RGB` -- 3-digit shorthand (each digit doubled: `#abc` -> `#aabbcc`)
/// - `#RGBA` / `RGBA` -- 4-digit shorthand with alpha
/// - `#RRGGBB` / `RRGGBB` -- standard 6-digit hex
/// - `#RRGGBBAA` / `RRGGBBAA` -- 8-digit hex with alpha
///
/// Display outputs lowercase hex: `#rrggbb` when alpha is 255,
/// `#rrggbbaa` otherwise.
///
/// # Examples
///
/// ```
/// use native_theme::Rgba;
///
/// // Create an opaque color
/// let blue = Rgba::rgb(0, 120, 215);
/// assert_eq!(blue.a, 255);
///
/// // Parse from a hex string
/// let parsed: Rgba = "#3daee9".parse().unwrap();
/// assert_eq!(parsed.r, 61);
///
/// // Convert to f32 array for toolkit interop
/// let arr = Rgba::rgb(255, 0, 0).to_f32_array();
/// assert_eq!(arr, [1.0, 0.0, 0.0, 1.0]);
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Rgba {
    /// Red component (0-255).
    pub r: u8,
    /// Green component (0-255).
    pub g: u8,
    /// Blue component (0-255).
    pub b: u8,
    /// Alpha component (0-255, where 255 is fully opaque).
    pub a: u8,
}

impl Rgba {
    /// Create an opaque color (alpha = 255).
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    /// Create a color with explicit alpha.
    #[allow(clippy::self_named_constructors)]
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Create a color from floating-point components in the 0.0..=1.0 range.
    ///
    /// Values are clamped to 0.0..=1.0 before conversion.
    pub fn from_f32(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {
            r: (r.clamp(0.0, 1.0) * 255.0).round() as u8,
            g: (g.clamp(0.0, 1.0) * 255.0).round() as u8,
            b: (b.clamp(0.0, 1.0) * 255.0).round() as u8,
            a: (a.clamp(0.0, 1.0) * 255.0).round() as u8,
        }
    }

    /// Convert to `[r, g, b, a]` in the 0.0..=1.0 range (for toolkit interop).
    pub fn to_f32_array(&self) -> [f32; 4] {
        [
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
            self.a as f32 / 255.0,
        ]
    }

    /// Convert to `(r, g, b, a)` tuple in the 0.0..=1.0 range.
    pub fn to_f32_tuple(&self) -> (f32, f32, f32, f32) {
        (
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
            self.a as f32 / 255.0,
        )
    }
}

impl fmt::Display for Rgba {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.a == 255 {
            write!(f, "#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
        } else {
            write!(
                f,
                "#{:02x}{:02x}{:02x}{:02x}",
                self.r, self.g, self.b, self.a
            )
        }
    }
}

impl FromStr for Rgba {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hex = s.strip_prefix('#').unwrap_or(s);

        if hex.is_empty() {
            return Err("empty hex color string".to_string());
        }

        match hex.len() {
            // #RGB shorthand: each digit doubled (e.g., 'a' -> 0xaa = a * 17)
            3 => {
                let r = u8::from_str_radix(&hex[0..1], 16)
                    .map_err(|e| format!("invalid red component: {e}"))?;
                let g = u8::from_str_radix(&hex[1..2], 16)
                    .map_err(|e| format!("invalid green component: {e}"))?;
                let b = u8::from_str_radix(&hex[2..3], 16)
                    .map_err(|e| format!("invalid blue component: {e}"))?;
                Ok(Rgba::rgb(r * 17, g * 17, b * 17))
            }
            // #RGBA shorthand
            4 => {
                let r = u8::from_str_radix(&hex[0..1], 16)
                    .map_err(|e| format!("invalid red component: {e}"))?;
                let g = u8::from_str_radix(&hex[1..2], 16)
                    .map_err(|e| format!("invalid green component: {e}"))?;
                let b = u8::from_str_radix(&hex[2..3], 16)
                    .map_err(|e| format!("invalid blue component: {e}"))?;
                let a = u8::from_str_radix(&hex[3..4], 16)
                    .map_err(|e| format!("invalid alpha component: {e}"))?;
                Ok(Rgba::rgba(r * 17, g * 17, b * 17, a * 17))
            }
            // #RRGGBB
            6 => {
                let r = u8::from_str_radix(&hex[0..2], 16)
                    .map_err(|e| format!("invalid red component: {e}"))?;
                let g = u8::from_str_radix(&hex[2..4], 16)
                    .map_err(|e| format!("invalid green component: {e}"))?;
                let b = u8::from_str_radix(&hex[4..6], 16)
                    .map_err(|e| format!("invalid blue component: {e}"))?;
                Ok(Rgba::rgb(r, g, b))
            }
            // #RRGGBBAA
            8 => {
                let r = u8::from_str_radix(&hex[0..2], 16)
                    .map_err(|e| format!("invalid red component: {e}"))?;
                let g = u8::from_str_radix(&hex[2..4], 16)
                    .map_err(|e| format!("invalid green component: {e}"))?;
                let b = u8::from_str_radix(&hex[4..6], 16)
                    .map_err(|e| format!("invalid blue component: {e}"))?;
                let a = u8::from_str_radix(&hex[6..8], 16)
                    .map_err(|e| format!("invalid alpha component: {e}"))?;
                Ok(Rgba::rgba(r, g, b, a))
            }
            other => Err(format!(
                "invalid hex color length {other}: expected 3, 4, 6, or 8 hex digits"
            )),
        }
    }
}

impl Serialize for Rgba {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Rgba {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        Rgba::from_str(&s).map_err(de::Error::custom)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

    // === Constructor tests ===

    #[test]
    fn rgb_constructor_sets_alpha_255() {
        let c = Rgba::rgb(61, 174, 233);
        assert_eq!(
            c,
            Rgba {
                r: 61,
                g: 174,
                b: 233,
                a: 255
            }
        );
    }

    #[test]
    fn rgba_constructor_sets_all_fields() {
        let c = Rgba::rgba(61, 174, 233, 128);
        assert_eq!(
            c,
            Rgba {
                r: 61,
                g: 174,
                b: 233,
                a: 128
            }
        );
    }

    // === FromStr parsing tests ===

    #[test]
    fn parse_6_digit_hex_with_hash() {
        let c: Rgba = "#3daee9".parse().unwrap();
        assert_eq!(c, Rgba::rgb(61, 174, 233));
    }

    #[test]
    fn parse_8_digit_hex_with_hash() {
        let c: Rgba = "#3daee980".parse().unwrap();
        assert_eq!(c, Rgba::rgba(61, 174, 233, 128));
    }

    #[test]
    fn parse_6_digit_hex_without_hash() {
        let c: Rgba = "3daee9".parse().unwrap();
        assert_eq!(c, Rgba::rgb(61, 174, 233));
    }

    #[test]
    fn parse_3_digit_shorthand() {
        let c: Rgba = "#abc".parse().unwrap();
        assert_eq!(c, Rgba::rgb(0xaa, 0xbb, 0xcc));
    }

    #[test]
    fn parse_4_digit_shorthand() {
        let c: Rgba = "#abcd".parse().unwrap();
        assert_eq!(c, Rgba::rgba(0xaa, 0xbb, 0xcc, 0xdd));
    }

    #[test]
    fn parse_uppercase_hex() {
        let c: Rgba = "#AABBCC".parse().unwrap();
        assert_eq!(c, Rgba::rgb(0xaa, 0xbb, 0xcc));
    }

    #[test]
    fn parse_empty_string_is_error() {
        assert!("".parse::<Rgba>().is_err());
    }

    #[test]
    fn parse_invalid_hex_chars_is_error() {
        assert!("#gggggg".parse::<Rgba>().is_err());
    }

    #[test]
    fn parse_invalid_length_5_chars_is_error() {
        assert!("#12345".parse::<Rgba>().is_err());
    }

    // === Display tests ===

    #[test]
    fn display_omits_alpha_when_255() {
        assert_eq!(Rgba::rgb(61, 174, 233).to_string(), "#3daee9");
    }

    #[test]
    fn display_includes_alpha_when_not_255() {
        assert_eq!(Rgba::rgba(61, 174, 233, 128).to_string(), "#3daee980");
    }

    // === Serde round-trip tests ===

    #[test]
    fn serde_json_round_trip() {
        let c = Rgba::rgb(61, 174, 233);
        let json = serde_json::to_string(&c).unwrap();
        assert_eq!(json, "\"#3daee9\"");
        let deserialized: Rgba = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, c);
    }

    #[test]
    fn serde_toml_round_trip() {
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct Wrapper {
            color: Rgba,
        }
        let w = Wrapper {
            color: Rgba::rgba(61, 174, 233, 128),
        };
        let toml_str = toml::to_string(&w).unwrap();
        let deserialized: Wrapper = toml::from_str(&toml_str).unwrap();
        assert_eq!(deserialized, w);
    }

    // === to_f32_array tests ===

    #[test]
    fn to_f32_array_black() {
        let arr = Rgba::rgb(0, 0, 0).to_f32_array();
        assert_eq!(arr, [0.0, 0.0, 0.0, 1.0]);
    }

    #[test]
    fn to_f32_array_white_transparent() {
        let arr = Rgba::rgba(255, 255, 255, 0).to_f32_array();
        assert_eq!(arr, [1.0, 1.0, 1.0, 0.0]);
    }

    // === Trait tests ===

    #[test]
    fn rgba_is_copy() {
        let a = Rgba::rgb(1, 2, 3);
        let b = a; // Copy
        assert_eq!(a, b); // a still accessible after copy
    }

    #[test]
    fn rgba_default_is_transparent_black() {
        let d = Rgba::default();
        assert_eq!(
            d,
            Rgba {
                r: 0,
                g: 0,
                b: 0,
                a: 0
            }
        );
    }

    #[test]
    fn rgba_is_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(Rgba::rgb(1, 2, 3));
        assert!(set.contains(&Rgba::rgb(1, 2, 3)));
    }

    // === from_f32 tests ===

    #[test]
    fn from_f32_basic() {
        let c = Rgba::from_f32(1.0, 0.5, 0.0, 1.0);
        assert_eq!(c.r, 255);
        assert_eq!(c.g, 128); // 0.5 * 255 = 127.5, round to 128
        assert_eq!(c.b, 0);
        assert_eq!(c.a, 255);
    }

    #[test]
    fn from_f32_clamps_out_of_range() {
        let c = Rgba::from_f32(-0.5, 1.5, 0.0, 0.0);
        assert_eq!(c.r, 0);
        assert_eq!(c.g, 255);
    }

    // === to_f32_tuple test ===

    #[test]
    fn to_f32_tuple_matches_array() {
        let c = Rgba::rgb(128, 64, 32);
        let arr = c.to_f32_array();
        let tup = c.to_f32_tuple();
        assert_eq!(tup, (arr[0], arr[1], arr[2], arr[3]));
    }
}
