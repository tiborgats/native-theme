use super::*;

use crate::next::preset::Theme;
use crate::next::query::{ButtonQuery, ListQuery, MotionPreference};
use crate::next::window::WindowState;

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn both_schemes() -> [ResolvedTheme; 2] {
    [ThemeContext::light(), ThemeContext::dark()]
        .map(|ctx| Theme::preset(Preset::Adwaita).resolve(&ctx))
}

#[test]
fn the_preset_defines_the_normal_and_the_high_contrast() {
    assert_eq!(CONTRAST_LEVELS, &[Contrast::Normal, Contrast::High]);
}

#[test]
fn the_constant_colours_are_their_8_bit_channels() {
    assert_eq!(LIGHT.focus_ring.to_srgb8(), [0x04, 0x61, 0xbe, 0x80]);
    assert_eq!(
        DARK.card_background.map(Color::to_srgb8),
        Some([0xff, 0xff, 0xff, 0x14])
    );
}

/// Under `Contrast::High` the focus ring, and the focused text field's
/// outline with it, is the ring's colour at alpha 0.8; nothing else changes.
#[test]
fn under_high_contrast_the_focus_ring_is_at_alpha_0_8() {
    for ctx in [ThemeContext::light(), ThemeContext::dark()] {
        let theme = Theme::preset(Preset::Adwaita);
        let normal = theme.resolve(&ctx);
        let high = theme.resolve(&ctx.clone().with_contrast(Contrast::High));
        assert_eq!(high.context.contrast, Contrast::High);
        let [r, g, b, a] = normal.colors.focus_ring.to_srgb8();
        assert_eq!(a, 0x80);
        assert_eq!(high.colors.focus_ring.to_srgb8(), [r, g, b, 0xcc]);
        assert_eq!(
            high.focus_ring.stroke,
            Stroke::uniform(FOCUS_RING_WIDTH, high.colors.focus_ring)
        );
        assert_eq!(
            high.input.field(WidgetState::FOCUSED).stroke,
            Stroke::uniform(INPUT_BORDER_WIDTH, high.colors.focus_ring)
        );
        let mut same = high.clone();
        same.colors.focus_ring = normal.colors.focus_ring;
        same.focus_ring = normal.focus_ring.clone();
        same.input = normal.input.clone();
        same.text_area = normal.text_area.clone();
        same.context = normal.context.clone();
        assert_eq!(same, normal);
    }
}

#[test]
fn a_primary_button_keeps_the_accent_fill_when_hovered_pressed_or_checked() {
    for theme in both_schemes() {
        for state in [
            WidgetState::HOVER,
            WidgetState::PRESSED,
            WidgetState::CHECKED,
        ] {
            let query = ButtonQuery::primary().state(state);
            assert_eq!(
                theme.button.body(query).fill,
                Paint::Solid(theme.colors.accent.fill),
                "{state:?}"
            );
        }
        let pressed_hover = ButtonQuery::new(WidgetState::PRESSED | WidgetState::HOVER);
        assert_eq!(
            theme.button.body(pressed_hover),
            theme.button.body(ButtonQuery::new(WidgetState::PRESSED))
        );
        let disabled = ButtonQuery::primary().state(WidgetState::DISABLED);
        assert_eq!(
            theme.button.label(disabled).color,
            theme.colors.disabled_text
        );
    }
}

#[test]
fn a_checked_indicator_keeps_its_fill_under_the_pointer() {
    for theme in both_schemes() {
        let checked = WidgetState::CHECKED;
        let hovered = checked | WidgetState::HOVER;
        assert_ne!(
            theme.checkbox.indicator(WidgetState::HOVER),
            theme.checkbox.indicator(checked)
        );
        assert_eq!(
            theme.checkbox.indicator(hovered),
            theme.checkbox.indicator(checked)
        );
        assert_eq!(
            theme.radio.indicator(hovered),
            theme.radio.indicator(checked)
        );
    }
}

#[test]
fn a_hovered_switch_that_is_on_has_its_own_colour() {
    for theme in both_schemes() {
        let on_hover = theme
            .switch
            .track(WidgetState::HOVER | WidgetState::CHECKED);
        assert_eq!(on_hover.fill, Paint::Solid(LIGHT.switch_hover_on));
        assert_ne!(on_hover, theme.switch.track(WidgetState::HOVER));
        assert_ne!(on_hover, theme.switch.track(WidgetState::CHECKED));
    }
}

#[test]
fn a_selected_item_wins_over_the_pointer() {
    for theme in both_schemes() {
        let selected = WidgetState::SELECTED;
        let hovered = selected | WidgetState::HOVER;
        let row = |state| theme.list.highlight(ListQuery::new(state));
        assert_ne!(row(WidgetState::HOVER), row(selected));
        assert_eq!(row(hovered), row(selected));
        assert_eq!(theme.sidebar.item(hovered), theme.sidebar.item(selected));
        assert_eq!(theme.tab.tab(hovered), theme.tab.tab(selected));
        assert_eq!(
            theme.segmented_control.segment(hovered),
            theme.segmented_control.segment(selected)
        );
    }
}

#[test]
fn a_checked_toolbar_button_keeps_its_fill() {
    for theme in both_schemes() {
        let checked = theme.toolbar.button(WidgetState::CHECKED);
        for state in [WidgetState::HOVER, WidgetState::PRESSED] {
            assert_ne!(theme.toolbar.button(state), checked, "{state:?}");
            assert_eq!(
                theme.toolbar.button(state | WidgetState::CHECKED),
                checked,
                "{state:?}"
            );
        }
        assert_eq!(
            theme
                .toolbar
                .button(WidgetState::PRESSED | WidgetState::HOVER),
            theme.toolbar.button(WidgetState::PRESSED)
        );
    }
}

#[test]
fn an_inactive_window_has_the_inactive_title_bar() {
    for ctx in [ThemeContext::light(), ThemeContext::dark()] {
        let theme = Theme::preset(Preset::Adwaita);
        let active = theme.resolve(&ctx);
        let inactive = theme.resolve(&ctx.clone().with_window_active(false));
        let state = WindowState::default();
        assert_eq!(
            Some(inactive.window.title_bar(state).fill),
            active.colors.window.background.map(Paint::Solid)
        );
        assert_ne!(
            inactive.window.title_bar(state),
            active.window.title_bar(state)
        );
        assert_eq!(inactive.window.title(state), active.window.title(state));
    }
}

#[test]
fn right_to_left_changes_nothing() {
    for ctx in [ThemeContext::light(), ThemeContext::dark()] {
        let theme = Theme::preset(Preset::Adwaita);
        let mut rtl = theme.resolve(&ctx.clone().with_direction(Direction::Rtl));
        rtl.context = ctx.clone();
        assert_eq!(rtl, theme.resolve(&ctx));
    }
}

/// The text scale of the test below.
const TEXT_SCALE_TWO: f32 = 2.0;

/// The text scale is linear on GNOME and not rounded: at 2.0 every font size
/// and line height is twice the one at 1.0, each `defined_size` stays the
/// unscaled size, and nothing but the text changes.
#[test]
fn at_text_scale_2_every_font_size_is_doubled() -> TestResult {
    for scheme in [Scheme::Light, Scheme::Dark] {
        let ctx = ThemeContext::default().with_scheme(scheme);
        let base = Theme::preset(Preset::Adwaita).resolve(&ctx);
        let double = Theme::preset(Preset::Adwaita).resolve(&ctx.with_text_scale(TEXT_SCALE_TWO));
        let slots = |t: &ResolvedTheme| -> Result<Vec<TextStyle>, String> {
            let entry = |slot: &Option<TextStyle>, name: &str| {
                slot.clone().ok_or_else(|| format!("no {name}"))
            };
            Ok(vec![
                t.typography.body.clone(),
                t.typography.monospace.clone(),
                entry(&t.typography.display, "display")?,
                entry(&t.typography.title1, "title1")?,
                entry(&t.typography.headline, "headline")?,
                entry(&t.typography.caption, "caption")?,
                t.button.label(ButtonQuery::new(WidgetState::NONE)),
                t.dialog.title.clone(),
                t.window.title(WindowState::default()),
            ])
        };
        for (one, two) in slots(&base)?.into_iter().zip(slots(&double)?) {
            let size = one.defined_size.ok_or("no defined size")?;
            assert_eq!(two.defined_size, Some(size), "{scheme:?}");
            assert_eq!(two.size_px, one.size_px * TEXT_SCALE_TWO, "{scheme:?}");
            assert_eq!(
                two.line_height,
                one.line_height * TEXT_SCALE_TWO,
                "{scheme:?}"
            );
        }
        let rest = ButtonQuery::new(WidgetState::NONE);
        assert_eq!(double.button.body(rest), base.button.body(rest));
        assert_eq!(double.layout, base.layout);
        assert_eq!(double.icons, base.icons);
    }
    Ok(())
}

/// `MotionPreference::None` sets the dialog's entrance to
/// `DialogEntranceStyle::None`; libadwaita's `ScaleFade` stays under full
/// and under reduced motion. The preset states no motion token.
#[test]
fn no_motion_removes_the_dialog_s_entrance() {
    let resolve = |motion: MotionPreference| {
        Theme::preset(Preset::Adwaita).resolve(&ThemeContext::light().with_motion(motion))
    };
    let none = resolve(MotionPreference::None);
    assert_eq!(none.dialog.entrance, Some(DialogEntranceStyle::None));
    assert_eq!(none.motion_tokens, MotionTokens::default());
    for motion in [MotionPreference::Full, MotionPreference::Reduced] {
        let theme = resolve(motion);
        assert_eq!(
            theme.dialog.entrance,
            Some(DialogEntranceStyle::ScaleFade),
            "{motion:?}"
        );
        assert_eq!(theme.motion_tokens, MotionTokens::default(), "{motion:?}");
    }
}
