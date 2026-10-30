//! The bundled presets (S16): none yet, as no preset states every field
//! the model holds; each enters with its facts (M1a tasks 15 and 16).

/// A bundled preset: one variant per preset file, none yet.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PresetId {}

impl PresetId {
    /// The preset's key: S11's `id`, `preset:<key>` and `for_preset.<key>`.
    #[must_use]
    pub fn key(self) -> &'static str {
        match self {}
    }

    /// The preset `key` names.
    #[must_use]
    pub fn from_key(key: &str) -> Option<PresetId> {
        PresetId::all().iter().copied().find(|id| id.key() == key)
    }

    /// Every bundled preset, in a stable order.
    #[must_use]
    pub fn all() -> &'static [PresetId] {
        &[]
    }
}
