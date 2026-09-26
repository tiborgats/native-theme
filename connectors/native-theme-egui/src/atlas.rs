//! The atlas handle, its builder and its diagnostics (spec §4.2, §4.3). Task 10: `Note`;
//! Task 11: `ThemeAtlas`, `AtlasInner`, `SchemeStyles`, `Builder`.

/// A non-fatal observation made while compiling a [`ThemeAtlas`]. Six variants.
///
/// `#[non_exhaustive]` on the *enum* keeps adding a variant non-breaking; it does **not**
/// protect a variant's payload, which stays exhaustively patternable downstream (verified with
/// a two-crate probe on rustc 1.97.1). Since the diagnostics channel is the surface most
/// likely to be enriched, each struct variant carries its own `#[non_exhaustive]`, so a
/// downstream `match` must spell `{ path, .. }` and adding a field stays additive.
/// [`Surface::Panel`](crate::Surface::Panel) and both [`fonts::FontBytes`](crate::fonts::FontBytes) tuple variants are deliberately left
/// exhaustive: `#[non_exhaustive]` on a *tuple* variant makes it unconstructible outside this
/// crate (`E0603`, verified), and constructing them is the documented call shape.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Note {
    /// A theme value was non-finite, or a text size not a positive normal `f32`, and was
    /// replaced by the documented fallback.
    /// `path` is the native-theme field path, e.g. `"button.border.corner_radius"`.
    #[non_exhaustive]
    ValueSanitised {
        /// The native-theme leaf path whose value was replaced (§7.2: the leaf, never the egui sink).
        path: &'static str,
    },
    /// A length saturated at the `i8` bound of an epaint type — in practice
    /// `epaint::Margin`, the only `i8` sink this crate writes from theme data (§6.14 leaves
    /// all shadow geometry to egui). **Real data loss.** `u8` saturation — corner radii — is
    /// benign and is deliberately **not** reported; see §7.2.
    #[non_exhaustive]
    ValueSaturated {
        /// The native-theme leaf path whose length saturated.
        path: &'static str,
    },
    /// The theme asked for a font family for which the plan holds no bytes — or, from
    /// `fonts::FontPlan::from_system`, which the OS has no face of under that name (compared
    /// case-insensitively, §8.2).
    /// egui's own face stays in that family's place.
    #[non_exhaustive]
    FontFamilyUnavailable {
        /// The family the theme asked for.
        family: std::sync::Arc<str>,
    },
    /// The face chosen for a family has no `wght` axis — `FontData::variation_axes()`
    /// (`epaint/src/text/fonts.rs:153-173`) reports none — and either its own weight differs
    /// from the one the theme asked for, or it was registered with
    /// [`fonts::FontPlan::variable_face`](crate::fonts::FontPlan::variable_face). The face renders at its own weight (§8.2).
    #[non_exhaustive]
    FontWeightAxisUnsupported {
        /// The family the face was chosen for.
        family: std::sync::Arc<str>,
    },
    /// A native colour with alpha `0` was written, as given, to a `WidgetVisuals::bg_fill`,
    /// which egui documents as "Must never be `Color32::TRANSPARENT`"
    /// (`egui/src/style.rs:1292-1294`). Nothing is substituted — a substitute would be a
    /// colour no platform stated — so the widget paints no background in that state (§6.4).
    #[non_exhaustive]
    TransparentFill {
        /// The native-theme leaf path of the colour with alpha `0`.
        path: &'static str,
    },
    /// A registered face's bytes failed the parse epaint makes when it loads a face —
    /// `skrifa::FontRef::from_index(data, index)`, the only fallible step of epaint's
    /// `FontFace::new` (`epaint/src/text/font.rs:386-388`), whose failure epaint turns into a
    /// panic in release builds too (`epaint/src/text/fonts.rs:990`) — or they parsed with a zero
    /// `unitsPerEm`, which epaint divides by (§8.2). The face was dropped and
    /// the family falls back to the next face in its chain, egui's own if no other. `family`
    /// is the name the face was registered under.
    #[non_exhaustive]
    FontDataInvalid {
        /// The family name the face was registered under.
        family: std::sync::Arc<str>,
    },
}
