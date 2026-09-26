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
