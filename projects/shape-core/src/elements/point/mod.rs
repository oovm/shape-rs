use super::*;
mod convert;
mod dim2;
mod dim3;
mod proj;

/// A 2D point.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Point3D<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Point4D<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}
