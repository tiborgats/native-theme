// Dialog entrance motion convention

use serde::{Deserialize, Serialize};

/// How a dialog moves onto the screen as it opens.
///
/// A **platform convention**, like
/// [`DialogButtonOrder`](crate::model::DialogButtonOrder): WinUI's
/// `ContentDialog`, libadwaita's floating `AdwDialog` and Material's dialog
/// scale up while they fade in, KDE's `Kirigami.Dialog` fades in while it
/// rises into place (`docs/platform-facts.md` §2.22, `entrance`). It names
/// the motion only; its durations and curves are the toolkit's.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DialogEntranceStyle {
    /// Fades in while scaling up to its full size -- Windows, GNOME, Material.
    #[serde(rename = "scale_fade")]
    ScaleFade,
    /// Fades in while rising into place from below -- KDE.
    #[serde(rename = "fade_rise")]
    FadeRise,
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

    // TOML cannot serialize a bare enum as a top-level value; use a wrapper struct.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    struct Wrapper {
        entrance: DialogEntranceStyle,
    }

    #[test]
    fn serde_round_trip_both_variants() {
        for (variant, expected_str) in [
            (DialogEntranceStyle::ScaleFade, "scale_fade"),
            (DialogEntranceStyle::FadeRise, "fade_rise"),
        ] {
            let original = Wrapper { entrance: variant };
            let serialized = toml::to_string(&original).unwrap();
            assert!(serialized.contains(expected_str), "got: {serialized}");
            let deserialized: Wrapper = toml::from_str(&serialized).unwrap();
            assert_eq!(deserialized, original);
        }
    }

    #[test]
    fn an_unknown_entrance_is_rejected() {
        assert!(toml::from_str::<Wrapper>(r#"entrance = "slide_down""#).is_err());
    }
}
