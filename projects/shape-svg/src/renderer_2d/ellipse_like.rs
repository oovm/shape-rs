use super::*;

impl<T> ToSVG for Circle<T>
where
    T: Display,
{
    fn to_svg(&self) -> SVG {
        let attributes = vec![
            //
            ("cx", format!("{}", self.center.x)),
            ("cy", format!("{}", self.center.y)),
            ("r", format!("{}", self.radius)),
        ];
        SVG::new("circle", attributes, vec![])
    }
}

impl<T> ToSVG for Ellipse<T>
where
    T: Display + PartialEq + Zero,
{
    fn to_svg(&self) -> SVG {
        let mut attributes = vec![
            ("cx", format!("{}", self.center.x)),
            ("cy", format!("{}", self.center.y)),
            ("rx", format!("{}", self.radius.0)),
            ("ry", format!("{}", self.radius.1)),
        ];
        if !self.is_horizontal() {
            let rotate = format!("rotate({}, {} {})", self.rotate, self.center.x, self.center.y);
            attributes.push(("transform", rotate));
        }

        SVG::new("ellipse", attributes, vec![])
    }
}
