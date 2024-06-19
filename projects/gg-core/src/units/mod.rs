#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Color {
    Pure(Srgb),
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Srgb {
    /// Red channel in linear space, with a range of 0.0 to 1.0.
    pub r: f32,
    /// Green channel in linear space, with a range of 0.0 to 1.0.
    pub g: f32,
    /// Blue channel in linear space, with a range of 0.0 to 1.0.
    pub b: f32,
    /// Alpha channel, with a range of 0.0 to 1.0.
    pub a: f32,
}

impl Srgb {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r: r as f32 / 255.0, g: g as f32 / 255.0, b: b as f32 / 255.0, a: 1.0 }
    }
    pub fn with_alpha(self, a: f32) -> Self {
        Self { a, ..self }
    }
}

impl Color {
    pub const BLACK: Self = Self::Pure(Srgb { r: 0.0, g: 0.0, b: 0.0, a: 1.0 });
    pub const WHITE: Self = Self::Pure(Srgb { r: 1.0, g: 1.0, b: 1.0, a: 1.0 });
}

impl Default for Color {
    fn default() -> Self {
        Self::Pure(Srgb::default())
    }
}

impl Default for Srgb {
    fn default() -> Self {
        Self { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }
    }
}
