//! The text input, `input` and `text_area` (spec §5).

use super::{PartRules, TextRules, WidgetOrigin, unstated_part, unstated_text};
use crate::next::query::{InputQuery, InputVariant};
use crate::next::style::{PartStyle, TextStyle};

/// A single-line text field (`input`) or a multi-line one (`text_area`).
/// The caret is the theme's (`ResolvedTheme::caret`, `colors.caret`).
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct InputWidget {
    /// Native or synthesized.
    pub origin: WidgetOrigin,
    /// The opacity of the whole disabled field drawn as one group; `None`
    /// where no source states it.
    pub disabled_opacity: Option<f32>,
    pub(super) field: PartRules<Option<InputVariant>>,
    pub(super) text: TextRules<Option<InputVariant>>,
    pub(super) placeholder: TextRules<Option<InputVariant>>,
    pub(super) helper_text: TextRules<Option<InputVariant>>,
}

/// The parts of a text input for one query.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct InputParts {
    /// The field's frame and background.
    pub field: PartStyle,
    /// The entered text.
    pub text: TextStyle,
    /// The placeholder shown while the field is empty.
    pub placeholder: TextStyle,
    /// The helper text beside the field.
    pub helper_text: TextStyle,
}

impl InputWidget {
    /// A text input of these rule sets and metrics.
    pub(crate) fn new(
        origin: WidgetOrigin,
        field: PartRules<Option<InputVariant>>,
        text: TextRules<Option<InputVariant>>,
        placeholder: TextRules<Option<InputVariant>>,
        helper_text: TextRules<Option<InputVariant>>,
        disabled_opacity: Option<f32>,
    ) -> Self {
        Self {
            origin,
            disabled_opacity,
            field,
            text,
            placeholder,
            helper_text,
        }
    }

    /// A text input that states nothing: every part's base style empty, no
    /// rule, no metric.
    pub(crate) fn unstated(origin: WidgetOrigin) -> Self {
        Self::new(
            origin,
            unstated_part(),
            unstated_text(),
            unstated_text(),
            unstated_text(),
            None,
        )
    }

    /// Whether the text input is the platform's own control.
    #[must_use]
    pub fn is_native(&self) -> bool {
        self.origin == WidgetOrigin::Native
    }

    /// The field's style for `query`.
    #[must_use]
    pub fn field(&self, query: impl Into<InputQuery>) -> PartStyle {
        self.field.eval(&query.into())
    }

    /// The entered text's style for `query`.
    #[must_use]
    pub fn text(&self, query: impl Into<InputQuery>) -> TextStyle {
        self.text.eval(&query.into())
    }

    /// The placeholder's style for `query`.
    #[must_use]
    pub fn placeholder(&self, query: impl Into<InputQuery>) -> TextStyle {
        self.placeholder.eval(&query.into())
    }

    /// The helper text's style for `query`.
    #[must_use]
    pub fn helper_text(&self, query: impl Into<InputQuery>) -> TextStyle {
        self.helper_text.eval(&query.into())
    }

    /// Every part's style for `query`.
    #[must_use]
    pub fn parts(&self, query: impl Into<InputQuery>) -> InputParts {
        let query = query.into();
        InputParts {
            field: self.field.eval(&query),
            text: self.text.eval(&query),
            placeholder: self.placeholder.eval(&query),
            helper_text: self.helper_text.eval(&query),
        }
    }
}
