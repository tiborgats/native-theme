//! S8's stack and its resolution for a window: the layers in their order
//! (the preset, the reader or its stand-in, the application's layers, the
//! accent layer), the conditions in force, and the resolution's three
//! steps: the bases merged in order, then the `Colors` and `Tokens` rules
//! whose conditions hold, then each part's and widget's base and its rules
//! whose conditions hold, kept in order for the query (S8); their
//! evaluation for an element ([`evaluate`]), a `Fields` patch over a
//! reference composed lazily ([`compose`], S2.2). Also S9.2's accent layer
//! (its inputs; `kde-breeze`'s derived roles are M1b's).
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

use super::color::Color;
use super::declare::Declarations;
use super::error::{LayerError, Problem, ProblemKind};
use super::inputs::{Accent, Contrast, InputBases, InputName, InputValue};
use super::key::Key;
use super::layer::{Bases, Layer, LayerId};
use super::presets::PresetId;
use super::roles::{ColorRolesPatch, SurfaceSet};
use super::selector::{
    Condition, Idiom, InputKind, PartRef, Posture, RuleBody, Scheme, Selector, TiledEdges,
    ValuesPatch, WidgetRef,
};
use super::tokens::TokensPatch;
use super::validate::{self, Context, LayerKind, Stacked};
use super::value::Value;
use super::widgets::{PartPatch, TypedPart, WidgetKind, WidgetValuesPatch, WidgetsPatch};

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
    /// Each input's value under the scheme and contrast in force: the last
    /// layer's that states the input, its per-contrast base where a contrast
    /// condition holds and it states one (S9).
    pub(crate) inputs: Vec<(InputName, Value<Color>)>,
    /// KDE's three lists under the scheme in force, the last layer's for it
    /// (S9.1).
    pub(crate) kde_lists: KdeLists,
    /// The tokens' bases, then the `Tokens` rules whose conditions hold.
    pub(crate) tokens: Vec<Step<TokensPatch>>,
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
    reader: Option<Reader>,
    /// The application's layers, each with the id it was added under.
    layers: Vec<(LayerId, Arc<Layer>)>,
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
}

impl Stack {
    /// The stack of a preset's declarations and layer, with no reader, under
    /// `scheme`; refused where the preset fails validation (S10, S12).
    pub(crate) fn new(
        preset: Option<PresetId>,
        declarations: Declarations,
        layer: Layer,
        scheme: Scheme,
    ) -> Result<Stack, LayerError> {
        let stack = Stack {
            preset,
            declarations,
            preset_layer: Arc::new(layer),
            reader: None,
            layers: Vec::new(),
            accent: None,
            accent_layer: Arc::default(),
            picked: None,
            default_scheme: scheme,
            contrast: None,
            flavor: None,
        };
        validate::check_preset(&stack.declarations, &stack.preset_layer, preset)?;
        stack.validate()?;
        Ok(stack)
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
            reader: Some(Reader::StandIn {
                state: ReaderState::new(Scheme::Light, 1.0),
                layer: Arc::default(),
            }),
            layers: layers.map(|layer| (LayerId::next(), layer)).collect(),
            accent: None,
            accent_layer: Arc::default(),
            picked: None,
            default_scheme: Scheme::Light,
            contrast: None,
            flavor,
        }
    }

    /// `change` over a copy of the stack, kept where the whole stack
    /// validates, else refused with the theme as it was (S8).
    fn change(&mut self, change: impl FnOnce(&mut Stack)) -> Result<(), LayerError> {
        let mut changed = Box::new(self.clone());
        change(&mut changed);
        changed.accent_layer = changed.derived_accent();
        changed.validate()?;
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
        self.change(|stack| stack.reader = Some(Reader::Read { state, layer, own }))
    }

    /// Sets the stand-in, or replaces the one set; refused while a reader
    /// exists (S8, R2.6).
    pub(crate) fn set_reader_stand_in(
        &mut self,
        state: ReaderState,
        layer: Layer,
    ) -> Result<(), LayerError> {
        if matches!(self.reader, Some(Reader::Read { .. })) {
            return Err(LayerError::ReaderPresent);
        }
        let layer = Arc::new(layer);
        self.change(|stack| stack.reader = Some(Reader::StandIn { state, layer }))
    }

    /// Removes the stand-in, back to the no-reader state (S16); a no-op
    /// without one.
    pub(crate) fn remove_reader_stand_in(&mut self) -> Result<(), LayerError> {
        if !matches!(self.reader, Some(Reader::StandIn { .. })) {
            return Ok(());
        }
        self.change(|stack| stack.reader = None)
    }

    /// Switches to another preset's declarations and layer (S16): refused
    /// where the preset fails validation, or the accent set lacks one of
    /// its accent inputs (S9.2); the accent layer is derived again (S8).
    pub(crate) fn set_preset(
        &mut self,
        preset: Option<PresetId>,
        declarations: Declarations,
        layer: Layer,
    ) -> Result<(), LayerError> {
        accent_inputs_held(&declarations, self.accent.as_ref())?;
        validate::check_preset(&declarations, &layer, preset)?;
        self.change(|stack| stack.switch(preset, declarations, layer))
    }

    /// The switch of [`Stack::set_preset`], unvalidated: the flavour set is
    /// kept where the new preset declares it, else the new preset's default
    /// is in force (S16).
    fn switch(&mut self, preset: Option<PresetId>, declarations: Declarations, layer: Layer) {
        if self
            .flavor
            .as_ref()
            .is_some_and(|flavor| !declarations.flavors.contains(flavor))
        {
            self.flavor = None;
        }
        self.preset = preset;
        self.declarations = declarations;
        self.preset_layer = Arc::new(layer);
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
    /// It states every accent input the preset declares, the `accent` by
    /// [`Accent::accent`] and the others among [`Accent::derived`];
    /// refused, naming those it lacks, otherwise.
    pub(crate) fn set_accent(&mut self, accent: Option<Accent>) -> Result<(), LayerError> {
        accent_inputs_held(&self.declarations, accent.as_ref())?;
        self.change(|stack| stack.accent = accent)
    }

    /// Picks the scheme, or follows the reader's (`None`).
    pub(crate) fn pick(&mut self, scheme: Option<Scheme>) -> Result<(), LayerError> {
        self.change(|stack| stack.picked = scheme)
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
    /// preset's accent inputs, nothing else: the `accent`, and of the
    /// accent's derived inputs those the preset's `accent_inputs` names, an
    /// accent carrying more for another preset (S9.5) (`kde-breeze`'s
    /// derived roles enter with its preset, M1b).
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

    /// The layers in the stack's order, each with what it is to the stack.
    fn stacked(&self) -> Vec<Stacked<'_>> {
        let mut out = vec![Stacked {
            layer: &self.preset_layer,
            kind: LayerKind::Preset,
        }];
        if let Some(Reader::Read { layer, .. } | Reader::StandIn { layer, .. }) = &self.reader {
            out.push(Stacked {
                layer,
                kind: LayerKind::Reader,
            });
        }
        out.extend(self.layers.iter().map(|(_, layer)| Stacked {
            layer,
            kind: LayerKind::Application,
        }));
        out.push(Stacked {
            layer: &self.accent_layer,
            kind: LayerKind::Accent,
        });
        out
    }

    /// The layers in the stack's order, shared.
    fn shared_layers(&self) -> Vec<Arc<Layer>> {
        let mut out = vec![Arc::clone(&self.preset_layer)];
        if let Some(Reader::Read { layer, .. } | Reader::StandIn { layer, .. }) = &self.reader {
            out.push(Arc::clone(layer));
        }
        out.extend(self.layers.iter().map(|(_, layer)| Arc::clone(layer)));
        out.push(Arc::clone(&self.accent_layer));
        out
    }

    /// Every problem of the stack above its preset, each layer checked over
    /// the layers below it, and the reader's or stand-in's state (S8, S12);
    /// the preset was checked when it was set. A layer's check meets the
    /// faults below it its combinations reach again: each is listed once.
    fn validate(&self) -> Result<(), LayerError> {
        let stacked = self.stacked();
        let mut problems: Vec<Problem> = self
            .reader_state()
            .map(validate::reader_state)
            .unwrap_or_default();
        for (index, layer) in stacked.iter().enumerate().skip(1) {
            let cx = Context {
                declarations: &self.declarations,
                preset: self.preset,
                kind: layer.kind,
                reader_accent_left_out: self.reader_accent_left_out(),
                below: stacked.get(..index).unwrap_or_default(),
            };
            match validate::check(layer.layer, &cx) {
                Ok(()) => {}
                Err(LayerError::Invalid(found)) => {
                    for problem in found {
                        if !problems.contains(&problem) {
                            problems.push(problem);
                        }
                    }
                }
                Err(other) => return Err(other),
            }
        }
        if problems.is_empty() {
            Ok(())
        } else {
            Err(LayerError::Invalid(problems))
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
    fn contrast(&self) -> Option<Contrast> {
        self.contrast
            .or_else(|| self.reader_state().and_then(|state| state.contrast))
    }

    /// The flavour in force: set, else the preset's default, its first.
    pub(crate) fn flavor(&self) -> Option<&Key> {
        self.flavor
            .as_ref()
            .or_else(|| self.declarations.flavors.first())
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
            kde_lists: KdeLists::default(),
            tokens: Vec::new(),
            parts: Vec::new(),
            values: Vec::new(),
            text_scaling_factor: reader.map_or(1.0, |state| state.text_scaling_factor),
            font_dpi: reader.and_then(|state| state.font_dpi),
            content_size_category: reader.and_then(|state| state.content_size_category),
            declarations: self.declarations.clone(),
            flavor: self.flavor().cloned(),
        };
        let stacked = self.stacked();
        let left_out = self.reader_accent_left_out();
        // Step 1: every layer's bases, in order, a preset's own part after
        // the common bases while that preset is current.
        for (index, layer) in stacked.iter().enumerate() {
            let drop_accent = layer.kind == LayerKind::Reader && left_out;
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
                out.base(bases, pos, scheme, contrast, drop_accent);
            }
        }
        // Steps 2 and 3: every layer's rules whose conditions hold, in
        // order; a part's or widget's kept with its selector for the query.
        for (index, layer) in stacked.iter().enumerate() {
            for (number, rule) in layer.layer.rules.iter().enumerate() {
                if !holding.conditions_hold(&rule.selector) {
                    continue;
                }
                let pos = Pos {
                    rule: true,
                    layer: index,
                    index: number,
                };
                out.rule(&rule.body, &rule.selector, pos);
            }
        }
        out
    }
}

/// Whether `accent` states every accent input `declarations` declares: the
/// `accent` by [`Accent::accent`], the others among [`Accent::derived`];
/// refused, naming those it lacks, otherwise (S9.2). Clearing the accent
/// (`None`) lacks none.
fn accent_inputs_held(
    declarations: &Declarations,
    accent: Option<&Accent>,
) -> Result<(), LayerError> {
    let Some(accent) = accent else {
        return Ok(());
    };
    let missing: Vec<InputName> = declarations
        .accent_inputs
        .iter()
        .filter(|name| !(is_accent(name) || accent.derived.iter().any(|(held, _)| held == *name)))
        .cloned()
        .collect();
    if missing.is_empty() {
        Ok(())
    } else {
        Err(LayerError::AccentInputsMissing(missing))
    }
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
    /// its inputs (a later layer's replacing an earlier one whole; the
    /// accent left out where `drop_accent`), KDE's lists, its tokens and its
    /// widgets' patches.
    fn base(
        &mut self,
        bases: &Bases,
        pos: Pos,
        scheme: Scheme,
        contrast: Option<Contrast>,
        drop_accent: bool,
    ) {
        if let Some(colors) = bases.colors.get(scheme) {
            self.colors.push(step(pos, None, colors.clone()));
        }
        self.inputs(&bases.inputs, scheme, contrast, drop_accent);
        if bases.tokens != TokensPatch::default() {
            self.tokens.push(step(pos, None, bases.tokens.clone()));
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
        scheme: Scheme,
        contrast: Option<Contrast>,
        drop_accent: bool,
    ) {
        for (name, value) in &inputs.entries {
            if drop_accent && is_accent(name) {
                continue;
            }
            // A layer stating no value for the scheme in force leaves the
            // earlier layers' (S9.3).
            if let Some(value) = input_value(value, scheme, contrast) {
                self.inputs.retain(|(held, _)| held != name);
                self.inputs.push((name.clone(), value));
            }
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

    /// Steps 2 and 3 for one rule whose conditions hold.
    fn rule(&mut self, body: &RuleBody, selector: &Selector, pos: Pos) {
        match body {
            RuleBody::Colors(roles) => self.colors.push(step(pos, None, roles.clone())),
            RuleBody::Tokens(tokens) => self.tokens.push(step(pos, None, tokens.clone())),
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
        let (own, own_values) = button.clone().into_parts();
        parts.extend(
            own.into_iter()
                .map(|(part, patch)| (TypedPart::Button(part), patch)),
        );
        if let Some(own_values) = own_values {
            values.push((WidgetKind::Button, WidgetValuesPatch::Button(own_values)));
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
