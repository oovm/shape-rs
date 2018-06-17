use super::*;
use shape_core::{Polygon, Triangle};

impl<T> ToSVG for Triangle<T>
where
    T: Display,
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
        let attributes = &[
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
        let attributes = &[
            ("x", format!("{}", self.anchor.x)),
            ("y", format!("{}", self.anchor.y)),
            ("width", format!("{}", self.side.0)),
            ("height", format!("{}", self.side.1)),
        ];
        SVG::new("rect", attributes, vec![])
    }
}

impl<T> ToSVG for Triangle<T>
where
    T: Display,
{
    fn to_svg(&self) -> SVG {
        Polygon::from(self).to_svg()
    }
}
