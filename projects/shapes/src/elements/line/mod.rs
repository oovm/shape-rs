mod traits;
mod dim2;
mod dim3;
use super::*;



/// A circle.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Line<T> {
    pub start: Point<T>,
    pub end: Point<T>,
}

/// A circle.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Line3D<T> {
    pub start: Point3D<T>,
    pub end: Point3D<T>,
}
