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
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

// Implementation will be added in GREEN phase

#[cfg(test)]
mod tests {
    use super::*;

    // === Constructor tests ===

    #[test]
    fn rgb_constructor_sets_alpha_255() {
        let c = Rgba::rgb(61, 174, 233);
        assert_eq!(c, Rgba { r: 61, g: 174, b: 233, a: 255 });
    }

    #[test]
    fn rgba_constructor_sets_all_fields() {
        let c = Rgba::rgba(61, 174, 233, 128);
        assert_eq!(c, Rgba { r: 61, g: 174, b: 233, a: 128 });
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
        let w = Wrapper { color: Rgba::rgba(61, 174, 233, 128) };
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
        assert_eq!(d, Rgba { r: 0, g: 0, b: 0, a: 0 });
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
