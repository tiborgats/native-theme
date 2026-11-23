//! Tier 2 of customization (spec §7): a [`ThemePatch`], the sparse changes an
//! application lays over a resolved theme (resolution step 3, spec §6).
//!
//! Three sparse mirrors of the part styles, [`PartPatch`], [`TextPatch`] and
//! [`IconPatch`], each an `Option` per property that turns into the rule
//! engine's properties through `to_props`; a sparse mirror of the colour
//! roles, [`ColorPatch`]; and per widget an entry of [`WidgetPatchMap`]: an
//! `Option` per direct metric and, for a ruled widget, the rules appended to
//! its parts (each a [`RuleSelector`] and the patch of every part it states),
//! for a static widget the unconditional patch of each part. Every one of
//! them is `#[non_exhaustive]`, with `new()` and one chained setter per field.

use std::collections::BTreeMap;
use std::sync::Arc;

use super::color::Color;
use super::error::Error;
use super::geometry::{CornerRadius, Corners, Cursor, Edges, ShadowLayer, Stroke, Transform};
use super::layout::{ArrowSide, DialogButtonOrder, TabIndicatorSide};
use super::motion::{DialogEntranceStyle, Timing, Transition};
use super::paint::{BlendMode, GradientStop, Paint, PaintLayer};
use super::query::{
    ButtonShape, ButtonVariant, ControlSize, GroupPosition, InputVariant, ListVariant, Orientation,
    Surface, TabVariant, WidgetState,
};
use super::rules::{ButtonMatcher, IconProp, ListMatcher, PartProp, RuleSelector, TextProp};
use super::style::{
    FocusIndicator, FontStyle, FontWidth, IconRenderingMode, RippleSpec, TextDecoration,
};

/// A struct of sparse fields, its `new()` (the patch that states nothing) and
/// one chained setter per field. A field is `opt` (an `Option` the setter
/// fills), `all` (a nested patch the setter replaces), `list` (rules the
/// setter `rule` appends to) or `selector` (a rule's [`RuleSelector`], whose
/// setter and the setters of its states and axes write it).
macro_rules! sparse {
    (
        $(#[$meta:meta])*
        pub struct $name:ident {
            $( $(#[$fmeta:meta])* $field:ident : $kind:ident $ty:ty ),* $(,)?
        }
    ) => {
        $(#[$meta])*
        #[derive(Clone, Debug, Default, PartialEq)]
        #[non_exhaustive]
        pub struct $name {
            $( $(#[$fmeta])* pub $field: sparse!(@type $kind $ty), )*
        }

        impl $name {
            /// The patch that states nothing.
            #[must_use]
            pub fn new() -> Self {
                Self::default()
            }

            $( sparse!(@setter $kind $field $ty); )*
        }
    };
    (@type opt $ty:ty) => { Option<$ty> };
    (@type all $ty:ty) => { $ty };
    (@type list $ty:ty) => { Vec<$ty> };
    (@type selector $ty:ty) => { $ty };
    (@setter opt $field:ident $ty:ty) => {
        #[doc = concat!("This patch with `", stringify!($field), "` stated.")]
        #[must_use]
        pub fn $field(mut self, value: $ty) -> Self {
            self.$field = Some(value);
            self
        }
    };
    (@setter all $field:ident $ty:ty) => {
        #[doc = concat!("This patch with `", stringify!($field), "` replaced.")]
        #[must_use]
        pub fn $field(mut self, value: $ty) -> Self {
            self.$field = value;
            self
        }
    };
    (@setter list $field:ident $ty:ty) => {
        /// This patch with `rule` appended to its rules.
        #[must_use]
        pub fn rule(mut self, rule: $ty) -> Self {
            self.$field.push(rule);
            self
        }
    };
    (@setter selector $field:ident $ty:ty) => {
        /// This rule with its selector replaced.
        #[must_use]
        pub fn $field(mut self, value: $ty) -> Self {
            self.$field = value;
            self
        }

        /// This rule, applying only where every one of `states` is set.
        #[must_use]
        pub fn states(mut self, states: WidgetState) -> Self {
            self.$field.required_states = states;
            self
        }

        /// This rule, applying only where none of `states` is set.
        #[must_use]
        pub fn not_states(mut self, states: WidgetState) -> Self {
            self.$field.forbidden_states = states;
            self
        }

        /// This rule, limited to `size`.
        #[must_use]
        pub fn size(mut self, size: ControlSize) -> Self {
            self.$field.size = Some(size);
            self
        }

        /// This rule, limited to `surface`.
        #[must_use]
        pub fn surface(mut self, surface: Surface) -> Self {
            self.$field.surface = Some(surface);
            self
        }

        /// This rule, limited to `position`.
        #[must_use]
        pub fn position(mut self, position: GroupPosition) -> Self {
            self.$field.position = Some(position);
            self
        }

        /// This rule, limited to `orientation`.
        #[must_use]
        pub fn orientation(mut self, orientation: Orientation) -> Self {
            self.$field.orientation = Some(orientation);
            self
        }
    };
}

/// The patch type of one widget's entry in [`WidgetPatchMap`]: for a ruled
/// widget, its rule type (a selector and a patch per part) and its entry (the
/// direct metrics and the rules); for a static widget, its entry (the direct
/// metrics and an unconditional patch per part). A metric of `set` is a plain
/// field of the widget, one of `opt` an `Option`; each names the check
/// [`validate`](ThemePatch::validate) runs on it.
macro_rules! widget_patch {
    (
        ruled $widget:ident : $Widget:ident, $Patch:ident, $Rule:ident<$M:ty> {
            parts { $( $part:ident : $PartPatch:ident ),* $(,)? }
            set { $( $set:ident : $set_ty:ty => $set_check:ident ),* $(,)? }
            opt { $( $opt:ident : $opt_ty:ty => $opt_check:ident ),* $(,)? }
        }
    ) => {
        sparse! {
            #[doc = concat!(
                "A rule appended to the parts of `", stringify!($widget),
                "`: where its selector matches, each part's patch applies, after the preset's rules."
            )]
            pub struct $Rule {
                /// Where the rule applies.
                selector: selector RuleSelector<$M>,
                $(
                    #[doc = concat!("The properties of `", stringify!($part), "`.")]
                    $part: all $PartPatch,
                )*
            }
        }

        sparse! {
            #[doc = concat!(
                "The patch of `", stringify!($widget),
                "`: its direct metrics and the rules appended to its parts."
            )]
            pub struct $Patch {
                $(
                    #[doc = concat!("`", stringify!($set), "`, where stated.")]
                    $set: opt $set_ty,
                )*
                $(
                    #[doc = concat!("`", stringify!($opt), "`, where stated.")]
                    $opt: opt $opt_ty,
                )*
                /// The rules, appended in order.
                rules: list $Rule,
            }
        }

        impl $Patch {
            /// The first invalid value, its path under `path`.
            fn check(&self, path: &str) -> Result<(), Error> {
                $(
                    if let Some(value) = &self.$set {
                        check::$set_check(&format!("{path}.{}", stringify!($set)), value)?;
                    }
                )*
                $(
                    if let Some(value) = &self.$opt {
                        check::$opt_check(&format!("{path}.{}", stringify!($opt)), value)?;
                    }
                )*
                for (index, rule) in self.rules.iter().enumerate() {
                    $( rule.$part.check(&format!("{path}.rules[{index}].{}", stringify!($part)))?; )*
                }
                Ok(())
            }
        }
    };
    (
        static $widget:ident : $Widget:ident, $Patch:ident {
            parts { $( $part:ident : $PartPatch:ident ),* $(,)? }
            set { $( $set:ident : $set_ty:ty => $set_check:ident ),* $(,)? }
            opt { $( $opt:ident : $opt_ty:ty => $opt_check:ident ),* $(,)? }
        }
    ) => {
        sparse! {
            #[doc = concat!(
                "The patch of `", stringify!($widget),
                "`: its direct metrics and the properties of each part, applied unconditionally."
            )]
            pub struct $Patch {
                $(
                    #[doc = concat!("`", stringify!($set), "`, where stated.")]
                    $set: opt $set_ty,
                )*
                $(
                    #[doc = concat!("`", stringify!($opt), "`, where stated.")]
                    $opt: opt $opt_ty,
                )*
                $(
                    #[doc = concat!("The properties of `", stringify!($part), "`.")]
                    $part: all $PartPatch,
                )*
            }
        }

        impl $Patch {
            /// The first invalid value, its path under `path`.
            fn check(&self, path: &str) -> Result<(), Error> {
                $(
                    if let Some(value) = &self.$set {
                        check::$set_check(&format!("{path}.{}", stringify!($set)), value)?;
                    }
                )*
                $(
                    if let Some(value) = &self.$opt {
                        check::$opt_check(&format!("{path}.{}", stringify!($opt)), value)?;
                    }
                )*
                $( self.$part.check(&format!("{path}.{}", stringify!($part)))?; )*
                Ok(())
            }
        }
    };
}

// ---------------------------------------------------------------------------
// The part, text and icon patches
// ---------------------------------------------------------------------------

sparse! {
    /// A sparse [`PartStyle`](super::style::PartStyle): each property it
    /// states becomes a [`PartProp`].
    pub struct PartPatch {
        /// `fill`.
        fill: opt Paint,
        /// `stroke`.
        stroke: opt Stroke,
        /// `corners`.
        corners: opt Corners,
        /// `shadows`.
        shadows: opt Option<Arc<[ShadowLayer]>>,
        /// `opacity`.
        opacity: opt f32,
        /// `transform`.
        transform: opt Transform,
        /// `padding`, all four sides.
        padding: opt Edges,
        /// `margin`, all four sides.
        margin: opt Edges,
        /// `min_size`, per dimension: a `Some` replaces, a `None` keeps.
        min_size: opt [Option<f32>; 2],
        /// `max_size`, per dimension: a `Some` replaces, a `None` keeps.
        max_size: opt [Option<f32>; 2],
        /// `focus`.
        focus: opt Option<Arc<FocusIndicator>>,
        /// `ripple`.
        ripple: opt Option<Arc<RippleSpec>>,
        /// `transition`.
        transition: opt Option<Transition>,
        /// `cursor`.
        cursor: opt Cursor,
    }
}

impl PartPatch {
    /// The properties this patch states, in field order.
    pub fn to_props(&self) -> impl Iterator<Item = PartProp> {
        [
            self.fill.clone().map(PartProp::Fill),
            self.stroke.clone().map(PartProp::Stroke),
            self.corners.map(PartProp::Corners),
            self.shadows.clone().map(PartProp::Shadows),
            self.opacity.map(PartProp::Opacity),
            self.transform.map(PartProp::Transform),
            self.padding.map(PartProp::Padding),
            self.margin.map(PartProp::Margin),
            self.min_size.map(PartProp::MinSize),
            self.max_size.map(PartProp::MaxSize),
            self.focus.clone().map(PartProp::Focus),
            self.ripple.clone().map(PartProp::Ripple),
            self.transition.map(PartProp::Transition),
            self.cursor.map(PartProp::Cursor),
        ]
        .into_iter()
        .flatten()
    }

    /// The first invalid value, its path under `path`.
    fn check(&self, path: &str) -> Result<(), Error> {
        let at = |field: &str| format!("{path}.{field}");
        if let Some(fill) = &self.fill {
            check::paint(&at("fill"), fill)?;
        }
        if let Some(stroke) = &self.stroke {
            check::stroke(&at("stroke"), stroke)?;
        }
        if let Some(corners) = &self.corners {
            check::corners(&at("corners"), corners)?;
        }
        if let Some(Some(shadows)) = &self.shadows {
            for (index, layer) in shadows.iter().enumerate() {
                check::shadow(&format!("{path}.shadows[{index}]"), layer)?;
            }
        }
        if let Some(opacity) = &self.opacity {
            check::unit(&at("opacity"), opacity)?;
        }
        if let Some(transform) = &self.transform {
            check::transform(&at("transform"), transform)?;
        }
        if let Some(padding) = &self.padding {
            check::edges(&at("padding"), padding, check::size)?;
        }
        if let Some(margin) = &self.margin {
            check::edges(&at("margin"), margin, check::finite)?;
        }
        if let Some(size) = &self.min_size {
            check::dimensions(&at("min_size"), size)?;
        }
        if let Some(size) = &self.max_size {
            check::dimensions(&at("max_size"), size)?;
        }
        if let Some(Some(focus)) = &self.focus {
            check::focus(&at("focus"), focus)?;
        }
        if let Some(Some(ripple)) = &self.ripple {
            check::color(&at("ripple.color"), &ripple.color)?;
        }
        if let Some(Some(transition)) = &self.transition {
            check::transition(&at("transition"), transition)?;
        }
        Ok(())
    }
}

sparse! {
    /// A sparse [`TextStyle`](super::style::TextStyle): each property it
    /// states becomes a [`TextProp`].
    pub struct TextPatch {
        /// `family`.
        family: opt Arc<str>,
        /// `color`.
        color: opt Color,
        /// `size_px`; it also clears `defined_size`.
        size_px: opt f32,
        /// `weight`.
        weight: opt u16,
        /// `style`.
        style: opt FontStyle,
        /// `width`.
        width: opt FontWidth,
        /// `line_height`.
        line_height: opt f32,
        /// `letter_spacing`.
        letter_spacing: opt f32,
        /// `optical_size`.
        optical_size: opt Option<f32>,
        /// `decoration`.
        decoration: opt TextDecoration,
        /// `blend_mode`.
        blend_mode: opt BlendMode,
    }
}

impl TextPatch {
    /// The properties this patch states, in field order.
    pub fn to_props(&self) -> impl Iterator<Item = TextProp> {
        [
            self.family.clone().map(TextProp::Family),
            self.color.map(TextProp::Color),
            self.size_px.map(TextProp::SizePx),
            self.weight.map(TextProp::Weight),
            self.style.map(TextProp::Style),
            self.width.map(TextProp::Width),
            self.line_height.map(TextProp::LineHeight),
            self.letter_spacing.map(TextProp::LetterSpacing),
            self.optical_size.map(TextProp::OpticalSize),
            self.decoration.map(TextProp::Decoration),
            self.blend_mode.map(TextProp::BlendMode),
        ]
        .into_iter()
        .flatten()
    }

    /// The first invalid value, its path under `path`.
    fn check(&self, path: &str) -> Result<(), Error> {
        let at = |field: &str| format!("{path}.{field}");
        if let Some(color) = &self.color {
            check::color(&at("color"), color)?;
        }
        if let Some(size) = &self.size_px {
            check::size(&at("size_px"), size)?;
        }
        if let Some(height) = &self.line_height {
            check::size(&at("line_height"), height)?;
        }
        if let Some(spacing) = &self.letter_spacing {
            check::finite(&at("letter_spacing"), spacing)?;
        }
        if let Some(Some(size)) = &self.optical_size {
            check::size(&at("optical_size"), size)?;
        }
        if let Some(decoration) = &self.decoration {
            if let Some(color) = &decoration.color {
                check::color(&at("decoration.color"), color)?;
            }
            if let Some(thickness) = &decoration.thickness {
                check::size(&at("decoration.thickness"), thickness)?;
            }
        }
        Ok(())
    }
}

sparse! {
    /// A sparse [`IconStyle`](super::style::IconStyle): each property it
    /// states becomes an [`IconProp`].
    pub struct IconPatch {
        /// `size`.
        size: opt f32,
        /// `color`.
        color: opt Color,
        /// `opacity`.
        opacity: opt f32,
        /// `weight`.
        weight: opt u16,
        /// `rendering_mode`.
        rendering_mode: opt IconRenderingMode,
        /// `stroke_width`.
        stroke_width: opt Option<f32>,
        /// `blend_mode`.
        blend_mode: opt BlendMode,
    }
}

impl IconPatch {
    /// The properties this patch states, in field order.
    pub fn to_props(&self) -> impl Iterator<Item = IconProp> {
        [
            self.size.map(IconProp::Size),
            self.color.map(IconProp::Color),
            self.opacity.map(IconProp::Opacity),
            self.weight.map(IconProp::Weight),
            self.rendering_mode.map(IconProp::RenderingMode),
            self.stroke_width.map(IconProp::StrokeWidth),
            self.blend_mode.map(IconProp::BlendMode),
        ]
        .into_iter()
        .flatten()
    }

    /// The first invalid value, its path under `path`.
    fn check(&self, path: &str) -> Result<(), Error> {
        let at = |field: &str| format!("{path}.{field}");
        if let Some(size) = &self.size {
            check::size(&at("size"), size)?;
        }
        if let Some(color) = &self.color {
            check::color(&at("color"), color)?;
        }
        if let Some(opacity) = &self.opacity {
            check::unit(&at("opacity"), opacity)?;
        }
        if let Some(Some(width)) = &self.stroke_width {
            check::size(&at("stroke_width"), width)?;
        }
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// The colour roles' patch
// ---------------------------------------------------------------------------

sparse! {
    /// A sparse [`SurfaceColors`](super::roles::SurfaceColors).
    pub struct SurfacePatch {
        /// `background`.
        background: opt Color,
        /// `foreground`.
        foreground: opt Color,
        /// `foreground_muted`.
        foreground_muted: opt Color,
        /// `border`.
        border: opt Color,
        /// `divider`.
        divider: opt Color,
    }
}

sparse! {
    /// A sparse [`SelectionColors`](super::roles::SelectionColors).
    pub struct SelectionPatch {
        /// `background`.
        background: opt Color,
        /// `foreground`.
        foreground: opt Color,
        /// `unfocused_background`.
        unfocused_background: opt Color,
        /// `unfocused_foreground`.
        unfocused_foreground: opt Color,
    }
}

sparse! {
    /// A sparse [`AccentColors`](super::roles::AccentColors).
    pub struct AccentPatch {
        /// `fill`.
        fill: opt Color,
        /// `foreground`.
        foreground: opt Color,
        /// `standalone`.
        standalone: opt Color,
    }
}

sparse! {
    /// A sparse [`StatusColors`](super::roles::StatusColors); a status colour
    /// it changes takes the origin
    /// [`Custom`](super::roles::RoleOrigin::Custom).
    pub struct StatusPatch {
        /// `fill`.
        fill: opt Color,
        /// `foreground`.
        foreground: opt Color,
        /// `standalone`.
        standalone: opt Color,
    }
}

sparse! {
    /// A sparse [`ColorRoles`](super::roles::ColorRoles): each role it states
    /// replaces the resolved one. Only the role changes: widget parts already
    /// derived from it keep their colours (a [`Palette`](super::Palette)
    /// recolours coherently).
    pub struct ColorPatch {
        /// `window`.
        window: all SurfacePatch,
        /// `view`.
        view: all SurfacePatch,
        /// `header`.
        header: all SurfacePatch,
        /// `sidebar`.
        sidebar: all SurfacePatch,
        /// `card`.
        card: all SurfacePatch,
        /// `dialog`.
        dialog: all SurfacePatch,
        /// `popover`.
        popover: all SurfacePatch,
        /// `tooltip`.
        tooltip: all SurfacePatch,
        /// `osd`.
        osd: all SurfacePatch,
        /// `selection`.
        selection: all SelectionPatch,
        /// `text_selection`.
        text_selection: all SelectionPatch,
        /// `find_highlight`.
        find_highlight: opt Color,
        /// `scrim`.
        scrim: opt Color,
        /// `accent`.
        accent: all AccentPatch,
        /// `focus_ring`.
        focus_ring: opt Color,
        /// `caret`.
        caret: opt Color,
        /// `link`.
        link: opt Color,
        /// `link_visited`.
        link_visited: opt Color,
        /// `disabled_text`.
        disabled_text: opt Color,
        /// `shadow`.
        shadow: opt Color,
        /// `disabled_opacity`.
        disabled_opacity: opt f32,
        /// `error`.
        error: all StatusPatch,
        /// `warning`.
        warning: all StatusPatch,
        /// `success`.
        success: all StatusPatch,
        /// `info`.
        info: all StatusPatch,
    }
}

impl SurfacePatch {
    /// The first invalid value, its path under `path`.
    fn check(&self, path: &str) -> Result<(), Error> {
        check::colors(
            path,
            [
                ("background", &self.background),
                ("foreground", &self.foreground),
                ("foreground_muted", &self.foreground_muted),
                ("border", &self.border),
                ("divider", &self.divider),
            ],
        )
    }
}

impl SelectionPatch {
    /// The first invalid value, its path under `path`.
    fn check(&self, path: &str) -> Result<(), Error> {
        check::colors(
            path,
            [
                ("background", &self.background),
                ("foreground", &self.foreground),
                ("unfocused_background", &self.unfocused_background),
                ("unfocused_foreground", &self.unfocused_foreground),
            ],
        )
    }
}

impl AccentPatch {
    /// The first invalid value, its path under `path`.
    fn check(&self, path: &str) -> Result<(), Error> {
        check::colors(
            path,
            [
                ("fill", &self.fill),
                ("foreground", &self.foreground),
                ("standalone", &self.standalone),
            ],
        )
    }
}

impl StatusPatch {
    /// The first invalid value, its path under `path`.
    fn check(&self, path: &str) -> Result<(), Error> {
        check::colors(
            path,
            [
                ("fill", &self.fill),
                ("foreground", &self.foreground),
                ("standalone", &self.standalone),
            ],
        )
    }
}

impl ColorPatch {
    /// The first invalid value, its path under `path`.
    fn check(&self, path: &str) -> Result<(), Error> {
        let at = |field: &str| format!("{path}.{field}");
        for (field, surface) in [
            ("window", &self.window),
            ("view", &self.view),
            ("header", &self.header),
            ("sidebar", &self.sidebar),
            ("card", &self.card),
            ("dialog", &self.dialog),
            ("popover", &self.popover),
            ("tooltip", &self.tooltip),
            ("osd", &self.osd),
        ] {
            surface.check(&at(field))?;
        }
        self.selection.check(&at("selection"))?;
        self.text_selection.check(&at("text_selection"))?;
        self.accent.check(&at("accent"))?;
        check::colors(
            path,
            [
                ("find_highlight", &self.find_highlight),
                ("scrim", &self.scrim),
                ("focus_ring", &self.focus_ring),
                ("caret", &self.caret),
                ("link", &self.link),
                ("link_visited", &self.link_visited),
                ("disabled_text", &self.disabled_text),
                ("shadow", &self.shadow),
            ],
        )?;
        if let Some(opacity) = &self.disabled_opacity {
            check::unit(&at("disabled_opacity"), opacity)?;
        }
        for (field, status) in [
            ("error", &self.error),
            ("warning", &self.warning),
            ("success", &self.success),
            ("info", &self.info),
        ] {
            status.check(&at(field))?;
        }
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// The widgets' patches
// ---------------------------------------------------------------------------

widget_patch! {
    ruled button: ButtonWidget, ButtonPatch, ButtonRule<ButtonMatcher> {
        parts { body: PartPatch, label: TextPatch, icon: IconPatch }
        set { icon_text_gap: f32 => size }
        opt { disabled_opacity: f32 => unit }
    }
}

widget_patch! {
    ruled input: InputWidget, InputPatch, InputRule<Option<InputVariant>> {
        parts { field: PartPatch, text: TextPatch, placeholder: TextPatch, helper_text: TextPatch }
        set {}
        opt { disabled_opacity: f32 => unit }
    }
}

widget_patch! {
    ruled checkbox: CheckboxWidget, CheckboxPatch, CheckboxRule<()> {
        parts { indicator: PartPatch, checkmark: IconPatch, label: TextPatch }
        set { label_gap: f32 => size }
        opt { disabled_opacity: f32 => unit }
    }
}

widget_patch! {
    ruled radio: RadioWidget, RadioPatch, RadioRule<()> {
        parts { indicator: PartPatch, dot: PartPatch, label: TextPatch }
        set { label_gap: f32 => size }
        opt { disabled_opacity: f32 => unit }
    }
}

widget_patch! {
    ruled switch: SwitchWidget, SwitchPatch, SwitchRule<()> {
        parts { track: PartPatch, thumb: PartPatch, label: TextPatch }
        set {}
        opt {
            label_gap: f32 => size,
            show_state_glyphs: bool => none,
            disabled_opacity: f32 => unit,
        }
    }
}

widget_patch! {
    ruled slider: SliderWidget, SliderPatch, SliderRule<()> {
        parts { track: PartPatch, active_track: PartPatch, thumb: PartPatch, tick: PartPatch }
        set { tick_length: f32 => size }
        opt { disabled_opacity: f32 => unit }
    }
}

widget_patch! {
    ruled progress_bar: ProgressBarWidget, ProgressBarPatch, ProgressBarRule<()> {
        parts { track: PartPatch, fill: PartPatch }
        set {}
        opt { indeterminate_period_ms: u32 => none }
    }
}

widget_patch! {
    ruled scrollbar: ScrollbarWidget, ScrollbarPatch, ScrollbarRule<()> {
        parts { track: PartPatch, thumb: PartPatch }
        set { min_thumb_length: f32 => size, overlay_mode: bool => none }
        opt {}
    }
}

widget_patch! {
    ruled tab: TabWidget, TabPatch, TabRule<Option<TabVariant>> {
        parts {
            bar: PartPatch,
            tab: PartPatch,
            label: TextPatch,
            indicator: PartPatch,
            close_button: PartPatch,
            close_icon: IconPatch,
        }
        set {}
        opt {
            item_gap: f32 => size,
            indicator_side: TabIndicatorSide => none,
            active_outward_corner_radius: f32 => size,
        }
    }
}

widget_patch! {
    ruled menu: MenuWidget, MenuPatch, MenuRule<()> {
        parts {
            panel: PartPatch,
            item: PartPatch,
            item_label: TextPatch,
            shortcut: TextPatch,
            icon: IconPatch,
            submenu_arrow: IconPatch,
            separator: PartPatch,
        }
        set { icon_text_gap: f32 => size }
        opt {}
    }
}

widget_patch! {
    ruled menu_bar: MenuBarWidget, MenuBarPatch, MenuBarRule<()> {
        parts { bar: PartPatch, item: PartPatch, item_label: TextPatch }
        set {}
        opt { item_gap: f32 => size }
    }
}

widget_patch! {
    ruled list: ListWidget, ListPatch, ListRule<ListMatcher> {
        parts {
            container: PartPatch,
            row: PartPatch,
            highlight: PartPatch,
            item_label: TextPatch,
            header: PartPatch,
            header_label: TextPatch,
            header_separator: PartPatch,
        }
        set { grid_color: Color => color }
        opt {}
    }
}

widget_patch! {
    ruled sidebar: SidebarWidget, SidebarPatch, SidebarRule<()> {
        parts {
            container: PartPatch,
            item: PartPatch,
            item_label: TextPatch,
            item_icon: IconPatch,
            section_header: TextPatch,
        }
        set {}
        opt { item_gap: f32 => size }
    }
}

widget_patch! {
    ruled toolbar: ToolbarWidget, ToolbarPatch, ToolbarRule<()> {
        parts {
            bar: PartPatch,
            button: PartPatch,
            button_label: TextPatch,
            button_icon: IconPatch,
            separator: PartPatch,
        }
        set {}
        opt { item_gap: f32 => size }
    }
}

widget_patch! {
    ruled combo_box: ComboBoxWidget, ComboBoxPatch, ComboBoxRule<()> {
        parts { field: PartPatch, label: TextPatch, chevron: IconPatch }
        set {}
        opt { arrow_area_width: f32 => size, disabled_opacity: f32 => unit }
    }
}

widget_patch! {
    ruled segmented_control: SegmentedControlWidget, SegmentedControlPatch, SegmentedControlRule<()> {
        parts { track: PartPatch, segment: PartPatch, label: TextPatch, separator: PartPatch }
        set { separator_width: f32 => size }
        opt { disabled_opacity: f32 => unit }
    }
}

widget_patch! {
    ruled expander: ExpanderWidget, ExpanderPatch, ExpanderRule<()> {
        parts { container: PartPatch, header: PartPatch, header_label: TextPatch, arrow: IconPatch }
        set {}
        opt {
            arrow_side: ArrowSide => none,
            arrow_gap: f32 => size,
            content_indent: f32 => size,
            frame_enabled: bool => none,
        }
    }
}

widget_patch! {
    ruled card: CardWidget, CardPatch, CardRule<()> {
        parts { container: PartPatch }
        set {}
        opt {}
    }
}

widget_patch! {
    ruled splitter: SplitterWidget, SplitterPatch, SplitterRule<()> {
        parts { handle: PartPatch }
        set { divider_width: f32 => size }
        opt {}
    }
}

widget_patch! {
    ruled link: LinkWidget, LinkPatch, LinkRule<()> {
        parts { container: PartPatch, label: TextPatch }
        set {}
        opt {}
    }
}

widget_patch! {
    static status_bar: StatusBarWidget, StatusBarPatch {
        parts { bar: PartPatch, label: TextPatch, separator: PartPatch }
        set {}
        opt {}
    }
}

widget_patch! {
    static popover: PopoverWidget, PopoverPatch {
        parts { panel: PartPatch, arrow: PartPatch, text: TextPatch }
        set {}
        opt {}
    }
}

widget_patch! {
    static dialog: DialogWidget, DialogPatch {
        parts { panel: PartPatch, title: TextPatch, body: TextPatch, icon: IconPatch }
        set { button_gap: f32 => size }
        opt {
            button_order: DialogButtonOrder => none,
            entrance: DialogEntranceStyle => none,
        }
    }
}

widget_patch! {
    static tooltip: TooltipWidget, TooltipPatch {
        parts { panel: PartPatch, text: TextPatch }
        set {}
        opt {}
    }
}

widget_patch! {
    static spinner: SpinnerWidget, SpinnerPatch {
        parts {}
        set {
            color: Color => color,
            diameter: f32 => size,
            min_diameter: f32 => size,
            stroke_width: f32 => size,
        }
        opt { period_ms: u32 => none }
    }
}

widget_patch! {
    static separator: SeparatorWidget, SeparatorPatch {
        parts {}
        set { color: Color => color, thickness: f32 => size }
        opt { inset: f32 => size, margin: f32 => finite }
    }
}

impl ButtonRule {
    /// This rule, limited to the buttons of `variant`.
    #[must_use]
    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.selector.matcher.variant = Some(variant);
        self
    }

    /// This rule, limited to the buttons of `shape`.
    #[must_use]
    pub fn shape(mut self, shape: ButtonShape) -> Self {
        self.selector.matcher.shape = Some(shape);
        self
    }
}

impl InputRule {
    /// This rule, limited to the fields of `variant`.
    #[must_use]
    pub fn variant(mut self, variant: InputVariant) -> Self {
        self.selector.matcher = Some(variant);
        self
    }
}

impl TabRule {
    /// This rule, limited to the tab bars of `variant`.
    #[must_use]
    pub fn variant(mut self, variant: TabVariant) -> Self {
        self.selector.matcher = Some(variant);
        self
    }
}

impl ListRule {
    /// This rule, limited to the lists of `variant`.
    #[must_use]
    pub fn variant(mut self, variant: ListVariant) -> Self {
        self.selector.matcher.variant = Some(variant);
        self
    }

    /// This rule, limited to the alternate rows (`true`) or the others.
    #[must_use]
    pub fn alternate(mut self, alternate: bool) -> Self {
        self.selector.matcher.alternate = Some(alternate);
        self
    }
}

sparse! {
    /// The widgets' patches, one entry per widget of a
    /// [`ResolvedTheme`](super::ResolvedTheme).
    pub struct WidgetPatchMap {
        /// `button`.
        button: all ButtonPatch,
        /// `input`.
        input: all InputPatch,
        /// `text_area`.
        text_area: all InputPatch,
        /// `checkbox`.
        checkbox: all CheckboxPatch,
        /// `radio`.
        radio: all RadioPatch,
        /// `switch`.
        switch: all SwitchPatch,
        /// `slider`.
        slider: all SliderPatch,
        /// `progress_bar`.
        progress_bar: all ProgressBarPatch,
        /// `scrollbar`.
        scrollbar: all ScrollbarPatch,
        /// `tab`.
        tab: all TabPatch,
        /// `menu`.
        menu: all MenuPatch,
        /// `menu_bar`.
        menu_bar: all MenuBarPatch,
        /// `list`.
        list: all ListPatch,
        /// `sidebar`.
        sidebar: all SidebarPatch,
        /// `toolbar`.
        toolbar: all ToolbarPatch,
        /// `status_bar`.
        status_bar: all StatusBarPatch,
        /// `combo_box`.
        combo_box: all ComboBoxPatch,
        /// `segmented_control`.
        segmented_control: all SegmentedControlPatch,
        /// `expander`.
        expander: all ExpanderPatch,
        /// `card`.
        card: all CardPatch,
        /// `popover`.
        popover: all PopoverPatch,
        /// `dialog`.
        dialog: all DialogPatch,
        /// `tooltip`.
        tooltip: all TooltipPatch,
        /// `spinner`.
        spinner: all SpinnerPatch,
        /// `splitter`.
        splitter: all SplitterPatch,
        /// `separator`.
        separator: all SeparatorPatch,
        /// `link`.
        link: all LinkPatch,
    }
}

/// Runs the macro `$callback` on the field names of every entry of
/// [`WidgetPatchMap`].
macro_rules! for_each_widget {
    ($callback:ident) => {
        $callback!(
            button input text_area checkbox radio switch slider progress_bar scrollbar tab menu
            menu_bar list sidebar toolbar status_bar combo_box segmented_control expander card
            popover dialog tooltip spinner splitter separator link
        )
    };
}

impl WidgetPatchMap {
    /// The first invalid value, its path under `path`.
    fn check(&self, path: &str) -> Result<(), Error> {
        macro_rules! check_each {
            ($($widget:ident)*) => {
                $( self.$widget.check(&format!("{path}.{}", stringify!($widget)))?; )*
            };
        }
        for_each_widget!(check_each);
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// ThemePatch
// ---------------------------------------------------------------------------

/// Sparse changes laid over a resolved theme (spec §7, Tier 2), applied by
/// [`Theme::with_patch`](super::Theme::with_patch) after the preset's
/// builder and before the transforms, or by
/// [`ResolvedTheme::apply_patch`](super::ResolvedTheme::apply_patch) inside
/// a transform. Within a patch the fields apply in the order they are
/// declared here (spec §6, resolution step 3), so a patch's scales do not
/// touch the rules it appends.
///
/// ```text
/// ThemePatch::new().button(ButtonRule::new().variant(Primary).states(HOVER).body(PartPatch::new().fill(..)))
/// ```
#[derive(Clone, Debug, Default, PartialEq)]
#[non_exhaustive]
pub struct ThemePatch {
    /// The factor of every concrete corner radius.
    pub corner_scale: Option<f32>,
    /// The factor of the spacing: the global gaps, every part's padding and
    /// margin, and the widgets' `*_gap` metrics.
    pub spacing_scale: Option<f32>,
    /// The UI font family, replacing the resolved one wherever it stands.
    pub font_family: Option<Arc<str>>,
    /// The monospace font family, replacing the resolved one wherever it
    /// stands.
    pub monospace_family: Option<Arc<str>>,
    /// The colour roles it replaces.
    pub color_overrides: ColorPatch,
    /// Per widget: the rules appended to a ruled widget's parts, the
    /// unconditional patches of a static widget's parts, and the direct
    /// metrics.
    pub widget_rules: WidgetPatchMap,
    /// The application's own colour tokens, merged into the resolved
    /// theme's.
    pub custom_colors: BTreeMap<String, Color>,
    /// The application's own metric tokens, merged into the resolved theme's.
    pub custom_metrics: BTreeMap<String, f32>,
}

/// A scale as the setters take it: clamped to the finite values `>= 0.0`, and
/// `1.0` (no change) where it is not a number.
fn clamped_scale(scale: f32) -> f32 {
    if scale.is_nan() {
        1.0
    } else {
        scale.clamp(0.0, f32::MAX)
    }
}

/// One shortcut per ruled widget: `ThemePatch::new().button(rule)` appends
/// `rule` to the button's rules.
macro_rules! rule_shortcuts {
    ($( $widget:ident : $Rule:ident ),* $(,)?) => {
        impl ThemePatch {
            $(
                #[doc = concat!("This patch with `rule` appended to the rules of `", stringify!($widget), "`.")]
                #[must_use]
                pub fn $widget(mut self, rule: $Rule) -> Self {
                    self.widget_rules.$widget.rules.push(rule);
                    self
                }
            )*
        }
    };
}

rule_shortcuts! {
    button: ButtonRule,
    input: InputRule,
    text_area: InputRule,
    checkbox: CheckboxRule,
    radio: RadioRule,
    switch: SwitchRule,
    slider: SliderRule,
    progress_bar: ProgressBarRule,
    scrollbar: ScrollbarRule,
    tab: TabRule,
    menu: MenuRule,
    menu_bar: MenuBarRule,
    list: ListRule,
    sidebar: SidebarRule,
    toolbar: ToolbarRule,
    combo_box: ComboBoxRule,
    segmented_control: SegmentedControlRule,
    expander: ExpanderRule,
    card: CardRule,
    splitter: SplitterRule,
    link: LinkRule,
}

impl ThemePatch {
    /// The patch that states nothing.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// This patch with the corner radii scaled by `scale`, clamped to the
    /// finite values `>= 0.0` (`1.0` where it is not a number).
    #[must_use]
    pub fn corner_scale(mut self, scale: f32) -> Self {
        self.corner_scale = Some(clamped_scale(scale));
        self
    }

    /// This patch with the spacing scaled by `scale`, clamped to the finite
    /// values `>= 0.0` (`1.0` where it is not a number).
    #[must_use]
    pub fn spacing_scale(mut self, scale: f32) -> Self {
        self.spacing_scale = Some(clamped_scale(scale));
        self
    }

    /// This patch with the UI font family `family`.
    #[must_use]
    pub fn font_family(mut self, family: impl Into<Arc<str>>) -> Self {
        self.font_family = Some(family.into());
        self
    }

    /// This patch with the monospace font family `family`.
    #[must_use]
    pub fn monospace_family(mut self, family: impl Into<Arc<str>>) -> Self {
        self.monospace_family = Some(family.into());
        self
    }

    /// This patch with its colour overrides replaced.
    #[must_use]
    pub fn color_overrides(mut self, colors: ColorPatch) -> Self {
        self.color_overrides = colors;
        self
    }

    /// This patch with its widget patches replaced.
    #[must_use]
    pub fn widget_rules(mut self, widgets: WidgetPatchMap) -> Self {
        self.widget_rules = widgets;
        self
    }

    /// This patch with the custom colour `key` set to `color`.
    #[must_use]
    pub fn custom_color(mut self, key: impl Into<String>, color: Color) -> Self {
        self.custom_colors.insert(key.into(), color);
        self
    }

    /// This patch with the custom metric `key` set to `value`.
    #[must_use]
    pub fn custom_metric(mut self, key: impl Into<String>, value: f32) -> Self {
        self.custom_metrics.insert(key.into(), value);
        self
    }

    /// The strict check: every number finite, every size (a scale, a radius,
    /// a width, a blur, a padding, a gap, a font or icon size) `>= 0.0`, every
    /// opacity (a colour's alpha included) in `0.0..=1.0`. The first value
    /// that fails is returned as [`Error::InvalidTheme`], its path as a TOML
    /// file would name it (`widgets.button.rules[0].body.opacity`).
    ///
    /// A static widget's entry holds no rule, so a state selector on a static
    /// widget cannot be written into a patch.
    ///
    /// # Errors
    ///
    /// [`Error::InvalidTheme`] for the first invalid value.
    pub fn validate(&self) -> Result<(), Error> {
        if let Some(scale) = &self.corner_scale {
            check::size("metrics.corner_scale", scale)?;
        }
        if let Some(scale) = &self.spacing_scale {
            check::size("metrics.spacing_scale", scale)?;
        }
        self.color_overrides.check("colors")?;
        self.widget_rules.check("widgets")?;
        for (key, color) in &self.custom_colors {
            check::color(&format!("custom_colors.{key}"), color)?;
        }
        for (key, value) in &self.custom_metrics {
            check::finite(&format!("custom_metrics.{key}"), value)?;
        }
        Ok(())
    }
}

/// The checks of [`ThemePatch::validate`], each returning the
/// [`Error::InvalidTheme`] of the value at `path` where it fails.
mod check {
    use super::{
        Color, CornerRadius, Corners, Edges, Error, FocusIndicator, GradientStop, Paint,
        PaintLayer, ShadowLayer, Stroke, Timing, Transform, Transition,
    };

    fn invalid(path: &str, reason: &str) -> Error {
        Error::InvalidTheme {
            path: path.to_owned(),
            reason: reason.to_owned(),
        }
    }

    /// A value without a numeric check.
    pub(super) fn none<T>(_path: &str, _value: &T) -> Result<(), Error> {
        Ok(())
    }

    /// A finite number.
    pub(super) fn finite(path: &str, value: &f32) -> Result<(), Error> {
        if value.is_finite() {
            Ok(())
        } else {
            Err(invalid(path, "not a finite number"))
        }
    }

    /// A size: finite and `>= 0.0`.
    pub(super) fn size(path: &str, value: &f32) -> Result<(), Error> {
        finite(path, value)?;
        if *value >= 0.0 {
            Ok(())
        } else {
            Err(invalid(path, "negative"))
        }
    }

    /// An opacity: finite and in `0.0..=1.0`.
    pub(super) fn unit(path: &str, value: &f32) -> Result<(), Error> {
        finite(path, value)?;
        if (0.0..=1.0).contains(value) {
            Ok(())
        } else {
            Err(invalid(path, "outside 0.0..=1.0"))
        }
    }

    /// A colour: finite channels, its alpha an opacity.
    pub(super) fn color(path: &str, color: &Color) -> Result<(), Error> {
        finite(&format!("{path}.r"), &color.r)?;
        finite(&format!("{path}.g"), &color.g)?;
        finite(&format!("{path}.b"), &color.b)?;
        unit(&format!("{path}.a"), &color.a)
    }

    /// The colours of `fields` that are stated, each under `path`.
    pub(super) fn colors<const N: usize>(
        path: &str,
        fields: [(&str, &Option<Color>); N],
    ) -> Result<(), Error> {
        for (field, value) in fields {
            if let Some(color) = value {
                self::color(&format!("{path}.{field}"), color)?;
            }
        }
        Ok(())
    }

    pub(super) fn paint(path: &str, paint: &Paint) -> Result<(), Error> {
        match paint {
            Paint::None => Ok(()),
            Paint::Solid(color) => self::color(path, color),
            Paint::Complex(complex) => {
                for (index, layer) in complex.layers.iter().enumerate() {
                    paint_layer(&format!("{path}.layers[{index}]"), layer)?;
                }
                Ok(())
            }
        }
    }

    fn gradient_stops(path: &str, stops: &[GradientStop]) -> Result<(), Error> {
        for (index, stop) in stops.iter().enumerate() {
            finite(&format!("{path}.stops[{index}].offset"), &stop.offset)?;
            self::color(&format!("{path}.stops[{index}].color"), &stop.color)?;
        }
        Ok(())
    }

    fn paint_layer(path: &str, layer: &PaintLayer) -> Result<(), Error> {
        let at = |field: &str| format!("{path}.{field}");
        match layer {
            PaintLayer::Solid { color, .. } => self::color(&at("color"), color),
            PaintLayer::LinearGradient {
                angle_deg, stops, ..
            } => {
                finite(&at("angle_deg"), angle_deg)?;
                gradient_stops(path, stops)
            }
            PaintLayer::RadialGradient {
                center,
                radius,
                stops,
                ..
            } => {
                for value in center {
                    finite(&at("center"), value)?;
                }
                size(&at("radius"), radius)?;
                gradient_stops(path, stops)
            }
            PaintLayer::Material(material) => {
                size(&at("blur_radius"), &material.blur_radius)?;
                self::color(&at("tint"), &material.tint)?;
                finite(&at("saturation"), &material.saturation)?;
                unit(&at("luminosity_opacity"), &material.luminosity_opacity)?;
                unit(&at("noise_opacity"), &material.noise_opacity)?;
                self::color(&at("fallback"), &material.fallback)
            }
        }
    }

    pub(super) fn stroke(path: &str, stroke: &Stroke) -> Result<(), Error> {
        match stroke {
            Stroke::None => Ok(()),
            Stroke::Uniform { width, color, .. } => {
                size(&format!("{path}.width"), width)?;
                self::color(&format!("{path}.color"), color)
            }
            Stroke::Complex(complex) => {
                for (side, value) in [
                    ("top", &complex.top),
                    ("right", &complex.right),
                    ("bottom", &complex.bottom),
                    ("left", &complex.left),
                ] {
                    size(&format!("{path}.{side}.width"), &value.width)?;
                    paint(&format!("{path}.{side}.paint"), &value.paint)?;
                }
                Ok(())
            }
        }
    }

    pub(super) fn corners(path: &str, corners: &Corners) -> Result<(), Error> {
        let radius = format!("{path}.radius");
        match &corners.radius {
            CornerRadius::Uniform(r) => size(&radius, r),
            CornerRadius::PerCorner {
                top_left,
                top_right,
                bottom_right,
                bottom_left,
            } => {
                for r in [top_left, top_right, bottom_right, bottom_left] {
                    size(&radius, r)?;
                }
                Ok(())
            }
            CornerRadius::Concentric { inset, min_radius } => {
                finite(&format!("{radius}.inset"), inset)?;
                size(&format!("{radius}.min_radius"), min_radius)
            }
            _ => Ok(()),
        }
    }

    pub(super) fn shadow(path: &str, layer: &ShadowLayer) -> Result<(), Error> {
        finite(&format!("{path}.offset_x"), &layer.offset_x)?;
        finite(&format!("{path}.offset_y"), &layer.offset_y)?;
        size(&format!("{path}.blur"), &layer.blur)?;
        finite(&format!("{path}.spread"), &layer.spread)?;
        self::color(&format!("{path}.color"), &layer.color)
    }

    pub(super) fn transform(path: &str, transform: &Transform) -> Result<(), Error> {
        let [sx, sy] = &transform.scale;
        let [tx, ty] = &transform.translation;
        let [ox, oy] = &transform.origin;
        for (field, value) in [
            ("scale", sx),
            ("scale", sy),
            ("rotation_deg", &transform.rotation_deg),
            ("translation", tx),
            ("translation", ty),
            ("origin", ox),
            ("origin", oy),
        ] {
            finite(&format!("{path}.{field}"), value)?;
        }
        Ok(())
    }

    /// The four sides of `edges`, each checked by `side`.
    pub(super) fn edges(
        path: &str,
        edges: &Edges,
        side: fn(&str, &f32) -> Result<(), Error>,
    ) -> Result<(), Error> {
        side(&format!("{path}.top"), &edges.top)?;
        side(&format!("{path}.right"), &edges.right)?;
        side(&format!("{path}.bottom"), &edges.bottom)?;
        side(&format!("{path}.left"), &edges.left)
    }

    /// A `[width, height]` pair, each stated dimension a size.
    pub(super) fn dimensions(path: &str, value: &[Option<f32>; 2]) -> Result<(), Error> {
        for dimension in value.iter().flatten() {
            size(path, dimension)?;
        }
        Ok(())
    }

    pub(super) fn focus(path: &str, focus: &FocusIndicator) -> Result<(), Error> {
        stroke(&format!("{path}.stroke"), &focus.stroke)?;
        stroke(&format!("{path}.inner_stroke"), &focus.inner_stroke)?;
        finite(&format!("{path}.offset"), &focus.offset)?;
        match &focus.corners {
            Some(c) => corners(&format!("{path}.corners"), c),
            None => Ok(()),
        }
    }

    pub(super) fn transition(path: &str, transition: &Transition) -> Result<(), Error> {
        let timing = format!("{path}.timing");
        match &transition.timing {
            Timing::CubicBezier(x1, y1, x2, y2) => {
                for value in [x1, y1, x2, y2] {
                    finite(&timing, value)?;
                }
                Ok(())
            }
            Timing::Spring {
                mass,
                stiffness,
                damping,
            } => {
                for value in [mass, stiffness, damping] {
                    finite(&timing, value)?;
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests;
