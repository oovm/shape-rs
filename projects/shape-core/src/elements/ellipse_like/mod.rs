use super::*;

mod circle;
mod circle3d;
mod convert;
mod ellipse;

///
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Circle<T> {
    /// The center point of the circle.
    pub center: Point<T>,
    /// The radius of the circle.
    pub radius: T,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Circle3D<T> {
    /// The center point of the circle.
    pub center: Point3D<T>,
    pub radius: T,
    pub rotate: (T, T, T),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Ellipse<T> {
    pub center: Point<T>,
    pub radius: (T, T),
    pub rotate: T,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Ellipse3D<T> {
    pub center: Point3D<T>,
    pub radius: (T, T),
    pub rotate: (T, T, T),
}

/// A Ball in 3D space
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Ball<T> {
    pub center: Point3D<T>,
    pub radius: T,
}

pub struct Ellipsoid<T> {
    pub center: Point3D<T>,
    pub radius: (T, T, T),
    pub rotate: (T, T, T),
}
