use super::*;

use crate::next::preset::Theme;
use crate::next::query::{ButtonQuery, ListQuery, MotionPreference};
use crate::next::window::WindowState;

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn both_schemes() -> [ResolvedTheme; 2] {
    [ThemeContext::light(), ThemeContext::dark()]
        .map(|ctx| Theme::preset(Preset::Material3).resolve(&ctx))
}

#[test]
fn the_preset_defines_the_normal_contrast_alone() {
    assert_eq!(CONTRAST_LEVELS, &[Contrast::Normal]);
    let ctx = ThemeContext::light().with_contrast(Contrast::High);
    let high = Theme::preset(Preset::Material3).resolve(&ctx);
    assert_eq!(high.context.contrast, Contrast::Normal);
}

#[test]
fn the_constant_colours_are_their_8_bit_channels() {
    assert_eq!(LIGHT.hover_layer.to_srgb8(), [0x1d, 0x1b, 0x20, 0x14]);
    assert_eq!(DARK.disabled_track.to_srgb8(), [0x36, 0x34, 0x3b, 0x1f]);
    assert_eq!(TRANSPARENT.to_srgb8(), [0x00, 0x00, 0x00, 0x00]);
}

/// A primary button keeps the accent under the pointer and pressed, and
/// takes the disabled container when disabled.
#[test]
fn a_primary_button_keeps_the_accent_until_disabled() {
    for theme in both_schemes() {
        let primary = |state| ButtonQuery::primary().state(state);
        let accent = Paint::Solid(theme.colors.accent.fill);
        for state in [
            WidgetState::NONE,
            WidgetState::HOVER,
            WidgetState::PRESSED,
            WidgetState::CHECKED,
        ] {
            assert_eq!(theme.button.body(primary(state)).fill, accent, "{state:?}");
        }
        assert_ne!(
            theme.button.body(primary(WidgetState::DISABLED)).fill,
            accent
        );
    }
}

/// A checked tonal button keeps its checked look under the pointer and
/// pressed: hovered on-secondary at the hover state layer's opacity over
/// secondary, pressed secondary alone, not the unchecked hover and pressed
/// fills; the constants are that composite, as a seed derives it again.
#[test]
fn a_checked_tonal_button_hovered_or_pressed_keeps_its_checked_look() {
    for (theme, c) in both_schemes().iter().zip([LIGHT, DARK]) {
        let mut derived = c;
        derive_state_layers(&mut derived);
        assert_eq!(
            derived.button_checked_hover.to_srgb8(),
            c.button_checked_hover.to_srgb8()
        );
        let body = |state| theme.button.body(ButtonQuery::new(state)).fill;
        let (hover, pressed) = (WidgetState::HOVER, WidgetState::PRESSED);
        assert_eq!(
            body(WidgetState::CHECKED | hover),
            Paint::Solid(c.button_checked_hover)
        );
        assert_eq!(
            body(WidgetState::CHECKED | pressed),
            Paint::Solid(c.secondary)
        );
        assert_eq!(
            body(WidgetState::CHECKED | pressed),
            body(WidgetState::CHECKED)
        );
        assert_ne!(body(WidgetState::CHECKED | hover), body(hover));
        assert_ne!(body(WidgetState::CHECKED | pressed), body(pressed));
    }
}

/// The flat button, M3's text button: no container and no outline at rest;
/// hovered and pressed primary at the state layer's opacities, disabled the
/// on-surface container at the text button's disabled opacity; its label
/// primary, disabled on-surface at the disabled content opacity; each stated
/// for its state.
#[test]
fn a_flat_button_is_the_text_button() {
    use crate::next::style::PartStated;
    for (theme, c) in both_schemes().iter().zip([LIGHT, DARK]) {
        let query = |state| ButtonQuery::flat().state(state);
        let rest = theme.button.body(query(WidgetState::NONE));
        assert_eq!(rest.fill, Paint::Solid(TRANSPARENT));
        assert_eq!(rest.stroke, Stroke::None);
        for (state, want) in [
            (
                WidgetState::HOVER,
                c.primary.mul_alpha(HOVER_STATE_LAYER_OPACITY),
            ),
            (
                WidgetState::PRESSED,
                c.primary.mul_alpha(PRESSED_STATE_LAYER_OPACITY),
            ),
            (
                WidgetState::DISABLED,
                c.on_surface
                    .mul_alpha(TEXT_BUTTON_DISABLED_CONTAINER_OPACITY),
            ),
        ] {
            let body = theme.button.body(query(state));
            assert_eq!(body.fill, Paint::Solid(want), "{state:?}");
            assert_eq!(body.stroke, Stroke::None, "{state:?}");
            assert!(body.from_state.contains(PartStated::FILL), "{state:?}");
        }
        for state in [WidgetState::NONE, WidgetState::HOVER, WidgetState::PRESSED] {
            assert_eq!(
                theme.button.label(query(state)).color,
                c.primary,
                "{state:?}"
            );
        }
        assert_eq!(
            theme.button.label(query(WidgetState::DISABLED)).color,
            c.on_surface.mul_alpha(DISABLED_CONTENT_OPACITY)
        );
    }
}

/// The focused text field's outline wins over the hovered one.
#[test]
fn a_focused_field_keeps_its_outline_under_the_pointer() {
    for theme in both_schemes() {
        let focused = theme.input.field(WidgetState::FOCUSED).stroke;
        assert_eq!(
            theme
                .input
                .field(WidgetState::FOCUSED | WidgetState::HOVER)
                .stroke,
            focused
        );
        assert_ne!(theme.input.field(WidgetState::HOVER).stroke, focused);
    }
}

/// The switch's thumb grows when on, and a disabled thumb takes its own
/// colour, on or off.
#[test]
fn the_switch_s_thumb_grows_when_on() {
    for theme in both_schemes() {
        let off = theme.switch.thumb(WidgetState::NONE);
        let on = theme.switch.thumb(WidgetState::CHECKED);
        assert_eq!(off.min_size, [Some(SWITCH_UNCHECKED_THUMB_DIAMETER); 2]);
        assert_eq!(on.min_size, [Some(SWITCH_THUMB_DIAMETER); 2]);
        assert_eq!(
            theme
                .switch
                .thumb(WidgetState::DISABLED | WidgetState::CHECKED)
                .fill,
            theme.switch.thumb(WidgetState::DISABLED).fill
        );
    }
}

/// The line between segments is their shared outline, outline-color =
/// outline at outline-width 1 (v0_192/_md-comp-outlined-segmented-button.scss:65-66).
#[test]
fn the_segments_are_parted_by_their_outline() {
    for theme in both_schemes() {
        let outline = match theme.context.scheme {
            Scheme::Dark => DARK.outline,
            _ => LIGHT.outline,
        };
        assert_eq!(
            theme.segmented_control.separator(WidgetState::NONE).fill,
            Paint::Solid(outline)
        );
        assert_eq!(theme.segmented_control.separator_width, 1.0);
    }
}

/// A disabled radio's ring is on-surface at 0.38, checked or not, under the
/// pointer or pressed too: material-web fills the whole icon with
/// disabled-unselected-icon-color / disabled-selected-icon-color at their
/// 0.38 opacity (_radio.scss:163-166, :180-183;
/// v0_192/_md-comp-radio-button.scss:28-33).
#[test]
fn a_disabled_radio_s_ring_takes_the_disabled_content_checked_or_not() {
    for theme in both_schemes() {
        let disabled_content = match theme.context.scheme {
            Scheme::Dark => DARK.disabled_content,
            _ => LIGHT.disabled_content,
        };
        let ring = Stroke::uniform(CHECKBOX_BORDER_WIDTH, disabled_content);
        let disabled = WidgetState::DISABLED;
        for state in [
            disabled,
            disabled | WidgetState::HOVER,
            disabled | WidgetState::PRESSED,
            disabled | WidgetState::CHECKED,
            disabled | WidgetState::CHECKED | WidgetState::HOVER,
        ] {
            assert_eq!(theme.radio.indicator(state).stroke, ring, "{state:?}");
        }
    }
}

/// Right to left mirrors the per-side values the preset states unequal on
/// the left and the right, the combo box's padding, and nothing else.
#[test]
fn right_to_left_mirrors_the_unequal_sides() {
    for ctx in [ThemeContext::light(), ThemeContext::dark()] {
        let theme = Theme::preset(Preset::Material3);
        let ltr = theme.resolve(&ctx);
        let rtl = theme.resolve(&ctx.clone().with_direction(Direction::Rtl));
        let mirrored = |e: Edges| Edges::new(e.top, e.left, e.bottom, e.right);
        let field = ltr.combo_box.field(WidgetState::NONE).padding;
        assert_eq!(field.left, COMBO_BOX_PADDING_LEFT);
        assert_eq!(
            rtl.combo_box.field(WidgetState::NONE).padding,
            mirrored(field)
        );
        let mut same = rtl.clone();
        same.context = ltr.context.clone();
        same.combo_box = ltr.combo_box.clone();
        assert_eq!(same, ltr);
    }
}

/// The text scale of the test below.
const TEXT_SCALE_TWO: f32 = 2.0;

/// The text scale is Android's non-linear one: at 2.0 the body's 14 px is
/// the 2.0 table's 26 (`FontScaleConverterFactory`'s row for 14 sp), every
/// font size goes through the converter, each line height keeps its ratio to
/// its font's unscaled size (above Compose's threshold), each `defined_size`
/// stays the unscaled size, and nothing but the text changes.
#[test]
fn at_text_scale_2_the_sizes_follow_android_s_converter() -> TestResult {
    for scheme in [Scheme::Light, Scheme::Dark] {
        let ctx = ThemeContext::default().with_scheme(scheme);
        let base = Theme::preset(Preset::Material3).resolve(&ctx);
        let double = Theme::preset(Preset::Material3).resolve(&ctx.with_text_scale(TEXT_SCALE_TWO));
        assert_eq!(double.typography.body.size_px, 26.0, "{scheme:?}");
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
            assert_eq!(
                two.size_px,
                android_non_linear(one.size_px, TEXT_SCALE_TWO),
                "{scheme:?}"
            );
            assert_eq!(
                two.line_height,
                one.line_height / android_non_linear_inverse(two.size_px, TEXT_SCALE_TWO)
                    * two.size_px,
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
/// `DialogEntranceStyle::None`; Material's `ScaleFade` stays under full and
/// under reduced motion. The preset states no motion token.
#[test]
fn no_motion_removes_the_dialog_s_entrance() {
    let resolve = |motion: MotionPreference| {
        Theme::preset(Preset::Material3).resolve(&ThemeContext::light().with_motion(motion))
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

/// The colour of a solid paint.
fn solid(paint: &Paint) -> Option<Color> {
    match paint {
        Paint::Solid(color) => Some(*color),
        _ => None,
    }
}

/// The colour of `rgb`, opaque.
fn hex(rgb: u32) -> Color {
    let [_, r, g, b] = rgb.to_be_bytes();
    Color::from_srgb8(r, g, b, u8::MAX)
}

/// The name of `scheme` in the kernel's table.
fn scheme_name(scheme: Scheme) -> &'static str {
    match scheme {
        Scheme::Light => "light",
        Scheme::Dark => "dark",
    }
}

/// The colour the table of `m3_role` (`tests/vectors/m3_role.tsv`, written
/// by material-color-utilities' own classes) states for `role` from the seed
/// `seed` (`#rrggbb`) at the standard contrast level in `scheme`, from its
/// bytes over 255.
fn table_role(role: &str, seed: &str, scheme: Scheme) -> Result<Color, String> {
    let scheme = scheme_name(scheme);
    let table = include_str!("../../../../tests/vectors/m3_role.tsv");
    let result = table
        .lines()
        .find_map(|line| {
            let cells: Vec<&str> = line.split('\t').collect();
            match cells.as_slice() {
                [name, accent, "tonal_spot", "spec_2021", "0.0", s, result]
                    if name.strip_prefix('@') == Some(role) && *accent == seed && *s == scheme =>
                {
                    Some(*result)
                }
                _ => None,
            }
        })
        .ok_or_else(|| format!("no row of {role} for {seed} in {scheme}"))?;
    let channels = result
        .strip_prefix("srgb(")
        .and_then(|rest| rest.strip_suffix(')'))
        .ok_or_else(|| format!("no colour `{result}`"))?
        .split(',')
        .map(|n| {
            let unit: f64 = n
                .trim()
                .parse()
                .map_err(|_| format!("no number `{n}` in `{result}`"))?;
            let byte = (unit * 255.0).round();
            if (0.0..=255.0).contains(&byte) && (byte - unit * 255.0).abs() < 1e-6 {
                Ok(byte as u8)
            } else {
                Err(format!("no byte over 255 in `{result}`"))
            }
        })
        .collect::<Result<Vec<u8>, String>>()?;
    let [r, g, b, a] = channels.as_slice() else {
        return Err(format!("no four channels in `{result}`"));
    };
    Ok(Color::from_srgb8(*r, *g, *b, *a))
}

/// The seeds of the tests below, two of the table's at the standard level:
/// its blue and its yellow (a hue `Hct.isYellow` holds).
const SEEDS: [(&str, u32); 2] = [("#0000ff", 0x0000ff), ("#ffff00", 0xffff00)];

/// An explicit accent is the seed of the dynamic scheme: every role the
/// builder takes from the scheme is the table's row of that role, and so the
/// widget colours drawn in them; the state colours are derived again from
/// them; the colours that are no role of the scheme keep the preset's.
#[test]
fn an_explicit_accent_reproduces_the_kernel_s_rows() -> TestResult {
    for (seed, rgb) in SEEDS {
        for scheme in [Scheme::Light, Scheme::Dark] {
            let ctx = ThemeContext::default().with_scheme(scheme);
            let theme = Theme::preset(Preset::Material3)
                .with_accent(hex(rgb))
                .resolve(&ctx);
            let role = |name| table_role(name, seed, scheme);
            let colors = &theme.colors;
            let none = WidgetState::NONE;
            for (name, got, want) in [
                ("accent", colors.accent.fill, role("primary")?),
                (
                    "accent's text",
                    colors.accent.foreground,
                    role("on_primary")?,
                ),
                (
                    "window",
                    colors.window.background.ok_or("no window")?,
                    role("surface")?,
                ),
                (
                    "window's text",
                    colors.window.foreground.ok_or("no text")?,
                    role("on_surface")?,
                ),
                (
                    "muted text",
                    colors.window.foreground_muted.ok_or("no muted text")?,
                    role("on_surface_variant")?,
                ),
                (
                    "border",
                    colors.window.border.ok_or("no border")?,
                    role("outline")?,
                ),
                (
                    "view",
                    colors.view.background.ok_or("no view")?,
                    role("surface")?,
                ),
                (
                    "tooltip",
                    colors.tooltip.background.ok_or("no tooltip")?,
                    role("inverse_surface")?,
                ),
                (
                    "tooltip's text",
                    colors.tooltip.foreground.ok_or("no tooltip text")?,
                    role("inverse_on_surface")?,
                ),
                (
                    "selection",
                    colors.selection.background,
                    role("secondary_container")?,
                ),
                (
                    "selection's text",
                    colors.selection.foreground,
                    role("on_secondary_container")?,
                ),
                ("focus ring", colors.focus_ring, role("secondary")?),
                ("caret", colors.caret, role("primary")?),
                ("link", colors.link, role("primary")?),
                ("error", colors.error.fill, role("error")?),
                ("error's text", colors.error.foreground, role("on_error")?),
                ("info", colors.info.fill, role("primary")?),
                (
                    "menu",
                    solid(&theme.menu.panel(none).fill).ok_or("no menu fill")?,
                    role("surface_container")?,
                ),
                (
                    "menu's separator",
                    solid(&theme.menu.separator(none).fill).ok_or("no separator fill")?,
                    role("surface_variant")?,
                ),
                (
                    "popover",
                    solid(&theme.popover.panel.fill).ok_or("no popover fill")?,
                    role("surface_container")?,
                ),
                (
                    "dialog",
                    solid(&theme.dialog.panel.fill).ok_or("no dialog fill")?,
                    role("surface_container_high")?,
                ),
                (
                    "card",
                    solid(&theme.card.container(none).fill).ok_or("no card fill")?,
                    role("surface_container_highest")?,
                ),
                (
                    "tonal button",
                    solid(&theme.button.body(ButtonQuery::new(none)).fill)
                        .ok_or("no button fill")?,
                    role("secondary_container")?,
                ),
                (
                    "checked button",
                    solid(
                        &theme
                            .button
                            .body(ButtonQuery::new(WidgetState::CHECKED))
                            .fill,
                    )
                    .ok_or("no checked fill")?,
                    role("secondary")?,
                ),
                ("separator", theme.separator.color, role("outline_variant")?),
            ] {
                assert_eq!(got, want, "{seed} {scheme:?} {name}");
            }
            let on_surface = role("on_surface")?;
            let tonal = role("secondary_container")?;
            let on_tonal = role("on_secondary_container")?;
            for (name, got, want) in [
                (
                    "disabled text",
                    colors.disabled_text,
                    on_surface.mul_alpha(DISABLED_CONTENT_OPACITY),
                ),
                (
                    "hovered menu item",
                    solid(&theme.menu.item(WidgetState::HOVER).fill).ok_or("no hover fill")?,
                    on_surface.mul_alpha(HOVER_STATE_LAYER_OPACITY),
                ),
                (
                    "disabled button",
                    solid(
                        &theme
                            .button
                            .body(ButtonQuery::new(WidgetState::DISABLED))
                            .fill,
                    )
                    .ok_or("no disabled fill")?,
                    on_surface.mul_alpha(DISABLED_CONTAINER_OPACITY),
                ),
                (
                    "hovered button",
                    solid(&theme.button.body(ButtonQuery::new(WidgetState::HOVER)).fill)
                        .ok_or("no hovered fill")?,
                    on_tonal
                        .mul_alpha(HOVER_STATE_LAYER_OPACITY)
                        .composite_over(tonal),
                ),
                (
                    "pressed button",
                    solid(
                        &theme
                            .button
                            .body(ButtonQuery::new(WidgetState::PRESSED))
                            .fill,
                    )
                    .ok_or("no pressed fill")?,
                    on_tonal
                        .mul_alpha(PRESSED_STATE_LAYER_OPACITY)
                        .composite_over(tonal),
                ),
                (
                    "disabled off switch",
                    solid(&theme.switch.track(WidgetState::DISABLED).fill)
                        .ok_or("no disabled track")?,
                    role("surface_container_highest")?.mul_alpha(DISABLED_TRACK_OPACITY),
                ),
            ] {
                assert_eq!(got, want, "{seed} {scheme:?} {name}");
            }
            let c = match scheme {
                Scheme::Light => &LIGHT,
                Scheme::Dark => &DARK,
            };
            assert_eq!(colors.shadow, c.shadow);
            assert_eq!(colors.warning.fill, c.warning);
            assert_eq!(colors.success.fill, c.success);
            assert_eq!(colors.error.origin, RoleOrigin::Native);
            assert_eq!(colors.info.origin, RoleOrigin::Harmonized);
            assert_eq!(theme.link.label(WidgetState::PRESSED).color, c.link_pressed);
        }
    }
    Ok(())
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

/// The role `role` of the dynamic scheme of `seed` in `scheme`.
fn seed_role(role: &str, seed: Color, scheme: Scheme) -> Result<Color, String> {
    let [r, g, b, a] = m3_role(
        role,
        [seed.r, seed.g, seed.b, seed.a],
        M3_CONTRAST_LEVEL,
        scheme,
    )
    .ok_or_else(|| format!("no role {role}"))?;
    Ok(Color::new(r, g, b, a))
}

/// A palette states the roles its slots name; the roles no slot names are
/// the dynamic scheme's of the palette's accent.
#[test]
fn a_palette_states_its_roles() -> TestResult {
    for scheme in [Scheme::Light, Scheme::Dark] {
        let p = distinct_palette(scheme);
        let theme = Theme::preset(Preset::Material3)
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
        let selection = SelectionColors::new(
            p.selection_bg,
            p.selection_fg,
            Some(p.selection_bg),
            Some(p.selection_fg),
        );
        assert_eq!(colors.selection, selection);
        assert_eq!(
            colors.text_selection,
            SelectionColors::new(p.selection_bg, p.selection_fg, None, None)
        );
        assert_eq!(colors.caret, p.caret);
        assert_eq!(colors.link, p.link);
        for (status, fill) in [
            (colors.error, p.error),
            (colors.warning, p.warning),
            (colors.success, p.success),
            (colors.info, p.info),
        ] {
            assert_eq!(status.fill, fill, "{scheme:?}");
            assert_eq!(status.origin, RoleOrigin::Custom);
        }
        assert_eq!(colors.info.foreground, p.accent_fg);
        assert_eq!(
            colors.error.foreground,
            seed_role("on_error", p.accent, scheme)?
        );
        assert_eq!(colors.focus_ring, seed_role("secondary", p.accent, scheme)?);
        assert_eq!(
            colors.disabled_text,
            p.foreground.mul_alpha(DISABLED_CONTENT_OPACITY)
        );
        assert_eq!(colors.link_visited, None);
        assert_eq!(colors.osd, SurfaceColors::default());
    }
    Ok(())
}

/// The widgets the preset draws in a colour a slot replaces take the slot's
/// colour, the state colours derived from them follow, and the colours no
/// slot replaces are the dynamic scheme's of the palette's accent.
#[test]
fn a_palette_recolours_what_it_states() -> TestResult {
    for scheme in [Scheme::Light, Scheme::Dark] {
        let p = distinct_palette(scheme);
        let ctx = ThemeContext::default().with_scheme(scheme);
        let theme = Theme::preset(Preset::Material3)
            .with_palette(p.clone())
            .resolve(&ctx);
        let none = WidgetState::NONE;
        let normal = WindowState::default();
        let primary = |state| ButtonQuery::primary().state(state);
        for (name, got, want) in [
            (
                "window frame",
                solid(&theme.window.frame(normal).fill),
                p.window_bg,
            ),
            ("toolbar", solid(&theme.toolbar.bar(none).fill), p.window_bg),
            (
                "list",
                solid(&theme.list.container(ListQuery::new(none)).fill),
                p.window_bg,
            ),
            (
                "list's alternate row",
                solid(&theme.list.row(ListQuery::new(none).alternate(true)).fill),
                p.window_bg,
            ),
            (
                "text field",
                solid(&theme.input.field(none).fill),
                p.window_bg,
            ),
            (
                "list's header",
                solid(&theme.list.header(ListQuery::new(none)).fill),
                p.view_bg,
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
                "selected row",
                solid(
                    &theme
                        .list
                        .highlight(ListQuery::new(WidgetState::SELECTED))
                        .fill,
                ),
                p.selection_bg,
            ),
            (
                "primary button",
                solid(&theme.button.body(primary(none)).fill),
                p.accent,
            ),
            (
                "switch's off thumb",
                solid(&theme.switch.thumb(none).fill),
                p.border,
            ),
            (
                "hovered menu item",
                solid(&theme.menu.item(WidgetState::HOVER).fill),
                p.foreground.mul_alpha(HOVER_STATE_LAYER_OPACITY),
            ),
            (
                "menu",
                solid(&theme.menu.panel(none).fill),
                seed_role("surface_container", p.accent, scheme)?,
            ),
            (
                "tonal button",
                solid(&theme.button.body(ButtonQuery::new(none)).fill),
                seed_role("secondary_container", p.accent, scheme)?,
            ),
        ] {
            assert_eq!(got, Some(want), "{scheme:?} {name}");
        }
        for (name, got, want) in [
            ("body", theme.typography.body.color, p.foreground),
            ("tooltip", theme.tooltip.text.color, p.foreground),
            ("popover", theme.popover.text.color, p.foreground),
            ("tab", theme.tab.label(none).color, p.foreground_muted),
            (
                "primary button",
                theme.button.label(primary(none)).color,
                p.accent_fg,
            ),
            (
                "selected row",
                theme
                    .list
                    .item_label(ListQuery::new(WidgetState::SELECTED))
                    .color,
                p.selection_fg,
            ),
            ("link", theme.link.label(none).color, p.link),
        ] {
            assert_eq!(got, want, "{scheme:?} {name}");
        }
    }
    Ok(())
}

/// An explicit accent over a palette replaces the palette's accent and its
/// text with the dynamic scheme's of the explicit seed; every other slot of
/// the palette stands.
#[test]
fn an_explicit_accent_replaces_the_palette_s_accent_alone() -> TestResult {
    for scheme in [Scheme::Light, Scheme::Dark] {
        let p = distinct_palette(scheme);
        let accent = hex(0x0000ff);
        let ctx = ThemeContext::default().with_scheme(scheme);
        let theme = Theme::preset(Preset::Material3)
            .with_palette(p.clone())
            .with_accent(accent)
            .resolve(&ctx);
        let palette_alone = Theme::preset(Preset::Material3)
            .with_palette(p.clone())
            .resolve(&ctx);
        let colors = &theme.colors;
        assert_eq!(colors.accent.fill, seed_role("primary", accent, scheme)?);
        assert_eq!(
            colors.accent.foreground,
            seed_role("on_primary", accent, scheme)?
        );
        assert_eq!(colors.focus_ring, seed_role("secondary", accent, scheme)?);
        let slots = |t: &ResolvedTheme| {
            (
                t.colors.window,
                t.colors.view,
                t.colors.card,
                t.colors.tooltip,
                t.colors.selection,
                t.colors.text_selection,
                t.colors.link,
                t.colors.caret,
                t.colors.info.fill,
                t.colors.error.fill,
                t.colors.warning.fill,
                t.colors.success.fill,
            )
        };
        assert_eq!(slots(&theme), slots(&palette_alone), "{scheme:?}");
    }
    Ok(())
}

/// Without a palette or an accent the roles are the constants', and an
/// explicit accent changes the theme.
#[test]
fn the_constants_stand_without_a_seed() {
    for (ctx, c) in [
        (ThemeContext::light(), &LIGHT),
        (ThemeContext::dark(), &DARK),
    ] {
        let theme = Theme::preset(Preset::Material3).resolve(&ctx);
        assert_eq!(theme.colors, roles(c, None));
        let seeded = Theme::preset(Preset::Material3)
            .with_accent(hex(0x0000ff))
            .resolve(&ctx);
        assert_ne!(seeded.colors, theme.colors);
    }
}
