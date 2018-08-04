use super::*;
mod traits;

mod para;
mod rect;
mod square;

/// A square without rotated.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Square<T> {
    pub anchor: Point<T>,
    pub side: T,
}

/// A rectangle without rotated.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Rectangle<T> {
    pub anchor: Point<T>,
    pub side: (T, T),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Parallelogram<T> {
    pub anchor: Point<T>,
    pub side: (Point<T>, Point<T>),
}
