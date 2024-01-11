#![deny(missing_debug_implementations, missing_copy_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

mod errors;
mod shapes;
mod traits;

mod units;

pub use crate::{
    errors::{PlotError, Result},
    shapes::{Ellipse, Point, Rectangle},
    traits::{Palette, Style},
    units::{Color, Srgb},
};

#[cfg(feature = "svg")]
pub use crate::traits::to_svg::ToSvg;
