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
    /// The shorthands a layer read from TOML was written with.
    pub(crate) shorthands: Shorthands,
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

/// The shorthands a layer read from TOML was written with (S11): a key
/// standing for several places (`corners_px`, a stroke's `width_px`) or a
/// form folding a place's steps (a colour for a paint layer). A problem at
/// a place one stands for is the shorthand's own, at its key as written:
/// one fault, one problem (S12).
#[derive(Clone, Debug, Default)]
pub(crate) struct Shorthands(Vec<(String, String)>);

impl Shorthands {
    /// Records that the key written at `written` stands for the place
    /// `place`, spelled as a walk spells it.
    pub(crate) fn add(&mut self, place: String, written: &str) {
        self.0.push((place, written.to_owned()));
    }

    /// `at`, or, where it lies at or below a place a shorthand stands for,
    /// the shorthand's key with the rest of `at` below it; a shorthand
    /// within another (a colour for a paint's one layer) folded too.
    pub(crate) fn written(&self, at: &str) -> String {
        let mut at = at.to_owned();
        for _ in 0..self.0.len() {
            let folded = self.0.iter().find_map(|(place, written)| {
                at.strip_prefix(place.as_str())
                    .filter(|rest| rest.is_empty() || rest.starts_with(['.', '[']))
                    .map(|rest| format!("{written}{rest}"))
            });
            match folded {
                Some(folded) => at = folded,
                None => break,
            }
        }
        at
    }
}

/// How a layer was written is not what it states.
impl PartialEq for Shorthands {
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
