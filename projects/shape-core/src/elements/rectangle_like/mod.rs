use super::*;
mod traits;

mod parallelogram;
mod rect;
mod square;

pub struct Square<T> {
    anchor: Point<T>,
    side: T,
}

pub struct Rectangle<T> {
    anchor: Point<T>,
    side: (T, T),
}

pub struct Parallelogram<T> {
    anchor: Point<T>,
    side: (Line<T>, Line<T>),
}
