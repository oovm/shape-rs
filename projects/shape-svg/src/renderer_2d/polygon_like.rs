use super::*;

impl<T> ToSVG for Point<T>
where
    T: Clone,
    T: Into<Value>,
{
    type Element = svg::node::element::Circle;

    fn to_svg(&self) -> Self::Element {
        svg::node::element::Circle::new().set("cx", self.x.clone()).set("cy", self.y.clone()).set("r", 1)
    }
}

impl<T> ToSVG for Triangle<T>
where
    T: Into<Value> + Display,
{
    type Element = svg::node::element::Polygon;

    fn to_svg(&self) -> Self::Element {
        let mut points = String::new();
        points.push_str(&format!("{},{} ", self.a.x, self.a.y));
        points.push_str(&format!("{},{} ", self.b.x, self.b.y));
        points.push_str(&format!("{},{} ", self.c.x, self.c.y));
        svg::node::element::Polygon::new().set("points", points)
    }
}

impl<T> ToSVG for Square<T>
where
    T: Clone,
    T: Into<Value>,
{
    type Element = svg::node::element::Rectangle;

    fn to_svg(&self) -> Self::Element {
        svg::node::element::Rectangle::new()
            .set("x", self.x.clone())
            .set("y", self.y.clone())
            .set("width", self.s.clone())
            .set("height", self.s.clone())
    }
}

impl<T> ToSVG for Rectangle<T>
where
    T: Clone,
    T: Into<Value>,
{
    type Element = svg::node::element::Rectangle;

    fn to_svg(&self) -> Self::Element {
        svg::node::element::Rectangle::new()
            .set("x", self.x.clone())
            .set("y", self.y.clone())
            .set("width", self.w.clone())
            .set("height", self.h.clone())
    }
}

impl<T> ToSVG for Parallelogram<T>
where
    T: Display,
{
    type Element = svg::node::element::Polygon;

    fn to_svg(&self) -> Self::Element {
        todo!()
    }
}

impl<T> ToSVG for Polyline<T> {
    type Element = svg::node::element::Polyline;

    fn to_svg(&self) -> Self::Element {
        todo!()
    }
}

impl<T> ToSVG for RegularPolygon<T> {
    type Element = svg::node::element::Polygon;

    fn to_svg(&self) -> Self::Element {
        todo!()
    }
}

impl<T> ToSVG for Polygon<T>
where
    T: Display,
{
    type Element = svg::node::element::Polygon;

    fn to_svg(&self) -> Self::Element {
        let mut points = String::new();
        for Point { x, y } in self.points_set.points.iter() {
            points.push_str(&format!("{},{} ", x, y));
        }
        svg::node::element::Polygon::new().set("points", points)
    }
}
