//! The bundled presets and what a platform builder resolves from (spec §6).

use super::color::Color;
use super::reader::SystemOverrides;

/// A bundled platform preset.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Preset {
    /// GNOME's Adwaita.
    Adwaita,
    /// KDE Plasma's Breeze.
    KdeBreeze,
    /// macOS Monterey.
    MacosMonterey,
    /// macOS Tahoe.
    MacosTahoe,
    /// Windows 11.
    Windows11,
    /// Material 3.
    Material3,
    /// iOS 13.
    Ios13,
}

impl Preset {
    /// Every preset, in declaration order.
    pub const ALL: &'static [Self] = &[
        Self::Adwaita,
        Self::KdeBreeze,
        Self::MacosMonterey,
        Self::MacosTahoe,
        Self::Windows11,
        Self::Material3,
        Self::Ios13,
    ];

    /// The preset of the machine-readable name `name` (`"kde-breeze"`), if
    /// there is one.
    #[must_use]
    pub fn from_name(name: &str) -> Option<Self> {
        Self::ALL.iter().copied().find(|p| p.as_str() == name)
    }

    /// The machine-readable name, as 0.7.2's preset keys spell it
    /// (`src/presets.rs`, the bundled presets' table).
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Adwaita => "adwaita",
            Self::KdeBreeze => "kde-breeze",
            Self::MacosMonterey => "macos-monterey",
            Self::MacosTahoe => "macos-tahoe",
            Self::Windows11 => "windows-11",
            Self::Material3 => "material-3",
            Self::Ios13 => "ios-13",
        }
    }

    /// The human-readable name, as 0.7.2's display names spell it
    /// (`src/presets.rs`, the bundled presets' table).
    #[must_use]
    pub fn display_name(self) -> &'static str {
        match self {
            Self::Adwaita => "Adwaita",
            Self::KdeBreeze => "KDE Breeze",
            Self::MacosMonterey => "macOS Monterey",
            Self::MacosTahoe => "macOS Tahoe",
            Self::Windows11 => "Windows 11",
            Self::Material3 => "Material",
            Self::Ios13 => "iOS",
        }
    }
}

/// What a platform builder resolves from besides the context: the system's
/// settings and an explicit accent.
///
/// The spec's `palette: Option<&'a Palette>` field comes with the `Palette`
/// type (plan, M1).
#[derive(Clone, Copy, Debug)]
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "read by the kde-breeze builder, the next task of M0"
    )
)]
pub(crate) struct ResolveInputs<'a> {
    /// The system's settings, where read.
    pub(crate) system: Option<&'a SystemOverrides>,
    /// An explicit accent.
    pub(crate) accent: Option<Color>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_round_trip_and_are_distinct() {
        for preset in Preset::ALL {
            assert_eq!(Preset::from_name(preset.as_str()), Some(*preset));
            let same_name = Preset::ALL
                .iter()
                .filter(|p| {
                    p.as_str() == preset.as_str() || p.display_name() == preset.display_name()
                })
                .count();
            assert_eq!(same_name, 1, "{preset:?}");
        }
        assert_eq!(Preset::from_name("kde-breeze"), Some(Preset::KdeBreeze));
        assert_eq!(Preset::KdeBreeze.display_name(), "KDE Breeze");
        assert_eq!(Preset::from_name("KDE Breeze"), None);
        assert_eq!(Preset::from_name("catppuccin-mocha"), None);
    }

    #[test]
    fn resolve_inputs_hold_what_they_are_given() {
        let system = SystemOverrides::default();
        let inputs = ResolveInputs {
            system: Some(&system),
            accent: Some(Color::WHITE),
        };
        assert_eq!(inputs.system, Some(&system));
        assert_eq!(inputs.accent, Some(Color::WHITE));
    }
}
