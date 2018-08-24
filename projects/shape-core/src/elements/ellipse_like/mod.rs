use super::*;

mod circle;
mod circle3d;
mod convert;
mod ellipse;

/// A circle defined by center and radius.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Circle<T> {
    /// The center point of the circle.
    pub center: Point<T>,
    /// The radius of the circle.
    pub radius: T,
}

/// A circle in 3D space defined by center and radius.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Circle3D<T> {
    /// The center point of the circle.
    pub center: Point3D<T>,
    pub radius: T,
    pub rotate: (T, T, T),
}

/// An ellipse defined by center and axes.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Ellipse<T> {
    /// The center point of the ellipse.
    pub center: Point<T>,
    /// The axes of the ellipse.
    pub radius: (T, T),
    /// The rotation of the ellipse.
    pub rotate: T,
}

/// A ellipse in 3D space defined by center and radius.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Ellipse3D<T> {
    /// The center point of the ellipse in 3D space.
    pub center: Point3D<T>,
    /// The axes of the ellipse.
    pub radius: (T, T),
    /// The rotation of the ellipse in 3D space.
    pub rotate: (T, T, T),
}

/// A Ball in 3D space
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Ball<T> {
    pub center: Point3D<T>,
    pub radius: T,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Ellipsoid<T> {
    pub center: Point3D<T>,
    pub radius: (T, T, T),
    pub rotate: (T, T, T),
}
