use super::*;

impl<T> ToSVG for Point<T>
where
    T: Display + One + Clone,
{
    fn to_svg(&self) -> SVG {
        Circle::from(self).to_svg()
    }
}

impl<T> ToSVG for Triangle<T>
where
    T: Display + Clone,
{
    fn to_svg(&self) -> SVG {
        Polygon::from(self).to_svg()
    }
}

impl<T> ToSVG for Square<T>
where
    T: Display,
{
    fn to_svg(&self) -> SVG {
        let attributes = vec![
            ("x", format!("{}", self.anchor.x)),
            ("y", format!("{}", self.anchor.y)),
            ("width", format!("{}", self.side)),
            ("height", format!("{}", self.side)),
        ];
        SVG::new("rect", attributes, vec![])
    }
}

impl<T> ToSVG for Rectangle<T>
where
    T: Display,
{
    fn to_svg(&self) -> SVG {
        let attributes = vec![
            ("x", format!("{}", self.origin.x)),
            ("y", format!("{}", self.origin.y)),
            ("width", format!("{}", self.side.0)),
            ("height", format!("{}", self.side.1)),
        ];
        SVG::new("rect", attributes, vec![])
    }
}

impl<T> ToSVG for Parallelogram<T>
where
    T: Display,
{
    fn to_svg(&self) -> SVG {
        todo!()
        // Polygon::from(self).to_svg()
    }
}

impl<T> ToSVG for Polyline<T> {
    fn to_svg(&self) -> SVG {
        todo!()
    }
}

impl<T> ToSVG for RegularPolygon<T> {
    fn to_svg(&self) -> SVG {
        todo!()
    }
}

impl<T> ToSVG for Polygon<T>
where
    T: Display,
{
    /// <polygon points="100,10 40,198 190,78 10,78 160,198"
    //   style="fill:lime;stroke:purple;stroke-width:5;fill-rule:evenodd;" />
    fn to_svg(&self) -> SVG {
        let points = self.vertex.iter().map(|p| format!("{},{}", p.x, p.y)).collect::<Vec<_>>().join(" ");
        let attributes = vec![("points", points)];
        SVG::new("polygon", attributes, vec![])
    }
}
