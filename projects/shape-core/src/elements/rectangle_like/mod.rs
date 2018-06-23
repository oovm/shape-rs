use super::*;
mod traits;

mod para;
mod rect;
mod square;

/// A square without rotated.
pub struct Square<T> {
    pub anchor: Point<T>,
    pub side: T,
}

/// A rectangle without rotated.
pub struct Rectangle<T> {
    pub anchor: Point<T>,
    pub side: (T, T),
}

pub struct Parallelogram<T> {
    pub anchor: Point<T>,
    pub side: (Point<T>, Point<T>),
}
