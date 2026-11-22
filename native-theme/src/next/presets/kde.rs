//! The `kde-breeze` preset's builder (spec §6): KDE Plasma's Breeze, light
//! and dark.
//!
//! The builder runs in three steps: the preset's constants, one table per
//! scheme for the colours and plain constants for the rest; the colour roles
//! and the globals made from them; the widgets derived from the roles, the
//! globals and the widget constants. A widget's colour that is a role's
//! colour is taken from the roles, not from a second copy of the constant.
//! So far it builds the static path: the system's settings
//! ([`ResolveInputs::system`]) and an explicit accent are not read yet.
//!
//! Defined by the preset (spec §1, "Axis values without data"): no variant,
//! no control size and the contrast level `Normal` only; a query for anything
//! else returns the part's base style.
//!
//! Of the context it acts on the scheme alone. The contrast, the motion
//! preference, the text scale, the direction and the accessibility and
//! window flags change nothing here: no row of the parity table needs them.
//!
//! Sources: `kde-breeze.toml` of 0.7.2 (`src/presets/kde-breeze.toml`), each
//! constant with the citation the TOML gives it (`docs/platform-facts.md` by
//! section, the upstream file at its pinned version) or `(uncited)`; 0.7.2's
//! inheritance rules (`src/resolve/inheritance.rs`, `src/resolve/validate.rs`,
//! `src/resolve/validate_helpers.rs`) where 0.7.2 fills a value the TOML does
//! not state.

use std::collections::BTreeMap;
use std::sync::Arc;

use crate::IconSet;
use crate::next::color::Color;
use crate::next::geometry::Stroke;
use crate::next::icons::{IconSizes, IconThemeSpec};
use crate::next::layout::LayoutMetrics;
use crate::next::motion::{CaretSpec, MotionTokens, UiTimings};
use crate::next::preset::{Preset, ResolveInputs, nothing_stated};
use crate::next::query::{Scheme, ThemeContext};
use crate::next::roles::{
    AccentColors, ColorRoles, RoleOrigin, SelectionColors, StatusColors, SurfaceColors,
};
use crate::next::style::{
    FocusIndicator, FocusKind, FocusTrigger, FontSize, FontStyle, TextStated, TextStyle,
};
use crate::next::typography::{TextRendering, Typography};
use crate::next::widgets::ResolvedTheme;

// ---------------------------------------------------------------------------
// The constants
// ---------------------------------------------------------------------------

/// The constants `kde-breeze.toml` states for one scheme: the colours as
/// 8-bit sRGB `0xRRGGBBAA`, and the icon theme.
struct SchemeConstants {
    window_background: u32,
    window_foreground: u32,
    window_foreground_muted: u32,
    window_border: u32,
    view_background: u32,
    selection_background: u32,
    selection_foreground: u32,
    selection_unfocused_background: u32,
    accent: u32,
    focus_ring: u32,
    link: u32,
    disabled_text: u32,
    shadow: u32,
    error: u32,
    warning: u32,
    success: u32,
    info: u32,
    icon_theme: &'static str,
}

/// The light scheme's constants.
const LIGHT: SchemeConstants = SchemeConstants {
    // `defaults.background_color`: platform-facts.md (§2.1.3): [Colors:Window] BackgroundNormal
    // 239,240,241 (BreezeLight.colors:129, breeze v6.7.5)
    window_background: 0xeff0_f1ff,
    // `defaults.text_color`: platform-facts.md (§2.1.3): [Colors:Window] ForegroundNormal
    // 35,38,41 (BreezeLight.colors:137, breeze v6.7.5)
    window_foreground: 0x2326_29ff,
    // `defaults.muted_color`: platform-facts.md (§2.1.3): [Colors:Window] ForegroundInactive
    // 112,125,138 (BreezeLight.colors:133, breeze v6.7.5)
    window_foreground_muted: 0x707d_8aff,
    // `defaults.border.color`: 0.7.2 kde-breeze.toml (uncited)
    window_border: 0xbcc0_bfff,
    // `defaults.surface_color`: platform-facts.md (§2.1.3): [Colors:View] BackgroundNormal
    // 255,255,255 (BreezeLight.colors:115, breeze v6.7.5)
    view_background: 0xffff_ffff,
    // `defaults.selection_background`: platform-facts.md (§2.1.3): [Colors:Selection]
    // BackgroundNormal 61,174,233 (BreezeLight.colors:87, breeze v6.7.5)
    selection_background: 0x3dae_e9ff,
    // `defaults.selection_text_color`: platform-facts.md (§2.1.3): [Colors:Selection]
    // ForegroundNormal 255,255,255 (BreezeLight.colors:95, breeze v6.7.5)
    selection_foreground: 0xffff_ffff,
    // `defaults.selection_inactive_background`: platform-facts.md (§2.1.3):
    // selection_inactive_background, KColorUtils::tint([Colors:Window] BackgroundNormal
    // 239,240,241, [Colors:Selection] BackgroundNormal 61,174,233, 0.4), as
    // [ColorEffects:Inactive] ChangeSelectionColor=true (BreezeLight.colors:19 at breeze
    // f0b1d75) has KColorScheme build an inactive window's selection from the Window set
    // (kcolorscheme.cpp:304-321 at kcolorscheme 27066d47; measured)
    selection_unfocused_background: 0xc2e0_f5ff,
    // `defaults.accent_color`: platform-facts.md (§2.1.3): BreezeLight.colors has no [General]
    // AccentColor, so [Colors:View] DecorationFocus 61,174,233 (BreezeLight.colors:116, breeze
    // v6.7.5)
    accent: 0x3dae_e9ff,
    // `defaults.focus_ring_color`: platform-facts.md (§2.1.5): focus_ring_color: Breeze's
    // keyboard focus band, [Colors:Selection] BackgroundNormal 61,174,233
    // (BreezeLight.colors:87, breeze v6.7.5) at alpha Blend_Value 0.3 (breezemetrics.h:176)
    focus_ring: 0x3dae_e94d,
    // `defaults.link_color`: platform-facts.md (§2.1.3): [Colors:View] ForegroundLink
    // 41,128,185 (BreezeLight.colors:120, breeze v6.7.5)
    link: 0x2980_b9ff,
    // `defaults.disabled_text_color`: platform-facts.md (§2.1.3): Disabled [Colors:Window]
    // ForegroundNormal, KColorScheme 6.31.0 [ColorEffects:Disabled]
    disabled_text: 0xa0a1_a3ff,
    // `defaults.shadow_color`: platform-facts.md (§2.1.3): (none) in Breeze, the preset's own
    // #00000040
    shadow: 0x0000_0040,
    // `defaults.danger_color`: platform-facts.md (§2.1.4): [Colors:View] ForegroundNegative
    // 218,68,83 (BreezeLight.colors:121, breeze v6.7.5)
    error: 0xda44_53ff,
    // `defaults.warning_color`: platform-facts.md (§2.1.4): [Colors:View] ForegroundNeutral
    // 246,116,0 (BreezeLight.colors:122, breeze v6.7.5)
    warning: 0xf674_00ff,
    // `defaults.success_color`: platform-facts.md (§2.1.4): [Colors:View] ForegroundPositive
    // 39,174,96 (BreezeLight.colors:124, breeze v6.7.5)
    success: 0x27ae_60ff,
    // `defaults.info_color`: platform-facts.md (§2.1.4): [Colors:View] ForegroundActive
    // 61,174,233 (BreezeLight.colors:118, breeze v6.7.5)
    info: 0x3dae_e9ff,
    // `light.defaults.icon_theme`: 0.7.2 kde-breeze.toml (uncited)
    icon_theme: "breeze",
};

/// The dark scheme's constants.
const DARK: SchemeConstants = SchemeConstants {
    // `defaults.background_color`: platform-facts.md (§2.1.3): [Colors:Window] BackgroundNormal
    // 32,35,38 (BreezeDark.colors:130, breeze v6.7.5)
    window_background: 0x2023_26ff,
    // `defaults.text_color`: platform-facts.md (§2.1.3): [Colors:Window] ForegroundNormal
    // 252,252,252 (BreezeDark.colors:138, breeze v6.7.5)
    window_foreground: 0xfcfc_fcff,
    // `defaults.muted_color`: platform-facts.md (§2.1.3): [Colors:Window] ForegroundInactive
    // 161,169,177 (BreezeDark.colors:134, breeze v6.7.5)
    window_foreground_muted: 0xa1a9_b1ff,
    // `defaults.border.color`: 0.7.2 kde-breeze.toml (uncited)
    window_border: 0x4d54_5bff,
    // `defaults.surface_color`: platform-facts.md (§2.1.3): [Colors:View] BackgroundNormal
    // 20,22,24 (BreezeDark.colors:116, breeze v6.7.5)
    view_background: 0x1416_18ff,
    // `defaults.selection_background`: platform-facts.md (§2.1.3): [Colors:Selection]
    // BackgroundNormal 61,174,233 (BreezeDark.colors:88, breeze v6.7.5)
    selection_background: 0x3dae_e9ff,
    // `defaults.selection_text_color`: platform-facts.md (§2.1.3): [Colors:Selection]
    // ForegroundNormal 252,252,252 (BreezeDark.colors:96, breeze v6.7.5)
    selection_foreground: 0xfcfc_fcff,
    // `defaults.selection_inactive_background`: platform-facts.md (§2.1.3):
    // selection_inactive_background, KColorUtils::tint([Colors:Window] BackgroundNormal
    // 32,35,38, [Colors:Selection] BackgroundNormal 61,174,233, 0.4), as
    // [ColorEffects:Inactive] ChangeSelectionColor=true (BreezeDark.colors:20 at breeze
    // f0b1d75) has KColorScheme build an inactive window's selection from the Window set
    // (kcolorscheme.cpp:304-321 at kcolorscheme 27066d47; measured)
    selection_unfocused_background: 0x1b41_55ff,
    // `defaults.accent_color`: platform-facts.md (§2.1.3): BreezeDark.colors has no [General]
    // AccentColor, so [Colors:View] DecorationFocus 61,174,233 (BreezeDark.colors:117, breeze
    // v6.7.5)
    accent: 0x3dae_e9ff,
    // `defaults.focus_ring_color`: platform-facts.md (§2.1.5): focus_ring_color: Breeze's
    // keyboard focus band, [Colors:Selection] BackgroundNormal 61,174,233
    // (BreezeDark.colors:88, breeze v6.7.5) at alpha Blend_Value 0.3 (breezemetrics.h:176)
    focus_ring: 0x3dae_e94d,
    // `defaults.link_color`: platform-facts.md (§2.1.3): [Colors:View] ForegroundLink
    // 29,153,243 (BreezeDark.colors:121, breeze v6.7.5)
    link: 0x1d99_f3ff,
    // `defaults.disabled_text_color`: platform-facts.md (§2.1.3): Disabled [Colors:Window]
    // ForegroundNormal, KColorScheme 6.31.0 [ColorEffects:Disabled]
    disabled_text: 0x686a_6cff,
    // `defaults.shadow_color`: platform-facts.md (§2.1.3): (none) in Breeze, the preset's own
    // #00000060
    shadow: 0x0000_0060,
    // `defaults.danger_color`: platform-facts.md (§2.1.4): [Colors:View] ForegroundNegative
    // 218,68,83 (BreezeDark.colors:122, breeze v6.7.5)
    error: 0xda44_53ff,
    // `defaults.warning_color`: platform-facts.md (§2.1.4): [Colors:View] ForegroundNeutral
    // 246,116,0 (BreezeDark.colors:123, breeze v6.7.5)
    warning: 0xf674_00ff,
    // `defaults.success_color`: platform-facts.md (§2.1.4): [Colors:View] ForegroundPositive
    // 39,174,96 (BreezeDark.colors:125, breeze v6.7.5)
    success: 0x27ae_60ff,
    // `defaults.info_color`: platform-facts.md (§2.1.4): [Colors:View] ForegroundActive
    // 61,174,233 (BreezeDark.colors:119, breeze v6.7.5)
    info: 0x3dae_e9ff,
    // `dark.defaults.icon_theme`: 0.7.2 kde-breeze.toml (uncited)
    icon_theme: "breeze-dark",
};

/// The UI font's family.
// `defaults.font.family`: platform-facts.md (§1.3.1): [General] font, Breeze default Noto Sans,
// 10pt, 400 (kfontsettingsdata.cpp)
const FONT_FAMILY: &str = "Noto Sans";

/// The UI font's size, in points.
// `defaults.font.size`: 0.7.2 kde-breeze.toml (uncited)
const FONT_SIZE_PT: f32 = 10.0;

/// The UI font's weight.
// `defaults.font.weight`: 0.7.2 kde-breeze.toml (uncited)
const FONT_WEIGHT: u16 = 400;

/// The monospace font's family.
// `defaults.mono_font.family`: platform-facts.md (§1.3.1): [General] fixed, Breeze default Hack,
// 10pt, 400 (kfontsettingsdata.cpp)
const MONO_FAMILY: &str = "Hack";

/// The monospace font's size, in points.
// `defaults.mono_font.size`: 0.7.2 kde-breeze.toml (uncited)
const MONO_SIZE_PT: f32 = 10.0;

/// The monospace font's weight.
// `defaults.mono_font.weight`: 0.7.2 kde-breeze.toml (uncited)
const MONO_WEIGHT: u16 = 400;

/// The line height, a multiple of the font size.
// `defaults.line_height`: platform-facts.md (§2.1.1): Noto Sans sTypo metrics, (1069+293+0)/1000
const LINE_HEIGHT: f32 = 1.36;

/// Whether a font's pixel size is rounded to a whole number.
// `defaults.font_whole_pixels`: platform-facts.md (§2.1.1): font_whole_pixels, Qt rounds a point
// size's pixel size to hundredths, then to a whole number, halves up, before the scale
// (qfontdatabase.cpp:2865-2869 at qtbase v6.12.0; measured)
const FONT_WHOLE_PIXELS: bool = true;

/// The caption's size, in points.
// `text_scale.caption.size`: platform-facts.md (§2.19): smallestReadableFont, default 8pt, 400
const CAPTION_SIZE_PT: f32 = 8.0;

/// The caption's weight.
// `text_scale.caption.weight`: 0.7.2 kde-breeze.toml (uncited)
const CAPTION_WEIGHT: u16 = 400;

/// A section heading's size, in points.
// `text_scale.section_heading.size`: platform-facts.md (§2.19): Kirigami Heading level 2, the 10pt
// body × 1.20, Font.Normal (kirigami 8319acc, controls/Heading.qml:20-21,35)
const HEADLINE_SIZE_PT: f32 = 12.0;

/// A section heading's weight.
// `text_scale.section_heading.weight`: 0.7.2 kde-breeze.toml (uncited)
const HEADLINE_WEIGHT: u16 = 400;

/// A dialog title's size, in points.
// `text_scale.dialog_title.size`: platform-facts.md (§2.19): Kirigami Heading level 1, the 10pt
// body × 1.35, Font.Normal (kirigami 8319acc, controls/Heading.qml:17-18,35)
const TITLE1_SIZE_PT: f32 = 13.5;

/// A dialog title's weight.
// `text_scale.dialog_title.weight`: 0.7.2 kde-breeze.toml (uncited)
const TITLE1_WEIGHT: u16 = 400;

/// The default corner radius.
// `defaults.border.corner_radius`: 0.7.2 kde-breeze.toml (uncited)
const CORNER_RADIUS: f32 = 5.0;

/// The large corner radius.
// `defaults.border.corner_radius_lg`: 0.7.2 kde-breeze.toml (uncited)
const CORNER_RADIUS_LG: f32 = 8.0;

/// The default border width.
// `defaults.border.line_width`: 0.7.2 kde-breeze.toml (uncited)
const BORDER_WIDTH: f32 = 1.0;

/// The default border opacity.
// `defaults.border.opacity`: 0.7.2 kde-breeze.toml (uncited)
const BORDER_OPACITY: f32 = 0.2;

/// Whether surfaces cast shadows.
// `defaults.border.shadow_enabled`: 0.7.2 kde-breeze.toml (uncited)
const SHADOW_ENABLED: bool = true;

/// The opacity of a disabled widget drawn as one group.
// `defaults.disabled_opacity`: platform-facts.md (§2.1.6): 1.0, Breeze dims a disabled control by
// its colours alone
const DISABLED_OPACITY: f32 = 1.0;

/// The focus ring's width.
// `defaults.focus_ring_width`: platform-facts.md (§2.1.5): focus_ring_width and focus_ring_offset:
// the band is PM_FocusFrameHMargin, 2, wide, and runs from 1 px inside the widget's rectangle to 1
// px outside it (breezestyle.cpp:737-739, :6642-6651 at breeze f0b1d75)
const FOCUS_RING_WIDTH: f32 = 2.0;

/// The focus ring's offset from the part's edge.
// `defaults.focus_ring_offset`: 0.7.2 kde-breeze.toml (uncited)
const FOCUS_RING_OFFSET: f32 = -1.0;

/// Whether the focus ring is shown only for keyboard focus.
// `defaults.focus_ring_keyboard_only`: platform-facts.md (§2.1.5): focus_ring_keyboard_only: drawn
// only after a focus change from the keyboard (breezestyle.cpp:1649-1652 at breeze f0b1d75)
const FOCUS_RING_KEYBOARD_ONLY: bool = true;

/// The icon set.
// `icon_set`: 0.7.2 kde-breeze.toml (uncited)
const ICON_SET: IconSet = IconSet::Freedesktop;

/// A toolbar icon's size.
// `defaults.icon_sizes.toolbar`: platform-facts.md (§2.1.8): KIconLoader's MainToolbar 22, Small
// 16, Desktop 48, Dialog 32 and Panel 48, Breeze's sizes
const ICON_TOOLBAR: f32 = 22.0;

/// A small icon's size.
// `defaults.icon_sizes.small`: 0.7.2 kde-breeze.toml (uncited)
const ICON_SMALL: f32 = 16.0;

/// A large icon's size.
// `defaults.icon_sizes.large`: 0.7.2 kde-breeze.toml (uncited)
const ICON_LARGE: f32 = 48.0;

/// A dialog icon's size.
// `defaults.icon_sizes.dialog`: 0.7.2 kde-breeze.toml (uncited)
const ICON_DIALOG: f32 = 32.0;

/// A panel icon's size.
// `defaults.icon_sizes.panel`: 0.7.2 kde-breeze.toml (uncited)
const ICON_PANEL: f32 = 48.0;

/// The space between widgets.
// `layout.widget_gap`: 0.7.2 kde-breeze.toml (uncited)
const WIDGET_GAP: f32 = 6.0;

/// A container's margin.
// `layout.container_margin`: 0.7.2 kde-breeze.toml (uncited)
const CONTAINER_MARGIN: f32 = 6.0;

/// A window's margin.
// `layout.window_margin`: 0.7.2 kde-breeze.toml (uncited)
const WINDOW_MARGIN: f32 = 10.0;

/// The space between sections.
// `layout.section_gap`: 0.7.2 kde-breeze.toml (uncited)
const SECTION_GAP: f32 = 18.0;

/// The delay before a tooltip shows, in milliseconds.
// `tooltip.show_delay`: platform-facts.md (§2.7): Qt's m_toolTipWakeUpDelay{700} for
// SH_ToolTip_WakeUpDelay (qstylehints_p.h:45; qcommonstyle.cpp:5351-5356)
const TOOLTIP_SHOW_MS: u32 = 700;

/// The caret's visible half of a blink, in milliseconds.
// `input.caret_blink_on`: platform-facts.md (§2.4): half Qt's cursor flash time, 1000 where
// CursorBlinkRate is unset (khintssettings.cpp:79-80 at plasma-integration 6.8.2;
// qwidgetlinecontrol.cpp:1497-1509 at qtbase 6.12.0)
const CARET_BLINK_ON_MS: u32 = 500;

/// The caret's hidden half of a blink, in milliseconds.
// `input.caret_blink_off`: platform-facts.md (§2.4): the other half, hidden
const CARET_BLINK_OFF_MS: u32 = 500;

/// The font DPI a point size converts at: 0.7.2's on every desktop but
/// macOS (`ResolutionContext::from_system`, `src/resolve/context.rs`; spec
/// §4, `Typography::font_dpi`).
const FONT_DPI: f32 = 96.0;

// ---------------------------------------------------------------------------
// The builder
// ---------------------------------------------------------------------------

/// The theme of `kde-breeze` for `ctx.scheme`, from the preset's constants.
pub(crate) fn resolve(ctx: &ThemeContext, _inputs: &ResolveInputs<'_>) -> ResolvedTheme {
    let constants = match ctx.scheme {
        Scheme::Light => &LIGHT,
        Scheme::Dark => &DARK,
    };
    let colors = roles(constants);
    let typography = typography(&colors);
    let unported = nothing_stated(Preset::KdeBreeze, ctx.clone());
    ResolvedTheme {
        name: Arc::from(Preset::KdeBreeze.display_name()),
        preset: Preset::KdeBreeze,
        context: ctx.clone(),
        icons: IconThemeSpec {
            set: ICON_SET,
            theme: Some(Arc::from(constants.icon_theme)),
            // 0.7.2's `Resolved::icon_theme_explicit`: the TOML states the theme
            // (`src/model/resolved.rs`).
            theme_explicit: true,
            sizes: IconSizes::new(
                Some(ICON_TOOLBAR),
                Some(ICON_SMALL),
                Some(ICON_LARGE),
                Some(ICON_DIALOG),
                Some(ICON_PANEL),
            ),
        },
        layout: LayoutMetrics {
            widget_gap: Some(WIDGET_GAP),
            container_margin: Some(CONTAINER_MARGIN),
            window_margin: Some(WINDOW_MARGIN),
            section_gap: Some(SECTION_GAP),
            corner_radius: CORNER_RADIUS,
            corner_radius_lg: CORNER_RADIUS_LG,
            border_width: BORDER_WIDTH,
            border_opacity: BORDER_OPACITY,
            shadow_enabled: SHADOW_ENABLED,
            cursor_size_px: None,
            cursor_theme: None,
        },
        timings: UiTimings {
            tooltip_show_ms: Some(TOOLTIP_SHOW_MS),
            tooltip_hide_ms: None,
            tooltip_reshow_ms: None,
            double_click_ms: None,
            drag_threshold_px: None,
            scrollbar_auto_hide_ms: None,
            menu_hover_open_ms: None,
            key_repeat_delay_ms: None,
            key_repeat_interval_ms: None,
        },
        motion_tokens: MotionTokens {
            fast: None,
            standard: None,
            emphasized: None,
            spring_spatial: None,
            spring_effects: None,
            theme_change: None,
        },
        // 0.7.2's `defaults.focus_ring_*`: a ring.
        focus_ring: FocusIndicator::new(
            FocusKind::Ring,
            Stroke::uniform(FOCUS_RING_WIDTH, colors.focus_ring),
            Stroke::None,
            FOCUS_RING_OFFSET,
            None,
            Some(if FOCUS_RING_KEYBOARD_ONLY {
                FocusTrigger::KeyboardOnly
            } else {
                FocusTrigger::Always
            }),
        ),
        caret: CaretSpec {
            width: None,
            blink_on_ms: Some(CARET_BLINK_ON_MS),
            blink_off_ms: Some(CARET_BLINK_OFF_MS),
            blink_timeout_ms: None,
            shape: None,
        },
        elevations: Arc::from([]),
        button: unported.button,
        checkbox: unported.checkbox,
        radio: unported.radio,
        scrollbar: unported.scrollbar,
        list: unported.list,
        tooltip: unported.tooltip,
        custom_colors: BTreeMap::new(),
        custom_metrics: BTreeMap::new(),
        colors,
        typography,
    }
}

/// The colour of `rgba`, 8-bit sRGB `0xRRGGBBAA`.
fn color(rgba: u32) -> Color {
    let [r, g, b, a] = rgba.to_be_bytes();
    Color::from_srgb8(r, g, b, a)
}

/// The colour roles of one scheme from its constants.
fn roles(c: &SchemeConstants) -> ColorRoles {
    let window_foreground = color(c.window_foreground);
    let selection_foreground = color(c.selection_foreground);
    // `defaults.{danger,warning,success,info}_text_color`: platform-facts.md (§2.1.4), each
    // [Colors:Window] ForegroundNormal, the window's foreground.
    let status = |fill| StatusColors {
        fill: color(fill),
        foreground: window_foreground,
        standalone: None,
        origin: RoleOrigin::Native,
    };
    let unstated = SurfaceColors::default();
    ColorRoles {
        window: SurfaceColors::new(
            Some(color(c.window_background)),
            Some(window_foreground),
            Some(color(c.window_foreground_muted)),
            Some(color(c.window_border)),
            None,
        ),
        view: SurfaceColors::new(Some(color(c.view_background)), None, None, None, None),
        header: unstated,
        sidebar: unstated,
        card: unstated,
        dialog: unstated,
        popover: unstated,
        tooltip: unstated,
        osd: unstated,
        // `defaults.selection_inactive_text_color`: platform-facts.md (§2.1.3), [Colors:Window]
        // ForegroundNormal, the window's foreground.
        selection: SelectionColors::new(
            color(c.selection_background),
            selection_foreground,
            Some(color(c.selection_unfocused_background)),
            Some(window_foreground),
        ),
        // 0.7.2: `text_selection_background` and `text_selection_color` are the selection's
        // where unstated (`src/resolve/inheritance.rs`, `resolve_defaults_internal`).
        text_selection: SelectionColors::new(
            color(c.selection_background),
            selection_foreground,
            None,
            None,
        ),
        find_highlight: None,
        scrim: None,
        // `defaults.accent_text_color`: platform-facts.md (§2.1.3), [Colors:Selection]
        // ForegroundNormal, the selection's foreground.
        accent: AccentColors {
            fill: color(c.accent),
            foreground: selection_foreground,
            standalone: None,
        },
        focus_ring: color(c.focus_ring),
        // 0.7.2: `input.caret_color` is `defaults.text_color` where unstated
        // (`src/resolve/inheritance.rs`, `resolve_safety_nets`).
        caret: window_foreground,
        link: color(c.link),
        link_visited: None,
        disabled_text: color(c.disabled_text),
        shadow: color(c.shadow),
        disabled_opacity: DISABLED_OPACITY,
        error: status(c.error),
        warning: status(c.warning),
        success: status(c.success),
        info: status(c.info),
    }
}

/// A point size in logical pixels at [`FONT_DPI`], rounded as Qt rounds it
/// where [`FONT_WHOLE_PIXELS`] (0.7.2: `src/resolve/validate.rs`, the
/// whole-pixel sizes).
fn font_px(pt: f32) -> f32 {
    let px = FontSize::Pt(pt).to_logical_px(FONT_DPI);
    if FONT_WHOLE_PIXELS {
        qt_whole_px(px)
    } else {
        px
    }
}

/// Qt's rounding of a font's pixel size: to hundredths, then to a whole
/// number, halves up (0.7.2's `qt_whole_px`, `src/resolve/validate_helpers.rs`;
/// `QFontDatabasePrivate::load`, qtbase v6.12.0 qfontdatabase.cpp:2865-2869).
fn qt_whole_px(px: f32) -> f32 {
    // Every f32 at or above 2^23 is whole already; scaling it by 100 could
    // overflow to infinity.
    if px.abs() >= 8_388_608.0 {
        return px;
    }
    ((px * 100.0).round() / 100.0).round()
}

/// The line height of a style of `size_pt` points, in logical pixels: the
/// multiplier times the size before whole-pixel rounding, converted at
/// [`FONT_DPI`] (0.7.2's `resolve_text_scale_entry`,
/// `src/resolve/inheritance.rs`, then its conversion in `validate`; spec §4,
/// `Typography`).
fn line_height_px(size_pt: f32) -> f32 {
    FontSize::Pt(LINE_HEIGHT * size_pt).to_logical_px(FONT_DPI)
}

/// A font as 0.7.2 resolves it: its family, size, weight and colour (where
/// given) stated, and its style, which 0.7.2 resolves to normal where the
/// theme states none (`src/resolve/validate_helpers.rs`, `require_font`).
fn font(family: &str, size_pt: f32, weight: u16, color: Option<Color>) -> TextStyle {
    let mut font = TextStyle {
        family: Arc::from(family),
        size_px: font_px(size_pt),
        defined_size: Some(FontSize::Pt(size_pt)),
        weight,
        style: FontStyle::Normal,
        stated: TextStated::FAMILY | TextStated::SIZE_PX | TextStated::WEIGHT | TextStated::STYLE,
        ..TextStyle::default()
    };
    if let Some(color) = color {
        font.color = color;
        font.stated |= TextStated::COLOR;
    }
    font
}

/// A text-scale entry as 0.7.2 resolves it: its size, weight and line height
/// stated, nothing else (0.7.2's `TextScaleEntry`).
fn text_scale_entry(size_pt: f32, weight: u16) -> TextStyle {
    TextStyle {
        size_px: font_px(size_pt),
        defined_size: Some(FontSize::Pt(size_pt)),
        weight,
        line_height: line_height_px(size_pt),
        stated: TextStated::SIZE_PX | TextStated::WEIGHT | TextStated::LINE_HEIGHT,
        ..TextStyle::default()
    }
}

/// The text styles. The fonts' colour is the window's foreground (0.7.2:
/// `defaults.font.color` is `defaults.text_color`, and `defaults.mono_font`'s
/// is `defaults.font`'s, where unstated: `resolve_defaults_internal`). The
/// body's line height follows the text-scale rule (spec §4); `display`, which
/// 0.7.2 resolves from the body font, is `None` and falls back to the body.
fn typography(colors: &ColorRoles) -> Typography {
    let text = colors.window.foreground;
    let mut body = font(FONT_FAMILY, FONT_SIZE_PT, FONT_WEIGHT, text);
    body.line_height = line_height_px(FONT_SIZE_PT);
    body.stated |= TextStated::LINE_HEIGHT;
    Typography {
        display: None,
        title1: Some(text_scale_entry(TITLE1_SIZE_PT, TITLE1_WEIGHT)),
        title2: None,
        title3: None,
        title4: None,
        headline: Some(text_scale_entry(HEADLINE_SIZE_PT, HEADLINE_WEIGHT)),
        body,
        callout: None,
        subheadline: None,
        caption: Some(text_scale_entry(CAPTION_SIZE_PT, CAPTION_WEIGHT)),
        monospace: font(MONO_FAMILY, MONO_SIZE_PT, MONO_WEIGHT, text),
        line_height_multiplier: LINE_HEIGHT,
        fallbacks: Arc::from([]),
        emoji_family: None,
        rendering: TextRendering::default(),
        font_dpi: FONT_DPI,
        font_whole_pixels: Some(FONT_WHOLE_PIXELS),
    }
}
