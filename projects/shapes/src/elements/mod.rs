pub use line::{Line, Line3D};
pub use point::{Point, Point3D};
pub use triangle::Triangle;

mod line;
mod point;
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

#[derive(Debug, Copy, Clone)]
pub struct Circle {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
}

#[derive(Debug, Clone)]
pub struct Polygon<T> {
    pub points: Vec<Point<T>>,
}

impl<T> Polygon<T> {
    pub fn new<P>(points: Vec<P>) -> Self
    where
        Point<T>: From<P>,
    {
        Self { points: points.into_iter().map(|p| p.into()).collect() }
    }
}
