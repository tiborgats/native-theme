//! Layers (S8): a layer's bases, its part for one preset, and its rules.

use super::error::LayerError;
use super::inputs::InputBases;
use super::presets::PresetId;
use super::roles::ColorRolesPatch;
use super::selector::Rule;
use super::tokens::TokensPatch;
use super::value::SchemeMap;
use super::widgets::WidgetsPatch;

/// A layer of the stack (S8): bases, a part for each preset it names, and
/// rules; empty by default.
#[derive(Clone, Debug, Default, PartialEq)]
#[non_exhaustive]
pub struct Layer {
    /// The bases every preset takes.
    pub bases: Bases,
    /// A preset's own bases, applied after the common bases while that
    /// preset is current, and validated only then.
    pub for_preset: Vec<(PresetId, Bases)>,
    /// The rules, in stated order.
    pub rules: Vec<Rule>,
    /// How the layer's problems name their places.
    pub(crate) spelled: Spelled,
}

/// How a layer's problems name their places (S1's `Problem.path`): by the
/// model's paths for a layer built in Rust, by the TOML keys it was read
/// under for one read from TOML.
#[derive(Clone, Copy, Debug, Default)]
pub(crate) enum Spelled {
    /// The model's paths.
    #[default]
    Model,
    /// The TOML keys.
    Toml,
}

/// How a layer was written is not what it states: two layers stating the
/// same are equal whichever way each was written.
impl PartialEq for Spelled {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl Layer {
    /// The application layer `text` states in S11's format, or every problem
    /// it has: TOML that does not parse, an unknown key (each with its TOML
    /// key path), a value of the wrong kind, a name the model does not have.
    ///
    /// ```
    /// use native_theme::next::Layer;
    ///
    /// let layer = Layer::from_toml(
    ///     r#"
    ///     [[rules]]
    ///     when = { states = ["hover"] }
    ///     target = "button.body"
    ///     set = { opacity = 0.5 }
    ///     "#,
    /// );
    /// assert!(layer.is_ok_and(|layer| layer.rules.len() == 1));
    /// assert!(Layer::from_toml("[widgets.button.body]\nshine = 1").is_err());
    /// ```
    pub fn from_toml(text: &str) -> Result<Layer, LayerError> {
        super::toml::layer::read_layer(text)
    }
}

/// A layer's bases (S8): colours per scheme, inputs, tokens and the typed
/// widgets' patches; empty by default.
#[derive(Clone, Debug, Default, PartialEq)]
#[non_exhaustive]
pub struct Bases {
    /// The colour roles, per scheme.
    pub colors: SchemeMap<ColorRolesPatch>,
    /// The inputs.
    pub inputs: InputBases,
    /// The tokens.
    pub tokens: TokensPatch,
    /// The typed widgets' parts and values.
    pub widgets: WidgetsPatch,
}
