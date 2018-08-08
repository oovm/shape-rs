use super::*;
use shape_core::{Circle, Ellipse, One, Parallelogram, Point, Polygon, Triangle, Zero};

impl<T> ToWolfram for Point<T>
where
    T: Display + One + Clone,
{
    fn to_wolfram(&self) -> SVG {
        Circle::from(self).to_wolfram()
    }
}

impl<T> ToWolfram for Circle<T>
where
    T: Display,
{
    fn to_wolfram(&self) -> SVG {
        let attributes = vec![
            //
            ("cx", format!("{}", self.center.x)),
            ("cy", format!("{}", self.center.y)),
            ("r", format!("{}", self.radius)),
        ];
        SVG::new("circle", attributes, vec![])
    }
}

impl<T> ToWolfram for Ellipse<T>
where
    T: Display + PartialEq + Zero,
{
    fn to_wolfram(&self) -> SVG {
        let mut attributes = vec![
            ("cx", format!("{}", self.center.x)),
            ("cy", format!("{}", self.center.y)),
            ("rx", format!("{}", self.radius.0)),
            ("ry", format!("{}", self.radius.1)),
        ];
        if !self.is_horizontal() {
            let rotate = format!("rotate({}, {} {})", self.angle, self.center.x, self.center.y);
            attributes.push(("transform", rotate));
        }

        SVG::new("ellipse", attributes, vec![])
    }
}

impl<T> ToWolfram for Triangle<T>
where
    T: Display + Clone,
{
    fn to_wolfram(&self) -> SVG {
        Polygon::from(self).to_wolfram()
    }
}

impl<T> ToWolfram for Square<T>
where
    T: Display,
{
    fn to_wolfram(&self) -> SVG {
        let attributes = vec![
            ("x", format!("{}", self.anchor.x)),
            ("y", format!("{}", self.anchor.y)),
            ("width", format!("{}", self.side)),
            ("height", format!("{}", self.side)),
        ];
        SVG::new("rect", attributes, vec![])
    }
}

impl<T> ToWolfram for Rectangle<T>
where
    T: Display,
{
    fn to_wolfram(&self) -> SVG {
        let attributes = vec![
            ("x", format!("{}", self.anchor.x)),
            ("y", format!("{}", self.anchor.y)),
            ("width", format!("{}", self.side.0)),
            ("height", format!("{}", self.side.1)),
        ];
        SVG::new("rect", attributes, vec![])
    }
}

impl<T> ToWolfram for Parallelogram<T>
where
    T: Display,
{
    fn to_wolfram(&self) -> SVG {
        todo!()
        // Polygon::from(self).to_svg()
    }
}

impl<T> ToWolfram for Polygon<T>
where
    T: Display,
{
    /// <polygon points="100,10 40,198 190,78 10,78 160,198"
    //   style="fill:lime;stroke:purple;stroke-width:5;fill-rule:evenodd;" />
    fn to_wolfram(&self) -> SVG {
        let points = self.vertex.iter().map(|p| format!("{},{}", p.x, p.y)).collect::<Vec<_>>().join(" ");
        let attributes = vec![("points", points)];
        SVG::new("polygon", attributes, vec![])
    }
}