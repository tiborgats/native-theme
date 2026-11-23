//! What the live OS readers carry: the system's settings as read, each
//! optional, for the platform builders to lay over a preset.
//!
//! `SystemOverrides` holds everything the four 0.7.2 readers read (KDE, the
//! XDG portal for GNOME, macOS, Windows); what a 0.7.2 reader derives from its
//! reads for the widgets is its preset's builder's. A slot that more than one
//! system can fill is named for what it holds (`alternate_row_background`,
//! `title_bar`, `button`, `menu_font`, `icon_sizes`, `disabled_text`,
//! `scrollbar_width_px`); one whose notion is KDE's own is named `kde_…`.

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
    // KDE populates the scheme it read, at `Normal`; macOS `(Light, Normal)` and `(Dark, Normal)`, the two appearances the
    // 0.7.2 reader reads; Windows `(scheme, High)`, and only under high contrast; the portal reader none.
    /// The colours per `(Scheme, Contrast)`, 0 to 4 entries, each applied
    /// only where the effective scheme and contrast match it. KDE fills the
    /// scheme it read, at `Normal`; macOS `(Light, Normal)` and `(Dark,
    /// Normal)`, the two appearances the 0.7.2 reader reads; Windows
    /// `(scheme, High)`, and only under high contrast; the portal reader
    /// none.
    pub color_schemes: Vec<((Scheme, Contrast), OsColors)>,
    // Scheme-independent OS overrides
    /// The scheme the system was in when read; every reader sets it.
    pub scheme: Option<Scheme>, // the scheme the system was in when read; every reader sets it
    /// The theme's name, e.g. the KDE colour scheme's display name.
    pub theme_name: Option<Arc<str>>, // e.g. KDE color scheme display name
    /// The accent setting that is one value for every scheme (the XDG
    /// portal's `accent-color`, Windows' `UIColorType::Accent`); the fallback
    /// where the active [`OsColors::accent`] is `None`.
    pub accent: Option<Color>, // the accent setting that is one value for every scheme (the XDG portal's `accent-color`, Windows' `UIColorType::Accent`); fallback when active OsColors.accent is None
    /// Windows' accent shades, `AccentLight1..3` and `AccentDark1..3`, each
    /// shade `None` where its read failed.
    pub windows_accent_ramp: Option<WindowsAccentRamp>, // AccentLight1..3, AccentDark1..3, each shade `None` where its read failed
    /// The user interface font.
    pub ui_font: Option<OsFont>,
    /// The monospace font.
    pub monospace_font: Option<OsFont>,
    /// The caption font (KDE `[General] smallestReadableFont`): its size and
    /// weight are `typography.caption`'s, as in 0.7.2.
    pub caption_font: Option<OsFont>, // KDE `[General] smallestReadableFont`: its size and weight are `typography.caption`'s, as in 0.7.2
    /// The menu font (KDE `[General] menuFont`, Windows `lfMenuFont`, macOS
    /// `NSFont.menuFontOfSize`).
    pub menu_font: Option<OsFont>, // KDE `[General] menuFont`, Windows `lfMenuFont`, macOS `NSFont.menuFontOfSize`
    /// The toolbar font (KDE `[General] toolBarFont`).
    pub toolbar_font: Option<OsFont>, // KDE `[General] toolBarFont`
    /// The title bar font (KDE `[WM] activeFont`, GNOME `titlebar-font`,
    /// Windows `lfCaptionFont`, macOS `NSFont.titleBarFontOfSize`).
    pub title_bar_font: Option<OsFont>, // KDE `[WM] activeFont`, GNOME `titlebar-font`, Windows `lfCaptionFont`, macOS `NSFont.titleBarFontOfSize`
    /// The status bar font (Windows `lfStatusFont`, "the font used in status
    /// bars and tooltips").
    pub status_bar_font: Option<OsFont>, // Windows `lfStatusFont` ("the font used in status bars and tooltips")
    /// The tooltip font (macOS `NSFont.toolTipsFontOfSize`).
    pub tooltip_font: Option<OsFont>, // macOS `NSFont.toolTipsFontOfSize`
    /// The font DPI.
    pub font_dpi: Option<f32>,
    /// How text is rendered.
    pub text_rendering: Option<TextRendering>,
    /// The icon theme.
    pub icon_theme: Option<Arc<str>>,
    /// The icon sizes the system states; `None` where none was read. KDE:
    /// the default sizes of `icon_theme` (its `index.theme`, with
    /// KIconTheme's own default for a key the file lacks), used and dropped
    /// together with `icon_theme`. Windows: `SM_CXSMICON` and `SM_CXICON` as
    /// `small` and `large`, with no `icon_theme`: they hold in every scheme.
    pub icon_sizes: Option<IconSizes>, // the icon sizes the system states; `None` where none was read. KDE: the default sizes of `icon_theme` (its `index.theme`, with KIconTheme's own default for a key the file lacks), used and dropped together with `icon_theme`. Windows: `SM_CXSMICON` and `SM_CXICON` as `small` and `large`, with no `icon_theme`: they hold in every scheme
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
    /// The scroll bar's thickness as the system states it (Windows
    /// `SM_CXVSCROLL`; macOS the scroller's width at the regular size and
    /// the preferred style, the one width the 0.7.2 reader reads); the
    /// groove's exact thickness.
    pub scrollbar_width_px: Option<f32>, // the scroll bar's thickness as the system states it (Windows `SM_CXVSCROLL`; macOS the scroller's width at the regular size and the preferred style, the one width the 0.7.2 reader reads); the groove's exact thickness
    /// The scroll bar handle's least length (Windows `SM_CYVTHUMB`);
    /// `scrollbar.min_thumb_length`.
    pub scrollbar_min_thumb_length_px: Option<f32>, // Windows `SM_CYVTHUMB`; `scrollbar.min_thumb_length`
    /// The system's line width (Windows `SM_CXBORDER`); `layout.border_width`.
    pub border_width_px: Option<f32>, // the system's line width (Windows `SM_CXBORDER`); `layout.border_width`
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
    /// Per-surface optional overrides (`window`..`osd`, both title bars, the button's face and text).
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
    /// Disabled text (macOS `disabledControlTextColor`; Windows
    /// `COLOR_GRAYTEXT` under high contrast); KDE leaves it `None`: its
    /// disabled text is `view.foreground_muted`.
    pub disabled_text: Option<Color>, // disabled text (macOS `disabledControlTextColor`; Windows `COLOR_GRAYTEXT` under high contrast); KDE leaves it `None`: its disabled text is `view.foreground_muted`
    /// A text field's placeholder (macOS `placeholderTextColor`).
    pub placeholder: Option<Color>, // a text field's placeholder (macOS `placeholderTextColor`)
    /// The text caret (macOS `textInsertionPointColor`, from macOS 14).
    pub caret: Option<Color>, // the text caret (macOS `textInsertionPointColor`, from macOS 14)
    /// A table view's column header text (macOS `headerTextColor`).
    pub list_header_foreground: Option<Color>, // a table view's column header text (macOS `headerTextColor`)
    /// A table view's grid lines (macOS `gridColor`); `list.grid_color`.
    pub grid: Option<Color>, // a table view's grid lines (macOS `gridColor`); `list.grid_color`
    /// A view's alternate row (KDE `[Colors:View] BackgroundAlternate`,
    /// macOS `alternatingContentBackgroundColors[1]`).
    pub alternate_row_background: Option<Color>, // a view's alternate row (KDE `[Colors:View] BackgroundAlternate`, macOS `alternatingContentBackgroundColors[1]`)
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
        disabled_text: Option<Color>,
        placeholder: Option<Color>,
        caret: Option<Color>,
        list_header_foreground: Option<Color>,
        grid: Option<Color>,
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
            disabled_text,
            placeholder,
            caret,
            list_header_foreground,
            grid,
            alternate_row_background,
            kde_decoration_focus,
            kde_decoration_hover,
            kde_frame_contrast,
            kde_contrast,
            kde_effects,
        }
    }

    /// Whether these colours state at least one colour: a colour field,
    /// nested ones included; the contrast scalars and the effects state none
    /// (spec §6, resolution step 1).
    pub(crate) fn states_a_colour(&self) -> bool {
        let Self {
            surfaces,
            selection,
            text_selection,
            accent,
            focus_ring,
            link,
            link_visited,
            status,
            disabled_text,
            placeholder,
            caret,
            list_header_foreground,
            grid,
            alternate_row_background,
            kde_decoration_focus,
            kde_decoration_hover,
            kde_frame_contrast: _,
            kde_contrast: _,
            kde_effects: _,
        } = self;
        surfaces
            .as_ref()
            .is_some_and(OsSurfaceOverrides::states_a_colour)
            || [selection, text_selection]
                .into_iter()
                .any(|s| s.as_ref().is_some_and(OsSelectionColors::states_a_colour))
            || status.is_some_and(|status| status.iter().any(Option::is_some))
            || [
                accent,
                focus_ring,
                link,
                link_visited,
                disabled_text,
                placeholder,
                caret,
                list_header_foreground,
                grid,
                alternate_row_background,
                kde_decoration_focus,
                kde_decoration_hover,
            ]
            .into_iter()
            .any(Option::is_some)
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

    /// Whether any of the four colours is stated.
    fn states_a_colour(&self) -> bool {
        let Self {
            background,
            foreground,
            unfocused_background,
            unfocused_foreground,
        } = self;
        [
            background,
            foreground,
            unfocused_background,
            unfocused_foreground,
        ]
        .into_iter()
        .any(Option::is_some)
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
    /// activeBackground`, `activeForeground`; macOS `windowFrameTextColor`
    /// as `foreground` alone).
    pub title_bar: Option<SurfaceColors>, // the active window's title bar: `background`, `foreground` (KDE `[WM] activeBackground`, `activeForeground`; macOS `windowFrameTextColor` as `foreground` alone)
    /// An inactive window's title bar (KDE `[WM] inactiveBackground`,
    /// `inactiveForeground`).
    pub title_bar_inactive: Option<SurfaceColors>, // an inactive window's (KDE `[WM] inactiveBackground`, `inactiveForeground`)
    /// A button's face and text: KDE's seventh colour set
    /// (`[Colors:Button]`), Windows `COLOR_BTNFACE` and `COLOR_BTNTEXT` under
    /// high contrast.
    pub button: Option<SurfaceColors>, // a button's face and text: KDE's 7th colour set (`[Colors:Button]`), Windows `COLOR_BTNFACE` and `COLOR_BTNTEXT` under high contrast
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
        button: Option<SurfaceColors>,
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
            button,
        }
    }

    /// Whether any surface states a colour.
    fn states_a_colour(&self) -> bool {
        let Self {
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
            button,
        } = self;
        [
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
            button,
        ]
        .into_iter()
        .flatten()
        .any(|s| {
            let SurfaceColors {
                background,
                foreground,
                foreground_muted,
                border,
                divider,
            } = s;
            [background, foreground, foreground_muted, border, divider]
                .into_iter()
                .any(Option::is_some)
        })
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
    /// `AccentLight1..3`; a shade is `None` where its read failed (0.7.2
    /// reads each shade alone).
    pub light: [Option<Color>; 3], // AccentLight1..3; a shade is `None` where its read failed (0.7.2 reads each shade alone)
    /// `AccentDark1..3`.
    pub dark: [Option<Color>; 3], // AccentDark1..3
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
pub(super) mod kde;
/// Plasma's dark-or-light rule: the clean-room function of `v0.8.0-rc1`
/// (`src/cleanroom_readers/kde_color_scheme.rs`), carried unchanged with its
/// vector test, `tests/vectors/reader.kde_color_scheme.tsv`.
#[cfg(all(target_os = "linux", feature = "kde"))]
#[allow(
    clippy::integer_division,
    reason = "the clean room's helper, placed unchanged: Qt's qGray truncates by its definition, and the divisor is the constant 32"
)]
mod kde_color_scheme;
/// Qt's `QFont::fromString`: the clean-room parser of `v0.8.0-rc1`
/// (`src/cleanroom_readers/qfont_from_string.rs`), carried unchanged with its
/// vector test, `tests/vectors/reader.qfont_from_string.tsv`.
#[cfg(all(target_os = "linux", feature = "kde"))]
#[allow(
    dead_code,
    reason = "the clean room's parser, placed unchanged: the fields the reader does not keep are read by its vector test alone"
)]
#[allow(
    clippy::collapsible_if,
    reason = "the clean room's parser, its style kept as written: a style lint only"
)]
mod qfont_from_string;
#[cfg(test)]
mod tests;
