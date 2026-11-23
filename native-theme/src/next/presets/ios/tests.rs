use super::*;

use crate::next::preset::Theme;
use crate::next::query::{ButtonQuery, Direction};

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn both_schemes() -> [ResolvedTheme; 2] {
    [ThemeContext::light(), ThemeContext::dark()]
        .map(|ctx| Theme::preset(Preset::Ios13).resolve(&ctx))
}

#[test]
fn the_preset_defines_the_normal_contrast_alone() {
    assert_eq!(CONTRAST_LEVELS, &[Contrast::Normal]);
    let ctx = ThemeContext::light().with_contrast(Contrast::High);
    let high = Theme::preset(Preset::Ios13).resolve(&ctx);
    assert_eq!(high.context.contrast, Contrast::Normal);
}

#[test]
fn the_constant_colours_are_their_8_bit_channels() {
    assert_eq!(LIGHT.muted.to_srgb8(), [0x3c, 0x3c, 0x43, 0x99]);
    assert_eq!(
        DARK.switch_disabled_off.to_srgb8(),
        [0x78, 0x78, 0x80, 0x21]
    );
    assert_eq!(TRANSPARENT.to_srgb8(), [0x00, 0x00, 0x00, 0x00]);
}

/// A primary button keeps the accent under the pointer and pressed, and
/// takes the disabled fill when disabled.
#[test]
fn a_primary_button_keeps_the_accent_until_disabled() {
    for theme in both_schemes() {
        let primary = |state| ButtonQuery::primary().state(state);
        let accent = Paint::Solid(theme.colors.accent.fill);
        for state in [WidgetState::NONE, WidgetState::HOVER, WidgetState::PRESSED] {
            assert_eq!(theme.button.body(primary(state)).fill, accent, "{state:?}");
        }
        assert_ne!(
            theme.button.body(primary(WidgetState::DISABLED)).fill,
            accent
        );
    }
}

/// The switch's track: hovered on over on and hovered off, the disabled
/// tracks over every other; the thumb white in every state.
#[test]
fn the_switch_s_track_follows_its_precedences() {
    for (theme, c) in both_schemes().iter().zip([LIGHT, DARK]) {
        let track = |state| theme.switch.track(state).fill;
        assert_eq!(
            track(WidgetState::CHECKED | WidgetState::HOVER),
            Paint::Solid(c.switch_hover_on)
        );
        assert_eq!(
            track(WidgetState::DISABLED | WidgetState::CHECKED | WidgetState::HOVER),
            Paint::Solid(c.switch_disabled_on)
        );
        assert_eq!(
            track(WidgetState::DISABLED | WidgetState::HOVER),
            Paint::Solid(c.switch_disabled_off)
        );
        assert_eq!(
            theme
                .switch
                .thumb(WidgetState::DISABLED | WidgetState::CHECKED)
                .fill,
            Paint::Solid(c.white)
        );
    }
}

/// The preset states no per-side value that differs on the left and the
/// right: right to left changes nothing but the context.
#[test]
fn right_to_left_changes_nothing() {
    for ctx in [ThemeContext::light(), ThemeContext::dark()] {
        let theme = Theme::preset(Preset::Ios13);
        let ltr = theme.resolve(&ctx);
        let mut rtl = theme.resolve(&ctx.clone().with_direction(Direction::Rtl));
        rtl.context = ltr.context.clone();
        assert_eq!(rtl, ltr);
    }
}

/// The desktop controls iOS lacks are synthesized, the others native.
#[test]
fn the_desktop_controls_are_synthesized() {
    for theme in both_schemes() {
        assert!(!theme.tooltip.is_native());
        assert!(!theme.status_bar.is_native());
        assert!(!theme.splitter.is_native());
        assert!(!theme.menu_bar.is_native());
        assert!(theme.button.is_native() && theme.list.is_native() && theme.switch.is_native());
    }
}

/// The text-scale entries the TOML states are the Dynamic Type table's rows
/// at the default category, `Large`, for the style each scales by.
#[test]
fn the_text_scale_entries_are_the_table_s_large_rows() -> TestResult {
    for (role, size, line_height) in [
        (TextRole::Body, FONT_SIZE_PT, None),
        (
            TextRole::Caption,
            CAPTION_SIZE_PT,
            Some(CAPTION_LINE_HEIGHT_PT),
        ),
        (
            TextRole::Headline,
            HEADLINE_SIZE_PT,
            Some(HEADLINE_LINE_HEIGHT_PT),
        ),
        (
            TextRole::Title1,
            TITLE1_SIZE_PT,
            Some(TITLE1_LINE_HEIGHT_PT),
        ),
        (
            TextRole::LargeTitle,
            DISPLAY_SIZE_PT,
            Some(DISPLAY_LINE_HEIGHT_PT),
        ),
    ] {
        let row = dynamic_type(role, DEFAULT_CATEGORY, FONT_DPI).ok_or("a row")?;
        assert_eq!(row.font_size, size, "{role:?}");
        if let Some(line_height) = line_height {
            assert_eq!(row.line_height, line_height, "{role:?}");
        }
    }
    Ok(())
}

/// The nearest category: `Large` at 1; at 21/17 `ExtraExtraLarge`, whose
/// Body is 21; the smallest and the largest at the ends of the text scale's
/// range; between two categories the nearer.
#[test]
fn the_text_scale_takes_the_nearest_category() {
    assert_eq!(nearest_category(1.0), ContentSizeCategory::Large);
    assert_eq!(
        nearest_category(21.0 / 17.0),
        ContentSizeCategory::ExtraExtraLarge
    );
    assert_eq!(nearest_category(0.5), ContentSizeCategory::ExtraSmall);
    assert_eq!(
        nearest_category(4.0),
        ContentSizeCategory::AccessibilityExtraExtraExtraLarge
    );
    // Body 33 and 40 at the two accessibility categories around 2.1 (35.7 pt).
    assert_eq!(
        nearest_category(2.1),
        ContentSizeCategory::AccessibilityLarge
    );
}

/// At a text scale of 21/17 (`ExtraExtraLarge`) each style takes its row of
/// the table: the body 21 pt, the caption 16 on 21, the title 32 on 39; the
/// body's line height the multiplier times its size; each `defined_size`
/// the unscaled size; the widgets' text the body's size.
#[test]
fn at_extra_extra_large_the_sizes_are_the_table_s() {
    for scheme in [Scheme::Light, Scheme::Dark] {
        let ctx = ThemeContext::default()
            .with_scheme(scheme)
            .with_text_scale(21.0 / 17.0);
        let theme = Theme::preset(Preset::Ios13).resolve(&ctx);
        let t = &theme.typography;
        assert_eq!(t.body.size_px, 21.0);
        assert_eq!(t.body.defined_size, Some(FontSize::Pt(FONT_SIZE_PT)));
        assert_eq!(
            t.body.line_height,
            (f64::from(LINE_HEIGHT * FONT_SIZE_PT) * 21.0 / 17.0) as f32
        );
        let caption = t.caption.as_ref().map(|s| (s.size_px, s.line_height));
        assert_eq!(caption, Some((16.0, 21.0)));
        let title = t.title1.as_ref().map(|s| (s.size_px, s.line_height));
        assert_eq!(title, Some((32.0, 39.0)));
        assert_eq!(t.monospace.size_px, 21.0);
        assert_eq!(
            theme
                .button
                .label(ButtonQuery::new(WidgetState::NONE))
                .size_px,
            21.0
        );
    }
}

/// An explicit accent of the tests: a test input, no theme value.
const RED: u32 = 0xe038_3e;

/// The opaque colour of `rgb`, `0xRRGGBB`.
fn hex(rgb: u32) -> Color {
    srgb8((rgb << 8) | 0xff)
}

/// The constants of `scheme`.
fn constants(scheme: Scheme) -> SchemeColors {
    match scheme {
        Scheme::Light => LIGHT,
        Scheme::Dark => DARK,
    }
}

/// The colour of a solid paint.
fn solid(paint: &Paint) -> Option<Color> {
    match paint {
        Paint::Solid(color) => Some(*color),
        _ => None,
    }
}

/// The colour of a uniform stroke.
fn stroke_color(stroke: &Stroke) -> Option<Color> {
    match stroke {
        Stroke::Uniform { color, .. } => Some(*color),
        _ => None,
    }
}

/// A palette for `scheme` whose every slot has a colour of its own.
fn distinct_palette(scheme: Scheme) -> Palette {
    Palette {
        name: Arc::from("distinct"),
        scheme,
        window_bg: hex(0xf0e0d0),
        view_bg: hex(0xfaf5f0),
        surface_bg: hex(0xe8f0e8),
        header_bg: hex(0xd8d0f0),
        sidebar_bg: hex(0xe0f0f8),
        tooltip_bg: hex(0xf8f0c0),
        foreground: hex(0x201810),
        foreground_muted: hex(0x605850),
        border: hex(0xa09080),
        divider: hex(0xb0a8a0),
        accent: hex(0x8020c0),
        accent_fg: hex(0xfff8e0),
        selection_bg: hex(0x40a060),
        selection_fg: hex(0x002010),
        link: hex(0x1040a0),
        caret: hex(0xc04010),
        error: hex(0xc00020),
        warning: hex(0xc08000),
        success: hex(0x208020),
        info: hex(0x2080c0),
    }
}

/// Every widget colour the theme takes from the accent's fill, each by its
/// name (the module documentation's list).
fn accent_fills(theme: &ResolvedTheme) -> Vec<(&'static str, Option<Color>)> {
    let checked = WidgetState::CHECKED;
    let selected = WidgetState::SELECTED;
    vec![
        (
            "primary button",
            solid(&theme.button.body(ButtonQuery::primary()).fill),
        ),
        (
            "checked check box",
            solid(&theme.checkbox.indicator(checked).fill),
        ),
        (
            "checked radio button",
            solid(&theme.radio.indicator(checked).fill),
        ),
        (
            "slider's filled part",
            solid(&theme.slider.active_track(WidgetState::NONE).fill),
        ),
        (
            "progress",
            solid(&theme.progress_bar.fill(WidgetState::NONE).fill),
        ),
        ("spinner", Some(theme.spinner.color)),
        (
            "selected segment",
            solid(&theme.segmented_control.segment(selected).fill),
        ),
        (
            "selected tab's label",
            Some(theme.tab.label(selected).color),
        ),
        (
            "hovered text field",
            stroke_color(&theme.input.field(WidgetState::HOVER).stroke),
        ),
        (
            "focused text field",
            stroke_color(&theme.input.field(WidgetState::FOCUSED).stroke),
        ),
        (
            "hovered splitter",
            solid(&theme.splitter.handle(WidgetState::HOVER).fill),
        ),
    ]
}

/// An explicit accent is written as given (`accent-decision.md`, iOS): the
/// accent's fill and every widget colour taken from it, the selection, its
/// unfocused colours (0.7.2 states them as the selection's) and the text
/// selection with the selection's text; every other role keeps the
/// preset's, the focus ring, the link, the info role and the caret among
/// them, and so do WebKit's `systemBlue` texts and the switch's green.
#[test]
fn an_explicit_accent_is_written_as_given() {
    let accent = hex(RED);
    for scheme in [Scheme::Light, Scheme::Dark] {
        let ctx = ThemeContext::default().with_scheme(scheme);
        let plain = Theme::preset(Preset::Ios13).resolve(&ctx);
        let theme = Theme::preset(Preset::Ios13)
            .with_accent(accent)
            .resolve(&ctx);
        for (name, got) in accent_fills(&theme) {
            assert_eq!(got, Some(accent), "{scheme:?} {name}");
        }
        let foreground = plain.colors.selection.foreground;
        let mut want = plain.colors.clone();
        want.accent.fill = accent;
        want.selection = SelectionColors::new(accent, foreground, Some(accent), Some(foreground));
        want.text_selection = SelectionColors::new(accent, foreground, None, None);
        assert_eq!(theme.colors, want, "{scheme:?}");
        let c = constants(scheme);
        let normal = ButtonQuery::new(WidgetState::NONE);
        assert_eq!(theme.button.label(normal).color, c.system_blue);
        assert_eq!(
            theme.switch.track(WidgetState::CHECKED).fill,
            Paint::Solid(c.system_green)
        );
        assert_eq!(
            solid(&theme.menu.item(WidgetState::HOVER).fill),
            Some(accent)
        );
    }
}

/// A palette states its roles (spec §7) and recolours the widget colours
/// the preset takes from them; the roles no slot names keep the preset's.
#[test]
fn a_palette_states_its_roles() {
    for scheme in [Scheme::Light, Scheme::Dark] {
        let p = distinct_palette(scheme);
        let ctx = ThemeContext::default().with_scheme(scheme);
        let theme = Theme::preset(Preset::Ios13)
            .with_palette(p.clone())
            .resolve(&ctx);
        let c = constants(scheme);
        let colors = &theme.colors;
        assert_eq!(
            colors.window,
            SurfaceColors::new(
                Some(p.window_bg),
                Some(p.foreground),
                Some(p.foreground_muted),
                Some(p.border),
                Some(p.divider)
            )
        );
        let surface = |bg| SurfaceColors::new(Some(bg), Some(p.foreground), None, None, None);
        assert_eq!(colors.view, surface(p.view_bg));
        assert_eq!(colors.header, surface(p.header_bg));
        assert_eq!(colors.sidebar, surface(p.sidebar_bg));
        assert_eq!(colors.card, surface(p.surface_bg));
        assert_eq!(colors.dialog, surface(p.surface_bg));
        assert_eq!(colors.popover, surface(p.surface_bg));
        assert_eq!(colors.tooltip, surface(p.tooltip_bg));
        assert_eq!(colors.accent.fill, p.accent);
        assert_eq!(colors.accent.foreground, p.accent_fg);
        assert_eq!(
            colors.selection,
            SelectionColors::new(
                p.selection_bg,
                p.selection_fg,
                Some(p.selection_bg),
                Some(p.selection_fg)
            )
        );
        assert_eq!(
            colors.text_selection,
            SelectionColors::new(p.selection_bg, p.selection_fg, None, None)
        );
        assert_eq!(colors.link, p.link);
        assert_eq!(colors.caret, p.caret);
        for (status, fill) in [
            (&colors.error, p.error),
            (&colors.warning, p.warning),
            (&colors.success, p.success),
            (&colors.info, p.info),
        ] {
            assert_eq!(status.fill, fill);
            assert_eq!(status.origin, RoleOrigin::Custom);
        }
        assert_eq!(colors.focus_ring, c.system_blue);
        assert_eq!(colors.disabled_text, c.tertiary_label);
        for (name, got) in accent_fills(&theme) {
            assert_eq!(got, Some(p.accent), "{scheme:?} {name}");
        }
        let none = WidgetState::NONE;
        assert_eq!(theme.input.field(none).fill, Paint::Solid(p.window_bg));
        assert_eq!(
            theme.sidebar.container(none).fill,
            Paint::Solid(p.sidebar_bg)
        );
        assert_eq!(theme.popover.panel.fill, Paint::Solid(p.surface_bg));
        assert_eq!(theme.card.container(none).fill, Paint::Solid(p.surface_bg));
        assert_eq!(theme.tooltip.panel.fill, Paint::Solid(p.tooltip_bg));
        assert_eq!(theme.tooltip.text.color, p.foreground);
        assert_eq!(theme.input.text(none).color, p.foreground);
        assert_eq!(theme.separator.color, p.border);
        assert_eq!(
            theme.menu.item_label(WidgetState::HOVER).color,
            p.selection_fg
        );
        assert_eq!(
            theme.switch.track(WidgetState::CHECKED).fill,
            Paint::Solid(c.system_green)
        );
    }
}

/// An explicit accent over a palette replaces the palette's accent, its
/// text and its selection, whose texts are then the preset's
/// (`accent-decision.md`, M7, which iOS follows); every other slot of the
/// palette stands, its link, informational colour and caret with it.
#[test]
fn an_explicit_accent_replaces_the_palette_s_accent_slots() {
    let accent = hex(RED);
    for scheme in [Scheme::Light, Scheme::Dark] {
        let p = distinct_palette(scheme);
        let ctx = ThemeContext::default().with_scheme(scheme);
        let theme = Theme::preset(Preset::Ios13)
            .with_palette(p.clone())
            .with_accent(accent)
            .resolve(&ctx);
        let palette_only = Theme::preset(Preset::Ios13)
            .with_palette(p.clone())
            .resolve(&ctx);
        let c = constants(scheme);
        let colors = &theme.colors;
        assert_eq!(colors.accent.fill, accent);
        assert_eq!(colors.accent.foreground, c.on_accent);
        let foreground = c.selection_foreground;
        assert_eq!(
            colors.selection,
            SelectionColors::new(accent, foreground, Some(accent), Some(foreground))
        );
        assert_eq!(
            colors.text_selection,
            SelectionColors::new(accent, foreground, None, None)
        );
        assert_eq!(colors.focus_ring, c.system_blue);
        assert_eq!(colors.caret, p.caret);
        assert_eq!(colors.link, p.link);
        assert_eq!(colors.info, palette_only.colors.info);
        assert_eq!(colors.window, palette_only.colors.window);
        assert_eq!(colors.card, palette_only.colors.card);
        assert_eq!(colors.tooltip, palette_only.colors.tooltip);
        assert_eq!(colors.error, palette_only.colors.error);
        for (name, got) in accent_fills(&theme) {
            assert_eq!(got, Some(accent), "{scheme:?} {name}");
        }
        assert_eq!(
            theme.menu.item_label(WidgetState::HOVER).color,
            c.selection_foreground
        );
    }
}
