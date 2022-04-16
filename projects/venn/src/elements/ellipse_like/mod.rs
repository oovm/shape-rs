use super::*;

mod circle;
mod circle3d;
mod convert;
mod ellipse;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Circle<T> {
    pub center: Point<T>,
    pub radius: T,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Circle3D<T> {
    pub center: Point3D<T>,
    pub radius: T,
    pub rotate: (T, T, T),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Ellipse<T> {
    pub center: Point<T>,
    pub radius: (T, T),
    pub angle: T,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Ellipse3D<T> {
    pub center: Point3D<T>,
    pub radius: (T, T, T),
    pub angle: T,
    pub rotate: (T, T, T),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Ball<T> {
    pub center: Point3D<T>,
    pub radius: T,
}
