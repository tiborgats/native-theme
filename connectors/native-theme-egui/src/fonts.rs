//! Font registration: a face enters epaint only as bytes, never by an OS family name (§8.2);
//! this crate never emits a `FontFamily::Name`, so at most two faces are installed — the one
//! matching `defaults.font` as `FontFamily::Proportional` and the one matching
//! `defaults.mono_font` as `FontFamily::Monospace` — and one weight per family (§8.1, §8.3);
//! and every face is validated with epaint's own parse before epaint sees it, so epaint's
//! release-mode parse panic is unreachable (§8.2). This crate reads no file itself: the bytes
//! come from the application ([`FontPlan::face`]) or, with feature `system-fonts`, from
//! native-theme (`FontPlan::from_system`, a plain name: an intra-doc link to a feature-gated
//! item breaks the documentation built without the feature).

use std::sync::Arc;

use egui::epaint::text::{Tag, VariationCoords};
use egui::{FontData, FontDefinitions, FontFamily};
use native_theme::fonts::{FaceTraits, select_face};
use native_theme::theme::{FontStyle, ResolvedFontSpec, ResolvedTheme};

use crate::Note;
use crate::style::push_note;

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

/// One registered face. `weight` is `Some` for a [`FontPlan::face`], the face's own weight,
/// and `None` for a [`FontPlan::variable_face`], which is described to `select_face` at the
/// weight asked for. `index` is the face's index in a collection file: `0` for an
/// application face, `SystemFace::index` for a system one.
#[derive(Clone)]
pub(crate) struct PlannedFace {
    pub(crate) family: Arc<str>,
    pub(crate) weight: Option<u16>,
    pub(crate) style: FontStyle,
    pub(crate) bytes: FontBytes,
    pub(crate) index: u32,
}

impl PlannedFace {
    fn bytes(&self) -> &[u8] {
        match &self.bytes {
            FontBytes::Static(bytes) => bytes,
            FontBytes::Shared(bytes) => bytes,
        }
    }
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
    faces: Vec<PlannedFace>,
    base: Option<FontDefinitions>,
    /// The lookup notes of [`FontPlan::from_system`], returned by [`font_definitions`].
    notes: Vec<Note>,
}

/// The `font_data` key of the face this crate prepends to a chain: one label per chain, so the
/// two can never collide with each other, and a name no egui default uses.
const PROPORTIONAL_KEY: &str = "native-theme-egui/proportional";
const MONOSPACE_KEY: &str = "native-theme-egui/monospace";

impl FontPlan {
    /// An empty plan. An atlas built with it installs the plan's base alone (§10.3 step 1).
    pub fn new() -> Self {
        Self::default()
    }

    /// Register one face at its own `weight` (a CSS weight in `100..=900`,
    /// `native-theme/src/model/font.rs:267-268`; values outside that range are clamped into
    /// it, never rejected) and `style`. `family` is matched against `ResolvedFontSpec::family`
    /// (`native-theme/src/model/font.rs:244`) by the rule on [`FontPlan`].
    pub fn face(self, family: &str, weight: u16, style: FontStyle, bytes: FontBytes) -> Self {
        // `100..=900`: values outside the range are clamped into it, never rejected (§4.9).
        self.face_at(family, Some(weight.clamp(100, 900)), style, bytes, 0)
    }

    /// Register one **variable** face, letting this crate set the `wght` axis to the weight the
    /// theme asks for instead of requiring one file per weight. It is described to
    /// `select_face` at the weight asked for, which its axis reaches, so within its family and
    /// style it matches exactly. The coordinate is applied through `FontTweak::coords`
    /// (`epaint/src/text/fonts.rs:250`) with the infallible `Tag::new(b"wght")` (§8.3). A face
    /// registered here that has no `wght` axis (§8.3) renders at its own weight and is a
    /// [`crate::Note::FontWeightAxisUnsupported`].
    pub fn variable_face(self, family: &str, style: FontStyle, bytes: FontBytes) -> Self {
        self.face_at(family, None, style, bytes, 0)
    }

    /// Register a face at a collection `index`; the route [`FontPlan::from_system`] takes for a
    /// `SystemFace`, and the one T6 (b) uses to address a single face at index `1`.
    pub(crate) fn face_at(
        mut self,
        family: &str,
        weight: Option<u16>,
        style: FontStyle,
        bytes: FontBytes,
        index: u32,
    ) -> Self {
        self.faces.push(PlannedFace {
            family: Arc::from(family),
            weight,
            style,
            bytes,
            index,
        });
        self
    }

    /// How many faces the plan registers (T6 (c)); test-only, as no library code asks.
    #[cfg(all(test, feature = "system-fonts"))]
    pub(crate) fn face_count(&self) -> usize {
        self.faces.len()
    }

    /// The OS's own faces for the theme's two families — `defaults.font` and
    /// `defaults.mono_font`, each asked for at its `weight` and `style` — from
    /// `native_theme::fonts::system_face` (§8.2), whose `SystemFace` carries the chosen face's
    /// bytes, its index in a collection file, and its own `family`, `weight` and `style` as
    /// fontdb records them; this crate reads the bytes, the weight and the style. Each face is registered at that weight and style — the face's own, not
    /// the ones asked for. A face with a `wght` axis
    /// gets the theme's weight as a coordinate (§8.3); a face whose weight differs from the
    /// theme's and has no `wght` axis renders at its own weight and is a
    /// [`crate::Note::FontWeightAxisUnsupported`]; a family the OS has no face of is a
    /// [`crate::Note::FontFamilyUnavailable`] and keeps egui's own face. The plan carries
    /// these lookup notes, and [`font_definitions`] returns them with its own, so they reach
    /// [`crate::ThemeAtlas::notes`] through [`crate::Builder::build`]. Feature
    /// `system-fonts`, which enables native-theme's own `system-fonts` feature. That the names
    /// the macOS and Windows readers report resolve is verified on those platforms' CI
    /// runners (§8.2, §13).
    ///
    /// No `#[must_use]` of its own: `FontPlan` is `#[must_use]`, and a second one on a function
    /// returning it trips `clippy::double_must_use`.
    #[cfg(feature = "system-fonts")]
    pub fn from_system(t: &ResolvedTheme) -> Self {
        let mut plan = Self::new();
        for spec in [&t.defaults.font, &t.defaults.mono_font] {
            match native_theme::fonts::system_face(&spec.family, spec.weight, spec.style) {
                // Registered under the family the theme asked for — `system_face` already
                // matched it — at the face's own weight and style (§8.2), so
                // `font_definitions`' `select_face` pass finds it again; fontdb's own name for
                // the face (`.SF NS` for "SF Pro") need not equal the theme's.
                Some(face) => {
                    plan = plan.face_at(
                        &spec.family,
                        Some(face.weight),
                        face.style,
                        FontBytes::Shared(face.data),
                        face.index,
                    );
                }
                None => push_note(
                    &mut plan.notes,
                    Note::FontFamilyUnavailable {
                        family: Arc::clone(&spec.family),
                    },
                ),
            }
        }
        plan
    }

    /// The `FontDefinitions` the plan's faces are prepended to — by [`crate::ThemeAtlas::install`]
    /// (§10.3 step 1) and by [`crate::Builder::build`]'s row height (§6.15). Without this call:
    /// `egui::FontDefinitions::default()`, whose emoji fallbacks it keeps (§8.1).
    /// `Context::set_fonts` overwrites the whole definition (`egui/src/context.rs:2105-2106`), so
    /// an application's own fonts — a `FontFamily::Name` family its code names, a script
    /// fallback — survive an install only in this base; an `add_font` still pending at the next
    /// pass is applied on top of them (`egui/src/context.rs:556-574`). A dropped `Name` family
    /// panics in release at the first text that names it (`epaint/src/text/fonts.rs:1025`). The base cannot be
    /// recovered from the `Context` instead: `Context::fonts` panics before the first pass
    /// (`egui/src/context.rs:1097-1106`), and a face re-added with `Context::add_font` after an
    /// install is skipped while the loaded definitions still hold its name (`:2133-2143`).
    pub fn with_base(self, base: FontDefinitions) -> Self {
        Self {
            base: Some(base),
            ..self
        }
    }
}

/// The face among `faces` that `select_face` picks for `spec`, by the rule on [`FontPlan`]:
/// every face described at width `5`, its own weight, or the asked weight for a variable face.
fn select<'a>(faces: &[&'a PlannedFace], spec: &ResolvedFontSpec) -> Option<&'a PlannedFace> {
    let names: Vec<[&str; 1]> = faces.iter().map(|f| [&*f.family]).collect();
    let traits: Vec<FaceTraits<'_>> = names
        .iter()
        .zip(faces)
        .map(|(families, face)| FaceTraits {
            families,
            width: 5,
            style: face.style,
            weight: face.weight.unwrap_or(spec.weight),
        })
        .collect();
    select_face(&traits, &spec.family, spec.weight, spec.style)
        .and_then(|i| faces.get(i))
        .copied()
}

/// Build a `FontDefinitions` that puts the theme's faces at the head of egui's `Proportional`
/// and `Monospace` chains. Pure: needs no `Context`, so it is fully testable and the
/// application may install the result itself. The starting point is the plan's base
/// ([`FontPlan::with_base`]; `FontDefinitions::default()`, with egui's emoji fallbacks,
/// without one, §8.1). A family the plan has no usable face for keeps the base's chain.
///
/// **How a chain is reached is part of the contract**, because `FontDefinitions`'s two fields
/// are both `pub` (`epaint/src/text/fonts.rs:435`, `:443`) and a plan may carry a base that
/// binds neither family. Both chains are reached with
/// `families.entry(family).or_default()` — **never** `get_mut` — before any face is looked
/// at, so the result binds both `FontFamily::Proportional` and `FontFamily::Monospace` even
/// when the base did not and no face of the plan is used, which is what keeps the
/// unbound-family panic (`epaint/src/text/fonts.rs:1025`) unreachable. And
/// every name prepended to a chain is inserted into `font_data` in the same call, which is
/// what keeps the missing-font-data panic (`:1033`) unreachable. Both epaint constructors
/// already bind both families — `default()` at `:534-550` and `empty()` at `:563-564`, the
/// latter to empty chains, which lay text out without a panic — so the `or_default` only
/// matters for a hand-built base.
///
/// Every face of the plan is validated before one is chosen (§8.2): a face whose bytes fail
/// epaint's parse, or whose `head` states a zero `unitsPerEm`, is never a candidate, and the
/// returned `Vec` carries a [`crate::Note::FontDataInvalid`] for its family, whether or not
/// the theme asks for it; the choice is made among the faces that remain, and a family none
/// of whose faces remains is left as the base had it. The `Vec` also carries the notes the plan
/// holds from `FontPlan::from_system`. Faces already in the base are the caller's and are
/// not checked.
#[must_use]
pub fn font_definitions(theme: &ResolvedTheme, plan: &FontPlan) -> (FontDefinitions, Vec<Note>) {
    let mut defs = plan.base.clone().unwrap_or_default();
    let mut notes = plan.notes.clone();
    // `entry(..).or_default()`, never `get_mut`, and before anything can return: the result
    // binds both families even when the base did not and no face of the plan survives (§4.9).
    for family in [FontFamily::Proportional, FontFamily::Monospace] {
        defs.families.entry(family).or_default();
    }
    if plan.faces.is_empty() {
        // An empty plan asks for nothing: the base alone, no note (§4.9, §8.2's first row).
        return (defs, notes);
    }
    // Every face is validated before one is chosen (§8.2), so a face that fails the parse
    // never hides a valid one of its family and is reported even when no spec asks for it.
    let all: Vec<&PlannedFace> = plan.faces.iter().collect();
    let mut valid = Vec::with_capacity(all.len());
    for &face in &all {
        if parses(face.bytes(), face.index) {
            valid.push(face);
        } else {
            push_note(
                &mut notes,
                Note::FontDataInvalid {
                    family: Arc::clone(&face.family),
                },
            );
        }
    }
    for (family, spec, key) in [
        (
            FontFamily::Proportional,
            &theme.defaults.font,
            PROPORTIONAL_KEY,
        ),
        (
            FontFamily::Monospace,
            &theme.defaults.mono_font,
            MONOSPACE_KEY,
        ),
    ] {
        if select(&all, spec).is_none() {
            // `push_note`: a `from_system` plan already carries this note for a family the OS
            // has no face of (§4.9); it is reported once.
            push_note(
                &mut notes,
                Note::FontFamilyUnavailable {
                    family: Arc::clone(&spec.family),
                },
            );
            continue;
        }
        // `None` here: every face of the family failed the parse, and each has its note.
        let Some(face) = select(&valid, spec) else {
            continue;
        };
        let mut data = match &face.bytes {
            FontBytes::Static(bytes) => FontData::from_static(bytes),
            FontBytes::Shared(bytes) => FontData::from_owned(bytes.to_vec()),
        };
        data.index = face.index;
        let own_weight_serves = face.weight == Some(spec.weight);
        if !own_weight_serves && !supports_weight_axis(&data) {
            push_note(
                &mut notes,
                Note::FontWeightAxisUnsupported {
                    family: Arc::clone(&face.family),
                },
            );
        }
        data.tweak.coords = weight_coords(spec.weight);
        defs.font_data.insert(key.to_owned(), Arc::new(data));
        // Prepend without an index: the rest of the chain follows.
        let chain = defs.families.entry(family).or_default();
        let rest = std::mem::take(chain);
        chain.push(key.to_owned());
        chain.extend(rest);
    }
    (defs, notes)
}

/// The parse epaint makes when it loads a face — `skrifa::FontRef::from_index`
/// (`epaint/src/text/font.rs:386-388`) — plus the zero-`unitsPerEm` check epaint does not
/// make: its `px_scale_factor` divides by `units_per_em` (`epaint/src/text/font.rs:215-218`; §8.2).
fn parses(bytes: &[u8], index: u32) -> bool {
    use skrifa::MetadataProvider as _;
    skrifa::FontRef::from_index(bytes, index).is_ok_and(|font| {
        font.metrics(
            skrifa::instance::Size::unscaled(),
            skrifa::instance::LocationRef::default(),
        )
        .units_per_em
            != 0
    })
}

/// Whether the face has a `wght` axis. `false` for a static font and for bytes that do not
/// parse alike: `FontData::variation_axes` (`epaint/src/text/fonts.rs:153-173`) answers an
/// empty list for both, and no public epaint API tells them apart (§8.3).
pub(crate) fn supports_weight_axis(data: &FontData) -> bool {
    data.variation_axes()
        .iter()
        .any(|axis| axis.tag == Tag::new(b"wght"))
}

/// The `wght` coordinate for a CSS weight (100–900), as the `VariationCoords` that egui takes
/// where it sets axes for a whole face or run: `FontTweak::coords`
/// (`epaint/src/text/fonts.rs:250`) of a face an application registers in a `FontDefinitions`
/// of its own rather than through a [`FontPlan`], and `TextFormat::coords`
/// (`epaint/src/text/text_layout_types.rs:505`) of a `LayoutJob` section. The counterpart of
/// iced's `to_iced_weight` (§4.7). Built with the infallible `Tag::new(b"wght")` (§8.3).
#[must_use]
pub fn weight_coords(css_weight: u16) -> VariationCoords {
    // `Tag::new` is the infallible `const fn` (§8.3); the `[u8; 4]` / `&str` `IntoTag` impls
    // `expect` and are never used.
    VariationCoords::new([(Tag::new(b"wght"), f32::from(css_weight))])
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

    /// T6 (a): `supports_weight_axis` is `false` for garbage bytes and for a static face,
    /// without panicking — `FontData::variation_axes` (`epaint/src/text/fonts.rs:153-173`)
    /// answers an empty list for both.
    #[test]
    fn supports_weight_axis_is_false_for_garbage_and_static_faces() {
        assert!(!supports_weight_axis(&egui::FontData::from_static(
            b"garbage"
        )));
        let defs = egui::FontDefinitions::default();
        assert!(!supports_weight_axis(defs.font_data.get("Hack").unwrap()));
    }

    #[cfg(feature = "system-fonts")]
    fn runner_themes() -> Vec<(String, native_theme::theme::ResolvedTheme)> {
        use native_theme::theme::{ColorMode, Theme};
        let mut themes = Vec::new();
        // The reader's path: the live theme, both modes.
        if let Ok(sys) = native_theme::SystemTheme::from_system() {
            themes.push(("system light".to_owned(), sys.light));
            themes.push(("system dark".to_owned(), sys.dark));
        }
        // The platform's bundled preset, both modes.
        let preset = if cfg!(target_os = "macos") {
            "macos-sonoma"
        } else {
            "windows-11"
        };
        for mode in [ColorMode::Light, ColorMode::Dark] {
            let t = Theme::preset(preset)
                .unwrap()
                .resolve(mode)
                .unwrap()
                .variant;
            themes.push((format!("{preset} {mode:?}"), t));
        }
        themes
    }

    /// The names the readers and the presets give resolve — by name through fontdb, and on
    /// macOS for the system UI font by file through Core Text (§8.2). `defaults.font` is
    /// asserted on both platforms and `defaults.mono_font` on Windows; macOS's "SF Mono" is
    /// printed, not asserted (§8.8, §15). On macOS it also prints the faces the system UI
    /// font's file holds and the family each records (§15's collection question).
    #[cfg(feature = "system-fonts")]
    #[test]
    #[ignore = "runs on the macOS and Windows screenshot runners (plan Task 39)"]
    fn system_faces_resolve() {
        use std::sync::Arc;
        for (label, t) in runner_themes() {
            let plan = FontPlan::from_system(&t);
            let (_, notes) = font_definitions(&t, &plan);
            let unavailable: Vec<Arc<str>> = notes
                .iter()
                .filter_map(|n| match n {
                    Note::FontFamilyUnavailable { family } => Some(Arc::clone(family)),
                    _ => None,
                })
                .collect();
            let sans = &t.defaults.font.family;
            let mono = &t.defaults.mono_font.family;
            assert!(
                !unavailable.iter().any(|f| f == sans),
                "{label}: {sans:?} has no system face; notes: {notes:?}"
            );
            let mono_resolves = !unavailable.iter().any(|f| f == mono);
            if cfg!(target_os = "windows") {
                assert!(
                    mono_resolves,
                    "{label}: {mono:?} has no system face; notes: {notes:?}"
                );
            } else {
                println!(
                    "system_faces_resolve: {label}: mono family {mono:?} resolves: {mono_resolves}"
                );
            }
            #[cfg(target_os = "macos")]
            if let Some(face) = native_theme::fonts::system_face(
                sans,
                t.defaults.font.weight,
                t.defaults.font.style,
            ) {
                // skrifa's re-export of read-fonts: `FileRef` tells a collection from a single
                // face (read-fonts 0.41.0 `src/lib.rs` lines 224–233), `CollectionRef::len` and
                // `get` (lines 271, 281) walk it, and the family is name id 1 (skrifa 0.44.0
                // `src/lib.rs` line 29, `src/provider.rs` line 30).
                use skrifa::MetadataProvider as _;
                use skrifa::raw::FileRef;
                use skrifa::string::StringId;
                let family_of = |f: &skrifa::FontRef<'_>| {
                    f.localized_strings(StringId::FAMILY_NAME)
                        .english_or_first()
                        .map(|s| s.to_string())
                        .unwrap_or_default()
                };
                match FileRef::new(&face.data) {
                    Ok(FileRef::Collection(c)) => {
                        println!(
                            "system_faces_resolve: {label}: {sans:?} is a collection of {} faces (chosen index {})",
                            c.len(),
                            face.index
                        );
                        for i in 0..c.len() {
                            if let Ok(f) = c.get(i) {
                                println!("  face {i}: family {:?}", family_of(&f));
                            }
                        }
                    }
                    Ok(FileRef::Font(f)) => {
                        println!(
                            "system_faces_resolve: {label}: {sans:?} is a single face, family {:?}",
                            family_of(&f)
                        );
                    }
                    Err(e) => println!(
                        "system_faces_resolve: {label}: {sans:?}: the file did not parse: {e}"
                    ),
                }
            }
        }
    }

    /// §6.15's `want − row` with the platform's own Body face, printed, and the atlas's built
    /// `extra_text_line_spacing` asserted equal to T5's value for that face.
    #[cfg(feature = "system-fonts")]
    #[test]
    #[ignore = "runs on the macOS and Windows screenshot runners (plan Task 39)"]
    fn system_line_spacing() {
        use egui::{FontId, TextStyle};
        for (label, t) in runner_themes() {
            let plan = FontPlan::from_system(&t);
            let (defs, _) = font_definitions(&t, &plan);
            let atlas = crate::ThemeAtlas::builder(&label, &t, &t)
                .fonts(plan)
                .build();
            let ctx = egui::Context::default();
            ctx.set_fonts(defs);
            let _ = pass(&ctx, egui::RawInput::default(), |_| {});
            let base = &atlas.scheme(egui::Theme::Light).base;
            let size = base.text_styles.get(&TextStyle::Body).unwrap().size;
            let row = ctx.fonts_mut(|f| f.row_height(&FontId::proportional(size)));
            let want = t.defaults.line_height * size;
            println!(
                "system_line_spacing: {label}: size {size}, row {row}, want - row = {}",
                want - row
            );
            assert_eq!(
                base.spacing.extra_text_line_spacing,
                crate::convert::clamp_length(want - row),
                "{label}"
            );
        }
    }
}
