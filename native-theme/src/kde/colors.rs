// KDE color group parsing -> populate per-widget fields on ThemeMode
// Maps semantic color roles from KDE INI color groups directly to ThemeMode.

use crate::Rgba;

/// Look up a color key from a KDE INI section and parse it as RGB.
fn get_color(ini: &configparser::ini::Ini, section: &str, key: &str) -> Option<Rgba> {
    let value = ini.get(section, key)?;
    super::parse_rgb(&value)
}

/// Breeze's `Metrics::Blend_Value`, the alpha of the selection colour over a
/// checked indicator's fill (breezemetrics.h:176 and breezehelper.cpp:40 at
/// breeze v6.7.5; docs/platform-facts.md §2.5).
const BLEND_VALUE: f32 = 0.3;

/// `kdeglobals` `[KDE] frameContrast` where it is unset, KColorScheme's own
/// default (kcolorscheme.cpp:529-538 at 27066d47; docs/platform-facts.md §2.11).
const FRAME_CONTRAST_DEFAULT: f32 = 0.2;

/// `color` at `alpha`, held in 8 bits as `Rgba` holds every alpha.
fn translucent(color: Rgba, alpha: f32) -> Rgba {
    let [r, g, b, _] = color.to_f32_array();
    Rgba::from_f32(r, g, b, alpha)
}

/// `top` painted over the opaque `bottom` (source over).
fn over(top: Rgba, bottom: Rgba) -> Rgba {
    let [tr, tg, tb, alpha] = top.to_f32_array();
    let [br, bg, bb, _] = bottom.to_f32_array();
    let channel = |t: f32, b: f32| t * alpha + b * (1.0 - alpha);
    Rgba::from_f32(channel(tr, br), channel(tg, bg), channel(tb, bb), 1.0)
}

/// `KColorUtils::mix(a, b, bias)`: each channel moved from `a` towards `b` by
/// `bias` (kcolorutils.cpp:144-165 at kguiaddons 7c766f6f).
fn mix(a: Rgba, b: Rgba, bias: f32) -> Rgba {
    let [ar, ag, ab, _] = a.to_f32_array();
    let [br, bg, bb, _] = b.to_f32_array();
    let channel = |from: f32, to: f32| from + (to - from) * bias;
    Rgba::from_f32(channel(ar, br), channel(ag, bg), channel(ab, bb), 1.0)
}

/// How far a checked, non-flat Breeze button's fill moves from `Button`
/// towards `ButtonText`: `KColorUtils::mix(palette.button(),
/// palette.buttonText(), 0.125)` (breezehelper.cpp:695-699 at breeze f0b1d75;
/// docs/platform-facts.md §2.3).
const CHECKED_BUTTON_MIX: f32 = 0.125;

/// The alpha of the accent over the window in a Breeze progress bar's
/// contents, `alphaColor(fg, 0.7)` (breezehelper.cpp:1216 at breeze f0b1d75;
/// docs/platform-facts.md §2.10).
const PROGRESS_FILL_ALPHA: f32 = 0.7;

/// `(x * a + 0x7f) / 0xff` as Qt's raster engine rounds it, `BYTE_MUL`
/// (qtbase `qdrawhelper_p.h`), for 8-bit `x` and `a`.
fn byte_mul(x: u64, a: u64) -> u64 {
    let t = x.saturating_mul(a);
    t.saturating_add(t.wrapping_shr(8))
        .saturating_add(0x80)
        .wrapping_shr(8)
}

/// `x * a / 0xffff` rounded, for 16-bit `x` and `a` (qtbase `qt_div_65535`).
fn mul_65535(x: u64, a: u64) -> u64 {
    let t = x.saturating_mul(a);
    t.saturating_add(t.wrapping_shr(16))
        .saturating_add(0x8000)
        .wrapping_shr(16)
}

/// A 16-bit value to 8 bits, `x / 257` rounded to nearest.
fn div_257(x: u64) -> u64 {
    let t = x.saturating_add(0x80);
    t.saturating_sub(t.wrapping_shr(8)).wrapping_shr(8)
}

/// `KColorUtils::overlayColors(base, paint)` for an opaque `paint` colour
/// given the alpha `alpha` by `QColor::setAlphaF`: `paint` filled
/// source-over onto the opaque `base` in a 1×1 `ARGB32_Premultiplied` image
/// (kcolorutils.cpp:168-181 at kguiaddons 7c766f6f). Qt premultiplies the
/// colour in 16 bits, stores it in 8, and blends the 8-bit destination by the
/// 8-bit inverse alpha; this arithmetic reproduces Qt 6.11.2's pixel on
/// 20 000 random inputs, so the result is Breeze's to the unit.
fn overlay_colors(base: Rgba, paint: Rgba, alpha: f32) -> Rgba {
    // QColor keeps alpha in 16 bits: qRound(alpha * 65535), in f32.
    let alpha16 = (alpha.clamp(0.0, 1.0) * 65535.0 + 0.5).floor() as u64;
    let alpha8 = div_257(alpha16);
    let inverse = 255_u64.saturating_sub(alpha8);
    let channel = |p: u8, b: u8| {
        let premultiplied = div_257(mul_65535(u64::from(p).saturating_mul(257), alpha16));
        let v = premultiplied.saturating_add(byte_mul(u64::from(b), inverse));
        u8::try_from(v).unwrap_or(u8::MAX)
    };
    Rgba::rgb(
        channel(paint.r, base.r),
        channel(paint.g, base.g),
        channel(paint.b, base.b),
    )
}

/// `KColorScheme::frameContrast()`: `[KDE] frameContrast`, clamped to
/// 0.0..=1.0, or its default where unset or not a number.
fn frame_contrast(ini: &configparser::ini::Ini) -> f32 {
    ini.get("KDE", "frameContrast")
        .and_then(|v| v.trim().parse::<f32>().ok())
        .filter(|v| v.is_finite())
        .map_or(FRAME_CONTRAST_DEFAULT, |v| v.clamp(0.0, 1.0))
}

/// Populate a ThemeMode with colors from KDE INI color groups.
///
/// Maps all standard KDE color groups (View, Window, Button, Selection,
/// Tooltip, Complementary, Header, WM) to per-widget fields on the variant.
/// Missing INI keys result in None fields (no hardcoded fallbacks).
pub(crate) fn populate_colors(ini: &configparser::ini::Ini, variant: &mut crate::ThemeMode) {
    let window_fg = get_color(ini, "Colors:Window", "ForegroundNormal");

    // === defaults-level colors ===
    variant.defaults.accent_color = get_color(ini, "Colors:View", "DecorationFocus");
    variant.defaults.background_color = get_color(ini, "Colors:Window", "BackgroundNormal");
    variant.defaults.text_color = window_fg;
    variant.defaults.surface_color = get_color(ini, "Colors:View", "BackgroundNormal");
    // border: not set by reader -- KDE has no native border color API.
    // The preset provides the correct neutral gray (platform-facts: "(preset)").
    variant.defaults.muted_color = get_color(ini, "Colors:Window", "ForegroundInactive");
    // KDE does not expose shadow color in kdeglobals
    variant.defaults.link_color = get_color(ini, "Colors:View", "ForegroundLink");
    variant.defaults.focus_ring_color = get_color(ini, "Colors:View", "DecorationFocus");

    // Selection
    variant.defaults.selection_background = get_color(ini, "Colors:Selection", "BackgroundNormal");
    variant.defaults.selection_text_color = get_color(ini, "Colors:Selection", "ForegroundNormal");
    // accent_text_color: foreground on accent-colored backgrounds (platform-facts 2.1.3)
    variant.defaults.accent_text_color = get_color(ini, "Colors:Selection", "ForegroundNormal");

    // Status colors
    variant.defaults.danger_color = get_color(ini, "Colors:View", "ForegroundNegative");
    variant.defaults.danger_text_color = window_fg;
    variant.defaults.warning_color = get_color(ini, "Colors:View", "ForegroundNeutral");
    variant.defaults.warning_text_color = window_fg;
    variant.defaults.success_color = get_color(ini, "Colors:View", "ForegroundPositive");
    variant.defaults.success_text_color = window_fg;
    variant.defaults.info_color = get_color(ini, "Colors:View", "ForegroundActive");
    variant.defaults.info_text_color = window_fg;

    // Disabled
    variant.defaults.disabled_text_color = get_color(ini, "Colors:View", "ForegroundInactive");

    // === per-widget colors ===

    // Button
    variant.button.background_color = get_color(ini, "Colors:Button", "BackgroundNormal");
    if let Some(color) = get_color(ini, "Colors:Button", "ForegroundNormal") {
        variant.button.font.get_or_insert_default().color = Some(color);
    }
    // A checked button (docs/platform-facts.md §2.3): the button colour moved
    // an eighth towards the button text, lettered in the button text.
    let button_fg = get_color(ini, "Colors:Button", "ForegroundNormal");
    variant.button.checked_background = variant
        .button
        .background_color
        .zip(button_fg)
        .map(|(button, text)| mix(button, text, CHECKED_BUTTON_MIX));
    variant.button.checked_text_color = button_fg;

    // Tooltip
    variant.tooltip.background_color = get_color(ini, "Colors:Tooltip", "BackgroundNormal");
    if let Some(color) = get_color(ini, "Colors:Tooltip", "ForegroundNormal") {
        variant.tooltip.font.get_or_insert_default().color = Some(color);
    }

    // Sidebar (from Complementary group)
    variant.sidebar.background_color = get_color(ini, "Colors:Complementary", "BackgroundNormal");
    if let Some(color) = get_color(ini, "Colors:Complementary", "ForegroundNormal") {
        variant.sidebar.font.get_or_insert_default().color = Some(color);
    }

    // Input
    variant.input.background_color = get_color(ini, "Colors:View", "BackgroundNormal");
    if let Some(color) = get_color(ini, "Colors:View", "ForegroundNormal") {
        variant.input.font.get_or_insert_default().color = Some(color);
    }
    // KDE-02: placeholder from View/ForegroundInactive
    variant.input.placeholder_color = get_color(ini, "Colors:View", "ForegroundInactive");
    // input.caret from View/DecorationFocus (the focus decoration color)
    variant.input.caret_color = get_color(ini, "Colors:View", "DecorationFocus");

    // Checkbox and radio (docs/platform-facts.md §2.5): Breeze fills an
    // unchecked indicator with the button colour and outlines it in
    // `separatorColor()`; a checked one is that fill under the selection
    // colour at `Metrics::Blend_Value`, outlined in the selection colour; the
    // mark is the text colour.
    let button_bg = get_color(ini, "Colors:Button", "BackgroundNormal");
    let selection_bg = get_color(ini, "Colors:Selection", "BackgroundNormal");
    variant.checkbox.unchecked_background = button_bg;
    variant.checkbox.checked_background = button_bg
        .zip(selection_bg)
        .map(|(button, selection)| over(translucent(selection, BLEND_VALUE), button));
    variant.checkbox.indicator_color = get_color(ini, "Colors:View", "ForegroundNormal");
    if let Some(color) = selection_bg {
        variant.checkbox.border.get_or_insert_default().color = Some(color);
    }
    variant.checkbox.unchecked_border_color = get_color(ini, "Colors:Window", "BackgroundNormal")
        .zip(window_fg)
        .map(|(window, text)| mix(window, text, frame_contrast(ini)));

    // Tab (docs/platform-facts.md §2.11): Breeze marks the selected tab with a
    // strip of `QPalette::Highlight`, the selection background.
    variant.tab.active_indicator_color = selection_bg;

    // Progress bar (docs/platform-facts.md §2.10): Breeze fills the contents
    // with `QPalette::Accent`, the selection background
    // (kcolorscheme.cpp:681 at 27066d47), at alpha 0.7 over the window, and
    // strokes the groove in the window text at alpha `frameContrast`.
    let window_bg = get_color(ini, "Colors:Window", "BackgroundNormal");
    variant.progress_bar.fill_color = selection_bg
        .zip(window_bg)
        .map(|(accent, window)| overlay_colors(window, accent, PROGRESS_FILL_ALPHA));
    if let Some(text) = window_fg {
        variant.progress_bar.border.get_or_insert_default().color =
            Some(translucent(text, frame_contrast(ini)));
    }

    // Popover (from View)
    variant.popover.background_color = get_color(ini, "Colors:View", "BackgroundNormal");
    if let Some(color) = get_color(ini, "Colors:View", "ForegroundNormal") {
        variant.popover.font.get_or_insert_default().color = Some(color);
    }

    // Separator
    variant.separator.line_color = get_color(ini, "Colors:Window", "ForegroundInactive");

    // KDE-02: list fields (Colors:View is the native source for list/table content areas)
    variant.list.background_color = get_color(ini, "Colors:View", "BackgroundNormal");
    if let Some(color) = get_color(ini, "Colors:View", "ForegroundNormal") {
        variant.list.item_font.get_or_insert_default().color = Some(color);
    }
    variant.list.alternate_row_background = get_color(ini, "Colors:View", "BackgroundAlternate");
    variant.list.header_background = get_color(ini, "Colors:Header", "BackgroundNormal");
    if let Some(color) = get_color(ini, "Colors:Header", "ForegroundNormal") {
        variant.list.header_font.get_or_insert_default().color = Some(color);
    }

    // KDE-02: link.visited
    variant.link.visited_text_color = get_color(ini, "Colors:View", "ForegroundVisited");
    // link.font.color from View/ForegroundLink
    if let Some(color) = get_color(ini, "Colors:View", "ForegroundLink") {
        variant.link.font.get_or_insert_default().color = Some(color);
    }

    // === KDE-01: Window Manager title bar colors ===
    variant.window.title_bar_background = get_color(ini, "WM", "activeBackground");
    if let Some(color) = get_color(ini, "WM", "activeForeground") {
        variant.window.title_bar_font.get_or_insert_default().color = Some(color);
    }
    variant.window.inactive_title_bar_background = get_color(ini, "WM", "inactiveBackground");
    variant.window.inactive_title_bar_text_color = get_color(ini, "WM", "inactiveForeground");
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use crate::ThemeMode;

    /// Full Breeze Dark kdeglobals fixture with all 6 color groups + WM + Header.
    const BREEZE_DARK: &str = "\
[Colors:View]
BackgroundNormal=35,38,41
BackgroundAlternate=30,33,36
ForegroundNormal=252,252,252
ForegroundInactive=161,169,177
ForegroundActive=61,174,233
ForegroundLink=29,153,243
ForegroundNegative=218,68,83
ForegroundNeutral=246,116,0
ForegroundPositive=39,174,96
ForegroundVisited=155,89,182
DecorationFocus=61,174,233
DecorationHover=29,153,243

[Colors:Window]
BackgroundNormal=49,54,59
BackgroundAlternate=44,49,54
ForegroundNormal=239,240,241
ForegroundInactive=161,169,177
ForegroundActive=61,174,233
ForegroundLink=29,153,243
ForegroundNegative=218,68,83
ForegroundNeutral=246,116,0
ForegroundPositive=39,174,96
DecorationFocus=61,174,233
DecorationHover=29,153,243

[Colors:Button]
BackgroundNormal=49,54,59
BackgroundAlternate=44,49,54
ForegroundNormal=239,240,241
ForegroundInactive=161,169,177

[Colors:Selection]
BackgroundNormal=61,174,233
BackgroundAlternate=29,153,243
ForegroundNormal=252,252,252
ForegroundInactive=161,169,177

[Colors:Tooltip]
BackgroundNormal=49,54,59
ForegroundNormal=252,252,252

[Colors:Complementary]
BackgroundNormal=42,46,50
ForegroundNormal=239,240,241

[Colors:Header]
BackgroundNormal=35,38,41
ForegroundNormal=252,252,252

[WM]
activeBackground=49,54,59
activeForeground=239,240,241
inactiveBackground=42,46,50
inactiveForeground=161,169,177
";

    fn populate_fixture(content: &str) -> ThemeMode {
        let mut ini = super::super::create_kde_parser();
        ini.read(content.to_string()).unwrap();
        let mut variant = ThemeMode::default();
        populate_colors(&ini, &mut variant);
        variant
    }

    // === defaults-level color mapping ===

    #[test]
    fn test_accent_from_view_decoration_focus() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(v.defaults.accent_color, Some(Rgba::rgb(61, 174, 233)));
    }

    #[test]
    fn test_background_from_window() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(v.defaults.background_color, Some(Rgba::rgb(49, 54, 59)));
    }

    #[test]
    fn test_text_color_from_window() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(v.defaults.text_color, Some(Rgba::rgb(239, 240, 241)));
    }

    #[test]
    fn test_surface_from_view() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(v.defaults.surface_color, Some(Rgba::rgb(35, 38, 41)));
    }

    #[test]
    fn test_border_not_set_by_reader() {
        // Border is a preset value, not reader-provided (platform-facts: "(preset)")
        let v = populate_fixture(BREEZE_DARK);
        assert!(v.defaults.border.is_empty());
    }

    #[test]
    fn test_muted_from_window_foreground_inactive() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(v.defaults.muted_color, Some(Rgba::rgb(161, 169, 177)));
    }

    // === Status colors ===

    #[test]
    fn test_status_colors() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(v.defaults.danger_color, Some(Rgba::rgb(218, 68, 83)));
        assert_eq!(v.defaults.warning_color, Some(Rgba::rgb(246, 116, 0)));
        assert_eq!(v.defaults.success_color, Some(Rgba::rgb(39, 174, 96)));
        assert_eq!(v.defaults.info_color, Some(Rgba::rgb(61, 174, 233)));
    }

    // === Selection ===

    #[test]
    fn test_selection_colors() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(
            v.defaults.selection_background,
            Some(Rgba::rgb(61, 174, 233))
        );
        assert_eq!(
            v.defaults.selection_text_color,
            Some(Rgba::rgb(252, 252, 252))
        );
    }

    // === Per-widget: Checkbox and radio ===

    /// The groups Breeze's checkbox reads, from BreezeLight.colors and
    /// BreezeDark.colors at breeze v6.7.5 (:29-39, :85-95, :113-137).
    fn breeze_checkbox_groups(
        window: &str,
        window_fg: &str,
        button: &str,
        view_fg: &str,
    ) -> String {
        format!(
            "[Colors:Window]\nBackgroundNormal={window}\nForegroundNormal={window_fg}\n\n\
             [Colors:Button]\nBackgroundNormal={button}\n\n\
             [Colors:View]\nForegroundNormal={view_fg}\n\n\
             [Colors:Selection]\nBackgroundNormal=61,174,233\n"
        )
    }

    /// docs/platform-facts.md §2.5's KDE cells, which the kde-breeze preset states.
    #[test]
    fn test_checkbox_colors_are_breezes() {
        let cases = [
            (
                breeze_checkbox_groups("239,240,241", "35,38,41", "252,252,252", "35,38,41"),
                [
                    (194, 228, 246),
                    (252, 252, 252),
                    (35, 38, 41),
                    (198, 200, 201),
                ],
            ),
            (
                breeze_checkbox_groups("32,35,38", "252,252,252", "41,44,48", "252,252,252"),
                [(47, 83, 104), (41, 44, 48), (252, 252, 252), (76, 78, 81)],
            ),
        ];
        for (content, [checked, unchecked, mark, unchecked_border]) in cases {
            let v = populate_fixture(&content);
            let rgb = |(r, g, b): (u8, u8, u8)| Some(Rgba::rgb(r, g, b));
            assert_eq!(v.checkbox.checked_background, rgb(checked));
            assert_eq!(v.checkbox.unchecked_background, rgb(unchecked));
            assert_eq!(v.checkbox.indicator_color, rgb(mark));
            assert_eq!(v.checkbox.unchecked_border_color, rgb(unchecked_border));
            assert_eq!(
                v.checkbox.border.as_ref().and_then(|b| b.color),
                rgb((61, 174, 233))
            );
        }
    }

    #[test]
    fn test_checkbox_unchecked_border_follows_frame_contrast() {
        let content = breeze_checkbox_groups("0,0,0", "200,100,0", "0,0,0", "0,0,0");
        let with = |contrast: &str| {
            populate_fixture(&format!("{content}\n[KDE]\nframeContrast={contrast}\n"))
                .checkbox
                .unchecked_border_color
        };
        assert_eq!(with("0.5"), Some(Rgba::rgb(100, 50, 0)));
        assert_eq!(with("2"), Some(Rgba::rgb(200, 100, 0)), "clamped to 1");
        assert_eq!(with("none"), Some(Rgba::rgb(40, 20, 0)), "the 0.2 default");
    }

    /// Pixels `KColorUtils::overlayColors` returned under Qt 6.11.2 and KF6
    /// KGuiAddons for a colour `setAlphaF`'d to the alpha, over an opaque base
    /// (scratchpad program `ovtest.cpp`, 20 000 random inputs, of which these
    /// are six); `overlay_colors` matched all of them.
    #[test]
    fn overlay_colors_is_qts_pixel() {
        let cases = [
            ((220, 4, 101), (170, 31, 173), 0.29, (205, 12, 122)),
            ((90, 218, 229), (172, 27, 30), 0.95, (168, 37, 40)),
            ((209, 238, 57), (16, 203, 72), 0.48, (117, 221, 65)),
            ((116, 55, 154), (111, 144, 12), 0.19, (115, 72, 127)),
            ((164, 249, 31), (161, 98, 185), 0.75, (162, 135, 147)),
            ((148, 102, 144), (95, 160, 85), 0.04, (146, 104, 141)),
        ];
        for (base, paint, alpha, expected) in cases {
            let rgb = |(r, g, b): (u8, u8, u8)| Rgba::rgb(r, g, b);
            assert_eq!(
                overlay_colors(rgb(base), rgb(paint), alpha),
                rgb(expected),
                "{base:?} under {paint:?} at {alpha}"
            );
        }
    }

    /// docs/platform-facts.md §2.10's KDE cells, which the kde-breeze preset
    /// states: the accent at 0.7 over the window, and the window text at the
    /// frame contrast for the outline.
    #[test]
    fn test_progress_bar_colors_are_breezes() {
        let cases = [
            (
                breeze_checkbox_groups("239,240,241", "35,38,41", "252,252,252", "35,38,41"),
                Rgba::rgb(0x72, 0xc2, 0xeb),
                Rgba::new(0x23, 0x26, 0x29, 0x33),
            ),
            (
                breeze_checkbox_groups("32,35,38", "252,252,252", "41,44,48", "252,252,252"),
                Rgba::rgb(0x35, 0x84, 0xae),
                Rgba::new(0xfc, 0xfc, 0xfc, 0x33),
            ),
        ];
        for (content, fill, outline) in cases {
            let v = populate_fixture(&content);
            assert_eq!(v.progress_bar.fill_color, Some(fill));
            assert_eq!(
                v.progress_bar.border.as_ref().and_then(|b| b.color),
                Some(outline)
            );
        }
    }

    /// docs/platform-facts.md §2.3's KDE cells, which the kde-breeze preset
    /// states: a checked button is `Button` mixed an eighth towards
    /// `ButtonText`, lettered in `ButtonText`.
    #[test]
    fn test_checked_button_colors_are_breezes() {
        let cases = [
            ("252,252,252", "35,38,41", Rgba::rgb(0xe1, 0xe1, 0xe2)),
            ("41,44,48", "252,252,252", Rgba::rgb(0x43, 0x46, 0x4a)),
        ];
        for (button, text, checked) in cases {
            let v = populate_fixture(&format!(
                "[Colors:Button]\nBackgroundNormal={button}\nForegroundNormal={text}\n"
            ));
            assert_eq!(v.button.checked_background, Some(checked), "{button}");
            assert_eq!(
                v.button.checked_text_color,
                v.button.font.as_ref().and_then(|f| f.color)
            );
        }
    }

    // === Per-widget: Button ===

    #[test]
    fn test_button_background_from_colors_button() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(v.button.background_color, Some(Rgba::rgb(49, 54, 59)));
    }

    #[test]
    fn test_button_font_color_from_colors_button() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(
            v.button.font.as_ref().and_then(|f| f.color),
            Some(Rgba::rgb(239, 240, 241))
        );
    }

    // === Per-widget: Tooltip ===

    #[test]
    fn test_tooltip_background_from_colors_tooltip() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(v.tooltip.background_color, Some(Rgba::rgb(49, 54, 59)));
    }

    #[test]
    fn test_tooltip_font_color_from_colors_tooltip() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(
            v.tooltip.font.as_ref().and_then(|f| f.color),
            Some(Rgba::rgb(252, 252, 252))
        );
    }

    // === Per-widget: Sidebar (Complementary) ===

    #[test]
    fn test_sidebar_background_from_complementary() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(v.sidebar.background_color, Some(Rgba::rgb(42, 46, 50)));
    }

    #[test]
    fn test_sidebar_font_color_from_complementary() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(
            v.sidebar.font.as_ref().and_then(|f| f.color),
            Some(Rgba::rgb(239, 240, 241))
        );
    }

    // === KDE-01: Title bar from WM ===

    #[test]
    fn test_title_bar_background_from_wm_active() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(v.window.title_bar_background, Some(Rgba::rgb(49, 54, 59)));
    }

    #[test]
    fn test_title_bar_font_color_from_wm_active() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(
            v.window.title_bar_font.as_ref().and_then(|f| f.color),
            Some(Rgba::rgb(239, 240, 241))
        );
    }

    #[test]
    fn test_inactive_title_bar_background_from_wm() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(
            v.window.inactive_title_bar_background,
            Some(Rgba::rgb(42, 46, 50))
        );
    }

    #[test]
    fn test_inactive_title_bar_text_color_from_wm() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(
            v.window.inactive_title_bar_text_color,
            Some(Rgba::rgb(161, 169, 177))
        );
    }

    // === KDE-02: Input placeholder and caret ===

    #[test]
    fn test_input_placeholder_from_view_foreground_inactive() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(v.input.placeholder_color, Some(Rgba::rgb(161, 169, 177)));
    }

    #[test]
    fn test_input_caret_from_view_decoration_focus() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(v.input.caret_color, Some(Rgba::rgb(61, 174, 233)));
    }

    // === accent_text_color from Selection ===

    #[test]
    fn test_accent_text_color_from_selection() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(v.defaults.accent_text_color, Some(Rgba::rgb(252, 252, 252)));
    }

    // === KDE-02: List background/item_font from View ===

    #[test]
    fn test_list_background_from_view() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(v.list.background_color, Some(Rgba::rgb(35, 38, 41)));
    }

    #[test]
    fn test_list_item_font_color_from_view() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(
            v.list.item_font.as_ref().and_then(|f| f.color),
            Some(Rgba::rgb(252, 252, 252))
        );
    }

    // === KDE-02: List alternate_row ===

    #[test]
    fn test_list_alternate_row_from_view() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(v.list.alternate_row_background, Some(Rgba::rgb(30, 33, 36)));
    }

    // === KDE-02: List header from Header group ===

    #[test]
    fn test_list_header_background_from_header() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(v.list.header_background, Some(Rgba::rgb(35, 38, 41)));
    }

    #[test]
    fn test_list_header_font_color_from_header() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(
            v.list.header_font.as_ref().and_then(|f| f.color),
            Some(Rgba::rgb(252, 252, 252))
        );
    }

    // === KDE-02: Link visited ===

    #[test]
    fn test_link_visited_from_view() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(v.link.visited_text_color, Some(Rgba::rgb(155, 89, 182)));
    }

    // === Sidebar None when Complementary missing ===

    #[test]
    fn test_sidebar_none_without_complementary() {
        let content = "\
[Colors:View]
BackgroundNormal=35,38,41
ForegroundNormal=252,252,252
";
        let v = populate_fixture(content);
        assert!(v.sidebar.background_color.is_none());
        assert!(v.sidebar.font.is_none());
    }

    // === Empty INI ===

    #[test]
    fn test_empty_ini() {
        let ini = super::super::create_kde_parser();
        let mut variant = ThemeMode::default();
        populate_colors(&ini, &mut variant);
        assert!(variant.defaults.accent_color.is_none());
        assert!(variant.defaults.background_color.is_none());
        assert!(variant.defaults.text_color.is_none());
        assert!(variant.button.background_color.is_none());
    }

    // === Partial sections ===

    #[test]
    fn test_partial_sections() {
        let content = "\
[Colors:Window]
BackgroundNormal=49,54,59
ForegroundNormal=239,240,241
ForegroundInactive=161,169,177
DecorationFocus=61,174,233
";
        let v = populate_fixture(content);
        assert_eq!(v.defaults.background_color, Some(Rgba::rgb(49, 54, 59)));
        assert_eq!(v.defaults.text_color, Some(Rgba::rgb(239, 240, 241)));
        assert_eq!(v.defaults.muted_color, Some(Rgba::rgb(161, 169, 177)));
        assert!(v.defaults.border.is_empty()); // border not set by reader
        assert!(v.defaults.accent_color.is_none());
        assert!(v.defaults.surface_color.is_none());
        assert!(v.button.background_color.is_none());
    }

    // === Malformed values ===

    #[test]
    fn test_malformed_values() {
        let content = "\
[Colors:View]
BackgroundNormal=abc,def,ghi
ForegroundNormal=252,252,252
DecorationFocus=61,174,233
";
        let v = populate_fixture(content);
        assert!(v.defaults.surface_color.is_none()); // BackgroundNormal was malformed
        assert_eq!(v.defaults.accent_color, Some(Rgba::rgb(61, 174, 233)));
    }

    // === Full mapping: all non-None fields populated ===

    #[test]
    fn test_full_mapping_completeness() {
        let v = populate_fixture(BREEZE_DARK);

        // defaults-level
        assert!(v.defaults.accent_color.is_some(), "accent_color missing");
        assert!(
            v.defaults.background_color.is_some(),
            "background_color missing"
        );
        assert!(v.defaults.text_color.is_some(), "text_color missing");
        assert!(v.defaults.surface_color.is_some(), "surface_color missing");
        // border is not set by reader (preset value)
        assert!(v.defaults.muted_color.is_some(), "muted_color missing");
        assert!(v.defaults.link_color.is_some(), "link_color missing");
        assert!(
            v.defaults.selection_background.is_some(),
            "selection_background missing"
        );
        assert!(
            v.defaults.selection_text_color.is_some(),
            "selection_text_color missing"
        );
        assert!(
            v.defaults.accent_text_color.is_some(),
            "accent_text_color missing"
        );
        assert!(v.defaults.danger_color.is_some(), "danger_color missing");
        assert!(v.defaults.warning_color.is_some(), "warning_color missing");
        assert!(v.defaults.success_color.is_some(), "success_color missing");
        assert!(v.defaults.info_color.is_some(), "info_color missing");

        // per-widget
        assert!(
            v.button.background_color.is_some(),
            "button.background_color missing"
        );
        assert!(
            v.button.font.as_ref().and_then(|f| f.color).is_some(),
            "button.font.color missing"
        );
        assert!(
            v.tooltip.background_color.is_some(),
            "tooltip.background_color missing"
        );
        assert!(
            v.tooltip.font.as_ref().and_then(|f| f.color).is_some(),
            "tooltip.font.color missing"
        );
        assert!(
            v.sidebar.background_color.is_some(),
            "sidebar.background_color missing"
        );
        assert!(
            v.sidebar.font.as_ref().and_then(|f| f.color).is_some(),
            "sidebar.font.color missing"
        );
        assert!(
            v.input.background_color.is_some(),
            "input.background_color missing"
        );
        assert!(
            v.input.font.as_ref().and_then(|f| f.color).is_some(),
            "input.font.color missing"
        );
        assert!(
            v.input.placeholder_color.is_some(),
            "input.placeholder_color missing"
        );
        assert!(v.input.caret_color.is_some(), "input.caret_color missing");
        assert!(
            v.separator.line_color.is_some(),
            "separator.line_color missing"
        );
        assert!(
            v.list.background_color.is_some(),
            "list.background_color missing"
        );
        assert!(
            v.list.item_font.as_ref().and_then(|f| f.color).is_some(),
            "list.item_font.color missing"
        );
        assert!(
            v.list.alternate_row_background.is_some(),
            "list.alternate_row_background missing"
        );
        assert!(
            v.list.header_background.is_some(),
            "list.header_background missing"
        );
        assert!(
            v.list.header_font.as_ref().and_then(|f| f.color).is_some(),
            "list.header_font.color missing"
        );
        assert!(
            v.link.visited_text_color.is_some(),
            "link.visited_text_color missing"
        );

        // WM title bar
        assert!(
            v.window.title_bar_background.is_some(),
            "window.title_bar_background missing"
        );
        assert!(
            v.window
                .title_bar_font
                .as_ref()
                .and_then(|f| f.color)
                .is_some(),
            "window.title_bar_font.color missing"
        );
        assert!(
            v.window.inactive_title_bar_background.is_some(),
            "window.inactive_title_bar_background missing"
        );
        assert!(
            v.window.inactive_title_bar_text_color.is_some(),
            "window.inactive_title_bar_text_color missing"
        );
    }
}
