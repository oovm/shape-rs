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
