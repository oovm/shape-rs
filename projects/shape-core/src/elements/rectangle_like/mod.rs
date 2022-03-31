use super::*;
mod traits;

mod para;
mod rect;
mod square;

pub struct Square<T> {}

pub struct Rectangle<T> {}

pub struct Parallelogram<T> {
    anchor: Point<T>,
    side: (Line<T>, Line<T>),
}
