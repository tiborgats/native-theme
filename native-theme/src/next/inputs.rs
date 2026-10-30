//! Inputs (S9): the colours a preset derives its roles from, which the
//! accent, a reader or a palette replaces, and KDE's scheme inputs.

use native_theme_derive::ThemeParts;

use std::fmt;

use super::color::Color;
use super::error::KeyError;
use super::key::Key;
use super::roles::{SurfaceRole, SurfaceSet};
use super::value::{SchemeMap, Value};

/// An input's path (`accent`, `accent_ramp.light1`,
/// `kde_scheme.view.foreground_visited`): `input.<path>` in expressions.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct InputName(Vec<Key>);

impl InputName {
    /// The input `path` names, its segments separated by `.`, or the first
    /// segment outside the key grammar.
    ///
    /// ```
    /// use native_theme::next::InputName;
    ///
    /// let name = InputName::new("accent_ramp.light1").map(|n| n.to_string());
    /// assert_eq!(name, Ok("accent_ramp.light1".to_owned()));
    /// assert!(InputName::new("accent.").is_err());
    /// ```
    pub fn new(path: &str) -> Result<InputName, KeyError> {
        path.split('.')
            .map(Key::new)
            .collect::<Result<Vec<_>, _>>()
            .map(InputName)
    }

    /// The path's segments.
    #[must_use]
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "read by the evaluator (M1a task 10)"
        )
    )]
    pub(crate) fn segments(&self) -> &[Key] {
        &self.0
    }
}

impl fmt::Display for InputName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut segments = self.0.iter();
        if let Some(first) = segments.next() {
            write!(f, "{first}")?;
        }
        segments.try_for_each(|segment| write!(f, ".{segment}"))
    }
}

/// The input bases of a layer (S9).
#[derive(Clone, Debug, Default, PartialEq)]
#[non_exhaustive]
pub struct InputBases {
    /// The inputs the layer states; a later layer's input replaces the
    /// earlier one whole.
    pub entries: Vec<(InputName, InputValue)>,
    /// `input.kde_scheme.groups`, per scheme: the declared sets the scheme
    /// file has a `[Colors:<Set>]` group for (S9.1); `kde-breeze`'s only,
    /// empty elsewhere. A later layer's list for a scheme replaces the
    /// earlier one whole, as for the two lists below.
    pub kde_groups: SchemeMap<Vec<SurfaceSet>>,
    /// `input.kde_scheme.inactive_groups`, per scheme: the declared sets
    /// whose group has an `[Inactive]` group.
    pub kde_inactive_groups: SchemeMap<Vec<SurfaceSet>>,
    /// `input.kde_scheme.unset_keys`, per scheme: the scheme inputs of the
    /// groups the file has that it leaves unset.
    pub kde_unset_keys: SchemeMap<Vec<InputName>>,
}

/// One input's value: per scheme, and per contrast condition where it
/// differs; where no contrast base is stated, the per-scheme value applies
/// under contrast too.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct InputValue {
    /// The value per scheme.
    pub per_scheme: SchemeMap<Value<Color>>,
    /// The value per contrast condition and scheme.
    pub per_contrast: Vec<(Contrast, SchemeMap<Value<Color>>)>,
}

impl InputValue {
    /// The input `per_scheme` states, with no contrast base.
    #[must_use]
    pub fn new(per_scheme: SchemeMap<Value<Color>>) -> InputValue {
        InputValue {
            per_scheme,
            per_contrast: Vec::new(),
        }
    }
}

/// A contrast condition, one at a time: the two exclude each other (S12).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ThemeParts)]
#[parts(toml)]
pub enum Contrast {
    /// Increased contrast.
    Increased,
    /// High contrast.
    High,
}

/// An accent an application sets (S9.2): the accent, and the preset's other
/// accent inputs derived from it.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct Accent {
    /// The accent.
    pub accent: InputValue,
    /// The preset's other accent inputs.
    pub derived: Vec<(InputName, InputValue)>,
}

impl Accent {
    /// The accent `accent`, with no derived inputs.
    #[must_use]
    pub fn new(accent: InputValue) -> Accent {
        Accent {
            accent,
            derived: Vec::new(),
        }
    }
}

/// A key of a KDE scheme file's colour group that KColorScheme reads and
/// Plasma's `applyScheme()` writes (S4.8): the `<key>` of
/// `input.kde_scheme.<set>.<key>`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum KdeSchemeKey {
    /// `BackgroundNormal`.
    BackgroundNormal,
    /// `BackgroundAlternate`.
    BackgroundAlternate,
    /// `ForegroundNormal`.
    ForegroundNormal,
    /// `ForegroundInactive`.
    ForegroundInactive,
    /// `ForegroundActive`.
    ForegroundActive,
    /// `ForegroundLink`.
    ForegroundLink,
    /// `ForegroundVisited`.
    ForegroundVisited,
    /// `ForegroundNegative`.
    ForegroundNegative,
    /// `ForegroundNeutral`.
    ForegroundNeutral,
    /// `ForegroundPositive`.
    ForegroundPositive,
    /// `DecorationFocus`.
    DecorationFocus,
    /// `DecorationHover`.
    DecorationHover,
}

impl KdeSchemeKey {
    /// Every key, in declaration order.
    pub const ALL: [KdeSchemeKey; 12] = [
        KdeSchemeKey::BackgroundNormal,
        KdeSchemeKey::BackgroundAlternate,
        KdeSchemeKey::ForegroundNormal,
        KdeSchemeKey::ForegroundInactive,
        KdeSchemeKey::ForegroundActive,
        KdeSchemeKey::ForegroundLink,
        KdeSchemeKey::ForegroundVisited,
        KdeSchemeKey::ForegroundNegative,
        KdeSchemeKey::ForegroundNeutral,
        KdeSchemeKey::ForegroundPositive,
        KdeSchemeKey::DecorationFocus,
        KdeSchemeKey::DecorationHover,
    ];

    /// Every key's name, in [`KdeSchemeKey::ALL`]'s order.
    pub(crate) const NAMES: [&'static str; 12] = [
        "background_normal",
        "background_alternate",
        "foreground_normal",
        "foreground_inactive",
        "foreground_active",
        "foreground_link",
        "foreground_visited",
        "foreground_negative",
        "foreground_neutral",
        "foreground_positive",
        "decoration_focus",
        "decoration_hover",
    ];

    /// The key's name in TOML and in paths, its variant in snake_case.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            KdeSchemeKey::BackgroundNormal => "background_normal",
            KdeSchemeKey::BackgroundAlternate => "background_alternate",
            KdeSchemeKey::ForegroundNormal => "foreground_normal",
            KdeSchemeKey::ForegroundInactive => "foreground_inactive",
            KdeSchemeKey::ForegroundActive => "foreground_active",
            KdeSchemeKey::ForegroundLink => "foreground_link",
            KdeSchemeKey::ForegroundVisited => "foreground_visited",
            KdeSchemeKey::ForegroundNegative => "foreground_negative",
            KdeSchemeKey::ForegroundNeutral => "foreground_neutral",
            KdeSchemeKey::ForegroundPositive => "foreground_positive",
            KdeSchemeKey::DecorationFocus => "decoration_focus",
            KdeSchemeKey::DecorationHover => "decoration_hover",
        }
    }

    /// The [`SurfaceRoles`](super::roles::SurfaceRoles) field the key names,
    /// by the catalogue's key table.
    #[must_use]
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "read where KDE's scheme inputs bind their roles (S9.1, the kde-breeze preset)"
        )
    )]
    pub(crate) const fn role(self) -> SurfaceRole {
        match self {
            KdeSchemeKey::BackgroundNormal => SurfaceRole::Background,
            KdeSchemeKey::BackgroundAlternate => SurfaceRole::BackgroundAlternate,
            KdeSchemeKey::ForegroundNormal => SurfaceRole::Foreground,
            KdeSchemeKey::ForegroundInactive => SurfaceRole::ForegroundInactive,
            KdeSchemeKey::ForegroundActive => SurfaceRole::ForegroundActive,
            KdeSchemeKey::ForegroundLink => SurfaceRole::Link,
            KdeSchemeKey::ForegroundVisited => SurfaceRole::LinkVisited,
            KdeSchemeKey::ForegroundNegative => SurfaceRole::Negative,
            KdeSchemeKey::ForegroundNeutral => SurfaceRole::Neutral,
            KdeSchemeKey::ForegroundPositive => SurfaceRole::Positive,
            KdeSchemeKey::DecorationFocus => SurfaceRole::DecorationFocus,
            KdeSchemeKey::DecorationHover => SurfaceRole::DecorationHover,
        }
    }
}

/// A key of a KDE scheme's `[WM]` group (S4.8): the `<key>` of
/// `input.kde_scheme.wm.<key>`, and of the keyed roles `role.@wm_<key>`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum KdeWmKey {
    /// `activeBackground`.
    ActiveBackground,
    /// `activeForeground`.
    ActiveForeground,
    /// `inactiveBackground`.
    InactiveBackground,
    /// `inactiveForeground`.
    InactiveForeground,
}

impl KdeWmKey {
    /// Every key, in declaration order.
    pub const ALL: [KdeWmKey; 4] = [
        KdeWmKey::ActiveBackground,
        KdeWmKey::ActiveForeground,
        KdeWmKey::InactiveBackground,
        KdeWmKey::InactiveForeground,
    ];

    /// Every key's name, in [`KdeWmKey::ALL`]'s order.
    pub(crate) const NAMES: [&'static str; 4] = [
        "active_background",
        "active_foreground",
        "inactive_background",
        "inactive_foreground",
    ];

    /// The key's name in TOML and in paths, its variant in snake_case.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            KdeWmKey::ActiveBackground => "active_background",
            KdeWmKey::ActiveForeground => "active_foreground",
            KdeWmKey::InactiveBackground => "inactive_background",
            KdeWmKey::InactiveForeground => "inactive_foreground",
        }
    }
}
