use super::*;

impl<T> ToSVG for Circle<T>
where
    T: Clone,
    T: Into<Value>,
{
    type Element = svg::node::element::Circle;

    fn to_svg(&self) -> Self::Element {
        svg::node::element::Circle::new()
            .set("cx", self.center.x.clone())
            .set("cy", self.center.y.clone())
            .set("r", self.radius.clone())
    }
}

impl<T> ToSVG for Ellipse<T>
where
    T: Clone,
    T: Into<Value>,
{
    type Element = svg::node::element::Ellipse;

    fn to_svg(&self) -> Self::Element {
        svg::node::element::Ellipse::new()
            .set("cx", self.center.x.clone())
            .set("cy", self.center.y.clone())
            .set("rx", self.radius.0.clone())
            .set("ry", self.radius.1.clone())
    }
}
