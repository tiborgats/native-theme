//! egui widgets drawn from a native theme, for applications themed with
//! [`native-theme-egui`](native_theme_egui).
//!
//! Target: **egui 0.36.2**.
//!
//! A companion widget crate for the connector. It draws what the connector
//! cannot give egui's own widgets: a switch, a slider whose knob is not its
//! rail, a spinner at the theme's stroke, and a segmented control, which egui
//! does not have. It also wraps egui's links in their state and visited
//! colours, the per-call route the connector names for them, written once,
//! egui's drop-down at the height the theme states, which egui's square
//! arrow box can exceed, with the open chevron the platforms draw, egui's
//! radio button with the dot the theme states, which egui sizes from the
//! check mark's box, egui's progress bar with the outline the theme
//! states, which egui does not draw, and an expander whose arrow side, gap,
//! body indent and frame are the theme's, which egui's `CollapsingHeader`
//! ties to one indent and one side.
//!
//! Everything it draws is driven by the `ResolvedTheme` of the atlas the
//! connector installed ([`ThemeAtlas::from_ctx`](native_theme_egui::ThemeAtlas::from_ctx)),
//! in the colour scheme egui is drawing. It contains no colour, no radius and
//! no metric of its own: a size the theme leaves unstated is the value egui
//! itself uses there. **With no atlas installed, every widget renders as
//! egui's own counterpart** — [`switch::Switch`] as `egui::Checkbox`,
//! [`slider::Slider`] as `egui::Slider`, [`spinner::Spinner`] as
//! `egui::Spinner`, [`segmented_control::SegmentedControl`] as unscoped
//! buttons, the [`wrap`] functions as egui's `Link` and `Hyperlink`,
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
//! Every widget is an [`egui::Widget`](native_theme_egui::egui::Widget) added
//! with `ui.add(..)`, so `ui.add_sized` and `ui.add_enabled` take them too —
//! but the drop-down and the expander, whose contents are a closure: as
//! egui's own, they are shown with [`combo_box::ComboBox::show_ui`] and
//! [`expander::Expander::show`]. No
//! extension trait on `egui::Ui` is provided: `Ui::button` and its siblings
//! are inherent methods, which Rust resolves before a trait's, so a trait
//! method of the same name would be silently ignored at every call site.
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
//! A [`switch::Switch`] or [`radio_button::RadioButton`] built with
//! `.enabled(false)` is faded as one widget over its backdrop, the window's
//! `defaults.background_color` unless `.backdrop(..)` names another
//! ([`fade::fade_as_one`]): its thumb or dot keeps its colour on the track or
//! circle, and the whole fades, as libadwaita's `filter: Opacity(..)` does,
//! where egui's per-shape fade would show the thumb through the faded track.

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
pub mod parts;
pub mod progress_bar;
pub mod radio_button;
pub mod segmented_control;
pub mod slider;
pub mod spinner;
pub mod switch;
pub mod wrap; // Tier W: `link`, `hyperlink`

mod scope;

pub use native_theme_egui as connector;

#[cfg(test)]
mod tests;
