//! KDE's configuration files as KConfig reads them, and a colour scheme
//! file's conversion as KColorScheme completes it (S9.1, S9.3's *The KDE
//! reader's readings*, S14): in every build, on every target, with no
//! feature, so the KDE reader and a KDE port ([`Palette::from_port`]) never
//! read a file differently (ADD-A142).
//!
//! - The reading: a file's bytes decoded as UTF-8, each invalid sequence
//!   replaced ([`read_kconfig_bytes`]), its lines by KConfig's rules
//!   ([`read_kconfig`]), each entry as KConfig converts it ([`color`],
//!   [`flag`], [`number`]).
//! - KColorScheme's and KIconTheme's built-in defaults (`defaults.rs`).
//! - The conversion of a scheme file: its scheme inputs and their three
//!   lists ([`scheme_inputs`]), the colour groups' values as KColorScheme
//!   reads them ([`surfaces`]), its tint and title-bar options
//!   ([`scheme_options`]) and its colour effects, `token.kde`
//!   ([`kde_tokens`]).
//!
//! [`Palette::from_port`]: crate::Palette::from_port

pub(crate) mod defaults;

use configparser::ini::Ini;

use defaults::SetDefaults;

use crate::blocks::BlockPatch;
use crate::color::Color;
use crate::expr::Expr;
use crate::inputs::{InputName, InputValue, KdeSchemeKey, KdeWmKey};
use crate::layer::Bases;
use crate::platform::kde_effects::KdeGroupEffects;
use crate::roles::{SurfaceRoles, SurfaceRolesPatch, SurfaceSet};
use crate::selector::Scheme;
use crate::tokens::{
    KdeColorEffect, KdeContrastEffect, KdeEffectsPatch, KdeIntensityEffect, KdeTokensPatch,
};
use crate::value::{SchemeMap, SurfaceMap, Value};

/// The colour groups KColorScheme reads and `applyScheme()` writes, each the
/// surface set `kde-breeze` declares for it, its group's name,
/// `[Colors:<name>]` (`colorsapplicator.cpp:157-163`, S9.1), and
/// KColorScheme's defaults for its keys (`defaults.rs`).
pub(crate) const COLOR_GROUPS: [(SurfaceSet, &str, &SetDefaults); 7] = [
    (SurfaceSet::View, "View", &defaults::VIEW),
    (SurfaceSet::Window, "Window", &defaults::WINDOW),
    (SurfaceSet::Button, "Button", &defaults::BUTTON),
    (SurfaceSet::Selection, "Selection", &defaults::SELECTION),
    (SurfaceSet::Tooltip, "Tooltip", &defaults::TOOLTIP),
    (
        SurfaceSet::Complementary,
        "Complementary",
        &defaults::COMPLEMENTARY,
    ),
    (SurfaceSet::Header, "Header", &defaults::HEADER),
];

/// Qt's `transparent`, `#00000000`, the `AccentColor` `applyScheme()` reads
/// as no accent (`plasma-workspace@6.8.2 kcms/colors/colorsapplicator.cpp:130`,
/// S9.3).
pub(crate) const TRANSPARENT: Color = Color {
    r: 0.0,
    g: 0.0,
    b: 0.0,
    a: 0.0,
};

/// A KConfig colour entry in the form KConfig writes, `r,g,b` or `r,g,b,a`,
/// each channel an integer `0..=255` (`kconfig@6.31.0
/// src/gui/kconfiggroupgui.cpp:54-81`, `:134-150`); `None` for any other
/// value: a list of another length or a component no byte holds, which
/// KConfig reads as the key's default, and a value it reads as an invalid
/// colour (empty, `invalid`) or hands to Qt's colour parser (one starting
/// with `#`, or one without a comma), each read as unset (S9.3, a recorded
/// inexactness for the last).
pub(crate) fn parse_color(value: &str) -> Option<Color> {
    let channels: Vec<u8> = value
        .split(',')
        .map(|channel| {
            channel
                .trim()
                .parse::<i32>()
                .ok()
                .and_then(|channel| u8::try_from(channel).ok())
        })
        .collect::<Option<_>>()?;
    let channel = |index: usize| channels.get(index).map(|c| f32::from(*c) / 255.0);
    let alpha = match channels.len() {
        3 => 1.0,
        4 => channel(3)?,
        _ => return None,
    };
    Some(Color {
        r: channel(0)?,
        g: channel(1)?,
        b: channel(2)?,
        a: alpha,
    })
}

/// The colour `ini` holds at `section`'s `key` ([`parse_color`]).
pub(crate) fn color(ini: &Ini, section: &str, key: &str) -> Option<Color> {
    ini.get(section, key).as_deref().and_then(parse_color)
}

/// Whether `ini` has the group `section`.
pub(crate) fn has_group(ini: &Ini, section: &str) -> bool {
    ini.get_map_ref().contains_key(section)
}

/// The group of a colour group `name`: `Colors:<name>`.
fn group(name: &str) -> String {
    format!("Colors:{name}")
}

/// The `[Inactive]` group of a colour group `name`:
/// `[Colors:<name>][Inactive]`, which the parser names
/// `Colors:<name>][Inactive`.
fn inactive_group(name: &str) -> String {
    format!("Colors:{name}][Inactive")
}

/// The key `key` of a colour group, as the scheme files and kdeglobals
/// spell it.
const fn kconfig_key(key: KdeSchemeKey) -> &'static str {
    match key {
        KdeSchemeKey::BackgroundNormal => "BackgroundNormal",
        KdeSchemeKey::BackgroundAlternate => "BackgroundAlternate",
        KdeSchemeKey::ForegroundNormal => "ForegroundNormal",
        KdeSchemeKey::ForegroundInactive => "ForegroundInactive",
        KdeSchemeKey::ForegroundActive => "ForegroundActive",
        KdeSchemeKey::ForegroundLink => "ForegroundLink",
        KdeSchemeKey::ForegroundVisited => "ForegroundVisited",
        KdeSchemeKey::ForegroundNegative => "ForegroundNegative",
        KdeSchemeKey::ForegroundNeutral => "ForegroundNeutral",
        KdeSchemeKey::ForegroundPositive => "ForegroundPositive",
        KdeSchemeKey::DecorationFocus => "DecorationFocus",
        KdeSchemeKey::DecorationHover => "DecorationHover",
    }
}

/// The key `key` of the `[WM]` group, as the scheme files and kdeglobals
/// spell it.
pub(crate) const fn kconfig_wm_key(key: KdeWmKey) -> &'static str {
    match key {
        KdeWmKey::ActiveBackground => "activeBackground",
        KdeWmKey::ActiveForeground => "activeForeground",
        KdeWmKey::InactiveBackground => "inactiveBackground",
        KdeWmKey::InactiveForeground => "inactiveForeground",
    }
}

/// The colour group `name`'s `key` in `ini` as KColorScheme reads it: the
/// group's value, else its default ([`default_value`]).
pub(crate) fn active_value(
    ini: &Ini,
    set: SurfaceSet,
    name: &str,
    defaults: &SetDefaults,
    key: KdeSchemeKey,
) -> Color {
    color(ini, &group(name), kconfig_key(key))
        .unwrap_or_else(|| default_value(ini, set, defaults, key))
}

/// The default KColorScheme reads for the colour set `set`'s `key` in `ini`
/// where its group, or its `[Inactive]` group, leaves it unset: for
/// `Header` the file's `[Colors:Window]` value over `Header`'s table
/// (`kcolorscheme.cpp:330-333`), for another set its table `defaults`
/// (`:289-342`, `:251-278`; the KDE facts' scheme inputs cite each row).
fn default_value(ini: &Ini, set: SurfaceSet, defaults: &SetDefaults, key: KdeSchemeKey) -> Color {
    let window = (set == SurfaceSet::Header)
        .then(|| color(ini, &group("Window"), kconfig_key(key)))
        .flatten();
    window.unwrap_or_else(|| defaults.color(key))
}

/// The colour groups' keys in `ini` as KColorScheme reads them: every
/// set's roles the catalogue's key table maps them to
/// ([`KdeSchemeKey::role`]), a key `ini` leaves unset KColorScheme's
/// default ([`active_value`]).
pub(crate) fn surfaces(ini: &Ini) -> SurfaceMap<BlockPatch<SurfaceRoles>> {
    let mut surfaces = SurfaceMap::default();
    for (set, name, defaults) in COLOR_GROUPS {
        let mut patch = SurfaceRolesPatch::default();
        for key in KdeSchemeKey::ALL {
            let color = active_value(ini, set, name, defaults, key);
            set_role(&mut patch, key, Value::Stated(color));
        }
        surfaces.insert(set, BlockPatch::Fields(patch));
    }
    surfaces
}

/// `patch`'s role the catalogue's key table maps `key` to.
fn set_role(patch: &mut SurfaceRolesPatch, key: KdeSchemeKey, value: Value<Color>) {
    let field = match key {
        KdeSchemeKey::BackgroundNormal => &mut patch.background,
        KdeSchemeKey::BackgroundAlternate => &mut patch.background_alternate,
        KdeSchemeKey::ForegroundNormal => &mut patch.foreground,
        KdeSchemeKey::ForegroundInactive => &mut patch.foreground_inactive,
        KdeSchemeKey::ForegroundActive => &mut patch.foreground_active,
        KdeSchemeKey::ForegroundLink => &mut patch.link,
        KdeSchemeKey::ForegroundVisited => &mut patch.link_visited,
        KdeSchemeKey::ForegroundNegative => &mut patch.negative,
        KdeSchemeKey::ForegroundNeutral => &mut patch.neutral,
        KdeSchemeKey::ForegroundPositive => &mut patch.positive,
        KdeSchemeKey::DecorationFocus => &mut patch.decoration_focus,
        KdeSchemeKey::DecorationHover => &mut patch.decoration_hover,
    };
    *field = Some(value);
}

/// The expression `text` states, as a colour value; `None` never for the
/// reader's own texts, which its tests parse.
pub(crate) fn expr(text: &str) -> Option<Value<Color>> {
    Expr::parse(text).ok().map(Value::Expr)
}

/// The input `path` names.
pub(crate) fn input(path: &str) -> Option<InputName> {
    InputName::new(path).ok()
}

/// The colour set `set`'s inactive value of `key` in `ini`, as KColorScheme
/// reads its `Inactive` group: the `[Inactive]` group's value where the
/// set's group has one, a key it lacks the set's default
/// ([`default_value`]), not its active value (`kcolorscheme.cpp:344-357`);
/// else the active value (`:345-351`). So `Header` without a group of its
/// own reads its active values, `Window`'s over `Header`'s table, never
/// `[Colors:Window][Inactive]`.
pub(crate) fn inactive_value(
    ini: &Ini,
    set: SurfaceSet,
    name: &str,
    defaults: &SetDefaults,
    key: KdeSchemeKey,
) -> Color {
    if has_inactive_group(ini, name) {
        color(ini, &inactive_group(name), kconfig_key(key))
            .unwrap_or_else(|| default_value(ini, set, defaults, key))
    } else {
        active_value(ini, set, name, defaults, key)
    }
}

/// Whether `ini` has the colour group `name` with an `[Inactive]` group.
fn has_inactive_group(ini: &Ini, name: &str) -> bool {
    has_group(ini, &group(name)) && has_group(ini, &inactive_group(name))
}

/// The scheme inputs of the scheme file `file` (or of kdeglobals read as
/// one), under `scheme`, into `bases` (S9.1):
///
/// - `input.kde_scheme.groups`, the sets the file has a group for;
///   `inactive_groups`, those whose group has an `[Inactive]` group; and
///   `unset_keys`, the scheme inputs of the groups it has that it leaves
///   unset;
/// - each `kde_scheme.<set>.<key>`, for every set, the file's value, a key
///   it leaves unset, or a whole group, as KColorScheme reads it unset
///   ([`active_value`]);
/// - each `kde_scheme.<set>.inactive.<key>`: for a set `inactive_groups`
///   names, the file's `[Inactive]` value, a key it lacks the set's default
///   ([`inactive_value`]), else `= input.kde_scheme.<set>.<key>`, what
///   KColorScheme reads for a set without one (`kcolorscheme.cpp:345-351`);
/// - each `kde_scheme.wm.<key>`: the file's `[WM]` key where it states one,
///   else the default `applyScheme()` takes over the same file
///   (`colorsapplicator.cpp:287-309`): the `Header` Normal background and
///   foreground for the active keys, and for the inactive ones the inactive
///   `Header` colours as the inactive effects apply to them.
pub(crate) fn scheme_inputs(file: &Ini, scheme: Scheme, bases: &mut Bases) {
    let mut groups = Vec::new();
    let mut inactive_groups = Vec::new();
    let mut unset_keys = Vec::new();
    for (set, name, defaults) in COLOR_GROUPS {
        let has = has_group(file, &group(name));
        let inactive = has_inactive_group(file, name);
        if has {
            groups.push(set);
        }
        if inactive {
            inactive_groups.push(set);
        }
        for key in KdeSchemeKey::ALL {
            let active = format!("kde_scheme.{}.{}", set.name(), key.name());
            let inactive_name = format!("kde_scheme.{}.inactive.{}", set.name(), key.name());
            let stated = color(file, &group(name), kconfig_key(key));
            if has && stated.is_none() {
                unset_keys.extend(input(&active));
            }
            let value = Some(Value::Stated(active_value(file, set, name, defaults, key)));
            let inactive_entry = if inactive {
                if color(file, &inactive_group(name), kconfig_key(key)).is_none() {
                    unset_keys.extend(input(&inactive_name));
                }
                Some(Value::Stated(inactive_value(
                    file, set, name, defaults, key,
                )))
            } else {
                expr(&format!("input.{active}"))
            };
            for (path, value) in [(active, value), (inactive_name, inactive_entry)] {
                if let (Some(name), Some(value)) = (input(&path), value) {
                    bases
                        .inputs
                        .entries
                        .push((name, InputValue::new(SchemeMap::one(scheme, value))));
                }
            }
        }
    }
    for key in KdeWmKey::ALL {
        let value = match color(file, "WM", kconfig_wm_key(key)) {
            Some(color) => Some(Value::Stated(color)),
            None => expr(match key {
                KdeWmKey::ActiveBackground => "input.kde_scheme.header.background_normal",
                KdeWmKey::ActiveForeground => "input.kde_scheme.header.foreground_normal",
                KdeWmKey::InactiveBackground => {
                    "kde_effects(input.kde_scheme.header.inactive.background_normal, \
                     input.kde_scheme.header.inactive.background_normal, inactive, background)"
                }
                KdeWmKey::InactiveForeground => {
                    "kde_effects(input.kde_scheme.header.inactive.foreground_normal, \
                     input.kde_scheme.header.inactive.background_normal, inactive, foreground)"
                }
            }),
        };
        if let (Some(name), Some(value)) = (input(&format!("kde_scheme.wm.{}", key.name())), value)
        {
            bases
                .inputs
                .entries
                .push((name, InputValue::new(SchemeMap::one(scheme, value))));
        }
    }
    *bases.inputs.kde_groups.slot(scheme) = Some(groups);
    *bases.inputs.kde_inactive_groups.slot(scheme) = Some(inactive_groups);
    *bases.inputs.kde_unset_keys.slot(scheme) = Some(unset_keys);
}

/// A KConfig boolean entry as KConfig reads one: false where it is
/// `false`, `no`, `off` or `0` in any case, else true, the empty value
/// included (`kconfig@6.31.0 src/core/kconfiggroup.cpp:249-255`, S4.8's
/// *Unset and empty*); `None` where the key is unset.
pub(crate) fn flag(ini: &Ini, section: &str, key: &str) -> Option<bool> {
    ini.get(section, key).map(|value| {
        let value = value.trim().to_ascii_lowercase();
        !matches!(value.as_str(), "false" | "no" | "off" | "0")
    })
}

/// A KConfig number entry, where it is finite; `None` where it is unset or
/// KConfig cannot convert it, the empty value included, so the caller takes
/// the key's default (`kconfig@6.31.0 src/core/kconfiggroup.cpp:256-268`).
pub(crate) fn number(ini: &Ini, section: &str, key: &str) -> Option<f32> {
    ini.get(section, key)
        .and_then(|value| value.trim().parse::<f32>().ok())
        .filter(|value| value.is_finite())
}

/// A KConfig integer entry.
fn integer(ini: &Ini, section: &str, key: &str) -> Option<i64> {
    ini.get(section, key)
        .and_then(|value| value.trim().parse::<i64>().ok())
}

/// The scheme file's tint and title-bar options (S4.8): `[General]
/// TintFactor`, `None` where it has none (`colorsapplicator.cpp:154-155`),
/// and whether the title bar takes the accent, `TitlebarIsAccentColored`,
/// else `accentActiveTitlebar`, else false (`:152-153`).
pub(crate) fn scheme_options(file: &Ini) -> (Option<f32>, bool) {
    let tint = number(file, "General", "TintFactor");
    let titlebar = flag(file, "General", "TitlebarIsAccentColored")
        .or_else(|| flag(file, "General", "accentActiveTitlebar"))
        .unwrap_or(false);
    (tint, titlebar)
}

/// One colour group's effects as KColorScheme reads kdeglobals'
/// `[ColorEffects:<group>]` (K3.40, K3.50, S4.8): each key's value where it
/// states one, the effects by KColorScheme's enums (`kcolorscheme@6.31.0
/// src/kcolorschemehelpers_p.h:57-79`, `docs/facts/kde/tokens.toml:10885`,
/// `:10940`, `:10995`), a key it leaves unset or KConfig cannot convert,
/// the empty value included, the group's default `defaults`
/// (`kcolorscheme.cpp:36-49`); an empty `Color` is stated as its default
/// too, which KColorScheme reads as an invalid colour, a recorded
/// inexactness (S4.8's *Unset and empty*). An effect number no enum member
/// holds is none ([`Effects`]). Where the disabled group's `Enable` is
/// false (`disabled`) its three effects are `none`, as KColorScheme then
/// applies none (`:39-40`).
fn effects(
    ini: &Ini,
    section: &str,
    defaults: &defaults::EffectDefaults,
    disabled: bool,
) -> Effects {
    let off = disabled && !flag(ini, section, "Enable").unwrap_or(defaults.enable);
    let effect = |key: &str| integer(ini, section, key);
    let intensity = match effect("IntensityEffect") {
        None => Some(defaults.intensity),
        Some(0) => Some(KdeIntensityEffect::None),
        Some(1) => Some(KdeIntensityEffect::Shade),
        Some(2) => Some(KdeIntensityEffect::Darken),
        Some(3) => Some(KdeIntensityEffect::Lighten),
        Some(_) => None,
    };
    let color_effect = match effect("ColorEffect") {
        None => Some(defaults.color),
        Some(0) => Some(KdeColorEffect::None),
        Some(1) => Some(KdeColorEffect::Desaturate),
        Some(2) => Some(KdeColorEffect::Fade),
        Some(3) => Some(KdeColorEffect::Tint),
        Some(_) => None,
    };
    let contrast = match effect("ContrastEffect") {
        None => Some(defaults.contrast),
        Some(0) => Some(KdeContrastEffect::None),
        Some(1) => Some(KdeContrastEffect::Fade),
        Some(2) => Some(KdeContrastEffect::Tint),
        Some(_) => None,
    };
    let amount = |key: &str, default: f32| number(ini, section, key).unwrap_or(default);
    Effects {
        intensity: if off {
            Some(KdeIntensityEffect::None)
        } else {
            intensity
        },
        intensity_amount: amount("IntensityAmount", defaults.intensity_amount),
        color: if off {
            Some(KdeColorEffect::None)
        } else {
            color_effect
        },
        color_amount: amount("ColorAmount", defaults.color_amount),
        color_color: color(ini, section, "Color").unwrap_or_else(|| defaults.color_color()),
        contrast: if off {
            Some(KdeContrastEffect::None)
        } else {
            contrast
        },
        contrast_amount: amount("ContrastAmount", defaults.contrast_amount),
    }
}

/// One colour group's effects as read ([`effects`]): each effect `None`
/// where its number names no member of its enum.
struct Effects {
    /// `IntensityEffect`.
    intensity: Option<KdeIntensityEffect>,
    /// `IntensityAmount`.
    intensity_amount: f32,
    /// `ColorEffect`.
    color: Option<KdeColorEffect>,
    /// `ColorAmount`.
    color_amount: f32,
    /// `Color`.
    color_color: Color,
    /// `ContrastEffect`.
    contrast: Option<KdeContrastEffect>,
    /// `ContrastAmount`.
    contrast_amount: f32,
}

impl Effects {
    /// The effects as `token.kde.<group>` states them: an effect whose
    /// number names no member not stated.
    fn patch(&self) -> KdeEffectsPatch {
        KdeEffectsPatch {
            intensity: self.intensity,
            intensity_amount: Some(Value::Stated(self.intensity_amount)),
            color: self.color,
            color_amount: Some(Value::Stated(self.color_amount)),
            color_color: Some(Value::Stated(self.color_color)),
            contrast: self.contrast,
            contrast_amount: Some(Value::Stated(self.contrast_amount)),
        }
    }

    /// The effects as `kde_effects` takes them: an effect whose number
    /// names no member none, as the evaluator takes an effect token it
    /// cannot name (`expr/eval.rs`, `kde_group_effects`).
    fn group(&self) -> KdeGroupEffects {
        let Color { r, g, b, a } = self.color_color;
        KdeGroupEffects {
            intensity: self.intensity.unwrap_or(KdeIntensityEffect::None),
            intensity_amount: self.intensity_amount,
            color: self.color.unwrap_or(KdeColorEffect::None),
            color_amount: self.color_amount,
            color_color: [r, g, b, a],
            contrast: self.contrast.unwrap_or(KdeContrastEffect::None),
            contrast_amount: self.contrast_amount,
        }
    }
}

/// The group of the disabled colour effects.
const DISABLED_EFFECTS: &str = "ColorEffects:Disabled";

/// The group of the inactive colour effects.
const INACTIVE_EFFECTS: &str = "ColorEffects:Inactive";

/// `[ColorEffects:Inactive] Enable` as KColorScheme reads it, false where
/// unset (`kcolorscheme.cpp:39-40`), read as KConfig reads a bool
/// ([`flag`]), so an empty one, which `applyScheme()` writes where the
/// scheme file lacks the key, is true.
fn inactive_enabled(ini: &Ini) -> bool {
    flag(ini, INACTIVE_EFFECTS, "Enable").unwrap_or(defaults::INACTIVE.enable)
}

/// `token.kde` of the colour effects `ini` holds (S4.8): the disabled and
/// inactive effects ([`effects`]); `inactive_enabled`, `[ColorEffects:Inactive]
/// Enable` ([`inactive_enabled`]); `inactive_change_selection_color`, its
/// `ChangeSelectionColor`, else its `Enable`, else true (`:307`), each read
/// as KConfig reads a bool; and the scheme file's tint and title-bar
/// options `options` ([`scheme_options`]) where a scheme file is in force,
/// none stated where `options` is `None`.
pub(crate) fn kde_tokens(ini: &Ini, options: Option<(Option<f32>, bool)>) -> KdeTokensPatch {
    let enable = flag(ini, INACTIVE_EFFECTS, "Enable");
    KdeTokensPatch {
        disabled: Some(effects(ini, DISABLED_EFFECTS, &defaults::DISABLED, true).patch()),
        inactive: Some(effects(ini, INACTIVE_EFFECTS, &defaults::INACTIVE, false).patch()),
        inactive_enabled: Some(Value::Stated(inactive_enabled(ini))),
        inactive_change_selection_color: Some(Value::Stated(
            flag(ini, INACTIVE_EFFECTS, "ChangeSelectionColor")
                .or(enable)
                .unwrap_or(defaults::INACTIVE_CHANGE_SELECTION_COLOR),
        )),
        tint_factor: options.map(|(tint, _)| tint.map(Value::Stated)),
        titlebar_is_accent_colored: options.map(|(_, titlebar)| Value::Stated(titlebar)),
    }
}

/// The inactive colour effects `ini` holds, as `kde_effects` takes them,
/// and whether they are enabled ([`inactive_enabled`]).
pub(crate) fn inactive_effects(ini: &Ini) -> (KdeGroupEffects, bool) {
    let effects = effects(ini, INACTIVE_EFFECTS, &defaults::INACTIVE, false);
    (effects.group(), inactive_enabled(ini))
}

/// Whether `ini` has a colour group of one of [`COLOR_GROUPS`]' sets,
/// `[Colors:<Set>]` or a subgroup of one, such as
/// `[Colors:Header][Inactive]` (S14, ADD-A142).
pub(crate) fn has_color_group(ini: &Ini) -> bool {
    ini.get_map_ref().keys().any(|section| {
        COLOR_GROUPS.iter().any(|(_, name, _)| {
            let group = group(name);
            section == &group
                || section
                    .strip_prefix(&group)
                    .is_some_and(|rest| rest.starts_with("]["))
        })
    })
}

/// Read the KConfig file `bytes` into `ini`, over the keys it already
/// holds: decoded as UTF-8, each invalid sequence replaced by U+FFFD, then
/// read by KConfig's rules ([`read_kconfig`]; S9.3's *The KDE reader's
/// readings*).
pub(crate) fn read_kconfig_bytes(ini: &mut Ini, bytes: &[u8]) {
    read_kconfig(ini, &String::from_utf8_lossy(bytes));
}

/// Parse KConfig `content` into `ini`, over the keys it already holds.
///
/// The lines KConfig skips are skipped (kconfig `src/core/kconfigini.cpp` at
/// a4e32e9d): a group header without `]` (`:100-112`, the group before it
/// stays current) and an entry with an empty key (`:168-171`). They are the
/// only lines configparser rejects, so one stray line no longer loses the
/// whole file.
pub(crate) fn read_kconfig(ini: &mut Ini, content: &str) {
    let valid: String = content
        .lines()
        .filter(|line| {
            let line = line.trim();
            !(line.starts_with('[') && !line.contains(']')) && !line.starts_with('=')
        })
        .flat_map(|line| [line, "\n"])
        .collect();
    // Cannot fail: the filter above drops every line configparser rejects.
    let _ = ini.read_and_append(valid);
}

/// Create a configparser Ini instance configured for KDE files.
///
/// Uses case-sensitive mode and equals-only delimiter to correctly
/// handle KDE's PascalCase keys and colon-containing section names.
/// Only a line starting with `#` is a comment, as in KConfig (kconfig
/// `src/core/kconfigini.cpp:95-98` at a4e32e9d): a `#` or `;` inside a value,
/// as in `ColorScheme=My#Scheme`, is part of it.
pub(crate) fn create_kde_parser() -> Ini {
    let tmp = Ini::new_cs();
    let mut defaults = tmp.defaults();
    defaults.delimiters = vec!['='];
    defaults.comment_symbols = vec!['#'];
    defaults.enable_inline_comments = false;
    Ini::new_from_defaults(defaults)
}
