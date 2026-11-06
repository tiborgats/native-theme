//! S2.2's platform functions, one file each; public for their vector tests
//! (`tests/platform_vectors.rs`). `adw_standalone`, `kde_shade`,
//! `kirigami_lerp` and the seven KDE colour functions (`kcu_mix`, `kcu_tint`,
//! `kcu_shade`, `kcu_overlay`, `kirigami_tint`, `kde_effects`,
//! `kde_selection_inactive`, their shared arithmetic in `kde_color_common`)
//! were written in a clean room, from behavioural descriptions and the vector
//! tables alone (the plan's D10); `sf_tracking` reads Apple's published
//! tables, `gen_sf_tracking` generated from them. The model's names for their
//! operands are here.

pub mod adw_standalone;
mod gen_sf_tracking;
pub mod kcu_mix;
pub mod kcu_overlay;
pub mod kcu_shade;
pub mod kcu_tint;
mod kde_color_common;
pub mod kde_effects;
pub mod kde_selection_inactive;
pub mod kde_shade;
pub mod kirigami_lerp;
pub mod kirigami_tint;
pub mod sf_tracking;

use kde_shade::KdeShadeRole;
use sf_tracking::TrackingDesign;

use crate::toml::read::Named;

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

impl TrackingDesign {
    /// Every design, in declaration order.
    pub(crate) const ALL: [TrackingDesign; 3] = [
        TrackingDesign::SfPro,
        TrackingDesign::SfProRounded,
        TrackingDesign::NewYork,
    ];

    /// Every design's name, in [`TrackingDesign::ALL`]'s order, as
    /// expressions spell it.
    pub(crate) const NAMES: [&'static str; 3] = ["sf_pro", "sf_pro_rounded", "new_york"];
}

/// `sf_tracking`'s `design` as expressions spell it (S2.2).
impl Named for TrackingDesign {
    fn name(&self) -> &'static str {
        match self {
            TrackingDesign::SfPro => "sf_pro",
            TrackingDesign::SfProRounded => "sf_pro_rounded",
            TrackingDesign::NewYork => "new_york",
        }
    }

    fn from_name(name: &str) -> Option<TrackingDesign> {
        TrackingDesign::ALL
            .into_iter()
            .find(|design| design.name() == name)
    }
}

#[cfg(test)]
mod tests {
    use super::{KdeShadeRole, TrackingDesign};
    use crate::toml::read::Named;

    /// `NAMES`, which types `kde_shade`'s `role`, and the names evaluation
    /// reads agree, role by role, each read back as itself.
    #[test]
    fn kde_shade_role_names_agree() {
        for (role, name) in KdeShadeRole::ALL.into_iter().zip(KdeShadeRole::NAMES) {
            assert_eq!(role.name(), name);
            assert_eq!(KdeShadeRole::from_name(name), Some(role));
        }
    }

    /// `NAMES`, which types `sf_tracking`'s `design`, and the names
    /// evaluation reads agree, design by design.
    #[test]
    fn tracking_design_names_agree() {
        for (design, name) in TrackingDesign::ALL.into_iter().zip(TrackingDesign::NAMES) {
            assert_eq!(design.name(), name);
            assert_eq!(TrackingDesign::from_name(name), Some(design));
        }
    }
}
