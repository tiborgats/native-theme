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
/// colour; the fills the preset states as libadwaita's layers over its own
/// window, the placeholder and the switch's hover colour keep the preset's.
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
        let c = constants(scheme);
        assert_eq!(
            solid(&theme.button.body(ButtonQuery::new(none)).fill),
            Some(c.button_background)
        );
        assert_eq!(theme.input.placeholder(none).color, c.placeholder);
        assert_eq!(
            solid(
                &theme
                    .switch
                    .track(WidgetState::HOVER | WidgetState::CHECKED)
                    .fill
            ),
            Some(c.switch_hover_on)
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
