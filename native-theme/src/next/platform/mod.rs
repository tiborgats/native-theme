//! The compiled colour kernels of the KDE preset, one file each, taken from
//! `v0.8.0-rc1` (`native-theme/src/platform/` at 78ee0efd) with their vector
//! tables in `tests/vectors/`. `kde_shade` and the KDE colour functions
//! `kcu_mix`, `kcu_tint`, `kcu_overlay` and `kde_selection_inactive`, with
//! their shared arithmetic in `kde_color_common`, were written in a clean
//! room, from behavioural descriptions and the vector tables alone (the
//! plan's D10). No line of KDE's or Qt's code is in them. The tables were
//! generated on that branch by `tools/kde-vectors` from the sources their
//! headers name; the generator is not on this branch.

pub(crate) mod kcu_mix;
pub(crate) mod kcu_overlay;
pub(crate) mod kcu_tint;
#[allow(
    dead_code,
    reason = "carried as the clean room wrote it; `shade_q` serves `kcu_shade`, which is not taken"
)]
mod kde_color_common;
pub(crate) mod kde_selection_inactive;
pub(crate) mod kde_shade;
#[cfg(test)]
mod vectors;

/// The kind of colour role a KDE effect applies to (`kde_selection_inactive`'s
/// `role_kind`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum KdeRoleKind {
    /// A foreground role.
    Foreground,
    /// A decoration role.
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "a role kind of the vector table; no colour of kde-breeze is a decoration role's"
        )
    )]
    Decoration,
    /// A background role.
    Background,
}
