//! What the live OS readers carry: the system's settings as read, each
//! optional, for the platform builders to lay over a preset.
//!
//! `SystemOverrides` holds everything the 0.7.2 KDE reader reads; the other
//! 0.7.2 readers' slots are added before each is ported. A slot that another
//! system can fill is named for what it holds (`alternate_row_background`,
//! `title_bar`, `menu_font`, `icon_sizes`); one whose notion is KDE's own is
//! named `kde_…`.

use std::sync::Arc;

use super::color::Color;
use super::icons::IconSizes;
use super::preset::Preset;
use super::query::{Contrast, Scheme, ThemeContext};
use super::roles::SurfaceColors;
use super::style::{FontSize, FontStyle, FontWidth};
use super::typography::TextRendering;
use super::window::CaptionLayout;

/// Returned by a reader: the detected preset, the initial live
/// [`ThemeContext`] and the [`SystemOverrides`]. Keeping `context` outside
/// `SystemOverrides` keeps `Theme::context()` the single source of truth.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct SystemSnapshot {
    /// The preset the system's settings belong to.
    pub preset: Preset,
    /// The context the system was in when read.
    pub context: ThemeContext,
    /// The system's settings.
    pub overrides: SystemOverrides,
}

impl SystemSnapshot {
    /// A snapshot of these fields.
    #[must_use]
    pub fn new(preset: Preset, context: ThemeContext, overrides: SystemOverrides) -> Self {
        Self {
            preset,
            context,
            overrides,
        }
    }
}

/// The system's settings as a reader gives them, or as an application with
/// its own source of system settings states them.
#[derive(Clone, Debug, Default, PartialEq)]
#[non_exhaustive]
pub struct SystemOverrides {
    // Context-scoped color overrides (0–4 entries; applied ONLY when matching effective `(Scheme, Contrast)`).
    // macOS populates Light/Dark × Normal/Increased; KDE/Windows populate the active OS `(Scheme, Contrast)`.
    /// The colours per `(Scheme, Contrast)`, 0 to 4 entries, each applied
    /// only where the effective scheme and contrast match it. macOS fills
    /// Light and Dark × Normal and Increased; KDE and Windows the system's
    /// active pair.
    pub color_schemes: Vec<((Scheme, Contrast), OsColors)>,
    // Scheme-independent OS overrides
    /// The scheme the system was in when read; every reader sets it.
    pub scheme: Option<Scheme>, // the scheme the system was in when read; every reader sets it
    /// The theme's name, e.g. the KDE colour scheme's display name.
    pub theme_name: Option<Arc<str>>, // e.g. KDE color scheme display name
    /// The accent setting that is one value for every scheme (the XDG
    /// portal's `accent-color`); the fallback where the active
    /// [`OsColors::accent`] is `None`.
    pub accent: Option<Color>, // the accent setting that is one value for every scheme (the XDG portal's `accent-color`); fallback when active OsColors.accent is None
    /// Windows' accent shades, `AccentLight1..3` and `AccentDark1..3`.
    pub windows_accent_ramp: Option<WindowsAccentRamp>, // AccentLight1..3, AccentDark1..3
    /// The user interface font.
    pub ui_font: Option<OsFont>,
    /// The monospace font.
    pub monospace_font: Option<OsFont>,
    /// The caption font (KDE `[General] smallestReadableFont`): its size and
    /// weight are `typography.caption`'s, as in 0.7.2.
    pub caption_font: Option<OsFont>, // KDE `[General] smallestReadableFont`: its size and weight are `typography.caption`'s, as in 0.7.2
    /// The menu font (KDE `[General] menuFont`).
    pub menu_font: Option<OsFont>, // KDE `[General] menuFont`
    /// The toolbar font (KDE `[General] toolBarFont`).
    pub toolbar_font: Option<OsFont>, // KDE `[General] toolBarFont`
    /// The title bar font (KDE `[WM] activeFont`).
    pub title_bar_font: Option<OsFont>, // KDE `[WM] activeFont`
    /// The font DPI.
    pub font_dpi: Option<f32>,
    /// How text is rendered.
    pub text_rendering: Option<TextRendering>,
    /// The icon theme.
    pub icon_theme: Option<Arc<str>>,
    /// The default sizes of `icon_theme` (KDE: its `index.theme`, with
    /// KIconTheme's own default for a key the file lacks); `None` where they
    /// were not read; used and dropped together with `icon_theme`.
    pub icon_sizes: Option<IconSizes>, // the default sizes of `icon_theme` (KDE: its `index.theme`, with KIconTheme's own default for a key the file lacks); `None` where they were not read; used and dropped together with `icon_theme`
    /// The pointer cursor theme.
    pub cursor_theme: Option<Arc<str>>,
    /// The pointer cursor's size, in logical pixels.
    pub cursor_size_px: Option<f32>,
    /// The animation speed factor (KDE `AnimationDurationFactor`).
    pub animation_scale: Option<f32>, // KDE AnimationDurationFactor
    /// How long the caret shows in a blink, in milliseconds.
    pub caret_blink_on_ms: Option<u32>,
    /// How long the caret hides in a blink, in milliseconds.
    pub caret_blink_off_ms: Option<u32>,
    /// After how long the caret stops blinking, in milliseconds.
    pub caret_blink_timeout_ms: Option<u32>,
    /// The double-click interval, in milliseconds.
    pub double_click_ms: Option<u32>,
    /// The distance a press must move to start a drag, in logical pixels.
    pub drag_threshold_px: Option<f32>,
    /// The title bar's buttons (GNOME's `button-layout`, KDE's `kwinrc`).
    pub caption_layout: Option<CaptionLayout>, // GNOME button-layout / KDE kwinrc
    /// The user's scroller style (macOS
    /// `NSScroller.preferredScrollerStyle`, GNOME's overlay setting as its
    /// 0.7.2 reader reads it); it overrides the preset's
    /// `scrollbar.overlay_mode`.
    pub overlay_scrollbars: Option<bool>, // the user's scroller style (macOS `NSScroller.preferredScrollerStyle`, GNOME's overlay setting as its 0.7.2 reader reads it); overrides the preset's `scrollbar.overlay_mode`
    /// Non-fatal partial fallbacks of the read.
    pub diagnostics: Vec<DiagnosticEntry>, // non-fatal partial fallbacks
}

impl SystemOverrides {
    /// The colours of the entry of `color_schemes` for `scheme` and
    /// `contrast`, the first such entry where there are several; `None`
    /// where there is none.
    #[must_use]
    pub fn colors_for(&self, scheme: Scheme, contrast: Contrast) -> Option<&OsColors> {
        self.color_schemes
            .iter()
            .find(|(key, _)| *key == (scheme, contrast))
            .map(|(_, colors)| colors)
    }
}

/// The colours a system states for one `(Scheme, Contrast)`.
#[derive(Clone, Debug, Default, PartialEq)]
#[non_exhaustive]
pub struct OsColors {
    /// Per-surface optional overrides (`window`..`osd`, both title bars + optional KDE `[Colors:Button]`).
    pub surfaces: Option<OsSurfaceOverrides>,
    /// The selection in a view.
    pub selection: Option<OsSelectionColors>,
    /// The selection of text.
    pub text_selection: Option<OsSelectionColors>,
    /// The accent for this scheme and contrast (e.g. macOS under Increase
    /// Contrast).
    pub accent: Option<Color>, // per-(Scheme, Contrast) OS accent (e.g., macOS Increase Contrast)
    /// The focus ring's colour.
    pub focus_ring: Option<Color>,
    /// A link's colour.
    pub link: Option<Color>,
    /// A visited link's colour.
    pub link_visited: Option<Color>,
    /// The status colours, `[error, warning, success, info]`.
    pub status: Option<[Option<Color>; 4]>, // [error, warning, success, info]
    /// A view's alternate row (KDE `[Colors:View] BackgroundAlternate`).
    pub alternate_row_background: Option<Color>, // a view's alternate row (KDE `[Colors:View] BackgroundAlternate`)
    /// KDE `[Colors:View] DecorationFocus`.
    pub kde_decoration_focus: Option<Color>, // KDE `[Colors:View] DecorationFocus`
    /// KDE `[Colors:View] DecorationHover`.
    pub kde_decoration_hover: Option<Color>, // KDE `[Colors:View] DecorationHover`
    /// KDE `[KDE] frameContrast` clamped to `0.0..=1.0`, a `qreal` as 0.7.2
    /// reads it; `None` (unset or not a number): KColorScheme's default 0.2.
    pub kde_frame_contrast: Option<f64>, // KDE `[KDE] frameContrast` clamped to 0.0..=1.0, a `qreal` as 0.7.2 reads it; `None` (unset or not a number): KColorScheme's default 0.2
    /// KDE `[KDE] contrast`; `None` (unset or not an integer):
    /// KColorScheme's default 7.
    pub kde_contrast: Option<i32>, // KDE `[KDE] contrast`; `None` (unset or not an integer): KColorScheme's default 7
    /// KDE `[ColorEffects:Inactive]` and `[ColorEffects:Disabled]`.
    pub kde_effects: Option<KdeColorEffects>, // KDE `[ColorEffects:Inactive]` & `[ColorEffects:Disabled]`
}

impl OsColors {
    /// System colours of these fields.
    #[must_use]
    #[allow(
        clippy::too_many_arguments,
        reason = "spec §1, \"Constructors\": `new` takes every present field"
    )]
    pub fn new(
        surfaces: Option<OsSurfaceOverrides>,
        selection: Option<OsSelectionColors>,
        text_selection: Option<OsSelectionColors>,
        accent: Option<Color>,
        focus_ring: Option<Color>,
        link: Option<Color>,
        link_visited: Option<Color>,
        status: Option<[Option<Color>; 4]>,
        alternate_row_background: Option<Color>,
        kde_decoration_focus: Option<Color>,
        kde_decoration_hover: Option<Color>,
        kde_frame_contrast: Option<f64>,
        kde_contrast: Option<i32>,
        kde_effects: Option<KdeColorEffects>,
    ) -> Self {
        Self {
            surfaces,
            selection,
            text_selection,
            accent,
            focus_ring,
            link,
            link_visited,
            status,
            alternate_row_background,
            kde_decoration_focus,
            kde_decoration_hover,
            kde_frame_contrast,
            kde_contrast,
            kde_effects,
        }
    }
}

/// `SelectionColors` with every colour optional: a system may state one colour of a pair (0.7.2 takes each alone).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[non_exhaustive]
pub struct OsSelectionColors {
    /// The selection's background.
    pub background: Option<Color>,
    /// The selected text's colour.
    pub foreground: Option<Color>,
    /// The background where the view lacks the focus.
    pub unfocused_background: Option<Color>,
    /// The text colour where the view lacks the focus.
    pub unfocused_foreground: Option<Color>,
}

impl OsSelectionColors {
    /// Selection colours of these fields.
    #[must_use]
    pub fn new(
        background: Option<Color>,
        foreground: Option<Color>,
        unfocused_background: Option<Color>,
        unfocused_foreground: Option<Color>,
    ) -> Self {
        Self {
            background,
            foreground,
            unfocused_background,
            unfocused_foreground,
        }
    }
}

/// The surfaces' colours a system states, each optional.
#[derive(Clone, Debug, Default, PartialEq)]
#[non_exhaustive]
pub struct OsSurfaceOverrides {
    /// A window's background surface.
    pub window: Option<SurfaceColors>,
    /// A content view.
    pub view: Option<SurfaceColors>,
    /// A header bar.
    pub header: Option<SurfaceColors>,
    /// A sidebar.
    pub sidebar: Option<SurfaceColors>,
    /// A card.
    pub card: Option<SurfaceColors>,
    /// A dialog.
    pub dialog: Option<SurfaceColors>,
    /// A popover.
    pub popover: Option<SurfaceColors>,
    /// A tooltip.
    pub tooltip: Option<SurfaceColors>,
    /// An on-screen display.
    pub osd: Option<SurfaceColors>,
    /// The active window's title bar: `background`, `foreground` (KDE `[WM]
    /// activeBackground`, `activeForeground`).
    pub title_bar: Option<SurfaceColors>, // the active window's title bar: `background`, `foreground` (KDE `[WM] activeBackground`, `activeForeground`)
    /// An inactive window's title bar (KDE `[WM] inactiveBackground`,
    /// `inactiveForeground`).
    pub title_bar_inactive: Option<SurfaceColors>, // an inactive window's (KDE `[WM] inactiveBackground`, `inactiveForeground`)
    /// KDE's seventh colour set (`[Colors:Button]`).
    pub kde_button: Option<SurfaceColors>, // KDE's 7th color set (`[Colors:Button]`)
}

impl OsSurfaceOverrides {
    /// Surface overrides of these fields.
    #[must_use]
    #[allow(
        clippy::too_many_arguments,
        reason = "spec §1, \"Constructors\": `new` takes every present field"
    )]
    pub fn new(
        window: Option<SurfaceColors>,
        view: Option<SurfaceColors>,
        header: Option<SurfaceColors>,
        sidebar: Option<SurfaceColors>,
        card: Option<SurfaceColors>,
        dialog: Option<SurfaceColors>,
        popover: Option<SurfaceColors>,
        tooltip: Option<SurfaceColors>,
        osd: Option<SurfaceColors>,
        title_bar: Option<SurfaceColors>,
        title_bar_inactive: Option<SurfaceColors>,
        kde_button: Option<SurfaceColors>,
    ) -> Self {
        Self {
            window,
            view,
            header,
            sidebar,
            card,
            dialog,
            popover,
            tooltip,
            osd,
            title_bar,
            title_bar_inactive,
            kde_button,
        }
    }
}

/// KDE's colour effects for inactive and disabled widgets.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub struct KdeColorEffects {
    /// The effects of `[ColorEffects:Inactive]`; `None` where the group's
    /// effect keys were not read (the 0.7.2 reader reads none): no effect is
    /// applied.
    pub inactive: Option<KdeStateEffects>, // `None` where the group's effect keys were not read (the 0.7.2 reader reads none): no effect is applied
    /// The effects of `[ColorEffects:Disabled]`, as `inactive`.
    pub disabled: Option<KdeStateEffects>, // as `inactive`
    /// `[ColorEffects:Inactive] ChangeSelectionColor`, else the group's
    /// `Enable`, else true, as KColorScheme and the 0.7.2 reader resolve it.
    pub change_selection_color: bool, // `[ColorEffects:Inactive] ChangeSelectionColor`, else the group's `Enable`, else true, as KColorScheme and the 0.7.2 reader resolve it
}

impl KdeColorEffects {
    /// Colour effects of these fields.
    #[must_use]
    pub fn new(
        inactive: Option<KdeStateEffects>,
        disabled: Option<KdeStateEffects>,
        change_selection_color: bool,
    ) -> Self {
        Self {
            inactive,
            disabled,
            change_selection_color,
        }
    }
}

/// One `[ColorEffects:*]` group of KDE's colour scheme.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[non_exhaustive]
pub struct KdeStateEffects {
    /// `[ColorEffects:*] Enable`.
    pub enabled: bool, // `[ColorEffects:*] Enable`
    /// The intensity effect.
    pub intensity: KdeIntensityEffect,
    /// The intensity effect's amount.
    pub intensity_amount: f32,
    /// The colour effect.
    pub color: KdeColorEffect,
    /// The colour effect's amount.
    pub color_amount: f32,
    /// The colour effect's colour.
    pub color_color: Color,
    /// The contrast effect.
    pub contrast: KdeContrastEffect,
    /// The contrast effect's amount.
    pub contrast_amount: f32,
}

impl KdeStateEffects {
    /// State effects of these fields.
    #[must_use]
    #[allow(
        clippy::too_many_arguments,
        reason = "spec §1, \"Constructors\": `new` takes every present field"
    )]
    pub fn new(
        enabled: bool,
        intensity: KdeIntensityEffect,
        intensity_amount: f32,
        color: KdeColorEffect,
        color_amount: f32,
        color_color: Color,
        contrast: KdeContrastEffect,
        contrast_amount: f32,
    ) -> Self {
        Self {
            enabled,
            intensity,
            intensity_amount,
            color,
            color_amount,
            color_color,
            contrast,
            contrast_amount,
        }
    }
}

/// KDE's intensity effect.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum KdeIntensityEffect {
    /// None.
    #[default]
    None,
    /// Shade.
    Shade,
    /// Darken.
    Darken,
    /// Lighten.
    Lighten,
}

/// KDE's colour effect.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum KdeColorEffect {
    /// None.
    #[default]
    None,
    /// Desaturate.
    Desaturate,
    /// Fade.
    Fade,
    /// Tint.
    Tint,
}

/// KDE's contrast effect.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum KdeContrastEffect {
    /// None.
    #[default]
    None,
    /// Fade.
    Fade,
    /// Tint.
    Tint,
}

/// A font as the system's setting gives it (0.7.2's `FontSpec` without the colour). Each field is `None`
/// where the setting does not give it; the preset's value for that field then stays, as in 0.7.2's merge.
#[derive(Clone, Debug, Default, PartialEq)]
#[non_exhaustive]
pub struct OsFont {
    /// The family.
    pub family: Option<Arc<str>>,
    /// The size in the unit the system states, unscaled (KDE: a `QFont` string's pixel size where positive,
    /// else its point size). The builder converts it via `font_dpi`, rounds it where `font_whole_pixels`,
    /// applies `ctx.text_scale`, and keeps it as the text's `defined_size`; what 0.7.2 derives from the
    /// unrounded size (the Kirigami headings, the line heights) is derived from this value.
    pub size: Option<FontSize>,
    /// The weight, CSS `100..=900`.
    pub weight: Option<u16>,
    /// The slant.
    pub style: Option<FontStyle>,
    /// The width; no 0.7.2 reader gives one.
    pub width: Option<FontWidth>, // no 0.7.2 reader gives one
}

impl OsFont {
    /// A system font of these fields.
    #[must_use]
    pub fn new(
        family: Option<Arc<str>>,
        size: Option<FontSize>,
        weight: Option<u16>,
        style: Option<FontStyle>,
        width: Option<FontWidth>,
    ) -> Self {
        Self {
            family,
            size,
            weight,
            style,
            width,
        }
    }
}

/// Windows' accent shades.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WindowsAccentRamp {
    /// `AccentLight1..3`.
    pub light: [Color; 3], // AccentLight1..3
    /// `AccentDark1..3`.
    pub dark: [Color; 3], // AccentDark1..3
}

/// A non-fatal partial fallback of a read.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct DiagnosticEntry {
    /// What was being read.
    pub source: &'static str,
    /// What fell back, and why.
    pub message: String,
}

impl DiagnosticEntry {
    /// A diagnostic of these fields.
    #[must_use]
    pub fn new(source: &'static str, message: String) -> Self {
        Self { source, message }
    }
}

#[cfg(all(target_os = "linux", feature = "kde"))]
mod kde;
#[cfg(test)]
mod tests;
