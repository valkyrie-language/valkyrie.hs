use crate::Rectangle;

pub trait ToSvg {
    type Target;
    fn to_svg(&self) -> Self::Target;
}

impl ToSvg for Rectangle {
    type Target = svg::node::element::Rectangle;

    fn to_svg(&self) -> Self::Target {
        let mut out = svg::node::element::Rectangle::new()
            .set("x", self.x)
            .set("y", self.y)
            .set("width", self.width)
            .set("height", self.height);
        // if self.color.eq(Color::BLACK) {
        //
        // }

        out
    }
}
