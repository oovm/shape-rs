use super::*;

mod dim2;
mod dim3;
mod traits;

/// A circle.
#[derive(Clone, Copy, Debug)]
pub struct Line<T> {
    pub start: Point<T>,
    pub end: Point<T>,
}

impl<T> PartialEq for Line<T>
where
    T: PartialEq,
{
    /// If two vectors are collinear, then the two lines coincide
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start && self.end == other.end
    }
}

/// A circle.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Line3D<T> {
    pub start: Point3D<T>,
    pub end: Point3D<T>,
}
