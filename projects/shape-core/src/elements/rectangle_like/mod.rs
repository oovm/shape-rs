use super::*;
mod traits;

mod para;
pub mod rectangle;
pub mod square;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Parallelogram<T> {
    /// The origin point of the parallelogram.
    pub anchor: Point<T>,
    /// The side length of the parallelogram.
    pub side: (Point<T>, Point<T>),
}
