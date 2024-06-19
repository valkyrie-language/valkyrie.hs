use crate::{Color, Rectangle, Srgb};

pub trait ToSvg {
    type Target;
    fn to_svg(&self) -> Self::Target;
}

impl ToSvg for Rectangle {
    type Target = svg::node::element::Rectangle;

    fn to_svg(&self) -> Self::Target {
        let out = svg::node::element::Rectangle::new()
            .set("x", self.x)
            .set("y", self.y)
            .set("width", self.width)
            .set("height", self.height)
            .set("fill", self.color.to_svg());
        // if self.color.eq(Color::BLACK) {
        //
        // }

        out
    }
}

impl ToSvg for Color {
    type Target = String;

    fn to_svg(&self) -> Self::Target {
        match self {
            Self::Pure(s) => s.to_svg(),
        }
    }
}

impl ToSvg for Srgb {
    type Target = String;

    fn to_svg(&self) -> Self::Target {
        format!("color(srgb-linear {} {} {} / {})", self.r, self.g, self.b, self.a)
    }
}
