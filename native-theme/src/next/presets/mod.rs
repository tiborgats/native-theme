//! The bundled presets (S16): none yet, as no preset states every field
//! the model holds; each enters with its facts (M1a tasks 15 and 16).

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

presets! {}

impl PresetId {
    /// The preset `key` names.
    #[must_use]
    pub fn from_key(key: &str) -> Option<PresetId> {
        PresetId::all().iter().copied().find(|id| id.key() == key)
    }
}
