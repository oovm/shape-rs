use super::*;
use shape_core::Line;

impl<T> ToSVG for Line<T>
where
    T: Clone,
    T: Into<Value>,
{
    type Element = svg::node::element::Line;

    fn to_svg(&self) -> Self::Element {
        svg::node::element::Line::new()
            .set("x1", self.s.x.clone())
            .set("y1", self.s.y.clone())
            .set("x2", self.e.x.clone())
            .set("y2", self.e.y.clone())
    }
}
