use super::*;

use crate::next::preset::Theme;
use crate::next::query::{ButtonQuery, ListQuery, MotionPreference};
use crate::next::window::WindowState;

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn both_schemes() -> [ResolvedTheme; 2] {
    [ThemeContext::light(), ThemeContext::dark()]
        .map(|ctx| Theme::preset(Preset::Windows11).resolve(&ctx))
}

#[test]
fn the_preset_defines_the_normal_contrast_alone() {
    assert_eq!(CONTRAST_LEVELS, &[Contrast::Normal]);
    let ctx = ThemeContext::light().with_contrast(Contrast::High);
    let high = Theme::preset(Preset::Windows11).resolve(&ctx);
    assert_eq!(high.context.contrast, Contrast::Normal);
}

#[test]
fn the_constant_colours_are_their_8_bit_channels() {
    assert_eq!(LIGHT.window_foreground.to_srgb8(), [0x00, 0x00, 0x00, 0xe4]);
    assert_eq!(DARK.switch_hover_on.to_srgb8(), [0x20, 0x20, 0x20, 0x19]);
    assert_eq!(DARK.scrollbar_thumb, None);
}

/// The push button's and the text field's outlines lie outside their fills
/// (`InnerBorderEdge`), the primary button's inside (`OuterBorderEdge`).
#[test]
fn the_default_button_and_the_text_field_fill_to_the_outline_s_inner_edge() {
    let align = |stroke: &Stroke| match stroke {
        Stroke::Uniform { align, .. } => Some(*align),
        _ => None,
    };
    for theme in both_schemes() {
        let rest = ButtonQuery::new(WidgetState::NONE);
        assert_eq!(
            align(&theme.button.body(rest).stroke),
            Some(StrokeAlign::Outside)
        );
        assert_eq!(
            align(&theme.button.body(ButtonQuery::primary()).stroke),
            Some(StrokeAlign::Inside)
        );
        for state in [WidgetState::NONE, WidgetState::FOCUSED] {
            assert_eq!(
                align(&theme.input.field(state).stroke),
                Some(StrokeAlign::Outside),
                "{state:?}"
            );
        }
    }
}

#[test]
fn a_primary_button_keeps_the_accent_fill_until_disabled() {
    for theme in both_schemes() {
        let accent_fill = theme
            .button
            .body(ButtonQuery::new(WidgetState::CHECKED))
            .fill;
        for state in [
            WidgetState::HOVER,
            WidgetState::PRESSED,
            WidgetState::CHECKED,
        ] {
            let query = ButtonQuery::primary().state(state);
            assert_eq!(theme.button.body(query).fill, accent_fill, "{state:?}");
        }
        let disabled = ButtonQuery::primary().state(WidgetState::DISABLED);
        assert_eq!(
            theme.button.body(disabled).fill,
            theme
                .button
                .body(ButtonQuery::new(WidgetState::DISABLED))
                .fill
        );
        assert_eq!(
            theme.button.label(disabled).color,
            theme.colors.disabled_text
        );
        let pressed_hover = ButtonQuery::new(WidgetState::PRESSED | WidgetState::HOVER);
        assert_eq!(
            theme.button.body(pressed_hover),
            theme.button.body(ButtonQuery::new(WidgetState::PRESSED))
        );
        let pressed_checked = ButtonQuery::new(WidgetState::PRESSED | WidgetState::CHECKED);
        assert_eq!(
            theme.button.label(pressed_checked).color,
            theme
                .button
                .label(ButtonQuery::new(WidgetState::PRESSED))
                .color
        );
    }
}

#[test]
fn a_checked_indicator_keeps_its_fill_under_the_pointer_and_loses_it_when_disabled() {
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
        assert_eq!(
            theme
                .checkbox
                .indicator(checked | WidgetState::DISABLED)
                .fill,
            theme.checkbox.indicator(WidgetState::DISABLED).fill
        );
    }
}

#[test]
fn the_switch_s_track_has_no_outline_when_on() {
    for theme in both_schemes() {
        let on_hover = theme
            .switch
            .track(WidgetState::HOVER | WidgetState::CHECKED);
        assert_ne!(on_hover, theme.switch.track(WidgetState::HOVER));
        assert_ne!(on_hover, theme.switch.track(WidgetState::CHECKED));
        for state in [
            WidgetState::CHECKED,
            WidgetState::CHECKED | WidgetState::HOVER,
            WidgetState::CHECKED | WidgetState::DISABLED,
            WidgetState::CHECKED | WidgetState::DISABLED | WidgetState::HOVER,
        ] {
            assert_eq!(theme.switch.track(state).stroke, Stroke::None, "{state:?}");
        }
        assert_ne!(
            theme.switch.track(WidgetState::DISABLED).stroke,
            Stroke::None
        );
    }
}

#[test]
fn a_selected_item_wins_over_the_pointer() {
    for theme in both_schemes() {
        let selected = WidgetState::SELECTED;
        let hovered = selected | WidgetState::HOVER;
        let row = |state| theme.list.highlight(ListQuery::new(state));
        assert_eq!(row(hovered), row(selected));
        assert_eq!(theme.sidebar.item(hovered), theme.sidebar.item(selected));
        assert_ne!(theme.tab.tab(WidgetState::HOVER), theme.tab.tab(selected));
        assert_eq!(theme.tab.tab(hovered), theme.tab.tab(selected));
        assert_eq!(theme.tab.label(hovered), theme.tab.label(selected));
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
fn a_pressed_link_wins_over_the_pointer_and_a_disabled_one_over_both() {
    for theme in both_schemes() {
        let pressed = theme.link.label(WidgetState::PRESSED);
        assert_eq!(
            theme.link.label(WidgetState::PRESSED | WidgetState::HOVER),
            pressed
        );
        assert_eq!(
            theme
                .link
                .label(WidgetState::DISABLED | WidgetState::PRESSED)
                .color,
            theme.link.label(WidgetState::DISABLED).color
        );
    }
}

#[test]
fn an_inactive_window_has_the_inactive_title() {
    for ctx in [ThemeContext::light(), ThemeContext::dark()] {
        let theme = Theme::preset(Preset::Windows11);
        let active = theme.resolve(&ctx);
        let inactive = theme.resolve(&ctx.clone().with_window_active(false));
        let state = WindowState::default();
        assert_eq!(
            inactive.window.title_bar(state),
            active.window.title_bar(state)
        );
        assert_ne!(
            inactive.window.title(state).color,
            active.window.title(state).color
        );
    }
}

/// Right to left mirrors the per-side values the preset states unequal on
/// the left and the right, and nothing else.
#[test]
fn right_to_left_mirrors_the_unequal_sides() {
    for ctx in [ThemeContext::light(), ThemeContext::dark()] {
        let theme = Theme::preset(Preset::Windows11);
        let ltr = theme.resolve(&ctx);
        let rtl = theme.resolve(&ctx.clone().with_direction(Direction::Rtl));
        let mirrored = |e: Edges| Edges::new(e.top, e.left, e.bottom, e.right);
        let row = ListQuery::new(WidgetState::NONE);
        assert_eq!(ltr.list.row(row).padding.left, LIST_ROW_PADDING_LEFT);
        assert_eq!(
            rtl.list.row(row).padding,
            mirrored(ltr.list.row(row).padding)
        );
        assert_eq!(
            rtl.input.field(WidgetState::NONE).padding,
            mirrored(ltr.input.field(WidgetState::NONE).padding)
        );
        assert_eq!(
            rtl.toolbar.bar(WidgetState::NONE).padding,
            mirrored(ltr.toolbar.bar(WidgetState::NONE).padding)
        );
        let mut same = rtl.clone();
        same.context = ltr.context.clone();
        same.list = ltr.list.clone();
        same.input = ltr.input.clone();
        same.text_area = ltr.text_area.clone();
        same.toolbar = ltr.toolbar.clone();
        same.combo_box = ltr.combo_box.clone();
        same.expander = ltr.expander.clone();
        assert_eq!(same, ltr);
    }
}

/// The text scale of the test below.
const TEXT_SCALE_TWO: f32 = 2.0;

/// The text scale is linear on Windows and not rounded: at 2.0 every font
/// size and line height is twice the one at 1.0, each `defined_size` stays
/// the unscaled size, and nothing but the text changes.
#[test]
fn at_text_scale_2_every_font_size_is_doubled() -> TestResult {
    for scheme in [Scheme::Light, Scheme::Dark] {
        let ctx = ThemeContext::default().with_scheme(scheme);
        let base = Theme::preset(Preset::Windows11).resolve(&ctx);
        let double = Theme::preset(Preset::Windows11).resolve(&ctx.with_text_scale(TEXT_SCALE_TWO));
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
/// `DialogEntranceStyle::None`; WinUI's `ScaleFade` stays under full and
/// under reduced motion. The preset states no motion token.
#[test]
fn no_motion_removes_the_dialog_s_entrance() {
    let resolve = |motion: MotionPreference| {
        Theme::preset(Preset::Windows11).resolve(&ThemeContext::light().with_motion(motion))
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
