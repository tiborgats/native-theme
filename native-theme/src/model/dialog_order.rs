// Dialog button ordering convention

use serde::{Deserialize, Serialize};

/// Specifies the order of primary/cancel buttons in dialogs.
///
/// This is a **platform convention**, not visual styling. Different desktop
/// environments place the confirmation button at different ends of the
/// button row (KDE: leading/left, Windows/GNOME/macOS: trailing/right).
/// It is part of the theme model because "native feel" includes layout
/// conventions that vary by platform, and it is overridable in theme presets.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DialogButtonOrder {
    /// Primary button at the trailing (right) end -- Windows, GNOME, macOS, iOS style.
    #[default]
    #[serde(rename = "primary_right")]
    PrimaryRight,
    /// Primary button at the leading (left) end -- KDE style.
    #[serde(rename = "primary_left")]
    PrimaryLeft,
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

    // TOML cannot serialize a bare enum as a top-level value; use a wrapper struct.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    struct Wrapper {
        order: DialogButtonOrder,
    }

    #[test]
    fn serde_round_trip_both_variants() {
        for (variant, expected_str) in [
            (DialogButtonOrder::PrimaryRight, "primary_right"),
            (DialogButtonOrder::PrimaryLeft, "primary_left"),
        ] {
            let original = Wrapper { order: variant };
            let serialized = toml::to_string(&original).unwrap();
            assert!(serialized.contains(expected_str), "got: {serialized}");
            let deserialized: Wrapper = toml::from_str(&serialized).unwrap();
            assert_eq!(deserialized, original);
        }
    }

    #[test]
    fn deserializes_from_toml_string_values() {
        for (toml_str, expected) in [
            (
                r#"order = "primary_right""#,
                DialogButtonOrder::PrimaryRight,
            ),
            (r#"order = "primary_left""#, DialogButtonOrder::PrimaryLeft),
        ] {
            let w: Wrapper = toml::from_str(toml_str).unwrap();
            assert_eq!(w.order, expected);
        }
    }

    #[test]
    fn debug_output_both_variants() {
        assert_eq!(
            format!("{:?}", DialogButtonOrder::PrimaryRight),
            "PrimaryRight"
        );
        assert_eq!(
            format!("{:?}", DialogButtonOrder::PrimaryLeft),
            "PrimaryLeft"
        );
    }

    #[test]
    fn default_is_primary_right() {
        assert_eq!(
            DialogButtonOrder::default(),
            DialogButtonOrder::PrimaryRight
        );
    }
}
