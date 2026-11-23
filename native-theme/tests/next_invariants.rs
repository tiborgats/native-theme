//! The invariants of every resolved theme (spec §11, item 5), over every
//! preset of [`PRESETS`] × {Light, Dark} × {Normal, Increased, High} × no
//! palette and every bundled palette, each with and without an explicit
//! accent, with and without the palette kept in high contrast, and with and
//! without system colours for the `High` level: every float finite, every
//! number non-negative but a signed offset, every opacity in `0.0..=1.0`,
//! every colour finite with its alpha in `0.0..=1.0`, every stated font
//! family non-empty, primary text legible on its surface (a contrast ratio of
//! at least 3:1), the palette and the accent suspended where the level used
//! is `High`, and `resolved.context` reporting that level.
//!
//! The theme is walked through its `Serialize` form, every number and string
//! with its path, so the test needs the `serde` feature.

#![cfg(feature = "serde")]

use std::fmt;

use native_theme::next::{
    Color, Contrast, OsColors, Palette, Preset, ResolvedTheme, Scheme, SurfaceColors,
    SystemOverrides, Theme, ThemeContext,
};
use serde::Serialize;
use serde::ser::{self, Serializer};

/// A test's result.
type TestResult = Result<(), Box<dyn std::error::Error>>;

/// The presets whose builders are written, each with the contrast levels it
/// defines (spec §1, "Axis values without data"). Every other preset resolves
/// to a theme that states nothing until its unit of the plan's `M2` adds it
/// here.
const PRESETS: &[(Preset, &[Contrast])] = &[
    (Preset::KdeBreeze, &[Contrast::Normal]),
    (Preset::Adwaita, &[Contrast::Normal, Contrast::High]),
    (Preset::Windows11, &[Contrast::Normal]),
    (Preset::MacosMonterey, &[Contrast::Normal]),
    (Preset::MacosTahoe, &[Contrast::Normal]),
    (Preset::Material3, &[Contrast::Normal]),
];

/// The least contrast ratio of primary text on its surface (spec §11, item 5:
/// WCAG 2.2's minimum for large text and user-interface components).
const MIN_CONTRAST: f32 = 3.0;

/// An explicit accent to resolve with: a test input, no theme value.
const ACCENT: Color = Color::rgb(0.2, 0.4, 0.8);

/// Path segments of the numbers that may be negative: offsets and spreads.
const SIGNED: &[&str] = &["offset", "spread", "letter_spacing", "angle"];

/// One leaf of a theme's serialized form.
#[derive(Debug)]
enum Leaf {
    Number(f64),
    Text(String),
}

/// Every leaf of a serialized value with its path, `a.b[2].c`.
#[derive(Default)]
struct Leaves(Vec<(String, Leaf)>);

#[derive(Debug)]
struct WalkError(String);

impl fmt::Display for WalkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for WalkError {}

impl ser::Error for WalkError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        WalkError(msg.to_string())
    }
}

/// The serializer of one value at `path`.
struct Walk<'a> {
    leaves: &'a mut Leaves,
    path: String,
}

impl Walk<'_> {
    fn push(self, leaf: Leaf) -> Result<(), WalkError> {
        self.leaves.0.push((self.path, leaf));
        Ok(())
    }
}

/// A compound value's serializer: its elements at `path[i]`, its fields at
/// `path.key`.
struct Compound<'a> {
    leaves: &'a mut Leaves,
    path: String,
    index: usize,
    key: Option<String>,
}

impl Compound<'_> {
    fn element<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), WalkError> {
        let path = format!("{}[{}]", self.path, self.index);
        self.index = self.index.saturating_add(1);
        value.serialize(Walk {
            leaves: self.leaves,
            path,
        })
    }

    fn field<T: Serialize + ?Sized>(&mut self, key: &str, value: &T) -> Result<(), WalkError> {
        let path = if self.path.is_empty() {
            key.to_owned()
        } else {
            format!("{}.{key}", self.path)
        };
        value.serialize(Walk {
            leaves: self.leaves,
            path,
        })
    }
}

macro_rules! numbers {
    ($($method:ident: $ty:ty),*) => {
        $(fn $method(self, v: $ty) -> Result<(), WalkError> {
            self.push(Leaf::Number(v as f64))
        })*
    };
}

impl<'a> Serializer for Walk<'a> {
    type Ok = ();
    type Error = WalkError;
    type SerializeSeq = Compound<'a>;
    type SerializeTuple = Compound<'a>;
    type SerializeTupleStruct = Compound<'a>;
    type SerializeTupleVariant = Compound<'a>;
    type SerializeMap = Compound<'a>;
    type SerializeStruct = Compound<'a>;
    type SerializeStructVariant = Compound<'a>;

    numbers!(
        serialize_i8: i8, serialize_i16: i16, serialize_i32: i32, serialize_i64: i64,
        serialize_u8: u8, serialize_u16: u16, serialize_u32: u32, serialize_u64: u64,
        serialize_f64: f64
    );

    fn serialize_f32(self, v: f32) -> Result<(), WalkError> {
        self.push(Leaf::Number(f64::from(v)))
    }

    fn serialize_bool(self, _: bool) -> Result<(), WalkError> {
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<(), WalkError> {
        self.push(Leaf::Text(v.to_string()))
    }

    fn serialize_str(self, v: &str) -> Result<(), WalkError> {
        self.push(Leaf::Text(v.to_owned()))
    }

    fn serialize_bytes(self, _: &[u8]) -> Result<(), WalkError> {
        Ok(())
    }

    fn serialize_none(self) -> Result<(), WalkError> {
        Ok(())
    }

    fn serialize_some<T: Serialize + ?Sized>(self, value: &T) -> Result<(), WalkError> {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<(), WalkError> {
        Ok(())
    }

    fn serialize_unit_struct(self, _: &'static str) -> Result<(), WalkError> {
        Ok(())
    }

    fn serialize_unit_variant(
        self,
        _: &'static str,
        _: u32,
        variant: &'static str,
    ) -> Result<(), WalkError> {
        self.push(Leaf::Text(variant.to_owned()))
    }

    fn serialize_newtype_struct<T: Serialize + ?Sized>(
        self,
        _: &'static str,
        value: &T,
    ) -> Result<(), WalkError> {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: Serialize + ?Sized>(
        self,
        _: &'static str,
        _: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<(), WalkError> {
        let path = format!("{}.{variant}", self.path);
        value.serialize(Walk {
            leaves: self.leaves,
            path,
        })
    }

    fn serialize_seq(self, _: Option<usize>) -> Result<Compound<'a>, WalkError> {
        Ok(self.compound(None))
    }

    fn serialize_tuple(self, _: usize) -> Result<Compound<'a>, WalkError> {
        Ok(self.compound(None))
    }

    fn serialize_tuple_struct(self, _: &'static str, _: usize) -> Result<Compound<'a>, WalkError> {
        Ok(self.compound(None))
    }

    fn serialize_tuple_variant(
        self,
        _: &'static str,
        _: u32,
        variant: &'static str,
        _: usize,
    ) -> Result<Compound<'a>, WalkError> {
        Ok(self.compound(Some(variant)))
    }

    fn serialize_map(self, _: Option<usize>) -> Result<Compound<'a>, WalkError> {
        Ok(self.compound(None))
    }

    fn serialize_struct(self, _: &'static str, _: usize) -> Result<Compound<'a>, WalkError> {
        Ok(self.compound(None))
    }

    fn serialize_struct_variant(
        self,
        _: &'static str,
        _: u32,
        variant: &'static str,
        _: usize,
    ) -> Result<Compound<'a>, WalkError> {
        Ok(self.compound(Some(variant)))
    }
}

impl<'a> Walk<'a> {
    fn compound(self, variant: Option<&str>) -> Compound<'a> {
        let path = match variant {
            Some(variant) => format!("{}.{variant}", self.path),
            None => self.path,
        };
        Compound {
            leaves: self.leaves,
            path,
            index: 0,
            key: None,
        }
    }
}

macro_rules! sequence {
    ($($trait:ident :: $method:ident),*) => {
        $(impl ser::$trait for Compound<'_> {
            type Ok = ();
            type Error = WalkError;

            fn $method<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), WalkError> {
                self.element(value)
            }

            fn end(self) -> Result<(), WalkError> {
                Ok(())
            }
        })*
    };
}

sequence!(
    SerializeSeq::serialize_element,
    SerializeTuple::serialize_element,
    SerializeTupleStruct::serialize_field,
    SerializeTupleVariant::serialize_field
);

macro_rules! structure {
    ($($trait:ident),*) => {
        $(impl ser::$trait for Compound<'_> {
            type Ok = ();
            type Error = WalkError;

            fn serialize_field<T: Serialize + ?Sized>(
                &mut self,
                key: &'static str,
                value: &T,
            ) -> Result<(), WalkError> {
                self.field(key, value)
            }

            fn end(self) -> Result<(), WalkError> {
                Ok(())
            }
        })*
    };
}

structure!(SerializeStruct, SerializeStructVariant);

impl ser::SerializeMap for Compound<'_> {
    type Ok = ();
    type Error = WalkError;

    fn serialize_key<T: Serialize + ?Sized>(&mut self, key: &T) -> Result<(), WalkError> {
        let mut leaves = Leaves::default();
        key.serialize(Walk {
            leaves: &mut leaves,
            path: String::new(),
        })?;
        let key = match leaves.0.as_slice() {
            [(_, Leaf::Text(text))] => text.clone(),
            [(_, Leaf::Number(n))] => n.to_string(),
            _ => format!("{{{}}}", self.index),
        };
        self.index = self.index.saturating_add(1);
        self.key = Some(key);
        Ok(())
    }

    fn serialize_value<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), WalkError> {
        let key = self.key.take().unwrap_or_default();
        self.field(&key, value)
    }

    fn end(self) -> Result<(), WalkError> {
        Ok(())
    }
}

/// Every leaf of `theme` with its path.
fn leaves(theme: &ResolvedTheme) -> Result<Vec<(String, Leaf)>, WalkError> {
    let mut leaves = Leaves::default();
    theme.serialize(Walk {
        leaves: &mut leaves,
        path: String::new(),
    })?;
    Ok(leaves.0)
}

/// The path's segments without their indices.
fn segments(path: &str) -> impl Iterator<Item = &str> {
    path.split('.')
        .map(|segment| segment.split('[').next().unwrap_or(segment))
}

/// Whether `text` is in one of a colour's text forms.
fn is_colour_form(text: &str) -> bool {
    text.starts_with('#') || text.starts_with("srgb(") || text.starts_with("p3(")
}

/// The violations of the leaf invariants in `theme`, each with its path.
fn leaf_violations(theme: &ResolvedTheme) -> Result<Vec<String>, WalkError> {
    let mut found = Vec::new();
    for (path, leaf) in leaves(theme)? {
        match leaf {
            Leaf::Number(n) => {
                let signed = segments(&path).any(|s| SIGNED.iter().any(|w| s.contains(w)));
                let opacity = segments(&path).any(|s| s.contains("opacity") || s == "alpha");
                if !n.is_finite() {
                    found.push(format!("{path}: {n} is not finite"));
                } else if !signed && n < 0.0 {
                    found.push(format!("{path}: {n} is negative"));
                } else if opacity && !(0.0..=1.0).contains(&n) {
                    found.push(format!("{path}: opacity {n} is outside 0..=1"));
                }
            }
            Leaf::Text(text) => {
                if is_colour_form(&text) && text.parse::<Color>().is_err() {
                    found.push(format!(
                        "{path}: colour {text} is not finite or its alpha is outside 0..=1"
                    ));
                }
                if segments(&path).last() == Some("family") && text.trim().is_empty() {
                    found.push(format!("{path}: an empty font family"));
                }
            }
        }
    }
    Ok(found)
}

/// The surfaces whose primary text is less legible than [`MIN_CONTRAST`].
fn illegible_surfaces(theme: &ResolvedTheme) -> Vec<String> {
    let roles = &theme.colors;
    let window = roles.window.background;
    let surfaces: [(&str, &SurfaceColors); 9] = [
        ("window", &roles.window),
        ("view", &roles.view),
        ("header", &roles.header),
        ("sidebar", &roles.sidebar),
        ("card", &roles.card),
        ("dialog", &roles.dialog),
        ("popover", &roles.popover),
        ("tooltip", &roles.tooltip),
        ("osd", &roles.osd),
    ];
    surfaces
        .iter()
        .filter_map(|(name, surface)| {
            let (background, foreground) = (surface.background?, surface.foreground?);
            let background = match window {
                Some(window) => background.composite_over(window),
                None => background,
            };
            let ratio = foreground
                .composite_over(background)
                .contrast_ratio(background);
            (ratio < MIN_CONTRAST)
                .then(|| format!("{name}: {foreground} on {background}, {ratio:.2}:1"))
        })
        .collect()
}

/// System settings stating a colour for both schemes at `High`.
fn high_contrast_system() -> SystemOverrides {
    let mut colors = OsColors::default();
    colors.focus_ring = Some(Color::BLACK);
    let mut system = SystemOverrides::default();
    system.color_schemes = vec![
        ((Scheme::Light, Contrast::High), colors.clone()),
        ((Scheme::Dark, Contrast::High), colors),
    ];
    system
}

/// The bundled palettes, each by its name.
fn bundled_palettes() -> Result<Vec<Palette>, String> {
    Palette::BUNDLED_NAMES
        .iter()
        .map(|name| Palette::from_name(name).ok_or_else(|| format!("no bundled palette {name}")))
        .collect()
}

/// The level a theme resolves with for `requested`, on a preset defining
/// `levels`, where `high_system` says whether the system states colours for
/// `High` (spec §6, resolution step 1): the requested level where it is
/// available, else the nearest available one below it. The levels here are
/// the same for both schemes, so a palette's scheme never holds `High` back.
fn level_used(requested: Contrast, levels: &[Contrast], high_system: bool) -> Contrast {
    let available =
        |level: Contrast| levels.contains(&level) || (high_system && level == Contrast::High);
    match requested {
        Contrast::High if available(Contrast::High) => Contrast::High,
        Contrast::High | Contrast::Increased if available(Contrast::Increased) => {
            Contrast::Increased
        }
        _ => Contrast::Normal,
    }
}

/// One theme of the test.
struct Case {
    preset: Preset,
    levels: &'static [Contrast],
    ctx: ThemeContext,
    high_system: bool,
    palette: Option<Palette>,
    accent: Option<Color>,
    preserve: bool,
}

impl Case {
    /// The theme without its palette, its accent and its choice to keep them.
    fn plain(&self) -> Theme {
        let theme = Theme::preset(self.preset);
        if self.high_system {
            theme.with_system_overrides(high_contrast_system())
        } else {
            theme
        }
    }

    /// The theme of the case.
    fn theme(&self) -> Theme {
        let mut theme = self
            .plain()
            .preserve_palette_in_high_contrast(self.preserve);
        if let Some(palette) = &self.palette {
            theme = theme.with_palette(palette.clone());
        }
        if let Some(accent) = self.accent {
            theme = theme.with_accent(accent);
        }
        theme
    }

    /// The violations of the invariants by the theme of the case.
    fn violations(&self) -> Result<Vec<String>, WalkError> {
        let got = self.theme().resolve(&self.ctx);
        let mut found = leaf_violations(&got)?;
        found.extend(illegible_surfaces(&got));
        let want = level_used(self.ctx.contrast, self.levels, self.high_system);
        if got.context.contrast != want {
            found.push(format!(
                "resolved.context reports {:?}, the level used is {want:?}",
                got.context.contrast
            ));
        }
        if want == Contrast::High && !self.preserve && got != self.plain().resolve(&self.ctx) {
            found.push("the palette or the accent is not suspended at High".to_owned());
        }
        Ok(found)
    }
}

impl fmt::Display for Case {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {:?} {:?}, palette {:?}, accent {:?}, preserved {}, system High {}",
            self.preset.as_str(),
            self.ctx.scheme,
            self.ctx.contrast,
            self.palette.as_ref().map(|p| p.name.to_string()),
            self.accent,
            self.preserve,
            self.high_system,
        )
    }
}

/// Every case of the test.
fn cases() -> Result<Vec<Case>, String> {
    let mut palettes: Vec<Option<Palette>> = vec![None];
    palettes.extend(bundled_palettes()?.into_iter().map(Some));
    let mut cases = Vec::new();
    for &(preset, levels) in PRESETS {
        for scheme in [Scheme::Light, Scheme::Dark] {
            for contrast in [Contrast::Normal, Contrast::Increased, Contrast::High] {
                let ctx = ThemeContext::default()
                    .with_scheme(scheme)
                    .with_contrast(contrast);
                for high_system in [false, true] {
                    for palette in &palettes {
                        for accent in [None, Some(ACCENT)] {
                            for preserve in [false, true] {
                                cases.push(Case {
                                    preset,
                                    levels,
                                    ctx: ctx.clone(),
                                    high_system,
                                    palette: palette.clone(),
                                    accent,
                                    preserve,
                                });
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(cases)
}

#[test]
fn every_resolved_theme_keeps_the_invariants() -> TestResult {
    let cases = cases()?;
    let palettes = Palette::BUNDLED_NAMES.len().saturating_add(1);
    assert_eq!(cases.len(), PRESETS.len() * 2 * 3 * 2 * palettes * 2 * 2);
    let mut violations = Vec::new();
    for case in &cases {
        for v in case.violations()? {
            violations.push(format!("{case}: {v}"));
        }
    }
    assert!(violations.is_empty(), "{}", violations.join("\n"));
    Ok(())
}

/// The walk reaches every number and every colour of the theme's JSON form,
/// so a check of its leaves is not vacuous.
#[test]
fn the_walk_reaches_every_number_and_colour() -> TestResult {
    fn count(value: &serde_json::Value, numbers: &mut usize, colours: &mut usize) {
        match value {
            serde_json::Value::Number(_) => *numbers = numbers.saturating_add(1),
            serde_json::Value::String(s) if is_colour_form(s) => {
                *colours = colours.saturating_add(1);
            }
            serde_json::Value::Array(items) => {
                items.iter().for_each(|v| count(v, numbers, colours));
            }
            serde_json::Value::Object(map) => {
                map.values().for_each(|v| count(v, numbers, colours));
            }
            _ => {}
        }
    }
    for ctx in [ThemeContext::light(), ThemeContext::dark()] {
        let theme = Theme::preset(Preset::KdeBreeze).resolve(&ctx);
        let (mut numbers, mut colours) = (0_usize, 0_usize);
        count(&serde_json::to_value(&theme)?, &mut numbers, &mut colours);
        let leaves = leaves(&theme)?;
        let walked_numbers = leaves
            .iter()
            .filter(|(_, leaf)| matches!(leaf, Leaf::Number(_)))
            .count();
        let walked_colours = leaves
            .iter()
            .filter(|(_, leaf)| matches!(leaf, Leaf::Text(t) if is_colour_form(t)))
            .count();
        assert!(
            numbers > 100 && colours > 100,
            "{numbers} numbers, {colours} colours"
        );
        assert_eq!((walked_numbers, walked_colours), (numbers, colours));
    }
    Ok(())
}

/// Each leaf check and the legibility check report a theme that breaks it.
#[test]
fn a_broken_theme_is_reported() -> TestResult {
    let broken = |f: fn(&mut ResolvedTheme)| {
        Theme::preset(Preset::KdeBreeze)
            .with_transform(f)
            .resolve(&ThemeContext::light())
    };
    let not_finite = broken(|t| t.layout.corner_radius = f32::NAN);
    let negative = broken(|t| t.layout.corner_radius_lg = -1.0);
    let opacity = broken(|t| t.layout.border_opacity = 1.5);
    let colour = broken(|t| t.colors.shadow = Color::new(0.0, 0.0, 0.0, 2.0));
    let family = broken(|t| t.typography.body.family = "".into());
    for (theme, what) in [
        (&not_finite, "layout.corner_radius: NaN is not finite"),
        (&negative, "layout.corner_radius_lg: -1 is negative"),
        (
            &opacity,
            "layout.border_opacity: opacity 1.5 is outside 0..=1",
        ),
        (&colour, "colors.shadow: colour"),
        (&family, "typography.body.family: an empty font family"),
    ] {
        let found = leaf_violations(theme)?;
        assert!(
            found.iter().any(|v| v.starts_with(what)),
            "{what}: {found:?}"
        );
    }
    let illegible = broken(|t| t.colors.window.foreground = t.colors.window.background);
    assert_eq!(illegible_surfaces(&illegible).len(), 1);
    Ok(())
}
