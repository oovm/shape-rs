use super::*;
mod convert;
mod dim2;
mod dim3;
mod proj;

/// A 2D point.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point3D<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}
