use super::*;

use crate::next::motion::DialogEntranceStyle;
use crate::next::preset::Theme;
use crate::next::query::{ButtonQuery, ListQuery, MotionPreference};
use crate::next::window::WindowState;

type TestResult = Result<(), Box<dyn std::error::Error>>;

/// Both presets, each in both schemes.
fn every_theme() -> Vec<(Preset, ResolvedTheme)> {
    [Preset::MacosMonterey, Preset::MacosTahoe]
        .into_iter()
        .flat_map(|preset| {
            [ThemeContext::light(), ThemeContext::dark()]
                .map(|ctx| (preset, Theme::preset(preset).resolve(&ctx)))
        })
        .collect()
}

#[test]
fn the_presets_define_the_normal_contrast_alone() {
    assert_eq!(CONTRAST_LEVELS, &[Contrast::Normal]);
    for preset in [Preset::MacosMonterey, Preset::MacosTahoe] {
        let ctx = ThemeContext::light().with_contrast(Contrast::High);
        let high = Theme::preset(preset).resolve(&ctx);
        assert_eq!(high.context.contrast, Contrast::Normal, "{preset:?}");
    }
}

#[test]
fn the_constant_colours_are_their_8_bit_channels() {
    assert_eq!(
        MONTEREY_LIGHT.foreground.to_srgb8(),
        [0x00, 0x00, 0x00, 0xd8]
    );
    assert_eq!(
        MONTEREY_DARK.switch_thumb.to_srgb8(),
        [0xff, 0xff, 0xff, 0xbf]
    );
    assert_eq!(
        TAHOE_DARK.slider_disabled_fill.to_srgb8(),
        [0xff, 0xff, 0xff, 0x1f]
    );
    assert_eq!(TAHOE_LIGHT.combo_hover, None);
}

#[test]
fn a_primary_button_keeps_the_accent_fill_until_disabled() {
    for (preset, theme) in every_theme() {
        for state in [WidgetState::NONE, WidgetState::HOVER, WidgetState::PRESSED] {
            let query = ButtonQuery::primary().state(state);
            assert_eq!(
                theme.button.body(query).fill,
                Paint::Solid(theme.colors.accent.fill),
                "{preset:?} {state:?}"
            );
            assert_eq!(
                theme.button.label(query).color,
                theme.colors.accent.foreground,
                "{preset:?} {state:?}"
            );
        }
        let disabled = WidgetState::DISABLED;
        let primary = ButtonQuery::primary().state(disabled | WidgetState::HOVER);
        assert_eq!(
            theme.button.body(primary).fill,
            theme.button.body(ButtonQuery::new(disabled)).fill,
            "{preset:?}"
        );
        assert_eq!(
            theme.button.label(primary).color,
            theme.colors.disabled_text,
            "{preset:?}"
        );
        // `macos-tahoe` states no pressed fill: pressed and hovered, its body is the hovered one.
        let pressed_hover = ButtonQuery::new(WidgetState::PRESSED | WidgetState::HOVER);
        let expected = match preset {
            Preset::MacosTahoe => WidgetState::HOVER,
            _ => WidgetState::PRESSED,
        };
        assert_eq!(
            theme.button.body(pressed_hover),
            theme.button.body(ButtonQuery::new(expected)),
            "{preset:?}"
        );
    }
}

#[test]
fn a_checked_indicator_keeps_its_fill_under_the_pointer() {
    for (preset, theme) in every_theme() {
        let checked = WidgetState::CHECKED;
        let hovered = checked | WidgetState::HOVER;
        assert_eq!(
            theme.checkbox.indicator(hovered),
            theme.checkbox.indicator(checked),
            "{preset:?}"
        );
        assert_eq!(
            theme.radio.indicator(hovered),
            theme.radio.indicator(checked),
            "{preset:?}"
        );
    }
    let monterey = Theme::preset(Preset::MacosMonterey).resolve(&ThemeContext::light());
    assert_ne!(
        monterey.checkbox.indicator(WidgetState::HOVER),
        monterey.checkbox.indicator(WidgetState::NONE)
    );
}

#[test]
fn a_disabled_look_wins_over_the_pointer_and_the_selection() {
    for (preset, theme) in every_theme() {
        let disabled = WidgetState::DISABLED;
        let selected = ListQuery::new(disabled | WidgetState::SELECTED);
        assert_eq!(
            theme.list.item_label(selected).color,
            theme.colors.disabled_text,
            "{preset:?}"
        );
        for state in [disabled, disabled | WidgetState::HOVER] {
            assert_eq!(
                theme.menu.item_label(state).color,
                theme.colors.disabled_text,
                "{preset:?} {state:?}"
            );
            assert_eq!(
                theme.combo_box.label(state).color,
                theme.colors.disabled_text,
                "{preset:?} {state:?}"
            );
        }
        assert_eq!(
            theme.combo_box.field(disabled | WidgetState::HOVER),
            theme.combo_box.field(disabled),
            "{preset:?}"
        );
        assert_eq!(
            theme.slider.thumb(disabled | WidgetState::HOVER),
            theme.slider.thumb(disabled),
            "{preset:?}"
        );
    }
    let monterey = Theme::preset(Preset::MacosMonterey).resolve(&ThemeContext::light());
    assert_ne!(
        monterey.combo_box.field(WidgetState::DISABLED),
        monterey.combo_box.field(WidgetState::HOVER)
    );
    assert_ne!(
        monterey.slider.thumb(WidgetState::DISABLED),
        monterey.slider.thumb(WidgetState::HOVER)
    );
}

#[test]
fn a_pressed_scroll_handle_wins_over_the_pointer() {
    for (preset, theme) in every_theme() {
        assert_eq!(
            theme
                .scrollbar
                .thumb(WidgetState::PRESSED | WidgetState::HOVER),
            theme.scrollbar.thumb(WidgetState::PRESSED),
            "{preset:?}"
        );
    }
}

#[test]
fn a_selected_item_wins_over_the_pointer() {
    for (preset, theme) in every_theme() {
        let selected = WidgetState::SELECTED;
        let hovered = selected | WidgetState::HOVER;
        assert_eq!(
            theme.tab.tab(hovered),
            theme.tab.tab(selected),
            "{preset:?}"
        );
        assert_eq!(
            theme.sidebar.item(hovered),
            theme.sidebar.item(selected),
            "{preset:?}"
        );
        assert_eq!(
            theme.sidebar.item_label(hovered),
            theme.sidebar.item_label(selected),
            "{preset:?}"
        );
        assert_eq!(
            theme.segmented_control.segment(hovered),
            theme.segmented_control.segment(selected),
            "{preset:?}"
        );
        assert_eq!(
            theme.segmented_control.label(hovered),
            theme.segmented_control.label(selected),
            "{preset:?}"
        );
    }
    let monterey = Theme::preset(Preset::MacosMonterey).resolve(&ThemeContext::dark());
    assert_ne!(
        monterey.tab.tab(WidgetState::HOVER),
        monterey.tab.tab(WidgetState::SELECTED)
    );
    assert_ne!(
        monterey.segmented_control.segment(WidgetState::HOVER),
        monterey.segmented_control.segment(WidgetState::SELECTED)
    );
}

/// The on track keeps the accent under the pointer; disabled, the off track
/// takes its own fill and the on track the accent at the preset's alpha
/// (0x80 on `macos-monterey`, 0x73 on `macos-tahoe`); no state draws an
/// outline; the disabled knob is one colour on and off.
#[test]
fn the_switch_s_disabled_look_wins_and_no_track_has_an_outline() {
    for (preset, theme) in every_theme() {
        let alpha = match preset {
            Preset::MacosTahoe => 0x73,
            _ => 0x80,
        };
        let on = WidgetState::CHECKED;
        let off_disabled = WidgetState::DISABLED;
        let on_disabled = off_disabled | on;
        assert_eq!(
            theme.switch.track(on | WidgetState::HOVER),
            theme.switch.track(on),
            "{preset:?}"
        );
        assert_eq!(
            theme.switch.track(off_disabled | WidgetState::HOVER),
            theme.switch.track(off_disabled),
            "{preset:?}"
        );
        let [r, g, b, _] = theme.colors.accent.fill.to_srgb8();
        for state in [on_disabled, on_disabled | WidgetState::HOVER] {
            assert_eq!(
                theme.switch.track(state).fill,
                Paint::Solid(Color::from_srgb8(r, g, b, alpha)),
                "{preset:?} {state:?}"
            );
        }
        for state in [
            WidgetState::NONE,
            WidgetState::HOVER,
            on,
            on | WidgetState::HOVER,
            off_disabled,
            on_disabled,
        ] {
            assert_eq!(
                theme.switch.track(state).stroke,
                Stroke::None,
                "{preset:?} {state:?}"
            );
            assert_eq!(
                theme.switch.thumb(state).stroke,
                Stroke::None,
                "{preset:?} {state:?}"
            );
        }
        assert_eq!(
            theme.switch.thumb(on_disabled),
            theme.switch.thumb(off_disabled),
            "{preset:?}"
        );
    }
}

#[test]
fn an_inactive_window_has_its_preset_s_inactive_title_bar() {
    for ctx in [ThemeContext::light(), ThemeContext::dark()] {
        let state = WindowState::default();
        let title_bar = |preset: Preset, active: bool| {
            Theme::preset(preset)
                .resolve(&ctx.clone().with_window_active(active))
                .window
                .title_bar(state)
        };
        assert_ne!(
            title_bar(Preset::MacosMonterey, false),
            title_bar(Preset::MacosMonterey, true)
        );
        assert_eq!(
            title_bar(Preset::MacosTahoe, false),
            title_bar(Preset::MacosTahoe, true)
        );
    }
}

/// Right to left moves the pop-up button's padding from its left to its
/// right side, and changes nothing else.
#[test]
fn right_to_left_mirrors_the_pop_up_button_s_padding_alone() {
    for preset in [Preset::MacosMonterey, Preset::MacosTahoe] {
        for ctx in [ThemeContext::light(), ThemeContext::dark()] {
            let theme = Theme::preset(preset);
            let ltr = theme.resolve(&ctx);
            let rtl = theme.resolve(&ctx.clone().with_direction(Direction::Rtl));
            let field = |t: &ResolvedTheme| t.combo_box.field(WidgetState::NONE);
            let (l, r) = (field(&ltr), field(&rtl));
            assert_eq!(r.padding.right, l.padding.left, "{preset:?}");
            assert_eq!(r.padding.top, l.padding.top, "{preset:?}");
            assert_eq!(r.padding.bottom, l.padding.bottom, "{preset:?}");
            assert_ne!(r.stated, l.stated, "{preset:?}");
            let mut same = rtl.clone();
            same.context = ltr.context.clone();
            same.combo_box = ltr.combo_box.clone();
            assert_eq!(same, ltr, "{preset:?}");
        }
    }
}

/// The text scale of the test below.
const TEXT_SCALE_TWO: f32 = 2.0;

/// The text scale is linear on macOS and not rounded: at 2.0 every font size
/// and line height is twice the one at 1.0, each `defined_size` stays the
/// unscaled size, and the layout, the icons and a widget's body do not change.
#[test]
fn at_text_scale_2_every_font_size_is_doubled() -> TestResult {
    for preset in [Preset::MacosMonterey, Preset::MacosTahoe] {
        for scheme in [Scheme::Light, Scheme::Dark] {
            let ctx = ThemeContext::default().with_scheme(scheme);
            let base = Theme::preset(preset).resolve(&ctx);
            let double = Theme::preset(preset).resolve(&ctx.with_text_scale(TEXT_SCALE_TWO));
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
                    t.tooltip.text.clone(),
                    t.dialog.title.clone(),
                    t.menu.item_label(WidgetState::NONE),
                    t.window.title(WindowState::default()),
                ])
            };
            for (one, two) in slots(&base)?.into_iter().zip(slots(&double)?) {
                let size = one.defined_size.ok_or("no defined size")?;
                assert_eq!(two.defined_size, Some(size), "{preset:?} {scheme:?}");
                assert_eq!(
                    two.size_px,
                    one.size_px * TEXT_SCALE_TWO,
                    "{preset:?} {scheme:?}"
                );
                assert_eq!(
                    two.line_height,
                    one.line_height * TEXT_SCALE_TWO,
                    "{preset:?} {scheme:?}"
                );
            }
            let rest = ButtonQuery::new(WidgetState::NONE);
            assert_eq!(double.button.body(rest), base.button.body(rest));
            assert_eq!(double.layout, base.layout);
            assert_eq!(double.icons, base.icons);
        }
    }
    Ok(())
}

/// `MotionPreference::None` sets the dialog's entrance to
/// `DialogEntranceStyle::None`; under full and reduced motion it stays
/// unstated, as the presets state none. The presets state no motion token.
#[test]
fn no_motion_sets_no_dialog_entrance() {
    for preset in [Preset::MacosMonterey, Preset::MacosTahoe] {
        let resolve = |motion: MotionPreference| {
            Theme::preset(preset).resolve(&ThemeContext::light().with_motion(motion))
        };
        let none = resolve(MotionPreference::None);
        assert_eq!(none.dialog.entrance, Some(DialogEntranceStyle::None));
        assert_eq!(none.motion_tokens, MotionTokens::default());
        for motion in [MotionPreference::Full, MotionPreference::Reduced] {
            let theme = resolve(motion);
            assert_eq!(theme.dialog.entrance, None, "{preset:?} {motion:?}");
            assert_eq!(theme.motion_tokens, MotionTokens::default());
        }
    }
}
