//! A preset's declarations (S10): its keyed names, its flavours and what
//! each widget has on the preset, which adding a layer checks every name
//! against (S12).

use native_theme_derive::ThemeParts;

use super::expr::ElementDatum;
use super::inputs::InputName;
use super::key::Key;
use super::roles::SurfaceSet;
use super::selector::{Breakpoint, ControlSize, Density, Scheme, StateFlag, VariantRef, WidgetRef};
use super::widgets::{TypedPart, WidgetKind};

/// A preset's `[declare]` table (S10); each list empty where the preset
/// declares none.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct Declarations {
    /// The flavours, the first the default.
    pub(crate) flavors: Vec<Key>,
    /// The materials.
    pub(crate) materials: Vec<Key>,
    /// The keyed colour roles.
    pub(crate) color_roles: Vec<Key>,
    /// The keyed text styles.
    pub(crate) text_styles: Vec<Key>,
    /// The elevation ramp's entries.
    pub(crate) elevation: Vec<Key>,
    /// The surface sets.
    pub(crate) surface_sets: Vec<SurfaceSet>,
    /// The system settings rules may select on.
    pub(crate) conditions: Vec<SettingDecl>,
    /// The keyed scalar groups, each of one unit.
    pub(crate) params: Vec<(Key, ParamGroup)>,
    /// The inputs.
    pub(crate) inputs: Vec<InputName>,
    /// The inputs that follow the accent, a subset of `inputs` (S9.1).
    pub(crate) accent_inputs: Vec<InputName>,
    /// The keyed blend modes.
    pub(crate) blend_modes: Vec<Key>,
    /// The motion timing tokens' keys.
    pub(crate) timings: Vec<Key>,
    /// The keyed transition kinds, with their parameters' units.
    pub(crate) transition_kinds: Vec<(Key, Vec<(Key, ScalarUnit)>)>,
    /// The keyed presentation kinds, with their parameters' units.
    pub(crate) presentation_kinds: Vec<(Key, Vec<(Key, ScalarUnit)>)>,
    /// The keyed presentation events.
    pub(crate) presentation_events: Vec<Key>,
    /// Each container's sizes, the first the default.
    pub(crate) container_sizes: Vec<(WidgetRef, Vec<Key>)>,
    /// The symbol variants.
    pub(crate) symbol_variants: Vec<Key>,
    /// The symbol scales (`IconStyle.scale`).
    pub(crate) symbol_scales: Vec<Key>,
    /// The Material spec versions (`MaterialScheme.spec_version`).
    pub(crate) spec_versions: Vec<Key>,
    /// The icon state effects.
    pub(crate) icon_state_effects: Vec<Key>,
    /// The keyed focus kinds.
    pub(crate) focus_kinds: Vec<Key>,
    /// The device models.
    pub(crate) device_models: Vec<Key>,
    /// Material parameters beyond the named ones, with their units.
    pub(crate) material_params: Vec<(Key, ScalarUnit)>,
    /// The motion durations' keys.
    pub(crate) durations: Vec<Key>,
    /// The icon contexts.
    pub(crate) icon_contexts: Vec<Key>,
    /// The icon status colours' keys.
    pub(crate) status_colors: Vec<Key>,
    /// The drag cursors' keys.
    pub(crate) drag_cursors: Vec<Key>,
    /// The accent ramp's entries.
    pub(crate) accent_ramp: Vec<Key>,
    /// The text rendering engines.
    pub(crate) rendering_engines: Vec<Key>,
    /// The LCD filters.
    pub(crate) lcd_filters: Vec<Key>,
    /// A palette's schemes (S14).
    pub(crate) schemes: Vec<Scheme>,
    /// What each typed widget has on the preset.
    pub(crate) widgets: Vec<(WidgetKind, WidgetDecl)>,
    /// A flavour's declarations where they differ, each key replacing the
    /// widget's own under that flavour.
    pub(crate) flavor: Vec<(Key, FlavorDecl)>,
}

/// A system setting rules may select on (`system_setting:<key>`).
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SettingDecl {
    /// The setting.
    pub(crate) key: Key,
    /// Only applications read it, no rule selects on it (`Theme::system`).
    pub(crate) for_applications: bool,
}

/// A keyed scalar group (S4.6): its unit and its keys.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ParamGroup {
    /// The group's one unit.
    pub(crate) unit: ScalarUnit,
    /// The group's keys.
    pub(crate) keys: Vec<Key>,
}

/// A keyed scalar's unit, as a declaration names it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ThemeParts)]
#[parts(toml)]
pub(crate) enum ScalarUnit {
    /// `px`.
    Px,
    /// `ms`.
    Ms,
    /// `deg`.
    Deg,
    /// `fraction`.
    Fraction,
    /// `number`.
    Number,
}

/// A flavour's declarations where they differ (S10).
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct FlavorDecl {
    /// The widgets' keys that differ under the flavour.
    pub(crate) widgets: Vec<(WidgetKind, WidgetDecl)>,
}

/// What a typed widget has on a preset (S10): each key `None` where the
/// section does not state it, so a flavour's section replaces only the keys
/// it states.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct WidgetDecl {
    /// The variants: typed, or keyed.
    pub(crate) variants: Option<Vec<VariantRef>>,
    /// The control sizes.
    pub(crate) sizes: Option<Vec<ControlSize>>,
    /// The densities.
    pub(crate) densities: Option<Vec<Density>>,
    /// The flags.
    pub(crate) flags: Option<Vec<StateFlag>>,
    /// The exclusive flag groups, each highest precedence first.
    pub(crate) exclusive: Option<Vec<Vec<StateFlag>>>,
    /// Groups of variants an element holds at most one of.
    pub(crate) exclusive_variants: Option<Vec<Vec<VariantRef>>>,
    /// The sizes in the nearest ancestor of a widget.
    pub(crate) sizes_by_context: Option<Vec<SizesByContext>>,
    /// The sizes of an element holding a variant.
    pub(crate) sizes_by_variant: Option<Vec<SizesByVariant>>,
    /// The size of an element that states none.
    pub(crate) default_size: Option<ControlSize>,
    /// The default size of an element holding a variant, first match first.
    pub(crate) default_size_by_variant: Option<Vec<DefaultSizeByVariant>>,
    /// Whether the platform has such a control.
    pub(crate) available: Option<bool>,
    /// The variants an element must hold.
    pub(crate) requires_variants: Option<Vec<VariantRef>>,
    /// The element data each part's values read, and the values'.
    pub(crate) data: Option<Vec<(DataOwner, Vec<DataName>)>>,
    /// The keys each keyed value may hold, by the value's name.
    pub(crate) keys: Option<Vec<(&'static str, Vec<Key>)>>,
    /// The breakpoints.
    pub(crate) breakpoints: Option<Vec<Breakpoint>>,
}

/// The sizes an element takes in the nearest ancestor of a widget.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SizesByContext {
    /// The ancestor's widget.
    pub(crate) context: WidgetRef,
    /// The sizes.
    pub(crate) sizes: Vec<ControlSize>,
}

/// The sizes an element holding a variant takes.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SizesByVariant {
    /// The variant.
    pub(crate) variant: VariantRef,
    /// The sizes.
    pub(crate) sizes: Vec<ControlSize>,
}

/// The default size of an element holding a variant.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct DefaultSizeByVariant {
    /// The variant.
    pub(crate) variant: VariantRef,
    /// The size.
    pub(crate) size: ControlSize,
}

/// What reads element data: a part's values, or the widget's values.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum DataOwner {
    /// A part.
    Part(TypedPart),
    /// The widget's values.
    Values,
}

/// An operand a part's values read from `Element.data` or
/// `Element.metrics` (S8, S10).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum DataName {
    /// An `element.` operand.
    Element(ElementDatum),
    /// The own font's line spacing, which `line_height` reads under `auto`.
    LineSpacing,
    /// The `body` slot's font's height.
    BodyFontHeight,
    /// Another part's font metric, `widget.<part>.<metric>`.
    PartMetric {
        /// The part.
        part: TypedPart,
        /// The metric.
        metric: FontMetric,
    },
}

/// A font metric another part's declaration names (S10).
#[derive(Clone, Copy, Debug, PartialEq, Eq, ThemeParts)]
#[parts(toml)]
pub(crate) enum FontMetric {
    /// The average character width.
    CharWidth,
    /// The widest glyph.
    MaxCharWidth,
    /// The x-height.
    FontXHeight,
    /// Ascent plus descent.
    FontHeight,
    /// The underline's offset.
    FontUnderlineOffset,
    /// The underline's thickness.
    FontUnderlineThickness,
    /// The strikeout's offset.
    FontStrikeoutOffset,
    /// The strikeout's thickness.
    FontStrikeoutThickness,
    /// The font size.
    FontSize,
    /// The line height.
    LineHeight,
}
