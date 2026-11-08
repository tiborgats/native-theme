//! The bundled presets (S16): each enters with its facts (M1a tasks 15 and
//! 16), its `[pending]` table listing what its facts leave pending (S17).

/// The bundled presets' one list: each variant with its key and its file,
/// so [`PresetId::key`], [`PresetId::source`] and [`PresetId::all`] cannot
/// disagree, and S19.1's build test, which walks `all`, reaches every
/// preset `Theme::from_preset` can be given.
macro_rules! presets {
    ($($(#[$doc:meta])* $variant:ident => $key:literal, $file:literal;)*) => {
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
    Adwaita => "adwaita", "adwaita.toml";
    /// Apple's macOS 26 Tahoe.
    MacosTahoe => "macos-tahoe", "macos-tahoe.toml";
    /// Apple's macOS 12 Monterey to macOS 15, its facts read on macOS 15.
    MacosMonterey => "macos-monterey", "macos-monterey.toml";
    /// Google's Material 3.
    Material3 => "material-3", "material-3.toml";
    /// Apple's iOS 26.
    Ios26 => "ios-26", "ios-26.toml";
    /// Apple's iOS 13 to iOS 18, its facts read on iOS 18.
    Ios13 => "ios-13", "ios-13.toml";
    /// KDE's Breeze, Plasma 6.
    KdeBreeze => "kde-breeze", "kde-breeze.toml";
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
