use super::*;

impl<T> ToSVG for Circle<T>
where
    T: Into<Value>,
{
    type Element = svg::node::element::Circle;

    fn to_svg(&self) -> Self::Element {
        svg::node::element::Circle::new().set("cx", self.center.x).set("cy", self.center.y).set("r", self.radius)
    }
}

impl<T> ToSVG for Ellipse<T>
where
    T: Into<Value>,
{
    type Element = svg::node::element::Ellipse;

    fn to_svg(&self) -> Self::Element {
        svg::node::element::Ellipse::new()
            .set("cx", self.center.x)
            .set("cy", self.center.y)
            .set("rx", self.rx)
            .set("ry", self.ry)
    }
}
