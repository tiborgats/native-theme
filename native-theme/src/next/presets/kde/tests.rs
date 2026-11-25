use super::*;

use crate::next::preset::Theme;
use crate::next::query::{ButtonQuery, Contrast, ListQuery, MotionPreference, StyleQuery};
use crate::next::reader::{OsSelectionColors, SystemOverrides};
use crate::next::style::PartStated;

/// A test's result.
type TestResult = Result<(), Box<dyn std::error::Error>>;

const SELECTION: Color = Color::rgb(0.0, 0.0, 1.0);
const DECORATION_FOCUS: Color = Color::rgb(0.0, 1.0, 0.0);
const PORTAL_ACCENT: Color = Color::rgb(1.0, 0.0, 0.0);

/// The light theme of [`system`].
fn with_system(accent: Option<Color>) -> ResolvedTheme {
    Theme::preset(Preset::KdeBreeze)
        .with_system_overrides(system(accent))
        .resolve(&ThemeContext::light())
}

/// A light system stating a selection colour and a decoration focus colour,
/// with `accent` as its portal accent.
fn system(accent: Option<Color>) -> SystemOverrides {
    let os = OsColors {
        selection: Some(OsSelectionColors {
            background: Some(SELECTION),
            ..OsSelectionColors::default()
        }),
        kde_decoration_focus: Some(DECORATION_FOCUS),
        ..OsColors::default()
    };
    SystemOverrides {
        color_schemes: vec![((Scheme::Light, Contrast::Normal), os)],
        scheme: Some(Scheme::Light),
        accent,
        ..SystemOverrides::default()
    }
}

/// The flat button draws Breeze's flat frame: nothing at rest and disabled;
/// hovered a Highlight pen; checked ButtonText at FLAT_CHECKED_ALPHA in the
/// button's pen; pressed the toolbar's pressed fill in a Highlight pen; checked
/// and disabled the Disabled group's ButtonText at FLAT_CHECKED_ALPHA in the
/// mix of the Disabled Button and ButtonText (breezehelper.cpp:665-712 at
/// breeze 6.8.2), #a8a9aa / #6d6f72 at 0x20 (platform-facts.md §2.1.6).
#[test]
fn a_flat_button_draws_breeze_s_flat_frame() {
    let pen = |stroke: &Stroke| match stroke {
        Stroke::Uniform { color, .. } => Some(*color),
        _ => None,
    };
    for (ctx, c, disabled_checked) in [
        (ThemeContext::light(), LIGHT, 0xa8a9_aa20),
        (ThemeContext::dark(), DARK, 0x6d6f_7220),
    ] {
        let theme = Theme::preset(Preset::KdeBreeze).resolve(&ctx);
        let of = |state: WidgetState| {
            let body = theme.button.body(ButtonQuery::flat().state(state));
            (body.fill, pen(&body.stroke))
        };
        let (checked, disabled) = (WidgetState::CHECKED, WidgetState::DISABLED);
        assert_eq!(of(WidgetState::NONE), (Paint::None, None));
        assert_eq!(of(disabled), (Paint::None, None));
        assert_eq!(of(disabled | WidgetState::HOVER), (Paint::None, None));
        assert_eq!(
            of(WidgetState::HOVER),
            (
                Paint::Solid(TOOLBAR_BUTTON_HOVER_BACKGROUND),
                Some(c.selection_background)
            )
        );
        assert_eq!(
            of(checked),
            (Paint::Solid(c.toolbar_button_checked), Some(c.button_frame))
        );
        assert_eq!(
            of(WidgetState::PRESSED),
            (
                Paint::Solid(c.toolbar_button_pressed),
                Some(c.selection_background)
            )
        );
        assert_eq!(
            of(checked | disabled),
            (
                Paint::Solid(srgb8(disabled_checked)),
                Some(mix(
                    c.button_disabled_background,
                    c.button_disabled_foreground,
                    c.frame_contrast
                ))
            )
        );
        assert_eq!(
            of(checked | disabled | WidgetState::HOVER),
            of(checked | disabled)
        );
    }
}

#[test]
fn a_portal_accent_replaces_the_accent_and_the_selection_but_not_the_kde_frames() {
    for (accent, fill, selection) in [
        (None, DECORATION_FOCUS, SELECTION),
        (Some(PORTAL_ACCENT), PORTAL_ACCENT, PORTAL_ACCENT),
    ] {
        let theme = with_system(accent);
        assert_eq!(theme.colors.accent.fill, fill, "{accent:?}");
        assert_eq!(theme.colors.selection.background, selection, "{accent:?}");
        assert_eq!(theme.colors.text_selection.background, selection);
        assert_eq!(theme.colors.focus_ring, focus_ring_of(selection));
        assert_eq!(
            theme
                .list
                .highlight(ListQuery::new(WidgetState::SELECTED))
                .fill,
            Paint::Solid(selection)
        );
        assert_eq!(
            theme.button.body(ButtonQuery::primary()).fill,
            Paint::Solid(fill)
        );
        // The frames 0.7.2 keeps on kdeglobals' colours.
        let stroke = |style: PartStyle| match style.stroke {
            Stroke::Uniform { color, .. } => Some(color),
            _ => None,
        };
        for state in [
            WidgetState::HOVER,
            WidgetState::PRESSED,
            WidgetState::FOCUS_VISIBLE,
        ] {
            assert_eq!(
                stroke(theme.button.body(ButtonQuery::new(state))),
                Some(SELECTION),
                "{accent:?} {state:?}"
            );
        }
        assert_eq!(
            stroke(theme.checkbox.indicator(WidgetState::CHECKED)),
            Some(SELECTION)
        );
        assert_eq!(
            stroke(theme.checkbox.indicator(WidgetState::HOVER)),
            Some(DECORATION_FOCUS)
        );
        assert_eq!(
            stroke(theme.radio.indicator(WidgetState::HOVER)),
            Some(DECORATION_FOCUS)
        );
    }
}

#[test]
fn system_colours_apply_only_to_their_scheme_and_contrast() {
    let [light, dark] = both_schemes();
    assert_ne!(with_system(None).colors, light.colors);
    let theme = Theme::preset(Preset::KdeBreeze)
        .with_system_overrides(system(None))
        .resolve(&ThemeContext::dark());
    assert_eq!(theme.colors, dark.colors);
}

#[test]
fn the_constant_colours_are_their_8_bit_channels() {
    for v in 0..=u8::MAX {
        let rgba = u32::from_be_bytes([v, 0, u8::MAX, v]);
        assert_eq!(srgb8(rgba), Color::from_srgb8(v, 0, u8::MAX, v), "{v}");
    }
}

#[test]
fn the_caret_is_the_window_text_else_the_system_s_decoration_focus() {
    for theme in both_schemes() {
        // 0.7.2: `input.caret_color` is `defaults.text_color` where unstated.
        assert_eq!(
            Some(theme.colors.caret),
            theme.colors.window.foreground,
            "{:?}",
            theme.context.scheme
        );
    }
    assert_eq!(with_system(None).colors.caret, DECORATION_FOCUS);
}

#[test]
fn the_visited_link_is_breeze_s_else_the_system_s() {
    // `link.visited_text_color`: platform-facts.md (§2.28), [Colors:View] ForegroundVisited
    // 155,89,182 in both schemes.
    let visited = Color::from_srgb8(155, 89, 182, u8::MAX);
    for theme in both_schemes() {
        assert_eq!(theme.colors.link_visited, Some(visited));
    }
    assert_eq!(with_system(None).colors.link_visited, Some(visited));
    let stated = Color::rgb(0.5, 0.0, 0.5);
    let mut overrides = system(None);
    for (_, os) in &mut overrides.color_schemes {
        os.link_visited = Some(stated);
    }
    assert_eq!(light_with(overrides).colors.link_visited, Some(stated));
}

/// [`system`] with `edit` applied to its colours.
fn system_with(edit: impl Fn(&mut OsColors)) -> SystemOverrides {
    let mut overrides = system(Some(PORTAL_ACCENT));
    for (_, os) in &mut overrides.color_schemes {
        edit(os);
    }
    overrides
}

#[test]
fn the_scheme_s_accent_comes_before_the_portal_accent() {
    let scheme_accent = Color::rgb(1.0, 1.0, 0.0);
    let theme = light_with(system_with(|os| os.accent = Some(scheme_accent)));
    assert_eq!(theme.colors.accent.fill, scheme_accent);
    assert_eq!(theme.colors.selection.background, scheme_accent);
    assert_eq!(theme.colors.focus_ring, focus_ring_of(scheme_accent));
    // Without either accent, the decoration focus colour (`a_portal_accent_replaces_…`).
    assert_eq!(with_system(None).colors.accent.fill, DECORATION_FOCUS);
}

#[test]
fn a_stated_focus_ring_is_the_ring_s_colour_as_given() {
    let ring = Color::new(0.0, 1.0, 1.0, 0.5);
    let theme = light_with(system_with(|os| os.focus_ring = Some(ring)));
    assert_eq!(theme.colors.focus_ring, ring);
    assert_eq!(
        theme.focus_ring.stroke,
        Stroke::uniform(FOCUS_RING_WIDTH, ring)
    );
}

#[test]
fn a_stated_text_selection_replaces_the_text_selection_s_colours() {
    let [background, foreground] = [Color::rgb(0.1, 0.2, 0.3), Color::rgb(0.9, 0.8, 0.7)];
    let theme = light_with(system_with(|os| {
        os.text_selection = Some(OsSelectionColors::new(
            Some(background),
            None,
            None,
            Some(foreground),
        ))
    }));
    let text_selection = theme.colors.text_selection;
    assert_eq!(text_selection.background, background);
    // A colour it does not state stays the selection's.
    assert_eq!(text_selection.foreground, theme.colors.selection.foreground);
    assert_eq!(text_selection.unfocused_background, None);
    assert_eq!(text_selection.unfocused_foreground, Some(foreground));
    // The view's selection keeps the accent.
    assert_eq!(theme.colors.selection.background, PORTAL_ACCENT);
}

/// The light theme of the scheme-independent settings `system`.
fn light_with(system: SystemOverrides) -> ResolvedTheme {
    Theme::preset(Preset::KdeBreeze)
        .with_system_overrides(system)
        .resolve(&ThemeContext::light())
}

#[test]
fn a_pixel_ui_font_keeps_its_unit_in_the_headings() {
    let theme = light_with(SystemOverrides {
        ui_font: Some(OsFont::new(
            Some(Arc::from("Inter")),
            Some(FontSize::Px(13.0)),
            None,
            Some(FontStyle::Italic),
            None,
        )),
        ..SystemOverrides::default()
    });
    let body = &theme.typography.body;
    assert_eq!(&*body.family, "Inter");
    assert_eq!(body.defined_size, Some(FontSize::Px(13.0)));
    assert_eq!(body.size_px, 13.0);
    // The weight the font does not give is the preset's.
    assert_eq!(body.weight, FONT_WEIGHT);
    assert_eq!(body.style, FontStyle::Italic);
    assert_eq!(body.line_height, LINE_HEIGHT * 13.0);
    for (entry, factor) in [
        (&theme.typography.headline, HEADING_LEVEL_2_FACTOR),
        (&theme.typography.title1, HEADING_LEVEL_1_FACTOR),
    ] {
        assert_eq!(
            entry.as_ref().map(|e| (e.defined_size, e.weight)),
            Some((Some(FontSize::Px(13.0 * factor)), HEADING_WEIGHT))
        );
    }
    // A label takes the body's family.
    assert_eq!(
        &*theme
            .button
            .label(ButtonQuery::new(WidgetState::NONE))
            .family,
        "Inter"
    );
}

#[test]
fn the_icon_theme_is_dropped_with_its_sizes_where_the_system_was_in_the_other_scheme() {
    let sizes = IconSizes::new(Some(24.0), None, Some(64.0), None, None);
    let system = |scheme| SystemOverrides {
        scheme: Some(scheme),
        icon_theme: Some(Arc::from("Papirus")),
        icon_sizes: Some(sizes),
        ..SystemOverrides::default()
    };
    let same = light_with(system(Scheme::Light));
    assert_eq!(same.icons.theme.as_deref(), Some("Papirus"));
    assert_eq!(same.icons.sizes.toolbar, Some(24.0));
    assert_eq!(same.icons.sizes.small, Some(ICON_SMALL));
    assert_eq!(same.icons.sizes.large, Some(64.0));
    let other = light_with(system(Scheme::Dark));
    let [light, _] = both_schemes();
    assert_eq!(other.icons, light.icons);
}

#[test]
fn a_caret_without_blink_keeps_the_preset_s_visible_half() {
    let theme = light_with(SystemOverrides {
        caret_blink_off_ms: Some(0),
        double_click_ms: Some(400),
        cursor_theme: Some(Arc::from("breeze_cursors")),
        cursor_size_px: Some(24.0),
        ..SystemOverrides::default()
    });
    assert_eq!(theme.caret.blink_on_ms, Some(CARET_BLINK_ON_MS));
    assert_eq!(theme.caret.blink_off_ms, Some(0));
    assert_eq!(theme.timings.double_click_ms, Some(400));
    assert_eq!(theme.layout.cursor_theme.as_deref(), Some("breeze_cursors"));
    assert_eq!(theme.layout.cursor_size_px, Some(24.0));
}

/// The resolved `kde-breeze` of both schemes, from its constants.
fn both_schemes() -> [ResolvedTheme; 2] {
    [ThemeContext::light(), ThemeContext::dark()]
        .map(|ctx| Theme::preset(Preset::KdeBreeze).resolve(&ctx))
}

/// A popover is rounded as qqc2-desktop-style's Popup rounds it,
/// `Kirigami.Units.cornerRadius` (5).
#[test]
fn a_popover_is_rounded_by_the_popup_s_radius() {
    for theme in both_schemes() {
        assert_eq!(theme.popover.panel.corners, Corners::uniform(5.0));
    }
}

#[test]
fn a_primary_button_keeps_the_accent_fill_when_pressed_checked_or_default() {
    for theme in both_schemes() {
        for state in [
            WidgetState::PRESSED,
            WidgetState::CHECKED,
            WidgetState::DEFAULT,
        ] {
            let query = ButtonQuery::primary().state(state);
            assert_eq!(
                theme.button.body(query).fill,
                Paint::Solid(theme.colors.accent.fill),
                "{:?} {state:?}",
                theme.context.scheme
            );
            assert_eq!(
                theme.button.label(query).color,
                theme.colors.accent.foreground,
                "{:?} {state:?}",
                theme.context.scheme
            );
            // A button of the default variant keeps its own fill in that state.
            assert_ne!(
                theme.button.body(ButtonQuery::new(state)).fill,
                Paint::Solid(theme.colors.accent.fill),
                "{:?} {state:?}",
                theme.context.scheme
            );
        }
        // Disabled, the primary button takes the disabled fill.
        let disabled = ButtonQuery::primary().state(WidgetState::DISABLED);
        assert_eq!(
            theme.button.body(disabled).fill,
            theme
                .button
                .body(ButtonQuery::new(WidgetState::DISABLED))
                .fill
        );
    }
}

#[test]
fn a_selected_row_has_no_hover_outline() {
    for theme in both_schemes() {
        let hover = theme.list.highlight(ListQuery::new(WidgetState::HOVER));
        let rest = theme.list.highlight(ListQuery::new(WidgetState::NONE));
        assert_ne!(hover.stroke, rest.stroke, "{:?}", theme.context.scheme);
        let selected = WidgetState::SELECTED;
        let hovered_selected = theme
            .list
            .highlight(ListQuery::new(selected.union(WidgetState::HOVER)));
        assert_eq!(
            hovered_selected.stroke,
            theme.list.highlight(ListQuery::new(selected)).stroke,
            "{:?}",
            theme.context.scheme
        );
        assert_eq!(
            hovered_selected.fill,
            Paint::Solid(lighter(
                theme.colors.selection.background,
                LIST_SELECTED_HOVER_LIGHTER
            ))
        );
    }
}

#[test]
fn pressed_changes_nothing_on_a_part_without_a_pressed_rule() {
    let pressed_hover = WidgetState::PRESSED.union(WidgetState::HOVER);
    for theme in both_schemes() {
        let scheme = theme.context.scheme;
        for checked in [WidgetState::NONE, WidgetState::CHECKED] {
            let hover = StyleQuery::<()>::new(WidgetState::HOVER.union(checked));
            let both = StyleQuery::<()>::new(pressed_hover.union(checked));
            assert_eq!(
                theme.radio.indicator(both),
                theme.radio.indicator(hover),
                "{scheme:?} {checked:?}"
            );
        }
        // The check box: checked, pressed changes nothing; unchecked, only the fill, Button
        // `darker(110)` (renderCheckBoxBackground, breezehelper.cpp:862 at breeze f0b1d75).
        let checked_hover = StyleQuery::<()>::new(WidgetState::HOVER | WidgetState::CHECKED);
        assert_eq!(
            theme
                .checkbox
                .indicator(StyleQuery::<()>::new(pressed_hover | WidgetState::CHECKED)),
            theme.checkbox.indicator(checked_hover),
            "{scheme:?}"
        );
        let hover = theme.checkbox.indicator(WidgetState::HOVER);
        let mut pressed = theme.checkbox.indicator(pressed_hover);
        let darker = match scheme {
            Scheme::Light => Color::from_srgb8(0xe5, 0xe5, 0xe5, 0xff),
            Scheme::Dark => Color::from_srgb8(0x25, 0x28, 0x2c, 0xff),
        };
        assert_eq!(pressed.fill, Paint::Solid(darker), "{scheme:?}");
        pressed.fill = hover.fill.clone();
        assert_eq!(pressed, hover, "{scheme:?}");
        for alternate in [false, true] {
            assert_eq!(
                theme
                    .list
                    .highlight(ListQuery::new(pressed_hover).alternate(alternate)),
                theme
                    .list
                    .highlight(ListQuery::new(WidgetState::HOVER).alternate(alternate)),
                "{scheme:?} alternate {alternate}"
            );
        }
        // Parts with a pressed rule keep pressed over hover.
        assert_eq!(
            theme.list.header(ListQuery::new(pressed_hover)),
            theme.list.header(ListQuery::new(WidgetState::PRESSED))
        );
        assert_eq!(
            theme.scrollbar.thumb(pressed_hover),
            theme.scrollbar.thumb(WidgetState::PRESSED)
        );
    }
}

#[test]
fn a_checked_toolbar_button_keeps_its_fill_and_takes_the_hover_outline() {
    for theme in both_schemes() {
        let scheme = theme.context.scheme;
        let checked = theme.toolbar.button(WidgetState::CHECKED);
        let hover = theme.toolbar.button(WidgetState::HOVER);
        assert_ne!(checked.stroke, hover.stroke, "{scheme:?}");
        for state in [WidgetState::HOVER, WidgetState::PRESSED] {
            let both = theme.toolbar.button(state.union(WidgetState::CHECKED));
            assert_eq!(both.fill, checked.fill, "{scheme:?} {state:?}");
            assert_eq!(both.stroke, hover.stroke, "{scheme:?} {state:?}");
        }
        let pressed_hover = WidgetState::PRESSED.union(WidgetState::HOVER);
        assert_eq!(
            theme.toolbar.button(pressed_hover),
            theme.toolbar.button(WidgetState::PRESSED),
            "{scheme:?}"
        );
    }
}

#[test]
fn a_hovered_selected_sidebar_item_is_the_selected_item() {
    for theme in both_schemes() {
        let scheme = theme.context.scheme;
        let selected = WidgetState::SELECTED;
        let hovered_selected = selected.union(WidgetState::HOVER);
        assert_ne!(
            theme.sidebar.item(WidgetState::HOVER),
            theme.sidebar.item(selected),
            "{scheme:?}"
        );
        assert_eq!(
            theme.sidebar.item(hovered_selected),
            theme.sidebar.item(selected),
            "{scheme:?}"
        );
        assert_eq!(
            theme.sidebar.item_label(hovered_selected),
            theme.sidebar.item_label(selected),
            "{scheme:?}"
        );
    }
}

#[test]
fn an_open_combo_box_ignores_the_pointer_and_a_disabled_one_both() {
    for theme in both_schemes() {
        let scheme = theme.context.scheme;
        let open = theme.combo_box.field(WidgetState::OPEN);
        assert_ne!(
            open,
            theme.combo_box.field(WidgetState::HOVER),
            "{scheme:?}"
        );
        assert_eq!(
            theme
                .combo_box
                .field(WidgetState::OPEN.union(WidgetState::HOVER)),
            open,
            "{scheme:?}"
        );
        let disabled = theme.combo_box.field(WidgetState::DISABLED);
        for state in [WidgetState::HOVER, WidgetState::OPEN] {
            assert_eq!(
                theme.combo_box.field(state.union(WidgetState::DISABLED)),
                disabled,
                "{scheme:?} {state:?}"
            );
        }
    }
}

#[test]
fn a_hovered_selected_segment_is_the_selected_segment() {
    for theme in both_schemes() {
        let scheme = theme.context.scheme;
        let selected = WidgetState::SELECTED;
        assert_ne!(
            theme.segmented_control.segment(WidgetState::HOVER),
            theme.segmented_control.segment(selected),
            "{scheme:?}"
        );
        assert_eq!(
            theme
                .segmented_control
                .segment(selected.union(WidgetState::HOVER)),
            theme.segmented_control.segment(selected),
            "{scheme:?}"
        );
    }
}

/// The colour of a solid fill.
fn solid(paint: &Paint) -> Option<Color> {
    match paint {
        Paint::Solid(color) => Some(*color),
        _ => None,
    }
}

/// The colour of a uniform outline.
fn outline_color(stroke: &Stroke) -> Option<Color> {
    match stroke {
        Stroke::Uniform { color, .. } => Some(*color),
        _ => None,
    }
}

/// Whether each channel of `got` is within one 8-bit step of the measured
/// pixel `measured`, `0xRRGGBB`.
fn near_pixel(got: Color, measured: u32) -> bool {
    let [_, r, g, b] = measured.to_be_bytes();
    let [got_r, got_g, got_b, _] = got.to_srgb8();
    [(r, got_r), (g, got_g), (b, got_b)]
        .iter()
        .all(|(m, g)| m.abs_diff(*g) <= 1)
}

const UNFOCUSED_HOVER: WidgetState = WidgetState::HOVER.union(WidgetState::UNFOCUSED_VIEW);

/// `L2`, a hovered row in a view without the focus, is derived: E5's hover
/// alphas, 0.3 for the fill and 0.8 for the outline, on the Inactive group's
/// Highlight, the inactive selection #c2e0f5 / #1b4155
/// (`selection_inactive_background`: platform-facts.md (§2.1.3)).
/// `docs/issues/0004_kde-leftovers-rationale.md` (L2) measures the row #ecf5fb /
/// #16232b and the outline's pixel #cce5f7 / #1a3a4b. The outline's pixel is
/// exact, the line lying half a pixel in over a half-covered edge pixel (E5).
/// The fill is within one 8-bit step: over the view the rule gives #edf6fc /
/// #16232a; that residual is not explained.
#[test]
fn a_hovered_row_in_a_view_without_the_focus_is_drawn_from_the_inactive_group() -> TestResult {
    let [light, dark] = both_schemes();
    for (theme, inactive_highlight, row_pixel, edge_pixel) in [
        (
            light,
            Color::from_srgb8(0xc2, 0xe0, 0xf5, 0xff),
            0x00ec_f5fb,
            [0xcc, 0xe5, 0xf7, 0xff],
        ),
        (
            dark,
            Color::from_srgb8(0x1b, 0x41, 0x55, 0xff),
            0x0016_232b,
            [0x1a, 0x3a, 0x4b, 0xff],
        ),
    ] {
        let scheme = theme.context.scheme;
        let row = theme.list.highlight(ListQuery::new(UNFOCUSED_HOVER));
        let fill = solid(&row.fill).ok_or("no fill")?;
        let stroke = outline_color(&row.stroke).ok_or("no outline")?;
        assert_eq!(
            fill.to_srgb8(),
            inactive_highlight.with_alpha(0.3).to_srgb8(),
            "{scheme:?}"
        );
        assert_eq!(
            stroke.to_srgb8(),
            inactive_highlight.with_alpha(0.8).to_srgb8(),
            "{scheme:?}"
        );
        let view = theme.colors.view.background.ok_or("no view background")?;
        assert!(
            near_pixel(fill.composite_over(view), row_pixel),
            "{scheme:?} {fill:?}"
        );
        let half_covered = fill.with_alpha(fill.a * 0.5).composite_over(view);
        assert_eq!(
            stroke.composite_over(half_covered).to_srgb8(),
            edge_pixel,
            "{scheme:?} {stroke:?}"
        );
    }
    Ok(())
}

/// `L2`'s precedences: over the hovered row, and under the selection, which
/// keeps no hover outline (the rationale, E5: "a hovered, unselected row").
#[test]
fn a_hovered_row_without_the_focus_is_over_the_hovered_row_and_under_the_selection() {
    for theme in both_schemes() {
        let scheme = theme.context.scheme;
        for alternate in [false, true] {
            let row = |state| {
                theme
                    .list
                    .highlight(ListQuery::new(state).alternate(alternate))
            };
            let (hover, unfocused) = (row(WidgetState::HOVER), row(UNFOCUSED_HOVER));
            assert_ne!(unfocused.fill, hover.fill, "{scheme:?} {alternate}");
            assert_ne!(unfocused.stroke, hover.stroke, "{scheme:?} {alternate}");
            // A selected row takes a selection's fill (`L3`) and no hover outline.
            let selected = row(WidgetState::SELECTED);
            let both = row(UNFOCUSED_HOVER.union(WidgetState::SELECTED));
            assert_ne!(both.fill, unfocused.fill, "{scheme:?} {alternate}");
            assert_eq!(both.stroke, selected.stroke, "{scheme:?} {alternate}");
        }
    }
}

/// `L2` on a colour scheme other than Breeze's: the hovered row of a view
/// without the focus takes that scheme's inactive selection colour at 0.3 and
/// at 0.8 (E5). The file states no inactive selection; KColorScheme builds it
/// from the file's Window and Selection colours, and 0.7.2's KDE reader gives
/// #6d3b0e on this file (`selection_inactive_background`).
#[cfg(all(target_os = "linux", feature = "kde"))]
#[test]
fn a_hovered_row_without_the_focus_follows_the_system_s_inactive_selection() -> TestResult {
    use crate::next::reader::kde::snapshot_from_kdeglobals;

    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/kde/custom-accent.ini");
    let mut ini = crate::kde::create_kde_parser();
    crate::kde::read_kconfig(&mut ini, &std::fs::read_to_string(path)?);
    let snapshot = snapshot_from_kdeglobals(&ini, None, None);
    let theme = Theme::preset(Preset::KdeBreeze)
        .with_system_overrides(snapshot.overrides)
        .resolve(&snapshot.context);
    let breeze = Theme::preset(Preset::KdeBreeze).resolve(&snapshot.context);
    let inactive = Color::from_srgb8(0x6d, 0x3b, 0x0e, 0xff);
    let row = theme.list.highlight(ListQuery::new(UNFOCUSED_HOVER));
    let fill = solid(&row.fill).ok_or("no fill")?;
    let stroke = outline_color(&row.stroke).ok_or("no outline")?;
    assert_eq!(fill.to_srgb8(), inactive.with_alpha(0.3).to_srgb8());
    assert_eq!(stroke.to_srgb8(), inactive.with_alpha(0.8).to_srgb8());
    // `S3` on the same file: the plain handle's fill, derived, in every view.
    let plain = theme.scrollbar.thumb(WidgetState::UNFOCUSED_VIEW);
    assert_ne!(
        plain.fill,
        breeze.scrollbar.thumb(WidgetState::UNFOCUSED_VIEW).fill
    );
    assert_eq!(theme.scrollbar.thumb(WidgetState::NONE).fill, plain.fill);
    // `S4` on the same file: derived from the file's window colours.
    let bar_hover = theme.scrollbar.thumb(BAR_HOVER_UNFOCUSED);
    assert!(bar_hover.stated.contains(PartStated::FILL));
    assert_ne!(bar_hover.fill, plain.fill);
    assert_ne!(
        bar_hover.fill,
        breeze.scrollbar.thumb(BAR_HOVER_UNFOCUSED).fill
    );
    Ok(())
}

/// `L4`, a row's own box: the highlight's radius,
/// `docs/issues/0004_kde-leftovers-rationale.md` (E5, L4): "Breeze rounds a
/// list's highlight 4.5 and draws a table's square", so a highlight at rest,
/// an alternate row's too, states no corners; the highlight's margins as
/// before (`list.border.padding`, platform-facts.md (§2.15)). The row itself
/// keeps the stripe and states neither margins nor corners nor the hover and
/// selection fills (spec §5, "The list's `row` and `highlight`").
#[test]
fn a_row_s_highlight_is_rounded_4_5_and_a_table_row_is_square() {
    for theme in both_schemes() {
        let scheme = theme.context.scheme;
        for alternate in [false, true] {
            let rest = theme
                .list
                .highlight(ListQuery::new(WidgetState::NONE).alternate(alternate));
            assert!(
                !rest.stated.contains(PartStated::CORNERS),
                "{scheme:?} {alternate}"
            );
            assert_eq!(rest.margin, Edges::symmetric(1.0, 2.0), "{scheme:?}");
        }
        for state in [
            WidgetState::HOVER,
            UNFOCUSED_HOVER,
            WidgetState::SELECTED,
            WidgetState::SELECTED.union(WidgetState::HOVER),
        ] {
            for alternate in [false, true] {
                let query = ListQuery::new(state).alternate(alternate);
                let highlight = theme.list.highlight(query);
                assert_eq!(
                    highlight.corners,
                    Corners::uniform(4.5),
                    "{scheme:?} {state:?}"
                );
                assert_eq!(
                    highlight.margin,
                    Edges::symmetric(1.0, 2.0),
                    "{scheme:?} {state:?}"
                );
                let row = theme.list.row(query);
                let rest = theme
                    .list
                    .row(ListQuery::new(WidgetState::NONE).alternate(alternate));
                assert_eq!(row, rest, "{scheme:?} {state:?} {alternate}");
                let own = PartStated::MARGIN_TOP | PartStated::CORNERS | PartStated::STROKE;
                assert!(
                    !row.stated.intersects(own),
                    "{scheme:?} {state:?} {alternate}"
                );
                assert_eq!(
                    row.stated.contains(PartStated::FILL),
                    alternate,
                    "{scheme:?} {state:?}"
                );
            }
        }
        for state in [
            WidgetState::NONE,
            WidgetState::HOVER,
            UNFOCUSED_HOVER,
            WidgetState::SELECTED,
        ] {
            let table = theme
                .list
                .highlight(ListQuery::from(ListVariant::Table).state(state));
            assert_eq!(table.corners, Corners::NONE, "{scheme:?} {state:?}");
            assert!(table.stated.contains(PartStated::CORNERS));
        }
    }
}

/// `S3`, the handle in the view that has the focus:
/// `docs/issues/0004_kde-leftovers-rationale.md` (S3) measures it #b1d9ef /
/// #2a546b; in a view without the focus, 0.7.2's plain handle, #cbcdcd /
/// #46494b (`scrollbar.thumb_color`: platform-facts.md (§2.8), measured). The
/// hovered and the pressed handle are over the view's focus.
#[test]
fn the_handle_of_the_focused_view_and_of_a_view_without_the_focus() {
    let [light, dark] = both_schemes();
    for (theme, focused, plain) in [
        (light, [0xb1, 0xd9, 0xef, 0xff], [0xcb, 0xcd, 0xcd, 0xff]),
        (dark, [0x2a, 0x54, 0x6b, 0xff], [0x46, 0x49, 0x4b, 0xff]),
    ] {
        let scheme = theme.context.scheme;
        let fill = |state| solid(&theme.scrollbar.thumb(state).fill).map(Color::to_srgb8);
        assert_eq!(fill(WidgetState::NONE), Some(focused), "{scheme:?}");
        assert_eq!(fill(WidgetState::UNFOCUSED_VIEW), Some(plain), "{scheme:?}");
        for state in [WidgetState::HOVER, WidgetState::PRESSED] {
            assert_eq!(
                theme
                    .scrollbar
                    .thumb(state.union(WidgetState::UNFOCUSED_VIEW)),
                theme.scrollbar.thumb(state),
                "{scheme:?} {state:?}"
            );
        }
    }
}

/// `S3` under the system's colours: Breeze's measured handle is no other
/// scheme's colour, and Breeze's rule, `focusColor()` through the idle
/// handle's overlay, does not reproduce it (#d0e4ef / #253b48 on Breeze's
/// colours), so the handle at rest is filled as the plain handle in every
/// view, as 0.7.2 draws it; its outline (`S2`) follows the view's focus, from
/// the system's DecorationFocus. 0.7.2's `scrollbar.thumb_color` is the
/// handle in a view without the focus, so that state's rule states the fill
/// (spec §2, `from_state`), and no rule the view with the focus. A system
/// without window colours states no `S4`: that query returns the plain
/// handle. No rule is dead.
#[test]
fn the_handle_at_rest_is_the_plain_handle_under_the_system_s_colours() {
    let theme = with_system(None);
    let [light, _] = both_schemes();
    let plain = light.scrollbar.thumb(WidgetState::UNFOCUSED_VIEW);
    for state in [
        WidgetState::NONE,
        WidgetState::UNFOCUSED_VIEW,
        WidgetState::HOVER_WITHIN,
        BAR_HOVER_UNFOCUSED,
    ] {
        let thumb = theme.scrollbar.thumb(state);
        assert_eq!(thumb.fill, plain.fill, "{state:?}");
        assert_eq!(
            thumb.from_state.contains(PartStated::FILL),
            state.contains(WidgetState::UNFOCUSED_VIEW),
            "{state:?}"
        );
    }
    assert_eq!(
        theme.scrollbar.thumb(WidgetState::NONE).stroke,
        Stroke::uniform_aligned(1.001, DECORATION_FOCUS.with_alpha(0.2), StrokeAlign::Center)
    );
    assert!(plain.stated.contains(PartStated::FILL));
    let dead: Vec<_> = theme
        .dead_rules()
        .into_iter()
        .filter(|(_, rules)| !rules.is_empty())
        .collect();
    assert!(dead.is_empty(), "{dead:?}");
}

const BAR_HOVER_UNFOCUSED: WidgetState =
    WidgetState::HOVER_WITHIN.union(WidgetState::UNFOCUSED_VIEW);

/// `S4`, the handle of a view without the focus with the pointer on the bar
/// but off the handle: the idle handle's overlay at the factor 1, #bcbebf /
/// #57595b (`scrollbar.thumb_color`: platform-facts.md (§2.8), computed). The
/// hovered and the pressed handle are over it; in the view that has the focus
/// no source states this state, and the query returns the focused view's
/// handle at rest.
#[test]
fn the_handle_with_the_pointer_on_the_bar_in_a_view_without_the_focus() {
    let [light, dark] = both_schemes();
    for (theme, bar_hover) in [
        (light, [0xbc, 0xbe, 0xbf, 0xff]),
        (dark, [0x57, 0x59, 0x5b, 0xff]),
    ] {
        let scheme = theme.context.scheme;
        let thumb = |state| theme.scrollbar.thumb(state);
        assert_eq!(
            solid(&thumb(BAR_HOVER_UNFOCUSED).fill).map(Color::to_srgb8),
            Some(bar_hover),
            "{scheme:?}"
        );
        for state in [WidgetState::HOVER, WidgetState::PRESSED] {
            assert_eq!(
                thumb(state.union(BAR_HOVER_UNFOCUSED)),
                thumb(state),
                "{scheme:?} {state:?}"
            );
        }
        assert_eq!(
            thumb(WidgetState::HOVER_WITHIN),
            thumb(WidgetState::NONE),
            "{scheme:?}"
        );
    }
}

const DECORATION_HOVER: Color = Color::rgb(1.0, 0.0, 1.0);

/// Breeze's light colours with the hover and the focus decorations told
/// apart (Breeze gives both #3daee9), so that the rule that wins shows, and
/// the roles and the globals of them.
fn distinct_decorations() -> (SchemeColors, ColorRoles, Typography, LayoutMetrics) {
    let mut c = LIGHT;
    c.decoration_hover = DECORATION_HOVER;
    c.accent = DECORATION_FOCUS;
    let colors = roles(&c, None, None, None, false);
    let typography = typography(&colors, None, ThemeContext::default().text_scale);
    let layout = Theme::preset(Preset::KdeBreeze)
        .resolve(&ThemeContext::light())
        .layout;
    (c, colors, typography, layout)
}

/// A text field's focus outline wins over its hover outline, and a disabled
/// field has no hover outline.
#[test]
fn a_text_field_s_focus_outline_wins_over_its_hover_outline() {
    let (c, colors, typography, layout) = distinct_decorations();
    let field = input(&c, &colors, &typography, &layout, Edges::all(0.0), None);
    let stroke = |state| field.field(state).stroke;
    assert_eq!(
        outline_color(&stroke(WidgetState::HOVER)),
        Some(DECORATION_HOVER)
    );
    assert_eq!(
        outline_color(&stroke(WidgetState::FOCUSED.union(WidgetState::HOVER))),
        Some(DECORATION_FOCUS)
    );
    assert_eq!(
        field.field(WidgetState::DISABLED.union(WidgetState::HOVER)),
        field.field(WidgetState::DISABLED)
    );
}

/// A hovered or pressed visited link takes the link colour, and a disabled
/// one the disabled colour, as 0.7.2's egui connector letters it.
#[test]
fn a_link_s_hover_pressed_and_disabled_colours_win_over_visited() {
    for theme in both_schemes() {
        let scheme = theme.context.scheme;
        let color = |state| theme.link.label(state).color;
        assert_eq!(theme.colors.link_visited.map(|_| ()), Some(()));
        assert_ne!(color(WidgetState::VISITED), color(WidgetState::NONE));
        for state in [WidgetState::HOVER, WidgetState::PRESSED] {
            assert_eq!(
                color(state.union(WidgetState::VISITED)),
                theme.colors.link,
                "{scheme:?} {state:?}"
            );
        }
        let disabled = WidgetState::DISABLED.union(WidgetState::VISITED);
        assert_eq!(
            color(disabled.union(WidgetState::HOVER)),
            color(WidgetState::DISABLED),
            "{scheme:?}"
        );
        assert_ne!(
            color(WidgetState::DISABLED),
            theme.colors.link,
            "{scheme:?}"
        );
    }
}

/// A slider handle's hover outline, which the pressed handle keeps, wins
/// over its focus outline.
#[test]
fn a_slider_handle_s_hover_outline_wins_over_its_focus_outline() {
    let (c, colors, ..) = distinct_decorations();
    let slider = slider(&c, &colors);
    let stroke = |state| outline_color(&slider.thumb(state).stroke);
    assert_eq!(stroke(WidgetState::FOCUSED), Some(DECORATION_FOCUS));
    for state in [WidgetState::HOVER, WidgetState::PRESSED] {
        assert_eq!(
            stroke(state.union(WidgetState::FOCUSED)),
            Some(DECORATION_HOVER),
            "{state:?}"
        );
    }
}

/// An open menu-bar title is filled over a hovered one.
#[test]
fn an_open_menu_bar_title_is_filled_over_a_hovered_one() {
    let (c, colors, ..) = distinct_decorations();
    let menu_bar = menu_bar(&c, &colors);
    let fill = |state| solid(&menu_bar.item(state).fill);
    assert_eq!(fill(WidgetState::HOVER), Some(DECORATION_HOVER));
    assert_eq!(
        fill(WidgetState::OPEN.union(WidgetState::HOVER)),
        Some(DECORATION_FOCUS)
    );
}

/// A pressed menu item, asked with `PRESSED` alone (spec §2, `from_state`),
/// is filled in the opaque focus colour and outlined in it at the hover
/// outline's width, by a rule for `PRESSED`, and so under the pointer too;
/// a hovered selected tab keeps the selected tab's fill.
#[test]
fn pressed_and_selected_fills_win_over_hover() {
    for theme in both_schemes() {
        let scheme = theme.context.scheme;
        let pressed = theme.menu.item(WidgetState::PRESSED);
        let hovered = theme.menu.item(WidgetState::HOVER);
        assert_ne!(pressed.fill, hovered.fill, "{scheme:?}");
        let hover_width = match hovered.stroke {
            Stroke::Uniform { width, .. } => Some(width),
            _ => None,
        };
        assert_eq!(
            Some(pressed.stroke.clone()),
            hover_width
                .zip(solid(&pressed.fill))
                .map(|(width, focus)| Stroke::uniform(width, focus)),
            "{scheme:?}"
        );
        assert!(
            pressed
                .from_state
                .contains(PartStated::FILL | PartStated::STROKE),
            "{scheme:?}"
        );
        assert_eq!(
            theme
                .menu
                .item(WidgetState::HOVER.union(WidgetState::PRESSED)),
            pressed,
            "{scheme:?}"
        );
        let selected = WidgetState::SELECTED;
        assert_eq!(
            theme.tab.tab(selected.union(WidgetState::HOVER)).fill,
            theme.tab.tab(selected).fill,
            "{scheme:?}"
        );
        assert_ne!(
            theme.tab.tab(WidgetState::HOVER).fill,
            theme.tab.tab(selected).fill,
            "{scheme:?}"
        );
    }
}

/// An inactive window has the inactive title bar and title colours and
/// nothing else changes; a maximized window takes the normal window's styles.
#[test]
fn an_inactive_window_has_the_inactive_title_bar() {
    use crate::next::window::WindowState;

    for ctx in [ThemeContext::light(), ThemeContext::dark()] {
        let scheme = ctx.scheme;
        let active = Theme::preset(Preset::KdeBreeze).resolve(&ctx);
        let inactive =
            Theme::preset(Preset::KdeBreeze).resolve(&ctx.clone().with_window_active(false));
        let normal = WindowState::default();
        let (scheme_colors, [bar, text]) = match scheme {
            Scheme::Light => (LIGHT, [0xeff0_f1ff_u32, 0x707d_8aff]),
            Scheme::Dark => (DARK, [0x2024_28ff, 0xa1a9_b1ff]),
        };
        assert_eq!(
            solid(&active.window.title_bar(normal).fill),
            Some(scheme_colors.title_bar_background),
            "{scheme:?}"
        );
        assert_eq!(
            solid(&inactive.window.title_bar(normal).fill),
            Some(srgb8(bar)),
            "{scheme:?}"
        );
        assert_eq!(
            inactive.window.title(normal).color,
            srgb8(text),
            "{scheme:?}"
        );
        assert_eq!(
            active.window.title(normal).color,
            active.typography.body.color,
            "{scheme:?}"
        );
        assert_eq!(
            inactive.window.frame(normal),
            active.window.frame(normal),
            "{scheme:?}"
        );
        let maximized = WindowState {
            maximized: true,
            ..WindowState::default()
        };
        assert_eq!(
            active.window.frame(maximized),
            active.window.frame(normal),
            "{scheme:?}"
        );
        assert_eq!(
            active.window.title_bar(maximized),
            active.window.title_bar(normal),
            "{scheme:?}"
        );
    }
}

const UNFOCUSED_SELECTED: WidgetState = WidgetState::SELECTED.union(WidgetState::UNFOCUSED_VIEW);

/// `L3`, a selected row under the pointer: Breeze fills it with
/// `Highlight.lighter(110)` (breezestyle.cpp:4876 at breeze f0b1d75), #44bfff,
/// measured (`docs/issues/0004_kde-leftovers-rationale.md`, L3); under the
/// system's colours, the system's selection colour through `qt_lighter`.
#[test]
fn a_hovered_selected_row_is_the_selection_lighter_110() {
    let query = ListQuery::new(WidgetState::SELECTED.union(WidgetState::HOVER));
    for theme in both_schemes() {
        assert_eq!(
            theme.list.highlight(query).fill,
            Paint::Solid(srgb8(0x44bf_ffff)),
            "{:?}",
            theme.context.scheme
        );
    }
    let theme = with_system(None);
    let [r, g, b, a] = qt_lighter(SELECTION.to_srgb8(), 110);
    let fill = theme.list.highlight(query).fill;
    assert_eq!(fill, Paint::Solid(Color::from_srgb8(r, g, b, a)));
    assert_ne!(fill, Paint::Solid(SELECTION));
}

/// A selected row in a view without the focus: such a view draws its rows
/// from the Inactive group (`docs/issues/0004_kde-leftovers-rationale.md`,
/// L2), so its selected row has the inactive selection's fill, #c2e0f5 /
/// #1b4155 (`selection_inactive_background`: platform-facts.md (§2.1.3)), and
/// text, `colors.selection`'s unfocused pair, no hover outline, and under the
/// pointer that fill `lighter(110)` (`L3`; derived, not measured).
#[test]
fn a_selected_row_in_a_view_without_the_focus_takes_the_inactive_selection() -> TestResult {
    let [light, dark] = both_schemes();
    for (theme, inactive) in [
        (light, [0xc2, 0xe0, 0xf5, 0xff]),
        (dark, [0x1b, 0x41, 0x55, 0xff]),
    ] {
        let scheme = theme.context.scheme;
        let selection = theme.colors.selection;
        let background = selection
            .unfocused_background
            .ok_or("no unfocused selection")?;
        let foreground = selection
            .unfocused_foreground
            .ok_or("no unfocused selection text")?;
        assert_eq!(background.to_srgb8(), inactive, "{scheme:?}");
        for alternate in [false, true] {
            let query = |state| ListQuery::new(state).alternate(alternate);
            let row = theme.list.highlight(query(UNFOCUSED_SELECTED));
            assert_eq!(row.fill, Paint::Solid(background), "{scheme:?}");
            assert_eq!(
                row.stroke,
                theme.list.highlight(query(WidgetState::SELECTED)).stroke,
                "{scheme:?}"
            );
            assert_eq!(
                theme
                    .list
                    .highlight(query(UNFOCUSED_SELECTED.union(WidgetState::HOVER)))
                    .fill,
                Paint::Solid(lighter(background, LIST_SELECTED_HOVER_LIGHTER)),
                "{scheme:?}"
            );
            assert_eq!(
                theme.list.item_label(query(UNFOCUSED_SELECTED)).color,
                foreground,
                "{scheme:?}"
            );
            assert_eq!(
                theme.list.item_label(query(WidgetState::SELECTED)).color,
                selection.foreground,
                "{scheme:?}"
            );
        }
    }
    Ok(())
}

/// `S1`, the groove's line along its inner edge: `Text` at alpha
/// frameContrast, #23262933 / #fcfcfc33, 1 px, inside the groove, on its top
/// row when horizontal in both directions, on its left column when vertical
/// from left to right and on its right column from right to left
/// (`Style::drawScrollBarComplexControl`, breezestyle.cpp:8213-8232 at breeze
/// f0b1d75; `QStyle::alignedRect`, qstyle.cpp:2180-2212 at qtbase v6.11.0).
#[test]
fn the_groove_s_line_is_on_the_bar_s_own_side_for_each_orientation_and_direction() {
    for (scheme, text) in [(Scheme::Light, 0x2326_2933), (Scheme::Dark, 0xfcfc_fc33)] {
        for direction in [Direction::Ltr, Direction::Rtl] {
            let ctx = ThemeContext::default()
                .with_scheme(scheme)
                .with_direction(direction);
            let theme = Theme::preset(Preset::KdeBreeze).resolve(&ctx);
            let line = StrokeSide {
                width: 1.0,
                paint: Paint::Solid(srgb8(text)),
            };
            let (left, right) = match direction {
                Direction::Ltr => (line.clone(), StrokeSide::default()),
                Direction::Rtl => (StrokeSide::default(), line.clone()),
            };
            let track = |orientation| {
                theme
                    .scrollbar
                    .track(StyleQuery::new(WidgetState::NONE).orientation(orientation))
                    .stroke
            };
            assert_eq!(
                track(Orientation::Vertical),
                Stroke::sides(
                    StrokeSide::default(),
                    right,
                    StrokeSide::default(),
                    left,
                    StrokeAlign::Inside
                ),
                "{scheme:?} {direction:?}"
            );
            assert_eq!(
                track(Orientation::Horizontal),
                Stroke::sides(
                    line,
                    StrokeSide::default(),
                    StrokeSide::default(),
                    StrokeSide::default(),
                    StrokeAlign::Inside
                ),
                "{scheme:?} {direction:?}"
            );
        }
    }
}

/// `S2`, the handle's outline: a `PenWidth::Frame` pen of 1.001 px centred on
/// the handle's edge in the handle's own colour at alpha frameContrast
/// (`Helper::renderScrollBarHandle`, breezehelper.cpp:1274-1287, and
/// `scrollBarHandleColor`, :277-303 at breeze f0b1d75): WindowText,
/// #23262933 / #fcfcfc33, in a view without the focus, with the pointer on
/// the bar too; DecorationFocus, #3daee933, in the view that has it;
/// DecorationHover, #3daee933, hovered and pressed in every view.
#[test]
fn the_handle_s_outline_is_its_own_colour_at_frame_contrast() {
    let [light, dark] = both_schemes();
    for (theme, text) in [(light, 0x2326_2933), (dark, 0xfcfc_fc33)] {
        let scheme = theme.context.scheme;
        let pen = |rgba| Stroke::uniform_aligned(1.001, srgb8(rgba), StrokeAlign::Center);
        let stroke = |state| theme.scrollbar.thumb(state).stroke;
        assert_eq!(stroke(WidgetState::NONE), pen(0x3dae_e933), "{scheme:?}");
        for state in [WidgetState::UNFOCUSED_VIEW, BAR_HOVER_UNFOCUSED] {
            assert_eq!(stroke(state), pen(text), "{scheme:?} {state:?}");
        }
        for state in [WidgetState::HOVER, WidgetState::PRESSED] {
            for view in [WidgetState::NONE, WidgetState::UNFOCUSED_VIEW] {
                assert_eq!(
                    stroke(state.union(view)),
                    pen(0x3dae_e933),
                    "{scheme:?} {state:?} {view:?}"
                );
            }
        }
    }
}

/// The handle is a pill in every state (breezehelper.cpp:1274-1287 at breeze
/// f0b1d75), and the progress bar groove's outline is centred on its fill
/// (breezehelper.cpp:1215).
#[test]
fn the_handle_is_a_pill_and_the_progress_groove_s_outline_is_centred() {
    for theme in both_schemes() {
        for state in [
            WidgetState::NONE,
            WidgetState::UNFOCUSED_VIEW,
            BAR_HOVER_UNFOCUSED,
            WidgetState::HOVER,
            WidgetState::PRESSED,
        ] {
            assert_eq!(
                theme.scrollbar.thumb(state).corners,
                Corners::pill(),
                "{state:?}"
            );
        }
        let track = theme.progress_bar.track(WidgetState::NONE);
        assert!(
            matches!(
                track.stroke,
                Stroke::Uniform {
                    align: StrokeAlign::Center,
                    ..
                }
            ),
            "{:?}",
            track.stroke
        );
    }
}

/// `L4`, a row's padding: a text-only row's content inset from the row's
/// rectangle, 7 left and right (the highlight's margin 2, Breeze's
/// `ItemView_ItemPaddingWidth` 2 and Qt's text margin 3) and 3 above and
/// below (the margin 1 and `ItemView_ItemPaddingHeight` 2)
/// (breezehelper.cpp:1759-1799, breezemetrics.h:157-165, breezestyle.cpp:1060-1084
/// at breeze f0b1d75; qcommonstyle.cpp:1002-1007 at qtbase v6.11.0).
#[test]
fn a_row_s_content_is_inset_by_the_margin_breeze_s_padding_and_qt_s_text_margin() {
    for theme in both_schemes() {
        for alternate in [false, true] {
            let row = theme
                .list
                .row(ListQuery::new(WidgetState::NONE).alternate(alternate));
            assert_eq!(row.padding, Edges::symmetric(3.0, 7.0), "{alternate}");
            assert!(row.stated.contains(
                PartStated::PADDING_TOP
                    | PartStated::PADDING_RIGHT
                    | PartStated::PADDING_BOTTOM
                    | PartStated::PADDING_LEFT
            ));
        }
    }
}

/// `Z2`: no source gives KDE's dialog size limits (spec §5, "Issue #4's
/// leftovers and their homes"), so the panel states neither.
#[test]
fn the_dialog_states_no_size_limits() {
    for theme in both_schemes() {
        assert_eq!(theme.dialog.panel.min_size, [None, None]);
        assert_eq!(theme.dialog.panel.max_size, [None, None]);
    }
}

/// `stroke` with its left and right sides swapped, where it has sides.
fn mirrored_stroke(stroke: Stroke) -> Stroke {
    match stroke {
        Stroke::Complex(sides) => Stroke::sides(
            sides.top.clone(),
            sides.left.clone(),
            sides.bottom.clone(),
            sides.right.clone(),
            sides.align,
        ),
        other => other,
    }
}

/// Right to left: the theme from left to right with every per-side value
/// mirrored, the menu item's, the status bar's and the combo box's padding
/// (stated for a left-to-right layout) and the vertical scroll bar's line
/// (`S1`), in every state; nothing else differs.
#[test]
fn right_to_left_is_left_to_right_mirrored() {
    for scheme in [Scheme::Light, Scheme::Dark] {
        let ctx = ThemeContext::default().with_scheme(scheme);
        let ltr = Theme::preset(Preset::KdeBreeze).resolve(&ctx);
        let rtl = Theme::preset(Preset::KdeBreeze).resolve(&ctx.with_direction(Direction::Rtl));
        let padded = |part: PartStyle| PartStyle {
            padding: physical_edges(part.padding, Direction::Rtl),
            ..part
        };
        for state in [
            WidgetState::NONE,
            WidgetState::HOVER,
            WidgetState::PRESSED,
            WidgetState::DISABLED,
        ] {
            assert_eq!(
                rtl.menu.item(state),
                padded(ltr.menu.item(state)),
                "{scheme:?} {state:?}"
            );
        }
        for state in [
            WidgetState::NONE,
            WidgetState::HOVER,
            WidgetState::OPEN,
            WidgetState::DISABLED,
        ] {
            assert_eq!(
                rtl.combo_box.field(state),
                padded(ltr.combo_box.field(state)),
                "{scheme:?} {state:?}"
            );
        }
        assert_eq!(rtl.status_bar.bar, padded(ltr.status_bar.bar.clone()));
        for state in [
            WidgetState::NONE,
            WidgetState::HOVER,
            WidgetState::UNFOCUSED_VIEW,
        ] {
            let query = |orientation| StyleQuery::new(state).orientation(orientation);
            let vertical = ltr.scrollbar.track(query(Orientation::Vertical));
            assert_eq!(
                rtl.scrollbar.track(query(Orientation::Vertical)),
                PartStyle {
                    stroke: mirrored_stroke(vertical.stroke.clone()),
                    ..vertical
                },
                "{scheme:?} {state:?}"
            );
            assert_eq!(
                rtl.scrollbar.track(query(Orientation::Horizontal)),
                ltr.scrollbar.track(query(Orientation::Horizontal)),
                "{scheme:?} {state:?}"
            );
            for orientation in [Orientation::Horizontal, Orientation::Vertical] {
                assert_eq!(
                    rtl.scrollbar.thumb(query(orientation)),
                    ltr.scrollbar.thumb(query(orientation)),
                    "{scheme:?} {state:?} {orientation:?}"
                );
            }
        }
        assert_ne!(rtl.menu, ltr.menu, "{scheme:?}");
        assert_ne!(rtl.combo_box, ltr.combo_box, "{scheme:?}");
        assert_ne!(rtl.status_bar, ltr.status_bar, "{scheme:?}");
        assert_ne!(rtl.scrollbar, ltr.scrollbar, "{scheme:?}");
        let mut rest = ltr.clone();
        rest.context = rtl.context.clone();
        rest.menu = rtl.menu.clone();
        rest.combo_box = rtl.combo_box.clone();
        rest.status_bar = rtl.status_bar.clone();
        rest.scrollbar = rtl.scrollbar.clone();
        assert_eq!(rest, rtl, "{scheme:?}");
    }
}

/// The text scale of the test below.
const TEXT_SCALE_TWO: f32 = 2.0;

/// The text scale is linear on KDE (spec §6, resolution step 2): at 2.0 every
/// font size is the size converted at the font DPI times 2, then rounded as
/// Qt rounds it, so the 10 pt body is 27 px (26.67 rounded), not twice the
/// rounded 13 px; every line height is twice the one at 1.0; each
/// `defined_size` stays the unscaled size; nothing but the text changes.
#[test]
fn at_text_scale_2_every_font_size_is_doubled_before_qt_s_rounding() -> TestResult {
    use crate::next::window::WindowState;

    for scheme in [Scheme::Light, Scheme::Dark] {
        let ctx = ThemeContext::default().with_scheme(scheme);
        let base = Theme::preset(Preset::KdeBreeze).resolve(&ctx);
        let double = Theme::preset(Preset::KdeBreeze).resolve(&ctx.with_text_scale(TEXT_SCALE_TWO));
        assert_eq!(base.typography.body.size_px, 13.0, "{scheme:?}");
        assert_eq!(double.typography.body.size_px, 27.0, "{scheme:?}");
        let slots = |t: &ResolvedTheme| -> Result<Vec<TextStyle>, String> {
            let entry = |slot: &Option<TextStyle>, name: &str| {
                slot.clone().ok_or_else(|| format!("no {name}"))
            };
            Ok(vec![
                t.typography.body.clone(),
                t.typography.monospace.clone(),
                entry(&t.typography.title1, "title1")?,
                entry(&t.typography.headline, "headline")?,
                entry(&t.typography.caption, "caption")?,
                t.button.label(ButtonQuery::new(WidgetState::NONE)),
                t.menu.item_label(WidgetState::NONE),
                t.window.title(WindowState::default()),
            ])
        };
        for (one, two) in slots(&base)?.into_iter().zip(slots(&double)?) {
            let size = one.defined_size.ok_or("no defined size")?;
            assert_eq!(two.defined_size, Some(size), "{scheme:?}");
            assert_eq!(
                two.size_px,
                qt_whole_px(size.to_logical_px(FONT_DPI) * TEXT_SCALE_TWO),
                "{scheme:?} {size:?}"
            );
            assert_eq!(
                two.line_height,
                one.line_height * TEXT_SCALE_TWO,
                "{scheme:?} {size:?}"
            );
        }
        let rest = ButtonQuery::new(WidgetState::NONE);
        assert_eq!(
            double.button.body(rest),
            base.button.body(rest),
            "{scheme:?}"
        );
        let row = ListQuery::new(WidgetState::NONE);
        assert_eq!(double.list.row(row), base.list.row(row), "{scheme:?}");
        assert_eq!(double.layout, base.layout, "{scheme:?}");
        assert_eq!(double.icons, base.icons, "{scheme:?}");
    }
    Ok(())
}

/// Motion: `MotionPreference::None`, or the system's animation scale
/// (KDE's `AnimationDurationFactor`) at 0, sets the dialog's entrance to
/// `DialogEntranceStyle::None` (spec §6, resolution step 2); Kirigami's
/// `FadeRise` stays under full motion, under a scale above 0 and under
/// `MotionPreference::Reduced`, for which KDE states no policy. The preset
/// states no motion token, under any motion.
#[test]
fn no_motion_or_an_animation_scale_of_0_removes_the_dialog_s_entrance() {
    let resolve = |motion: MotionPreference, animation_scale: Option<f32>| {
        Theme::preset(Preset::KdeBreeze)
            .with_system_overrides(SystemOverrides {
                animation_scale,
                ..SystemOverrides::default()
            })
            .resolve(&ThemeContext::light().with_motion(motion))
    };
    for (motion, scale) in [
        (MotionPreference::None, None),
        (MotionPreference::Full, Some(0.0)),
        (MotionPreference::Reduced, Some(0.0)),
    ] {
        let theme = resolve(motion, scale);
        assert_eq!(
            theme.dialog.entrance,
            Some(DialogEntranceStyle::None),
            "{motion:?} {scale:?}"
        );
        assert_eq!(theme.motion_tokens, MotionTokens::default(), "{motion:?}");
    }
    for (motion, scale) in [
        (MotionPreference::Full, None),
        (MotionPreference::Full, Some(0.5)),
        (MotionPreference::Reduced, None),
    ] {
        let theme = resolve(motion, scale);
        assert_eq!(
            theme.dialog.entrance,
            Some(DialogEntranceStyle::FadeRise),
            "{motion:?} {scale:?}"
        );
        assert_eq!(theme.motion_tokens, MotionTokens::default(), "{motion:?}");
    }
}

// ---------------------------------------------------------------------------
// The palette and the explicit accent (spec §8, *Palette*, *Explicit accent*)
// ---------------------------------------------------------------------------

/// The 8-bit opaque colour `0xRRGGBB`.
fn hex(rgb: u32) -> Color {
    let [_, r, g, b] = rgb.to_be_bytes();
    Color::from_srgb8(r, g, b, u8::MAX)
}

/// A light palette whose every slot has a colour of its own.
fn distinct_palette() -> Palette {
    Palette {
        name: Arc::from("distinct"),
        scheme: Scheme::Light,
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

/// The colours of `scheme` with `palette` and `accent` laid over them.
fn layered(scheme: SchemeColors, palette: Option<&Palette>, accent: Option<Color>) -> SchemeColors {
    let mut c = scheme;
    lay_layer_3_over(&mut c, palette, accent);
    c
}

/// A colour of [`SchemeColors`] with its name.
type Field = (&'static str, fn(&SchemeColors) -> Color);

/// The colours no slot and no accent setting replaces.
const KEPT: &[Field] = &[
    ("button_background", |c| c.button_background),
    ("button_foreground", |c| c.button_foreground),
    ("button_checked", |c| c.button_checked),
    ("button_frame", |c| c.button_frame),
    ("switch_frame", |c| c.switch_frame),
    ("switch_off", |c| c.switch_off),
    ("toolbar_button_checked", |c| c.toolbar_button_checked),
    ("list_alternate_row", |c| c.list_alternate_row),
    ("disabled_text", |c| c.disabled_text),
    ("shadow", |c| c.shadow),
    ("button_disabled_background", |c| {
        c.button_disabled_background
    }),
    ("button_disabled_foreground", |c| {
        c.button_disabled_foreground
    }),
    ("checkbox_disabled_checked", |c| c.checkbox_disabled_checked),
    ("checkbox_disabled_foreground", |c| {
        c.checkbox_disabled_foreground
    }),
    ("view_disabled_background", |c| c.view_disabled_background),
    ("view_disabled_foreground", |c| c.view_disabled_foreground),
    ("link_disabled", |c| c.link_disabled),
    ("slider_disabled", |c| c.slider_disabled),
    ("switch_disabled_off", |c| c.switch_disabled_off),
    ("switch_disabled_on", |c| c.switch_disabled_on),
    ("switch_disabled_frame", |c| c.switch_disabled_frame),
    ("switch_disabled_on_frame", |c| c.switch_disabled_on_frame),
    ("scrollbar_track", |c| c.scrollbar_track),
    ("tab_background", |c| c.tab_background),
    ("tab_selected_background", |c| c.tab_selected_background),
    ("tab_text", |c| c.tab_text),
    ("tab_frame", |c| c.tab_frame),
    ("sidebar_hover", |c| c.sidebar_hover),
    ("segment_hover", |c| c.segment_hover),
    ("expander_arrow", |c| c.expander_arrow),
];

/// The names of the [`KEPT`] colours of `got` that differ from `want`'s.
fn moved(got: &SchemeColors, want: &SchemeColors) -> Vec<&'static str> {
    KEPT.iter()
        .filter(|(_, field)| field(got) != field(want))
        .map(|(name, _)| *name)
        .collect()
}

/// Plasma's accent run (rc1 `docs/facts/kde/accent.toml`, run 41158393408,
/// tolerance 0): over Breeze's own scheme, the accent, then the selection's
/// background and foreground, the link and DecorationFocus, DecorationHover
/// and ForegroundActive Plasma wrote.
const MEASURED_ACCENT_RUN: [(Scheme, u32, u32, u32, u32, u32); 4] = [
    (
        Scheme::Light,
        0xe5a50a,
        0xecbf53,
        0x000000,
        0xa77807,
        0xe5a50a,
    ),
    (
        Scheme::Light,
        0x5e2ca5,
        0x8e6bbf,
        0x000000,
        0x5e2ca5,
        0x5e2ca5,
    ),
    (
        Scheme::Dark,
        0xe5a50a,
        0xa67a0e,
        0x000000,
        0xe5a50a,
        0xe5a50a,
    ),
    (
        Scheme::Dark,
        0x5e2ca5,
        0x47257a,
        0xffffff,
        0xc7b8ed,
        0x5e2ca5,
    ),
];

fn context(scheme: Scheme) -> ThemeContext {
    ThemeContext::default().with_scheme(scheme)
}

/// An explicit accent over Breeze gives Plasma's measured colours.
#[test]
fn an_explicit_accent_writes_plasma_s_measured_colours() {
    for (scheme, accent, selection, text, link, decoration) in MEASURED_ACCENT_RUN {
        let theme = Theme::preset(Preset::KdeBreeze)
            .with_accent(hex(accent))
            .resolve(&context(scheme));
        let colors = &theme.colors;
        let what = format!("{scheme:?} #{accent:06x}");
        assert_eq!(colors.selection.background, hex(selection), "{what}");
        assert_eq!(colors.selection.foreground, hex(text), "{what}");
        assert_eq!(colors.link, hex(link), "{what}");
        assert_eq!(colors.accent.fill, hex(decoration), "{what}");
        assert_eq!(colors.info.fill, hex(decoration), "{what}");
        assert_eq!(
            solid(&theme.menu_bar.item(WidgetState::HOVER).fill),
            Some(hex(decoration)),
            "{what}: DecorationHover"
        );
        // The accent's text is the selection's, the caret DecorationFocus.
        assert_eq!(colors.accent.foreground, hex(text), "{what}");
        assert_eq!(colors.caret, hex(decoration), "{what}");
        assert_eq!(colors.text_selection.background, hex(selection), "{what}");
        assert_eq!(colors.focus_ring, focus_ring_of(hex(selection)), "{what}");
        assert_eq!(colors.info.origin, RoleOrigin::Native, "{what}");
    }
}

/// A derived colour that reads an accent setting follows the accent; one that
/// reads none keeps Breeze's colour, and `S3` is unstated.
#[test]
fn an_explicit_accent_moves_only_what_reads_its_settings() {
    let (_, accent, selection, ..) = MEASURED_ACCENT_RUN[0];
    let (accent, selection) = (hex(accent), hex(selection));
    let c = layered(LIGHT, None, Some(accent));
    assert_eq!(c.menu_hover, translucent(accent, BLEND_VALUE));
    assert_eq!(c.tab_hover, translucent(accent, TAB_HOVER_ALPHA));
    assert_eq!(
        c.button_pressed,
        mix(LIGHT.button_background, selection, PRESSED_BUTTON_MIX)
    );
    assert_eq!(c.focus_ring, focus_ring_of(selection));
    assert_ne!(c.list_row_hover, LIGHT.list_row_hover);
    assert_eq!(c.list_grid, LIGHT.list_grid);
    assert_eq!(c.scrollbar_thumb, LIGHT.scrollbar_thumb);
    assert_eq!(c.checkbox_frame, LIGHT.checkbox_frame);
    assert_eq!(c.toolbar_border, LIGHT.toolbar_border);
    assert_eq!(c.title_bar_background, LIGHT.title_bar_background);
    assert_eq!(c.scrollbar_thumb_focused, None);
    assert_eq!(moved(&c, &LIGHT), Vec::<&str>::new());
}

/// `#00000000` is Plasma's "accent from the colour scheme": no accent.
#[test]
fn a_fully_transparent_accent_is_no_accent() {
    for scheme in [Scheme::Light, Scheme::Dark] {
        let plain = Theme::preset(Preset::KdeBreeze).resolve(&context(scheme));
        let zero = Theme::preset(Preset::KdeBreeze)
            .with_accent(Color::new(0.0, 0.0, 0.0, 0.0))
            .resolve(&context(scheme));
        assert!(zero == plain, "{scheme:?}");
    }
}

/// A palette replaces the settings table A names and the colours derived
/// from them, and nothing else.
#[test]
fn a_palette_replaces_the_settings_its_slots_name() {
    let p = distinct_palette();
    let c = layered(LIGHT, Some(&p), None);
    for (name, got, want) in [
        ("window background", c.window_background, p.window_bg),
        ("view background", c.view_background, p.view_bg),
        ("tooltip background", c.tooltip_background, p.tooltip_bg),
        ("header background", c.toolbar_background, p.header_bg),
        ("[WM] activeBackground", c.title_bar_background, p.header_bg),
        (
            "[WM] inactiveBackground",
            c.title_bar_inactive_background,
            p.header_bg,
        ),
        ("window text", c.window_foreground, p.foreground),
        ("view text", c.view_foreground, p.foreground),
        ("tooltip text", c.tooltip_foreground, p.foreground),
        (
            "[WM] inactiveForeground",
            c.title_bar_inactive_foreground,
            p.foreground,
        ),
        (
            "window muted",
            c.window_foreground_muted,
            p.foreground_muted,
        ),
        ("placeholder", c.placeholder, p.foreground_muted),
        ("selection", c.selection_background, p.selection_bg),
        ("selection text", c.selection_foreground, p.selection_fg),
        ("DecorationFocus", c.accent, p.accent),
        ("DecorationHover", c.decoration_hover, p.accent),
        ("link", c.link, p.link),
        ("negative", c.error, p.error),
        ("neutral", c.warning, p.warning),
        ("positive", c.success, p.success),
        ("active", c.info, p.info),
        ("border", c.window_border, p.border),
    ] {
        assert_eq!(got, want, "{name}");
    }
    assert_eq!(c.title_bar_foreground, Some(p.foreground));
    assert_eq!(c.view_foreground_muted, Some(p.foreground_muted));
    assert_eq!(c.caret, Some(p.caret));
    assert_eq!(c.scrollbar_thumb_focused, None);
    // Derived again from the settings in force.
    assert_eq!(
        c.checkbox_frame,
        mix(p.window_bg, p.foreground, LIGHT.frame_contrast)
    );
    assert_eq!(
        c.menu_background,
        mix(p.window_bg, p.view_bg, MENU_PANEL_MIX)
    );
    assert_eq!(
        c.toolbar_border,
        mix(p.header_bg, p.foreground, LIGHT.frame_contrast)
    );
    assert_eq!(c.focus_ring, focus_ring_of(p.selection_bg));
    assert_eq!(
        c.button_default,
        mix(LIGHT.button_background, p.selection_bg, DEFAULT_BUTTON_MIX)
    );
    // The Button set and the colours the 0.7.2 reader never replaces stay.
    assert_eq!(moved(&c, &LIGHT), Vec::<&str>::new());
}

/// A palette states the roles its slots name (spec §7), and the widgets of
/// those names draw them; text on a fill is the fill's pair.
#[test]
fn a_palette_states_its_roles_and_their_widgets() {
    let p = distinct_palette();
    let theme = Theme::preset(Preset::KdeBreeze)
        .with_palette(p.clone())
        .resolve(&ThemeContext::light());
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
        assert_eq!(surface.background, Some(background), "{name}");
        assert_eq!(surface.foreground, Some(p.foreground), "{name}");
    }
    assert_eq!(colors.window.foreground_muted, Some(p.foreground_muted));
    assert_eq!(colors.window.border, Some(p.border));
    assert_eq!(colors.window.divider, Some(p.divider));
    assert_eq!(colors.disabled_text, p.foreground_muted);
    assert_eq!(colors.accent.fill, p.accent);
    assert_eq!(colors.accent.foreground, p.accent_fg);
    assert_eq!(colors.selection.background, p.selection_bg);
    assert_eq!(colors.selection.foreground, p.selection_fg);
    assert_eq!(colors.text_selection.background, p.selection_bg);
    assert_eq!(colors.text_selection.foreground, p.selection_fg);
    assert_eq!(colors.focus_ring, focus_ring_of(p.selection_bg));
    assert_eq!(colors.link, p.link);
    assert_eq!(colors.caret, p.caret);
    for (status, fill) in [
        (colors.error, p.error),
        (colors.warning, p.warning),
        (colors.success, p.success),
        (colors.info, p.info),
    ] {
        assert_eq!(status.fill, fill);
        assert_eq!(status.origin, RoleOrigin::Custom);
    }
    let text = |style: TextStyle| style.color;
    assert_eq!(solid(&theme.dialog.panel.fill), Some(p.surface_bg));
    assert_eq!(solid(&theme.popover.panel.fill), Some(p.surface_bg));
    assert_eq!(
        solid(&theme.card.container(WidgetState::NONE).fill),
        Some(p.surface_bg)
    );
    assert_eq!(
        solid(&theme.sidebar.container(WidgetState::NONE).fill),
        Some(p.sidebar_bg)
    );
    assert_eq!(theme.separator.color, p.border);
    assert_eq!(
        text(theme.menu.item_label(WidgetState::PRESSED)),
        p.accent_fg
    );
    assert_eq!(
        text(theme.menu_bar.item_label(WidgetState::OPEN)),
        p.accent_fg
    );
    assert_eq!(
        text(theme.segmented_control.label(WidgetState::SELECTED)),
        p.selection_fg
    );
    assert_eq!(
        text(theme.button.label(ButtonQuery::primary())),
        p.accent_fg
    );
}

/// Under a palette the system's accent, focus ring and text selection are not
/// applied: the palette, the later layer, restates those roles.
#[test]
fn a_palette_stands_over_the_system_s_accent() {
    let palette = |system: SystemOverrides| {
        Theme::preset(Preset::KdeBreeze)
            .with_system_overrides(system)
            .with_palette(distinct_palette())
            .resolve(&ThemeContext::light())
    };
    let without = palette(system(None));
    for system in [
        system(Some(PORTAL_ACCENT)),
        system_with(|os| os.accent = Some(PORTAL_ACCENT)),
        system_with(|os| os.focus_ring = Some(PORTAL_ACCENT)),
        system_with(|os| {
            os.text_selection = Some(OsSelectionColors::new(
                Some(PORTAL_ACCENT),
                Some(PORTAL_ACCENT),
                None,
                None,
            ))
        }),
    ] {
        assert!(palette(system) == without);
    }
}

/// An explicit accent over a palette is written over the palette's view
/// background and replaces its selection, link and information colour; the
/// caret stays the palette's, and the accent's text is the selection's.
#[test]
fn an_explicit_accent_over_a_palette() {
    // Over Breeze Dark's view background, Plasma's measured colours for it.
    let (_, accent, selection, text, link, decoration) = MEASURED_ACCENT_RUN[2];
    let p = Palette {
        view_bg: DARK.view_background,
        ..distinct_palette()
    };
    let c = layered(LIGHT, Some(&p), Some(hex(accent)));
    assert_eq!(c.selection_background, hex(selection));
    assert_eq!(c.selection_foreground, hex(text));
    assert_eq!(c.link, hex(link));
    assert_eq!(c.info, hex(decoration));
    assert_eq!(c.accent, hex(decoration));
    assert_eq!(c.caret, Some(p.caret));
    assert_eq!(c.window_background, p.window_bg);
    let theme = Theme::preset(Preset::KdeBreeze)
        .with_palette(p.clone())
        .with_accent(hex(accent))
        .resolve(&ThemeContext::light());
    assert_eq!(theme.colors.accent.foreground, hex(text));
    assert_eq!(theme.colors.caret, p.caret);
    assert_eq!(theme.colors.info.origin, RoleOrigin::Custom);
}

/// Each derived colour computed again on Breeze's own settings, against the
/// preset's constant: the ones that do not reproduce it (spec §8, *Palette*,
/// *Derived values*: a derivation on Breeze's own settings need not give the
/// measured constant). `toolbar_border` is not computed: the preset states
/// no `[Colors:Header] ForegroundNormal`.
#[test]
fn derivations_on_breeze_s_own_settings() {
    const DERIVED: &[Field] = &[
        ("focus_ring", |c| c.focus_ring),
        ("selection_unfocused_background", |c| {
            c.selection_unfocused_background
        }),
        ("selection_unfocused_foreground", |c| {
            c.selection_unfocused_foreground
        }),
        ("button_checked", |c| c.button_checked),
        ("button_pressed", |c| c.button_pressed),
        ("button_default", |c| c.button_default),
        ("button_default_frame", |c| c.button_default_frame),
        ("button_frame", |c| c.button_frame),
        ("checkbox_checked", |c| c.checkbox_checked),
        ("checkbox_frame", |c| c.checkbox_frame),
        ("list_grid", |c| c.list_grid),
        ("list_row_hover", |c| c.list_row_hover),
        ("list_row_hover_frame", |c| c.list_row_hover_frame),
        ("list_header_hover", |c| c.list_header_hover),
        ("list_header_pressed", |c| c.list_header_pressed),
        ("list_header_separator", |c| c.list_header_separator),
        ("scrollbar_thumb_hover", |c| c.scrollbar_thumb_hover),
        ("scrollbar_thumb", |c| c.scrollbar_thumb),
        ("progress_fill", |c| c.progress_fill),
        ("tab_hover", |c| c.tab_hover),
        ("toolbar_button_pressed", |c| c.toolbar_button_pressed),
        ("toolbar_button_checked", |c| c.toolbar_button_checked),
        ("menu_background", |c| c.menu_background),
        ("menu_hover", |c| c.menu_hover),
        ("menu_hover_frame", |c| c.menu_hover_frame),
        ("popover_border", |c| c.popover_border),
        ("card_border", |c| c.card_border),
        ("switch_frame", |c| c.switch_frame),
        ("switch_off", |c| c.switch_off),
        ("switch_on", |c| c.switch_on),
    ];
    for (scheme, breeze, expected) in [
        (Scheme::Light, LIGHT, BREEZE_LIGHT_DIFFERENCES),
        (Scheme::Dark, DARK, BREEZE_DARK_DIFFERENCES),
    ] {
        let laid = |value| Laid {
            value: Some(value),
            laid: true,
        };
        let mut c = breeze;
        derive(
            &mut c,
            &Settings {
                window_bg: laid(breeze.window_background),
                window_fg: laid(breeze.window_foreground),
                view_bg: laid(breeze.view_background),
                view_fg: laid(breeze.view_foreground),
                button_bg: laid(breeze.button_background),
                button_fg: laid(breeze.button_foreground),
                highlight: laid(breeze.selection_background),
                highlight_fg: laid(breeze.selection_foreground),
                focus: laid(breeze.accent),
                hover: laid(breeze.decoration_hover),
                header_bg: laid(breeze.toolbar_background),
                header_fg: Laid {
                    value: None,
                    laid: false,
                },
            },
            Recompute::WhereAnInputIsLaid,
        );
        let differences: Vec<String> = DERIVED
            .iter()
            .filter(|(_, field)| field(&c) != field(&breeze))
            .map(|(name, field)| format!("{name} {} {}", field(&c), field(&breeze)))
            .collect();
        assert_eq!(differences, expected, "{scheme:?}");
        assert_eq!(c.toolbar_border, breeze.toolbar_border);
    }
}

/// [`derivations_on_breeze_s_own_settings`]' differences on Breeze's light
/// colours: the derived colour, then the preset's constant. None: `list_grid`
/// reads Breeze's own contrast 4, at which `kde_shade` gives the constant.
const BREEZE_LIGHT_DIFFERENCES: &[&str] = &[];

/// [`derivations_on_breeze_s_own_settings`]' differences on Breeze Dark's
/// colours: the derived colour, then the preset's constant.
const BREEZE_DARK_DIFFERENCES: &[&str] = &[];

/// Under a palette whose surface colours are Breeze's own, `list_grid` is
/// derived again from them at the preset's contrast and gives the constant;
/// a system that does not state `[KDE] contrast` keeps KColorScheme's 7.
#[test]
fn list_grid_under_breeze_s_own_colours_is_the_constant() {
    for (scheme, breeze) in [(Scheme::Light, LIGHT), (Scheme::Dark, DARK)] {
        assert_eq!(breeze.contrast, BREEZE_CONTRAST, "{scheme:?}");
        let p = Palette {
            scheme,
            window_bg: breeze.window_background,
            view_bg: breeze.view_background,
            header_bg: breeze.toolbar_background,
            foreground: breeze.window_foreground,
            ..distinct_palette()
        };
        let c = layered(breeze, Some(&p), None);
        assert_eq!(c.list_grid, breeze.list_grid, "{scheme:?}");
    }
    let mut c = LIGHT;
    lay_system_over(&mut c, &OsColors::default());
    assert_eq!(c.contrast, CONTRAST_DEFAULT);
}
