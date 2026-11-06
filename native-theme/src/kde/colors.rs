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
const FRAME_CONTRAST_DEFAULT: f64 = 0.2;

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

/// How far a checked, non-flat Breeze button's fill moves from `Button`
/// towards `ButtonText`: `KColorUtils::mix(palette.button(),
/// palette.buttonText(), 0.125)` (breezehelper.cpp:695-699 at breeze f0b1d75;
/// docs/platform-facts.md §2.3).
const CHECKED_BUTTON_MIX: f64 = 0.125;

/// How far a pressed, enabled, non-flat Breeze button's fill moves from
/// `Button` towards `Highlight`: `KColorUtils::mix(palette.button().color(),
/// highlightColor, Metrics::Blend_Value)`, the `qreal` 0.3
/// (breezehelper.cpp:694, breezemetrics.h:176 at breeze f0b1d75;
/// docs/platform-facts.md §2.3).
const PRESSED_BUTTON_MIX: f64 = 0.3;

/// The alpha of the hover decoration a hovered, unselected Breeze tab is
/// filled with, `alphaColor(hoverColor(palette), 0.2)` (breezehelper.cpp:1497
/// at breeze f0b1d75; docs/platform-facts.md §2.11).
const TAB_HOVER_ALPHA: f32 = 0.2;

/// The alpha a Breeze scroll-bar handle's colour is filled with over the
/// window, `overlayColors(bg, alphaColor(fg, 0.5))` (breezehelper.cpp:1284
/// at breeze f0b1d75; docs/platform-facts.md §2.8).
const SCROLLBAR_HANDLE_ALPHA: f32 = 0.5;

/// The alpha of the accent over the window in a Breeze progress bar's
/// contents, `alphaColor(fg, 0.7)` (breezehelper.cpp:1216 at breeze f0b1d75;
/// docs/platform-facts.md §2.10).
const PROGRESS_FILL_ALPHA: f32 = 0.7;

/// `KColorScheme::frameContrast()`: `[KDE] frameContrast`, clamped to
/// 0.0..=1.0, or its default where unset or not a number; a `qreal`, as
/// KConfig reads it.
fn frame_contrast(ini: &configparser::ini::Ini) -> f64 {
    ini.get("KDE", "frameContrast")
        .and_then(|v| v.trim().parse::<f64>().ok())
        .filter(|v| v.is_finite())
        .map_or(FRAME_CONTRAST_DEFAULT, |v| v.clamp(0.0, 1.0))
}

/// Populate a ThemeMode with colors from KDE INI color groups.
///
/// Maps all standard KDE color groups (View, Window, Button, Selection,
/// Tooltip, Header, WM) to per-widget fields on the variant.
/// Missing INI keys result in None fields (no hardcoded fallbacks), so a
/// colour the reader cannot derive keeps the preset's value.
///
/// Some colours Plasma computes from the scheme, and the reader computes
/// them as Breeze, Kirigami and qqc2-desktop-style do, in Qt's arithmetic:
/// the checked button, the checkbox and progress-bar blends, the switch
/// tracks and outlines, the toolbar's fill and edge, and the popover's and the
/// card's edges. The blends on the accent (the checked checkbox, the progress
/// fill, the switch's on track and its outline) come from kdeglobals'
/// `[Colors:Selection] BackgroundNormal`, so an application's `with_overlay` accent or the
/// portal's (`gnome::apply_accent`, applied by `GnomePortalKdeReader`) does
/// not move them.
///
/// The toolbar takes the Header set's fill as Breeze paints it where the
/// window decoration's border size is automatic, None or NoSides
/// (breezehelper.cpp:1904-1941 at breeze f0b1d75); the reader does not read
/// kwinrc and assumes Plasma's default, automatic.
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
    // A pressed button (docs/platform-facts.md §2.3): the button colour moved
    // `Metrics::Blend_Value` towards the highlight (breezehelper.cpp:694 at
    // breeze f0b1d75); its text stays the button text, which
    // `active_text_color` inherits from the font.
    variant.button.active_background = variant
        .button
        .background_color
        .zip(get_color(ini, "Colors:Selection", "BackgroundNormal"))
        .map(|(button, highlight)| mix(button, highlight, PRESSED_BUTTON_MIX));
    // A hovered button keeps its fill and its text: Breeze's hover changes
    // only the pen (breezehelper.cpp:693-707, :710-712 at breeze f0b1d75), and
    // a non-flat button is lettered in `QPalette::ButtonText` in every state
    // (breezestyle.cpp:5350-5362; docs/platform-facts.md §2.3).
    variant.button.hover_background = variant.button.background_color;
    variant.button.hover_text_color = button_fg;

    // Tooltip
    variant.tooltip.background_color = get_color(ini, "Colors:Tooltip", "BackgroundNormal");
    if let Some(color) = get_color(ini, "Colors:Tooltip", "ForegroundNormal") {
        variant.tooltip.font.get_or_insert_default().color = Some(color);
    }

    // Sidebar (docs/platform-facts.md:1370, :1372, §2.12): the View set, as the KDE HIG's
    // sidebar, the non-modal GlobalDrawer, is docked in it (Kirigami GlobalDrawer.qml:337)
    // and System Settings' sidebar sets it (CategoriesPage.qml:19-20).
    variant.sidebar.background_color = get_color(ini, "Colors:View", "BackgroundNormal");
    if let Some(color) = get_color(ini, "Colors:View", "ForegroundNormal") {
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
    // A field's frame (docs/platform-facts.md §2.4): `frameOutlineColor`
    // strokes a hovered one in `hoverColor()` and a focused one in
    // `focusColor()`, focus winning, the View set's DecorationHover and
    // DecorationFocus (breezehelper.cpp:89-90, :142-168 at breeze f0b1d75).
    let view_hover = get_color(ini, "Colors:View", "DecorationHover");
    variant.input.hover_border_color = view_hover;
    variant.input.focus_border_color = get_color(ini, "Colors:View", "DecorationFocus");

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

    // Slider (docs/platform-facts.md §2.9): Breeze fills the handle with the
    // button colour (breezestyle.cpp:8122 at breeze f0b1d75).
    variant.slider.thumb_color = button_bg;

    // Switch (docs/platform-facts.md §2.21): qqc2-desktop-style's tracks, the
    // off one the Button set's text tinted by its background, the on one the
    // highlight at half alpha over it; hover changes no fill (SwitchIndicator.qml
    // :49, :62, :77 at v6.31.0). The handle is the Button set's background
    // (:29). The off track and the handle are outlined in the Button set's
    // background moved towards its text by the frame contrast, the on track
    // in the highlight (:27, :30, :50, :78).
    let switch_off = button_bg
        .zip(button_fg)
        .map(|(background, text)| switch_off_track(background, text));
    variant.switch.unchecked_background = switch_off.map(QtColor::to_rgba);
    variant.switch.checked_background = switch_off
        .zip(selection_bg)
        .map(|(off, highlight)| switch_on_track(off, highlight));
    variant.switch.hover_unchecked_background = variant.switch.unchecked_background;
    variant.switch.hover_checked_background = variant.switch.checked_background;
    variant.switch.thumb_background = button_bg;
    let switch_edge = button_bg
        .zip(button_fg)
        .map(|(bg, fg)| linear_interpolation(bg, fg, frame_contrast(ini)));
    variant.switch.unchecked_track_border_color = switch_edge;
    variant.switch.thumb_border_color = switch_edge;
    variant.switch.checked_track_border_color = selection_bg;

    // Tab (docs/platform-facts.md §2.11): Breeze marks the selected tab with a
    // strip of `QPalette::Highlight`, the selection background.
    variant.tab.active_indicator_color = selection_bg;
    // A hovered, unselected tab is filled with the hover decoration at alpha
    // 0.2 in place of its own fill (breezehelper.cpp:1496-1503 at breeze
    // f0b1d75; docs/platform-facts.md §2.11).
    variant.tab.hover_background = view_hover.map(|hover| translucent(hover, TAB_HOVER_ALPHA));

    // Segmented control (docs/platform-facts.md §2.25): the selected segment
    // is the selection background; its text, the selection's text, reaches
    // it through `defaults.accent_text_color`.
    variant.segmented_control.active_background = selection_bg;

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
            Some(translucent(text, frame_contrast(ini) as f32));
    }

    // Scroll bar (docs/platform-facts.md §2.8): a hovered handle is the hover
    // decoration, at full alpha while the pointer is over the scroll bar,
    // filled at half alpha over the window (breezehelper.cpp:292, :1284;
    // breezestyle.cpp:6400-6405 at breeze f0b1d75); Breeze has no colour of
    // its own for a dragged handle, which keeps the hovered one.
    let handle = window_bg
        .zip(view_hover)
        .map(|(window, hover)| overlay_colors(window, hover, SCROLLBAR_HANDLE_ALPHA));
    variant.scrollbar.thumb_hover_color = handle;
    variant.scrollbar.thumb_active_color = handle;

    // Toolbar (docs/platform-facts.md §2.13): Breeze fills a main window's
    // tools area with the Header set's background and strokes its bottom edge
    // in that set's `separatorColor()` (breezestyle.cpp:1322, :1341-1350,
    // breezehelper.cpp:329-332, breezetoolsareamanager.cpp:201-215 at breeze
    // f0b1d75). KColorScheme takes each key the Header group lacks, or the
    // whole set where the scheme has no Header group, from `[Colors:Window]`
    // (kcolorscheme.cpp:330-332 at 27066d47); without the group Breeze paints
    // no fill and the toolbar shows the window, the same colour.
    let header_bg = get_color(ini, "Colors:Header", "BackgroundNormal").or(window_bg);
    let header_fg = get_color(ini, "Colors:Header", "ForegroundNormal").or(window_fg);
    variant.toolbar.background_color = header_bg;
    if let Some(edge) = header_bg
        .zip(header_fg)
        .map(|(bg, fg)| mix(bg, fg, frame_contrast(ini)))
    {
        variant.toolbar.border.get_or_insert_default().color = Some(edge);
    }

    // Popover (docs/platform-facts.md §2.16): its fill and text inherit the
    // window's, as qqc2-desktop-style's Popup fills the Window set's; its edge
    // is the Window set's background moved towards its text by the frame
    // contrast (Popup.qml:50 at v6.31.0). A separator inherits the border
    // colour (§2.18), not set here.
    if let Some(edge) = window_bg
        .zip(window_fg)
        .map(|(bg, fg)| linear_interpolation(bg, fg, frame_contrast(ini)))
    {
        variant.popover.border.get_or_insert_default().color = Some(edge);
    }

    // Card (docs/platform-facts.md §2.26): Kirigami's Card takes the View set,
    // its edge the background moved towards the text by the frame contrast
    // (DefaultCardBackground.qml:70, templates/AbstractCard.qml:92-93 at
    // kirigami 8319acc7); its fill inherits the surface colour.
    let view_bg = get_color(ini, "Colors:View", "BackgroundNormal");
    let view_fg = get_color(ini, "Colors:View", "ForegroundNormal");
    if let Some(edge) = view_bg
        .zip(view_fg)
        .map(|(bg, fg)| linear_interpolation(bg, fg, frame_contrast(ini)))
    {
        variant.card.border.get_or_insert_default().color = Some(edge);
    }

    // Combo box (docs/platform-facts.md §2.24): the Button set.
    variant.combo_box.background_color = button_bg;
    if let Some(color) = button_fg {
        variant.combo_box.font.get_or_insert_default().color = Some(color);
    }

    // Menu text (docs/platform-facts.md §2.6): the window text.
    if let Some(color) = window_fg {
        variant.menu.font.get_or_insert_default().color = Some(color);
    }

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
    // link.font.color: not set here -- it inherits View/ForegroundLink from
    // defaults.link_color, so an overlay's link_color recolours link text.

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

    /// A scheme's accent other than Breeze's reaches the slider's value through
    /// inheritance (docs/platform-facts.md:1317, KDE ← `defaults.accent_color`),
    /// which kde-breeze does not state, and its selection colour reaches the
    /// three blends the reader writes, the checked checkbox, the on switch and
    /// the progress bar.
    #[test]
    fn a_custom_accent_reaches_the_accent_fills() {
        let breeze = populate_fixture(BREEZE_DARK);
        let custom = BREEZE_DARK
            .replace(
                "[Colors:Selection]\nBackgroundNormal=61,174,233",
                "[Colors:Selection]\nBackgroundNormal=200,40,90",
            )
            .replace("DecorationFocus=61,174,233", "DecorationFocus=200,40,90");
        assert_ne!(custom, BREEZE_DARK, "the fixture's accent moved");
        let v = populate_fixture(&custom);
        assert_eq!(
            v.slider.fill_color, None,
            "the reader leaves the slider to inheritance"
        );
        let mut mode = crate::Theme::preset("kde-breeze")
            .ok()
            .and_then(|t| t.dark)
            .unwrap_or_default();
        assert_eq!(
            mode.slider.fill_color, None,
            "kde-breeze states no slider fill"
        );
        mode.merge(&v);
        let r = mode
            .into_resolved(&crate::ResolutionContext::for_tests())
            .unwrap();
        assert_eq!(r.slider.fill_color, Rgba::rgb(200, 40, 90));
        for (field, ours, breezes) in [
            (
                "checkbox",
                v.checkbox.checked_background,
                breeze.checkbox.checked_background,
            ),
            (
                "switch",
                v.switch.checked_background,
                breeze.switch.checked_background,
            ),
            (
                "progress_bar",
                v.progress_bar.fill_color,
                breeze.progress_bar.fill_color,
            ),
        ] {
            assert!(ours.is_some(), "{field}: no fill");
            assert_ne!(
                ours, breezes,
                "{field}: the custom selection colour did not reach it"
            );
        }
    }

    /// D7: the colours Breeze paints a pressed button, a hovered and a
    /// focused field's frame, a hovered tab, the selected segment and a
    /// hovered or dragged scroll-bar handle with, each from the key it reads
    /// (docs/platform-facts.md §2.3, §2.4, §2.11, §2.25, §2.8). The fixture's
    /// DecorationHover differs from its DecorationFocus, so the frames are
    /// told apart.
    #[test]
    fn the_pressed_hovered_and_focused_colours_come_from_their_keys() {
        let v = populate_fixture(BREEZE_DARK);
        let button = Rgba::rgb(49, 54, 59);
        let selection = Rgba::rgb(61, 174, 233);
        let hover = Rgba::rgb(29, 153, 243);
        let window = Rgba::rgb(49, 54, 59);
        assert_eq!(
            v.button.active_background,
            Some(mix(button, selection, PRESSED_BUTTON_MIX))
        );
        assert_eq!(v.input.hover_border_color, Some(hover));
        assert_eq!(v.input.focus_border_color, Some(selection));
        assert_eq!(
            v.tab.hover_background,
            Some(translucent(hover, TAB_HOVER_ALPHA))
        );
        assert_eq!(v.tab.hover_background.map(|c| c.a), Some(0x33));
        assert_eq!(v.segmented_control.active_background, Some(selection));
        let handle = Some(overlay_colors(window, hover, SCROLLBAR_HANDLE_ALPHA));
        assert_eq!(v.scrollbar.thumb_hover_color, handle);
        assert_eq!(v.scrollbar.thumb_active_color, handle);
    }

    /// D7: a scheme whose selection, focus and hover colours are not
    /// Breeze's reaches the same fields, so a live desktop does not keep
    /// kde-breeze's blue there.
    #[test]
    fn a_custom_scheme_reaches_the_pressed_hovered_and_focused_colours() {
        let breeze = populate_fixture(BREEZE_DARK);
        let custom = BREEZE_DARK
            .replace(
                "[Colors:Selection]\nBackgroundNormal=61,174,233",
                "[Colors:Selection]\nBackgroundNormal=200,40,90",
            )
            .replace("DecorationFocus=61,174,233", "DecorationFocus=20,200,90")
            .replace("DecorationHover=29,153,243", "DecorationHover=240,160,20");
        assert_ne!(custom, BREEZE_DARK, "the fixture's colours moved");
        let v = populate_fixture(&custom);
        assert_eq!(v.input.hover_border_color, Some(Rgba::rgb(240, 160, 20)));
        assert_eq!(v.input.focus_border_color, Some(Rgba::rgb(20, 200, 90)));
        assert_eq!(
            v.segmented_control.active_background,
            Some(Rgba::rgb(200, 40, 90))
        );
        for (field, ours, breezes) in [
            (
                "button.active_background",
                v.button.active_background,
                breeze.button.active_background,
            ),
            (
                "tab.hover_background",
                v.tab.hover_background,
                breeze.tab.hover_background,
            ),
            (
                "scrollbar.thumb_hover_color",
                v.scrollbar.thumb_hover_color,
                breeze.scrollbar.thumb_hover_color,
            ),
            (
                "scrollbar.thumb_active_color",
                v.scrollbar.thumb_active_color,
                breeze.scrollbar.thumb_active_color,
            ),
        ] {
            assert!(ours.is_some(), "{field}: not written");
            assert_ne!(ours, breezes, "{field}: the custom scheme did not reach it");
        }
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

    // === Per-widget: Sidebar (View) ===

    #[test]
    fn test_sidebar_background_from_view() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(v.sidebar.background_color, Some(Rgba::rgb(35, 38, 41)));
    }

    #[test]
    fn test_sidebar_font_color_from_view() {
        let v = populate_fixture(BREEZE_DARK);
        assert_eq!(
            v.sidebar.font.as_ref().and_then(|f| f.color),
            Some(Rgba::rgb(252, 252, 252))
        );
    }

    /// The Complementary set, which no KDE sidebar uses, does not reach the
    /// sidebar (it gave Breeze Light a dark one).
    #[test]
    fn test_sidebar_ignores_complementary() {
        let content = "\
[Colors:Complementary]
BackgroundNormal=42,46,50
ForegroundNormal=239,240,241
";
        let v = populate_fixture(content);
        assert!(v.sidebar.background_color.is_none());
        assert!(v.sidebar.font.is_none());
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

    // === Combo box from Button, menu text from Window ===

    /// docs/platform-facts.md §2.24 and §2.6's KDE cells: the combo box in
    /// the Button set, the menu text in the Window set's foreground.
    #[test]
    fn test_combo_box_from_button_and_menu_text_from_window() {
        let v = populate_fixture(
            "[Colors:Window]\nBackgroundNormal=1,1,1\nForegroundNormal=2,2,2\n\n\
             [Colors:Button]\nBackgroundNormal=3,3,3\nForegroundNormal=4,4,4\n",
        );
        assert_eq!(v.combo_box.background_color, Some(Rgba::rgb(3, 3, 3)));
        assert_eq!(
            v.combo_box.font.as_ref().and_then(|f| f.color),
            Some(Rgba::rgb(4, 4, 4))
        );
        assert_eq!(
            v.menu.font.as_ref().and_then(|f| f.color),
            Some(Rgba::rgb(2, 2, 2))
        );
    }

    /// docs/platform-facts.md §2.16 and §2.18's KDE cells: a popover takes
    /// the window's fill and text and a separator the border colour, by
    /// inheritance, so the reader states none of them; it derives only the
    /// popover's edge.
    #[test]
    fn test_popover_and_separator_are_inherited() {
        let v = populate_fixture(BREEZE_DARK);
        assert!(v.popover.background_color.is_none());
        assert!(v.popover.font.is_none());
        assert!(v.separator.line_color.is_none());
    }

    // === Sidebar None when View missing ===

    #[test]
    fn test_sidebar_none_without_view() {
        let content = "\
[Colors:Window]
BackgroundNormal=49,54,59
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
            v.combo_box.background_color.is_some(),
            "combo_box.background_color missing"
        );
        assert!(
            v.combo_box.font.as_ref().and_then(|f| f.color).is_some(),
            "combo_box.font.color missing"
        );
        assert!(
            v.menu.font.as_ref().and_then(|f| f.color).is_some(),
            "menu.font.color missing"
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
