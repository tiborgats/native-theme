//! `RoleVariant::Selected` and `RoleVariant::Disabled` cells (spec §6.2, §6.3).
//!
//! Each cell starts from its role's `Normal` cell and writes only the variant's leaves (§3.4).
//! A role with no variant data returns `None`, and `compile` then shares the `Normal` `Arc`.

use crate::convert::to_color32;
use crate::style::{BuildInput, note_transparent_fill};
use crate::{Note, Role};

/// The `Selected` cell of `role`, or `None` where the role's selected look lives in its
/// `Normal` cell (`Button`, `Tab`, `SegmentedControl`, `Switch`, §6.2) or the role has none.
pub(crate) fn selected_cell(
    role: Role,
    normal: &egui::Style,
    input: &BuildInput<'_>,
    notes: &mut Vec<Note>,
) -> Option<egui::Style> {
    if role != Role::Checkbox {
        return None;
    }
    let t = input.theme;
    let mut style = normal.clone();
    // The checked box: `checkbox_style` fills from the state's `bg_fill`
    // (`egui/src/widget_style.rs:182`) and outlines from `bg_stroke` (`:184`); the mark keeps
    // `indicator_color` in `fg_stroke` (§5.3). `checkbox.hover_background` is the unchecked
    // box's hover fill, so the hovered entry holds the checked fill too (§6.4).
    let fill = to_color32(t.checkbox.checked_background);
    note_transparent_fill(fill, "checkbox.checked_background", notes);
    let outline = to_color32(t.checkbox.border.color);
    let w = &mut style.visuals.widgets;
    for entry in [
        &mut w.noninteractive,
        &mut w.inactive,
        &mut w.hovered,
        &mut w.active,
        &mut w.open,
    ] {
        entry.bg_fill = fill;
        entry.bg_stroke.color = outline;
    }
    Some(style)
}

/// The `Disabled` cell of `role`, or `None` where the role writes none of §6.3's fifteen leaves.
pub(crate) fn disabled_cell(
    role: Role,
    normal: &egui::Style,
    input: &BuildInput<'_>,
    notes: &mut Vec<Note>,
) -> Option<egui::Style> {
    let t = input.theme;
    let mut style = normal.clone();
    let v = &mut style.visuals;
    match role {
        // `button_style` fills a button from `weak_bg_fill` (`egui/src/widget_style.rs:159`) and
        // a selected one from `selection.*` regardless of state (`:150-155`), so the primary
        // button in a disabled scope takes the disabled pair too (§6.2) — where the platform
        // states a disabled fill. With none, it dims by opacity alone (docs/platform-facts.md
        // §2.1.6), and the primary button keeps its own pair.
        Role::Button => {
            let text = to_color32(t.button.disabled_text_color);
            v.widgets.inactive.weak_bg_fill = to_color32(
                t.button
                    .disabled_background
                    .unwrap_or(t.button.background_color),
            );
            v.widgets.inactive.fg_stroke.color = text;
            v.widgets.noninteractive.fg_stroke.color = text;
            if let Some(fill) = t.button.disabled_background {
                v.selection.bg_fill = to_color32(fill);
                v.selection.stroke.color = text;
            }
        }
        // `button_frame` fills the trigger from `weak_bg_fill` (`egui/src/containers/combo_box.rs:460`).
        Role::ComboBox => {
            let fill = t
                .combo_box
                .disabled_background
                .unwrap_or(t.combo_box.background_color);
            let text = to_color32(t.combo_box.disabled_text_color);
            v.widgets.inactive.weak_bg_fill = to_color32(fill);
            v.widgets.inactive.fg_stroke.color = text;
            v.widgets.noninteractive.fg_stroke.color = text;
        }
        // The box fills from `bg_fill` (`egui/src/widget_style.rs:182`); the label reads
        // `override_text_color` (`:133-136`) and the mark strokes `inactive.fg_stroke` (`:188`).
        Role::Checkbox => {
            let (fill, path) = match (
                t.checkbox.disabled_background,
                t.checkbox.unchecked_background,
            ) {
                (Some(c), _) => (c, "checkbox.disabled_background"),
                (None, Some(c)) => (c, "checkbox.unchecked_background"),
                (None, None) => (t.checkbox.background_color, "checkbox.background_color"),
            };
            note_transparent_fill(to_color32(fill), path, notes);
            let text = to_color32(t.checkbox.disabled_text_color);
            v.widgets.inactive.bg_fill = to_color32(fill);
            v.override_text_color = Some(text);
            v.widgets.inactive.fg_stroke.color = text;
        }
        // A `TextEdit` fills from `Visuals::text_edit_bg_color()`
        // (`egui/src/widgets/text_edit/builder.rs:739`) and takes its text colour from
        // `inactive` in every state (`:480-483`).
        Role::Input => {
            let fill = t
                .input
                .disabled_background
                .unwrap_or(t.input.background_color);
            let text = to_color32(t.input.disabled_text_color);
            v.text_edit_bg_color = Some(to_color32(fill));
            v.widgets.inactive.fg_stroke.color = text;
            v.widgets.noninteractive.fg_stroke.color = text;
        }
        // The rail is always `inactive.bg_fill` (`egui/src/widgets/slider.rs:775`) and the
        // trailing fill `selection.bg_fill` (`:802`).
        Role::Slider => {
            let (track, path) = match t.slider.disabled_track_color {
                Some(c) => (c, "slider.disabled_track_color"),
                None => (t.slider.track_color, "slider.track_color"),
            };
            note_transparent_fill(to_color32(track), path, notes);
            v.widgets.inactive.bg_fill = to_color32(track);
            v.selection.bg_fill =
                to_color32(t.slider.disabled_fill_color.unwrap_or(t.slider.fill_color));
        }
        // The substitute `Button::new(..).selected(checked)`: unchecked it fills from the state's
        // `weak_bg_fill`, checked from `selection.bg_fill` in every state (§6.2, §6.3).
        Role::Switch => {
            v.widgets.inactive.weak_bg_fill = to_color32(
                t.switch
                    .disabled_unchecked_background
                    .unwrap_or(t.switch.unchecked_background),
            );
            v.selection.bg_fill = to_color32(
                t.switch
                    .disabled_checked_background
                    .unwrap_or(t.switch.checked_background),
            );
        }
        // A disabled item is a `Button` in `WidgetState::Inactive`; a plain label in the menu
        // reads `noninteractive` (§5.2).
        Role::Menu => {
            let text = to_color32(t.menu.disabled_text_color);
            v.widgets.inactive.fg_stroke.color = text;
            v.widgets.noninteractive.fg_stroke.color = text;
        }
        // Cells are `Label`s, painted from `Visuals::text_color()` (§5.4).
        Role::List => {
            v.widgets.noninteractive.fg_stroke.color = to_color32(t.list.disabled_text_color);
        }
        // `Link::ui` paints with `hyperlink_color` alone (`egui/src/widgets/hyperlink.rs:47`).
        Role::Link => {
            v.hyperlink_color = to_color32(t.link.disabled_text_color);
        }
        _ => return None,
    }
    // `disabled_alpha` stays the role's own, its `disabled_opacity` (or, for a role with none,
    // `defaults.disabled_opacity`, the base style's): `Ui::disable` multiplies painter opacity
    // by it (`egui/src/ui.rs:497-502`) on top of these colours. A platform dims by one of the
    // two, and the data makes the other an identity (docs/platform-facts.md §2.1.6).
    Some(style)
}

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    reason = "a test fails by panicking"
)]
mod tests {
    use std::sync::Arc;

    use egui::Theme;
    use native_theme::color::Rgba;
    use native_theme::theme::{ColorMode, ResolvedTheme};

    use crate::convert::to_color32;
    use crate::install_tests::resolved;
    use crate::style::{BuildInput, SchemeStyles, compile};
    use crate::{AccessibilityPreferences, LayoutTheme, Note, Role, RoleVariant};

    fn build(t: &ResolvedTheme) -> (SchemeStyles, Vec<Note>) {
        let prefs = AccessibilityPreferences::default();
        let layout = LayoutTheme::default();
        let input = BuildInput {
            scheme: Theme::Light,
            theme: t,
            prefs: &prefs,
            layout: &layout,
            row_height: None,
            patch: None,
        };
        let mut notes = Vec::new();
        let styles = compile(&input, &mut notes);
        (styles, notes)
    }

    /// §6.2, §6.3: only Checkbox has a `Selected` cell of its own; only the nine roles §6.3
    /// lists have a `Disabled` cell of their own. Every other slot is the `Normal` `Arc`.
    #[test]
    fn cells_without_variant_data_share_the_normal_arc() {
        let t = resolved("kde-breeze", ColorMode::Light);
        let (s, _) = build(&t);
        const DISABLED: [Role; 9] = [
            Role::Button,
            Role::ComboBox,
            Role::Checkbox,
            Role::Input,
            Role::Slider,
            Role::Switch,
            Role::Menu,
            Role::List,
            Role::Link,
        ];
        for role in Role::all() {
            let normal = s.cell(*role, RoleVariant::Normal);
            let selected = s.cell(*role, RoleVariant::Selected);
            let disabled = s.cell(*role, RoleVariant::Disabled);
            assert_eq!(
                Arc::ptr_eq(normal, selected),
                *role != Role::Checkbox,
                "{role:?} Selected"
            );
            assert_eq!(
                Arc::ptr_eq(normal, disabled),
                !DISABLED.contains(role),
                "{role:?} Disabled"
            );
        }
    }

    /// §6.2: the checked box fills from `checked_background` in all five entries, its outline is
    /// `checkbox.border.color`, the mark keeps `indicator_color`, and nothing else moves.
    #[test]
    fn the_checkbox_selected_cell_fills_the_box_and_outlines_it() {
        let t = resolved("kde-breeze", ColorMode::Light);
        let (s, _) = build(&t);
        let normal = s.cell(Role::Checkbox, RoleVariant::Normal);
        let sel = s.cell(Role::Checkbox, RoleVariant::Selected);
        let fill = to_color32(t.checkbox.checked_background);
        let outline = to_color32(t.checkbox.border.color);
        let entries =
            |w: &egui::style::Widgets| [w.noninteractive, w.inactive, w.hovered, w.active, w.open];
        for (n, e) in entries(&normal.visuals.widgets)
            .iter()
            .zip(entries(&sel.visuals.widgets))
        {
            assert_eq!(e.bg_fill, fill);
            assert_eq!(e.bg_stroke.color, outline);
            assert_eq!(e.bg_stroke.width, n.bg_stroke.width);
            assert_eq!(e.fg_stroke, n.fg_stroke, "the mark keeps indicator_color");
            assert_eq!(e.weak_bg_fill, n.weak_bg_fill);
            assert_eq!(e.corner_radius, n.corner_radius);
        }
        assert_eq!(sel.spacing, normal.spacing);
        assert_eq!(sel.text_styles, normal.text_styles);
        assert_eq!(sel.visuals.selection, normal.visuals.selection);
        assert_eq!(
            sel.visuals.override_text_color,
            normal.visuals.override_text_color
        );
        assert_eq!(sel.visuals.disabled_alpha, normal.visuals.disabled_alpha);
    }

    /// §6.3, §6.4: windows-11 states a fully transparent disabled checkbox fill, `#f9f9f900`
    /// (`native-theme/src/presets/windows-11.toml:133`; dark `#33333300`, `:513`); it is written
    /// as given into the `Disabled` cell's `inactive.bg_fill` and reported once, for the leaf.
    #[test]
    fn a_transparent_disabled_fill_is_written_and_noted_once() {
        let t = resolved("windows-11", ColorMode::Light);
        let stated = t
            .checkbox
            .disabled_background
            .expect("windows-11 states it");
        assert_eq!(stated.a, 0, "the preset states alpha 0");
        let (s, notes) = build(&t);
        let dis = s.cell(Role::Checkbox, RoleVariant::Disabled);
        assert_eq!(dis.visuals.widgets.inactive.bg_fill.a(), 0);
        let hits = notes
            .iter()
            .filter(|n| matches!(n, Note::TransparentFill { path, .. } if *path == "checkbox.disabled_background"))
            .count();
        assert_eq!(hits, 1);
        // The `Selected` cell's checked box is opaque on this preset and reports nothing.
        let sel = s.cell(Role::Checkbox, RoleVariant::Selected);
        assert_ne!(sel.visuals.widgets.inactive.bg_fill.a(), 0);
        assert!(!notes.iter().any(|n| matches!(n, Note::TransparentFill { path, .. } if *path == "checkbox.checked_background")));
    }

    /// §6.3: the Button `Disabled` cell — stated fills and text colour, the selection pair for a
    /// primary button, and the button's own `disabled_opacity` for egui to fade it by on top
    /// (docs/platform-facts.md §2.1.6); the `hovered` entry is untouched.
    #[test]
    fn the_button_disabled_cell_writes_what_a_disabled_button_reads() {
        let mut t = resolved("kde-breeze", ColorMode::Light);
        let fill = Rgba {
            r: 1,
            g: 2,
            b: 3,
            a: 255,
        };
        let text = Rgba {
            r: 10,
            g: 20,
            b: 30,
            a: 255,
        };
        t.button.disabled_background = Some(fill);
        t.button.disabled_text_color = text;
        let (s, _) = build(&t);
        let normal = s.cell(Role::Button, RoleVariant::Normal);
        let dis = s.cell(Role::Button, RoleVariant::Disabled);
        let w = &dis.visuals.widgets;
        assert_eq!(w.inactive.weak_bg_fill, to_color32(fill));
        assert_eq!(dis.visuals.selection.bg_fill, to_color32(fill));
        assert_eq!(w.inactive.fg_stroke.color, to_color32(text));
        assert_eq!(w.noninteractive.fg_stroke.color, to_color32(text));
        assert_eq!(dis.visuals.selection.stroke.color, to_color32(text));
        assert_eq!(dis.visuals.disabled_alpha, normal.visuals.disabled_alpha);
        assert_eq!(
            dis.visuals.disabled_alpha,
            crate::convert::unit_interval(t.button.disabled_opacity)
        );
        assert_eq!(w.hovered, normal.visuals.widgets.hovered);
        assert_eq!(
            w.inactive.bg_stroke,
            normal.visuals.widgets.inactive.bg_stroke
        );

        // A `None` disabled fill is the platform stating no distinct fill: the idle fill is
        // copied as given (§6.4, C16), never composited, and a primary button keeps its own
        // pair, which `disabled_opacity` alone dims.
        t.button.disabled_background = None;
        let (s, _) = build(&t);
        let normal = s.cell(Role::Button, RoleVariant::Normal);
        let dis = s.cell(Role::Button, RoleVariant::Disabled);
        assert_eq!(
            dis.visuals.widgets.inactive.weak_bg_fill,
            to_color32(t.button.background_color)
        );
        assert_eq!(dis.visuals.selection, normal.visuals.selection);
    }

    /// §6.3: each of the other eight `Disabled` cells writes the field its widget reads.
    #[test]
    fn each_disabled_cell_writes_the_fields_its_widget_reads() {
        let mut t = resolved("kde-breeze", ColorMode::Light);
        let c = |r: u8| Rgba {
            r,
            g: r,
            b: r,
            a: 255,
        };
        t.combo_box.disabled_background = Some(c(11));
        t.combo_box.disabled_text_color = c(12);
        t.checkbox.disabled_background = Some(c(21));
        t.checkbox.disabled_text_color = c(22);
        t.input.disabled_background = Some(c(31));
        t.input.disabled_text_color = c(32);
        t.slider.disabled_track_color = Some(c(41));
        t.slider.disabled_fill_color = Some(c(42));
        t.switch.disabled_unchecked_background = Some(c(51));
        t.switch.disabled_checked_background = Some(c(52));
        t.menu.disabled_text_color = c(61);
        t.list.disabled_text_color = c(71);
        t.link.disabled_text_color = c(81);
        let (s, _) = build(&t);
        let cell = |r: Role| s.cell(r, RoleVariant::Disabled);

        let d = cell(Role::ComboBox);
        assert_eq!(d.visuals.widgets.inactive.weak_bg_fill, to_color32(c(11)));
        assert_eq!(
            d.visuals.widgets.inactive.fg_stroke.color,
            to_color32(c(12))
        );
        assert_eq!(
            d.visuals.widgets.noninteractive.fg_stroke.color,
            to_color32(c(12))
        );

        let d = cell(Role::Checkbox);
        assert_eq!(d.visuals.widgets.inactive.bg_fill, to_color32(c(21)));
        assert_eq!(d.visuals.override_text_color, Some(to_color32(c(22))));
        assert_eq!(
            d.visuals.widgets.inactive.fg_stroke.color,
            to_color32(c(22))
        );

        let d = cell(Role::Input);
        assert_eq!(d.visuals.text_edit_bg_color, Some(to_color32(c(31))));
        assert_eq!(
            d.visuals.widgets.inactive.fg_stroke.color,
            to_color32(c(32))
        );
        assert_eq!(
            d.visuals.widgets.noninteractive.fg_stroke.color,
            to_color32(c(32))
        );

        let d = cell(Role::Slider);
        assert_eq!(d.visuals.widgets.inactive.bg_fill, to_color32(c(41)));
        assert_eq!(d.visuals.selection.bg_fill, to_color32(c(42)));

        let d = cell(Role::Switch);
        assert_eq!(d.visuals.widgets.inactive.weak_bg_fill, to_color32(c(51)));
        assert_eq!(d.visuals.selection.bg_fill, to_color32(c(52)));

        let d = cell(Role::Menu);
        assert_eq!(
            d.visuals.widgets.inactive.fg_stroke.color,
            to_color32(c(61))
        );
        assert_eq!(
            d.visuals.widgets.noninteractive.fg_stroke.color,
            to_color32(c(61))
        );

        let d = cell(Role::List);
        assert_eq!(
            d.visuals.widgets.noninteractive.fg_stroke.color,
            to_color32(c(71))
        );

        let d = cell(Role::Link);
        assert_eq!(d.visuals.hyperlink_color, to_color32(c(81)));

        // Each keeps its role's `disabled_opacity`, and a role with none the defaults' one, for
        // egui to fade it by on top of those colours (docs/platform-facts.md §2.1.6).
        let u = crate::convert::unit_interval;
        for (role, opacity) in [
            (Role::ComboBox, t.combo_box.disabled_opacity),
            (Role::Checkbox, t.checkbox.disabled_opacity),
            (Role::Input, t.input.disabled_opacity),
            (Role::Slider, t.slider.disabled_opacity),
            (Role::Switch, t.switch.disabled_opacity),
            (Role::Menu, t.defaults.disabled_opacity),
            (Role::List, t.defaults.disabled_opacity),
            (Role::Link, t.defaults.disabled_opacity),
        ] {
            assert_eq!(cell(role).visuals.disabled_alpha, u(opacity), "{role:?}");
        }
    }

    /// The disabled rule on the platforms (docs/platform-facts.md §2.1.6): kde-breeze dims by
    /// its disabled colours with an opacity of 1.0, adwaita by its opacity of 0.5 with no
    /// disabled fill of its own, so each `Disabled` cell carries exactly one of the two.
    #[test]
    fn each_platform_dims_by_one_mechanism() {
        let kde = resolved("kde-breeze", ColorMode::Light);
        let (s, _) = build(&kde);
        let dis = s.cell(Role::Button, RoleVariant::Disabled);
        assert_eq!(dis.visuals.disabled_alpha, 1.0);
        assert_eq!(
            dis.visuals.widgets.inactive.weak_bg_fill,
            to_color32(
                kde.button
                    .disabled_background
                    .expect("kde-breeze states it")
            )
        );

        let adw = resolved("adwaita", ColorMode::Light);
        let (s, _) = build(&adw);
        let normal = s.cell(Role::Button, RoleVariant::Normal);
        let dis = s.cell(Role::Button, RoleVariant::Disabled);
        assert_eq!(dis.visuals.disabled_alpha, 0.5);
        assert_eq!(adw.button.disabled_background, None);
        assert_eq!(
            dis.visuals.widgets.inactive.weak_bg_fill,
            normal.visuals.widgets.inactive.weak_bg_fill
        );
        assert_eq!(
            dis.visuals.widgets.inactive.fg_stroke.color,
            normal.visuals.widgets.inactive.fg_stroke.color
        );
    }

    /// §6.3: the eight disabled fills fall back to the role's own fill for that part.
    #[test]
    fn a_none_disabled_fill_copies_the_roles_own_fill() {
        let mut t = resolved("kde-breeze", ColorMode::Light);
        t.combo_box.disabled_background = None;
        t.checkbox.disabled_background = None;
        t.checkbox.unchecked_background = None;
        t.input.disabled_background = None;
        t.slider.disabled_track_color = None;
        t.slider.disabled_fill_color = None;
        t.switch.disabled_unchecked_background = None;
        t.switch.disabled_checked_background = None;
        let (s, _) = build(&t);
        let cell = |r: Role| s.cell(r, RoleVariant::Disabled);
        assert_eq!(
            cell(Role::ComboBox).visuals.widgets.inactive.weak_bg_fill,
            to_color32(t.combo_box.background_color)
        );
        assert_eq!(
            cell(Role::Checkbox).visuals.widgets.inactive.bg_fill,
            to_color32(t.checkbox.background_color)
        );
        assert_eq!(
            cell(Role::Input).visuals.text_edit_bg_color,
            Some(to_color32(t.input.background_color))
        );
        assert_eq!(
            cell(Role::Slider).visuals.widgets.inactive.bg_fill,
            to_color32(t.slider.track_color)
        );
        assert_eq!(
            cell(Role::Slider).visuals.selection.bg_fill,
            to_color32(t.slider.fill_color)
        );
        assert_eq!(
            cell(Role::Switch).visuals.widgets.inactive.weak_bg_fill,
            to_color32(t.switch.unchecked_background)
        );
        assert_eq!(
            cell(Role::Switch).visuals.selection.bg_fill,
            to_color32(t.switch.checked_background)
        );
    }

    /// §6.3: a role that writes none of the disabled leaves keeps the theme's own alpha.
    #[test]
    fn every_other_role_keeps_the_themes_disabled_alpha() {
        let t = resolved("adwaita", ColorMode::Dark);
        let (s, _) = build(&t);
        for role in [
            Role::Tab,
            Role::Sidebar,
            Role::Toolbar,
            Role::Expander,
            Role::Card,
        ] {
            let d = s.cell(role, RoleVariant::Disabled);
            assert_eq!(
                d.visuals.disabled_alpha, s.base.visuals.disabled_alpha,
                "{role:?}"
            );
        }
    }
}
