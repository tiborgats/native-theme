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
/// use native_theme::color::Rgba;
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
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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

// Phase 93-01 (G1): no `impl Default for Rgba`. §16 of the v0.5.7 API
// critique flagged that a theme library where `Rgba::default()` silently
// returns "transparent black" is a footgun. Callers who need a named zero
// colour use `Rgba::TRANSPARENT`, `Rgba::BLACK`, or `Rgba::WHITE` (all
// `const`). Task 1 of this plan broke the sole transitive bound chain that
// forced the Default impl (see `resolve::validate_helpers::require`).

impl Rgba {
    /// Create an opaque color (alpha = 255).
    #[must_use]
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    /// Create a color with explicit red, green, blue, and alpha components.
    #[must_use]
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Transparent black `(0, 0, 0, 0)` -- the zero colour.
    ///
    /// ```
    /// use native_theme::color::Rgba;
    ///
    /// assert_eq!(Rgba::TRANSPARENT, Rgba::new(0, 0, 0, 0));
    /// assert_eq!(Rgba::BLACK, Rgba::new(0, 0, 0, 255));
    /// assert_eq!(Rgba::WHITE, Rgba::new(255, 255, 255, 255));
    /// ```
    pub const TRANSPARENT: Self = Self {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
    };

    /// Opaque black `(0, 0, 0, 255)`.
    pub const BLACK: Self = Self {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    };

    /// Opaque white `(255, 255, 255, 255)`.
    pub const WHITE: Self = Self {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    };

    /// Create a color from floating-point components in the 0.0..=1.0 range.
    ///
    /// Values are clamped to 0.0..=1.0 before conversion.
    ///
    /// Note: round-trip through `from_f32` -> `to_f32_array` is lossy due to
    /// u8 quantization (e.g., `from_f32(0.5, ...)` -> r=128 ->
    /// `to_f32_array()` -> 0.50196...).
    #[must_use]
    pub fn from_f32(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {
            r: (r.clamp(0.0, 1.0) * 255.0).round() as u8,
            g: (g.clamp(0.0, 1.0) * 255.0).round() as u8,
            b: (b.clamp(0.0, 1.0) * 255.0).round() as u8,
            a: (a.clamp(0.0, 1.0) * 255.0).round() as u8,
        }
    }

    /// Convert to `[r, g, b, a]` in the 0.0..=1.0 range (for toolkit interop).
    ///
    /// Note: round-trip through `from_f32` -> `to_f32_array` is lossy due to
    /// u8 quantization (256 discrete steps per channel).
    #[must_use]
    pub fn to_f32_array(&self) -> [f32; 4] {
        [
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
            self.a as f32 / 255.0,
        ]
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

/// Error returned when parsing a hex color string fails.
///
/// Wraps a human-readable message describing the failure cause.
/// Implements [`std::error::Error`] so it works with `?` in functions
/// returning `Box<dyn Error>`.
#[derive(Debug, Clone)]
pub struct ParseColorError(String);

impl fmt::Display for ParseColorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for ParseColorError {}

/// Parse one ASCII hex digit byte to a nibble (0..=15).
fn hex_nibble(b: u8, label: &str) -> Result<u8, ParseColorError> {
    char::from(b)
        .to_digit(16)
        .and_then(|v| u8::try_from(v).ok())
        .ok_or_else(|| ParseColorError(format!("invalid {label} hex digit {:?}", char::from(b))))
}

/// Combine two nibbles (high, low) into a byte. Inputs assumed 0..=15.
fn hex_byte(hi: u8, lo: u8) -> u8 {
    // wrapping_shl(4) on 0..=15 produces 0..=240 in u8; BitOr is not panic-prone.
    hi.wrapping_shl(4) | (lo & 0x0f)
}

/// Expand a nibble to a byte by doubling it: `0xf -> 0xff`, `0xa -> 0xaa`.
/// Equivalent to `n * 17` for `n` in 0..=15, but panic-free.
fn double_nibble(n: u8) -> u8 {
    n.wrapping_shl(4) | (n & 0x0f)
}

impl FromStr for Rgba {
    type Err = ParseColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hex = s.strip_prefix('#').unwrap_or(s);

        if hex.is_empty() {
            return Err(ParseColorError("empty hex color string".into()));
        }

        // Destructure hex bytes directly. This is both UTF-8-safe (rejects any
        // non-ASCII input implicitly, since each non-ASCII char occupies 2+ bytes
        // and wouldn't match the single-byte slots) and avoids panic-prone string
        // slicing / indexing.
        match hex.as_bytes() {
            // #RGB shorthand: each digit doubled (e.g., 'a' -> 0xaa)
            [r, g, b] => {
                let r = hex_nibble(*r, "red")?;
                let g = hex_nibble(*g, "green")?;
                let b = hex_nibble(*b, "blue")?;
                Ok(Rgba::rgb(
                    double_nibble(r),
                    double_nibble(g),
                    double_nibble(b),
                ))
            }
            // #RGBA shorthand
            [r, g, b, a] => {
                let r = hex_nibble(*r, "red")?;
                let g = hex_nibble(*g, "green")?;
                let b = hex_nibble(*b, "blue")?;
                let a = hex_nibble(*a, "alpha")?;
                Ok(Rgba::new(
                    double_nibble(r),
                    double_nibble(g),
                    double_nibble(b),
                    double_nibble(a),
                ))
            }
            // #RRGGBB
            [r1, r2, g1, g2, b1, b2] => {
                let r = hex_byte(hex_nibble(*r1, "red")?, hex_nibble(*r2, "red")?);
                let g = hex_byte(hex_nibble(*g1, "green")?, hex_nibble(*g2, "green")?);
                let b = hex_byte(hex_nibble(*b1, "blue")?, hex_nibble(*b2, "blue")?);
                Ok(Rgba::rgb(r, g, b))
            }
            // #RRGGBBAA
            [r1, r2, g1, g2, b1, b2, a1, a2] => {
                let r = hex_byte(hex_nibble(*r1, "red")?, hex_nibble(*r2, "red")?);
                let g = hex_byte(hex_nibble(*g1, "green")?, hex_nibble(*g2, "green")?);
                let b = hex_byte(hex_nibble(*b1, "blue")?, hex_nibble(*b2, "blue")?);
                let a = hex_byte(hex_nibble(*a1, "alpha")?, hex_nibble(*a2, "alpha")?);
                Ok(Rgba::new(r, g, b, a))
            }
            other => Err(ParseColorError(format!(
                "invalid hex color length {}: expected 3, 4, 6, or 8 hex digits",
                other.len()
            ))),
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

/// Convert premultiplied RGBA pixel data to straight (non-premultiplied) alpha.
///
/// For each pixel where `a > 0 && a < 255`:
///   `channel = min(255, channel * 255 / a)`
///
/// Fully opaque pixels (a == 255) and fully transparent pixels (a == 0)
/// are left unchanged.
///
/// Used by `rasterize`, `sficons`, and `winicons` modules (feature/platform gated).
#[allow(dead_code)]
pub(crate) fn unpremultiply_alpha(buffer: &mut [u8]) {
    for pixel in buffer.chunks_exact_mut(4) {
        // Slice-pattern destructuring binds each byte without panic-prone indexing.
        // `chunks_exact_mut(4)` guarantees length 4, so the pattern always matches.
        let [r, g, b, a] = pixel else { continue };
        let a_val = u16::from(*a);
        // `a_val in 1..=254` guarantees the divisor is non-zero; `saturating_mul`
        // cannot overflow u16 (max 255 * 255 = 65025 < 65535). The `.min(255)` cap
        // makes the final `as u8` a lossless conversion (no truncation of high bits).
        if (1..255).contains(&a_val) {
            #[allow(clippy::integer_division, clippy::arithmetic_side_effects)]
            {
                *r = (u16::from(*r).saturating_mul(255) / a_val).min(255) as u8;
                *g = (u16::from(*g).saturating_mul(255) / a_val).min(255) as u8;
                *b = (u16::from(*b).saturating_mul(255) / a_val).min(255) as u8;
            }
        }
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
        let c = Rgba::new(61, 174, 233, 128);
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
        assert_eq!(c, Rgba::new(61, 174, 233, 128));
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
        assert_eq!(c, Rgba::new(0xaa, 0xbb, 0xcc, 0xdd));
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
        assert_eq!(Rgba::new(61, 174, 233, 128).to_string(), "#3daee980");
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
            color: Rgba::new(61, 174, 233, 128),
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
        let arr = Rgba::new(255, 255, 255, 0).to_f32_array();
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
}
