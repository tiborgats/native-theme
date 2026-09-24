//! The two declared-data tables that say what the rows do *not* claim: every
//! emitted field iced fills for us, and the native values iced cannot take.
//!
//! Test-only, like the rest of `contract`. Nothing here is logic beyond the
//! one-line readers `UNREACHABLE` names its values with: the tests that read
//! these tables live in the parent module with the rows they are checked
//! against.

use super::*;

/// How many of the 32 preset/mode combinations emit a substitute for the
/// platform's own label in `extended.secondary.base.text`, and which ones do
/// not -- the two numbers the `extended.secondary.base.text` entry below
/// quotes in prose.
///
/// They live here, beside the sentence that cites them, so that a new preset
/// moves the count and turns
/// `readable_substitutes_the_secondary_label_on_every_combination_but_one`
/// red instead of letting the sentence rot. The measurement is that test's,
/// not a note of one made elsewhere.
pub(super) const SECONDARY_LABEL_SUBSTITUTED: usize = 31;

/// The combinations that emit `defaults.text_color` exactly as the platform
/// states it -- the exception the entry below names.
pub(super) const SECONDARY_LABEL_AS_STATED: &[&str] = &["ios/light"];

/// Every field the tripwire walks that no row claims, with the derivation that
/// fills it instead. A field is in the rows or here, never both and never
/// neither -- that is the whole of the tripwire (section 5.2) -- save the one
/// emitted field `UNREACHABLE` names, which is in neither and there instead.
pub(super) const DERIVED: &[(&str, &str)] = &[
    (
        "extended.background.base.color",
        "iced: Extended::generate(palette).background.base.color, from palette.background",
    ),
    (
        "extended.secondary.base.text",
        "Pair::new(input.placeholder_color, extended.background.base.text) -- \
         the window's own label, which the connector sets from \
         defaults.text_color. Pair::new does not emit that label as it is: it \
         runs readable(color, text) against the placeholder fill \
         (iced_core theme/palette.rs:440-445), and a label that is not \
         readable on that fill is lightened, darkened, or replaced by black or \
         white (palette.rs:692-720). So the emitted value is the platform's \
         text colour only where it clears iced's contrast bar, and mostly it \
         does not: over all 32 bundled preset/mode combinations, 31 emit a \
         substitute of iced's own, ios light being the only one that does \
         not. Those are SECONDARY_LABEL_SUBSTITUTED and \
         SECONDARY_LABEL_AS_STATED above, counted again on every run by the \
         test named there rather than asserted here. Placeholder grey and \
         window text are close by construction, which is exactly the pair \
         readable() rejects",
    ),
    (
        "extended.secondary.strong.text",
        "copy of extended.secondary.base.text",
    ),
    (
        "extended.primary.base.color",
        "iced: Extended::generate(palette).primary.base.color, from palette.primary",
    ),
    (
        "extended.success.base.color",
        "iced: Extended::generate(palette).success.base.color, from palette.success",
    ),
    (
        "extended.danger.base.color",
        "iced: Extended::generate(palette).danger.base.color, from palette.danger",
    ),
    (
        "extended.warning.base.color",
        "iced: Extended::generate(palette).warning.base.color, from palette.warning",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::button.shadow",
        "iced default: button::Style::default().shadow -- the model has a \
         shadow color but no offset or blur",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::button.snap",
        "iced default: button::Style::default().snap -- a renderer setting, \
         cfg!(feature = \"crisp\")",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::button_primary.shadow",
        "iced default: button::primary(theme, status).shadow -- the model has \
         a shadow color but no offset or blur",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::button_primary.snap",
        "iced default: button::primary(theme, status).snap -- a renderer \
         setting, cfg!(feature = \"crisp\")",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::button_danger.shadow",
        "iced default: button::danger(theme, status).shadow -- the model has \
         a shadow color but no offset or blur",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::button_danger.snap",
        "iced default: button::danger(theme, status).snap -- a renderer \
         setting, cfg!(feature = \"crisp\")",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::button_success.shadow",
        "iced default: button::success(theme, status).shadow -- the model has \
         a shadow color but no offset or blur",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::button_success.snap",
        "iced default: button::success(theme, status).snap -- a renderer \
         setting, cfg!(feature = \"crisp\")",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::button_warning.shadow",
        "iced default: button::warning(theme, status).shadow -- the model has \
         a shadow color but no offset or blur",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::button_warning.snap",
        "iced default: button::warning(theme, status).snap -- a renderer \
         setting, cfg!(feature = \"crisp\")",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::button_link.shadow",
        "iced default: button::text(theme, status).shadow -- the model has a \
         shadow color but no offset or blur",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::button_link.snap",
        "iced default: button::text(theme, status).snap -- a renderer \
         setting, cfg!(feature = \"crisp\")",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::button_link.border.color",
        "iced default: button::text(theme, status).border.color -- LinkTheme \
         carries no border",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::button_link.border.width",
        "iced default: button::text(theme, status).border.width -- LinkTheme \
         carries no border",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::button_link.border.radius.top_left",
        "iced default: button::text(theme, status).border.radius -- LinkTheme \
         carries no border",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::button_link.border.radius.top_right",
        "iced default: button::text(theme, status).border.radius -- LinkTheme \
         carries no border",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::button_link.border.radius.bottom_right",
        "iced default: button::text(theme, status).border.radius -- LinkTheme \
         carries no border",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::button_link.border.radius.bottom_left",
        "iced default: button::text(theme, status).border.radius -- LinkTheme \
         carries no border",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::text_input.icon",
        "iced default: text_input::default(theme, status).icon -- the model \
         carries no input-icon color",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::toggler.background_border_width",
        "iced default: toggler::default(theme, status).background_border_width \
         -- SwitchTheme carries no border",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::toggler.background_border_color",
        "iced default: toggler::default(theme, status).background_border_color \
         -- SwitchTheme carries no border",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::toggler.foreground_border_width",
        "iced default: toggler::default(theme, status).foreground_border_width \
         -- SwitchTheme carries no border",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::toggler.foreground_border_color",
        "iced default: toggler::default(theme, status).foreground_border_color \
         -- SwitchTheme carries no border",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::toggler.text_color",
        "iced default: toggler::default(theme, status).text_color -- the model \
         states no font for a switch, and iced's None inherits the surrounding \
         one",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::pick_list.handle_color",
        "iced default: pick_list::default(theme, status).handle_color -- \
         ComboBoxTheme states the arrow's sizes but not its color",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::menu.shadow",
        "iced default: overlay::menu::default(theme).shadow -- the model has a \
         shadow color but no offset or blur",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::slider.rail.border.color",
        "iced default: slider::default(theme, status).rail.border.color -- \
         SliderTheme carries no border",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::slider.rail.border.width",
        "iced default: slider::default(theme, status).rail.border.width -- \
         SliderTheme carries no border",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::slider.rail.border.radius.top_left",
        "iced default: slider::default(theme, status).rail.border.radius -- \
         SliderTheme carries no border",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::slider.rail.border.radius.top_right",
        "iced default: slider::default(theme, status).rail.border.radius -- \
         SliderTheme carries no border",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::slider.rail.border.radius.bottom_right",
        "iced default: slider::default(theme, status).rail.border.radius -- \
         SliderTheme carries no border",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::slider.rail.border.radius.bottom_left",
        "iced default: slider::default(theme, status).rail.border.radius -- \
         SliderTheme carries no border",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::slider.handle.border_width",
        "iced default: slider::default(theme, status).handle.border_width -- \
         SliderTheme carries no border",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::slider.handle.border_color",
        "iced default: slider::default(theme, status).handle.border_color -- \
         SliderTheme carries no border",
    ),
    // `scrollable::Style` nests a whole `container::Style`, two `Rail`s with a
    // `Scroller` each, and the autoscroll overlay. `ScrollbarTheme` states two
    // rail colors and three thumb colors and nothing else, so everything below
    // is iced's own, read from `scrollable::default(theme, status)`.
    #[cfg(feature = "widgets")]
    (
        "styles::scrollable.container.text_color",
        "iced default: scrollable::default(theme, status).container.text_color \
         -- ScrollbarTheme states nothing about the scrolled container",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::scrollable.container.background",
        "iced default: scrollable::default(theme, status).container.background \
         -- ScrollbarTheme states nothing about the scrolled container",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::scrollable.container.border.color",
        "iced default: scrollable::default(theme, status).container.border -- \
         ScrollbarTheme states nothing about the scrolled container",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::scrollable.container.border.width",
        "iced default: scrollable::default(theme, status).container.border -- \
         ScrollbarTheme states nothing about the scrolled container",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::scrollable.container.border.radius.top_left",
        "iced default: scrollable::default(theme, status).container.border -- \
         ScrollbarTheme states nothing about the scrolled container",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::scrollable.container.border.radius.top_right",
        "iced default: scrollable::default(theme, status).container.border -- \
         ScrollbarTheme states nothing about the scrolled container",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::scrollable.container.border.radius.bottom_right",
        "iced default: scrollable::default(theme, status).container.border -- \
         ScrollbarTheme states nothing about the scrolled container",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::scrollable.container.border.radius.bottom_left",
        "iced default: scrollable::default(theme, status).container.border -- \
         ScrollbarTheme states nothing about the scrolled container",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::scrollable.container.shadow",
        "iced default: scrollable::default(theme, status).container.shadow -- \
         ScrollbarTheme states nothing about the scrolled container",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::scrollable.container.snap",
        "iced default: scrollable::default(theme, status).container.snap -- a \
         renderer setting, cfg!(feature = \"crisp\")",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::scrollable.vertical_rail.border.color",
        "iced default: scrollable::default(theme, status).vertical_rail.border \
         -- ScrollbarTheme carries no border",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::scrollable.vertical_rail.border.width",
        "iced default: scrollable::default(theme, status).vertical_rail.border \
         -- ScrollbarTheme carries no border",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::scrollable.vertical_rail.border.radius.top_left",
        "iced default: scrollable::default(theme, status).vertical_rail.border \
         -- ScrollbarTheme carries no border",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::scrollable.vertical_rail.border.radius.top_right",
        "iced default: scrollable::default(theme, status).vertical_rail.border \
         -- ScrollbarTheme carries no border",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::scrollable.vertical_rail.border.radius.bottom_right",
        "iced default: scrollable::default(theme, status).vertical_rail.border \
         -- ScrollbarTheme carries no border",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::scrollable.vertical_rail.border.radius.bottom_left",
        "iced default: scrollable::default(theme, status).vertical_rail.border \
         -- ScrollbarTheme carries no border",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::scrollable.vertical_rail.scroller.border.color",
        "iced default: scrollable::default(theme, \
         status).vertical_rail.scroller.border -- ScrollbarTheme carries no \
         border",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::scrollable.vertical_rail.scroller.border.width",
        "iced default: scrollable::default(theme, \
         status).vertical_rail.scroller.border -- ScrollbarTheme carries no \
         border",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::scrollable.vertical_rail.scroller.border.radius.top_left",
        "iced default: scrollable::default(theme, \
         status).vertical_rail.scroller.border -- ScrollbarTheme carries no \
         border",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::scrollable.vertical_rail.scroller.border.radius.top_right",
        "iced default: scrollable::default(theme, \
         status).vertical_rail.scroller.border -- ScrollbarTheme carries no \
         border",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::scrollable.vertical_rail.scroller.border.radius.bottom_right",
        "iced default: scrollable::default(theme, \
         status).vertical_rail.scroller.border -- ScrollbarTheme carries no \
         border",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::scrollable.vertical_rail.scroller.border.radius.bottom_left",
        "iced default: scrollable::default(theme, \
         status).vertical_rail.scroller.border -- ScrollbarTheme carries no \
         border",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::scrollable.horizontal_rail.border.color",
        "iced default: scrollable::default(theme, \
         status).horizontal_rail.border -- ScrollbarTheme carries no border",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::scrollable.horizontal_rail.border.width",
        "iced default: scrollable::default(theme, \
         status).horizontal_rail.border -- ScrollbarTheme carries no border",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::scrollable.horizontal_rail.border.radius.top_left",
        "iced default: scrollable::default(theme, \
         status).horizontal_rail.border -- ScrollbarTheme carries no border",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::scrollable.horizontal_rail.border.radius.top_right",
        "iced default: scrollable::default(theme, \
         status).horizontal_rail.border -- ScrollbarTheme carries no border",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::scrollable.horizontal_rail.border.radius.bottom_right",
        "iced default: scrollable::default(theme, \
         status).horizontal_rail.border -- ScrollbarTheme carries no border",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::scrollable.horizontal_rail.border.radius.bottom_left",
        "iced default: scrollable::default(theme, \
         status).horizontal_rail.border -- ScrollbarTheme carries no border",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::scrollable.horizontal_rail.scroller.border.color",
        "iced default: scrollable::default(theme, \
         status).horizontal_rail.scroller.border -- ScrollbarTheme carries no \
         border",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::scrollable.horizontal_rail.scroller.border.width",
        "iced default: scrollable::default(theme, \
         status).horizontal_rail.scroller.border -- ScrollbarTheme carries no \
         border",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::scrollable.horizontal_rail.scroller.border.radius.top_left",
        "iced default: scrollable::default(theme, \
         status).horizontal_rail.scroller.border -- ScrollbarTheme carries no \
         border",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::scrollable.horizontal_rail.scroller.border.radius.top_right",
        "iced default: scrollable::default(theme, \
         status).horizontal_rail.scroller.border -- ScrollbarTheme carries no \
         border",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::scrollable.horizontal_rail.scroller.border.radius.bottom_right",
        "iced default: scrollable::default(theme, \
         status).horizontal_rail.scroller.border -- ScrollbarTheme carries no \
         border",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::scrollable.horizontal_rail.scroller.border.radius.bottom_left",
        "iced default: scrollable::default(theme, \
         status).horizontal_rail.scroller.border -- ScrollbarTheme carries no \
         border",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::scrollable.gap",
        "iced default: scrollable::default(theme, status).gap -- the fill \
         between a horizontal and a vertical scrollbar has no native \
         counterpart",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::scrollable.auto_scroll.background",
        "iced default: scrollable::default(theme, \
         status).auto_scroll.background -- the autoscroll overlay has no \
         native counterpart",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::scrollable.auto_scroll.border.color",
        "iced default: scrollable::default(theme, status).auto_scroll.border \
         -- the autoscroll overlay has no native counterpart",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::scrollable.auto_scroll.border.width",
        "iced default: scrollable::default(theme, status).auto_scroll.border \
         -- the autoscroll overlay has no native counterpart",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::scrollable.auto_scroll.border.radius.top_left",
        "iced default: scrollable::default(theme, status).auto_scroll.border \
         -- the autoscroll overlay has no native counterpart",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::scrollable.auto_scroll.border.radius.top_right",
        "iced default: scrollable::default(theme, status).auto_scroll.border \
         -- the autoscroll overlay has no native counterpart",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::scrollable.auto_scroll.border.radius.bottom_right",
        "iced default: scrollable::default(theme, status).auto_scroll.border \
         -- the autoscroll overlay has no native counterpart",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::scrollable.auto_scroll.border.radius.bottom_left",
        "iced default: scrollable::default(theme, status).auto_scroll.border \
         -- the autoscroll overlay has no native counterpart",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::scrollable.auto_scroll.shadow",
        "iced default: scrollable::default(theme, status).auto_scroll.shadow \
         -- the autoscroll overlay has no native counterpart",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::scrollable.auto_scroll.icon",
        "iced default: scrollable::default(theme, status).auto_scroll.icon -- \
         the autoscroll overlay has no native counterpart",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::rule.radius",
        "iced default: rule::default(theme).radius -- SeparatorTheme is a line \
         color and a line width",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::rule.fill_mode",
        "iced default: rule::default(theme).fill_mode -- how much of its \
         container a separator spans is the consumer's layout, and \
         SeparatorTheme states nothing about it",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::rule.snap",
        "iced default: rule::default(theme).snap -- a renderer setting",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::tooltip.shadow",
        "iced default: container::Style::default().shadow -- the model has a \
         shadow color but no offset or blur",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::tooltip.snap",
        "iced default: container::Style::default().snap -- a renderer setting, \
         cfg!(feature = \"crisp\")",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::container_card.text_color",
        "iced default: container::Style::default().text_color -- CardTheme \
         carries no font, and iced's None inherits the label color of \
         whatever the card sits in",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::container_card.shadow",
        "iced default: container::Style::default().shadow -- the model has a \
         shadow color but no offset or blur",
    ),
    #[cfg(feature = "widgets")]
    (
        "styles::container_card.snap",
        "iced default: container::Style::default().snap -- a renderer setting, \
         cfg!(feature = \"crisp\")",
    ),
    #[cfg(feature = "iced_aw")]
    (
        "styles::aw::menu.bar_shadow",
        "iced_aw default: menu_bar::primary(..).bar_shadow -- the model has a \
         shadow color but no offset or blur",
    ),
    #[cfg(feature = "iced_aw")]
    (
        "styles::aw::menu.menu_shadow",
        "iced_aw default: menu_bar::primary(..).menu_shadow -- the same",
    ),
    #[cfg(feature = "iced_aw")]
    (
        "styles::aw::menu.path.border.color",
        "iced_aw default: menu_bar::primary(..).path_border -- the model's one \
         menu border is the panel's, and drawing it around a highlighted row \
         would outline something the platform does not",
    ),
    #[cfg(feature = "iced_aw")]
    (
        "styles::aw::menu.path.border.width",
        "iced_aw default: menu_bar::primary(..).path_border -- the same",
    ),
    #[cfg(feature = "iced_aw")]
    (
        "styles::aw::menu.path.border.radius.top_left",
        "iced_aw default: menu_bar::primary(..).path_border -- the same",
    ),
    #[cfg(feature = "iced_aw")]
    (
        "styles::aw::menu.path.border.radius.top_right",
        "iced_aw default: menu_bar::primary(..).path_border -- the same",
    ),
    #[cfg(feature = "iced_aw")]
    (
        "styles::aw::menu.path.border.radius.bottom_right",
        "iced_aw default: menu_bar::primary(..).path_border -- the same",
    ),
    #[cfg(feature = "iced_aw")]
    (
        "styles::aw::menu.path.border.radius.bottom_left",
        "iced_aw default: menu_bar::primary(..).path_border -- the same",
    ),
    #[cfg(feature = "iced_aw")]
    (
        "styles::aw::tab_bar.border_color",
        "iced_aw default: tab_bar::primary(..).border_color -- TabTheme states \
         one border and it is the tab's, not the strip's",
    ),
    #[cfg(feature = "iced_aw")]
    (
        "styles::aw::tab_bar.border_width",
        "iced_aw default: tab_bar::primary(..).border_width -- the same",
    ),
    #[cfg(feature = "iced_aw")]
    (
        "styles::aw::tab_bar.icon_background",
        "iced_aw default: tab_bar::primary(..).icon_background -- the model \
         states no fill behind a tab's close icon",
    ),
    #[cfg(feature = "iced_aw")]
    (
        "styles::aw::tab_bar.icon_border_radius",
        "iced_aw default: tab_bar::primary(..).icon_border_radius -- the model \
         states no close icon at all, so nothing states its rounding",
    ),
    #[cfg(feature = "iced_aw")]
    (
        "styles::aw::sidebar.tab_label_border_color",
        "iced_aw default: sidebar::primary(..).tab_label_border_color -- \
         SidebarTheme states one border and it is the panel's, not the item's",
    ),
    #[cfg(feature = "iced_aw")]
    (
        "styles::aw::sidebar.tab_label_border_width",
        "iced_aw default: sidebar::primary(..).tab_label_border_width -- the \
         same",
    ),
    #[cfg(feature = "iced_aw")]
    (
        "styles::aw::sidebar.icon_background",
        "iced_aw default: sidebar::primary(..).icon_background -- the model \
         states no fill behind a sidebar item's close icon",
    ),
    #[cfg(feature = "iced_aw")]
    (
        "styles::aw::sidebar.icon_border_radius",
        "iced_aw default: sidebar::primary(..).icon_border_radius -- the same",
    ),
    // The container `styles::aw::spinner` styles exists to state an inherited
    // text color and nothing else, so every field but `text_color` is iced's
    // own and paints nothing (`container.rs:442-445` draws no quad at all
    // without a background, a border width or a shadow).
    #[cfg(feature = "iced_aw")]
    (
        "styles::aw::spinner.background",
        "iced default: container::Style::default().background -- a spinner is \
         painted on whatever the consumer puts it on, and the wrapper states \
         a color rather than a panel",
    ),
    #[cfg(feature = "iced_aw")]
    (
        "styles::aw::spinner.shadow",
        "iced default: container::Style::default().shadow -- the model has a \
         shadow color but no offset or blur",
    ),
    #[cfg(feature = "iced_aw")]
    (
        "styles::aw::spinner.snap",
        "iced default: container::Style::default().snap -- a renderer setting, \
         cfg!(feature = \"crisp\")",
    ),
    #[cfg(feature = "iced_aw")]
    (
        "styles::aw::spinner.border.color",
        "iced default: container::Style::default().border -- SpinnerTheme \
         carries no border",
    ),
    #[cfg(feature = "iced_aw")]
    (
        "styles::aw::spinner.border.width",
        "iced default: container::Style::default().border -- the same",
    ),
    #[cfg(feature = "iced_aw")]
    (
        "styles::aw::spinner.border.radius.top_left",
        "iced default: container::Style::default().border -- the same",
    ),
    #[cfg(feature = "iced_aw")]
    (
        "styles::aw::spinner.border.radius.top_right",
        "iced default: container::Style::default().border -- the same",
    ),
    #[cfg(feature = "iced_aw")]
    (
        "styles::aw::spinner.border.radius.bottom_right",
        "iced default: container::Style::default().border -- the same",
    ),
    #[cfg(feature = "iced_aw")]
    (
        "styles::aw::spinner.border.radius.bottom_left",
        "iced default: container::Style::default().border -- the same",
    ),
];

/// One native value iced cannot carry: its name, the evidence, and what each
/// preset states that iced does not draw.
pub(super) struct Unreachable {
    /// The value's path in the model -- or, for a `Style` field the connector
    /// fills and the widget never paints with, that field's emitted path.
    pub(super) field: &'static str,
    pub(super) evidence: &'static str,
    /// What the combination states that iced does not draw, as the report
    /// prints it; `None` where what iced draws is the platform's value anyway.
    pub(super) lost: fn(&Theme, &ResolvedTheme) -> Option<String>,
    /// The combinations where `lost` is `None`, each with its reason.
    pub(super) exceptions: &'static [(&'static str, &'static str)],
}

/// A length iced draws as nothing: lost wherever the platform states more.
fn lost_length(length: f32) -> Option<String> {
    (length > 0.0).then(|| format!("{length}px"))
}

/// A color iced has nowhere to paint: lost wherever the platform states it.
#[cfg(feature = "iced_aw")]
fn lost_color(color: native_theme::color::Rgba) -> Option<String> {
    Some(show(to_color(color)))
}

/// Native values iced 0.14 and `iced_aw` 0.14.1 have no receiver for, each
/// with its evidence.
///
/// Not a way out of a row: an entry here says the toolkit cannot carry the
/// value at all, so approximating it would state something untrue. The list
/// holds values with no receiver anywhere in iced or `iced_aw`, which is
/// narrower than what the `styles` doc comments call receiverless: those
/// speak of one widget's `Style`, and several of their values are carried
/// elsewhere or left to the consumer on purpose.
///
/// - `menu.font.color` and `.hover_text_color` have no receiver in `iced_aw`'s
///   `menu_bar::Style`, but iced's own menu takes both: `styles::menu` emits
///   them as the `PickList` / `ComboBox` menu's `text_color` and
///   `selected_text_color`.
/// - `menu.disabled_text_color`, `.separator_color`, `.row_height`,
///   `.icon_size` and `.icon_text_gap` belong to the menu's items, and an
///   `iced_aw` menu's items are the consumer's own widgets, built with their
///   own colors and sizes.
/// - A radio's disabled fields are the checkbox's (the model states one
///   `CheckboxTheme` for both) and have no receiver on a radio because
///   `radio::Status` has no disabled variant (`radio.rs:474-487`); on a
///   checkbox, `styles::checkbox` emits `.disabled_background` and
///   `.disabled_text_color`.
/// - `tab.min_width` and `.min_height` are left to the fixed extents
///   `iced_aw`'s `TabBar` takes (`tab_width(..)`, `height(..)`), the closest
///   receiver it has for a minimum, as `styles::aw::tab_bar` says.
/// - `sidebar.border.corner_radius` has no receiver either -- `iced_aw` rounds
///   the panel and its items with a hardcoded `(0.0).into()`
///   (`sidebar/sidebar.rs:616`, `:992`) -- yet every bundled preset states 0,
///   which is what `iced_aw` draws, so no preset loses it and
///   `every_unreachable_value_is_one_a_preset_loses` would reject the entry.
///
/// One entry names an emitted field rather than a native one:
/// `styles::aw::card.close_color` is filled, and `iced_aw` paints the close
/// icon with another class's.
///
/// The list is part of the accounting rather than beside it:
/// `every_named_field_has_exactly_one_declared_source` requires an unreachable
/// name to be claimed by no row *and* no `DERIVED` entry, so a value cannot be
/// called unreachable and mapped at the same time, and
/// `every_unreachable_value_is_one_a_preset_loses` reads each field through
/// `lost`, so an entry about a value the model no longer carries fails to
/// compile, and one no preset loses fails the test.
///
/// What does *not* belong here is native geometry whose only iced receiver is
/// a **builder method** rather than a `Style` field -- `Checkbox::spacing` for
/// `checkbox.label_gap`, `Toggler::size` for `switch.track_height`,
/// `pick_list::Handle::Arrow { size }` for `combo_box.arrow_icon_size`, a
/// menu's padding for `menu.row_height`. iced can carry those perfectly well;
/// they are simply the consumer's layout, set where the widget is built, and
/// `styles::*` returns a `Style`. They are neither unreachable nor a gap in
/// the contract, and the tripwire never sees them because it walks emitted
/// `Style` fields.
pub(super) const UNREACHABLE: &[Unreachable] = &[
    Unreachable {
        field: "scrollbar.min_thumb_length",
        evidence: "iced sizes the scroller itself and exposes no minimum: \
             `let scroller_length = (scrollbar_bounds.width * ratio).max(2.0);` \
             (`scrollable.rs:2068` horizontally, `:1997-1998` vertically), a \
             hardcoded floor under its own comment \"min width for easier \
             grabbing\". `Scrollbar` takes a width, a margin, a scroller width, \
             an anchor and a spacing (`scrollable.rs:342-387`) and no length at \
             all",
        lost: |_, r| lost_length(r.scrollbar.min_thumb_length),
        exceptions: &[],
    },
    Unreachable {
        field: "progress_bar.min_width",
        evidence: "`ProgressBar::length(..)` (`progress_bar.rs:91`) takes the \
             `Length` the consumer lays the bar out with, and nothing in the \
             widget states a floor under it",
        lost: |_, r| lost_length(r.progress_bar.min_width),
        exceptions: &[],
    },
    #[cfg(feature = "iced_aw")]
    Unreachable {
        field: "spinner.stroke_width",
        evidence: "`iced_aw`'s `Spinner` is four fields -- width, height, rate \
             and `circle_radius` (`spinner.rs:18-27`) -- and draws one filled \
             dot orbiting the centre, `fill_circle(renderer, position, \
             self.circle_radius, style.text_color)` (`spinner.rs:151`), not a \
             stroked arc, so there is no stroke to size",
        lost: |_, r| lost_length(r.spinner.stroke_width),
        exceptions: &[],
    },
    #[cfg(feature = "iced_aw")]
    Unreachable {
        field: "spinner.min_diameter",
        evidence: "the same four fields (`spinner.rs:18-27`): the orbit is half \
             the smaller side of the bounds the consumer gives it \
             (`spinner.rs:137-144`), and nothing states a minimum",
        lost: |_, r| lost_length(r.spinner.min_diameter),
        exceptions: &[],
    },
    #[cfg(feature = "iced_aw")]
    Unreachable {
        field: "list.alternate_row_background",
        evidence: "`selection_list::Style` is four fields -- text_color, \
             background, border_width, border_color \
             (`style/selection_list.rs:10-19`) -- and a row is filled only \
             while selected or hovered (`selection_list/list.rs:226-248`); \
             every other row shows the list's one background \
             (`selection_list.rs:302-315`), so there is no striping",
        lost: |_, r| lost_color(r.list.alternate_row_background),
        exceptions: &[],
    },
    #[cfg(feature = "iced_aw")]
    Unreachable {
        field: "list.header_background",
        evidence: "a `SelectionList` draws rows and nothing above them \
             (`selection_list/list.rs:215-275`): it has no column header, and \
             its `Style` no field for one (`style/selection_list.rs:10-19`)",
        lost: |_, r| lost_color(r.list.header_background),
        exceptions: &[],
    },
    #[cfg(feature = "iced_aw")]
    Unreachable {
        field: "list.header_font",
        evidence: "the same: a `SelectionList` has no column header to set \
             in it (`selection_list/list.rs:215-275`)",
        lost: |_, r| {
            let f = &r.list.header_font;
            Some(format!("{} {}px weight {}", f.family, f.size, f.weight))
        },
        exceptions: &[],
    },
    #[cfg(feature = "iced_aw")]
    Unreachable {
        field: "list.grid_color",
        evidence: "a `SelectionList` draws no line between its rows or \
             columns (`selection_list/list.rs:215-275`), and its `Style` has \
             no color for one (`style/selection_list.rs:10-19`)",
        lost: |_, r| lost_color(r.list.grid_color),
        exceptions: &[],
    },
    #[cfg(feature = "iced_aw")]
    Unreachable {
        field: "list.border.corner_radius",
        evidence: "`selection_list::Style` carries no radius, and `iced_aw` \
             outlines the list with a hardcoded `radius: (0.0).into()` \
             (`selection_list.rs:309`)",
        lost: |_, r| lost_length(r.list.border.corner_radius),
        exceptions: &[],
    },
    #[cfg(feature = "iced_aw")]
    Unreachable {
        field: "styles::aw::card.close_color",
        evidence: "set, and never painted: `Card::on_close` builds its close \
             button with a style closure that reads `iced_aw`'s *default* card \
             class, `<Theme as Catalog>::style(theme, &<Theme as \
             Catalog>::default(), Status::Active)`, and takes that style's \
             `close_color` as the button's `text_color` (`widget/card.rs:193-206`). \
             The card does hand the button the caller's `close_color` \
             (`widget/card.rs:942-948`), but an iced button ignores the style \
             it is handed (`_style`, `button.rs:368`) and draws its content in \
             its own style's `text_color` (`button.rs:401-407`). The default \
             class is `primary` (`style/card.rs:78-80`), whose close color is \
             `colors::WHITE` (`:89-91`, `:141-149`), so the icon is white on \
             every theme",
        lost: |t, r| {
            let drawn = iced_aw::style::card::primary(t, AwStatus::Active).close_color;
            let native = to_color(r.defaults.text_color);
            (native != drawn).then(|| format!("{} drawn as {}", show(native), show(drawn)))
        },
        exceptions: &[
            (
                "adwaita/dark",
                "defaults.text_color is white, the color iced_aw paints",
            ),
            ("windows-11/dark", "the same"),
            ("macos-sonoma/dark", "the same"),
            ("ios/dark", "the same"),
        ],
    },
];
