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
    /// For [`Paint::Solid`] its colour; for layers, from the top down, the
    /// first opaque solid layer's colour or the first material's
    /// [`fallback`](MaterialSpec::fallback); `None` for [`Paint::None`] and
    /// for layers with neither.
    #[must_use]
    pub fn fallback_color(&self) -> Option<Color> {
        match self {
            Self::None => None,
            Self::Solid(c) => Some(*c),
            Self::Complex(complex) => complex.layers.iter().rev().find_map(|layer| match layer {
                PaintLayer::Solid { color, .. } if color.a >= 1.0 => Some(*color),
                PaintLayer::Material(material) => Some(material.fallback),
                _ => None,
            }),
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

/// A colour stop of a gradient.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GradientStop {
    /// The stop's position along the gradient, `0.0..=1.0`.
    pub offset: f32,
    /// The colour at that position.
    pub color: Color,
}

/// How a layer blends with what lies below it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
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
    /// A material of `kind` drawn as `fallback` where blur is unavailable,
    /// with neutral settings: no blur, a transparent tint, saturation 1.0,
    /// and luminosity and noise opacity 0.0.
    #[must_use]
    pub fn new(kind: MaterialKind, fallback: Color) -> Self {
        Self {
            kind,
            blur_radius: 0.0,
            tint: Color::TRANSPARENT,
            saturation: 1.0,
            luminosity_opacity: 0.0,
            noise_opacity: 0.0,
            fallback,
        }
    }
}

/// The platform material a [`MaterialSpec`] stands for.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
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
            PaintLayer::Material(MaterialSpec::new(MaterialKind::Mica, grey)),
            PaintLayer::Solid {
                color: half_red,
                blend: BlendMode::Normal,
            },
        ]);
        assert_eq!(paint.fallback_color(), Some(grey));
        let gradient_only = Paint::layers(vec![PaintLayer::LinearGradient {
            angle_deg: 90.0,
            stops: Arc::from(vec![GradientStop {
                offset: 0.0,
                color: Color::BLACK,
            }]),
            blend: BlendMode::Normal,
        }]);
        assert_eq!(gradient_only.fallback_color(), None);
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
            PaintLayer::Material(MaterialSpec::new(MaterialKind::Acrylic, Color::WHITE)),
            PaintLayer::Solid {
                color: half_black,
                blend: BlendMode::Normal,
            },
            PaintLayer::Solid {
                color: half_black,
                blend: BlendMode::Normal,
            },
        ]);
        assert_eq!(
            paint.flatten_over(Color::BLACK),
            Color::new(0.25, 0.25, 0.25, 1.0)
        );
    }

    #[test]
    fn material_new_is_neutral() {
        let m = MaterialSpec::new(MaterialKind::KwinBlur, Color::BLACK);
        assert_eq!(m.saturation, 1.0);
        assert_eq!(m.fallback, Color::BLACK);
    }
}
