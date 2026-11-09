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
//! each from the built-in defaults in `kde_config/defaults.rs`, which cite
//! their rows and the KDE facts that cite them (S9.1, S9.3, S4.8). The
//! files are read, and the scheme file converted, by `kde_config`, which a
//! KDE port's conversion shares in every build (S14). A value is read as
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

use crate::blocks::BlockPatch;
use crate::blocks::icon::IconStylePatch;
use crate::color::Color;
use crate::error::ReaderFailure;
#[cfg(test)]
use crate::expr::Expr;
use crate::inputs::{InputName, InputValue, KdeSchemeKey, KdeWmKey};
use crate::kde_config::{
    COLOR_GROUPS, TRANSPARENT, active_value, color, create_kde_parser, defaults, expr,
    inactive_value, input, kconfig_wm_key, kde_tokens, number, read_kconfig, read_kconfig_bytes,
    scheme_inputs, scheme_options, surfaces,
};
#[cfg(test)]
use crate::kde_config::{flag, parse_color};
use crate::key::Key;
use crate::layer::{Bases, Layer};
use crate::presets::PresetId;
use crate::reader::Reading;
use crate::resolve::ReaderState;
use crate::roles::{ColorRolesPatch, SurfaceSet};
use crate::selector::{Condition, Rule, RuleBody, Scheme, Selector, SetMatch};
use crate::tokens::{
    IconSizeTokensPatch, IconTokensPatch, MotionTokensPatch, TextRolesPatch, TextTokensPatch,
    TokensPatch,
};
#[cfg(test)]
use crate::tokens::{KdeColorEffect, KdeContrastEffect, KdeEffectsPatch, KdeIntensityEffect};
use crate::units::{Px, Scalar};
use crate::value::{SchemeMap, Value};

/// The text-scaling factor of unscaled text, S16's no-reader state's: the
/// identity of S9.4's scaling, not a platform value. Plasma scales text by
/// the font sizes it writes (K3.46).
const UNSCALED: f32 = 1.0;

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

/// The colour role patch of kdeglobals' results: each colour group's keys
/// as KColorScheme reads them ([`surfaces`]); and the `[WM]` keyed roles (K2.11), each key kdeglobals' where
/// it states one, else the references Breeze falls back to, the palette's
/// `Highlight` and `HighlightedText`, active for the active title bar and
/// disabled for the inactive one (`breeze@c97883a
/// kstyle/breezehelper.cpp:98-102`, the widget catalogue's `[WM]` roles).
fn results(ini: &Ini) -> ColorRolesPatch {
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
        surfaces: Some(surfaces(ini)),
        keyed: Some(keyed),
        ..ColorRolesPatch::default()
    }
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

/// The scheme's tokens the reader states as a `Tokens` rule under the
/// scheme it read (S4.8): `token.kde` from kdeglobals' colour effects, with
/// the scheme file's tint and title-bar options `options` where a scheme
/// file is in force (kdeglobals standing in for an unreadable one gives
/// none and false, S9.3), [`kde_tokens`]; and `token.params.color.contrast`
/// and `frame_contrast`, kdeglobals' `[KDE] contrast` and `frameContrast`
/// as stored, else KColorScheme's defaults.
fn scheme_tokens(ini: &Ini, options: Option<(Option<f32>, bool)>) -> TokensPatch {
    let kde = kde_tokens(ini, options);
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
                read_kconfig_bytes(&mut ini, &bytes);
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
/// earlier keys ([`read_kconfig_bytes`]). A file that cannot be read is
/// skipped, and its bytes are decoded lossily; `None` when no file can be
/// read.
fn read_kconfig_files(files: &[std::path::PathBuf]) -> Option<Ini> {
    let mut ini = create_kde_parser();
    let mut any = false;
    for bytes in files.iter().filter_map(|path| std::fs::read(path).ok()) {
        read_kconfig_bytes(&mut ini, &bytes);
        any = true;
    }
    any.then_some(ini)
}

/// The user's kdeglobals file, in the user's config directory
/// ([`crate::detect::xdg_config_home`]); `None` without one.
pub(crate) fn kdeglobals_path() -> Option<std::path::PathBuf> {
    crate::detect::xdg_config_home().map(|dir| dir.join("kdeglobals"))
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
pub(crate) mod tests;
