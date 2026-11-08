//! The KDE reader (the `kde` feature, Linux): kdeglobals as KConfig cascades
//! it, the colour scheme file it names, the icon theme's sizes and, on X11,
//! `Xft.dpi`, into a [`ReaderState`] and a reader layer (S8, S9.3, S9.5);
//! and the icon theme `detect` reports.
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
//! Every key the reader reads that its file leaves unset is read as KDE
//! reads it there, never left to the layers beneath: a colour key and a
//! colour effect as KColorScheme reads it, an icon size as KIconTheme does,
//! each from the built-in defaults in `defaults.rs`, which cite their rows
//! and the KDE facts that cite them (S9.1, S9.3, S4.8). A value is read as
//! KConfig reads it: a bool true unless it is `false`, `no`, `off` or `0`,
//! the empty value included; a number or colour KConfig cannot convert, an
//! empty one included, its default.
//!
//! The kdeglobals files are `$XDG_CONFIG_DIRS/kdeglobals`
//! (`/etc/xdg/kdeglobals`), then `kdedefaults/kdeglobals`, the defaults
//! Plasma keeps beneath the user's file, then the user's `kdeglobals`, each
//! in the user's config directory (`$XDG_CONFIG_HOME`, else `~/.config`).
//!
//! Three helpers come from the clean room (the 0.8.0 plan's D10, S2.2's
//! *The readers' helpers*), their code placed unchanged in
//! `src/cleanroom_readers/`, each with an adapter of its own: the font
//! entries are parsed by Qt's font parser, `qfont_from_string` (`font.rs`;
//! unchanged but for its panic-free arithmetic); the scheme is Plasma's
//! dark-or-light test, `kde_color_scheme` (`scheme.rs`); and the font DPI is
//! the one Qt's xcb plugin forces from `Xft.dpi`, `qxcb_xft_dpi`
//! (`font_dpi.rs`).
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
//!
//! # Sources not watched
//!
//! - `$XDG_CONFIG_DIRS/kdeglobals` (`/etc/xdg/kdeglobals`): the system's
//!   files, which the distribution or an administrator writes, not the
//!   user's settings.
//! - `color-schemes/<ColorScheme>.colors` in the data directories: the
//!   scheme file kdeglobals names, which a scheme's package writes; Plasma
//!   applies a scheme by writing kdeglobals, which is watched.
//! - `Xft.dpi` (`xrdb -query`): the font DPI on X11, read from the X
//!   server, not from a settings file.
//! - The icon theme's `index.theme`: its sizes, which the icon theme's
//!   package writes.
//! - `$XDG_SESSION_TYPE`: the session type, which decides whether the font
//!   DPI is read, fixed for a session.

mod defaults;
mod font;
mod font_dpi;
#[path = "../cleanroom_readers/kde_color_scheme.rs"]
#[allow(
    clippy::integer_division,
    reason = "the clean room's helper, placed unchanged (D10): Qt's qGray truncates by its definition, and the divisor is the constant 32"
)]
mod kde_color_scheme;
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
#[path = "../cleanroom_readers/qxcb_xft_dpi.rs"]
mod qxcb_xft_dpi;
mod scheme;

use configparser::ini::Ini;

use defaults::SetDefaults;

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
/// surface set `kde-breeze` declares for it, its group's name,
/// `[Colors:<name>]` (`colorsapplicator.cpp:157-163`, S9.1), and
/// KColorScheme's defaults for its keys (`defaults.rs`).
const COLOR_GROUPS: [(SurfaceSet, &str, &SetDefaults); 7] = [
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
const TRANSPARENT: Color = Color {
    r: 0.0,
    g: 0.0,
    b: 0.0,
    a: 0.0,
};

/// `[KDE] contrast` where kdeglobals sets none: KColorScheme's default,
/// `g.readEntry("contrast", 7)` (`kcolorscheme@6.31.0
/// src/kcolorscheme.cpp:519-527`, `docs/facts/kde/tokens.toml:11335`).
const DEFAULT_CONTRAST: f32 = 7.0;

/// `[KDE] frameContrast` where kdeglobals sets none: KColorScheme's
/// default, `const qreal default_value = 0.2;` (`kcolorscheme@6.31.0
/// src/kcolorscheme.cpp:529-539`, `docs/facts/kde/tokens.toml:11362`).
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
    /// The font DPI: `Xft.dpi`, on an X11 session
    /// ([`font_dpi::read_font_dpi`]); `None` elsewhere.
    pub(crate) font_dpi: Option<f32>,
    /// The icon theme's `index.theme`, where it was found.
    pub(crate) icon_index: Option<Ini>,
}

/// The reader's state and layer from what it read, its own preset's part
/// under `own` (S9.5). Zero I/O.
///
/// - The state: the scheme ([`scheme::scheme`]); animations off and reduced motion
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
    let scheme = scheme::scheme(ini);
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
    let sizes = data.icon_index.as_ref().map(icon_sizes);
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
///   it, the `Selection` set's `BackgroundNormal` (KColorScheme's default
///   where kdeglobals leaves it unset), under the scheme read only; in the
///   own part `#00000000` at that scheme.
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
            let selection = active_value(
                ini,
                SurfaceSet::Selection,
                "Selection",
                &defaults::SELECTION,
                KdeSchemeKey::BackgroundNormal,
            );
            let highlight = InputValue::new(SchemeMap::one(scheme, Value::Stated(selection)));
            let none = InputValue::new(SchemeMap::one(scheme, Value::Stated(TRANSPARENT)));
            (Some(highlight), Some(none))
        }
    }
}

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

/// The colour group `name`'s `key` in `ini` as KColorScheme reads it: the
/// group's value, else its default ([`default_value`]).
fn active_value(
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

/// The colour role patch of kdeglobals' results: each colour group's keys,
/// every set's roles the catalogue's key table maps them to
/// ([`KdeSchemeKey::role`]), a key kdeglobals leaves unset KColorScheme's
/// default ([`active_value`]); and the `[WM]` keyed roles (K2.11), each key kdeglobals' where
/// it states one, else the references Breeze falls back to, the palette's
/// `Highlight` and `HighlightedText`, active for the active title bar and
/// disabled for the inactive one (`breeze@c97883a
/// kstyle/breezehelper.cpp:98-102`, the widget catalogue's `[WM]` roles).
fn results(ini: &Ini) -> ColorRolesPatch {
    let mut surfaces = SurfaceMap::default();
    for (set, name, defaults) in COLOR_GROUPS {
        let mut patch = SurfaceRolesPatch::default();
        for key in KdeSchemeKey::ALL {
            let color = active_value(ini, set, name, defaults, key);
            set_role(&mut patch, key, Value::Stated(color));
        }
        surfaces.insert(set, BlockPatch::Fields(patch));
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

/// The colour set `set`'s inactive value of `key` in `ini`, as KColorScheme
/// reads its `Inactive` group: the `[Inactive]` group's value where the
/// set's group has one, a key it lacks the set's default
/// ([`default_value`]), not its active value (`kcolorscheme.cpp:344-357`);
/// else the active value (`:345-351`). So `Header` without a group of its
/// own reads its active values, `Window`'s over `Header`'s table, never
/// `[Colors:Window][Inactive]`.
fn inactive_value(
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

/// kdeglobals' inactive results, the inactive input roles
/// `input.inactive.<set>.<key>` under `scheme` (K3.51, S9.1), each key's
/// [`inactive_value`].
fn inactive_results(ini: &Ini, scheme: Scheme) -> Vec<(InputName, InputValue)> {
    let mut entries = Vec::new();
    for (set, name, defaults) in COLOR_GROUPS {
        for key in KdeSchemeKey::ALL {
            let value = inactive_value(ini, set, name, defaults, key);
            if let Some(name) = input(&format!("inactive.{}.{}", set.name(), key.name())) {
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
fn scheme_inputs(file: &Ini, scheme: Scheme, bases: &mut Bases) {
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
fn flag(ini: &Ini, section: &str, key: &str) -> Option<bool> {
    ini.get(section, key).map(|value| {
        let value = value.trim().to_ascii_lowercase();
        !matches!(value.as_str(), "false" | "no" | "off" | "0")
    })
}

/// A KConfig number entry, where it is finite; `None` where it is unset or
/// KConfig cannot convert it, the empty value included, so the caller takes
/// the key's default (`kconfig@6.31.0 src/core/kconfiggroup.cpp:256-268`).
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

/// One colour group's effects as KColorScheme reads kdeglobals'
/// `[ColorEffects:<group>]` (K3.40, K3.50, S4.8): each key's value where it
/// states one, the effects by KColorScheme's enums (`kcolorscheme@6.31.0
/// src/kcolorschemehelpers_p.h:57-79`, `docs/facts/kde/tokens.toml:10885`,
/// `:10940`, `:10995`), a key it leaves unset or KConfig cannot convert,
/// the empty value included, the group's default `defaults`
/// (`kcolorscheme.cpp:36-49`); an empty `Color` is stated as its default
/// too, which KColorScheme reads as an invalid colour, a recorded
/// inexactness (S4.8's *Unset and empty*). An effect number no enum member
/// holds is not stated. Where the disabled group's `Enable` is false
/// (`disabled`) its three effects are `none`, as KColorScheme then applies
/// none (`:39-40`).
fn effects(
    ini: &Ini,
    section: &str,
    defaults: &defaults::EffectDefaults,
    disabled: bool,
) -> KdeEffectsPatch {
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
    let amount =
        |key: &str, default: f32| Some(Value::Stated(number(ini, section, key).unwrap_or(default)));
    KdeEffectsPatch {
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
        color_color: Some(Value::Stated(
            color(ini, section, "Color").unwrap_or_else(|| defaults.color_color()),
        )),
        contrast: if off {
            Some(KdeContrastEffect::None)
        } else {
            contrast
        },
        contrast_amount: amount("ContrastAmount", defaults.contrast_amount),
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
/// `ChangeSelectionColor`, else its `Enable`, else true (`:307`); each read
/// as KConfig reads a bool ([`flag`]), so an empty one, which
/// `applyScheme()` writes where the scheme file lacks the key, is true.
fn scheme_tokens(ini: &Ini, options: Option<(Option<f32>, bool)>) -> TokensPatch {
    let inactive_section = "ColorEffects:Inactive";
    let enable = flag(ini, inactive_section, "Enable");
    let kde = KdeTokensPatch {
        disabled: Some(effects(
            ini,
            "ColorEffects:Disabled",
            &defaults::DISABLED,
            true,
        )),
        inactive: Some(effects(ini, inactive_section, &defaults::INACTIVE, false)),
        inactive_enabled: Some(Value::Stated(enable.unwrap_or(defaults::INACTIVE.enable))),
        inactive_change_selection_color: Some(Value::Stated(
            flag(ini, inactive_section, "ChangeSelectionColor")
                .or(enable)
                .unwrap_or(defaults::INACTIVE_CHANGE_SELECTION_COLOR),
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
/// `panel` `PanelDefault` and `keyed.main_toolbar` `MainToolbarDefault`,
/// the default size KIconTheme reads for each group
/// (`kiconthemes@6.31.0 src/kicontheme.cpp:469-472`); a key the theme
/// omits, or one KConfig does not convert to an integer, is KIconTheme's
/// own default for the group (`defaults::ICON_SIZES`), and a whole number
/// not above zero is not stated (S9.3).
fn icon_sizes(index: &Ini) -> IconSizeTokensPatch {
    let size = |key: &str, default: u16| {
        let size = match index
            .get("Icon Theme", key)
            .map(|value| value.trim().parse::<i32>())
        {
            None | Some(Err(_)) => Some(f32::from(default)),
            Some(Ok(size)) => (size > 0).then_some(size as f32),
        };
        size.map(|size| {
            BlockPatch::Fields(IconStylePatch {
                size: Some(Value::Stated(Px(size))),
                ..IconStylePatch::default()
            })
        })
    };
    let defaults = &defaults::ICON_SIZES;
    let keyed: Vec<_> = size("MainToolbarDefault", defaults.main_toolbar)
        .map(|patch| (crate::key!("main_toolbar"), patch))
        .into_iter()
        .collect();
    IconSizeTokensPatch {
        toolbar: size("ToolbarDefault", defaults.toolbar),
        small: size("SmallDefault", defaults.small),
        large: size("DesktopDefault", defaults.desktop),
        dialog: size("DialogDefault", defaults.dialog),
        panel: size("PanelDefault", defaults.panel),
        keyed: (!keyed.is_empty()).then_some(keyed),
        ..IconSizeTokensPatch::default()
    }
}

/// Reads the KDE settings, its own preset's part under `own`: kdeglobals
/// as KConfig cascades it ([`read_kconfig_files`] of [`kdeglobals_files`]),
/// the scheme file it names
/// ([`read_scheme_file`]), the font DPI ([`font_dpi::read_font_dpi`]) and
/// the icon theme's `index.theme`.
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
        font_dpi: font_dpi::read_font_dpi(crate::detect::session_type().as_deref()),
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
/// `color-schemes/<name>.colors` in the first of `data_dirs` where it is a
/// file, as Qt's `QStandardPaths::locate` finds it for the colours KCM and
/// plasma-integration (`plasma-workspace@6.8.2 kcms/colors/colors.cpp:468-474`,
/// `plasma-integration@6.8.2 qt6/src/platformtheme/khintssettings.cpp:457-458`),
/// read as KConfig reads it; [`SchemeFile::Unnamed`] where kdeglobals names
/// none, [`SchemeFile::Unreadable`] where no data directory holds it, where
/// the file found cannot be read (not a reason to look further, S9.3) or
/// where the name is not one plain path component, the reason naming the
/// scheme and the files.
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
    match files.iter().find(|file| file.is_file()) {
        Some(file) => match std::fs::read(file) {
            Ok(bytes) => {
                let mut ini = create_kde_parser();
                read_kconfig(&mut ini, &String::from_utf8_lossy(&bytes));
                SchemeFile::Read(ini)
            }
            Err(error) => SchemeFile::Unreadable(format!(
                "the colour scheme `{name}`'s file {} cannot be read ({error}); kdeglobals stands in for it",
                file.display()
            )),
        },
        None => SchemeFile::Unreadable(format!(
            "the colour scheme `{name}` has no scheme file ({}); kdeglobals stands in for it",
            files
                .iter()
                .map(|file| file.display().to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )),
    }
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
