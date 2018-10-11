mod ellipse_like;
mod line_like;
mod point_like;
mod polygon_like;
use shape_core::{Circle, Ellipse, Parallelogram, Point, Polygon, Polyline, Rectangle, RegularPolygon, Square, Triangle};
use std::fmt::Display;
use svg::node::Value;

/// Mark a type that can be convert to svg
pub trait ToSVG {
    /// The svg element type
    type Element;
    /// Convert to svg by reference
    fn to_svg(&self) -> Self::Element;
}
