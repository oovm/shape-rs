use super::*;
use shape_core::Line;

impl<T> ToSVG for Line<T>
where
    T: Display,
{
    type Element = svg::node::element::Line;

    fn to_svg(&self) -> Self::Element {
        svg::node::element::Line::new()
            .set("x1", self.s.x.to_string())
            .set("y1", self.s.y.to_string())
            .set("x2", self.e.x.to_string())
            .set("y2", self.e.y.to_string())
    }
}
