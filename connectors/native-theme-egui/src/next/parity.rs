//! Connector parity of the whole mapping (docs/v0.8.0-plan.md, section 1, rule 2; M4, step 1):
//! what the connector hands to egui, compiled from the 0.7.2 model by the crate root's path and
//! from the 0.8.0 model by [`super`], is equal on every preset of
//! [`super::root_parity::PRESETS`], light and dark.
//!
//! The 0.7.2 side is resolved as [`super::root_parity::resolve`] resolves it, and compiled with
//! the default preferences, whose text scale is 1, as the 0.8.0 theme is resolved for.
//!
//! The form of comparison per egui type the connector produces:
//!
//! - `egui::Style` derives `PartialEq` and `Debug`, and so do every type it holds that the
//!   mapping writes: `Visuals` (with `Widgets` and its five `WidgetVisuals`, `Selection`,
//!   `TextCursorStyle`, the two `Shadow`s), `Spacing` (with `ScrollStyle` and the `Margin`s),
//!   `Interaction`, `ScrollAnimation`, and the `text_styles` map of `FontId`s (egui 0.37.0
//!   `src/style.rs`). The base style and each role cell, in each of its four variants, are
//!   compared by `PartialEq`; a difference is reported by the lines in which their `Debug`
//!   forms (`{:#?}`) differ, so one listed difference names one leaf of the `Style`.
//! - `egui::Frame` derives `PartialEq` and `Debug` (egui 0.37.0 `src/containers/frame.rs`):
//!   each surface's frame is compared as the style is.
//! - The focus ring the install plugin paints is the crate's own `FocusRing` (its stroke, an
//!   `egui::Stroke`, its offset and whether it shows for the keyboard only), which derives
//!   `PartialEq` and `Debug`, compared as the style is.
//! - The fonts: `egui::FontDefinitions` are made from a `fonts::FontPlan`, which the
//!   application passes to the 0.7.2 builder; neither `to_theme` makes one, so none is
//!   compared. The text sizes the mapping writes are the `FontId`s of the styles above.
//! - The notes (`Note`) name the leaf whose value was sanitised, saturated or transparent, a
//!   0.7.2 leaf on one side and its 0.8.0 home on the other: not compared.
