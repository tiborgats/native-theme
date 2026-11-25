//! The per-widget builders of [`crate::geometry`], from the 0.8.0 model: one
//! function of the same name for each, taking the 0.8.0
//! [`ResolvedTheme`] in place of
//! [`Native`](crate::Native), and returning the same value, or the
//! [`Unmapped`](super::Unmapped) source it lacks where the theme leaves a
//! value it needs unstated.
//!
//! The text sizes are the model's, which the theme's text scale
//! (`ThemeContext::text_scale`) has already scaled, so no accessibility
//! preferences are passed; the control-height rule takes that scale from the
//! theme's context. The four layout metrics, which the 0.7.2 builders read
//! from the theme's `LayoutTheme`, are read from the resolved theme's layout.
//! [`title_bar_inactive`] takes, beside the theme, the same theme resolved
//! for an inactive window (`ThemeContext::window_active` false), where the
//! model states an inactive title bar.
//!
//! Each returns what the 0.7.2 builder returns for the same platform values;
//! where the 0.8.0 preset states a value apart from 0.7.2's (a kde-breeze
//! button's height, which it leaves to the toolkit, a list row's padding),
//! the connector's parity test lists the difference with its reason. A
//! stroke is drawn as the crate's README says under *Limits*.

use gpui::{Pixels, StyleRefinement};
use gpui_component::Size;
use gpui_component::dialog::DialogEntrance;
use native_theme::next::ResolvedTheme;

use super::{Mapped, builders};

/// The builders of the form `fn(&ResolvedTheme) -> Mapped<T>`, each the
/// 0.7.2 builder of its name from the 0.8.0 model, and, for the tests, the
/// comparison of each with the internal mapping.
macro_rules! builders {
    ($($name:ident -> $ty:ty;)*) => {
        $(
            #[doc = concat!(
                "[`crate::geometry::", stringify!($name), "`], from the 0.8.0 model.",
            )]
            pub fn $name(r: &ResolvedTheme) -> Mapped<$ty> {
                builders::$name(r)
            }
        )*

        /// Each builder of the form `fn(&ResolvedTheme) -> Mapped<T>` against
        /// the internal mapping's function of its name: the names compared
        /// and those whose values differ.
        #[cfg(test)]
        fn compare_with_builders(r: &ResolvedTheme) -> (Vec<&'static str>, Vec<&'static str>) {
            let mut names = Vec::new();
            let mut differ = Vec::new();
            $(
                names.push(stringify!($name));
                if $name(r) != builders::$name(r) {
                    differ.push(stringify!($name));
                }
            )*
            (names, differ)
        }
    };
}

builders! {
    button -> StyleRefinement;
    button_label -> StyleRefinement;
    toggle -> StyleRefinement;
    input -> StyleRefinement;
    text_area -> StyleRefinement;
    editor -> StyleRefinement;
    menu_item -> StyleRefinement;
    list_item -> StyleRefinement;
    tooltip -> StyleRefinement;
    tooltip_content -> StyleRefinement;
    popover -> StyleRefinement;
    status_bar -> StyleRefinement;
    dialog -> StyleRefinement;
    dialog_entrance -> Option<DialogEntrance>;
    input_group_button -> StyleRefinement;
    dialog_footer -> StyleRefinement;
    dialog_title -> StyleRefinement;
    dialog_description -> StyleRefinement;
    list -> StyleRefinement;
    table -> StyleRefinement;
    progress -> StyleRefinement;
    group_box_content -> StyleRefinement;
    accordion_title -> StyleRefinement;
    accordion -> StyleRefinement;
    accordion_hover -> StyleRefinement;
    checkbox -> StyleRefinement;
    radio -> StyleRefinement;
    select -> StyleRefinement;
    combobox -> StyleRefinement;
    title_bar -> StyleRefinement;
    toolbar -> StyleRefinement;
    spinner_size -> Size;
    data_table_size -> Size;
    list_header_height -> Option<Pixels>;
    icon_size_toolbar -> Size;
    icon_size_small -> Size;
    icon_size_large -> Size;
    icon_size_dialog -> Size;
    icon_size_panel -> Size;
    scrollbar_gutter -> StyleRefinement;
    dialog_max_width -> Option<Pixels>;
    input_height -> StyleRefinement;
    button_disabled -> StyleRefinement;
    button_checked -> StyleRefinement;
    toolbar_button_rest -> StyleRefinement;
    toolbar_button_checked -> StyleRefinement;
    default_button_rest -> Option<StyleRefinement>;
    list_item_selected -> StyleRefinement;
    link -> StyleRefinement;
    widget_gap -> Option<Pixels>;
    container_margin -> Option<Pixels>;
    window_margin -> Option<Pixels>;
    section_gap -> Option<Pixels>;
}

/// [`crate::geometry::title_bar_inactive`], from the 0.8.0 model: the title
/// bar's fill and the title's colour of `inactive`, the theme `r` resolved
/// for an inactive window (`ThemeContext::window_active` false), over the
/// window's fill of `r`.
pub fn title_bar_inactive(r: &ResolvedTheme, inactive: &ResolvedTheme) -> Mapped<StyleRefinement> {
    builders::title_bar_inactive(r, inactive)
}

/// [`crate::geometry::input_fill`], from the 0.8.0 model.
pub fn input_fill(r: &ResolvedTheme, disabled: bool) -> Mapped<StyleRefinement> {
    builders::input_fill(r, disabled)
}

/// [`crate::geometry::combo_box_fill`], from the 0.8.0 model.
pub fn combo_box_fill(r: &ResolvedTheme, disabled: bool) -> Mapped<StyleRefinement> {
    builders::combo_box_fill(r, disabled)
}

/// The rest state of a flat `Button` built on
/// [`next::variants::ghost_button`](super::variants::ghost_button), a 0.8.0
/// builder with no 0.7.2 one, whose ghost has no outline: where a rule
/// states the flat button's hovered outline,
/// `button.body(HOVER; variant=Flat).stroke.color`, a transparent fill and a
/// transparent border of that hovered stroke's width, each stated side of
/// `button.body(NONE)`'s padding less that width, floored at 0; nothing
/// where none does. As [`crate::geometry::toolbar_button_rest`] for the
/// toolbar's outline: upstream draws the variant's colour as the border in
/// every state and mixes it into the fill at rest, with no border width for
/// a custom variant, so this hides both at rest and lays the border out, the
/// Button keeping the size [`button`], which it is refined after, gives it.
pub fn ghost_button_rest(r: &ResolvedTheme) -> Mapped<StyleRefinement> {
    builders::ghost_button_rest(r)
}

#[cfg(test)]
mod tests {
    use native_theme::next::{Preset, Theme, ThemeContext};

    use super::*;

    /// A checked toolbar button on either macOS preset is filled with the
    /// checked fill the preset states, `toolbar.button(CHECKED).fill`, the
    /// toolbar-bezel button's as an on toggle: macos-monterey #dfdfdf /
    /// #414141, macos-tahoe the accent, #007aff (os-probe run 45634379188,
    /// toolbar states sheet, `*.normal.toolbar_toggle_on` `fill`), not with
    /// gpui-component's own selected ghost's.
    #[test]
    fn a_checked_macos_toolbar_button_takes_the_stated_checked_fill() {
        use gpui::Styled as _;
        use native_theme::next::{Color, WidgetState};
        for (preset, light, dark) in [
            (Preset::MacosMonterey, 0xdfdfdf_u32, 0x414141_u32),
            (Preset::MacosTahoe, 0x007aff, 0x007aff),
        ] {
            for (ctx, rgb) in [(ThemeContext::light(), light), (ThemeContext::dark(), dark)] {
                let r = Theme::preset(preset).resolve(&ctx);
                let [_, red, green, blue] = rgb.to_be_bytes();
                let want = super::super::hsla(Color::from_srgb8(red, green, blue, u8::MAX));
                assert_eq!(
                    super::super::state_fill_of(
                        &r.toolbar.button(WidgetState::CHECKED),
                        "toolbar.button(CHECKED).fill"
                    ),
                    Ok(Some(want)),
                    "{preset:?} {:?}",
                    ctx.scheme
                );
                assert_eq!(
                    toolbar_button_checked(&r).map(|c| c.background),
                    Ok(StyleRefinement::default().bg(want).background),
                    "{preset:?} {:?}",
                    ctx.scheme
                );
            }
        }
    }

    /// A checked toolbar button on either macOS preset is lettered in the
    /// toolbar's checked label the preset states,
    /// `toolbar.button_label(CHECKED).color`, the toolbar-bezel button's as an
    /// on toggle: macos-monterey #424242 / #ececec, macos-tahoe #ffffff
    /// (os-probe run 45634379188, toolbar states sheet,
    /// `*.normal.toolbar_toggle_on` `label`), not in the push button's pressed
    /// label (spec §10, rule 0a).
    #[test]
    fn a_checked_macos_toolbar_button_takes_the_stated_checked_label() {
        use native_theme::next::Color;
        for (preset, light, dark) in [
            (Preset::MacosMonterey, 0x424242_u32, 0xececec_u32),
            (Preset::MacosTahoe, 0xffffff, 0xffffff),
        ] {
            for (ctx, rgb) in [(ThemeContext::light(), light), (ThemeContext::dark(), dark)] {
                let r = Theme::preset(preset).resolve(&ctx);
                let [_, red, green, blue] = rgb.to_be_bytes();
                let want = super::super::hsla(Color::from_srgb8(red, green, blue, u8::MAX));
                assert_eq!(
                    toolbar_button_checked(&r).map(|c| c.text.color),
                    Ok(Some(want)),
                    "{preset:?} {:?}",
                    ctx.scheme
                );
            }
        }
    }

    /// A checked button whose checked fill and label no rule states
    /// (macos-monterey states neither, its checked toggle unmeasured) is
    /// refined with neither, so gpui-component's own selected look stands,
    /// not with the pressed pair the preset states (spec §10, rule 0a).
    #[test]
    fn an_unstated_checked_button_takes_no_pressed_pair() {
        use native_theme::next::WidgetState;
        for ctx in [ThemeContext::light(), ThemeContext::dark()] {
            let r = Theme::preset(Preset::MacosMonterey).resolve(&ctx);
            let b = &r.button;
            assert_eq!(
                super::super::state_fill_of(
                    &b.body(WidgetState::CHECKED),
                    "button.body(CHECKED).fill"
                ),
                Ok(None),
                "macos-monterey {:?} states a checked fill",
                ctx.scheme
            );
            assert_eq!(
                super::super::state_text_color_of(&b.label(WidgetState::CHECKED)),
                None,
                "macos-monterey {:?} states a checked label",
                ctx.scheme
            );
            assert!(
                matches!(
                    super::super::state_fill_of(
                        &b.body(WidgetState::PRESSED),
                        "button.body(PRESSED).fill"
                    ),
                    Ok(Some(_))
                ),
                "macos-monterey {:?} states no pressed fill, so this proves nothing",
                ctx.scheme
            );
            assert_eq!(
                button_checked(&r),
                Ok(StyleRefinement::default()),
                "macos-monterey {:?}",
                ctx.scheme
            );
        }
    }

    /// A checked toolbar button whose checked fill no rule states (ios-13
    /// states none) is filled as gpui-component's own selected ghost, with
    /// `tokens.secondary_active` as `apply` installs it, not with a pressed
    /// fill of the theme's: the flat button's pressed fill is patched to one
    /// of its own, which `apply` gives `secondary_active`, so it differs from
    /// the push button's and the toolbar's pressed fills.
    #[gpui::test]
    fn an_unstated_checked_toolbar_fill_is_the_selected_ghosts(cx: &mut gpui::TestAppContext) {
        use gpui::{Hsla, Styled as _};
        use gpui_component::ActiveTheme as _;
        use native_theme::next::{
            ButtonRule, ButtonVariant, Color, Paint, PartPatch, ThemePatch, WidgetState,
        };
        cx.update(|cx| {
            gpui_component::init(cx);
            for ctx in [ThemeContext::light(), ThemeContext::dark()] {
                let mut r = Theme::preset(Preset::Ios13).resolve(&ctx);
                r.apply_patch(
                    &ThemePatch::new().button(
                        ButtonRule::new()
                            .variant(ButtonVariant::Flat)
                            .states(WidgetState::PRESSED)
                            .body(PartPatch::new().fill(Paint::Solid(Color::rgb(1.0, 0.0, 0.0)))),
                    ),
                );
                let t = &r.toolbar;
                assert_eq!(
                    super::super::state_fill_of(
                        &t.button(WidgetState::CHECKED),
                        "toolbar.button(CHECKED).fill"
                    ),
                    Ok(None),
                    "ios-13 {:?} states a checked toolbar fill",
                    ctx.scheme
                );
                assert_eq!(super::super::apply(&r, cx), Ok(()));
                let own = Hsla::from(cx.theme().tokens.secondary_active);
                let pressed = [
                    super::super::state_fill_of(
                        &t.button(WidgetState::PRESSED),
                        "toolbar.button(PRESSED).fill",
                    ),
                    super::super::state_fill_of(
                        &r.button.body(WidgetState::PRESSED),
                        "button.body(PRESSED).fill",
                    ),
                ];
                assert!(
                    !pressed.contains(&Ok(Some(own))),
                    "ios-13 {:?}: a pressed fill is the selected ghost's, so this proves \
                     nothing",
                    ctx.scheme
                );
                assert_eq!(
                    toolbar_button_checked(&r).map(|c| c.background),
                    Ok(StyleRefinement::default().bg(own).background),
                    "ios-13 {:?}",
                    ctx.scheme
                );
            }
        });
    }

    /// Every public builder returns what the internal mapping, which the
    /// parity test compares with 0.7.2's builders, returns; and every public
    /// builder of [`crate::geometry`] has one here.
    #[test]
    fn every_builder_returns_the_internal_mappings_value() {
        let mut names = Vec::new();
        for ctx in [ThemeContext::light(), ThemeContext::dark()] {
            let r = Theme::preset(Preset::KdeBreeze).resolve(&ctx);
            let mut inactive_ctx = ctx.clone();
            inactive_ctx.window_active = false;
            let inactive = Theme::preset(Preset::KdeBreeze).resolve(&inactive_ctx);
            let (compared, differ) = compare_with_builders(&r);
            assert!(differ.is_empty(), "{differ:?}");
            assert_eq!(
                title_bar_inactive(&r, &inactive),
                builders::title_bar_inactive(&r, &inactive)
            );
            assert_eq!(ghost_button_rest(&r), builders::ghost_button_rest(&r));
            for disabled in [false, true] {
                assert_eq!(input_fill(&r, disabled), builders::input_fill(&r, disabled));
                assert_eq!(
                    combo_box_fill(&r, disabled),
                    builders::combo_box_fill(&r, disabled)
                );
            }
            names = compared;
        }
        names.extend(["title_bar_inactive", "input_fill", "combo_box_fill"]);
        let old: Vec<&str> = include_str!("../geometry.rs")
            .lines()
            .filter_map(|line| line.strip_prefix("pub fn "))
            .filter_map(|rest| rest.split(['(', '<']).next())
            .collect();
        let missing: Vec<&&str> = old.iter().filter(|name| !names.contains(name)).collect();
        let extra: Vec<&&str> = names.iter().filter(|name| !old.contains(name)).collect();
        assert!(
            !old.is_empty() && missing.is_empty() && extra.is_empty(),
            "0.7.2 builders without a 0.8.0 one: {missing:?}; 0.8.0 builders without a \
             0.7.2 one: {extra:?}"
        );
    }
}
