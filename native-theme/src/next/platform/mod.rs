//! S2.2's platform functions, one file each; public for their vector tests
//! (`tests/platform_vectors.rs`). `kde_shade` and `kirigami_lerp` were written
//! in a clean room, from behavioural descriptions and the vector tables alone
//! (the plan's D10); the model's names for their operands are here.

pub mod kde_shade;
pub mod kirigami_lerp;

use kde_shade::KdeShadeRole;

use crate::next::toml::read::Named;

impl KdeShadeRole {
    /// Every role, in declaration order.
    pub(crate) const ALL: [KdeShadeRole; 5] = [
        KdeShadeRole::Light,
        KdeShadeRole::Midlight,
        KdeShadeRole::Mid,
        KdeShadeRole::Dark,
        KdeShadeRole::Shadow,
    ];

    /// Every role's name, in [`KdeShadeRole::ALL`]'s order, as expressions
    /// spell it.
    pub(crate) const NAMES: [&'static str; 5] = ["light", "midlight", "mid", "dark", "shadow"];
}

/// `kde_shade`'s `role` as expressions spell it (S2.2).
impl Named for KdeShadeRole {
    fn name(&self) -> &'static str {
        match self {
            KdeShadeRole::Light => "light",
            KdeShadeRole::Midlight => "midlight",
            KdeShadeRole::Mid => "mid",
            KdeShadeRole::Dark => "dark",
            KdeShadeRole::Shadow => "shadow",
        }
    }

    fn from_name(name: &str) -> Option<KdeShadeRole> {
        KdeShadeRole::ALL
            .into_iter()
            .find(|role| role.name() == name)
    }
}
