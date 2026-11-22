use super::*;

use crate::next::preset::Theme;
use crate::next::query::{ButtonQuery, Contrast, ListQuery, StyleQuery};
use crate::next::reader::{OsSelectionColors, SystemOverrides};

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
            theme.list.row(ListQuery::new(WidgetState::SELECTED)).fill,
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
    for (ctx, static_theme) in [
        (ThemeContext::dark(), dark),
        (ThemeContext::light().with_contrast(Contrast::High), light),
    ] {
        let theme = Theme::preset(Preset::KdeBreeze)
            .with_system_overrides(system(None))
            .resolve(&ctx);
        assert_eq!(theme.colors, static_theme.colors, "{ctx:?}");
    }
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
        let hover = theme.list.row(ListQuery::new(WidgetState::HOVER));
        let rest = theme.list.row(ListQuery::new(WidgetState::NONE));
        assert_ne!(hover.stroke, rest.stroke, "{:?}", theme.context.scheme);
        let selected = WidgetState::SELECTED;
        let hovered_selected = theme
            .list
            .row(ListQuery::new(selected.union(WidgetState::HOVER)));
        assert_eq!(
            hovered_selected.stroke,
            theme.list.row(ListQuery::new(selected)).stroke,
            "{:?}",
            theme.context.scheme
        );
        assert_eq!(
            hovered_selected.fill,
            Paint::Solid(theme.colors.selection.background)
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
                theme.checkbox.indicator(both),
                theme.checkbox.indicator(hover),
                "{scheme:?} {checked:?}"
            );
            assert_eq!(
                theme.radio.indicator(both),
                theme.radio.indicator(hover),
                "{scheme:?} {checked:?}"
            );
        }
        for alternate in [false, true] {
            assert_eq!(
                theme
                    .list
                    .row(ListQuery::new(pressed_hover).alternate(alternate)),
                theme
                    .list
                    .row(ListQuery::new(WidgetState::HOVER).alternate(alternate)),
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
