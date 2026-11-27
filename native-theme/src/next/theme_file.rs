//! The TOML file formats (spec §7, "TOML file formats"): a standalone theme
//! ([`Theme::from_toml`]), a theme patch ([`ThemePatch::from_toml`]) and a
//! palette ([`Palette::from_toml`]).
//!
//! Text that is not TOML is [`Error::Toml`]. TOML whose content does not fit
//! the schema (an unknown key, a value of the wrong form, a selector on a
//! static widget) is [`Error::InvalidTheme`], its `path` the line and column
//! where the TOML parser found it; a value of the right form that fails the
//! strict check is [`Error::InvalidTheme`] at its key path; an unknown preset
//! or palette name is [`Error::UnknownPreset`] or [`Error::UnknownPalette`],
//! and a palette name while the `palettes` feature is off
//! [`Error::FeatureDisabled`].

use std::collections::BTreeMap;
use std::fmt;
use std::sync::Arc;

use serde::Deserialize;
use serde::de::value::MapAccessDeserializer;
use serde::de::{self, DeserializeOwned, Deserializer, MapAccess, Visitor};

use super::color::Color;
use super::error::Error;
use super::palette::Palette;
use super::patch::check;
use super::patch::{
    ColorPatch, MetricsSection, PatchFile, ThemePatch, TypographySection, WidgetPatchMap,
};
use super::preset::{Preset, Theme};

/// The value of `base` that picks the platform's preset ([`Preset::detect`]).
const AUTO: &str = "auto";

/// A standalone theme file: its identity, its palettes and accent, and the
/// patch sections of [`PatchFile`], each declared here (no `flatten`, so that
/// an unknown key is an error).
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ThemeFile {
    name: Arc<str>,
    base: String,
    #[serde(default)]
    accent: Option<Color>,
    #[serde(default)]
    palette: Option<PaletteRef>,
    #[serde(default)]
    palettes: Option<PalettePair>,
    #[serde(default)]
    metrics: MetricsSection,
    #[serde(default)]
    typography: TypographySection,
    #[serde(default)]
    colors: ColorPatch,
    #[serde(default)]
    widgets: WidgetPatchMap,
    #[serde(default)]
    custom_colors: BTreeMap<String, Color>,
    #[serde(default)]
    custom_metrics: BTreeMap<String, f32>,
}

/// The `[palettes]` table: a palette for each scheme.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct PalettePair {
    light: PaletteRef,
    dark: PaletteRef,
}

/// A palette as a theme file gives it: a bundled palette's name or an inline
/// table.
enum PaletteRef {
    Name(String),
    Inline(Box<Palette>),
}

impl<'de> Deserialize<'de> for PaletteRef {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct V;

        impl<'de> Visitor<'de> for V {
            type Value = PaletteRef;

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("a bundled palette's name or a palette table")
            }

            fn visit_str<E: de::Error>(self, v: &str) -> Result<PaletteRef, E> {
                Ok(PaletteRef::Name(v.to_owned()))
            }

            fn visit_map<A: MapAccess<'de>>(self, map: A) -> Result<PaletteRef, A::Error> {
                Palette::deserialize(MapAccessDeserializer::new(map))
                    .map(|palette| PaletteRef::Inline(Box::new(palette)))
            }
        }

        d.deserialize_any(V)
    }
}

impl PaletteRef {
    /// The palette: a name's bundled palette, or the inline table checked
    /// under `path`.
    fn resolve(self, path: &str) -> Result<Palette, Error> {
        match self {
            PaletteRef::Name(name) => bundled(name),
            PaletteRef::Inline(palette) => {
                check_palette(path, &palette)?;
                Ok(*palette)
            }
        }
    }
}

/// The bundled palette of `name`.
fn bundled(name: String) -> Result<Palette, Error> {
    if !cfg!(feature = "palettes") {
        return Err(Error::FeatureDisabled {
            feature: "palettes",
            needed_for: "a bundled palette's name in a theme file",
        });
    }
    Palette::from_name(&name).ok_or(Error::UnknownPalette { name })
}

/// The first colour of `palette` that fails the strict check, its path
/// under `path`.
fn check_palette(path: &str, palette: &Palette) -> Result<(), Error> {
    for (slot, color) in [
        ("window_bg", &palette.window_bg),
        ("view_bg", &palette.view_bg),
        ("surface_bg", &palette.surface_bg),
        ("header_bg", &palette.header_bg),
        ("sidebar_bg", &palette.sidebar_bg),
        ("tooltip_bg", &palette.tooltip_bg),
        ("foreground", &palette.foreground),
        ("foreground_muted", &palette.foreground_muted),
        ("border", &palette.border),
        ("divider", &palette.divider),
        ("accent", &palette.accent),
        ("accent_fg", &palette.accent_fg),
        ("selection_bg", &palette.selection_bg),
        ("selection_fg", &palette.selection_fg),
        ("link", &palette.link),
        ("caret", &palette.caret),
        ("error", &palette.error),
        ("warning", &palette.warning),
        ("success", &palette.success),
        ("info", &palette.info),
    ] {
        let at = if path.is_empty() {
            slot.to_owned()
        } else {
            format!("{path}.{slot}")
        };
        check::color(&at, color)?;
    }
    Ok(())
}

/// `text` read as a `T`: [`Error::Toml`] where it is not TOML, else
/// [`Error::InvalidTheme`] where its content does not fit `T`.
fn parse<T: DeserializeOwned>(text: &str) -> Result<T, Error> {
    toml::from_str::<toml::Table>(text)?;
    toml::from_str(text).map_err(|err| Error::InvalidTheme {
        path: err
            .span()
            .map(|span| location(text, span.start))
            .unwrap_or_default(),
        reason: err.message().to_owned(),
    })
}

/// The line and column, both from 1, of the byte `offset` of `text`.
fn location(text: &str, offset: usize) -> String {
    let (line, column) = text
        .char_indices()
        .take_while(|(index, _)| *index < offset)
        .fold((1_usize, 1_usize), |(line, column), (_, c)| {
            if c == '\n' {
                (line.saturating_add(1), 1)
            } else {
                (line, column.saturating_add(1))
            }
        });
    format!("line {line}, column {column}")
}

impl Theme {
    /// The theme of a standalone theme file (spec §7): `name`, `base`
    /// (`"auto"` for [`Preset::detect`], or a preset's
    /// [`as_str`](Preset::as_str) name), an optional `accent`, an optional
    /// `palette` (a bundled palette's name or an inline table) or `[palettes]`
    /// (`light` and `dark`, each a name or a table), and the patch sections
    /// of [`ThemePatch::from_toml`], applied as one patch. It reads nothing
    /// from the system: a caller lays it over the live system with
    /// [`with_context`](Self::with_context) and
    /// [`with_system_overrides`](Self::with_system_overrides).
    ///
    /// # Errors
    ///
    /// [`Error::Toml`] where `text` is not TOML; [`Error::InvalidTheme`]
    /// for an unknown key, a value of the wrong form, both `palette` and
    /// `palettes`, or a value that fails [`ThemePatch::validate`] (an inline
    /// palette's colours and the accent included); [`Error::UnknownPreset`],
    /// [`Error::UnknownPalette`], or [`Error::FeatureDisabled`] for a
    /// palette's name without the `palettes` feature.
    pub fn from_toml(text: &str) -> Result<Self, Error> {
        let file: ThemeFile = parse(text)?;
        let preset = if file.base == AUTO {
            Preset::detect()
        } else {
            Preset::from_name(&file.base).ok_or(Error::UnknownPreset { name: file.base })?
        };
        let mut theme = Theme::preset(preset).with_name(file.name);
        match (file.palette, file.palettes) {
            (Some(_), Some(_)) => {
                return Err(Error::InvalidTheme {
                    path: "palettes".to_owned(),
                    reason: "`palette` and `palettes` are mutually exclusive".to_owned(),
                });
            }
            (Some(palette), None) => theme = theme.with_palette(palette.resolve("palette")?),
            (None, Some(pair)) => {
                let light = pair.light.resolve("palettes.light")?;
                let dark = pair.dark.resolve("palettes.dark")?;
                theme = theme.with_palettes(light, dark);
            }
            (None, None) => {}
        }
        if let Some(accent) = file.accent {
            check::color("accent", &accent)?;
            theme = theme.with_accent(accent);
        }
        let patch = ThemePatch::from(PatchFile {
            metrics: file.metrics,
            typography: file.typography,
            colors: file.colors,
            widgets: file.widgets,
            custom_colors: file.custom_colors,
            custom_metrics: file.custom_metrics,
        });
        patch.validate()?;
        if patch != ThemePatch::default() {
            theme = theme.with_patch(patch);
        }
        Ok(theme)
    }
}

impl ThemePatch {
    /// The patch of a file of the base-independent patch sections (spec §7):
    /// `[metrics]` (`corner_scale`, `spacing_scale`), `[typography]`
    /// (`font_family`, `monospace_family`), `[colors]` (any
    /// [`ColorRoles`](super::ColorRoles) field), `[widgets]` (per widget its
    /// direct metrics, a ruled widget's `[[widgets.<widget>.rules]]`, a static
    /// widget's parts), `[custom_colors]` and `[custom_metrics]`; checked by
    /// [`validate`](Self::validate).
    ///
    /// # Errors
    ///
    /// [`Error::Toml`] where `text` is not TOML; [`Error::InvalidTheme`]
    /// for an unknown key (`name`, `base` and the palettes among them), a
    /// value of the wrong form, or a value that fails
    /// [`validate`](Self::validate).
    pub fn from_toml(text: &str) -> Result<Self, Error> {
        let patch: ThemePatch = parse(text)?;
        patch.validate()?;
        Ok(patch)
    }
}

impl Palette {
    /// The palette of a standalone palette file: `name`, `scheme` (`"light"`
    /// or `"dark"`) and every colour slot.
    ///
    /// # Errors
    ///
    /// [`Error::Toml`] where `text` is not TOML; [`Error::InvalidTheme`]
    /// for an unknown or missing key, a value of the wrong form, or a colour
    /// whose channels are not finite or whose alpha is outside `0.0..=1.0`.
    pub fn from_toml(text: &str) -> Result<Self, Error> {
        let palette: Palette = parse(text)?;
        check_palette("", &palette)?;
        Ok(palette)
    }
}

#[cfg(test)]
mod tests;
