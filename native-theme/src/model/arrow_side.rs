// Disclosure arrow placement convention

use serde::{Deserialize, Serialize};

/// Which end of an expander's header row its disclosure arrow sits at.
///
/// A **platform convention**, like
/// [`DialogButtonOrder`](crate::model::DialogButtonOrder): KDE's
/// `KCollapsibleGroupBox` and macOS's disclosure triangle lead the title,
/// libadwaita's `AdwExpanderRow` and WinUI's `Expander` end the row with it
/// (`docs/platform-facts.md` §2.27, `arrow_side`). Leading and trailing are
/// the reading direction's: left and right in a left-to-right layout.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ArrowSide {
    /// Before the title, at the row's start -- KDE, macOS.
    #[serde(rename = "leading")]
    Leading,
    /// After the title, at the row's end -- GNOME, Windows.
    #[serde(rename = "trailing")]
    Trailing,
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

    // TOML cannot serialize a bare enum as a top-level value; use a wrapper struct.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    struct Wrapper {
        side: ArrowSide,
    }

    #[test]
    fn serde_round_trip_both_variants() {
        for (variant, expected_str) in [
            (ArrowSide::Leading, "leading"),
            (ArrowSide::Trailing, "trailing"),
        ] {
            let original = Wrapper { side: variant };
            let serialized = toml::to_string(&original).unwrap();
            assert!(serialized.contains(expected_str), "got: {serialized}");
            let deserialized: Wrapper = toml::from_str(&serialized).unwrap();
            assert_eq!(deserialized, original);
        }
    }

    #[test]
    fn an_unknown_side_is_rejected() {
        assert!(toml::from_str::<Wrapper>(r#"side = "left""#).is_err());
    }
}
