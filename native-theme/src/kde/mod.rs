//! The KDE reader (the `kde` feature, Linux): kdeglobals as KConfig cascades
//! it, the colour scheme file it names, `kcmfonts` and the icon theme's
//! sizes, into a [`ReaderState`] and a reader layer (S8, S9.3, S9.5); and
//! the icon theme `detect` reports.
//!
//! kdeglobals holds what Plasma's `applyScheme()` wrote from the scheme file
//! and the accent: the results, which the reader states under the scheme it
//! read only, in its `for_preset.kde-breeze` part (S9.3): each colour
//! group's roles, the inactive input roles, the `[WM]` keyed roles, the
//! colour effects and contrast as a `Tokens` rule under that scheme (S4.8).
//! The scheme file `[General] ColorScheme` names gives the scheme inputs
//! (`input.kde_scheme.*`, their three lists) and the scheme's tint and
//! title-bar options, the values before Plasma applied the accent (S9.1).
//! The accent, the conditions and the fonts are the layer's portable part,
//! which any preset takes (S9.5).
//!
//! The kdeglobals files are `$XDG_CONFIG_DIRS/kdeglobals`
//! (`/etc/xdg/kdeglobals`), then `kdedefaults/kdeglobals`, the defaults
//! Plasma keeps beneath the user's file, then the user's `kdeglobals`, each
//! in the user's config directory (`$XDG_CONFIG_HOME`, else `~/.config`).
//!
//! The font entries are parsed by the clean room's Qt font parser,
//! `qfont_from_string` (the 0.8.0 plan's D10), its code placed in
//! `src/cleanroom_readers/`, unchanged but for its panic-free arithmetic.
//!
//! # Sources
//!
//! The files the reader reads that the user's settings write, each a path in
//! the user's config directory; the watcher observes each of them:
//!
//! - `kdeglobals`: the colours Plasma applied, the accent, the colour
//!   effects and contrast, the colour scheme's name, the fonts, the icon
//!   theme and the animation speed.
//! - `kdedefaults/kdeglobals`: the defaults Plasma keeps beneath the user's
//!   file, read under it as KConfig cascades them.
//! - `kcmfonts`: `forceFontDPI`, on X11, the file the fonts KCM writes it
//!   to.
//!
//! # Sources not watched
//!
//! - `$XDG_CONFIG_DIRS/kdeglobals` (`/etc/xdg/kdeglobals`): the system's
//!   files, which the distribution or an administrator writes, not the
//!   user's settings.
//! - `color-schemes/<ColorScheme>.colors` in the data directories: the
//!   scheme file kdeglobals names, which a scheme's package writes; Plasma
//!   applies a scheme by writing kdeglobals, which is watched.
//! - `Xft.dpi` (`xrdb -query`): the rest of the font DPI, read from the X
//!   server, not from a settings file.
//! - The icon theme's `index.theme`: its sizes, which the icon theme's
//!   package writes.
//! - `$XDG_SESSION_TYPE`: the session type, which decides whether the font
//!   DPI is read, fixed for a session.

mod font;
#[path = "../cleanroom_readers/qfont_from_string.rs"]
#[allow(
    dead_code,
    reason = "the clean room's parser, placed unchanged (D10): the fields the reader does not keep are read by its vector test alone"
)]
#[allow(
    clippy::collapsible_if,
    reason = "the clean room's parser, its style kept as written (D10): a style lint only"
)]
mod qfont_from_string;

use configparser::ini::Ini;

use crate::blocks::BlockPatch;
use crate::blocks::icon::IconStylePatch;
use crate::color::Color;
use crate::error::ReaderFailure;
use crate::expr::Expr;
use crate::inputs::{InputName, InputValue, KdeSchemeKey, KdeWmKey};
use crate::key::Key;
use crate::layer::{Bases, Layer};
use crate::presets::PresetId;
use crate::reader::Reading;
use crate::resolve::ReaderState;
use crate::roles::{ColorRolesPatch, SurfaceRolesPatch, SurfaceSet};
use crate::selector::{Condition, Rule, RuleBody, Scheme, Selector, SetMatch};
use crate::tokens::{
    IconSizeTokensPatch, IconTokensPatch, KdeColorEffect, KdeContrastEffect, KdeEffectsPatch,
    KdeIntensityEffect, KdeTokensPatch, MotionTokensPatch, TextRolesPatch, TextTokensPatch,
    TokensPatch,
};
use crate::units::{Px, Scalar};
use crate::value::{SchemeMap, SurfaceMap, Value};

/// The text-scaling factor of unscaled text, S16's no-reader state's: the
/// identity of S9.4's scaling, not a platform value. Plasma scales text by
/// the font sizes it writes (K3.46).
const UNSCALED: f32 = 1.0;

/// The colour groups KColorScheme reads and `applyScheme()` writes, each the
/// surface set `kde-breeze` declares for it and its group's name,
/// `[Colors:<name>]` (`colorsapplicator.cpp:157-163`, S9.1).
const COLOR_GROUPS: [(SurfaceSet, &str); 7] = [
    (SurfaceSet::View, "View"),
    (SurfaceSet::Window, "Window"),
    (SurfaceSet::Button, "Button"),
    (SurfaceSet::Selection, "Selection"),
    (SurfaceSet::Tooltip, "Tooltip"),
    (SurfaceSet::Complementary, "Complementary"),
    (SurfaceSet::Header, "Header"),
];

/// Qt's `transparent`, `#00000000`, the `AccentColor` `applyScheme()` reads
/// as no accent (`plasma-workspace@6.8.2 kcms/colors/colorsapplicator.cpp:130`,
/// S9.3).
const TRANSPARENT: Color = Color {
    r: 0.0,
    g: 0.0,
    b: 0.0,
    a: 0.0,
};

/// `[KDE] contrast` where kdeglobals sets none: KColorScheme's default,
/// `g.readEntry("contrast", 7)` (`kcolorscheme@6.31.0
/// src/kcolorscheme.cpp:519-527`, `docs/facts/kde/tokens.toml:11171`).
const DEFAULT_CONTRAST: f32 = 7.0;

/// `[KDE] frameContrast` where kdeglobals sets none: KColorScheme's
/// default, `const qreal default_value = 0.2;` (`kcolorscheme@6.31.0
/// src/kcolorscheme.cpp:529-539`, `docs/facts/kde/tokens.toml:11198`).
const DEFAULT_FRAME_CONTRAST: f32 = 0.2;

/// The scheme file `[General] ColorScheme` names, as the reader found it.
pub(crate) enum SchemeFile {
    /// kdeglobals names no scheme: Plasma's default, Breeze Light
    /// (`startplasma.cpp:452`), whose inputs are the preset's (S9.3).
    Unnamed,
    /// The file, read as KConfig reads it.
    Read(Ini),
    /// The file named cannot be found or read: why. The reader takes the
    /// scheme inputs from kdeglobals itself (S9.3).
    Unreadable(String),
}

/// What the KDE reader read, in plain types: built from the files in
/// production, directly in tests.
pub(crate) struct KdeData {
    /// kdeglobals as KConfig cascades it.
    pub(crate) kdeglobals: Ini,
    /// The scheme file kdeglobals names.
    pub(crate) scheme_file: SchemeFile,
    /// The font DPI: `forceFontDPI`, else `Xft.dpi`, on an X11 session
    /// ([`read_font_dpi`]); `None` elsewhere.
    pub(crate) font_dpi: Option<f32>,
    /// The icon theme's `index.theme`, where it was found.
    pub(crate) icon_index: Option<Ini>,
}

/// The reader's state and layer from what it read, its own preset's part
/// under `own` (S9.5). Zero I/O.
///
/// - The state: the scheme ([`scheme`]); animations off and reduced motion
///   where `[KDE] AnimationDurationFactor` is 0 (K3.30, K3.90); the font DPI;
///   text unscaled ([`UNSCALED`]).
/// - The common bases: `input.accent` ([`accent`]); the text-role slots
///   `body` (`[General] font`), `monospace` (`fixed`) and `caption`
///   (`smallestReadableFont`), their point sizes converted by the font DPI,
///   else `own`'s `token.text.font_dpi` (S9.4).
/// - `for_preset.<own>`, under the scheme read: the results, the scheme
///   inputs and their lists, and the `#00000000` accent where kdeglobals
///   sets none; beside them the icon theme and its sizes, the animation
///   speed factor and the keyed text styles `toolbar` (`toolBarFont`) and
///   `menu` (`menuFont`).
/// - A `Tokens` rule under `scheme:<s>` and `preset:<own>`: the colour
///   effects and contrast kdeglobals holds, the scheme file's tint and
///   title-bar options (S4.8).
///
/// The status's fallback names the scheme file where it could not be read
/// and kdeglobals stood in for it (S9.3).
pub(crate) fn build_reading(data: &KdeData, own: PresetId) -> Reading {
    let ini = &data.kdeglobals;
    let scheme = scheme(ini);
    let mut state = ReaderState::new(scheme, UNSCALED);
    let factor = animation_duration_factor(ini);
    state.animations_off = factor == Some(0.0);
    state.reduced_motion = state.animations_off;
    state.font_dpi = data.font_dpi;

    let mut layer = Layer::default();
    let mut own_bases = Bases::default();

    let (common_accent, own_accent) = accent(ini, scheme);
    if let Some(value) = common_accent {
        layer
            .bases
            .inputs
            .entries
            .push((InputName::accent(), value));
    }
    if let Some(value) = own_accent {
        own_bases.inputs.entries.push((InputName::accent(), value));
    }

    let font_dpi = data.font_dpi.or_else(|| own.font_dpi());
    let font = |key: &str| {
        ini.get("General", key)
            .filter(|text| !text.is_empty())
            .and_then(|text| font::font_patch(&text, font_dpi))
            .map(BlockPatch::Fields)
    };
    let roles = TextRolesPatch {
        body: font("font"),
        monospace: font("fixed"),
        caption: font("smallestReadableFont"),
        ..TextRolesPatch::default()
    };
    if roles != TextRolesPatch::default() {
        layer.bases.tokens.text = Some(Box::new(TextTokensPatch {
            roles: Some(Box::new(roles)),
            ..TextTokensPatch::default()
        }));
    }

    *own_bases.colors.slot(scheme) = Some(results(ini));
    own_bases
        .inputs
        .entries
        .extend(inactive_results(ini, scheme));

    let fallback = match &data.scheme_file {
        SchemeFile::Unnamed => None,
        SchemeFile::Read(file) => {
            scheme_inputs(file, scheme, &mut own_bases);
            None
        }
        SchemeFile::Unreadable(why) => {
            scheme_inputs(ini, scheme, &mut own_bases);
            Some(ReaderFailure::Platform(why.clone()))
        }
    };

    let keyed: Vec<(Key, BlockPatch<_>)> = [
        (crate::key!("toolbar"), font("toolBarFont")),
        (crate::key!("menu"), font("menuFont")),
    ]
    .into_iter()
    .filter_map(|(key, patch)| Some((key, patch?)))
    .collect();
    if !keyed.is_empty() {
        own_bases.tokens.text = Some(Box::new(TextTokensPatch {
            keyed: Some(keyed),
            ..TextTokensPatch::default()
        }));
    }
    let theme = icon_theme(ini);
    let sizes = data.icon_index.as_ref().and_then(icon_sizes);
    if theme.is_some() || sizes.is_some() {
        own_bases.tokens.icons = Some(Box::new(IconTokensPatch {
            theme: theme.map(Value::Stated),
            sizes,
            ..IconTokensPatch::default()
        }));
    }
    if let Some(factor) = factor {
        own_bases.tokens.motion = Some(MotionTokensPatch {
            speed_factor: Some(Value::Stated(factor)),
            ..MotionTokensPatch::default()
        });
    }
    layer.for_preset.push((own, own_bases));

    let scheme_options = match &data.scheme_file {
        SchemeFile::Unnamed => None,
        SchemeFile::Read(file) => Some(scheme_options(file)),
        SchemeFile::Unreadable(_) => Some((None, false)),
    };
    let tokens = scheme_tokens(ini, scheme_options);
    let selector = Selector {
        conditions: SetMatch {
            all_of: vec![Condition::Scheme(scheme), Condition::Preset(own)],
            none_of: Vec::new(),
        },
        ..Selector::default()
    };
    layer
        .rules
        .push(Rule::new(selector, RuleBody::Tokens(Box::new(tokens))));

    Reading {
        state,
        layer,
        fallback,
    }
}

/// The scheme kdeglobals' colours are: dark where `[Colors:Window]
/// BackgroundNormal` is dark by its BT.601 luminance, else light, as 0.7.0's
/// reader decided (`2796b26e:native-theme/src/kde/mod.rs:469-480`).
fn scheme(ini: &Ini) -> Scheme {
    let dark = color(ini, "Colors:Window", "BackgroundNormal").is_some_and(|bg| {
        let [r, g, b, _] = bg.to_srgb8_clamped();
        0.299 * f32::from(r) + 0.587 * f32::from(g) + 0.114 * f32::from(b) < 128.0
    });
    if dark { Scheme::Dark } else { Scheme::Light }
}

/// `[KDE] AnimationDurationFactor`, where it is a finite number not below
/// zero.
fn animation_duration_factor(ini: &Ini) -> Option<f32> {
    ini.get("KDE", "AnimationDurationFactor")
        .and_then(|value| value.trim().parse::<f32>().ok())
        .filter(|factor| factor.is_finite() && *factor >= 0.0)
}

/// The reader's `input.accent` (S9.3): its common value, and its value in
/// the `for_preset.kde-breeze` part.
///
/// - `[General] AccentColor` set and not Qt's `transparent`: that colour, in
///   the common bases at both schemes, the accent the user chose.
/// - Absent or `transparent`, Plasma's default "Accent color from color
///   scheme": in the common bases the scheme's own accent as KDE publishes
///   it, the `Selection` set's `BackgroundNormal`, under the scheme read
///   only; in the own part `#00000000` at that scheme.
fn accent(ini: &Ini, scheme: Scheme) -> (Option<InputValue>, Option<InputValue>) {
    let chosen = color(ini, "General", "AccentColor").filter(|color| *color != TRANSPARENT);
    match chosen {
        Some(accent) => {
            let accent = Value::Stated(accent);
            (
                Some(InputValue::new(SchemeMap::new(accent.clone(), accent))),
                None,
            )
        }
        None => {
            let highlight = color(ini, "Colors:Selection", "BackgroundNormal")
                .map(|color| InputValue::new(SchemeMap::one(scheme, Value::Stated(color))));
            let none = InputValue::new(SchemeMap::one(scheme, Value::Stated(TRANSPARENT)));
            (highlight, Some(none))
        }
    }
}

/// A KConfig colour entry, `r,g,b` or `r,g,b,a`, each channel an integer
/// `0..=255`; `None` for any other value.
pub(crate) fn parse_color(value: &str) -> Option<Color> {
    let channels: Vec<u8> = value
        .split(',')
        .map(|channel| channel.trim().parse::<u8>().ok())
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
fn color(ini: &Ini, section: &str, key: &str) -> Option<Color> {
    ini.get(section, key).as_deref().and_then(parse_color)
}

/// Whether `ini` has the group `section`.
fn has_group(ini: &Ini, section: &str) -> bool {
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
const fn kconfig_wm_key(key: KdeWmKey) -> &'static str {
    match key {
        KdeWmKey::ActiveBackground => "activeBackground",
        KdeWmKey::ActiveForeground => "activeForeground",
        KdeWmKey::InactiveBackground => "inactiveBackground",
        KdeWmKey::InactiveForeground => "inactiveForeground",
    }
}

/// The colour group `name`'s `key` in `ini` as KColorScheme reads it over a
/// file with the key unset: `Header`'s from `[Colors:Window]` where
/// `[Colors:Header]` lacks it (`kcolorscheme.cpp:330-333`, S9.1); `None`
/// for another set, whose unset key is KColorScheme's default, which no
/// fact states yet.
fn active_value(ini: &Ini, set: SurfaceSet, name: &str, key: KdeSchemeKey) -> Option<Color> {
    color(ini, &group(name), kconfig_key(key)).or_else(|| {
        (set == SurfaceSet::Header)
            .then(|| color(ini, &group("Window"), kconfig_key(key)))
            .flatten()
    })
}

/// The colour role patch of kdeglobals' results: each colour group's keys,
/// every set's roles the catalogue's key table maps them to
/// ([`KdeSchemeKey::role`]), `Header`'s from `[Colors:Window]` where it
/// lacks one; and the `[WM]` keyed roles (K2.11), each key kdeglobals' where
/// it states one, else the references Breeze falls back to, the palette's
/// `Highlight` and `HighlightedText`, active for the active title bar and
/// disabled for the inactive one (`breeze@c97883a
/// kstyle/breezehelper.cpp:98-102`, the widget catalogue's `[WM]` roles).
fn results(ini: &Ini) -> ColorRolesPatch {
    let mut surfaces = SurfaceMap::default();
    for (set, name) in COLOR_GROUPS {
        let mut patch = SurfaceRolesPatch::default();
        for key in KdeSchemeKey::ALL {
            if let Some(color) = active_value(ini, set, name, key) {
                set_role(&mut patch, key, Value::Stated(color));
            }
        }
        if patch != SurfaceRolesPatch::default() {
            surfaces.insert(set, BlockPatch::Fields(patch));
        }
    }
    let keyed: Vec<(Key, Value<Color>)> = KdeWmKey::ALL
        .into_iter()
        .filter_map(|key| {
            let role = Key::new(&format!("wm_{}", key.name())).ok()?;
            let value = match color(ini, "WM", kconfig_wm_key(key)) {
                Some(color) => Value::Stated(color),
                None => expr(match key {
                    KdeWmKey::ActiveBackground => "active(role.selection.background)",
                    KdeWmKey::ActiveForeground => "active(role.selection.foreground)",
                    KdeWmKey::InactiveBackground => "role.selection.disabled.background",
                    KdeWmKey::InactiveForeground => "role.selection.disabled.foreground",
                })?,
            };
            Some((role, value))
        })
        .collect();
    ColorRolesPatch {
        surfaces: Some(surfaces),
        keyed: Some(keyed),
        ..ColorRolesPatch::default()
    }
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
fn expr(text: &str) -> Option<Value<Color>> {
    Expr::parse(text).ok().map(Value::Expr)
}

/// The input `path` names.
fn input(path: &str) -> Option<InputName> {
    InputName::new(path).ok()
}

/// kdeglobals' inactive results, the inactive input roles
/// `input.inactive.<set>.<key>` under `scheme` (K3.51, S9.1): each key of
/// the group's `[Inactive]` group where it has one, else the group's active
/// value, as KColorScheme reads a set without one (`kcolorscheme.cpp:344-352`);
/// a key an `[Inactive]` group lacks is KColorScheme's default, which no fact
/// states yet, and is not stated. `Header` without a group of its own reads
/// `Window`'s.
fn inactive_results(ini: &Ini, scheme: Scheme) -> Vec<(InputName, InputValue)> {
    let mut entries = Vec::new();
    for (set, name) in COLOR_GROUPS {
        let source = if set == SurfaceSet::Header && !has_group(ini, &group(name)) {
            "Window"
        } else {
            name
        };
        let inactive = has_group(ini, &inactive_group(source));
        for key in KdeSchemeKey::ALL {
            let value = if inactive {
                color(ini, &inactive_group(source), kconfig_key(key))
            } else {
                active_value(ini, set, name, key)
            };
            let name = input(&format!("inactive.{}.{}", set.name(), key.name()));
            if let (Some(name), Some(value)) = (name, value) {
                entries.push((
                    name,
                    InputValue::new(SchemeMap::one(scheme, Value::Stated(value))),
                ));
            }
        }
    }
    entries
}

/// The scheme inputs of the scheme file `file` (or of kdeglobals read as
/// one), under `scheme`, into `bases` (S9.1):
///
/// - `input.kde_scheme.groups`, the sets the file has a group for;
///   `inactive_groups`, those whose group has an `[Inactive]` group; and
///   `unset_keys`, the scheme inputs of the groups it has that it leaves
///   unset;
/// - each `kde_scheme.<set>.<key>` the file states, `Header`'s from
///   `[Colors:Window]` where it lacks one; a key it leaves unset in
///   another set is KColorScheme's default, which no fact states yet, so
///   the layers beneath keep theirs there;
/// - each `kde_scheme.<set>.inactive.<key>`: the file's `[Inactive]` value
///   for a set `inactive_groups` names (unset likewise), else
///   `= input.kde_scheme.<set>.<key>`, what KColorScheme reads for a set
///   without one (`kcolorscheme.cpp:345-351`);
/// - each `kde_scheme.wm.<key>`: the file's `[WM]` key where it states one,
///   else the default `applyScheme()` takes over the same file
///   (`colorsapplicator.cpp:287-309`): the `Header` Normal background and
///   foreground for the active keys, and for the inactive ones the inactive
///   `Header` colours as the inactive effects apply to them.
fn scheme_inputs(file: &Ini, scheme: Scheme, bases: &mut Bases) {
    let mut groups = Vec::new();
    let mut inactive_groups = Vec::new();
    let mut unset_keys = Vec::new();
    for (set, name) in COLOR_GROUPS {
        let has = has_group(file, &group(name));
        let inactive = has && has_group(file, &inactive_group(name));
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
            let value = active_value(file, set, name, key).map(Value::Stated);
            let inactive_value = if inactive {
                let stated = color(file, &inactive_group(name), kconfig_key(key));
                if stated.is_none() {
                    unset_keys.extend(input(&inactive_name));
                }
                stated.map(Value::Stated)
            } else {
                expr(&format!("input.{active}"))
            };
            for (path, value) in [(active, value), (inactive_name, inactive_value)] {
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

/// A KConfig boolean entry, `true` or `false`.
fn flag(ini: &Ini, section: &str, key: &str) -> Option<bool> {
    ini.get(section, key)
        .and_then(|value| value.trim().parse::<bool>().ok())
}

/// A KConfig number entry, where it is finite.
fn number(ini: &Ini, section: &str, key: &str) -> Option<f32> {
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
fn scheme_options(file: &Ini) -> (Option<f32>, bool) {
    let tint = number(file, "General", "TintFactor");
    let titlebar = flag(file, "General", "TitlebarIsAccentColored")
        .or_else(|| flag(file, "General", "accentActiveTitlebar"))
        .unwrap_or(false);
    (tint, titlebar)
}

/// One colour group's effects as kdeglobals' `[ColorEffects:<group>]`
/// holds them (K3.40, K3.50, S4.8): each key's value where it states one,
/// the effects by KColorScheme's enums (`kcolorscheme@6.31.0
/// src/kcolorschemehelpers_p.h:57-79`, `docs/facts/kde/tokens.toml:10792`,
/// `:10837`, `:10882`); a key it leaves unset is KColorScheme's default,
/// which no fact states yet, so the layers beneath keep theirs. Where
/// `[ColorEffects:Disabled] Enable` is false the disabled group's three
/// effects are `none`, as KColorScheme then applies none
/// (`kcolorscheme.cpp:39-40`).
fn effects(ini: &Ini, section: &str, disabled: bool) -> KdeEffectsPatch {
    let off = disabled && flag(ini, section, "Enable") == Some(false);
    let intensity = integer(ini, section, "IntensityEffect").and_then(|v| match v {
        0 => Some(KdeIntensityEffect::None),
        1 => Some(KdeIntensityEffect::Shade),
        2 => Some(KdeIntensityEffect::Darken),
        3 => Some(KdeIntensityEffect::Lighten),
        _ => None,
    });
    let color_effect = integer(ini, section, "ColorEffect").and_then(|v| match v {
        0 => Some(KdeColorEffect::None),
        1 => Some(KdeColorEffect::Desaturate),
        2 => Some(KdeColorEffect::Fade),
        3 => Some(KdeColorEffect::Tint),
        _ => None,
    });
    let contrast = integer(ini, section, "ContrastEffect").and_then(|v| match v {
        0 => Some(KdeContrastEffect::None),
        1 => Some(KdeContrastEffect::Fade),
        2 => Some(KdeContrastEffect::Tint),
        _ => None,
    });
    let amount = |key: &str| number(ini, section, key).map(Value::Stated);
    KdeEffectsPatch {
        intensity: if off {
            Some(KdeIntensityEffect::None)
        } else {
            intensity
        },
        intensity_amount: amount("IntensityAmount"),
        color: if off {
            Some(KdeColorEffect::None)
        } else {
            color_effect
        },
        color_amount: amount("ColorAmount"),
        color_color: color(ini, section, "Color").map(Value::Stated),
        contrast: if off {
            Some(KdeContrastEffect::None)
        } else {
            contrast
        },
        contrast_amount: amount("ContrastAmount"),
    }
}

/// The scheme's tokens the reader states as a `Tokens` rule under the
/// scheme it read (S4.8): `token.kde` from kdeglobals' colour effects, with
/// the scheme file's tint and title-bar options `options` where a scheme
/// file is in force (kdeglobals standing in for an unreadable one gives
/// none and false, S9.3); and `token.params.color.contrast` and
/// `frame_contrast`, kdeglobals' `[KDE] contrast` and `frameContrast` as
/// stored, else KColorScheme's defaults.
///
/// `inactive_enabled` is `[ColorEffects:Inactive] Enable`, false where unset
/// (`kcolorscheme.cpp:39-40`); `inactive_change_selection_color` its
/// `ChangeSelectionColor`, else its `Enable`, else true (`:307`).
fn scheme_tokens(ini: &Ini, options: Option<(Option<f32>, bool)>) -> TokensPatch {
    let inactive_section = "ColorEffects:Inactive";
    let enable = flag(ini, inactive_section, "Enable");
    let kde = KdeTokensPatch {
        disabled: Some(effects(ini, "ColorEffects:Disabled", true)),
        inactive: Some(effects(ini, inactive_section, false)),
        inactive_enabled: Some(Value::Stated(enable.unwrap_or(false))),
        inactive_change_selection_color: Some(Value::Stated(
            flag(ini, inactive_section, "ChangeSelectionColor")
                .or(enable)
                .unwrap_or(true),
        )),
        tint_factor: options.map(|(tint, _)| tint.map(Value::Stated)),
        titlebar_is_accent_colored: options.map(|(_, titlebar)| Value::Stated(titlebar)),
    };
    let contrast = number(ini, "KDE", "contrast").unwrap_or(DEFAULT_CONTRAST);
    let frame_contrast = number(ini, "KDE", "frameContrast").unwrap_or(DEFAULT_FRAME_CONTRAST);
    TokensPatch {
        kde: Some(BlockPatch::Fields(kde)),
        params: Some(vec![(
            crate::key!("color"),
            vec![
                (
                    crate::key!("contrast"),
                    Value::Stated(Scalar::Number(contrast)),
                ),
                (
                    crate::key!("frame_contrast"),
                    Value::Stated(Scalar::Number(frame_contrast)),
                ),
            ],
        )]),
        ..TokensPatch::default()
    }
}

/// The icon sizes of an icon theme's `index.theme` (K3.63): each typed
/// context's size from the `[Icon Theme]` key its preset's fact read
/// (`docs/facts/kde/tokens.toml`), `toolbar` `ToolbarDefault`, `small`
/// `SmallDefault`, `large` `DesktopDefault`, `dialog` `DialogDefault`,
/// `panel` `PanelDefault`, the default size KIconTheme reads for each group
/// (`kiconthemes@6.31.0 src/kicontheme.cpp:469-472`); a key the theme
/// omits or that is no whole number above zero is KIconTheme's fallback for
/// the group, which no fact states, and keeps the preset's. `None` where it
/// states none.
fn icon_sizes(index: &Ini) -> Option<IconSizeTokensPatch> {
    let size = |key: &str| {
        index
            .get("Icon Theme", key)
            .and_then(|value| value.trim().parse::<u16>().ok())
            .filter(|size| *size > 0)
            .map(|size| {
                BlockPatch::Fields(IconStylePatch {
                    size: Some(Value::Stated(Px(f32::from(size)))),
                    ..IconStylePatch::default()
                })
            })
    };
    let sizes = IconSizeTokensPatch {
        toolbar: size("ToolbarDefault"),
        small: size("SmallDefault"),
        large: size("DesktopDefault"),
        dialog: size("DialogDefault"),
        panel: size("PanelDefault"),
        ..IconSizeTokensPatch::default()
    };
    (sizes != IconSizeTokensPatch::default()).then_some(sizes)
}

/// Reads the KDE settings, its own preset's part under `own`: kdeglobals
/// as KConfig cascades it ([`read_kconfig_files`] of [`kdeglobals_files`]),
/// the scheme file it names
/// ([`read_scheme_file`]), the font DPI ([`read_font_dpi`]) and the icon
/// theme's `index.theme`.
///
/// Fails only where no kdeglobals file can be read
/// (`ReaderFailure::Platform`, S9.3); a scheme file that cannot be found or
/// read is the status's fallback, kdeglobals standing in for it.
pub(crate) fn read(own: PresetId) -> Result<Reading, ReaderFailure> {
    let files = kdeglobals_files();
    let kdeglobals = read_kconfig_files(&files).ok_or_else(|| {
        let listed = files
            .iter()
            .map(|file| file.display().to_string())
            .collect::<Vec<_>>()
            .join(", ");
        ReaderFailure::Platform(format!("no kdeglobals file is readable: {listed}"))
    })?;
    let data_dirs = data_dirs();
    let scheme_file = read_scheme_file(&kdeglobals, &data_dirs);
    let icon_index = icon_theme(&kdeglobals).and_then(|theme| {
        let bases = icon_base_dirs(&data_dirs);
        crate::icons::theme_index(&theme, &bases).map(|text| {
            let mut ini = create_kde_parser();
            read_kconfig(&mut ini, &text);
            ini
        })
    });
    let data = KdeData {
        font_dpi: read_font_dpi(crate::detect::session_type().as_deref()),
        kdeglobals,
        scheme_file,
        icon_index,
    };
    Ok(build_reading(&data, own))
}

/// The data directories Qt's `GenericDataLocation` lists: the data home
/// (`$XDG_DATA_HOME`, else `~/.local/share`), then each data dir
/// (`$XDG_DATA_DIRS`), as KDE locates its colour schemes and icon themes.
fn data_dirs() -> Vec<std::path::PathBuf> {
    crate::icons::xdg_data_home(
        std::env::var_os("XDG_DATA_HOME").as_deref(),
        std::env::home_dir().as_deref(),
    )
    .into_iter()
    .chain(crate::icons::xdg_data_dirs(
        std::env::var_os("XDG_DATA_DIRS").as_deref(),
    ))
    .collect()
}

/// The icon base directories KIconTheme searches for a theme's index.theme,
/// in its order: each data directory joined with `icons` (kiconthemes
/// `src/kicontheme.cpp:382` at v6.31.0, the first holding the theme's
/// index.theme winning, `:395-405`), as 0.7.0's reader searched them
/// (`2796b26e:native-theme/src/kde/mod.rs:208-234`).
fn icon_base_dirs(data_dirs: &[std::path::PathBuf]) -> Vec<std::path::PathBuf> {
    data_dirs.iter().map(|dir| dir.join("icons")).collect()
}

/// The scheme file kdeglobals' `[General] ColorScheme` names:
/// `color-schemes/<name>.colors` in the first of `data_dirs` that holds it,
/// read as KConfig reads it; [`SchemeFile::Unnamed`] where kdeglobals names
/// none, [`SchemeFile::Unreadable`] where no data directory holds a readable
/// one, the reason naming the scheme and the files.
pub(crate) fn read_scheme_file(kdeglobals: &Ini, data_dirs: &[std::path::PathBuf]) -> SchemeFile {
    let Some(name) = kdeglobals
        .get("General", "ColorScheme")
        .filter(|name| !name.is_empty())
    else {
        return SchemeFile::Unnamed;
    };
    if !crate::icons::is_theme_name(&name) {
        return SchemeFile::Unreadable(format!(
            "the colour scheme `{name}` names no scheme file; kdeglobals stands in for it"
        ));
    }
    let files: Vec<std::path::PathBuf> = data_dirs
        .iter()
        .map(|dir| dir.join("color-schemes").join(format!("{name}.colors")))
        .collect();
    match files.iter().find_map(|file| std::fs::read(file).ok()) {
        Some(bytes) => {
            let mut ini = create_kde_parser();
            read_kconfig(&mut ini, &String::from_utf8_lossy(&bytes));
            SchemeFile::Read(ini)
        }
        None => SchemeFile::Unreadable(format!(
            "the colour scheme `{name}` has no readable scheme file ({}); kdeglobals stands in for it",
            files
                .iter()
                .map(|file| file.display().to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )),
    }
}

/// The font DPI the KDE reader reports (S9.4, K3.46): on an X11 session
/// `kcmfonts`' `forceFontDPI` where it is set ([`force_font_dpi`]), else the
/// X server's `Xft.dpi` ([`crate::detect::xft_dpi`]); `None` on any other
/// session, where Plasma has no font DPI setting and the preset's
/// `token.text.font_dpi` applies (`docs/facts/kde/tokens.toml:7937`).
fn read_font_dpi(session_type: Option<&str>) -> Option<f32> {
    if session_type != Some("x11") {
        return None;
    }
    crate::detect::xdg_config_home()
        .and_then(|dir| kcmfonts_key_in(&dir, "General", "forceFontDPI"))
        .as_deref()
        .and_then(force_font_dpi)
        .or_else(crate::detect::xft_dpi)
}

/// A `forceFontDPI` value as a font DPI: Plasma declares the key a UInt
/// whose default 0 means unset, so it is parsed as a `u32` and 0 is `None`,
/// as is anything that is not a `u32` (0.7.0's reading,
/// `2796b26e:native-theme/src/detect.rs:696-703`).
fn force_font_dpi(value: &str) -> Option<f32> {
    let dpi = value.trim().parse::<u32>().ok()?;
    (dpi > 0).then_some(dpi as f32)
}

/// [`kcmfonts`]'s `section` `key` in the config directory `config_dir`, the
/// file read as KConfig reads it ([`read_kconfig`]), its bytes decoded
/// lossily.
fn kcmfonts_key_in(config_dir: &std::path::Path, section: &str, key: &str) -> Option<String> {
    let bytes = std::fs::read(config_dir.join("kcmfonts")).ok()?;
    let mut ini = create_kde_parser();
    read_kconfig(&mut ini, &String::from_utf8_lossy(&bytes));
    ini.get(section, key)
}

/// The icon theme `ini` names: `[Icons] Theme`, `None` where it is absent or
/// empty.
fn icon_theme(ini: &Ini) -> Option<String> {
    ini.get("Icons", "Theme").filter(|s| !s.is_empty())
}

/// The icon theme of the kdeglobals files ([`kdeglobals_files`]), for
/// `detect`'s icon-theme detection; the error says why
/// there is none.
pub(crate) fn kdeglobals_icon_theme() -> Result<String, String> {
    kconfig_files_icon_theme(&kdeglobals_files())
}

/// [`icon_theme`] of the KConfig files `files` layered
/// ([`read_kconfig_files`]); the error names the files.
fn kconfig_files_icon_theme(files: &[std::path::PathBuf]) -> Result<String, String> {
    let listed = || {
        files
            .iter()
            .map(|file| file.display().to_string())
            .collect::<Vec<_>>()
            .join(", ")
    };
    let ini =
        read_kconfig_files(files).ok_or_else(|| format!("no file is readable: {}", listed()))?;
    icon_theme(&ini).ok_or_else(|| format!("`[Icons] Theme` is absent or empty in {}", listed()))
}

/// The kdeglobals files KConfig reads, lowest priority first
/// ([`kdeglobals_files_pure`] for `$XDG_CONFIG_DIRS` and [`kdeglobals_path`]).
fn kdeglobals_files() -> Vec<std::path::PathBuf> {
    kdeglobals_files_pure(
        std::env::var("XDG_CONFIG_DIRS").ok().as_deref(),
        kdeglobals_path(),
    )
}

/// `kdeglobals` in each of `config_dirs` (`$XDG_CONFIG_DIRS`), last first,
/// then the user's `user_file`, which is `None` without a config directory.
///
/// KConfig reads every `kdeglobals` `QStandardPaths::locateAll` finds, in
/// reverse, so the user's file comes last and wins (kconfig
/// `src/core/kconfig.cpp:740`, `:761-766` at a4e32e9d). The config dirs are
/// `$XDG_CONFIG_DIRS`'s absolute entries without duplicates, `/etc/xdg` when
/// none is left (qtbase `src/corelib/io/qstandardpaths_unix.cpp:370-389`,
/// `:403-413` at 512d45ad). Plasma puts `kdedefaults` beside the user's file
/// first in `$XDG_CONFIG_DIRS` (plasma-workspace
/// `startkde/startplasma.cpp:419-426` at 8bd4f86a); it is added there when
/// `config_dirs` lacks it, for a process started outside Plasma's session;
/// without a `user_file` there is no `kdedefaults` either.
fn kdeglobals_files_pure(
    config_dirs: Option<&str>,
    user_file: Option<std::path::PathBuf>,
) -> Vec<std::path::PathBuf> {
    let mut dirs: Vec<std::path::PathBuf> = Vec::new();
    for dir in config_dirs
        .unwrap_or_default()
        .split(':')
        .filter(|dir| dir.starts_with('/'))
    {
        let dir = std::path::PathBuf::from(dir);
        if !dirs.contains(&dir) {
            dirs.push(dir);
        }
    }
    if dirs.is_empty() {
        dirs.push(std::path::PathBuf::from("/etc/xdg"));
    }
    if let Some(defaults) = user_file
        .as_deref()
        .and_then(std::path::Path::parent)
        .map(|dir| dir.join("kdedefaults"))
        && !dirs.contains(&defaults)
    {
        dirs.insert(0, defaults);
    }
    let mut files: Vec<std::path::PathBuf> = dirs
        .iter()
        .rev()
        .map(|dir| dir.join("kdeglobals"))
        .collect();
    if let Some(user_file) = user_file
        && !files.contains(&user_file)
    {
        files.push(user_file);
    }
    files
}

/// The KConfig files `files` layered in order, later files overriding
/// earlier keys ([`read_kconfig`]). A file that cannot be read is skipped,
/// and its bytes are decoded lossily; `None` when no file can be read.
fn read_kconfig_files(files: &[std::path::PathBuf]) -> Option<Ini> {
    let mut ini = create_kde_parser();
    let mut any = false;
    for bytes in files.iter().filter_map(|path| std::fs::read(path).ok()) {
        read_kconfig(&mut ini, &String::from_utf8_lossy(&bytes));
        any = true;
    }
    any.then_some(ini)
}

/// Parse KConfig `content` into `ini`, over the keys it already holds.
///
/// The lines KConfig skips are skipped (kconfig `src/core/kconfigini.cpp` at
/// a4e32e9d): a group header without `]` (`:100-112`, the group before it
/// stays current) and an entry with an empty key (`:168-171`). They are the
/// only lines configparser rejects, so one stray line no longer loses the
/// whole file.
fn read_kconfig(ini: &mut Ini, content: &str) {
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

/// The user's kdeglobals file, in the user's config directory
/// ([`crate::detect::xdg_config_home`]); `None` without one.
pub(crate) fn kdeglobals_path() -> Option<std::path::PathBuf> {
    crate::detect::xdg_config_home().map(|dir| dir.join("kdeglobals"))
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
pub(crate) mod tests;
