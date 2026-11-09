//! Palettes (S14, R19): for now a KDE port, [`Palette::from_port`], a
//! `.colors` file converted as Plasma's default path applies it ("Accent
//! color from color scheme", no accent) to the layer `kde-breeze` takes in
//! the mapping's place.
//!
//! The conversion reads the file as the KDE reader reads a scheme file, its
//! KConfig reading, KColorScheme's defaults and the scheme inputs' own
//! conversion all `kde_config`'s, in every build with no feature
//! (ADD-A142); the roles `applyScheme()` derives with no accent are the
//! clean room's `kde_accent` and `kde_accent_wm` with the accent
//! `#00000000` (S2.2, the 0.8.0 plan's D10), through their adapter
//! ([`KdeAccentScheme`]), so no second derivation exists beside them.

use configparser::ini::Ini;

use crate::blocks::BlockPatch;
use crate::color::Color;
use crate::error::PortError;
use crate::inputs::{InputName, InputValue, KdeSchemeKey, KdeWmKey};
use crate::kde_config::{
    COLOR_GROUPS, TRANSPARENT, active_value, color, create_kde_parser, defaults, expr,
    has_color_group, inactive_effects, inactive_value, input, kconfig_wm_key, kde_tokens,
    read_kconfig_bytes, scheme_inputs, scheme_options, surfaces,
};
use crate::key::Key;
use crate::layer::{Bases, Layer};
use crate::platform::kde_accent_operands::{KdeAccentKeyInputs, KdeAccentScheme};
use crate::platform::kde_effects::{KdeGroupEffects, kde_effects};
use crate::presets::PresetId;
use crate::roles::{ColorRolesPatch, SurfaceSet};
use crate::selector::{Condition, Rule, RuleBody, Scheme, Selector, SetMatch};
use crate::tokens::{KdeGroup, KdeRoleKind, TokensPatch};
use crate::value::{SchemeMap, Value};

/// A palette (S14): the ports it has, each for one preset and one scheme.
/// [`Palette::from_port`] makes one from a KDE colour scheme file.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct Palette {
    /// The ports, one per preset.
    #[allow(
        dead_code,
        reason = "read by `Theme::apply_palette` (S14), the accent run's next task"
    )]
    ports: Vec<Port>,
}

/// A palette's port (S14): the layer a `.colors` file converts to, for the
/// preset it carries and the one scheme its caller named.
#[derive(Clone, Debug, PartialEq)]
#[allow(
    dead_code,
    reason = "read by `Theme::apply_palette` (S14), the accent run's next task"
)]
pub(crate) struct Port {
    /// The preset the port is for: a `.colors` file's is `kde-breeze`.
    pub(crate) preset: PresetId,
    /// The scheme the caller named for the file.
    pub(crate) scheme: Scheme,
    /// The conversion: under [`Port::scheme`], in the preset's own part,
    /// the scheme inputs and their lists, the colour roles and the inactive
    /// input roles; and `token.kde` as a `Tokens` rule under the scheme
    /// and the preset.
    pub(crate) layer: Layer,
}

impl Palette {
    /// Converts the KDE colour scheme file `file` (a `.colors` file, its
    /// bytes) to a palette with one port, for `kde-breeze` under `scheme`,
    /// the scheme the caller names for it (S14): a `.colors` file does not
    /// say whether it is light or dark.
    ///
    /// The bytes are decoded as UTF-8, each invalid sequence replaced by
    /// U+FFFD, and the lines read by KConfig's rules, as the KDE reader
    /// reads a scheme file (S9.3), so a line KConfig skips is skipped and a
    /// group or key the file lacks is read as KColorScheme reads it unset.
    /// The port states, under `scheme`:
    ///
    /// - the scheme inputs `input.kde_scheme.*` and their three lists, as
    ///   the KDE reader converts a scheme file (S9.1);
    /// - the colour roles, the file's values as KColorScheme reads them;
    /// - each inactive input role `input.inactive.<set>.<key>`: for a set
    ///   whose group has an `[Inactive]` group, `kde_accent(<set>, <key>,
    ///   inactive)` with the accent `#00000000` over the port's own scheme
    ///   inputs, computed here; for another set `= active(role.<set>.<role>)`,
    ///   as KColorScheme reads a set without one;
    /// - each `[WM]` keyed role `wm_<key>`, `kde_accent_wm(<key>)` with the
    ///   accent `#00000000`, computed here;
    /// - `token.kde`: the file's colour effects, tint factor and title-bar
    ///   option (S4.8), as a `Tokens` rule under `scheme`.
    ///
    /// # Errors
    ///
    /// [`PortError::NoColorGroup`] where the file has no colour group of
    /// one of `kde-breeze`'s seven sets, `[Colors:<Set>]` or a subgroup of
    /// one (empty, or no colour scheme at all).
    pub fn from_port(file: &[u8], scheme: Scheme) -> Result<Palette, PortError> {
        let mut ini = create_kde_parser();
        read_kconfig_bytes(&mut ini, file);
        if !has_color_group(&ini) {
            return Err(PortError::NoColorGroup);
        }
        Ok(Palette {
            ports: vec![Port {
                preset: PresetId::KdeBreeze,
                scheme,
                layer: port_layer(&ini, scheme),
            }],
        })
    }
}

/// The layer of the port `ini` under `scheme` ([`Palette::from_port`]).
fn port_layer(ini: &Ini, scheme: Scheme) -> Layer {
    let mut bases = Bases::default();
    scheme_inputs(ini, scheme, &mut bases);
    let options = scheme_options(ini);
    let groups = list(&bases.inputs.kde_groups, scheme);
    let inactive_groups = list(&bases.inputs.kde_inactive_groups, scheme);
    let unset_keys = list(&bases.inputs.kde_unset_keys, scheme);
    let (tint_factor, titlebar_is_accent_colored) = options;
    let derivation = KdeAccentScheme {
        accent: TRANSPARENT,
        groups: &groups,
        inactive_groups: &inactive_groups,
        unset_keys: &unset_keys,
        tint_factor,
        titlebar_is_accent_colored,
    };

    let keyed = wm_roles(ini, &derivation);
    *bases.colors.slot(scheme) = Some(ColorRolesPatch {
        surfaces: Some(surfaces(ini)),
        keyed: Some(keyed),
        ..ColorRolesPatch::default()
    });
    let inactive = inactive_roles(ini, scheme, &derivation);
    bases.inputs.entries.extend(inactive);

    let mut layer = Layer::default();
    layer.for_preset.push((PresetId::KdeBreeze, bases));
    let tokens = TokensPatch {
        kde: Some(BlockPatch::Fields(kde_tokens(ini, Some(options)))),
        ..TokensPatch::default()
    };
    let selector = Selector {
        conditions: SetMatch {
            all_of: vec![
                Condition::Scheme(scheme),
                Condition::Preset(PresetId::KdeBreeze),
            ],
            none_of: Vec::new(),
        },
        ..Selector::default()
    };
    layer
        .rules
        .push(Rule::new(selector, RuleBody::Tokens(Box::new(tokens))));
    layer
}

/// The list `lists` holds under `scheme`, which [`scheme_inputs`] states.
fn list<T: Clone>(lists: &SchemeMap<Vec<T>>, scheme: Scheme) -> Vec<T> {
    lists.get(scheme).cloned().unwrap_or_default()
}

/// `Window`'s scheme input `input.kde_scheme.window.<key>` of `ini`.
fn window(ini: &Ini, key: KdeSchemeKey) -> Color {
    active_value(ini, SurfaceSet::Window, "Window", &defaults::WINDOW, key)
}

/// `View`'s scheme input `key` of `ini` ([`window`]).
fn view(ini: &Ini, key: KdeSchemeKey) -> Color {
    active_value(ini, SurfaceSet::View, "View", &defaults::VIEW, key)
}

/// `Header`'s scheme input `key` of `ini` ([`window`]).
fn header(ini: &Ini, key: KdeSchemeKey) -> Color {
    active_value(ini, SurfaceSet::Header, "Header", &defaults::HEADER, key)
}

/// The inactive input roles of the port `ini` under `scheme`
/// ([`Palette::from_port`]): `derivation`'s `kde_accent(<set>, <key>,
/// inactive)` for a set `inactive_groups` names, else `=
/// active(role.<set>.<role>)`.
fn inactive_roles(
    ini: &Ini,
    scheme: Scheme,
    derivation: &KdeAccentScheme<'_>,
) -> Vec<(InputName, InputValue)> {
    let view_background = view(ini, KdeSchemeKey::BackgroundNormal);
    let mut entries = Vec::new();
    for (set, name, defaults) in COLOR_GROUPS {
        let named = derivation.inactive_groups.contains(&set);
        for key in KdeSchemeKey::ALL {
            let value = if named {
                let inputs = KdeAccentKeyInputs {
                    value: active_value(ini, set, name, defaults, key),
                    inactive_value: inactive_value(ini, set, name, defaults, key),
                    background: active_value(
                        ini,
                        set,
                        name,
                        defaults,
                        KdeSchemeKey::BackgroundNormal,
                    ),
                    window_value: window(ini, key),
                    window_background: window(ini, KdeSchemeKey::BackgroundNormal),
                    view_background,
                };
                derivation
                    .kde_accent(set, key, KdeGroup::Inactive, &inputs)
                    .map(Value::Stated)
            } else {
                expr(&format!(
                    "active(role.{}.{})",
                    set.name(),
                    key.role().path()
                ))
            };
            let path = format!("inactive.{}.{}", set.name(), key.name());
            if let (Some(name), Some(value)) = (input(&path), value) {
                entries.push((name, InputValue::new(SchemeMap::one(scheme, value))));
            }
        }
    }
    entries
}

/// The `[WM]` keyed roles `wm_<key>` of the port `ini`: `derivation`'s
/// `kde_accent_wm(<key>)` over `input.kde_scheme.wm.<key>`
/// ([`wm_input`]).
fn wm_roles(ini: &Ini, derivation: &KdeAccentScheme<'_>) -> Vec<(Key, Value<Color>)> {
    let window_background = window(ini, KdeSchemeKey::BackgroundNormal);
    let (effects, enabled) = inactive_effects(ini);
    KdeWmKey::ALL
        .into_iter()
        .filter_map(|key| {
            let role = Key::new(&format!("wm_{}", key.name())).ok()?;
            let value = wm_input(ini, key, &effects, enabled);
            let color = derivation.kde_accent_wm(key, value, window_background);
            Some((role, Value::Stated(color)))
        })
        .collect()
}

/// The colour `input.kde_scheme.wm.<key>` of `ini` holds, as
/// [`scheme_inputs`] states it: the file's `[WM]` key where it states one,
/// else the value of the default it states, the `Header` set's Normal
/// background or foreground for an active key, and for an inactive one
/// that of its `[Inactive]` group as the inactive effects `effects`, where
/// `enabled`, apply to it (`kde_effects`).
fn wm_input(ini: &Ini, key: KdeWmKey, effects: &KdeGroupEffects, enabled: bool) -> Color {
    if let Some(color) = color(ini, "WM", kconfig_wm_key(key)) {
        return color;
    }
    let header_inactive =
        |key| inactive_value(ini, SurfaceSet::Header, "Header", &defaults::HEADER, key);
    let inactive = |key, role_kind| {
        let [r, g, b, a] = kde_effects(
            channels(header_inactive(key)),
            channels(header_inactive(KdeSchemeKey::BackgroundNormal)),
            KdeGroup::Inactive,
            role_kind,
            effects,
            enabled,
        );
        Color { r, g, b, a }
    };
    match key {
        KdeWmKey::ActiveBackground => header(ini, KdeSchemeKey::BackgroundNormal),
        KdeWmKey::ActiveForeground => header(ini, KdeSchemeKey::ForegroundNormal),
        KdeWmKey::InactiveBackground => {
            inactive(KdeSchemeKey::BackgroundNormal, KdeRoleKind::Background)
        }
        KdeWmKey::InactiveForeground => {
            inactive(KdeSchemeKey::ForegroundNormal, KdeRoleKind::Foreground)
        }
    }
}

/// The channels of `c`, as the platform functions take a colour.
fn channels(c: Color) -> [f32; 4] {
    [c.r, c.g, c.b, c.a]
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests;
