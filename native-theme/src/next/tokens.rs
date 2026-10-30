//! Tokens (S4): the values a preset states once for the whole theme, which
//! parts refer to as `token.<path>`.
//!
//! `Behavior.scroll` and its `ScrollPhysics` and `ScrollDecay` are staged:
//! they enter with the unit holding the `scroll_view` widget (S6.1).
//!
//! The marks of what may be absent are S6.3's: the core's marks of these
//! blocks' fields, and the catalogue's token table for the tokens no block
//! mark covers, per field the union over the presets that may leave it
//! absent.

use native_theme_derive::ThemeParts;

use super::blocks::focus::{
    FocusIndicator, FocusIndicatorPatch, FocusVisibleWhen, ResolvedFocusIndicator,
    ResolvedFocusVisibleWhen,
};
use super::blocks::icon::ResolvedIconStyle;
use super::blocks::icon::{IconStyle, Mirroring};
use super::blocks::motion::ResolvedTiming;
use super::blocks::motion::Timing;
use super::blocks::shadow::ResolvedShadowLayer;
use super::blocks::shadow::ShadowLayer;
use super::blocks::text::ResolvedTextStyle;
use super::blocks::text::{LanguageTag, TextStyle};
use super::color::Color;
use super::key::Key;
use super::selector::SizeClass;
use super::units::{Ms, Px, Scalar};
use super::value::Value;
use crate::{IconRole, IconSet};

/// The tokens (S4).
#[derive(Clone, Debug, PartialEq, ThemeParts)]
#[parts(block)]
#[non_exhaustive]
pub struct Tokens {
    /// Durations, timings (keyed) and the speed factor (S4.1).
    #[parts(block = "fields")]
    pub motion: MotionTokens,
    /// How a theme change is drawn (S4.3).
    #[parts(block = "set")]
    pub theme_change: ThemeChange,
    /// The platform's elevation ramp, keyed, or empty (R8).
    #[parts(block = "fields")]
    pub elevation: Vec<(Key, Vec<ShadowLayer>)>,
    /// The text ramp, fallbacks and rendering (S4.4).
    #[parts(block = "fields")]
    pub text: TextTokens,
    /// The icon set, theme and sizes (S4.5).
    #[parts(block = "fields")]
    pub icons: IconTokens,
    /// The platform's focus indicator (R12).
    #[parts(block = "fields")]
    pub focus: FocusIndicator,
    /// The caret (S4.2).
    #[parts(block = "fields")]
    pub caret: Caret,
    /// Input behaviour (S4.2).
    #[parts(block = "fields")]
    pub behavior: Behavior,
    /// The size classes' thresholds, where the platform states widths (R16).
    #[parts(block = "fields")]
    pub size_classes: Vec<(SizeClass, SizeClassThreshold)>,
    /// Keyed groups of keyed scalars, each group of the one unit its
    /// declaration names (S4.6); a key of a group `ABSENT_PARAM_GROUPS`
    /// names may be `Absent`.
    #[expect(clippy::type_complexity, reason = "S4's type, as the spec writes it")]
    pub params: Vec<(Key, Vec<(Key, Value<Scalar>)>)>,
    /// The pointer (S4.7).
    #[parts(block = "fields")]
    pub pointer: PointerTokens,
    /// KDE's colour effects and scheme options (S4.8); `kde-breeze` only.
    #[parts(block = "fields", may_be_absent)]
    pub kde: Value<KdeTokens>,
    /// Material's dynamic scheme (S4.8); `material-3` only.
    #[parts(block = "fields", may_be_absent)]
    pub material: Value<MaterialScheme>,
}

/// The keyed scalar groups whose keys may be `Absent` (S4.6, S6.3):
/// `display`, the measured display radii per corner, `Absent` for a device
/// model the preset does not list, where concentric corners fall back to
/// their stated minimum.
pub(crate) const ABSENT_PARAM_GROUPS: [&str; 1] = ["display"];

/// Motion tokens (S4.1). A key names the platform's motion in snake_case
/// and carries no unit suffix in a path (`token.motion.durations.button`);
/// in TOML a duration's key takes `_ms` as a field's does (`button_ms`,
/// S11).
#[derive(Clone, Debug, PartialEq, ThemeParts)]
#[parts(block)]
#[non_exhaustive]
pub struct MotionTokens {
    /// Keyed durations, in ms: a `token.motion.durations.<key>` is a
    /// duration; a scaled one is stated as
    /// `= mul(token.motion.durations.x, token.motion.speed_factor)`.
    pub durations: Vec<(Key, Value<Ms>)>,
    /// Keyed timings (S2.7): a `token.motion.timings.<key>` is a `Timing`,
    /// stated whole (S6.1).
    #[parts(block = "set")]
    pub timings: Vec<(Key, Timing)>,
    /// The factor the platform scales its animations' durations by (R9).
    pub speed_factor: Value<f32>,
}

/// How a theme change is drawn (S4.3): a re-resolution caused by a change of
/// the preset, a layer, an input, a reader value, or the scheme or contrast
/// conditions.
#[derive(Clone, Debug, PartialEq, ThemeParts)]
#[parts(block)]
pub enum ThemeChange {
    /// The window crossfades from the old frame to the new; the property
    /// transitions the change triggers do not run.
    Crossfade {
        /// The fade's duration.
        duration: Value<Ms>,
        /// The fade's timing.
        #[parts(block = "set")]
        timing: Value<Timing>,
    },
    /// Suspends the transitions marked `suspended_by_theme_change`.
    Suspend {
        /// How long they are suspended.
        duration: Value<Ms>,
    },
    /// No window effect: the property transitions the change triggers run as
    /// stated.
    Instant,
}

/// How text is rendered (S4): `token.text.rendering`. A toolkit applies what
/// it can (R23).
#[derive(Clone, Debug, PartialEq, ThemeParts)]
#[parts(block)]
#[non_exhaustive]
pub struct TextRendering {
    /// Qt Quick's render type, a key the preset declares.
    #[parts(may_be_absent)]
    pub engine: Value<Key>,
    /// The antialiasing.
    #[parts(may_be_absent)]
    pub antialiasing: Value<Antialiasing>,
    /// The hinting.
    #[parts(may_be_absent)]
    pub hinting: Value<Hinting>,
    /// The subpixel order.
    #[parts(may_be_absent)]
    pub subpixel_order: Value<SubpixelOrder>,
    /// The LCD filter, a key the preset declares.
    #[parts(may_be_absent)]
    pub lcd_filter: Value<Key>,
    /// The gamma.
    #[parts(may_be_absent)]
    pub gamma: Value<f32>,
    /// The contrast.
    #[parts(may_be_absent)]
    pub contrast: Value<f32>,
}

/// Text antialiasing.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ThemeParts)]
#[parts(toml)]
pub enum Antialiasing {
    /// None.
    None,
    /// Grayscale.
    Grayscale,
    /// Subpixel.
    Subpixel,
}

/// Font hinting.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ThemeParts)]
#[parts(toml)]
pub enum Hinting {
    /// None.
    None,
    /// Slight.
    Slight,
    /// Medium.
    Medium,
    /// Full.
    Full,
}

/// The order of a display's subpixels.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ThemeParts)]
#[parts(toml)]
pub enum SubpixelOrder {
    /// None.
    None,
    /// Red, green, blue, horizontally.
    Rgb,
    /// Blue, green, red, horizontally.
    Bgr,
    /// Red, green, blue, vertically.
    Vrgb,
    /// Blue, green, red, vertically.
    Vbgr,
}

/// The caret (S4.2); its shape is the caret parts' rules.
#[derive(Clone, Debug, PartialEq, ThemeParts)]
#[parts(block)]
#[non_exhaustive]
pub struct Caret {
    /// The caret's colour.
    pub color: Value<Color>,
    /// The caret's width.
    pub width: Value<Px>,
    /// How long the caret is shown in a blink.
    pub blink_on: Value<Ms>,
    /// How long the caret is hidden in a blink; zero: it does not blink.
    pub blink_off: Value<Ms>,
    /// How long the caret blinks before it stays shown; `None`: the blink
    /// never stops.
    pub blink_timeout: Option<Value<Ms>>,
    /// How the caret blinks.
    pub blink_style: BlinkStyle,
    /// The bidirectional secondary caret's colour.
    #[parts(may_be_absent)]
    pub secondary: Value<Color>,
    /// The caret is shown in read-only text.
    #[parts(may_be_absent)]
    pub shown_in_read_only: Value<bool>,
    /// The caret is kept solid while typing.
    #[parts(may_be_absent)]
    pub solid_while_typing: Value<bool>,
}

/// How a caret blinks.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ThemeParts)]
#[parts(toml)]
pub enum BlinkStyle {
    /// On and off.
    Hard,
    /// Fading in and out.
    Fade,
}

/// Input behaviour (S4.2). Every `Value` field may be absent where the
/// platform has no counterpart (S6.3).
#[derive(Clone, Debug, PartialEq, ThemeParts)]
#[parts(block)]
#[non_exhaustive]
pub struct Behavior {
    /// The longest time between the clicks of a double click.
    #[parts(may_be_absent)]
    pub double_click_time: Value<Ms>,
    /// The farthest the pointer may move between the clicks of a double
    /// click, horizontally.
    #[parts(may_be_absent)]
    pub double_click_distance: Value<Px>,
    /// The distance the pointer moves before a drag starts.
    #[parts(may_be_absent)]
    pub drag_threshold: Value<Px>,
    /// How long the pointer hovers before a hovered item opens.
    #[parts(may_be_absent)]
    pub hover_open_delay: Value<Ms>,
    /// Key repeat; `None`: keys never repeat (the user turned it off).
    #[parts(block = "set")]
    pub key_repeat: Option<KeyRepeat>,
    /// The lines one wheel step scrolls.
    #[parts(may_be_absent)]
    pub scroll_lines_per_step: Value<f32>,
    /// The pixels one wheel step scrolls.
    #[parts(may_be_absent)]
    pub scroll_px_per_step: Value<Px>,
    /// The pointer is hidden while typing.
    #[parts(may_be_absent)]
    pub hide_pointer_while_typing: Value<bool>,
    /// How long after typing starts the pointer is hidden.
    #[parts(may_be_absent)]
    pub hide_pointer_delay: Value<Ms>,
    /// How long the pointer takes to fade out.
    #[parts(may_be_absent)]
    pub hide_pointer_fade: Value<Ms>,
    /// How long a press lasts to be a long press.
    #[parts(may_be_absent)]
    pub long_press: Value<Ms>,
    /// How long a drag hovers over a target before it opens.
    #[parts(may_be_absent)]
    pub spring_loading: Value<Ms>,
    /// How long a press lasts before a drag starts without moving.
    #[parts(may_be_absent)]
    pub drag_time: Value<Ms>,
    /// A single click activates an item.
    #[parts(may_be_absent)]
    pub single_click_activates: Value<bool>,
    /// The farthest the pointer may move between the clicks of a double
    /// click, vertically.
    #[parts(may_be_absent)]
    pub double_click_height: Value<Px>,
    /// Touch behaviour.
    #[parts(block = "fields")]
    pub touch: TouchBehavior,
    /// When mnemonics are shown.
    #[parts(block = "set")]
    pub mnemonics: Mnemonics,
    /// When focus becomes visible, every widget's default.
    #[parts(block = "set", may_be_absent)]
    pub focus_visible_when: Value<FocusVisibleWhen>,
}

/// Key repeat: neither value may be absent.
#[derive(Clone, Debug, PartialEq, ThemeParts)]
#[parts(block)]
#[non_exhaustive]
pub struct KeyRepeat {
    /// How long a key is held before it repeats.
    pub delay: Value<Ms>,
    /// The time between repeats.
    pub interval: Value<Ms>,
}

impl KeyRepeat {
    /// Key repeat after `delay`, every `interval`.
    #[must_use]
    pub fn new(delay: Value<Ms>, interval: Value<Ms>) -> KeyRepeat {
        KeyRepeat { delay, interval }
    }
}

/// Touch behaviour (S4.2). Every field may be absent where the platform has
/// no counterpart (S6.3).
#[derive(Clone, Debug, PartialEq, ThemeParts)]
#[parts(block)]
#[non_exhaustive]
pub struct TouchBehavior {
    /// The distance a touch moves before it is a scroll.
    #[parts(may_be_absent)]
    pub slop: Value<Px>,
    /// The distance a touch moves before it pages.
    #[parts(may_be_absent)]
    pub paging_slop: Value<Px>,
    /// The longest time between the taps of a double tap.
    #[parts(may_be_absent)]
    pub double_tap_time: Value<Ms>,
    /// The farthest a touch may move between the taps of a double tap.
    #[parts(may_be_absent)]
    pub double_tap_slop: Value<Px>,
    /// How long a touch waits before it presses.
    #[parts(may_be_absent)]
    pub delay: Value<Ms>,
}

/// When mnemonics are shown.
#[derive(Clone, Debug, PartialEq, ThemeParts)]
#[parts(block)]
pub enum Mnemonics {
    /// Never.
    Never,
    /// While Alt is held.
    OnAlt {
        /// How long Alt is held before they show.
        delay: Value<Ms>,
    },
    /// Always.
    Always,
}

/// The text ramp's nineteen semantic slots (S4.4), each filled by every
/// preset: a reference to a keyed style, or to the slot that owns it.
#[derive(Clone, Debug, PartialEq, ThemeParts)]
#[parts(block)]
#[non_exhaustive]
pub struct TextRoles {
    /// `body`.
    #[parts(block = "fields")]
    pub body: Value<TextStyle>,
    /// `body_emphasized`.
    #[parts(block = "fields")]
    pub body_emphasized: Value<TextStyle>,
    /// `caption`.
    #[parts(block = "fields")]
    pub caption: Value<TextStyle>,
    /// `caption_emphasized`.
    #[parts(block = "fields")]
    pub caption_emphasized: Value<TextStyle>,
    /// `caption2`.
    #[parts(block = "fields")]
    pub caption2: Value<TextStyle>,
    /// `footnote`.
    #[parts(block = "fields")]
    pub footnote: Value<TextStyle>,
    /// `subheadline`.
    #[parts(block = "fields")]
    pub subheadline: Value<TextStyle>,
    /// `callout`.
    #[parts(block = "fields")]
    pub callout: Value<TextStyle>,
    /// `headline`.
    #[parts(block = "fields")]
    pub headline: Value<TextStyle>,
    /// `title1`.
    #[parts(block = "fields")]
    pub title1: Value<TextStyle>,
    /// `title2`.
    #[parts(block = "fields")]
    pub title2: Value<TextStyle>,
    /// `title3`.
    #[parts(block = "fields")]
    pub title3: Value<TextStyle>,
    /// `large_title`.
    #[parts(block = "fields")]
    pub large_title: Value<TextStyle>,
    /// `display_large`.
    #[parts(block = "fields")]
    pub display_large: Value<TextStyle>,
    /// `display_medium`.
    #[parts(block = "fields")]
    pub display_medium: Value<TextStyle>,
    /// `display_small`.
    #[parts(block = "fields")]
    pub display_small: Value<TextStyle>,
    /// `label`.
    #[parts(block = "fields")]
    pub label: Value<TextStyle>,
    /// `monospace`.
    #[parts(block = "fields")]
    pub monospace: Value<TextStyle>,
    /// `numeric`.
    #[parts(block = "fields")]
    pub numeric: Value<TextStyle>,
}

/// The text tokens (S4.4).
#[derive(Clone, Debug, PartialEq, ThemeParts)]
#[parts(block)]
#[non_exhaustive]
pub struct TextTokens {
    /// The semantic slots.
    #[parts(block = "fields")]
    pub roles: TextRoles,
    /// The platform's text styles, keyed.
    #[parts(block = "fields")]
    pub keyed: Vec<(Key, TextStyle)>,
    /// The font fallbacks.
    #[parts(block = "fields")]
    pub fallbacks: FontFallbacks,
    /// How text is rendered.
    #[parts(block = "fields")]
    pub rendering: TextRendering,
    /// How the text-scaling factor applies (S9.4).
    pub scaling: TextScaling,
    /// Logical pixels per inch (72 points), the platform's; the reader's
    /// where it reports one (S9.4).
    pub font_dpi: Value<f32>,
}

/// How the text-scaling factor applies (S9.4).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ThemeParts)]
#[parts(toml)]
pub enum TextScaling {
    /// A product.
    Linear,
    /// Android 14's `FontScaleConverter`, which scales large sizes less.
    AndroidNonLinear,
    /// Apple's table of each text style's size per content-size category.
    DynamicType,
}

/// The platform's font fallbacks (S4.4): one chain, per family where the
/// platform keeps one per font.
#[derive(Clone, Debug, PartialEq, ThemeParts)]
#[parts(block)]
#[non_exhaustive]
pub struct FontFallbacks {
    /// The families tried, in order.
    pub families: Vec<String>,
    /// The emoji family.
    pub emoji_family: Value<String>,
    /// The families tried per language.
    pub per_language: Vec<(LanguageTag, Vec<String>)>,
    /// The families tried, in order, where a family is missing or lacks a
    /// character, per family (Core Text's cascade list): a text font's and a
    /// glyph's family alike (`GlyphSource::FontCodePoint`), each followed by
    /// `families`.
    pub per_family: Vec<(String, Vec<String>)>,
}

/// The icon tokens (S4.5).
#[derive(Clone, Debug, PartialEq, ThemeParts)]
#[parts(block)]
#[non_exhaustive]
pub struct IconTokens {
    /// The icon set.
    pub set: Value<IconSet>,
    /// The icon theme; absent where the set has no named theme.
    #[parts(may_be_absent)]
    pub theme: Value<String>,
    /// The theme the platform's own lookup falls back to before the system
    /// fallback.
    #[parts(may_be_absent)]
    pub theme_fallback: Value<String>,
    /// The icon-size contexts.
    #[parts(block = "fields")]
    pub sizes: IconSizeTokens,
    /// How each icon role mirrors under right-to-left.
    pub mirroring: Vec<(IconRole, Mirroring)>,
}

/// The icon-size contexts (S4.5): each an icon style in which only `size` is
/// required, its other `Value` fields stated where the platform's context
/// sets them; a context the platform does not name is absent.
#[derive(Clone, Debug, PartialEq, ThemeParts)]
#[parts(block)]
#[non_exhaustive]
pub struct IconSizeTokens {
    /// Toolbar icons.
    #[parts(
        block = "fields",
        may_be_absent,
        inner_may_be_absent,
        required = "size"
    )]
    pub toolbar: Value<IconStyle>,
    /// Small icons.
    #[parts(
        block = "fields",
        may_be_absent,
        inner_may_be_absent,
        required = "size"
    )]
    pub small: Value<IconStyle>,
    /// Large icons.
    #[parts(
        block = "fields",
        may_be_absent,
        inner_may_be_absent,
        required = "size"
    )]
    pub large: Value<IconStyle>,
    /// Dialog icons.
    #[parts(
        block = "fields",
        may_be_absent,
        inner_may_be_absent,
        required = "size"
    )]
    pub dialog: Value<IconStyle>,
    /// Panel icons.
    #[parts(
        block = "fields",
        may_be_absent,
        inner_may_be_absent,
        required = "size"
    )]
    pub panel: Value<IconStyle>,
    /// The platform's further contexts, keyed.
    #[parts(block = "fields", inner_may_be_absent, required = "size")]
    pub keyed: Vec<(Key, IconStyle)>,
}

/// The pointer (S4.7), read where the platform reports it. Every `Value`
/// field may be absent (S6.3).
#[derive(Clone, Debug, PartialEq, ThemeParts)]
#[parts(block)]
#[non_exhaustive]
pub struct PointerTokens {
    /// The cursor theme.
    #[parts(may_be_absent)]
    pub theme: Value<String>,
    /// The cursor size.
    #[parts(may_be_absent)]
    pub size: Value<Px>,
    /// The pointer's colour.
    #[parts(may_be_absent)]
    pub color: Value<Color>,
    /// The cursors of a drag, keyed by drop operation.
    pub drag_cursors: Vec<(Key, String)>,
}

/// A size class's inclusive lower bound on each axis (S1); a window's class
/// on an axis is the one with the largest bound not above its size there.
#[derive(Clone, Debug, Default, PartialEq, ThemeParts)]
#[parts(block)]
#[non_exhaustive]
pub struct SizeClassThreshold {
    /// The bound on the width; `None`: not a class of that axis.
    pub horizontal: Option<Value<Px>>,
    /// The bound on the height; `None`: not a class of that axis.
    pub vertical: Option<Value<Px>>,
}

/// KDE's colour effects and scheme options (S4.8), every one stated: the
/// KDE functions branch only on `Option`s, bools, enums and the scheme
/// inputs' lists, never on an absent value.
#[derive(Clone, Debug, PartialEq, ThemeParts)]
#[parts(block)]
#[non_exhaustive]
pub struct KdeTokens {
    /// The disabled group's effects.
    #[parts(block = "fields")]
    pub disabled: KdeEffects,
    /// The inactive group's effects.
    #[parts(block = "fields")]
    pub inactive: KdeEffects,
    /// `[ColorEffects:Inactive] Enable`.
    pub inactive_enabled: Value<bool>,
    /// `ChangeSelectionColor`: the inactive selection takes the effects.
    pub inactive_change_selection_color: Value<bool>,
    /// The scheme's `[General] TintFactor`; `None`: the scheme does not tint.
    pub tint_factor: Option<Value<f32>>,
    /// The title bar takes the accent colour.
    pub titlebar_is_accent_colored: Value<bool>,
}

/// One colour group's effects, the amounts as kdeglobals states them: each
/// function clamps them as KColorScheme does.
#[derive(Clone, Debug, PartialEq, ThemeParts)]
#[parts(block)]
#[non_exhaustive]
pub struct KdeEffects {
    /// The intensity effect.
    pub intensity: KdeIntensityEffect,
    /// The intensity effect's amount.
    pub intensity_amount: Value<f32>,
    /// The colour effect.
    pub color: KdeColorEffect,
    /// The colour effect's amount.
    pub color_amount: Value<f32>,
    /// The colour effect's colour.
    pub color_color: Value<Color>,
    /// The contrast effect.
    pub contrast: KdeContrastEffect,
    /// The contrast effect's amount.
    pub contrast_amount: Value<f32>,
}

/// KDE's intensity effect.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ThemeParts)]
#[parts(toml)]
pub enum KdeIntensityEffect {
    /// KDE's "no effect" setting.
    None,
    /// Shade.
    Shade,
    /// Darken.
    Darken,
    /// Lighten.
    Lighten,
}

/// KDE's colour effect.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ThemeParts)]
#[parts(toml)]
pub enum KdeColorEffect {
    /// KDE's "no effect" setting.
    None,
    /// Desaturate.
    Desaturate,
    /// Fade.
    Fade,
    /// Tint.
    Tint,
}

/// KDE's contrast effect.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ThemeParts)]
#[parts(toml)]
pub enum KdeContrastEffect {
    /// KDE's "no effect" setting.
    None,
    /// Fade.
    Fade,
    /// Tint.
    Tint,
}

/// A KDE colour group (S4.8).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum KdeGroup {
    /// The active group.
    Active,
    /// The disabled group.
    Disabled,
    /// The inactive group.
    Inactive,
}

impl KdeGroup {
    /// Every group's name, in declaration order, its variant in snake_case.
    pub(crate) const NAMES: [&'static str; 3] = ["active", "disabled", "inactive"];
}

/// The kind of colour role a KDE effect applies to (S4.8).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum KdeRoleKind {
    /// A foreground role.
    Foreground,
    /// A decoration role.
    Decoration,
    /// A background role.
    Background,
}

impl KdeRoleKind {
    /// Every kind's name, in declaration order, its variant in snake_case.
    pub(crate) const NAMES: [&'static str; 3] = ["foreground", "decoration", "background"];
}

/// Material's dynamic scheme (S4.8); the seed is `input.accent`.
#[derive(Clone, Debug, PartialEq, ThemeParts)]
#[parts(block)]
#[non_exhaustive]
pub struct MaterialScheme {
    /// The scheme's variant.
    pub variant: MaterialVariant,
    /// The specification version `m3_role` implements, declared in
    /// `spec_versions` (S10).
    pub spec_version: Key,
    /// The contrast level: the standard level in the base, and `Tokens` rules
    /// under the contrast conditions; `m3_role` reads it.
    pub contrast_level: Value<f32>,
}

/// material-color-utilities' `Variant`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ThemeParts)]
#[parts(toml)]
pub enum MaterialVariant {
    /// `TONAL_SPOT`.
    TonalSpot,
    /// `NEUTRAL`.
    Neutral,
    /// `VIBRANT`.
    Vibrant,
    /// `EXPRESSIVE`.
    Expressive,
    /// `FIDELITY`.
    Fidelity,
    /// `CONTENT`.
    Content,
    /// `MONOCHROME`.
    Monochrome,
    /// `RAINBOW`.
    Rainbow,
    /// `FRUIT_SALAD`.
    FruitSalad,
    /// `CMF`.
    Cmf,
}
