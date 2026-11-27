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
    assert_eq!(DARK.checkbox_hover.to_srgb8(), [0xff, 0xff, 0xff, 0x0b]);
    assert_eq!(DARK.switch_hover_on, None);
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
    for (theme, c) in both_schemes()
        .iter()
        .zip([constants(Scheme::Light), constants(Scheme::Dark)])
    {
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
            c.accent_text_secondary
        );
    }
}

/// A checked box and a checked radio button change their fill under the
/// pointer ([`a_checked_or_mixed_box_hovered_and_pressed_takes_the_accent_s_secondary_and_tertiary_fills`]);
/// disabled, a checked or mixed box and a checked radio button take `AccentFillColorDisabled`
/// (#37000000 / #28FFFFFF), filled and stroked, where an unchecked one keeps
/// the disabled fill, outlined in `ControlStrongStrokeColorDisabled`
/// (CheckBox_themeresources.xaml:220, :44; RadioButton_themeresources.xaml:137,
/// :21); the mark takes `TextOnAccentFillColorDisabled` (#FFFFFF / #87FFFFFF),
/// the dot the rest's colour.
#[test]
fn a_checked_indicator_keeps_its_fill_under_the_pointer_and_its_own_when_disabled() {
    let [light, dark] = both_schemes();
    for (theme, accent_disabled, mark_disabled, colors) in [
        (light, [0, 0, 0, 0x37], [0xff, 0xff, 0xff, 0xff], &LIGHT),
        (
            dark,
            [0xff, 0xff, 0xff, 0x28],
            [0xff, 0xff, 0xff, 0x87],
            &DARK,
        ),
    ] {
        let checked = WidgetState::CHECKED;
        let hovered = checked | WidgetState::HOVER;
        let disabled = WidgetState::DISABLED;
        assert_ne!(
            theme.checkbox.indicator(WidgetState::HOVER),
            theme.checkbox.indicator(checked)
        );
        assert_ne!(
            theme.checkbox.indicator(hovered),
            theme.checkbox.indicator(checked)
        );
        assert_ne!(
            theme.radio.indicator(hovered),
            theme.radio.indicator(checked)
        );
        for part in [
            theme.checkbox.indicator(checked | disabled),
            theme
                .checkbox
                .indicator(WidgetState::INDETERMINATE | disabled),
            theme.radio.indicator(checked | disabled),
        ] {
            assert_ne!(part.fill, theme.checkbox.indicator(disabled).fill);
            assert_eq!(
                solid(&part.fill).map(Color::to_srgb8),
                Some(accent_disabled)
            );
            let stroke = match part.stroke {
                Stroke::Uniform { color, .. } => Some(color.to_srgb8()),
                _ => None,
            };
            assert_eq!(stroke, Some(accent_disabled), "{:?}", part.stroke);
        }
        assert_eq!(
            theme
                .checkbox
                .checkmark(checked | disabled)
                .color
                .to_srgb8(),
            mark_disabled
        );
        assert_eq!(
            theme.radio.dot(checked | disabled).fill,
            theme.radio.dot(checked).fill
        );
        for state in [
            disabled,
            disabled | WidgetState::HOVER,
            disabled | WidgetState::PRESSED,
        ] {
            let edge = Stroke::uniform(theme.layout.border_width, colors.strong_stroke_disabled);
            assert_eq!(theme.checkbox.indicator(state).stroke, edge, "{state:?}");
            assert_eq!(theme.radio.indicator(state).stroke, edge, "{state:?}");
            assert_eq!(
                theme.radio.indicator(state).fill,
                theme.radio.indicator(disabled).fill,
                "{state:?}"
            );
        }
    }
}

/// The hovered on switch and the hovered check box are WinUI's whole
/// hovered fills (spec §9, step 2): `AccentFillColorSecondary`, the on
/// track's accent fill at 0.9 opacity, and `ControlAltFillColorTertiary`.
#[test]
fn the_hovered_switch_and_check_box_are_whole_fills() {
    let [light, dark] = both_schemes();
    for (theme, checkbox_hover) in [(light, [0, 0, 0, 0x0f]), (dark, [0xff, 0xff, 0xff, 0x0b])] {
        let on = solid(&theme.switch.track(WidgetState::CHECKED).fill);
        let on_hover = solid(
            &theme
                .switch
                .track(WidgetState::HOVER | WidgetState::CHECKED)
                .fill,
        );
        assert_eq!(
            on_hover,
            on.map(|c| c.mul_alpha(ACCENT_FILL_SECONDARY_OPACITY))
        );
        for hovered in [
            theme.checkbox.indicator(WidgetState::HOVER).fill,
            theme.radio.indicator(WidgetState::HOVER).fill,
        ] {
            assert_eq!(solid(&hovered).map(Color::to_srgb8), Some(checkbox_hover));
        }
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

/// The push button checked is WinUI's ToggleButton checked: hovered the
/// accent's fill at the secondary opacity, pressed at the tertiary one in no
/// outline, lettered in the accent's secondary text, not the unchecked
/// hovered and pressed looks (ToggleButton_themeresources.xaml:12-13, :25,
/// :37).
#[test]
fn a_checked_push_button_hovered_or_pressed_keeps_its_checked_look() {
    for (theme, c) in both_schemes()
        .iter()
        .zip([constants(Scheme::Light), constants(Scheme::Dark)])
    {
        let body = |state| theme.button.body(ButtonQuery::new(state));
        let label = |state| theme.button.label(ButtonQuery::new(state)).color;
        let (checked, hover, pressed) = (
            WidgetState::CHECKED,
            WidgetState::HOVER,
            WidgetState::PRESSED,
        );
        assert_eq!(
            body(checked | hover).fill,
            Paint::Solid(c.accent_fill.mul_alpha(ACCENT_FILL_SECONDARY_OPACITY))
        );
        assert_eq!(body(checked | hover).stroke, body(checked).stroke);
        assert_eq!(label(checked | hover), label(checked));
        let checked_pressed = body(checked | pressed);
        assert_eq!(
            checked_pressed.fill,
            Paint::Solid(c.accent_fill.mul_alpha(ACCENT_FILL_TERTIARY_OPACITY))
        );
        assert_eq!(checked_pressed.stroke, Stroke::None);
        assert_ne!(body(pressed).stroke, Stroke::None);
        assert_eq!(label(checked | pressed), c.accent_text_secondary);
        assert_ne!(label(checked | pressed), label(pressed));
    }
}

/// The push button's, the text field's and the combo box's outline at rest,
/// hovered and disabled is the three sides of their elevation brushes,
/// `ControlStrokeColorDefault` #0F000000 / #12FFFFFF
/// (Common_themeresources_any.xaml:243, :39), and in light also the default
/// border (`CardStrokeColorDefault` #0F000000, :250); under a palette its
/// `border`.
#[test]
fn the_neutral_elevation_borders_are_control_stroke_color_default() {
    for (scheme, want) in [
        (Scheme::Light, srgb8(0x0000_000f)),
        (Scheme::Dark, srgb8(0xffff_ff12)),
    ] {
        let ctx = ThemeContext::default().with_scheme(scheme);
        let preset = Theme::preset(Preset::Windows11).resolve(&ctx);
        let p = distinct_palette(scheme);
        let custom = Theme::preset(Preset::Windows11)
            .with_palette(p.clone())
            .resolve(&ctx);
        let edges = |t: &ResolvedTheme| {
            [WidgetState::NONE, WidgetState::HOVER, WidgetState::DISABLED].map(|state| {
                [
                    stroke_color(&t.button.body(ButtonQuery::new(state)).stroke),
                    stroke_color(&t.input.field(state).stroke),
                    stroke_color(&t.combo_box.field(state).stroke),
                ]
            })
        };
        assert_eq!(edges(&preset), [[Some(want); 3]; 3], "{scheme:?}");
        assert_eq!(edges(&custom), [[Some(p.border); 3]; 3], "{scheme:?}");
    }
    assert_eq!(
        Theme::preset(Preset::Windows11)
            .resolve(&ThemeContext::light())
            .colors
            .window
            .border,
        Some(srgb8(0x0000_000f))
    );
}

/// The primary button and a checked push or toolbar button are outlined in
/// the three sides of `AccentControlElevationBorderBrush`,
/// `ControlStrokeColorOnAccentDefault` #14FFFFFF in both schemes
/// (Common_themeresources_any.xaml:245, :41), inside their edge
/// (`OuterBorderEdge`), at rest and hovered, under a palette too; pressed
/// and disabled in none (`ControlFillColorTransparentBrush`).
#[test]
fn the_accent_elevation_border_is_control_stroke_color_on_accent_default() {
    let on_accent = srgb8(0xffff_ff14);
    for scheme in [Scheme::Light, Scheme::Dark] {
        let ctx = ThemeContext::default().with_scheme(scheme);
        for theme in [
            Theme::preset(Preset::Windows11).resolve(&ctx),
            Theme::preset(Preset::Windows11)
                .with_palette(distinct_palette(scheme))
                .resolve(&ctx),
        ] {
            let width = theme.layout.border_width;
            let edge = Stroke::uniform(width, on_accent);
            let checked = WidgetState::CHECKED;
            let primary = |state| {
                theme
                    .button
                    .body(ButtonQuery::primary().state(state))
                    .stroke
            };
            let push = |state| theme.button.body(ButtonQuery::new(state)).stroke;
            let tool = |state| theme.toolbar.button(state).stroke;
            let tool_edge = Stroke::uniform(TOOLBAR_TOGGLE_BORDER_WIDTH, on_accent);
            for state in [WidgetState::NONE, WidgetState::HOVER] {
                assert_eq!(primary(state), edge, "{scheme:?} {state:?}");
                assert_eq!(push(checked | state), edge, "{scheme:?} {state:?}");
                assert_eq!(tool(checked | state), tool_edge, "{scheme:?} {state:?}");
            }
            for state in [WidgetState::PRESSED, WidgetState::DISABLED] {
                assert_eq!(primary(state), Stroke::None, "{scheme:?} {state:?}");
                assert_eq!(push(checked | state), Stroke::None, "{scheme:?} {state:?}");
                assert_eq!(tool(checked | state), Stroke::None, "{scheme:?} {state:?}");
            }
        }
    }
}

#[test]
fn a_checked_toolbar_button_keeps_its_fill() {
    for theme in both_schemes() {
        let checked = theme.toolbar.button(WidgetState::CHECKED);
        for state in [WidgetState::HOVER, WidgetState::PRESSED] {
            assert_ne!(theme.toolbar.button(state), checked, "{state:?}");
            assert_eq!(
                theme.toolbar.button(state | WidgetState::CHECKED).fill,
                checked.fill,
                "{state:?}"
            );
        }
        assert_eq!(
            theme
                .toolbar
                .button(WidgetState::HOVER | WidgetState::CHECKED),
            checked
        );
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

/// The system's settings of `accent` and, for `(Scheme::Light, Contrast::High)`, of the
/// selection's colours `highlight` alone.
fn system_with(accent: Color, highlight: Option<(Color, Color)>) -> SystemOverrides {
    use crate::next::reader::{OsColors, OsSelectionColors};
    let color_schemes = highlight
        .map(|(background, foreground)| {
            let colors = OsColors {
                selection: Some(OsSelectionColors::new(
                    Some(background),
                    Some(foreground),
                    None,
                    None,
                )),
                ..OsColors::default()
            };
            vec![((Scheme::Light, Contrast::High), colors)]
        })
        .unwrap_or_default();
    SystemOverrides {
        accent: Some(accent),
        color_schemes,
        ..SystemOverrides::default()
    }
}

/// The system's accent is one value for both schemes: without a shade it is
/// the accent and the accent's fill in either scheme.
#[test]
fn the_system_s_accent_applies_in_either_scheme() {
    let accent = Color::from_srgb8(0x12, 0x34, 0x56, 0xff);
    let theme = Theme::preset(Preset::Windows11).with_system_overrides(system_with(accent, None));
    for ctx in [ThemeContext::light(), ThemeContext::dark()] {
        let resolved = theme.resolve(&ctx);
        assert_eq!(resolved.colors.accent.fill, accent, "{:?}", ctx.scheme);
        assert_eq!(
            resolved.button.body(ButtonQuery::primary()).fill,
            Paint::Solid(accent),
            "{:?}",
            ctx.scheme
        );
    }
}

/// The system's high-contrast colours make `Contrast::High` available for
/// their scheme alone; there a stated colour replaces the accent's values,
/// and a key whose colour is not stated keeps the preset's.
#[test]
fn the_system_s_high_contrast_colours_make_high_available() {
    let accent = Color::from_srgb8(0x12, 0x34, 0x56, 0xff);
    let highlight = Color::from_srgb8(5, 5, 5, 0xff);
    let highlight_text = Color::from_srgb8(6, 6, 6, 0xff);
    let theme = Theme::preset(Preset::Windows11)
        .with_system_overrides(system_with(accent, Some((highlight, highlight_text))));
    let high = theme.resolve(&ThemeContext::light().with_contrast(Contrast::High));
    assert_eq!(high.context.contrast, Contrast::High);
    assert_eq!(high.colors.accent.fill, highlight);
    assert_eq!(high.colors.selection.foreground, highlight_text);
    assert_eq!(
        high.button.body(ButtonQuery::primary()).fill,
        Paint::Solid(highlight)
    );
    let preset = Theme::preset(Preset::Windows11).resolve(&ThemeContext::light());
    assert_eq!(
        high.button.body(ButtonQuery::new(WidgetState::NONE)).fill,
        preset.button.body(ButtonQuery::new(WidgetState::NONE)).fill
    );
    assert_eq!(high.layout.border_opacity, preset.layout.border_opacity);
    let dark = theme.resolve(&ThemeContext::dark().with_contrast(Contrast::High));
    assert_eq!(dark.context.contrast, Contrast::Normal);
    assert_eq!(dark.colors.accent.fill, accent);
}

/// Under high contrast the hovered unchecked box and circle are the
/// selection's text colour, a disabled checked or mixed box the disabled text
/// with its mark in the window colour, a disabled checked radio's circle
/// the window colour in the disabled text with its dot in the disabled text,
/// and a disabled unchecked box and circle the window colour in the disabled
/// text (WinUI's high-contrast dictionaries, CheckBox_themeresources.xaml:142,
/// :148, :136, :160, :144, :132; RadioButton_themeresources.xaml:81, :91, :87,
/// :95, :83, :79).
#[test]
fn a_high_contrast_check_box_takes_the_system_colours() {
    use crate::next::reader::{OsColors, OsSelectionColors, OsSurfaceOverrides};
    let [highlight, highlight_text, window, gray] =
        [1, 2, 3, 4].map(|b| Color::from_srgb8(0, 0, b, 0xff));
    let colors = OsColors {
        surfaces: Some(OsSurfaceOverrides {
            window: Some(SurfaceColors {
                background: Some(window),
                ..SurfaceColors::default()
            }),
            ..OsSurfaceOverrides::default()
        }),
        selection: Some(OsSelectionColors::new(
            Some(highlight),
            Some(highlight_text),
            None,
            None,
        )),
        disabled_text: Some(gray),
        ..OsColors::default()
    };
    let system = SystemOverrides {
        color_schemes: vec![((Scheme::Light, Contrast::High), colors)],
        ..SystemOverrides::default()
    };
    let theme = Theme::preset(Preset::Windows11).with_system_overrides(system);
    let high = theme.resolve(&ThemeContext::light().with_contrast(Contrast::High));
    assert_eq!(high.context.contrast, Contrast::High);
    let solid = Paint::Solid;
    assert_eq!(
        high.checkbox.indicator(WidgetState::HOVER).fill,
        solid(highlight_text)
    );
    assert_eq!(
        high.radio.indicator(WidgetState::HOVER).fill,
        solid(highlight_text)
    );
    let width = high.layout.border_width;
    for on in [WidgetState::CHECKED, WidgetState::INDETERMINATE] {
        let state = WidgetState::DISABLED | on;
        let indicator = high.checkbox.indicator(state);
        assert_eq!(indicator.fill, solid(gray));
        assert_eq!(indicator.stroke, Stroke::uniform(width, gray));
        assert_eq!(high.checkbox.checkmark(state).color, window);
    }
    let state = WidgetState::DISABLED | WidgetState::CHECKED;
    let circle = high.radio.indicator(state);
    assert_eq!(circle.fill, solid(window));
    assert_eq!(circle.stroke, Stroke::uniform(width, gray));
    assert_eq!(high.radio.dot(state).fill, solid(gray));
    for part in [
        high.checkbox.indicator(WidgetState::DISABLED),
        high.radio.indicator(WidgetState::DISABLED),
    ] {
        assert_eq!(part.fill, solid(window));
        assert_eq!(part.stroke, Stroke::uniform(width, gray));
    }
    let circle = high.radio.indicator(WidgetState::CHECKED);
    assert_eq!(circle.fill, solid(highlight_text));
    assert_eq!(circle.stroke, Stroke::uniform(width, highlight));
    assert_eq!(high.radio.dot(WidgetState::CHECKED).fill, solid(highlight));
    let normal = theme.resolve(&ThemeContext::light());
    assert_eq!(
        normal.checkbox.indicator(WidgetState::HOVER).fill,
        solid(LIGHT.checkbox_hover)
    );
}

/// A checked or mixed box at rest is filled and outlined in the accent's
/// fill, as WinUI paints it (CheckBox_themeresources.xaml:233, :221, :237,
/// :225), a checked radio button outlined in it (RadioButton_themeresources.xaml:142).
#[test]
fn a_mixed_box_is_the_accent_fill() -> TestResult {
    for theme in both_schemes() {
        let accent = theme.checkbox.indicator(WidgetState::CHECKED).fill;
        let Paint::Solid(color) = accent else {
            return Err("the checked fill is not solid".into());
        };
        let edge = Stroke::uniform(theme.layout.border_width, color);
        for state in [WidgetState::CHECKED, WidgetState::INDETERMINATE] {
            let part = theme.checkbox.indicator(state);
            assert_eq!(part.fill, accent);
            assert_eq!(part.stroke, edge);
        }
        assert_eq!(theme.radio.indicator(WidgetState::CHECKED).stroke, edge);
    }
    Ok(())
}

/// A checked or mixed box, hovered, is filled and outlined in
/// `AccentFillColorSecondaryBrush`, pressed in `AccentFillColorTertiaryBrush`:
/// the accent's fill at 0.9 and 0.8 (CheckBox_themeresources.xaml:234, :222,
/// :238, :226, :235, :223, :239, :227; Common_themeresources_any.xaml:330,
/// :331); the mark hovered as at rest, pressed `TextOnAccentFillColorSecondary`.
/// A checked radio button's circle the same (RadioButton_themeresources.xaml:147,
/// :143, :148, :144), its dot as at rest (:151, :152).
#[test]
fn a_checked_or_mixed_box_hovered_and_pressed_takes_the_accent_s_secondary_and_tertiary_fills()
-> TestResult {
    for theme in both_schemes() {
        let Paint::Solid(accent) = theme.checkbox.indicator(WidgetState::CHECKED).fill else {
            return Err("the checked fill is not solid".into());
        };
        let rest_mark = theme.checkbox.checkmark(WidgetState::CHECKED).color;
        for state in [WidgetState::CHECKED, WidgetState::INDETERMINATE] {
            for (extra, opacity) in [
                (WidgetState::HOVER, ACCENT_FILL_SECONDARY_OPACITY),
                (WidgetState::PRESSED, ACCENT_FILL_TERTIARY_OPACITY),
            ] {
                let expected = accent.mul_alpha(opacity);
                let part = theme.checkbox.indicator(state | extra);
                assert_eq!(part.fill, Paint::Solid(expected), "{state:?} {extra:?}");
                assert_eq!(
                    part.stroke,
                    Stroke::uniform(theme.layout.border_width, expected),
                    "{state:?} {extra:?}"
                );
            }
            assert_eq!(
                theme.checkbox.checkmark(state | WidgetState::HOVER).color,
                rest_mark
            );
            assert_ne!(
                theme.checkbox.checkmark(state | WidgetState::PRESSED).color,
                rest_mark
            );
        }
        let rest_dot = theme.radio.dot(WidgetState::CHECKED).fill;
        for (extra, opacity) in [
            (WidgetState::HOVER, ACCENT_FILL_SECONDARY_OPACITY),
            (WidgetState::PRESSED, ACCENT_FILL_TERTIARY_OPACITY),
        ] {
            let expected = accent.mul_alpha(opacity);
            let state = WidgetState::CHECKED | extra;
            let circle = theme.radio.indicator(state);
            assert_eq!(circle.fill, Paint::Solid(expected), "{extra:?}");
            assert_eq!(
                circle.stroke,
                Stroke::uniform(theme.layout.border_width, expected),
                "{extra:?}"
            );
            assert_eq!(theme.radio.dot(state).fill, rest_dot, "{extra:?}");
        }
    }
    Ok(())
}

/// Under high contrast the hovered on switch is the system's button face
/// (`ToggleSwitchFillOnPointerOver` = `SystemColorButtonFaceColorBrush`,
/// ToggleSwitch_themeresources.xaml:88), whatever the accent.
#[test]
fn a_high_contrast_hovered_on_switch_is_the_button_face() {
    use crate::next::reader::{OsColors, OsSurfaceOverrides};
    let accent = Color::from_srgb8(0x12, 0x34, 0x56, 0xff);
    let face = Color::from_srgb8(3, 3, 3, 0xff);
    let colors = OsColors {
        surfaces: Some(OsSurfaceOverrides {
            button: Some(SurfaceColors {
                background: Some(face),
                ..SurfaceColors::default()
            }),
            ..OsSurfaceOverrides::default()
        }),
        ..OsColors::default()
    };
    let system = SystemOverrides {
        accent: Some(accent),
        color_schemes: vec![((Scheme::Light, Contrast::High), colors)],
        ..SystemOverrides::default()
    };
    let theme = Theme::preset(Preset::Windows11).with_system_overrides(system);
    let high = theme.resolve(&ThemeContext::light().with_contrast(Contrast::High));
    assert_eq!(high.context.contrast, Contrast::High);
    assert_eq!(
        high.switch
            .track(WidgetState::HOVER | WidgetState::CHECKED)
            .fill,
        Paint::Solid(face)
    );
    let normal = theme.resolve(&ThemeContext::light());
    assert_eq!(
        normal
            .switch
            .track(WidgetState::HOVER | WidgetState::CHECKED)
            .fill,
        Paint::Solid(accent.mul_alpha(ACCENT_FILL_SECONDARY_OPACITY))
    );
}

/// Under high contrast the box at rest is the button face in the button text,
/// a checked box hovered the button text with its mark in the button face and
/// pressed the reverse, a mixed box hovered the highlight's text in the
/// highlight with its bar in the highlight and pressed the highlight with its
/// bar in the highlight's text, an unchecked box hovered outlined and pressed
/// filled and outlined in the highlight (CheckBox_themeresources.xaml:129-163
/// at microsoft-ui-xaml 8463f45); an unchecked circle hovered outlined in the
/// highlight and pressed filled and outlined in the highlight's text, a
/// checked one hovered the button face in the button text and pressed the
/// button face in it, its dot the button text in both
/// (RadioButton_themeresources.xaml:77-94).
#[test]
fn a_high_contrast_box_hovered_and_pressed_takes_the_system_colours() {
    use crate::next::reader::{OsColors, OsSelectionColors, OsSurfaceOverrides};
    let [face, button_text, highlight, highlight_text] =
        [1, 2, 3, 4].map(|b| Color::from_srgb8(0, 0, b, 0xff));
    let colors = OsColors {
        surfaces: Some(OsSurfaceOverrides {
            button: Some(SurfaceColors {
                background: Some(face),
                foreground: Some(button_text),
                ..SurfaceColors::default()
            }),
            ..OsSurfaceOverrides::default()
        }),
        selection: Some(OsSelectionColors::new(
            Some(highlight),
            Some(highlight_text),
            None,
            None,
        )),
        ..OsColors::default()
    };
    let system = SystemOverrides {
        color_schemes: vec![((Scheme::Light, Contrast::High), colors)],
        ..SystemOverrides::default()
    };
    let theme = Theme::preset(Preset::Windows11).with_system_overrides(system);
    let high = theme.resolve(&ThemeContext::light().with_contrast(Contrast::High));
    assert_eq!(high.context.contrast, Contrast::High);
    let width = high.layout.border_width;
    let look = |fill: Color, edge: Color| (Paint::Solid(fill), Stroke::uniform(width, edge));
    let of = |state: WidgetState| {
        let part = high.checkbox.indicator(state);
        (part.fill, part.stroke)
    };
    let mark = |state: WidgetState| high.checkbox.checkmark(state).color;
    let (checked, mixed) = (WidgetState::CHECKED, WidgetState::INDETERMINATE);
    let (hover, pressed) = (WidgetState::HOVER, WidgetState::PRESSED);
    assert_eq!(of(WidgetState::NONE), look(face, button_text));
    assert_eq!(
        high.radio.indicator(WidgetState::NONE).stroke,
        Stroke::uniform(width, button_text)
    );
    assert_eq!(of(hover).1, Stroke::uniform(width, highlight));
    assert_eq!(of(pressed), look(highlight, highlight));
    assert_eq!(of(checked), look(highlight, highlight));
    assert_eq!(mark(checked), highlight_text);
    assert_eq!(of(checked | hover), look(button_text, button_text));
    assert_eq!(mark(checked | hover), face);
    assert_eq!(of(checked | pressed), look(face, face));
    assert_eq!(mark(checked | pressed), button_text);
    assert_eq!(of(mixed | hover), look(highlight_text, highlight));
    assert_eq!(mark(mixed | hover), highlight);
    assert_eq!(of(mixed | pressed), look(highlight, highlight));
    assert_eq!(mark(mixed | pressed), highlight_text);
    let circle = |state: WidgetState| {
        let part = high.radio.indicator(state);
        (part.fill, part.stroke)
    };
    let dot = |state: WidgetState| high.radio.dot(state).fill;
    assert_eq!(circle(hover).1, Stroke::uniform(width, highlight));
    assert_eq!(circle(pressed), look(highlight_text, highlight_text));
    assert_eq!(circle(checked | hover), look(face, button_text));
    assert_eq!(dot(checked | hover), Paint::Solid(button_text));
    assert_eq!(circle(checked | pressed), look(face, face));
    assert_eq!(dot(checked | pressed), Paint::Solid(button_text));
}

/// Under high contrast the push button is the button face in the button text,
/// hovered the highlight's text in the highlight, pressed the highlight's
/// text in it, disabled the button face in the grey text
/// (Button_themeresources.xaml:78-89 at microsoft-ui-xaml 8463f45); the flat
/// (subtle) button the same at rest, hovered, pressed and disabled (:66-77);
/// the push button checked and hovered the button text in it lettered in the
/// button face, checked and pressed the highlight's text in it lettered in
/// the highlight (ToggleButton_themeresources.xaml:70-71, :82-83, :94-95);
/// the toolbar's button hovered and pressed the highlight behind the
/// highlight's text, checked and hovered or pressed the highlight's text,
/// checked and disabled the grey text (AppBarButton_themeresources.xaml:41-47,
/// AppBarToggleButton_themeresources.xaml:74-77).
#[test]
fn a_high_contrast_button_takes_the_system_colours() {
    use crate::next::reader::{OsColors, OsSelectionColors, OsSurfaceOverrides};
    let [face, button_text, highlight, highlight_text, gray] =
        [1, 2, 3, 4, 5].map(|b| Color::from_srgb8(0, 0, b, 0xff));
    let colors = OsColors {
        surfaces: Some(OsSurfaceOverrides {
            button: Some(SurfaceColors {
                background: Some(face),
                foreground: Some(button_text),
                ..SurfaceColors::default()
            }),
            ..OsSurfaceOverrides::default()
        }),
        selection: Some(OsSelectionColors::new(
            Some(highlight),
            Some(highlight_text),
            None,
            None,
        )),
        disabled_text: Some(gray),
        ..OsColors::default()
    };
    let system = SystemOverrides {
        color_schemes: vec![((Scheme::Light, Contrast::High), colors)],
        ..SystemOverrides::default()
    };
    let theme = Theme::preset(Preset::Windows11).with_system_overrides(system);
    let high = theme.resolve(&ThemeContext::light().with_contrast(Contrast::High));
    assert_eq!(high.context.contrast, Contrast::High);
    let width = high.layout.border_width;
    let look = |fill: Color, edge: Color| {
        (
            Paint::Solid(fill),
            Stroke::uniform_aligned(width, edge, StrokeAlign::Outside),
        )
    };
    let (hover, pressed, disabled) = (
        WidgetState::HOVER,
        WidgetState::PRESSED,
        WidgetState::DISABLED,
    );
    for query in [ButtonQuery::new(WidgetState::NONE), ButtonQuery::flat()] {
        let of = |state: WidgetState| {
            let body = high.button.body(query.state(state));
            (body.fill, body.stroke)
        };
        assert_eq!(of(WidgetState::NONE), look(face, button_text), "{query:?}");
        assert_eq!(of(hover), look(highlight_text, highlight), "{query:?}");
        assert_eq!(
            of(pressed),
            look(highlight_text, highlight_text),
            "{query:?}"
        );
        assert_eq!(of(disabled), look(face, gray), "{query:?}");
        assert_eq!(
            high.button.label(query.state(disabled)).color,
            gray,
            "{query:?}"
        );
    }
    let checked = WidgetState::CHECKED;
    let push = |state: WidgetState| {
        let body = high.button.body(ButtonQuery::new(state));
        (body.fill, body.stroke)
    };
    let push_label = |state: WidgetState| high.button.label(ButtonQuery::new(state)).color;
    let inside = |edge: Color| Stroke::uniform(width, edge);
    assert_eq!(push(checked).1, inside(highlight_text));
    assert_eq!(
        push(checked | hover),
        (Paint::Solid(button_text), inside(button_text))
    );
    assert_eq!(push_label(checked | hover), face);
    assert_eq!(
        push(checked | pressed),
        (Paint::Solid(highlight_text), inside(highlight_text))
    );
    assert_eq!(push_label(checked | pressed), highlight);
    assert_eq!(push(checked | disabled), look(face, gray));
    let primary = |state: WidgetState| high.button.body(ButtonQuery::primary().state(state)).stroke;
    for state in [WidgetState::NONE, hover, pressed] {
        assert_eq!(primary(state), inside(highlight), "{state:?}");
    }
    assert_eq!(primary(disabled), inside(gray));
    let toolbar_edge = |state: WidgetState| high.toolbar.button(state).stroke;
    for state in [WidgetState::NONE, hover, pressed] {
        assert_eq!(
            toolbar_edge(checked | state),
            Stroke::uniform(TOOLBAR_TOGGLE_BORDER_WIDTH, highlight),
            "{state:?}"
        );
    }
    assert_eq!(
        toolbar_edge(checked | disabled),
        Stroke::uniform(TOOLBAR_TOGGLE_BORDER_WIDTH, gray)
    );
    let toolbar = |state: WidgetState| high.toolbar.button(state).fill;
    assert_eq!(toolbar(hover), Paint::Solid(highlight));
    assert_eq!(toolbar(pressed), Paint::Solid(highlight));
    assert_eq!(
        toolbar(WidgetState::CHECKED | hover),
        Paint::Solid(highlight_text)
    );
    assert_eq!(
        toolbar(WidgetState::CHECKED | pressed),
        Paint::Solid(highlight_text)
    );
    assert_eq!(toolbar(WidgetState::CHECKED | disabled), Paint::Solid(gray));
    assert_eq!(high.toolbar.button_label(hover).color, highlight_text);
}

/// The focus visual is the 2px outer ring around the 1px inner one in
/// `FocusStrokeColorInner` (#B3FFFFFF / #B3000000), 1 px outside the part
/// (Button's `FocusVisualMargin` -3); a list row's lies inside it, its outer
/// ring's inner edge 3 px inside (`ListViewItem`'s `FocusVisualMargin` 1,
/// ListViewItem_themeresources.xaml:248-252); under high contrast the rings
/// are the window text around the window (Common_themeresources_any.xaml:472,
/// :473; ListViewItem_themeresources.xaml:94, :95).
#[test]
fn the_focus_visual_is_two_rings_and_a_list_row_s_lies_inside_it() -> TestResult {
    use crate::next::reader::{OsColors, OsSurfaceOverrides};
    for (theme, inner) in both_schemes().into_iter().zip([0xffff_ffb3, 0x0000_00b3]) {
        let ring = &theme.focus_ring;
        assert_eq!(ring.kind, FocusKind::Ring);
        assert_eq!(ring.stroke, Stroke::uniform(2.0, theme.colors.focus_ring));
        assert_eq!(ring.inner_stroke, Stroke::uniform(1.0, srgb8(inner)));
        assert_eq!(ring.offset, 1.0);
        assert_eq!(ring.trigger, Some(FocusTrigger::KeyboardOnly));
        let row = theme.list.row(ListQuery::new(WidgetState::NONE));
        let Some(row_ring) = row.focus.as_deref() else {
            return Err("the list row states no focus ring".into());
        };
        assert_eq!(row_ring.offset, -3.0);
        assert_eq!(
            (&row_ring.stroke, &row_ring.inner_stroke, row_ring.trigger),
            (&ring.stroke, &ring.inner_stroke, ring.trigger)
        );
    }
    let [window_text, window] = [1, 2].map(|b| Color::from_srgb8(0, 0, b, 0xff));
    let colors = OsColors {
        surfaces: Some(OsSurfaceOverrides {
            window: Some(SurfaceColors {
                background: Some(window),
                foreground: Some(window_text),
                ..SurfaceColors::default()
            }),
            ..OsSurfaceOverrides::default()
        }),
        ..OsColors::default()
    };
    let system = SystemOverrides {
        color_schemes: vec![((Scheme::Light, Contrast::High), colors)],
        ..SystemOverrides::default()
    };
    let theme = Theme::preset(Preset::Windows11).with_system_overrides(system);
    let high = theme.resolve(&ThemeContext::light().with_contrast(Contrast::High));
    assert_eq!(high.context.contrast, Contrast::High);
    assert_eq!(high.focus_ring.stroke, Stroke::uniform(2.0, window_text));
    assert_eq!(high.focus_ring.inner_stroke, Stroke::uniform(1.0, window));
    let row = high.list.row(ListQuery::new(WidgetState::NONE));
    let Some(row_ring) = row.focus.as_deref() else {
        return Err("the high-contrast list row states no focus ring".into());
    };
    assert_eq!(row_ring.inner_stroke, Stroke::uniform(1.0, window));
    Ok(())
}

// ---------------------------------------------------------------------------
// The palette and the explicit accent
// ---------------------------------------------------------------------------

/// The 8-bit opaque colour `0xRRGGBB`.
fn hex(rgb: u32) -> Color {
    let [_, r, g, b] = rgb.to_be_bytes();
    Color::from_srgb8(r, g, b, u8::MAX)
}

/// An explicit accent whose text is white in both schemes
/// (`accent-decision.md`, W4's cases).
const PURPLE: u32 = 0x5c2d91;

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

/// The preset's constants of `scheme`.
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

/// Every colour the theme draws in the accent's fill of the resolved scheme
/// (WinUI's `AccentFillColorDefault`), each by its name.
fn accent_fills(theme: &ResolvedTheme) -> [(&'static str, Option<Color>); 10] {
    let checked = WidgetState::CHECKED;
    [
        (
            "primary button",
            solid(&theme.button.body(ButtonQuery::primary()).fill),
        ),
        (
            "checked button",
            solid(&theme.button.body(ButtonQuery::new(checked)).fill),
        ),
        (
            "checked check box",
            solid(&theme.checkbox.indicator(checked).fill),
        ),
        (
            "checked radio button",
            solid(&theme.radio.indicator(checked).fill),
        ),
        ("on switch", solid(&theme.switch.track(checked).fill)),
        (
            "checked toolbar button",
            solid(&theme.toolbar.button(checked).fill),
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
            "focused text field's outline",
            stroke_color(&theme.input.field(WidgetState::FOCUSED).stroke),
        ),
    ]
}

/// The link's colours at rest, hovered and pressed.
fn link_trio(theme: &ResolvedTheme) -> [Color; 3] {
    [WidgetState::NONE, WidgetState::HOVER, WidgetState::PRESSED]
        .map(|state| theme.link.label(state).color)
}

/// Every text and mark the theme draws on the accent's fill, each by its
/// name.
fn on_accent(theme: &ResolvedTheme) -> [(&'static str, Option<Color>); 5] {
    let checked = WidgetState::CHECKED;
    [
        (
            "primary button's label",
            Some(theme.button.label(ButtonQuery::primary()).color),
        ),
        (
            "checked button's label",
            Some(theme.button.label(ButtonQuery::new(checked)).color),
        ),
        ("check mark", Some(theme.checkbox.checkmark(checked).color)),
        ("radio button's dot", solid(&theme.radio.dot(checked).fill)),
        ("on switch's knob", solid(&theme.switch.thumb(checked).fill)),
    ]
}

/// An explicit accent is written as given: the accent, every use of the
/// accent's fill, the informational fill and the selection; the link and its
/// hovered and pressed colours, and every other role, keep the preset's.
#[test]
fn an_explicit_accent_is_written_as_given() {
    let accent = hex(PURPLE);
    for scheme in [Scheme::Light, Scheme::Dark] {
        let ctx = ThemeContext::default().with_scheme(scheme);
        let preset = Theme::preset(Preset::Windows11).resolve(&ctx);
        let theme = Theme::preset(Preset::Windows11)
            .with_accent(accent)
            .resolve(&ctx);
        for (name, got) in accent_fills(&theme) {
            assert_eq!(got, Some(accent), "{scheme:?} {name}");
        }
        assert_eq!(link_trio(&theme), link_trio(&preset), "{scheme:?}");
        let mut want = preset.colors.clone();
        want.accent.fill = accent;
        want.accent.foreground = text_on_accent(accent, scheme);
        want.info.fill = accent;
        want.selection.background = accent;
        want.selection.unfocused_background = Some(accent);
        want.text_selection.background = accent;
        assert_eq!(theme.colors, want, "{scheme:?}");
    }
}

/// An explicit accent stands over the system's accent and its six shades:
/// none of the shades is drawn as the accent's fill, and the link's three
/// colours are the system's text shades.
#[test]
fn an_explicit_accent_stands_over_the_system_s_shades() {
    use crate::next::reader::WindowsAccentRamp;
    let accent = hex(PURPLE);
    let [light1, light2, light3] = [hex(0xa0c0e0), hex(0xb0d0f0), hex(0xc0e0ff)];
    let [dark1, dark2, dark3] = [hex(0x103050), hex(0x082840), hex(0x002030)];
    let shades = [light1, light2, light3, dark1, dark2, dark3];
    let system = SystemOverrides {
        accent: Some(hex(0x2060a0)),
        windows_accent_ramp: Some(WindowsAccentRamp {
            light: [Some(light1), Some(light2), Some(light3)],
            dark: [Some(dark1), Some(dark2), Some(dark3)],
        }),
        ..SystemOverrides::default()
    };
    for (scheme, link) in [
        (Scheme::Light, [dark2, dark3, dark1]),
        (Scheme::Dark, [light3, light3, light2]),
    ] {
        let theme = Theme::preset(Preset::Windows11)
            .with_system_overrides(system.clone())
            .with_accent(accent)
            .resolve(&ThemeContext::default().with_scheme(scheme));
        assert_eq!(theme.colors.accent.fill, accent, "{scheme:?}");
        assert_eq!(theme.colors.info.fill, accent, "{scheme:?}");
        assert_eq!(theme.colors.selection.background, accent, "{scheme:?}");
        for (name, got) in accent_fills(&theme) {
            assert_eq!(got, Some(accent), "{scheme:?} {name}");
            assert!(
                got.is_none_or(|got| !shades.contains(&got)),
                "{scheme:?} {name}"
            );
        }
        assert_eq!(theme.colors.link, link[0], "{scheme:?}");
        assert_eq!(link_trio(&theme), link, "{scheme:?}");
    }
}

/// The text on an explicit accent is the scheme's
/// `TextOnAccentFillColorPrimary` where it reads at 3:1 or better, else the
/// other scheme's (`accent-decision.md`, W4's four cases), and every text and
/// mark on the accent's fill is that role.
#[test]
fn the_text_on_an_explicit_accent_reads_on_it() {
    let white = [0xff, 0xff, 0xff, 0xff];
    let black = [0x00, 0x00, 0x00, 0xff];
    assert_eq!(LIGHT.accent_foreground.to_srgb8(), white);
    assert_eq!(DARK.accent_foreground.to_srgb8(), black);
    for (scheme, accent, want) in [
        (Scheme::Light, 0x0078d4, white),
        (Scheme::Light, 0xffb900, black),
        (Scheme::Dark, 0x7b2ff7, black),
        (Scheme::Dark, PURPLE, white),
    ] {
        let theme = Theme::preset(Preset::Windows11)
            .with_accent(hex(accent))
            .resolve(&ThemeContext::default().with_scheme(scheme));
        let role = theme.colors.accent.foreground;
        assert_eq!(role.to_srgb8(), want, "{scheme:?} {accent:06x}");
        for (name, got) in on_accent(&theme) {
            assert_eq!(got, Some(role), "{scheme:?} {accent:06x} {name}");
        }
    }
}

/// A palette states the roles its slots name (spec §7), its accent written
/// as given; the link's hovered and pressed colours are its link.
#[test]
fn a_palette_states_its_roles() {
    for scheme in [Scheme::Light, Scheme::Dark] {
        let p = distinct_palette(scheme);
        let theme = Theme::preset(Preset::Windows11)
            .with_palette(p.clone())
            .resolve(&ThemeContext::default().with_scheme(scheme));
        let colors = &theme.colors;
        for (name, surface, background) in [
            ("window", colors.window, p.window_bg),
            ("view", colors.view, p.view_bg),
            ("header", colors.header, p.header_bg),
            ("sidebar", colors.sidebar, p.sidebar_bg),
            ("card", colors.card, p.surface_bg),
            ("dialog", colors.dialog, p.surface_bg),
            ("popover", colors.popover, p.surface_bg),
            ("tooltip", colors.tooltip, p.tooltip_bg),
        ] {
            assert_eq!(surface.background, Some(background), "{scheme:?} {name}");
            assert_eq!(surface.foreground, Some(p.foreground), "{scheme:?} {name}");
        }
        assert_eq!(colors.window.foreground_muted, Some(p.foreground_muted));
        assert_eq!(colors.window.border, Some(p.border));
        assert_eq!(colors.window.divider, Some(p.divider));
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
        let c = constants(scheme);
        for (status, fill) in [
            (colors.error, p.error),
            (colors.warning, p.warning),
            (colors.success, p.success),
            (colors.info, p.info),
        ] {
            assert_eq!(status.fill, fill, "{scheme:?}");
            assert_eq!(status.foreground, c.status_foreground, "{scheme:?}");
            assert_eq!(status.origin, RoleOrigin::Custom, "{scheme:?}");
        }
        assert_eq!(colors.focus_ring, c.focus_ring);
        assert_eq!(colors.disabled_text, c.disabled_text);
        assert_eq!(colors.shadow, c.shadow);
        for (name, got) in accent_fills(&theme) {
            assert_eq!(got, Some(p.accent), "{scheme:?} {name}");
        }
        for (name, got) in on_accent(&theme) {
            assert_eq!(got, Some(p.accent_fg), "{scheme:?} {name}");
        }
        assert_eq!(link_trio(&theme), [p.link; 3], "{scheme:?}");
        assert_eq!(
            theme.link.label(WidgetState::DISABLED).color,
            c.link_disabled
        );
    }
}

/// The widgets the preset draws in a colour a slot replaces take the slot's
/// colour; the control fills, the hover layers, the strokes of their own and
/// the disabled colours keep the preset's.
#[test]
fn a_palette_recolours_what_it_states_and_nothing_else() {
    for scheme in [Scheme::Light, Scheme::Dark] {
        let p = distinct_palette(scheme);
        let ctx = ThemeContext::default().with_scheme(scheme);
        let preset = Theme::preset(Preset::Windows11).resolve(&ctx);
        let theme = Theme::preset(Preset::Windows11)
            .with_palette(p.clone())
            .resolve(&ctx);
        let normal = WindowState::default();
        let none = WidgetState::NONE;
        for (name, got, want) in [
            (
                "window frame",
                solid(&theme.window.frame(normal).fill),
                p.window_bg,
            ),
            (
                "title bar",
                solid(&theme.window.title_bar(normal).fill),
                p.window_bg,
            ),
            (
                "list's header",
                solid(&theme.list.header(ListQuery::new(none)).fill),
                p.window_bg,
            ),
            (
                "sidebar",
                solid(&theme.sidebar.container(none).fill),
                p.sidebar_bg,
            ),
            (
                "card",
                solid(&theme.card.container(none).fill),
                p.surface_bg,
            ),
            ("popover", solid(&theme.popover.panel.fill), p.surface_bg),
            ("dialog", solid(&theme.dialog.panel.fill), p.surface_bg),
            ("tooltip", solid(&theme.tooltip.panel.fill), p.tooltip_bg),
            (
                "card's outline",
                stroke_color(&theme.card.container(none).stroke),
                p.border,
            ),
            (
                "button's outline",
                stroke_color(&theme.button.body(ButtonQuery::new(none)).stroke),
                p.border,
            ),
            ("separator", Some(theme.separator.color), p.border),
            ("list's grid", Some(theme.list.grid_color), p.border),
        ] {
            assert_eq!(got, Some(want), "{scheme:?} {name}");
        }
        assert_eq!(theme.typography.body.color, p.foreground);
        assert_eq!(theme.tooltip.text.color, p.foreground);
        assert_eq!(theme.popover.text.color, p.foreground);
        assert_eq!(
            theme.button.label(ButtonQuery::new(none)).color,
            p.foreground
        );
        assert_eq!(theme.input.placeholder(none).color, p.foreground_muted);
        let unchanged = |t: &ResolvedTheme| {
            [
                solid(&t.button.body(ButtonQuery::new(none)).fill),
                solid(&t.button.body(ButtonQuery::new(WidgetState::HOVER)).fill),
                solid(&t.button.body(ButtonQuery::new(WidgetState::DISABLED)).fill),
                solid(&t.input.field(none).fill),
                solid(&t.checkbox.indicator(none).fill),
                stroke_color(&t.checkbox.indicator(none).stroke),
                solid(
                    &t.switch
                        .track(WidgetState::DISABLED | WidgetState::CHECKED)
                        .fill,
                ),
                solid(&t.slider.thumb(none).fill),
                solid(&t.slider.active_track(WidgetState::DISABLED).fill),
                solid(&t.menu.panel(none).fill),
                solid(&t.list.highlight(ListQuery::new(WidgetState::HOVER)).fill),
                solid(&t.toolbar.button(WidgetState::HOVER).fill),
                Some(
                    t.button
                        .label(ButtonQuery::new(WidgetState::DISABLED))
                        .color,
                ),
                Some(t.colors.focus_ring),
                Some(t.colors.disabled_text),
            ]
        };
        assert_eq!(unchanged(&theme), unchanged(&preset), "{scheme:?}");
        assert_eq!(
            solid(
                &theme
                    .switch
                    .track(WidgetState::HOVER | WidgetState::CHECKED)
                    .fill
            ),
            Some(p.accent.mul_alpha(ACCENT_FILL_SECONDARY_OPACITY)),
            "{scheme:?}"
        );
    }
}

/// An explicit accent over a palette replaces the palette's accent, its
/// text, its informational colour and its selection, whose text is then the
/// preset's; every other slot of the palette stands, its link with it.
#[test]
fn an_explicit_accent_replaces_the_palette_s_accent_slots() {
    let accent = hex(PURPLE);
    for scheme in [Scheme::Light, Scheme::Dark] {
        let p = distinct_palette(scheme);
        let ctx = ThemeContext::default().with_scheme(scheme);
        let theme = Theme::preset(Preset::Windows11)
            .with_palette(p.clone())
            .with_accent(accent)
            .resolve(&ctx);
        let palette_only = Theme::preset(Preset::Windows11)
            .with_palette(p.clone())
            .resolve(&ctx);
        let c = constants(scheme);
        let colors = &theme.colors;
        assert_eq!(colors.accent.fill, accent);
        assert_eq!(colors.accent.foreground, text_on_accent(accent, scheme));
        assert_eq!(
            colors.selection,
            SelectionColors::new(
                accent,
                c.selection_foreground,
                Some(accent),
                Some(c.selection_foreground)
            )
        );
        assert_eq!(colors.info.fill, accent);
        assert_eq!(colors.info.origin, RoleOrigin::Custom);
        for (name, got) in accent_fills(&theme) {
            assert_eq!(got, Some(accent), "{scheme:?} {name}");
        }
        assert_eq!(colors.window, palette_only.colors.window);
        assert_eq!(colors.card, palette_only.colors.card);
        assert_eq!(colors.tooltip, palette_only.colors.tooltip);
        assert_eq!(colors.error, palette_only.colors.error);
        assert_eq!(colors.caret, p.caret);
        assert_eq!(link_trio(&theme), [p.link; 3], "{scheme:?}");
    }
}

/// An application with shades of its own states them as the system's accent
/// and ramp and gives no explicit accent: the accent's fill is the scheme's
/// shade and the link's colours the text shades (spec §8, "The Windows
/// part").
#[test]
fn an_application_s_own_shades_run_the_system_s_rule() {
    use crate::next::reader::WindowsAccentRamp;
    let accent = hex(PURPLE);
    let [light1, light2, light3] = [hex(0x8a5cc0), hex(0xa880d8), hex(0xc8a8f0)];
    let [dark1, dark2, dark3] = [hex(0x4a2078), hex(0x3a1860), hex(0x2a1048)];
    let system = SystemOverrides {
        accent: Some(accent),
        windows_accent_ramp: Some(WindowsAccentRamp {
            light: [Some(light1), Some(light2), Some(light3)],
            dark: [Some(dark1), Some(dark2), Some(dark3)],
        }),
        ..SystemOverrides::default()
    };
    for (scheme, fill, info, link) in [
        (Scheme::Light, dark1, accent, [dark2, dark3, dark1]),
        (Scheme::Dark, light2, light2, [light3, light3, light2]),
    ] {
        let theme = Theme::preset(Preset::Windows11)
            .with_system_overrides(system.clone())
            .resolve(&ThemeContext::default().with_scheme(scheme));
        assert_eq!(theme.colors.accent.fill, accent, "{scheme:?}");
        assert_eq!(theme.colors.info.fill, info, "{scheme:?}");
        for (name, got) in accent_fills(&theme) {
            assert_eq!(got, Some(fill), "{scheme:?} {name}");
        }
        assert_eq!(link_trio(&theme), link, "{scheme:?}");
    }
}
