use super::*;

mod circle;
mod circle3d;
mod ellipse;

pub struct Circle<T> {
    pub center: Point<T>,
    pub radius: T,
}

pub struct Circle3D<T> {
    pub center: Point3D<T>,
    pub radius: T,
    pub rotate: (T, T, T),
}

pub struct Ellipse<T> {
    pub center: Point<T>,
    pub radius: (T, T),
    pub angle: T,
}

pub struct Ellipse3D<T> {
    pub center: Point3D<T>,
    pub radius: (T, T, T),
    pub angle: T,
    pub rotate: (T, T, T),
}

pub struct Ball<T> {
    pub center: Point3D<T>,
    pub radius: T,
}
