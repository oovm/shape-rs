use super::*;
use std::ops::Sub;

impl<T> ToSVG for Point<T>
where
    T: Display,
{
    type Element = svg::node::element::Circle;

    fn to_svg(&self) -> Self::Element {
        svg::node::element::Circle::new().set("cx", self.x.to_string()).set("cy", self.y.to_string()).set("r", 1)
    }
}

impl<T> ToSVG for Triangle<T>
where
    T: Display,
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
    T: Display,
{
    type Element = svg::node::element::Rectangle;

    fn to_svg(&self) -> Self::Element {
        svg::node::element::Rectangle::new()
            .set("x", self.x.to_string())
            .set("y", self.y.to_string())
            .set("width", self.s.to_string())
            .set("height", self.s.to_string())
    }
}

impl<T> ToSVG for Rectangle<T>
where
    T: Display + Clone + Sub<Output = T>,
{
    type Element = svg::node::element::Rectangle;

    fn to_svg(&self) -> Self::Element {
        svg::node::element::Rectangle::new()
            .set("x", self.min.to_string())
            .set("y", self.max.to_string())
            .set("width", self.width().to_string())
            .set("height", self.height().to_string())
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
