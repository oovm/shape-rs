use super::*;
mod traits;

mod para;
pub mod rectangle;
pub mod square;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Parallelogram<T> {
    pub anchor: Point<T>,
    pub side: (Point<T>, Point<T>),
}
