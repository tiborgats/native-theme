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
/// outline with it, is the ring's colour at alpha 0.8, and the disabled
/// opacity, the disabled switch label's too, is 0.4; nothing else changes.
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
        assert_eq!(normal.colors.disabled_opacity, DISABLED_OPACITY);
        assert_eq!(high.colors.disabled_opacity, DISABLED_OPACITY_HIGH);
        let rest = high.switch.label(WidgetState::NONE).color;
        assert_eq!(
            high.switch.label(WidgetState::DISABLED).color,
            rest.mul_alpha(DISABLED_OPACITY_HIGH)
        );
        let opacities = |t: &ResolvedTheme| {
            [
                t.button.disabled_opacity,
                t.checkbox.disabled_opacity,
                t.radio.disabled_opacity,
                t.input.disabled_opacity,
                t.text_area.disabled_opacity,
                t.slider.disabled_opacity,
                t.switch.disabled_opacity,
                t.combo_box.disabled_opacity,
                t.segmented_control.disabled_opacity,
            ]
        };
        assert_eq!(opacities(&high), [Some(DISABLED_OPACITY_HIGH); 9]);
        let mut same = high.clone();
        same.colors.disabled_opacity = normal.colors.disabled_opacity;
        same.button.disabled_opacity = normal.button.disabled_opacity;
        same.checkbox.disabled_opacity = normal.checkbox.disabled_opacity;
        same.radio.disabled_opacity = normal.radio.disabled_opacity;
        same.slider.disabled_opacity = normal.slider.disabled_opacity;
        same.switch = normal.switch.clone();
        same.combo_box.disabled_opacity = normal.combo_box.disabled_opacity;
        same.segmented_control.disabled_opacity = normal.segmented_control.disabled_opacity;
        same.colors.focus_ring = normal.colors.focus_ring;
        same.focus_ring = normal.focus_ring.clone();
        // The parts' own rings take the ring's colour too
        // (`a_part_with_its_own_ring_offset_states_its_ring`); in every state their rules
        // name, the parts are otherwise the same.
        let unringed = |mut part: PartStyle| {
            part.focus = None;
            part
        };
        let states = [
            WidgetState::NONE,
            WidgetState::HOVER,
            WidgetState::PRESSED,
            WidgetState::CHECKED,
            WidgetState::SELECTED,
            WidgetState::DISABLED,
            WidgetState::FOCUSED,
            WidgetState::HOVER | WidgetState::CHECKED,
            WidgetState::PRESSED | WidgetState::CHECKED,
            WidgetState::DISABLED | WidgetState::CHECKED,
        ];
        for state in states {
            for query in [ButtonQuery::new(state), ButtonQuery::primary().state(state)] {
                assert_eq!(
                    unringed(high.button.body(query)),
                    unringed(normal.button.body(query)),
                    "{state:?}"
                );
            }
            let pairs = [
                (high.slider.thumb(state), normal.slider.thumb(state)),
                (
                    high.segmented_control.segment(state),
                    normal.segmented_control.segment(state),
                ),
                (high.expander.header(state), normal.expander.header(state)),
            ];
            for (high_part, normal_part) in pairs {
                assert_eq!(unringed(high_part), unringed(normal_part), "{state:?}");
            }
        }
        same.button.body = normal.button.body.clone();
        same.slider.thumb = normal.slider.thumb.clone();
        same.segmented_control.segment = normal.segmented_control.segment.clone();
        same.expander.header = normal.expander.header.clone();
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

/// A checked indicator under the pointer keeps its checked outline, none,
/// not the unchecked one's hover outline; its fill is the checked fill under
/// libadwaita's hover layer
/// (`a_hovered_or_pressed_checked_control_lays_libadwaita_s_layer_over_the_accent`).
#[test]
fn a_checked_indicator_keeps_its_outline_under_the_pointer() {
    for theme in both_schemes() {
        let checked = WidgetState::CHECKED;
        let hovered = checked | WidgetState::HOVER;
        assert_ne!(
            theme.checkbox.indicator(WidgetState::HOVER),
            theme.checkbox.indicator(checked)
        );
        assert_eq!(
            theme.checkbox.indicator(hovered).stroke,
            theme.checkbox.indicator(checked).stroke
        );
        assert_eq!(
            theme.radio.indicator(hovered).stroke,
            theme.radio.indicator(checked).stroke
        );
        assert_ne!(
            theme.checkbox.indicator(hovered).fill,
            theme.checkbox.indicator(checked).fill
        );
    }
}

/// A mixed box under the pointer or pressed is drawn as a checked one: the
/// accent under the checked box's layer with no outline, not the unchecked
/// box's hover outline or pressed trough. No exclusive group clears `HOVER`
/// or `PRESSED` from a mixed box's state; the mixed rules, last, win, as libadwaita 1.10.0 keeps
/// `:indeterminate` out of the unchecked hover and active rules
/// (`&:not(:checked):not(:indeterminate)`, _checks.scss:33-50) and draws it
/// with `:checked` (`&:checked, &:indeterminate`, :52-60).
#[test]
fn a_mixed_box_under_the_pointer_or_pressed_is_drawn_as_a_checked_one() {
    for theme in both_schemes() {
        for state in [
            WidgetState::HOVER,
            WidgetState::PRESSED,
            WidgetState::HOVER | WidgetState::PRESSED,
        ] {
            let mixed = theme.checkbox.indicator(WidgetState::INDETERMINATE | state);
            let checked = theme.checkbox.indicator(WidgetState::CHECKED | state);
            assert_eq!(mixed.stroke, Stroke::None, "{state:?}");
            assert_eq!(mixed.fill, checked.fill, "{state:?}");
            assert_eq!(mixed.stroke, checked.stroke, "{state:?}");
        }
    }
}

/// An on switch, and a checked or mixed check or radio, hovered and pressed
/// take libadwaita's layers over the accent (_checks.scss:52-59,
/// _switch.scss:41-46 at libadwaita 1.10.0): currentColor, the accent's
/// text, at 10 %, and RGB(0 0 6 / 20%), each laid over the accent's fill.
#[test]
fn a_hovered_or_pressed_checked_control_lays_libadwaita_s_layer_over_the_accent() {
    for theme in both_schemes() {
        let accent = theme.colors.accent;
        let hover = accent
            .foreground
            .mul_alpha(CHECKED_HOVER_LAYER_ALPHA)
            .composite_over(accent.fill);
        let pressed = CHECKED_PRESSED_LAYER.composite_over(accent.fill);
        for (state, want) in [
            (WidgetState::HOVER, hover),
            (WidgetState::PRESSED, pressed),
            (WidgetState::HOVER | WidgetState::PRESSED, pressed),
        ] {
            let on = WidgetState::CHECKED | state;
            let mixed = WidgetState::INDETERMINATE | state;
            for (name, fill) in [
                ("switch", theme.switch.track(on).fill),
                ("check", theme.checkbox.indicator(on).fill),
                ("mixed check", theme.checkbox.indicator(mixed).fill),
                ("radio", theme.radio.indicator(on).fill),
            ] {
                assert_eq!(fill, Paint::Solid(want), "{name} {state:?}");
            }
        }
        assert_ne!(
            theme
                .switch
                .track(WidgetState::HOVER | WidgetState::CHECKED),
            theme.switch.track(WidgetState::HOVER)
        );
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

/// The line between toggles is GtkSeparator's `$border_color`, the
/// splitter's line, 1 px wide (_misc.scss:21-24 at libadwaita 1.10.0).
#[test]
fn the_segments_are_parted_by_the_separator_s_line() {
    for theme in both_schemes() {
        let line = theme.splitter.handle(WidgetState::NONE).fill;
        assert_ne!(line, Paint::None);
        assert_eq!(
            theme.segmented_control.separator(WidgetState::NONE).fill,
            line
        );
        assert_eq!(theme.segmented_control.separator_width, 1.0);
    }
}

/// The flat button: at rest no fill and no shadow; hovered, pressed and
/// checked the window text at libadwaita's 7, 16 and 10 %, the toolbar's
/// buttons' fills; checked and hovered or pressed at 13 and 19 %; each
/// stated for its state.
#[test]
fn a_flat_button_lays_the_window_text_s_layers() {
    use crate::next::style::PartStated;
    for (theme, c) in both_schemes()
        .iter()
        .zip([constants(Scheme::Light), constants(Scheme::Dark)])
    {
        let flat = |state| theme.button.body(ButtonQuery::flat().state(state));
        let rest = flat(WidgetState::NONE);
        assert_eq!(rest.fill, Paint::None);
        assert_eq!(rest.shadows, None);
        assert!(rest.stated.contains(PartStated::FILL));
        let text = c.row_foreground;
        for (state, alpha) in [
            (WidgetState::HOVER, TOOLBAR_BUTTON_HOVER_ALPHA),
            (WidgetState::PRESSED, TOOLBAR_BUTTON_PRESSED_ALPHA),
            (WidgetState::CHECKED, TOOLBAR_BUTTON_CHECKED_ALPHA),
            (
                WidgetState::CHECKED | WidgetState::HOVER,
                FLAT_CHECKED_HOVER_ALPHA,
            ),
            (
                WidgetState::CHECKED | WidgetState::PRESSED,
                FLAT_CHECKED_PRESSED_ALPHA,
            ),
        ] {
            let body = flat(state);
            assert_eq!(
                solid(&body.fill).map(Color::to_srgb8),
                Some(text.mul_alpha(alpha).to_srgb8()),
                "{state:?}"
            );
            assert!(body.from_state.contains(PartStated::FILL), "{state:?}");
        }
        for state in [
            WidgetState::HOVER,
            WidgetState::PRESSED,
            WidgetState::CHECKED,
        ] {
            assert_eq!(
                flat(state).fill,
                theme.toolbar.button(state).fill,
                "{state:?}"
            );
        }
    }
}

/// A checked push button keeps its checked look under the pointer and
/// pressed: the window text at libadwaita's 35 and 40 %
/// (`$button_checked_hover_color`, `$button_checked_active_color`), not the
/// unchecked hover and pressed fills, each stated for its state.
#[test]
fn a_checked_push_button_hovered_or_pressed_keeps_its_checked_look() {
    use crate::next::style::PartStated;
    for (theme, c) in both_schemes()
        .iter()
        .zip([constants(Scheme::Light), constants(Scheme::Dark)])
    {
        let body = |state| theme.button.body(ButtonQuery::new(state));
        for (pointer, alpha) in [
            (WidgetState::HOVER, BUTTON_CHECKED_HOVER_ALPHA),
            (WidgetState::PRESSED, BUTTON_CHECKED_PRESSED_ALPHA),
        ] {
            let checked = body(WidgetState::CHECKED | pointer);
            assert_eq!(
                checked.fill,
                Paint::Solid(c.row_foreground.mul_alpha(alpha)),
                "{pointer:?}"
            );
            assert_ne!(checked.fill, body(pointer).fill, "{pointer:?}");
            assert!(checked.from_state.contains(PartStated::FILL), "{pointer:?}");
        }
    }
}

/// A push button's six fills are libadwaita's table, each the window text at
/// its `color-mix(in srgb, currentColor N%, transparent)`, translucent:
/// `$button_color` 10, `$button_hover_color` 15, `$button_active_color` 30,
/// `$button_checked_color` 30, `$button_checked_hover_color` 35,
/// `$button_checked_active_color` 40 (_buttons.scss:1-6 at libadwaita 1.10.0),
/// each stated for its state.
#[test]
fn a_push_button_s_fills_are_libadwaita_s_table() {
    use crate::next::style::PartStated;
    for (theme, c) in both_schemes()
        .iter()
        .zip([constants(Scheme::Light), constants(Scheme::Dark)])
    {
        for (state, percent) in [
            (WidgetState::NONE, 10.0),
            (WidgetState::HOVER, 15.0),
            (WidgetState::PRESSED, 30.0),
            (WidgetState::CHECKED, 30.0),
            (WidgetState::CHECKED | WidgetState::HOVER, 35.0),
            (WidgetState::CHECKED | WidgetState::PRESSED, 40.0),
        ] {
            let body = theme.button.body(ButtonQuery::new(state));
            assert_eq!(
                body.fill,
                Paint::Solid(c.row_foreground.mul_alpha(percent / 100.0)),
                "{state:?}"
            );
            assert!(body.stated.contains(PartStated::FILL), "{state:?}");
            assert_eq!(
                body.from_state.contains(PartStated::FILL),
                state != WidgetState::NONE,
                "{state:?}"
            );
        }
    }
}

/// The drop-down's trigger, a raised button, takes the push button's table
/// (`$button_color` 10, `$button_hover_color` 15, `$button_active_color` 30;
/// _buttons.scss:1-3, :28-42, _dropdowns.scss:1-12 at libadwaita 1.10.0);
/// the entry and the toggle group's track fill with `$button_color`
/// (_entries.scss:9, _toggle-group.scss:12); a hovered toggle is
/// `$hover_color`, the window text at 7 % (_toggle-group.scss:65-67,
/// _colors.scss:268): each the window text at its alpha, translucent. A
/// checked toggle is `--active-toggle-bg-color`, `#ffffff` in light and
/// `rgb(255 255 255 / 20%)` in dark, a layer over the track
/// (_toggle-group.scss:79-80, _colors.scss:238, :245-247; platform-facts.md
/// (§2.25)). The 8-bit values are those `next_homes.toml` lists for 0.8.0.
#[test]
fn the_drop_down_entry_and_toggle_group_take_libadwaita_s_fills() {
    use crate::next::style::PartStated;
    // Rest ($button_color), hovered drop-down ($button_hover_color), hovered
    // toggle ($hover_color), checked toggle.
    let literals = [
        [
            [0x00, 0x00, 0x06, 0x14],
            [0x00, 0x00, 0x06, 0x1f],
            [0x00, 0x00, 0x06, 0x0e],
            [0xff, 0xff, 0xff, 0xff],
        ],
        [
            [0xff, 0xff, 0xff, 0x1a],
            [0xff, 0xff, 0xff, 0x26],
            [0xff, 0xff, 0xff, 0x12],
            [0xff, 0xff, 0xff, 0x33],
        ],
    ];
    for ((theme, c), [rest, hover, toggle_hover, checked]) in both_schemes()
        .iter()
        .zip([constants(Scheme::Light), constants(Scheme::Dark)])
        .zip(literals)
    {
        let layer = |alpha: f32| Paint::Solid(c.row_foreground.mul_alpha(alpha));
        for (state, alpha) in [
            (WidgetState::NONE, BUTTON_ALPHA),
            (WidgetState::HOVER, BUTTON_HOVER_ALPHA),
            (WidgetState::PRESSED, BUTTON_PRESSED_ALPHA),
        ] {
            let field = theme.combo_box.field(state);
            assert_eq!(field.fill, layer(alpha), "drop-down {state:?}");
            assert!(field.stated.contains(PartStated::FILL), "{state:?}");
        }
        let eight_bit = |paint: &Paint| solid(paint).map(Color::to_srgb8);
        for (name, paint) in [
            ("drop-down", &theme.combo_box.field(WidgetState::NONE).fill),
            ("entry", &theme.input.field(WidgetState::NONE).fill),
            ("text area", &theme.text_area.field(WidgetState::NONE).fill),
            (
                "toggle group",
                &theme.segmented_control.track(WidgetState::NONE).fill,
            ),
        ] {
            assert_eq!(*paint, layer(BUTTON_ALPHA), "{name}");
            assert_eq!(eight_bit(paint), Some(rest), "{name}");
        }
        assert_eq!(
            eight_bit(&theme.combo_box.field(WidgetState::HOVER).fill),
            Some(hover)
        );
        let toggle = |state| theme.segmented_control.segment(state).fill;
        assert_eq!(
            toggle(WidgetState::HOVER),
            Paint::Solid(layer_of(c.row_foreground, TOOLBAR_BUTTON_HOVER_ALPHA))
        );
        assert_eq!(eight_bit(&toggle(WidgetState::HOVER)), Some(toggle_hover));
        assert_eq!(
            toggle(WidgetState::PRESSED),
            Paint::Solid(layer_of(c.row_foreground, TOOLBAR_BUTTON_PRESSED_ALPHA))
        );
        assert_eq!(
            eight_bit(&toggle(WidgetState::PRESSED)),
            Some(c.active_layer.to_srgb8())
        );
        assert_eq!(eight_bit(&toggle(WidgetState::SELECTED)), Some(checked));
    }
}

/// A disabled check, radio and off switch state their rest fill for
/// `DISABLED`, the fade alone changing them; checked, each keeps its checked
/// fill, the accent.
#[test]
fn a_disabled_control_states_its_rest_fill_and_a_checked_one_keeps_the_accent() {
    use crate::next::style::PartStated;
    for theme in both_schemes() {
        let disabled = WidgetState::DISABLED;
        let checked = WidgetState::CHECKED;
        let parts: [(&str, &dyn Fn(WidgetState) -> PartStyle); 3] = [
            ("check", &|s| theme.checkbox.indicator(s)),
            ("radio", &|s| theme.radio.indicator(s)),
            ("switch", &|s| theme.switch.track(s)),
        ];
        for (name, part) in parts {
            let off = part(disabled);
            assert_eq!(off.fill, part(WidgetState::NONE).fill, "{name}");
            assert!(off.from_state.contains(PartStated::FILL), "{name}");
            assert_eq!(
                part(disabled | checked).fill,
                Paint::Solid(theme.colors.accent.fill),
                "{name}"
            );
            assert_eq!(part(disabled | checked).fill, part(checked).fill, "{name}");
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

/// The toolbar button's hovered, pressed and checked layers, derived from
/// the window text, are libadwaita's in 8 bits: platform-facts.md (§2.13),
/// #0000060e, #00000621, #00000614 light and #ffffff12, #ffffff29, #ffffff1a
/// dark.
#[test]
fn the_toolbar_button_layers_are_the_platform_facts_values() {
    for (theme, want) in both_schemes().iter().zip([
        [
            [0x00, 0x00, 0x06, 0x0e],
            [0x00, 0x00, 0x06, 0x21],
            [0x00, 0x00, 0x06, 0x14],
        ],
        [
            [0xff, 0xff, 0xff, 0x12],
            [0xff, 0xff, 0xff, 0x29],
            [0xff, 0xff, 0xff, 0x1a],
        ],
    ]) {
        let got = [
            WidgetState::HOVER,
            WidgetState::PRESSED,
            WidgetState::CHECKED,
        ]
        .map(|state| solid(&theme.toolbar.button(state).fill).map(Color::to_srgb8));
        assert_eq!(got, want.map(Some), "{:?}", theme.context.scheme);
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

// ---------------------------------------------------------------------------
// The palette and the explicit accent (spec §6, resolution step 2; §7)
// ---------------------------------------------------------------------------

/// libadwaita's blue and purple accents (libadwaita 1.10.0
/// `doc/css-variables.md`, the table `src/gnome/accent.rs` tests against).
const BLUE: Color = srgb8(0x3584_e4ff);
const PURPLE: Color = srgb8(0x9141_acff);

/// The focus ring of [`PURPLE`]: its documented standalone colour on a light
/// and on a dark background, at 50 % (`src/gnome/accent.rs`, `DOCUMENTED`).
const PURPLE_RING_LIGHT: Color = srgb8(0x8939_a480);
const PURPLE_RING_DARK: Color = srgb8(0xfba7_ff80);

/// The 8-bit opaque colour `0xRRGGBB`.
fn hex(rgb: u32) -> Color {
    let [_, r, g, b] = rgb.to_be_bytes();
    Color::from_srgb8(r, g, b, u8::MAX)
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

/// The preset's constants of `scheme`.
fn constants(scheme: Scheme) -> SchemeColors {
    match scheme {
        Scheme::Light => LIGHT,
        Scheme::Dark => DARK,
    }
}

/// The focus ring of `accent` for `scheme`: its standalone colour at 50 %.
fn ring_of(accent: Color, scheme: Scheme) -> Color {
    let mut c = constants(scheme);
    lay_accent_over(&mut c, accent, scheme);
    c.focus_ring
}

/// The colour of a solid paint.
fn solid(paint: &Paint) -> Option<Color> {
    match paint {
        Paint::Solid(color) => Some(*color),
        _ => None,
    }
}

/// A palette states the roles its slots name (spec §7); the focus ring is
/// derived again from its accent, and the disabled text follows its text.
#[test]
fn a_palette_states_its_roles() {
    for scheme in [Scheme::Light, Scheme::Dark] {
        let p = distinct_palette(scheme);
        let theme = Theme::preset(Preset::Adwaita)
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
        assert_eq!(colors.focus_ring, ring_of(p.accent, scheme));
        assert_ne!(colors.focus_ring, constants(scheme).focus_ring);
        assert_eq!(colors.link, p.link);
        assert_eq!(colors.caret, p.caret);
        assert_eq!(colors.disabled_text, p.foreground);
        let c = constants(scheme);
        for (status, fill, foreground) in [
            (colors.error, p.error, c.error_foreground),
            (colors.warning, p.warning, c.warning_foreground),
            (colors.success, p.success, c.success_foreground),
            (colors.info, p.info, c.accent_foreground),
        ] {
            assert_eq!(status.fill, fill, "{scheme:?}");
            assert_eq!(status.foreground, foreground, "{scheme:?}");
            assert_eq!(status.origin, RoleOrigin::Custom, "{scheme:?}");
        }
        assert_eq!(colors.shadow, c.shadow);
        assert_eq!(theme.name.as_ref(), "distinct");
    }
}

/// The widgets the preset draws in a colour a slot replaces take the slot's
/// colour, the push button's, the entry's, the drop-down's and the toggle
/// group's fills and the toolbar's, the flat button's and the hovered and
/// pressed toggle's layers of the window text the palette's foreground at
/// libadwaita's alphas; the fills the preset states as libadwaita's layers
/// over its own window, the placeholder and the switch's hover colour keep
/// the preset's.
#[test]
fn a_palette_recolours_the_widgets_of_its_colours() {
    for scheme in [Scheme::Light, Scheme::Dark] {
        let p = distinct_palette(scheme);
        let ctx = ThemeContext::default().with_scheme(scheme);
        let theme = Theme::preset(Preset::Adwaita)
            .with_palette(p.clone())
            .resolve(&ctx);
        let inactive = Theme::preset(Preset::Adwaita)
            .with_palette(p.clone())
            .resolve(&ctx.clone().with_window_active(false));
        let normal = WindowState::default();
        let none = WidgetState::NONE;
        for (name, got, want) in [
            (
                "window frame",
                solid(&theme.window.frame(normal).fill),
                p.window_bg,
            ),
            (
                "active title bar",
                solid(&theme.window.title_bar(normal).fill),
                p.header_bg,
            ),
            (
                "inactive title bar",
                solid(&inactive.window.title_bar(normal).fill),
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
                "selected tab's indicator",
                solid(&theme.tab.indicator(WidgetState::SELECTED).fill),
                p.accent,
            ),
            (
                "selected list row",
                solid(
                    &theme
                        .list
                        .highlight(ListQuery::new(WidgetState::SELECTED))
                        .fill,
                ),
                p.selection_bg,
            ),
        ] {
            assert_eq!(got, Some(want), "{scheme:?} {name}");
        }
        assert_eq!(theme.typography.body.color, p.foreground);
        assert_eq!(theme.tooltip.text.color, p.foreground);
        assert_eq!(
            theme.button.label(ButtonQuery::new(none)).color,
            p.foreground
        );
        assert_eq!(
            theme.button.label(ButtonQuery::primary()).color,
            p.accent_fg
        );
        assert_eq!(
            theme.input.field(WidgetState::FOCUSED).stroke,
            Stroke::uniform(INPUT_BORDER_WIDTH, ring_of(p.accent, scheme))
        );
        for (state, alpha) in [
            (WidgetState::HOVER, TOOLBAR_BUTTON_HOVER_ALPHA),
            (WidgetState::PRESSED, TOOLBAR_BUTTON_PRESSED_ALPHA),
            (WidgetState::CHECKED, TOOLBAR_BUTTON_CHECKED_ALPHA),
            (
                WidgetState::CHECKED | WidgetState::HOVER,
                FLAT_CHECKED_HOVER_ALPHA,
            ),
            (
                WidgetState::CHECKED | WidgetState::PRESSED,
                FLAT_CHECKED_PRESSED_ALPHA,
            ),
        ] {
            let layer = Some(p.foreground.mul_alpha(alpha).to_srgb8());
            assert_eq!(
                solid(&theme.button.body(ButtonQuery::flat().state(state)).fill)
                    .map(Color::to_srgb8),
                layer,
                "{scheme:?} flat {state:?}"
            );
            if !state.contains(WidgetState::CHECKED) || state == WidgetState::CHECKED {
                assert_eq!(
                    solid(&theme.toolbar.button(state).fill).map(Color::to_srgb8),
                    layer,
                    "{scheme:?} toolbar {state:?}"
                );
            }
        }
        let c = constants(scheme);
        assert_eq!(
            solid(&theme.button.body(ButtonQuery::new(none)).fill),
            Some(p.foreground.mul_alpha(BUTTON_ALPHA))
        );
        for (name, got) in [
            ("entry", solid(&theme.input.field(none).fill)),
            ("drop-down", solid(&theme.combo_box.field(none).fill)),
            (
                "toggle group",
                solid(&theme.segmented_control.track(none).fill),
            ),
        ] {
            assert_eq!(
                got,
                Some(p.foreground.mul_alpha(BUTTON_ALPHA)),
                "{scheme:?} {name}"
            );
        }
        assert_eq!(
            solid(&theme.segmented_control.segment(WidgetState::HOVER).fill).map(Color::to_srgb8),
            Some(
                p.foreground
                    .mul_alpha(TOOLBAR_BUTTON_HOVER_ALPHA)
                    .to_srgb8()
            ),
            "{scheme:?} hovered toggle"
        );
        assert_eq!(
            solid(&theme.segmented_control.segment(WidgetState::PRESSED).fill).map(Color::to_srgb8),
            Some(
                p.foreground
                    .mul_alpha(TOOLBAR_BUTTON_PRESSED_ALPHA)
                    .to_srgb8()
            ),
            "{scheme:?} pressed toggle"
        );
        assert_eq!(theme.input.placeholder(none).color, c.placeholder);
        assert_eq!(
            solid(
                &theme
                    .switch
                    .track(WidgetState::HOVER | WidgetState::CHECKED)
                    .fill
            ),
            Some(
                p.accent_fg
                    .mul_alpha(CHECKED_HOVER_LAYER_ALPHA)
                    .composite_over(p.accent)
            )
        );
    }
}

/// An explicit accent is libadwaita's accent as the system's is: the
/// accent's fill, the selection, the info status and the tab's indicator,
/// and the focus ring its standalone colour through `adw_standalone`; it
/// stands over the system's accent.
#[test]
fn an_explicit_accent_is_libadwaita_s_accent() {
    for (scheme, ring) in [
        (Scheme::Light, PURPLE_RING_LIGHT),
        (Scheme::Dark, PURPLE_RING_DARK),
    ] {
        let ctx = ThemeContext::default().with_scheme(scheme);
        let explicit = Theme::preset(Preset::Adwaita)
            .with_accent(PURPLE)
            .resolve(&ctx);
        assert_eq!(explicit.colors.accent.fill, PURPLE);
        assert_eq!(explicit.colors.selection.background, PURPLE);
        assert_eq!(explicit.colors.info.fill, PURPLE);
        assert_eq!(explicit.colors.focus_ring, ring);
        assert_eq!(
            solid(&explicit.tab.indicator(WidgetState::SELECTED).fill),
            Some(PURPLE)
        );
        let system = |accent| SystemOverrides {
            accent: Some(accent),
            ..SystemOverrides::default()
        };
        let by_system = Theme::preset(Preset::Adwaita)
            .with_system_overrides(system(PURPLE))
            .resolve(&ctx);
        assert_eq!(explicit, by_system, "{scheme:?}");
        let over_system = Theme::preset(Preset::Adwaita)
            .with_system_overrides(system(BLUE))
            .with_accent(PURPLE)
            .resolve(&ctx);
        assert_eq!(over_system, explicit, "{scheme:?}");
    }
}

/// An explicit accent over a palette replaces the palette's accent, its
/// selection and its info colour, and with the accent its text, which are
/// then the preset's; every other slot of the palette stands.
#[test]
fn an_explicit_accent_replaces_the_palette_s_accent_slots() {
    for scheme in [Scheme::Light, Scheme::Dark] {
        let p = distinct_palette(scheme);
        let ctx = ThemeContext::default().with_scheme(scheme);
        let theme = Theme::preset(Preset::Adwaita)
            .with_palette(p.clone())
            .with_accent(PURPLE)
            .resolve(&ctx);
        let palette_only = Theme::preset(Preset::Adwaita)
            .with_palette(p.clone())
            .resolve(&ctx);
        let c = constants(scheme);
        let colors = &theme.colors;
        assert_eq!(colors.accent.fill, PURPLE);
        assert_eq!(colors.accent.foreground, c.accent_foreground);
        assert_eq!(
            colors.selection,
            SelectionColors::new(
                PURPLE,
                c.accent_foreground,
                Some(PURPLE),
                Some(c.accent_foreground)
            )
        );
        assert_eq!(colors.info.fill, PURPLE);
        assert_eq!(colors.focus_ring, ring_of(PURPLE, scheme));
        assert_eq!(colors.window, palette_only.colors.window);
        assert_eq!(colors.card, palette_only.colors.card);
        assert_eq!(colors.link, p.link);
        assert_eq!(colors.caret, p.caret);
        assert_eq!(colors.error, palette_only.colors.error);
        assert_eq!(colors.info.origin, RoleOrigin::Custom);
    }
}

/// The parts whose libadwaita `focus-ring` takes an offset of its own state
/// their ring there, in the global ring's width and colour: the primary
/// button and the switch 1 px outside, the slider thumb and a segment at
/// their edge, the expander header 1 px inside (_buttons.scss:158,
/// _switch.scss:18, _scale.scss:29, _toggle-group.scss:63, _lists.scss:344 at
/// libadwaita 1.10.0); a standard button and the check box keep the global
/// ring, 2 px inside.
#[test]
fn a_part_with_its_own_ring_offset_states_its_ring() -> TestResult {
    for ctx in [ThemeContext::light(), ThemeContext::dark()] {
        for contrast in [Contrast::Normal, Contrast::High] {
            let theme =
                Theme::preset(Preset::Adwaita).resolve(&ctx.clone().with_contrast(contrast));
            let none = WidgetState::NONE;
            for (what, part, offset) in [
                (
                    "primary button",
                    theme.button.body(ButtonQuery::primary().state(none)),
                    1.0,
                ),
                ("switch track", theme.switch.track(none), 1.0),
                ("slider thumb", theme.slider.thumb(none), 0.0),
                ("segment", theme.segmented_control.segment(none), 0.0),
                ("expander header", theme.expander.header(none), -1.0),
            ] {
                let Some(ring) = part.focus.as_deref() else {
                    return Err(format!("{what} states no focus ring").into());
                };
                assert_eq!(
                    ring,
                    &FocusIndicator::new(
                        FocusKind::Ring,
                        theme.focus_ring.stroke.clone(),
                        Stroke::None,
                        offset,
                        None,
                        None,
                    ),
                    "{what} {contrast:?}"
                );
            }
            assert_eq!(theme.button.body(ButtonQuery::new(none)).focus, None);
            assert_eq!(theme.checkbox.indicator(none).focus, None);
            assert_eq!(theme.focus_ring.offset, FOCUS_RING_OFFSET);
        }
    }
    Ok(())
}
