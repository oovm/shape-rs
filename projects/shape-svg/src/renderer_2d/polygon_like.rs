use super::*;

impl<T> ToSVG for Point<T>
where
    T: Display + One + Clone,
{
    type Element = svg::node::element::Circle;

    fn to_svg(&self) -> Self::Element {
        svg::node::element::Circle::new().set("cx", self.x).set("cy", self.y).set("r", 1)
    }
}

impl<T> ToSVG for Triangle<T>
where
    T: Display + Clone,
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
            .set("x", self.anchor)
            .set("y", self.anchor)
            .set("width", self.s)
            .set("height", self.s)
    }
}

impl<T> ToSVG for Rectangle<T>
where
    T: Display,
{
    type Element = svg::node::element::Rectangle;

    fn to_svg(&self) -> Self::Element {
        svg::node::element::Rectangle::new()
            .set("x", self.center.x - self.width / 2)
            .set("y", self.center.y - self.height / 2)
            .set("width", self.width)
            .set("height", self.height)
    }
}

impl<T> ToSVG for Parallelogram<T>
where
    T: Display,
{
    type Element = svg::node::element::Polygon;

    fn to_svg(&self) -> Self::Element {
        let mut points = String::new();
        points.push_str(&format!("{},{} ", self.a.x, self.a.y));
        points.push_str(&format!("{},{} ", self.b.x, self.b.y));
        points.push_str(&format!("{},{} ", self.c.x, self.c.y));
        points.push_str(&format!("{},{} ", self.d.x, self.d.y));
        svg::node::element::Polygon::new().set("points", points)
    }
}

impl<T> ToSVG for Polyline<T> {}

impl<T> ToSVG for RegularPolygon<T> {}

impl<T> ToSVG for Polygon<T> where T: Display {}
