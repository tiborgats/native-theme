use super::*;

use crate::next::motion::DialogEntranceStyle;
use crate::next::preset::Theme;
use crate::next::query::{ButtonQuery, ListQuery, MotionPreference};
use crate::next::style::PartStated;
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
        // Pressed and hovered, the body is the pressed one: hover draws nothing on a push button.
        let pressed_hover = theme
            .button
            .body(ButtonQuery::new(WidgetState::PRESSED | WidgetState::HOVER));
        let pressed = theme.button.body(ButtonQuery::new(WidgetState::PRESSED));
        assert_eq!(
            (pressed_hover.fill, pressed_hover.stroke),
            (pressed.fill, pressed.stroke),
            "{preset:?}"
        );
        let hovered = theme.button.body(ButtonQuery::new(WidgetState::HOVER));
        let rest = theme.button.body(ButtonQuery::new(WidgetState::NONE));
        assert_eq!(
            (hovered.fill, hovered.stroke),
            (rest.fill, rest.stroke),
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
        // The same look; the on thumb's size is the `CHECKED` rule's, so its bits differ.
        let look = |state: WidgetState| PartStyle {
            from_state: PartStated::default(),
            ..theme.switch.thumb(state)
        };
        assert_eq!(look(on_disabled), look(off_disabled), "{preset:?}");
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

// ---------------------------------------------------------------------------
// The palette and the explicit accent
// ---------------------------------------------------------------------------

/// The 8-bit opaque colour `0xRRGGBB`.
fn hex(rgb: u32) -> Color {
    let [_, r, g, b] = rgb.to_be_bytes();
    Color::from_srgb8(r, g, b, u8::MAX)
}

/// The red accent's `controlAccentColor` in light (`accent-decision.md`,
/// F8; the OS probe's `accent-red` reads).
const RED: u32 = 0xe0383e;

/// Every preset with each of its schemes.
fn every_scheme() -> [(Preset, Scheme); 4] {
    [
        (Preset::MacosMonterey, Scheme::Light),
        (Preset::MacosMonterey, Scheme::Dark),
        (Preset::MacosTahoe, Scheme::Light),
        (Preset::MacosTahoe, Scheme::Dark),
    ]
}

/// The constants of `preset`'s generation.
fn generation(preset: Preset) -> Generation {
    match preset {
        Preset::MacosTahoe => TAHOE,
        _ => MONTEREY,
    }
}

/// The constants of `preset` for `scheme`.
fn constants(preset: Preset, scheme: Scheme) -> SchemeColors {
    let g = generation(preset);
    match scheme {
        Scheme::Light => g.light,
        Scheme::Dark => g.dark,
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

/// `accent` at the focus ring's alpha.
fn ring(accent: Color) -> Color {
    accent.with_alpha(f32::from(FOCUS_RING_ALPHA) / f32::from(u8::MAX))
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

/// Every colour `preset`'s theme draws in the accent's fill, each by its
/// name: the checked fills, the slider's and the progress bar's fill, and
/// the selected segment where the scheme states no fill of its own for it.
fn accent_fills(
    preset: Preset,
    scheme: Scheme,
    theme: &ResolvedTheme,
) -> Vec<(&'static str, Option<Color>)> {
    let checked = WidgetState::CHECKED;
    let mut fills = vec![
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
        ("on switch", solid(&theme.switch.track(checked).fill)),
        (
            "slider's filled part",
            solid(&theme.slider.active_track(WidgetState::NONE).fill),
        ),
        (
            "progress",
            solid(&theme.progress_bar.fill(WidgetState::NONE).fill),
        ),
    ];
    if constants(preset, scheme).segmented_active.is_none() {
        fills.push((
            "selected segment",
            solid(&theme.segmented_control.segment(WidgetState::SELECTED).fill),
        ));
    }
    fills
}

/// Every text and mark `preset`'s theme draws on the accent's fill, each by
/// its name.
fn on_accent(
    preset: Preset,
    scheme: Scheme,
    theme: &ResolvedTheme,
) -> Vec<(&'static str, Option<Color>)> {
    let checked = WidgetState::CHECKED;
    let mut marks = vec![
        (
            "primary button's label",
            Some(theme.button.label(ButtonQuery::primary()).color),
        ),
        ("check mark", Some(theme.checkbox.checkmark(checked).color)),
        ("radio button's dot", solid(&theme.radio.dot(checked).fill)),
    ];
    if constants(preset, scheme)
        .segmented_active_foreground
        .is_none()
    {
        marks.push((
            "selected segment's label",
            Some(theme.segmented_control.label(WidgetState::SELECTED).color),
        ));
    }
    marks
}

/// The disabled on switch's track.
fn disabled_on_track(theme: &ResolvedTheme) -> Option<Color> {
    solid(
        &theme
            .switch
            .track(WidgetState::DISABLED | WidgetState::CHECKED)
            .fill,
    )
}

/// An explicit accent is written as given (`accent-decision.md`, M1-M6):
/// the accent's fill and every use of it, the disabled on switch's track at
/// the generation's alpha, the selection, the text selection with the
/// selection's text, the focus ring at alpha 0x7F and the caret; the link,
/// the status colours, the unfocused selection and the two texts keep the
/// preset's.
#[test]
fn an_explicit_accent_is_written_as_given() {
    let accent = hex(RED);
    for (preset, scheme) in every_scheme() {
        let ctx = ThemeContext::default().with_scheme(scheme);
        let plain = Theme::preset(preset).resolve(&ctx);
        let theme = Theme::preset(preset).with_accent(accent).resolve(&ctx);
        for (name, got) in accent_fills(preset, scheme, &theme) {
            assert_eq!(got, Some(accent), "{preset:?} {scheme:?} {name}");
        }
        let alpha = match preset {
            Preset::MacosTahoe => 0x73,
            _ => 0x80,
        };
        assert_eq!(
            disabled_on_track(&theme).map(Color::to_srgb8),
            Some([0xe0, 0x38, 0x3e, alpha]),
            "{preset:?} {scheme:?}"
        );
        assert_eq!(
            theme.colors.focus_ring.to_srgb8(),
            [0xe0, 0x38, 0x3e, 0x7f],
            "{preset:?} {scheme:?}"
        );
        assert_eq!(
            stroke_color(&theme.focus_ring.stroke),
            Some(theme.colors.focus_ring),
            "{preset:?} {scheme:?}"
        );
        let mut want = plain.colors.clone();
        want.accent.fill = accent;
        want.selection.background = accent;
        want.text_selection =
            SelectionColors::new(accent, plain.colors.selection.foreground, None, None);
        want.focus_ring = ring(accent);
        want.caret = accent;
        assert_eq!(theme.colors, want, "{preset:?} {scheme:?}");
        assert_eq!(
            solid(
                &theme
                    .list
                    .highlight(ListQuery::new(WidgetState::SELECTED))
                    .fill
            ),
            Some(accent),
            "{preset:?} {scheme:?}"
        );
        assert_eq!(
            theme.menu.item_label(WidgetState::HOVER).color,
            plain.menu.item_label(WidgetState::HOVER).color,
            "{preset:?} {scheme:?}"
        );
    }
}

/// The system's settings of `scheme` holding the default accent's reads of
/// macOS 14 (`accent-decision.md`, F8): the accent, the selection's
/// background, the text selection's background, the focus ring and the
/// caret; the two texts are the preset's.
fn blue_reads(preset: Preset, scheme: Scheme) -> (OsColors, [Color; 4]) {
    use crate::next::reader::OsSelectionColors;
    let c = constants(preset, scheme);
    let accent = hex(0x007aff);
    let (selection, text_selection, focus_ring) = match scheme {
        Scheme::Light => (
            hex(0x0064e1),
            hex(0xb3d7ff),
            Color::from_srgb8(0x00, 0x67, 0xf4, 0x7f),
        ),
        Scheme::Dark => (
            hex(0x0059d1),
            hex(0x3f638b),
            Color::from_srgb8(0x1a, 0xa9, 0xff, 0x7f),
        ),
    };
    let colors = OsColors {
        accent: Some(accent),
        selection: Some(OsSelectionColors::new(
            Some(selection),
            Some(c.selection_foreground),
            None,
            None,
        )),
        text_selection: Some(OsSelectionColors::new(
            Some(text_selection),
            Some(c.text_selection_foreground),
            None,
            None,
        )),
        focus_ring: Some(focus_ring),
        caret: Some(accent),
        ..OsColors::default()
    };
    (colors, [accent, selection, text_selection, focus_ring])
}

/// An explicit accent over a system snapshot holding the default accent's
/// reads replaces them (`accent-decision.md`, M7): the result is the one
/// without the snapshot, and none of the reads remains in the accent's
/// roles.
#[test]
fn an_explicit_accent_stands_over_the_system_s_accent_reads() {
    let accent = hex(RED);
    for (preset, scheme) in every_scheme() {
        let ctx = ThemeContext::default().with_scheme(scheme);
        let (colors, reads) = blue_reads(preset, scheme);
        let system = SystemOverrides {
            color_schemes: vec![((scheme, Contrast::Normal), colors)],
            ..SystemOverrides::default()
        };
        let read = Theme::preset(preset)
            .with_system_overrides(system.clone())
            .resolve(&ctx);
        assert_eq!(read.colors.accent.fill, reads[0], "{preset:?} {scheme:?}");
        assert_eq!(read.colors.focus_ring, reads[3], "{preset:?} {scheme:?}");
        let theme = Theme::preset(preset)
            .with_system_overrides(system)
            .with_accent(accent)
            .resolve(&ctx);
        let alone = Theme::preset(preset).with_accent(accent).resolve(&ctx);
        let accent_roles = |t: &ResolvedTheme| {
            [
                t.colors.accent.fill,
                t.colors.selection.background,
                t.colors.text_selection.background,
                t.colors.text_selection.foreground,
                t.colors.focus_ring,
                t.colors.caret,
            ]
        };
        assert_eq!(
            accent_roles(&theme),
            accent_roles(&alone),
            "{preset:?} {scheme:?}"
        );
        for got in accent_roles(&theme) {
            assert!(!reads.contains(&got), "{preset:?} {scheme:?} {got}");
        }
        assert_eq!(
            disabled_on_track(&theme),
            disabled_on_track(&alone),
            "{preset:?} {scheme:?}"
        );
        for (name, got) in accent_fills(preset, scheme, &theme) {
            assert_eq!(got, Some(accent), "{preset:?} {scheme:?} {name}");
        }
    }
}

/// A palette states the roles its slots name (spec §7), its accent written
/// as given: the focus ring is its accent at alpha 0x7F, the text selection
/// its selection, the caret its caret.
#[test]
fn a_palette_states_its_roles() {
    for (preset, scheme) in every_scheme() {
        let p = distinct_palette(scheme);
        let theme = Theme::preset(preset)
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
            assert_eq!(
                surface.background,
                Some(background),
                "{preset:?} {scheme:?} {name}"
            );
            assert_eq!(
                surface.foreground,
                Some(p.foreground),
                "{preset:?} {scheme:?} {name}"
            );
        }
        let c = constants(preset, scheme);
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
                Some(c.selection_inactive_background),
                Some(c.selection_inactive_foreground)
            )
        );
        assert_eq!(
            colors.text_selection,
            SelectionColors::new(p.selection_bg, p.selection_fg, None, None)
        );
        assert_eq!(colors.focus_ring, ring(p.accent), "{preset:?} {scheme:?}");
        assert_eq!(colors.caret, p.caret);
        assert_eq!(colors.link, p.link);
        for (status, fill) in [
            (colors.error, p.error),
            (colors.warning, p.warning),
            (colors.success, p.success),
            (colors.info, p.info),
        ] {
            assert_eq!(status.fill, fill, "{preset:?} {scheme:?}");
            assert_eq!(status.foreground, c.status_foreground);
            assert_eq!(status.origin, RoleOrigin::Custom);
        }
        assert_eq!(colors.disabled_text, c.disabled_text);
        assert_eq!(colors.shadow, c.shadow);
        assert_eq!(colors.link_visited, None);
        assert_eq!(colors.osd, SurfaceColors::default());
        for (name, got) in accent_fills(preset, scheme, &theme) {
            assert_eq!(got, Some(p.accent), "{preset:?} {scheme:?} {name}");
        }
        for (name, got) in on_accent(preset, scheme, &theme) {
            assert_eq!(got, Some(p.accent_fg), "{preset:?} {scheme:?} {name}");
        }
    }
}

/// The widgets the preset draws in a colour a slot replaces take the slot's
/// colour; the control fills with their text, the hover fills, the outlines
/// of their own, the disabled colours, the title bar, the list's header and
/// alternate row keep the preset's.
#[test]
fn a_palette_recolours_what_it_states_and_nothing_else() {
    for (preset, scheme) in every_scheme() {
        let p = distinct_palette(scheme);
        let ctx = ThemeContext::default().with_scheme(scheme);
        let plain = Theme::preset(preset).resolve(&ctx);
        let theme = Theme::preset(preset).with_palette(p.clone()).resolve(&ctx);
        let normal = WindowState::default();
        let none = WidgetState::NONE;
        let selected = WidgetState::SELECTED;
        for (name, got, want) in [
            (
                "window frame",
                solid(&theme.window.frame(normal).fill),
                p.window_bg,
            ),
            ("menu", solid(&theme.menu.panel(none).fill), p.window_bg),
            (
                "menu bar",
                solid(&theme.menu_bar.bar(none).fill),
                p.window_bg,
            ),
            ("tab bar", solid(&theme.tab.bar(none).fill), p.window_bg),
            ("toolbar", solid(&theme.toolbar.bar(none).fill), p.window_bg),
            ("status bar", solid(&theme.status_bar.bar.fill), p.window_bg),
            (
                "segmented control",
                solid(&theme.segmented_control.track(none).fill),
                p.window_bg,
            ),
            (
                "list",
                solid(&theme.list.container(ListQuery::new(none)).fill),
                p.view_bg,
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
                "text field's outline",
                stroke_color(&theme.input.field(none).stroke),
                p.border,
            ),
            ("separator", Some(theme.separator.color), p.border),
            (
                "scroll bar's handle",
                solid(&theme.scrollbar.thumb(none).fill),
                p.foreground_muted,
            ),
            (
                "progress bar's groove",
                solid(&theme.progress_bar.track(none).fill),
                p.foreground_muted,
            ),
            (
                "selected row",
                solid(&theme.list.highlight(ListQuery::new(selected)).fill),
                p.selection_bg,
            ),
            (
                "hovered menu item",
                solid(&theme.menu.item(WidgetState::HOVER).fill),
                p.selection_bg,
            ),
        ] {
            assert_eq!(got, Some(want), "{preset:?} {scheme:?} {name}");
        }
        // The button's outline, where the generation draws one: macos-tahoe's push button has none.
        let outline = match preset {
            Preset::MacosTahoe => None,
            _ => Some(p.border),
        };
        assert_eq!(
            stroke_color(&theme.button.body(ButtonQuery::new(none)).stroke),
            outline,
            "{preset:?} {scheme:?} button's outline"
        );
        for (name, got, want) in [
            ("body", theme.typography.body.color, p.foreground),
            ("tooltip", theme.tooltip.text.color, p.foreground),
            ("popover", theme.popover.text.color, p.foreground),
            (
                "sidebar's item",
                theme.sidebar.item_label(none).color,
                p.foreground,
            ),
            (
                "list's item",
                theme.list.item_label(ListQuery::new(none)).color,
                p.foreground,
            ),
            (
                "selected row",
                theme.list.item_label(ListQuery::new(selected)).color,
                p.selection_fg,
            ),
            (
                "hovered menu item",
                theme.menu.item_label(WidgetState::HOVER).color,
                p.selection_fg,
            ),
            ("link", theme.link.label(none).color, p.link),
        ] {
            assert_eq!(got, want, "{preset:?} {scheme:?} {name}");
        }
        let unchanged = |t: &ResolvedTheme| {
            [
                solid(&t.button.body(ButtonQuery::new(none)).fill),
                solid(&t.button.body(ButtonQuery::new(WidgetState::HOVER)).fill),
                solid(&t.button.body(ButtonQuery::new(WidgetState::DISABLED)).fill),
                Some(t.button.label(ButtonQuery::new(none)).color),
                solid(&t.input.field(none).fill),
                Some(t.input.text(none).color),
                Some(t.input.placeholder(none).color),
                solid(&t.checkbox.indicator(none).fill),
                stroke_color(&t.checkbox.indicator(none).stroke),
                solid(&t.switch.track(none).fill),
                solid(&t.switch.track(WidgetState::DISABLED).fill),
                solid(&t.switch.thumb(none).fill),
                solid(&t.slider.track(none).fill),
                solid(&t.slider.active_track(WidgetState::DISABLED).fill),
                solid(&t.combo_box.field(none).fill),
                Some(t.combo_box.label(none).color),
                solid(&t.window.title_bar(normal).fill),
                solid(&t.list.header(ListQuery::new(none)).fill),
                solid(&t.list.row(ListQuery::new(none).alternate(true)).fill),
                Some(t.list.grid_color),
                Some(t.spinner.color),
                Some(t.colors.disabled_text),
                Some(t.colors.shadow),
                t.colors.selection.unfocused_background,
                t.colors.selection.unfocused_foreground,
            ]
        };
        assert_eq!(
            unchanged(&theme),
            unchanged(&plain),
            "{preset:?} {scheme:?}"
        );
    }
}

/// An explicit accent over a palette replaces the palette's accent, its
/// text and its selection, whose texts are then the preset's
/// (`accent-decision.md`, M7); every other slot of the palette stands, its
/// link, informational colour and caret with it.
#[test]
fn an_explicit_accent_replaces_the_palette_s_accent_slots() {
    let accent = hex(RED);
    for (preset, scheme) in every_scheme() {
        let p = distinct_palette(scheme);
        let ctx = ThemeContext::default().with_scheme(scheme);
        let theme = Theme::preset(preset)
            .with_palette(p.clone())
            .with_accent(accent)
            .resolve(&ctx);
        let palette_only = Theme::preset(preset).with_palette(p.clone()).resolve(&ctx);
        let c = constants(preset, scheme);
        let colors = &theme.colors;
        assert_eq!(colors.accent.fill, accent);
        assert_eq!(colors.accent.foreground, c.accent_foreground);
        assert_eq!(
            colors.selection,
            SelectionColors::new(
                accent,
                c.selection_foreground,
                Some(c.selection_inactive_background),
                Some(c.selection_inactive_foreground)
            )
        );
        assert_eq!(
            colors.text_selection,
            SelectionColors::new(accent, c.selection_foreground, None, None)
        );
        assert_eq!(colors.focus_ring, ring(accent), "{preset:?} {scheme:?}");
        assert_eq!(colors.caret, p.caret);
        assert_eq!(colors.link, p.link);
        assert_eq!(colors.info, palette_only.colors.info);
        assert_eq!(colors.window, palette_only.colors.window);
        assert_eq!(colors.card, palette_only.colors.card);
        assert_eq!(colors.tooltip, palette_only.colors.tooltip);
        assert_eq!(colors.error, palette_only.colors.error);
        for (name, got) in accent_fills(preset, scheme, &theme) {
            assert_eq!(got, Some(accent), "{preset:?} {scheme:?} {name}");
        }
        assert_eq!(
            theme.menu.item_label(WidgetState::HOVER).color,
            c.menu_hover_foreground,
            "{preset:?} {scheme:?}"
        );
    }
}

/// `macos-tahoe`'s checked toggle mixes the accent in force with black
/// (light) or white (dark) by [`BUTTON_CHECKED_MIX`], which gives each
/// reading of the parked facts, under the default and the red accent, to
/// within one level per channel (os-probe run 41006842699,
/// `os-probe-macos-26/tiebreak-derivations.toml`,
/// `derivation.checked_fill_*`); its label is white. A hovered checked
/// toggle keeps the checked look, the primary look wins over it, the
/// checked and disabled look over the disabled one
/// ([`tahoe_s_pressed_and_disabled_checked_button_mix_the_accent_in_force`]),
/// and `macos-monterey` states no checked look.
#[test]
fn tahoe_s_checked_button_mixes_the_accent_in_force() {
    let readings = [
        (Scheme::Light, 0x007aff, 0x0077f9),
        (Scheme::Light, RED, 0xdb373c),
        (Scheme::Dark, 0x007aff, 0x077dff),
        (Scheme::Dark, 0xff5257, 0xff575b),
    ];
    let checked = ButtonQuery::new(WidgetState::CHECKED);
    for (scheme, accent, reading) in readings {
        let ctx = ThemeContext::default().with_scheme(scheme);
        let theme = Theme::preset(Preset::MacosTahoe)
            .with_accent(hex(accent))
            .resolve(&ctx);
        let body = theme.button.body(checked);
        let got = solid(&body.fill).map(Color::to_srgb8);
        let want = hex(reading).to_srgb8();
        assert!(
            got.is_some_and(|got| got.iter().zip(want).all(|(g, w)| g.abs_diff(w) <= 1)),
            "{scheme:?} {accent:06x}: {got:?} against {want:?}"
        );
        assert!(body.from_state.contains(PartStated::FILL), "{scheme:?}");
        assert_eq!(
            theme.button.label(checked).color,
            hex(0xffffff),
            "{scheme:?}"
        );
        assert_eq!(
            theme
                .button
                .body(ButtonQuery::new(WidgetState::CHECKED | WidgetState::HOVER))
                .fill,
            body.fill,
            "{scheme:?}"
        );
        assert_eq!(
            theme
                .button
                .body(ButtonQuery::primary().state(WidgetState::CHECKED))
                .fill,
            Paint::Solid(theme.colors.accent.fill),
            "{scheme:?}"
        );
    }
    for ctx in [ThemeContext::light(), ThemeContext::dark()] {
        let theme = Theme::preset(Preset::MacosMonterey).resolve(&ctx);
        let rest = ButtonQuery::new(WidgetState::NONE);
        assert_eq!(theme.button.body(checked), theme.button.body(rest));
        assert_eq!(theme.button.label(checked), theme.button.label(rest));
        for state in [WidgetState::PRESSED, WidgetState::DISABLED] {
            assert_eq!(
                theme
                    .button
                    .body(ButtonQuery::new(WidgetState::CHECKED | state)),
                theme.button.body(ButtonQuery::new(state)),
                "{state:?}"
            );
            assert_eq!(
                theme
                    .button
                    .label(ButtonQuery::new(WidgetState::CHECKED | state)),
                theme.button.label(ButtonQuery::new(state)),
                "{state:?}"
            );
        }
    }
}

/// `macos-tahoe`'s pressed checked toggle mixes the accent in force with
/// black (light) or white (dark) by [`BUTTON_CHECKED_PRESSED_MIX`], its
/// disabled checked toggle by [`BUTTON_CHECKED_DISABLED_MIX`] at
/// [`BUTTON_CHECKED_DISABLED_ALPHA`], which give each reading of the parked
/// facts, under the default and the red accent, within the facts' own
/// tolerance (2 levels pressed, 3 disabled; os-probe run 41006842699,
/// `os-probe-macos-26/tiebreak-derivations.toml`,
/// `derivation.checked_pressed_fill_*` and `derivation.checked_disabled_fill_*`).
/// The pressed toggle's label stays white, the disabled one's is #ffffff20
/// (`layers-key-push.toml`, `key.push.*.disabled.*-toggle_on.label`). The
/// disabled checked look wins over the pressed one and over the primary
/// variant; the primary look wins over the pressed checked one.
#[test]
fn tahoe_s_pressed_and_disabled_checked_button_mix_the_accent_in_force() {
    let readings = [
        (Scheme::Light, 0x007aff, 0x006b_e1ff, 0x0076_f470),
        (Scheme::Light, RED, 0xc532_36ff, 0xd636_3d71),
        (Scheme::Dark, 0x007aff, 0x1f8a_ffff, 0x0982_ff70),
        (Scheme::Dark, 0xff5257, 0xff67_6bff, 0xff5b_5d70),
    ];
    let near = |got: Option<Color>, want: u32, levels: u8| {
        let want = want.to_be_bytes();
        got.map(Color::to_srgb8)
            .is_some_and(|got| got.iter().zip(want).all(|(g, w)| g.abs_diff(w) <= levels))
    };
    let pressed = ButtonQuery::new(WidgetState::CHECKED | WidgetState::PRESSED);
    let disabled = ButtonQuery::new(WidgetState::CHECKED | WidgetState::DISABLED);
    for (scheme, accent, pressed_reading, disabled_reading) in readings {
        let ctx = ThemeContext::default().with_scheme(scheme);
        let theme = Theme::preset(Preset::MacosTahoe)
            .with_accent(hex(accent))
            .resolve(&ctx);
        let body = theme.button.body(pressed);
        assert!(
            near(solid(&body.fill), pressed_reading, 2),
            "{scheme:?} {accent:06x}: {:?} pressed",
            solid(&body.fill).map(Color::to_srgb8)
        );
        assert!(body.from_state.contains(PartStated::FILL), "{scheme:?}");
        assert_eq!(
            theme.button.label(pressed).color,
            hex(0xffffff),
            "{scheme:?}"
        );
        let body = theme.button.body(disabled);
        assert!(
            near(solid(&body.fill), disabled_reading, 3),
            "{scheme:?} {accent:06x}: {:?} disabled",
            solid(&body.fill).map(Color::to_srgb8)
        );
        let label = Color::from_srgb8(0xff, 0xff, 0xff, 0x20);
        assert_eq!(theme.button.label(disabled).color, label, "{scheme:?}");
        for query in [
            ButtonQuery::new(WidgetState::CHECKED | WidgetState::DISABLED | WidgetState::PRESSED),
            ButtonQuery::primary().state(WidgetState::CHECKED | WidgetState::DISABLED),
        ] {
            assert_eq!(theme.button.body(query).fill, body.fill, "{scheme:?}");
            assert_eq!(theme.button.label(query).color, label, "{scheme:?}");
        }
        assert_eq!(
            theme
                .button
                .body(ButtonQuery::primary().state(WidgetState::CHECKED | WidgetState::PRESSED))
                .fill,
            Paint::Solid(theme.colors.accent.fill),
            "{scheme:?}"
        );
    }
}
