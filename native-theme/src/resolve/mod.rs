//! S8's stack and its resolution for a window: the layers in their order
//! (the preset, the accent-derivation layer on a preset that derives its
//! accent-dependent roles, the reader or its stand-in, the application's
//! layers, the accent layer), the conditions in force, and the resolution's three
//! steps: the bases merged in order, then the `Colors` and `Tokens` rules
//! whose conditions hold, then each part's and widget's base and its rules
//! whose conditions hold, kept in order for the query (S8); their
//! evaluation for an element ([`evaluate`]), a `Fields` patch over a
//! reference composed lazily ([`compose`], S2.2). Also S9.2's accent layer
//! (its inputs) and `kde-breeze`'s accent-derivation layer (Plasma's
//! derivation of the accent-dependent roles and inactive input roles
//! through `kde_accent` and `kde_accent_wm`, at the schemes where a layer
//! above the preset gives the accent in force), with S8's leave-out beneath
//! it of the reader's or stand-in's stated roles ([`LeftOut`]).
#![cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "the system's reader is set by the readers (M1b); `Cx::unscaled` is called by the derive for a widget value of a font size, which no enabled unit has yet"
    )
)]

use std::sync::Arc;

pub(crate) mod compose;
pub(crate) mod evaluate;

use super::blocks::BlockPatch;
use super::color::Color;
use super::declare::Declarations;
use super::error::{LayerError, Problem, ProblemKind};
use super::expr::{Expr, Func, Literal, Name};
use super::inputs::{Accent, Contrast, InputBases, InputName, InputValue, KdeSchemeKey, KdeWmKey};
use super::key::Key;
use super::layer::{Bases, Layer, LayerId};
use super::palette::{Palette, Port};
use super::presets::PresetId;
use super::roles::{ColorRolesPatch, SurfaceRoles, SurfaceRolesPatch, SurfaceSet};
use super::selector::{
    Condition, Idiom, InputKind, PartRef, Posture, RuleBody, Scheme, Selector, TiledEdges,
    ValuesPatch, WidgetRef,
};
use super::tokens::TokensPatch;
use super::validate::{self, Context, LayerKind, Stacked};
use super::value::{SchemeMap, SurfaceMap, Value};
use super::widgets::{PartPatch, Shown, TypedPart, WidgetKind, WidgetValuesPatch, WidgetsPatch};

/// The system conditions a reader reports (S8, R2.3, S7.1), and its text
/// scaling factor and font DPI (S9.4).
#[derive(Clone, Debug, PartialEq)]
pub struct ReaderState {
    /// The system's scheme.
    pub scheme: Scheme,
    /// The contrast condition that holds, if one does.
    pub contrast: Option<Contrast>,
    /// Reduced transparency.
    pub reduced_transparency: bool,
    /// Reduced motion.
    pub reduced_motion: bool,
    /// Animations off.
    pub animations_off: bool,
    /// Bold text.
    pub bold_text: bool,
    /// Button shapes.
    pub button_shapes: bool,
    /// Differentiate without colour.
    pub differentiate_without_color: bool,
    /// Tablet mode.
    pub tablet_mode: bool,
    /// A mobile form factor (Plasma Mobile's shell).
    pub mobile_form_factor: bool,
    /// Battery saver.
    pub battery_saver: bool,
    /// Overlay scroll bars.
    pub overlay_scrollbars: bool,
    /// On/off labels.
    pub on_off_labels: bool,
    /// Full keyboard access.
    pub full_keyboard_access: bool,
    /// Prefer cross-fade transitions.
    pub prefer_cross_fade: bool,
    /// The device's idiom, where the platform has one.
    pub idiom: Option<Idiom>,
    /// The device model, where the preset declares models.
    pub device_model: Option<Key>,
    /// The size each container reports; a container not listed takes its
    /// declared default (S10).
    pub container_sizes: Vec<(WidgetRef, Key)>,
    /// The system settings that hold.
    pub system_settings: Vec<Key>,
    /// The factor text scales by (S9.4).
    pub text_scaling_factor: f32,
    /// The system's font DPI where the platform has a setting; `None`: the
    /// preset's `token.text.font_dpi` (S9.4).
    pub font_dpi: Option<f32>,
    /// iOS's content size category, for `DynamicType` (S9.4); `None`: the
    /// default category.
    pub content_size_category: Option<ContentSizeCategory>,
}

impl ReaderState {
    /// A reader's state under `scheme`, text scaled by
    /// `text_scaling_factor`, with no other condition holding and no DPI or
    /// category of its own.
    #[must_use]
    pub fn new(scheme: Scheme, text_scaling_factor: f32) -> ReaderState {
        ReaderState {
            scheme,
            contrast: None,
            reduced_transparency: false,
            reduced_motion: false,
            animations_off: false,
            bold_text: false,
            button_shapes: false,
            differentiate_without_color: false,
            tablet_mode: false,
            mobile_form_factor: false,
            battery_saver: false,
            overlay_scrollbars: false,
            on_off_labels: false,
            full_keyboard_access: false,
            prefer_cross_fade: false,
            idiom: None,
            device_model: None,
            container_sizes: Vec::new(),
            system_settings: Vec::new(),
            text_scaling_factor,
            font_dpi: None,
            content_size_category: None,
        }
    }

    /// Whether the two states' values differ: the text scaling factor, the
    /// font DPI, the content size category.
    fn values_differ(&self, other: &ReaderState) -> bool {
        self.text_scaling_factor != other.text_scaling_factor
            || self.font_dpi != other.font_dpi
            || self.content_size_category != other.content_size_category
    }

    /// Whether the two states' other conditions differ: all but the scheme
    /// and the contrast, which the theme change compares in force.
    fn conditions_differ(&self, other: &ReaderState) -> bool {
        let mut this = self.clone();
        this.scheme = other.scheme;
        this.contrast = other.contrast;
        this.text_scaling_factor = other.text_scaling_factor;
        this.font_dpi = other.font_dpi;
        this.content_size_category = other.content_size_category;
        this != *other
    }
}

/// UIKit's `UIContentSizeCategory`, smallest first (S8).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ContentSizeCategory {
    /// Extra small.
    ExtraSmall,
    /// Small.
    Small,
    /// Medium.
    Medium,
    /// Large.
    Large,
    /// Extra large.
    ExtraLarge,
    /// Extra extra large.
    ExtraExtraLarge,
    /// Extra extra extra large.
    ExtraExtraExtraLarge,
    /// Accessibility medium.
    AccessibilityMedium,
    /// Accessibility large.
    AccessibilityLarge,
    /// Accessibility extra large.
    AccessibilityExtraLarge,
    /// Accessibility extra extra large.
    AccessibilityExtraExtraLarge,
    /// Accessibility extra extra extra large.
    AccessibilityExtraExtraExtraLarge,
}

impl ContentSizeCategory {
    /// Whether `AccessibilityTextSize` holds: one of the five accessibility
    /// categories (S8).
    #[must_use]
    pub fn is_accessibility(self) -> bool {
        matches!(
            self,
            ContentSizeCategory::AccessibilityMedium
                | ContentSizeCategory::AccessibilityLarge
                | ContentSizeCategory::AccessibilityExtraLarge
                | ContentSizeCategory::AccessibilityExtraExtraLarge
                | ContentSizeCategory::AccessibilityExtraExtraExtraLarge
        )
    }
}

/// A window's conditions, which its connector states (S8).
#[derive(Clone, Debug, PartialEq)]
pub struct WindowConditions {
    /// The window is inactive.
    pub inactive: bool,
    /// The window is not the main one.
    pub not_main: bool,
    /// The application is inactive.
    pub app_inactive: bool,
    /// The window is maximized.
    pub maximized: bool,
    /// The window is full screen.
    pub full_screen: bool,
    /// The edges the window is tiled to.
    pub tiled: TiledEdges,
    /// The window is elevated.
    pub elevated: bool,
    /// The kind of the last input.
    pub input: InputKind,
    /// The window is not composited.
    pub not_composited: bool,
    /// The window is shaded.
    pub shaded: bool,
    /// The window flashes for attention.
    pub attention_flash: bool,
    /// The window's tint is dimmed.
    pub tint_dimmed: bool,
    /// Mnemonics are visible.
    pub mnemonics_visible: bool,
    /// Android's desktop windowing.
    pub freeform: bool,
    /// A foldable's posture.
    pub posture: Option<Posture>,
    /// The device pixel ratio is 1.
    pub unit_scale: bool,
}

impl WindowConditions {
    /// A window whose last input was `input`, with no other condition
    /// holding.
    #[must_use]
    pub fn new(input: InputKind) -> WindowConditions {
        WindowConditions {
            inactive: false,
            not_main: false,
            app_inactive: false,
            maximized: false,
            full_screen: false,
            tiled: TiledEdges::default(),
            elevated: false,
            input,
            not_composited: false,
            shaded: false,
            attention_flash: false,
            tint_dimmed: false,
            mnemonics_visible: false,
            freeform: false,
            posture: None,
            unit_scale: false,
        }
    }
}

/// What a window's resolution changed since its last (S8, S4.3).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Change {
    /// The window's first resolution.
    First,
    /// Nothing changed.
    Unchanged,
    /// A layer's values, an input, a reader value, the preset, the scheme
    /// or the contrast changed: a theme change, which wins where conditions
    /// changed too (S4.3).
    ThemeChange,
    /// Another condition changed: a window's, the reader's, the flavour.
    ConditionChange,
}

/// A window's record of its last resolution (S8); `Default`: never
/// resolved.
#[derive(Clone, Debug, Default)]
pub struct WindowState {
    last: Option<Recorded>,
}

/// What a window's last resolution read.
#[derive(Clone, Debug)]
struct Recorded {
    /// The layers, in the stack's order, shared with the stack.
    layers: Vec<Arc<Layer>>,
    preset: Option<PresetId>,
    reader: Option<ReaderState>,
    /// Whether the reader's common accent was left out (S8, S9.5), which
    /// its own preset and whether it is a stand-in decide.
    accent_left_out: bool,
    scheme: Scheme,
    contrast: Option<Contrast>,
    window: WindowConditions,
    flavor: Option<Key>,
}

/// A window's resolution and what it changed (S8).
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct Resolution {
    /// The theme resolved for the window.
    pub theme: ResolvedTheme,
    /// What changed since the window's last resolution.
    pub change: Change,
}

/// Where a step lies in the stack's order (S8): every layer's bases in
/// layer order, then every layer's rules in layer order, each layer's in
/// stated order.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Pos {
    /// `false`: a base; `true`: a rule.
    pub(crate) rule: bool,
    /// The layer, by its place in the stack.
    pub(crate) layer: usize,
    /// A rule's index in its layer; a layer's common bases are 0, its
    /// `for_preset` part's 1.
    pub(crate) index: usize,
}

/// One patch of a target, where it lies in the stack, and, for a part's or
/// widget's rule, its selector, which the query matches against the
/// element (S8 step 3).
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Step<P> {
    /// Its place in the stack's order.
    pub(crate) pos: Pos,
    /// A part's or widget's rule's selector; `None` for a base, and for a
    /// `Colors` or `Tokens` rule, whose conditions hold (step 2).
    pub(crate) selector: Option<Selector>,
    /// The patch.
    pub(crate) patch: P,
}

/// The theme resolved for one window (S8): the steps each target keeps, in
/// order, which the query evaluates (S2.2).
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct ResolvedTheme {
    /// The scheme in force.
    pub(crate) scheme: Scheme,
    /// The colour roles' bases of the scheme in force, then the `Colors`
    /// rules whose conditions hold.
    pub(crate) colors: Vec<Step<ColorRolesPatch>>,
    /// Each input's value at the slot in force, the scheme and contrast in
    /// force: that of the last applying layer that gives it a value there,
    /// its per-contrast base where a contrast condition holds and it states
    /// one for that scheme, else its per-scheme value (S9, ADD-A132).
    pub(crate) inputs: Vec<(InputName, Value<Color>)>,
    /// The layer each input in force was taken from, its place in the
    /// stack: whether a listed input's value in force is the preset's own
    /// base, where the synthetic route folds it (S17, ADD-A132).
    pub(crate) input_layers: Vec<(InputName, usize)>,
    /// KDE's three lists under the scheme in force, the last layer's for it
    /// (S9.1).
    pub(crate) kde_lists: KdeLists,
    /// The tokens' bases, then the `Tokens` rules whose conditions hold.
    pub(crate) tokens: Vec<Step<Box<TokensPatch>>>,
    /// Each typed part's bases, then its rules whose conditions hold.
    pub(crate) parts: Vec<(TypedPart, Vec<Step<PartPatch>>)>,
    /// Each typed widget's values' bases, then its rules whose conditions
    /// hold.
    pub(crate) values: Vec<(WidgetKind, Vec<Step<WidgetValuesPatch>>)>,
    /// The reader's or stand-in's text scaling factor; 1 without either,
    /// as S16's no-reader state has it (S9.4).
    pub(crate) text_scaling_factor: f32,
    /// The reader's font DPI, where it reports one (S9.4).
    pub(crate) font_dpi: Option<f32>,
    /// The reader's content size category (S9.4).
    pub(crate) content_size_category: Option<ContentSizeCategory>,
    /// The preset's declarations, which expressions are typed against
    /// (S2.2, S10).
    pub(crate) declarations: Declarations,
    /// The flavour in force, whose declarations the query reads (S10);
    /// `None` where the preset declares none.
    pub(crate) flavor: Option<Key>,
    /// The places in the stack of the synthetic route's layers (S16's
    /// `Theme::from_preset_synthetic`, S17, ADD-A128), whose steps
    /// evaluation records read; none outside it.
    pub(crate) synthetic: Vec<usize>,
    /// The listed inputs whose value in force is the synthetic one the
    /// synthetic route folded into the preset's input base (S17,
    /// ADD-A132), which evaluation records read; none outside it.
    pub(crate) synthetic_inputs: Vec<InputName>,
}

/// KDE's scheme lists under the scheme in force (S9.1).
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct KdeLists {
    /// `input.kde_scheme.groups`.
    pub(crate) groups: Vec<SurfaceSet>,
    /// `input.kde_scheme.inactive_groups`.
    pub(crate) inactive_groups: Vec<SurfaceSet>,
    /// `input.kde_scheme.unset_keys`.
    pub(crate) unset_keys: Vec<InputName>,
}

/// No layer's lists, for an [`Env`](crate::expr::Env) of the tests that
/// reads none.
#[cfg(test)]
pub(crate) static NO_KDE_LISTS: KdeLists = KdeLists {
    groups: Vec::new(),
    inactive_groups: Vec::new(),
    unset_keys: Vec::new(),
};

/// The reader's layer, or the stand-in an application without a reader
/// sets (S8, R2.6).
#[derive(Clone, Debug)]
#[cfg_attr(test, derive(PartialEq))]
enum Reader {
    /// The system's reader: its state and layer, and its own preset (S9.5).
    Read {
        state: ReaderState,
        layer: Arc<Layer>,
        own: Option<PresetId>,
    },
    /// The application's stand-in.
    StandIn {
        state: ReaderState,
        layer: Arc<Layer>,
    },
}

/// S8's leave-out of one layer beneath the accent-derivation layer's
/// statements (S8, S9.2; ADD-A132): at the schemes `schemes`, the slots of
/// the scheme in force, the layer's stated accent-dependent roles (S9.1's
/// list, the `[WM]` keyed roles among them) and inactive input roles, in its
/// bases and rules, are not in force, so the accent-derivation layer's roles
/// over the accent in force show there. Resolution and validation read it
/// alike, through the layer's [`Stacked`]. The reader's or stand-in's holds
/// where a layer above it gives the accent in force; a KDE port's (S14) is
/// the same leave-out at every scheme where the accent-derivation layer
/// states its roles ([`Stack::accent_givers`]).
///
/// The reader's or stand-in's holds S8's palette leave-out too: at the
/// schemes where a palette above it applies (`palette`), its colour bases,
/// colour rules, colour inputs but those `kept` (a KDE port's: the accent
/// inputs, R2.6; ADD-A142), KDE's three lists and `token.kde` are not in
/// force. Validation reads the colours and inputs of it; its `token.kde`
/// it keeps, conservatively, as it reads the tokens under every scheme at
/// once, and a port's `Tokens` rule states every field of `token.kde`
/// over it at the port's scheme.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct LeftOut {
    /// The schemes at which the layer's accent-dependent roles and inactive
    /// input roles are left out.
    schemes: Vec<Scheme>,
    /// The accent-dependent roles left out.
    roles: Vec<validate::AccentRole>,
    /// Their paths in a scheme's colour roles (`surfaces.view.link`).
    paths: Vec<String>,
    /// The inactive input roles left out.
    inputs: Vec<InputName>,
    /// The schemes at which a palette above the layer applies (S8, S14).
    palette: Vec<Scheme>,
    /// The inputs kept beneath the palette.
    kept: Vec<InputName>,
}

impl LeftOut {
    /// The leave-out of `roles` and the inactive input roles `inputs` at
    /// `schemes`.
    pub(crate) fn new(
        schemes: Vec<Scheme>,
        roles: &[validate::AccentRole],
        inputs: Vec<InputName>,
    ) -> LeftOut {
        LeftOut {
            schemes,
            roles: roles.to_vec(),
            paths: roles.iter().map(|role| role.path()).collect(),
            inputs,
            palette: Vec::new(),
            kept: Vec::new(),
        }
    }

    /// The leave-out, and S8's beneath a palette applying at `schemes`,
    /// the inputs `kept` in force there.
    pub(crate) fn beneath_palette(self, schemes: Vec<Scheme>, kept: Vec<InputName>) -> LeftOut {
        LeftOut {
            palette: schemes,
            kept,
            ..self
        }
    }

    /// Whether anything of the layer is left out at `scheme`.
    pub(crate) fn at(&self, scheme: Scheme) -> bool {
        self.schemes.contains(&scheme) || self.palette.contains(&scheme)
    }

    /// Whether a palette above the layer applies at `scheme`, leaving its
    /// colours, colour inputs but the kept ones, lists and `token.kde` out.
    pub(crate) fn under_palette(&self, scheme: Scheme) -> bool {
        self.palette.contains(&scheme)
    }

    /// Whether the role at `path` of a scheme's colour roles is left out
    /// at `scheme`: every role beneath a palette.
    pub(crate) fn role(&self, scheme: Scheme, path: &str) -> bool {
        self.under_palette(scheme)
            || (self.schemes.contains(&scheme) && self.paths.iter().any(|held| held == path))
    }

    /// The schemes at which the layer's accent-dependent roles are left
    /// out and no palette above it leaves its colours out whole.
    pub(crate) fn schemes(&self) -> Vec<Scheme> {
        self.schemes
            .iter()
            .copied()
            .filter(|scheme| !self.under_palette(*scheme))
            .collect()
    }

    /// Whether an accent-dependent role of the surface set `set` is left
    /// out.
    pub(crate) fn holds_set(&self, set: SurfaceSet) -> bool {
        self.roles
            .iter()
            .any(|role| matches!(role, validate::AccentRole::Surface(held, _) if *held == set))
    }

    /// Whether the input `name` is left out at `scheme`: an inactive input
    /// role beneath the accent-derivation layer, any input but the kept
    /// ones beneath a palette.
    pub(crate) fn input(&self, scheme: Scheme, name: &InputName) -> bool {
        (self.under_palette(scheme) && !self.kept.contains(name))
            || (self.schemes.contains(&scheme) && self.inputs.contains(name))
    }

    /// `roles` without the roles left out: each stated as a field of its
    /// set's patch or as a keyed entry taken out. A set stated whole is
    /// left as it is: validation refuses one where its roles are left out
    /// ([`validate::sets_stated_whole`]), and no reader or stand-in states
    /// one, each stating its roles field by field.
    fn strip(&self, roles: &mut ColorRolesPatch) {
        for role in &self.roles {
            match role {
                validate::AccentRole::Surface(set, key) => {
                    if let Some(BlockPatch::Fields(patch)) = roles
                        .surfaces
                        .as_mut()
                        .and_then(|surfaces| surfaces.get_mut(*set))
                    {
                        *key_slot(patch, *key) = None;
                    }
                }
                validate::AccentRole::Wm(key) => {
                    if let Some(keyed) = roles.keyed.as_mut() {
                        keyed.retain(|(held, _)| !is_wm(held, *key));
                    }
                }
            }
        }
    }
}

/// The field of a surface set's roles patch that the scheme key `key` names
/// (the catalogue's key table, [`KdeSchemeKey::role`]).
fn key_slot(patch: &mut SurfaceRolesPatch, key: KdeSchemeKey) -> &mut Option<Value<Color>> {
    match key {
        KdeSchemeKey::BackgroundNormal => &mut patch.background,
        KdeSchemeKey::BackgroundAlternate => &mut patch.background_alternate,
        KdeSchemeKey::ForegroundNormal => &mut patch.foreground,
        KdeSchemeKey::ForegroundInactive => &mut patch.foreground_inactive,
        KdeSchemeKey::ForegroundActive => &mut patch.foreground_active,
        KdeSchemeKey::ForegroundLink => &mut patch.link,
        KdeSchemeKey::ForegroundVisited => &mut patch.link_visited,
        KdeSchemeKey::ForegroundNegative => &mut patch.negative,
        KdeSchemeKey::ForegroundNeutral => &mut patch.neutral,
        KdeSchemeKey::ForegroundPositive => &mut patch.positive,
        KdeSchemeKey::DecorationFocus => &mut patch.decoration_focus,
        KdeSchemeKey::DecorationHover => &mut patch.decoration_hover,
    }
}

/// Whether `held` is the key of the keyed role `wm_<key>`.
fn is_wm(held: &Key, key: KdeWmKey) -> bool {
    held.as_str()
        .strip_prefix("wm_")
        .is_some_and(|rest| rest == key.name())
}

/// The stack (S8): the preset's declarations and layer, the reader or its
/// stand-in, the application's layers, the accent the application set, and
/// the application's conditions. Every change validates the whole stack
/// and is refused whole (S8, S12). The layers are shared: a copy of the
/// stack, a change's or a window's record, copies no layer.
#[derive(Clone, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub(crate) struct Stack {
    preset: Option<PresetId>,
    declarations: Declarations,
    preset_layer: Arc<Layer>,
    /// The accent-derivation layer, directly above the preset on a preset
    /// that derives its accent-dependent roles (S8, S9.2; ADD-A132),
    /// derived on every change of the stack and slot; `None` on every
    /// other preset.
    derivation: Option<Arc<Layer>>,
    reader: Option<Reader>,
    /// S8's leave-out of the reader's or stand-in's roles beneath the
    /// accent-derivation layer, and of its colours beneath a palette,
    /// derived with it; `None` where nothing of it is left out.
    reader_left_out: Option<LeftOut>,
    /// The application's layers, each with the id it was added under, the
    /// palette's entry among them.
    layers: Vec<(LayerId, Arc<Layer>)>,
    /// The palette `apply_palette` added (S8, S14), with the id of its
    /// entry among `layers`, where it was first added; `None` without one.
    /// The entry holds the palette's source for the current preset where
    /// it applies under the contrast in force ([`Stack::derive_palette`]).
    palette: Option<(LayerId, Arc<Palette>)>,
    /// S8's leave-out of the palette's KDE port's roles beneath the
    /// accent-derivation layer, derived with it; `None` where nothing of
    /// it is left out.
    port_left_out: Option<LeftOut>,
    accent: Option<Accent>,
    /// The accent layer, derived from `accent` (S9.2).
    accent_layer: Arc<Layer>,
    /// The scheme the application picked; `None`: the reader's.
    picked: Option<Scheme>,
    /// The scheme without a reader (`from_preset`'s).
    default_scheme: Scheme,
    /// The contrast the application set; `None`: the reader's.
    contrast: Option<Contrast>,
    /// The flavour the application set; `None`: the preset's default.
    flavor: Option<Key>,
    /// The current preset's listed paths, its `[pending]` table (S17).
    pending: Vec<Listed>,
    /// The synthetic route's state while the preset it loaded is current
    /// (S16's `Theme::from_preset_synthetic`, S17, ADD-A131); `None` on
    /// every other theme.
    synthetic: Option<Synthetic>,
}

/// What the synthetic route added to a stack (S17, ADD-A131, ADD-A132).
#[derive(Clone, Debug, Default, PartialEq)]
struct Synthetic {
    /// The application layers stating the listed paths.
    layers: Vec<LayerId>,
    /// The listed inputs' per-scheme values folded into the preset's own
    /// input bases, each by its input and scheme.
    inputs: Vec<(InputName, Scheme)>,
    /// The inputs the stand-in states with the preset's own value, the
    /// facts gate's pass layer completing the accent group (S9.2, S17 *A
    /// measured derivation*): a folded value it carries stays marked. Set
    /// with that stand-in alone ([`Stack::set_reader_stand_in_carrying`]),
    /// and emptied wherever the reader or stand-in changes.
    carried: Vec<InputName>,
}

impl Stack {
    /// The stack of a preset's declarations, layer and listed paths
    /// `pending`, with no reader, under `scheme`; refused where the preset
    /// fails validation (S10, S12), a listed path no layer states among its
    /// problems (S17).
    pub(crate) fn new(
        preset: Option<PresetId>,
        declarations: Declarations,
        layer: Layer,
        scheme: Scheme,
        pending: Vec<Listed>,
    ) -> Result<Stack, LayerError> {
        let reused = validate::Reused::default();
        let mut problems = preset_problems(&declarations, &layer, preset, &pending, None, &reused)?;
        let mut stack = Stack {
            preset,
            declarations,
            preset_layer: Arc::new(layer),
            derivation: None,
            reader_left_out: None,
            reader: None,
            layers: Vec::new(),
            palette: None,
            port_left_out: None,
            accent: None,
            accent_layer: Arc::default(),
            picked: None,
            default_scheme: scheme,
            contrast: None,
            flavor: None,
            pending,
            synthetic: None,
        };
        stack.derive_derivation();
        add_problems(&mut problems, stack.validate_reusing(&reused))?;
        if problems.is_empty() {
            Ok(stack)
        } else {
            Err(LayerError::Invalid(problems))
        }
    }

    /// The synthetic route's stack (S16's `Theme::from_preset_synthetic`,
    /// S17, ADD-A131): the preset's declarations and layer, with `layers`
    /// over it as application layers stating its listed paths `pending`,
    /// added at once and validated with the preset, so a listed path none of
    /// them states is S12's missing field. Before validation, the per-scheme
    /// value the last of the layers stating one gives at each listed input's
    /// scheme is folded into the preset's own input base there and marked
    /// synthetic, never staying in its layer (ADD-A132). The layers left
    /// state constant values at disjoint places, which the one pass that
    /// validates them takes for granted: refused, naming each, where one
    /// states an expression or a place another states
    /// ([`validate::synthetic_shape`]). A listed default size is waived and
    /// a listed presence drawn while the preset is current.
    pub(crate) fn synthetic(
        preset: Option<PresetId>,
        declarations: Declarations,
        mut layer: Layer,
        scheme: Scheme,
        pending: Vec<Listed>,
        mut layers: Vec<Layer>,
    ) -> Result<Stack, LayerError> {
        let inputs = fold_inputs(&pending, &mut layer, &mut layers);
        let reused = validate::Reused::default();
        let mut problems = preset_problems(
            &declarations,
            &layer,
            preset,
            &pending,
            Some(&layers),
            &reused,
        )?;
        let shape = validate::synthetic_shape(&layers, &declarations, preset);
        if !shape.is_empty() {
            for problem in shape {
                if !problems.contains(&problem) {
                    problems.push(problem);
                }
            }
            return Err(LayerError::Invalid(problems));
        }
        let layers: Vec<(LayerId, Arc<Layer>)> = layers
            .into_iter()
            .map(|layer| (LayerId::next(), Arc::new(layer)))
            .collect();
        let mut stack = Stack {
            preset,
            declarations,
            preset_layer: Arc::new(layer),
            derivation: None,
            reader_left_out: None,
            reader: None,
            synthetic: Some(Synthetic {
                layers: layers.iter().map(|(id, _)| *id).collect(),
                inputs,
                carried: Vec::new(),
            }),
            layers,
            palette: None,
            port_left_out: None,
            accent: None,
            accent_layer: Arc::default(),
            picked: None,
            default_scheme: scheme,
            contrast: None,
            flavor: None,
            pending,
        };
        stack.derive_derivation();
        add_problems(&mut problems, stack.validate_reusing(&reused))?;
        if problems.is_empty() {
            Ok(stack)
        } else {
            Err(LayerError::Invalid(problems))
        }
    }

    /// The stack of a preset's declarations and its layers, the first the
    /// preset's, a stand-in under `scheme` scaling text by 1, and the
    /// flavour `flavor`, unvalidated: the query alone is tested on it.
    #[cfg(test)]
    pub(crate) fn unvalidated(
        declarations: Declarations,
        layers: Vec<Layer>,
        flavor: Option<Key>,
    ) -> Stack {
        let mut layers = layers.into_iter().map(Arc::new);
        let preset_layer = layers.next().unwrap_or_default();
        Stack {
            preset: None,
            declarations,
            preset_layer,
            derivation: None,
            reader_left_out: None,
            reader: Some(Reader::StandIn {
                state: ReaderState::new(Scheme::Light, 1.0),
                layer: Arc::default(),
            }),
            layers: layers.map(|layer| (LayerId::next(), layer)).collect(),
            palette: None,
            port_left_out: None,
            accent: None,
            accent_layer: Arc::default(),
            picked: None,
            default_scheme: Scheme::Light,
            contrast: None,
            flavor,
            pending: Vec::new(),
            synthetic: None,
        }
    }

    /// `change` over a copy of the stack, kept where the whole stack
    /// validates, else refused with the theme as it was (S8).
    fn change(&mut self, change: impl FnOnce(&mut Stack)) -> Result<(), LayerError> {
        self.change_beside(Vec::new(), change)
    }

    /// [`Stack::change`], refused too with the `problems` found before it,
    /// listed first.
    fn change_beside(
        &mut self,
        mut problems: Vec<Problem>,
        change: impl FnOnce(&mut Stack),
    ) -> Result<(), LayerError> {
        let mut changed = Box::new(self.clone());
        change(&mut changed);
        changed.derive_palette();
        changed.accent_layer = changed.derived_accent();
        changed.derive_derivation();
        add_problems(&mut problems, changed.validate())?;
        if !problems.is_empty() {
            return Err(LayerError::Invalid(problems));
        }
        *self = *changed;
        Ok(())
    }

    /// Sets the system's reader layer, its state and its own preset.
    pub(crate) fn set_reader(
        &mut self,
        state: ReaderState,
        layer: Layer,
        own: Option<PresetId>,
    ) -> Result<(), LayerError> {
        let layer = Arc::new(layer);
        self.change(|stack| {
            stack.reader = Some(Reader::Read { state, layer, own });
            stack.carry(Vec::new());
        })
    }

    /// Sets the stand-in, or replaces the one set; refused while a reader
    /// exists (S8, R2.6), and where its common inputs with its part for the
    /// current preset do not state the preset's accent inputs as one group,
    /// naming those it lacks (S9.2).
    pub(crate) fn set_reader_stand_in(
        &mut self,
        state: ReaderState,
        layer: Layer,
    ) -> Result<(), LayerError> {
        self.set_reader_stand_in_carrying(state, layer, Vec::new())
    }

    /// [`Stack::set_reader_stand_in`] with a stand-in stating the inputs
    /// `carried` with the preset's own values (S9.2, S17 *A measured
    /// derivation*): where one is a listed input's folded synthetic value,
    /// it stays marked synthetic while this stand-in is set (ADD-A132). A
    /// no-op for the marks off the synthetic route.
    pub(crate) fn set_reader_stand_in_carrying(
        &mut self,
        state: ReaderState,
        layer: Layer,
        carried: Vec<InputName>,
    ) -> Result<(), LayerError> {
        if matches!(self.reader, Some(Reader::Read { .. })) {
            return Err(LayerError::ReaderPresent);
        }
        accent_inputs_held(&self.declarations, self.preset, None, Some(&layer))?;
        let layer = Arc::new(layer);
        self.change(|stack| {
            stack.reader = Some(Reader::StandIn { state, layer });
            stack.carry(carried);
        })
    }

    /// Removes the stand-in, back to the no-reader state (S16); a no-op
    /// without one.
    pub(crate) fn remove_reader_stand_in(&mut self) -> Result<(), LayerError> {
        if !matches!(self.reader, Some(Reader::StandIn { .. })) {
            return Ok(());
        }
        self.change(|stack| {
            stack.reader = None;
            stack.carry(Vec::new());
        })
    }

    /// Switches to another preset's declarations, layer and listed paths
    /// `pending` (S16): refused where the preset fails validation, or the
    /// accent set or the stand-in (its common inputs with its part for the
    /// new preset) does not state the new preset's accent inputs as one
    /// group (S9.2, S9.5), or the applied palette has no source for the new
    /// preset (S14), or a listed path no layer states is left (S17),
    /// or a declared input is left with no value at a slot (S12); the
    /// accent layer is derived again and the palette's source chosen again
    /// (S8). On a synthetic theme it ends the synthetic route, removing the
    /// layers it added and the values it folded, the same preset's included
    /// (S17, ADD-A131).
    pub(crate) fn set_preset(
        &mut self,
        preset: Option<PresetId>,
        declarations: Declarations,
        layer: Layer,
        pending: Vec<Listed>,
    ) -> Result<(), LayerError> {
        accent_inputs_held(&declarations, preset, self.accent.as_ref(), self.stand_in())?;
        if let Some((_, palette)) = &self.palette {
            palette_serves(palette, preset)?;
        }
        let problems = preset_problems(
            &declarations,
            &layer,
            preset,
            &pending,
            None,
            &validate::Reused::default(),
        )?;
        self.change_beside(problems, |stack| {
            stack.switch(preset, declarations, layer, pending);
        })
    }

    /// The switch of [`Stack::set_preset`], unvalidated: the flavour set is
    /// kept where the new preset declares it, else the new preset's default
    /// is in force (S16); the synthetic route's layers are removed (S17).
    fn switch(
        &mut self,
        preset: Option<PresetId>,
        declarations: Declarations,
        layer: Layer,
        pending: Vec<Listed>,
    ) {
        if self
            .flavor
            .as_ref()
            .is_some_and(|flavor| !declarations.flavors.contains(flavor))
        {
            self.flavor = None;
        }
        if let Some(synthetic) = self.synthetic.take() {
            self.layers.retain(|(id, _)| !synthetic.layers.contains(id));
        }
        self.preset = preset;
        self.declarations = declarations;
        self.preset_layer = Arc::new(layer);
        self.pending = pending;
    }

    /// Adds an application layer, last, and returns its id.
    pub(crate) fn add_layer(&mut self, layer: Layer) -> Result<LayerId, LayerError> {
        let id = LayerId::next();
        self.change(|stack| stack.layers.push((id, Arc::new(layer))))?;
        Ok(id)
    }

    /// Replaces the application layer `id`, in its place; refused for an
    /// id the stack does not hold (S16).
    pub(crate) fn replace_layer(&mut self, id: LayerId, layer: Layer) -> Result<(), LayerError> {
        let index = self.layer_index(id)?;
        self.change(|stack| {
            if let Some((_, held)) = stack.layers.get_mut(index) {
                *held = Arc::new(layer);
            }
        })
    }

    /// Removes the application layer `id`; refused for an id the stack
    /// does not hold (S16).
    pub(crate) fn remove_layer(&mut self, id: LayerId) -> Result<(), LayerError> {
        self.layer_index(id)?;
        self.change(|stack| stack.layers.retain(|(held, _)| *held != id))
    }

    /// Where the application layer `id` lies among the application's.
    fn layer_index(&self, id: LayerId) -> Result<usize, LayerError> {
        self.layers
            .iter()
            .position(|(held, _)| *held == id)
            .ok_or(LayerError::UnknownLayer(id))
    }

    /// Sets the accent (S9.2), or clears it (`None`: the system's again).
    /// It states the accent inputs the preset declares as one group, the
    /// `accent` by [`Accent::accent`] and the others among
    /// [`Accent::derived`]: at every scheme at which it states any of them
    /// it states every one's per-scheme value; refused, naming those it
    /// lacks, otherwise. A scheme at which it states none keeps the layers'
    /// beneath.
    pub(crate) fn set_accent(&mut self, accent: Option<Accent>) -> Result<(), LayerError> {
        accent_inputs_held(&self.declarations, self.preset, accent.as_ref(), None)?;
        self.change(|stack| stack.accent = accent)
    }

    /// Picks the scheme, or follows the reader's (`None`); under a palette
    /// a scheme it lacks is refused (S14), while following the reader's
    /// suspends the palette at such a scheme.
    pub(crate) fn pick(&mut self, scheme: Option<Scheme>) -> Result<(), LayerError> {
        if let (Some(scheme), Some((_, palette))) = (scheme, &self.palette)
            && !palette.schemes().contains(&scheme)
        {
            return Err(LayerError::SchemeNotInPalette(scheme));
        }
        self.change(|stack| stack.picked = scheme)
    }

    /// Applies a palette, or removes the applied one (`None`; a no-op
    /// without one) (S8, S14): an application layer at the place the first
    /// palette applied took, the last application layer's then, which a
    /// palette replacing it keeps. Refused where it has no source for the
    /// current preset (`Err(PaletteNotForPreset)`) or lacks the scheme in
    /// force (`Err(SchemeNotInPalette)`), and where the stack fails
    /// validation over it.
    pub(crate) fn apply_palette(&mut self, palette: Option<Palette>) -> Result<(), LayerError> {
        let Some(palette) = palette else {
            let Some((id, _)) = self.palette else {
                return Ok(());
            };
            return self.change(|stack| {
                stack.palette = None;
                stack.layers.retain(|(held, _)| *held != id);
            });
        };
        palette_serves(&palette, self.preset)?;
        let scheme = self.scheme();
        if !palette.schemes().contains(&scheme) {
            return Err(LayerError::SchemeNotInPalette(scheme));
        }
        let palette = Arc::new(palette);
        self.change(|stack| match &mut stack.palette {
            Some((_, held)) => *held = palette,
            None => {
                let id = LayerId::next();
                stack.layers.push((id, Arc::default()));
                stack.palette = Some((id, palette));
            }
        })
    }

    /// Sets the palette's entry among the application layers to its source
    /// for the current preset, a KDE port (S14), where the palette applies
    /// under the contrast in force; else to a layer stating nothing: the
    /// palette is skipped under a contrast it states no colours for (S8,
    /// R19). A port states its colours under its scheme alone, so at the
    /// palette's other schemes it is suspended by itself.
    fn derive_palette(&mut self) {
        let layer = self
            .palette_port()
            .map_or_else(Arc::default, |port| Arc::clone(&port.layer));
        let Some((id, _)) = self.palette else {
            return;
        };
        if let Some((_, held)) = self.layers.iter_mut().find(|(held, _)| *held == id) {
            *held = layer;
        }
    }

    /// The palette's source for the current preset where the palette
    /// applies under the contrast in force (S8, S14): its port for the
    /// preset; `None` without a palette or one, or where the contrast in
    /// force is one the port states no colours for.
    fn palette_port(&self) -> Option<&Port> {
        let (_, palette) = self.palette.as_ref()?;
        let port = palette.port(self.preset?)?;
        let contrast = self.contrast();
        contrast
            .is_none_or(|contrast| states_colors_for(&port.layer, contrast))
            .then_some(port)
    }

    /// The schemes at which the palette applies (S8, S14): its port's
    /// scheme, where [`Stack::palette_port`] gives one.
    fn palette_schemes(&self) -> Vec<Scheme> {
        self.palette_port()
            .map(|port| vec![port.scheme])
            .unwrap_or_default()
    }

    /// Sets the contrast, or follows the reader's (`None`).
    pub(crate) fn set_contrast(&mut self, contrast: Option<Contrast>) -> Result<(), LayerError> {
        self.change(|stack| stack.contrast = contrast)
    }

    /// Sets the flavour; refused where the preset does not declare it
    /// (S10, S16).
    pub(crate) fn set_flavor(&mut self, flavor: Key) -> Result<(), LayerError> {
        if !self.declarations.flavors.contains(&flavor) {
            return Err(LayerError::Invalid(vec![Problem::new(
                "",
                ProblemKind::Undeclared {
                    name: flavor.to_string(),
                    declaration: "flavors".to_owned(),
                },
            )]));
        }
        self.change(|stack| stack.flavor = Some(flavor))
    }

    /// The accent layer the accent gives (S9.2): its bases state the
    /// preset's accent inputs, nothing else, at the slots the accent states
    /// them: the `accent`, and of the accent's derived inputs those the
    /// preset's `accent_inputs` names, an accent carrying more for another
    /// preset (S9.5). On `kde-breeze` that is the `accent` alone; Plasma's
    /// derivation from it is the accent-derivation layer's (S8, S9.2;
    /// [`Stack::derive_derivation`]).
    fn derived_accent(&self) -> Arc<Layer> {
        let mut layer = Layer::default();
        if let Some(accent) = &self.accent {
            if let Ok(name) = InputName::new("accent") {
                let entries = &mut layer.bases.inputs.entries;
                entries.push((name, accent.accent.clone()));
            }
            let declared = &self.declarations.accent_inputs;
            layer.bases.inputs.entries.extend(
                accent
                    .derived
                    .iter()
                    .filter(|(name, _)| !is_accent(name) && declared.contains(name))
                    .cloned(),
            );
        }
        Arc::new(layer)
    }

    /// Derives the accent-derivation layer for the current preset and the
    /// slot in force, and S8's leave-out beneath it (S8, S9.2; ADD-A132):
    /// on a preset that derives its accent-dependent roles
    /// ([`derives_accent`]), a layer directly above the preset, beneath the
    /// reader or stand-in; `None` on every other preset. On such a preset
    /// the layer holds its place whatever the accent: where no layer above
    /// the preset gives the accent in force it states nothing, which is
    /// ADD-A132's "no stated accent, no layer", so no place in the stack
    /// moves with `set_accent`. At each scheme where a layer above the
    /// preset gives the accent in force under the contrast in force
    /// ([`Stack::accent_givers`]), and at no other, it states each
    /// accent-dependent role (S9.1's list, conservatively: every key of
    /// every declared set wherever `token.kde.tint_factor` can be `Some`) as
    /// `= kde_accent(<set>, <key>, active)`, each `[WM]` keyed role as
    /// `= kde_accent_wm(<key>)` and each inactive input role as
    /// `= kde_accent(<set>, <key>, inactive)`, nothing else: each function
    /// gives the untinted value where the scheme does not tint (S2.2). So
    /// the layer states both schemes' slots under the contrast in force,
    /// which validation checks it at (S12), the slot in force among them.
    /// The reader's or stand-in's stated roles and inputs among those are
    /// left out at each scheme where a layer above it gives the accent in
    /// force, and a KDE port's at each scheme where the layer states them
    /// ([`LeftOut`]). Beneath a palette applying at a scheme
    /// ([`Stack::palette_schemes`]), the reader's or stand-in's colours,
    /// colour inputs, lists and `token.kde` are left out there, a KDE
    /// port's accent inputs kept (S8, R2.6; ADD-A142).
    fn derive_derivation(&mut self) {
        self.reader_left_out = None;
        self.port_left_out = None;
        let palette = self.palette_schemes();
        let beneath = |left: LeftOut, kept: &[InputName]| {
            if palette.is_empty() {
                left
            } else {
                left.beneath_palette(palette.clone(), kept.to_vec())
            }
        };
        let kept = self.declarations.accent_inputs.clone();
        if self.reader.is_some() && !palette.is_empty() {
            self.reader_left_out = Some(beneath(LeftOut::default(), &kept));
        }
        self.derivation = derives_accent(self.preset).then(Arc::default);
        if self.derivation.is_none() {
            return;
        }
        let givers = self.accent_givers(self.contrast());
        if givers.is_empty() {
            return;
        }
        let roles = validate::accent_dependent_roles(&self.declarations, self.may_tint());
        let inputs = inactive_inputs(&self.declarations);
        let schemes: Vec<Scheme> = givers.iter().map(|(scheme, _)| *scheme).collect();
        self.derivation = Some(Arc::new(derivation_layer(&schemes, &roles, &inputs)));
        let names: Vec<InputName> = inputs.into_iter().map(|(name, _, _)| name).collect();
        if self.reader.is_some() {
            let reader = self.reader_place();
            let above: Vec<Scheme> = givers
                .iter()
                .filter(|(_, giver)| *giver > reader)
                .map(|(scheme, _)| *scheme)
                .collect();
            if !above.is_empty() {
                let left = LeftOut::new(above, &roles, names.clone());
                self.reader_left_out = Some(beneath(left, &kept));
            }
        }
        if !palette.is_empty() {
            self.port_left_out = Some(LeftOut::new(schemes, &roles, names));
        }
    }

    /// Each scheme at which a layer above the preset gives the `input.accent`
    /// in force under `contrast` (S8, S9), with that layer's place in the
    /// stack's order: the last whose bases give it a value there, its
    /// per-contrast base or its per-scheme value, the reader's `accent` not
    /// counting where it is left out (S9.5). The accent-derivation layer
    /// states its roles at these schemes.
    fn accent_givers(&self, contrast: Option<Contrast>) -> Vec<(Scheme, usize)> {
        let left_out = self.reader_accent_left_out();
        let stacked = self.stacked();
        Scheme::ALL
            .into_iter()
            .filter_map(|scheme| {
                let giver = stacked
                    .iter()
                    .enumerate()
                    .skip(1)
                    .filter(|(_, stacked)| !(stacked.kind == LayerKind::Reader && left_out))
                    .filter(|(_, stacked)| {
                        validate::applying_bases(stacked.layer, self.preset)
                            .flat_map(|bases| &bases.inputs.entries)
                            .any(|(name, value)| {
                                is_accent(name) && input_value(value, scheme, contrast).is_some()
                            })
                    })
                    .map(|(place, _)| place)
                    .next_back()?;
                Some((scheme, giver))
            })
            .collect()
    }

    /// Whether `token.kde.tint_factor` can be `Some` (S9.1, conservatively,
    /// S12): a layer's bases or one of its `Tokens` rules state it other than
    /// `none`, or state the KDE tokens whole by reference.
    fn may_tint(&self) -> bool {
        let tints = |tokens: &TokensPatch| match &tokens.kde {
            None | Some(BlockPatch::Set(Value::Absent | Value::Auto)) => false,
            Some(BlockPatch::Fields(kde)) => matches!(kde.tint_factor, Some(Some(_))),
            Some(BlockPatch::Set(Value::Stated(kde))) => kde.tint_factor.is_some(),
            Some(BlockPatch::Set(Value::Expr(_))) => true,
        };
        self.stacked().iter().any(|stacked| {
            validate::applying_bases(stacked.layer, self.preset).any(|bases| tints(&bases.tokens))
                || stacked.layer.rules.iter().any(|rule| match &rule.body {
                    RuleBody::Tokens(tokens) => tints(tokens),
                    RuleBody::Colors(_) | RuleBody::Part(..) | RuleBody::Values(_) => false,
                })
        })
    }

    /// The place in the stack's order of the reader or stand-in, while
    /// one is set: directly above the preset and the accent-derivation
    /// layer (S8).
    fn reader_place(&self) -> usize {
        if self.derivation.is_some() { 2 } else { 1 }
    }

    /// The place in the stack's order of the first application layer
    /// (S8).
    fn first_application_place(&self) -> usize {
        self.reader_place()
            .saturating_add(usize::from(self.reader.is_some()))
    }

    /// The layers in the stack's order, each with what it is to the stack.
    fn stacked(&self) -> Vec<Stacked<'_>> {
        let mut out = vec![Stacked {
            layer: &self.preset_layer,
            kind: LayerKind::Preset,
            left_out: None,
        }];
        if let Some(layer) = &self.derivation {
            out.push(Stacked {
                layer,
                kind: LayerKind::AccentDerivation,
                left_out: None,
            });
        }
        if let Some(Reader::Read { layer, .. } | Reader::StandIn { layer, .. }) = &self.reader {
            out.push(Stacked {
                layer,
                kind: LayerKind::Reader,
                left_out: self.reader_left_out.as_ref(),
            });
        }
        let palette = self.palette.as_ref().map(|(id, _)| *id);
        out.extend(self.layers.iter().map(|(id, layer)| {
            if Some(*id) == palette {
                Stacked {
                    layer,
                    kind: LayerKind::Palette,
                    left_out: self.port_left_out.as_ref(),
                }
            } else {
                Stacked {
                    layer,
                    kind: LayerKind::Application,
                    left_out: None,
                }
            }
        }));
        out.push(Stacked {
            layer: &self.accent_layer,
            kind: LayerKind::Accent,
            left_out: None,
        });
        out
    }

    /// The places in the stack's order of the synthetic route's layers.
    pub(crate) fn synthetic_places(&self) -> Vec<usize> {
        let Some(synthetic) = &self.synthetic else {
            return Vec::new();
        };
        let first = self.first_application_place();
        self.layers
            .iter()
            .enumerate()
            .filter(|(_, (id, _))| synthetic.layers.contains(id))
            .map(|(index, _)| first.saturating_add(index))
            .collect()
    }

    /// Marks the inputs `carried` as the stand-in's copies of the preset's
    /// own values, with the stand-in that carries them; none with any other
    /// reader or stand-in, or none. A no-op off the synthetic route.
    fn carry(&mut self, carried: Vec<InputName>) {
        if let Some(synthetic) = &mut self.synthetic {
            synthetic.carried = carried;
        }
    }

    /// The listed inputs whose folded synthetic value is in force under
    /// `scheme` and `contrast` (S17, ADD-A132): the preset's own input base
    /// gives the input, or the stand-in carrying the preset's value of it
    /// ([`Stack::set_reader_stand_in_carrying`]), at a listed scheme, with no
    /// contrast base of the preset's for the contrast in force, and no layer
    /// above gives one.
    fn synthetic_inputs(
        &self,
        scheme: Scheme,
        contrast: Option<Contrast>,
        input_layers: &[(InputName, usize)],
    ) -> Vec<InputName> {
        let Some(synthetic) = &self.synthetic else {
            return Vec::new();
        };
        let stand_in = match self.reader {
            Some(Reader::StandIn { .. }) => Some(self.reader_place()),
            Some(Reader::Read { .. }) | None => None,
        };
        synthetic
            .inputs
            .iter()
            .filter(|(name, listed)| {
                *listed == scheme
                    && input_layers.iter().any(|(held, layer)| {
                        held == name
                            && (*layer == 0
                                || (Some(*layer) == stand_in && synthetic.carried.contains(name)))
                    })
                    && !self
                        .preset_layer
                        .bases
                        .inputs
                        .entries
                        .iter()
                        .any(|(held, value)| {
                            held == name
                                && contrast.is_some_and(|contrast| {
                                    value.per_contrast.iter().any(|(on, schemes)| {
                                        *on == contrast && schemes.get(scheme).is_some()
                                    })
                                })
                        })
            })
            .map(|(name, _)| name.clone())
            .collect()
    }

    /// The layers in the stack's order, shared.
    fn shared_layers(&self) -> Vec<Arc<Layer>> {
        let mut out = vec![Arc::clone(&self.preset_layer)];
        out.extend(self.derivation.iter().map(Arc::clone));
        if let Some(Reader::Read { layer, .. } | Reader::StandIn { layer, .. }) = &self.reader {
            out.push(Arc::clone(layer));
        }
        out.extend(self.layers.iter().map(|(_, layer)| Arc::clone(layer)));
        out.push(Arc::clone(&self.accent_layer));
        out
    }

    /// Every problem of the stack above its preset, each layer checked over
    /// the layers below it, the reader's or stand-in's state, each listed
    /// path no layer states and each declared input missing at a slot (S8,
    /// S12, S17); the preset was checked when it was set. A layer's check meets the
    /// faults below it its combinations reach again: each is listed once.
    /// Problems are compared by value and a [`Problem`] names no layer, so
    /// two layers with one fault at one path are listed once too: the
    /// caller fixing one layer and retrying is shown the other's.
    fn validate(&self) -> Result<(), LayerError> {
        self.validate_reusing(&validate::Reused::default())
    }

    /// [`Stack::validate`], each layer's checks over combinations taking
    /// from `reused` what the check of the layers below it found that the
    /// layer leaves as it was, the preset's included where it was checked
    /// with `reused`.
    fn validate_reusing(&self, reused: &validate::Reused) -> Result<(), LayerError> {
        let stacked = self.stacked();
        let mut problems: Vec<Problem> = self
            .reader_state()
            .map(validate::reader_state)
            .unwrap_or_default();
        // The synthetic route's layers, constant values at disjoint places
        // (`Stack::synthetic`), are checked at once.
        let synthetic = self.synthetic_places();
        let mut index = 1_usize;
        while let Some(layer) = stacked.get(index) {
            let mut end = index.saturating_add(1);
            if synthetic.contains(&index) {
                while synthetic.contains(&end) {
                    end = end.saturating_add(1);
                }
            }
            let added = stacked.get(index..end).unwrap_or_default();
            let cx = Context {
                declarations: &self.declarations,
                preset: self.preset,
                kind: layer.kind,
                reader_accent_left_out: self.reader_accent_left_out(),
                below: stacked.get(..index).unwrap_or_default(),
            };
            add_problems(&mut problems, validate::check_stacked(added, &cx, reused))?;
            index = end;
        }
        let listed = validate::listed_unstated(
            &self.pending,
            &stacked,
            self.preset,
            self.synthetic.is_some(),
        );
        let inputs = validate::inputs_missing(
            &self.declarations,
            &stacked,
            self.preset,
            self.reader_accent_left_out(),
        );
        for problem in listed.into_iter().chain(inputs) {
            if !problems.contains(&problem) {
                problems.push(problem);
            }
        }
        if problems.is_empty() {
            Ok(())
        } else {
            Err(LayerError::Invalid(problems))
        }
    }

    /// The stand-in's layer; `None` without one, a reader's included.
    fn stand_in(&self) -> Option<&Layer> {
        match &self.reader {
            Some(Reader::StandIn { layer, .. }) => Some(layer),
            Some(Reader::Read { .. }) | None => None,
        }
    }

    /// The state the reader or stand-in reports; `None` without either.
    pub(crate) fn reader_state(&self) -> Option<&ReaderState> {
        match &self.reader {
            Some(Reader::Read { state, .. } | Reader::StandIn { state, .. }) => Some(state),
            None => None,
        }
    }

    /// The scheme in force: picked, else the reader's or stand-in's, else
    /// the one the stack was built with.
    pub(crate) fn scheme(&self) -> Scheme {
        self.picked
            .or_else(|| self.reader_state().map(|state| state.scheme))
            .unwrap_or(self.default_scheme)
    }

    /// The contrast in force: set, else the reader's.
    pub(crate) fn contrast(&self) -> Option<Contrast> {
        self.contrast
            .or_else(|| self.reader_state().and_then(|state| state.contrast))
    }

    /// The flavour in force: set, else the preset's default, its first.
    pub(crate) fn flavor(&self) -> Option<&Key> {
        self.flavor
            .as_ref()
            .or_else(|| self.declarations.flavors.first())
    }

    /// Whether `selector`'s conditions hold for a window whose connector
    /// states `window`, as resolution decides them (S7, S8): the facts
    /// gate's test of which fact applies (S17).
    pub(crate) fn conditions_hold(&self, window: &WindowConditions, selector: &Selector) -> bool {
        Holding {
            reader: self.reader_state(),
            window,
            scheme: self.scheme(),
            contrast: self.contrast(),
            flavor: self.flavor(),
            preset: self.preset,
            declarations: &self.declarations,
        }
        .conditions_hold(selector)
    }

    /// The preset's declarations (S10).
    pub(crate) fn declarations(&self) -> &Declarations {
        &self.declarations
    }

    /// The preset's own layer.
    pub(crate) fn preset_layer(&self) -> &Layer {
        &self.preset_layer
    }

    /// Whether the reader's common accent is left out (S8, S9.5): the
    /// current preset is not the reader's own and declares accent inputs
    /// beyond `accent`.
    fn reader_accent_left_out(&self) -> bool {
        let Some(Reader::Read { own, .. }) = &self.reader else {
            return false;
        };
        *own != self.preset
            && self
                .declarations
                .accent_inputs
                .iter()
                .any(|name| !is_accent(name))
    }

    /// Resolves the stack for a window (S8); records what it read in
    /// `window` and says what changed since the window's last resolution.
    pub(crate) fn resolve(
        &self,
        window: &mut WindowState,
        conditions: &WindowConditions,
    ) -> Resolution {
        let recorded = self.recorded(conditions);
        let change = match &window.last {
            None => Change::First,
            Some(last) => classify(last, &recorded),
        };
        window.last = Some(recorded);
        Resolution {
            theme: self.resolved(conditions),
            change,
        }
    }

    fn recorded(&self, conditions: &WindowConditions) -> Recorded {
        Recorded {
            layers: self.shared_layers(),
            preset: self.preset,
            reader: self.reader_state().cloned(),
            accent_left_out: self.reader_accent_left_out(),
            scheme: self.scheme(),
            contrast: self.contrast(),
            window: conditions.clone(),
            flavor: self.flavor().cloned(),
        }
    }

    /// The resolution's three steps (S8).
    fn resolved(&self, window: &WindowConditions) -> ResolvedTheme {
        let scheme = self.scheme();
        let contrast = self.contrast();
        let reader = self.reader_state();
        let holding = Holding {
            reader,
            window,
            scheme,
            contrast,
            flavor: self.flavor(),
            preset: self.preset,
            declarations: &self.declarations,
        };
        let mut out = ResolvedTheme {
            scheme,
            colors: Vec::new(),
            inputs: Vec::new(),
            input_layers: Vec::new(),
            kde_lists: KdeLists::default(),
            tokens: Vec::new(),
            parts: Vec::new(),
            values: Vec::new(),
            text_scaling_factor: reader.map_or(1.0, |state| state.text_scaling_factor),
            font_dpi: reader.and_then(|state| state.font_dpi),
            content_size_category: reader.and_then(|state| state.content_size_category),
            declarations: self.declarations.clone(),
            flavor: self.flavor().cloned(),
            synthetic: self.synthetic_places(),
            synthetic_inputs: Vec::new(),
        };
        let stacked = self.stacked();
        let left_out = self.reader_accent_left_out();
        // Step 1: every layer's bases, in order, a preset's own part after
        // the common bases while that preset is current.
        for (index, layer) in stacked.iter().enumerate() {
            let omit = Omit::of(layer, scheme, left_out);
            let mut parts: Vec<(&Bases, usize)> = vec![(&layer.layer.bases, 0)];
            for (preset, bases) in &layer.layer.for_preset {
                if Some(*preset) == self.preset {
                    parts.push((bases, 1));
                }
            }
            for (bases, minor) in parts {
                let pos = Pos {
                    rule: false,
                    layer: index,
                    index: minor,
                };
                out.base(bases, pos, scheme, contrast, omit);
            }
        }
        out.synthetic_inputs = self.synthetic_inputs(scheme, contrast, &out.input_layers);
        // Steps 2 and 3: every layer's rules whose conditions hold, in
        // order; a part's or widget's kept with its selector for the query.
        for (index, layer) in stacked.iter().enumerate() {
            let omit = Omit::of(layer, scheme, left_out);
            for (number, rule) in layer.layer.rules.iter().enumerate() {
                if !holding.conditions_hold(&rule.selector) {
                    continue;
                }
                let pos = Pos {
                    rule: true,
                    layer: index,
                    index: number,
                };
                out.rule(&rule.body, &rule.selector, pos, omit);
            }
        }
        out
    }
}

/// Whether the application's `accent` and stand-in `stand_in` state the
/// accent inputs `declarations` declares as one group under the preset
/// `preset` (S9.2): each where [`group_missing`] finds none missing, the
/// stand-in's common inputs with its part for `preset` together; refused,
/// naming each accent input either lacks once, otherwise. Clearing the
/// accent (`None`) and no stand-in lack none; a reader states its own
/// preset's group by construction and is not checked (S9.3).
fn accent_inputs_held(
    declarations: &Declarations,
    preset: Option<PresetId>,
    accent: Option<&Accent>,
    stand_in: Option<&Layer>,
) -> Result<(), LayerError> {
    let group = &declarations.accent_inputs;
    let mut missing = Vec::new();
    if let Some(accent) = accent {
        let name = InputName::new("accent").ok();
        let stated: Vec<(&InputName, &InputValue)> = name
            .iter()
            .map(|name| (name, &accent.accent))
            .chain(
                accent
                    .derived
                    .iter()
                    .filter(|(name, _)| !is_accent(name))
                    .map(|(name, value)| (name, value)),
            )
            .collect();
        missing = group_missing(group, &stated);
    }
    if let Some(layer) = stand_in {
        let stated: Vec<(&InputName, &InputValue)> = validate::applying_bases(layer, preset)
            .flat_map(|bases| bases.inputs.entries.iter())
            .map(|(name, value)| (name, value))
            .collect();
        for name in group_missing(group, &stated) {
            if !missing.contains(&name) {
                missing.push(name);
            }
        }
    }
    if missing.is_empty() {
        Ok(())
    } else {
        Err(LayerError::AccentInputsMissing(missing))
    }
}

/// The accent inputs of `group` (the current preset's `accent_inputs`,
/// S9.1) that the inputs `stated`, one layer's, lack where the group needs
/// them (S9.2): at every scheme at which `stated` gives any of them a
/// value, a per-scheme value or a contrast base, every one's per-scheme
/// value. Each is named once, in the group's order, whatever the schemes.
fn group_missing(group: &[InputName], stated: &[(&InputName, &InputValue)]) -> Vec<InputName> {
    fn values<'a>(
        stated: &'a [(&'a InputName, &'a InputValue)],
        name: &'a InputName,
    ) -> impl Iterator<Item = &'a InputValue> {
        stated
            .iter()
            .filter(move |(held, _)| *held == name)
            .map(|(_, value)| *value)
    }
    let mut missing: Vec<InputName> = Vec::new();
    for scheme in Scheme::ALL {
        let any = group.iter().any(|name| {
            values(stated, name).any(|value| {
                value.per_scheme.get(scheme).is_some()
                    || value
                        .per_contrast
                        .iter()
                        .any(|(_, schemes)| schemes.get(scheme).is_some())
            })
        });
        if !any {
            continue;
        }
        for name in group {
            let held = values(stated, name).any(|value| value.per_scheme.get(scheme).is_some());
            if !held && !missing.contains(name) {
                missing.push(name.clone());
            }
        }
    }
    missing
}

/// A preset's listed path as S11 spells it and its register ids (S17).
pub(crate) type Listed = crate::toml::layer::Listed;

/// The problems of a preset's declarations and layer (S10, S12), its
/// listed paths `pending` left to the layers above it (S17), the synthetic
/// route's layers `synthetic` (S17, ADD-A133), what its
/// checks over combinations found kept in `reused` for the layers' checks
/// above it; a problem other than a list of S12's is the error.
fn preset_problems(
    declarations: &Declarations,
    layer: &Layer,
    preset: Option<PresetId>,
    pending: &[Listed],
    synthetic: Option<&[Layer]>,
    reused: &validate::Reused,
) -> Result<Vec<Problem>, LayerError> {
    let mut problems = Vec::new();
    add_problems(
        &mut problems,
        validate::check_preset(declarations, layer, preset, pending, synthetic, reused),
    )?;
    Ok(problems)
}

/// Adds the problems a check `found` to `problems`, each once; a refusal
/// other than a list of problems is the error.
fn add_problems(
    problems: &mut Vec<Problem>,
    found: Result<(), LayerError>,
) -> Result<(), LayerError> {
    match found {
        Ok(()) => Ok(()),
        Err(LayerError::Invalid(found)) => {
            for problem in found {
                if !problems.contains(&problem) {
                    problems.push(problem);
                }
            }
            Ok(())
        }
        Err(other) => Err(other),
    }
}

/// The synthetic route's fold of the listed inputs (S17, ADD-A132): at
/// each listed `inputs.<name>.<scheme>` of `pending`, the per-scheme value
/// the last of `layers` stating one gives is set in `preset`'s own input
/// base at that scheme and taken out of every one of `layers`; each value
/// folded, by its input and scheme. The layers' other input values stay.
fn fold_inputs(
    pending: &[Listed],
    preset: &mut Layer,
    layers: &mut [Layer],
) -> Vec<(InputName, Scheme)> {
    let mut folded = Vec::new();
    for (path, _) in pending {
        let Some((name, scheme)) = crate::toml::read::listed_input(path) else {
            continue;
        };
        let mut value = None;
        for layer in layers.iter_mut() {
            let entries = &mut layer.bases.inputs.entries;
            for (held, stated) in entries.iter_mut() {
                if *held == name
                    && let Some(taken) = stated.per_scheme.slot(scheme).take()
                {
                    value = Some(taken);
                }
            }
            entries.retain(|(_, stated)| {
                stated.per_scheme.light.is_some()
                    || stated.per_scheme.dark.is_some()
                    || !stated.per_contrast.is_empty()
            });
        }
        let Some(value) = value else {
            continue;
        };
        let entries = &mut preset.bases.inputs.entries;
        match entries.iter_mut().find(|(held, _)| *held == name) {
            Some((_, stated)) => *stated.per_scheme.slot(scheme) = Some(value),
            None => entries.push((
                name.clone(),
                InputValue::new(super::value::SchemeMap::one(scheme, value)),
            )),
        }
        folded.push((name, scheme));
    }
    folded
}

/// Refused where `palette` has neither a mapping nor a port for `preset`
/// (`Err(PaletteNotForPreset)`, S14): a KDE port alone, on `adwaita`. A
/// stack with no bundled preset (`None`, the tests') has no source to
/// choose, and the palette states nothing there.
fn palette_serves(palette: &Palette, preset: Option<PresetId>) -> Result<(), LayerError> {
    match preset {
        Some(preset) if palette.port(preset).is_none() => {
            Err(LayerError::PaletteNotForPreset(preset))
        }
        Some(_) | None => Ok(()),
    }
}

/// Whether `layer` states colours for `contrast` (S8, R19): a colour rule
/// whose conditions name it, or a per-contrast input base for it, in its
/// common bases or a preset's part. A palette that states none is skipped
/// while that contrast holds.
fn states_colors_for(layer: &Layer, contrast: Contrast) -> bool {
    let condition = match contrast {
        Contrast::Increased => Condition::ContrastIncreased,
        Contrast::High => Condition::ContrastHigh,
    };
    let rules = layer.rules.iter().any(|rule| {
        matches!(rule.body, RuleBody::Colors(_))
            && rule.selector.conditions.all_of.contains(&condition)
    });
    let inputs = std::iter::once(&layer.bases)
        .chain(layer.for_preset.iter().map(|(_, bases)| bases))
        .flat_map(|bases| &bases.inputs.entries)
        .any(|(_, value)| value.per_contrast.iter().any(|(held, _)| *held == contrast));
    rules || inputs
}

/// Whether `Theme` derives an accent-derivation layer on `preset` (S8,
/// S9.2): on `kde-breeze`, Plasma's derivation (ADD-A132). `material-3`'s
/// layer enters with its `m3_role` statements (ADD-A134, ADD-A142: the
/// 0.8.0 plan's M2.4); no other preset derives its accent-dependent roles.
fn derives_accent(preset: Option<PresetId>) -> bool {
    preset.is_some_and(|preset| match preset {
        PresetId::KdeBreeze => true,
        PresetId::Adwaita
        | PresetId::MacosTahoe
        | PresetId::MacosMonterey
        | PresetId::Material3
        | PresetId::Ios26
        | PresetId::Ios13
        | PresetId::Windows11 => false,
    })
}

/// The inactive input roles `decl` declares (S9.1), every declared set's
/// and key's `inactive.<set>.<key>`, each with its set and key.
fn inactive_inputs(decl: &Declarations) -> Vec<(InputName, SurfaceSet, KdeSchemeKey)> {
    let mut out = Vec::new();
    for set in &decl.surface_sets {
        for key in KdeSchemeKey::ALL {
            let name = InputName::new(&format!("inactive.{}.{}", set.name(), key.name()));
            if let Ok(name) = name
                && decl.inputs.contains(&name)
            {
                out.push((name, *set, key));
            }
        }
    }
    out
}

/// The accent-derivation layer's bases at `schemes` (S9.2; ADD-A132): each
/// of `roles` as `= kde_accent(<set>, <key>, active)` or, a `[WM]` keyed
/// role, `= kde_accent_wm(<key>)`, and each of the inactive input roles
/// `inputs` as `= kde_accent(<set>, <key>, inactive)`.
fn derivation_layer(
    schemes: &[Scheme],
    roles: &[validate::AccentRole],
    inputs: &[(InputName, SurfaceSet, KdeSchemeKey)],
) -> Layer {
    let name = |text: &'static str| {
        Key::from_static(text).map(|key| Expr::Lit(Literal::Name(Name::Variant(key))))
    };
    let accent = |set: SurfaceSet, key: KdeSchemeKey, group: &'static str| {
        let args = [name(set.name()), name(key.name()), name(group)];
        let args: Option<Vec<Expr>> = args.into_iter().collect();
        args.map(|args| Value::Expr(Expr::Call(Func::KdeAccent, args)))
    };
    let mut surfaces: SurfaceMap<BlockPatch<SurfaceRoles>> = SurfaceMap::default();
    let mut keyed: Vec<(Key, Value<Color>)> = Vec::new();
    for role in roles {
        match *role {
            validate::AccentRole::Surface(set, key) => {
                let Some(value) = accent(set, key, "active") else {
                    continue;
                };
                if surfaces.get(set).is_none() {
                    surfaces.insert(set, BlockPatch::Fields(SurfaceRolesPatch::default()));
                }
                if let Some(BlockPatch::Fields(patch)) = surfaces.get_mut(set) {
                    *key_slot(patch, key) = Some(value);
                }
            }
            validate::AccentRole::Wm(key) => {
                let held = Key::new(&format!("wm_{}", key.name()));
                if let (Ok(held), Some(arg)) = (held, name(key.name())) {
                    let call = Expr::Call(Func::KdeAccentWm, vec![arg]);
                    keyed.push((held, Value::Expr(call)));
                }
            }
        }
    }
    let colors = ColorRolesPatch {
        surfaces: Some(surfaces),
        keyed: Some(keyed),
        ..ColorRolesPatch::default()
    };
    let mut layer = Layer::default();
    for scheme in schemes {
        *layer.bases.colors.slot(*scheme) = Some(colors.clone());
    }
    for (input, set, key) in inputs {
        let Some(value) = accent(*set, *key, "inactive") else {
            continue;
        };
        let mut per_scheme = SchemeMap::default();
        for scheme in schemes {
            *per_scheme.slot(*scheme) = Some(value.clone());
        }
        let entries = &mut layer.bases.inputs.entries;
        entries.push((input.clone(), InputValue::new(per_scheme)));
    }
    layer
}

/// Whether `name` is the `accent` input.
pub(crate) fn is_accent(name: &InputName) -> bool {
    matches!(name.segments(), [only] if only.as_str() == "accent")
}

/// Whether two records' layers are equal, however they were reached: the
/// same shared layer, else equal values.
fn same_layers(last: &[Arc<Layer>], now: &[Arc<Layer>]) -> bool {
    last.len() == now.len()
        && last
            .iter()
            .zip(now)
            .all(|(a, b)| Arc::ptr_eq(a, b) || a == b)
}

/// What changed between two resolutions of a window (S8, S4.3).
fn classify(last: &Recorded, now: &Recorded) -> Change {
    let reader_values = match (&last.reader, &now.reader) {
        (Some(a), Some(b)) => a.values_differ(b),
        (None, None) => false,
        _ => true,
    };
    let theme = !same_layers(&last.layers, &now.layers)
        || last.preset != now.preset
        || last.accent_left_out != now.accent_left_out
        || reader_values
        || last.scheme != now.scheme
        || last.contrast != now.contrast;
    let reader_conditions = match (&last.reader, &now.reader) {
        (Some(a), Some(b)) => a.conditions_differ(b),
        _ => false,
    };
    if theme {
        Change::ThemeChange
    } else if reader_conditions || last.window != now.window || last.flavor != now.flavor {
        Change::ConditionChange
    } else {
        Change::Unchanged
    }
}

impl ResolvedTheme {
    /// Step 1 for one layer's bases at `pos`: its colours under `scheme`,
    /// its inputs (each replaced at the slot in force alone, S9), KDE's
    /// lists, its tokens and its widgets' patches, what `omit` names left
    /// out. A layer's part for the current preset comes after its common
    /// bases, so it replaces them slot by slot too.
    fn base(
        &mut self,
        bases: &Bases,
        pos: Pos,
        scheme: Scheme,
        contrast: Option<Contrast>,
        omit: Omit<'_>,
    ) {
        if let Some(colors) = bases.colors.get(scheme).and_then(|c| omit.roles(c)) {
            self.colors.push(step(pos, None, colors));
        }
        self.inputs(&bases.inputs, pos.layer, scheme, contrast, omit);
        let tokens = omit.tokens(&bases.tokens);
        if *tokens != TokensPatch::default() {
            self.tokens.push(step(pos, None, tokens));
        }
        let (parts, values) = widget_patches(&bases.widgets);
        for (part, patch) in parts {
            self.part(part, step(pos, None, patch));
        }
        for (widget, patch) in values {
            self.value(widget, step(pos, None, patch));
        }
    }

    fn inputs(
        &mut self,
        inputs: &InputBases,
        layer: usize,
        scheme: Scheme,
        contrast: Option<Contrast>,
        omit: Omit<'_>,
    ) {
        for (name, value) in &inputs.entries {
            if omit.input(name) {
                continue;
            }
            // A layer giving no value at the slot in force leaves the
            // earlier layers' (S9, S9.3).
            if let Some(value) = input_value(value, scheme, contrast) {
                self.inputs.retain(|(held, _)| held != name);
                self.inputs.push((name.clone(), value));
                self.input_layers.retain(|(held, _)| held != name);
                self.input_layers.push((name.clone(), layer));
            }
        }
        if omit.under_palette() {
            return;
        }
        let lists = &mut self.kde_lists;
        if let Some(groups) = inputs.kde_groups.get(scheme) {
            lists.groups.clone_from(groups);
        }
        if let Some(groups) = inputs.kde_inactive_groups.get(scheme) {
            lists.inactive_groups.clone_from(groups);
        }
        if let Some(keys) = inputs.kde_unset_keys.get(scheme) {
            lists.unset_keys.clone_from(keys);
        }
    }

    /// Steps 2 and 3 for one rule whose conditions hold, what `omit` names
    /// left out of a `Colors` or `Tokens` rule.
    fn rule(&mut self, body: &RuleBody, selector: &Selector, pos: Pos, omit: Omit<'_>) {
        match body {
            RuleBody::Colors(roles) => {
                if let Some(roles) = omit.roles(roles) {
                    self.colors.push(step(pos, None, roles));
                }
            }
            RuleBody::Tokens(tokens) => self.tokens.push(step(pos, None, omit.tokens(tokens))),
            RuleBody::Part(PartRef::Typed(part), patch) => {
                self.part(*part, step(pos, Some(selector.clone()), patch.clone()));
            }
            // A component's rule names a component no preset declares yet
            // (they are staged), which validation refuses (S10, S12).
            RuleBody::Part(PartRef::Component { .. }, _) => {}
            RuleBody::Values(ValuesPatch::Typed(values)) => {
                let widget = values_widget(values);
                self.value(widget, step(pos, Some(selector.clone()), values.clone()));
            }
        }
    }

    fn part(&mut self, part: TypedPart, step: Step<PartPatch>) {
        match self.parts.iter_mut().find(|(held, _)| *held == part) {
            Some((_, steps)) => steps.push(step),
            None => self.parts.push((part, vec![step])),
        }
    }

    fn value(&mut self, widget: WidgetKind, step: Step<WidgetValuesPatch>) {
        match self.values.iter_mut().find(|(held, _)| *held == widget) {
            Some((_, steps)) => steps.push(step),
            None => self.values.push((widget, vec![step])),
        }
    }

    /// The steps the typed part `part` keeps, in order; none where no
    /// layer patches it.
    pub(crate) fn part_steps(&self, part: TypedPart) -> &[Step<PartPatch>] {
        self.parts
            .iter()
            .find(|(held, _)| *held == part)
            .map(|(_, steps)| steps.as_slice())
            .unwrap_or_default()
    }

    /// Whether a step the element keeps (`kept`) states `part`'s presence,
    /// `Absent`, `Present` or drawn as another (S6.3): where none does, a
    /// pending presence fact has no presence beneath it, a listed presence
    /// (S17, ADD-A131).
    pub(crate) fn presence_stated(
        &self,
        part: TypedPart,
        kept: impl Fn(&Step<PartPatch>) -> bool,
    ) -> bool {
        self.part_steps(part)
            .iter()
            .filter(|step| kept(step))
            .any(|step| step.patch.presence.is_some())
    }

    /// Whether a synthetic layer's step decides whether `part` is drawn
    /// (S17, ADD-A128), `kept` telling the steps the element keeps: the
    /// last kept step stating a presence is a synthetic layer's and makes
    /// the part other than the last such step of another layer does, no
    /// step's presence counting as drawn (S6.3). A synthetic step stating
    /// no presence decides nothing of it. S17's own synthetic layers state
    /// no presence (a listed presence is drawn, the route stating nothing
    /// for it), so on the gate's route this is never true; it holds for the
    /// layers a caller gives `Theme::from_preset_synthetic`.
    pub(crate) fn shown_by_synthetic(
        &self,
        part: TypedPart,
        kept: impl Fn(&Step<PartPatch>) -> bool,
    ) -> bool {
        let mut steps = self
            .part_steps(part)
            .iter()
            .rev()
            .filter(|step| kept(step))
            .filter_map(|step| Some((step.pos.layer, step.patch.presence.as_ref()?)));
        let Some((layer, last)) = steps.next() else {
            return false;
        };
        if !self.synthetic.contains(&layer) {
            return false;
        }
        let below = steps
            .find(|(layer, _)| !self.synthetic.contains(layer))
            .map_or(Shown::Drawn, |(_, presence)| Shown::from(presence));
        Shown::from(last) != below
    }
}

/// What resolution leaves out of one layer at the slot in force (S8): its
/// `accent` input where `accent` (S9.5), and what `left_out` names at the
/// scheme in force `scheme`.
#[derive(Clone, Copy)]
struct Omit<'a> {
    accent: bool,
    left_out: Option<&'a LeftOut>,
    scheme: Scheme,
}

impl<'a> Omit<'a> {
    /// What is left out of `layer` at `scheme`, its `accent` where it is
    /// the reader's and `reader_accent` holds.
    fn of(layer: &Stacked<'a>, scheme: Scheme, reader_accent: bool) -> Omit<'a> {
        Omit {
            accent: layer.kind == LayerKind::Reader && reader_accent,
            left_out: layer.left_out.filter(|left| left.at(scheme)),
            scheme,
        }
    }

    /// Whether the layer's input `name` is left out.
    fn input(self, name: &InputName) -> bool {
        (self.accent && is_accent(name))
            || self
                .left_out
                .is_some_and(|left| left.input(self.scheme, name))
    }

    /// Whether a palette above the layer leaves its colours, lists and
    /// `token.kde` out.
    fn under_palette(self) -> bool {
        self.left_out
            .is_some_and(|left| left.under_palette(self.scheme))
    }

    /// `roles` without the roles left out; `None` beneath a palette.
    fn roles(self, roles: &ColorRolesPatch) -> Option<ColorRolesPatch> {
        if self.under_palette() {
            return None;
        }
        let mut roles = roles.clone();
        if let Some(left) = self.left_out {
            left.strip(&mut roles);
        }
        Some(roles)
    }

    /// `tokens` without `token.kde` beneath a palette.
    fn tokens(self, tokens: &TokensPatch) -> Box<TokensPatch> {
        let mut tokens = Box::new(tokens.clone());
        if self.under_palette() {
            tokens.kde = None;
        }
        tokens
    }
}

fn step<P>(pos: Pos, selector: Option<Selector>, patch: P) -> Step<P> {
    Step {
        pos,
        selector,
        patch,
    }
}

/// An input's value under `scheme`: its per-contrast base where `contrast`
/// holds and it states one for it, else its per-scheme value (S9).
pub(crate) fn input_value(
    value: &InputValue,
    scheme: Scheme,
    contrast: Option<Contrast>,
) -> Option<Value<Color>> {
    let per_contrast = contrast.and_then(|contrast| {
        value
            .per_contrast
            .iter()
            .find(|(held, _)| *held == contrast)
            .and_then(|(_, schemes)| schemes.get(scheme))
    });
    per_contrast
        .or_else(|| value.per_scheme.get(scheme))
        .cloned()
}

/// A layer's typed parts' patches.
type PartPatches = Vec<(TypedPart, PartPatch)>;

/// A layer's typed widgets' values patches.
type ValuesPatches = Vec<(WidgetKind, WidgetValuesPatch)>;

/// The typed widgets' part and values patches a layer's bases state.
fn widget_patches(widgets: &WidgetsPatch) -> (PartPatches, ValuesPatches) {
    let mut parts = Vec::new();
    let mut values = Vec::new();
    if let Some(button) = &widgets.button {
        let (own, own_values) = (**button).clone().into_parts();
        parts.extend(
            own.into_iter()
                .map(|(part, patch)| (TypedPart::Button(part), patch)),
        );
        if let Some(own_values) = own_values {
            values.push((
                WidgetKind::Button,
                WidgetValuesPatch::Button(Box::new(own_values)),
            ));
        }
    }
    (parts, values)
}

fn values_widget(values: &WidgetValuesPatch) -> WidgetKind {
    match values {
        WidgetValuesPatch::Button(_) => WidgetKind::Button,
    }
}

/// The conditions in force for one window (S8): the reader's or stand-in's,
/// the window's, and the application's (its scheme, contrast and flavour,
/// which win over the reader's).
struct Holding<'a> {
    reader: Option<&'a ReaderState>,
    window: &'a WindowConditions,
    scheme: Scheme,
    contrast: Option<Contrast>,
    flavor: Option<&'a Key>,
    preset: Option<PresetId>,
    declarations: &'a Declarations,
}

impl Holding<'_> {
    /// Whether a rule's conditions hold: every one of `all_of`, none of
    /// `none_of` (S7). The selector's other parts are the element's, which
    /// the query matches (S8 step 3).
    fn conditions_hold(&self, selector: &Selector) -> bool {
        let conditions = &selector.conditions;
        conditions.all_of.iter().all(|c| self.holds(c))
            && !conditions.none_of.iter().any(|c| self.holds(c))
    }

    fn holds(&self, condition: &Condition) -> bool {
        let reader = |read: fn(&ReaderState) -> bool| self.reader.is_some_and(read);
        let window = self.window;
        match condition {
            Condition::Scheme(scheme) => self.scheme == *scheme,
            Condition::ContrastIncreased => self.contrast == Some(Contrast::Increased),
            Condition::ContrastHigh => self.contrast == Some(Contrast::High),
            Condition::ReducedTransparency => reader(|r| r.reduced_transparency),
            Condition::ReducedMotion => reader(|r| r.reduced_motion),
            Condition::AnimationsOff => reader(|r| r.animations_off),
            Condition::BoldText => reader(|r| r.bold_text),
            Condition::ButtonShapes => reader(|r| r.button_shapes),
            Condition::DifferentiateWithoutColor => reader(|r| r.differentiate_without_color),
            Condition::TabletMode => reader(|r| r.tablet_mode),
            Condition::MobileFormFactor => reader(|r| r.mobile_form_factor),
            Condition::BatterySaver => reader(|r| r.battery_saver),
            Condition::OverlayScrollbars => reader(|r| r.overlay_scrollbars),
            Condition::OnOffLabels => reader(|r| r.on_off_labels),
            Condition::FullKeyboardAccess => reader(|r| r.full_keyboard_access),
            Condition::PreferCrossFade => reader(|r| r.prefer_cross_fade),
            Condition::AccessibilityTextSize => self
                .reader
                .and_then(|r| r.content_size_category)
                .is_some_and(ContentSizeCategory::is_accessibility),
            Condition::ContainerSize { container, size } => {
                self.container_size(container) == Some(size)
            }
            Condition::Idiom(idiom) => self.reader.and_then(|r| r.idiom) == Some(*idiom),
            Condition::SystemSetting(key) => {
                self.reader.is_some_and(|r| r.system_settings.contains(key))
            }
            Condition::DeviceModel(key) => {
                self.reader.and_then(|r| r.device_model.as_ref()) == Some(key)
            }
            Condition::WindowInactive => window.inactive,
            Condition::NotMain => window.not_main,
            Condition::AppInactive => window.app_inactive,
            Condition::Maximized => window.maximized,
            Condition::FullScreen => window.full_screen,
            // A window tiled to these edges at least; a maximized one to
            // every edge (S12).
            Condition::Tiled(edges) => {
                window.maximized
                    || ((!edges.top || window.tiled.top)
                        && (!edges.right || window.tiled.right)
                        && (!edges.bottom || window.tiled.bottom)
                        && (!edges.left || window.tiled.left))
            }
            Condition::Elevated => window.elevated,
            Condition::Input(input) => window.input == *input,
            Condition::NotComposited => window.not_composited,
            Condition::Shaded => window.shaded,
            Condition::AttentionFlash => window.attention_flash,
            Condition::TintDimmed => window.tint_dimmed,
            Condition::MnemonicsVisible => window.mnemonics_visible,
            Condition::Freeform => window.freeform,
            Condition::Posture(posture) => window.posture == Some(*posture),
            Condition::UnitScale => window.unit_scale,
            Condition::Flavor(flavor) => self.flavor == Some(flavor),
            Condition::Preset(preset) => self.preset == Some(*preset),
        }
    }

    /// The size a container reports where the preset declares it, else its
    /// declared default, the first (S10), with or without a reader: exactly
    /// one declared size holds (S7.1).
    fn container_size(&self, container: &WidgetRef) -> Option<&Key> {
        let declared = self
            .declarations
            .container_sizes
            .iter()
            .find(|(held, _)| held == container)
            .map(|(_, sizes)| sizes.as_slice())
            .unwrap_or_default();
        self.reader
            .and_then(|r| {
                r.container_sizes
                    .iter()
                    .find(|(held, _)| held == container)
                    .map(|(_, size)| size)
            })
            .filter(|size| declared.contains(size))
            .or_else(|| declared.first())
    }
}

/// The stack's refusals, each triggered (S8, S9.2, S16): a stand-in while a
/// reader exists, an accent lacking a declared accent input, an id the
/// stack does not hold.
#[cfg(test)]
pub(crate) fn refusals() -> Vec<LayerError> {
    tests::refusals()
}

#[cfg(test)]
mod mutation_tests;
#[cfg(test)]
mod tests;
