use super::*;

pub mod line_2d;
mod line_3d;
mod traits;

/// A line_like segment of finite length in 3D space, determined by a starting point and an ending point
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Line3D<T> {
    /// Start point of the line_like segment in 3D space.
    pub s: Point3D<T>,
    /// End point of the line_like segment in 3D space.
    pub e: Point3D<T>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Vector3D<T> {
    pub dx: T,
    pub dy: T,
    pub dz: T,
}
