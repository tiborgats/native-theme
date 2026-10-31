//! S16's `Theme` over the bundled presets, and S19.1's build test. The
//! mutations' own tests are the stack's (`resolve/mutation_tests.rs`).

#![allow(clippy::unwrap_used, clippy::expect_used)]

use super::{Theme, preset_stack, synthetic_stack};
use crate::next::error::{LayerError, ThemeError};
use crate::next::gate::synthetic_layers;
use crate::next::layer::Layer;
use crate::next::presets::PresetId;
use crate::next::query::RootInputs;
use crate::next::resolve::{WindowConditions, WindowState};
use crate::next::selector::{Direction, InputKind, Scheme};
use crate::next::toml::layer::read_preset;
use crate::next::units::{Ms, Px, Size};

/// S19.1's build test: every bundled preset reads, states its own key as
/// its `id`, and validates, so `Theme::from_preset` never meets its error
/// in a release (S16): through `from_preset` where its `[pending]` table is
/// empty, else through `from_preset_synthetic` with one synthetic layer per
/// listed path, all at once (S17, ADD-A128). `PresetId::all` lists every
/// variant, as the one list in `presets` writes it. One scheme is enough:
/// the stack's validation covers every scheme whichever is in force (S12).
#[test]
fn every_bundled_preset_validates() {
    for &id in PresetId::all() {
        let file = read_preset(id.source()).unwrap_or_else(|e| panic!("{}: {e}", id.key()));
        assert_eq!(file.id, id.key());
        assert_eq!(PresetId::from_key(id.key()), Some(id));
        validates(id, &file.pending)
            .map(drop)
            .unwrap_or_else(|e| panic!("{}: {e}", id.key()));
    }
}

/// The build test's route for a preset whose `[pending]` table is
/// `pending`: the plain one where it is empty, else the synthetic one.
fn validates(id: PresetId, pending: &[(String, String)]) -> Result<Theme, String> {
    if pending.is_empty() {
        Theme::from_preset(id, Scheme::Light).map_err(|e| e.to_string())
    } else {
        let layers = synthetic_layers(pending, false)?;
        Theme::from_preset_synthetic(id, Scheme::Light, layers).map_err(|e| e.to_string())
    }
}

/// The build test's two routes, over the fixture preset with a listed
/// path (no preset is bundled with one): the synthetic layers state it,
/// so the preset validates through them, and the plain route refuses it.
#[test]
fn build_test_takes_both_routes() {
    let text = listed_preset();
    let file = read_preset(&text).unwrap();
    assert!(!file.pending.is_empty());
    let layers = synthetic_layers(&file.pending, false).unwrap();
    assert_eq!(layers.len(), 1);
    synthetic_stack(None, &text, Scheme::Light, layers).unwrap();
    assert!(preset_stack(None, &text, Scheme::Light).is_err());
    let complete = include_str!("../../../tests/fixtures/facts_gate/base.toml");
    assert!(read_preset(complete).unwrap().pending.is_empty());
    preset_stack(None, complete, Scheme::Light).unwrap();
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

/// The facts gate fixtures' base preset (every field stated, every value
/// synthetic), its touch delay listed in `[pending]` as a register id of
/// the fixture's (S17, ADD-A128), and the line stating it left out.
fn listed_preset() -> String {
    let base = include_str!("../../../tests/fixtures/facts_gate/base.toml");
    let line = "\ndelay_ms = \"absent\"\n";
    assert!(base.contains(line));
    format!(
        "[pending]\n\"tokens.behavior.touch.delay_ms\" = \"U1\"\n{}",
        base.replacen(line, "\n", 1)
    )
}

/// A layer stating the listed path, its value synthetic.
fn synthetic_layer(text: &str) -> Layer {
    Layer::from_toml(text).unwrap()
}

/// The reader reads `[pending]` with its register ids (S11) and leaves a
/// listed path out of a block built whole, and refuses an entry whose id
/// is not `U<n>` or whose path the preset states anyway (S17, ADD-A128).
#[test]
fn pending_table_read() {
    let file = read_preset(&listed_preset()).unwrap();
    assert_eq!(
        file.pending,
        [("tokens.behavior.touch.delay_ms".to_owned(), "U1".to_owned())]
    );
    let base = include_str!("../../../tests/fixtures/facts_gate/base.toml");
    let refused = |table: &str| match read_preset(&format!("[pending]\n{table}\n{base}")) {
        Err(LayerError::Invalid(problems)) => {
            problems.iter().map(ToString::to_string).collect::<Vec<_>>()
        }
        other => panic!("{table}: {other:?}"),
    };
    assert_eq!(
        refused("\"tokens.behavior.drag_time_ms\" = \"151\""),
        ["pending.\"tokens.behavior.drag_time_ms\": expected a register id, `U<n>`"]
    );
    assert_eq!(
        refused("\"tokens.behavior.drag_time_ms\" = 151"),
        ["pending.\"tokens.behavior.drag_time_ms\": expected a register id, `U<n>`"]
    );
    assert_eq!(
        refused("\"tokens.behavior.touch.delay_ms\" = \"U1\""),
        [
            "pending.\"tokens.behavior.touch.delay_ms\": states the field \
             tokens.behavior.touch.delay_ms states too"
        ]
    );
}

/// The reader's exemption covers exactly the listed leaves: a map entry
/// built whole may leave out its listed leaf, never another (S17,
/// ADD-A128).
#[test]
fn pending_exemption_is_the_listed_leaves() {
    let base = include_str!("../../../tests/fixtures/facts_gate/base.toml");
    let entry = "rendering_mode = \"template\"\nweight = \"absent\"\n";
    let with_entry = |listed: &str, entry: &str| {
        format!(
            "[pending]\n{listed}\n{}\n[tokens.icons.sizes.keyed.check]\nsize_px = 16\n\
             fill = \"#000000\"\nscale = \"absent\"\naxes = {{}}\nstate_effect = \"absent\"\n\
             symbol_variants = []\nstatus_colors = {{}}\nanimation = \"absent\"\n{entry}",
            base.replace("keyed = {}\n[tokens.focus]", "[tokens.focus]")
        )
    };
    let listed = "\"tokens.icons.sizes.keyed.check.rendering_mode\" = \"U2\"";
    assert!(read_preset(&with_entry(listed, entry)).is_err());
    read_preset(&with_entry(listed, "weight = \"absent\"\n")).unwrap();
    let Err(LayerError::Invalid(problems)) = read_preset(&with_entry(listed, "")) else {
        panic!("an unlisted leaf left out is refused");
    };
    let problems: Vec<String> = problems.iter().map(ToString::to_string).collect();
    assert_eq!(problems, ["tokens.icons.sizes.keyed.check.weight: missing"]);
}

/// Through the plain route a listed path is the missing field it is (S16,
/// S17, ADD-A128): `ThemeError::Preset(LayerError::Invalid)`, its problem
/// S12's own.
#[test]
fn from_preset_refuses_a_listed_path_as_missing() {
    let Err(ThemeError::Preset(LayerError::Invalid(problems))) =
        preset_stack(None, &listed_preset(), Scheme::Light)
    else {
        panic!("a listed path is missing through the plain route");
    };
    assert_eq!(
        problems.iter().map(ToString::to_string).collect::<Vec<_>>(),
        ["tokens.behavior.touch.delay: missing"]
    );
}

/// The synthetic route validates the preset with a layer stating the
/// listed path, which then resolves as that layer states it; without one
/// the listed path is the missing field (S16, S17, ADD-A128).
#[test]
fn from_preset_synthetic_validates_and_resolves() {
    let layer = synthetic_layer("[tokens.behavior.touch]\ndelay_ms = 7\n");
    let stack = synthetic_stack(None, &listed_preset(), Scheme::Light, vec![layer]).unwrap();
    let theme = stack
        .resolve(
            &mut WindowState::default(),
            &WindowConditions::new(InputKind::Mouse),
        )
        .theme;
    let scope = theme.root_scope(&RootInputs::new(
        Size {
            width: Px(800.0),
            height: Px(600.0),
        },
        Size {
            width: Px(1920.0),
            height: Px(1080.0),
        },
        Direction::Ltr,
    ));
    let tokens = theme.tokens(&scope).unwrap();
    assert_eq!(tokens.behavior.touch.delay, Some(Ms(7.0)));
    let Err(ThemeError::Preset(LayerError::Invalid(problems))) =
        synthetic_stack(None, &listed_preset(), Scheme::Light, Vec::new())
    else {
        panic!("a listed path no layer states is missing");
    };
    assert_eq!(
        problems.iter().map(ToString::to_string).collect::<Vec<_>>(),
        ["tokens.behavior.touch.delay_ms: missing"]
    );
}
