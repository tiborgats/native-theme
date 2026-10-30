//! The typed widgets: the enums over the units `scripts/widget_units.toml`
//! enables, which `scripts/generate_widgets.py` writes from the catalogue
//! (S6.1). No unit is enabled yet, so each enum has no variant.

/// The typed widgets, one variant per widget of an enabled unit.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum WidgetKind {}

/// The catalogue's typed variants, one enum over the enabled units' widgets.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Variant {}

/// A typed widget's part: one variant per typed widget, each holding that
/// widget's part enum (`TypedPart::Button(ButtonPart::Label)`), so a part
/// paired with the wrong widget does not compile (S1).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TypedPart {}
