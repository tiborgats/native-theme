//! `kde_effects`: KColorScheme's state effects (intensity, colour, contrast) on a colour role.

#[allow(dead_code)]
#[path = "kde_color_common.rs"]
mod kde_color_common;

pub use kde_color_common::KdeRoleKind;
use kde_color_common::{enter, from_hcy, mix_q, norm, read, shade_q, tint_q, to_hcy, QtColor, Rgba};

/// A KColorScheme colour group.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KdeGroup {
    /// The active window's colours: no effects.
    Active,
    /// Disabled widgets' colours.
    Disabled,
    /// Inactive windows' colours.
    Inactive,
}

/// KColorScheme's intensity effect.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KdeIntensityEffect {
    /// Unchanged.
    None,
    /// `KColorUtils::shade` by the amount.
    Shade,
    /// `KColorUtils::darken` by the amount.
    Darken,
    /// `KColorUtils::lighten` by the amount.
    Lighten,
}

/// KColorScheme's colour effect.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KdeColorEffect {
    /// Unchanged.
    None,
    /// Chroma scaled by one minus the amount.
    Desaturate,
    /// Mixed toward the effect colour by the amount.
    Fade,
    /// Tinted toward the effect colour by the amount.
    Tint,
}

/// KColorScheme's contrast effect, against the group's background.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KdeContrastEffect {
    /// Unchanged.
    None,
    /// Mixed toward the background by the amount.
    Fade,
    /// Tinted toward the background by the amount.
    Tint,
}

/// One group's effect tokens (`token.kde.<group>.*`).
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

#[cfg(test)]
mod tests {
    use super::kde_color_common::table::{boolean, check, colour, number};
    use super::*;

    fn group(s: &str) -> Option<KdeGroup> {
        match s {
            "active" => Some(KdeGroup::Active),
            "disabled" => Some(KdeGroup::Disabled),
            "inactive" => Some(KdeGroup::Inactive),
            _ => None,
        }
    }

    fn intensity(s: &str) -> Option<KdeIntensityEffect> {
        match s {
            "none" => Some(KdeIntensityEffect::None),
            "shade" => Some(KdeIntensityEffect::Shade),
            "darken" => Some(KdeIntensityEffect::Darken),
            "lighten" => Some(KdeIntensityEffect::Lighten),
            _ => None,
        }
    }

    fn color(s: &str) -> Option<KdeColorEffect> {
        match s {
            "none" => Some(KdeColorEffect::None),
            "desaturate" => Some(KdeColorEffect::Desaturate),
            "fade" => Some(KdeColorEffect::Fade),
            "tint" => Some(KdeColorEffect::Tint),
            _ => None,
        }
    }

    fn contrast(s: &str) -> Option<KdeContrastEffect> {
        match s {
            "none" => Some(KdeContrastEffect::None),
            "fade" => Some(KdeContrastEffect::Fade),
            "tint" => Some(KdeContrastEffect::Tint),
            _ => None,
        }
    }

    #[test]
    fn vectors() {
        let (total, failures) = check(include_str!("../../vectors/kde_effects.tsv"), |t, row| {
            let effects = KdeGroupEffects {
                intensity: intensity(t.cell(row, "intensity")?)?,
                intensity_amount: number(t.cell(row, "intensity_amount")?)?,
                color: color(t.cell(row, "color")?)?,
                color_amount: number(t.cell(row, "color_amount")?)?,
                color_color: colour(t.cell(row, "color_color")?)?,
                contrast: contrast(t.cell(row, "contrast")?)?,
                contrast_amount: number(t.cell(row, "contrast_amount")?)?,
            };
            Some(kde_effects(
                colour(t.cell(row, "c")?)?,
                colour(t.cell(row, "background")?)?,
                group(t.cell(row, "group")?)?,
                KdeRoleKind::from_name(t.cell(row, "role_kind")?)?,
                &effects,
                boolean(t.cell(row, "inactive_enabled")?)?,
            ))
        });
        assert!(total > 0, "no rows");
        assert!(failures.is_empty(), "{} of {total} rows fail:\n{}", failures.len(), failures.join("\n"));
    }
}
