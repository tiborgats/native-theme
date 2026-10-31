//! S16's `Theme` over the bundled presets, and S19.1's build test. The
//! mutations' own tests are the stack's (`resolve/mutation_tests.rs`).

#![allow(clippy::unwrap_used, clippy::expect_used)]

use super::{Theme, preset_stack};
use crate::next::error::{LayerError, ThemeError};
use crate::next::presets::PresetId;
use crate::next::selector::Scheme;
use crate::next::toml::layer::read_preset;

/// S19.1's build test: every bundled preset reads, states its own key as
/// its `id`, and validates, so `Theme::from_preset` never meets its error
/// in a release (S16). `PresetId::all` lists every variant, as the one
/// list in `presets` writes it. One scheme is enough: the stack's
/// validation covers every scheme whichever is in force (S12).
#[test]
fn every_bundled_preset_validates() {
    for &id in PresetId::all() {
        let file = read_preset(id.source()).unwrap_or_else(|e| panic!("{}: {e}", id.key()));
        assert_eq!(file.id, id.key());
        assert_eq!(PresetId::from_key(id.key()), Some(id));
        Theme::from_preset(id, Scheme::Light)
            .map(drop)
            .unwrap_or_else(|e| panic!("{}: {e}", id.key()));
    }
}

/// A preset that does not read, or fails validation, is
/// `ThemeError::Preset` with the preset's problems (S1, S16).
#[test]
fn invalid_preset_is_a_theme_error() {
    let unread = preset_stack(None, "[preset]\nid = 1\n", Scheme::Light);
    assert!(matches!(
        unread,
        Err(ThemeError::Preset(LayerError::Invalid(_)))
    ));
    let incomplete = preset_stack(
        None,
        "[preset]\nid = \"probe\"\ngeneration = \"probe-1\"\n",
        Scheme::Dark,
    );
    assert!(matches!(
        incomplete,
        Err(ThemeError::Preset(LayerError::Invalid(_)))
    ));
}
