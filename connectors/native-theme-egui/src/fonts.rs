//! Font registration: the byte buffers and the plan of faces the atlas installs.

/// A byte buffer for one font face.
#[derive(Clone)]
#[non_exhaustive]
pub enum FontBytes {
    /// `&'static [u8]`, typically `include_bytes!`. Registered with `FontData::from_static`
    /// (`epaint/src/text/fonts.rs:125`), avoiding the per-`Fonts::new` copy that `Cow::Owned`
    /// incurs (`fonts.rs:391-396`).
    Static(&'static [u8]),
    /// A shared owned buffer. epaint's only constructors are `FontData::from_static`
    /// (`epaint/src/text/fonts.rs:125`) and `FontData::from_owned` (`:133`), so these bytes
    /// are copied into a `Vec<u8>` once when [`font_definitions`] builds the `FontData`, and
    /// again at every `FontsImpl::new` (`:391-396`). The `Arc` serves a [`FontPlan`] reused
    /// across rebuilds and `native_theme::fonts::SystemFace`'s `Arc<[u8]>`; it does **not**
    /// avoid epaint's copies. Prefer [`FontBytes::Static`] for bytes the application embeds.
    Shared(std::sync::Arc<[u8]>),
}

/// One registered face (spec §4.9): the inputs `select_face` describes it by (Task 22).
#[derive(Clone)]
pub(crate) struct PlannedFace {
    pub(crate) family: std::sync::Arc<str>,
    /// The face's own weight, or `None` for a variable face, which takes the theme's weight
    /// through its `wght` axis (§8.3).
    pub(crate) weight: Option<u16>,
    pub(crate) style: crate::FontStyle,
    pub(crate) bytes: FontBytes,
}

/// Which faces the application has bytes for, and the choice among them.
///
/// The choice is made by `native_theme::fonts::select_face` (§8.2), the pure function
/// native-theme exports without a feature and `native_theme::fonts::system_face` also uses:
/// each registered face is described to it as a `native_theme::fonts::FaceTraits` (its
/// family, weight and style as registered, and width `5`, normal, since a registered face
/// states no width), and it applies CSS Fonts Level 4 §5.1 caseless
/// family matching, never substituting another family, then §5.2's width, style and weight
/// steps. So an application's faces and the system's are matched by one rule. A
/// request with no face of its family is a [`crate::Note::FontFamilyUnavailable`] in
/// [`crate::ThemeAtlas::notes`]; nothing is ever invented. Faces registered beyond the two
/// the theme selects are unused and cost nothing.
#[derive(Clone, Default)]
#[must_use]
pub struct FontPlan {
    pub(crate) faces: Vec<PlannedFace>,
    /// `FontPlan::with_base`'s definitions (Task 22); `None` means `FontDefinitions::default()`.
    pub(crate) base: Option<egui::FontDefinitions>,
    /// The lookup notes `FontPlan::from_system` carries (Task 22); `Builder::build` moves them
    /// into the atlas's notes.
    pub(crate) notes: Vec<crate::Note>,
}

impl FontPlan {
    /// An empty plan. An atlas built with it installs the plan's base alone (§10.3 step 1).
    pub fn new() -> Self {
        Self::default()
    }

    /// Register one face at its own `weight` (a CSS weight in `100..=900`,
    /// `native-theme/src/model/font.rs:267-268`; values outside that range are clamped into
    /// it, never rejected) and `style`. `family` is matched against `ResolvedFontSpec::family`
    /// (`native-theme/src/model/font.rs:244`) by the rule on [`FontPlan`].
    pub fn face(
        mut self,
        family: &str,
        weight: u16,
        style: crate::FontStyle,
        bytes: FontBytes,
    ) -> Self {
        self.faces.push(PlannedFace {
            family: family.into(),
            weight: Some(weight.clamp(100, 900)),
            style,
            bytes,
        });
        self
    }

    /// Register one **variable** face, letting this crate set the `wght` axis to the weight the
    /// theme asks for instead of requiring one file per weight. It is described to
    /// `select_face` at the weight asked for, which its axis reaches, so within its family and
    /// style it matches exactly. The coordinate is applied through `FontTweak::coords`
    /// (`epaint/src/text/fonts.rs:250`) with the infallible `Tag::new(b"wght")` (§8.3). A face
    /// registered here that has no `wght` axis (§8.3) renders at its own weight and is a
    /// [`crate::Note::FontWeightAxisUnsupported`].
    pub fn variable_face(
        mut self,
        family: &str,
        style: crate::FontStyle,
        bytes: FontBytes,
    ) -> Self {
        self.faces.push(PlannedFace {
            family: family.into(),
            weight: None,
            style,
            bytes,
        });
        self
    }
}

/// egui's row height for the head face of `defs`' `Proportional` chain at `size` points,
/// computed as epaint computes it (§6.15): the face's unscaled metrics at the default
/// variation location (`epaint/src/text/font.rs:397-400`), scaled by
/// `size · tweak.scale / units_per_em` (`:561`, `:215-218`); ascent, descent and line gap each
/// rounded with `round_ui` (`:563-565`) and summed (`:587`). epaint measures a family by the
/// first face of its chain (`:697-702`, `epaint/src/text/fonts.rs:865-875`). `None` for an
/// empty chain, a face epaint cannot parse, or a row that is not finite and positive (a zero
/// `units_per_em`, which §8.2 drops before it gets here; kept for totality).
pub(crate) fn body_row_height(defs: &egui::FontDefinitions, size: f32) -> Option<f32> {
    use egui::emath::GuiRounding as _;
    use skrifa::MetadataProvider as _;

    let name = defs
        .families
        .get(&egui::FontFamily::Proportional)?
        .first()?;
    let data = defs.font_data.get(name)?;
    let face = skrifa::FontRef::from_index(data.font.as_ref(), data.index).ok()?;
    let m = face.metrics(
        skrifa::instance::Size::unscaled(),
        skrifa::instance::LocationRef::default(),
    );
    let k = size * data.tweak.scale / f32::from(m.units_per_em);
    let row = (m.ascent * k).round_ui() - (m.descent * k).round_ui() + (m.leading * k).round_ui();
    (row.is_finite() && row > 0.0).then_some(row)
}

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    reason = "a test fails by panicking"
)]
mod tests {
    use super::*;
    use crate::install_tests::pass;

    /// §6.15, T5's oracle: for egui's own faces the recomputation equals egui's `row_height`
    /// (`epaint/src/text/fonts.rs:865-875`) bit for bit. Fonts exist only after the first
    /// pass (`egui/src/context.rs:1114-1122`).
    #[test]
    fn body_row_height_is_egui_row_height_for_the_default_faces() {
        let ctx = egui::Context::default();
        let _ = pass(&ctx, egui::RawInput::default(), |_ui| {}); // fonts exist after one pass; `pass` clears its texture delta
        let defs = egui::FontDefinitions::default();
        for size in [9.0_f32, 13.0, 18.0, 26.0, 11.5] {
            let want = ctx.fonts_mut(|f| f.row_height(&egui::FontId::proportional(size)));
            assert_eq!(body_row_height(&defs, size), Some(want), "size {size}");
        }
        assert_eq!(body_row_height(&egui::FontDefinitions::empty(), 13.0), None);
    }
}
