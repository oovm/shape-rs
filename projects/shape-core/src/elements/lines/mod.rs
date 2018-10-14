use super::*;

pub mod line_2d;
mod line_3d;
mod traits;

/// A lines segment of finite length in 3D space, determined by a starting points and an ending points
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Line3D<T> {
    /// Start points of the lines segment in 3D space.
    pub s: Point3D<T>,
    /// End points of the lines segment in 3D space.
    pub e: Point3D<T>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Vector3D<T> {
    pub dx: T,
    pub dy: T,
    pub dz: T,
}
