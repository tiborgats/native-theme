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

#[cfg(test)]
mod tests {
    use native_theme::next::{Preset, Theme, ThemeContext};

    use super::*;

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
