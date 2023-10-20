use super::*;

mod circle;
mod circle3d;
mod convert;
mod ellipse;

/// A circle defined by center and radius.
#[cfg_attr(feature = "serde", repr(C), derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Circle<T> {
    /// The center points of the circle.
    pub center: Point<T>,
    /// The radius of the circle.
    pub radius: T,
}

/// A circle in 3D space defined by center and radius.
#[cfg_attr(feature = "serde", repr(C), derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Circle3D<T> {
    /// The center points of the circle.
    pub center: Point3D<T>,
    pub radius: T,
    pub rotate: (T, T, T),
}

/// An ellipse defined by center and axes.
#[cfg_attr(feature = "serde", repr(C), derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Ellipse<T> {
    /// The center points of the ellipse.
    pub center: Point<T>,
    /// The axes of the ellipse.
    pub radius: (T, T),
    /// The rotation of the ellipse.
    pub rotate: T,
}

/// A ellipse in 3D space defined by center and radius.
#[cfg_attr(feature = "serde", repr(C), derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Ellipse3D<T> {
    /// The center points of the ellipse in 3D space.
    pub center: Point3D<T>,
    /// The axes of the ellipse.
    pub radius: (T, T),
    /// The rotation of the ellipse in 3D space.
    pub rotate: (T, T, T),
}

/// A Ball in 3D space
#[cfg_attr(feature = "serde", repr(C), derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Ball<T> {
    /// The coordinates of the center point of the ball
    pub center: Point3D<T>,
    /// The radius of the ball
    pub radius: T,
}

#[cfg_attr(feature = "serde", repr(C), derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Ellipsoid<T> {
    /// The coordinates of the center point of the ellipsoid
    pub center: Point3D<T>,
    /// The radius from x, y, z axes
    pub radius: (T, T, T),
    /// The rotate from a, b, y corner
    pub rotate: (T, T, T),
}
