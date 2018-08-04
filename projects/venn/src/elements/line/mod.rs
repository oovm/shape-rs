use super::*;

mod dim2;
mod dim3;
mod traits;

/// A circle.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Line<T> {
    pub start: Point<T>,
    pub end: Point<T>,
}

/// A circle.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Line3D<T> {
    pub start: Point3D<T>,
    pub end: Point3D<T>,
}

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
