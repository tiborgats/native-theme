//! What the window's last frame painted, read the same on either core: one
//! file for both test modules that read painted primitives, the widget
//! tests (`src/widgets/tests.rs`) and the showcase's
//! (`examples/showcase-gpui/tests.rs`), each including it with `#[path]`,
//! so a change to how a frame is read is made once.
//!
//! On gpui-pre the scene's primitives are `painted_quads()` and
//! `painted_underlines()`. On GPUI Fast `painted_quads()` (gpui-fast 0.1.12
//! src/window.rs:2925) reads the scene's own quads only, and since 0.1.10
//! what a rounded overflow clip holds is painted into a clip-group
//! `LayerFrame` instead (src/fast/rounded_clip_scene.rs:1-10, :60-75), so
//! these read `painted_scene()` (src/fast/rounded_clip.rs:232-237) and the
//! scene of every frame with `tile_size == 0` (src/fast/layers/scene.rs:80-93,
//! `LayerContent::scene` :186), clip groups inside clip groups included.
//! Tiled frames, the composited scroll layers, are not followed: every clip
//! group's scene holds a copy of them (rounded_clip_scene.rs, `new_scene`),
//! so their content would be counted more than once; an absence test over
//! a composited scroll layer therefore still sees none of its content.

/// Every quad the window's last frame painted, on either core (module
/// docs). The order is not paint order.
pub(super) fn scene_quads(window: &gpui::Window) -> Vec<gpui::Quad> {
    #[cfg(feature = "gpui-fast")]
    {
        let mut quads = Vec::new();
        for_each_clip_group_scene(window.painted_scene(), &mut |scene| {
            quads.extend(scene.quads.iter().cloned())
        });
        quads
    }
    #[cfg(not(feature = "gpui-fast"))]
    {
        window.painted_quads()
    }
}

/// Every underline the window's last frame painted, on either core, read
/// as [`scene_quads`] reads the quads: a clip group's scene takes every
/// kind of primitive.
pub(super) fn scene_underlines(window: &gpui::Window) -> Vec<gpui::Underline> {
    #[cfg(feature = "gpui-fast")]
    {
        let mut underlines = Vec::new();
        for_each_clip_group_scene(window.painted_scene(), &mut |scene| {
            underlines.extend(scene.underlines.iter().cloned())
        });
        underlines
    }
    #[cfg(not(feature = "gpui-fast"))]
    {
        window.painted_underlines()
    }
}

/// `scene`, then the scene of each of its clip-group frames, depth first.
#[cfg(feature = "gpui-fast")]
fn for_each_clip_group_scene(scene: &gpui::Scene, f: &mut impl FnMut(&gpui::Scene)) {
    f(scene);
    for frame in &scene.layers.frames {
        if frame.tile_size == 0
            && let Some(group) = frame.content.scene()
        {
            for_each_clip_group_scene(group, f);
        }
    }
}
