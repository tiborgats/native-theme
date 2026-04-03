// Dialog button ordering convention

use serde::{Deserialize, Serialize};

/// Specifies the order of affirmative/cancel buttons in dialogs.
///
/// This is a **platform convention**, not visual styling. Different desktop
/// environments place the confirmation button at different ends of the
/// button row (KDE: leading, Windows/GNOME/macOS: trailing). It is part
/// of the theme model because "native feel" includes layout conventions
/// that vary by platform, and it is overridable in theme presets.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DialogButtonOrder {
    /// Affirmative button at the trailing (right) end -- Windows, GNOME, macOS, iOS style.
    #[default]
    TrailingAffirmative,
    /// Affirmative button at the leading (left) end -- KDE style.
    LeadingAffirmative,
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
    fn trailing_affirmative_serializes_correctly() {
        let w = Wrapper {
            order: DialogButtonOrder::TrailingAffirmative,
        };
        let toml_str = toml::to_string(&w).unwrap();
        assert!(toml_str.contains("trailing_affirmative"), "got: {toml_str}");
    }

    #[test]
    fn leading_affirmative_serializes_correctly() {
        let w = Wrapper {
            order: DialogButtonOrder::LeadingAffirmative,
        };
        let toml_str = toml::to_string(&w).unwrap();
        assert!(toml_str.contains("leading_affirmative"), "got: {toml_str}");
    }

    #[test]
    fn trailing_affirmative_round_trip() {
        let original = Wrapper {
            order: DialogButtonOrder::TrailingAffirmative,
        };
        let serialized = toml::to_string(&original).unwrap();
        let deserialized: Wrapper = toml::from_str(&serialized).unwrap();
        assert_eq!(deserialized, original);
    }

    #[test]
    fn leading_affirmative_round_trip() {
        let original = Wrapper {
            order: DialogButtonOrder::LeadingAffirmative,
        };
        let serialized = toml::to_string(&original).unwrap();
        let deserialized: Wrapper = toml::from_str(&serialized).unwrap();
        assert_eq!(deserialized, original);
    }

    #[test]
    fn trailing_affirmative_deserializes_from_toml_value() {
        let toml_str = r#"order = "trailing_affirmative""#;
        let w: Wrapper = toml::from_str(toml_str).unwrap();
        assert_eq!(w.order, DialogButtonOrder::TrailingAffirmative);
    }

    #[test]
    fn leading_affirmative_deserializes_from_toml_value() {
        let toml_str = r#"order = "leading_affirmative""#;
        let w: Wrapper = toml::from_str(toml_str).unwrap();
        assert_eq!(w.order, DialogButtonOrder::LeadingAffirmative);
    }
}
