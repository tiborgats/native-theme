// Rgba color type -- implemented in Task 2

/// An sRGB color with alpha, stored as four u8 components.
///
/// All values are in the sRGB color space. When parsing hex strings,
/// alpha defaults to 255 (fully opaque) if omitted.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
