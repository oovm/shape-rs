mod ellipse_like;
mod polygon_like;
use shape_core::{
    Circle, Ellipse, One, Parallelogram, Point, Polygon, Polyline, Rectangle, RegularPolygon, Square, Triangle, Zero,
};
use std::fmt::Display;
use svg::node::{element::SVG, Value};
/// Mark a type that can be convert to svg
pub trait ToSVG {
    type Element;
    /// Convert to svg by reference
    fn to_svg(&self) -> Self::Element;
}
