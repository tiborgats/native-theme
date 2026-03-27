// Per-widget struct pairs and macros
// Full define_widget_pair! macro and ResolvedFontSpec implemented in Task 2.

/// A resolved (non-optional) font specification produced after theme resolution.
#[derive(Clone, Debug, PartialEq)]
pub struct ResolvedFontSpec {
    /// Font family name.
    pub family: String,
    /// Font size in logical pixels.
    pub size: f32,
    /// CSS font weight (100–900).
    pub weight: u16,
}
