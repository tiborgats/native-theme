//! The bundled presets (S16): each enters with its facts (M1a tasks 15 and
//! 16), its `[pending]` table listing what its facts leave pending (S17).

/// The bundled presets' one list: each variant with its key, its file and
/// its display name, so [`PresetId::key`], [`PresetId::source`],
/// [`PresetId::display_name`] and [`PresetId::all`] cannot disagree, and S19.1's build test, which walks `all`, reaches every
/// preset `Theme::from_preset` can be given.
macro_rules! presets {
    ($($(#[$doc:meta])* $variant:ident => $key:literal, $file:literal, $name:literal;)*) => {
        /// A bundled preset: one variant per preset file.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub enum PresetId {
            $($(#[$doc])* $variant,)*
        }

        impl PresetId {
            /// The preset's key: S11's `id`, `preset:<key>` and
            /// `for_preset.<key>`.
            #[must_use]
            pub fn key(self) -> &'static str {
                match self {
                    $(PresetId::$variant => $key,)*
                }
            }

            /// The preset's name for people (S16): `"KDE Breeze"`, what a
            /// preset picker lists and a connector names its toolkit's theme.
            #[must_use]
            pub fn display_name(self) -> &'static str {
                match self {
                    $(PresetId::$variant => $name,)*
                }
            }
            /// The preset's file, in S11's format.
            pub(crate) fn source(self) -> &'static str {
                match self {
                    $(PresetId::$variant => include_str!($file),)*
                }
            }

            /// Every bundled preset, in a stable order.
            #[must_use]
            pub fn all() -> &'static [PresetId] {
                &[$(PresetId::$variant),*]
            }
        }
    };
}

presets! {
    /// GNOME's libadwaita.
    Adwaita => "adwaita", "adwaita.toml", "Adwaita";
    /// Apple's macOS 26 Tahoe.
    MacosTahoe => "macos-tahoe", "macos-tahoe.toml", "macOS Tahoe";
    /// Apple's macOS 12 Monterey to macOS 15, its facts read on macOS 15.
    MacosMonterey => "macos-monterey", "macos-monterey.toml", "macOS Monterey";
    /// Google's Material 3; Expressive is a flavour of it, not a preset
    /// (R1.6).
    Material3 => "material-3", "material-3.toml", "Material 3";
    /// Apple's iOS 26.
    Ios26 => "ios-26", "ios-26.toml", "iOS 26";
    /// Apple's iOS 13 to iOS 18, its facts read on iOS 18; named by its
    /// first version, as `macos-monterey` is.
    Ios13 => "ios-13", "ios-13.toml", "iOS 13";
    /// KDE's Breeze, Plasma 6.
    KdeBreeze => "kde-breeze", "kde-breeze.toml", "KDE Breeze";
    /// Microsoft's Windows 11 (WinUI 3).
    Windows11 => "windows-11", "windows-11.toml", "Windows 11";
}

impl PresetId {
    /// The preset `key` names.
    #[must_use]
    pub fn from_key(key: &str) -> Option<PresetId> {
        PresetId::all().iter().copied().find(|id| id.key() == key)
    }

    /// The preset's own `token.text.font_dpi`, the DPI a reader converts
    /// the point sizes it reads by where its platform reports none (S9.4);
    /// `None` where its file states none as a number in its bases.
    #[cfg_attr(
        not(all(target_os = "linux", feature = "portal")),
        allow(
            dead_code,
            reason = "a reader's: no reader reading points is linked in this build"
        )
    )]
    pub(crate) fn font_dpi(self) -> Option<f32> {
        let file = crate::toml::layer::read_preset(self.source()).ok()?;
        match file.layer.bases.tokens.text?.font_dpi? {
            crate::value::Value::Stated(dpi) => Some(dpi),
            _ => None,
        }
    }
}
