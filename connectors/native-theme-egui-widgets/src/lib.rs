//! egui widgets drawn from a native theme, for applications themed with
//! [`native-theme-egui`](native_theme_egui).
//!
//! Target: **egui 0.36.2**.
//!
//! A companion widget crate for the connector. The connector themes egui's own
//! widgets through egui's `Style`; this crate draws what no `Style` can produce,
//! because egui draws it from a hardcoded value or not at all:
//!
//! * [`switch::Switch`]: a switch; egui has no switch or toggle widget.
//! * [`slider::Slider`]: a slider whose knob has its own colour; egui paints the
//!   resting knob in the rail's.
//! * [`spinner::Spinner`]: the icon set's animated indicator where it has one, else
//!   an arc at the theme's stroke width, which `egui::Spinner` hardcodes.
//! * [`segmented_control::SegmentedControl`]: a segmented control; egui has none.
//! * [`links::link`] and [`links::hyperlink`]: egui's links in their hover,
//!   pressed, disabled and visited colours, the per-call colours the connector's
//!   mapping names for them, written once.
//! * [`combo_box::ComboBox`]: egui's drop-down at the height the theme states,
//!   which egui's square arrow box can exceed, with the open chevron the
//!   platforms draw where egui draws a filled triangle.
//! * [`radio_button::RadioButton`]: egui's radio button with the dot the theme
//!   states; egui sizes it from the check mark's box.
//! * [`progress_bar::ProgressBar`]: egui's progress bar with the outline the
//!   theme states; egui draws none.
//! * [`expander::Expander`]: an expander whose arrow side, arrow gap, body indent
//!   and frame are the theme's; egui's `CollapsingHeader` ties the arrow, the
//!   title and the body to one indent, leads with its arrow and frames the header
//!   alone.
//!
//! Add it beside the connector and install the connector's atlas as its crate
//! documentation shows ([`from_system`](native_theme_egui::from_system) or
//! [`from_preset`](native_theme_egui::from_preset), then
//! [`ThemeAtlas::install`](native_theme_egui::ThemeAtlas::install)); the
//! connector is re-exported as [`connector`], so its `egui` is
//! `native_theme_egui_widgets::connector::egui`.
//!
//! Everything it draws is driven by the `ResolvedTheme` of the atlas the
//! connector installed ([`ThemeAtlas::from_ctx`](native_theme_egui::ThemeAtlas::from_ctx)),
//! in the colour scheme egui is drawing. It contains no colour, no radius and
//! no metric of its own: a size the theme leaves unstated is the value egui
//! itself uses there. **With no atlas installed, every widget renders as
//! egui's own counterpart** — [`switch::Switch`] as `egui::Checkbox`,
//! [`slider::Slider`] as `egui::Slider`, [`spinner::Spinner`] as
//! `egui::Spinner`, [`segmented_control::SegmentedControl`] as unscoped
//! buttons, the [`links`] functions as egui's `Link` and `Hyperlink`,
//! [`combo_box::ComboBox`] as `egui::ComboBox`, [`radio_button::RadioButton`]
//! as `egui::RadioButton`, [`progress_bar::ProgressBar`] as
//! `egui::ProgressBar`, [`expander::Expander`] as `egui::CollapsingHeader`.
//!
//! ```rust,no_run
//! use native_theme_egui_widgets::connector::egui;
//! use native_theme_egui_widgets::switch::Switch;
//!
//! fn settings(ui: &mut egui::Ui, wifi_on: &mut bool) {
//!     ui.add(Switch::new(wifi_on).label("Wi-Fi"));
//! }
//! ```
//!
//! Every widget but the drop-down and the expander is an
//! [`egui::Widget`](native_theme_egui::egui::Widget) added with `ui.add(..)`, so
//! `ui.add_sized` and `ui.add_enabled` take them too. The drop-down and the
//! expander take their contents as a closure and, as egui's own, are shown with
//! [`combo_box::ComboBox::show_ui`] and [`expander::Expander::show`]; the links
//! are functions returning an `egui::Widget`. No extension trait on `egui::Ui`
//! is provided: `Ui::button` and its siblings are inherent methods, which Rust
//! resolves before a trait's, so a trait method of the same name would be
//! silently ignored at every call site.
//!
//! # Spec citations
//!
//! A `spec §N` in these docs cites section N of the crate's design specification,
//! [`docs/archive/todo_egui-widgets-spec.md`](https://github.com/tiborgats/native-theme/blob/main/docs/archive/todo_egui-widgets-spec.md)
//! in the repository, which the published crate does not include; a citation of another
//! document, such as `docs/platform-facts.md`, names it.
//!
//! # Tiers
//!
//! Each widget's documentation names its tier — what this crate owns, and so
//! re-audits every egui release: **W** (wrapped: one egui widget in a role
//! scope), **C** (composed: several egui widgets arranged into one API) or
//! **P** (painted: allocated, sensed and painted here, admitted only with the
//! upstream line that makes a cheaper tier insufficient).
//!
//! # Disabled
//!
//! A widget built with `.enabled(false)`, or added to a disabled `Ui`, opens
//! its role's `Disabled` scope and calls `Ui::disable` inside it: egui's own
//! disabled semantics (no click or drag, no focus, `enabled: false` for
//! assistive technology), the platform's disabled colours, and egui's fade
//! by the scope's `disabled_alpha`, the role's `disabled_opacity`. A platform
//! dims by one of the two, and its data makes the other an identity
//! (`docs/platform-facts.md` §2.1.6): 1.0 on KDE and Windows, no disabled
//! colours of its own on GNOME. Under `ui.add_enabled(false, w)` the fade is
//! the calling `Ui`'s, which egui applied before the widget's scope opened.
//!
//! A [`switch::Switch`], [`radio_button::RadioButton`] or [`slider::Slider`]
//! built with `.enabled(false)` or added to a disabled `Ui` is faded as one
//! widget over its backdrop, the window's `defaults.background_color` unless
//! `.backdrop(..)` names another ([`fade::fade_as_one`]): its thumb or dot
//! keeps its colour on the track or circle, a knob its own over the backdrop,
//! and the whole fades, as libadwaita's `filter: Opacity(..)` does, where
//! egui's per-shape fade would show the thumb through the faded track. Under
//! a disabled `Ui` the fade is that `Ui`'s `disabled_alpha`, the one level of
//! fade egui gave the painter taken back; a `Ui` disabled twice, or restyled
//! after it was disabled, keeps the rest. A
//! [`segmented_control::SegmentedControl`] (which has no `.enabled(..)`) added
//! to a disabled `Ui` is faded as one widget the same way, over its backdrop,
//! but by its own `segmented_control.disabled_opacity`, the `Ui`'s fade taken
//! back. A checked radio button opens the connector's
//! `SelectedDisabled` scope: where
//! the theme states no `checkbox.disabled_background` it is its checked self,
//! the checked fill and its dot, faded.

#![warn(missing_docs)]
#![forbid(unsafe_code)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::indexing_slicing)]
#![deny(clippy::panic)]
#![deny(clippy::unreachable)]
#![deny(clippy::todo)]
#![deny(clippy::unimplemented)]

pub mod combo_box;
pub mod expander;
pub mod fade;
pub mod links; // Tier W: `link`, `hyperlink`
pub mod parts;
pub mod progress_bar;
pub mod radio_button;
pub mod segmented_control;
pub mod slider;
pub mod spinner;
pub mod switch;

mod scope;

pub use native_theme_egui as connector;

#[cfg(test)]
mod tests;

/// The README's code blocks, compiled as doctests so they cannot drift from the API.
#[cfg(doctest)]
#[doc = include_str!("../README.md")]
pub struct ReadmeDoctests;
