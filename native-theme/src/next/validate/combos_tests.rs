//! S12's checks over combinations, one `combination_<item>` test per item:
//! a stack that triggers exactly that problem, and a near miss that does
//! not; S12's own examples among them. Every number and name here is
//! synthetic.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use super::{Context, LayerKind, Stacked, check};
use crate::next::declare::Declarations;
use crate::next::error::{LayerError, Problem};
use crate::next::layer::Layer;
use crate::next::toml::layer::read_preset;

const PRESET_HEAD: &str = "[preset]\nid = \"probe\"\ngeneration = \"probe-1\"\n";

const NONE: [&str; 0] = [];

/// The declarations a `[declare]` table's body states.
fn declare(body: &str) -> Declarations {
    read_preset(&format!("{PRESET_HEAD}[declare]\n{body}"))
        .unwrap()
        .declarations
}

fn toml(text: &str) -> Layer {
    Layer::from_toml(text).unwrap()
}

/// The problems of `top` added as an application layer over `below`, a
/// preset's layer, under `decl`.
fn over(below: &str, top: &str, decl: &Declarations) -> Vec<String> {
    let below = toml(below);
    let below = [Stacked {
        layer: &below,
        kind: LayerKind::Preset,
    }];
    let cx = Context {
        declarations: decl,
        preset: None,
        kind: LayerKind::Application,
        below: &below,
    };
    match check(&toml(top), &cx) {
        Ok(()) => Vec::new(),
        Err(LayerError::Invalid(problems)) => problems.iter().map(Problem::to_string).collect(),
        Err(other) => panic!("{other}"),
    }
}

/// The problems of `top` alone, an application layer over nothing.
fn alone(top: &str, decl: &Declarations) -> Vec<String> {
    over("", top, decl)
}

fn bare(top: &str) -> Vec<String> {
    alone(top, &Declarations::default())
}

/// An edge's `dash` absent where its `width` and `paint` are drawn (S2.4,
/// S12); absent with its width, it is an edge not drawn.
#[test]
fn combination_edge_dash_absent() {
    let base = "[widgets.button.body.stroke.top]\nwidth_px = 1\npaint = \"#000000\"\n\
                dash = \"solid\"\nsnap = \"none\"";
    let rule = |set: &str| {
        format!(
            "[[rules]]\nwhen = {{ states = [\"hover\"] }}\ntarget = \"button.body\"\nset = {set}"
        )
    };
    assert_eq!(
        over(
            base,
            &rule("{ stroke.top.dash = \"absent\" }"),
            &Declarations::default()
        ),
        ["rules[0].set.stroke.top.dash: may not be absent"]
    );
    assert_eq!(
        over(
            base,
            &rule("{ stroke.top.dash = \"absent\", stroke.top.width_px = \"absent\" }"),
            &Declarations::default()
        ),
        NONE
    );
}

/// S12's own example: one rule stating `rendering_mode = original` with
/// `fill = "absent"` passes, as the fill is absent only where the mode is
/// `Original`; a rule leaving the mode as it was fails.
#[test]
fn combination_icon_fill_absent() {
    let base = "[widgets.button.icon.icon]\nsize_px = 16\nfill = \"#000000\"\n\
                rendering_mode = \"template\"";
    let rule = |set: &str| {
        format!(
            "[[rules]]\nwhen = {{ states = [\"hover\"] }}\ntarget = \"button.icon\"\nset = {set}"
        )
    };
    assert_eq!(
        over(
            base,
            &rule("{ icon = { rendering_mode = \"original\", fill = \"absent\" } }"),
            &Declarations::default()
        ),
        NONE
    );
    assert_eq!(
        over(
            base,
            &rule("{ icon.fill = \"absent\" }"),
            &Declarations::default()
        ),
        ["rules[0].set.icon.fill: may not be absent"]
    );
}

/// A glyph's `fill` absent where its `stroke` is absent too (S2.9, S12).
#[test]
fn combination_glyph_fill_absent() {
    let base = "[widgets.button.menu_indicator.glyph]\nsize_px = 8\nfill = \"#000000\"\n\
                stroke = \"absent\"";
    let rule = "[[rules]]\nwhen = { states = [\"hover\"] }\ntarget = \"button.menu_indicator\"\n\
                set = { glyph.fill = \"absent\" }";
    assert_eq!(
        over(base, rule, &Declarations::default()),
        ["rules[0].set.glyph.fill: may not be absent"]
    );
    let stroked = "[widgets.button.menu_indicator.glyph]\nsize_px = 8\nfill = \"#000000\"\n\
                   stroke = { paint = \"#000000\", width_px = 1, cap = \"round\", join = \"round\" }";
    assert_eq!(over(stroked, rule, &Declarations::default()), NONE);
}

/// S12's own examples: an `Absent` under `scheme:dark` and a `Fields` patch
/// under `scheme:light` pass, the two conditions never holding together; a
/// part's stroke absent under `sizes = ["mini"]` and patched under `sizes =
/// ["regular"]` likewise; under two independent conditions they fail.
#[test]
fn combination_fields_over_absent() {
    let rules = |first: &str, second: &str| {
        format!(
            "[[rules]]\nwhen = {first}\ntarget = \"button.body\"\nset = {{ stroke = \"absent\" }}\n\
             [[rules]]\nwhen = {second}\ntarget = \"button.body\"\n\
             set = {{ stroke.top.width_px = 2 }}"
        )
    };
    let decl = declare("[declare.widget.button]\nsizes = [\"mini\", \"regular\"]");
    assert_eq!(
        alone(
            &rules(
                "{ conditions = [\"scheme:dark\"] }",
                "{ conditions = [\"scheme:light\"] }"
            ),
            &decl
        ),
        NONE
    );
    assert_eq!(
        alone(
            &rules("{ sizes = [\"mini\"] }", "{ sizes = [\"regular\"] }"),
            &decl
        ),
        NONE
    );
    assert_eq!(
        alone(
            &rules(
                "{ conditions = [\"scheme:dark\"] }",
                "{ conditions = [\"reduced_motion\"] }"
            ),
            &decl
        ),
        ["rules[1].set.stroke: patches the fields of a value that can be `absent`"]
    );
    // A rule between them restoring the stroke under the same condition.
    let restored = "[[rules]]\nwhen = { states = [\"hover\"] }\ntarget = \"button.body\"\n\
                    set = { stroke = \"absent\" }\n\
                    [[rules]]\nwhen = { states = [\"hover\"] }\ntarget = \"button.body\"\n\
                    set = { stroke = { replace = { top = { width_px = 1, paint = \"#000000\", \
                    dash = \"solid\", snap = \"none\" }, right = { width_px = 1, paint = \
                    \"#000000\", dash = \"solid\", snap = \"none\" }, bottom = { width_px = 1, \
                    paint = \"#000000\", dash = \"solid\", snap = \"none\" }, left = { width_px \
                    = 1, paint = \"#000000\", dash = \"solid\", snap = \"none\" }, position = \
                    \"inside\", outset_px = \"absent\", fill_clip = \"box\" } } }\n\
                    [[rules]]\nwhen = { states = [\"hover\"] }\ntarget = \"button.body\"\n\
                    set = { stroke.top.width_px = 2 }";
    assert_eq!(bare(restored), NONE);
}

/// `DrawnAs` patches forming a cycle (S6.1, S12): a button's icon drawn as
/// a button draws its own icon as a button again; drawn as a button of a
/// variant under which the icon is drawn, it does not.
#[test]
fn combination_drawn_as_cycle() {
    assert_eq!(
        bare("[widgets.button.icon]\ndrawn_as = { widget = \"button\", variants = [] }"),
        ["widgets.button.icon: drawn as itself through `drawn_as`"]
    );
    let decl = declare("[declare.widget.button]\nvariants = [\"@inner\"]");
    let rule = "[[rules]]\nwhen = { not_variants = [\"@inner\"] }\ntarget = \"button.icon\"\n\
                set = { drawn_as = { widget = \"button\", variants = [\"@inner\"] } }";
    assert_eq!(alone(rule, &decl), NONE);
}

/// `text_scale` read where `token.text.scaling` can be `DynamicType` (S2.2,
/// S9.4, S12): under a condition that sets it so, too.
#[test]
fn combination_text_scale() {
    let read = "[widgets.button.body]\nmin_height_px = \"= mul(24px, text_scale)\"";
    let scaling = |scaling: &str| format!("[tokens.text]\nscaling = \"{scaling}\"");
    assert_eq!(
        over(&scaling("dynamic_type"), read, &Declarations::default()),
        [
            "widgets.button.body.min_height_px: `text_scale` where the text scaling can be `dynamic_type`"
        ]
    );
    assert_eq!(
        over(&scaling("linear"), read, &Declarations::default()),
        NONE
    );
    let conditional = format!(
        "{}\n[[rules]]\nwhen = {{ conditions = [\"accessibility_text_size\"] }}\n\
         target = \"tokens\"\nset = {{ text.scaling = \"dynamic_type\" }}",
        scaling("linear")
    );
    assert_eq!(
        over(&conditional, read, &Declarations::default()),
        [
            "widgets.button.body.min_height_px: `text_scale` where the text scaling can be `dynamic_type`"
        ]
    );
}

/// An operand that can resolve `Absent` or `Auto` (S2.2, S12): in a call it
/// is refused, a plain reference bringing either only where its field does
/// not allow it; `line_height` is the line spacing under `auto`. S2.2's own
/// example: `add(part.max_width, 4px)` over an `auto` maximum.
#[test]
fn combination_absent_and_auto_operands() {
    let decl = Declarations::default();
    let body = |text: &str| format!("[widgets.button.body]\n{text}");
    assert_eq!(
        bare(&body(
            "max_width_px = \"auto\"\nmin_width_px = \"= add(part.max_width, 4px)\""
        )),
        ["widgets.button.body.min_width_px: `part.max_width` can be `auto` here"]
    );
    assert_eq!(
        bare(&body(
            "max_width_px = 40\nmin_width_px = \"= add(part.max_width, 4px)\""
        )),
        NONE
    );
    // A plain reference: `min_width` allows `auto`, `padding` does not.
    assert_eq!(
        bare(&body(
            "max_width_px = \"auto\"\nmin_width_px = \"= part.max_width\""
        )),
        NONE
    );
    assert_eq!(
        bare(&body(
            "max_width_px = \"auto\"\npadding.top_px = \"= part.max_width\""
        )),
        ["widgets.button.body.padding.top_px: may not be auto"]
    );
    // An absent margin.
    assert_eq!(
        bare(&body(
            "margin.top_px = \"absent\"\nmin_width_px = \"= add(part.margin.top, 4px)\""
        )),
        ["widgets.button.body.min_width_px: `part.margin.top` can be `absent` here"]
    );
    assert_eq!(
        bare(&body(
            "margin.top_px = \"absent\"\ntouch_target_px = \"= part.margin.top\""
        )),
        NONE
    );
    assert_eq!(
        bare(&body(
            "margin.top_px = \"absent\"\nmin_width_px = \"= part.margin.top\""
        )),
        ["widgets.button.body.min_width_px: may not be absent"]
    );
    // The absent value under a rule only.
    let rule = "[widgets.button.body]\nmargin.top_px = 2\nmin_width_px = \"= add(part.margin.top, \
                4px)\"\n[[rules]]\nwhen = { states = [\"hover\"] }\ntarget = \"button.body\"\n\
                set = { margin.top_px = \"absent\" }";
    assert_eq!(
        alone(rule, &decl),
        ["widgets.button.body.min_width_px: `part.margin.top` can be `absent` here"]
    );
    // `line_height` under `auto` is the line spacing in a part's call; a
    // text style token's reading it is refused.
    assert_eq!(
        bare(
            "[widgets.button.label]\ntext.line_height = \"auto\"\n\
             padding.top_px = \"= mul(line_height, 0.5)\""
        ),
        NONE
    );
    let styles = declare("text_styles = [\"body\"]");
    let token = "[tokens.text.keyed.body]\nline_height = \"auto\"\n\
                 letter_spacing = \"= mul(line_height, 0.1)\"";
    assert_eq!(
        over(token, token, &styles),
        ["tokens.text.keyed.body.letter_spacing: `line_height` can be `auto` here"]
    );
}

/// An expression reading itself, and a chain of references longer than 32
/// (S2.2, S12).
#[test]
fn combination_cycles_and_chains() {
    assert_eq!(
        bare(
            "[widgets.button.body]\nmin_width_px = \"= part.min_height\"\n\
             min_height_px = \"= part.min_width\""
        ),
        [
            "widgets.button.body.min_width_px: the expression reads itself",
            "widgets.button.body.min_height_px: the expression reads itself",
        ]
    );
    // Under exclusive conditions the two references never meet.
    let exclusive = "[[rules]]\nwhen = { conditions = [\"scheme:dark\"] }\ntarget = \
                     \"button.body\"\nset = { min_width_px = \"= part.min_height\" }\n\
                     [[rules]]\nwhen = { conditions = [\"scheme:light\"] }\ntarget = \
                     \"button.body\"\nset = { min_height_px = \"= part.min_width\" }";
    assert_eq!(bare(exclusive), NONE);
    let keys = |count: usize| {
        let names: Vec<String> = (0..count).map(|i| format!("\"k{i}\"")).collect();
        declare(&format!(
            "params = {{ layout = {{ unit = \"px\", keys = [{}] }} }}",
            names.join(", ")
        ))
    };
    let chain = |count: usize| {
        let mut text = String::from("[tokens.params.layout]\n");
        for i in 0..count {
            text.push_str(&format!("k{i} = \"= token.params.layout.k{}\"\n", i + 1));
        }
        text.push_str(&format!("k{count} = 4\n"));
        text
    };
    // 33 references from `k0` to the number; 32 from `k1`.
    assert_eq!(
        alone(&chain(33), &keys(34)),
        ["tokens.params.layout.k0: a chain of references longer than 32"]
    );
    assert_eq!(alone(&chain(32), &keys(33)), NONE);
}

/// A `widget.<part>` reference to a part that can be absent as a whole
/// (S2.2, S6.3, S12).
#[test]
fn combination_widget_part_not_drawn() {
    let reads = "[widgets.button.label]\npadding.top_px = \"= widget.menu_indicator.min_height\"\n\
                 [widgets.button.menu_indicator]\nmin_height_px = 4";
    assert_eq!(bare(reads), NONE);
    let absent = format!(
        "{reads}\n[[rules]]\nwhen = {{ states = [\"hover\"] }}\ntarget = \"button.menu_indicator\"\n\
         set = {{ absent = true }}"
    );
    assert_eq!(
        bare(&absent),
        [
            "widgets.button.label.padding.top_px: `widget.menu_indicator` can be absent or drawn as another widget"
        ]
    );
}

/// A corner operand (S2.4, S8, S12): `Concentric` and `FollowPart` are the
/// connector's at layout, refused anywhere; `Full` only on the element's
/// own part, here the root. (`button`'s root is its one part listing
/// corners, so a `Full` corner of another part has no fixture yet.)
#[test]
fn combination_corner_operands() {
    let corners = |corner: &str| {
        format!(
            "[widgets.button.body]\ncorners.top_left = {corner}\n\
             focus.width_px = \"= part.corners.top_left\""
        )
    };
    assert_eq!(
        bare(&corners("{ concentric = { minimum_px = 2 } }")),
        [
            "widgets.button.body.focus.width_px: `part.corners.top_left` is a corner computed at layout"
        ]
    );
    assert_eq!(bare(&corners("\"follow_part\"")).len(), 2);
    assert_eq!(
        bare(&corners("{ full = { maximum_px = \"absent\" } }")),
        NONE
    );
    let px = "[widgets.button.body]\ncorners.top_left_px = 4\n\
              focus.width_px = \"= part.corners.top_left\"";
    assert_eq!(bare(px), NONE);
}

/// A focus indicator's `FollowPart` corner whose sides' offsets can
/// differ, in either direction (S2.4, S2.10, S12): the top-left corner's
/// top offset against its left one, and its right one under `rtl`.
#[test]
fn combination_follow_part_offsets() {
    let focus = |offsets: &str| {
        format!("[widgets.button.body.focus]\ncorners.top_left = \"follow_part\"\n{offsets}")
    };
    assert_eq!(
        bare(&focus(
            "offset.top_px = 2\noffset.left_px = 3\noffset.right_px = 2"
        )),
        [
            "widgets.button.body.focus.corners.top_left: `follow_part` on `top_left`, whose sides' offsets can differ"
        ]
    );
    assert_eq!(
        bare(&focus(
            "offset.top_px = 2\noffset.left_px = 2\noffset.right_px = 3"
        )),
        [
            "widgets.button.body.focus.corners.top_left: `follow_part` on `top_left`, whose sides' offsets can differ"
        ]
    );
    assert_eq!(
        bare(&focus(
            "offset.top_px = 2\noffset.left_px = 2\noffset.right_px = 2"
        )),
        NONE
    );
    // Equal at rest, different under a rule.
    let rule = format!(
        "{}\n[[rules]]\nwhen = {{ states = [\"pressed\"] }}\ntarget = \"button.body\"\n\
         set = {{ focus.offset.left_px = 1 }}",
        focus("offset.top_px = 2\noffset.left_px = 2\noffset.right_px = 2")
    );
    assert_eq!(
        bare(&rule),
        [
            "widgets.button.body.focus.corners.top_left: `follow_part` on `top_left`, whose sides' offsets can differ"
        ]
    );
}

/// A `Concentric` corner whose two insets can differ, in either direction
/// (S2.4, S12, ADD-A122): the root's own margins here, its container lying
/// outside the widget.
#[test]
fn combination_concentric_insets() {
    let corner = |margins: &str| {
        format!(
            "[widgets.button.body]\ncorners.top_left = {{ concentric = {{ minimum_px = 2 }} }}\n\
             {margins}"
        )
    };
    assert_eq!(
        bare(&corner(
            "margin.top_px = 2\nmargin.left_px = 3\nmargin.right_px = 2"
        )),
        [
            "widgets.button.body.corners.top_left: `concentric` on `top_left`, whose insets can differ"
        ]
    );
    assert_eq!(
        bare(&corner(
            "margin.top_px = 2\nmargin.left_px = 2\nmargin.right_px = 2"
        )),
        NONE
    );
    // An absent margin is no inset.
    assert_eq!(
        bare(&corner(
            "margin.top_px = \"absent\"\nmargin.left_px = \"absent\"\nmargin.right_px = \"absent\""
        )),
        NONE
    );
}

/// `TextRoles` (S4.4, S12): two slots referencing one keyed style directly,
/// and a slot referencing a slot later in the owner order, are refused; a
/// slot sharing its owner's style references the owner.
#[test]
fn combination_text_roles() {
    let decl = declare("text_styles = [\"body\"]");
    let roles = |text: &str| format!("[tokens.text.roles]\n{text}");
    assert_eq!(
        alone(
            &roles("body = \"= token.text.keyed.body\"\ncaption = \"= token.text.keyed.body\""),
            &decl
        ),
        ["tokens.text.roles.caption: another slot references `text.keyed.body` directly"]
    );
    assert_eq!(
        alone(
            &roles("body = \"= token.text.keyed.body\"\ncaption = \"= token.text.roles.body\""),
            &decl
        ),
        NONE
    );
    assert_eq!(
        alone(
            &roles("caption = \"= token.text.keyed.body\"\nbody = \"= token.text.roles.caption\""),
            &decl
        ),
        ["tokens.text.roles.body: references `caption`, a slot later in the owner order"]
    );
    // Under one condition only.
    let rule = format!(
        "{}\n[[rules]]\nwhen = {{ conditions = [\"bold_text\"] }}\ntarget = \"tokens\"\n\
         set = {{ text.roles.caption = \"= token.text.keyed.body\" }}",
        roles("body = \"= token.text.keyed.body\"\ncaption = \"= token.text.roles.body\"")
    );
    assert_eq!(
        alone(&rule, &decl),
        ["rules[0].set.text.roles.caption: another slot references `text.keyed.body` directly"]
    );
}

/// A map entry's patch creating its entry with a field left unset that may
/// not be `Absent` (S6.1, S12, ADD-A122): a base's over no base stating the
/// key; a rule's where some combination before it leaves the key with no
/// entry.
#[test]
fn combination_entry_created_unset() {
    let decl = declare("text_styles = [\"caption\"]");
    let patch = "[tokens.text.keyed.caption]\nline_height = \"auto\"";
    let problems = alone(patch, &decl);
    assert!(
        problems.contains(
            &"tokens.text.keyed.caption: can create the entry with `font` unset".to_owned()
        ),
        "{problems:?}"
    );
    assert_eq!(over(patch, patch, &decl), NONE);
    // A rule creating the entry under a condition, a second patching it
    // under the same: the second meets the first's entry.
    let rule = |when: &str| {
        format!(
            "[[rules]]\nwhen = {when}\ntarget = \"tokens\"\n\
             set = {{ text.keyed.caption.line_height = \"auto\" }}"
        )
    };
    let two = format!(
        "{}\n{}",
        rule("{ conditions = [\"scheme:dark\"] }"),
        rule("{ conditions = [\"scheme:dark\"] }")
    );
    let problems = alone(&two, &decl);
    assert!(
        problems.iter().all(|p| p.starts_with("rules[0].set.")),
        "{problems:?}"
    );
    assert!(!problems.is_empty());
}

/// A layout field reading `element.width` (S8, S12), but over a fixed box;
/// a glyph drawn to its box, where the element's own part states its
/// minimum as a length.
#[test]
fn combination_element_size_in_layout() {
    let decl = declare(
        "[declare.widget.button]\ndata = { body = [\"width\"], menu_indicator = [\"width\"] }",
    );
    let padding = "[widgets.button.body]\npadding.left_px = \"= mul(element.width, 0.1)\"";
    assert_eq!(
        alone(
            &format!("{padding}\nmin_width_px = 40\nmax_width_px = \"auto\""),
            &decl
        ),
        ["widgets.button.body.padding.left_px: `element.width` in a field laying the element out"]
    );
    assert_eq!(
        alone(
            &format!("{padding}\nmin_width_px = 40\nmax_width_px = 40"),
            &decl
        ),
        NONE
    );
    // Through another field.
    let through = "[widgets.button.body]\nmin_width_px = 40\nmax_width_px = \"auto\"\n\
                   touch_target_px = \"= element.width\"\npadding.left_px = \"= part.touch_target\"";
    let problems = alone(through, &decl);
    assert!(
        problems.contains(
            &"widgets.button.body.padding.left_px: `element.width` in a field laying the element out"
                .to_owned()
        ),
        "{problems:?}"
    );
    // A glyph drawn to its box: the root's minimum stated as a length.
    let glyph = "[widgets.button.menu_indicator]\nglyph.size_px = \"= element.width\"";
    assert_eq!(
        alone(
            &format!("{glyph}\n[widgets.button.body]\nmin_width_px = 24"),
            &decl
        ),
        NONE
    );
    assert_eq!(
        alone(
            &format!("{glyph}\n[widgets.button.body]\nmin_width_px = \"auto\""),
            &decl
        ),
        [
            "widgets.button.menu_indicator.glyph.size_px: `element.width` in a field laying the element out"
        ]
    );
}

/// Under each flavour the declarations in force are the widget's own with
/// that flavour's keys replacing them (S10): a rule is checked against the
/// declarations of each flavour it can apply under, a base against every
/// flavour's (S12).
#[test]
fn combination_per_flavour_declarations() {
    let decl = declare(
        "flavors = [\"widgets\", \"kirigami\"]\n[declare.flavor.kirigami.widget.button]\n\
         variants = [\"@pill\"]\ndata = { body = [\"progress\"] }",
    );
    let rule = |when: &str| {
        format!("[[rules]]\nwhen = {when}\ntarget = \"button.body\"\nset = {{ opacity = 0.5 }}")
    };
    assert_eq!(
        alone(&rule("{ variants = [\"@pill\"] }"), &decl),
        [
            "rules[0].when.variants[0]: `@pill` is not declared in `flavor.widgets.widget.button.variants`"
        ]
    );
    assert_eq!(
        alone(
            &rule("{ conditions = [\"flavor:kirigami\"], variants = [\"@pill\"] }"),
            &decl
        ),
        NONE
    );
    assert_eq!(
        alone(
            &rule("{ not_conditions = [\"flavor:widgets\"], variants = [\"@pill\"] }"),
            &decl
        ),
        NONE
    );
    // A base reads element data under every flavour.
    assert_eq!(
        alone(
            "[widgets.button.body]\nopacity = \"= element.progress\"",
            &decl
        ),
        [
            "widgets.button.body.opacity: `element.progress` is element data the part does not declare"
        ]
    );
    let flagged = "[[rules]]\nwhen = { conditions = [\"flavor:kirigami\"] }\ntarget = \
                   \"button.body\"\nset = { opacity = \"= element.progress\" }";
    assert_eq!(alone(flagged, &decl), NONE);
    // Availability under a flavour.
    let unavailable = declare(
        "flavors = [\"widgets\", \"kirigami\"]\n[declare.flavor.kirigami.widget.button]\n\
         available = false",
    );
    let drawn = |when: &str| {
        format!(
            "[[rules]]\nwhen = {when}\ntarget = \"button.icon\"\n\
             set = {{ drawn_as = {{ widget = \"button\", part = \"body\", variants = [] }} }}"
        )
    };
    assert_eq!(
        alone(&drawn("{ states = [\"hover\"] }"), &unavailable),
        ["rules[0].set.drawn_as.widget: `button` is not available on the preset"]
    );
    assert_eq!(
        alone(
            &drawn("{ conditions = [\"flavor:widgets\"] }"),
            &unavailable
        ),
        NONE
    );
}

/// On `kde-breeze`, a mapping palette leaving an accent-dependent role or
/// an inactive input role to the preset (S9.1, S9.2, S14, S12). No preset
/// is bundled yet, so the check runs here directly over a stack whose
/// preset states the roles (M1b's `kde-breeze` and palettes exercise it
/// through `Theme::apply_palette`).
#[test]
fn combination_mapping_palette() {
    use super::combos::mapping_palette;
    use super::stack::Stack;
    let decl = declare(
        "surface_sets = [\"window\"]\ninputs = [\"accent\", \"inactive.window.foreground_active\"]",
    );
    let roles = "[colors.light.surfaces.window]\nforeground_active = \"#000001\"\n\
                 link = \"#000002\"\ndecoration_focus = \"#000003\"\ndecoration_hover = \"#000004\"\n\
                 background = \"#000005\"";
    let preset = toml(&format!("{roles}\n[tokens]\nkde = \"absent\""));
    let problems = |palette: &str, preset: &Layer| {
        let palette = toml(palette);
        let layers = [
            Stacked {
                layer: preset,
                kind: LayerKind::Preset,
            },
            Stacked {
                layer: &palette,
                kind: LayerKind::Palette,
            },
        ];
        let cx = Context {
            declarations: &decl,
            preset: None,
            kind: LayerKind::Palette,
            below: &[],
        };
        let stack = Stack::new(&layers, cx);
        mapping_palette(&stack, 1)
            .iter()
            .map(Problem::to_string)
            .collect::<Vec<_>>()
    };
    let input = "[inputs.\"inactive.window.foreground_active\"]\nlight = \"#000006\"";
    let partial = "[colors.light.surfaces.window]\nforeground_active = \"#100001\"\n\
                   link = \"#100002\"\ndecoration_focus = \"#100003\"";
    assert_eq!(
        problems(&format!("{partial}\n{input}"), &preset),
        ["colors.light.surfaces.window.decoration_hover: \
             `colors.light.surfaces.window.decoration_hover` left to the preset's value under \
             the palette"]
    );
    let whole = format!("{partial}\ndecoration_hover = \"#100004\"\n{input}");
    assert_eq!(problems(&whole, &preset), NONE);
    // The inactive input role.
    assert_eq!(
        problems(
            &format!("{partial}\ndecoration_hover = \"#100004\""),
            &preset
        ),
        [
            "inputs.inactive.window.foreground_active: `inputs.inactive.window.foreground_active` \
             left to the preset's value under the palette"
        ]
    );
    // Wherever the scheme can tint, every key of every set.
    let tinting = toml(&format!("{roles}\n[tokens.kde]\ntint_factor = 0.5"));
    let left = problems(&whole, &tinting);
    assert!(
        left.iter()
            .any(|p| p.starts_with("colors.light.surfaces.window.background:")),
        "{left:?}"
    );
    // A port states the scheme inputs: no mapping, not checked here.
    let port =
        format!("{partial}\n[inputs.\"kde_scheme.window.background_normal\"]\nlight = \"#000000\"");
    assert_eq!(problems(&port, &preset), NONE);
}

/// `part.corners.<c>` names a field the part lists (S12): a part with no
/// corners has none.
#[test]
fn combination_part_corner_on_a_part_without_corners() {
    assert_eq!(
        bare("[widgets.button.label]\npadding.top_px = \"= part.corners.top_left\""),
        [
            "widgets.button.label.padding.top_px: the expression does not type: `part.corners.top_left` names nothing"
        ]
    );
    assert_eq!(
        bare("[widgets.button.label]\npadding.top_px = \"= part.focus.corners.top_left\""),
        [
            "widgets.button.label.padding.top_px: the expression does not type: `part.focus.corners.top_left` names nothing"
        ]
    );
}

/// `active(…)` reads the role's bases, before any rule (S2.2): a `Colors`
/// rule restating a role from its own active value reads no rule, so it is
/// no cycle (R5.1's inactive-window colours are written so).
#[test]
fn combination_active_reads_the_bases_not_itself() {
    let below = "[colors.light.global]\naccent = \"#3366ff\"\n\
                 [colors.dark.global]\naccent = \"#3366ff\"";
    let rule = |expr: &str| {
        format!(
            "[[rules]]\nwhen = {{ conditions = [\"reduced_motion\"] }}\ntarget = \"colors\"\n\
             set = {{ global.accent = \"{expr}\" }}"
        )
    };
    assert_eq!(
        over(
            below,
            &rule("= mix(active(role.global.accent), #000000, 0.2)"),
            &Declarations::default()
        ),
        NONE
    );
    assert_eq!(
        over(
            below,
            &rule("= mix(role.global.accent, #000000, 0.2)"),
            &Declarations::default()
        ),
        ["rules[0].set.global.accent: the expression reads itself"]
    );
}
