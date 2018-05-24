pub use ellipse::{Circle, Ellipse};
pub use line::{Line, Line3D};
pub use point::{Point, Point3D};
pub use polygon::Polygon;
pub use triangle::Triangle;

mod circle;
mod ellipse;
mod line;
mod point;
mod polygon;
mod rectangle;
mod triangle;

/// A rectangle.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Square {
    pub x: f32,
    pub y: f32,
    pub side: f32,
}

/// A rectangle.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}
