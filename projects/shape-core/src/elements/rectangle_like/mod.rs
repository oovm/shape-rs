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
    /// origin x point of the rectangle
    pub x: T,
    /// origin y point of the rectangle
    pub y: T,
    /// width of the rectangle
    pub w: T,
    /// height of the rectangle
    pub h: T,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Parallelogram<T> {
    pub anchor: Point<T>,
    pub side: (Point<T>, Point<T>),
}
