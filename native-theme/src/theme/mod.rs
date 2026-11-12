//! S16's `Theme`: the one owner of the stack, the inputs and the
//! application's conditions. Every mutation validates the whole stack and
//! is refused whole on failure, the theme staying as it was (S8, S12).

use super::error::{LayerError, ReaderFailure, ReaderStatus, ThemeError};
use super::inputs::{Accent, Contrast};
use super::key::Key;
use super::layer::{Layer, LayerId};
use super::palette::Palette;
use super::presets::PresetId;
use super::reader::{Reading, SystemReader};
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
    /// The system's reader and its own preset, the one the theme was built
    /// with (S9.5); `None` on a theme no system read built.
    reader: Option<(SystemReader, PresetId)>,
    /// The reader's last read, while `reader` is set.
    status: ReaderStatus,
}

impl Theme {
    /// Reads the system (S9.3, S16): the reader of the running platform and
    /// desktop, over its own preset, `generation` where it names one of the
    /// platform's generations (R1.5), else the one the platform chooses.
    ///
    /// Refused with [`ThemeError::PresetNotForPlatform`] for a `generation`
    /// that is not one of the running platform's, with
    /// [`ThemeError::FeatureDisabled`] on macOS and Windows where the
    /// platform's reader feature is off and
    /// [`ThemeError::PlatformUnsupported`] where the crate has no reader for
    /// the platform; on Linux, where the desktop's reader feature is off,
    /// 0.7.0's scheme and motion chain reads the desktop under that reader's
    /// preset, [`ReaderStatus::Fallback`] naming the reader not compiled. A
    /// failed first read is `Err(ThemeError::Reader)`,
    /// with the preset it chose, for [`Theme::from_preset`]; a bundled
    /// preset failing validation is `Err(ThemeError::Preset)`. A read where
    /// one source stood in for another builds the theme, its
    /// [`Theme::reader_status`] [`ReaderStatus::Fallback`].
    ///
    /// The portal's reads are D-Bus calls, on which this blocks; an
    /// application inside an async executor awaits
    /// [`Theme::from_system_async`].
    ///
    /// On macOS the scroller style and keyboard navigation are AppKit's
    /// main-thread API: called off the main thread, as from an executor's
    /// worker, this does not read them, and [`Theme::system`]'s
    /// `overlay_scrollbars` and `full_keyboard_access` are `false`, with no
    /// [`ReaderStatus::Fallback`] (S16). AppKit runs an application's event
    /// loop on the main thread; one that builds the theme on another thread
    /// and needs the two settings calls [`Theme::reload_reader`] from its
    /// event loop.
    pub fn from_system(generation: Option<PresetId>) -> Result<Theme, ThemeError> {
        pollster::block_on(Theme::from_system_async(generation))
    }

    /// [`Theme::from_system`], awaited. On macOS it reads the scroller
    /// style and keyboard navigation only when polled on the main thread, as
    /// [`Theme::from_system`] says.
    pub async fn from_system_async(generation: Option<PresetId>) -> Result<Theme, ThemeError> {
        let (reader, preset) = super::reader::select(generation).await?;
        let failed = |failure| ThemeError::Reader { preset, failure };
        let reading = reader.read(preset).await.map_err(failed)?;
        let mut theme = Theme::from_preset(preset, reading.state.scheme)?;
        let status = theme
            .take(reading, preset)
            .map_err(|error| failed(ReaderFailure::Invalid(error)))?;
        theme.reader = Some((reader, preset));
        theme.status = status;
        Ok(theme)
    }

    /// The bundled preset `id` with no reader, under `scheme`: S16's
    /// no-reader state. A preset failing validation is
    /// `Err(ThemeError::Preset)`, which a build test rules out (S19.1).
    pub fn from_preset(id: PresetId, scheme: Scheme) -> Result<Theme, ThemeError> {
        let stack = preset_stack(Some(id), id.source(), scheme)?;
        Ok(Theme::over(id, stack))
    }

    /// The bundled preset `id` with `layers` over it, under `scheme`: the
    /// facts gate's and S19.1's build test's route while the preset's
    /// `[pending]` table is not empty (S17, ADD-A128, ADD-A131). The preset
    /// is validated with the layers, added at once as application layers,
    /// so a listed path they leave unstated is the missing-field problem it
    /// is through [`Theme::from_preset`]; a listed input's per-scheme value
    /// is folded into the preset's own input base (ADD-A132). Past the fold
    /// the layers state constant values at disjoint places: one stating an
    /// expression, or a place another states, is refused
    /// (`ProblemKind::SyntheticExpression`, `SyntheticOverlap`). The theme is
    /// a synthetic one while its preset is current: every mutation keeps
    /// the route's waivers (a listed default size waived, a listed presence
    /// drawn), and [`Theme::set_preset`] ends it.
    #[doc(hidden)]
    pub fn from_preset_synthetic(
        id: PresetId,
        scheme: Scheme,
        layers: Vec<Layer>,
    ) -> Result<Theme, ThemeError> {
        let stack = synthetic_stack(Some(id), id.source(), scheme, layers)?;
        Ok(Theme::over(id, stack))
    }

    /// Switches to the bundled preset `id`, a theme change (S16): the
    /// reader's or stand-in's common part applies to any preset, its
    /// `[for_preset.<id>]` part and preset-conditioned rules to their own;
    /// the accent layer is derived again; the flavour set is kept where the
    /// new preset declares it, else the new preset's default is in force.
    /// Refused where the accent set or the stand-in (its common inputs with
    /// its part for the new preset) does not state the new preset's accent
    /// inputs as one group (`Err(AccentInputsMissing)`, S9.2, S9.5), where
    /// the applied palette has no source for it (`Err(PaletteNotForPreset)`,
    /// S14; the application removes the palette first), where a
    /// path its `[pending]` table lists is left that no layer states (S17),
    /// or where the stack fails validation over it.
    /// On a theme [`Theme::from_preset_synthetic`] built, it ends the
    /// synthetic route, whose layers and folded values it removes, the same
    /// preset's included (S17, ADD-A131).
    pub fn set_preset(&mut self, id: PresetId) -> Result<(), LayerError> {
        let file = read_preset(id.source())?;
        self.stack
            .set_preset(Some(id), file.declarations, file.layer, file.pending)?;
        self.preset = id;
        Ok(())
    }

    /// A theme of `stack`, whose preset is `id`, with no system reader.
    fn over(id: PresetId, stack: Stack) -> Theme {
        Theme {
            preset: id,
            stack,
            reader: None,
            status: ReaderStatus::NoReader,
        }
    }

    /// Takes `reading` as the reader layer, its own preset `own`; the
    /// status it gives, or the stack's refusal, the layer beneath kept.
    fn take(&mut self, reading: Reading, own: PresetId) -> Result<ReaderStatus, LayerError> {
        self.stack
            .set_reader(reading.state, reading.layer, Some(own))?;
        Ok(match reading.fallback {
            Some(failure) => ReaderStatus::Fallback(failure),
            None => ReaderStatus::Read,
        })
    }

    /// How the reader layer was last read: [`ReaderStatus::Fallback`] where
    /// a read succeeded with a source standing in for another (S9.1, S9.3),
    /// [`ReaderStatus::Failed`] where the last re-read failed and the layer
    /// before it stays; [`ReaderStatus::StandIn`] with a stand-in, and
    /// [`ReaderStatus::NoReader`] with neither.
    #[must_use]
    pub fn reader_status(&self) -> ReaderStatus {
        if self.reader.is_some() {
            self.status.clone()
        } else if self.stack.reader_state().is_some() {
            ReaderStatus::StandIn
        } else {
            ReaderStatus::NoReader
        }
    }

    /// Reads the system again into the reader layer, in place, with the
    /// reader and own preset [`Theme::from_system`] chose, after clearing
    /// the detection caches it reads through
    /// ([`crate::detect::invalidate_caches`]); a failure keeps the layer
    /// before it, reported as [`ReaderStatus::Failed`] (S8, S16). The
    /// application calls it from its event loop on the watcher's
    /// notification. On a theme no system read built it reads nothing and
    /// returns [`Theme::reader_status`].
    ///
    /// Blocks on the portal's D-Bus calls, as [`Theme::from_system`] does;
    /// [`Theme::reload_reader_async`] awaits them. On macOS it reads the
    /// scroller style and keyboard navigation only when called on the main
    /// thread, as [`Theme::from_system`] says.
    pub fn reload_reader(&mut self) -> ReaderStatus {
        pollster::block_on(self.reload_reader_async())
    }

    /// [`Theme::reload_reader`], awaited. On macOS it reads the scroller
    /// style and keyboard navigation only when polled on the main thread, as
    /// [`Theme::from_system`] says.
    pub async fn reload_reader_async(&mut self) -> ReaderStatus {
        let Some((reader, own)) = self.reader else {
            return self.reader_status();
        };
        super::detect::invalidate_caches();
        self.status = match reader.read(own).await {
            Ok(reading) => self
                .take(reading, own)
                .unwrap_or_else(|error| ReaderStatus::Failed(ReaderFailure::Invalid(error))),
            Err(failure) => ReaderStatus::Failed(failure),
        };
        self.status.clone()
    }

    /// Sets the reader stand-in, or replaces the one set (S8, R2.6): its
    /// state and its layer, validated as a reader's; refused while a reader
    /// exists (`Err(ReaderPresent)`), and where its common inputs with its
    /// part for the current preset do not state the preset's accent inputs
    /// as one group (`Err(AccentInputsMissing)`, S9.2): a stand-in stating
    /// the `accent` alone is refused on a preset that derives others from
    /// it.
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
    /// the one [`Theme::from_preset`] was given. Under a palette (S14),
    /// `Some` of a scheme the palette lacks is refused
    /// (`Err(SchemeNotInPalette)`), while `None` suspends the palette while
    /// the scheme followed is one it lacks.
    pub fn pick(&mut self, scheme: Option<Scheme>) -> Result<(), LayerError> {
        self.stack.pick(scheme)
    }

    /// Applies `palette` (S8, S14), or removes the applied one (`None`; a
    /// no-op without one): an application layer, at the place the first
    /// palette applied took among the application's layers (the last
    /// then), which a palette replacing it keeps. It applies its source for
    /// the current preset, a KDE port on `kde-breeze`, under the palette's
    /// schemes, chosen again on every change of the stack: while it
    /// applies, the reader's or stand-in's colours, inputs (every input it
    /// states, but a KDE port's accent inputs, which stay in force: the
    /// user's accent is derived over the port's scheme inputs, R2.6), KDE's
    /// three scheme lists and `token.kde` are left out. At a scheme the palette lacks it is
    /// suspended, and under a contrast condition it states no colours for
    /// it is skipped; the reader's or stand-in's colours apply then.
    ///
    /// Refused where the palette has no source for the current preset
    /// (`Err(PaletteNotForPreset)`: a KDE port alone, on `adwaita`), where
    /// it lacks the scheme in force (`Err(SchemeNotInPalette)`), and where
    /// the stack fails validation over it; the theme stays as it was.
    pub fn apply_palette(&mut self, palette: Option<Palette>) -> Result<(), LayerError> {
        self.stack.apply_palette(palette)
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

    /// Sets the accent (S9.2); `None`: the system's again. It states the
    /// preset's accent inputs as one group: at every scheme at which it
    /// states any of them, every one's per-scheme value; refused, naming
    /// those it lacks, otherwise. A scheme at which it states none keeps
    /// the accent beneath.
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
/// `Err(ThemeError::Preset)` where it does not read or fails validation, a
/// listed path no layer states among the problems (S17).
fn preset_stack(preset: Option<PresetId>, text: &str, scheme: Scheme) -> Result<Stack, ThemeError> {
    let file = read_preset(text).map_err(ThemeError::Preset)?;
    Stack::new(preset, file.declarations, file.layer, scheme, file.pending)
        .map_err(ThemeError::Preset)
}

/// The synthetic route's stack of the preset `text` states, `layers` over
/// it (S17, ADD-A131); `Err(ThemeError::Preset)` where it does not read or
/// fails validation.
fn synthetic_stack(
    preset: Option<PresetId>,
    text: &str,
    scheme: Scheme,
    layers: Vec<Layer>,
) -> Result<Stack, ThemeError> {
    let file = read_preset(text).map_err(ThemeError::Preset)?;
    Stack::synthetic(
        preset,
        file.declarations,
        file.layer,
        scheme,
        file.pending,
        layers,
    )
    .map_err(ThemeError::Preset)
}

#[cfg(test)]
mod tests;
