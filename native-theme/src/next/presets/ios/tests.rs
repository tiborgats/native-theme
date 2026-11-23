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
