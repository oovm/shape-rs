use super::*;

mod circle;
mod ellipse;

pub struct Circle<T> {
    pub center: Point<T>,
    pub radius: T,
}

pub struct Ellipse<T> {
    pub center: Point<T>,
    pub radius: (T, T),
    pub angle: T,
}

pub struct Ball<T> {
    pub center: Point3D<T>,
    pub radius: (T, T, T),
    pub angle: (T, T, T),
}
