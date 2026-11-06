//! `kde_effects`: KColorScheme's state effects (intensity, colour, contrast) on a colour role.

use super::kde_color_common::{
    QtColor, Rgba, enter, from_hcy, mix_q, norm, read, shade_q, tint_q, to_hcy,
};
use crate::tokens::{KdeColorEffect, KdeContrastEffect, KdeGroup, KdeIntensityEffect, KdeRoleKind};

/// One group's effect tokens (`token.kde.<group>.*`), operands `kde_effects`
/// reads itself: an input its caller fills in full, so exhaustive (S21.2).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KdeGroupEffects {
    /// The intensity effect.
    pub intensity: KdeIntensityEffect,
    /// The intensity effect's amount.
    pub intensity_amount: f32,
    /// The colour effect.
    pub color: KdeColorEffect,
    /// The colour effect's amount.
    pub color_amount: f32,
    /// The colour effect's colour.
    pub color_color: Rgba,
    /// The contrast effect.
    pub contrast: KdeContrastEffect,
    /// The contrast effect's amount.
    pub contrast_amount: f32,
}

/// `KColorUtils::darken` with double amounts.
fn darken(x: QtColor, ky: f64, kc: f64) -> QtColor {
    let mut h = to_hcy(x);
    h.y = norm(h.y * (1.0 - ky));
    h.c = norm(h.c * kc);
    from_hcy(h)
}

/// `KColorUtils::lighten` with double amounts.
fn lighten(x: QtColor, ky: f64, kc: f64) -> QtColor {
    let mut h = to_hcy(x);
    h.y = 1.0 - norm((1.0 - h.y) * (1.0 - ky));
    h.c = 1.0 - norm((1.0 - h.c) * kc);
    from_hcy(h)
}

/// Applies a group's state effects to the colour `c` of a role of kind `role_kind`, as
/// KColorScheme does: for a foreground or decoration role first the contrast effect against
/// `background` (the group's starting background), then for every kind the intensity effect and
/// the colour effect. `effects` are the group's tokens; the active group has no effects, and the
/// inactive group none unless `inactive_enabled`.
pub fn kde_effects(
    c: Rgba,
    background: Rgba,
    group: KdeGroup,
    role_kind: KdeRoleKind,
    effects: &KdeGroupEffects,
    inactive_enabled: bool,
) -> Rgba {
    let mut x = enter(c);
    let background = enter(background);
    match group {
        KdeGroup::Active => return read(x),
        KdeGroup::Inactive if !inactive_enabled => return read(x),
        KdeGroup::Inactive | KdeGroup::Disabled => {}
    }
    if matches!(role_kind, KdeRoleKind::Foreground | KdeRoleKind::Decoration) {
        let amount = f64::from(effects.contrast_amount);
        x = match effects.contrast {
            KdeContrastEffect::None => x,
            KdeContrastEffect::Fade => mix_q(x, background, amount),
            KdeContrastEffect::Tint => tint_q(x, background, amount),
        };
    }
    let amount = f64::from(effects.intensity_amount);
    x = match effects.intensity {
        KdeIntensityEffect::None => x,
        KdeIntensityEffect::Shade => shade_q(x, amount, 0.0),
        KdeIntensityEffect::Darken => darken(x, amount, 1.0),
        KdeIntensityEffect::Lighten => lighten(x, amount, 1.0),
    };
    let amount = f64::from(effects.color_amount);
    x = match effects.color {
        KdeColorEffect::None => x,
        KdeColorEffect::Desaturate => darken(x, 0.0, 1.0 - amount),
        KdeColorEffect::Fade => mix_q(x, enter(effects.color_color), amount),
        KdeColorEffect::Tint => tint_q(x, enter(effects.color_color), amount),
    };
    read(x)
}
