use super::*;

mod dim2;
mod dim3;
mod traits;

/// A line segment of finite length, determined by a starting point and an ending point.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Line<T> {
    /// Start point of the line segment.
    pub start: Point<T>,
    /// End point of the line segment.
    pub end: Point<T>,
}

/// A line segment of finite length in 3D space, determined by a starting point and an ending point
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Line3D<T> {
    /// Start point of the line segment in 3D space.
    pub start: Point3D<T>,
    /// End point of the line segment in 3D space.
    pub end: Point3D<T>,
}

/// represents an infinitely long line segment
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Vector<T> {
    pub dx: T,
    pub dy: T,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Vector3D<T> {
    pub dx: T,
    pub dy: T,
    pub dz: T,
}
