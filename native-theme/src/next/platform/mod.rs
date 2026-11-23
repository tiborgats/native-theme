//! The compiled colour kernels of the KDE preset, one file each, taken from
//! `v0.8.0-rc1` (`native-theme/src/platform/` at 78ee0efd) with their vector
//! tables in `tests/vectors/`. `kde_shade`, `kirigami_lerp`, the KDE colour
//! functions `kcu_mix`, `kcu_tint`, `kcu_overlay` and `kde_selection_inactive`,
//! with their shared arithmetic in `kde_color_common`, and Plasma's accent
//! derivation `kde_accent` were written in a clean room, from behavioural
//! descriptions and the vector tables alone (the plan's D10). No line of
//! KDE's or Qt's code is in them. Of rc1's `kde_accent.rs` the `[WM]`
//! function `kde_accent_wm` and its table are not taken: `kde-breeze` runs
//! neither the tint nor the accent-coloured title bar, the only steps that
//! change `[WM]`. The tables were
//! generated on that branch by `tools/kde-vectors` from the sources their
//! headers name; the generator is not on this branch. `kcu_mix_of_mix.tsv`
//! was generated on this branch, by the harness its header names.

pub(crate) mod kcu_mix;
pub(crate) mod kcu_overlay;
pub(crate) mod kcu_tint;
pub(crate) mod kde_accent;
#[allow(
    dead_code,
    reason = "carried as the clean room wrote it; `shade_q` serves `kcu_shade`, which is not taken"
)]
mod kde_color_common;
pub(crate) mod kde_selection_inactive;
pub(crate) mod kde_shade;
pub(crate) mod kirigami_lerp;
pub(crate) mod qt_lighter;
pub(crate) mod qt_tint;
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
