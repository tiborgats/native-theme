//! Fills: solid colours and layered paints with gradients and backdrop
//! materials.

use std::sync::Arc;

use super::color::Color;

/// What fills an area: nothing, one colour, or a stack of layers.
#[derive(Clone, Debug, Default, PartialEq)]
pub enum Paint {
    /// No fill.
    #[default]
    None,
    /// One colour.
    Solid(Color),
    /// Layers, bottom to top.
    Complex(Arc<ComplexPaint>),
}

impl Paint {
    /// One colour.
    #[must_use]
    pub fn solid(c: Color) -> Self {
        Self::Solid(c)
    }

    /// A stack of layers, bottom to top.
    #[must_use]
    pub fn layers(layers: impl Into<Arc<[PaintLayer]>>) -> Self {
        Self::Complex(Arc::new(ComplexPaint {
            layers: layers.into(),
        }))
    }

    /// Best single representative solid color (top opaque/fallback layer) for simple toolkits.
    ///
    /// One rule for [`Paint::Solid`] and for layers, a solid paint being one
    /// solid layer: each solid layer stands for its colour and each material
    /// for its [`fallback`](MaterialSpec::fallback); of these, the topmost
    /// opaque one, else (no layer opaque) the topmost one. A material counts
    /// as opaque only where its fallback colour is, so a material above an
    /// opaque solid layer whose fallback is translucent does not win over
    /// that layer. `None` for [`Paint::None`] and for layers that are all
    /// gradients, which have no single colour a source states.
    #[must_use]
    pub fn fallback_color(&self) -> Option<Color> {
        match self {
            Self::None => None,
            Self::Solid(c) => Some(*c),
            Self::Complex(complex) => {
                let colors = || complex.layers.iter().rev().filter_map(PaintLayer::color);
                colors().find(|c| c.a >= 1.0).or_else(|| colors().next())
            }
        }
    }

    /// Composites all layers in order over `backdrop` using each layer's fallback for materials.
    ///
    /// Each solid layer and each material's fallback is laid over the result
    /// so far by source-over ([`Color::composite_over`]); gradient layers,
    /// which have no single colour, are left out, and blend modes are not
    /// applied.
    #[must_use]
    pub fn flatten_over(&self, backdrop: Color) -> Color {
        match self {
            Self::None => backdrop,
            Self::Solid(c) => c.composite_over(backdrop),
            Self::Complex(complex) => {
                complex
                    .layers
                    .iter()
                    .fold(backdrop, |under, layer| match layer {
                        PaintLayer::Solid { color, .. } => color.composite_over(under),
                        PaintLayer::Material(material) => material.fallback.composite_over(under),
                        PaintLayer::LinearGradient { .. } | PaintLayer::RadialGradient { .. } => {
                            under
                        }
                    })
            }
        }
    }
}

/// The layers of a [`Paint::Complex`]; made by [`Paint::layers`].
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct ComplexPaint {
    /// The layers, bottom to top; a layer holds no paint of its own.
    pub layers: Arc<[PaintLayer]>, // bottom-to-top, non-recursive
}

/// One layer of a [`ComplexPaint`].
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum PaintLayer {
    /// One colour.
    Solid {
        /// The colour.
        color: Color,
        /// How it blends with the layers below.
        blend: BlendMode,
    },
    /// A linear gradient.
    LinearGradient {
        /// The gradient line's angle, in degrees.
        angle_deg: f32,
        /// The colour stops, in offset order.
        stops: Arc<[GradientStop]>,
        /// How it blends with the layers below.
        blend: BlendMode,
    },
    /// A radial gradient.
    RadialGradient {
        /// The centre, normalised to the area (`[0.5, 0.5]` is its middle).
        center: [f32; 2],
        /// The radius, normalised to the area.
        radius: f32,
        /// The colour stops, in offset order.
        stops: Arc<[GradientStop]>,
        /// How it blends with the layers below.
        blend: BlendMode,
    },
    /// A backdrop material.
    Material(MaterialSpec),
}

impl PaintLayer {
    /// The one colour this layer stands for: a solid layer's colour, a
    /// material's fallback; `None` for a gradient.
    fn color(&self) -> Option<Color> {
        match self {
            Self::Solid { color, .. } => Some(*color),
            Self::Material(material) => Some(material.fallback),
            Self::LinearGradient { .. } | Self::RadialGradient { .. } => None,
        }
    }
}

/// A colour stop of a gradient.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(deny_unknown_fields)
)]
pub struct GradientStop {
    /// The stop's position along the gradient, `0.0..=1.0`.
    pub offset: f32,
    /// The colour at that position.
    pub color: Color,
}

/// How a layer blends with what lies below it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
pub enum BlendMode {
    /// Source-over.
    #[default]
    Normal,
    /// Multiply.
    Multiply,
    /// Screen.
    Screen,
    /// Overlay.
    Overlay,
    /// Plus darker (Apple's `plusDarker`).
    PlusDarker,
    /// Plus lighter (Apple's `plusLighter`).
    PlusLighter,
}

/// A backdrop material: a blurred, tinted view of what lies behind.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct MaterialSpec {
    /// Which platform material.
    pub kind: MaterialKind,
    /// The backdrop blur radius, in logical pixels.
    pub blur_radius: f32,
    /// The tint laid over the blurred backdrop.
    pub tint: Color,
    /// The backdrop's saturation factor.
    pub saturation: f32, // 1.0 = neutral
    /// The luminosity layer's opacity.
    pub luminosity_opacity: f32, // 0.0..=1.0 (WinUI Acrylic/Mica)
    /// The noise texture's opacity.
    pub noise_opacity: f32, // 0.0..=1.0
    /// The opaque colour a toolkit without backdrop blur draws instead.
    pub fallback: Color, // opaque fallback for toolkits without backdrop blur
}

impl MaterialSpec {
    /// A material of every field stated (spec §1, "Constructors"): no source
    /// states a blur, tint or opacity for a material in general, so none is
    /// assumed.
    #[must_use]
    pub fn new(
        kind: MaterialKind,
        blur_radius: f32,
        tint: Color,
        saturation: f32,
        luminosity_opacity: f32,
        noise_opacity: f32,
        fallback: Color,
    ) -> Self {
        Self {
            kind,
            blur_radius,
            tint,
            saturation,
            luminosity_opacity,
            noise_opacity,
            fallback,
        }
    }
}

/// The platform material a [`MaterialSpec`] stands for.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
pub enum MaterialKind {
    /// Windows 11 Mica.
    Mica,
    /// Windows 11 Mica Alt.
    MicaAlt,
    /// Windows Acrylic.
    Acrylic,
    /// macOS vibrancy (`NSVisualEffectView`).
    MacosVibrancy,
    /// macOS Liquid Glass.
    MacosLiquidGlass,
    /// iOS blur (`UIBlurEffect`).
    IosBlur,
    /// iOS Liquid Glass.
    IosLiquidGlass,
    /// KWin's blur effect.
    KwinBlur,
    /// A blur the theme describes itself.
    CustomBlur,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A material whose only meaningful field here is its fallback; the
    /// other fields are arbitrary test values.
    fn material(kind: MaterialKind, fallback: Color) -> MaterialSpec {
        MaterialSpec::new(kind, 30.0, Color::TRANSPARENT, 1.0, 0.0, 0.0, fallback)
    }

    fn solid(color: Color) -> PaintLayer {
        PaintLayer::Solid {
            color,
            blend: BlendMode::Normal,
        }
    }

    fn gradient() -> PaintLayer {
        PaintLayer::LinearGradient {
            angle_deg: 90.0,
            stops: Arc::from(vec![GradientStop {
                offset: 0.0,
                color: Color::BLACK,
            }]),
            blend: BlendMode::Normal,
        }
    }

    #[test]
    fn fallback_color_takes_the_top_opaque_or_material_layer() {
        assert_eq!(Paint::None.fallback_color(), None);
        assert_eq!(
            Paint::solid(Color::WHITE).fallback_color(),
            Some(Color::WHITE)
        );
        let grey = Color::rgb(0.5, 0.5, 0.5);
        let half_red = Color::new(1.0, 0.0, 0.0, 0.5);
        let paint = Paint::layers(vec![
            PaintLayer::Material(material(MaterialKind::Mica, grey)),
            solid(half_red),
        ]);
        assert_eq!(paint.fallback_color(), Some(grey));
        assert_eq!(Paint::layers(vec![gradient()]).fallback_color(), None);
    }

    #[test]
    fn fallback_color_is_one_rule_for_solid_and_layers() {
        let half_red = Color::new(1.0, 0.0, 0.0, 0.5);
        let half_blue = Color::new(0.0, 0.0, 1.0, 0.5);
        // A translucent solid paint and the stack of that one layer agree.
        assert_eq!(Paint::solid(half_red).fallback_color(), Some(half_red));
        assert_eq!(
            Paint::layers(vec![solid(half_red)]).fallback_color(),
            Some(half_red)
        );
        // No layer opaque: the topmost solid or material layer, gradients skipped.
        assert_eq!(
            Paint::layers(vec![solid(half_red), solid(half_blue), gradient()]).fallback_color(),
            Some(half_blue)
        );
        assert_eq!(
            Paint::layers(vec![
                solid(half_red),
                PaintLayer::Material(material(MaterialKind::Acrylic, half_blue)),
            ])
            .fallback_color(),
            Some(half_blue)
        );
        // An opaque layer below translucent ones wins over them.
        assert_eq!(
            Paint::layers(vec![solid(Color::WHITE), solid(half_red), gradient()]).fallback_color(),
            Some(Color::WHITE)
        );
        assert_eq!(
            Paint::layers(vec![gradient(), gradient()]).fallback_color(),
            None
        );
    }

    #[test]
    fn a_translucent_material_fallback_does_not_win_over_an_opaque_solid_below() {
        let half_grey = Color::new(0.5, 0.5, 0.5, 0.5);
        let paint = Paint::layers(vec![
            solid(Color::WHITE),
            PaintLayer::Material(material(MaterialKind::Acrylic, half_grey)),
        ]);
        assert_eq!(paint.fallback_color(), Some(Color::WHITE));
        // With an opaque fallback the material is the topmost opaque layer.
        let grey = Color::rgb(0.5, 0.5, 0.5);
        let paint = Paint::layers(vec![
            solid(Color::WHITE),
            PaintLayer::Material(material(MaterialKind::Acrylic, grey)),
        ]);
        assert_eq!(paint.fallback_color(), Some(grey));
    }

    #[test]
    fn flatten_over_composites_bottom_to_top() {
        assert_eq!(Paint::None.flatten_over(Color::WHITE), Color::WHITE);
        let half_black = Color::new(0.0, 0.0, 0.0, 0.5);
        assert_eq!(
            Paint::solid(half_black).flatten_over(Color::WHITE),
            Color::new(0.5, 0.5, 0.5, 1.0)
        );
        // White, then half black over it (0.5), then half black again: 0.5 × 0.5 = 0.25.
        let paint = Paint::layers(vec![
            PaintLayer::Material(material(MaterialKind::Acrylic, Color::WHITE)),
            solid(half_black),
            solid(half_black),
        ]);
        assert_eq!(
            paint.flatten_over(Color::BLACK),
            Color::new(0.25, 0.25, 0.25, 1.0)
        );
    }

    #[test]
    fn material_new_takes_every_field() {
        let tint = Color::new(1.0, 1.0, 1.0, 0.25);
        let m = MaterialSpec::new(
            MaterialKind::KwinBlur,
            12.0,
            tint,
            1.5,
            0.75,
            0.125,
            Color::BLACK,
        );
        assert_eq!(m.kind, MaterialKind::KwinBlur);
        assert_eq!(m.blur_radius, 12.0);
        assert_eq!(m.tint, tint);
        assert_eq!(m.saturation, 1.5);
        assert_eq!(m.luminosity_opacity, 0.75);
        assert_eq!(m.noise_opacity, 0.125);
        assert_eq!(m.fallback, Color::BLACK);
    }
}
