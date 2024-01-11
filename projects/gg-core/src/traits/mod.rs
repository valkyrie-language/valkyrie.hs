use crate::Srgb;
use std::sync::Arc;

#[cfg(feature = "svg")]
pub mod to_svg;

/// A color palette.
pub trait Palette {
    /// Get the next color in the palette.
    fn next_color(&mut self) -> Srgb;
    /// Get the color at the given position in the palette.
    fn interpolate(&self, t: f32) -> Srgb;
}

/// A style attribute that can be inherited from the parent node.
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Style<T> {
    ///
    Initial,
    /// Adopt the same settings as the parent node
    #[default]
    Inherit,
    ///
    Custom(Box<T>),
    ///
    Shared(Arc<T>),
}

impl<T> Style<T> {
    /// Get the style from the current theme.
    pub fn unwrap<'a, 'b, 'c>(&'a self, parent: &'b Option<Style<T>>, default: &'c T) -> &'a T
    where
        'b: 'a,
        'c: 'a,
    {
        match self {
            Self::Custom(s) => s,
            Self::Shared(s) => s,
            Self::Inherit => match parent {
                Some(Self::Custom(s)) => s,
                Some(Self::Shared(s)) => s,
                _ => default,
            },
            Self::Initial => default,
        }
    }
}
