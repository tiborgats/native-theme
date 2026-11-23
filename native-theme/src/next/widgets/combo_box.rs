//! The combo box (spec §5).

use super::{
    IconRules, PartRules, TextRules, WidgetOrigin, unstated_icon, unstated_part, unstated_text,
};
use crate::next::query::StyleQuery;
use crate::next::style::{IconStyle, PartStyle, TextStyle};

/// A combo box, the button that opens a drop-down list: its field, the
/// current choice's label and the chevron. The open combo box is the one
/// queried with `OPEN`.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct ComboBoxWidget {
    /// Native or synthesized.
    pub origin: WidgetOrigin,
    /// The width of the area at the field's end that holds the chevron, in
    /// logical pixels; `None` where no source states it.
    pub arrow_area_width: Option<f32>,
    /// The opacity of the whole disabled combo box drawn as one group;
    /// `None` where no source states it.
    pub disabled_opacity: Option<f32>,
    pub(crate) field: PartRules,
    pub(crate) label: TextRules,
    pub(crate) chevron: IconRules,
}

/// The parts of a combo box for one query.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct ComboBoxParts {
    /// The field.
    pub field: PartStyle,
    /// The current choice's text.
    pub label: TextStyle,
    /// The chevron.
    pub chevron: IconStyle,
}

impl ComboBoxWidget {
    /// A combo box of these rule sets and metrics.
    pub(crate) fn new(
        origin: WidgetOrigin,
        field: PartRules,
        label: TextRules,
        chevron: IconRules,
        arrow_area_width: Option<f32>,
        disabled_opacity: Option<f32>,
    ) -> Self {
        Self {
            origin,
            arrow_area_width,
            disabled_opacity,
            field,
            label,
            chevron,
        }
    }

    /// A combo box that states nothing: every part's base style empty, no
    /// rule, no metric.
    pub(crate) fn unstated(origin: WidgetOrigin) -> Self {
        Self::new(
            origin,
            unstated_part(),
            unstated_text(),
            unstated_icon(),
            None,
            None,
        )
    }

    /// Whether the combo box is the platform's own control.
    #[must_use]
    pub fn is_native(&self) -> bool {
        self.origin == WidgetOrigin::Native
    }

    /// The field's style for `query`.
    #[must_use]
    pub fn field(&self, query: impl Into<StyleQuery>) -> PartStyle {
        self.field.eval(&query.into())
    }

    /// The label's style for `query`.
    #[must_use]
    pub fn label(&self, query: impl Into<StyleQuery>) -> TextStyle {
        self.label.eval(&query.into())
    }

    /// The chevron's style for `query`.
    #[must_use]
    pub fn chevron(&self, query: impl Into<StyleQuery>) -> IconStyle {
        self.chevron.eval(&query.into())
    }

    /// Every part's style for `query`.
    #[must_use]
    pub fn parts(&self, query: impl Into<StyleQuery>) -> ComboBoxParts {
        let query = query.into();
        ComboBoxParts {
            field: self.field.eval(&query),
            label: self.label.eval(&query),
            chevron: self.chevron.eval(&query),
        }
    }

    /// The dead rules of each rule set, named `combo_box.<part>`.
    #[cfg(test)]
    pub(crate) fn dead_rules(&self) -> Vec<(&'static str, Vec<usize>)> {
        vec![
            ("combo_box.field", self.field.dead_rules()),
            ("combo_box.label", self.label.dead_rules()),
            ("combo_box.chevron", self.chevron.dead_rules()),
        ]
    }
}
