//! S16's `Theme`: the one owner of the stack, the inputs and the
//! application's conditions. Every mutation validates the whole stack and
//! is refused whole on failure, the theme staying as it was (S8, S12).

use super::error::{LayerError, ThemeError};
use super::inputs::{Accent, Contrast};
use super::key::Key;
use super::layer::{Layer, LayerId};
use super::presets::PresetId;
use super::resolve::{ReaderState, Resolution, Stack, WindowConditions, WindowState};
use super::selector::Scheme;
use super::toml::layer::read_preset;

/// A theme: a bundled preset, the reader or its stand-in, the
/// application's layers and the accent it set, and the application's
/// conditions (S8, S16). Not `Clone`: a copy would hold the original's
/// layers under their ids, which name one theme's layers only (S1).
#[derive(Debug)]
#[non_exhaustive]
pub struct Theme {
    preset: PresetId,
    stack: Stack,
}

impl Theme {
    /// The bundled preset `id` with no reader, under `scheme`: S16's
    /// no-reader state. A preset failing validation is
    /// `Err(ThemeError::Preset)`, which a build test rules out (S19.1).
    pub fn from_preset(id: PresetId, scheme: Scheme) -> Result<Theme, ThemeError> {
        let stack = preset_stack(Some(id), id.source(), scheme)?;
        Ok(Theme { preset: id, stack })
    }

    /// Switches to the bundled preset `id`, a theme change (S16): the
    /// reader's or stand-in's common part applies to any preset, its
    /// `[for_preset.<id>]` part and preset-conditioned rules to their own;
    /// the accent layer is derived again; the flavour set is kept where the
    /// new preset declares it, else the new preset's default is in force.
    /// Refused where the accent set lacks one of the new preset's accent
    /// inputs (S9.2), or the stack fails validation over it.
    pub fn set_preset(&mut self, id: PresetId) -> Result<(), LayerError> {
        let file = read_preset(id.source())?;
        self.stack
            .set_preset(Some(id), file.declarations, file.layer)?;
        self.preset = id;
        Ok(())
    }

    /// Sets the reader stand-in, or replaces the one set (S8, R2.6): its
    /// state and its layer, validated as a reader's; refused while a reader
    /// exists (`Err(ReaderPresent)`).
    pub fn set_reader_stand_in(
        &mut self,
        state: ReaderState,
        layer: Layer,
    ) -> Result<(), LayerError> {
        self.stack.set_reader_stand_in(state, layer)
    }

    /// Removes the stand-in, back to the no-reader state (S16); a no-op
    /// without a stand-in.
    pub fn remove_reader_stand_in(&mut self) -> Result<(), LayerError> {
        self.stack.remove_reader_stand_in()
    }

    /// Picks the scheme; `None`: follow the reader's or stand-in's, else
    /// the one [`Theme::from_preset`] was given.
    pub fn pick(&mut self, scheme: Option<Scheme>) -> Result<(), LayerError> {
        self.stack.pick(scheme)
    }

    /// Sets the flavour (R1.6); refused where the preset does not declare
    /// it (S10).
    pub fn set_flavor(&mut self, flavor: Key) -> Result<(), LayerError> {
        self.stack.set_flavor(flavor)
    }

    /// The current preset.
    #[must_use]
    pub fn preset(&self) -> PresetId {
        self.preset
    }

    /// The flavour in force: set, or the preset's default, its first
    /// declared; `None` on a preset that declares no flavours (R1.6).
    #[must_use]
    pub fn flavor(&self) -> Option<Key> {
        self.stack.flavor().cloned()
    }

    /// The scheme in force: picked, else the reader's or stand-in's, else
    /// the one [`Theme::from_preset`] was given.
    #[must_use]
    pub fn scheme(&self) -> Scheme {
        self.stack.scheme()
    }

    /// Adds an application layer over the others (R2.4), validated over the
    /// stack (S12), and returns its id.
    pub fn add_layer(&mut self, layer: Layer) -> Result<LayerId, LayerError> {
        self.stack.add_layer(layer)
    }

    /// Replaces the application layer `id` in its place;
    /// `Err(UnknownLayer(id))` for an id this theme does not hold, a stale
    /// id being the caller's error (S1).
    pub fn replace_layer(&mut self, id: LayerId, layer: Layer) -> Result<(), LayerError> {
        self.stack.replace_layer(id, layer)
    }

    /// Removes the application layer `id`; `Err(UnknownLayer(id))` for an
    /// id this theme does not hold, a stale id being the caller's error
    /// (S1).
    pub fn remove_layer(&mut self, id: LayerId) -> Result<(), LayerError> {
        self.stack.remove_layer(id)
    }

    /// Sets the accent (S9.2); `None`: the system's again. Refused,
    /// naming them, where it lacks accent inputs the preset declares.
    pub fn set_accent(&mut self, accent: Option<Accent>) -> Result<(), LayerError> {
        self.stack.set_accent(accent)
    }

    /// The conditions the reader or stand-in reports, for what an
    /// application acts on itself; `None` without either.
    #[must_use]
    pub fn system(&self) -> Option<&ReaderState> {
        self.stack.reader_state()
    }

    /// Sets the contrast; `None`: the reader's.
    pub fn set_contrast(&mut self, contrast: Option<Contrast>) -> Result<(), LayerError> {
        self.stack.set_contrast(contrast)
    }

    /// Resolves the theme for a window (S8), which `window` records, with
    /// what changed since its last resolution.
    #[must_use]
    pub fn resolve(&self, window: &mut WindowState, conditions: &WindowConditions) -> Resolution {
        self.stack.resolve(window, conditions)
    }
}

/// The stack of the preset `text` states, with no reader, under `scheme`;
/// `Err(ThemeError::Preset)` where it does not read or fails validation.
fn preset_stack(preset: Option<PresetId>, text: &str, scheme: Scheme) -> Result<Stack, ThemeError> {
    let file = read_preset(text).map_err(ThemeError::Preset)?;
    Stack::new(preset, file.declarations, file.layer, scheme).map_err(ThemeError::Preset)
}

#[cfg(test)]
mod tests;
