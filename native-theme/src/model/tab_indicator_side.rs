// Active-tab indicator placement convention

use serde::{Deserialize, Serialize};

/// Which edge of the active tab its indicator line lies along, for tabs above
/// their pages.
///
/// A **platform convention**, like [`ArrowSide`](crate::model::ArrowSide):
/// Breeze lays its highlight strip along a North tab's top edge, libadwaita's
/// GtkNotebook and Material's primary tabs along the bottom one
/// (`docs/platform-facts.md` §2.11, `active_indicator_side`). Tabs below their
/// pages mirror it on every platform that draws one.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TabIndicatorSide {
    /// Along the tab's top edge, away from its page -- KDE.
    #[serde(rename = "top")]
    Top,
    /// Along the tab's bottom edge, next to its page -- GNOME, Material.
    #[serde(rename = "bottom")]
    Bottom,
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

    // TOML cannot serialize a bare enum as a top-level value; use a wrapper struct.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    struct Wrapper {
        side: TabIndicatorSide,
    }

    #[test]
    fn serde_round_trip_both_variants() {
        for (variant, expected_str) in [
            (TabIndicatorSide::Top, "top"),
            (TabIndicatorSide::Bottom, "bottom"),
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
