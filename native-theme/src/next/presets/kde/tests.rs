use super::*;

use crate::next::preset::Theme;
use crate::next::query::{ButtonQuery, Contrast, ListQuery, StyleQuery};
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
        let row = theme.list.row(ListQuery::new(UNFOCUSED_HOVER));
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
            let row = |state| theme.list.row(ListQuery::new(state).alternate(alternate));
            let (hover, unfocused) = (row(WidgetState::HOVER), row(UNFOCUSED_HOVER));
            assert_ne!(unfocused.fill, hover.fill, "{scheme:?} {alternate}");
            assert_ne!(unfocused.stroke, hover.stroke, "{scheme:?} {alternate}");
            let selected = row(WidgetState::SELECTED);
            let both = row(UNFOCUSED_HOVER.union(WidgetState::SELECTED));
            assert_eq!(both.fill, selected.fill, "{scheme:?} {alternate}");
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
    let row = theme.list.row(ListQuery::new(UNFOCUSED_HOVER));
    let fill = solid(&row.fill).ok_or("no fill")?;
    let stroke = outline_color(&row.stroke).ok_or("no outline")?;
    assert_eq!(fill.to_srgb8(), inactive.with_alpha(0.3).to_srgb8());
    assert_eq!(stroke.to_srgb8(), inactive.with_alpha(0.8).to_srgb8());
    // `S3` on the same file: the plain handle, derived, in every view.
    let plain = theme.scrollbar.thumb(WidgetState::UNFOCUSED_VIEW);
    assert_ne!(
        plain.fill,
        breeze.scrollbar.thumb(WidgetState::UNFOCUSED_VIEW).fill
    );
    assert_eq!(theme.scrollbar.thumb(WidgetState::NONE), plain);
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
/// list's highlight 4.5 and draws a table's square", so a row at rest, an
/// alternate one too, states no corners; the highlight's margins as before
/// (`list.border.padding`, platform-facts.md (§2.15)).
#[test]
fn a_row_s_highlight_is_rounded_4_5_and_a_table_row_is_square() {
    for theme in both_schemes() {
        let scheme = theme.context.scheme;
        for alternate in [false, true] {
            let rest = theme
                .list
                .row(ListQuery::new(WidgetState::NONE).alternate(alternate));
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
                let row = theme.list.row(ListQuery::new(state).alternate(alternate));
                assert_eq!(row.corners, Corners::uniform(4.5), "{scheme:?} {state:?}");
                assert_eq!(
                    row.margin,
                    Edges::symmetric(1.0, 2.0),
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
                .row(ListQuery::from(ListVariant::Table).state(state));
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
/// colours), so the handle at rest is the plain handle in every view, as
/// 0.7.2 draws it. A system without window colours states no `S4`: that
/// query returns the plain handle. No rule is dead.
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
        assert_eq!(theme.scrollbar.thumb(state), plain, "{state:?}");
    }
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
