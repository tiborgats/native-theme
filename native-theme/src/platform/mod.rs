//! S2.2's platform functions, one file each; public for their vector tests
//! (`tests/platform_vectors.rs`). `adw_standalone`, `kde_shade`,
//! `kirigami_lerp`, the seven KDE colour functions (`kcu_mix`, `kcu_tint`,
//! `kcu_shade`, `kcu_overlay`, `kirigami_tint`, `kde_effects`,
//! `kde_selection_inactive`, their shared arithmetic in `kde_color_common`),
//! Plasma's accent derivation (`kde_accent`, `kde_accent_wm`) and Kirigami's
//! label-sized icon step (`kirigami_label_icon_size`) were written in a
//! clean room, from behavioural descriptions and the vector tables alone
//! (the plan's D10), `kde_accent_operands` mapping the model's operands
//! onto the accent derivation's; `sf_tracking` reads Apple's published
//! tables, `gen_sf_tracking` generated from them; `m3_role` ports
//! material-color-utilities under that code's Apache-2.0 licence (D10). S9.4's
//! scaling curves live here too: `android_non_linear` ports the Android Open
//! Source Project's `FontScaleConverter` under its Apache-2.0 licence, and
//! `dynamic_type` reads Apple's published Dynamic Type table,
//! `gen_dynamic_type` generated from it, and `win_ui` ports WinUI's
//! `TextFormatting::GetScaledFontSize` under its MIT licence. The model's
//! names for their operands are here.

pub mod adw_standalone;
pub mod android_non_linear;
pub mod dynamic_type;
mod gen_dynamic_type;
mod gen_sf_tracking;
pub mod kcu_mix;
pub mod kcu_overlay;
pub mod kcu_shade;
pub mod kcu_tint;
pub mod kde_accent;
pub(crate) mod kde_accent_operands;
mod kde_color_common;
pub mod kde_effects;
pub mod kde_selection_inactive;
pub mod kde_shade;
// The clean room's file stays as written (D10): its range check, two
// comparisons, is what clippy's `manual_range_contains` would have spelled
// `(-LIMIT..LIMIT).contains(&whole)`.
#[allow(clippy::manual_range_contains)]
pub mod kirigami_label_icon_size;
pub mod kirigami_lerp;
pub mod kirigami_tint;
pub mod m3_role;
pub mod sf_tracking;
pub mod win_ui;

use kde_shade::KdeShadeRole;
use sf_tracking::TrackingDesign;

use crate::tokens::{KdeGroup, KdeRoleKind};
use crate::toml::read::Named;

/// `kde_effects`' `group` as expressions spell it (S2.2, S4.8).
impl Named for KdeGroup {
    fn name(&self) -> &'static str {
        match self {
            KdeGroup::Active => "active",
            KdeGroup::Disabled => "disabled",
            KdeGroup::Inactive => "inactive",
        }
    }

    fn from_name(name: &str) -> Option<KdeGroup> {
        KdeGroup::ALL.into_iter().find(|group| group.name() == name)
    }
}

/// `kde_effects`' and `kde_selection_inactive`'s `role_kind` as expressions
/// spell it (S2.2, S4.8).
impl Named for KdeRoleKind {
    fn name(&self) -> &'static str {
        match self {
            KdeRoleKind::Foreground => "foreground",
            KdeRoleKind::Decoration => "decoration",
            KdeRoleKind::Background => "background",
        }
    }

    fn from_name(name: &str) -> Option<KdeRoleKind> {
        KdeRoleKind::ALL
            .into_iter()
            .find(|kind| kind.name() == name)
    }
}

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
    use crate::tokens::{KdeGroup, KdeRoleKind};
    use crate::toml::read::Named;

    /// `NAMES`, which types `kde_effects`' `group` and both KDE functions'
    /// `role_kind`, and the names evaluation reads agree, variant by variant.
    #[test]
    fn kde_group_and_role_kind_names_agree() {
        for (group, name) in KdeGroup::ALL.into_iter().zip(KdeGroup::NAMES) {
            assert_eq!(group.name(), name);
            assert_eq!(KdeGroup::from_name(name), Some(group));
        }
        for (kind, name) in KdeRoleKind::ALL.into_iter().zip(KdeRoleKind::NAMES) {
            assert_eq!(kind.name(), name);
            assert_eq!(KdeRoleKind::from_name(name), Some(kind));
        }
    }

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
